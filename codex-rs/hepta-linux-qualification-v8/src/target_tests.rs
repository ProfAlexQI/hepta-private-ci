use pretty_assertions::assert_eq;

use super::*;
use crate::CANDIDATE_HEAD;
use crate::CANDIDATE_TREE;

fn hex64(byte: char) -> String {
    std::iter::repeat_n(byte, 64).collect()
}

fn boot() -> TargetBootIdentityV1 {
    TargetBootIdentityV1 {
        boot_epoch: 1,
        boot_id: "01234567-89ab-cdef-0123-456789abcdef".into(),
    }
}

fn expected_attempt(runner_snapshot_sha256: String) -> AttemptIdentityV8 {
    AttemptIdentityV8 {
        attempt_nonce: hex64('1'),
        barrier_generation: 7,
        candidate_head: CANDIDATE_HEAD.into(),
        candidate_tree: CANDIDATE_TREE.into(),
        driver_manifest_sha256: hex64('2'),
        profile_manifest_sha256: hex64('3'),
        parameter_manifest_sha256: hex64('4'),
        machine_id_sha256: hex64('5'),
        runner_snapshot_sha256,
        restore_plan_sha256: hex64('6'),
    }
}

fn runner_process(
    runner_id: u32,
    role: RunnerProcessRoleV1,
    pid: u32,
    state: ProcessExecutionStateV1,
) -> RunnerProcessV1 {
    RunnerProcessV1 {
        role,
        pid,
        start_ticks: u64::from(pid) * 10,
        pidfd_token_sha256: format!("{pid:064x}"),
        uid: 1000,
        gid: 1000,
        pgid: 3_075_593,
        sid: 3_075_593,
        cgroup_v2_path: "/user.slice/user-1000.slice/session.scope".into(),
        executable_sha256: match role {
            RunnerProcessRoleV1::RunSh { .. } => hex64('2'),
            RunnerProcessRoleV1::RunHelperSh { .. } => hex64('3'),
            RunnerProcessRoleV1::RunnerListener { .. } => hex64('4'),
            RunnerProcessRoleV1::Worker { .. } => hex64('5'),
        },
        cwd_identity_sha256: hex64('6'),
        argv_sha256: hex64('7'),
        environ_sha256: hex64('8'),
        runner_name: format!("runner-{runner_id}"),
        runner_labels_sha256: if runner_id == 22 {
            hex64('9')
        } else {
            hex64('a')
        },
        runner_version: "2.327.1".into(),
        runner_config_sha256: if runner_id == 22 {
            hex64('b')
        } else {
            hex64('c')
        },
        workroot_identity_sha256: if runner_id == 22 {
            hex64('d')
        } else {
            hex64('e')
        },
        execution_state: state,
    }
}

fn group(state: ProcessExecutionStateV1) -> SharedProcessGroupV1 {
    let processes = vec![
        runner_process(22, RunnerProcessRoleV1::RunSh { runner_id: 22 }, 101, state),
        runner_process(
            22,
            RunnerProcessRoleV1::RunHelperSh { runner_id: 22 },
            102,
            state,
        ),
        runner_process(
            22,
            RunnerProcessRoleV1::RunnerListener { runner_id: 22 },
            103,
            state,
        ),
        runner_process(23, RunnerProcessRoleV1::RunSh { runner_id: 23 }, 201, state),
        runner_process(
            23,
            RunnerProcessRoleV1::RunHelperSh { runner_id: 23 },
            202,
            state,
        ),
        runner_process(
            23,
            RunnerProcessRoleV1::RunnerListener { runner_id: 23 },
            203,
            state,
        ),
    ];
    SharedProcessGroupV1 {
        schema: SHARED_PROCESS_GROUP_SCHEMA_V1.into(),
        pgid: 3_075_593,
        sid: 3_075_593,
        session_leader: SessionLeaderV1::OrphanedAbsent {
            former_pid: 3_075_593,
        },
        enumeration: ProcessGroupEnumerationV1::Complete,
        enumerated_group_member_pids: vec![101, 102, 103, 201, 202, 203],
        enumerated_worker_pids: Vec::new(),
        processes,
    }
}

