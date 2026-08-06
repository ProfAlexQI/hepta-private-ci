use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
#[cfg(unix)]
use std::time::Duration;

use tempfile::TempDir;

use super::*;
use crate::store::canonical_json;

fn digest(label: &str) -> Sha256Digest {
    Sha256Digest::for_bytes(label.as_bytes())
}

fn store(temp: &TempDir) -> ProofStore {
    ProofStore::open(temp.path()).expect("proof store")
}

fn subject() -> ProofSubject {
    ProofSubject::new(digest("candidate"), digest("diagnostic-context")).expect("proof subject")
}

fn command(temp: &TempDir, max_stdout_bytes: u64) -> ProofCommandSpec {
    ProofCommandSpec::new(
        std::env::current_exe().expect("current test executable"),
        vec![
            "--list".to_string(),
            "--format".to_string(),
            "terse".to_string(),
        ],
        temp.path(),
        BTreeMap::new(),
        30_000,
        max_stdout_bytes,
        1024 * 1024,
    )
    .expect("proof command")
}

fn invocation(temp: &TempDir, nonce: [u8; 16]) -> ProofInvocation {
    ProofInvocation::new(subject(), nonce, command(temp, 1024 * 1024))
}

fn intent_path(store: &ProofStore, invocation_id: &ProofInvocationId) -> PathBuf {
    store
        .root()
        .join("intents")
        .join(format!("{}.json", invocation_id.digest_suffix()))
}

fn receipt_path(store: &ProofStore, receipt_id: &ProofReceiptId) -> PathBuf {
    store
        .root()
        .join("receipts")
        .join(format!("{}.json", receipt_id.digest_suffix()))
}

fn lock_path(store: &ProofStore, invocation_id: &ProofInvocationId) -> PathBuf {
    store
        .root()
        .join("locks")
        .join(format!("{}.lock", invocation_id.digest_suffix()))
}

fn assert_receipt_read_fails_corrupt(store: &ProofStore, receipt_id: &ProofReceiptId) {
    assert!(matches!(
        store
            .get_receipt(receipt_id)
            .expect_err("corrupt proof record must fail closed"),
        ProofError::Corrupt(_)
    ));
}

fn set_json_pointer(
    value: &serde_json::Value,
    pointer: &str,
    replacement: serde_json::Value,
) -> serde_json::Value {
    let mut value = value.clone();
    *value
        .pointer_mut(pointer)
        .unwrap_or_else(|| panic!("missing JSON pointer {pointer}")) = replacement;
    value
}

fn canonical_with_duplicate_schema_version(bytes: &[u8]) -> Vec<u8> {
    let mut duplicate = br#"{"schema_version":1,"#.to_vec();
    duplicate.extend_from_slice(&bytes[1..]);
    duplicate
}

fn rebind_receipt(receipt: &mut ProofReceipt) {
    receipt.receipt_sha256 = expected_receipt_sha256(receipt).expect("receipt binding");
}

fn assert_typed_receipt_rejected(receipt: &ProofReceipt) {
    let bytes = serde_json::to_vec(receipt).expect("serialize invalid receipt fixture");
    assert!(serde_json::from_slice::<ProofReceipt>(&bytes).is_err());
}

#[cfg(unix)]
fn shell_command(
    temp: &TempDir,
    script: &str,
    arguments: &[PathBuf],
    timeout_ms: u64,
    max_stdout_bytes: u64,
    max_stderr_bytes: u64,
) -> ProofCommandSpec {
    let mut command_arguments = vec![
        "-c".to_string(),
        script.to_string(),
        "hepta-proof".to_string(),
    ];
    command_arguments.extend(
        arguments
            .iter()
            .map(|path| path.to_string_lossy().into_owned()),
    );
    ProofCommandSpec::new(
        fs::canonicalize("/bin/sh").expect("canonical shell"),
        command_arguments,
        temp.path(),
        BTreeMap::new(),
        timeout_ms,
        max_stdout_bytes,
        max_stderr_bytes,
    )
    .expect("shell proof command")
}

