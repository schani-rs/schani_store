use storage::Storage;

use aws::aws::common::credentials::DefaultCredentialsProviderSync;
use aws::aws::common::region::Region;
use aws::aws::s3::bucket::CreateBucketRequest;
use aws::aws::s3::endpoint::{Endpoint, Signature};
use aws::aws::s3::object::{GetObjectRequest, PutObjectRequest};
use aws::aws::s3::s3client::S3Client;
use hyper::client::Client;
use url::Url;

pub struct S3Storage {
    client: S3Client<DefaultCredentialsProviderSync, Client>,
}

impl S3Storage {
    pub fn new(url: Url) -> Self {
        let provider = DefaultCredentialsProviderSync::new(None).unwrap();
        let endpoint = Endpoint::new(
            Region::EuWest1,
            Signature::V4,
            Some(url),
            None,
            None,
            Some(false),
        );
        let client = S3Client::new(provider, endpoint);

        let storage = S3Storage { client: client };
        storage.init();
        storage
    }

    fn init(&self) {
        if !self.client
            .list_buckets()
            .unwrap()
            .buckets
            .iter()
            .any(|ref bucket| bucket.name == "raw")
        {
            let mut req = CreateBucketRequest::default();
            req.bucket = "raw".to_string();
            self.client.create_bucket(&req).unwrap();
        }
        if !self.client
            .list_buckets()
            .unwrap()
            .buckets
            .iter()
            .any(|ref bucket| bucket.name == "sidecar")
        {
            let mut req = CreateBucketRequest::default();
            req.bucket = "sidecar".to_string();
            self.client.create_bucket(&req).unwrap();
        }
        if !self.client
            .list_buckets()
            .unwrap()
            .buckets
            .iter()
            .any(|ref bucket| bucket.name == "image")
        {
            let mut req = CreateBucketRequest::default();
            req.bucket = "image".to_string();
            self.client.create_bucket(&req).unwrap();
        }
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
        let resp = self.client.get_object(&req, None).unwrap();
        if resp.is_body {
            resp.body
        } else {
            resp.body_buffer
        }
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;

    use super::*;

    #[test]
    fn store_and_get_text() {
        dotenv().unwrap();

        let s3_storage = S3Storage::new("http://127.0.0.1:9100".parse().unwrap());
        s3_storage.store(&"raw".to_string(), &"text.txt".to_string(), b"hello");
        let result = s3_storage.get(&"raw".to_string(), &"text.txt".to_string());
        assert_eq!(b"hello", result.as_slice());
    }
}
