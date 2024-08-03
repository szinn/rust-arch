use async_trait::async_trait;

use crate::HealthService;

#[derive(Clone)]
pub(crate) struct HealthServiceImpl {}

#[async_trait]
impl HealthService for HealthServiceImpl {
    #[tracing::instrument(level = "trace", skip(self))]
    async fn is_healthy(&self) -> bool {
        true
    }
}
