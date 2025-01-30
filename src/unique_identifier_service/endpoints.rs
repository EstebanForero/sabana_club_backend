use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};

use crate::global_traits::HttpService;

use super::{
    repository::UniqueIdentifierRepository,
    usecases::{EMailIdentifier, PhoneIdentifier, UniqueIdentifier},
};

pub struct UniqueIdentifierHttpServer {
    unique_identifier_repository: Arc<dyn UniqueIdentifierRepository>,
}

impl UniqueIdentifierHttpServer {
    pub async fn new(unique_identifier_repository: Arc<dyn UniqueIdentifierRepository>) -> Self {
        Self {
            unique_identifier_repository,
        }
    }
}

#[derive(Clone)]
struct ServiceState {
    phone_identifier: Arc<PhoneIdentifier>,
    email_identifier: Arc<EMailIdentifier>,
}

impl HttpService for UniqueIdentifierHttpServer {
    fn get_router(&self) -> axum::Router {
        let state: ServiceState = ServiceState {
            phone_identifier: Arc::new(PhoneIdentifier::new(
                self.unique_identifier_repository.clone(),
                None,
            )),
            email_identifier: Arc::new(EMailIdentifier::new(
                self.unique_identifier_repository.clone(),
                None,
            )),
        };

        Router::new()
            .route("/check_email/{email}", get(exists_email))
            .route("/check_phone/{phone}", get(exists_phone))
            .with_state(state.into())
    }
}

async fn exists_email(
    Path(email): Path<String>,
    State(state): State<Arc<ServiceState>>,
) -> Result<Json<bool>, StatusCode> {
    let exists = state.email_identifier.identify(email).await.is_some();
    Ok(Json(exists))
}

async fn exists_phone(
    Path(phone): Path<String>,
    State(state): State<Arc<ServiceState>>,
) -> Result<Json<bool>, StatusCode> {
    let exists = state.phone_identifier.identify(phone).await.is_some();
    Ok(Json(exists))
}
