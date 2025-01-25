use std::error::Error;

use api_server::start_http_server;
use serde::Deserialize;
use tracing::{error, info};

mod api_server;
mod models;

mod tournament_service;
mod trainings_service;
mod tuition_service;
mod user_service;

mod global_traits;

#[derive(Debug, Deserialize)]
struct Config {
    db_url: String,
    db_token: String,
    port: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    let config: Config = envy::from_env()?;

    let services = vec![];

    match start_http_server(config.port, services).await {
        Ok(_) => info!("Http server started succesfully"),
        Err(err) => error!("Error starting http server: {err}"),
    };

    let idk: Option<Box<dyn Something>> = None;

    Ok(())
}

trait Something: Sync {
    async fn something(&self);
}
