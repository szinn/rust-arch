use std::{sync::Arc, time::Duration};

use arch_api::start_http;
use arch_core::create_service;

use anyhow::Result;
use tokio_graceful_shutdown::{SubsystemBuilder, Toplevel};

mod logging;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init_logging()?;

    let crate_version = clap::crate_version!();
    let git_revision = env!("BUILD_GIT_HASH");

    let arch_service = Arc::new(create_service());
    tracing::info!("RustArch {}-{}", crate_version, git_revision);
    tracing::info!("Healthy={}", arch_service.health_service.is_healthy().await);

    let server = Toplevel::new(|s| async move {
        s.start(SubsystemBuilder::new("http_api", |h| start_http(3000, arch_service, h)));
    })
    .catch_signals()
    .handle_shutdown_requests(Duration::from_secs(5));

    server.await?;

    Ok(())
}
