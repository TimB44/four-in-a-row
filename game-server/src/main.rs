use axum::{routing::post, Router};
use axum_server::tls_rustls::RustlsConfig;
use multiplayer::start_cleanup;
use std::net::SocketAddr;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tower_http::services::{ServeDir, ServeFile};

mod ai;
mod multiplayer;
use ai::get_ai_move;
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
        .nest_service("/static", ServeDir::new("static"));

    let config = RustlsConfig::from_pem_file("keys/MyCertificate.crt", "keys/MyKey.key")
        .await
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], 443));
    println!("listening on {}", addr);
    axum_server::bind_rustls(addr, config)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
