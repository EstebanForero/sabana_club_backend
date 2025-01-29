use crate::{
    auth_middleware::auth_middleware, global_traits::HttpService,
    unique_identifier_service::usecases::UniqueIdentifier,
};
use async_trait::async_trait;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    middleware,
    routing::{get, post},
    Extension, Router,
};
use std::sync::Arc;
use tracing::error;

use super::{
    domain::{Tuition, TuitionInfo},
    repository::TuitionRepository,
    use_cases::TuitionService,
};

pub struct TuitionHttpServer {
    tuition_service: TuitionService,
    token_key: String,
}

impl TuitionHttpServer {
    pub async fn new(
        tuition_repository: Arc<dyn TuitionRepository>,
        unique_identifier: Arc<dyn UniqueIdentifier>,
        token_key: &str,
    ) -> Self {
        let tuition_service = TuitionService::new(tuition_repository, unique_identifier.clone());
        Self {
            tuition_service,
            token_key: token_key.to_string(),
        }
    }
}

#[async_trait]
impl HttpService for TuitionHttpServer {
    fn get_router(&self) -> Router {
        Router::new()
            .route("/tuition", get(get_tuitions_for_user_with_extension))
            .layer(middleware::from_fn_with_state(
                self.token_key.clone(),
                auth_middleware,
            ))
            .route("/tuition", post(create_tuition))
            .route(
                "/tuition/user/{user_identifier}",
                get(get_tuitions_for_user),
            )
            .route(
                "/tuition/user/{id_persona}/recent",
                get(get_most_recent_tuition),
            )
            .with_state(self.tuition_service.clone())
    }
}

async fn create_tuition(
    State(state): State<TuitionService>,
    Json(payload): Json<TuitionInfo>,
) -> StatusCode {
    match state
        .create_tuition(payload.id_persona, payload.monto_usd)
        .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            error!("Error creating tuition: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn get_tuitions_for_user_with_extension(
    State(state): State<TuitionService>,
    Extension(user_id): Extension<String>,
) -> Result<Json<Vec<Tuition>>, StatusCode> {
    match state.get_tuitions_for_user(user_id).await {
        Ok(tuitions) => Ok(Json(tuitions)),
        Err(err) => {
            error!("Error fetching tuitions for user: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_tuitions_for_user(
    State(state): State<TuitionService>,
    Path(user_identifier): Path<String>,
) -> Result<Json<Vec<Tuition>>, StatusCode> {
    match state.get_tuitions_for_user(user_identifier).await {
        Ok(tuitions) => Ok(Json(tuitions)),
        Err(err) => {
            error!("Error fetching tuitions for user: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_most_recent_tuition(
    State(state): State<TuitionService>,
    Path(id_persona): Path<String>,
) -> Result<Json<Tuition>, StatusCode> {
    match state.get_most_recent_tuition(id_persona).await {
        Ok(tuition) => Ok(Json(tuition)),
        Err(err) => {
            error!("Error fetching most recent tuition: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
