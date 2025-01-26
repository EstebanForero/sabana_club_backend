use async_trait::async_trait;

pub mod err;
pub mod lib_sql_implementation;

#[async_trait]
pub trait TuitionRepository: Send + Sync {
    async fn create_tuition(&self, tuition: Tuition) -> Result<()>;

    async fn get_tuitions_for_user(&self, id_persona: &String) -> Result<Vec<Tuition>>;

    async fn get_most_recent_tuition(&self, id_persona: &String) -> Result<Tuition>;
}
