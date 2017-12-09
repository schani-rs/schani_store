mod s3;

pub use self::s3::S3Storage;

pub trait Storage {
    fn store(&self, bucket: &String, name: &String, data: &[u8]) -> u64;

    fn get(&self, bucket: &String, name: &String) -> Vec<u8>;
}
