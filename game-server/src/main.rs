use std::fmt::Display;

use lambda_http::{run, service_fn, Error, Request};
use serde::{Deserialize, Serialize};
use serde_json::{json, value::Value};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Value, Error> {
    let msg = event.body();
    let body =match msg {
        lambda_http::Body::Text(s) => &s[..],
        _ => return Err(Box::new(GameMoveRequestError("Invalid Body")))
    };

    let body:GameMoveRequest = serde_json::from_str(body)?; 


    let json = match &body.difficulty[..] {
        "easy" => match game_lib::next_easy_move(body.board) {
            Ok(col) => json!({
                "move": col
            }),
            Err(msg) => json!({
                "msg": msg
            }),
        },
        _ => json!({
            "msg": "The only supported difficulties are easy."
        }),
    };

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    // let resp = Response::builder()
    //     .status(200)
    //     .header("content-type", "text/html")
    //     .body(message.into())
    //     .map_err(Box::new)?;
    Ok(json)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await?;

    Ok(())
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameMoveRequest {
    pub difficulty: String,
    pub board: [[i32; 7]; 6],
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameMoveResponse {
    game_move: u8,
}

#[derive(Debug)]
struct GameMoveRequestError(&'static str);

impl Display for GameMoveRequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for GameMoveRequestError {}
