use std::io;
use std::sync::Arc;
use std::net::SocketAddr;
use std::env;

use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::fs;

use tokio_util::codec::Framed;

use futures::{StreamExt, SinkExt};

use crate::server::UserRegistry;
use crate::server::User;
use crate::server::CmdCodec;
use crate::server::cmd_codec::Packet;

const WELCOME_FILE: &str = "welcome.txt";

pub struct Server {
    pub registry: Mutex<UserRegistry>,
}

impl Server {
    pub fn new() -> Self {
        let registry = UserRegistry::default();

        Self {
            registry: Mutex::new(registry),
        }
    }

    pub async fn listen(self: Arc<Self>) -> io::Result<()> {
        let addr = env::var("BIND_ADDR")
            .unwrap_or_else(|_| { "127.0.0.1:9999".to_owned() });
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (socket, addr) = listener.accept().await?;

            tokio::spawn({
                let me = Arc::clone(&self);
                async move {
                    me.clone().handle_connection(socket, addr).await;
                }
            });
        }
    }

    pub async fn welcome_message(&self) -> Option<String> {
        fs::read_to_string(WELCOME_FILE).await.ok()
    }

    async fn handle_connection(&self, socket: TcpStream, addr: SocketAddr) {
        println!("got connection from {addr}");

        let (tx, mut rx) = channel::<String>(32);
        let sender = tx.clone();

        let user_id = {
            let mut registry = self.registry.lock().await;

            let id = registry.next_id();

            let new_user = User {
                id,
                name: format!("anonymous[{id}]"),
                addr,
                sender,
            };

            registry.users.push(new_user);

            id
        };

        let framed_stream = Framed::new(socket, CmdCodec);
        let (mut sink, mut stream) = framed_stream.split();
        let sender = tx.clone();

        if let Some(msg) = self.welcome_message().await {
            sender.send(msg).await.unwrap();
        }

        // loop to receive messages to send to this connection
        tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Some(msg) => {
                        if let Err(err) = sink.send(msg).await {
                            eprintln!("Failed to send: {:#?}", err);
                        }
                    },
                    None => {
                        eprintln!("ending recv loop.");
                        break;
                    }
                }
            }
        });

        // loop to read data from the connection
        loop {
            match stream.next().await {
                Some(Err(err)) => {
                    eprintln!("got an error... ignoring... {:#?}", err);
                },
                Some(Ok(Packet::Chat(chatstring))) => {
                    eprintln!("got a chat string");
                    let (username, senders) = {
                        let reg = self.registry.lock().await;

                        let user = reg.get_user(user_id).unwrap();
                        let username = user.name.clone();

                        (username, reg.get_senders())
                    };

                    let chatline = format!("<{}> {}", username, chatstring);
                    for (user_id, username, tx) in senders {
                        if tx.send(chatline.clone()).await.is_err() {
                            eprintln!("Failed to send chat to {user_id}");
                        } else {
                            eprintln!("sent chat to {username}");
                        }
                    }
                },
                Some(Ok(Packet::Command(cmd))) => {
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
                                let users = self.registry.lock().await;

                                let user = users.get_user(user_id).unwrap();

                                let count = users.users.len();
                                format!("You are {}.\nthere are {} users online, including you.", user.name, count)
                            }
                        },
                        "name" => {
                            if let Some(new_name) = cmd.args.get(0) {
                                let mut reg = self.registry.lock().await;
                                let user: &mut User = reg.get_user_mut(user_id).unwrap();

                                let old_name = user.name.clone();
                                user.name = new_name.clone();
                                reg.broadcast(format!("User renamed {old_name} -> {new_name}")).await.unwrap();
                                format!("Changed name to {new_name}")
                            } else {
                                format!("Usage: name <new-name>")
                            }
                        },
                        unknown => {
                            format!(">> Unknown command {unknown}")
                        }
                    };
                    sender.send(resp).await.unwrap();
                },
                None => {
                    eprintln!("user disconnected?");
                    {
                        let mut reg = self.registry.lock().await;
                        reg.remove_user(user_id);
                    }
                    break;
                },
            }
        }
    }
}
