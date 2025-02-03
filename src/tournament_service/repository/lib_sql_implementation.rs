use super::err::Result;
use std::sync::Arc;

use async_trait::async_trait;
use libsql::{de, params};

use crate::tournament_service::domain::{
    Tournament, UserTournamentInfo, UserTournamentRegistration,
};

use super::{err::TournamentRepositoryError, TournamentRepository};

#[derive(Clone)]
pub struct TournamentRepositoryImpl {
    db: Arc<libsql::Database>,
}

impl TournamentRepositoryImpl {
    pub async fn new(
        url: &str,
        token: &str,
    ) -> std::result::Result<Arc<dyn TournamentRepository>, String> {
        let db = libsql::Builder::new_remote(url.to_string(), token.to_string())
            .build()
            .await
            .map_err(|err| format!("Error creating new remote database for libsql: {err}"))?;

        Ok(Arc::new(Self { db: Arc::new(db) }))
    }

    async fn get_connection(&self) -> Result<libsql::Connection> {
        self.db
            .connect()
            .map_err(|err| TournamentRepositoryError::DatabaseError("Error connecting".to_string()))
    }
}

#[async_trait]
impl TournamentRepository for TournamentRepositoryImpl {
    async fn get_tournament(&self, tournament_id: &str) -> Result<Tournament> {
        let conn = self.get_connection().await?;

        let mut row = conn
            .query(
                "SELECT id_torneo, nombre FROM torneo WHERE id_torneo = ?1",
                params![tournament_id],
            )
            .await
            .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?;

        if let Some(row) = row
            .next()
            .await
            .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?
        {
            let tournament = de::from_row(&row)
                .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?;

            Ok(tournament)
        } else {
            return Err(TournamentRepositoryError::TournamentNotFound);
        }
    }

    async fn get_tournament_positions(&self, tournament_id: &str) -> Result<Vec<u32>> {
        let conn = self.get_connection().await?;

        let mut rows = conn
            .query(
                "SELECT puesto FROM persona_torneo WHERE id_torneo = ?1",
                params![tournament_id],
            )
            .await
            .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?;

        let mut puestos = Vec::new();

        while let Some(row) = rows
            .next()
            .await
            .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?
        {
            puestos.push(
                row.get(0)
                    .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?,
            );
        }

        Ok(puestos)
    }

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

    async fn delete_tournament(&self, tournament_id: &str) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute(
            "DELETE FROM torneo WHERE id_torneo = ?1",
            libsql::params![tournament_id],
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
        id_torneo: &str,
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

    async fn get_tournaments_info_for_user(
        &self,
        user_id: &str,
    ) -> Result<Vec<UserTournamentInfo>> {
        let conn = self.get_connection().await?;

        let mut rows = conn
            .query(
                "SELECT torneo.id_torneo, torneo.nombre, persona_torneo.puesto
                 FROM torneo
                 INNER JOIN persona_torneo ON torneo.id_torneo = persona_torneo.id_torneo
                 WHERE persona_torneo.id_persona = ?1",
                libsql::params![user_id],
            )
            .await
            .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?;

        let mut tournaments = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?
        {
            tournaments.push(UserTournamentInfo {
                id_torneo: row
                    .get(0)
                    .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?,
                nombre: row
                    .get(1)
                    .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?,
                puesto: row
                    .get(2)
                    .map_err(|e| TournamentRepositoryError::DatabaseError(e.to_string()))?,
            });
        }

        Ok(tournaments)
    }
}
