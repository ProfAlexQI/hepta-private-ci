#[test]
fn sha256_binding_is_strict() {
    assert!(parse_sha256_binding(&format!("sha256:{}", "a".repeat(64))).is_ok());
    assert!(parse_sha256_binding(&format!("sha256:{}", "A".repeat(64))).is_err());
    assert!(parse_sha256_binding(&"a".repeat(64)).is_err());
    assert!(parse_sha256_binding("sha256:abc").is_err());
}

#[tokio::test]
async fn executable_provenance_matches_exact_digest() {
    let directory = test_directory();
    let executable = directory.join("lms-fixture");
    std::fs::write(&executable, b"fixture executable").expect("write executable");
    let canonical = canonical_executable(&executable).expect("canonical executable");
    let expected = format!("sha256:{}", digest_bytes(b"fixture executable"));
    verify_lms_cli_with_expected(&canonical, &expected)
        .await
        .expect("matching digest");
    let error = verify_lms_cli_with_expected(&canonical, &format!("sha256:{}", "0".repeat(64)))
        .await
        .expect_err("mismatch must fail");
    assert!(error.to_string().contains("LMSTUDIO_CLI_DIGEST_MISMATCH"));
    std::fs::remove_dir_all(directory).expect("remove fixture");
}

#[cfg(unix)]
#[tokio::test]
async fn explicit_download_command_is_killed_on_timeout() {
    use std::os::unix::fs::PermissionsExt;

    let directory = test_directory();
    let executable = directory.join("lms-timeout");
    std::fs::write(&executable, b"#!/bin/sh\nwhile :; do :; done\n").expect("write script");
    let mut permissions = std::fs::metadata(&executable)
        .expect("metadata")
        .permissions();
    permissions.set_mode(0o700);
    std::fs::set_permissions(&executable, permissions).expect("permissions");
    let error = run_download_command(&executable, "fixture", Duration::from_millis(50))
        .await
        .expect_err("timeout must fail");
    assert_eq!(error.kind(), io::ErrorKind::TimedOut);
    assert!(error.to_string().contains("LMSTUDIO_CLI_TIMEOUT"));
    std::fs::remove_dir_all(directory).expect("remove fixture");
}

#[test]
fn model_identifier_is_bounded() {
    assert!(validate_model_identifier("openai/gpt-oss-20b").is_ok());
    assert!(validate_model_identifier("").is_err());
    assert!(validate_model_identifier(" padded").is_err());
    assert!(validate_model_identifier("bad\nmodel").is_err());
    assert!(validate_model_identifier(&"x".repeat(513)).is_err());
}

#[test]
fn base_url_requires_loopback_http_without_credentials() {
    assert!(validate_loopback_http_base_url("http://localhost:1234/v1").is_ok());
    assert!(validate_loopback_http_base_url("http://127.0.0.1:1234").is_ok());
    assert!(validate_loopback_http_base_url("http://[::1]:1234/v1").is_ok());
    assert!(validate_loopback_http_base_url("https://127.0.0.1:1234/v1").is_err());
    assert!(validate_loopback_http_base_url("http://example.com:1234/v1").is_err());
    assert!(validate_loopback_http_base_url("http://user@127.0.0.1:1234/v1").is_err());
    assert!(validate_loopback_http_base_url("http://127.0.0.1:1234/other").is_err());
}

#[test]
fn missing_fallback_is_typed() {
    let result = LMStudioClient::find_lms_with_home_dir(Some("/path/that/does/not/exist"));
    if let Err(error) = result {
        assert!(error.to_string().contains("LMSTUDIO_CLI_NOT_FOUND"));
    }
}

fn test_directory() -> PathBuf {
    let id = NEXT_TEST_DIR.fetch_add(1, Ordering::Relaxed);
    let path =
        std::env::temp_dir().join(format!("codex-lmstudio-inf0c-{}-{id}", std::process::id()));
    std::fs::create_dir_all(&path).expect("create fixture directory");
    path
}
