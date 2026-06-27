use sdkwork_portal_gateway_assembly::assemble_application_router;
use sdkwork_portal_service_host::PortalServiceHost;
use sdkwork_web_bootstrap::{service_router, ServiceRouterConfig};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting SDKWork Portal API Server...");

    let host = Arc::new(PortalServiceHost::new().await);
    let assembly = assemble_application_router(host).await;

    let business = assembly.router.layer(CorsLayer::permissive());
    let app = service_router(business, ServiceRouterConfig::default().with_always_ready());

    let addr = std::env::var("PORTAL_API_BIND").unwrap_or_else(|_| "0.0.0.0:18091".to_owned());
    tracing::info!("Portal API server listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("bind portal server");
    axum::serve(listener, app)
        .await
        .expect("serve portal server");
}
