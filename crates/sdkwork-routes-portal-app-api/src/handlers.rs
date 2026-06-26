use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sdkwork_platform_portal_service::UpdatePortalPreferenceCommand;
use sdkwork_web_core::WebRequestContext;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::routes::PortalAppState;

#[derive(Debug, Deserialize)]
pub struct UpdatePreferencesBody {
    pub pinned_app_keys: Vec<String>,
    pub theme: String,
}

#[derive(Debug, Serialize)]
pub struct PreferencesResponse {
    pub pinned_app_keys: Vec<String>,
    pub theme: String,
}

pub async fn retrieve_preferences(
    State(state): State<PortalAppState>,
    context: WebRequestContext,
) -> Result<Json<PreferencesResponse>, (StatusCode, Json<serde_json::Value>)> {
    let tenant_id = parse_uuid(context.tenant_id(), "tenant_id")?;
    let user_id = parse_uuid(context.user_id(), "user_id")?;

    let preference = state
        .host
        .portal_service()
        .retrieve_preferences(tenant_id, user_id)
        .await
        .map_err(internal_error)?;

    Ok(Json(PreferencesResponse {
        pinned_app_keys: preference.pinned_app_keys,
        theme: preference.theme,
    }))
}

pub async fn update_preferences(
    State(state): State<PortalAppState>,
    context: WebRequestContext,
    Json(body): Json<UpdatePreferencesBody>,
) -> Result<Json<PreferencesResponse>, (StatusCode, Json<serde_json::Value>)> {
    let tenant_id = parse_uuid(context.tenant_id(), "tenant_id")?;
    let user_id = parse_uuid(context.user_id(), "user_id")?;

    let preference = state
        .host
        .portal_service()
        .update_preferences(UpdatePortalPreferenceCommand {
            tenant_id,
            user_id,
            pinned_app_keys: body.pinned_app_keys,
            theme: body.theme,
        })
        .await
        .map_err(bad_request)?;

    Ok(Json(PreferencesResponse {
        pinned_app_keys: preference.pinned_app_keys,
        theme: preference.theme,
    }))
}

fn parse_uuid(
    value: Option<&str>,
    field: &str,
) -> Result<Uuid, (StatusCode, Json<serde_json::Value>)> {
    let raw = value.ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "code": "request.missing_context",
                "message": format!("missing {field}")
            })),
        )
    })?;
    Uuid::parse_str(raw).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "code": "request.invalid_context",
                "message": format!("invalid {field}")
            })),
        )
    })
}

fn internal_error(message: String) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "code": "portal.internal_error", "message": message })),
    )
}

fn bad_request(message: String) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({ "code": "portal.bad_request", "message": message })),
    )
}
