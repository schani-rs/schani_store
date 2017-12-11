use fileid::IdGenerator;

pub struct IdGeneratorStub;

impl IdGeneratorStub {
    pub fn new() -> Self {
        IdGeneratorStub {}
    }
}

impl IdGenerator for IdGeneratorStub {
    fn get_id(&self, _data: &[u8]) -> String {
        "hash".to_string()
    }
}
