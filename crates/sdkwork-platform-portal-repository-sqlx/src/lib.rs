use async_trait::async_trait;
use chrono::Utc;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_platform_portal_service::domain::{
    PortalPreference, PortalPreferenceSummary, UpdatePortalPreferenceCommand,
};
use sdkwork_platform_portal_service::ports::PortalRepository;
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct SqlxPortalRepository {
    pool: PgPool,
}

impl SqlxPortalRepository {
    pub fn new(pool: DatabasePool) -> Self {
        Self {
            pool: pool
                .as_postgres()
                .cloned()
                .expect("portal repository requires postgres DatabasePool"),
        }
    }
}

#[async_trait]
impl PortalRepository for SqlxPortalRepository {
    async fn find_by_tenant_and_user(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<PortalPreference>, String> {
        let row = sqlx::query(
            r#"
            SELECT id, tenant_id, user_id, pinned_app_keys, theme, created_at, updated_at
            FROM platform_portal_preference
            WHERE tenant_id = $1 AND user_id = $2
            LIMIT 1
            "#,
        )
        .bind(tenant_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|error| format!("query portal preference failed: {error}"))?;

        Ok(row.map(map_preference))
    }

    async fn upsert_preference(
        &self,
        command: UpdatePortalPreferenceCommand,
    ) -> Result<PortalPreference, String> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let pinned_json = serde_json::to_value(&command.pinned_app_keys)
            .map_err(|error| format!("serialize pinned apps failed: {error}"))?;

        sqlx::query(
            r#"
            INSERT INTO platform_portal_preference (id, tenant_id, user_id, pinned_app_keys, theme, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (tenant_id, user_id) DO UPDATE
            SET pinned_app_keys = EXCLUDED.pinned_app_keys,
                theme = EXCLUDED.theme,
                updated_at = EXCLUDED.updated_at
            "#,
        )
        .bind(id)
        .bind(command.tenant_id)
        .bind(command.user_id)
        .bind(pinned_json)
        .bind(&command.theme)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|error| format!("upsert portal preference failed: {error}"))?;

        self.find_by_tenant_and_user(command.tenant_id, command.user_id)
            .await?
            .ok_or_else(|| "portal preference missing after upsert".to_owned())
    }

    async fn list_for_admin(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<PortalPreferenceSummary>, String> {
        let rows = sqlx::query(
            r#"
            SELECT user_id, theme, pinned_app_keys
            FROM platform_portal_preference
            WHERE tenant_id = $1
            ORDER BY updated_at DESC
            "#,
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| format!("list portal preferences failed: {error}"))?;

        Ok(rows
            .into_iter()
            .map(|row| {
                let pinned: serde_json::Value = row.get("pinned_app_keys");
                let pinned_count = pinned.as_array().map(|items| items.len()).unwrap_or(0);
                PortalPreferenceSummary {
                    user_id: row.get("user_id"),
                    theme: row.get("theme"),
                    pinned_count,
                }
            })
            .collect())
    }
}

fn map_preference(row: sqlx::postgres::PgRow) -> PortalPreference {
    let pinned: serde_json::Value = row.get("pinned_app_keys");
    let pinned_app_keys = pinned
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|value| value.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default();

    PortalPreference {
        id: row.get("id"),
        tenant_id: row.get("tenant_id"),
        user_id: row.get("user_id"),
        pinned_app_keys,
        theme: row.get("theme"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repository_type_name_is_stable() {
        let _ = std::any::type_name::<SqlxPortalRepository>();
    }
}
