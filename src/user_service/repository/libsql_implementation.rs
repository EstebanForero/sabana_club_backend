use crate::user_service::domain::UserCreationInfo;
use crate::user_service::domain::UserInfo;
use async_trait::async_trait;
use libsql::{de, params, Connection, Database};
use std::result;
use std::sync::Arc;

use super::err::{Result, UserRepositoryError};
use super::UserRepository;

#[derive(Clone)]
pub struct LibSqlUserRepository {
    db: Arc<Database>,
}

impl LibSqlUserRepository {
    pub async fn new(url: &str, token: &str) -> result::Result<Arc<dyn UserRepository>, String> {
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
impl UserRepository for LibSqlUserRepository {
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

    async fn get_users(&self) -> Result<Vec<UserInfo>> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query(
                "SELECT id_persona, nombre, correo, telefono, identificacion, nombre_tipo_identificacion, es_admin FROM persona",
                libsql::params![],
            )
            .await?;

        let mut users = Vec::new();
        while let Some(row) = rows.next().await? {
            let user = de::from_row::<UserInfo>(&row)?;
            users.push(user);
        }

        Ok(users)
    }

    async fn get_user_by_id(&self, user_id: &String) -> Result<UserInfo> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query(
                "SELECT id_persona, nombre, correo, telefono, identificacion, nombre_tipo_identificacion, es_admin FROM persona WHERE id_persona = ?1",
                libsql::params![user_id.clone()],
            )
            .await?;

        if let Some(row) = rows.next().await? {
            Ok(de::from_row::<UserInfo>(&row)?)
        } else {
            Err(UserRepositoryError::UserNotFound)
        }
    }
}
