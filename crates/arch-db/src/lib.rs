use std::sync::Arc;

use migration::apply_migrations;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing_log::log;

pub mod entities;
pub mod error;
pub mod migration;

pub use error::*;

pub struct DatabaseRepository {
    pub database: DatabaseConnection,
}

pub async fn create_database_connection(url: &str) -> Result<Arc<DatabaseRepository>, Error> {
    tracing::debug!("Connecting to database...");
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let database = Database::connect(opt).await.map_err(handle_dberr)?;
    apply_migrations(&database).await?;

    let database = Arc::new(DatabaseRepository { database });

    Ok(database)
}
