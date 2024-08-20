use server::server::server_router;

mod direct;
mod server;
mod utils;

#[tokio::main]
async fn main() {
    let _ = server_router().await;
}
