//! TransferDaemon v2 – Self-Certifying Identity (Iroh-inspired)
//!
//! This module provides cryptographic identity for TransferDaemon v2:
//! - Self-certifying NodeId: the peer's identity IS its public key
//! - DIDs + Verifiable Credentials for Self-Sovereign Identity (SSI)
//! - No external PKI; fully sovereign

pub mod node_id;
pub mod did;
pub mod vc;

pub use node_id::NodeId;
pub use did::DidDocument;
pub use vc::VerifiableCredential;
