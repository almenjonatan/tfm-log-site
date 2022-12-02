use axum::{
    response::Json,
    routing::{get, post},
    Router,
};

use crate::models::game::Game;

async fn upload(Json(game): Json<Game>) {
    println!("{:#?}", game);
}

async fn hello() -> &'static str {
    "hello world"
}

pub async fn run() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "example_print_request_response=debug,tower_http=debug",
        )
    }

    let app = Router::new()
        .route("/upload", post(upload))
        .route("/hello", get(hello));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
