use std::path::Path;

use codex_hepta_linux_qualification_v8::JournalEffectV8;
use codex_hepta_linux_qualification_v8::JournalEventV8;
use codex_hepta_linux_qualification_v8::QualificationJournalPhaseV8;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest as _;

use crate::DirectoryAnchorV8;
use crate::DurablyObservedStoppedRunnerScopeV8;
use crate::FileIdentityV8;
use crate::NativeErrorV8;
#[cfg(test)]
use crate::NativeSysErrorV8;
use crate::ObservedRunnerScopeV8;
use crate::RestoreAuthorizedStoppedRunnerScopeV8;
use crate::RunnerScopeContinueExecutionV8;
use crate::StateRootLockV8;
use crate::StoppedRunnerScopeV8;
use crate::TrustedNodeMetadataV8;
use crate::invalid;

use super::CandidateExecutionEffectEvidenceV8;
use super::DescriptorReplayOriginV8;
use super::DurableJournalRecordV8;
use super::FreshActiveAttemptPublicationV8;
use super::FrozenTransitionEvidencePhaseV8;
use super::JOURNAL_DIRECTORY_V8;
use super::PublishedDurableJournalRecordV8;
use super::VerifiedDescriptorBoundDurableJournalRecordsV8;
use super::VerifiedDurableJournalScanV8;
#[cfg(all(test, target_os = "linux"))]
use super::append_journal_record_durably_v8;
use super::attempt_relative_path_v8;
use super::scan_journal_directory_descriptor_bound_v8;
use super::scan_journal_directory_with_records_v8;

const DURABLE_JOURNAL_EVENT_SCHEMA_V8: &[u8] = b"hepta-linux-v8-durable-journal-event-v2\0";
const MAX_DURABLE_EFFECT_EVIDENCE_BYTES_V8: usize = 4 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RunnerStopEvidencePhaseV8 {
    PreEffect,
    StoppedObservation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunnerStopProcessEvidenceV8 {
    pub(crate) pid: u32,
    pub(crate) pidfd_bound: bool,
    pub(crate) start_ticks: u64,
    pub(crate) parent_pid: u32,
    pub(crate) process_group_id: u32,
    pub(crate) session_id: u32,
    pub(crate) state: u8,
    pub(crate) executable_device: u64,
    pub(crate) executable_inode: u64,
    pub(crate) executable_size: u64,
    pub(crate) executable_sha256: String,
    pub(crate) uid: u32,
    pub(crate) gid: u32,
    pub(crate) argv_sha256: String,
    pub(crate) cwd_device: u64,
    pub(crate) cwd_inode: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunnerStopEffectEvidenceV8 {
    pub(crate) schema: String,
    pub(crate) phase: RunnerStopEvidencePhaseV8,
    pub(crate) boot_id: String,
    pub(crate) unit_name: String,
    pub(crate) control_group: String,
    pub(crate) main_pid: u32,
    pub(crate) cgroup_device: u64,
    pub(crate) cgroup_inode: u64,
    pub(crate) cgroup_mount_id: u64,
    pub(crate) proc_mount_id: u64,
    pub(crate) pid_namespace_device: u64,
    pub(crate) pid_namespace_inode: u64,
    pub(crate) cgroup_namespace_device: u64,
    pub(crate) cgroup_namespace_inode: u64,
    pub(crate) mount_namespace_device: u64,
    pub(crate) mount_namespace_inode: u64,
    pub(crate) process_group_id: u32,
    pub(crate) session_id: u32,
    pub(crate) observation_started_boottime_ns: u64,
    pub(crate) observation_completed_boottime_ns: u64,
    pub(crate) observation_started_monotonic_ns: u64,
    pub(crate) observation_completed_monotonic_ns: u64,
    pub(crate) intent_manifest_sha256: Option<String>,
    pub(crate) intent_record_sha256: Option<String>,
    pub(crate) processes: Vec<RunnerStopProcessEvidenceV8>,
}

#[derive(Serialize)]
struct RunnerStopProcessScopeBindingV8<'a> {
    pid: u32,
    pidfd_bound: bool,
    start_ticks: u64,
    parent_pid: u32,
    process_group_id: u32,
    session_id: u32,
    executable_device: u64,
    executable_inode: u64,
    executable_size: u64,
    executable_sha256: &'a str,
    uid: u32,
    gid: u32,
    argv_sha256: &'a str,
    cwd_device: u64,
    cwd_inode: u64,
}

#[derive(Serialize)]
struct RunnerStopScopeBindingV8<'a> {
    schema: &'static str,
    boot_id: &'a str,
    unit_name: &'a str,
    control_group: &'a str,
    main_pid: u32,
    cgroup_device: u64,
    cgroup_inode: u64,
    cgroup_mount_id: u64,
    proc_mount_id: u64,
    pid_namespace_device: u64,
    pid_namespace_inode: u64,
    cgroup_namespace_device: u64,
    cgroup_namespace_inode: u64,
    mount_namespace_device: u64,
    mount_namespace_inode: u64,
    process_group_id: u32,
    session_id: u32,
    processes: Vec<RunnerStopProcessScopeBindingV8<'a>>,
}

impl RunnerStopEffectEvidenceV8 {
    pub(crate) fn canonical_bytes(&self) -> Result<Vec<u8>, NativeErrorV8> {
        self.validate()?;
        serde_json::to_vec(self)
            .map_err(|error| invalid(format!("encode runner STOP evidence: {error}")))
    }

    pub(crate) fn decode_exact(bytes: &[u8]) -> Result<Self, NativeErrorV8> {
        if bytes.is_empty() || bytes.len() > MAX_DURABLE_EFFECT_EVIDENCE_BYTES_V8 {
            return Err(invalid("runner STOP evidence size is invalid"));
        }
        let evidence: Self = serde_json::from_slice(bytes)
            .map_err(|error| invalid(format!("decode runner STOP evidence: {error}")))?;
        evidence.validate()?;
        if evidence.canonical_bytes()? != bytes {
            return Err(invalid("runner STOP evidence bytes are not canonical"));
        }
        Ok(evidence)
    }

    pub(crate) fn sha256(&self) -> Result<String, NativeErrorV8> {
        Ok(format!(
            "{:x}",
            sha2::Sha256::digest(self.canonical_bytes()?)
        ))
    }

    pub(crate) fn scope_binding_bytes(&self) -> Result<Vec<u8>, NativeErrorV8> {
        self.validate()?;
        let binding = RunnerStopScopeBindingV8 {
            schema: "hepta-linux-v8-runner-stop-scope-binding-v1",
            boot_id: &self.boot_id,
            unit_name: &self.unit_name,
            control_group: &self.control_group,
            main_pid: self.main_pid,
            cgroup_device: self.cgroup_device,
            cgroup_inode: self.cgroup_inode,
            cgroup_mount_id: self.cgroup_mount_id,
            proc_mount_id: self.proc_mount_id,
            pid_namespace_device: self.pid_namespace_device,
            pid_namespace_inode: self.pid_namespace_inode,
            cgroup_namespace_device: self.cgroup_namespace_device,
            cgroup_namespace_inode: self.cgroup_namespace_inode,
            mount_namespace_device: self.mount_namespace_device,
            mount_namespace_inode: self.mount_namespace_inode,
            process_group_id: self.process_group_id,
            session_id: self.session_id,
            processes: self
                .processes
                .iter()
                .map(|process| RunnerStopProcessScopeBindingV8 {
                    pid: process.pid,
                    pidfd_bound: process.pidfd_bound,
                    start_ticks: process.start_ticks,
                    parent_pid: process.parent_pid,
                    process_group_id: process.process_group_id,
                    session_id: process.session_id,
                    executable_device: process.executable_device,
                    executable_inode: process.executable_inode,
                    executable_size: process.executable_size,
                    executable_sha256: &process.executable_sha256,
                    uid: process.uid,
                    gid: process.gid,
                    argv_sha256: &process.argv_sha256,
                    cwd_device: process.cwd_device,
                    cwd_inode: process.cwd_inode,
                })
                .collect(),
        };
        serde_json::to_vec(&binding)
            .map_err(|error| invalid(format!("encode runner STOP scope binding: {error}")))
    }

    fn closes_exact_manifest(&self, intent: &Self) -> bool {
        self.phase == RunnerStopEvidencePhaseV8::StoppedObservation
            && intent.phase == RunnerStopEvidencePhaseV8::PreEffect
            && self.boot_id == intent.boot_id
            && self.unit_name == intent.unit_name
            && self.control_group == intent.control_group
            && self.main_pid == intent.main_pid
            && self.cgroup_device == intent.cgroup_device
            && self.cgroup_inode == intent.cgroup_inode
            && self.cgroup_mount_id == intent.cgroup_mount_id
            && self.proc_mount_id == intent.proc_mount_id
            && self.pid_namespace_device == intent.pid_namespace_device
            && self.pid_namespace_inode == intent.pid_namespace_inode
            && self.cgroup_namespace_device == intent.cgroup_namespace_device
            && self.cgroup_namespace_inode == intent.cgroup_namespace_inode
            && self.mount_namespace_device == intent.mount_namespace_device
            && self.mount_namespace_inode == intent.mount_namespace_inode
            && self.process_group_id == intent.process_group_id
            && self.session_id == intent.session_id
            && self.observation_started_boottime_ns >= intent.observation_completed_boottime_ns
            && self.observation_started_monotonic_ns >= intent.observation_completed_monotonic_ns
            && self.processes.len() == intent.processes.len()
            && self
                .processes
                .iter()
                .zip(&intent.processes)
                .all(|(after, before)| {
                    after.pid == before.pid
                        && after.pidfd_bound == before.pidfd_bound
                        && after.start_ticks == before.start_ticks
                        && after.parent_pid == before.parent_pid
                        && after.process_group_id == before.process_group_id
                        && after.session_id == before.session_id
                        && after.executable_device == before.executable_device
                        && after.executable_inode == before.executable_inode
                        && after.executable_size == before.executable_size
                        && after.executable_sha256 == before.executable_sha256
                        && after.uid == before.uid
                        && after.gid == before.gid
                        && after.argv_sha256 == before.argv_sha256
                        && after.cwd_device == before.cwd_device
                        && after.cwd_inode == before.cwd_inode
                })
    }

