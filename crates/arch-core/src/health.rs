use async_trait::async_trait;

#[async_trait]
pub trait HealthService: Send + Sync {
    async fn is_healthy(&self) -> bool;
}

#[derive(Clone)]
pub(crate) struct HealthServiceImpl {}

impl HealthServiceImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
#[async_trait]
impl HealthService for HealthServiceImpl {
    #[tracing::instrument(level = "trace", skip(self))]
    async fn is_healthy(&self) -> bool {
        true
    }
}
