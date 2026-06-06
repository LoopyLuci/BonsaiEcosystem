//! bonsai-transfer-client — Native TransferDaemon client for peer-to-peer sessions.
//!
//! Provides a high-level async Rust interface for establishing direct peer connections,
//! opening multiplexed streams, and exchanging framed messages over relay or direct P2P transport.
//!
//! # Usage
//!
//! ```ignore
//! use transfer_client::TransferDaemonClient;
//!
//! let client = TransferDaemonClient::new().await?;
//! let session = client.connect_to_peer("peer-id").await?;
//! let stream = client.open_stream(&session, "api.bridge.v1").await?;
//! let response = stream.exchange(b"request").await?;
//! ```

pub mod error;
pub mod framing;
pub mod stream;
pub mod session;
pub mod client;

pub use client::TransferDaemonClient;
pub use error::TransferClientError;
pub use session::PeerSession;
pub use stream::PeerStream;
