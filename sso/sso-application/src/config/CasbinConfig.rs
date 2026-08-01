use std::sync::Arc;
use casbin::{CoreApi, DefaultModel, Enforcer};
use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;
use tracing::info;

const CASBIN_MODEL: &str = include_str!("casbin/casbin_model.conf");

pub struct CasbinConfig;

impl CasbinConfig {
    pub async fn init_enforcer(db: &DatabaseConnection) -> Arc<RwLock<Enforcer>> {
        info!("Initializing Casbin enforcer from database...");

        let model = DefaultModel::from_str(CASBIN_MODEL)
            .await
            .expect("Failed to load Casbin model");

        // Policies are managed entirely in the DB; the adapter reads/writes
        // the casbin_rule table directly, so no static policy file is needed.
        let adapter = sea_orm_adapter::SeaOrmAdapter::new(db.clone())
            .await
            .expect("Failed to create casbin sea-orm adapter");

        let enforcer = Enforcer::new(model, adapter)
            .await
            .expect("Failed to create enforcer");

        Arc::new(RwLock::new(enforcer))
    }
}
