use crate::{user_service::domain::UserCreationInfo, Error};
use async_trait::async_trait;
use libsql::{params, Connection, Database};
use std::result;
use std::sync::Arc;

use super::err::{Result, UserRepositoryError};
use super::UserRepository;

#[derive(Clone)]
pub struct Repository {
    db: Arc<Database>,
}

impl Repository {
    pub async fn new(url: String, token: String) -> result::Result<Self, String> {
        let db = libsql::Builder::new_remote(url, token)
            .build()
            .await
            .map_err(|err| format!("Error creating new remote database for libsql: {err}"))?;

        Ok(Self { db: Arc::new(db) })
    }

    async fn get_connection(&self) -> Result<Connection> {
        Ok(self.db.connect()?)
    }
}

#[async_trait]
impl UserRepository for Repository {
    async fn create_user(&self, user_creation_info: UserCreationInfo) -> Result<()> {
        let conn = self.get_connection().await?;
        conn.execute(
            "INSERT INTO persona (
                id_persona, nombre, contrasena, correo, telefono, identificacion, nombre_tipo_identificacion, es_admin
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, 0
            )",
            params![
                uuid::Uuid::new_v4().to_string(),
                user_creation_info.nombre,
                user_creation_info.contrasena,
                user_creation_info.correo,
                user_creation_info.telefono,
                user_creation_info.identificacion,
                user_creation_info.nombre_tipo_identificacion
            ],
        )
        .await?;

        Ok(())
    }

    async fn get_user_id_by_email(&self, email: &String) -> Result<String> {
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

    async fn get_user_id_by_phone_number(&self, phone_number: &String) -> Result<String> {
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

    async fn get_user_password(&self, user_id: &String) -> Result<String> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query(
                "SELECT contrasena FROM persona WHERE id_persona = ?1 LIMIT 1",
                params![user_id.to_string()],
            )
            .await?;

        if let Some(row) = rows.next().await? {
            let contrasena: String = row.get(0)?;
            Ok(contrasena)
        } else {
            Err(UserRepositoryError::UserNotFound)
        }
    }
}
