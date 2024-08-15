use std::sync::Arc;

use arch_db::Repository;
use arch_domain_api::HealthApi;
use async_trait::async_trait;

#[derive(Clone)]
pub(crate) struct HealthService {
    repository: Arc<Repository>,
}

impl HealthService {
    pub(crate) fn new(repository: Arc<Repository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
impl HealthApi for HealthService {
    #[tracing::instrument(level = "trace", skip(self))]
    async fn is_healthy(&self) -> bool {
        let db_healthy = !self.repository.database.get_postgres_connection_pool().is_closed();

        db_healthy
    }
}
