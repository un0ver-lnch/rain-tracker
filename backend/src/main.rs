mod models;

use axum::{routing::get, extract::State, Json, Router};
use models::{AppState, RainRecord};
use std::net::SocketAddr;

async fn root() -> &'static str {
    "Rain Tracker backend running"
}

async fn list_records(State(state): State<AppState>) -> Json<Vec<RainRecord>> {
    Json(state.records.clone())
}

#[tokio::main]
async fn main() {
    let app_state = AppState::default();

    let app = Router::new()
        .route("/", get(root))
        .route("/records", get(list_records))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

