use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use tracing::{error, info};

use crate::{
    auth_middleware::auth_middleware,
    global_traits::HttpService,
    tournament_service::{repository::TournamentRepository, use_cases::TournamentService},
    trainings_service::{
        repository::{lib_sql_implementation::TrainingRepositoryImpl, TrainingRepository},
        use_cases::TrainingService,
    },
    unique_identifier_service::usecases::UniqueIdentifier,
    user_service::{
        repository::UserRepository, token_provider::TokenProvider, use_cases::UserService,
    },
};

use super::{
    domain::{CommandExecutor, RequestContent, RequestCreation, RequestForApproval},
    repository::RequestRepository,
    usecases::RequestService,
};

pub struct RequestHttpServer {
    user_repository: Arc<dyn UserRepository>,
    tournament_repository: Arc<dyn TournamentRepository>,
    unique_identifier: Arc<dyn UniqueIdentifier>,
    training_repository: Arc<dyn TrainingRepository>,
    request_repository: Arc<dyn RequestRepository>,
    token_key: String,
}

impl RequestHttpServer {
    pub async fn new(
        user_repository: Arc<dyn UserRepository>,
        tournament_repository: Arc<dyn TournamentRepository>,
        request_repository: Arc<dyn RequestRepository>,
        training_repository: Arc<dyn TrainingRepository>,
        unique_identifier: Arc<dyn UniqueIdentifier>,
        token_key: String,
    ) -> Self {
        Self {
            user_repository,
            tournament_repository,
            unique_identifier,
            token_key,
            request_repository,
            training_repository,
        }
    }
}

impl HttpService for RequestHttpServer {
    fn get_router(&self) -> axum::Router {
        let token_provider = TokenProvider::new(self.token_key.clone());

        let user_service = UserService::new(
            self.user_repository.clone(),
            self.unique_identifier.clone(),
            token_provider,
        );

        let tournament_service = TournamentService::new(
            self.tournament_repository.clone(),
            self.unique_identifier.clone(),
        );

        let training_service = TrainingService::new(
            self.training_repository.clone(),
            self.unique_identifier.clone(),
        );

        let command_executor = CommandExecutor {
            user_service,
            tournament_service,
            training_service,
        };

        let request_service =
            RequestService::new(command_executor, self.request_repository.clone());

        Router::new()
            .route("/request/name/{name}", get(get_requests_by_name))
            .route("/request/id/{request_id}", get(get_request_by_id))
            .route("/request", post(create_request))
            .route("/request/execute/{request_id}", post(execute_request))
            .route("/request/all", get(get_all_requests))
            .route("/request/{request_id}", delete(delete_request))
            .layer(middleware::from_fn_with_state(
                self.token_key.clone(),
                auth_middleware,
            ))
            .with_state(request_service)
    }
}

async fn get_all_requests(
    State(request_service): State<RequestService>,
) -> Result<Json<Vec<RequestForApproval>>, StatusCode> {
    match request_service.get_all_requests().await {
        Err(err) => {
            error!("Error getting request by name: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(requests) => Ok(Json(requests)),
    }
}

async fn delete_request(
    State(request_service): State<RequestService>,
    Path(request_id): Path<String>,
) -> StatusCode {
    match request_service.delete_request(request_id).await {
        Err(err) => {
            error!("Error executing request: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
        Ok(_) => StatusCode::OK,
    }
}

async fn execute_request(
    State(request_service): State<RequestService>,
    Extension(aprover_id): Extension<String>,
    Path(request_id): Path<String>,
) -> StatusCode {
    match request_service
        .execute_request(request_id, &aprover_id)
        .await
    {
        Err(err) => {
            error!("Error executing request: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
        Ok(_) => StatusCode::OK,
    }
}

async fn get_requests_by_name(
    State(request_service): State<RequestService>,
    Path(request_name): Path<String>,
) -> Result<Json<Vec<RequestForApproval>>, StatusCode> {
    match request_service.get_requests_by_name(request_name).await {
        Err(err) => {
            error!("Error getting request by name: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(requests) => Ok(Json(requests)),
    }
}

async fn get_request_by_id(
    State(request_service): State<RequestService>,
    Path(request_id): Path<String>,
) -> Result<Json<RequestForApproval>, StatusCode> {
    match request_service.get_request_by_id(request_id).await {
        Err(err) => {
            error!("Error getting request by name: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(requests) => Ok(Json(requests)),
    }
}

async fn create_request(
    State(request_service): State<RequestService>,
    Extension(user_id): Extension<String>,
    Json(request_creation): Json<RequestContent>,
) -> StatusCode {
    match request_service
        .create_request(request_creation, user_id)
        .await
    {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            error!("Error creating request: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
