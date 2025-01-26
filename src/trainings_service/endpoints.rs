use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::global_traits::HttpService;

use super::{
    model::{Training, TrainingRegistration},
    repository::lib_sql_implementation::TrainingRepositoryImpl,
    use_cases::TrainingService,
};

pub struct TrainingHttpServer {
    training_service: Arc<TrainingService>,
}

impl TrainingHttpServer {
    pub async fn new(db_url: &str, db_token: &str) -> Self {
        let training_repository =
            TrainingRepositoryImpl::new(db_url.to_string(), db_token.to_string())
                .await
                .expect("Failed to initialize TrainingRepository");
        let training_service = TrainingService::new(Arc::new(training_repository));
        Self {
            training_service: Arc::new(training_service),
        }
    }
}

#[async_trait]
impl HttpService for TrainingHttpServer {
    fn get_router(&self) -> Router {
        Router::new()
            .route("/training", post(create_training))
            .route("/training/register", post(register_user_in_training))
            .route("/training/all", get(get_all_trainings))
            .route(
                "/training/users/{id_entrenamiento}",
                get(get_users_in_training),
            )
            .with_state(self.training_service.clone())
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct TrainingInfo {
    nombre_entrenamiento: String,
    tiempo_minutos: i32,
}

async fn create_training(
    State(state): State<Arc<TrainingService>>,
    Json(training_info): Json<TrainingInfo>,
) -> Result<Json<String>, StatusCode> {
    state
        .create_training(
            training_info.nombre_entrenamiento,
            training_info.tiempo_minutos,
        )
        .await
        .map(Json)
        .map_err(|err| {
            error!("Error creating training: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

async fn register_user_in_training(
    State(state): State<Arc<TrainingService>>,
    Json(registration): Json<TrainingRegistration>,
) -> StatusCode {
    match state
        .register_user_in_training(registration.id_entrenamiento, registration.id_persona)
        .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            error!("Error registering user in training: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn get_all_trainings(
    State(state): State<Arc<TrainingService>>,
) -> Result<Json<Vec<Training>>, StatusCode> {
    state.get_all_trainings().await.map(Json).map_err(|err| {
        error!("Error fetching all trainings: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

async fn get_users_in_training(
    State(state): State<Arc<TrainingService>>,
    Path(id_entrenamiento): Path<String>,
) -> Result<Json<Vec<TrainingRegistration>>, StatusCode> {
    state
        .get_users_in_training(id_entrenamiento)
        .await
        .map(Json)
        .map_err(|err| {
            error!("Error fetching users in training: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        })
}
