use axum::{
    Router,
    routing::{get, post}
};

use crate::handlers;
use crate::types::Args;

pub async fn app() -> Router {
    log::info!("Create app - route");

    let api_ver = "v1";

    Router::new()
        .route(
            format!("/{}/generate/:secret", api_ver).as_str(),
            get(handlers::generate)
        )
        .route(
            format!("/{}/generate", api_ver).as_str(),
            post(handlers::generate_by_url)
        )
}

pub async fn serve(args: Args) {
    log::info!("Start serve app: {}:{}", args.host, args.port);

    let bind_addr = format!("{}:{}", args.host, args.port);
    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();
    let router = app().await;

    axum::serve(listener, router)
        .await
        .expect("failed to serve API");
}