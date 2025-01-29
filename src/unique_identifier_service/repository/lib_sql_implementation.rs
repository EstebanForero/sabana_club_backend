use std::{result, sync::Arc};

use super::{
    err::{Result, UserRepositoryError},
    UniqueIdentifierRepository,
};
use async_trait::async_trait;
use libsql::{params, Connection, Database};

#[derive(Clone)]
pub struct LibSqlUniqueIdentifierRepo {
    db: Arc<Database>,
}

impl LibSqlUniqueIdentifierRepo {
    pub async fn new(
        url: &str,
        token: &str,
    ) -> result::Result<Arc<dyn UniqueIdentifierRepository>, String> {
        let db = libsql::Builder::new_remote(url.to_string(), token.to_string())
            .build()
            .await
            .map_err(|err| format!("Error creating new remote database for libsql: {err}"))?;

        Ok(Arc::new(Self { db: Arc::new(db) }))
    }

    async fn get_connection(&self) -> Result<Connection> {
        Ok(self.db.connect()?)
    }
}

#[async_trait]
impl UniqueIdentifierRepository for LibSqlUniqueIdentifierRepo {
    async fn get_user_id_by_email(&self, email: &str) -> Result<String> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query(
                "SELECT id_persona FROM persona WHERE correo = ?1 LIMIT 1",
                params![email.to_string()],
            )
            .await?;

        if let Some(row) = rows.next().await? {
            let id_persona: String = row.get(0)?;
            Ok(id_persona)
        } else {
            Err(UserRepositoryError::UserNotFound)
        }
    }

    async fn get_user_id_by_phone_number(&self, phone_number: &str) -> Result<String> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query(
                "SELECT id_persona FROM persona WHERE telefono = ?1 LIMIT 1",
                params![phone_number.to_string()],
            )
            .await?;

        if let Some(row) = rows.next().await? {
            let id_persona: String = row.get(0)?;
            Ok(id_persona)
        } else {
            Err(UserRepositoryError::UserNotFound)
        }
    }

    async fn comprove_id_existance(&self, user_id: &str) -> Result<()> {
        let conn = self.get_connection().await?;

        let mut rows = conn
            .query(
                "SELECT EXISTS (SELECT 1 FROM persona WHERE id_persona = ?1)",
                params![user_id],
            )
            .await?;

        if let Some(row) = rows.next().await? {
            let exists: i32 = row.get(0)?;

            if exists == 1 {
                Ok(())
            } else {
                Err(UserRepositoryError::UserNotFound)
            }
        } else {
            Err(UserRepositoryError::UserNotFound)
        }
    }
}
