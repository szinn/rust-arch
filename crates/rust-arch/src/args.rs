#[derive(Debug, clap::Parser)]
#[clap(name = "rust-arch", about = "A CLI for rust exemplar")]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// Start server.
    Server,
}
