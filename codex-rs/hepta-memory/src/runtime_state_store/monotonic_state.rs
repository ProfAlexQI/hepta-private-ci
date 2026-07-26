use sha2::Digest;
use sha2::Sha256;

use super::RuntimeStatePersistence;
use super::RuntimeStateStoreError;
use super::read_envelope;
use super::validate_namespace_identity;
use super::verify_envelope;

const RUNTIME_STATE_HASH_DOMAIN: &[u8] = b"hepta.runtime-state.monotonic-state.sha256.v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeStateMonotonicState {
    generation: u64,
    state_hash: String,
}

impl RuntimeStateMonotonicState {
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn state_hash(&self) -> &str {
        &self.state_hash
    }
}

impl RuntimeStatePersistence {
    pub(crate) fn monotonic_state(
        &self,
    ) -> Result<RuntimeStateMonotonicState, RuntimeStateStoreError> {
        let generation = self
            .generation
            .lock()
            .map_err(|_| RuntimeStateStoreError::corrupt("generation mutex poisoned"))?;
        let identity = self
            .identity
            .lock()
            .map_err(|_| RuntimeStateStoreError::corrupt("identity mutex poisoned"))?;
        validate_namespace_identity(&self.path, *identity)?;
        let (envelope, current_identity) = read_envelope(&self.path)?;
        if current_identity != *identity {
            return Err(RuntimeStateStoreError::corrupt(
                "runtime state identity changed during monotonic projection",
            ));
        }
        verify_envelope(&self.integrity, &envelope)?;
        if envelope.payload.generation != *generation {
            return Err(RuntimeStateStoreError::corrupt(
                "runtime state generation diverged from the opened state",
            ));
        }
        let payload = serde_json::to_vec(&envelope.payload).map_err(|error| {
            RuntimeStateStoreError::persistence("encode runtime monotonic state", error)
        })?;
        let mut hasher = Sha256::new();
        hasher.update((RUNTIME_STATE_HASH_DOMAIN.len() as u64).to_be_bytes());
        hasher.update(RUNTIME_STATE_HASH_DOMAIN);
        hasher.update((payload.len() as u64).to_be_bytes());
        hasher.update(payload);
        Ok(RuntimeStateMonotonicState {
            generation: *generation,
            state_hash: format!("sha256:{:x}", hasher.finalize()),
        })
    }
}
