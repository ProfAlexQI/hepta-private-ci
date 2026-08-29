use std::fmt;
use std::fs;
use std::path::PathBuf;

use codex_hepta_infer_core::Digest;
use codex_hepta_infer_core::GgufModelManifest;
use codex_hepta_infer_core::HEPTA_BACKEND_ABI_VERSION;
use codex_hepta_infer_core::LLAMA_CPP_PINNED_COMMIT;
use libloading::Library;
use libloading::Symbol;
use sha2::Digest as ShaDigest;
use sha2::Sha256;

const MAX_FIXTURE_INPUT_BYTES: usize = 1024 * 1024;
const MAX_FIXTURE_OUTPUT_BYTES: usize = 64 * 1024;
const ABI_VERSION_SYMBOL: &[u8] = b"hepta_backend_abi_version\0";
const ABI_EXECUTE_FIXTURE_SYMBOL: &[u8] = b"hepta_backend_execute_fixture\0";

type AbiVersionFn = unsafe extern "C" fn() -> u32;
type AbiExecuteFixtureFn =
    unsafe extern "C" fn(*const u8, usize, *mut u8, usize) -> isize;

pub type NativeAbiResult<T> = std::result::Result<T, NativeAbiError>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeAbiError {
    AbiVersionMismatch,
    DigestMismatch,
    ExecutionFailed,
    InputBound,
    InvalidManifest,
    Io,
    LoadFailed,
    OutputBound,
    SymbolMissing,
}

impl NativeAbiError {
    pub const fn code(self) -> &'static str {
        match self {
            Self::AbiVersionMismatch => "INF_NATIVE_ABI_VERSION_MISMATCH",
            Self::DigestMismatch => "INF_NATIVE_LIBRARY_DIGEST_MISMATCH",
            Self::ExecutionFailed => "INF_NATIVE_ABI_EXECUTION_FAILED",
            Self::InputBound => "INF_NATIVE_ABI_INPUT_BOUND",
            Self::InvalidManifest => "INF_NATIVE_MANIFEST_INVALID",
            Self::Io => "INF_NATIVE_LIBRARY_IO",
            Self::LoadFailed => "INF_NATIVE_LIBRARY_LOAD_FAILED",
            Self::OutputBound => "INF_NATIVE_ABI_OUTPUT_BOUND",
            Self::SymbolMissing => "INF_NATIVE_ABI_SYMBOL_MISSING",
        }
    }
}

impl fmt::Display for NativeAbiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for NativeAbiError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeRuntimeManifest {
    pub library_path: PathBuf,
    pub library_digest: Digest,
    pub model: GgufModelManifest,
    pub chat_template_digest: Digest,
    pub build_flags_digest: Digest,
    pub runtime_binary_digest: Digest,
    pub fixture_only: bool,
}

