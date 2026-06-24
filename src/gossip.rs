use rand::seq::IteratorRandom;
use tokio::{
    net::UdpSocket,
    time::{sleep, Duration},
};
use tracing::info;

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

    info!("Node {} listening on {}", node.id, node.address);

    let socket = std::sync::Arc::new(socket);
    let receiver_membership = node.membership.clone();
   
    let detector_membership = node.membership.clone();
    let local_address = node.address.clone();
    let receiver_peers = node.peers.clone();
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

                    if state.address != local_address {

                        let mut peers =
                            receiver_peers.lock().await;

                        peers.insert(
                            state.address.clone()
                        );
                    }

                    match membership.get_mut(&state.node_id) {

                        Some(member) => {

                            if should_update(
                                member.heartbeat,
                                &member.status,
                                state.heartbeat,
                                &state.status,
                            ) {
                                member.heartbeat = state.heartbeat;
                                member.status = state.status.clone();
                                member.last_seen = Instant::now();
                            }
                        }
                        _ => {

                            membership.insert(
                                state.node_id.clone(),
                                Member {
                                    node_id: state.node_id.clone(),
                                    address: state.address.clone(),
                                    heartbeat: state.heartbeat,
                                    last_seen: Instant::now(),
                                    status: MemberStatus::Alive,
                                },
                            );                            
                        }
                    }
                }

                info!(
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
                    info!(
                        "Node {} -> heartbeat={} status={} last_seen={} seconds ago",
                        member.address,
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
            peers.iter().choose(&mut rand::rng()).cloned()
        };

        if let Some(peer) = peer {

            let states = {
                let membership = receiver_membership.lock().await;

                membership.values()
                    .map(|m| NodeState {
                        node_id: m.node_id.clone(),
                        address: m.address.clone(),
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

            info!(
                "Sent gossip to {} => {:?}",
                peer,
                msg
            );
        }

        sleep(Duration::from_secs(2)).await;
    }
}

fn should_update(
    current_heartbeat: u64,
    current_status: &MemberStatus,
    incoming_heartbeat: u64,
    incoming_status: &MemberStatus,
) -> bool {

    if incoming_heartbeat > current_heartbeat {
        return true;
    }

    if incoming_heartbeat < current_heartbeat {
        return false;
    }

    incoming_status.priority()
        > current_status.priority()
}