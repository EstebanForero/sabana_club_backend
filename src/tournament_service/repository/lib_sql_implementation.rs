use super::err::Result;
use std::sync::Arc;

use async_trait::async_trait;

use crate::tournament_service::domain::{Tournament, UserTournamentRegistration};

use super::{err::TournamentRepositoryError, TournamentRepository};

#[derive(Clone)]
pub struct TournamentRepositoryImpl {
    db: Arc<libsql::Database>,
}

impl TournamentRepositoryImpl {
    pub async fn new(url: String, token: String) -> std::result::Result<Self, String> {
        let db = libsql::Builder::new_remote(url, token)
            .build()
            .await
            .map_err(|err| format!("Error creating new remote database for libsql: {err}"))?;

        Ok(Self { db: Arc::new(db) })
    }

    async fn get_connection(&self) -> Result<libsql::Connection> {
        self.db
            .connect()
            .map_err(|err| TournamentRepositoryError::DatabaseError("Error connecting".to_string()))
    }
}

#[async_trait]
impl TournamentRepository for TournamentRepositoryImpl {
    async fn create_tournament(&self, tournament: Tournament) -> Result<()> {
        let conn = self.get_connection().await?;
        conn.execute(
            "INSERT INTO torneo (id_torneo, nombre) VALUES (?1, ?2)",
            libsql::params![tournament.id_torneo, tournament.nombre],
        )
        .await
        .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn register_user_in_tournament(
        &self,
        registration: UserTournamentRegistration,
    ) -> Result<()> {
        let conn = self.get_connection().await?;
        conn.execute(
            "INSERT INTO persona_torneo (id_persona, id_torneo, puesto) VALUES (?1, ?2, ?3)",
            libsql::params![
                registration.id_persona,
                registration.id_torneo,
                registration.puesto
            ],
        )
        .await
        .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_all_tournaments(&self) -> Result<Vec<Tournament>> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query("SELECT id_torneo, nombre FROM torneo", libsql::params![])
            .await
            .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?;

        let mut tournaments = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?
        {
            tournaments.push(Tournament {
                id_torneo: row
                    .get(0)
                    .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?,
                nombre: row
                    .get(1)
                    .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?,
            });
        }

        Ok(tournaments)
    }

    async fn get_users_in_tournament(
        &self,
        id_torneo: &String,
    ) -> Result<Vec<UserTournamentRegistration>> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query(
                "SELECT id_persona, id_torneo, puesto FROM persona_torneo WHERE id_torneo = ?1",
                libsql::params![id_torneo.to_string()],
            )
            .await
            .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?;

        let mut registrations = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?
        {
            registrations.push(UserTournamentRegistration {
                id_persona: row
                    .get(0)
                    .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?,
                id_torneo: row
                    .get(1)
                    .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?,
                puesto: row
                    .get(2)
                    .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?,
            });
        }

        Ok(registrations)
    }
}
