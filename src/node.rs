use std::sync::Arc;

use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub struct Node {
    pub id: String,
    pub address: String,
    pub peers: Arc<Mutex<Vec<String>>>,
}

impl Node {
    pub fn new(address: String, peers: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            address,
            peers: Arc::new(Mutex::new(peers)),
        }
    }
}