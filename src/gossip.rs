use rand::seq::IndexedRandom;
use tokio::{
    net::UdpSocket,
    time::{sleep, Duration},
};

use crate::{
    message::GossipMessage,
    node::Node,
};

pub async fn start_gossip(node: Node) {
    let socket = UdpSocket::bind(&node.address)
        .await
        .expect("Failed to bind UDP socket");

    println!("Node {} listening on {}", node.id, node.address);

    let socket = std::sync::Arc::new(socket);

    // Receiver
    {
        let socket = socket.clone();

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];

            loop {
                let (len, addr) = socket.recv_from(&mut buf)
                    .await
                    .expect("Failed to receive");

                let msg: GossipMessage =
                    serde_json::from_slice(&buf[..len])
                        .expect("Invalid message");

                println!(
                    "Received gossip from {} => {:?}",
                    addr,
                    msg
                );
            }
        });
    }

    // Sender
    let mut heartbeat = 0u64;

    loop {
        heartbeat += 1;

        let peers = node.peers.lock().await;

        if let Some(peer) = peers.choose(&mut rand::rng()) {
            let msg = GossipMessage {
                node_id: node.id.clone(),
                heartbeat,
            };

            let bytes = serde_json::to_vec(&msg)
                .expect("Serialization failed");

            socket.send_to(&bytes, peer)
                .await
                .expect("Failed to send");

            println!(
                "Sent gossip to {} => {:?}",
                peer,
                msg
            );
        }

        sleep(Duration::from_secs(2)).await;
    }
}