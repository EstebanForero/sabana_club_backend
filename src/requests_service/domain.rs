use serde::{Deserialize, Serialize};

use crate::{
    tournament_service::use_cases::TournamentService,
    user_service::{
        domain::{UserCreationInfo, UserUpdating},
        use_cases::UserService,
    },
};

use super::err::Result;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct RequestForApproval {
    pub requester_id: String,
    pub request_id: String,
    pub command_name: String,
    pub command_content: RequestContent,
    pub aprover_id: Option<String>,
    pub completed: bool,
}

impl TryFrom<RequestForApprovalDb> for RequestForApproval {
    type Error = serde_json::Error;

    fn try_from(value: RequestForApprovalDb) -> std::result::Result<Self, Self::Error> {
        let command_content: RequestContent = serde_json::from_str(&value.command_content)?;

        let result = Self {
            request_id: value.request_id,
            requester_id: value.requester_id,
            command_name: value.command_name,
            command_content,
            aprover_id: value.aprover_id,
            completed: value.completed,
        };

        Ok(result)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestForApprovalDb {
    pub requester_id: String,
    pub request_id: String,
    pub command_name: String,
    pub command_content: String,
    pub aprover_id: Option<String>,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestCreation {
    pub command_name: String,
    pub command_content: RequestContent,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum RequestContent {
    UpdateUser {
        user_updation: UserUpdating,
        user_id: String,
    },
}

impl RequestContent {
    pub fn get_name(&self) -> String {
        match &self {
            RequestContent::UpdateUser { .. } => "update_user",
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
