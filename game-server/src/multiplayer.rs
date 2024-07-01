use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::GameMap;
use game_lib::board::{GameBoard, GameMove, GamePlayer};
use serde::Deserialize;
use serde_json::json;
use std::{sync::atomic::AtomicU32, time::Instant};
use tokio::sync::broadcast::{self, Sender};

const CHANNEL_CAPACITY: usize = 100;
const SECS_BETWEEN_CLEANUP: u64 = 60;
const GAME_TIMEOUT_DURATION: u64 = 300;

static GAME_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
pub(crate) struct GameState {
    game_state: GameBoard,
    update_event: Sender<[[i8; 7]; 6]>,
    last_update: Instant,
    reset_by_one: bool,
}
impl GameState {
    fn new() -> GameState {
        GameState {
            game_state: GameBoard::new(),
            update_event: broadcast::channel(CHANNEL_CAPACITY).0,
            last_update: Instant::now(),
            reset_by_one: false,
        }
    }

    fn reset(&mut self) -> Result<(), &'static str> {
        self.last_update = Instant::now();
        if !self.game_state.is_over() {
            return Err("Can not reset a game that is not over");
        }

        if !self.reset_by_one {
            self.reset_by_one = true;
            return Ok(());
        }

        self.game_state = GameBoard::new();
        self.reset_by_one = false;
        let _ = self.update_event.send([[0; 7]; 6]);
        Ok(())
    }
}

pub(crate) fn routes() -> Router<GameMap> {
    Router::new()
        .route("/create_game", post(create_game))
        .route("/get_board", post(get_next_update))
        .route("/make_move", post(make_move))
        .route("/reset_game", post(reset_game))
}

#[derive(Deserialize)]
struct GetBoardParams {
    game_id: Option<u32>,
    current_board: Option<[[i8; 7]; 6]>,
}

async fn create_game(State(games): State<GameMap>) -> impl IntoResponse {
    let game_id = GAME_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let game = GameState::new();
    games.lock().await.insert(game_id, game);
    Json(json!({"id" : game_id}))
}

async fn get_next_update(
    State(games): State<GameMap>,
    Json(params): Json<GetBoardParams>,
) -> impl IntoResponse {
    let id = match params.game_id {
        Some(id) => id,
        None => {
            return (StatusCode::BAD_REQUEST, "No game_id found in the request").into_response()
        }
    };
    let prev_board = match params.current_board {
        Some(board) => board,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                "No current_board found in the request",
            )
                .into_response()
        }
    };

    let game_map = games.lock().await;

    let game = match game_map.get(&id) {
        Some(game) => game,
        None => return (StatusCode::NOT_FOUND, "No game found with given id").into_response(),
    };
    let mut rx = game.update_event.subscribe();

    if !game.game_state.equals_arr(&prev_board) {
        return Json(json!({"board" :  game.game_state.to_arr()})).into_response();
    }

    // Do not hold onto the mutex while waiting for an update
    drop(game_map);

    let update = match rx.recv().await {
        Ok(board) => board,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "No further update for the game",
            )
                .into_response()
        }
    };

    Json(json!({"board" : update})).into_response()
}

#[derive(Deserialize)]
struct MakeMoveParams {
    game_move: Option<u8>,
    is_first: Option<bool>,
    game_id: Option<u32>,
}
async fn make_move(
    State(games): State<GameMap>,
    Json(params): Json<MakeMoveParams>,
) -> impl IntoResponse {
    let game_move = match params.game_move {
        Some(game_move) => game_move,
        None => {
            return (StatusCode::BAD_REQUEST, "Must provide game_move parameter").into_response()
        }
    };

    let is_first = match params.is_first {
        Some(is_first) => is_first,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                "Must provide is_first boolean parameter",
            )
                .into_response()
        }
    };

    let game_id = match params.game_id {
        Some(game_id) => game_id,
        None => return (StatusCode::BAD_REQUEST, "Must provide game_id parameter").into_response(),
    };

    let mut lock = games.lock().await;

    let game = match lock.get_mut(&game_id) {
        Some(game) => game,
        None => {
            return (
                StatusCode::NOT_FOUND,
                "No game with the given game_id found",
            )
                .into_response()
        }
    };

    let player = match is_first {
        true => GamePlayer::FirstPlayer,
        false => GamePlayer::SecondPlayer,
    };

    if game.game_state.current_player() != player {
        return (
            StatusCode::BAD_REQUEST,
            "Can not play move when not your turn",
        )
            .into_response();
    }

    let game_move = match GameMove::build(game_move) {
        Ok(game_move) => game_move,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                format!(
                    "{} is not a valid move, valid moves must be between 0 and 6 inclusive",
                    game_move
                ),
            )
                .into_response()
        }
    };

    match game.game_state.make_move(&game_move) {
        Ok(_) => (),
        Err(msg) => return (StatusCode::BAD_REQUEST, msg).into_response(),
    }

    // If no one is listening that is fine
    let _ = game.update_event.send(game.game_state.to_arr());
    game.last_update = Instant::now();

    StatusCode::OK.into_response()
}

pub(crate) fn start_cleanup(map: GameMap) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(SECS_BETWEEN_CLEANUP)).await;
            let mut guard = map.lock().await;
            let now = Instant::now();
            guard.retain(|_, game| {
                now.duration_since(game.last_update).as_secs() < GAME_TIMEOUT_DURATION
            });
        }
    })
}

#[derive(Deserialize)]
struct ResetGameParams {
    game_id: Option<u32>,
}
async fn reset_game(
    State(games): State<GameMap>,
    Json(params): Json<ResetGameParams>,
) -> impl IntoResponse {
    let id = match params.game_id {
        Some(id) => id,
        None => return (StatusCode::BAD_REQUEST, "Must provide game_id parameter").into_response(),
    };

    let mut map = games.lock().await;

    let game = match map.get_mut(&id) {
        Some(game) => game,
        None => {
            return (
                StatusCode::NOT_FOUND,
                "No game with the given game_id found",
            )
                .into_response()
        }
    };

    let mut rx = game.update_event.subscribe();

    if let Err(err) = game.reset() {
        (StatusCode::BAD_REQUEST, err).into_response();
    }

    // Don't Hold onto Mutex while waiting for other player to reset
    drop(map);

    if rx.recv().await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }
    StatusCode::OK.into_response()
}
