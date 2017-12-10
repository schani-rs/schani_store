mod s3;
#[cfg(test)]
mod dummy;

pub use self::s3::S3Storage;
#[cfg(test)]
pub use self::dummy::DummyStorage;

pub trait Storage {
    fn store(&self, bucket: &String, name: &String, data: &[u8]);

    fn get(&self, bucket: &String, name: &String) -> Vec<u8>;
}
