use std::process::ExitStatus;
use std::time::Duration;

use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::process::Child;
use tokio::process::Command;
use tokio::time::timeout;

use crate::ProofError;
use crate::command::MAX_PROOF_HASH_FILE_BYTES;
use crate::command::ProofCommandSpec;
use crate::command::ProofStreamEvidence;
use crate::command::ProofStreamKind;
use crate::command::ProofTerminal;
use crate::file_hash::sha256_regular_file;
use crate::file_hash::validate_execution_directory;

const PROCESS_REAP_TIMEOUT: Duration = Duration::from_secs(2);

pub(crate) struct ExecutionCapture {
    pub terminal: ProofTerminal,
    pub stdout_evidence: ProofStreamEvidence,
    pub stderr_evidence: ProofStreamEvidence,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub(crate) async fn execute(spec: &ProofCommandSpec) -> ExecutionCapture {
    if cfg!(not(unix)) {
        let _ = spec;
        return unavailable_capture(ProofTerminal::NotStarted {
            reason_code: "proof_process_containment_unsupported".to_string(),
        });
    }
    if validate_execution_paths(spec).is_err() {
        return unavailable_capture(ProofTerminal::NotStarted {
            reason_code: "proof_execution_path_invalid".to_string(),
        });
    }
    let mut command = Command::new(&spec.program);
    command
        .args(&spec.arguments)
        .current_dir(&spec.cwd)
        .env_clear()
        .envs(&spec.environment)
        .kill_on_drop(true)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;

        command.as_std_mut().process_group(0);
    }
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(_) => {
            return unavailable_capture(ProofTerminal::NotStarted {
                reason_code: "proof_process_spawn_failed".to_string(),
            });
        }
    };
    let mut process_tree = ProcessTreeGuard::for_child(&child);
    let Some(stdout) = child.stdout.take() else {
        return unavailable_capture(ProofTerminal::Indeterminate {
            reason_code: "proof_stdout_pipe_missing".to_string(),
        });
    };
    let Some(stderr) = child.stderr.take() else {
        return unavailable_capture(ProofTerminal::Indeterminate {
            reason_code: "proof_stderr_pipe_missing".to_string(),
        });
    };
    let capture = capture_child(
        &mut child,
        stdout,
        stderr,
        spec.max_stdout_bytes,
        spec.max_stderr_bytes,
    );
    match timeout(Duration::from_millis(spec.timeout_ms), capture).await {
        Ok(Ok((status, stdout, stderr))) => {
            let terminal = if process_tree.kill() {
                completed_terminal(status)
            } else {
                ProofTerminal::Indeterminate {
                    reason_code: "proof_process_tree_cleanup_failed".to_string(),
                }
            };
            ExecutionCapture {
                terminal,
                stdout_evidence: ProofStreamEvidence::complete(&stdout),
                stderr_evidence: ProofStreamEvidence::complete(&stderr),
                stdout,
                stderr,
            }
        }
        Ok(Err(CaptureFailure::OutputLimit(stream))) => {
            match terminate_child(&mut child, &mut process_tree).await {
                Ok(()) => unavailable_capture(ProofTerminal::OutputLimitExceeded { stream }),
                Err(reason_code) => unavailable_capture(ProofTerminal::Indeterminate {
                    reason_code: reason_code.to_string(),
                }),
            }
        }
        Ok(Err(CaptureFailure::Io)) => {
            let reason_code = terminate_child(&mut child, &mut process_tree)
                .await
                .err()
                .unwrap_or("proof_process_capture_failed");
            unavailable_capture(ProofTerminal::Indeterminate {
                reason_code: reason_code.to_string(),
            })
        }
        Err(_) => match terminate_child(&mut child, &mut process_tree).await {
            Ok(()) => unavailable_capture(ProofTerminal::TimedOut),
            Err(reason_code) => unavailable_capture(ProofTerminal::Indeterminate {
                reason_code: reason_code.to_string(),
            }),
        },
    }
}