#[cfg(unix)]
fn process_tree_command(
    temp: &TempDir,
    timeout_ms: u64,
    max_capture_bytes: u64,
    overflow_stream: Option<ProofStreamKind>,
) -> (ProofCommandSpec, PathBuf, PathBuf) {
    let parent_pid = temp.path().join("proof-parent.pid");
    let child_pid = temp.path().join("proof-child.pid");
    let tail = match overflow_stream {
        Some(ProofStreamKind::Stdout) => "yes x",
        Some(ProofStreamKind::Stderr) => "yes x >&2",
        None => "wait",
    };
    let script = format!("echo \"$$\" > \"$1\"; sleep 30 & echo \"$!\" > \"$2\"; {tail}");
    (
        shell_command(
            temp,
            &script,
            &[parent_pid.clone(), child_pid.clone()],
            timeout_ms,
            max_capture_bytes,
            max_capture_bytes,
        ),
        parent_pid,
        child_pid,
    )
}

#[cfg(unix)]
fn marker_command(temp: &TempDir, marker: &PathBuf, pause: bool) -> ProofCommandSpec {
    let script = if pause {
        "printf x >> \"$1\"; sleep 1"
    } else {
        "printf x >> \"$1\""
    };
    shell_command(
        temp,
        script,
        std::slice::from_ref(marker),
        30_000,
        1024,
        1024,
    )
}

