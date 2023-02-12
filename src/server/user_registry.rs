use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::error::SendError;
use crate::server::User;

pub struct UserRegistry {
    pub counter: u32,
    pub users: Vec<User>,
}

impl UserRegistry {
    pub fn next_id(&mut self) -> u32 {
        self.counter += 1;
        self.counter
    }

    pub fn remove_user(&mut self, id: u32) {
        self.users.retain(|u| u.id != id);
    }

    pub fn get_user(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }

    pub fn get_user_mut(&mut self, id: u32) -> Option<&mut User> {
        self.users.iter_mut().find(|u| u.id == id)
    }

    pub fn get_senders(&self) -> Vec<(u32, String, Sender<String>)> {
        self.users
            .iter()
            .map(|u| {
                (u.id, u.name.clone(), u.sender.clone())
            })
            .collect()
    }

    pub async fn broadcast(&self, msg: String) -> Result<(), SendError<String>> {
        let msg = format!(">> {msg}");

        for (_user_id, _, sender) in self.get_senders() {
            sender.send(msg.clone()).await?;
        }

        Ok(())
    }
    // pub async fn send_chat(&self, id: u32, chat: &str) {
        // let name = {
            // &self.get_user(id).unwrap().name
        // };

        // for user in &self.users {

        // }
    // }
}


