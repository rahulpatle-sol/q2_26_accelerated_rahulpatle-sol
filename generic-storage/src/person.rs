use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer::{BincodeSerializer, BorshSerializer, JsonSerializer};
    use crate::storage::Storage;

    #[test]
    fn test_borsh() {
        let person = Person {
            name: "André".to_string(),
            age: 30,
        };
        let mut storage = Storage::new(BorshSerializer);
        storage.save(&person).unwrap();
        assert!(storage.has_data());
        let loaded = storage.load().unwrap();
        assert_eq!(loaded, person);
    }

    #[test]
    fn test_bincode() {
        let person = Person {
            name: "Alice".to_string(),
            age: 25,
        };
        let mut storage = Storage::new(BincodeSerializer);
        storage.save(&person).unwrap();
        assert!(storage.has_data());
        let loaded = storage.load().unwrap();
        assert_eq!(loaded, person);
    }

    #[test]
    fn test_json() {
        let person = Person {
            name: "Bob".to_string(),
            age: 42,
        };
        let mut storage = Storage::new(JsonSerializer);
        storage.save(&person).unwrap();
        assert!(storage.has_data());
        let loaded = storage.load().unwrap();
        assert_eq!(loaded, person);
    }

    #[test]
    fn test_load_empty() {
        let storage: Storage<Person, BorshSerializer> = Storage::new(BorshSerializer);
        assert!(!storage.has_data());
        let result = storage.load();
        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_saves() {
        let person1 = Person {
            name: "First".to_string(),
            age: 10,
        };
        let person2 = Person {
            name: "Second".to_string(),
            age: 20,
        };
        let mut storage = Storage::new(JsonSerializer);
        storage.save(&person1).unwrap();
        storage.save(&person2).unwrap();
        let loaded = storage.load().unwrap();
        assert_eq!(loaded, person2);
    }
}
