use std::fs::File;
use std::io::Read;
use std::path::Path;

use codex_hepta_contracts::Sha256Digest;
use sha2::Digest;
use sha2::Sha256;

use crate::ProofError;
use crate::command::MAX_PROOF_HASH_FILE_BYTES;

pub fn sha256_regular_file(path: &Path, max_bytes: u64) -> Result<Sha256Digest, ProofError> {
    if max_bytes == 0 || max_bytes > MAX_PROOF_HASH_FILE_BYTES {
        return Err(ProofError::InvalidInput(
            "proof file hash limit is outside the hard bound".to_string(),
        ));
    }
    let metadata = std::fs::symlink_metadata(path)
        .map_err(|error| ProofError::StoreUnavailable(error.to_string()))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(ProofError::InvalidInput(
            "proof hash input must be a non-symlink regular file".to_string(),
        ));
    }
    if metadata.len() > max_bytes {
        return Err(ProofError::InvalidInput(
            "proof hash input exceeds the hard bound".to_string(),
        ));
    }
    let mut file =
        File::open(path).map_err(|error| ProofError::StoreUnavailable(error.to_string()))?;
    let mut hasher = Sha256::new();
    let mut observed = 0_u64;
    let mut chunk = [0_u8; 64 * 1024];
    loop {
        let read = file
            .read(&mut chunk)
            .map_err(|error| ProofError::StoreUnavailable(error.to_string()))?;
        if read == 0 {
            break;
        }
        observed = observed.saturating_add(read as u64);
        if observed > max_bytes {
            return Err(ProofError::InvalidInput(
                "proof hash input exceeds the hard bound".to_string(),
            ));
        }
        hasher.update(&chunk[..read]);
    }
    Sha256Digest::parse(format!("{:x}", hasher.finalize()))
        .map_err(|error| ProofError::Corrupt(format!("proof file hash is invalid: {error}")))
}

pub(crate) fn validate_execution_directory(cwd: &Path) -> Result<(), ProofError> {
    let metadata = std::fs::symlink_metadata(cwd)
        .map_err(|_| ProofError::InvalidInput("proof cwd metadata is unavailable".to_string()))?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(ProofError::InvalidInput(
            "proof cwd must be a non-symlink directory".to_string(),
        ));
    }
    Ok(())
}
