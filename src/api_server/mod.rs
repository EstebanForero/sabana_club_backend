use std::error::Error;

use axum::Router;

async fn start_http_server(port: String) -> Result<(), Box<dyn Error>> {
    let main_router = Router::new();

    let ip_addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(ip_addr).await?;

    axum::serve(listener, main_router).await?;

    Ok(())
}
