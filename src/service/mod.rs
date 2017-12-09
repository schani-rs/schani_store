use url::Url;

use storage::{S3Storage, Storage};

pub struct Store<S: Storage> {
    storage: S,
}

impl Store<S3Storage> {
    pub fn new(url: Url) -> Self {
        Store {
            storage: S3Storage::new(url),
        }
    }
}

impl<S: Storage> Store<S> {
    pub fn save_raw_image(&self, data: &[u8]) -> u64 {
        self.storage.store(&"raw".to_string(), data)
    }

    pub fn save_image(&self, data: &[u8]) -> u64 {
        self.storage.store(&"image".to_string(), data)
    }

    pub fn get_raw_image(&self, id: u64) -> Vec<u8> {
        self.storage.get(&"raw".to_string(), id)
    }

    pub fn get_image(&self, id: u64) {
        self.storage.get(&"image".to_string(), id)
    }
}
