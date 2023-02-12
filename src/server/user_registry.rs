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
}


