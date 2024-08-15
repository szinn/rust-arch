use sea_orm::DbErr;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SeaOrm(#[from] DbErr),

    #[error(transparent)]
    Any(#[from] Box<dyn std::error::Error + Send + Sync>),
}
