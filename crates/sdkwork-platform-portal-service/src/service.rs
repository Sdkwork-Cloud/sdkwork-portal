use uuid::Uuid;

use crate::domain::{PortalPreference, PortalPreferenceSummary, UpdatePortalPreferenceCommand};
use crate::ports::PortalRepository;

pub struct PortalService<R: PortalRepository> {
    repository: R,
}

impl<R: PortalRepository> PortalService<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn retrieve_preferences(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
    ) -> Result<PortalPreference, String> {
        match self
            .repository
            .find_by_tenant_and_user(tenant_id, user_id)
            .await?
        {
            Some(preference) => Ok(preference),
            None => Ok(default_preference(tenant_id, user_id)),
        }
    }

    pub async fn update_preferences(
        &self,
        command: UpdatePortalPreferenceCommand,
    ) -> Result<PortalPreference, String> {
        if command.theme.trim().is_empty() {
            return Err("portal theme is required".to_owned());
        }
        self.repository.upsert_preference(command).await
    }

    pub async fn list_preferences_for_admin(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<PortalPreferenceSummary>, String> {
        self.repository.list_for_admin(tenant_id).await
    }
}

fn default_preference(tenant_id: Uuid, user_id: Uuid) -> PortalPreference {
    let now = chrono::Utc::now();
    PortalPreference {
        id: Uuid::new_v4(),
        tenant_id,
        user_id,
        pinned_app_keys: Vec::new(),
        theme: "system".to_owned(),
        created_at: now,
        updated_at: now,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use async_trait::async_trait;
    use chrono::Utc;

    use super::*;
    use crate::domain::PortalPreference;

    struct MemoryPortalRepository {
        preferences: Mutex<Vec<PortalPreference>>,
    }

    impl MemoryPortalRepository {
        fn new() -> Self {
            Self {
                preferences: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl PortalRepository for MemoryPortalRepository {
        async fn find_by_tenant_and_user(
            &self,
            tenant_id: Uuid,
            user_id: Uuid,
        ) -> Result<Option<PortalPreference>, String> {
            Ok(self
                .preferences
                .lock()
                .map_err(|_| "lock poisoned".to_owned())?
                .iter()
                .find(|item| item.tenant_id == tenant_id && item.user_id == user_id)
                .cloned())
        }

        async fn upsert_preference(
            &self,
            command: UpdatePortalPreferenceCommand,
        ) -> Result<PortalPreference, String> {
            let now = Utc::now();
            let preference = PortalPreference {
                id: Uuid::new_v4(),
                tenant_id: command.tenant_id,
                user_id: command.user_id,
                pinned_app_keys: command.pinned_app_keys,
                theme: command.theme,
                created_at: now,
                updated_at: now,
            };
            self.preferences
                .lock()
                .map_err(|_| "lock poisoned".to_owned())?
                .push(preference.clone());
            Ok(preference)
        }

        async fn list_for_admin(
            &self,
            tenant_id: Uuid,
        ) -> Result<Vec<PortalPreferenceSummary>, String> {
            Ok(self
                .preferences
                .lock()
                .map_err(|_| "lock poisoned".to_owned())?
                .iter()
                .filter(|item| item.tenant_id == tenant_id)
                .map(|item| PortalPreferenceSummary {
                    user_id: item.user_id,
                    theme: item.theme.clone(),
                    pinned_count: item.pinned_app_keys.len(),
                })
                .collect())
        }
    }

    #[tokio::test]
    async fn portal_service_rejects_empty_theme() {
        let service = PortalService::new(MemoryPortalRepository::new());
        let result = service
            .update_preferences(UpdatePortalPreferenceCommand {
                tenant_id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                pinned_app_keys: vec![],
                theme: "   ".to_owned(),
            })
            .await;
        assert!(result.is_err());
    }
}
