use std::sync::Arc;

use uuid::Uuid;

use super::domain::UserTournamentRegistration;
use super::err::Result;
use super::{domain::Tournament, repository::TournamentRepository};

#[derive(Clone)]
pub struct TournamentService {
    tournament_repository: Arc<dyn TournamentRepository>,
}

impl TournamentService {
    pub fn new(tournament_repository: Arc<dyn TournamentRepository>) -> Self {
        Self {
            tournament_repository,
        }
    }

    pub async fn create_tournament(&self, nombre: String) -> Result<()> {
        let tournament_id = Uuid::new_v4().to_string();

        let tournament = Tournament {
            id_torneo: tournament_id,
            nombre,
        };
        self.tournament_repository
            .create_tournament(tournament)
            .await?;

        Ok(())
    }

    pub async fn register_user_in_tournament(
        &self,
        id_persona: String,
        id_torneo: String,
        puesto: i32,
    ) -> Result<()> {
        let registration = UserTournamentRegistration {
            id_persona,
            id_torneo,
            puesto,
        };
        self.tournament_repository
            .register_user_in_tournament(registration)
            .await?;

        Ok(())
    }

    pub async fn get_all_tournaments(&self) -> Result<Vec<Tournament>> {
        Ok(self.tournament_repository.get_all_tournaments().await?)
    }

    pub async fn get_users_in_tournament(
        &self,
        id_torneo: String,
    ) -> Result<Vec<UserTournamentRegistration>> {
        Ok(self
            .tournament_repository
            .get_users_in_tournament(&id_torneo)
            .await?)
    }
}
