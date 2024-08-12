use arch_db::create_database_connection;
use arch_utils::{arcbox, arcbox::ArcBox};
use error::Error;
use health::HealthServiceImpl;

pub mod error;

mod health;
pub use health::HealthService;

pub struct ArchService {
    pub health_service: ArcBox<dyn HealthService>,
}

#[tracing::instrument(level = "trace", skip(database_url))]
pub async fn create_service(database_url: &str) -> Result<ArchService, Error> {
    let repository = create_database_connection(database_url).await?;

    let health_service = HealthServiceImpl::new(repository.clone());
    let health_service: ArcBox<dyn HealthService> = arcbox!(health_service);

    Ok(ArchService { health_service })
}
