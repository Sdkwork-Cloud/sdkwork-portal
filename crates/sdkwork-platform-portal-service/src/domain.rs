use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortalPreference {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub pinned_app_keys: Vec<String>,
    pub theme: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortalPreferenceSummary {
    pub user_id: Uuid,
    pub theme: String,
    pub pinned_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdatePortalPreferenceCommand {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub pinned_app_keys: Vec<String>,
    pub theme: String,
}
