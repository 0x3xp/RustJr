# Async TCP Echo Server

A lightweight, concurrent TCP echo server written in Rust using the [Tokio](https://tokio.rs/) asynchronous runtime.

The server listens for incoming TCP connections on `127.0.0.1:4444`, handles multiple concurrent clients without blocking the main event loop, and mirrors back all received bytes to the sender.

---

## Technical Highlights

- **Non-blocking Concurrency**: Uses `tokio::net::TcpListener` and spawns a separate asynchronous green thread (`tokio::spawn`) for every accepted connection.
- **Buffer Management**: Allocates a 1 KB fixed-size stack buffer (`[0u8; 1024]`) per client loop, avoiding unnecessary heap reallocations during active stream reads.
- **Connection Lifecycle Logging**: Tracks client connection and disconnection events with peer address metadata (`socket.peer_addr()`) and stream byte counts.
- **Clean Disconnect Handling**: Correctly handles EOF states (`read_bytes == 0`) when a client closes its socket.

---

## Architecture Overview

```
Client (nc / telnet)
        │  TCP connection
        ▼
[TcpListener: 127.0.0.1:4444]
        │
        ├──> tokio::spawn(handle_client(socket_1)) ──> Read buffer ──> Write echo
        ├──> tokio::spawn(handle_client(socket_2)) ──> Read buffer ──> Write echo
        └──> tokio::spawn(handle_client(socket_N)) ──> Read buffer ──> Write echo
```

---

## Requirements

- **Rust toolchain** (2024 edition supported, Rust 1.80+)
- **Netcat (`nc`)** or any TCP client for testing

---

## Running the Server

1. Start the server from this directory:
   ```bash
   cargo run
   ```

   Expected output:
   ```text
   Echo server listening on: 127.0.0.1:4444
   ```

2. Open a separate terminal and connect using `nc`:
   ```bash
   nc 127.0.0.1 4444
   ```

3. Type any message and press Enter. The server immediately returns the exact byte stream back to your terminal.

---

## Terminal Demonstration

**Client session:**
```text
$ nc 127.0.0.1 4444
hello world
hello world
ping
ping
```

**Server session:**
```text
Echo server listening on: 127.0.0.1:4444
New connection from 127.0.0.1:51234
Recieved 12 bytes from 127.0.0.1:51234
Recieved 5 bytes from 127.0.0.1:51234
CLient disconnected: 127.0.0.1:51234
```

---

## Author

**Piyusha Akash**  
Portfolio: [https://0x3xp.github.io](https://0x3xp.github.io)

---

## Copyright & License

Copyright (c) 2026 Piyusha Akash. All rights reserved.  
Personal portfolio and projects: [0x3xp.github.io](https://0x3xp.github.io)
