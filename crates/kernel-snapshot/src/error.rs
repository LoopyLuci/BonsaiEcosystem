//! Kernel snapshot error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum KernelError {
    #[error("Vault not found: {0}")]
    VaultNotFound(u64),

    #[error("Memory access failed: {0}")]
    MemoryAccessFailed(String),

    #[error("Snapshot creation failed: {0}")]
    SnapshotCreationFailed(String),

    #[error("Snapshot restoration failed: {0}")]
    SnapshotRestorationFailed(String),

    #[error("Capability table error: {0}")]
    CapabilityTableError(String),

    #[error("CAS operation failed: {0}")]
    CasError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("System error: {0}")]
    SystemError(String),

    #[error("Invalid state: {0}")]
    InvalidState(String),
}

pub type Result<T> = std::result::Result<T, KernelError>;
