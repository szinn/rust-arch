use std::sync::Arc;

use arch_db::DatabaseRepository;
use async_trait::async_trait;

#[async_trait]
pub trait HealthService: Send + Sync {
    async fn is_healthy(&self) -> bool;
}

#[derive(Clone)]
pub(crate) struct HealthServiceImpl {
    repository: Arc<DatabaseRepository>,
}

impl HealthServiceImpl {
    pub(crate) fn new(repository: Arc<DatabaseRepository>) -> Self {
        Self { repository }
    }
}
#[async_trait]
impl HealthService for HealthServiceImpl {
    #[tracing::instrument(level = "trace", skip(self))]
    async fn is_healthy(&self) -> bool {
        let db_healthy = !self.repository.database.get_postgres_connection_pool().is_closed();

        db_healthy
    }
}