#[cfg(unix)]
async fn wait_for_pid_files(paths: &[&PathBuf]) {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while paths.iter().any(|path| !path.exists()) {
        assert!(
            tokio::time::Instant::now() < deadline,
            "PID files timed out"
        );
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

#[cfg(unix)]
fn read_pid(path: &PathBuf) -> u32 {
    fs::read_to_string(path)
        .expect("read PID")
        .trim()
        .parse()
        .expect("parse PID")
}

#[cfg(unix)]
fn process_exists(pid: u32) -> bool {
    let pid = pid.to_string();
    std::process::Command::new("/bin/kill")
        .args(["-0", pid.as_str()])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .expect("run /bin/kill probe")
        .success()
}

#[cfg(unix)]
async fn assert_processes_gone(pids: &[u32]) {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    loop {
        if pids.iter().all(|pid| !process_exists(*pid)) {
            return;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "proof process tree remained alive: {pids:?}"
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

#[test]
fn canonical_bindings_have_fixed_oracles() {
    let arguments = vec!["--locked".to_string(), "check".to_string()];
    let environment = BTreeMap::from([
        ("LANG".to_string(), "C".to_string()),
        ("TZ".to_string(), "UTC".to_string()),
    ]);
    let program_sha256 = digest("program");
    let binding = CommandBinding {
        domain: PROOF_COMMAND_BINDING_DOMAIN,
        schema_version: PROOF_SCHEMA_VERSION,
        program: "/proof/bin/check",
        program_sha256: &program_sha256,
        arguments: &arguments,
        cwd: "/proof/worktree",
        environment: &environment,
        timeout_ms: 30_000,
        max_stdout_bytes: 4096,
        max_stderr_bytes: 8192,
    };
    let command_binding_sha256 =
        Sha256Digest::for_bytes(&serde_json::to_vec(&binding).expect("binding JSON"));
    assert_eq!(
        command_binding_sha256.as_str(),
        "1a06565c83b074b3345029c4ef452677f431b99c8d2c2df9341f83e5a48c7db4"
    );

    let subject = ProofSubject::new(digest("candidate"), digest("context")).expect("proof subject");
    let nonce_sha256 = digest("nonce");
    let invocation_id =
        ProofInvocationId::for_intent(&subject, &command_binding_sha256, &nonce_sha256);
    assert_eq!(
        invocation_id.as_str(),
        "proof-invocation:v1:1ce9d00e667d92607f02e77e8bbeeca55a2dd2d709fa375f87d05b91e32d57a7"
    );
    let receipt_id = ProofReceiptId::for_invocation(&invocation_id);
    assert_eq!(
        receipt_id.as_str(),
        "proof-receipt:v1:56bb3cb9606f573bc1c07d92de13059476fd007133aee59ffdeb0c79d6c0bf44"
    );

    let intent = ProofIntent {
        schema_version: PROOF_SCHEMA_VERSION,
        invocation_id: invocation_id.clone(),
        subject: subject.clone(),
        command_binding_sha256: command_binding_sha256.clone(),
        nonce_sha256,
    };
    let intent_sha256 =
        Sha256Digest::for_bytes(&canonical_json(&intent).expect("canonical intent"));
    assert_eq!(
        intent_sha256.as_str(),
        "89d8cd551abe8866e58c14a8d14d01ac4517f1416d69c98abb6e4dee15ebd37d"
    );

    let mut receipt = ProofReceipt {
        schema_version: PROOF_SCHEMA_VERSION,
        receipt_id,
        invocation_id,
        subject,
        command_binding_sha256,
        started_at_unix_ms: 100,
        finished_at_unix_ms: 200,
        terminal: ProofTerminal::Completed {
            success: true,
            exit_code: Some(0),
        },
        stdout: ProofStreamEvidence::complete(b"ok\n"),
        stderr: ProofStreamEvidence::complete(b""),
        receipt_sha256: Sha256Digest::for_bytes(b"pending-proof-receipt"),
    };
    receipt.receipt_sha256 = expected_receipt_sha256(&receipt).expect("receipt binding");
    assert_eq!(
        receipt.receipt_sha256.as_str(),
        "4e2b71ba6c12188bb95742b38770f2a85fb63492d66e9872f991c2a34e18f0d1"
    );
    let receipt_sha256 =
        Sha256Digest::for_bytes(&canonical_json(&receipt).expect("canonical receipt"));
    assert_eq!(
        receipt_sha256.as_str(),
        "bf5859187f7782137273cac9d527766a93b8d0925e1deac8660f17f053f5fb5c"
    );
}

#[test]
fn proof_ids_strictly_reject_path_and_digest_substitution() {
    assert!(
        serde_json::from_value::<ProofSubject>(serde_json::json!({
            "candidate_sha256": "short",
            "context_sha256": digest("context").as_str(),
        }))
        .is_err()
    );

    let invalid_invocations = [
        "proof-invocation:v1:short".to_string(),
        format!("proof-invocation:v1:{}", "A".repeat(64)),
        format!("proof-invocation:v1:{}/..", "a".repeat(64)),
        format!("proof-invocation:v1:../{}", "a".repeat(64)),
        format!("other:v1:{}", "a".repeat(64)),
    ];
    for invalid in invalid_invocations {
        assert!(ProofInvocationId::parse(invalid.clone()).is_err());
        assert!(serde_json::from_value::<ProofInvocationId>(serde_json::json!(invalid)).is_err());
    }

    let invalid_receipts = [
        "proof-receipt:v1:short".to_string(),
        format!("proof-receipt:v1:{}", "A".repeat(64)),
        format!("proof-receipt:v1:{}/..", "a".repeat(64)),
        format!("proof-receipt:v1:../{}", "a".repeat(64)),
        format!("other:v1:{}", "a".repeat(64)),
    ];
    for invalid in invalid_receipts {
        assert!(ProofReceiptId::parse(invalid.clone()).is_err());
        assert!(serde_json::from_value::<ProofReceiptId>(serde_json::json!(invalid)).is_err());
    }
}

#[cfg(unix)]
#[tokio::test]
async fn successful_command_claims_once_and_persists_before_return() {
    let temp = TempDir::new().expect("temp dir");
    let store = store(&temp);
    let harness = ProofHarness::new(store.clone());
    let first_invocation = invocation(&temp, [1; 16]);
    let receipt_id = first_invocation.receipt_id();

    let result = harness.run(first_invocation).await.expect("proof run");
    assert!(matches!(
        result.receipt.terminal(),
        ProofTerminal::Completed { success: true, .. }
    ));
    assert!(result.receipt.stdout().is_complete());
    assert!(!result.stdout.is_empty());
    assert_eq!(
        store
            .get_receipt(&receipt_id)
            .expect("read receipt")
            .expect("stored receipt"),
        result.receipt,
    );

    assert!(matches!(
        harness
            .run(invocation(&temp, [1; 16]))
            .await
            .expect_err("replay must block"),
        ProofError::ReplayBlocked { .. }
    ));
    assert!(matches!(
        ProofHarness::new(ProofStore::open(temp.path()).expect("reopen store"))
            .run(invocation(&temp, [1; 16]))
            .await
            .expect_err("restart replay must block"),
        ProofError::ReplayBlocked { .. }
    ));
}

#[cfg(unix)]
#[tokio::test]
async fn concurrent_exact_invocation_has_one_execution_owner() {
    let temp = TempDir::new().expect("temp dir");
    let marker = temp.path().join("execution-count");
    let first = ProofHarness::new(store(&temp));
    let second = ProofHarness::new(ProofStore::open(temp.path()).expect("second store"));
    let first_run = first.run(ProofInvocation::new(
        subject(),
        [11; 16],
        marker_command(&temp, &marker, true),
    ));
    let second_run = second.run(ProofInvocation::new(
        subject(),
        [11; 16],
        marker_command(&temp, &marker, true),
    ));
    let (first_result, second_result) = tokio::join!(first_run, second_run);
    assert_eq!(
        usize::from(first_result.is_ok()) + usize::from(second_result.is_ok()),
        1
    );
    assert_eq!(fs::read(&marker).expect("execution marker"), b"x");

    let replay = ProofInvocation::new(subject(), [11; 16], marker_command(&temp, &marker, true));
    assert!(matches!(
        ProofHarness::new(ProofStore::open(temp.path()).expect("replay store"))
            .run(replay)
            .await
            .expect_err("winner must block later replay"),
        ProofError::ReplayBlocked { .. }
    ));
    assert_eq!(fs::read(&marker).expect("execution marker"), b"x");
}

#[test]
fn command_contract_rejects_relative_and_unbounded_inputs() {
    let temp = TempDir::new().expect("temp dir");
    let executable = std::env::current_exe().expect("current test executable");
    assert!(
        ProofCommandSpec::new(
            "relative-program",
            Vec::new(),
            temp.path(),
            BTreeMap::new(),
            1,
            1,
            1,
        )
        .is_err()
    );
    assert!(
        ProofCommandSpec::new(
            &executable,
            Vec::new(),
            temp.path(),
            BTreeMap::new(),
            MAX_PROOF_TIMEOUT_MS + 1,
            1,
            1,
        )
        .is_err()
    );
    assert!(
        ProofCommandSpec::new(
            &executable,
            vec!["x".repeat(MAX_PROOF_ARGUMENT_BYTES + 1)],
            temp.path(),
            BTreeMap::new(),
            1,
            1,
            1,
        )
        .is_err()
    );
    assert!(
        ProofCommandSpec::new(
            executable,
            Vec::new(),
            temp.path(),
            BTreeMap::from([("BAD-NAME".to_string(), "value".to_string())]),
            1,
            1,
            1,
        )
        .is_err()
    );
}

#[test]
fn regular_file_hash_is_bounded_and_rejects_symlinks() {
    let temp = TempDir::new().expect("temp dir");
    let file = temp.path().join("candidate.bin");
    fs::write(&file, b"candidate").expect("write fixture");
    assert_eq!(
        sha256_regular_file(&file, 64).expect("hash file"),
        digest("candidate"),
    );
    assert!(sha256_regular_file(&file, 1).is_err());

    #[cfg(unix)]
    {
        let link = temp.path().join("candidate-link");
        std::os::unix::fs::symlink(&file, &link).expect("create symlink");
        assert!(sha256_regular_file(&link, 64).is_err());
    }
}

#[test]
fn subject_context_substitution_changes_invocation_identity() {
    let temp = TempDir::new().expect("temp dir");
    let first = ProofInvocation::new(
        ProofSubject::new(digest("candidate"), digest("context-a")).expect("subject"),
        [9; 16],
        command(&temp, 1024),
    );
    let substituted = ProofInvocation::new(
        ProofSubject::new(digest("candidate"), digest("context-b")).expect("subject"),
        [9; 16],
        command(&temp, 1024),
    );
    assert_ne!(first.invocation_id(), substituted.invocation_id());
}

#[cfg(unix)]
#[tokio::test]
async fn output_limit_is_terminal_and_replay_remains_blocked() {
    for (stream, nonce) in [
        (ProofStreamKind::Stdout, [2; 16]),
        (ProofStreamKind::Stderr, [15; 16]),
    ] {
        let temp = TempDir::new().expect("temp dir");
        let store = store(&temp);
        let (command, parent_pid_path, child_pid_path) =
            process_tree_command(&temp, 30_000, 1, Some(stream));
        let invocation = ProofInvocation::new(subject(), nonce, command);
        let receipt_id = invocation.receipt_id();

        let result = ProofHarness::new(store.clone())
            .run(invocation)
            .await
            .expect("proof run");
        assert_eq!(
            result.receipt.terminal(),
            &ProofTerminal::OutputLimitExceeded { stream }
        );
        assert!(!result.receipt.stdout().is_complete());
        assert!(!result.receipt.stderr().is_complete());
        assert!(result.stdout.is_empty());
        assert!(result.stderr.is_empty());
        wait_for_pid_files(&[&parent_pid_path, &child_pid_path]).await;
        let parent_pid = read_pid(&parent_pid_path);
        let child_pid = read_pid(&child_pid_path);
        assert_processes_gone(&[parent_pid, child_pid]).await;
        assert!(
            store
                .get_receipt(&receipt_id)
                .expect("read receipt")
                .is_some()
        );
    }
}

#[cfg(unix)]
#[tokio::test]
async fn executable_substitution_is_terminal_and_never_spawns() {
    let temp = TempDir::new().expect("temp dir");
    let executable = temp.path().join("proof-executable");
    fs::copy("/bin/sh", &executable).expect("copy executable");
    let command = ProofCommandSpec::new(
        &executable,
        Vec::new(),
        temp.path(),
        BTreeMap::new(),
        30_000,
        1024,
        1024,
    )
    .expect("proof command");
    let marker = temp.path().join("substituted-executable-ran");
    fs::write(
        &executable,
        format!("#!/bin/sh\n: > '{}'\n", marker.display()),
    )
    .expect("replace executable with valid script");
    let result = ProofHarness::new(store(&temp))
        .run(ProofInvocation::new(subject(), [4; 16], command))
        .await
        .expect("terminal receipt");
    assert_eq!(
        result.receipt.terminal(),
        &ProofTerminal::NotStarted {
            reason_code: "proof_execution_path_invalid".to_string(),
        }
    );
    assert!(!marker.exists(), "substituted executable must not spawn");
}

#[cfg(unix)]
#[tokio::test]
async fn pending_intent_survives_restart_and_blocks_execution() {
    let temp = TempDir::new().expect("temp dir");
    let store = store(&temp);
    let invocation = invocation(&temp, [5; 16]);
    assert_eq!(
        store
            .claim_intent(&invocation.intent)
            .expect("persist intent"),
        ProofAppendDisposition::Inserted,
    );
    assert!(matches!(
        ProofHarness::new(ProofStore::open(temp.path()).expect("reopen store"))
            .run(invocation)
            .await
            .expect_err("pending replay must block"),
        ProofError::ReplayBlocked { .. }
    ));
}

#[cfg(unix)]
#[tokio::test]
async fn canonical_receipt_and_intent_substitution_fail_closed() {
    let temp = TempDir::new().expect("temp dir");
    let store = store(&temp);
    let invocation = invocation(&temp, [6; 16]);
    let invocation_id = invocation.invocation_id().clone();
    let receipt_id = invocation.receipt_id();
    ProofHarness::new(store.clone())
        .run(invocation)
        .await
        .expect("proof run");

    let receipt_path = receipt_path(&store, &receipt_id);
    let intent_path = intent_path(&store, &invocation_id);
    let original_receipt = fs::read(&receipt_path).expect("read receipt");
    let original_intent = fs::read(&intent_path).expect("read intent");
    let receipt: serde_json::Value =
        serde_json::from_slice(&original_receipt).expect("decode receipt");
    let intent: serde_json::Value =
        serde_json::from_slice(&original_intent).expect("decode intent");

    let other_invocation = ProofInvocationId::parse(format!(
        "proof-invocation:v1:{}",
        digest("other-invocation").as_str()
    ))
    .expect("other invocation ID");
    let other_receipt = ProofReceiptId::parse(format!(
        "proof-receipt:v1:{}",
        digest("other-receipt").as_str()
    ))
    .expect("other receipt ID");

    let intent_substitutions = [
        ("/schema_version", serde_json::json!(2)),
        (
            "/invocation_id",
            serde_json::json!(other_invocation.as_str()),
        ),
        (
            "/subject/candidate_sha256",
            serde_json::json!(digest("other-candidate").as_str()),
        ),
        (
            "/subject/context_sha256",
            serde_json::json!(digest("other-context").as_str()),
        ),
        (
            "/command_binding_sha256",
            serde_json::json!(digest("other-command").as_str()),
        ),
        (
            "/nonce_sha256",
            serde_json::json!(digest("other-nonce").as_str()),
        ),
    ];
    for (pointer, replacement) in intent_substitutions {
        let substituted = set_json_pointer(&intent, pointer, replacement);
        fs::write(
            &intent_path,
            canonical_json(&substituted).expect("canonical substituted intent"),
        )
        .expect("replace intent");
        assert_receipt_read_fails_corrupt(&store, &receipt_id);
        fs::write(&intent_path, &original_intent).expect("restore intent");
    }

    let mut unknown_intent = intent;
    unknown_intent["unknown"] = serde_json::json!(true);
    fs::write(
        &intent_path,
        canonical_json(&unknown_intent).expect("canonical unknown intent"),
    )
    .expect("replace intent");
    assert_receipt_read_fails_corrupt(&store, &receipt_id);
    fs::write(
        &intent_path,
        canonical_with_duplicate_schema_version(&original_intent),
    )
    .expect("duplicate intent field");
    assert_receipt_read_fails_corrupt(&store, &receipt_id);
    let mut spaced_intent = b" \n".to_vec();
    spaced_intent.extend_from_slice(&original_intent);
    fs::write(&intent_path, spaced_intent).expect("noncanonical intent");
    assert_receipt_read_fails_corrupt(&store, &receipt_id);
    fs::write(&intent_path, &original_intent).expect("restore intent");

    let receipt_substitutions = [
        ("/schema_version", serde_json::json!(2)),
        ("/receipt_id", serde_json::json!(other_receipt.as_str())),
        (
            "/invocation_id",
            serde_json::json!(other_invocation.as_str()),
        ),
        (
            "/subject/candidate_sha256",
            serde_json::json!(digest("other-candidate").as_str()),
        ),
        (
            "/subject/context_sha256",
            serde_json::json!(digest("other-context").as_str()),
        ),
        (
            "/command_binding_sha256",
            serde_json::json!(digest("other-command").as_str()),
        ),
        ("/started_at_unix_ms", serde_json::json!(0)),
        ("/finished_at_unix_ms", serde_json::json!(u64::MAX)),
        (
            "/terminal",
            serde_json::json!({"kind":"completed","success":false,"exit_code":1}),
        ),
        ("/stdout/complete", serde_json::json!(false)),
        ("/stdout/bytes_observed", serde_json::json!(0)),
        ("/stdout/sha256", serde_json::Value::Null),
        ("/stderr/complete", serde_json::json!(false)),
        ("/stderr/bytes_observed", serde_json::json!(1)),
        (
            "/stderr/sha256",
            serde_json::json!(digest("other-stderr").as_str()),
        ),
        (
            "/receipt_sha256",
            serde_json::json!(digest("other-receipt-content").as_str()),
        ),
    ];
    for (pointer, replacement) in receipt_substitutions {
        let substituted = set_json_pointer(&receipt, pointer, replacement);
        fs::write(
            &receipt_path,
            canonical_json(&substituted).expect("canonical substituted receipt"),
        )
        .expect("replace receipt");
        assert_receipt_read_fails_corrupt(&store, &receipt_id);
        fs::write(&receipt_path, &original_receipt).expect("restore receipt");
    }

    let mut unknown_receipt = receipt;
    unknown_receipt["unknown"] = serde_json::json!(true);
    fs::write(
        &receipt_path,
        canonical_json(&unknown_receipt).expect("canonical unknown receipt"),
    )
    .expect("replace receipt");
    assert_receipt_read_fails_corrupt(&store, &receipt_id);
    fs::write(
        &receipt_path,
        canonical_with_duplicate_schema_version(&original_receipt),
    )
    .expect("duplicate receipt field");
    assert_receipt_read_fails_corrupt(&store, &receipt_id);
    let mut spaced_receipt = b" \n".to_vec();
    spaced_receipt.extend_from_slice(&original_receipt);
    fs::write(&receipt_path, spaced_receipt).expect("noncanonical receipt");
    assert_receipt_read_fails_corrupt(&store, &receipt_id);
}

#[cfg(unix)]
#[tokio::test]
async fn noncanonical_or_wrong_identity_receipt_fails_closed() {
    let temp = TempDir::new().expect("temp dir");
    let store = store(&temp);
    let invocation = invocation(&temp, [7; 16]);
    let receipt_id = invocation.receipt_id();
    ProofHarness::new(store.clone())
        .run(invocation)
        .await
        .expect("proof run");
    let path = receipt_path(&store, &receipt_id);
    let original = fs::read(&path).expect("receipt bytes");
    let mut spaced = b" \n".to_vec();
    spaced.extend_from_slice(&original);
    fs::write(&path, spaced).expect("noncanonical receipt");
    assert!(matches!(
        store
            .get_receipt(&receipt_id)
            .expect_err("noncanonical JSON must fail"),
        ProofError::Corrupt(_)
    ));

    let mut value: serde_json::Value = serde_json::from_slice(&original).expect("decode receipt");
    value["receipt_id"] = serde_json::json!(format!(
        "proof-receipt:v1:{}",
        digest("other-receipt").as_str()
    ));
    fs::write(&path, canonical_json(&value).expect("canonical JSON")).expect("wrong ID receipt");
    assert!(matches!(
        store
            .get_receipt(&receipt_id)
            .expect_err("wrong identity must fail"),
        ProofError::Corrupt(_)
    ));
}

#[cfg(unix)]
#[tokio::test]
async fn public_receipt_types_self_validate_cross_field_invariants() {
    let temp = TempDir::new().expect("temp dir");
    let store = store(&temp);
    let invocation = invocation(&temp, [14; 16]);
    let receipt_id = invocation.receipt_id();
    ProofHarness::new(store.clone())
        .run(invocation)
        .await
        .expect("proof run");
    let valid = store
        .get_receipt(&receipt_id)
        .expect("read receipt")
        .expect("stored receipt");

    let mut reversed_time = valid.clone();
    reversed_time.started_at_unix_ms = 2;
    reversed_time.finished_at_unix_ms = 1;
    rebind_receipt(&mut reversed_time);
    assert_typed_receipt_rejected(&reversed_time);

    let mut exit_mismatch = valid.clone();
    exit_mismatch.terminal = ProofTerminal::Completed {
        success: true,
        exit_code: Some(1),
    };
    rebind_receipt(&mut exit_mismatch);
    assert_typed_receipt_rejected(&exit_mismatch);

    let mut interrupted_with_complete_streams = valid.clone();
    interrupted_with_complete_streams.terminal = ProofTerminal::TimedOut;
    rebind_receipt(&mut interrupted_with_complete_streams);
    assert_typed_receipt_rejected(&interrupted_with_complete_streams);

    let mut indeterminate_stream_disagreement = valid.clone();
    indeterminate_stream_disagreement.terminal = ProofTerminal::Indeterminate {
        reason_code: "proof_test_indeterminate".to_string(),
    };
    indeterminate_stream_disagreement.stderr = ProofStreamEvidence::unavailable();
    rebind_receipt(&mut indeterminate_stream_disagreement);
    assert_typed_receipt_rejected(&indeterminate_stream_disagreement);

    let mut invalid_stream_shape = valid.clone();
    invalid_stream_shape.stdout.complete = false;
    rebind_receipt(&mut invalid_stream_shape);
    assert_typed_receipt_rejected(&invalid_stream_shape);

    let mut invalid_reason = valid;
    invalid_reason.terminal = ProofTerminal::NotStarted {
        reason_code: "NOT-CANONICAL".to_string(),
    };
    invalid_reason.stdout = ProofStreamEvidence::unavailable();
    invalid_reason.stderr = ProofStreamEvidence::unavailable();
    rebind_receipt(&mut invalid_reason);
    assert_typed_receipt_rejected(&invalid_reason);

    assert!(
        serde_json::from_value::<ProofStreamEvidence>(serde_json::json!({
            "complete": false,
            "bytes_observed": 1,
            "sha256": null,
        }))
        .is_err()
    );
    assert!(
        serde_json::from_value::<ProofTerminal>(serde_json::json!({
            "kind": "not_started",
            "reason_code": "NOT-CANONICAL",
        }))
        .is_err()
    );
}

#[cfg(unix)]
#[tokio::test]
async fn retained_receipt_with_deleted_intent_fails_before_reexecution() {
    let temp = TempDir::new().expect("temp dir");
    let store = store(&temp);
    let first_invocation = invocation(&temp, [8; 16]);
    let invocation_id = first_invocation.invocation_id().clone();
    ProofHarness::new(store.clone())
        .run(first_invocation)
        .await
        .expect("proof run");
    fs::remove_file(intent_path(&store, &invocation_id)).expect("delete intent fixture");
    assert!(matches!(
        ProofHarness::new(store)
            .run(invocation(&temp, [8; 16]))
            .await
            .expect_err("orphan receipt must fail before execution"),
        ProofError::Corrupt(_)
    ));
}

#[cfg(unix)]
#[tokio::test]
async fn deleting_the_entire_record_pair_demonstrates_local_revival() {
    let temp = TempDir::new().expect("temp dir");
    let store = store(&temp);
    let marker = temp.path().join("rollback-revival-marker");
    let first = ProofInvocation::new(subject(), [16; 16], marker_command(&temp, &marker, false));
    let invocation_id = first.invocation_id().clone();
    let receipt_id = first.receipt_id();
    ProofHarness::new(store.clone())
        .run(first)
        .await
        .expect("first local observation");
    assert_eq!(fs::read(&marker).expect("first marker"), b"x");

    fs::remove_file(intent_path(&store, &invocation_id)).expect("remove intent fixture");
    fs::remove_file(receipt_path(&store, &receipt_id)).expect("remove receipt fixture");

    ProofHarness::new(ProofStore::open(temp.path()).expect("reopen rolled-back store"))
        .run(ProofInvocation::new(
            subject(),
            [16; 16],
            marker_command(&temp, &marker, false),
        ))
        .await
        .expect("local root rollback revives the invocation");
    assert_eq!(fs::read(&marker).expect("revival marker"), b"xx");
}

#[cfg(unix)]
#[tokio::test]
async fn timeout_terminal_cleans_the_same_process_group() {
    let temp = TempDir::new().expect("temp dir");
    let store = store(&temp);
    let (command, parent_pid_path, child_pid_path) = process_tree_command(&temp, 500, 1024, None);
    let invocation = ProofInvocation::new(subject(), [12; 16], command);
    let receipt_id = invocation.receipt_id();
    let result = ProofHarness::new(store.clone())
        .run(invocation)
        .await
        .expect("timeout terminal");
    assert_eq!(result.receipt.terminal(), &ProofTerminal::TimedOut);
    wait_for_pid_files(&[&parent_pid_path, &child_pid_path]).await;
    let parent_pid = read_pid(&parent_pid_path);
    let child_pid = read_pid(&child_pid_path);
    assert_processes_gone(&[parent_pid, child_pid]).await;
    assert!(
        store
            .get_receipt(&receipt_id)
            .expect("read receipt")
            .is_some()
    );
}

#[cfg(unix)]
#[tokio::test]
async fn cancellation_leaves_pending_intent_and_blocks_restart_replay() {
    let temp = TempDir::new().expect("temp dir");
    let store = store(&temp);
    let harness = ProofHarness::new(store.clone());
    let (command, parent_pid_path, child_pid_path) =
        process_tree_command(&temp, 30_000, 1024, None);
    let invocation = ProofInvocation::new(subject(), [13; 16], command);
    let receipt_id = invocation.receipt_id();
    let task = tokio::spawn(async move { harness.run(invocation).await });
    wait_for_pid_files(&[&parent_pid_path, &child_pid_path]).await;
    let parent_pid = read_pid(&parent_pid_path);
    let child_pid = read_pid(&child_pid_path);
    task.abort();
    assert!(
        task.await
            .expect_err("proof task must cancel")
            .is_cancelled()
    );
    assert_processes_gone(&[parent_pid, child_pid]).await;

    let (replay_command, _, _) = process_tree_command(&temp, 30_000, 1024, None);
    let replay = ProofInvocation::new(subject(), [13; 16], replay_command);
    assert!(matches!(
        ProofHarness::new(store.clone())
            .run(replay)
            .await
            .expect_err("cancelled replay must block"),
        ProofError::ReplayBlocked { .. }
    ));
    assert!(
        store
            .get_receipt(&receipt_id)
            .expect("read receipt")
            .is_none()
    );
}

#[test]
fn lock_records_are_exclusive_within_an_intact_root() {
    let temp = TempDir::new().expect("temp dir");
    let store = store(&temp);
    let invocation = invocation(&temp, [3; 16]);
    let lock = store
        .acquire_lock(invocation.invocation_id())
        .expect("first lock");
    assert!(store.acquire_lock(invocation.invocation_id()).is_err());
    drop(lock);
    assert!(store.acquire_lock(invocation.invocation_id()).is_ok());
}

#[cfg(unix)]
#[tokio::test]
async fn stale_lock_record_fails_before_execution() {
    let temp = TempDir::new().expect("temp dir");
    let store = store(&temp);
    let marker = temp.path().join("stale-lock-marker");
    let invocation =
        ProofInvocation::new(subject(), [17; 16], marker_command(&temp, &marker, false));
    fs::write(lock_path(&store, invocation.invocation_id()), b"stale-lock")
        .expect("write stale lock fixture");
    assert!(matches!(
        ProofHarness::new(store)
            .run(invocation)
            .await
            .expect_err("stale lock must fail closed"),
        ProofError::StoreUnavailable(_)
    ));
    assert!(!marker.exists(), "stale lock must prevent process spawn");
}

#[cfg(not(unix))]
#[tokio::test]
async fn unsupported_platform_returns_not_started_before_spawn() {
    let temp = TempDir::new().expect("temp dir");
    let result = ProofHarness::new(store(&temp))
        .run(invocation(&temp, [18; 16]))
        .await
        .expect("unsupported terminal receipt");
    assert_eq!(
        result.receipt.terminal(),
        &ProofTerminal::NotStarted {
            reason_code: "proof_process_containment_unsupported".to_string(),
        }
    );
}
