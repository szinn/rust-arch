use arch_core::create_service;

use anyhow::Result;

mod logging;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init_logging()?;

    let arch_service = create_service();
    tracing::info!("Hello, world!");
    tracing::info!("Healthy={}", arch_service.health_service.is_healthy().await);

    Ok(())
}
