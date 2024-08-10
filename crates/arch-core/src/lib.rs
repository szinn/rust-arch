use std::sync::Arc;

use health::HealthServiceImpl;

mod health;
pub use health::HealthService;

pub struct ArchService {
    pub health_service: Arc<Box<dyn HealthService>>,
}

pub fn create_service() -> ArchService {
    let health_service = HealthServiceImpl::new();
    let health_service = Arc::new(Box::new(health_service) as Box<dyn HealthService>);

    ArchService { health_service }
}