fn snapshot(
    phase: RunnerSnapshotPhaseV1,
    state: ProcessExecutionStateV1,
    sequence: u64,
) -> RunnerSnapshotV1 {
    let mut snapshot = RunnerSnapshotV1 {
        schema: RUNNER_SNAPSHOT_SCHEMA_V1.into(),
        phase,
        attempt_identity_sha256: hex64('f'),
        boot: boot(),
        observation_sequence: sequence,
        monotonic_ns: sequence * 1_000,
        snapshot_sha256: String::new(),
        group: group(state),
    };
    snapshot.snapshot_sha256 = snapshot.computed_snapshot_sha256_v1();
    snapshot
}

fn runner_lifecycle() -> RunnerLifecycleEvidenceV1 {
    let mut evidence = RunnerLifecycleEvidenceV1 {
        schema: RUNNER_LIFECYCLE_SCHEMA_V1.into(),
        pre_stop: snapshot(
            RunnerSnapshotPhaseV1::PreStop,
            ProcessExecutionStateV1::Running,
            10,
        ),
        stopped: snapshot(
            RunnerSnapshotPhaseV1::Stopped,
            ProcessExecutionStateV1::Stopped,
            40,
        ),
        post_restore: snapshot(
            RunnerSnapshotPhaseV1::PostRestore,
            ProcessExecutionStateV1::Running,
            80,
        ),
        stop_once: StopOnceEvidenceV1 {
            attempt_identity_sha256: hex64('f'),
            operation: StopOperationV1::ProcessGroupSigstop { pgid: 3_075_593 },
            delivery_count: 1,
            intent_sequence: 20,
            effect_sequence: 30,
            pre_stop_snapshot_sha256: String::new(),
            stopped_snapshot_sha256: String::new(),
        },
        restore_once: RestoreOnceEvidenceV1 {
            attempt_identity_sha256: hex64('f'),
            operation: RestoreOperationV1::ProcessGroupSigcont { pgid: 3_075_593 },
            delivery_count: 1,
            intent_sequence: 60,
            effect_sequence: 70,
            stopped_snapshot_sha256: String::new(),
            post_restore_snapshot_sha256: String::new(),
        },
    };
    evidence.stop_once.pre_stop_snapshot_sha256 = evidence.pre_stop.snapshot_sha256.clone();
    evidence.stop_once.stopped_snapshot_sha256 = evidence.stopped.snapshot_sha256.clone();
    evidence.restore_once.stopped_snapshot_sha256 = evidence.stopped.snapshot_sha256.clone();
    evidence.restore_once.post_restore_snapshot_sha256 =
        evidence.post_restore.snapshot_sha256.clone();
    evidence
}

fn terminal_observation(sequence: u64) -> AndroidUnitObservationV1 {
    AndroidUnitObservationV1 {
        boot: boot(),
        observation_sequence: sequence,
        monotonic_ns: sequence * 1_000,
        active_state: UnitActiveStateV1::Active,
        sub_state: UnitSubStateV1::Exited,
        remain_after_exit: RemainAfterExitV1::Enabled,
        result: UnitResultV1::Success,
        main_pid: 0,
        invocation_id: "0123456789abcdef0123456789abcdef".into(),
        control_group: String::new(),
        tasks_current: None,
        restart_count: 0,
        fragment_path: format!(
            "/run/user/1000/systemd/transient/{EXPECTED_ANDROID_TERMINAL_UNIT_V1}"
        ),
    }
}

