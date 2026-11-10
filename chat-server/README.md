# Async Broadcast Chat Server

A multi-client TCP broadcast chat server built in Rust with the [Tokio](https://tokio.rs/) asynchronous runtime.

The server listens on `127.0.0.1:5050` and acts as a centralized relay. When a client sends a message, it is broadcast to every other connected peer in real time while suppressing echoes back to the sender. It includes defensive timeouts against hanging idle connections and supports clean server-side termination.

---

## Core Features

- **Multi-Client Broadcast Engine**: Uses `tokio::sync::broadcast` (buffer capacity: 128) to distribute messages across independent async client tasks without lock contention.
- **Selective Echo Suppression**: Filters broadcast messages by sender IP/port prefix so users do not receive their own sent messages reflected back to them.
- **Inactivity & Slowloris Defense**: Wraps client reads in a strict 30-second timeout (`tokio::time::timeout`). Connections that hold open sockets without transmitting newline-terminated data are safely disconnected to conserve server resources.
- **Stream Half-Splitting**: Splits incoming TCP streams into isolated reader and writer halves (`stream.into_split()`), allowing simultaneous full-duplex reading and writing without mutex locks.
- **Graceful Shutdown**: Catches `SIGINT` (`Ctrl+C`) through `tokio::signal::ctrl_c()`. Upon trigger, the server issues a shutdown notification across all channels, waits 200 ms to flush buffers, and terminates cleanly.
- **Presence Notifications**: Automatically broadcasts formatted join (`[+]`) and leave (`[-]`) notices to the room when clients connect or drop.

---

## Architecture Flow

```
              ┌─────────────────────────────────────────┐
              │           tokio::select! Loop           │
              │                                         │
Incoming ────>│ Branch 1: Inactivity-Guarded Read Line   ├─(broadcast::Sender)──┐
TCP Socket    │           (30s Slowloris Timeout)       │                      │
              │                                         │                      │
Outgoing <────┤ Branch 2: Broadcast Receiver (rx.recv)   │<────────────────────┘
TCP Socket    │           (Suppresses self-addressed)   │ (Relayed to all peers)
              └─────────────────────────────────────────┘
```

---

## Configuration

Default constants defined in `src/main.rs`:

| Parameter | Value | Description |
| :--- | :--- | :--- |
| `bind_addr` | `127.0.0.1:5050` | TCP bind address and port |
| `MAX_CLIENTS` | `128` | Maximum broadcast channel queue capacity |
| `CLIENTS_TIMEOUT_SECS` | `30` | Inactivity threshold before dropping idle clients |

---

## Building and Running

1. Compile and run the server:
   ```bash
   cargo run
   ```

   Output:
   ```text
   [*] Broadcast server running on 127.0.0.1:5050
   [*] Press Ctrl+C to initiate graceful shutdown
   ```

2. Connect from two or more terminals:
   ```bash
   # Terminal 1
   nc 127.0.0.1 5050

   # Terminal 2
   nc 127.0.0.1 5050
   ```

3. Type in any terminal. Messages will appear in all other connected terminals prefixed by the sender's endpoint address.

---

## Interaction Example

**Client A (`nc 127.0.0.1 5050`):**
```text
[::] Welcome to Rust broadcast Server! [::]
[+] 127.0.0.1:54320 Joined to server
Hello from client A!
[127.0.0.1:54320] Hey client A, loud and clear.
```

**Client B (`nc 127.0.0.1 5050`):**
```text
[::] Welcome to Rust broadcast Server! [::]
[127.0.0.1:54318] Hello from client A!
Hey client A, loud and clear.
```

**Timeout Defense Trigger (Idle Client):**
```text
[!] Disconnected due to inactivity - Slowloris Defend
```

---

## Author

**Piyusha Akash**  
Portfolio: [https://0x3xp.github.io](https://0x3xp.github.io)

---

## Copyright & License

Copyright (c) 2026 Piyusha Akash. All rights reserved.  
Personal portfolio and projects: [0x3xp.github.io](https://0x3xp.github.io)
