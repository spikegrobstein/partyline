use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;

pub struct User {
    pub id: u32,
    pub name: String,
    pub addr: SocketAddr,
    pub sender: Sender<String>,
}
