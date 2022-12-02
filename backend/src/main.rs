mod db;
mod models;
mod server;
mod tests;

#[tokio::main]
async fn main() {
    server::run().await;
}
