use axum::extract::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use serde_json::json;
use tokio::task;

pub(crate) async fn get_ai_move(Json(params): Json<GetMoveParams>) -> impl IntoResponse {
    let dif = match params.difficulty {
        Some(dif) => dif,
        None => return (StatusCode::BAD_REQUEST, "Must give difficulty parameter").into_response(),
    };
    let board = match params.board {
        Some(board) => board,
        None => return (StatusCode::BAD_REQUEST, "Must give board parameter").into_response(),
    };
    let game_move = match dif.as_str() {
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

#[derive(Deserialize, Debug)]
pub(crate) struct GetMoveParams {
    board: Option<[[i8; 7]; 6]>,
    difficulty: Option<String>,
}
