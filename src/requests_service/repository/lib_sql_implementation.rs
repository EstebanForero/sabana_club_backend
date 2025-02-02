use std::{result, sync::Arc};

use crate::requests_service::domain::{RequestForApproval, RequestForApprovalDb};

use super::{
    err::{RequestRepositoryError, Result},
    RequestRepository,
};
use async_trait::async_trait;
use libsql::{de, params, Connection, Database};

#[derive(Clone)]
pub struct LibSqlRequestRepository {
    db: Arc<Database>,
}

impl LibSqlRequestRepository {
    pub async fn new(url: &str, token: &str) -> result::Result<Arc<dyn RequestRepository>, String> {
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
impl RequestRepository for LibSqlRequestRepository {
    async fn get_commands_by_name(&self, command_name: &str) -> Result<Vec<RequestForApproval>> {
        let conn = self.get_connection().await?;

        let mut rows = conn.query(
            "SELECT r.requester_id, r.request_id, r.command_name, r.command_content, r.aprover_id
            FROM request_for_approval AS r WHERE r.command_name = ?1",
            params![command_name],
        ).await?;

        let mut requests = Vec::new();

        while let Some(row) = rows.next().await? {
            let request = de::from_row(&row)?;

            requests.push(request);
        }

        Ok(requests)
    }

    async fn get_commands_by_id(&self, command_id: &str) -> Result<RequestForApproval> {
        let conn = self.get_connection().await?;

        let mut row = conn.query(
            "SELECT r.requester_id, r.request_id, r.command_name, r.command_content, r.aprover_id
            FROM request_for_approval AS r WHERE r.request_id = ?1",
            params![command_id],
        ).await?;

        let command: RequestForApproval = match row.next().await? {
            Some(next_row) => de::from_row(&next_row)?,
            None => return Err(RequestRepositoryError::CommandDontExist),
        };

        Ok(command)
    }

    async fn create_command(&self, request: RequestForApprovalDb) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute(
            "INSERT INTO request_for_approval 
            (requester_id, request_id, command_name, command_content, aprover_id)
            VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                request.requester_id,
                request.request_id,
                request.command_name,
                request.command_content,
                request.aprover_id
            ],
        )
        .await?;

        Ok(())
    }
}
