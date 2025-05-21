mod server;
mod state;
mod client;

#[tokio::main]
async fn main() {
    server::start().await;
}