fn android_terminal_unit() -> AndroidTerminalUnitV1 {
    let mut evidence = AndroidTerminalUnitV1 {
        schema: ANDROID_TERMINAL_UNIT_SCHEMA_V1.into(),
        attempt_identity_sha256: hex64('f'),
        unit_name: EXPECTED_ANDROID_TERMINAL_UNIT_V1.into(),
        manager: SystemdManagerV1::User { uid: 1000 },
        observations: vec![terminal_observation(100), terminal_observation(110)],
        root_proc_scan: RootProcScanProofV1 {
            schema: ROOT_PROC_SCAN_SCHEMA_V1.into(),
            attempt_identity_sha256: hex64('f'),
            boot: boot(),
            observation_sequence: 120,
            monotonic_ns: 120_000,
            observer_effective_uid: 0,
            observer_effective_gid: 0,
            observer_elf_sha256: hex64('1'),
            procfs_mount_identity_sha256: hex64('2'),
            unit_invocation_id: "0123456789abcdef0123456789abcdef".into(),
            workroot_device: 42,
            workroot_inode: 43,
            workroot_identity_sha256: hex64('3'),
            enumerated_process_count: 300,
            enumerated_fd_count: 1_000,
            unreadable_process_count: 0,
            unreadable_fd_table_count: 0,
            matching_process_pids: Vec::new(),
            open_workroot_fds: Vec::new(),
            completeness: ProcScanCompletenessV1::Complete,
            proof_sha256: String::new(),
        },
    };
    evidence.root_proc_scan.proof_sha256 = evidence.root_proc_scan.computed_proof_sha256_v1();
    evidence
}

fn bind_runner_attempt(evidence: &mut RunnerLifecycleEvidenceV1, attempt_sha256: &str) {
    evidence.pre_stop.attempt_identity_sha256 = attempt_sha256.into();
    evidence.stopped.attempt_identity_sha256 = attempt_sha256.into();
    evidence.post_restore.attempt_identity_sha256 = attempt_sha256.into();
    evidence.stop_once.attempt_identity_sha256 = attempt_sha256.into();
    evidence.restore_once.attempt_identity_sha256 = attempt_sha256.into();
}

fn bind_android_attempt(evidence: &mut AndroidTerminalUnitV1, attempt_sha256: &str) {
    evidence.attempt_identity_sha256 = attempt_sha256.into();
    evidence.root_proc_scan.attempt_identity_sha256 = attempt_sha256.into();
    evidence.root_proc_scan.proof_sha256 = evidence.root_proc_scan.computed_proof_sha256_v1();
}

fn reseal_and_bind_runner(evidence: &mut RunnerLifecycleEvidenceV1) -> AttemptIdentityV8 {
    evidence.pre_stop.snapshot_sha256 = evidence.pre_stop.computed_snapshot_sha256_v1();
    evidence.stopped.snapshot_sha256 = evidence.stopped.computed_snapshot_sha256_v1();
    evidence.post_restore.snapshot_sha256 = evidence.post_restore.computed_snapshot_sha256_v1();
    evidence.stop_once.pre_stop_snapshot_sha256 = evidence.pre_stop.snapshot_sha256.clone();
    evidence.stop_once.stopped_snapshot_sha256 = evidence.stopped.snapshot_sha256.clone();
    evidence.restore_once.stopped_snapshot_sha256 = evidence.stopped.snapshot_sha256.clone();
    evidence.restore_once.post_restore_snapshot_sha256 =
        evidence.post_restore.snapshot_sha256.clone();
    let attempt = expected_attempt(evidence.pre_stop.snapshot_sha256.clone());
    bind_runner_attempt(evidence, &attempt.sha256().unwrap());
    attempt
}

fn bound_runner_lifecycle() -> (AttemptIdentityV8, RunnerLifecycleEvidenceV1) {
    let mut evidence = runner_lifecycle();
    let attempt = reseal_and_bind_runner(&mut evidence);
    (attempt, evidence)
}

fn bound_android_terminal_unit() -> (AttemptIdentityV8, AndroidTerminalUnitV1) {
    let attempt = expected_attempt(hex64('7'));
    let mut evidence = android_terminal_unit();
    bind_android_attempt(&mut evidence, &attempt.sha256().unwrap());
    (attempt, evidence)
}

fn target_evidence() -> (AttemptIdentityV8, TargetEvidenceV1) {
    let mut runners = runner_lifecycle();
    let attempt = expected_attempt(runners.pre_stop.snapshot_sha256.clone());
    let attempt_sha256 = attempt.sha256().unwrap();
    bind_runner_attempt(&mut runners, &attempt_sha256);
    let mut android_terminal_unit = android_terminal_unit();
    bind_android_attempt(&mut android_terminal_unit, &attempt_sha256);
    let evidence = TargetEvidenceV1 {
        schema: TARGET_EVIDENCE_SCHEMA_V1.into(),
        attempt_identity_sha256: attempt_sha256,
        runners,
        android_terminal_unit,
    };
    (attempt, evidence)
}

