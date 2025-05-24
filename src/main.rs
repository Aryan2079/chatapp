mod server;
mod state;
mod client;
mod message;

#[tokio::main]
async fn main() {
    server::start().await;
}
