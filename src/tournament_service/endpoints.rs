use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    middleware,
    routing::{delete, get, post},
    Extension, Router,
};
use tracing::error;

use std::sync::Arc;

use crate::{
    auth_middleware::auth_middleware, global_traits::HttpService,
    unique_identifier_service::usecases::UniqueIdentifier,
};

use super::{
    domain::{Tournament, UserTournamentInfo, UserTournamentRegistration},
    repository::TournamentRepository,
    use_cases::TournamentService,
};

pub struct TournamentHttpServer {
    tournament_repository: Arc<dyn TournamentRepository>,
    unique_identifier: Arc<dyn UniqueIdentifier>,
    token_key: String,
}

impl TournamentHttpServer {
    pub async fn new(
        tournament_repository: Arc<dyn TournamentRepository>,
        unique_identifier: Arc<dyn UniqueIdentifier>,
        token_key: &str,
    ) -> Self {
        Self {
            tournament_repository,
            unique_identifier,
            token_key: token_key.to_string(),
        }
    }
}

impl HttpService for TournamentHttpServer {
    fn get_router(&self) -> axum::Router {
        let tournament_service = TournamentService::new(
            self.tournament_repository.clone(),
            self.unique_identifier.clone(),
        );

        Router::new()
            .route("/tournament", get(get_tournament_by_user_with_extension))
            .route(
                "/tournament/positions/{tournament_id}",
                get(get_tournament_positions),
            )
            .layer(middleware::from_fn_with_state(
                self.token_key.clone(),
                auth_middleware,
            ))
            .route(
                "/tournament/delete/{tournament_id}",
                delete(delete_tournament),
            )
            .route(
                "/tournament/name/{tournament_name}",
                post(create_tournament),
            )
            .route("/tournament/register", post(register_user_in_tournament))
            .route("/tournament/all", get(get_all_tournaments))
            .route("/tournament/{identificator}", get(get_tournament_by_user))
            .route(
                "/tournament/users/{id_tournament}",
                post(get_users_in_tournament),
            )
            .with_state(tournament_service)
    }
}

async fn get_tournament_positions(
    State(state): State<TournamentService>,
    Path(tournament_id): Path<String>,
) -> Result<Json<Vec<u32>>, StatusCode> {
    match state.get_tournament_positions(&tournament_id).await {
        Err(err) => {
            error!("Error getting tournament positions: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(tournament_positions) => Ok(Json(tournament_positions)),
    }
}

async fn delete_tournament(
    State(state): State<TournamentService>,
    Path(tournament_id): Path<String>,
) -> StatusCode {
    match state.delete_tournament(&tournament_id).await {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            error!("Error creating tournament: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn create_tournament(
    State(state): State<TournamentService>,
    Path(tournament_name): Path<String>,
) -> StatusCode {
    match state.create_tournament(tournament_name).await {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            error!("Error creating tournament: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn get_tournament_by_user_with_extension(
    State(state): State<TournamentService>,
    Extension(user_id): Extension<String>,
) -> Result<Json<Vec<UserTournamentInfo>>, StatusCode> {
    match state.get_tournaments_by_identificator(user_id).await {
        Err(err) => {
            error!("Error getting tournament by user: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(user_tournaments_info) => Ok(Json(user_tournaments_info)),
    }
}

async fn get_tournament_by_user(
    State(state): State<TournamentService>,
    Path(identificator): Path<String>,
) -> Result<Json<Vec<UserTournamentInfo>>, StatusCode> {
    match state.get_tournaments_by_identificator(identificator).await {
        Err(err) => {
            error!("Error getting tournament by user: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Ok(user_tournaments_info) => Ok(Json(user_tournaments_info)),
    }
}

async fn register_user_in_tournament(
    State(state): State<TournamentService>,
    Json(registration): Json<UserTournamentRegistration>,
) -> StatusCode {
    match state
        .register_user_in_tournament(
            registration.id_persona,
            registration.id_torneo,
            registration.puesto,
        )
        .await
    {
        Ok(_) => StatusCode::CREATED,
        Err(err) => {
            error!("Error registering user in tournament: {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

async fn get_all_tournaments(
    State(state): State<TournamentService>,
) -> Result<Json<Vec<Tournament>>, StatusCode> {
    match state.get_all_tournaments().await {
        Ok(tournaments) => Ok(Json(tournaments)),
        Err(err) => {
            error!("Error fetching all tournaments: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_users_in_tournament(
    State(state): State<TournamentService>,
    Path(id_torneo): Path<String>,
) -> Result<Json<Vec<UserTournamentRegistration>>, StatusCode> {
    match state.get_users_in_tournament(id_torneo).await {
        Ok(users) => Ok(Json(users)),
        Err(err) => {
            error!("Error fetching users in tournament: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
