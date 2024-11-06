pub mod query;
pub use query::query;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EnvError(#[from] dotenvy::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    ReqError(#[from] reqwest::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct VideoQuery {
    pub name: String,
    pub by: String,
    pub id: String,
}

