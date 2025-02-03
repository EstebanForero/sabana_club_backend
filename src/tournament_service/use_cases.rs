use std::sync::Arc;

use uuid::Uuid;

use crate::unique_identifier_service::usecases::UniqueIdentifier;

use super::domain::{UserTournamentInfo, UserTournamentRegistration};
use super::err::Result;
use super::{domain::Tournament, repository::TournamentRepository};

#[derive(Clone)]
pub struct TournamentService {
    tournament_repository: Arc<dyn TournamentRepository>,
    unique_identifier: Arc<dyn UniqueIdentifier>,
}

impl TournamentService {
    pub fn new(
        tournament_repository: Arc<dyn TournamentRepository>,
        unique_identifier: Arc<dyn UniqueIdentifier>,
    ) -> Self {
        Self {
            tournament_repository,
            unique_identifier,
        }
    }

    pub async fn get_tournament(&self, tournament_id: &str) -> Result<Tournament> {
        Ok(self
            .tournament_repository
            .get_tournament(tournament_id)
            .await?)
    }

    pub async fn get_tournament_positions(&self, tournament_id: &str) -> Result<Vec<u32>> {
        let positions = self
            .tournament_repository
            .get_tournament_positions(tournament_id)
            .await?;

        Ok(positions)
    }

    pub async fn delete_tournament(&self, tournament_id: &str) -> Result<()> {
        self.tournament_repository
            .delete_tournament(tournament_id)
            .await?;

        Ok(())
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

    pub async fn get_tournaments_by_identificator(
        &self,
        identificator: String,
    ) -> Result<Vec<UserTournamentInfo>> {
        let user_id = self.unique_identifier.identify(identificator.clone()).await;

        let user_id = match user_id {
            Some(user_id) => user_id,
            None => {
                return Err(super::err::TournamentServiceError::UserNotIdentifiable(
                    identificator,
                ))
            }
        };

        let user_tournaments_info = self
            .tournament_repository
            .get_tournaments_info_for_user(&user_id)
            .await?;

        Ok(user_tournaments_info)
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
