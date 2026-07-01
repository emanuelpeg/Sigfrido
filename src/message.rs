use serde::{Deserialize, Serialize};
use crate::membership::MemberStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeState {
    pub node_id: String,
    pub address: String,
    pub heartbeat: u64,
    pub status: MemberStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    Gossip(GossipMessage),
    Ping(PingMessage),
    Ack(AckMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GossipMessage {
    pub states: Vec<NodeState>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingMessage {
    pub from: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AckMessage {
    pub from: String,
}