use anyhow::Result;

mod logging;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init_logging()?;

    tracing::info!("Hello, world!");

    Ok(())
}
