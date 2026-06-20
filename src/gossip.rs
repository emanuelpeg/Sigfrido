use rand::seq::IndexedRandom;
use tokio::{
    net::UdpSocket,
    time::{sleep, Duration},
};

use std::time::Instant;

use crate::{
    message::GossipMessage,
    message::NodeState,
    node::Node,
    membership::{
        MemberStatus,
        Member
    }
};

pub async fn start_gossip(node: Node) {
    let socket = UdpSocket::bind(&node.address)
        .await
        .expect("Failed to bind UDP socket");

    println!("Node {} listening on {}", node.id, node.address);

    let socket = std::sync::Arc::new(socket);
    let receiver_membership = node.membership.clone();
   
    let detector_membership = node.membership.clone();
    
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
                    serde_json::from_slice(&buf[..len]).unwrap();

                let mut membership =
                    node.membership.lock().await;

                for state in msg.states {

                    match membership.get_mut(&state.node_id) {

                        Some(member) => {
                            if state.heartbeat > member.heartbeat {
                                member.last_seen = Instant::now();
                            }
                            member.heartbeat = state.heartbeat;
                            member.status = state.status;
                        }
                        _ => {

                            membership.insert(
                                state.node_id.clone(),
                                Member {
                                    node_id: state.node_id.clone(),
                                    heartbeat: state.heartbeat,
                                    last_seen: Instant::now(),
                                    status: MemberStatus::Alive,
                                },
                            );
                            
                        }
                    }
                }

                println!(
                    "Received gossip from {}",
                    addr
                );
            }
        });
    }


    tokio::spawn(async move {

        loop {

            {
                let mut membership =
                    detector_membership.lock().await;

                for member in membership.values_mut() {

                    let elapsed =
                        member.last_seen.elapsed();

                    if elapsed.as_secs() > 5 {
                        member.status = MemberStatus::Suspect;
                    }

                    if elapsed.as_secs() > 15 {
                        member.status = MemberStatus::Dead;
                    }

                }

                for member in membership.values() {
                    println!(
                        "Node {} -> heartbeat={} status={} last_seen={} seconds ago",
                        member.node_id,
                        member.heartbeat,
                        member.status, 
                        member.last_seen.elapsed().as_secs()
                    );
                }

            }

            sleep(Duration::from_secs(1)).await;

        }

    });

    // Sender
    loop {

        {
            let mut membership = receiver_membership.lock().await;

            let myself = membership
                .get_mut(&node.id)
                .unwrap();

            myself.heartbeat += 1;
            myself.last_seen = Instant::now();

        } // <- acá se libera el lock

        let peer = {
            let peers = node.peers.lock().await;
            peers.choose(&mut rand::rng()).cloned()
        };

        if let Some(peer) = peer {

            let states = {
                let membership = receiver_membership.lock().await;

                membership.values()
                    .map(|m| NodeState {
                        node_id: m.node_id.clone(),
                        heartbeat: m.heartbeat,
                        status: m.status.clone(),
                    })
                    .collect::<Vec<_>>()
            };

            let msg = GossipMessage {
                states
            };

            let bytes = serde_json::to_vec(&msg).unwrap();

            socket.send_to(&bytes, &peer)
                .await
                .unwrap();

            println!(
                "Sent gossip to {} => {:?}",
                peer,
                msg
            );
        }

        sleep(Duration::from_secs(2)).await;
    }
}