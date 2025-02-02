use std::{result, sync::Arc};

use crate::requests_service::domain::{RequestContent, RequestForApproval, RequestForApprovalDb};

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
    async fn aprove_request(&self, request_id: &str, approver_id: &str) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute(
            "UPDATE request_for_approval SET completed = 1, aprover_id = ?1 WHERE request_id = ?2",
            params![approver_id, request_id],
        )
        .await?;

        Ok(())
    }

    async fn get_commands_by_name(&self, command_name: &str) -> Result<Vec<RequestForApproval>> {
        let conn = self.get_connection().await?;

        let mut rows = conn.query(
            "SELECT r.requester_id, r.request_id, r.command_name, r.command_content, r.aprover_id, r.completed
            FROM request_for_approval AS r WHERE r.command_name = ?1",
            params![command_name],
        ).await?;

        let mut requests = Vec::new();

        while let Some(row) = rows.next().await? {
            let request: RequestForApprovalDb = de::from_row(&row)?;

            requests.push(RequestForApproval::try_from(request)?);
        }

        Ok(requests)
    }

    async fn get_commands_by_id(&self, command_id: &str) -> Result<RequestForApproval> {
        let conn = self.get_connection().await?;

        let mut row = conn.query(
            "SELECT r.requester_id, r.request_id, r.command_name, r.command_content, r.aprover_id, r.completed
            FROM request_for_approval AS r WHERE r.request_id = ?1",
            params![command_id],
        ).await?;

        let command: RequestForApprovalDb = match row.next().await? {
            Some(next_row) => de::from_row(&next_row)?,
            None => return Err(RequestRepositoryError::CommandDontExist),
        };

        Ok(RequestForApproval::try_from(command)?)
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

    async fn delete_request(&self, request_id: &str) -> Result<()> {
        let conn = self.get_connection().await?;

        conn.execute(
            "DELETE FROM request_for_approval WHERE request_id = ?1",
            params![request_id],
        )
        .await?;

        Ok(())
    }

    async fn get_all_commands(&self) -> Result<Vec<RequestForApproval>> {
        let conn = self.get_connection().await?;

        let mut rows = conn
            .query(
                "SELECT requester_id, request_id, command_name, command_content, aprover_id, completed
FROM request_for_approval",
                params![],
            )
            .await?;

        let mut requests = Vec::new();

        while let Some(row) = rows.next().await? {
            let request: RequestForApprovalDb = de::from_row(&row)?;

            let request = RequestForApproval::try_from(request)?;

            requests.push(request);
        }

        Ok(requests)
    }
}

#[cfg(test)]
mod tests {
    use crate::{requests_service::domain::RequestContent, user_service::domain::UserUpdating};

    use super::*;
    use serde_json;

    #[test]
    fn test_serialization_and_deserialization() {
        let original_request = RequestForApproval {
            requester_id: "12345".to_string(),
            request_id: "54321".to_string(),
            command_name: "UpdateUserCommand".to_string(),
            command_content: RequestContent::UpdateUser {
                user_updation: UserUpdating {
                    nombre: "Esteban".to_string(),
                    correo: "estebanmff@gmail.com".to_string(),
                    telefono: 3185920708,
                    identificacion: "1014739191".to_string(),
                    nombre_tipo_identificacion: "CC".to_string(),
                },
                user_id: "5f405541-d1df-454a-b2fc-56004ba380cc".to_string(),
            },
            completed: false,
            aprover_id: Some("67890".to_string()),
        };

        let serialized =
            serde_json::to_string(&original_request).expect("Serialization should succeed");

        let deserialized: RequestForApproval =
            serde_json::from_str(&serialized).expect("Deserialization should succeed");

        assert_eq!(original_request, deserialized);

        println!("Serialized JSON: {}", serialized);
    }

    #[test]
    fn test_deserialization_of_command_content() {
        let json = r#"
        {
            "requester_id": "12345",
            "request_id": "54321",
            "command_name": "UpdateUserCommand",
            "command_content": {
                "UpdateUser": {
                    "user_updation": {
                        "nombre": "Esteban",
                        "correo": "estebanmff@gmail.com",
                        "telefono": 3185920708,
                        "identificacion": "1014739191",
                        "nombre_tipo_identificacion": "CC"
                    },
                    "user_id": "5f405541-d1df-454a-b2fc-56004ba380cc"
                }
            },
            "aprover_id": "67890"
        }
        "#;

        let deserialized: RequestForApproval =
            serde_json::from_str(json).expect("Deserialization should succeed");

        if let RequestContent::UpdateUser {
            user_updation,
            user_id,
        } = deserialized.command_content
        {
            assert_eq!(user_updation.nombre, "Esteban");
            assert_eq!(user_updation.correo, "estebanmff@gmail.com");
            assert_eq!(user_updation.telefono, 3185920708);
            assert_eq!(user_updation.identificacion, "1014739191");
            assert_eq!(user_updation.nombre_tipo_identificacion, "CC");
            assert_eq!(user_id, "5f405541-d1df-454a-b2fc-56004ba380cc");
        } else {
            panic!("Expected command_content to be UpdateUser variant");
        }
    }
}
