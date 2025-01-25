use std::sync::Arc;

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};

use crate::global_traits::HttpService;

use super::repository::libsql_implementation::Repository;
use super::token_provider::TokenProvider;
use super::{domain::UserCreationInfo, repository::UserRepository, use_cases::UserService};

use super::unique_identifier::{EMailIdentifier, PhoneIdentifier, UniqueIdentifier};

pub struct UserHttpServer {
    user_repository: Arc<dyn UserRepository>,
    token_key: String,
}

impl UserHttpServer {
    pub async fn new(db_url: String, db_token: String, token_key: String) -> Self {
        let user_repository = Repository::new(db_url, db_token)
            .await
            .expect("Error creating the user repository");

        Self {
            user_repository: Arc::new(user_repository),
            token_key,
        }
    }
}

impl HttpService for UserHttpServer {
    fn get_router(&self) -> axum::Router {
        let email_unique_identifier: Arc<dyn UniqueIdentifier> =
            Arc::new(EMailIdentifier::new(self.user_repository.clone(), None));
        let phone_unique_identifier: Arc<dyn UniqueIdentifier> = Arc::new(PhoneIdentifier::new(
            self.user_repository.clone(),
            Some(email_unique_identifier),
        ));

        let token_provider = TokenProvider::new(self.token_key.clone());

        let user_service = UserService::new(
            self.user_repository.clone(),
            phone_unique_identifier,
            token_provider,
        );

        Router::new()
            .route("/user", post(create_user))
            .with_state(user_service)
    }
}

async fn create_user(
    State(state): State<UserService>,
    Json(user_creation_info): Json<UserCreationInfo>,
) -> StatusCode {
    match state.create_user(user_creation_info).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
