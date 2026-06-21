use sdkwork_database_sqlx::DatabasePool;
use sdkwork_platform_portal_repository_sqlx::SqlxPortalRepository;
use sdkwork_platform_portal_service::PortalService;
use sdkwork_portal_database_host::{bootstrap_portal_database_from_env, PortalDatabaseHost};

pub struct PortalServiceHost {
    database: PortalDatabaseHost,
    portal_service: PortalService<SqlxPortalRepository>,
}

impl PortalServiceHost {
    pub async fn new() -> Self {
        Self::from_env()
            .await
            .expect("portal service host bootstrap failed")
    }

    pub async fn from_env() -> Result<Self, String> {
        let database = bootstrap_portal_database_from_env().await?;
        let repository = SqlxPortalRepository::new(database.pool().clone());
        Ok(Self {
            portal_service: PortalService::new(repository),
            database,
        })
    }

    pub fn portal_service(&self) -> &PortalService<SqlxPortalRepository> {
        &self.portal_service
    }

    pub fn database_pool(&self) -> &DatabasePool {
        self.database.pool()
    }

    pub fn database_module(&self) -> std::sync::Arc<sdkwork_database_spi::DefaultDatabaseModule> {
        self.database.module()
    }
}

pub fn default_seed_locale() -> &'static str {
    "zh-CN"
}

pub fn default_seed_profile() -> &'static str {
    "standard"
}
