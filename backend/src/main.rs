mod tests;
mod models;
mod db;

use std::sync::Arc;

use axum::{extract::State, response::Json, routing::get, Router};

async fn return_json(State(state): State<Arc<AppState>>) -> Json<Value> {
    Json(json!({"data": state.count  }))
}

use serde_json::{json, Value};

struct AppState {
    count: i32,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState { count: 1 });

    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/json", get(return_json))
        .with_state(shared_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
