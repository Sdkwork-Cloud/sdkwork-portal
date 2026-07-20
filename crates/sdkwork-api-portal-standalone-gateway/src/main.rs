use sdkwork_api_portal_assembly::assemble_api_router;
use sdkwork_portal_service_host::PortalServiceHost;
use sdkwork_web_bootstrap::{service_router, ServiceRouterConfig};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting SDKWork Portal API Server...");

    let host = Arc::new(PortalServiceHost::new().await);
    let assembly = assemble_api_router(host).await;

    let business = assembly.router.layer(
        sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_PORTAL_ENVIRONMENT"],
            &["SDKWORK_PORTAL_CORS_ALLOWED_ORIGINS", "SDKWORK_CORS_ALLOWED_ORIGINS"],
        ),
    );
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
