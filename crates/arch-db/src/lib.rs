use std::sync::Arc;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
pub use sea_orm_migration::prelude::*;

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

pub async fn connect_database(url: &str) -> Result<Arc<Repository>, Error> {
    tracing::debug!("Connecting to database...");
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let database = Database::connect(opt).await?;
    Migrator::up(&database, None).await?;
    tracing::debug!("...connected to database");

    Ok(Arc::new(Repository { database }))
}

pub async fn run_migration_cli() {
    cli::run_cli(Migrator).await
}
