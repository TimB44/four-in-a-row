use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use multiplayer::start_cleanup;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex, task};
use tower_http::services::{ServeDir, ServeFile};

mod ai;
mod multiplayer;
use multiplayer::GameState;

type GameMap = Arc<Mutex<HashMap<u32, GameState>>>;

#[tokio::main]
async fn main() {
    let state: Arc<Mutex<HashMap<u32, GameState>>> = Arc::new(Mutex::new(HashMap::new()));
    start_cleanup(Arc::clone(&state));

    let routes_hello = Router::new()
        .nest("/multiplayer", multiplayer::routes())
        .with_state(state)
        .route("/ai", post(get_ai_move))
        .nest_service("/", ServeFile::new("static/index.html"))
        .nest_service("/static", ServeDir::new("static"));

    //"127.0.0.1:8080"
    let listener = TcpListener::bind("127.0.0.1:8080") //192.168.86.21:8080 127.0.0.1:8080
        .await
        .expect("Could not create the tcp listener");
    println!("->> LISTENING on {:?} \n", listener.local_addr());

    axum::serve(listener, routes_hello).await.unwrap();
}

async fn get_ai_move(Json(params): Json<GameParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let dif = params.difficulty.as_deref().unwrap_or("BAD");
    let board = params.board.unwrap();
    let game_move = match dif {
        "easy" => task::spawn_blocking(move || game_lib::next_easy_move(board)).await,
        "medium" => task::spawn_blocking(move || game_lib::next_medium_move(board)).await,
        "hard" => task::spawn_blocking(move || game_lib::next_hard_move(board)).await,
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                "Invalid difficulty, must be easy, medium or hard",
            )
                .into_response()
        }
    };
    let game_move = match game_move {
        Ok(Ok(game_move)) => game_move,
        Ok(Err(msg)) => return (StatusCode::BAD_REQUEST, msg).into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Json(json!({"move" : game_move,})).into_response()
}

#[derive(Debug, Deserialize)]
struct GameParams {
    board: Option<[[i8; 7]; 6]>,
    difficulty: Option<String>,
}
