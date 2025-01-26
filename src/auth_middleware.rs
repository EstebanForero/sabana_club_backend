use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::CookieJar;
use tracing::{error, info};

use crate::user_service::token_provider::TokenProvider;

#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub client_id: String,
}

pub async fn auth_middleware(
    State(jwt_secret): State<String>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, StatusCode> {
    let token_provider = TokenProvider::new(jwt_secret);

    let cookie = jar.get("auth_token").ok_or(StatusCode::UNAUTHORIZED)?;

    let jwt_token_string = cookie.value();

    info!("The token is: |{jwt_token_string}|");

    let claims = token_provider
        .verify_token(jwt_token_string)
        .map_err(|err| {
            error!("Error in the token verification process: {err}");
            StatusCode::UNAUTHORIZED
        })?;

    let client_id = claims.claims.sub;

    request.extensions_mut().insert(client_id);

    let response = next.run(request).await;

    Ok(response)
}
