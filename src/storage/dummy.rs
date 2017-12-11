use storage::Storage;

pub struct DummyStorage;

impl DummyStorage {
    pub fn new() -> Self {
        DummyStorage {}
    }
}

impl Storage for DummyStorage {
    fn store(&self, _bucket: &String, _name: &String, data: &[u8]) {}

    fn get(&self, _bucket: &String, _name: &String) -> Vec<u8> {
        vec![]
    }
}
