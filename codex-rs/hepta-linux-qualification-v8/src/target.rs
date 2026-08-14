use std::collections::BTreeMap;
use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::AttemptIdentityV8;
use crate::QualificationError;
use crate::append_text;
use crate::append_u64;
use crate::invalid;
use crate::sha256_hex;
use crate::validate_lower_hex;
use crate::validate_sha256;

pub const SHARED_PROCESS_GROUP_SCHEMA_V1: &str = "hepta_linux_v8_shared_process_group_v1";
pub const RUNNER_SNAPSHOT_SCHEMA_V1: &str = "hepta_linux_v8_runner_snapshot_v1";
pub const RUNNER_LIFECYCLE_SCHEMA_V1: &str = "hepta_linux_v8_runner_lifecycle_v1";
pub const ANDROID_TERMINAL_UNIT_SCHEMA_V1: &str =
    "hepta_linux_v8_android_terminal_retained_user_unit_v1";
pub const ROOT_PROC_SCAN_SCHEMA_V1: &str = "hepta_linux_v8_root_proc_scan_v1";
pub const TARGET_EVIDENCE_SCHEMA_V1: &str = "hepta_linux_v8_target_evidence_v1";
pub const EXPECTED_ANDROID_TERMINAL_UNIT_V1: &str = "trillionnium-v27-image-package-retry4.service";

const EXPECTED_RUNNER_IDS: [u32; 2] = [22, 23];
const EXPECTED_PROCESS_COUNT: usize = 6;
const MAX_TERMINAL_WINDOW_OBSERVATIONS: usize = 16;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TargetBootIdentityV1 {
    pub boot_epoch: u64,
    pub boot_id: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "role", rename_all = "snake_case", deny_unknown_fields)]
pub enum RunnerProcessRoleV1 {
    RunSh { runner_id: u32 },
    RunHelperSh { runner_id: u32 },
    RunnerListener { runner_id: u32 },
    Worker { runner_id: u32 },
}

impl RunnerProcessRoleV1 {
    fn runner_id(self) -> u32 {
        match self {
            Self::RunSh { runner_id }
            | Self::RunHelperSh { runner_id }
            | Self::RunnerListener { runner_id }
            | Self::Worker { runner_id } => runner_id,
        }
    }

