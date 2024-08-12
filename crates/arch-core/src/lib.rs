use arch_utils::{arcbox, arcbox::ArcBox};
use health::HealthServiceImpl;

mod health;
pub use health::HealthService;

pub struct ArchService {
    pub health_service: ArcBox<dyn HealthService>,
}

#[tracing::instrument(level = "trace", skip(_database_url))]
pub fn create_service(_database_url: &str) -> ArchService {
    let health_service = HealthServiceImpl::new();
    let health_service: ArcBox<dyn HealthService> = arcbox!(health_service);

    ArchService { health_service }
}
