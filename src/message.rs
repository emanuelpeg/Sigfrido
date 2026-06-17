use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeState {
    pub node_id: String,
    pub heartbeat: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GossipMessage {
    pub states: Vec<NodeState>,
}