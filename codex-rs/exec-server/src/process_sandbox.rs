use std::collections::HashMap;

use codex_app_server_protocol::JSONRPCErrorError;
use codex_sandboxing::SandboxCommand;
use codex_sandboxing::SandboxManager;
use codex_sandboxing::SandboxTransformRequest;
use codex_sandboxing::SandboxType;
use codex_sandboxing::SandboxablePreference;
use codex_utils_absolute_path::AbsolutePathBuf;

use crate::ExecServerRuntimePaths;
use crate::protocol::ExecParams;
use crate::rpc::invalid_params;

#[derive(Debug)]
pub(crate) struct PreparedExecRequest {
    pub(crate) command: Vec<String>,
    pub(crate) cwd: AbsolutePathBuf,
    pub(crate) env: HashMap<String, String>,
    pub(crate) arg0: Option<String>,
}

pub(crate) fn prepare_exec_request(
    params: &ExecParams,
    env: HashMap<String, String>,
    runtime_paths: Option<&ExecServerRuntimePaths>,
) -> Result<PreparedExecRequest, JSONRPCErrorError> {
    let native_cwd = AbsolutePathBuf::from_absolute_path(&params.cwd).map_err(|err| {
        invalid_params(format!(
            "cwd `{}` is not an absolute path on this exec-server host: {err}",
            params.cwd.display()
        ))
    })?;
    let Some(sandbox_context) = params.sandbox.as_ref() else {
        return Ok(PreparedExecRequest {
            command: params.argv.clone(),
            cwd: native_cwd,
            env,
            arg0: params.arg0.clone(),
        });
    };
    let runtime_paths = runtime_paths
        .ok_or_else(|| invalid_params("sandbox runtime paths are not configured".to_string()))?;
    let sandbox_policy_cwd = sandbox_context.cwd.as_ref().unwrap_or(&native_cwd);
    let workspace_roots = if sandbox_context.workspace_roots.is_empty() {
        std::slice::from_ref(sandbox_policy_cwd)
    } else {
        sandbox_context.workspace_roots.as_slice()
    };
    let permissions = sandbox_context
        .permissions
        .clone()
        .materialize_project_roots_with_workspace_roots(workspace_roots);
    let (file_system_policy, network_policy) = permissions.to_runtime_permissions();
    let sandbox_manager = SandboxManager::new();
    let sandbox = sandbox_manager.select_initial(
        &file_system_policy,
        network_policy,
        SandboxablePreference::Require,
        sandbox_context.windows_sandbox_level,
        params.enforce_managed_network,
    );
    match sandbox {
        SandboxType::None => {
            return Err(invalid_params(
                "sandbox intent cannot be enforced on this executor".to_string(),
            ));
        }
        SandboxType::WindowsRestrictedToken => {
            return Err(invalid_params(
                "sandboxed remote process launch is not supported on Windows".to_string(),
            ));
        }
        SandboxType::MacosSeatbelt | SandboxType::LinuxSeccomp => {}
    }
    let (program, args) = params
        .argv
        .split_first()
        .ok_or_else(|| invalid_params("argv must not be empty".to_string()))?;
    let request = sandbox_manager
        .transform(SandboxTransformRequest {
            // The platform wrapper owns arg0 after this boundary. Preserve or
            // explicitly reject a custom inner arg0 in a future protocol revision.
            command: SandboxCommand {
                program: program.into(),
                args: args.to_vec(),
                cwd: native_cwd.clone(),
                env,
                additional_permissions: None,
            },
            permissions: &permissions,
            sandbox,
            enforce_managed_network: params.enforce_managed_network,
            network: None,
            sandbox_policy_cwd: sandbox_policy_cwd.as_path(),
            codex_linux_sandbox_exe: runtime_paths.codex_linux_sandbox_exe.as_deref(),
            use_legacy_landlock: sandbox_context.use_legacy_landlock,
            windows_sandbox_level: sandbox_context.windows_sandbox_level,
            windows_sandbox_private_desktop: sandbox_context.windows_sandbox_private_desktop,
        })
        .map_err(|err| invalid_params(format!("failed to prepare process sandbox: {err}")))?;
    Ok(PreparedExecRequest {
        command: request.command,
        cwd: request.cwd,
        env: request.env,
        arg0: request.arg0,
    })
}

#[cfg(test)]
#[path = "process_sandbox_tests.rs"]
mod tests;
