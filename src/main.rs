use std::io;
use tokio::sync::Mutex;
use std::sync::Arc;

mod parser;
mod command;
mod server;

use crate::server::UserRegistry;
use crate::server::Server;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = Server {
        users: Arc::new(Mutex::new(UserRegistry { counter: 0, users: vec![] })),
    };

    server.listen().await
}
