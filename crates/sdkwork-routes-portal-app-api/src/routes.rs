use axum::routing::get;
use axum::Router;
use sdkwork_portal_service_host::PortalServiceHost;
use std::sync::Arc;

use crate::handlers;
use crate::web_bootstrap::wrap_router_with_web_framework_from_env;

#[derive(Clone)]
pub struct PortalAppState {
    pub host: Arc<PortalServiceHost>,
}

pub fn build_portal_app_router(host: Arc<PortalServiceHost>) -> Router {
    let state = PortalAppState { host };
    Router::new()
        .route(
            "/app/v3/api/portal/preferences",
            get(handlers::retrieve_preferences).put(handlers::update_preferences),
        )
        .with_state(state)
}

pub async fn build_portal_app_router_with_framework(host: Arc<PortalServiceHost>) -> Router {
    wrap_router_with_web_framework_from_env(build_portal_app_router(host)).await
}
