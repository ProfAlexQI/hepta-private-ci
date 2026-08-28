use std::path::Path;

use codex_hepta_contracts::Sha256Digest;
use sha2::Digest;
use sha2::Sha256;

#[path = "fact_grounding/durable.rs"]
mod durable_fact_grounding;
#[path = "fact_grounding/shadow_projection_gate.rs"]
mod shadow_projection_gate;
#[allow(dead_code)]
#[path = "intelligence_mutation_state.rs"]
mod intelligence_mutation_state;
#[allow(dead_code, unused_imports)]
#[path = "intelligence_mutation_journal_v3.rs"]
mod intelligence_mutation_journal;
#[allow(dead_code)]
#[path = "intelligence_mutation_shadow_host.rs"]
mod intelligence_mutation_shadow_host;

pub(crate) fn frame_part(hasher: &mut Sha256, part: &[u8]) {
    hasher.update((part.len() as u64).to_be_bytes());
    hasher.update(part);
}

/// Stable, platform-aware identity for a registered workspace root.
///
/// Fleet control uses this exact function when binding a federation grant;
/// the model-input extension uses the same function when proving the current
/// workspace at recall and physical-send time.
pub fn workspace_binding_digest(path: &Path) -> Sha256Digest {
    let mut hasher = Sha256::new();
    frame_part(&mut hasher, b"hepta-memory:workspace:v1");
    frame_part(&mut hasher, &path_identity_bytes(path));
    Sha256Digest::for_bytes(&hasher.finalize())
}

#[cfg(unix)]
fn path_identity_bytes(path: &Path) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;

    path.as_os_str().as_bytes().to_vec()
}

#[cfg(windows)]
fn path_identity_bytes(path: &Path) -> Vec<u8> {
    use std::os::windows::ffi::OsStrExt;

    path.as_os_str()
        .encode_wide()
        .flat_map(u16::to_le_bytes)
        .collect()
}

#[cfg(not(any(unix, windows)))]
fn path_identity_bytes(path: &Path) -> Vec<u8> {
    path.as_os_str().to_string_lossy().as_bytes().to_vec()
}
