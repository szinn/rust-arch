use std::sync::Arc;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
pub use sea_orm_migration::prelude::*;

pub mod entities;
pub mod error;
pub use error::*;
use tracing_log::log;

mod m20220101_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220101_000001_create_table::Migration)]
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

    let database = Database::connect(opt).await.map_err(handle_dberr)?;
    Migrator::up(&database, None).await?;

    let database = Arc::new(Repository { database });
    tracing::debug!("...connected to database");

    Ok(database)
}

pub async fn run_migration_cli() {
    cli::run_cli(Migrator).await
}
