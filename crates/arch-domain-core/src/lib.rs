use std::sync::Arc;

use arch_db::DatabaseRepository;
use arch_domain_api::{ArchApi, Error, HealthApi};
use arch_utils::{arcbox, arcbox::ArcBox};
use health::HealthService;

mod health;

#[tracing::instrument(level = "trace", skip(database))]
pub async fn create_service(database: Arc<DatabaseRepository>) -> Result<ArchApi, Error> {
    let health_service = HealthService::new(database.clone());
    let health_api: ArcBox<dyn HealthApi> = arcbox!(health_service);

    Ok(ArchApi { health_api })
}
