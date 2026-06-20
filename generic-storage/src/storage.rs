use std::marker::PhantomData;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::serializer::Serializer;

pub struct Storage<T, S> {
    data: Vec<u8>,
    _marker: PhantomData<T>,
    serializer: S,
}

impl<T: BorshSerialize + BorshDeserialize + Serialize + DeserializeOwned, S: Serializer> Storage<T, S> {
    pub fn new(serializer: S) -> Self {
        Storage {
            data: Vec::new(),
            _marker: PhantomData,
            serializer,
        }
    }

    pub fn save(&mut self, value: &T) -> Result<(), String> {
        self.data = self.serializer.to_bytes(value)?;
        Ok(())
    }

    pub fn load(&self) -> Result<T, String> {
        if self.data.is_empty() {
            return Err("No data stored".to_string());
        }
        self.serializer.from_bytes(&self.data)
    }

    pub fn has_data(&self) -> bool {
        !self.data.is_empty()
    }
}
