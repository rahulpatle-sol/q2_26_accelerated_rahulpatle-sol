use borsh::{BorshDeserialize, BorshSerialize};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Serializer {
    fn to_bytes<T: BorshSerialize + Serialize>(&self, value: &T) -> Result<Vec<u8>, String>;
    fn from_bytes<T: BorshDeserialize + DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, String>;
}

pub struct BorshSerializer;

impl Serializer for BorshSerializer {
    fn to_bytes<T: BorshSerialize + Serialize>(&self, value: &T) -> Result<Vec<u8>, String> {
        borsh::to_vec(value).map_err(|e| format!("Borsh serialization error: {}", e))
    }

    fn from_bytes<T: BorshDeserialize + DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, String> {
        T::try_from_slice(bytes).map_err(|e| format!("Borsh deserialization error: {}", e))
    }
}

pub struct BincodeSerializer;

impl Serializer for BincodeSerializer {
    fn to_bytes<T: BorshSerialize + Serialize>(&self, value: &T) -> Result<Vec<u8>, String> {
        bincode::serialize(value).map_err(|e| format!("Bincode serialization error: {}", e))
    }

    fn from_bytes<T: BorshDeserialize + DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, String> {
        bincode::deserialize(bytes).map_err(|e| format!("Bincode deserialization error: {}", e))
    }
}

pub struct JsonSerializer;

impl Serializer for JsonSerializer {
    fn to_bytes<T: BorshSerialize + Serialize>(&self, value: &T) -> Result<Vec<u8>, String> {
        serde_json::to_vec(value).map_err(|e| format!("JSON serialization error: {}", e))
    }

    fn from_bytes<T: BorshDeserialize + DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, String> {
        serde_json::from_slice(bytes).map_err(|e| format!("JSON deserialization error: {}", e))
    }
}
