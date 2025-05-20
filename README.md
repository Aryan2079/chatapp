| File         | Responsibility                                                                  |
| ------------ | ------------------------------------------------------------------------------- |
| `main.rs`    | Entry point. Sets up state, starts server loop. Very thin.                      |
| `server.rs`  | Binds to a port (e.g., 3030), accepts WS connections, spawns `client.rs`.       |
| `client.rs`  | Handles per-client logic: read/write loop, send/receive messages, notify state. |
| `message.rs` | Defines `Message`, `ClientMessage`, etc. (JSON-serde structs).                  |
| `state.rs`   | Defines the `AppState` structure holding `DashMap`s (registries, buffers).      |
