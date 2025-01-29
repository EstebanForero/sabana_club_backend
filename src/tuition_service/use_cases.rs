use std::sync::Arc;

use crate::unique_identifier_service::usecases::UniqueIdentifier;

use super::domain::TuitionInfo;
use super::{domain::Tuition, repository::TuitionRepository};

use super::err::{Result, TuitionServiceError};

#[derive(Clone)]
pub struct TuitionService {
    tuition_repository: Arc<dyn TuitionRepository>,
    unique_identifier: Arc<dyn UniqueIdentifier>,
}

impl TuitionService {
    pub fn new(
        tuition_repository: Arc<dyn TuitionRepository>,
        unique_identifier: Arc<dyn UniqueIdentifier>,
    ) -> Self {
        Self {
            tuition_repository,
            unique_identifier,
        }
    }

    pub async fn create_tuition(&self, id_persona: String, monto_usd: f64) -> Result<()> {
        let tuition = TuitionInfo {
            id_persona,
            monto_usd,
        };

        self.tuition_repository.create_tuition(tuition).await?;

        Ok(())
    }

    pub async fn get_tuitions_for_user(&self, user_identifier: String) -> Result<Vec<Tuition>> {
        let id_persona = match self
            .unique_identifier
            .identify(user_identifier.clone())
            .await
        {
            Some(id_persona) => id_persona,
            None => return Err(TuitionServiceError::UserNotIdentifiable(user_identifier)),
        };

        Ok(self
            .tuition_repository
            .get_tuitions_for_user(&id_persona)
            .await?)
    }

    pub async fn get_most_recent_tuition(&self, id_persona: String) -> Result<Tuition> {
        Ok(self
            .tuition_repository
            .get_most_recent_tuition(&id_persona)
            .await?)
    }
}
