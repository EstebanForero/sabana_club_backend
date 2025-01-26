use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    routing::{get, post},
    Router,
};
use tracing::error;

use std::sync::Arc;

use crate::global_traits::HttpService;

use super::{
    domain::{Tournament, UserTournamentRegistration},
    repository::{lib_sql_implementation::TournamentRepositoryImpl, TournamentRepository},
    use_cases::TournamentService,
};

pub struct TournamentHttpServer {
    tournament_repository: Arc<dyn TournamentRepository>,
}

impl TournamentHttpServer {
    pub async fn new(db_url: &str, db_token: &str) -> Self {
        let tournament_repository =
            TournamentRepositoryImpl::new(db_url.to_string(), db_token.to_string())
                .await
                .expect("Failed to connect to database");

        Self {
            tournament_repository: Arc::new(tournament_repository),
        }
    }
}

impl HttpService for TournamentHttpServer {
    fn get_router(&self) -> axum::Router {
        let tournament_service = TournamentService::new(self.tournament_repository.clone());

        Router::new()
            .route("/tournament/{tournament_name}", post(create_tournament))
            .route("/tournament/register", post(register_user_in_tournament))
            .route("/tournament/all", get(get_all_tournaments))
            .route("/tournament/users", post(get_users_in_tournament))
            .with_state(tournament_service)
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
    Json(id_torneo): Json<String>,
) -> Result<Json<Vec<UserTournamentRegistration>>, StatusCode> {
    match state.get_users_in_tournament(id_torneo).await {
        Ok(users) => Ok(Json(users)),
        Err(err) => {
            error!("Error fetching users in tournament: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
