use sha2::Digest;
use sha2::Sha256;

pub(crate) fn frame_part(hasher: &mut Sha256, part: &[u8]) {
    hasher.update((part.len() as u64).to_be_bytes());
    hasher.update(part);
}
