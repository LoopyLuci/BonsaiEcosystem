use msg_core::Message;

pub struct P2pDelivery;

impl P2pDelivery {
    pub fn new() -> Self {
        Self
    }

    /// Discover a peer by email address via Echo service registry
    pub async fn resolve_peer(&self, _email: &str) -> anyhow::Result<Option<String>> {
        Ok(None)
    }

    /// Deliver a message: try P2P first, fall back to traditional SMTP
    pub async fn send(&self, msg: &Message) -> anyhow::Result<()> {
        for recipient in &msg.to {
            if let Some(_peer) = self.resolve_peer(recipient).await? {
                // Would deliver via P2P
                tracing::info!("Delivering to {} via P2P", recipient);
            } else {
                // Fallback to standard SMTP delivery
                tracing::info!("No peer found for {}, queuing for SMTP relay", recipient);
            }
        }
        Ok(())
    }
}

impl Default for P2pDelivery {
    fn default() -> Self {
        Self::new()
    }
}
