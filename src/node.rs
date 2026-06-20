use std::{
    collections::HashMap,
    sync::Arc,
};

use tokio::sync::Mutex;
use uuid::Uuid;
use std::time::Instant;
use crate::membership::{
    Member, MemberStatus
};

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub address: String,

    pub peers: Arc<Mutex<Vec<String>>>,

    pub membership: Arc<Mutex<HashMap<String, Member>>>,
}

impl Node {
    pub fn new(address: String, peers: Vec<String>) -> Self {
        let id = Uuid::new_v4().to_string();

        let mut membership = HashMap::new();

        membership.insert(
            id.clone(),
            Member {
                node_id: id.clone(),
                heartbeat: 0,
                last_seen: Instant::now(),
                status: MemberStatus::Alive,
            },
        );

        Self {
            id,
            address,
            peers: Arc::new(Mutex::new(peers)),
            membership: Arc::new(Mutex::new(membership)),
        }
    }
}