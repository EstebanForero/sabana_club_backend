use std::sync::Arc;

use async_trait::async_trait;
use chrono::NaiveDate;

use crate::tuition_service::domain::{Tuition, TuitionInfo};

use super::{
    err::{Result, TuitionRepositoryError},
    TuitionRepository,
};

#[derive(Clone)]
pub struct TuitionRepositoryImpl {
    db: Arc<libsql::Database>,
}

impl TuitionRepositoryImpl {
    pub async fn new(db_url: String, db_token: String) -> Result<Self> {
        let db = libsql::Builder::new_remote(db_url, db_token)
            .build()
            .await
            .map_err(|e| TuitionRepositoryError::DatabaseError(e.to_string()))?;

        Ok(Self { db: Arc::new(db) })
    }

    async fn get_connection(&self) -> Result<libsql::Connection> {
        self.db
            .connect()
            .map_err(|e| TuitionRepositoryError::DatabaseError(e.to_string()))
    }
}

#[async_trait]
impl TuitionRepository for TuitionRepositoryImpl {
    async fn create_tuition(&self, tuition: TuitionInfo) -> Result<()> {
        let conn = self.get_connection().await?;
        conn.execute(
            "INSERT INTO new_matricula (id_persona, monto_usd) VALUES (?1, ?2)",
            libsql::params![tuition.id_persona, tuition.monto_usd,],
        )
        .await
        .map_err(|e| TuitionRepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn get_tuitions_for_user(&self, id_persona: &String) -> Result<Vec<Tuition>> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query(
                "SELECT id_persona, monto_usd, fecha_inscripccion FROM new_matricula WHERE id_persona = ?1",
                libsql::params![id_persona.clone()],
            )
            .await
            .map_err(|e| TuitionRepositoryError::DatabaseError(e.to_string()))?;

        let mut tuitions = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .map_err(|e| TuitionRepositoryError::DatabaseError(e.to_string()))?
        {
            tuitions.push(Tuition {
                id_persona: row
                    .get(0)
                    .map_err(|e| TuitionRepositoryError::DatabaseError(e.to_string()))?,
                monto_usd: row
                    .get(1)
                    .map_err(|e| TuitionRepositoryError::DatabaseError(e.to_string()))?,
                fecha_inscripccion: row
                    .get(2)
                    .map_err(|e| TuitionRepositoryError::DatabaseError(e.to_string()))?,
            });
        }

        Ok(tuitions)
    }

    async fn get_most_recent_tuition(&self, id_persona: &String) -> Result<Tuition> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query(
                "SELECT id_persona, monto_usd, fecha_inscripccion FROM new_matricula WHERE id_persona = ?1",
                libsql::params![id_persona.clone()],
            )
            .await
            .map_err(|e| TuitionRepositoryError::DatabaseError(e.to_string()))?;

        let mut tuitions = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .map_err(|e| TuitionRepositoryError::DatabaseError(e.to_string()))?
        {
            let fecha_inscripccion_str: String = row
                .get(2)
                .map_err(|e| TuitionRepositoryError::DatabaseError(e.to_string()))?;
            let fecha_inscripccion = NaiveDate::parse_from_str(&fecha_inscripccion_str, "%Y-%m-%d")
                .map_err(|e| {
                    TuitionRepositoryError::DatabaseError(format!("Invalid date format: {}", e))
                })?;

            let tuition = Tuition {
                id_persona: row
                    .get(0)
                    .map_err(|e| TuitionRepositoryError::DatabaseError(e.to_string()))?,
                monto_usd: row
                    .get(1)
                    .map_err(|e| TuitionRepositoryError::DatabaseError(e.to_string()))?,
                fecha_inscripccion: fecha_inscripccion.to_string(),
            };
            tuitions.push((fecha_inscripccion, tuition));
        }

        tuitions
            .into_iter()
            .max_by_key(|(date, _)| *date)
            .map(|(_, tuition)| tuition)
            .ok_or(TuitionRepositoryError::TuitionNotFound)
    }
}
