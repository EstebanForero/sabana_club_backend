pub mod err;
use std::sync::Arc;

use super::domain::{Tournament, UserTournamentInfo, UserTournamentRegistration};
use async_trait::async_trait;
use err::Result;
use mockall::automock;
pub mod lib_sql_implementation;

#[automock]
#[async_trait]
pub trait TournamentRepository: Send + Sync {
    async fn create_tournament(&self, tournament: Tournament) -> Result<()>;

    async fn register_user_in_tournament(
        &self,
        registration: UserTournamentRegistration,
    ) -> Result<()>;

    async fn get_all_tournaments(&self) -> Result<Vec<Tournament>>;

    async fn get_users_in_tournament(
        &self,
        id_torneo: &str,
    ) -> Result<Vec<UserTournamentRegistration>>;

    async fn get_tournaments_info_for_user(&self, user_id: &str)
        -> Result<Vec<UserTournamentInfo>>;

    async fn delete_tournament(&self, tournament_id: &str) -> Result<()>;

    async fn get_tournament_positions(&self, tournament_id: &str) -> Result<Vec<u32>>;
}
