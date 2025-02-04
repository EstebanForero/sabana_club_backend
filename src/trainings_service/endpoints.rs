use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    auth_middleware::auth_middleware, global_traits::HttpService,
    unique_identifier_service::usecases::UniqueIdentifier,
};

use super::{
    model::{Training, TrainingRegistration},
    repository::TrainingRepository,
    use_cases::TrainingService,
};

pub struct TrainingHttpServer {
    training_service: Arc<TrainingService>,
    token_key: String,
}

impl TrainingHttpServer {
    pub async fn new(
        training_repository: Arc<dyn TrainingRepository>,
        unique_identifier: Arc<dyn UniqueIdentifier>,
        token_key: &str,
    ) -> Self {
        let training_service = TrainingService::new(training_repository, unique_identifier.clone());
        Self {
            training_service: Arc::new(training_service),
            token_key: token_key.to_string(),
        }
    }
}

#[async_trait]
impl HttpService for TrainingHttpServer {
    fn get_router(&self) -> Router {
        Router::new()
            .route("/training", get(get_trainings_for_user_with_extension))
            .route(
                "/training/delete/{id_entrenamiento}",
                delete(delete_training),
            )
            .route("/training/id/{id_entrenamiento}", get(get_training))
            .layer(middleware::from_fn_with_state(
                self.token_key.clone(),
                auth_middleware,
            ))
            .route("/training", post(create_training))
            .route("/training/register", post(register_user_in_training))
            .route("/training/all", get(get_all_trainings))
            .route(
                "/training/users/{id_entrenamiento}",
                get(get_users_in_training),
            )
            .route("/training/{user_identifier}", get(get_trainings_for_user))
            .with_state(self.training_service.clone())
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct TrainingInfo {
    nombre_entrenamiento: String,
    tiempo_minutos: i32,
}

async fn get_training(
    State(state): State<Arc<TrainingService>>,
    Path(training_id): Path<String>,
) -> Result<Json<Training>, StatusCode> {
    match state.get_training(&training_id).await {
        Err(err) => {
            error!("Error getting trainings for id: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(user_trainings) => Ok(Json(user_trainings)),
    }
}

async fn delete_training(
    State(state): State<Arc<TrainingService>>,
    Path(training_id): Path<String>,
) -> StatusCode {
    match state.delete_training(&training_id).await {
        Err(err) => {
            error!("Error deleting training: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
        Ok(_) => StatusCode::OK,
    }
}
async fn get_trainings_for_user_with_extension(
    State(state): State<Arc<TrainingService>>,
    Extension(user_id): Extension<String>,
) -> Result<Json<Vec<Training>>, StatusCode> {
    match state.get_trainings_for_user(user_id).await {
        Err(err) => {
            error!("Error getting trainings for user: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(user_trainings) => Ok(Json(user_trainings)),
    }
}

async fn get_trainings_for_user(
    State(state): State<Arc<TrainingService>>,
    Path(user_identification): Path<String>,
) -> Result<Json<Vec<Training>>, StatusCode> {
    match state.get_trainings_for_user(user_identification).await {
        Err(err) => {
            error!("Error getting trainings for user: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(user_trainings) => Ok(Json(user_trainings)),
    }
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
