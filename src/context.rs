use std::borrow::Borrow;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use axum::{Router, Server};
use axum::routing::{IntoMakeService, MethodRouter};

pub struct AxumRunner {
    addr: SocketAddr,
    router: Router,
}

impl AxumRunner {
    pub fn new(addr: SocketAddr) -> AxumRunner {
        AxumRunner {
            addr,
            router: Router::new()
        }
    }

    fn chain(addr: SocketAddr, router: Router) -> Self {
        Self {addr, router}
    }

    pub async fn run(self)  {
        Server::bind(&self.addr)
            .serve(self.router.into_make_service())
            .await
            .unwrap()
    }

    pub fn add_route(mut self, path: &str, service: MethodRouter) -> Self {
        Self { addr: self.addr, router: self.router.clone().route(path, service) }
    }
}
