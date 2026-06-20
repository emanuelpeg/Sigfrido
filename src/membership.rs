use std::time::Instant;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MemberStatus {
    Alive,
    Suspect,
    Dead,
}

use std::fmt;

impl fmt::Display for MemberStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemberStatus::Alive => write!(f, "Alive"),
            MemberStatus::Suspect => write!(f, "Suspect"),
            MemberStatus::Dead => write!(f, "Dead"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Member {
    pub node_id: String,
    pub heartbeat: u64,
    pub last_seen: Instant,
    pub status: MemberStatus,
}