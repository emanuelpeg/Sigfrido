use std::time::Instant;

use serde::{Deserialize, Serialize};

use std::fmt;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MemberStatus {
    Alive,
    Suspect,
    Dead,
}

impl MemberStatus {

    pub fn priority(&self) -> u8 {
        match self {
            MemberStatus::Alive => 0,
            MemberStatus::Suspect => 1,
            MemberStatus::Dead => 2,
        }
    }

}


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
    pub address: String,
    pub heartbeat: u64,
    pub last_seen: Instant,
    pub status: MemberStatus,
}