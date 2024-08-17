use std::sync::Arc;

use arch_db::RepositoryAdapters;
use arch_domain_api::{ArchApi, HealthApi, ItemApi};
use arch_utils::{arcbox, arcbox::ArcBox};
use error::Error;
use health::HealthService;
use item::ItemService;

mod error;
mod health;
mod item;

#[tracing::instrument(level = "trace", skip(repository_adapters))]
pub async fn create_arch(repository_adapters: Arc<RepositoryAdapters>) -> Result<ArchApi, Error> {
    let health_service = HealthService::new(repository_adapters.repository.clone());
    let item_service = ItemService::new(repository_adapters.item_adapter.clone());

    let health_api: ArcBox<dyn HealthApi> = arcbox!(health_service);
    let item_api: ArcBox<dyn ItemApi> = arcbox!(item_service);

    Ok(ArchApi { health_api, item_api })
}
