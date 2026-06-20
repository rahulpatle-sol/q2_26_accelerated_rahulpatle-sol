use borsh::{BorshDeserialize, BorshSerialize};
use std::fs;
use std::path::Path;

use crate::queue::Queue;

pub fn save_to_file<T: BorshSerialize>(queue: &Queue<T>, path: &str) -> Result<(), String> {
    let bytes = borsh::to_vec(queue).map_err(|e| format!("Serialization error: {}", e))?;
    fs::write(path, bytes).map_err(|e| format!("File write error: {}", e))
}

pub fn load_from_file<T: BorshDeserialize>(path: &str) -> Result<Queue<T>, String> {
    if !Path::new(path).exists() {
        return Ok(Queue::new());
    }
    let bytes = fs::read(path).map_err(|e| format!("File read error: {}", e))?;
    if bytes.is_empty() {
        return Ok(Queue::new());
    }
    Queue::try_from_slice(&bytes).map_err(|e| format!("Deserialization error: {}", e))
}
