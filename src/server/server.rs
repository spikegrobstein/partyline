use std::io;
use std::sync::Arc;
use std::net::SocketAddr;

use tokio::sync::Mutex;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use tokio_util::codec::Framed;

use futures::{StreamExt, SinkExt};

use crate::server::UserRegistry;
use crate::server::User;
use crate::server::CmdCodec;

pub struct Server {
    pub users: Arc<Mutex<UserRegistry>>,
}

impl Server {
    pub async fn listen(&self) -> io::Result<()> {
        let addr = "127.0.0.1:9999";
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (socket, addr) = listener.accept().await?;

            let registry = self.users.clone();

            tokio::spawn(async move {
                handle_connection(registry, socket, addr).await;
            });
        }
    }

}

async fn handle_connection(registry: Arc<Mutex<UserRegistry>>, socket: TcpStream, addr: SocketAddr) {
    println!("got connection from {addr}");

    {
        let mut registry = registry.lock().await;

        let new_user = User {
            id: registry.next_id(),
            name: "anonymous".to_owned(),
            addr,
        };

        registry.users.push(new_user);
    }

    let mut framed_stream = Framed::new(socket, CmdCodec);

    loop {
        match framed_stream.next().await {
            Some(Err(err)) => {
                eprintln!("got an error... ignoring... {:#?}", err);
            },
            Some(Ok(cmd)) => {
                println!("got cmd: {}", cmd.command);
                let resp = match cmd.command.as_ref() {
                    "echo" => {
                        if cmd.args.is_empty() {
                            ">> Expected args".to_owned()
                        } else {
                            cmd.args.join(" ")
                        }
                    },
                    "news" => {
                        "no news.".to_owned()
                    },
                    unknown => {
                        format!(">> Unknown command {unknown}")
                    }
                };
                framed_stream.send(&resp).await.unwrap();
            },
            None => {
                eprintln!("got nothin'");
            },
        }
    }
}

