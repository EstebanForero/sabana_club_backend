use std::{error::Error, sync::Arc};

use api_server::start_http_server;
use global_traits::HttpService;
use serde::Deserialize;
use tournament_service::{
    endpoints::TournamentHttpServer, repository::lib_sql_implementation::TournamentRepositoryImpl,
};
use tracing::{error, info};
use trainings_service::{
    endpoints::TrainingHttpServer, repository::lib_sql_implementation::TrainingRepositoryImpl,
};
use tuition_service::{
    endpoints::TuitionHttpServer, repository::lib_sql_implementation::TuitionRepositoryImpl,
};
use unique_identifier_service::{
    repository::lib_sql_implementation::LibSqlUniqueIdentifierRepo,
    usecases::build_unique_identifier,
};
use user_service::{
    endpoints::UserHttpServer, repository::libsql_implementation::LibSqlUserRepository,
};

mod api_server;
mod models;

pub mod auth_middleware;
mod global_traits;
mod tournament_service;
mod trainings_service;
mod tuition_service;
pub mod unique_identifier_service;
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

    let user_repository = LibSqlUserRepository::new(&config.db_url, &config.db_token)
        .await
        .expect("Error creating user repository");

    let unique_identifier_repository =
        LibSqlUniqueIdentifierRepo::new(&config.db_url, &config.db_token)
            .await
            .expect("Error creating unique identifier repository");

    let tournament_repository = TournamentRepositoryImpl::new(&config.db_url, &config.db_token)
        .await
        .expect("Error creating tournment repository");

    let training_repository = TrainingRepositoryImpl::new(&config.db_url, &config.db_token)
        .await
        .expect("Error creating the training repository");

    let tuition_repository = TuitionRepositoryImpl::new(&config.db_url, &config.db_token)
        .await
        .expect("Error creating the tuition repository");

    let unique_identifier = build_unique_identifier(unique_identifier_repository);

    let services: Vec<Box<dyn HttpService>> = vec![
        Box::new(
            UserHttpServer::new(
                config.token_key.clone(),
                unique_identifier.clone(),
                user_repository.clone(),
            )
            .await,
        ),
        Box::new(
            TournamentHttpServer::new(
                tournament_repository,
                unique_identifier.clone(),
                &config.token_key,
            )
            .await,
        ),
        Box::new(
            TrainingHttpServer::new(
                training_repository,
                unique_identifier.clone(),
                &config.token_key,
            )
            .await,
        ),
        Box::new(
            TuitionHttpServer::new(
                tuition_repository,
                unique_identifier.clone(),
                &config.token_key,
            )
            .await,
        ),
    ];

    match start_http_server(config.port, services).await {
        Ok(_) => info!("Http server started succesfully"),
        Err(err) => error!("Error starting http server: {err}"),
    };

    Ok(())
}
