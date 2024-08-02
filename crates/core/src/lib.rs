use async_trait::async_trait;
use health::HealthServiceImpl;

mod health;

#[async_trait]
pub trait HealthService: Send + Sync {
    async fn is_healthy(&self) -> bool;
}

pub struct ArchService {
    pub health_service: Box<dyn HealthService>,
}

pub fn create_service() -> ArchService {
    let health_service = Box::new(HealthServiceImpl {});

    ArchService { health_service }
}
