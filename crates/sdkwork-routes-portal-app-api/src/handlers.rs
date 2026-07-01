use axum::extract::State;
use axum::response::Response;
use sdkwork_routes_portal_common::{
    envelope::{preference_resource, PortalPreferenceItem},
    finish_api_json, ApiProblem, SdkWorkResourceData,
};
use sdkwork_web_core::WebRequestContext;
use serde::Deserialize;
use uuid::Uuid;

use crate::routes::PortalAppState;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePreferencesBody {
    pub pinned_app_keys: Vec<String>,
    pub theme: String,
}

pub async fn retrieve_preferences(
    ctx: WebRequestContext,
    State(state): State<PortalAppState>,
) -> Response {
    finish_api_json(&ctx, retrieve_preferences_inner(&ctx, state).await)
}

pub async fn update_preferences(
    ctx: WebRequestContext,
    State(state): State<PortalAppState>,
    axum::Json(body): axum::Json<UpdatePreferencesBody>,
) -> Response {
    finish_api_json(&ctx, update_preferences_inner(&ctx, state, body).await)
}

async fn retrieve_preferences_inner(
    ctx: &WebRequestContext,
    state: PortalAppState,
) -> Result<SdkWorkResourceData<PortalPreferenceItem>, ApiProblem> {
    let tenant_id = parse_uuid(ctx.tenant_id(), "tenant_id")?;
    let user_id = parse_uuid(ctx.user_id(), "user_id")?;

    let preference = state
        .host
        .portal_service()
        .retrieve_preferences(tenant_id, user_id)
        .await
        .map_err(ApiProblem::internal_server_error)?;

    Ok(preference_resource(PortalPreferenceItem {
        pinned_app_keys: preference.pinned_app_keys,
        theme: preference.theme,
    }))
}

async fn update_preferences_inner(
    ctx: &WebRequestContext,
    state: PortalAppState,
    body: UpdatePreferencesBody,
) -> Result<SdkWorkResourceData<PortalPreferenceItem>, ApiProblem> {
    let tenant_id = parse_uuid(ctx.tenant_id(), "tenant_id")?;
    let user_id = parse_uuid(ctx.user_id(), "user_id")?;

    let preference = state
        .host
        .portal_service()
        .update_preferences(
            sdkwork_platform_portal_service::UpdatePortalPreferenceCommand {
                tenant_id,
                user_id,
                pinned_app_keys: body.pinned_app_keys,
                theme: body.theme,
            },
        )
        .await
        .map_err(ApiProblem::bad_request)?;

    Ok(preference_resource(PortalPreferenceItem {
        pinned_app_keys: preference.pinned_app_keys,
        theme: preference.theme,
    }))
}

fn parse_uuid(value: Option<&str>, field: &str) -> Result<Uuid, ApiProblem> {
    let raw = value.ok_or_else(|| ApiProblem::unauthorized(format!("missing {field}")))?;
    Uuid::parse_str(raw).map_err(|_| ApiProblem::bad_request(format!("invalid {field}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_body_deserializes_camel_case() {
        let body: UpdatePreferencesBody =
            serde_json::from_str(r#"{"pinnedAppKeys":["drive"],"theme":"dark"}"#).expect("json");
        assert_eq!(body.theme, "dark");
        assert_eq!(body.pinned_app_keys, vec!["drive".to_owned()]);
    }
}
