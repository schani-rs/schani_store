use url::Url;

use fileid::{HashIdGenerator, IdGenerator};
use storage::{S3Storage, Storage};

pub struct Store<S: Storage, I: IdGenerator> {
    storage: S,
    id_gen: I,
}

impl Store<S3Storage, HashIdGenerator> {
    pub fn new(url: Url) -> Self {
        Store {
            storage: S3Storage::new(url),
            id_gen: HashIdGenerator::new(),
        }
    }
}

impl<S: Storage, I: IdGenerator> Store<S, I> {
    fn save_file(&self, bucket: &String, data: &[u8]) -> String {
        let id = self.id_gen.get_id(data);
        self.storage.store(bucket, &id, data);
        id
    }

    pub fn save_raw_image(&self, data: &[u8]) -> String {
        self.save_file(&"raw".to_string(), data)
    }

    pub fn save_image(&self, data: &[u8]) -> String {
        self.save_file(&"image".to_string(), data)
    }

    pub fn get_raw_image(&self, id: &String) -> Vec<u8> {
        self.storage.get(&"raw".to_string(), id)
    }

    pub fn get_image(&self, id: &String) -> Vec<u8> {
        self.storage.get(&"image".to_string(), id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use fileid::IdGeneratorStub;
    use storage::DummyStorage;

    fn create_test_store() -> Store<DummyStorage, IdGeneratorStub> {
        Store {
            storage: DummyStorage::new(),
            id_gen: IdGeneratorStub::new(),
        }
    }

    #[test]
    fn test_create() {
        let _story = create_test_store();
    }

    #[test]
    fn test_save_raw_image() {
        let data = b"raw";
        let store = create_test_store();

        let id = store.save_raw_image(data);

        assert_eq!("hash".to_string(), id);
    }

    #[test]
    fn test_save_image() {
        let data = b"raw";
        let store = create_test_store();

        let id = store.save_image(data);

        assert_eq!("hash".to_string(), id);
    }
}
