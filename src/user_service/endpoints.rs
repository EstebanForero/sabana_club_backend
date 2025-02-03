use std::sync::Arc;

use axum::extract::Path;
use axum::routing::{get, put};
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use axum::{middleware, Extension};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::auth_middleware::auth_middleware;
use crate::global_traits::HttpService;
use crate::unique_identifier_service::usecases::UniqueIdentifier;

use super::domain::{SearchSelection, UserInfo, UserRol, UserSelectionInfo, UserUpdating};
use super::repository::UserRepository;
use super::token_provider::TokenProvider;
use super::{domain::UserCreationInfo, use_cases::UserService};

pub struct UserHttpServer {
    user_repository: Arc<dyn UserRepository>,
    token_key: String,
    unique_identifier: Arc<dyn UniqueIdentifier>,
}

impl UserHttpServer {
    pub async fn new(
        token_key: String,
        unique_identifier: Arc<dyn UniqueIdentifier>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            user_repository,
            token_key,
            unique_identifier,
        }
    }
}

impl HttpService for UserHttpServer {
    fn get_router(&self) -> axum::Router {
        let token_provider = TokenProvider::new(self.token_key.clone());

        let user_service = UserService::new(
            self.user_repository.clone(),
            self.unique_identifier.clone(),
            token_provider,
        );

        Router::new()
            .route("/test_auth", get(test_auth))
            .route("/user", get(get_user))
            .route("/user/admin", get(user_rol))
            .route("/user", put(update_user))
            .route("/user/id/{user_id}", put(update_other_user))
            .layer(middleware::from_fn_with_state(
                self.token_key.clone(),
                auth_middleware,
            ))
            .route(
                "/user/search/{query}/{selection}/{limt}",
                get(search_user_selection_info),
            )
            .route("/user", post(create_user))
            .route("/user/{identification}", get(get_user_by_identification))
            .route("/user/all", get(get_all_users))
            .route("/log_in", post(login_user))
            .with_state(user_service)
    }
}

async fn test_auth() -> &'static str {
    "Test for auth"
}

async fn user_rol(
    State(service): State<UserService>,
    Extension(user_id): Extension<String>,
) -> Result<Json<UserRol>, StatusCode> {
    match service.user_rol(&user_id).await {
        Err(err) => {
            error!("Error in is admin: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(result) => Ok(Json(result)),
    }
}

async fn update_other_user(
    State(service): State<UserService>,
    Path(user_id): Path<String>,
    Json(user_updation_info): Json<UserUpdating>,
) -> StatusCode {
    match service.update_user(user_updation_info, &user_id).await {
        Err(err) => {
            error!("Error in update user: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
        Ok(_) => StatusCode::OK,
    }
}

async fn update_user(
    State(service): State<UserService>,
    Extension(user_id): Extension<String>,
    Json(user_updation_info): Json<UserUpdating>,
) -> StatusCode {
    match service.update_user(user_updation_info, &user_id).await {
        Err(err) => {
            error!("Error in update user: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
        Ok(_) => StatusCode::OK,
    }
}

async fn search_user_selection_info(
    State(state): State<UserService>,
    Path(query_info): Path<(String, SearchSelection, u8)>,
) -> Result<Json<Vec<UserSelectionInfo>>, StatusCode> {
    match state
        .search_user_by_search_selection(&query_info.0, query_info.2, query_info.1)
        .await
    {
        Ok(users_selection) => Ok(Json(users_selection)),
        Err(err) => {
            error!("Error searching user selection info: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_all_users(
    State(user_service): State<UserService>,
) -> Result<Json<Vec<UserInfo>>, StatusCode> {
    match user_service.get_users().await {
        Ok(user_info) => Ok(Json(user_info)),
        Err(err) => {
            error!("Error fetching user by identification: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_user(
    State(user_service): State<UserService>,
    Extension(user_id): Extension<String>,
) -> Result<Json<UserInfo>, StatusCode> {
    match user_service.get_user_by_identification(user_id).await {
        Ok(user_info) => Ok(Json(user_info)),
        Err(err) => {
            error!("Error fetching user by identification: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_user_by_identification(
    State(user_service): State<UserService>,
    Path(identification): Path<String>,
) -> Result<Json<UserInfo>, StatusCode> {
    match user_service
        .get_user_by_identification(identification)
        .await
    {
        Ok(user_info) => Ok(Json(user_info)),
        Err(err) => {
            error!("Error fetching user by identification: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// async fn get_user_by_identification(
//     State(user_service): State<UserService>,
//     Path(identification): Path<String>,
// ) -> Result<Json<UserInfo>, StatusCode> {
//     match user_service
//         .get_user_by_identification(identification)
//         .await
//     {
//         Ok(user_info) => Ok(Json(user_info)),
//         Err(err) => {
//             error!("Error fetching user by identification: {err}");
//             Err(StatusCode::INTERNAL_SERVER_ERROR)
//         }
//     }
// }

async fn create_user(
    State(state): State<UserService>,
    Json(user_creation_info): Json<UserCreationInfo>,
) -> StatusCode {
    match state.create_user_with_hashing(user_creation_info).await {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            error!("Error creating user: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AuthInfo {
    identificacion: String,
    contrasena: String,
}

pub async fn login_user(
    State(service): State<UserService>,
    Json(payload): Json<AuthInfo>,
) -> Result<String, StatusCode> {
    match service
        .authenticate_user(payload.identificacion, payload.contrasena)
        .await
    {
        Ok(token) => Ok(token),
        Err(err) => {
            error!("Error authenticating user: {err}");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
