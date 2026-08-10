use codex_protocol::models::PermissionProfile;
use codex_utils_path_uri::PathUri;
use pretty_assertions::assert_eq;

use super::file_system_handler::FileSystemHandler;
use crate::ExecServerRuntimePaths;
use crate::FileSystemSandboxContext;
use crate::protocol::FsReadFileAuthorizedParams;
use crate::protocol::JSONRPCErrorError;

#[tokio::test]
async fn authorized_read_validates_bounds_and_platform_support() {
    let temp_dir = tempfile::tempdir().expect("tempdir");
    let runtime_paths = ExecServerRuntimePaths::new(
        std::env::current_exe().expect("current exe"),
        /*codex_linux_sandbox_exe*/ None,
    )
    .expect("runtime paths");
    let handler = FileSystemHandler::new(runtime_paths);
    let cwd = PathUri::from_host_native_path(temp_dir.path()).expect("tempdir URI");
    let params = |max_bytes| FsReadFileAuthorizedParams {
        path: cwd.join("bounded.txt").expect("file URI"),
        sandbox: FileSystemSandboxContext::from_permission_profile_with_cwd(
            PermissionProfile::default(),
            cwd.clone(),
        ),
        max_bytes,
    };
    let invalid_bound = handler
        .read_file_authorized(params(0))
        .await
        .expect_err("zero bound must fail closed");
    let expected_invalid_bound = JSONRPCErrorError {
        code: -32600,
        data: None,
        message: "authorized file read bound must leave room for an overflow sentinel".to_string(),
    };
    assert_eq!(invalid_bound, expected_invalid_bound);
    let overflowing_bound = handler
        .read_file_authorized(params(u64::MAX))
        .await
        .expect_err("unrepresentable bound must fail closed");
    assert_eq!(overflowing_bound, expected_invalid_bound);
    let error = handler
        .read_file_authorized(params(4096))
        .await
        .expect_err("missing authorized read target must fail closed");

    let expected = if cfg!(any(target_os = "macos", target_os = "linux")) {
        JSONRPCErrorError {
            code: -32004,
            data: None,
            message: "authorized file read target was not found".to_string(),
        }
    } else {
        JSONRPCErrorError {
            code: -32603,
            data: None,
            message: "bounded authorized file reads are unsupported".to_string(),
        }
    };
    assert_eq!(error, expected);
}
