# Sigfrido

**Sigfrido** is a lightweight and experimental Gossip Protocol implementation written in Rust, designed for decentralized cluster membership, node discovery, and failure detection in distributed systems.

The project is inspired by the internal communication mechanisms used in distributed systems like Apache Cassandra, where nodes exchange state information through peer-to-peer gossip instead of relying on a centralized coordinator.

Sigfrido aims to provide a simple, modular, and educational foundation for understanding and building distributed infrastructures in Rust.

---

# Features

- Gossip-based peer-to-peer communication
- Decentralized node membership management
- Heartbeat propagation
- Failure detection
- Cluster state synchronization
- Lightweight and efficient architecture
- Async networking with Rust ecosystem tools
- Modular design for experimentation and extension

---

# Goals

The main objective of Sigfrido is to explore the core ideas behind distributed coordination systems while keeping the implementation understandable and hackable.

This project is intended to serve as:

- A learning platform for distributed systems concepts
- A reusable gossip layer for future distributed projects
- A playground for experimenting with scalability and fault tolerance
- The networking foundation for a future distributed database

---

# Planned Features

- Phi Accrual Failure Detector
- Seed node support
- Anti-entropy synchronization
- SWIM-inspired optimizations
- Distributed metadata propagation
- Cluster partition handling
- Metrics and observability
- TCP and UDP transport layers
- TLS support
- Persistent node state

---

# Why Rust?

Rust is a strong fit for distributed infrastructure software because it offers:

- Memory safety without garbage collection
- High-performance networking
- Fearless concurrency
- Strong compile-time guarantees
- Low-level control with modern ergonomics

Sigfrido leverages these capabilities to build a reliable and efficient gossip communication layer.

---

# Project Status

🚧 Early development stage.

The protocol, APIs, and internal structures are still evolving and may change frequently as the project grows.

---

# Vision

Sigfrido is the first building block toward creating larger distributed systems in Rust, including eventually consistent databases, clustered services, and fault-tolerant infrastructures.

The long-term vision is to provide a minimal but powerful distributed systems toolkit that developers can study, extend, and build upon.