impl NativeRuntimeManifest {
    pub fn validate(&self) -> NativeAbiResult<()> {
        self.model
            .validate()
            .map_err(|_| NativeAbiError::InvalidManifest)?;
        let metadata = fs::symlink_metadata(&self.library_path).map_err(|_| NativeAbiError::Io)?;
        if !self.library_path.is_absolute()
            || !metadata.file_type().is_file()
            || metadata.file_type().is_symlink()
            || self.model.backend.abi_version != HEPTA_BACKEND_ABI_VERSION
            || self.model.backend.backend_id != "llama.cpp"
            || self.model.backend.upstream_commit != LLAMA_CPP_PINNED_COMMIT
        {
            return Err(NativeAbiError::InvalidManifest);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeRuntimeBindingReceipt {
    pub tuple_digest: Digest,
    pub runtime_library_digest: Digest,
    pub runtime_binary_digest: Digest,
    pub model_digest: Digest,
    pub tokenizer_digest: Digest,
    pub gguf_artifact_digest: Digest,
    pub chat_template_digest: Digest,
    pub sbom_digest: Digest,
    pub license_digest: Digest,
    pub device_profile_digest: Digest,
    pub build_flags_digest: Digest,
    pub pinned_upstream_commit: String,
    pub fixture_only: bool,
    pub real_native_model_executed: bool,
    pub remote_fallback_attempted: bool,
}

pub struct NativeRuntimeLoader {
    _library: Library,
    execute_fixture: AbiExecuteFixtureFn,
    manifest: NativeRuntimeManifest,
}

impl NativeRuntimeLoader {
    pub fn open(manifest: NativeRuntimeManifest) -> NativeAbiResult<Self> {
        manifest.validate()?;
        let bytes = fs::read(&manifest.library_path).map_err(|_| NativeAbiError::Io)?;
        if digest_bytes(&bytes)? != manifest.library_digest {
            return Err(NativeAbiError::DigestMismatch);
        }
        let library = unsafe {
            // SAFETY: the path is absolute, points to a regular non-symlink file, and its
            // complete bytes were verified against the fixed manifest before loading.
            Library::new(&manifest.library_path)
        }
        .map_err(|_| NativeAbiError::LoadFailed)?;
        let abi_version = unsafe {
            // SAFETY: the symbol type and name are part of the versioned Hepta ABI contract.
            let symbol: Symbol<'_, AbiVersionFn> = library
                .get(ABI_VERSION_SYMBOL)
                .map_err(|_| NativeAbiError::SymbolMissing)?;
            *symbol
        };
        let execute_fixture = unsafe {
            // SAFETY: the symbol type and name are part of the versioned Hepta ABI contract.
            let symbol: Symbol<'_, AbiExecuteFixtureFn> = library
                .get(ABI_EXECUTE_FIXTURE_SYMBOL)
                .map_err(|_| NativeAbiError::SymbolMissing)?;
            *symbol
        };
        let observed_version = unsafe {
            // SAFETY: `abi_version` was resolved from the validated ABI symbol above.
            abi_version()
        };
        if observed_version != HEPTA_BACKEND_ABI_VERSION {
            return Err(NativeAbiError::AbiVersionMismatch);
        }
        Ok(Self {
            _library: library,
            execute_fixture,
            manifest,
        })
    }

    pub fn execute_fixture(&self, input: &[u8], output_capacity: usize) -> NativeAbiResult<Vec<u8>> {
        if input.is_empty() || input.len() > MAX_FIXTURE_INPUT_BYTES {
            return Err(NativeAbiError::InputBound);
        }
        if output_capacity == 0 || output_capacity > MAX_FIXTURE_OUTPUT_BYTES {
            return Err(NativeAbiError::OutputBound);
        }
        let mut output = vec![0u8; output_capacity];
        let written = unsafe {
            // SAFETY: input and output pointers are valid for their declared lengths for the
            // duration of the call. The loaded function was resolved from the verified ABI.
            (self.execute_fixture)(
                input.as_ptr(),
                input.len(),
                output.as_mut_ptr(),
                output.len(),
            )
        };
        let written = usize::try_from(written).map_err(|_| NativeAbiError::ExecutionFailed)?;
        if written == 0 || written > output.len() {
            return Err(NativeAbiError::ExecutionFailed);
        }
        output.truncate(written);
        Ok(output)
    }

    pub fn binding_receipt(&self) -> NativeRuntimeBindingReceipt {
        NativeRuntimeBindingReceipt {
            tuple_digest: self.manifest.model.tuple_digest.clone(),
            runtime_library_digest: self.manifest.library_digest.clone(),
            runtime_binary_digest: self.manifest.runtime_binary_digest.clone(),
            model_digest: self.manifest.model.model_digest.clone(),
            tokenizer_digest: self.manifest.model.tokenizer_digest.clone(),
            gguf_artifact_digest: self.manifest.model.gguf_artifact_digest.clone(),
            chat_template_digest: self.manifest.chat_template_digest.clone(),
            sbom_digest: self.manifest.model.sbom_digest.clone(),
            license_digest: self.manifest.model.license_digest.clone(),
            device_profile_digest: self.manifest.model.device_profile_digest.clone(),
            build_flags_digest: self.manifest.build_flags_digest.clone(),
            pinned_upstream_commit: self.manifest.model.backend.upstream_commit.clone(),
            fixture_only: self.manifest.fixture_only,
            real_native_model_executed: false,
            remote_fallback_attempted: false,
        }
    }
}

pub fn digest_bytes(bytes: &[u8]) -> NativeAbiResult<Digest> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(71);
    encoded.push_str("sha256:");
    for byte in digest {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    Digest::parse(&encoded).map_err(|_| NativeAbiError::InvalidManifest)
}
