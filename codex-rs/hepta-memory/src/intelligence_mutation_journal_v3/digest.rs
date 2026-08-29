use codex_hepta_contracts::Sha256Digest;
use sha2::Digest;
use sha2::Sha256;

use super::IntelligenceMutationBinding;

pub(super) fn binding_digest(binding: &IntelligenceMutationBinding) -> Sha256Digest {
    let mut hasher = Sha256::new();
    super::super::frame_part(
        &mut hasher,
        b"hepta:cognitive:intelligence-mutation-journal-binding:v1",
    );
    super::super::frame_part(&mut hasher, binding.operation_id.as_bytes());
    super::super::frame_part(&mut hasher, binding.lease_id.as_bytes());
    super::super::frame_part(&mut hasher, &binding.lease_epoch.to_be_bytes());
    match binding.expected_revision {
        Some(revision) => {
            super::super::frame_part(&mut hasher, &[1]);
            super::super::frame_part(&mut hasher, &revision.to_be_bytes());
        }
        None => super::super::frame_part(&mut hasher, &[0]),
    }
    super::super::frame_part(
        &mut hasher,
        &binding.starting_projection_generation.to_be_bytes(),
    );
    super::super::frame_part(&mut hasher, binding.causal_root_sha256.as_str().as_bytes());
    Sha256Digest::from_sha256_output(hasher.finalize())
}
