use crate::global_traits::HttpService;
use async_trait::async_trait;
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tracing::error;

use super::{
    domain::{Tuition, TuitionInfo},
    repository::lib_sql_implementation::TuitionRepositoryImpl,
    use_cases::TuitionService,
};

pub struct TuitionHttpServer {
    tuition_service: TuitionService,
}

impl TuitionHttpServer {
    pub async fn new(db_url: &str, db_token: &str) -> Self {
        let tuition_repository =
            TuitionRepositoryImpl::new(db_url.to_string(), db_token.to_string())
                .await
                .expect("Failed to initialize TuitionRepository");
        let tuition_service = TuitionService::new(Arc::new(tuition_repository));
        Self { tuition_service }
    }
}

#[async_trait]
impl HttpService for TuitionHttpServer {
    fn get_router(&self) -> Router {
        Router::new()
            .route("/tuition", post(create_tuition))
            .route("/tuition/user/:id_persona", get(get_tuitions_for_user))
            .route(
                "/tuition/user/:id_persona/recent",
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

async fn get_tuitions_for_user(
    State(state): State<TuitionService>,
    Path(id_persona): Path<String>,
) -> Result<Json<Vec<Tuition>>, StatusCode> {
    match state.get_tuitions_for_user(id_persona).await {
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
