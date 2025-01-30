use crate::user_service::domain::SearchSelection;
use crate::user_service::domain::UserCreationInfo;
use crate::user_service::domain::UserInfo;
use crate::user_service::domain::UserSelectionInfo;
use async_trait::async_trait;
use libsql::{de, params, Connection, Database};
use serde::Deserialize;
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

    async fn search_users_by_search_selection(
        &self,
        search: &str,
        limit: u8,
        search_parameter: SearchSelection,
    ) -> Result<Vec<UserSelectionInfo>> {
        use chrono::{NaiveDate, Utc};

        #[derive(Deserialize)]
        struct TempSelectionInfo {
            id_persona: String,
            nombre: String,
            correo: String,
            telefono: u32,
            identificacion: String,
            nombre_tipo_identificacion: String,
            es_admin: bool,
            fecha_inscripccion: String,
        }

        let conn = self.get_connection().await?;

        let column = match search_parameter {
            SearchSelection::Email => "correo",
            SearchSelection::PhoneNumber => "telefono",
            SearchSelection::UserName => "nombre",
        };

        let query = format!(
            "SELECT p.id_persona, p.nombre, p.correo, p.telefono, \
                p.identificacion, p.nombre_tipo_identificacion, p.es_admin, \
                m.fecha_inscripccion \
                FROM persona p \
                JOIN matricula m ON p.id_persona = m.id_persona \
                WHERE p.{} LIKE ?1 \
                LIMIT ?2",
            column
        );

        let limit_i64 = limit as i64;

        let mut rows = conn
            .query(&query, params![format!("%{}%", search), limit_i64])
            .await?;

        let mut users = Vec::new();
        while let Some(row) = rows.next().await? {
            let temp_user = de::from_row::<TempSelectionInfo>(&row)?;

            let parsed_date = NaiveDate::parse_from_str(&temp_user.fecha_inscripccion, "%Y-%m-%d")?;

            let days_passed = Utc::now()
                .date_naive()
                .signed_duration_since(parsed_date)
                .num_days();
            let matricula_valida = days_passed <= 30;

            users.push(UserSelectionInfo {
                id_persona: temp_user.id_persona,
                nombre: temp_user.nombre,
                correo: temp_user.correo,
                telefono: temp_user.telefono,
                identificacion: temp_user.identificacion,
                nombre_tipo_identificacion: temp_user.nombre_tipo_identificacion,
                es_admin: temp_user.es_admin,
                matricula_valida,
            });
        }

        Ok(users)
    }
}
