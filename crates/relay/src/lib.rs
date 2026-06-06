//! Blind relay client/server for Bonsai peer-to-peer transfer.
//!
//! The relay is "blind": it sees only encrypted `ChunkCiphertext` blobs and
//! forwards them by session token without inspecting content.
//!
//! Protocol:
//!   1. Peer A registers a session token with the relay (signed, PoW-protected).
//!   2. Peer B connects using the same token.
//!   3. Relay forwards chunks between the two TCP connections until either
//!      peer disconnects or the session TTL expires.

pub mod client;
pub mod error;
pub mod server;
pub mod token;

pub use client::RelayClient;
pub use error::{RelayError, RelayResult};
pub use server::RelayServer;
pub use token::RelayToken;
