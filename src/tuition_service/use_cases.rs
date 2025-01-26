use std::sync::Arc;

use super::domain::TuitionInfo;
use super::{domain::Tuition, repository::TuitionRepository};

use super::err::Result;

#[derive(Clone)]
pub struct TuitionService {
    tuition_repository: Arc<dyn TuitionRepository>,
}

impl TuitionService {
    pub fn new(tuition_repository: Arc<dyn TuitionRepository>) -> Self {
        Self { tuition_repository }
    }

    pub async fn create_tuition(&self, id_persona: String, monto_usd: f64) -> Result<()> {
        let tuition = TuitionInfo {
            id_persona,
            monto_usd,
        };

        self.tuition_repository.create_tuition(tuition).await?;

        Ok(())
    }

    pub async fn get_tuitions_for_user(&self, id_persona: String) -> Result<Vec<Tuition>> {
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
