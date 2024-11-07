pub mod query;
pub mod lev;
pub use lev::lev;
pub use query::query;
pub use std::path::PathBuf;

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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct VideoQuery {
    pub name: String,
    pub by: String,
    pub id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DownloadedVideo {
    pub id: String,
    pub by: String,
    pub name: String,
    pub path: PathBuf,
}

