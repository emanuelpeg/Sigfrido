use std::{
    collections::{HashMap, HashSet},
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

    pub peers: Arc<Mutex<HashSet<String>>>,

    pub membership: Arc<Mutex<HashMap<String, Member>>>,
}

impl Node {
    pub fn new(address: String, peers: HashSet<String>) -> Self {
        let id = Uuid::new_v4().to_string();

        let mut membership = HashMap::new();

        membership.insert(
            id.clone(),
            Member {
                node_id: id.clone(),
                address: address.clone(),
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