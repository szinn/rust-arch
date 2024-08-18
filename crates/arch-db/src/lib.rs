use std::sync::Arc;

use adapters::{item::ItemAdapterImpl, ItemAdapter};
use arch_utils::{arcbox, arcbox::ArcBox};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
pub use sea_orm_migration::prelude::*;

pub mod adapters;
pub mod entities;
pub mod error;
pub use error::*;
use tracing_log::log;

mod m20240815_124028_create_items;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240815_124028_create_items::Migration)]
    }
}

pub struct Repository {
    pub database: DatabaseConnection,
}

pub struct RepositoryAdapters {
    pub repository: Arc<Repository>,
    pub item_adapter: ArcBox<dyn ItemAdapter>,
}

pub async fn connect_database(url: &str) -> Result<Arc<RepositoryAdapters>, Error> {
    tracing::debug!("Connecting to database...");
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let database = Database::connect(opt).await?;
    Migrator::up(&database, None).await?;
    tracing::debug!("...connected to database");

    let repository = Arc::new(Repository { database });

    let item_adapter = ItemAdapterImpl::new(repository.clone());
    let item_adapter: ArcBox<dyn ItemAdapter> = arcbox!(item_adapter);

    let adapters = Arc::new(RepositoryAdapters { repository, item_adapter });

    Ok(adapters)
}

pub async fn run_migration_cli() {
    cli::run_cli(Migrator).await
}
