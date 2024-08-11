use config::{Config, Environment};
use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct RustArchConfig {
    /// (required) Fully qualified URL for accessing Postgres server.
    /// e.g. postgres://user:password@host/database
    pub database_url: String,
}

impl RustArchConfig {
    pub fn load() -> Result<RustArchConfig, Error> {
        let config = Config::builder()
            .add_source(Environment::with_prefix("RUST_ARCH").try_parsing(true).separator("__"))
            .build()?;

        let config: RustArchConfig = config.try_deserialize()?;

        Ok(config)
    }
}