    fn key(self) -> (u32, u8) {
        let role = match self {
            Self::RunSh { .. } => 0,
            Self::RunHelperSh { .. } => 1,
            Self::RunnerListener { .. } => 2,
            Self::Worker { .. } => 3,
        };
        (self.runner_id(), role)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProcessExecutionStateV1 {
    Running,
    Stopped,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RunnerProcessV1 {
    pub role: RunnerProcessRoleV1,
    pub pid: u32,
    pub start_ticks: u64,
    pub pidfd_token_sha256: String,
    pub uid: u32,
    pub gid: u32,
    pub pgid: u32,
    pub sid: u32,
    pub cgroup_v2_path: String,
    pub executable_sha256: String,
    pub cwd_identity_sha256: String,
    pub argv_sha256: String,
    pub environ_sha256: String,
    pub runner_name: String,
    pub runner_labels_sha256: String,
    pub runner_version: String,
    pub runner_config_sha256: String,
    pub workroot_identity_sha256: String,
    pub execution_state: ProcessExecutionStateV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum SessionLeaderV1 {
    OrphanedAbsent { former_pid: u32 },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "completeness", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProcessGroupEnumerationV1 {
    Complete,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SharedProcessGroupV1 {
    pub schema: String,
    pub pgid: u32,
    pub sid: u32,
    pub session_leader: SessionLeaderV1,
    pub enumeration: ProcessGroupEnumerationV1,
    pub enumerated_group_member_pids: Vec<u32>,
    pub enumerated_worker_pids: Vec<u32>,
    pub processes: Vec<RunnerProcessV1>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "phase", rename_all = "snake_case", deny_unknown_fields)]
pub enum RunnerSnapshotPhaseV1 {
    PreStop,
    Stopped,
    PostRestore,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RunnerSnapshotV1 {
    pub schema: String,
    pub phase: RunnerSnapshotPhaseV1,
    pub attempt_identity_sha256: String,
    pub boot: TargetBootIdentityV1,
    pub observation_sequence: u64,
    pub monotonic_ns: u64,
    pub snapshot_sha256: String,
    pub group: SharedProcessGroupV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum StopOperationV1 {
    ProcessGroupSigstop { pgid: u32 },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StopOnceEvidenceV1 {
    pub attempt_identity_sha256: String,
    pub operation: StopOperationV1,
    pub delivery_count: u32,
    pub intent_sequence: u64,
    pub effect_sequence: u64,
    pub pre_stop_snapshot_sha256: String,
    pub stopped_snapshot_sha256: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum RestoreOperationV1 {
    ProcessGroupSigcont { pgid: u32 },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RestoreOnceEvidenceV1 {
    pub attempt_identity_sha256: String,
    pub operation: RestoreOperationV1,
    pub delivery_count: u32,
    pub intent_sequence: u64,
    pub effect_sequence: u64,
    pub stopped_snapshot_sha256: String,
    pub post_restore_snapshot_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RunnerLifecycleEvidenceV1 {
    pub schema: String,
    pub pre_stop: RunnerSnapshotV1,
    pub stopped: RunnerSnapshotV1,
    pub post_restore: RunnerSnapshotV1,
    pub stop_once: StopOnceEvidenceV1,
    pub restore_once: RestoreOnceEvidenceV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "manager", rename_all = "snake_case", deny_unknown_fields)]
pub enum SystemdManagerV1 {
    User { uid: u32 },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "active_state", rename_all = "snake_case", deny_unknown_fields)]
pub enum UnitActiveStateV1 {
    Active,
    Inactive,
    Failed,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "sub_state", rename_all = "snake_case", deny_unknown_fields)]
pub enum UnitSubStateV1 {
    Exited,
    Running,
    Dead,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "result", rename_all = "snake_case", deny_unknown_fields)]
pub enum UnitResultV1 {
    Success,
    Failure,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    tag = "remain_after_exit",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum RemainAfterExitV1 {
    Enabled,
    Disabled,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AndroidUnitObservationV1 {
    pub boot: TargetBootIdentityV1,
    pub observation_sequence: u64,
    pub monotonic_ns: u64,
    pub active_state: UnitActiveStateV1,
    pub sub_state: UnitSubStateV1,
    pub remain_after_exit: RemainAfterExitV1,
    pub result: UnitResultV1,
    pub main_pid: u32,
    pub invocation_id: String,
    pub control_group: String,
    pub tasks_current: Option<u64>,
    pub restart_count: u32,
    pub fragment_path: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "completeness", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProcScanCompletenessV1 {
    Complete,
    Partial,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProcFdMatchV1 {
    pub pid: u32,
    pub fd: u32,
    pub device: u64,
    pub inode: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RootProcScanProofV1 {
    pub schema: String,
    pub attempt_identity_sha256: String,
    pub boot: TargetBootIdentityV1,
    pub observation_sequence: u64,
    pub monotonic_ns: u64,
    pub observer_effective_uid: u32,
    pub observer_effective_gid: u32,
    pub observer_elf_sha256: String,
    pub procfs_mount_identity_sha256: String,
    pub unit_invocation_id: String,
    pub workroot_device: u64,
    pub workroot_inode: u64,
    pub workroot_identity_sha256: String,
    pub enumerated_process_count: u64,
    pub enumerated_fd_count: u64,
    pub unreadable_process_count: u64,
    pub unreadable_fd_table_count: u64,
    pub matching_process_pids: Vec<u32>,
    pub open_workroot_fds: Vec<ProcFdMatchV1>,
    pub completeness: ProcScanCompletenessV1,
    pub proof_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AndroidTerminalUnitV1 {
    pub schema: String,
    pub attempt_identity_sha256: String,
    pub unit_name: String,
    pub manager: SystemdManagerV1,
    pub observations: Vec<AndroidUnitObservationV1>,
    pub root_proc_scan: RootProcScanProofV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TargetEvidenceV1 {
    pub schema: String,
    pub attempt_identity_sha256: String,
    pub runners: RunnerLifecycleEvidenceV1,
    pub android_terminal_unit: AndroidTerminalUnitV1,
}

impl RunnerSnapshotV1 {
    /// Canonical snapshot digest excludes the two outer binding fields:
    /// `attempt_identity_sha256` (the attempt binds this digest) and
    /// `snapshot_sha256` (the digest itself). Every observed target-state field
    /// is included.
    pub fn computed_snapshot_sha256_v1(&self) -> String {
        sha256_hex(&canonical_runner_snapshot_bytes(self))
    }
}

impl RootProcScanProofV1 {
    /// Canonical proof digest includes the attempt and boot bindings and every
    /// collected proof field, excluding only `proof_sha256` itself.
    pub fn computed_proof_sha256_v1(&self) -> String {
        sha256_hex(&canonical_root_proc_scan_bytes(self))
    }
}

pub fn verify_target_evidence_v1(
    evidence: &TargetEvidenceV1,
    expected_attempt: &AttemptIdentityV8,
) -> Result<(), QualificationError> {
    exact(&evidence.schema, TARGET_EVIDENCE_SCHEMA_V1, "target schema")?;
    expected_attempt.validate()?;
    let expected_attempt_sha256 = expected_attempt.sha256()?;
    exact(
        &evidence.attempt_identity_sha256,
        &expected_attempt_sha256,
        "target attempt identity",
    )?;
    verify_runner_lifecycle_v1(&evidence.runners, expected_attempt)?;
    verify_android_terminal_unit_v1(&evidence.android_terminal_unit, expected_attempt)?;
    for (label, observed) in [
        (
            "runner pre-stop attempt",
            evidence.runners.pre_stop.attempt_identity_sha256.as_str(),
        ),
        (
            "Android attempt",
            evidence
                .android_terminal_unit
                .attempt_identity_sha256
                .as_str(),
        ),
    ] {
        exact(observed, &expected_attempt_sha256, label)?;
    }
    exact(
        &evidence.runners.pre_stop.snapshot_sha256,
        &expected_attempt.runner_snapshot_sha256,
        "attempt runner snapshot",
    )?;
    exact_boot(
        &evidence.android_terminal_unit.observations[0].boot,
        &evidence.runners.pre_stop.boot,
        "runner/Android boot identity",
    )?;
    Ok(())
}

pub fn verify_runner_lifecycle_v1(
    evidence: &RunnerLifecycleEvidenceV1,
    expected_attempt: &AttemptIdentityV8,
) -> Result<(), QualificationError> {
    exact(
        &evidence.schema,
        RUNNER_LIFECYCLE_SCHEMA_V1,
        "runner lifecycle schema",
    )?;
    expected_attempt.validate()?;
    let expected_attempt_sha256 = expected_attempt.sha256()?;
    verify_snapshot(
        &evidence.pre_stop,
        RunnerSnapshotPhaseV1::PreStop,
        &expected_attempt_sha256,
    )?;
    verify_snapshot(
        &evidence.stopped,
        RunnerSnapshotPhaseV1::Stopped,
        &expected_attempt_sha256,
    )?;
    verify_snapshot(
        &evidence.post_restore,
        RunnerSnapshotPhaseV1::PostRestore,
        &expected_attempt_sha256,
    )?;
    exact(
        &evidence.pre_stop.snapshot_sha256,
        &expected_attempt.runner_snapshot_sha256,
        "attempt runner snapshot",
    )?;

    for (label, observed) in [
        ("stopped attempt", &evidence.stopped.attempt_identity_sha256),
        (
            "post-restore attempt",
            &evidence.post_restore.attempt_identity_sha256,
        ),
        (
            "stop evidence attempt",
            &evidence.stop_once.attempt_identity_sha256,
        ),
        (
            "restore evidence attempt",
            &evidence.restore_once.attempt_identity_sha256,
        ),
    ] {
        exact(observed, &expected_attempt_sha256, label)?;
    }

    exact_boot(
        &evidence.stopped.boot,
        &evidence.pre_stop.boot,
        "stopped snapshot boot identity",
    )?;
    exact_boot(
        &evidence.post_restore.boot,
        &evidence.pre_stop.boot,
        "post-restore snapshot boot identity",
    )?;

    verify_stopped_transition(&evidence.pre_stop.group, &evidence.stopped.group)?;
    verify_restored_transition(&evidence.pre_stop.group, &evidence.post_restore.group)?;
    verify_stop_once(evidence)?;
    verify_restore_once(evidence)?;
    if !(evidence.pre_stop.monotonic_ns < evidence.stopped.monotonic_ns
        && evidence.stopped.monotonic_ns < evidence.post_restore.monotonic_ns)
    {
        return Err(invalid(
            "runner lifecycle monotonic observations are not ordered",
        ));
    }
    Ok(())
}

pub fn verify_android_terminal_unit_v1(
    evidence: &AndroidTerminalUnitV1,
    expected_attempt: &AttemptIdentityV8,
) -> Result<(), QualificationError> {
    exact(
        &evidence.schema,
        ANDROID_TERMINAL_UNIT_SCHEMA_V1,
        "Android terminal unit schema",
    )?;
    expected_attempt.validate()?;
    exact(
        &evidence.attempt_identity_sha256,
        &expected_attempt.sha256()?,
        "Android attempt identity",
    )?;
    exact(
        &evidence.unit_name,
        EXPECTED_ANDROID_TERMINAL_UNIT_V1,
        "Android transient unit name",
    )?;
    let SystemdManagerV1::User { uid } = evidence.manager;
    if uid != 1000 {
        return Err(invalid(
            "Android terminal unit must be observed in user manager uid 1000",
        ));
    }
    if !(2..=MAX_TERMINAL_WINDOW_OBSERVATIONS).contains(&evidence.observations.len()) {
        return Err(invalid(
            "Android terminal window must contain between 2 and 16 observations",
        ));
    }

    let expected_fragment = format!("/run/user/{uid}/systemd/transient/{}", evidence.unit_name);
    let first = &evidence.observations[0];
    validate_terminal_observation(first, &expected_fragment)?;
    for pair in evidence.observations.windows(2) {
        let previous = &pair[0];
        let current = &pair[1];
        validate_terminal_observation(current, &expected_fragment)?;
        exact_boot(
            &current.boot,
            &previous.boot,
            "Android terminal observation boot identity",
        )?;
        if current.observation_sequence <= previous.observation_sequence
            || current.monotonic_ns <= previous.monotonic_ns
        {
            return Err(invalid(
                "Android terminal window observations are not strictly ordered",
            ));
        }
        let mut normalized = current.clone();
        normalized.observation_sequence = previous.observation_sequence;
        normalized.monotonic_ns = previous.monotonic_ns;
        if &normalized != previous {
            return Err(invalid(
                "Android terminal retained state drifted during observation window",
            ));
        }
    }
    verify_root_proc_scan(
        &evidence.root_proc_scan,
        &evidence.attempt_identity_sha256,
        &first.invocation_id,
        &first.boot,
        &evidence.observations[evidence.observations.len() - 1],
    )
}

fn verify_snapshot(
    snapshot: &RunnerSnapshotV1,
    phase: RunnerSnapshotPhaseV1,
    expected_attempt_sha256: &str,
) -> Result<(), QualificationError> {
    exact(
        &snapshot.schema,
        RUNNER_SNAPSHOT_SCHEMA_V1,
        "runner snapshot schema",
    )?;
    if snapshot.phase != phase {
        return Err(invalid("runner snapshot phase is not canonical"));
    }
    exact(
        &snapshot.attempt_identity_sha256,
        expected_attempt_sha256,
        "runner snapshot attempt identity",
    )?;
    validate_boot_identity(&snapshot.boot)?;
    exact(
        &snapshot.snapshot_sha256,
        &snapshot.computed_snapshot_sha256_v1(),
        "runner snapshot digest",
    )?;
    if snapshot.observation_sequence == 0 || snapshot.monotonic_ns == 0 {
        return Err(invalid(
            "runner snapshot sequence and monotonic time must be non-zero",
        ));
    }
    verify_shared_process_group(&snapshot.group)?;
    let expected_state = match phase {
        RunnerSnapshotPhaseV1::PreStop | RunnerSnapshotPhaseV1::PostRestore => {
            ProcessExecutionStateV1::Running
        }
        RunnerSnapshotPhaseV1::Stopped => ProcessExecutionStateV1::Stopped,
    };
    if snapshot
        .group
        .processes
        .iter()
        .any(|process| process.execution_state != expected_state)
    {
        return Err(invalid(
            "runner snapshot contains a partial or wrong execution state",
        ));
    }
    Ok(())
}

fn verify_shared_process_group(group: &SharedProcessGroupV1) -> Result<(), QualificationError> {
    exact(
        &group.schema,
        SHARED_PROCESS_GROUP_SCHEMA_V1,
        "shared process-group schema",
    )?;
    let SessionLeaderV1::OrphanedAbsent { former_pid } = group.session_leader;
    if group.pgid == 0 || group.pgid != group.sid || group.pgid != former_pid {
        return Err(invalid(
            "shared process group must retain one orphaned PGID/SID identity",
        ));
    }
    if group.enumeration != ProcessGroupEnumerationV1::Complete {
        return Err(invalid("process-group enumeration is not complete"));
    }
    if group.processes.len() != EXPECTED_PROCESS_COUNT {
        return Err(invalid("runner closure must contain exactly six processes"));
    }
    if !group.enumerated_worker_pids.is_empty() {
        return Err(invalid("runner closure contains a Worker process"));
    }

    let mut pids = BTreeSet::new();
    let mut pidfd_tokens = BTreeSet::new();
    let mut roles = BTreeSet::new();
    let mut owners = BTreeSet::new();
    let mut runner_bindings: BTreeMap<u32, (&str, &str, &str, &str, &str)> = BTreeMap::new();
    let mut previous_role_key = None;
    for process in &group.processes {
        validate_runner_process(process, group.pgid, group.sid)?;
        if matches!(process.role, RunnerProcessRoleV1::Worker { .. }) {
            return Err(invalid("runner closure contains a Worker role"));
        }
        if !EXPECTED_RUNNER_IDS.contains(&process.role.runner_id()) {
            return Err(invalid("runner closure contains an unexpected runner id"));
        }
        let role_key = process.role.key();
        if previous_role_key.is_some_and(|previous| previous >= role_key) {
            return Err(invalid(
                "runner closure processes are not in canonical runner/role order",
            ));
        }
        previous_role_key = Some(role_key);
        if !pids.insert(process.pid)
            || !pidfd_tokens.insert(process.pidfd_token_sha256.as_str())
            || !roles.insert(role_key)
        {
            return Err(invalid(
                "runner closure contains a duplicate PID, pidfd token, or role",
            ));
        }
        owners.insert((process.uid, process.gid));
        let binding = (
            process.runner_name.as_str(),
            process.runner_labels_sha256.as_str(),
            process.runner_version.as_str(),
            process.runner_config_sha256.as_str(),
            process.workroot_identity_sha256.as_str(),
        );
        if let Some(expected) = runner_bindings.insert(process.role.runner_id(), binding)
            && expected != binding
        {
            return Err(invalid(
                "runner identity drifts within its three-process closure",
            ));
        }
    }
    if owners.len() != 1 || owners.iter().any(|(uid, gid)| *uid == 0 || *gid == 0) {
        return Err(invalid(
            "six-process runner closure must have one non-root UID/GID identity",
        ));
    }
    for runner_id in EXPECTED_RUNNER_IDS {
        for role in 0..=2 {
            if !roles.contains(&(runner_id, role)) {
                return Err(invalid(
                    "runner closure is missing run.sh, run-helper.sh, or listener",
                ));
            }
        }
    }

    let expected_pids: Vec<u32> = pids.into_iter().collect();
    if expected_pids.contains(&former_pid) {
        return Err(invalid(
            "orphaned session leader is still present in the runner closure",
        ));
    }
    if group.enumerated_group_member_pids != expected_pids {
        return Err(invalid(
            "complete PGID enumeration does not exactly match modeled members",
        ));
    }
    Ok(())
}

fn validate_runner_process(
    process: &RunnerProcessV1,
    pgid: u32,
    sid: u32,
) -> Result<(), QualificationError> {
    if process.pid == 0 || process.start_ticks == 0 || process.pgid != pgid || process.sid != sid {
        return Err(invalid(
            "runner process PID/startticks/PGID/SID identity is invalid",
        ));
    }
    if !process.cgroup_v2_path.starts_with('/')
        || process
            .cgroup_v2_path
            .split('/')
            .any(|component| component == "..")
    {
        return Err(invalid("runner process cgroup v2 path is invalid"));
    }
    if process.runner_name.is_empty() || process.runner_version.is_empty() {
        return Err(invalid("runner name and version must be non-empty"));
    }
    for (label, value) in [
        ("pidfd token", &process.pidfd_token_sha256),
        ("executable", &process.executable_sha256),
        ("cwd identity", &process.cwd_identity_sha256),
        ("argv", &process.argv_sha256),
        ("environment", &process.environ_sha256),
        ("runner labels", &process.runner_labels_sha256),
        ("runner config", &process.runner_config_sha256),
        ("workroot identity", &process.workroot_identity_sha256),
    ] {
        validate_sha256(label, value)?;
    }
    Ok(())
}

fn verify_stopped_transition(
    pre_stop: &SharedProcessGroupV1,
    stopped: &SharedProcessGroupV1,
) -> Result<(), QualificationError> {
    let mut expected = pre_stop.clone();
    for process in &mut expected.processes {
        process.execution_state = ProcessExecutionStateV1::Stopped;
    }
    if &expected != stopped {
        return Err(invalid(
            "stopped snapshot is not the exact six-process pre-stop closure",
        ));
    }
    Ok(())
}

fn verify_restored_transition(
    pre_stop: &SharedProcessGroupV1,
    post_restore: &SharedProcessGroupV1,
) -> Result<(), QualificationError> {
    if pre_stop.pgid != post_restore.pgid
        || pre_stop.sid != post_restore.sid
        || pre_stop.session_leader != post_restore.session_leader
        || pre_stop.enumeration != post_restore.enumeration
    {
        return Err(invalid(
            "shared runner PGID/SID identity drifted during restore",
        ));
    }
    let pre_by_role: BTreeMap<_, _> = pre_stop
        .processes
        .iter()
        .map(|process| (process.role.key(), process))
        .collect();
    let post_by_role: BTreeMap<_, _> = post_restore
        .processes
        .iter()
        .map(|process| (process.role.key(), process))
        .collect();
    if pre_by_role.keys().ne(post_by_role.keys()) {
        return Err(invalid("runner role closure drifted during restore"));
    }
    for (role, pre) in pre_by_role {
        let post = post_by_role[&role];
        let mut normalized = post.clone();
        normalized.execution_state = pre.execution_state;
        if matches!(pre.role, RunnerProcessRoleV1::RunnerListener { .. }) {
            normalized.pid = pre.pid;
            normalized.start_ticks = pre.start_ticks;
            normalized
                .pidfd_token_sha256
                .clone_from(&pre.pidfd_token_sha256);
        }
        if &normalized != pre {
            return Err(invalid("runner identity drifted during restore"));
        }
    }
    Ok(())
}

fn verify_stop_once(evidence: &RunnerLifecycleEvidenceV1) -> Result<(), QualificationError> {
    let StopOperationV1::ProcessGroupSigstop { pgid } = evidence.stop_once.operation;
    if pgid != evidence.pre_stop.group.pgid || evidence.stop_once.delivery_count != 1 {
        return Err(invalid(
            "SIGSTOP was not delivered exactly once to the exact shared PGID",
        ));
    }
    if evidence.stop_once.pre_stop_snapshot_sha256 != evidence.pre_stop.snapshot_sha256
        || evidence.stop_once.stopped_snapshot_sha256 != evidence.stopped.snapshot_sha256
    {
        return Err(invalid(
            "stop-once evidence does not bind the exact snapshots",
        ));
    }
    if !(evidence.pre_stop.observation_sequence < evidence.stop_once.intent_sequence
        && evidence.stop_once.intent_sequence < evidence.stop_once.effect_sequence
        && evidence.stop_once.effect_sequence < evidence.stopped.observation_sequence)
    {
        return Err(invalid("stop-once evidence sequence is invalid"));
    }
    Ok(())
}

fn verify_restore_once(evidence: &RunnerLifecycleEvidenceV1) -> Result<(), QualificationError> {
    let RestoreOperationV1::ProcessGroupSigcont { pgid } = evidence.restore_once.operation;
    if pgid != evidence.stopped.group.pgid || evidence.restore_once.delivery_count != 1 {
        return Err(invalid(
            "SIGCONT was not delivered exactly once to the exact shared PGID",
        ));
    }
    if evidence.restore_once.stopped_snapshot_sha256 != evidence.stopped.snapshot_sha256
        || evidence.restore_once.post_restore_snapshot_sha256
            != evidence.post_restore.snapshot_sha256
    {
        return Err(invalid(
            "restore-once evidence does not bind the exact snapshots",
        ));
    }
    if !(evidence.stopped.observation_sequence < evidence.restore_once.intent_sequence
        && evidence.restore_once.intent_sequence < evidence.restore_once.effect_sequence
        && evidence.restore_once.effect_sequence < evidence.post_restore.observation_sequence)
    {
        return Err(invalid("restore-once evidence sequence is invalid"));
    }
    Ok(())
}

fn validate_terminal_observation(
    observation: &AndroidUnitObservationV1,
    expected_fragment: &str,
) -> Result<(), QualificationError> {
    validate_boot_identity(&observation.boot)?;
    if observation.observation_sequence == 0 || observation.monotonic_ns == 0 {
        return Err(invalid(
            "Android terminal observation ordering fields are zero",
        ));
    }
    if observation.active_state != UnitActiveStateV1::Active
        || observation.sub_state != UnitSubStateV1::Exited
        || observation.remain_after_exit != RemainAfterExitV1::Enabled
        || observation.result != UnitResultV1::Success
        || observation.main_pid != 0
        || !observation.control_group.is_empty()
        || observation.tasks_current.is_some()
        || observation.restart_count != 0
    {
        return Err(invalid(
            "Android unit is not retained active/exited terminal state",
        ));
    }
    validate_lower_hex("Android InvocationID", &observation.invocation_id, 32)?;
    exact(
        &observation.fragment_path,
        expected_fragment,
        "Android transient fragment path",
    )
}

fn verify_root_proc_scan(
    proof: &RootProcScanProofV1,
    attempt: &str,
    invocation_id: &str,
    expected_boot: &TargetBootIdentityV1,
    last_observation: &AndroidUnitObservationV1,
) -> Result<(), QualificationError> {
    exact(
        &proof.schema,
        ROOT_PROC_SCAN_SCHEMA_V1,
        "root /proc scan schema",
    )?;
    exact(
        &proof.attempt_identity_sha256,
        attempt,
        "root /proc scan attempt",
    )?;
    exact(
        &proof.unit_invocation_id,
        invocation_id,
        "root /proc scan InvocationID",
    )?;
    validate_boot_identity(&proof.boot)?;
    exact_boot(&proof.boot, expected_boot, "root /proc scan boot identity")?;
    if proof.observer_effective_uid != 0 || proof.observer_effective_gid != 0 {
        return Err(invalid(
            "/proc and workroot scan was not performed by root observer",
        ));
    }
    if proof.completeness != ProcScanCompletenessV1::Complete
        || proof.unreadable_process_count != 0
        || proof.unreadable_fd_table_count != 0
        || proof.enumerated_process_count == 0
        || proof.enumerated_fd_count == 0
    {
        return Err(invalid("root /proc FD scan is incomplete"));
    }
    if !proof.matching_process_pids.is_empty() || !proof.open_workroot_fds.is_empty() {
        return Err(invalid("Android workroot still has a process or open FD"));
    }
    if proof.workroot_device == 0 || proof.workroot_inode == 0 {
        return Err(invalid(
            "root scan lacks exact workroot device/inode identity",
        ));
    }
    if proof.observation_sequence <= last_observation.observation_sequence
        || proof.monotonic_ns <= last_observation.monotonic_ns
    {
        return Err(invalid(
            "root /proc scan does not follow the retained terminal window",
        ));
    }
    for (label, value) in [
        ("root observer ELF", &proof.observer_elf_sha256),
        ("procfs mount identity", &proof.procfs_mount_identity_sha256),
        ("workroot identity", &proof.workroot_identity_sha256),
    ] {
        validate_sha256(label, value)?;
    }
    exact(
        &proof.proof_sha256,
        &proof.computed_proof_sha256_v1(),
        "root scan proof digest",
    )?;
    Ok(())
}

fn validate_boot_identity(boot: &TargetBootIdentityV1) -> Result<(), QualificationError> {
    if boot.boot_epoch == 0 {
        return Err(invalid("target boot epoch must be non-zero"));
    }
    let bytes = boot.boot_id.as_bytes();
    if bytes.len() != 36
        || bytes[8] != b'-'
        || bytes[13] != b'-'
        || bytes[18] != b'-'
        || bytes[23] != b'-'
    {
        return Err(invalid(
            "target boot id must be a lowercase hyphenated UUID",
        ));
    }
    let compact: String = boot
        .boot_id
        .chars()
        .filter(|character| *character != '-')
        .collect();
    validate_lower_hex("target boot id", &compact, 32)
}

fn exact_boot(
    observed: &TargetBootIdentityV1,
    expected: &TargetBootIdentityV1,
    label: &str,
) -> Result<(), QualificationError> {
    validate_boot_identity(observed)?;
    validate_boot_identity(expected)?;
    if observed != expected {
        return Err(invalid(format!(
            "{label} does not match the exact boot epoch/id"
        )));
    }
    Ok(())
}

fn canonical_runner_snapshot_bytes(snapshot: &RunnerSnapshotV1) -> Vec<u8> {
    let mut bytes = Vec::new();
    append_text(&mut bytes, "schema", &snapshot.schema);
    append_text(
        &mut bytes,
        "phase",
        match snapshot.phase {
            RunnerSnapshotPhaseV1::PreStop => "pre_stop",
            RunnerSnapshotPhaseV1::Stopped => "stopped",
            RunnerSnapshotPhaseV1::PostRestore => "post_restore",
        },
    );
    append_boot_identity(&mut bytes, &snapshot.boot);
    append_u64(
        &mut bytes,
        "observation_sequence",
        snapshot.observation_sequence,
    );
    append_u64(&mut bytes, "monotonic_ns", snapshot.monotonic_ns);
    append_shared_process_group(&mut bytes, &snapshot.group);
    bytes
}

fn append_boot_identity(bytes: &mut Vec<u8>, boot: &TargetBootIdentityV1) {
    append_u64(bytes, "boot_epoch", boot.boot_epoch);
    append_text(bytes, "boot_id", &boot.boot_id);
}

fn append_shared_process_group(bytes: &mut Vec<u8>, group: &SharedProcessGroupV1) {
    append_text(bytes, "group_schema", &group.schema);
    append_u64(bytes, "pgid", u64::from(group.pgid));
    append_u64(bytes, "sid", u64::from(group.sid));
    let SessionLeaderV1::OrphanedAbsent { former_pid } = group.session_leader;
    append_text(bytes, "session_leader_status", "orphaned_absent");
    append_u64(bytes, "session_leader_former_pid", u64::from(former_pid));
    append_text(bytes, "process_group_enumeration", "complete");
    append_u64(
        bytes,
        "enumerated_group_member_pids_count",
        group.enumerated_group_member_pids.len() as u64,
    );
    for pid in &group.enumerated_group_member_pids {
        append_u64(bytes, "enumerated_group_member_pid", u64::from(*pid));
    }
    append_u64(
        bytes,
        "enumerated_worker_pids_count",
        group.enumerated_worker_pids.len() as u64,
    );
    for pid in &group.enumerated_worker_pids {
        append_u64(bytes, "enumerated_worker_pid", u64::from(*pid));
    }
    append_u64(bytes, "process_count", group.processes.len() as u64);
    for process in &group.processes {
        append_runner_process(bytes, process);
    }
}

fn append_runner_process(bytes: &mut Vec<u8>, process: &RunnerProcessV1) {
    let (role, runner_id) = match process.role {
        RunnerProcessRoleV1::RunSh { runner_id } => ("run_sh", runner_id),
        RunnerProcessRoleV1::RunHelperSh { runner_id } => ("run_helper_sh", runner_id),
        RunnerProcessRoleV1::RunnerListener { runner_id } => ("runner_listener", runner_id),
        RunnerProcessRoleV1::Worker { runner_id } => ("worker", runner_id),
    };
    append_text(bytes, "process_role", role);
    append_u64(bytes, "process_runner_id", u64::from(runner_id));
    append_u64(bytes, "process_pid", u64::from(process.pid));
    append_u64(bytes, "process_start_ticks", process.start_ticks);
    append_text(
        bytes,
        "process_pidfd_token_sha256",
        &process.pidfd_token_sha256,
    );
    append_u64(bytes, "process_uid", u64::from(process.uid));
    append_u64(bytes, "process_gid", u64::from(process.gid));
    append_u64(bytes, "process_pgid", u64::from(process.pgid));
    append_u64(bytes, "process_sid", u64::from(process.sid));
    append_text(bytes, "process_cgroup_v2_path", &process.cgroup_v2_path);
    append_text(
        bytes,
        "process_executable_sha256",
        &process.executable_sha256,
    );
    append_text(
        bytes,
        "process_cwd_identity_sha256",
        &process.cwd_identity_sha256,
    );
    append_text(bytes, "process_argv_sha256", &process.argv_sha256);
    append_text(bytes, "process_environ_sha256", &process.environ_sha256);
    append_text(bytes, "process_runner_name", &process.runner_name);
    append_text(
        bytes,
        "process_runner_labels_sha256",
        &process.runner_labels_sha256,
    );
    append_text(bytes, "process_runner_version", &process.runner_version);
    append_text(
        bytes,
        "process_runner_config_sha256",
        &process.runner_config_sha256,
    );
    append_text(
        bytes,
        "process_workroot_identity_sha256",
        &process.workroot_identity_sha256,
    );
    append_text(
        bytes,
        "process_execution_state",
        match process.execution_state {
            ProcessExecutionStateV1::Running => "running",
            ProcessExecutionStateV1::Stopped => "stopped",
        },
    );
}

fn canonical_root_proc_scan_bytes(proof: &RootProcScanProofV1) -> Vec<u8> {
    let mut bytes = Vec::new();
    append_text(&mut bytes, "schema", &proof.schema);
    append_text(
        &mut bytes,
        "attempt_identity_sha256",
        &proof.attempt_identity_sha256,
    );
    append_boot_identity(&mut bytes, &proof.boot);
    append_u64(
        &mut bytes,
        "observation_sequence",
        proof.observation_sequence,
    );
    append_u64(&mut bytes, "monotonic_ns", proof.monotonic_ns);
    append_u64(
        &mut bytes,
        "observer_effective_uid",
        u64::from(proof.observer_effective_uid),
    );
    append_u64(
        &mut bytes,
        "observer_effective_gid",
        u64::from(proof.observer_effective_gid),
    );
    append_text(
        &mut bytes,
        "observer_elf_sha256",
        &proof.observer_elf_sha256,
    );
    append_text(
        &mut bytes,
        "procfs_mount_identity_sha256",
        &proof.procfs_mount_identity_sha256,
    );
    append_text(&mut bytes, "unit_invocation_id", &proof.unit_invocation_id);
    append_u64(&mut bytes, "workroot_device", proof.workroot_device);
    append_u64(&mut bytes, "workroot_inode", proof.workroot_inode);
    append_text(
        &mut bytes,
        "workroot_identity_sha256",
        &proof.workroot_identity_sha256,
    );
    append_u64(
        &mut bytes,
        "enumerated_process_count",
        proof.enumerated_process_count,
    );
    append_u64(&mut bytes, "enumerated_fd_count", proof.enumerated_fd_count);
    append_u64(
        &mut bytes,
        "unreadable_process_count",
        proof.unreadable_process_count,
    );
    append_u64(
        &mut bytes,
        "unreadable_fd_table_count",
        proof.unreadable_fd_table_count,
    );
    append_u64(
        &mut bytes,
        "matching_process_pids_count",
        proof.matching_process_pids.len() as u64,
    );
    for pid in &proof.matching_process_pids {
        append_u64(&mut bytes, "matching_process_pid", u64::from(*pid));
    }
    append_u64(
        &mut bytes,
        "open_workroot_fds_count",
        proof.open_workroot_fds.len() as u64,
    );
    for fd_match in &proof.open_workroot_fds {
        append_u64(&mut bytes, "open_fd_pid", u64::from(fd_match.pid));
        append_u64(&mut bytes, "open_fd_number", u64::from(fd_match.fd));
        append_u64(&mut bytes, "open_fd_device", fd_match.device);
        append_u64(&mut bytes, "open_fd_inode", fd_match.inode);
    }
    append_text(
        &mut bytes,
        "completeness",
        match proof.completeness {
            ProcScanCompletenessV1::Complete => "complete",
            ProcScanCompletenessV1::Partial => "partial",
        },
    );
    bytes
}

fn exact(observed: &str, expected: &str, label: &str) -> Result<(), QualificationError> {
    if observed != expected {
        return Err(invalid(format!(
            "{label} does not match the exact contract"
        )));
    }
    Ok(())
}

#[cfg(test)]
#[path = "target_tests.rs"]
mod tests;
