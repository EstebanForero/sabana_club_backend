use std::sync::Arc;

use uuid::Uuid;

use super::{
    domain::{
        CommandExecutor, RequestContent, RequestCreation, RequestForApproval, RequestForApprovalDb,
    },
    repository::RequestRepository,
};

use super::err::Result;

#[derive(Clone)]
pub struct RequestService {
    command_executor: CommandExecutor,
    request_repository: Arc<dyn RequestRepository>,
}

impl RequestService {
    pub fn new(
        command_executor: CommandExecutor,
        request_repository: Arc<dyn RequestRepository>,
    ) -> Self {
        Self {
            command_executor,
            request_repository,
        }
    }

    pub async fn get_requests_by_name(
        &self,
        request_name: String,
    ) -> Result<Vec<RequestForApproval>> {
        Ok(self
            .request_repository
            .get_commands_by_name(&request_name)
            .await?)
    }

    pub async fn get_request_by_id(&self, request_id: String) -> Result<RequestForApproval> {
        Ok(self
            .request_repository
            .get_commands_by_id(&request_id)
            .await?)
    }

    pub async fn execute_request(&self, request_id: String) -> Result<RequestForApproval> {
        let request = self.get_request_by_id(request_id).await?;

        Ok(request)
    }

    pub async fn create_request(
        &self,
        request_content: RequestContent,
        requester_id: String,
    ) -> Result<()> {
        let command_content = serde_json::to_string(&request_content)?;

        let request = RequestForApprovalDb {
            requester_id,
            request_id: Uuid::new_v4().to_string(),
            command_name: request_content.get_name(),
            command_content,
            aprover_id: None,
        };

        self.request_repository.create_command(request).await?;

        Ok(())
    }
}
