use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Member {
    pub node_id: String,
    pub heartbeat: u64,
    pub last_seen: Instant,
}