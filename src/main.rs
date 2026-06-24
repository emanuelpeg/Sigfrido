mod gossip;
mod message;
mod node;
mod membership;

use gossip::start_gossip;
use node::Node;
use tracing::info;

use std::collections::HashSet;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        info!("Usage:");
        info!("cargo run -- <PORT> [PEER]");
        return;
    }

    let port = &args[1];

    let address = format!("127.0.0.1:{}", port);

    let peers = if args.len() > 2 {
        args.iter()
        .skip(2)
        .cloned()
        .collect()
    } else {
        HashSet::new()
    };

    let node = Node::new(address, peers);

    start_gossip(node).await;
}