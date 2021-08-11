mod biggest;
mod dir_info;
mod server;
extern crate dirs;

#[tokio::main]
async fn main() {
    server::serve().await
}
