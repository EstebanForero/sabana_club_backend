use crate::user_service::domain::SearchSelection;
use crate::user_service::domain::UserCreationInfo;
use crate::user_service::domain::UserInfo;
use crate::user_service::domain::UserRol;
use crate::user_service::domain::UserSelectionInfo;
use crate::user_service::domain::UserUpdating;
use async_trait::async_trait;
use libsql::{de, params, Connection, Database};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::result;
use std::sync::Arc;
use tracing::info;

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
                id_persona, nombre, contrasena, correo, telefono, identificacion, nombre_tipo_identificacion, nombre_rol
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, 'Usuario'
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
                "SELECT id_persona, nombre, correo, telefono, identificacion, nombre_tipo_identificacion, nombre_rol FROM persona",
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
                "SELECT id_persona, nombre, correo, telefono, identificacion, nombre_tipo_identificacion, nombre_rol FROM persona WHERE id_persona = ?1",
                libsql::params![user_id.clone()],
            )
            .await?;

        if let Some(row) = rows.next().await? {
            Ok(de::from_row::<UserInfo>(&row)?)
        } else {
            Err(UserRepositoryError::UserNotFound)
        }
    }

    async fn search_users_by_search_selection(
        &self,
        search: &str,
        limit: u8,
        search_parameter: SearchSelection,
    ) -> Result<Vec<UserSelectionInfo>> {
        info!("Executing with search: |{search}|, limit: |{limit}| and seach parameter: |{search_parameter:?}|");

        let conn = self.get_connection().await?;

        let column = match search_parameter {
            SearchSelection::Email => "correo",
            SearchSelection::PhoneNumber => "telefono",
            SearchSelection::UserName => "nombre",
        };

        let query = format!(
            "SELECT p.id_persona, p.nombre, p.correo, p.telefono,
                p.identificacion, p.nombre_tipo_identificacion, p.nombre_rol,
                CASE 
                    WHEN (JULIANDAY(DATE('now')) - JULIANDAY(MAX(m.fecha_inscripccion))) > 30 OR MAX(m.fecha_inscripccion) IS NULL THEN FALSE
                    ELSE TRUE
                END AS matricula_valida
                FROM persona p
                LEFT JOIN matricula m ON p.id_persona = m.id_persona
                WHERE p.{} LIKE ?1
                GROUP BY p.id_persona
                LIMIT ?2",
            column
        );

        let limit_i64 = limit as i64;

        let mut rows = conn
            .query(&query, params![format!("%{}%", search), limit_i64])
            .await?;

        let mut users = Vec::new();

        while let Some(row) = rows.next().await? {
            let temp_user = de::from_row::<UserSelectionInfo>(&row)?;

            users.push(temp_user);
        }

        Ok(users)
    }

    async fn modify_user(&self, updated_user_info: UserUpdating, user_id: &str) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute("UPDATE persona SET nombre = ?1, correo = ?2, telefono = ?3, identificacion = ?4, nombre_tipo_identificacion = ?5
                WHERE id_persona = ?6"
            , params![
            updated_user_info.nombre,
            updated_user_info.correo,
            updated_user_info.telefono,
            updated_user_info.identificacion,
            updated_user_info.nombre_tipo_identificacion,
            user_id
        ]).await?;

        Ok(())
    }

    async fn user_rol(&self, user_id: &str) -> Result<UserRol> {
        let conn = self.get_connection().await?;
        let mut rows = conn
            .query(
                "SELECT nombre_rol FROM persona WHERE id_persona = ?1 LIMIT 1",
                params![user_id],
            )
            .await?;

        if let Some(row) = rows.next().await? {
            let string_role = &row.get::<String>(0)?;
            let nombre_rol: UserRol = serde_json::from_str(&json!(string_role).to_string())?;
            Ok(nombre_rol)
        } else {
            Err(UserRepositoryError::UserNotFound)
        }
    }

    async fn update_user_role(&self, user_role: UserRol, user_id: &str) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute(
            "UPDATE persona SET nombre_rol = ?1 WHERE id_persona = ?2",
            params![user_role.to_string(), user_id],
        )
        .await?;

        Ok(())
    }
}
