use std::sync::Arc;

use arch_db::Repository;
use arch_domain_api::{ArchApi, HealthApi};
use arch_utils::{arcbox, arcbox::ArcBox};
use error::Error;
use health::HealthService;

mod error;
mod health;

#[tracing::instrument(level = "trace", skip(repository))]
pub async fn create_arch(repository: Arc<Repository>) -> Result<ArchApi, Error> {
    let health_service = HealthService::new(repository.clone());
    let health_api: ArcBox<dyn HealthApi> = arcbox!(health_service);

    Ok(ArchApi { health_api })
}
