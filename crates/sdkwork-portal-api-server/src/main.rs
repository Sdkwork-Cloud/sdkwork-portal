use axum::Router;
use sdkwork_portal_api_server::portal_health_router;
use sdkwork_portal_service_host::PortalServiceHost;
use sdkwork_router_portal_app_api::build_portal_app_router_with_framework;
use sdkwork_router_portal_backend_api::build_portal_backend_router_with_framework;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting SDKWork Portal API Server...");

    let host = Arc::new(PortalServiceHost::new().await);
    let app_router = build_portal_app_router_with_framework(host.clone()).await;
    let backend_router = build_portal_backend_router_with_framework(host).await;

    let app = Router::new()
        .merge(portal_health_router())
        .merge(app_router)
        .merge(backend_router)
        .layer(CorsLayer::permissive());

    let addr = std::env::var("PORTAL_API_BIND").unwrap_or_else(|_| "0.0.0.0:18091".to_owned());
    tracing::info!("Portal API server listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind portal server");
    axum::serve(listener, app)
        .await
        .expect("serve portal server");
}
