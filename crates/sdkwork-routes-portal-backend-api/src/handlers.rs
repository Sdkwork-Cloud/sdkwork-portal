use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sdkwork_web_core::WebRequestContext;
use serde_json::json;
use uuid::Uuid;

use crate::routes::PortalBackendState;

pub async fn list_preferences_admin(
    State(state): State<PortalBackendState>,
    context: WebRequestContext,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let tenant_id = parse_uuid(context.tenant_id(), "tenant_id")?;
    let items = state
        .host
        .portal_service()
        .list_preferences_for_admin(tenant_id)
        .await
        .map_err(|message| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "code": "portal.internal_error", "message": message })),
            )
        })?;

    Ok(Json(json!({
        "items": items.into_iter().map(|item| json!({
            "userId": item.user_id,
            "theme": item.theme,
            "pinnedCount": item.pinned_count
        })).collect::<Vec<_>>()
    })))
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
