use std::io;

mod parser;
mod command;
mod server;

use crate::server::Server;

#[tokio::main]
async fn main() -> io::Result<()> {
    Server::new()
        .listen()
        .await
}
