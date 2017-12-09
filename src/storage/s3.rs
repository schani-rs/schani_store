use storage::Storage;

use aws::aws::common::credentials::DefaultCredentialsProvider;
use aws::aws::common::region::Region;
use aws::aws::s3::endpoint::{Endpoint, Signature};
use aws::aws::s3::object::{GetObjectRequest, PutObjectRequest};
use aws::aws::s3::s3client::S3Client;
use hyper::client::Client;
use url::Url;

pub struct S3Storage {
    client: S3Client<DefaultCredentialsProvider, Client>,
}

impl S3Storage {
    pub fn new(url: Url) -> Self {
        let provider = DefaultCredentialsProvider::new(None).unwrap();
        let endpoint = Endpoint::new(
            Region::EuWest1,
            Signature::V4,
            Some(url),
            None,
            None,
            Some(false),
        );
        let client = S3Client::new(provider, endpoint);

        S3Storage { client: client }
    }
}

impl Storage for S3Storage {
    fn store(&self, bucket: &String, name: &String, data: &[u8]) {
        let mut put_obj = PutObjectRequest::default();
        put_obj.bucket = bucket.to_owned();
        put_obj.key = name.to_string();
        put_obj.body = Some(data);
        self.client.put_object(&put_obj, None).unwrap();
    }

    fn get(&self, bucket: &String, name: &String) -> Vec<u8> {
        let mut req = GetObjectRequest::default();
        req.bucket = bucket.to_owned();
        req.key = name.to_owned();
        self.client.get_object(&req, None).unwrap().body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_s3_storage() {
        let _s3_storage = S3Storage::new("http://172.17.0.3:9000".parse().unwrap());
    }

    #[test]
    fn store_and_get_text() {
        let s3_storage = S3Storage::new("http://172.17.0.3:9000".parse().unwrap());
        s3_storage.store(&"test".to_string(), &"text.txt".to_string(), b"hello");
        let result = s3_storage.get(&"test".to_string(), &"text.txt".to_string());
        assert_eq!(b"hello", result.as_slice());
    }
}
