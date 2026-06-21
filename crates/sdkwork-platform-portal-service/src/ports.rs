use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{PortalPreference, PortalPreferenceSummary, UpdatePortalPreferenceCommand};

#[async_trait]
pub trait PortalRepository: Send + Sync {
    async fn find_by_tenant_and_user(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<PortalPreference>, String>;

    async fn upsert_preference(
        &self,
        command: UpdatePortalPreferenceCommand,
    ) -> Result<PortalPreference, String>;

    async fn list_for_admin(&self, tenant_id: Uuid)
        -> Result<Vec<PortalPreferenceSummary>, String>;
}
