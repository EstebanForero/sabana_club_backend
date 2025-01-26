use std::error::Error;

use api_server::start_http_server;
use global_traits::HttpService;
use serde::Deserialize;
use tournament_service::endpoints::TournamentHttpServer;
use tracing::{error, info};
use trainings_service::endpoints::TrainingHttpServer;
use user_service::endpoints::UserHttpServer;

mod api_server;
mod models;

pub mod auth_middleware;
mod global_traits;
mod tournament_service;
mod trainings_service;
mod tuition_service;
pub mod user_service;

#[derive(Debug, Deserialize)]
struct Config {
    db_url: String,
    db_token: String,
    port: String,
    token_key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::fmt::init();

    let config: Config = envy::from_env()?;

    let services: Vec<Box<dyn HttpService>> = vec![
        Box::new(UserHttpServer::new(&config.db_url, &config.db_token, config.token_key).await),
        Box::new(TournamentHttpServer::new(&config.db_url, &config.db_token).await),
        Box::new(TrainingHttpServer::new(&config.db_url, &config.db_token).await),
    ];

    match start_http_server(config.port, services).await {
        Ok(_) => info!("Http server started succesfully"),
        Err(err) => error!("Error starting http server: {err}"),
    };

    Ok(())
}