async fn capture_child(
    child: &mut Child,
    stdout: impl AsyncRead + Unpin,
    stderr: impl AsyncRead + Unpin,
    max_stdout_bytes: u64,
    max_stderr_bytes: u64,
) -> Result<(ExitStatus, Vec<u8>, Vec<u8>), CaptureFailure> {
    let stdout = read_bounded(stdout, max_stdout_bytes, ProofStreamKind::Stdout);
    let stderr = read_bounded(stderr, max_stderr_bytes, ProofStreamKind::Stderr);
    let status = async { child.wait().await.map_err(|_| CaptureFailure::Io) };
    let (stdout, stderr, status) = tokio::try_join!(stdout, stderr, status)?;
    Ok((status, stdout, stderr))
}

async fn read_bounded(
    mut reader: impl AsyncRead + Unpin,
    limit: u64,
    stream: ProofStreamKind,
) -> Result<Vec<u8>, CaptureFailure> {
    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 16 * 1024];
    loop {
        let read = reader
            .read(&mut chunk)
            .await
            .map_err(|_| CaptureFailure::Io)?;
        if read == 0 {
            return Ok(bytes);
        }
        if bytes.len() as u64 + read as u64 > limit {
            return Err(CaptureFailure::OutputLimit(stream));
        }
        bytes.extend_from_slice(&chunk[..read]);
    }
}

#[derive(Clone, Copy)]
enum CaptureFailure {
    OutputLimit(ProofStreamKind),
    Io,
}

struct ProcessTreeGuard {
    process_group_id: Option<u32>,
}

impl ProcessTreeGuard {
    fn for_child(child: &Child) -> Self {
        Self {
            #[cfg(unix)]
            process_group_id: child.id(),
            #[cfg(not(unix))]
            process_group_id: None,
        }
    }

    fn kill(&mut self) -> bool {
        let Some(process_group_id) = self.process_group_id else {
            return true;
        };
        let killed = codex_utils_pty::process_group::kill_process_group(process_group_id).is_ok();
        if killed {
            self.process_group_id = None;
        }
        killed
    }
}

impl Drop for ProcessTreeGuard {
    fn drop(&mut self) {
        let _ = self.kill();
    }
}

async fn terminate_child(
    child: &mut Child,
    process_tree: &mut ProcessTreeGuard,
) -> Result<(), &'static str> {
    let process_group_killed = process_tree.kill();
    let child_result = match child.try_wait() {
        Ok(Some(_)) => Ok(()),
        Ok(None) => {
            let direct_kill = child.start_kill();
            match timeout(PROCESS_REAP_TIMEOUT, child.wait()).await {
                Ok(Ok(_)) => Ok(()),
                Ok(Err(_)) if direct_kill.is_err() => {
                    Err("proof_process_direct_termination_failed")
                }
                Ok(Err(_)) => Err("proof_process_reap_failed"),
                Err(_) => Err("proof_process_reap_timed_out"),
            }
        }
        Err(_) => Err("proof_process_status_failed"),
    };
    if !process_group_killed {
        return Err("proof_process_tree_termination_failed");
    }
    child_result
}

fn unavailable_capture(terminal: ProofTerminal) -> ExecutionCapture {
    ExecutionCapture {
        terminal,
        stdout_evidence: ProofStreamEvidence::unavailable(),
        stderr_evidence: ProofStreamEvidence::unavailable(),
        stdout: Vec::new(),
        stderr: Vec::new(),
    }
}

fn completed_terminal(status: ExitStatus) -> ProofTerminal {
    ProofTerminal::Completed {
        success: status.success(),
        exit_code: status.code(),
    }
}

fn validate_execution_paths(spec: &ProofCommandSpec) -> Result<(), ProofError> {
    let observed_program_sha256 = sha256_regular_file(&spec.program, MAX_PROOF_HASH_FILE_BYTES)?;
    if observed_program_sha256 != spec.program_sha256 {
        return Err(ProofError::InvalidInput(
            "proof program content no longer matches the command binding".to_string(),
        ));
    }
    validate_execution_directory(&spec.cwd)
}
