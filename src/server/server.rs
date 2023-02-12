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

    let user_id = {
        let mut registry = registry.lock().await;

        let id = registry.next_id();

        let new_user = User {
            id,
            name: "anonymous".to_owned(),
            addr,
        };

        registry.users.push(new_user);

        id
    };

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
                    "who" => {
                        {
                            let users = registry.lock().await;

                            let user = users.get_user(user_id).unwrap();

                            let count = users.users.len();
                            format!("You are {}.\nthere are {} users online, including you.", user.name, count)
                        }
                    },
                    "name" => {
                        if let Some(new_name) = cmd.args.get(0) {
                            let mut reg = registry.lock().await;
                            let user: &mut User = reg.get_user_mut(user_id).unwrap();

                            user.name = new_name.clone();
                            format!("Changed name to {new_name}")
                        } else {
                            format!("Usage: name <new-name>")
                        }
                    },
                    unknown => {
                        format!(">> Unknown command {unknown}")
                    }
                };
                framed_stream.send(&resp).await.unwrap();
            },
            None => {
                eprintln!("user disconnected?");
                {
                    let mut reg = registry.lock().await;
                    reg.remove_user(user_id);
                }
                break;
            },
        }
    }
}

