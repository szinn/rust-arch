use async_trait::async_trait;

use crate::HealthService;

pub(crate) struct HealthServiceImpl {}

#[async_trait]
impl HealthService for HealthServiceImpl {
    async fn is_healthy(&self) -> bool {
        true
    }
}
