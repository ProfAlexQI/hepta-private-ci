use hepta_contracts::ContentHash;
use sha2::Digest;
use sha2::Sha256;

pub(super) struct FramedHash(Sha256);

impl FramedHash {
    pub(super) fn new(domain: &str) -> Self {
        let mut hash = Self(Sha256::new());
        hash.bytes("domain", domain.as_bytes());
        hash
    }

    pub(super) fn text(&mut self, name: &str, value: &str) {
        self.bytes(name, value.as_bytes());
    }

    pub(super) fn number(&mut self, name: &str, value: u64) {
        self.bytes(name, &value.to_be_bytes());
    }

    fn bytes(&mut self, name: &str, value: &[u8]) {
        self.0.update((name.len() as u64).to_be_bytes());
        self.0.update(name.as_bytes());
        self.0.update((value.len() as u64).to_be_bytes());
        self.0.update(value);
    }

    pub(super) fn finish(self) -> ContentHash {
        ContentHash::new(format!("sha256:{:x}", self.0.finalize()))
    }
}
