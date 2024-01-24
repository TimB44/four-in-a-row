use axum::{response::{ IntoResponse}, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/ai", 
        post(handler_hello));

    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Could not create the tcp listener");
    println!("->> LISTENING on {:?} \n", listener.local_addr());

    axum::serve(listener, routes_hello.into_make_service()).await.unwrap();
} 

async fn handler_hello(Json(params): Json<GameParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let dif = params.difficulty.as_deref().unwrap_or("BAD"); 
    let board = params.board.unwrap();

    let game_move = match dif {
        "easy" => {game_lib::next_easy_move(board).unwrap()}
        _ => todo!()
    };


    Json(json!({"move" : game_move,}))
}
#[derive(Debug, Deserialize)]
struct GameParams {
    board: Option<[[i32; 7]; 6]>,
    difficulty: Option<String>, 
}
