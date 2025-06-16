use clap::Parser;
use lib::env::{AppEnv, Env};
use std::sync::Arc;
use tracing::info;

fn init_logger(env: Arc<Env>) {
    match env.env {
        AppEnv::Development => {
            tracing_subscriber::fmt::init();
        }
        AppEnv::Production => {
            tracing_subscriber::fmt()
                .json()
                .with_max_level(tracing::Level::INFO)
                .with_writer(std::io::stdout)
                .init();
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), snafu::Whatever> {
    dotenv::dotenv().ok();

    let env = Arc::new(Env::parse());
    init_logger(env.clone());

    info!("Starting application in {} mode", env.env.to_string());

    Ok(())
}
