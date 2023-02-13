use std::io;
use std::sync::Arc;

mod parser;
mod command;
mod server;

use crate::server::Server;

#[tokio::main]
async fn main() -> io::Result<()> {
    Arc::new(Server::new())
        .listen()
        .await
}
