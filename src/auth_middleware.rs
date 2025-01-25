use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use tracing::{error, info};

use crate::user_service::token_provider::TokenProvider;

#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub client_name: String,
}

pub async fn auth_middleware(
    State(jwt_secret): State<String>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let token_provider = TokenProvider::new(jwt_secret);

    let authorization_header = request
        .headers()
        .get("Authorization")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let auth_str = authorization_header.to_str().map_err(|err| {
        error!("Error converting auth header to string: {}", err);
        StatusCode::BAD_REQUEST
    })?;

    let jwt_token_string = auth_str
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    info!("The token is: |{}|", jwt_token_string);

    let claims = token_provider
        .verify_token(jwt_token_string)
        .map_err(|err| {
            error!("Error in the token verification process: {}", err);
            StatusCode::UNAUTHORIZED
        })?;

    let client_name = claims.claims.sub;

    request.extensions_mut().insert(ClientInfo { client_name });

    let response = next.run(request).await;

    Ok(response)
}
