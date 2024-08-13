use std::sync::Arc;

use arch_db::DatabaseRepository;
use arch_utils::{arcbox, arcbox::ArcBox};
use error::Error;
use health::HealthServiceImpl;

pub mod error;

mod health;
pub use health::HealthService;

pub struct ArchService {
    pub health_service: ArcBox<dyn HealthService>,
}

#[tracing::instrument(level = "trace", skip(database))]
pub async fn create_service(database: Arc<DatabaseRepository>) -> Result<ArchService, Error> {
    let health_service = HealthServiceImpl::new(database.clone());
    let health_service: ArcBox<dyn HealthService> = arcbox!(health_service);

    Ok(ArchService { health_service })
}
