use std::{sync::Arc, time::Duration};

use arch_api::http::start_server;
use arch_core::create_service;

use anyhow::{Context, Result};
use rust_arch::{
    args::{self, Args},
    config::RustArchConfig,
    logging,
};
use tokio_graceful_shutdown::{SubsystemBuilder, Toplevel};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = clap::Parser::parse();

    let config = RustArchConfig::load().context("Cannot load configuration")?;

    match args.cmd {
        args::Subcommands::Server => {
            logging::init_logging()?;

            let crate_version = clap::crate_version!();
            let git_revision = env!("BUILD_GIT_HASH");
            tracing::info!("RustArch {}-{}", crate_version, git_revision);

            let arch_service = Arc::new(create_service(&config.database_url));

            let server = Toplevel::new(|s| async move {
                s.start(SubsystemBuilder::new("http_api", |h| start_server(3000, arch_service, h)));
            })
            .catch_signals()
            .handle_shutdown_requests(Duration::from_secs(5));

            server.await?;
        }
    }

    Ok(())
}
