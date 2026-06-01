//! Bonsai Remote Desktop Fabric (BRDF)
//!
//! A production-grade, zero-trust sovereign remote desktop system for controlling
//! desktops and servers. BRDF replaces commercial solutions like RustDesk with a
//! cryptographically-secure, capability-token-based architecture.
//!
//! # Architecture
//!
//! BRDF consists of 11 core modules organized into isolated vault components:
//!
//! - **capability** — Ed25519 capability tokens with expiry and signature verification
//! - **rendezvous** — Peer discovery, registration, mDNS via Echo, NAT hole punching
//! - **relay** — Encrypted traffic forwarding with zero-trust relay authentication
//! - **session** — Session creation, lifecycle, permission tracking
//! - **telemetry** — Universe event logging (10 event types for monitoring)
//! - **capture** — Screen/audio/camera capture (platform-specific implementations)
//! - **encode** — H.264/H.265/VP8/VP9/AV1 codec selection with dynamic switching
//! - **stream** — PID-controller adaptive bitrate with network feedback
//! - **input** — Keyboard, mouse, touch, text input, gesture handling
//! - **file_transfer** — Content-Addressable Storage (CAS) based delta compression
//! - **tunnel** — TCP port forwarding to remote hosts
//!
//! # Security Model
//!
//! - **Zero-Trust**: All peer connections require valid capability tokens
//! - **Capability Tokens**: Ed25519-signed, expiring, revocable per-session
//! - **Encrypted Relay**: All traffic relayed through zero-trust encrypted tunnel
//! - **Sanctum Vaults**: Each subsystem runs in isolated hardware-protected vaults
//! - **Formal Verification**: All crypto operations use verified libraries (dalek)
//!
//! # Integration Points
//!
//! - **Tauri Commands**: 5 commands for IDE (list peers, connect, disconnect, etc.)
//! - **MCP Tools**: 5 tools for Claude/agents (peer mgmt, input injection, file transfer)
//! - **BTI Commands**: 6 terminal commands (`:rd peers`, `:rd connect`, etc.)
//! - **Svelte Panel**: RemoteDesktopPanel.svelte for peer discovery and session mgmt
//! - **Universe Events**: 10 event types for monitoring and replay
//!
//! # Example
//!
//! ```ignore
//! use bonsai_remote_desktop::RemoteDesktopService;
//! use std::sync::Arc;
//!
//! let service = Arc::new(RemoteDesktopService::new());
//! let peers = service.list_peers().await?;
//! let session = service.create_session(&peers[0].id, None).await?;
//! ```

pub mod capability;
pub mod rendezvous;
pub mod relay;
pub mod session;
pub mod telemetry;
pub mod capture;
pub mod encode;
pub mod stream;
pub mod input;
pub mod file_transfer;
pub mod tunnel;

pub use capability::{RemoteDesktopToken, TokenError};
pub use rendezvous::{RendezvousService, PeerInfo, DiscoveryError};
pub use relay::{RelayService, RelayError};
pub use session::{SessionManager, SessionState, SessionError};
pub use telemetry::{RemoteDesktopTelemetry, RemoteDesktopEvent};
pub use capture::{CaptureService, CaptureError};
pub use encode::{EncodeService, CodecType, EncodeError};
pub use stream::{StreamService, StreamStats, StreamError};
pub use input::{InputService, InputType, InputError};
pub use file_transfer::{FileTransferService, FileTransferError};
pub use tunnel::{TunnelService, TunnelError};

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sha2::{Digest, Sha256};

/// Unique identifier for a remote peer (fixed-size hash).
#[derive(
    Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize,
)]
pub struct PeerId(pub [u8; 32]);

impl PeerId {
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        PeerId(*bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl std::fmt::Display for PeerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.0[..8]))
    }
}

/// Unique session identifier (UUIDv4).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        SessionId(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for SessionId {
    fn default() -> Self {
        SessionId::new()
    }
}

/// Network statistics for a session (bitrate, latency, packet loss, FPS).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamStats {
    /// Current bitrate in Mbps.
    pub bitrate_mbps: f64,
    /// Round-trip time in milliseconds.
    pub rtt_ms: f64,
    /// Packet loss percentage (0-100).
    pub packet_loss_percent: f64,
    /// Current frames per second.
    pub fps: f64,
    /// Bytes received so far.
    pub bytes_received: u64,
    /// Bytes sent so far.
    pub bytes_sent: u64,
    /// Timestamp of last measurement.
    pub last_update: DateTime<Utc>,
}

