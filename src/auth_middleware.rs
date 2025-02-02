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
    pub client_id: String,
}

pub async fn auth_middleware(
    State(jwt_secret): State<String>,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let token_provider = TokenProvider::new(jwt_secret);

    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let jwt_token_string = match auth_header {
        Some(header_value) if header_value.starts_with("Bearer ") => {
            &header_value[7..] // Remove "Bearer " prefix
        }
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    info!("The token is: |{jwt_token_string}|");

    let claims = token_provider
        .verify_token(jwt_token_string)
        .map_err(|err| {
            error!("Error in token verification: {err}");
            StatusCode::UNAUTHORIZED
        })?;

    let client_id = claims.claims.sub;
    request.extensions_mut().insert(client_id);

    let response = next.run(request).await;

    Ok(response)
}
