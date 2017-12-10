use storage::Storage;

pub struct DummyStorage;

impl DummyStorage {
    pub fn new() -> Self {
        DummyStorage {}
    }
}

impl Storage for DummyStorage {
    fn store(&self, bucket: &String, name: &String, data: &[u8]) {}

    fn get(&self, bucket: &String, name: &String) -> Vec<u8> {
        vec![]
    }
}
