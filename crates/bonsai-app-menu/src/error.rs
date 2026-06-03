use thiserror::Error;

#[derive(Error, Debug)]
pub enum MenuError {
    #[error("App not found: {0}")]
    AppNotFound(String),

    #[error("Launch failed: {0}")]
    LaunchFailed(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Discovery error: {0}")]
    DiscoveryError(String),
}

pub type Result<T> = std::result::Result<T, MenuError>;
