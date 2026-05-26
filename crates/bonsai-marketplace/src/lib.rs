use libp2p::{
    kad::{Kademlia, store::MemoryStore},
    mdns,
    identity,
    swarm::NetworkBehaviour,
};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use tokio::sync::mpsc;

#[derive(Clone, Serialize, Deserialize)]
pub struct Asset {
    pub cid: String,
    pub asset_type: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub tags: Vec<String>,
    pub publisher: Vec<u8>,
    pub signature: Vec<u8>,
    pub size_bytes: u64,
}

#[derive(NetworkBehaviour)]
struct MarketBehaviour {
    kademlia: Kademlia<MemoryStore>,
    mdns: mdns::tokio::Behaviour,
}

pub struct Marketplace {
    // swarm omitted for brevity
    assets: HashMap<String, Asset>,
    event_tx: mpsc::Sender<MarketEvent>,
}

#[derive(Debug)]
pub enum MarketEvent {
    AssetDiscovered(Asset),
    AssetInstalled(String),
}

impl Marketplace {
    pub async fn new() -> Result<(Self, mpsc::Receiver<MarketEvent>), Box<dyn std::error::Error>> {
        let id = identity::Keypair::generate_ed25519();
        let peer_id = libp2p::PeerId::from(id.public());
        // For simplicity we do not build a full swarm here
        let (tx, rx) = mpsc::channel(256);
        Ok((Self { assets: HashMap::new(), event_tx: tx }, rx))
    }

    pub async fn publish(&mut self, asset: Asset) {
        self.assets.insert(asset.cid.clone(), asset.clone());
        // In real code: announce via DHT
    }

    pub async fn search(&self, query: &str) -> Vec<Asset> {
        self.assets
            .values()
            .filter(|a| a.tags.iter().any(|t| t.contains(query)) || a.name.contains(query))
            .cloned()
            .collect()
    }
}
