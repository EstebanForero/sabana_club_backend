use serde::{Deserialize, Serialize};

use crate::{
    tournament_service::use_cases::TournamentService,
    user_service::{
        domain::{UserCreationInfo, UserUpdating},
        use_cases::UserService,
    },
};

use super::err::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestForApproval {
    pub requester_id: String,
    pub request_id: String,
    pub command_name: String,
    pub command_content: RequestContent,
    pub aprover_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestForApprovalDb {
    pub requester_id: String,
    pub request_id: String,
    pub command_name: String,
    pub command_content: String,
    pub aprover_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestCreation {
    pub command_name: String,
    pub command_content: RequestContent,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RequestContent {
    UpdateUser {
        user_updation: UserUpdating,
        user_id: String,
    },
}

impl RequestContent {
    pub fn get_name(&self) -> String {
        match &self {
            RequestContent::UpdateUser { .. } => "create_user",
        }
        .to_string()
    }
}

#[derive(Clone)]
pub struct CommandExecutor {
    pub user_service: UserService,
    pub tournament_service: TournamentService,
}

impl CommandExecutor {
    pub async fn execute_command(&self, request_content: RequestContent) -> Result<()> {
        match request_content {
            RequestContent::UpdateUser {
                user_updation,
                user_id,
            } => {
                self.user_service
                    .update_user(user_updation, &user_id)
                    .await?;
            }
        }

        Ok(())
    }
}
