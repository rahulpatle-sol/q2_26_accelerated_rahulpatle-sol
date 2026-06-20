use generic_storage::person::Person;
use generic_storage::serializer::{BincodeSerializer, BorshSerializer, JsonSerializer};
use generic_storage::storage::Storage;

fn main() {
    let person = Person {
        name: "André".to_string(),
        age: 30,
    };

    let mut borsh_storage = Storage::new(BorshSerializer);
    borsh_storage.save(&person).unwrap();
    let loaded = borsh_storage.load().unwrap();
    println!("Borsh  -> Loaded: {:?}", loaded);

    let mut bincode_storage = Storage::new(BincodeSerializer);
    bincode_storage.save(&person).unwrap();
    let loaded = bincode_storage.load().unwrap();
    println!("Bincode -> Loaded: {:?}", loaded);

    let mut json_storage = Storage::new(JsonSerializer);
    json_storage.save(&person).unwrap();
    let loaded = json_storage.load().unwrap();
    println!("Json   -> Loaded: {:?}", loaded);
}
