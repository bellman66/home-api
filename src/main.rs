use std::net::SocketAddr;
use axum::{routing::get, Router, Server};
use axum::routing::{IntoMakeService, MethodRouter};
use context::AxumRunner;

mod context;
mod domain;
use domain::account;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref HASHMAP: HashMap<i8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}

#[tokio::main]
async fn main() {
    // Set Default Router, Addr
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // First access to `HASHMAP` initializes it
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

    AxumRunner::new(addr)
        .add_route("/", get(root))
        .add_route("/status", get(account::controller::accountcontroller::get_status))
        .run().await;
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
