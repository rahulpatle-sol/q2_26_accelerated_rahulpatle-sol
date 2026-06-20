use borsh::{BorshDeserialize, BorshSerialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Todo {
    pub id: u64,
    pub description: String,
    pub created_at: u64,
}

impl Todo {
    pub fn new(id: u64, description: String) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Todo {
            id,
            description,
            created_at,
        }
    }
}
