#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),

    #[error(transparent)]
    DatabaseError(#[from] arch_db::Error),

    #[error("Not found")]
    NotFound,
}
