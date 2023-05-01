use std::net::SocketAddr;
use axum::{routing::get, Router, Server};
use axum::routing::{IntoMakeService, MethodRouter};
use context::AxumRunner;

mod context;
mod domain;
use domain::account;
use std::collections::HashMap;
use std::process::id;
use tokio_postgres::{Config, Error, NoTls};
use crate::domain::index;

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
async fn main() -> Result<(), Error> {
    // Set Default Router, Addr
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Connect Postgre
    let (client, connection) = Config::new().host("localhost").user("rndso15").port(5432).password("ring123").dbname("homeapi").connect(NoTls).await.unwrap();

    // Connection Spawn
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error : {}", e);
        }
    });

    let mut rows = client.query("SELECT * FROM account", &[]).await?;
    for row in rows {
        let id:i32 = row.get("id");
        let email:&str = row.get("email");
        println!("{} / {}", id, email);
    }

    // First access to `HASHMAP` initializes it
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());

    AxumRunner::new(addr)
        .add_route("/", get(root))
        .add_route("/status", get(index::controller::index_controller::get_status))
        .run().await;

    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
