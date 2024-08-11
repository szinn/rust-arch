use arch_db::migration::run_migration_cli;

#[tokio::main]
async fn main() {
    run_migration_cli().await;
}
