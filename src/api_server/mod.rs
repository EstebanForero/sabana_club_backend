use std::error::Error;

use axum::Router;

use crate::global_traits::HttpService;

pub async fn start_http_server(
    port: String,
    http_services: Vec<Box<dyn HttpService>>,
) -> Result<(), Box<dyn Error>> {
    let mut main_router = Router::new();

    for http_service in http_services {
        let service_router = http_service.get_router();

        main_router = main_router.merge(service_router);
    }

    let ip_addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(ip_addr).await?;

    axum::serve(listener, main_router).await?;

    Ok(())
}
