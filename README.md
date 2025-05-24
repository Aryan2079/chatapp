# 🧪 Chat App Prototype (Rust + WebSocket, No UI)

This is a local-only prototype of a scalable chat backend written in **Rust** using **WebSockets**, built to explore the architecture for:

- ✅ 1-to-1 messaging  
- ✅ Group messaging  
- ✅ Pub-sub (broadcast / topic-based messaging)  
- 🕓 Missed-message buffering (planned)  
- 🧪 No frontend – tested via `websocat` or custom clients  

This project is meant to be a **prototype** for a future chat app built with Tauri + WebRTC, but this version focuses entirely on the **backend communication model**.

---

## 📁 Project Structure

| File         | Responsibility                                                                  |
| ------------ | ------------------------------------------------------------------------------- |
| `main.rs`    | Entry point. Sets up state, starts server loop. Very thin.                      |
| `server.rs`  | Binds to a port (e.g., 3030), accepts WS connections, spawns `client.rs`.       |
| `client.rs`  | Handles per-client logic: read/write loop, send/receive messages, notify state. |
| `message.rs` | Defines `Message`, `ClientMessage`, etc. (JSON-serde structs).                  |
| `state.rs`   | Defines the `AppState` structure holding `DashMap`s (registries, buffers).      |

---

## 🚀 How to Run

### 1. Install dependencies

```bash
cargo build
```

### 2. Start the server

```bash
cargo run
```

This starts a WebSocket server on `ws://127.0.0.1:3030/ws`.

---

## 💬 How to Test It (No UI)

Use [`websocat`](https://github.com/vi/websocat) to simulate WebSocket clients.

### Install websocat

```bash
cargo install websocat
```

### Start two clients in separate terminals:

**Client A:**

```bash
websocat ws://127.0.0.1:3030/ws
```

**Client B:**

```bash
websocat ws://127.0.0.1:3030/ws
```

### Send a 1-to-1 message:

In **client-a** terminal:

```json
{"from":"client-a","to":"client-b","msg":"Hello from A!"}
```

In **client-b**, you'll receive:

```json
{"from":"client-a","to":"client-b","msg":"Hello from A!"}
```

---

## 🗃 AppState Internals

The app uses in-memory registries for fast, scalable message routing:

| Component        | Description                                           |
| ---------------- | ----------------------------------------------------- |
| `ClientRegistry` | Maps `ClientId → SenderChannel`. (for 1-to-1)         |
| `GroupRegistry`  | Maps `GroupId → HashSet<ClientId>`. (for group chats) |
| `TopicRegistry`  | Maps `Topic → HashSet<ClientId>`. (for pub-sub)       |
| `MessageBuffer`  | Maps `ClientId → Vec<Message>`. (for missed messages) |

All state is shared via `Arc<AppState>` and concurrent-safe via `DashMap`.

---

## 🧠 Future Ideas (Not Yet Implemented)

- Client auth and persistent ID mapping  
- Storing missed messages when recipient is offline  
- Group membership persistence  
- Web UI or Tauri frontend  
- WebRTC-based P2P messaging layer  

---

## 🔧 Tech Stack

- [`tokio`](https://tokio.rs/) — async runtime  
- [`warp`](https://docs.rs/warp) — lightweight HTTP/WebSocket server  
- [`futures`](https://docs.rs/futures) — async combinators  
- [`serde`](https://serde.rs/) — JSON (de)serialization  
- [`dashmap`](https://docs.rs/dashmap) — concurrent hashmap  

---

## 🙌 Credits

Built as a learning/prototype project to explore scalable chat architecture using Rust. Feel free to fork and adapt.
