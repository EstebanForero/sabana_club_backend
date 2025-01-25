use std::error::Error;

mod api_server;
mod auth_service;

mod tournament_service;
mod trainings_service;
mod tuition_service;
mod user_service;

mod global_traits;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    Ok(())
}
