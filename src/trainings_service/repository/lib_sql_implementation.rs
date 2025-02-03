use std::sync::Arc;

use async_trait::async_trait;
use libsql::params;

use crate::trainings_service::model::{Training, TrainingRegistration};

use super::{err::Result, err::TrainingRepositoryError, TrainingRepository};

#[derive(Clone)]
pub struct TrainingRepositoryImpl {
    db: Arc<libsql::Database>,
}

impl TrainingRepositoryImpl {
    pub async fn new(db_url: &str, db_token: &str) -> Result<Arc<dyn TrainingRepository>> {
        let db = libsql::Builder::new_remote(db_url.to_string(), db_token.to_string())
            .build()
            .await
            .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?;

        Ok(Arc::new(Self { db: Arc::new(db) }))
    }

    async fn get_connection(&self) -> Result<libsql::Connection> {
        self.db
            .connect()
            .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))
    }
}

#[async_trait]
impl TrainingRepository for TrainingRepositoryImpl {
    async fn delete_training(&self, training_id: &str) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute(
            "DELETE FROM entrenamiento WHERE id_entrenamiento = ?1",
            params![training_id],
        )
        .await
        .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn create_training(&self, training: Training) -> Result<()> {
        let conn = self.get_connection().await?;
        conn.execute(
            "INSERT INTO entrenamiento (id_entrenamiento, tiempo_minutos, nombre_entrenamiento) VALUES (?1, ?2, ?3)",
            libsql::params![training.id_entrenamiento, training.tiempo_minutos, training.nombre_entrenamiento],
        )
        .await
        .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn register_user_in_training(&self, registration: TrainingRegistration) -> Result<()> {
        let conn = self.get_connection().await?;
        conn.execute(
            "INSERT INTO entrenamiento_persona (id_entrenamiento, id_persona) VALUES (?1, ?2)",
            libsql::params![registration.id_entrenamiento, registration.id_persona],
        )
        .await
        .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_all_trainings(&self) -> Result<Vec<Training>> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query(
                "SELECT id_entrenamiento, tiempo_minutos, nombre_entrenamiento FROM entrenamiento",
                libsql::params![],
            )
            .await
            .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?;

        let mut trainings = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?
        {
            trainings.push(Training {
                id_entrenamiento: row
                    .get(0)
                    .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?,
                tiempo_minutos: row
                    .get(1)
                    .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?,
                nombre_entrenamiento: row
                    .get(2)
                    .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?,
            });
        }

        Ok(trainings)
    }

    async fn get_users_in_training(
        &self,
        id_entrenamiento: &str,
    ) -> Result<Vec<TrainingRegistration>> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query(
                "SELECT id_entrenamiento, id_persona FROM entrenamiento_persona WHERE id_entrenamiento = ?1",
                libsql::params![id_entrenamiento],
            )
            .await
            .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?;

        let mut registrations = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?
        {
            registrations.push(TrainingRegistration {
                id_entrenamiento: row
                    .get(0)
                    .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?,
                id_persona: row
                    .get(1)
                    .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?,
            });
        }

        Ok(registrations)
    }

    async fn get_trainings_for_user(&self, user_id: &str) -> Result<Vec<Training>> {
        let conn = self.get_connection().await?;

        let mut rows = conn
            .query(
                "SELECT entrenamiento.id_entrenamiento, entrenamiento.nombre_entrenamiento, entrenamiento.tiempo_minutos
                 FROM entrenamiento
                 INNER JOIN entrenamiento_persona ON entrenamiento.id_entrenamiento = entrenamiento_persona.id_entrenamiento
                 WHERE entrenamiento_persona.id_persona = ?1",
                libsql::params![user_id],
            )
            .await
            .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?;

        let mut trainings = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?
        {
            trainings.push(Training {
                id_entrenamiento: row
                    .get(0)
                    .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?,
                nombre_entrenamiento: row
                    .get(1)
                    .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?,
                tiempo_minutos: row
                    .get(2)
                    .map_err(|e| TrainingRepositoryError::DatabaseError(e.to_string()))?,
            });
        }

        Ok(trainings)
    }
}
