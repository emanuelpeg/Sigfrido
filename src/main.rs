mod gossip;
mod message;
mod node;

use gossip::start_gossip;
use node::Node;

#[tokio::main]
async fn main() {

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("cargo run -- <PORT> [PEER]");
        return;
    }

    let port = &args[1];

    let address = format!("127.0.0.1:{}", port);

    let peers = if args.len() > 2 {
        vec![args[2].clone()]
    } else {
        vec![]
    };

    let node = Node::new(address, peers);

    start_gossip(node).await;
}