    fn validate(&self) -> Result<(), NativeErrorV8> {
        if self.schema != "hepta-linux-v8-runner-stop-evidence-v1"
            || self.boot_id.len() != 36
            || self.unit_name.is_empty()
            || self.unit_name.len() > 256
            || !self.control_group.starts_with('/')
            || self.main_pid == 0
            || self.cgroup_device == 0
            || self.cgroup_inode == 0
            || self.cgroup_mount_id == 0
            || self.proc_mount_id == 0
            || self.pid_namespace_device == 0
            || self.pid_namespace_inode == 0
            || self.cgroup_namespace_device == 0
            || self.cgroup_namespace_inode == 0
            || self.mount_namespace_device == 0
            || self.mount_namespace_inode == 0
            || self.process_group_id <= 1
            || self.session_id <= 1
            || self.observation_started_boottime_ns == 0
            || self.observation_completed_boottime_ns < self.observation_started_boottime_ns
            || self.observation_started_monotonic_ns == 0
            || self.observation_completed_monotonic_ns < self.observation_started_monotonic_ns
            || self.processes.len() != 6
        {
            return Err(invalid("runner STOP evidence header is malformed"));
        }
        super::validate_boot_id_v8(&self.boot_id)?;
        match self.phase {
            RunnerStopEvidencePhaseV8::PreEffect => {
                if self.intent_manifest_sha256.is_some() || self.intent_record_sha256.is_some() {
                    return Err(invalid(
                        "pre-effect runner STOP evidence cannot name an issued intent",
                    ));
                }
            }
            RunnerStopEvidencePhaseV8::StoppedObservation => {
                super::validate_digest(
                    "runner STOP observation manifest",
                    self.intent_manifest_sha256
                        .as_deref()
                        .ok_or_else(|| invalid("runner STOP observation lacks manifest digest"))?,
                )?;
                super::validate_digest(
                    "runner STOP observation intent",
                    self.intent_record_sha256
                        .as_deref()
                        .ok_or_else(|| invalid("runner STOP observation lacks intent digest"))?,
                )?;
            }
        }
        let mut previous_pid = 0;
        for process in &self.processes {
            super::validate_digest("runner executable", &process.executable_sha256)?;
            super::validate_digest("runner argv", &process.argv_sha256)?;
            let stopped = matches!(process.state, b'T' | b't');
            if process.pid <= previous_pid
                || !process.pidfd_bound
                || process.start_ticks == 0
                || process.parent_pid == 0
                || process.process_group_id != self.process_group_id
                || process.session_id != self.session_id
                || process.executable_device == 0
                || process.executable_inode == 0
                || process.executable_size == 0
                || process.cwd_device == 0
                || process.cwd_inode == 0
                || (self.phase == RunnerStopEvidencePhaseV8::PreEffect
                    && (stopped || matches!(process.state, b'Z' | b'X' | b'I')))
                || (self.phase == RunnerStopEvidencePhaseV8::StoppedObservation && !stopped)
            {
                return Err(invalid("runner STOP process evidence is malformed"));
            }
            previous_pid = process.pid;
        }
        if !self
            .processes
            .iter()
            .any(|process| process.pid == self.main_pid)
        {
            return Err(invalid("runner STOP evidence omits the exact main PID"));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RunnerRestoreEvidencePhaseV8 {
    StoppedPreEffect,
    RunningObservation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunnerRestoreProcessEvidenceV8 {
    pub(crate) pid: u32,
    pub(crate) pidfd_bound: bool,
    pub(crate) start_ticks: u64,
    pub(crate) parent_pid: u32,
    pub(crate) process_group_id: u32,
    pub(crate) session_id: u32,
    pub(crate) state: u8,
    pub(crate) executable_device: u64,
    pub(crate) executable_inode: u64,
    pub(crate) executable_size: u64,
    pub(crate) executable_sha256: String,
    pub(crate) uid: u32,
    pub(crate) gid: u32,
    pub(crate) argv_sha256: String,
    pub(crate) cwd_device: u64,
    pub(crate) cwd_inode: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RunnerRestoreEffectEvidenceV8 {
    pub(crate) schema: String,
    pub(crate) phase: RunnerRestoreEvidencePhaseV8,
    pub(crate) boot_id: String,
    pub(crate) unit_name: String,
    pub(crate) control_group: String,
    pub(crate) main_pid: u32,
    pub(crate) cgroup_device: u64,
    pub(crate) cgroup_inode: u64,
    pub(crate) cgroup_mount_id: u64,
    pub(crate) proc_mount_id: u64,
    pub(crate) pid_namespace_device: u64,
    pub(crate) pid_namespace_inode: u64,
    pub(crate) cgroup_namespace_device: u64,
    pub(crate) cgroup_namespace_inode: u64,
    pub(crate) mount_namespace_device: u64,
    pub(crate) mount_namespace_inode: u64,
    pub(crate) process_group_id: u32,
    pub(crate) session_id: u32,
    pub(crate) stopped_observation_record_sha256: String,
    pub(crate) observation_started_boottime_ns: u64,
    pub(crate) observation_completed_boottime_ns: u64,
    pub(crate) observation_started_monotonic_ns: u64,
    pub(crate) observation_completed_monotonic_ns: u64,
    pub(crate) intent_manifest_sha256: Option<String>,
    pub(crate) intent_record_sha256: Option<String>,
    pub(crate) processes: Vec<RunnerRestoreProcessEvidenceV8>,
}

#[derive(Serialize)]
struct RunnerRestoreProcessScopeBindingV8<'a> {
    pid: u32,
    pidfd_bound: bool,
    start_ticks: u64,
    parent_pid: u32,
    process_group_id: u32,
    session_id: u32,
    executable_device: u64,
    executable_inode: u64,
    executable_size: u64,
    executable_sha256: &'a str,
    uid: u32,
    gid: u32,
    argv_sha256: &'a str,
    cwd_device: u64,
    cwd_inode: u64,
}

#[derive(Serialize)]
struct RunnerRestoreScopeBindingV8<'a> {
    schema: &'static str,
    boot_id: &'a str,
    unit_name: &'a str,
    control_group: &'a str,
    main_pid: u32,
    cgroup_device: u64,
    cgroup_inode: u64,
    cgroup_mount_id: u64,
    proc_mount_id: u64,
    pid_namespace_device: u64,
    pid_namespace_inode: u64,
    cgroup_namespace_device: u64,
    cgroup_namespace_inode: u64,
    mount_namespace_device: u64,
    mount_namespace_inode: u64,
    process_group_id: u32,
    session_id: u32,
    stopped_observation_record_sha256: &'a str,
    processes: Vec<RunnerRestoreProcessScopeBindingV8<'a>>,
}

impl RunnerRestoreEffectEvidenceV8 {
    pub(crate) fn canonical_bytes(&self) -> Result<Vec<u8>, NativeErrorV8> {
        self.validate()?;
        serde_json::to_vec(self)
            .map_err(|error| invalid(format!("encode runner RESTORE evidence: {error}")))
    }

    pub(crate) fn decode_exact(bytes: &[u8]) -> Result<Self, NativeErrorV8> {
        if bytes.is_empty() || bytes.len() > MAX_DURABLE_EFFECT_EVIDENCE_BYTES_V8 {
            return Err(invalid("runner RESTORE evidence size is invalid"));
        }
        let evidence: Self = serde_json::from_slice(bytes)
            .map_err(|error| invalid(format!("decode runner RESTORE evidence: {error}")))?;
        evidence.validate()?;
        if evidence.canonical_bytes()? != bytes {
            return Err(invalid("runner RESTORE evidence bytes are not canonical"));
        }
        Ok(evidence)
    }

    pub(crate) fn sha256(&self) -> Result<String, NativeErrorV8> {
        Ok(format!(
            "{:x}",
            sha2::Sha256::digest(self.canonical_bytes()?)
        ))
    }

    pub(crate) fn scope_binding_bytes(&self) -> Result<Vec<u8>, NativeErrorV8> {
        self.validate()?;
        let binding = RunnerRestoreScopeBindingV8 {
            schema: "hepta-linux-v8-runner-restore-scope-binding-v1",
            boot_id: &self.boot_id,
            unit_name: &self.unit_name,
            control_group: &self.control_group,
            main_pid: self.main_pid,
            cgroup_device: self.cgroup_device,
            cgroup_inode: self.cgroup_inode,
            cgroup_mount_id: self.cgroup_mount_id,
            proc_mount_id: self.proc_mount_id,
            pid_namespace_device: self.pid_namespace_device,
            pid_namespace_inode: self.pid_namespace_inode,
            cgroup_namespace_device: self.cgroup_namespace_device,
            cgroup_namespace_inode: self.cgroup_namespace_inode,
            mount_namespace_device: self.mount_namespace_device,
            mount_namespace_inode: self.mount_namespace_inode,
            process_group_id: self.process_group_id,
            session_id: self.session_id,
            stopped_observation_record_sha256: &self.stopped_observation_record_sha256,
            processes: self
                .processes
                .iter()
                .map(|process| RunnerRestoreProcessScopeBindingV8 {
                    pid: process.pid,
                    pidfd_bound: process.pidfd_bound,
                    start_ticks: process.start_ticks,
                    parent_pid: process.parent_pid,
                    process_group_id: process.process_group_id,
                    session_id: process.session_id,
                    executable_device: process.executable_device,
                    executable_inode: process.executable_inode,
                    executable_size: process.executable_size,
                    executable_sha256: &process.executable_sha256,
                    uid: process.uid,
                    gid: process.gid,
                    argv_sha256: &process.argv_sha256,
                    cwd_device: process.cwd_device,
                    cwd_inode: process.cwd_inode,
                })
                .collect(),
        };
        serde_json::to_vec(&binding)
            .map_err(|error| invalid(format!("encode runner RESTORE scope binding: {error}")))
    }

    fn closes_exact_manifest(&self, intent: &Self) -> bool {
        self.phase == RunnerRestoreEvidencePhaseV8::RunningObservation
            && intent.phase == RunnerRestoreEvidencePhaseV8::StoppedPreEffect
            && self.boot_id == intent.boot_id
            && self.unit_name == intent.unit_name
            && self.control_group == intent.control_group
            && self.main_pid == intent.main_pid
            && self.cgroup_device == intent.cgroup_device
            && self.cgroup_inode == intent.cgroup_inode
            && self.cgroup_mount_id == intent.cgroup_mount_id
            && self.proc_mount_id == intent.proc_mount_id
            && self.pid_namespace_device == intent.pid_namespace_device
            && self.pid_namespace_inode == intent.pid_namespace_inode
            && self.cgroup_namespace_device == intent.cgroup_namespace_device
            && self.cgroup_namespace_inode == intent.cgroup_namespace_inode
            && self.mount_namespace_device == intent.mount_namespace_device
            && self.mount_namespace_inode == intent.mount_namespace_inode
            && self.process_group_id == intent.process_group_id
            && self.session_id == intent.session_id
            && self.stopped_observation_record_sha256 == intent.stopped_observation_record_sha256
            && self.observation_started_boottime_ns >= intent.observation_completed_boottime_ns
            && self.observation_started_monotonic_ns >= intent.observation_completed_monotonic_ns
            && self.processes.len() == intent.processes.len()
            && self
                .processes
                .iter()
                .zip(&intent.processes)
                .all(|(after, before)| {
                    after.pid == before.pid
                        && after.pidfd_bound == before.pidfd_bound
                        && after.start_ticks == before.start_ticks
                        && after.parent_pid == before.parent_pid
                        && after.process_group_id == before.process_group_id
                        && after.session_id == before.session_id
                        && after.executable_device == before.executable_device
                        && after.executable_inode == before.executable_inode
                        && after.executable_size == before.executable_size
                        && after.executable_sha256 == before.executable_sha256
                        && after.uid == before.uid
                        && after.gid == before.gid
                        && after.argv_sha256 == before.argv_sha256
                        && after.cwd_device == before.cwd_device
                        && after.cwd_inode == before.cwd_inode
                })
    }

    fn validate(&self) -> Result<(), NativeErrorV8> {
        if self.schema != "hepta-linux-v8-runner-restore-evidence-v1"
            || self.boot_id.len() != 36
            || self.unit_name.is_empty()
            || self.unit_name.len() > 256
            || !self.control_group.starts_with('/')
            || self.main_pid == 0
            || self.cgroup_device == 0
            || self.cgroup_inode == 0
            || self.cgroup_mount_id == 0
            || self.proc_mount_id == 0
            || self.pid_namespace_device == 0
            || self.pid_namespace_inode == 0
            || self.cgroup_namespace_device == 0
            || self.cgroup_namespace_inode == 0
            || self.mount_namespace_device == 0
            || self.mount_namespace_inode == 0
            || self.process_group_id <= 1
            || self.session_id <= 1
            || self.observation_started_boottime_ns == 0
            || self.observation_completed_boottime_ns < self.observation_started_boottime_ns
            || self.observation_started_monotonic_ns == 0
            || self.observation_completed_monotonic_ns < self.observation_started_monotonic_ns
            || self.processes.len() != 6
        {
            return Err(invalid("runner RESTORE evidence header is malformed"));
        }
        super::validate_boot_id_v8(&self.boot_id)?;
        super::validate_digest(
            "runner RESTORE stopped observation record",
            &self.stopped_observation_record_sha256,
        )?;
        match self.phase {
            RunnerRestoreEvidencePhaseV8::StoppedPreEffect => {
                if self.intent_manifest_sha256.is_some() || self.intent_record_sha256.is_some() {
                    return Err(invalid(
                        "pre-effect runner RESTORE evidence cannot name an issued intent",
                    ));
                }
            }
            RunnerRestoreEvidencePhaseV8::RunningObservation => {
                super::validate_digest(
                    "runner RESTORE observation manifest",
                    self.intent_manifest_sha256.as_deref().ok_or_else(|| {
                        invalid("runner RESTORE observation lacks manifest digest")
                    })?,
                )?;
                super::validate_digest(
                    "runner RESTORE observation intent",
                    self.intent_record_sha256
                        .as_deref()
                        .ok_or_else(|| invalid("runner RESTORE observation lacks intent digest"))?,
                )?;
            }
        }
        let mut previous_pid = 0;
        for process in &self.processes {
            super::validate_digest("runner executable", &process.executable_sha256)?;
            super::validate_digest("runner argv", &process.argv_sha256)?;
            let stopped = matches!(process.state, b'T' | b't');
            if process.pid <= previous_pid
                || !process.pidfd_bound
                || process.start_ticks == 0
                || process.parent_pid == 0
                || process.process_group_id != self.process_group_id
                || process.session_id != self.session_id
                || process.executable_device == 0
                || process.executable_inode == 0
                || process.executable_size == 0
                || process.cwd_device == 0
                || process.cwd_inode == 0
                || (self.phase == RunnerRestoreEvidencePhaseV8::StoppedPreEffect && !stopped)
                || (self.phase == RunnerRestoreEvidencePhaseV8::RunningObservation
                    && (stopped || matches!(process.state, b'Z' | b'X' | b'I')))
            {
                return Err(invalid("runner RESTORE process evidence is malformed"));
            }
            previous_pid = process.pid;
        }
        if !self
            .processes
            .iter()
            .any(|process| process.pid == self.main_pid)
        {
            return Err(invalid("runner RESTORE evidence omits the exact main PID"));
        }
        Ok(())
    }
}

struct DecodedDurableJournalEventV8 {
    event: JournalEventV8,
    evidence: Vec<u8>,
}

/// Exact semantic payload stored inside a native durable journal record.
/// JSON is accepted only in the byte-for-byte form emitted by serde for the
/// frozen model type; whitespace, key reorder, aliases, and unknown fields are
/// rejected by decode-and-reencode equality.
pub fn encode_durable_journal_event_v8(event: &JournalEventV8) -> Result<Vec<u8>, NativeErrorV8> {
    encode_durable_journal_event_with_evidence_v8(event, &[])
}

fn encode_durable_journal_event_with_evidence_v8(
    event: &JournalEventV8,
    evidence: &[u8],
) -> Result<Vec<u8>, NativeErrorV8> {
    event
        .validate()
        .map_err(|error| invalid(format!("invalid durable journal event: {error}")))?;
    let encoded = serde_json::to_vec(event)
        .map_err(|error| invalid(format!("encode durable journal event: {error}")))?;
    if evidence.len() > MAX_DURABLE_EFFECT_EVIDENCE_BYTES_V8 {
        return Err(invalid("durable effect evidence exceeds the frozen bound"));
    }
    let event_length =
        u64::try_from(encoded.len()).map_err(|_| invalid("durable event length overflows"))?;
    let evidence_length =
        u64::try_from(evidence.len()).map_err(|_| invalid("durable evidence length overflows"))?;
    let mut payload = Vec::with_capacity(
        DURABLE_JOURNAL_EVENT_SCHEMA_V8.len() + 16 + encoded.len() + evidence.len(),
    );
    payload.extend_from_slice(DURABLE_JOURNAL_EVENT_SCHEMA_V8);
    payload.extend_from_slice(&event_length.to_be_bytes());
    payload.extend_from_slice(&encoded);
    payload.extend_from_slice(&evidence_length.to_be_bytes());
    payload.extend_from_slice(evidence);
    Ok(payload)
}

pub fn decode_durable_journal_event_v8(payload: &[u8]) -> Result<JournalEventV8, NativeErrorV8> {
    let decoded = decode_durable_journal_event_envelope_v8(payload)?;
    if !decoded.evidence.is_empty() {
        return Err(invalid(
            "durable journal event carries evidence; typed decoding is required",
        ));
    }
    Ok(decoded.event)
}

fn decode_durable_journal_event_envelope_v8(
    payload: &[u8],
) -> Result<DecodedDurableJournalEventV8, NativeErrorV8> {
    let encoded = payload
        .strip_prefix(DURABLE_JOURNAL_EVENT_SCHEMA_V8)
        .ok_or_else(|| invalid("durable journal event schema mismatches"))?;
    let (event_length_bytes, remainder) = encoded
        .split_first_chunk::<8>()
        .ok_or_else(|| invalid("durable journal event length is truncated"))?;
    let event_length = usize::try_from(u64::from_be_bytes(*event_length_bytes))
        .map_err(|_| invalid("durable journal event length overflows"))?;
    let event_bytes = remainder
        .get(..event_length)
        .ok_or_else(|| invalid("durable journal event bytes are truncated"))?;
    let remainder = &remainder[event_length..];
    let (evidence_length_bytes, evidence) = remainder
        .split_first_chunk::<8>()
        .ok_or_else(|| invalid("durable journal evidence length is truncated"))?;
    let evidence_length = usize::try_from(u64::from_be_bytes(*evidence_length_bytes))
        .map_err(|_| invalid("durable journal evidence length overflows"))?;
    if evidence.len() != evidence_length || evidence_length > MAX_DURABLE_EFFECT_EVIDENCE_BYTES_V8 {
        return Err(invalid("durable journal evidence length is not exact"));
    }
    let event: JournalEventV8 = serde_json::from_slice(event_bytes)
        .map_err(|error| invalid(format!("decode durable journal event: {error}")))?;
    event
        .validate()
        .map_err(|error| invalid(format!("invalid durable journal event: {error}")))?;
    if encode_durable_journal_event_with_evidence_v8(&event, evidence)? != payload {
        return Err(invalid("durable journal event bytes are not canonical"));
    }
    Ok(DecodedDurableJournalEventV8 {
        event,
        evidence: evidence.to_vec(),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum TypedPendingDurableEffectV8 {
    RunnerStop,
    RunnerRestore,
    CandidateExecution(CandidateExecutionEffectEvidenceV8),
}

impl TypedPendingDurableEffectV8 {
    fn effect(&self) -> JournalEffectV8 {
        match self {
            Self::RunnerStop => JournalEffectV8::RunnerStop,
            Self::RunnerRestore => JournalEffectV8::RunnerRestore,
            Self::CandidateExecution(_) => JournalEffectV8::CandidateExecution,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PendingDurableEffectV8 {
    typed: TypedPendingDurableEffectV8,
    effect_manifest_sha256: String,
    effect_manifest_bytes: Vec<u8>,
    intent_record_sha256: String,
}

impl PendingDurableEffectV8 {
    pub fn effect(&self) -> JournalEffectV8 {
        self.typed.effect()
    }

    pub fn effect_manifest_sha256(&self) -> &str {
        &self.effect_manifest_sha256
    }

    pub fn effect_manifest_bytes(&self) -> &[u8] {
        &self.effect_manifest_bytes
    }

    pub fn intent_record_sha256(&self) -> &str {
        &self.intent_record_sha256
    }
}

/// Complete descriptor-anchored fold of typed durable journal events.
/// A pending intent is not a retry permit: it is an issued-or-uncertain
/// recovery obligation and permanently blocks ordinary continuation after a
/// daemon restart.
#[derive(Debug)]
pub struct VerifiedDurableJournalEventScanV8 {
    journal: VerifiedDurableJournalScanV8,
    pending_effect: Option<PendingDurableEffectV8>,
    runner_stop_observation_record_sha256: Option<String>,
    qualification_phase: Option<QualificationJournalPhaseV8>,
    boot_recovery_detected: bool,
    current_boot_id: String,
    current_boot_mismatch_detected: bool,
    qualification_abandoned: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DurableJournalEventAssessmentV8 {
    pending_effect: Option<PendingDurableEffectV8>,
    runner_stop_observation_record_sha256: Option<String>,
    qualification_phase: Option<QualificationJournalPhaseV8>,
    boot_recovery_detected: bool,
    current_boot_id: String,
    current_boot_mismatch_detected: bool,
    qualification_abandoned: bool,
}

/// Internal descriptor-retained replay. Record descriptors are dropped before
/// the journal and attempt anchors, and all of them are dropped before the
/// owning trusted state root in the outer capsule. This is read-only evidence,
/// not a recovery token or an admission barrier.
pub(crate) struct VerifiedDescriptorBoundDurableJournalEventScanV8 {
    journal_records: VerifiedDescriptorBoundDurableJournalRecordsV8,
    journal_directory: DirectoryAnchorV8,
    journal_directory_identity: FileIdentityV8,
    journal_directory_metadata: TrustedNodeMetadataV8,
    attempt_directory: DirectoryAnchorV8,
    attempt_directory_identity: FileIdentityV8,
    attempt_directory_metadata: TrustedNodeMetadataV8,
    assessment: DurableJournalEventAssessmentV8,
}

impl VerifiedDurableJournalEventScanV8 {
    pub fn journal(&self) -> &VerifiedDurableJournalScanV8 {
        &self.journal
    }

    pub fn pending_effect(&self) -> Option<&PendingDurableEffectV8> {
        self.pending_effect.as_ref()
    }

    pub fn unfinished_intent_detected(&self) -> bool {
        self.pending_effect.is_some()
    }

    pub fn runner_stop_observation_record_sha256(&self) -> Option<&str> {
        self.runner_stop_observation_record_sha256.as_deref()
    }

    pub fn qualification_phase(&self) -> Option<QualificationJournalPhaseV8> {
        self.qualification_phase
    }

    pub fn boot_recovery_detected(&self) -> bool {
        self.boot_recovery_detected
    }

    pub fn current_boot_id(&self) -> &str {
        &self.current_boot_id
    }

    pub fn current_boot_mismatch_detected(&self) -> bool {
        self.current_boot_mismatch_detected
    }

    pub fn qualification_abandoned(&self) -> bool {
        self.qualification_abandoned
    }

    pub fn ordinary_continuation_allowed(&self) -> bool {
        self.qualification_phase.is_none()
            && ordinary_continuation_allowed_v8(
                self.unfinished_intent_detected(),
                self.boot_recovery_detected,
                self.current_boot_mismatch_detected,
                self.qualification_abandoned,
                self.journal.incoming_residue_detected(),
            )
    }
}

fn ordinary_continuation_allowed_v8(
    unfinished_intent: bool,
    boot_recovery_detected: bool,
    current_boot_mismatch_detected: bool,
    qualification_abandoned: bool,
    incoming_residue_detected: bool,
) -> bool {
    !unfinished_intent
        && !boot_recovery_detected
        && !current_boot_mismatch_detected
        && !qualification_abandoned
        && !incoming_residue_detected
}

pub fn scan_durable_journal_events_v8(
    state_root: &DirectoryAnchorV8,
    state_root_lock: &StateRootLockV8,
    expected_attempt_identity_sha256: &str,
) -> Result<VerifiedDurableJournalEventScanV8, NativeErrorV8> {
    if !state_root_lock
        .state_root_identity()
        .matches_stable_directory(state_root.identity())
    {
        return Err(invalid(
            "durable event replay lock belongs to a different state root",
        ));
    }
    state_root_lock.revalidate_for_root(state_root)?;
    let current_boot_before = crate::observe_boot_id_v8()?;
    let attempt_directory = attempt_relative_path_v8(expected_attempt_identity_sha256)?;
    let journal_relative = format!("{attempt_directory}/journal");
    let journal_directory = state_root.open_directory_beneath(Path::new(&journal_relative))?;
    let replay = scan_journal_directory_with_records_v8(
        &journal_directory,
        expected_attempt_identity_sha256,
        state_root.identity(),
    )?;
    let assessment = fold_durable_journal_events_v8(
        replay.records.iter(),
        &replay.scan,
        state_root,
        state_root_lock,
        current_boot_before,
        None,
    )?;
    Ok(VerifiedDurableJournalEventScanV8 {
        journal: replay.scan,
        pending_effect: assessment.pending_effect,
        runner_stop_observation_record_sha256: assessment.runner_stop_observation_record_sha256,
        qualification_phase: assessment.qualification_phase,
        boot_recovery_detected: assessment.boot_recovery_detected,
        current_boot_id: assessment.current_boot_id,
        current_boot_mismatch_detected: assessment.current_boot_mismatch_detected,
        qualification_abandoned: assessment.qualification_abandoned,
    })
}

pub(crate) fn scan_durable_journal_events_descriptor_bound_v8(
    state_root: &DirectoryAnchorV8,
    state_root_lock: &StateRootLockV8,
    expected_attempt_identity_sha256: &str,
    descriptor_origin: &DescriptorReplayOriginV8<'_>,
) -> Result<VerifiedDescriptorBoundDurableJournalEventScanV8, NativeErrorV8> {
    if !state_root_lock
        .state_root_identity()
        .matches_stable_directory(state_root.identity())
    {
        return Err(invalid(
            "descriptor-bound event replay lock belongs to a different state root",
        ));
    }
    state_root_lock.revalidate_for_root(state_root)?;
    let current_boot_before = crate::observe_boot_id_v8()?;
    let state_root_metadata = state_root.trusted_node_metadata()?;
    let attempt_relative = attempt_relative_path_v8(expected_attempt_identity_sha256)?;
    let attempt_directory = state_root.open_directory_beneath(Path::new(&attempt_relative))?;
    let attempt_directory_identity = attempt_directory.current_identity()?;
    let attempt_directory_metadata = attempt_directory.trusted_node_metadata()?;
    require_descriptor_bound_directory_v8(
        "attempt",
        attempt_directory_identity,
        attempt_directory_metadata,
        state_root.identity(),
        state_root_metadata,
    )?;
    let journal_directory =
        attempt_directory.open_directory_beneath(Path::new(JOURNAL_DIRECTORY_V8))?;
    let journal_directory_identity = journal_directory.current_identity()?;
    let journal_directory_metadata = journal_directory.trusted_node_metadata()?;
    require_descriptor_bound_directory_v8(
        "journal",
        journal_directory_identity,
        journal_directory_metadata,
        attempt_directory_identity,
        attempt_directory_metadata,
    )?;
    let journal_records = scan_journal_directory_descriptor_bound_v8(
        &journal_directory,
        expected_attempt_identity_sha256,
        state_root.identity(),
    )?;
    let assessment = fold_durable_journal_events_v8(
        journal_records
            .records
            .iter()
            .map(super::RetainedDurableJournalRecordV8::record),
        &journal_records.scan,
        state_root,
        state_root_lock,
        current_boot_before,
        Some(descriptor_origin),
    )?;
    revalidate_named_descriptor_bound_directories_v8(
        state_root,
        state_root_lock,
        expected_attempt_identity_sha256,
        &attempt_directory,
        attempt_directory_identity,
        attempt_directory_metadata,
        &journal_directory,
        journal_directory_identity,
        journal_directory_metadata,
    )?;
    journal_records.revalidate_descriptor_bound_v8(&journal_directory)?;
    Ok(VerifiedDescriptorBoundDurableJournalEventScanV8 {
        journal_records,
        journal_directory,
        journal_directory_identity,
        journal_directory_metadata,
        attempt_directory,
        attempt_directory_identity,
        attempt_directory_metadata,
        assessment,
    })
}

impl VerifiedDescriptorBoundDurableJournalEventScanV8 {
    pub(crate) fn journal(&self) -> &VerifiedDurableJournalScanV8 {
        &self.journal_records.scan
    }

    pub(crate) fn pending_effect(&self) -> Option<&PendingDurableEffectV8> {
        self.assessment.pending_effect.as_ref()
    }

    pub(crate) fn qualification_phase(&self) -> Option<QualificationJournalPhaseV8> {
        self.assessment.qualification_phase
    }

    pub(crate) fn boot_recovery_detected(&self) -> bool {
        self.assessment.boot_recovery_detected
    }

    pub(crate) fn current_boot_id(&self) -> &str {
        &self.assessment.current_boot_id
    }

    pub(crate) fn current_boot_mismatch_detected(&self) -> bool {
        self.assessment.current_boot_mismatch_detected
    }

    pub(crate) fn qualification_abandoned(&self) -> bool {
        self.assessment.qualification_abandoned
    }

    pub(crate) fn equivalent_read_only_assessment_v8(&self, other: &Self) -> bool {
        self.assessment == other.assessment
            && self.journal().attempt_identity_sha256() == other.journal().attempt_identity_sha256()
            && self.journal().incoming_residue_detected()
                == other.journal().incoming_residue_detected()
            && self.journal().last_boot_epoch() == other.journal().last_boot_epoch()
            && self.journal().last_boot_id() == other.journal().last_boot_id()
            && self.journal().record_count() == other.journal().record_count()
            && self.journal().state_root_identity() == other.journal().state_root_identity()
            && self.journal().tip_sha256() == other.journal().tip_sha256()
    }

    pub(crate) fn revalidate_descriptor_bound_v8(
        &self,
        state_root: &DirectoryAnchorV8,
        state_root_lock: &StateRootLockV8,
        descriptor_origin: &DescriptorReplayOriginV8<'_>,
    ) -> Result<(), NativeErrorV8> {
        revalidate_named_descriptor_bound_directories_v8(
            state_root,
            state_root_lock,
            self.journal().attempt_identity_sha256(),
            &self.attempt_directory,
            self.attempt_directory_identity,
            self.attempt_directory_metadata,
            &self.journal_directory,
            self.journal_directory_identity,
            self.journal_directory_metadata,
        )?;
        self.journal_records
            .revalidate_descriptor_bound_v8(&self.journal_directory)?;
        let current_boot_before = crate::observe_boot_id_v8()?;
        let assessment = fold_durable_journal_events_v8(
            self.journal_records
                .records
                .iter()
                .map(super::RetainedDurableJournalRecordV8::record),
            self.journal(),
            state_root,
            state_root_lock,
            current_boot_before,
            Some(descriptor_origin),
        )?;
        if assessment != self.assessment {
            return Err(invalid(
                "descriptor-bound journal semantics differ from the pinned read-only assessment",
            ));
        }
        self.journal_records
            .revalidate_descriptor_bound_v8(&self.journal_directory)?;
        revalidate_named_descriptor_bound_directories_v8(
            state_root,
            state_root_lock,
            self.journal().attempt_identity_sha256(),
            &self.attempt_directory,
            self.attempt_directory_identity,
            self.attempt_directory_metadata,
            &self.journal_directory,
            self.journal_directory_identity,
            self.journal_directory_metadata,
        )
    }
}

fn require_descriptor_bound_directory_v8(
    label: &str,
    identity: FileIdentityV8,
    metadata: TrustedNodeMetadataV8,
    parent_identity: FileIdentityV8,
    parent_metadata: TrustedNodeMetadataV8,
) -> Result<(), NativeErrorV8> {
    if identity.device() != parent_identity.device()
        || identity.owner_uid() != parent_identity.owner_uid()
        || identity.owner_gid() != parent_identity.owner_gid()
        || identity.mode() != 0o700
        || identity.link_count() == 0
        || !metadata.matches_filesystem_domain(parent_metadata)
    {
        return Err(invalid(format!(
            "descriptor-bound {label} directory identity or mount domain is not exact"
        )));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn revalidate_named_descriptor_bound_directories_v8(
    state_root: &DirectoryAnchorV8,
    state_root_lock: &StateRootLockV8,
    expected_attempt_identity_sha256: &str,
    attempt_directory: &DirectoryAnchorV8,
    attempt_identity: FileIdentityV8,
    attempt_metadata: TrustedNodeMetadataV8,
    journal_directory: &DirectoryAnchorV8,
    journal_identity: FileIdentityV8,
    journal_metadata: TrustedNodeMetadataV8,
) -> Result<(), NativeErrorV8> {
    state_root_lock.revalidate_for_root(state_root)?;
    if attempt_directory.current_identity()? != attempt_identity
        || attempt_directory.trusted_node_metadata()? != attempt_metadata
        || journal_directory.current_identity()? != journal_identity
        || journal_directory.trusted_node_metadata()? != journal_metadata
    {
        return Err(invalid(
            "retained attempt or journal directory identity drifted",
        ));
    }
    let attempt_relative = attempt_relative_path_v8(expected_attempt_identity_sha256)?;
    let named_attempt = state_root.open_directory_beneath(Path::new(&attempt_relative))?;
    if named_attempt.current_identity()? != attempt_identity
        || named_attempt.trusted_node_metadata()? != attempt_metadata
    {
        return Err(invalid(
            "attempt pathname no longer names the retained directory identity",
        ));
    }
    let retained_named_journal =
        attempt_directory.open_directory_beneath(Path::new(JOURNAL_DIRECTORY_V8))?;
    let fresh_named_journal =
        named_attempt.open_directory_beneath(Path::new(JOURNAL_DIRECTORY_V8))?;
    for named_journal in [&retained_named_journal, &fresh_named_journal] {
        if named_journal.current_identity()? != journal_identity
            || named_journal.trusted_node_metadata()? != journal_metadata
        {
            return Err(invalid(
                "journal pathname no longer names the retained directory identity",
            ));
        }
    }
    state_root_lock.revalidate_for_root(state_root)?;
    Ok(())
}

fn fold_durable_journal_events_v8<'a, I>(
    records: I,
    journal: &VerifiedDurableJournalScanV8,
    state_root: &DirectoryAnchorV8,
    state_root_lock: &StateRootLockV8,
    current_boot_before: crate::BootIdV8,
    descriptor_origin: Option<&DescriptorReplayOriginV8<'_>>,
) -> Result<DurableJournalEventAssessmentV8, NativeErrorV8>
where
    I: IntoIterator<Item = &'a DurableJournalRecordV8>,
{
    let state_root_mount_id = state_root.trusted_node_metadata()?.mount_id();
    let state_root_lock_identity = state_root_lock.identity();
    let mut pending_effect: Option<PendingDurableEffectV8> = None;
    let mut runner_stop_observation_record_sha256: Option<String> = None;
    let mut candidate_execution_observation_record_sha256: Option<String> = None;
    let mut candidate_execution_result_sha256: Option<String> = None;
    let mut candidate_completed_record_sha256: Option<String> = None;
    let mut qualification_phase: Option<QualificationJournalPhaseV8> = None;
    let mut boot_recovery_detected = false;
    let mut qualification_abandoned = false;
    let mut previous: Option<&DurableJournalRecordV8> = None;

    for (index, record) in records.into_iter().enumerate() {
        let decoded = decode_durable_journal_event_envelope_v8(record.payload())?;
        let event = decoded.event;
        let evidence = decoded.evidence;
        match &event {
            JournalEventV8::EffectIntent {
                effect: JournalEffectV8::RunnerStop,
                effect_manifest_sha256,
            } => {
                let typed = RunnerStopEffectEvidenceV8::decode_exact(&evidence)?;
                if typed.phase != RunnerStopEvidencePhaseV8::PreEffect
                    || format!("{:x}", sha2::Sha256::digest(&evidence)) != *effect_manifest_sha256
                {
                    return Err(invalid(
                        "runner STOP intent does not bind its exact typed manifest bytes",
                    ));
                }
            }
            JournalEventV8::EffectObserved {
                effect: JournalEffectV8::RunnerStop,
                intent_record_sha256,
                observation_sha256,
            } => {
                let typed = RunnerStopEffectEvidenceV8::decode_exact(&evidence)?;
                if typed.phase != RunnerStopEvidencePhaseV8::StoppedObservation
                    || typed.intent_record_sha256.as_deref() != Some(intent_record_sha256.as_str())
                    || format!("{:x}", sha2::Sha256::digest(&evidence)) != *observation_sha256
                {
                    return Err(invalid(
                        "runner STOP observation does not bind its exact typed closure bytes",
                    ));
                }
            }
            JournalEventV8::EffectIntent {
                effect: JournalEffectV8::RunnerRestore,
                effect_manifest_sha256,
            } => {
                let typed = RunnerRestoreEffectEvidenceV8::decode_exact(&evidence)?;
                if typed.phase != RunnerRestoreEvidencePhaseV8::StoppedPreEffect
                    || format!("{:x}", sha2::Sha256::digest(&evidence)) != *effect_manifest_sha256
                {
                    return Err(invalid(
                        "runner RESTORE intent does not bind its exact typed manifest bytes",
                    ));
                }
            }
            JournalEventV8::EffectObserved {
                effect: JournalEffectV8::RunnerRestore,
                intent_record_sha256,
                observation_sha256,
            } => {
                let typed = RunnerRestoreEffectEvidenceV8::decode_exact(&evidence)?;
                if typed.phase != RunnerRestoreEvidencePhaseV8::RunningObservation
                    || typed.intent_record_sha256.as_deref() != Some(intent_record_sha256.as_str())
                    || format!("{:x}", sha2::Sha256::digest(&evidence)) != *observation_sha256
                {
                    return Err(invalid(
                        "runner RESTORE observation does not bind its exact typed closure bytes",
                    ));
                }
            }
            JournalEventV8::EffectIntent {
                effect: JournalEffectV8::CandidateExecution,
                effect_manifest_sha256,
            } => {
                let typed = CandidateExecutionEffectEvidenceV8::decode_exact(&evidence)?;
                typed.validate_record_context(
                    record,
                    state_root.identity(),
                    state_root_mount_id,
                    state_root_lock_identity,
                )?;
                if let Some(origin) = descriptor_origin {
                    typed.validate_descriptor_origin(
                        record,
                        previous.ok_or_else(|| {
                            invalid("candidate intent lacks a retained predecessor")
                        })?,
                        origin,
                    )?;
                }
                if typed.phase() != FrozenTransitionEvidencePhaseV8::Intent
                    || format!("{:x}", sha2::Sha256::digest(&evidence)) != *effect_manifest_sha256
                {
                    return Err(invalid(
                        "transition intent does not bind its exact typed manifest bytes",
                    ));
                }
            }
            JournalEventV8::EffectObserved {
                effect: JournalEffectV8::CandidateExecution,
                intent_record_sha256,
                observation_sha256,
            } => {
                let typed = CandidateExecutionEffectEvidenceV8::decode_exact(&evidence)?;
                typed.validate_record_context(
                    record,
                    state_root.identity(),
                    state_root_mount_id,
                    state_root_lock_identity,
                )?;
                if let Some(origin) = descriptor_origin {
                    typed.validate_descriptor_origin(
                        record,
                        previous.ok_or_else(|| {
                            invalid("candidate observation lacks a retained predecessor")
                        })?,
                        origin,
                    )?;
                }
                if typed.phase() != FrozenTransitionEvidencePhaseV8::Observation
                    || typed.intent_record_sha256() != Some(intent_record_sha256.as_str())
                    || format!("{:x}", sha2::Sha256::digest(&evidence)) != *observation_sha256
                {
                    return Err(invalid(
                        "transition observation does not bind its exact typed closure bytes",
                    ));
                }
            }
            JournalEventV8::EffectIntent {
                effect:
                    JournalEffectV8::CandidateRelay
                    | JournalEffectV8::PostRestoreSnapshot
                    | JournalEffectV8::BarrierRelease,
                ..
            }
            | JournalEventV8::EffectObserved {
                effect:
                    JournalEffectV8::CandidateRelay
                    | JournalEffectV8::PostRestoreSnapshot
                    | JournalEffectV8::BarrierRelease,
                ..
            } => {
                return Err(invalid(
                    "effect remains unsupported until its V2 semantic schema and backend are frozen",
                ));
            }
            JournalEventV8::AttemptOpened { .. }
            | JournalEventV8::CandidateCompleted { .. }
            | JournalEventV8::BootRecovery { .. }
            | JournalEventV8::QualificationAbandoned { .. }
                if !evidence.is_empty() =>
            {
                return Err(invalid(
                    "non-effect durable event cannot carry effect evidence",
                ));
            }
            _ => {}
        }
        if index == 0 {
            if !matches!(event, JournalEventV8::AttemptOpened { .. }) {
                return Err(invalid(
                    "typed durable journal must begin with ATTEMPT_OPENED",
                ));
            }
            previous = Some(record);
            continue;
        }
        let prior = previous.ok_or_else(|| invalid("durable event fold lost its predecessor"))?;
        let prior_record_sha256 = prior.record_sha256()?;
        let boot_changed = record.boot_epoch() != prior.boot_epoch();
        if boot_changed {
            match &event {
                JournalEventV8::BootRecovery {
                    previous_boot_id,
                    previous_journal_tip_sha256,
                    ..
                } if previous_boot_id == prior.boot_id()
                    && previous_journal_tip_sha256 == &prior_record_sha256 => {}
                _ => {
                    return Err(invalid(
                        "new durable boot epoch must bind exact BOOT_RECOVERY predecessor",
                    ));
                }
            }
        } else if matches!(event, JournalEventV8::BootRecovery { .. }) {
            return Err(invalid(
                "BOOT_RECOVERY cannot occur within one durable boot epoch",
            ));
        }
        if qualification_abandoned {
            return Err(invalid(
                "typed durable journal contains records after abandonment",
            ));
        }
        if boot_recovery_detected && !matches!(event, JournalEventV8::QualificationAbandoned { .. })
        {
            return Err(invalid(
                "rebooted durable journal may only record terminal abandonment",
            ));
        }

        match event {
            JournalEventV8::AttemptOpened { .. } => {
                return Err(invalid("typed durable journal reopens an attempt"));
            }
            JournalEventV8::EffectIntent {
                effect,
                effect_manifest_sha256,
            } => {
                if pending_effect.is_some() {
                    return Err(invalid(
                        "typed durable journal contains nested effect intents",
                    ));
                }
                if effect == JournalEffectV8::RunnerRestore {
                    let typed = RunnerRestoreEffectEvidenceV8::decode_exact(&evidence)?;
                    if runner_stop_observation_record_sha256.as_deref()
                        != Some(typed.stopped_observation_record_sha256.as_str())
                    {
                        return Err(invalid(
                            "runner RESTORE intent does not bind the durable runner STOP observation",
                        ));
                    }
                    if candidate_execution_observation_record_sha256.is_some() {
                        return Err(invalid(
                            "runner RESTORE cannot bypass the still-unsupported typed relay lifecycle",
                        ));
                    }
                }
                let typed = match effect {
                    JournalEffectV8::RunnerStop => TypedPendingDurableEffectV8::RunnerStop,
                    JournalEffectV8::RunnerRestore => TypedPendingDurableEffectV8::RunnerRestore,
                    JournalEffectV8::CandidateExecution => {
                        let typed = CandidateExecutionEffectEvidenceV8::decode_exact(&evidence)?;
                        let expected_predecessor = runner_stop_observation_record_sha256
                            .as_deref()
                            .ok_or_else(|| {
                                invalid(
                                    "candidate execution intent lacks a durable runner STOP observation",
                                )
                            })?;
                        if candidate_execution_observation_record_sha256.is_some()
                            || candidate_completed_record_sha256.is_some()
                            || qualification_phase.is_some()
                            || typed.predecessor_record_sha256() != expected_predecessor
                            || prior_record_sha256 != expected_predecessor
                        {
                            return Err(invalid(
                                "candidate execution intent is duplicated or does not bind its exact runner STOP predecessor",
                            ));
                        }
                        qualification_phase = Some(
                            QualificationJournalPhaseV8::AwaitCandidateExecutionIntent.advance(
                                &JournalEventV8::EffectIntent {
                                    effect,
                                    effect_manifest_sha256: effect_manifest_sha256.clone(),
                                },
                            )
                            .map_err(|error| {
                                invalid(format!(
                                    "shared qualification phase rejected candidate intent: {error}"
                                ))
                            })?,
                        );
                        TypedPendingDurableEffectV8::CandidateExecution(typed)
                    }
                    JournalEffectV8::CandidateRelay
                    | JournalEffectV8::PostRestoreSnapshot
                    | JournalEffectV8::BarrierRelease => {
                        return Err(invalid(
                            "effect remains unsupported until its V2 semantic schema and backend are frozen",
                        ));
                    }
                };
                pending_effect = Some(PendingDurableEffectV8 {
                    typed,
                    effect_manifest_sha256,
                    effect_manifest_bytes: evidence,
                    intent_record_sha256: record.record_sha256()?,
                });
            }
            JournalEventV8::EffectObserved {
                effect,
                intent_record_sha256,
                observation_sha256: _,
            } => match pending_effect.as_ref() {
                Some(pending)
                    if pending.effect() == effect
                        && pending.intent_record_sha256 == intent_record_sha256 =>
                {
                    match effect {
                        JournalEffectV8::RunnerStop => {
                            let typed = RunnerStopEffectEvidenceV8::decode_exact(&evidence)?;
                            let intent_typed = RunnerStopEffectEvidenceV8::decode_exact(
                                &pending.effect_manifest_bytes,
                            )?;
                            if typed.intent_manifest_sha256.as_deref()
                                != Some(pending.effect_manifest_sha256.as_str())
                                || !typed.closes_exact_manifest(&intent_typed)
                            {
                                return Err(invalid(
                                    "runner STOP observation does not preserve the exact typed manifest closure",
                                ));
                            }
                            runner_stop_observation_record_sha256 = Some(record.record_sha256()?);
                        }
                        JournalEffectV8::RunnerRestore => {
                            let typed = RunnerRestoreEffectEvidenceV8::decode_exact(&evidence)?;
                            let intent_typed = RunnerRestoreEffectEvidenceV8::decode_exact(
                                &pending.effect_manifest_bytes,
                            )?;
                            if typed.intent_manifest_sha256.as_deref()
                                != Some(pending.effect_manifest_sha256.as_str())
                                || !typed.closes_exact_manifest(&intent_typed)
                            {
                                return Err(invalid(
                                    "runner RESTORE observation does not preserve the exact typed manifest closure",
                                ));
                            }
                        }
                        JournalEffectV8::CandidateExecution => {
                            let typed =
                                CandidateExecutionEffectEvidenceV8::decode_exact(&evidence)?;
                            let intent_typed = match &pending.typed {
                                TypedPendingDurableEffectV8::CandidateExecution(intent) => intent,
                                TypedPendingDurableEffectV8::RunnerStop
                                | TypedPendingDurableEffectV8::RunnerRestore => {
                                    return Err(invalid(
                                        "candidate execution observation has a non-candidate pending type",
                                    ));
                                }
                            };
                            if !typed.closes_exact_manifest(
                                intent_typed,
                                &pending.effect_manifest_sha256,
                                &pending.intent_record_sha256,
                            ) {
                                return Err(invalid(
                                    "transition observation does not preserve the exact typed intent/issued closure",
                                ));
                            }
                            let result = typed
                                .effect_result_sha256()
                                .ok_or_else(|| {
                                    invalid("typed transition observation omits its result")
                                })?
                                .to_string();
                            candidate_execution_observation_record_sha256 =
                                Some(record.record_sha256()?);
                            candidate_execution_result_sha256 = Some(result);
                            qualification_phase = Some(
                                qualification_phase
                                    .ok_or_else(|| {
                                        invalid(
                                            "candidate execution observation lacks its shared phase",
                                        )
                                    })?
                                    .advance(&JournalEventV8::EffectObserved {
                                        effect,
                                        intent_record_sha256: intent_record_sha256.clone(),
                                        observation_sha256: typed.sha256()?,
                                    })
                                    .map_err(|error| {
                                        invalid(format!(
                                            "shared qualification phase rejected candidate observation: {error}"
                                        ))
                                    })?,
                            );
                        }
                        JournalEffectV8::CandidateRelay
                        | JournalEffectV8::PostRestoreSnapshot
                        | JournalEffectV8::BarrierRelease => {
                            return Err(invalid(
                                "effect remains unsupported until its V2 semantic schema and backend are frozen",
                            ));
                        }
                    }
                    pending_effect = None;
                }
                _ => {
                    return Err(invalid(
                        "effect observation does not close the exact durable intent",
                    ));
                }
            },
            JournalEventV8::CandidateCompleted {
                candidate_result_sha256,
            } => {
                if pending_effect.is_some() {
                    return Err(invalid(
                        "candidate completion cannot bypass a durable effect obligation",
                    ));
                }
                if candidate_completed_record_sha256.is_some()
                    || candidate_execution_result_sha256.as_deref()
                        != Some(candidate_result_sha256.as_str())
                    || candidate_execution_observation_record_sha256.as_deref()
                        != Some(prior_record_sha256.as_str())
                {
                    return Err(invalid(
                        "candidate completion does not bind the exact typed execution observation",
                    ));
                }
                candidate_completed_record_sha256 = Some(record.record_sha256()?);
                qualification_phase = Some(
                    qualification_phase
                        .ok_or_else(|| {
                            invalid("candidate completion lacks its shared qualification phase")
                        })?
                        .advance(&JournalEventV8::CandidateCompleted {
                            candidate_result_sha256,
                        })
                        .map_err(|error| {
                            invalid(format!(
                                "shared qualification phase rejected candidate completion: {error}"
                            ))
                        })?,
                );
            }
            JournalEventV8::BootRecovery { .. } => {
                boot_recovery_detected = true;
            }
            JournalEventV8::QualificationAbandoned { .. } => {
                qualification_abandoned = true;
            }
        }
        previous = Some(record);
    }
    state_root_lock.revalidate_for_root(state_root)?;
    let current_boot_after = crate::observe_boot_id_v8()?;
    if current_boot_before != current_boot_after {
        return Err(invalid(
            "kernel boot identity changed during durable event replay",
        ));
    }
    let current_boot_id = current_boot_after.to_string();
    let current_boot_mismatch_detected = journal.last_boot_id() != current_boot_id;
    Ok(DurableJournalEventAssessmentV8 {
        pending_effect,
        runner_stop_observation_record_sha256,
        qualification_phase,
        boot_recovery_detected,
        current_boot_id,
        current_boot_mismatch_detected,
        qualification_abandoned,
    })
}

/// Non-cloneable proof that a RunnerStop intent was durably appended and the
/// complete typed journal replayed to that exact record before any runner
/// cgroup/process mutation. The cgroup API consumes this proof at its first
/// effect boundary and carries it through every issued-or-uncertain token.
#[derive(Debug)]
pub(crate) struct DurableRunnerStopIntentV8 {
    attempt_identity_sha256: String,
    active_attempt_record_sha256: String,
    boot_epoch: u64,
    boot_id: String,
    global_sequence: u64,
    effect_manifest_sha256: String,
    effect_manifest_bytes: Vec<u8>,
    scope_binding_bytes: Vec<u8>,
    intent_record_sha256: String,
    state_root_identity: Option<FileIdentityV8>,
}

impl DurableRunnerStopIntentV8 {
    pub(crate) fn intent_record_sha256(&self) -> &str {
        &self.intent_record_sha256
    }

    pub(crate) fn effect_manifest_sha256(&self) -> &str {
        &self.effect_manifest_sha256
    }

    pub(crate) fn effect_manifest_bytes(&self) -> &[u8] {
        &self.effect_manifest_bytes
    }

    pub(crate) fn scope_binding_bytes(&self) -> &[u8] {
        &self.scope_binding_bytes
    }

    pub(crate) fn attempt_identity_sha256(&self) -> &str {
        &self.attempt_identity_sha256
    }

    pub(crate) fn boot_epoch(&self) -> u64 {
        self.boot_epoch
    }

    pub(crate) fn boot_id(&self) -> &str {
        &self.boot_id
    }

    pub(crate) fn global_sequence(&self) -> u64 {
        self.global_sequence
    }

    pub(crate) fn matches_origin(
        &self,
        state_root: FileIdentityV8,
        active_attempt: &FreshActiveAttemptPublicationV8,
    ) -> bool {
        self.state_root_identity
            .is_some_and(|identity| identity.matches_stable_directory(state_root))
            && self.attempt_identity_sha256 == active_attempt.attempt_identity_sha256()
            && self.active_attempt_record_sha256 == active_attempt.record_sha256()
            && self.boot_id == active_attempt.boot_id()
    }

    #[cfg(test)]
    pub(crate) fn test_only_for_observed(
        observed: &ObservedRunnerScopeV8,
    ) -> Result<Self, NativeSysErrorV8> {
        let manifest = observed.capture_runner_stop_manifest_v8()?;
        let effect_manifest_bytes = manifest
            .canonical_bytes()
            .map_err(|error| NativeSysErrorV8::IdentityMismatch(error.to_string()))?;
        let scope_binding_bytes = manifest
            .scope_binding_bytes()
            .map_err(|error| NativeSysErrorV8::IdentityMismatch(error.to_string()))?;
        let effect_manifest_sha256 = manifest
            .sha256()
            .map_err(|error| NativeSysErrorV8::IdentityMismatch(error.to_string()))?;
        Ok(Self {
            attempt_identity_sha256: "1".repeat(64),
            active_attempt_record_sha256: "2".repeat(64),
            boot_epoch: 1,
            boot_id: manifest.boot_id,
            global_sequence: 2,
            effect_manifest_sha256,
            effect_manifest_bytes,
            scope_binding_bytes,
            intent_record_sha256: "4".repeat(64),
            state_root_identity: None,
        })
    }
}

#[must_use = "runner STOP intent failure retains the exact observed scope"]
#[derive(Debug)]
pub(crate) struct DurableRunnerStopIntentFailureV8 {
    observed: ObservedRunnerScopeV8,
    cause: NativeErrorV8,
    journal_publication_issued_or_uncertain: bool,
}

impl DurableRunnerStopIntentFailureV8 {
    pub(crate) fn cause(&self) -> &NativeErrorV8 {
        &self.cause
    }

    pub(crate) fn journal_publication_issued_or_uncertain(&self) -> bool {
        self.journal_publication_issued_or_uncertain
    }

    pub(crate) fn into_observed(self) -> ObservedRunnerScopeV8 {
        self.observed
    }
}

pub(crate) fn append_runner_stop_intent_durably_v8(
    state_root: &DirectoryAnchorV8,
    state_root_lock: &mut StateRootLockV8,
    active_attempt: &FreshActiveAttemptPublicationV8,
    observed: ObservedRunnerScopeV8,
    publication_nonce: &str,
) -> Result<
    (
        ObservedRunnerScopeV8,
        DurableRunnerStopIntentV8,
        PublishedDurableJournalRecordV8,
    ),
    DurableRunnerStopIntentFailureV8,
> {
    let fail = |observed, cause, journal_publication_issued_or_uncertain| {
        DurableRunnerStopIntentFailureV8 {
            observed,
            cause,
            journal_publication_issued_or_uncertain,
        }
    };
    let before = match scan_durable_journal_events_v8(
        state_root,
        state_root_lock,
        active_attempt.attempt_identity_sha256(),
    ) {
        Ok(before) => before,
        Err(cause) => return Err(fail(observed, cause, false)),
    };
    if !before.ordinary_continuation_allowed()
        || before.current_boot_id() != active_attempt.boot_id()
        || before.journal().last_boot_id() != active_attempt.boot_id()
    {
        return Err(fail(
            observed,
            invalid("runner STOP durable issue requires one clean current-boot typed journal tip"),
            false,
        ));
    }
    let manifest = match observed.capture_runner_stop_manifest_v8() {
        Ok(manifest) => manifest,
        Err(cause) => return Err(fail(observed, cause.into(), false)),
    };
    let effect_manifest_bytes = match manifest.canonical_bytes() {
        Ok(bytes) => bytes,
        Err(cause) => return Err(fail(observed, cause, false)),
    };
    let effect_manifest_sha256 = match manifest.sha256() {
        Ok(digest) => digest,
        Err(cause) => return Err(fail(observed, cause, false)),
    };
    let scope_binding_bytes = match manifest.scope_binding_bytes() {
        Ok(bytes) => bytes,
        Err(cause) => return Err(fail(observed, cause, false)),
    };
    let payload = match encode_durable_journal_event_with_evidence_v8(
        &JournalEventV8::EffectIntent {
            effect: JournalEffectV8::RunnerStop,
            effect_manifest_sha256: effect_manifest_sha256.clone(),
        },
        &effect_manifest_bytes,
    ) {
        Ok(payload) => payload,
        Err(cause) => return Err(fail(observed, cause, false)),
    };
    let global_sequence = match before.journal().record_count().checked_add(1) {
        Some(sequence) => sequence,
        None => {
            return Err(fail(
                observed,
                invalid("runner STOP intent journal sequence overflows"),
                false,
            ));
        }
    };
    let record = match DurableJournalRecordV8::new(
        active_attempt.attempt_identity_sha256().to_string(),
        before.journal().last_boot_epoch(),
        active_attempt.boot_id().to_string(),
        global_sequence,
        before.journal().tip_sha256().to_string(),
        payload,
    ) {
        Ok(record) => record,
        Err(cause) => return Err(fail(observed, cause, false)),
    };
    let intent_record_sha256 = match record.record_sha256() {
        Ok(digest) => digest,
        Err(cause) => return Err(fail(observed, cause, false)),
    };
    let mut journal_publication_issued_or_uncertain = false;
    let publication = match super::journal_publish::append_journal_record_durably_observed_v8(
        state_root,
        state_root_lock,
        active_attempt,
        &record,
        publication_nonce,
        |_| journal_publication_issued_or_uncertain = true,
    ) {
        Ok(publication) => publication,
        Err(cause) => {
            return Err(fail(
                observed,
                cause,
                journal_publication_issued_or_uncertain,
            ));
        }
    };
    let after = match scan_durable_journal_events_v8(
        state_root,
        state_root_lock,
        active_attempt.attempt_identity_sha256(),
    ) {
        Ok(after) => after,
        Err(cause) => return Err(fail(observed, cause, true)),
    };
    let Some(pending) = after.pending_effect() else {
        return Err(fail(
            observed,
            invalid("runner STOP intent disappeared during durable replay"),
            true,
        ));
    };
    if pending.effect() != JournalEffectV8::RunnerStop
        || pending.effect_manifest_sha256() != effect_manifest_sha256
        || pending.effect_manifest_bytes() != effect_manifest_bytes
        || pending.intent_record_sha256() != intent_record_sha256
        || after.journal().tip_sha256() != intent_record_sha256
    {
        return Err(fail(
            observed,
            invalid("runner STOP durable replay does not bind the exact appended typed intent"),
            true,
        ));
    }
    let intent = DurableRunnerStopIntentV8 {
        attempt_identity_sha256: active_attempt.attempt_identity_sha256().to_string(),
        active_attempt_record_sha256: active_attempt.record_sha256().to_string(),
        boot_epoch: record.boot_epoch(),
        boot_id: record.boot_id().to_string(),
        global_sequence: record.global_sequence(),
        effect_manifest_sha256,
        effect_manifest_bytes,
        scope_binding_bytes,
        intent_record_sha256,
        state_root_identity: Some(state_root.identity()),
    };
    Ok((observed, intent, publication))
}

#[must_use = "runner STOP observation failure retains the exact stopped closure"]
#[derive(Debug)]
pub(crate) struct DurableRunnerStopObservationFailureV8 {
    stopped: StoppedRunnerScopeV8,
    cause: NativeErrorV8,
    publication_issued_or_uncertain: bool,
}

impl DurableRunnerStopObservationFailureV8 {
    pub(crate) fn cause(&self) -> &NativeErrorV8 {
        &self.cause
    }

    pub(crate) fn publication_issued_or_uncertain(&self) -> bool {
        self.publication_issued_or_uncertain
    }

    pub(crate) fn into_stopped(self) -> StoppedRunnerScopeV8 {
        self.stopped
    }
}

/// Successful exact observation publication plus the only normal CONT token.
#[must_use = "durable runner STOP closure retains the ordinary restore route"]
#[derive(Debug)]
pub(crate) struct DurableRunnerStopObservationV8 {
    stopped: DurablyObservedStoppedRunnerScopeV8,
    publication: PublishedDurableJournalRecordV8,
}

impl DurableRunnerStopObservationV8 {
    pub(crate) fn into_stopped(self) -> DurablyObservedStoppedRunnerScopeV8 {
        self.stopped
    }

    pub(crate) fn publication(&self) -> &PublishedDurableJournalRecordV8 {
        &self.publication
    }
}

/// Closes the exact RunnerStop obligation only after the cgroup/process
/// typestate proves the complete runner closure stopped. Any failure after an
/// incoming journal file exists is explicitly issued-or-uncertain and retains
/// the stopped kernel obligation; it never returns a fresh append permit.
pub(crate) fn append_runner_stop_observation_durably_v8(
    state_root: &DirectoryAnchorV8,
    state_root_lock: &mut StateRootLockV8,
    active_attempt: &FreshActiveAttemptPublicationV8,
    stopped: StoppedRunnerScopeV8,
    publication_nonce: &str,
) -> Result<DurableRunnerStopObservationV8, DurableRunnerStopObservationFailureV8> {
    let fail =
        |stopped, cause, publication_issued_or_uncertain| DurableRunnerStopObservationFailureV8 {
            stopped,
            cause,
            publication_issued_or_uncertain,
        };
    let observation = match stopped.capture_runner_stop_observation_v8() {
        Ok(observation) => observation,
        Err(cause) => return Err(fail(stopped, cause.into(), false)),
    };
    let observation_bytes = match observation.canonical_bytes() {
        Ok(bytes) => bytes,
        Err(cause) => return Err(fail(stopped, cause, false)),
    };
    let observation_sha256 = match observation.sha256() {
        Ok(digest) => digest,
        Err(cause) => return Err(fail(stopped, cause, false)),
    };
    let Some(intent) = stopped.runner_stop_intent() else {
        return Err(fail(
            stopped,
            invalid("stopped runner scope lacks its durable STOP obligation"),
            false,
        ));
    };
    if !intent.matches_origin(state_root.identity(), active_attempt) {
        return Err(fail(
            stopped,
            invalid("stopped runner durable STOP obligation has a different origin"),
            false,
        ));
    }
    let intent_record_sha256 = intent.intent_record_sha256().to_string();
    let attempt_identity_sha256 = intent.attempt_identity_sha256().to_string();
    let boot_epoch = intent.boot_epoch();
    let boot_id = intent.boot_id().to_string();
    let next_sequence = match intent.global_sequence().checked_add(1) {
        Some(sequence) => sequence,
        None => {
            return Err(fail(
                stopped,
                invalid("runner STOP observation journal sequence overflows"),
                false,
            ));
        }
    };
    let payload = match encode_durable_journal_event_with_evidence_v8(
        &JournalEventV8::EffectObserved {
            effect: JournalEffectV8::RunnerStop,
            intent_record_sha256: intent_record_sha256.clone(),
            observation_sha256,
        },
        &observation_bytes,
    ) {
        Ok(payload) => payload,
        Err(cause) => return Err(fail(stopped, cause, false)),
    };
    let record = match DurableJournalRecordV8::new(
        attempt_identity_sha256.clone(),
        boot_epoch,
        boot_id,
        next_sequence,
        intent_record_sha256.clone(),
        payload,
    ) {
        Ok(record) => record,
        Err(cause) => return Err(fail(stopped, cause, false)),
    };
    let observation_record_sha256 = match record.record_sha256() {
        Ok(digest) => digest,
        Err(cause) => return Err(fail(stopped, cause, false)),
    };
    let mut publication_issued_or_uncertain = false;
    let publication = match super::journal_publish::append_journal_record_durably_observed_v8(
        state_root,
        state_root_lock,
        active_attempt,
        &record,
        publication_nonce,
        |_| publication_issued_or_uncertain = true,
    ) {
        Ok(publication) => publication,
        Err(cause) => {
            return Err(fail(stopped, cause, publication_issued_or_uncertain));
        }
    };
    let replay =
        match scan_durable_journal_events_v8(state_root, state_root_lock, &attempt_identity_sha256)
        {
            Ok(replay) => replay,
            Err(cause) => return Err(fail(stopped, cause, true)),
        };
    if replay.unfinished_intent_detected()
        || replay.journal().tip_sha256() != observation_record_sha256
    {
        return Err(fail(
            stopped,
            invalid("runner STOP observation replay did not close the exact intent"),
            true,
        ));
    }
    let stopped = match DurablyObservedStoppedRunnerScopeV8::bind_durable_observation(
        stopped,
        &intent_record_sha256,
        observation_record_sha256,
    ) {
        Ok(stopped) => stopped,
        Err(stopped) => {
            return Err(fail(
                stopped,
                invalid("runner STOP typestate lost its published intent binding"),
                true,
            ));
        }
    };
    Ok(DurableRunnerStopObservationV8 {
        stopped,
        publication,
    })
}

/// Non-cloneable proof that a RunnerRestore intent was durably appended after
/// the exact RunnerStop observation and replayed before the unique SIGCONT
/// authority can be planned.
#[derive(Debug)]
pub(crate) struct DurableRunnerRestoreIntentV8 {
    attempt_identity_sha256: String,
    active_attempt_record_sha256: String,
    boot_epoch: u64,
    boot_id: String,
    global_sequence: u64,
    stopped_observation_record_sha256: String,
    effect_manifest_sha256: String,
    effect_manifest_bytes: Vec<u8>,
    scope_binding_bytes: Vec<u8>,
    intent_record_sha256: String,
    state_root_identity: Option<FileIdentityV8>,
}

impl DurableRunnerRestoreIntentV8 {
    pub(crate) fn intent_record_sha256(&self) -> &str {
        &self.intent_record_sha256
    }

    pub(crate) fn effect_manifest_sha256(&self) -> &str {
        &self.effect_manifest_sha256
    }

    pub(crate) fn effect_manifest_bytes(&self) -> &[u8] {
        &self.effect_manifest_bytes
    }

    pub(crate) fn scope_binding_bytes(&self) -> &[u8] {
        &self.scope_binding_bytes
    }

    pub(crate) fn stopped_observation_record_sha256(&self) -> &str {
        &self.stopped_observation_record_sha256
    }

    pub(crate) fn attempt_identity_sha256(&self) -> &str {
        &self.attempt_identity_sha256
    }

    pub(crate) fn boot_epoch(&self) -> u64 {
        self.boot_epoch
    }

    pub(crate) fn boot_id(&self) -> &str {
        &self.boot_id
    }

    pub(crate) fn global_sequence(&self) -> u64 {
        self.global_sequence
    }

    pub(crate) fn matches_origin(
        &self,
        state_root: FileIdentityV8,
        active_attempt: &FreshActiveAttemptPublicationV8,
    ) -> bool {
        self.state_root_identity
            .is_some_and(|identity| identity.matches_stable_directory(state_root))
            && self.attempt_identity_sha256 == active_attempt.attempt_identity_sha256()
            && self.active_attempt_record_sha256 == active_attempt.record_sha256()
            && self.boot_id == active_attempt.boot_id()
    }

    #[cfg(test)]
    pub(crate) fn test_only_for_stopped(
        stopped: &DurablyObservedStoppedRunnerScopeV8,
    ) -> Result<Self, NativeSysErrorV8> {
        let manifest = stopped.capture_runner_restore_manifest_v8()?;
        let effect_manifest_bytes = manifest
            .canonical_bytes()
            .map_err(|error| NativeSysErrorV8::IdentityMismatch(error.to_string()))?;
        let scope_binding_bytes = manifest
            .scope_binding_bytes()
            .map_err(|error| NativeSysErrorV8::IdentityMismatch(error.to_string()))?;
        let effect_manifest_sha256 = manifest
            .sha256()
            .map_err(|error| NativeSysErrorV8::IdentityMismatch(error.to_string()))?;
        Ok(Self {
            attempt_identity_sha256: "6".repeat(64),
            active_attempt_record_sha256: "7".repeat(64),
            boot_epoch: 1,
            boot_id: manifest.boot_id,
            global_sequence: 4,
            stopped_observation_record_sha256: stopped.observation_record_sha256().to_string(),
            effect_manifest_sha256,
            effect_manifest_bytes,
            scope_binding_bytes,
            intent_record_sha256: "8".repeat(64),
            state_root_identity: None,
        })
    }
}

#[must_use = "runner RESTORE intent failure retains the exact durably stopped scope"]
#[derive(Debug)]
pub(crate) struct DurableRunnerRestoreIntentFailureV8 {
    stopped: DurablyObservedStoppedRunnerScopeV8,
    cause: NativeErrorV8,
    journal_publication_issued_or_uncertain: bool,
}

impl DurableRunnerRestoreIntentFailureV8 {
    pub(crate) fn cause(&self) -> &NativeErrorV8 {
        &self.cause
    }

    pub(crate) fn journal_publication_issued_or_uncertain(&self) -> bool {
        self.journal_publication_issued_or_uncertain
    }

    pub(crate) fn into_stopped(self) -> DurablyObservedStoppedRunnerScopeV8 {
        self.stopped
    }
}

pub(crate) fn append_runner_restore_intent_durably_v8(
    state_root: &DirectoryAnchorV8,
    state_root_lock: &mut StateRootLockV8,
    active_attempt: &FreshActiveAttemptPublicationV8,
    stopped: DurablyObservedStoppedRunnerScopeV8,
    publication_nonce: &str,
) -> Result<
    (
        RestoreAuthorizedStoppedRunnerScopeV8,
        PublishedDurableJournalRecordV8,
    ),
    DurableRunnerRestoreIntentFailureV8,
> {
    let fail = |stopped, cause, journal_publication_issued_or_uncertain| {
        DurableRunnerRestoreIntentFailureV8 {
            stopped,
            cause,
            journal_publication_issued_or_uncertain,
        }
    };
    let before = match scan_durable_journal_events_v8(
        state_root,
        state_root_lock,
        active_attempt.attempt_identity_sha256(),
    ) {
        Ok(before) => before,
        Err(cause) => return Err(fail(stopped, cause, false)),
    };
    if !before.ordinary_continuation_allowed()
        || before.current_boot_id() != active_attempt.boot_id()
        || before.journal().last_boot_id() != active_attempt.boot_id()
        || before.runner_stop_observation_record_sha256()
            != Some(stopped.observation_record_sha256())
    {
        return Err(fail(
            stopped,
            invalid(
                "runner RESTORE durable issue requires the exact current-boot runner STOP observation",
            ),
            false,
        ));
    }
    let manifest = match stopped.capture_runner_restore_manifest_v8() {
        Ok(manifest) => manifest,
        Err(cause) => return Err(fail(stopped, cause.into(), false)),
    };
    let effect_manifest_bytes = match manifest.canonical_bytes() {
        Ok(bytes) => bytes,
        Err(cause) => return Err(fail(stopped, cause, false)),
    };
    let effect_manifest_sha256 = match manifest.sha256() {
        Ok(digest) => digest,
        Err(cause) => return Err(fail(stopped, cause, false)),
    };
    let scope_binding_bytes = match manifest.scope_binding_bytes() {
        Ok(bytes) => bytes,
        Err(cause) => return Err(fail(stopped, cause, false)),
    };
    let stopped_observation_record_sha256 = stopped.observation_record_sha256().to_string();
    let payload = match encode_durable_journal_event_with_evidence_v8(
        &JournalEventV8::EffectIntent {
            effect: JournalEffectV8::RunnerRestore,
            effect_manifest_sha256: effect_manifest_sha256.clone(),
        },
        &effect_manifest_bytes,
    ) {
        Ok(payload) => payload,
        Err(cause) => return Err(fail(stopped, cause, false)),
    };
    let global_sequence = match before.journal().record_count().checked_add(1) {
        Some(sequence) => sequence,
        None => {
            return Err(fail(
                stopped,
                invalid("runner RESTORE intent journal sequence overflows"),
                false,
            ));
        }
    };
    let record = match DurableJournalRecordV8::new(
        active_attempt.attempt_identity_sha256().to_string(),
        before.journal().last_boot_epoch(),
        active_attempt.boot_id().to_string(),
        global_sequence,
        before.journal().tip_sha256().to_string(),
        payload,
    ) {
        Ok(record) => record,
        Err(cause) => return Err(fail(stopped, cause, false)),
    };
    let intent_record_sha256 = match record.record_sha256() {
        Ok(digest) => digest,
        Err(cause) => return Err(fail(stopped, cause, false)),
    };
    let mut journal_publication_issued_or_uncertain = false;
    let publication = match super::journal_publish::append_journal_record_durably_observed_v8(
        state_root,
        state_root_lock,
        active_attempt,
        &record,
        publication_nonce,
        |_| journal_publication_issued_or_uncertain = true,
    ) {
        Ok(publication) => publication,
        Err(cause) => {
            return Err(fail(
                stopped,
                cause,
                journal_publication_issued_or_uncertain,
            ));
        }
    };
    let after = match scan_durable_journal_events_v8(
        state_root,
        state_root_lock,
        active_attempt.attempt_identity_sha256(),
    ) {
        Ok(after) => after,
        Err(cause) => return Err(fail(stopped, cause, true)),
    };
    let Some(pending) = after.pending_effect() else {
        return Err(fail(
            stopped,
            invalid("runner RESTORE intent disappeared during durable replay"),
            true,
        ));
    };
    if pending.effect() != JournalEffectV8::RunnerRestore
        || pending.effect_manifest_sha256() != effect_manifest_sha256
        || pending.effect_manifest_bytes() != effect_manifest_bytes
        || pending.intent_record_sha256() != intent_record_sha256
        || after.journal().tip_sha256() != intent_record_sha256
        || after.runner_stop_observation_record_sha256()
            != Some(stopped_observation_record_sha256.as_str())
    {
        return Err(fail(
            stopped,
            invalid("runner RESTORE durable replay does not bind the exact appended typed intent"),
            true,
        ));
    }
    let intent = DurableRunnerRestoreIntentV8 {
        attempt_identity_sha256: active_attempt.attempt_identity_sha256().to_string(),
        active_attempt_record_sha256: active_attempt.record_sha256().to_string(),
        boot_epoch: record.boot_epoch(),
        boot_id: record.boot_id().to_string(),
        global_sequence: record.global_sequence(),
        stopped_observation_record_sha256: stopped_observation_record_sha256.clone(),
        effect_manifest_sha256,
        effect_manifest_bytes,
        scope_binding_bytes,
        intent_record_sha256,
        state_root_identity: Some(state_root.identity()),
    };
    let stopped = match RestoreAuthorizedStoppedRunnerScopeV8::bind_durable_intent(
        stopped,
        &stopped_observation_record_sha256,
        intent,
    ) {
        Ok(stopped) => stopped,
        Err((stopped, _intent)) => {
            return Err(fail(
                stopped,
                invalid("runner RESTORE typestate lost its STOP observation binding"),
                true,
            ));
        }
    };
    Ok((stopped, publication))
}

#[must_use = "runner RESTORE observation failure retains post-CONT quarantine evidence"]
#[derive(Debug)]
pub(crate) struct DurableRunnerRestoreObservationFailureV8 {
    execution: RunnerScopeContinueExecutionV8,
    cause: NativeErrorV8,
    publication_issued_or_uncertain: bool,
}

impl DurableRunnerRestoreObservationFailureV8 {
    pub(crate) fn cause(&self) -> &NativeErrorV8 {
        &self.cause
    }

    pub(crate) fn publication_issued_or_uncertain(&self) -> bool {
        self.publication_issued_or_uncertain
    }

    pub(crate) fn into_execution(self) -> RunnerScopeContinueExecutionV8 {
        self.execution
    }
}

#[must_use = "durable runner RESTORE closure is the only successful restored outcome"]
#[derive(Debug)]
pub(crate) struct DurableRunnerRestoreObservationV8 {
    execution: RunnerScopeContinueExecutionV8,
    observation_record_sha256: String,
    publication: PublishedDurableJournalRecordV8,
}

impl DurableRunnerRestoreObservationV8 {
    pub(crate) fn execution(&self) -> &RunnerScopeContinueExecutionV8 {
        &self.execution
    }

    pub(crate) fn observation_record_sha256(&self) -> &str {
        &self.observation_record_sha256
    }

    pub(crate) fn publication(&self) -> &PublishedDurableJournalRecordV8 {
        &self.publication
    }
}

pub(crate) fn append_runner_restore_observation_durably_v8(
    state_root: &DirectoryAnchorV8,
    state_root_lock: &mut StateRootLockV8,
    active_attempt: &FreshActiveAttemptPublicationV8,
    execution: RunnerScopeContinueExecutionV8,
    publication_nonce: &str,
) -> Result<DurableRunnerRestoreObservationV8, DurableRunnerRestoreObservationFailureV8> {
    let fail = |execution, cause, publication_issued_or_uncertain| {
        DurableRunnerRestoreObservationFailureV8 {
            execution,
            cause,
            publication_issued_or_uncertain,
        }
    };
    let observation = match execution.capture_runner_restore_observation_v8() {
        Ok(observation) => observation,
        Err(cause) => return Err(fail(execution, cause.into(), false)),
    };
    let observation_bytes = match observation.canonical_bytes() {
        Ok(bytes) => bytes,
        Err(cause) => return Err(fail(execution, cause, false)),
    };
    let observation_sha256 = match observation.sha256() {
        Ok(digest) => digest,
        Err(cause) => return Err(fail(execution, cause, false)),
    };
    let Some(intent) = execution.runner_restore_intent() else {
        return Err(fail(
            execution,
            invalid("post-CONT execution lacks its durable RESTORE obligation"),
            false,
        ));
    };
    if !intent.matches_origin(state_root.identity(), active_attempt) {
        return Err(fail(
            execution,
            invalid("post-CONT durable RESTORE obligation has a different origin"),
            false,
        ));
    }
    let intent_record_sha256 = intent.intent_record_sha256().to_string();
    let attempt_identity_sha256 = intent.attempt_identity_sha256().to_string();
    let boot_epoch = intent.boot_epoch();
    let boot_id = intent.boot_id().to_string();
    let next_sequence = match intent.global_sequence().checked_add(1) {
        Some(sequence) => sequence,
        None => {
            return Err(fail(
                execution,
                invalid("runner RESTORE observation journal sequence overflows"),
                false,
            ));
        }
    };
    let payload = match encode_durable_journal_event_with_evidence_v8(
        &JournalEventV8::EffectObserved {
            effect: JournalEffectV8::RunnerRestore,
            intent_record_sha256: intent_record_sha256.clone(),
            observation_sha256,
        },
        &observation_bytes,
    ) {
        Ok(payload) => payload,
        Err(cause) => return Err(fail(execution, cause, false)),
    };
    let record = match DurableJournalRecordV8::new(
        attempt_identity_sha256.clone(),
        boot_epoch,
        boot_id,
        next_sequence,
        intent_record_sha256.clone(),
        payload,
    ) {
        Ok(record) => record,
        Err(cause) => return Err(fail(execution, cause, false)),
    };
    let observation_record_sha256 = match record.record_sha256() {
        Ok(digest) => digest,
        Err(cause) => return Err(fail(execution, cause, false)),
    };
    let mut publication_issued_or_uncertain = false;
    let publication = match super::journal_publish::append_journal_record_durably_observed_v8(
        state_root,
        state_root_lock,
        active_attempt,
        &record,
        publication_nonce,
        |_| publication_issued_or_uncertain = true,
    ) {
        Ok(publication) => publication,
        Err(cause) => return Err(fail(execution, cause, publication_issued_or_uncertain)),
    };
    let replay =
        match scan_durable_journal_events_v8(state_root, state_root_lock, &attempt_identity_sha256)
        {
            Ok(replay) => replay,
            Err(cause) => return Err(fail(execution, cause, true)),
        };
    if replay.unfinished_intent_detected()
        || replay.journal().tip_sha256() != observation_record_sha256
        || replay.runner_stop_observation_record_sha256()
            != Some(intent.stopped_observation_record_sha256())
    {
        return Err(fail(
            execution,
            invalid("runner RESTORE observation replay did not close the exact intent"),
            true,
        ));
    }
    Ok(DurableRunnerRestoreObservationV8 {
        execution,
        observation_record_sha256,
        publication,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(character: char) -> String {
        character.to_string().repeat(64)
    }

    #[test]
    fn event_payload_is_exact_and_model_validated() {
        let event = JournalEventV8::EffectIntent {
            effect: JournalEffectV8::RunnerStop,
            effect_manifest_sha256: digest('1'),
        };
        let bytes = encode_durable_journal_event_v8(&event).unwrap();
        assert_eq!(decode_durable_journal_event_v8(&bytes).unwrap(), event);

        let mut noncanonical = bytes.clone();
        noncanonical.insert(DURABLE_JOURNAL_EVENT_SCHEMA_V8.len(), b' ');
        assert!(decode_durable_journal_event_v8(&noncanonical).is_err());

        let invalid_event = JournalEventV8::EffectIntent {
            effect: JournalEffectV8::RunnerStop,
            effect_manifest_sha256: "not-a-digest".to_string(),
        };
        assert!(encode_durable_journal_event_v8(&invalid_event).is_err());
    }

    #[test]
    fn prior_boot_tip_without_a_new_record_never_allows_continuation() {
        assert!(ordinary_continuation_allowed_v8(
            false, false, false, false, false
        ));
        assert!(
            !ordinary_continuation_allowed_v8(false, false, true, false, false),
            "a prior-boot persisted tip with no BOOT_RECOVERY record must force recovery"
        );
    }
}

#[cfg(all(test, target_os = "linux"))]
mod linux_tests {
    use std::ffi::OsStr;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    use super::*;
    use crate::ATTEMPTS_DIRECTORY_V8;
    use crate::ActiveAttemptPublicationOutcomeV8;
    use crate::ActiveAttemptRequestV8;
    use crate::RecoveryScanFactsV8;
    use crate::acquire_state_root_lock_v8;
    use crate::bind_durable_event_replay_v8;
    use crate::durable::FrozenTransitionIntentContextV8;
    use crate::publish_active_attempt_durably_v8;

    const ZERO_SHA256: &str = "0000000000000000000000000000000000000000000000000000000000000000";

    fn digest(character: char) -> String {
        character.to_string().repeat(64)
    }

    fn temporary_state_root(attempt: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "hepta-linux-v8-effect-obligation-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir(&root).unwrap();
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
        fs::create_dir_all(
            root.join(ATTEMPTS_DIRECTORY_V8)
                .join(attempt)
                .join("journal"),
        )
        .unwrap();
        fs::set_permissions(
            root.join(ATTEMPTS_DIRECTORY_V8).join(attempt),
            fs::Permissions::from_mode(0o700),
        )
        .unwrap();
        fs::set_permissions(
            root.join(ATTEMPTS_DIRECTORY_V8)
                .join(attempt)
                .join("journal"),
            fs::Permissions::from_mode(0o700),
        )
        .unwrap();
        root
    }

    #[test]
    fn candidate_execution_origin_pending_and_phase_are_exact_and_no_authority() {
        let attempt = digest('1');
        let root = temporary_state_root(&attempt);
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let mut lock = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).unwrap();
        let machine = crate::observe_machine_id_v8().unwrap();
        let current_boot_id = crate::observe_boot_id_v8().unwrap().to_string();
        let active = match publish_active_attempt_durably_v8(
            &anchor,
            &mut lock,
            &ActiveAttemptRequestV8::new(
                attempt.clone(),
                7,
                current_boot_id,
                machine.machine_id_sha256().to_string(),
                digest('9'),
            )
            .unwrap(),
            &digest('7'),
        )
        .unwrap()
        {
            ActiveAttemptPublicationOutcomeV8::Fresh(active) => active,
            ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(_) => panic!(),
        };
        let opened = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            active.boot_id().to_string(),
            1,
            ZERO_SHA256.to_string(),
            encode_durable_journal_event_v8(&JournalEventV8::AttemptOpened {
                authority_manifest_sha256: digest('a'),
            })
            .unwrap(),
        )
        .unwrap();
        append_journal_record_durably_v8(&anchor, &mut lock, &active, &opened, &digest('2'))
            .unwrap();

        let stop_manifest = synthetic_runner_stop_evidence(
            RunnerStopEvidencePhaseV8::PreEffect,
            active.boot_id(),
            None,
            None,
        );
        let stop_manifest_bytes = stop_manifest.canonical_bytes().unwrap();
        let stop_manifest_sha256 = stop_manifest.sha256().unwrap();
        let stop_intent = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            active.boot_id().to_string(),
            2,
            opened.record_sha256().unwrap(),
            encode_durable_journal_event_with_evidence_v8(
                &JournalEventV8::EffectIntent {
                    effect: JournalEffectV8::RunnerStop,
                    effect_manifest_sha256: stop_manifest_sha256.clone(),
                },
                &stop_manifest_bytes,
            )
            .unwrap(),
        )
        .unwrap();
        append_journal_record_durably_v8(&anchor, &mut lock, &active, &stop_intent, &digest('3'))
            .unwrap();
        let stop_intent_sha256 = stop_intent.record_sha256().unwrap();
        let stop_observation = synthetic_runner_stop_evidence(
            RunnerStopEvidencePhaseV8::StoppedObservation,
            active.boot_id(),
            Some(stop_manifest_sha256),
            Some(stop_intent_sha256.clone()),
        );
        let stop_observation_bytes = stop_observation.canonical_bytes().unwrap();
        let stopped = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            active.boot_id().to_string(),
            3,
            stop_intent_sha256.clone(),
            encode_durable_journal_event_with_evidence_v8(
                &JournalEventV8::EffectObserved {
                    effect: JournalEffectV8::RunnerStop,
                    intent_record_sha256: stop_intent_sha256,
                    observation_sha256: stop_observation.sha256().unwrap(),
                },
                &stop_observation_bytes,
            )
            .unwrap(),
        )
        .unwrap();
        append_journal_record_durably_v8(&anchor, &mut lock, &active, &stopped, &digest('4'))
            .unwrap();
        let stopped_sha256 = stopped.record_sha256().unwrap();

        let candidate_intent =
            CandidateExecutionEffectEvidenceV8::intent(FrozenTransitionIntentContextV8 {
                machine_id_sha256: machine.machine_id_sha256().to_string(),
                machine_id_source_identity: machine.source_identity(),
                state_root_binding_sha256: digest('b'),
                state_root_identity: anchor.identity(),
                state_root_mount_id: anchor.trusted_node_metadata().unwrap().mount_id(),
                state_root_lock_identity: lock.identity(),
                attempt_identity_sha256: attempt.clone(),
                active_attempt_record_sha256: active.record_sha256().to_string(),
                active_attempt_file_identity: active.publication().identity(),
                barrier_generation: active.barrier_generation(),
                restore_plan_sha256: active.restore_plan_sha256().to_string(),
                boot_id: active.boot_id().to_string(),
                boot_epoch: 1,
                global_sequence: 4,
                journal_tip_sha256: stopped_sha256.clone(),
                predecessor_record_sha256: stopped_sha256.clone(),
                candidate_execution_request_sha256: digest('c'),
            })
            .unwrap();
        let candidate_intent_bytes = candidate_intent.canonical_bytes().unwrap();
        let candidate_intent_manifest_sha256 = candidate_intent.sha256().unwrap();
        let candidate_intent_record = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            active.boot_id().to_string(),
            4,
            stopped_sha256,
            encode_durable_journal_event_with_evidence_v8(
                &JournalEventV8::EffectIntent {
                    effect: JournalEffectV8::CandidateExecution,
                    effect_manifest_sha256: candidate_intent_manifest_sha256.clone(),
                },
                &candidate_intent_bytes,
            )
            .unwrap(),
        )
        .unwrap();
        append_journal_record_durably_v8(
            &anchor,
            &mut lock,
            &active,
            &candidate_intent_record,
            &digest('5'),
        )
        .unwrap();
        let candidate_intent_record_sha256 = candidate_intent_record.record_sha256().unwrap();
        let pending = scan_durable_journal_events_v8(&anchor, &lock, &attempt).unwrap();
        assert_eq!(
            pending.pending_effect().unwrap().effect(),
            JournalEffectV8::CandidateExecution
        );
        assert_eq!(
            pending.qualification_phase(),
            Some(QualificationJournalPhaseV8::AwaitCandidateExecutionObservation)
        );
        assert!(!pending.ordinary_continuation_allowed());

        let state_root_binding_sha256 = digest('b');
        let descriptor_origin = DescriptorReplayOriginV8 {
            machine_id_sha256: machine.machine_id_sha256(),
            machine_id_source_identity: machine.source_identity(),
            state_root_binding_sha256: &state_root_binding_sha256,
            state_root_identity: anchor.identity(),
            state_root_mount_id: anchor.trusted_node_metadata().unwrap().mount_id(),
            state_root_lock_identity: lock.identity(),
            attempt_identity_sha256: &attempt,
            active_attempt_record_sha256: active.record_sha256(),
            active_attempt_file_identity: active.publication().identity(),
            barrier_generation: active.barrier_generation(),
            restore_plan_sha256: active.restore_plan_sha256(),
            boot_id: active.boot_id(),
        };
        let descriptor_pending = scan_durable_journal_events_descriptor_bound_v8(
            &anchor,
            &lock,
            &attempt,
            &descriptor_origin,
        )
        .unwrap();
        assert_eq!(
            descriptor_pending.pending_effect().unwrap().effect(),
            JournalEffectV8::CandidateExecution
        );
        descriptor_pending
            .revalidate_descriptor_bound_v8(&anchor, &lock, &descriptor_origin)
            .unwrap();
        let wrong_binding = digest('f');
        let mut wrong_origin = descriptor_origin;
        wrong_origin.state_root_binding_sha256 = &wrong_binding;
        assert!(
            scan_durable_journal_events_descriptor_bound_v8(
                &anchor,
                &lock,
                &attempt,
                &wrong_origin,
            )
            .is_err()
        );

        let candidate_result_sha256 = digest('d');
        let candidate_observation = CandidateExecutionEffectEvidenceV8::observation(
            &candidate_intent,
            candidate_intent_manifest_sha256,
            candidate_intent_record_sha256.clone(),
            digest('e'),
            100,
            110,
            candidate_result_sha256.clone(),
            120,
            130,
        )
        .unwrap();
        let candidate_observation_bytes = candidate_observation.canonical_bytes().unwrap();
        let candidate_observation_record = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            active.boot_id().to_string(),
            5,
            candidate_intent_record_sha256.clone(),
            encode_durable_journal_event_with_evidence_v8(
                &JournalEventV8::EffectObserved {
                    effect: JournalEffectV8::CandidateExecution,
                    intent_record_sha256: candidate_intent_record_sha256,
                    observation_sha256: candidate_observation.sha256().unwrap(),
                },
                &candidate_observation_bytes,
            )
            .unwrap(),
        )
        .unwrap();
        append_journal_record_durably_v8(
            &anchor,
            &mut lock,
            &active,
            &candidate_observation_record,
            &digest('6'),
        )
        .unwrap();
        let observed = scan_durable_journal_events_v8(&anchor, &lock, &attempt).unwrap();
        assert!(observed.pending_effect().is_none());
        assert_eq!(
            observed.qualification_phase(),
            Some(QualificationJournalPhaseV8::AwaitCandidateCompleted)
        );
        assert!(!observed.ordinary_continuation_allowed());

        let completed = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            active.boot_id().to_string(),
            6,
            candidate_observation_record.record_sha256().unwrap(),
            encode_durable_journal_event_v8(&JournalEventV8::CandidateCompleted {
                candidate_result_sha256,
            })
            .unwrap(),
        )
        .unwrap();
        append_journal_record_durably_v8(&anchor, &mut lock, &active, &completed, &digest('8'))
            .unwrap();
        let completed_scan = scan_durable_journal_events_v8(&anchor, &lock, &attempt).unwrap();
        assert!(completed_scan.pending_effect().is_none());
        assert_eq!(
            completed_scan.qualification_phase(),
            Some(QualificationJournalPhaseV8::AwaitCandidateRelayIntent)
        );
        assert!(!completed_scan.ordinary_continuation_allowed());

        let unsupported_relay = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            active.boot_id().to_string(),
            7,
            completed.record_sha256().unwrap(),
            encode_durable_journal_event_with_evidence_v8(
                &JournalEventV8::EffectIntent {
                    effect: JournalEffectV8::CandidateRelay,
                    effect_manifest_sha256: digest('f'),
                },
                b"{}",
            )
            .unwrap(),
        )
        .unwrap();
        append_journal_record_durably_v8(
            &anchor,
            &mut lock,
            &active,
            &unsupported_relay,
            &digest('a'),
        )
        .unwrap();
        assert!(scan_durable_journal_events_v8(&anchor, &lock, &attempt).is_err());

        drop(lock);
        fs::remove_dir_all(root).unwrap();
    }

    fn synthetic_runner_stop_evidence(
        phase: RunnerStopEvidencePhaseV8,
        boot_id: &str,
        intent_manifest_sha256: Option<String>,
        intent_record_sha256: Option<String>,
    ) -> RunnerStopEffectEvidenceV8 {
        let stopped = phase == RunnerStopEvidencePhaseV8::StoppedObservation;
        RunnerStopEffectEvidenceV8 {
            schema: "hepta-linux-v8-runner-stop-evidence-v1".to_string(),
            phase,
            boot_id: boot_id.to_string(),
            unit_name: "runner22-23.service".to_string(),
            control_group: "/system.slice/runner22-23.service".to_string(),
            main_pid: 100,
            cgroup_device: 11,
            cgroup_inode: 12,
            cgroup_mount_id: 13,
            proc_mount_id: 14,
            pid_namespace_device: 15,
            pid_namespace_inode: 16,
            cgroup_namespace_device: 17,
            cgroup_namespace_inode: 18,
            mount_namespace_device: 19,
            mount_namespace_inode: 20,
            process_group_id: 100,
            session_id: 100,
            observation_started_boottime_ns: if stopped { 30 } else { 10 },
            observation_completed_boottime_ns: if stopped { 40 } else { 20 },
            observation_started_monotonic_ns: if stopped { 30 } else { 10 },
            observation_completed_monotonic_ns: if stopped { 40 } else { 20 },
            intent_manifest_sha256,
            intent_record_sha256,
            processes: (100..106)
                .map(|pid| RunnerStopProcessEvidenceV8 {
                    pid,
                    pidfd_bound: true,
                    start_ticks: 1_000 + u64::from(pid),
                    parent_pid: 1,
                    process_group_id: 100,
                    session_id: 100,
                    state: if stopped { b'T' } else { b'S' },
                    executable_device: 21,
                    executable_inode: 10_000 + u64::from(pid),
                    executable_size: 4_096,
                    executable_sha256: digest('a'),
                    uid: 1_000,
                    gid: 1_000,
                    argv_sha256: digest('b'),
                    cwd_device: 22,
                    cwd_inode: 23,
                })
                .collect(),
        }
    }

    fn synthetic_runner_restore_evidence(
        phase: RunnerRestoreEvidencePhaseV8,
        boot_id: &str,
        stopped_observation_record_sha256: String,
        intent_manifest_sha256: Option<String>,
        intent_record_sha256: Option<String>,
    ) -> RunnerRestoreEffectEvidenceV8 {
        let running = phase == RunnerRestoreEvidencePhaseV8::RunningObservation;
        RunnerRestoreEffectEvidenceV8 {
            schema: "hepta-linux-v8-runner-restore-evidence-v1".to_string(),
            phase,
            boot_id: boot_id.to_string(),
            unit_name: "runner22-23.service".to_string(),
            control_group: "/system.slice/runner22-23.service".to_string(),
            main_pid: 100,
            cgroup_device: 11,
            cgroup_inode: 12,
            cgroup_mount_id: 13,
            proc_mount_id: 14,
            pid_namespace_device: 15,
            pid_namespace_inode: 16,
            cgroup_namespace_device: 17,
            cgroup_namespace_inode: 18,
            mount_namespace_device: 19,
            mount_namespace_inode: 20,
            process_group_id: 100,
            session_id: 100,
            stopped_observation_record_sha256,
            observation_started_boottime_ns: if running { 70 } else { 50 },
            observation_completed_boottime_ns: if running { 80 } else { 60 },
            observation_started_monotonic_ns: if running { 70 } else { 50 },
            observation_completed_monotonic_ns: if running { 80 } else { 60 },
            intent_manifest_sha256,
            intent_record_sha256,
            processes: (100..106)
                .map(|pid| RunnerRestoreProcessEvidenceV8 {
                    pid,
                    pidfd_bound: true,
                    start_ticks: 1_000 + u64::from(pid),
                    parent_pid: 1,
                    process_group_id: 100,
                    session_id: 100,
                    state: if running { b'S' } else { b'T' },
                    executable_device: 21,
                    executable_inode: 10_000 + u64::from(pid),
                    executable_size: 4_096,
                    executable_sha256: digest('a'),
                    uid: 1_000,
                    gid: 1_000,
                    argv_sha256: digest('b'),
                    cwd_device: 22,
                    cwd_inode: 23,
                })
                .collect(),
        }
    }

    #[test]
    fn durable_runner_stop_intent_replays_as_unfinished_until_exact_observation() {
        let attempt = digest('1');
        let root = temporary_state_root(&attempt);
        let anchor = DirectoryAnchorV8::open(&root).unwrap();
        let mut lock = acquire_state_root_lock_v8(&anchor, OsStr::new("state.lock")).unwrap();
        let current_boot_id = crate::observe_boot_id_v8().unwrap().to_string();
        let active = match publish_active_attempt_durably_v8(
            &anchor,
            &mut lock,
            &ActiveAttemptRequestV8::new(
                attempt.clone(),
                7,
                current_boot_id,
                digest('8'),
                digest('9'),
            )
            .unwrap(),
            &digest('7'),
        )
        .unwrap()
        {
            ActiveAttemptPublicationOutcomeV8::Fresh(active) => active,
            ActiveAttemptPublicationOutcomeV8::ExistingRequiresRecovery(_) => panic!(),
        };
        let opened = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            active.boot_id().to_string(),
            1,
            ZERO_SHA256.to_string(),
            encode_durable_journal_event_v8(&JournalEventV8::AttemptOpened {
                authority_manifest_sha256: digest('a'),
            })
            .unwrap(),
        )
        .unwrap();
        append_journal_record_durably_v8(&anchor, &mut lock, &active, &opened, &digest('2'))
            .unwrap();

        let manifest = synthetic_runner_stop_evidence(
            RunnerStopEvidencePhaseV8::PreEffect,
            active.boot_id(),
            None,
            None,
        );
        let manifest_bytes = manifest.canonical_bytes().unwrap();
        let manifest_sha256 = manifest.sha256().unwrap();
        let intent_record = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            active.boot_id().to_string(),
            2,
            opened.record_sha256().unwrap(),
            encode_durable_journal_event_with_evidence_v8(
                &JournalEventV8::EffectIntent {
                    effect: JournalEffectV8::RunnerStop,
                    effect_manifest_sha256: manifest_sha256.clone(),
                },
                &manifest_bytes,
            )
            .unwrap(),
        )
        .unwrap();
        append_journal_record_durably_v8(&anchor, &mut lock, &active, &intent_record, &digest('3'))
            .unwrap();
        let intent_record_sha256 = intent_record.record_sha256().unwrap();
        let pending = scan_durable_journal_events_v8(&anchor, &lock, &attempt).unwrap();
        assert!(pending.unfinished_intent_detected());
        assert!(!pending.ordinary_continuation_allowed());
        let root_identity = anchor.identity();
        let facts = bind_durable_event_replay_v8(
            RecoveryScanFactsV8 {
                attempt_identity_sha256: attempt.clone(),
                current_boot_id: active.boot_id().to_string(),
                current_journal_tip_sha256: digest('0'),
                daemon_restart_detected: false,
                existing_attempt_detected: false,
                prior_boot_detected: false,
                incoming_residue_detected: false,
                unfinished_intent_detected: false,
                journal_corruption_detected: false,
                runner_restore_required: false,
                runner_closure_matches_restore_plan: true,
                runner_snapshot_sha256: digest('d'),
                state_root_device: root_identity.device(),
                state_root_inode: root_identity.inode(),
                state_root_mode: root_identity.mode(),
                state_root_owner_gid: root_identity.owner_gid(),
                state_root_owner_uid: root_identity.owner_uid(),
                restore_plan_sha256: digest('e'),
            },
            &pending,
        )
        .unwrap();
        assert!(facts.unfinished_intent_detected);
        assert_eq!(
            facts.current_journal_tip_sha256,
            pending.journal().tip_sha256()
        );

        let observation = synthetic_runner_stop_evidence(
            RunnerStopEvidencePhaseV8::StoppedObservation,
            active.boot_id(),
            Some(manifest_sha256),
            Some(intent_record_sha256.clone()),
        );
        assert!(observation.closes_exact_manifest(&manifest));
        let mut wrong_roster = observation.clone();
        wrong_roster.processes[5].executable_inode += 1;
        assert!(!wrong_roster.closes_exact_manifest(&manifest));
        let mut time_overlap = observation.clone();
        time_overlap.observation_started_boottime_ns = 19;
        assert!(!time_overlap.closes_exact_manifest(&manifest));
        let mut idle_pre_effect = manifest.clone();
        idle_pre_effect.processes[0].state = b'I';
        assert!(idle_pre_effect.canonical_bytes().is_err());
        let observation_bytes = observation.canonical_bytes().unwrap();
        let observation_sha256 = observation.sha256().unwrap();
        let observed = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            active.boot_id().to_string(),
            3,
            intent_record_sha256.clone(),
            encode_durable_journal_event_with_evidence_v8(
                &JournalEventV8::EffectObserved {
                    effect: JournalEffectV8::RunnerStop,
                    intent_record_sha256,
                    observation_sha256,
                },
                &observation_bytes,
            )
            .unwrap(),
        )
        .unwrap();
        append_journal_record_durably_v8(&anchor, &mut lock, &active, &observed, &digest('4'))
            .unwrap();
        let stopped_observation_record_sha256 = observed.record_sha256().unwrap();
        let closed = scan_durable_journal_events_v8(&anchor, &lock, &attempt).unwrap();
        assert!(!closed.unfinished_intent_detected());
        assert!(closed.ordinary_continuation_allowed());

        let restore_manifest = synthetic_runner_restore_evidence(
            RunnerRestoreEvidencePhaseV8::StoppedPreEffect,
            active.boot_id(),
            stopped_observation_record_sha256.clone(),
            None,
            None,
        );
        let restore_manifest_bytes = restore_manifest.canonical_bytes().unwrap();
        let restore_manifest_sha256 = restore_manifest.sha256().unwrap();
        let restore_intent = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            active.boot_id().to_string(),
            4,
            stopped_observation_record_sha256.clone(),
            encode_durable_journal_event_with_evidence_v8(
                &JournalEventV8::EffectIntent {
                    effect: JournalEffectV8::RunnerRestore,
                    effect_manifest_sha256: restore_manifest_sha256.clone(),
                },
                &restore_manifest_bytes,
            )
            .unwrap(),
        )
        .unwrap();
        append_journal_record_durably_v8(
            &anchor,
            &mut lock,
            &active,
            &restore_intent,
            &digest('5'),
        )
        .unwrap();
        let restore_intent_record_sha256 = restore_intent.record_sha256().unwrap();
        let pending_restore = scan_durable_journal_events_v8(&anchor, &lock, &attempt).unwrap();
        assert!(pending_restore.unfinished_intent_detected());
        assert!(!pending_restore.ordinary_continuation_allowed());
        assert_eq!(
            pending_restore.pending_effect().unwrap().effect(),
            JournalEffectV8::RunnerRestore
        );
        assert_eq!(
            pending_restore.runner_stop_observation_record_sha256(),
            Some(stopped_observation_record_sha256.as_str())
        );

        let restore_observation = synthetic_runner_restore_evidence(
            RunnerRestoreEvidencePhaseV8::RunningObservation,
            active.boot_id(),
            stopped_observation_record_sha256,
            Some(restore_manifest_sha256),
            Some(restore_intent_record_sha256.clone()),
        );
        assert!(restore_observation.closes_exact_manifest(&restore_manifest));
        let mut wrong_roster = restore_observation.clone();
        wrong_roster.processes[5].executable_inode += 1;
        assert!(!wrong_roster.closes_exact_manifest(&restore_manifest));
        let mut time_overlap = restore_observation.clone();
        time_overlap.observation_started_boottime_ns = 59;
        assert!(!time_overlap.closes_exact_manifest(&restore_manifest));
        let mut still_stopped = restore_observation.clone();
        still_stopped.processes[0].state = b'T';
        assert!(still_stopped.canonical_bytes().is_err());
        let mut wrong_stop_record = restore_observation.clone();
        wrong_stop_record.stopped_observation_record_sha256 = digest('f');
        assert!(!wrong_stop_record.closes_exact_manifest(&restore_manifest));

        let restore_observation_bytes = restore_observation.canonical_bytes().unwrap();
        let restore_observation_sha256 = restore_observation.sha256().unwrap();
        let restored = DurableJournalRecordV8::new(
            attempt.clone(),
            1,
            active.boot_id().to_string(),
            5,
            restore_intent_record_sha256.clone(),
            encode_durable_journal_event_with_evidence_v8(
                &JournalEventV8::EffectObserved {
                    effect: JournalEffectV8::RunnerRestore,
                    intent_record_sha256: restore_intent_record_sha256,
                    observation_sha256: restore_observation_sha256,
                },
                &restore_observation_bytes,
            )
            .unwrap(),
        )
        .unwrap();
        append_journal_record_durably_v8(&anchor, &mut lock, &active, &restored, &digest('6'))
            .unwrap();
        let restored_scan = scan_durable_journal_events_v8(&anchor, &lock, &attempt).unwrap();
        assert!(!restored_scan.unfinished_intent_detected());
        assert!(restored_scan.ordinary_continuation_allowed());
        assert_eq!(
            restored_scan.journal().tip_sha256(),
            restored.record_sha256().unwrap()
        );
        drop(lock);
        fs::remove_dir_all(root).unwrap();
    }
}
