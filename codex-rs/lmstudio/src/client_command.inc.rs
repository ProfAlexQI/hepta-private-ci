async fn verify_lms_cli_provenance(path: &Path) -> io::Result<()> {
    let expected = std::env::var(LMS_CLI_SHA256_ENV).map_err(|_| {
        coded_error(
            io::ErrorKind::PermissionDenied,
            "LMSTUDIO_CLI_DIGEST_REQUIRED",
            format!("set {LMS_CLI_SHA256_ENV}=sha256:<64 lowercase hex>"),
        )
    })?;
    verify_lms_cli_with_expected(path, &expected).await
}

async fn verify_lms_cli_with_expected(path: &Path, expected: &str) -> io::Result<()> {
    let expected = parse_sha256_binding(expected)?;
    let path = path.to_path_buf();
    let actual = tokio::task::spawn_blocking(move || {
        let mut file = File::open(&path)?;
        digest_reader(&mut file)
    })
    .await
    .map_err(|error| {
        coded_error(
            io::ErrorKind::Other,
            "LMSTUDIO_CLI_HASH_TASK_FAILED",
            sanitize_diagnostic(&error.to_string()),
        )
    })??;
    if actual == expected {
        return Ok(());
    }
    Err(coded_error(
        io::ErrorKind::PermissionDenied,
        "LMSTUDIO_CLI_DIGEST_MISMATCH",
        format!("expected=sha256:{expected} actual=sha256:{actual}"),
    ))
}

async fn run_download_command(path: &Path, model: &str, timeout: Duration) -> io::Result<()> {
    let mut command = tokio::process::Command::new(path);
    command.env_clear();
    apply_sanitized_environment(&mut command);
    command
        .args(["get", "--yes", model])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    let mut child = command.spawn().map_err(|error| {
        coded_error(
            io::ErrorKind::Other,
            "LMSTUDIO_CLI_SPAWN_FAILED",
            sanitize_diagnostic(&error.to_string()),
        )
    })?;
    let stderr = child.stderr.take().ok_or_else(|| {
        coded_error(
            io::ErrorKind::Other,
            "LMSTUDIO_CLI_STDERR_UNAVAILABLE",
            "failed to capture bounded stderr",
        )
    })?;

    let completion = tokio::time::timeout(timeout, async {
        let wait = child.wait();
        let read = read_bounded_stderr(stderr);
        tokio::try_join!(wait, read)
    })
    .await;

    let (status, stderr) = match completion {
        Ok(result) => result?,
        Err(_) => {
            let _ = child.kill().await;
            let _ = child.wait().await;
            return Err(coded_error(
                io::ErrorKind::TimedOut,
                "LMSTUDIO_CLI_TIMEOUT",
                format!("model download exceeded {} seconds", timeout.as_secs()),
            ));
        }
    };

    if !status.success() {
        return Err(coded_error(
            io::ErrorKind::Other,
            "LMSTUDIO_CLI_EXIT_FAILURE",
            format!(
                "exit_code={} stderr={}",
                status.code().unwrap_or(-1),
                sanitize_diagnostic(&String::from_utf8_lossy(&stderr))
            ),
        ));
    }
    tracing::info!(
        model = model,
        executable = %path.display(),
        "LM Studio model downloaded explicitly"
    );
    Ok(())
}

fn apply_sanitized_environment(command: &mut tokio::process::Command) {
    const ALLOWED_ENVIRONMENT: &[&str] = &[
        "HOME",
        "USERPROFILE",
        "APPDATA",
        "LOCALAPPDATA",
        "SystemRoot",
        "WINDIR",
        "TEMP",
        "TMP",
        "PATH",
        "LANG",
        "LC_ALL",
    ];
    for &name in ALLOWED_ENVIRONMENT {
        if let Some(value) = std::env::var_os(name) {
            command.env(name, value);
        }
    }
}

async fn read_bounded_stderr(mut stderr: impl AsyncRead + Unpin) -> io::Result<Vec<u8>> {
    let mut captured = Vec::with_capacity(MAX_STDERR_BYTES);
    let mut buffer = [0u8; 1024];
    loop {
        let read = stderr.read(&mut buffer).await?;
        if read == 0 {
            break;
        }
        let remaining = MAX_STDERR_BYTES.saturating_sub(captured.len());
        if remaining != 0 {
            captured.extend_from_slice(&buffer[..read.min(remaining)]);
        }
    }
    Ok(captured)
}
