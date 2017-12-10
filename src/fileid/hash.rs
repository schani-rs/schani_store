use crypto::digest::Digest;
use crypto::sha3::Sha3;

use fileid::IdGenerator;

pub struct HashIdGenerator {
}

impl HashIdGenerator {
    pub fn new() -> Self {
        HashIdGenerator {
        }
    }
}

impl IdGenerator for HashIdGenerator {
    fn get_id(&self, data: &[u8]) -> String {
        let mut hasher = Sha3::sha3_512();
        hasher.input(data);
        hasher.result_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_data() {
        let gen = HashIdGenerator::new();
        let data = b"hello";

        let hashed = gen.get_id(data);

        assert_eq!("75d527c368f2efe848ecf6b073a36767800805e9eef2b1857d5f984f036eb6df891d75f72d9b154518c1cd58835286d1da9a38deba3de98b5a53e5ed78a84976".to_string(), hashed);
    }
}
