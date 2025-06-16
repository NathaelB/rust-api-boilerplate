use std::sync::Arc;
use axum::Router;
use crate::env::Env;

pub struct HttpServerConfig {
    port: String,
}

impl HttpServerConfig {
    pub fn new(port: String) -> Self {
        Self { port }
    }
}

pub struct HttpServer {
    router: Router,
    listener: tokio::net::TcpListener,
}

impl HttpServer {
    pub async fn new(
        env: Arc<Env>,
        config: HttpServerConfig,
    ) -> Result<Self, snafu::Whatever> {
        todo!("Implement the HTTP server setup logic here, including routing, middleware, and listener setup.");
    }

    pub async fn run(self) -> Result<(), snafu::Whatever> {
        todo!("Implement the logic to run the HTTP server, including binding to the listener and starting the server.");
    }
}