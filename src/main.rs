mod api_server;
mod auth_service;

mod tournament_service;
mod trainings_service;
mod tuition_service;
mod user_service;

fn main() {
    tracing_subscriber::fmt::init();

    println!("Hello, world!");
}
