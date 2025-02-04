use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    tournament_service::{err::TournamentServiceError, use_cases::TournamentService},
    trainings_service::use_cases::TrainingService,
    user_service::{
        domain::{UserCreationInfo, UserUpdating},
        err::UserServiceError,
        use_cases::UserService,
    },
};

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
#[serde(tag = "type")]
pub enum RequestContent {
    UpdateUser {
        user_updation: UserUpdating,
        user_id: String,
    },
    DeleteTournament {
        tournament_id: String,
    },
    DeleteTraining {
        training_id: String,
    },
}

impl RequestContent {
    pub fn get_name(&self) -> String {
        match &self {
            RequestContent::UpdateUser { .. } => "update_user",
            RequestContent::DeleteTournament { .. } => "delete_tournament",
            RequestContent::DeleteTraining { .. } => "delete_training",
        }
        .to_string()
    }
}

#[derive(Clone)]
pub struct CommandExecutor {
    pub user_service: UserService,
    pub tournament_service: TournamentService,
    pub training_service: TrainingService,
}

impl CommandExecutor {
    pub async fn execute_command(
        &self,
        request_content: RequestContent,
    ) -> std::result::Result<(), CommandError> {
        match request_content {
            RequestContent::UpdateUser {
                user_updation,
                user_id,
            } => {
                self.user_service
                    .update_user(user_updation, &user_id)
                    .await?;
            }
            RequestContent::DeleteTournament { tournament_id } => {
                self.tournament_service
                    .delete_tournament(&tournament_id)
                    .await?;
            }
            RequestContent::DeleteTraining { training_id } => {
                self.training_service.delete_training(&training_id);
            }
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Error executing commad in the user service")]
    UserServiceError(#[from] UserServiceError),
    #[error("Error executing commad in the user service")]
    TournamentServiceError(#[from] TournamentServiceError),
}