#[test]
fn exact_target_evidence_passes() {
    let (attempt, evidence) = target_evidence();
    assert_eq!(
        verify_target_evidence_v1(&evidence, &attempt).map_err(|error| error.to_string()),
        Ok(())
    );
}

#[test]
fn listeners_may_restart_without_runner_identity_drift() {
    let (_, mut evidence) = bound_runner_lifecycle();
    for process in &mut evidence.post_restore.group.processes {
        if matches!(process.role, RunnerProcessRoleV1::RunnerListener { .. }) {
            process.pid += 1_000;
            process.start_ticks += 10_000;
            process.pidfd_token_sha256 = format!("{:064x}", process.pid);
        }
    }
    evidence.post_restore.group.enumerated_group_member_pids =
        vec![101, 102, 201, 202, 1_103, 1_203];
    let attempt = reseal_and_bind_runner(&mut evidence);
    assert_eq!(
        verify_runner_lifecycle_v1(&evidence, &attempt).map_err(|error| error.to_string()),
        Ok(())
    );
}

#[test]
fn extra_group_member_is_rejected() {
    let (_, mut evidence) = bound_runner_lifecycle();
    for snapshot in [
        &mut evidence.pre_stop,
        &mut evidence.stopped,
        &mut evidence.post_restore,
    ] {
        snapshot.group.enumerated_group_member_pids.push(999);
    }
    let attempt = reseal_and_bind_runner(&mut evidence);
    let error = verify_runner_lifecycle_v1(&evidence, &attempt)
        .unwrap_err()
        .to_string();
    assert!(error.contains("enumeration does not exactly match"));
}

#[test]
fn partial_stop_is_rejected() {
    let (_, mut evidence) = bound_runner_lifecycle();
    evidence.stopped.group.processes[0].execution_state = ProcessExecutionStateV1::Running;
    let attempt = reseal_and_bind_runner(&mut evidence);
    let error = verify_runner_lifecycle_v1(&evidence, &attempt)
        .unwrap_err()
        .to_string();
    assert!(error.contains("partial or wrong"));
}

#[test]
fn process_group_drift_is_rejected() {
    let (_, mut evidence) = bound_runner_lifecycle();
    evidence.post_restore.group.pgid += 1;
    for process in &mut evidence.post_restore.group.processes {
        process.pgid += 1;
    }
    let attempt = reseal_and_bind_runner(&mut evidence);
    let error = verify_runner_lifecycle_v1(&evidence, &attempt)
        .unwrap_err()
        .to_string();
    assert!(error.contains("orphaned PGID/SID") || error.contains("drifted"));
}

#[test]
fn stop_and_restore_must_each_be_delivered_once() {
    let (attempt, mut stop) = bound_runner_lifecycle();
    stop.stop_once.delivery_count = 2;
    assert!(
        verify_runner_lifecycle_v1(&stop, &attempt)
            .unwrap_err()
            .to_string()
            .contains("SIGSTOP")
    );

    let (attempt, mut restore) = bound_runner_lifecycle();
    restore.restore_once.delivery_count = 2;
    assert!(
        verify_runner_lifecycle_v1(&restore, &attempt)
            .unwrap_err()
            .to_string()
            .contains("SIGCONT")
    );
}

#[test]
fn running_android_unit_is_rejected() {
    let (attempt, mut evidence) = bound_android_terminal_unit();
    evidence.observations[1].sub_state = UnitSubStateV1::Running;
    let error = verify_android_terminal_unit_v1(&evidence, &attempt)
        .unwrap_err()
        .to_string();
    assert!(error.contains("not retained active/exited"));
}

#[test]
fn android_restart_and_window_drift_are_rejected() {
    let (attempt, mut restarted) = bound_android_terminal_unit();
    restarted.observations[1].restart_count = 1;
    assert!(verify_android_terminal_unit_v1(&restarted, &attempt).is_err());

    let (attempt, mut drifted) = bound_android_terminal_unit();
    drifted.observations[1].invocation_id = "abcdef0123456789abcdef0123456789".into();
    assert!(
        verify_android_terminal_unit_v1(&drifted, &attempt)
            .unwrap_err()
            .to_string()
            .contains("drifted")
    );
}

