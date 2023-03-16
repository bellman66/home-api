use std::net::SocketAddr;
use axum::{routing::get, Router, Server};

mod domain;
use domain::account;

#[tokio::main]
async fn main() {
    // Set Default Router, Addr
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let router = get_router();

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

fn get_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/status", get(account::controller::accountcontroller::get_status))
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
