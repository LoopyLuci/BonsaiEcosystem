use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransferClientError {
    #[error("peer not found: {0}")]
    PeerNotFound(String),

    #[error("failed to connect to peer {peer}: {reason}")]
    ConnectionFailed { peer: String, reason: String },

    #[error("failed to open stream '{name}' on peer {peer}: {reason}")]
    StreamOpenFailed {
        peer: String,
        name: String,
        reason: String,
    },

    #[error("send failed on stream '{name}': {reason}")]
    SendError { name: String, reason: String },

    #[error("receive failed on stream '{name}': {reason}")]
    ReceiveError { name: String, reason: String },

    #[error("relay error: {0}")]
    RelayError(String),

    #[error("timeout after {0}ms")]
    Timeout(u64),

    #[error("invalid configuration: {0}")]
    ConfigError(String),

    #[error("transport not available: {0}")]
    TransportUnavailable(String),

    #[error("internal error: {0}")]
    Internal(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
