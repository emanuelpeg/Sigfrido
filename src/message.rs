use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GossipMessage {
    pub node_id: String,
    pub heartbeat: u64,
}