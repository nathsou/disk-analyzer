mod biggest;
mod dir_info;
mod server;

#[tokio::main]
async fn main() {
    server::serve().await
}
