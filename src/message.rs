use serde::{Deserialize, Serialize};
use crate::membership::MemberStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeState {
    pub node_id: String,
    pub heartbeat: u64,
    pub status: MemberStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GossipMessage {
    pub states: Vec<NodeState>,
}