mod hash;
#[cfg(test)]
mod stub;

pub use self::hash::HashIdGenerator;
#[cfg(test)]
pub use self::stub::IdGeneratorStub;

pub trait IdGenerator {
    fn get_id(&self, data: &[u8]) -> String;
}
