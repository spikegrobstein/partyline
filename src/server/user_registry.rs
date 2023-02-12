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
}


