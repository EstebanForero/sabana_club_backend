use async_trait::async_trait;

pub mod err;
pub mod lib_sql_implementation;

#[async_trait]
pub trait RequestRepository {}