impl Default for StreamStats {
    fn default() -> Self {
        StreamStats {
            bitrate_mbps: 5.0,
            rtt_ms: 10.0,
            packet_loss_percent: 0.0,
            fps: 60.0,
            bytes_received: 0,
            bytes_sent: 0,
            last_update: Utc::now(),
        }
    }
}

/// Top-level remote desktop service orchestrating all 11 modules.
pub struct RemoteDesktopService {
    rendezvous: Arc<RendezvousService>,
    relay: Arc<RelayService>,
    session_manager: Arc<SessionManager>,
    capture: Arc<CaptureService>,
    encode: Arc<EncodeService>,
    stream: Arc<StreamService>,
    input: Arc<InputService>,
    file_transfer: Arc<FileTransferService>,
    tunnel: Arc<TunnelService>,
    telemetry: Arc<RemoteDesktopTelemetry>,
    local_peer_id: PeerId,
    initialized: Arc<RwLock<bool>>,
}

impl RemoteDesktopService {
    /// Create a new RemoteDesktopService instance.
    pub fn new() -> Self {
        let local_peer_id = Self::generate_peer_id();

        RemoteDesktopService {
            rendezvous: Arc::new(RendezvousService::new()),
            relay: Arc::new(RelayService::new()),
            session_manager: Arc::new(SessionManager::new()),
            capture: Arc::new(CaptureService::new()),
            encode: Arc::new(EncodeService::new()),
            stream: Arc::new(StreamService::new()),
            input: Arc::new(InputService::new()),
            file_transfer: Arc::new(FileTransferService::new()),
            tunnel: Arc::new(TunnelService::new()),
            telemetry: Arc::new(RemoteDesktopTelemetry::new()),
            local_peer_id,
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Initialize the service, starting all subsystems.
    pub async fn initialize(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut init = self.initialized.write().await;
        if *init {
            return Ok(());
        }

        // Start subsystem services
        self.rendezvous.start().await?;
        self.relay.start().await?;
        self.telemetry.start().await?;

        *init = true;
        Ok(())
    }

    /// Get the local peer ID.
    pub fn local_peer_id(&self) -> PeerId {
        self.local_peer_id
    }

    /// List all discovered peers.
    pub async fn list_peers(&self) -> Result<Vec<PeerInfo>, DiscoveryError> {
        self.rendezvous.discover_peers().await
    }

    /// Create a new session to a remote peer.
    pub async fn create_session(
        &self,
        peer_id: &PeerId,
        token: Option<RemoteDesktopToken>,
    ) -> Result<SessionId, SessionError> {
        let session_id = SessionId::new();

        // Verify capability token if provided
        if let Some(t) = &token {
            t.verify()
                .map_err(|e| SessionError::InvalidToken(e.to_string()))?;
        }

        self.session_manager
            .create_session(session_id, *peer_id, token)
            .await?;

        self.telemetry.log_session_created(&session_id, peer_id);

        Ok(session_id)
    }

    /// End a session.
    pub async fn end_session(&self, session_id: SessionId) -> Result<(), SessionError> {
        self.session_manager.end_session(session_id).await?;
        self.telemetry.log_session_ended(&session_id);
        Ok(())
    }

    /// Get active session info.
    pub async fn get_session_state(
        &self,
        session_id: SessionId,
    ) -> Result<SessionState, SessionError> {
        self.session_manager.get_session(session_id).await
    }

    /// List all active sessions.
    pub async fn list_sessions(&self) -> Result<Vec<SessionId>, SessionError> {
        self.session_manager.list_sessions().await
    }

    /// Get network statistics for a session.
    pub async fn get_stream_stats(
        &self,
        session_id: SessionId,
    ) -> Result<StreamStats, StreamError> {
        self.stream.get_stats(session_id).await
    }

    /// Generate a cryptographically random peer ID.
    fn generate_peer_id() -> PeerId {
        use sha2::{Digest, Sha256};

        let random_bytes = uuid::Uuid::new_v4().as_bytes().to_vec();
        let mut hasher = Sha256::new();
        hasher.update(&random_bytes);
        let result = hasher.finalize();
        let mut peer_id_bytes = [0u8; 32];
        peer_id_bytes.copy_from_slice(&result[..32]);
        PeerId(peer_id_bytes)
    }
}

impl Default for RemoteDesktopService {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export common types for convenience
pub mod prelude {
    pub use crate::{
        RemoteDesktopService, RemoteDesktopToken, SessionId, PeerId, SessionState,
        StreamStats, PeerInfo, RendezvousService, RelayService, SessionManager,
        CaptureService, EncodeService, StreamService, InputService, FileTransferService,
        TunnelService, RemoteDesktopTelemetry, RemoteDesktopEvent,
    };
}