#[test]
fn android_fd_or_process_residue_is_rejected() {
    let (attempt, mut fd) = bound_android_terminal_unit();
    fd.root_proc_scan.open_workroot_fds.push(ProcFdMatchV1 {
        pid: 500,
        fd: 9,
        device: 42,
        inode: 43,
    });
    fd.root_proc_scan.proof_sha256 = fd.root_proc_scan.computed_proof_sha256_v1();
    assert!(
        verify_android_terminal_unit_v1(&fd, &attempt)
            .unwrap_err()
            .to_string()
            .contains("open FD")
    );

    let (attempt, mut process) = bound_android_terminal_unit();
    process.root_proc_scan.matching_process_pids.push(500);
    process.root_proc_scan.proof_sha256 = process.root_proc_scan.computed_proof_sha256_v1();
    assert!(verify_android_terminal_unit_v1(&process, &attempt).is_err());
}

#[test]
fn android_terminal_shape_is_exact() {
    let (attempt, mut evidence) = bound_android_terminal_unit();
    evidence.observations[0].control_group = "/user.slice/unit.service".into();
    assert!(verify_android_terminal_unit_v1(&evidence, &attempt).is_err());

    let (attempt, mut evidence) = bound_android_terminal_unit();
    evidence.observations[0].tasks_current = Some(0);
    assert!(verify_android_terminal_unit_v1(&evidence, &attempt).is_err());
}

#[test]
fn serde_rejects_unknown_fields_and_unknown_role_tags() {
    let (_, evidence) = target_evidence();
    let mut value = serde_json::to_value(evidence).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("authority".into(), true.into());
    assert!(serde_json::from_value::<TargetEvidenceV1>(value).is_err());

    let value = serde_json::json!({ "role": "controller", "runner_id": 22 });
    assert!(serde_json::from_value::<RunnerProcessRoleV1>(value).is_err());
}

#[test]
fn cross_boot_splice_is_rejected_across_all_target_evidence() {
    let (attempt, mut runner_splice) = target_evidence();
    runner_splice.runners.stopped.boot.boot_epoch = 2;
    runner_splice.runners.stopped.boot.boot_id = "fedcba98-7654-3210-fedc-ba9876543210".into();
    runner_splice.runners.stopped.snapshot_sha256 =
        runner_splice.runners.stopped.computed_snapshot_sha256_v1();
    runner_splice.runners.stop_once.stopped_snapshot_sha256 =
        runner_splice.runners.stopped.snapshot_sha256.clone();
    runner_splice.runners.restore_once.stopped_snapshot_sha256 =
        runner_splice.runners.stopped.snapshot_sha256.clone();
    assert!(
        verify_target_evidence_v1(&runner_splice, &attempt)
            .unwrap_err()
            .to_string()
            .contains("boot identity")
    );

    let (attempt, mut android_splice) = target_evidence();
    android_splice.android_terminal_unit.observations[1]
        .boot
        .boot_epoch = 2;
    android_splice.android_terminal_unit.observations[1]
        .boot
        .boot_id = "fedcba98-7654-3210-fedc-ba9876543210".into();
    assert!(
        verify_target_evidence_v1(&android_splice, &attempt)
            .unwrap_err()
            .to_string()
            .contains("boot identity")
    );

    let (attempt, mut proof_splice) = target_evidence();
    proof_splice
        .android_terminal_unit
        .root_proc_scan
        .boot
        .boot_epoch = 2;
    proof_splice
        .android_terminal_unit
        .root_proc_scan
        .boot
        .boot_id = "fedcba98-7654-3210-fedc-ba9876543210".into();
    proof_splice
        .android_terminal_unit
        .root_proc_scan
        .proof_sha256 = proof_splice
        .android_terminal_unit
        .root_proc_scan
        .computed_proof_sha256_v1();
    assert!(
        verify_target_evidence_v1(&proof_splice, &attempt)
            .unwrap_err()
            .to_string()
            .contains("boot identity")
    );
}

