use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("HOME is not available")]
    MissingHome(#[source] std::env::VarError),
    #[error("failed to read {path}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid json line in {path}:{line}")]
    InvalidJsonLine {
        path: PathBuf,
        line: usize,
        #[source]
        source: serde_json::Error,
    },
    #[error("invalid timestamp `{value}` in {path}:{line}")]
    InvalidTimestamp {
        path: PathBuf,
        line: usize,
        value: String,
        #[source]
        source: time::error::Parse,
    },
    #[error("failed to open sqlite database {path}")]
    SqliteOpen {
        path: PathBuf,
        #[source]
        source: rusqlite::Error,
    },
    #[error("sqlite query failed for {path}")]
    SqliteQuery {
        path: PathBuf,
        #[source]
        source: rusqlite::Error,
    },
    #[error("blocking task failed")]
    Join(#[source] tokio::task::JoinError),
}
