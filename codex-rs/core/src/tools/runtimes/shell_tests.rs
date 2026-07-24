use super::*;
use pretty_assertions::assert_eq;

#[cfg(unix)]
#[test]
fn approval_path_uri_uses_opaque_fallback_for_null_path() {
    use base64::Engine as _;
    use std::os::unix::ffi::OsStringExt;

    let path_bytes = b"/tmp/\0approval".to_vec();
    let path = AbsolutePathBuf::from_absolute_path_checked(std::path::PathBuf::from(
        std::ffi::OsString::from_vec(path_bytes.clone()),
    ))
    .expect("test path is absolute");
    let expected_uri = format!(
        "{OPAQUE_APPROVAL_PATH_URI_PREFIX}{}",
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(path_bytes)
    );

    assert_eq!(
        ApprovalPathUri::from_abs_path(&path),
        ApprovalPathUri(expected_uri)
    );
}

#[tokio::test]
async fn approval_key_uses_path_uri_and_includes_environment_id() {
    let cwd = AbsolutePathBuf::try_from(std::env::current_dir().expect("read current dir"))
        .expect("current dir is absolute");
    let mut request = ShellRequest {
        command: vec!["echo".to_string(), "hello".to_string()],
        environment_id: "remote".to_string(),
        shell_type: None,
        hook_command: "echo hello".to_string(),
        cwd: cwd.clone(),
        timeout_ms: None,
        env: HashMap::new(),
        explicit_env_overrides: HashMap::new(),
        network: None,
        sandbox_permissions: SandboxPermissions::UseDefault,
        additional_permissions: None,
        #[cfg(unix)]
        additional_permissions_preapproved: false,
        justification: None,
        exec_approval_requirement: ExecApprovalRequirement::Skip {
            bypass_sandbox: false,
            proposed_execpolicy_amendment: None,
        },
    };
    let runtime = ShellRuntime::for_shell_command(ShellRuntimeBackend::ShellCommandClassic);
    let original_key = runtime.approval_keys(&request);
    let expected_cwd_uri = Url::from_file_path(cwd.as_path())
        .expect("absolute current dir should convert to a file URI")
        .to_string();
    assert_eq!(
        original_key,
        vec![ApprovalKey {
            environment_id: "remote".to_string(),
            command: request.command.clone(),
            cwd: ApprovalPathUri(expected_cwd_uri.clone()),
            sandbox_permissions: request.sandbox_permissions,
            additional_permissions: request.additional_permissions.clone(),
        }]
    );
    assert_eq!(
        serde_json::to_value(&original_key[0])
            .expect("serialize approval key")
            .get("cwd"),
        Some(&serde_json::Value::String(expected_cwd_uri))
    );
    request.environment_id = "other".to_string();
    let other_key = runtime.approval_keys(&request);

    assert_ne!(original_key, other_key);
}