#[test]
fn arbitrary_android_service_name_is_rejected() {
    let (attempt, mut evidence) = bound_android_terminal_unit();
    evidence.unit_name = "another-valid-looking.service".into();
    for observation in &mut evidence.observations {
        observation.fragment_path =
            "/run/user/1000/systemd/transient/another-valid-looking.service".into();
    }
    assert!(
        verify_android_terminal_unit_v1(&evidence, &attempt)
            .unwrap_err()
            .to_string()
            .contains("unit name")
    );
}

#[test]
fn snapshot_and_root_proof_tampering_without_reseal_is_rejected() {
    let (attempt, mut target) = target_evidence();
    target.runners.stopped.group.processes[0].argv_sha256 = hex64('a');
    assert!(
        verify_target_evidence_v1(&target, &attempt)
            .unwrap_err()
            .to_string()
            .contains("snapshot digest")
    );

    let (attempt, mut target) = target_evidence();
    target.android_terminal_unit.root_proc_scan.workroot_inode += 1;
    assert!(
        verify_target_evidence_v1(&target, &attempt)
            .unwrap_err()
            .to_string()
            .contains("proof digest")
    );
}

#[test]
fn verifier_rejects_a_different_expected_attempt() {
    let (mut attempt, evidence) = target_evidence();
    attempt.parameter_manifest_sha256 = hex64('a');
    assert!(verify_target_evidence_v1(&evidence, &attempt).is_err());
}

#[test]
fn canonical_digests_cover_security_relevant_state() {
    let (_, target) = target_evidence();
    let baseline_snapshot = target.runners.pre_stop;
    let baseline_digest = baseline_snapshot.computed_snapshot_sha256_v1();
    let mut variants = Vec::new();
    let mut changed = baseline_snapshot.clone();
    changed.boot.boot_epoch += 1;
    variants.push(changed);
    let mut changed = baseline_snapshot.clone();
    changed.observation_sequence += 1;
    variants.push(changed);
    let mut changed = baseline_snapshot.clone();
    changed.group.enumerated_group_member_pids.swap(0, 1);
    variants.push(changed);
    let mut changed = baseline_snapshot.clone();
    changed.group.processes[0].pid += 1;
    variants.push(changed);
    let mut changed = baseline_snapshot.clone();
    changed.group.processes[0].cgroup_v2_path.push_str("/drift");
    variants.push(changed);
    let mut changed = baseline_snapshot.clone();
    changed.group.processes[0].runner_config_sha256 = hex64('a');
    variants.push(changed);
    let mut changed = baseline_snapshot;
    changed.group.processes[0].execution_state = ProcessExecutionStateV1::Stopped;
    variants.push(changed);
    assert!(
        variants
            .iter()
            .all(|changed| { changed.computed_snapshot_sha256_v1() != baseline_digest })
    );

    let baseline_proof = target.android_terminal_unit.root_proc_scan;
    let baseline_digest = baseline_proof.computed_proof_sha256_v1();
    let mut variants = Vec::new();
    let mut changed = baseline_proof.clone();
    changed.attempt_identity_sha256 = hex64('a');
    variants.push(changed);
    let mut changed = baseline_proof.clone();
    changed.boot.boot_id = "fedcba98-7654-3210-fedc-ba9876543210".into();
    variants.push(changed);
    let mut changed = baseline_proof.clone();
    changed.observer_elf_sha256 = hex64('a');
    variants.push(changed);
    let mut changed = baseline_proof.clone();
    changed.workroot_device += 1;
    variants.push(changed);
    let mut changed = baseline_proof.clone();
    changed.enumerated_fd_count += 1;
    variants.push(changed);
    let mut changed = baseline_proof.clone();
    changed.matching_process_pids.push(999);
    variants.push(changed);
    let mut changed = baseline_proof;
    changed.open_workroot_fds.push(ProcFdMatchV1 {
        pid: 9,
        fd: 8,
        device: 7,
        inode: 6,
    });
    variants.push(changed);
    assert!(
        variants
            .iter()
            .all(|changed| changed.computed_proof_sha256_v1() != baseline_digest)
    );
}
