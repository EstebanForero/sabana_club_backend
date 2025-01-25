use axum::{http::StatusCode, routing::post, Json, Router};

use crate::global_traits::HttpService;

use super::domain::UserCreationInfo;

pub struct UserHttpServer {}

impl HttpService for UserHttpServer {
    fn get_router(&self) -> axum::Router {
        Router::new().route("/user", post(create_user))
    }
}

async fn create_user(Json(user_creation_info): Json<UserCreationInfo>) -> StatusCode {
    StatusCode::OK
}
