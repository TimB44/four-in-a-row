use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use game_lib::board::GameBoard;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::broadcast::{Receiver, Sender};
use std::{sync::atomic::AtomicU32, time::Instant};

use crate::GameMap;

static GAME_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
pub(crate) struct GameState{
    game_id: u32,
    game_state: GameBoard,
    update_event: Sender<[[i8;7];6]>,
    last_update: Instant, 
    // some controller
}
impl GameState {
    fn new(game_id: u32) -> GameState {
        todo!()
    }
}

#[derive(Deserialize)]
pub(crate) struct GetBoardParams{
    game_id: Option<u32>,
    current_board: Option<[[i8; 7]; 6]>,
}

pub(crate) async fn create_game(State(games): State<GameMap>) -> impl IntoResponse {
    let game_id = GAME_ID_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let game = GameState::new(game_id);
    games.lock().expect("Mutex around games was poisoned").insert(game_id, game);
    Json(json!({"id" : game_id}))
}

pub(crate) async fn get_next_update(State(games): State<GameMap>, Json(params):Json<GetBoardParams>) -> impl IntoResponse {
    let id =  match params.game_id {
        Some(id) => id,
        None => return (StatusCode::BAD_REQUEST, "No game_id found in the request").into_response()
    };
    let prev_board = match params.current_board {
        Some(board) => board,
        None => return (StatusCode::BAD_REQUEST, "No current_board found in the request").into_response(),
    };

    let mut game_map = games.lock().expect("Mutex around games was poisoned");

    let game  = match game_map.get(&id) {
        Some(game) => game,
        None => return (StatusCode::NOT_FOUND, "No game found with given id").into_response(),
    };
    let mut rx = game.update_event.subscribe();
    
    if !game.game_state.equals_arr(&prev_board) {
        return Json(json!({"board" : game.game_state.to_arr()})).into_response();
    }

    // Do not hold onto the mutex while waiting for an update
    drop(game_map);

    let update = match rx.recv().await {
        Ok(board) => board,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "No further update for the game").into_response(),
    };
    
    Json(json!({"board" : update})).into_response()
}