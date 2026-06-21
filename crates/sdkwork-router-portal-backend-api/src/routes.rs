use axum::routing::get;
use axum::Router;
use sdkwork_portal_service_host::PortalServiceHost;
use std::sync::Arc;

use crate::handlers;
use crate::web_bootstrap::wrap_router_with_web_framework_from_env;

#[derive(Clone)]
pub struct PortalBackendState {
    pub host: Arc<PortalServiceHost>,
}

pub fn build_portal_backend_router(host: Arc<PortalServiceHost>) -> Router {
    let state = PortalBackendState { host };
    Router::new()
        .route(
            "/backend/v3/api/portal/preferences",
            get(handlers::list_preferences_admin),
        )
        .with_state(state)
}

pub async fn build_portal_backend_router_with_framework(host: Arc<PortalServiceHost>) -> Router {
    wrap_router_with_web_framework_from_env(build_portal_backend_router(host)).await
}
