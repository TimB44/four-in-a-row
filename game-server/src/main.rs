use axum::{http::{StatusCode, Uri}, response::IntoResponse, routing::post, Json, Router};
use axum_server::tls_rustls::RustlsConfig;
use multiplayer::start_cleanup;
use serde::Deserialize;
use serde_json::json;
use std::{collections::HashMap, net::SocketAddr};
use std::sync::Arc;
use tokio::{ sync::Mutex, task};
use tower_http::services::{ServeDir, ServeFile};

mod ai;
mod multiplayer;
use multiplayer::GameState;

type GameMap = Arc<Mutex<HashMap<u32, GameState>>>;

#[tokio::main]
async fn main() {
    let state: Arc<Mutex<HashMap<u32, GameState>>> = Arc::new(Mutex::new(HashMap::new()));
    start_cleanup(Arc::clone(&state));

    let routes = Router::new()
        .nest("/multiplayer", multiplayer::routes())
        .with_state(state)
        .route("/ai", post(get_ai_move))
        .nest_service("/", ServeFile::new("static/index.html"))
        .nest_service("/static", ServeDir::new("static"))
        .fallback(handle_fallback);

        let config = RustlsConfig::from_pem_file(
            "keys/MyCertificate.crt",
            "keys/MyKey.key",
        )
        .await
        .unwrap();
    
        let addr = SocketAddr::from(([0, 0, 0, 0], 443));
        println!("listening on {}", addr);
        axum_server::bind_rustls(addr, config)
            .serve(routes.into_make_service())
            .await
            .unwrap();
}

async fn handle_fallback(uri: Uri){
    println!("In fallback, uri: {:?}", uri);
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
