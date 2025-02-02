use async_trait::async_trait;

pub mod err;
pub mod lib_sql_implementation;

use super::domain::{RequestForApproval, RequestForApprovalDb};
use err::Result;

#[async_trait]
pub trait RequestRepository: Send + Sync {
    async fn get_commands_by_name(&self, command_name: &str) -> Result<Vec<RequestForApproval>>;

    async fn get_commands_by_id(&self, command_id: &str) -> Result<RequestForApproval>;

    async fn create_command(&self, request: RequestForApprovalDb) -> Result<()>;
}
