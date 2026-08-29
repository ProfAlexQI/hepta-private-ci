#![cfg(unix)]

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use codex_hepta_infer_core::BackendAbiContract;
use codex_hepta_infer_core::Digest;
use codex_hepta_infer_core::GgufModelManifest;
use codex_hepta_infer_core::WorkerQualificationDisposition;
use codex_hepta_infer_worker_host::NativeAbiError;
use codex_hepta_infer_worker_host::NativeRuntimeLoader;
use codex_hepta_infer_worker_host::NativeRuntimeManifest;
use codex_hepta_infer_worker_host::digest_bytes;
use tempfile::TempDir;

fn must<T, E: std::fmt::Display>(result: std::result::Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => panic!("unexpected error: {error}"),
    }
}

fn digest(fill: char) -> Digest {
    must(Digest::parse(&format!(
        "sha256:{}",
        fill.to_string().repeat(64)
    )))
}

fn build_fixture_library(temp: &TempDir) -> PathBuf {
    let source = temp.path().join("fixture.c");
    must(fs::write(
        &source,
        r#"
#include <stdint.h>
#include <stddef.h>
#include <string.h>

uint32_t hepta_backend_abi_version(void) {
    return 1;
}

int64_t hepta_backend_execute_fixture(
    const uint8_t *input,
    size_t input_len,
    uint8_t *output,
    size_t output_capacity
) {
    static const uint8_t token[] = "fixture-token";
    const size_t token_len = sizeof(token) - 1;
    if (input == NULL || input_len == 0 || output == NULL || output_capacity < token_len) {
        return -1;
    }
    memcpy(output, token, token_len);
    return (int64_t)token_len;
}
"#,
    ));
    #[cfg(target_os = "macos")]
    let library = temp.path().join("libhepta_fixture.dylib");
    #[cfg(not(target_os = "macos"))]
    let library = temp.path().join("libhepta_fixture.so");

    let mut command = Command::new("cc");
    #[cfg(target_os = "macos")]
    command.arg("-dynamiclib");
    #[cfg(not(target_os = "macos"))]
    command.args(["-shared", "-fPIC"]);
    let status = must(command.arg(&source).arg("-o").arg(&library).status());
    assert!(status.success(), "C fixture library must compile");
    library
}

fn manifest(library_path: PathBuf, library_digest: Digest) -> NativeRuntimeManifest {
    NativeRuntimeManifest {
        library_path,
        library_digest,
        model: GgufModelManifest {
            tuple_digest: digest('a'),
            model_digest: digest('b'),
            tokenizer_digest: digest('c'),
            gguf_artifact_digest: digest('d'),
            sbom_digest: digest('e'),
            license_digest: digest('f'),
            device_profile_digest: digest('1'),
            backend: BackendAbiContract::pinned_llama_cpp(),
            quantization: "Q4_K_M".to_owned(),
            disposition: WorkerQualificationDisposition::KnownGapNotRouted,
        },
        chat_template_digest: digest('2'),
        build_flags_digest: digest('3'),
        runtime_binary_digest: digest('4'),
        fixture_only: true,
    }
}

#[test]
fn verified_dynamic_library_executes_only_the_pinned_fixture_abi() {
    let temp = must(TempDir::new());
    let library = build_fixture_library(&temp);
    let bytes = must(fs::read(&library));
    let loader = must(NativeRuntimeLoader::open(manifest(
        library,
        must(digest_bytes(&bytes)),
    )));
    assert_eq!(
        must(loader.execute_fixture(b"bounded-input", 64)),
        b"fixture-token"
    );
    let receipt = loader.binding_receipt();
    assert!(receipt.fixture_only);
    assert!(!receipt.real_native_model_executed);
    assert!(!receipt.remote_fallback_attempted);
    assert_eq!(
        receipt.pinned_upstream_commit,
        BackendAbiContract::pinned_llama_cpp().upstream_commit
    );
}

#[test]
fn library_digest_mismatch_fails_before_symbol_execution() {
    let temp = must(TempDir::new());
    let library = build_fixture_library(&temp);
    assert!(matches!(
        NativeRuntimeLoader::open(manifest(library, digest('9'))),
        Err(NativeAbiError::DigestMismatch)
    ));
}
