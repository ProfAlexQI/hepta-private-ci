use std::fs;
use std::path::PathBuf;

use hepta_core::HeptaError;
use serde::Deserialize;
use serde::Serialize;

use crate::current_unix_ms;
use crate::delivery_queue::ReadbackEvidenceLedger;

pub const DEFAULT_SCHEDULER_STORE_PATH: &str = ".hepta/scheduler-store-v0.json";
pub const DEFAULT_SCHEDULER_STORE_ID: &str = "hepta-native-scheduler-store";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchedulerJobStatus {
    Enabled,
    Disabled,
    Due,
    Running,
    Completed,
    Failed,
}

impl SchedulerJobStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::Disabled => "disabled",
            Self::Due => "due",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchedulerScheduleKind {
    At,
    Every,
    Cron,
}

impl SchedulerScheduleKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::At => "at",
            Self::Every => "every",
            Self::Cron => "cron",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerStoreFile {
    pub version: u32,
    pub store_id: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default)]
    pub jobs: Vec<SchedulerJobRecord>,
    #[serde(default)]
    pub runs: Vec<SchedulerRunRecord>,
    #[serde(default)]
    pub wake_handoffs: Vec<SchedulerWakeHandoffRecord>,
    #[serde(default)]
    pub queued_wakes: Vec<SchedulerQueuedWakeRecord>,
    #[serde(default)]
    pub events: Vec<SchedulerStoreEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerJobRecord {
    pub job_id: String,
    pub name: String,
    pub schedule_kind: SchedulerScheduleKind,
    pub schedule_expr: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    pub payload_kind: String,
    pub payload_preview: String,
    pub idempotency_key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explicit_timeout_seconds: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_bound_heartbeat_route: Option<String>,
    pub status: SchedulerJobStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_due_unix_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_run_unix_ms: Option<u64>,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerRunRecord {
    pub run_id: String,
    pub job_id: String,
    pub status: SchedulerJobStatus,
    pub started_at_unix_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<u64>,
    #[serde(default)]
    pub long_run_active: bool,
    #[serde(default)]
    pub active_in_task_registry: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at_unix_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readback_evidence_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerWakeHandoffRecord {
    pub handoff_id: String,
    pub job_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    pub session_target: String,
    pub wake_mode: String,
    pub payload_preview: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub wake_enqueued_by_gate: bool,
    pub session_mutated_by_gate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerQueuedWakeRecord {
    pub wake_id: String,
    pub handoff_id: String,
    pub job_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    pub session_target: String,
    pub wake_mode: String,
    pub payload_preview: String,
    pub status: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub session_mutated_by_adapter: bool,
    pub gateway_rpc_performed_by_adapter: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchedulerStoreEvent {
    pub event_id: String,
    pub event_type: String,
    pub job_id: String,
    pub occurred_at_unix_ms: u64,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchedulerStoreReport {
    pub store_path: String,
    pub store: SchedulerStoreFile,
    pub enabled_count: usize,
    pub disabled_count: usize,
    pub due_count: usize,
    pub running_count: usize,
    pub completed_count: usize,
    pub failed_count: usize,
    pub queued_wake_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchedulerStoreScheduleReport {
    pub store_path: String,
    pub job: SchedulerJobRecord,
    pub duplicate_idempotency_key: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchedulerStoreTransitionReport {
    pub store_path: String,
    pub job_id: String,
    pub status: SchedulerJobStatus,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchedulerStoreRunReport {
    pub store_path: String,
    pub run: SchedulerRunRecord,
    pub job_status: SchedulerJobStatus,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchedulerJobReadbackReport {
    pub store_path: String,
    pub job: SchedulerJobRecord,
    pub active_run_count: usize,
    pub explicit_timeout_intent_preserved: bool,
    pub session_bound_heartbeat_route_present: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchedulerRunStartInput {
    pub long_run_active: bool,
    pub active_in_task_registry: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchedulerWakeHandoffInput {
    pub run_id: Option<String>,
    pub session_target: String,
    pub wake_mode: String,
    pub payload_preview: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchedulerWakeHandoffReport {
    pub store_path: String,
    pub evidence_ledger_path: String,
    pub handoff: SchedulerWakeHandoffRecord,
    pub duplicate_idempotency_key: bool,
    pub scheduler_store_mutated_by_gate: bool,
    pub wake_enqueued_by_gate: bool,
    pub session_mutated_by_gate: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchedulerWakeMaterializationInput {
    pub handoff_id: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchedulerWakeMaterializationReport {
    pub store_path: String,
    pub evidence_ledger_path: String,
    pub wake: SchedulerQueuedWakeRecord,
    pub duplicate_idempotency_key: bool,
    pub scheduler_store_mutated_by_adapter: bool,
    pub wake_enqueued_by_adapter: bool,
    pub session_mutated_by_adapter: bool,
    pub gateway_rpc_performed_by_adapter: bool,
    pub persisted: bool,
}

pub struct SchedulerStore {
    path: PathBuf,
}

impl SchedulerStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_in_current_dir() -> Result<Self, HeptaError> {
        Ok(Self::new(crate::default_state_path(
            DEFAULT_SCHEDULER_STORE_PATH,
        )?))
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }

    pub fn report(&self, now_unix_ms: Option<u64>) -> Result<SchedulerStoreReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let store = self.load_or_default(now)?;
        Ok(SchedulerStoreReport {
            store_path: self.path_display(),
            enabled_count: count_status(&store, SchedulerJobStatus::Enabled),
            disabled_count: count_status(&store, SchedulerJobStatus::Disabled),
            due_count: count_status(&store, SchedulerJobStatus::Due),
            running_count: count_status(&store, SchedulerJobStatus::Running),
            completed_count: count_status(&store, SchedulerJobStatus::Completed),
            failed_count: count_status(&store, SchedulerJobStatus::Failed),
            queued_wake_count: store.queued_wakes.len(),
            persisted: self.path.exists(),
            store,
        })
    }

    pub fn schedule_job(
        &self,
        input: SchedulerJobInput,
    ) -> Result<SchedulerStoreScheduleReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let name = normalize_non_empty(&input.name, "name")?;
        let schedule_expr = normalize_non_empty(&input.schedule_expr, "schedule expr")?;
        let payload_kind = normalize_non_empty(&input.payload_kind, "payload kind")?;
        let payload_preview = normalize_non_empty(&input.payload_preview, "payload preview")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        let explicit_timeout_seconds = normalize_timeout_seconds(input.explicit_timeout_seconds)?;
        let session_bound_heartbeat_route = input
            .session_bound_heartbeat_route
            .as_deref()
            .map(normalize_session_target)
            .transpose()?;
        if let Some(existing) = store
            .jobs
            .iter()
            .find(|job| job.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(SchedulerStoreScheduleReport {
                store_path: self.path_display(),
                job: existing,
                duplicate_idempotency_key: true,
                persisted: self.path.exists(),
            });
        }
        let job_id = format!("sched-{}-{}", now, store.jobs.len() + 1);
        let job = SchedulerJobRecord {
            job_id: job_id.clone(),
            name,
            schedule_kind: input.schedule_kind,
            schedule_expr,
            timezone: input.timezone,
            payload_kind,
            payload_preview,
            idempotency_key,
            explicit_timeout_seconds,
            session_bound_heartbeat_route,
            status: SchedulerJobStatus::Enabled,
            next_due_unix_ms: input.next_due_unix_ms,
            last_run_unix_ms: None,
            created_at_unix_ms: now,
            updated_at_unix_ms: now,
        };
        store.jobs.push(job.clone());
        push_event(
            &mut store,
            "job_scheduled",
            &job_id,
            now,
            "scheduler job persisted; no cron materialized by store",
        );
        self.save(&mut store, now)?;
        Ok(SchedulerStoreScheduleReport {
            store_path: self.path_display(),
            job,
            duplicate_idempotency_key: false,
            persisted: true,
        })
    }

    pub fn get_job(&self, job_id: &str) -> Result<SchedulerJobReadbackReport, HeptaError> {
        let now = current_unix_ms()?;
        let store = self.load_or_default(now)?;
        let job_id = normalize_non_empty(job_id, "job id")?;
        let job = store
            .jobs
            .iter()
            .find(|candidate| candidate.job_id == job_id)
            .cloned()
            .ok_or_else(|| HeptaError(format!("scheduler job not found: {job_id}")))?;
        let active_run_count = store
            .runs
            .iter()
            .filter(|run| run.job_id == job_id && run.status == SchedulerJobStatus::Running)
            .count();
        Ok(SchedulerJobReadbackReport {
            store_path: self.path_display(),
            explicit_timeout_intent_preserved: job.explicit_timeout_seconds.is_some(),
            session_bound_heartbeat_route_present: job.session_bound_heartbeat_route.is_some(),
            job,
            active_run_count,
            persisted: self.path.exists(),
        })
    }

    pub fn set_enabled(
        &self,
        job_id: &str,
        enabled: bool,
    ) -> Result<SchedulerStoreTransitionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let job_id = normalize_non_empty(job_id, "job id")?;
        let job = find_job_mut(&mut store, &job_id)?;
        job.status = if enabled {
            SchedulerJobStatus::Enabled
        } else {
            SchedulerJobStatus::Disabled
        };
        job.updated_at_unix_ms = now;
        let status = job.status;
        push_event(
            &mut store,
            status.label(),
            &job_id,
            now,
            "scheduler job enabled state updated",
        );
        self.save(&mut store, now)?;
        Ok(SchedulerStoreTransitionReport {
            store_path: self.path_display(),
            job_id,
            status,
            persisted: true,
        })
    }

    pub fn mark_due(&self, job_id: &str) -> Result<SchedulerStoreTransitionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let job_id = normalize_non_empty(job_id, "job id")?;
        let job = find_job_mut(&mut store, &job_id)?;
        if job.status != SchedulerJobStatus::Enabled {
            return Err(HeptaError(format!(
                "scheduler job {job_id} cannot become due from {}",
                job.status.label()
            )));
        }
        job.status = SchedulerJobStatus::Due;
        job.updated_at_unix_ms = now;
        push_event(
            &mut store,
            "job_due",
            &job_id,
            now,
            "scheduler job marked due; no wake enqueued by store",
        );
        self.save(&mut store, now)?;
        Ok(SchedulerStoreTransitionReport {
            store_path: self.path_display(),
            job_id,
            status: SchedulerJobStatus::Due,
            persisted: true,
        })
    }

    pub fn start_run(&self, job_id: &str) -> Result<SchedulerStoreRunReport, HeptaError> {
        self.start_run_with_intent(
            job_id,
            SchedulerRunStartInput {
                long_run_active: false,
                active_in_task_registry: false,
            },
        )
    }

    pub fn start_run_with_intent(
        &self,
        job_id: &str,
        input: SchedulerRunStartInput,
    ) -> Result<SchedulerStoreRunReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let job_id = normalize_non_empty(job_id, "job id")?;
        let job = find_job_mut(&mut store, &job_id)?;
        if !matches!(
            job.status,
            SchedulerJobStatus::Due | SchedulerJobStatus::Enabled
        ) {
            return Err(HeptaError(format!(
                "scheduler job {job_id} cannot start from {}",
                job.status.label()
            )));
        }
        job.status = SchedulerJobStatus::Running;
        job.last_run_unix_ms = Some(now);
        job.updated_at_unix_ms = now;
        let timeout_seconds = job.explicit_timeout_seconds;
        let run = SchedulerRunRecord {
            run_id: format!("run-{}-{}", now, store.runs.len() + 1),
            job_id: job_id.clone(),
            status: SchedulerJobStatus::Running,
            started_at_unix_ms: now,
            timeout_seconds,
            long_run_active: input.long_run_active,
            active_in_task_registry: input.active_in_task_registry,
            finished_at_unix_ms: None,
            readback_evidence_id: None,
            summary: None,
        };
        store.runs.push(run.clone());
        push_event(
            &mut store,
            "job_run_started",
            &job_id,
            now,
            if input.long_run_active || input.active_in_task_registry {
                "scheduler long/manual run started and kept active in local task registry readback"
            } else {
                "scheduler run started by native adapter readback"
            },
        );
        self.save(&mut store, now)?;
        Ok(SchedulerStoreRunReport {
            store_path: self.path_display(),
            run,
            job_status: SchedulerJobStatus::Running,
            persisted: true,
        })
    }

    pub fn finish_run(
        &self,
        run_id: &str,
        success: bool,
        readback_evidence_id: &str,
        summary: &str,
        next_due_unix_ms: Option<u64>,
    ) -> Result<SchedulerStoreRunReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let run_id = normalize_non_empty(run_id, "run id")?;
        let readback_evidence_id =
            normalize_non_empty(readback_evidence_id, "readback evidence id")?;
        let summary = normalize_non_empty(summary, "summary")?;
        let run_index = store
            .runs
            .iter()
            .position(|run| run.run_id == run_id)
            .ok_or_else(|| HeptaError(format!("scheduler run not found: {run_id}")))?;
        if store.runs[run_index].status != SchedulerJobStatus::Running {
            return Err(HeptaError(format!(
                "scheduler run {run_id} is not running; current status is {}",
                store.runs[run_index].status.label()
            )));
        }
        let job_id = store.runs[run_index].job_id.clone();
        let status = if success {
            SchedulerJobStatus::Completed
        } else {
            SchedulerJobStatus::Failed
        };
        {
            let run = &mut store.runs[run_index];
            run.status = status;
            run.finished_at_unix_ms = Some(now);
            run.readback_evidence_id = Some(readback_evidence_id);
            run.summary = Some(summary);
        }
        let job = find_job_mut(&mut store, &job_id)?;
        job.status = if success && next_due_unix_ms.is_some() {
            SchedulerJobStatus::Enabled
        } else {
            status
        };
        job.next_due_unix_ms = next_due_unix_ms;
        job.updated_at_unix_ms = now;
        let run = store.runs[run_index].clone();
        push_event(
            &mut store,
            status.label(),
            &job_id,
            now,
            "scheduler run finished with readback evidence",
        );
        self.save(&mut store, now)?;
        Ok(SchedulerStoreRunReport {
            store_path: self.path_display(),
            run,
            job_status: if success && next_due_unix_ms.is_some() {
                SchedulerJobStatus::Enabled
            } else {
                status
            },
            persisted: true,
        })
    }

    pub fn gated_wake_session_handoff(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        job_id: &str,
        input: SchedulerWakeHandoffInput,
    ) -> Result<SchedulerWakeHandoffReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let job_id = normalize_non_empty(job_id, "job id")?;
        let session_target = normalize_session_target(&input.session_target)?;
        let wake_mode = normalize_wake_mode(&input.wake_mode)?;
        let payload_preview = normalize_non_empty(&input.payload_preview, "payload preview")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "scheduler wake handoff for {job_id} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "scheduler wake handoff for {job_id} requires allow/approved policy decision"
            )));
        }
        let job = store
            .jobs
            .iter()
            .find(|candidate| candidate.job_id == job_id)
            .cloned()
            .ok_or_else(|| HeptaError(format!("scheduler job not found: {job_id}")))?;
        if !matches!(
            job.status,
            SchedulerJobStatus::Due | SchedulerJobStatus::Running
        ) {
            return Err(HeptaError(format!(
                "scheduler wake handoff for {job_id} requires due/running job; current status is {}",
                job.status.label()
            )));
        }
        if let Some(run_id) = input.run_id.as_deref() {
            let run_id = normalize_non_empty(run_id, "run id")?;
            if !store
                .runs
                .iter()
                .any(|run| run.run_id == run_id && run.job_id == job_id)
            {
                return Err(HeptaError(format!(
                    "scheduler wake handoff run {run_id} does not belong to job {job_id}"
                )));
            }
        }
        if let Some(existing) = store
            .wake_handoffs
            .iter()
            .find(|handoff| handoff.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(SchedulerWakeHandoffReport {
                store_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                handoff: existing,
                duplicate_idempotency_key: true,
                scheduler_store_mutated_by_gate: false,
                wake_enqueued_by_gate: false,
                session_mutated_by_gate: false,
                persisted: self.path.exists(),
            });
        }
        let handoff_id = format!("wake-{}-{}", now, store.wake_handoffs.len() + 1);
        let evidence = evidence_ledger.append(
            "scheduler_wake_handoff",
            &handoff_id,
            "handoff_recorded",
            &format!(
                "scheduler wake/session-target handoff recorded for job {job_id}; session_target={session_target}; wake_mode={wake_mode}; wake/session mutation not performed by this gate"
            ),
        )?;
        let handoff = SchedulerWakeHandoffRecord {
            handoff_id: handoff_id.clone(),
            job_id: job_id.clone(),
            run_id: input.run_id,
            session_target,
            wake_mode,
            payload_preview,
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            readback_evidence_id: evidence.entry.evidence_id,
            created_at_unix_ms: now,
            wake_enqueued_by_gate: false,
            session_mutated_by_gate: false,
        };
        store.wake_handoffs.push(handoff.clone());
        store.wake_handoffs.truncate(1024);
        push_event(
            &mut store,
            "wake_handoff_recorded",
            &job_id,
            now,
            "scheduler wake/session-target handoff recorded with readback evidence",
        );
        self.save(&mut store, now)?;
        Ok(SchedulerWakeHandoffReport {
            store_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            handoff,
            duplicate_idempotency_key: false,
            scheduler_store_mutated_by_gate: true,
            wake_enqueued_by_gate: false,
            session_mutated_by_gate: false,
            persisted: evidence.persisted,
        })
    }

    pub fn materialize_wake_from_handoff(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        input: SchedulerWakeMaterializationInput,
    ) -> Result<SchedulerWakeMaterializationReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut store = self.load_or_default(now)?;
        let handoff_id = normalize_non_empty(&input.handoff_id, "handoff id")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "scheduler wake materialization for {handoff_id} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "scheduler wake materialization for {handoff_id} requires allow/approved policy decision"
            )));
        }
        if let Some(existing) = store
            .queued_wakes
            .iter()
            .find(|wake| wake.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(SchedulerWakeMaterializationReport {
                store_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                wake: existing,
                duplicate_idempotency_key: true,
                scheduler_store_mutated_by_adapter: false,
                wake_enqueued_by_adapter: false,
                session_mutated_by_adapter: false,
                gateway_rpc_performed_by_adapter: false,
                persisted: self.path.exists(),
            });
        }
        let handoff_index = store
            .wake_handoffs
            .iter()
            .position(|handoff| handoff.handoff_id == handoff_id)
            .ok_or_else(|| HeptaError(format!("scheduler wake handoff not found: {handoff_id}")))?;
        let handoff = store.wake_handoffs[handoff_index].clone();
        if !handoff.operator_confirmed || !policy_allows_handoff(&handoff.policy_decision) {
            return Err(HeptaError(format!(
                "scheduler wake handoff {handoff_id} is not approved for materialization"
            )));
        }
        if handoff.wake_enqueued_by_gate {
            return Err(HeptaError(format!(
                "scheduler wake handoff {handoff_id} has already been materialized"
            )));
        }
        let wake_id = format!("queuedwake-{}-{}", now, store.queued_wakes.len() + 1);
        let evidence = evidence_ledger.append(
            "scheduler_wake_queue",
            &wake_id,
            "queued",
            &format!(
                "scheduler wake queued locally for job {}; session_target={}; wake_mode={}; session/Gateway mutation not performed by this adapter",
                handoff.job_id, handoff.session_target, handoff.wake_mode
            ),
        )?;
        let wake = SchedulerQueuedWakeRecord {
            wake_id: wake_id.clone(),
            handoff_id: handoff.handoff_id.clone(),
            job_id: handoff.job_id.clone(),
            run_id: handoff.run_id.clone(),
            session_target: handoff.session_target.clone(),
            wake_mode: handoff.wake_mode.clone(),
            payload_preview: handoff.payload_preview.clone(),
            status: "queued".into(),
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            readback_evidence_id: evidence.entry.evidence_id,
            created_at_unix_ms: now,
            session_mutated_by_adapter: false,
            gateway_rpc_performed_by_adapter: false,
        };
        store.queued_wakes.push(wake.clone());
        store.queued_wakes.truncate(1024);
        store.wake_handoffs[handoff_index].wake_enqueued_by_gate = true;
        push_event(
            &mut store,
            "wake_queued",
            &handoff.job_id,
            now,
            "scheduler wake materialized into Hepta local wake queue with readback evidence",
        );
        self.save(&mut store, now)?;
        Ok(SchedulerWakeMaterializationReport {
            store_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            wake,
            duplicate_idempotency_key: false,
            scheduler_store_mutated_by_adapter: true,
            wake_enqueued_by_adapter: true,
            session_mutated_by_adapter: false,
            gateway_rpc_performed_by_adapter: false,
            persisted: evidence.persisted,
        })
    }

    fn load_or_default(&self, now_unix_ms: u64) -> Result<SchedulerStoreFile, HeptaError> {
        if !self.path.exists() {
            return Ok(SchedulerStoreFile {
                version: 1,
                store_id: DEFAULT_SCHEDULER_STORE_ID.into(),
                created_at_unix_ms: now_unix_ms,
                updated_at_unix_ms: now_unix_ms,
                jobs: Vec::new(),
                runs: Vec::new(),
                wake_handoffs: Vec::new(),
                queued_wakes: Vec::new(),
                events: Vec::new(),
            });
        }
        let text = fs::read_to_string(&self.path).map_err(|err| {
            HeptaError(format!(
                "failed to read scheduler-store {}: {err}",
                self.path.display()
            ))
        })?;
        let mut store: SchedulerStoreFile = serde_json::from_str(&text).map_err(|err| {
            HeptaError(format!(
                "failed to parse scheduler-store {}: {err}",
                self.path.display()
            ))
        })?;
        if store.version != 1 {
            return Err(HeptaError(format!(
                "unsupported scheduler-store version {} in {}",
                store.version,
                self.path.display()
            )));
        }
        store.events.truncate(1024);
        store.runs.truncate(1024);
        store.wake_handoffs.truncate(1024);
        store.queued_wakes.truncate(1024);
        Ok(store)
    }

    fn save(&self, store: &mut SchedulerStoreFile, now_unix_ms: u64) -> Result<(), HeptaError> {
        store.updated_at_unix_ms = now_unix_ms;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!(
                    "failed to create scheduler-store directory {}: {err}",
                    parent.display()
                ))
            })?;
        }
        let text = serde_json::to_string_pretty(store)
            .map_err(|err| HeptaError(format!("failed to serialize scheduler-store: {err}")))?;
        fs::write(&self.path, text).map_err(|err| {
            HeptaError(format!(
                "failed to write scheduler-store {}: {err}",
                self.path.display()
            ))
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedulerJobInput {
    pub name: String,
    pub schedule_kind: SchedulerScheduleKind,
    pub schedule_expr: String,
    pub timezone: Option<String>,
    pub payload_kind: String,
    pub payload_preview: String,
    pub idempotency_key: String,
    pub explicit_timeout_seconds: Option<u64>,
    pub session_bound_heartbeat_route: Option<String>,
    pub next_due_unix_ms: Option<u64>,
}

fn count_status(store: &SchedulerStoreFile, status: SchedulerJobStatus) -> usize {
    store.jobs.iter().filter(|job| job.status == status).count()
}

fn find_job_mut<'a>(
    store: &'a mut SchedulerStoreFile,
    job_id: &str,
) -> Result<&'a mut SchedulerJobRecord, HeptaError> {
    store
        .jobs
        .iter_mut()
        .find(|job| job.job_id == job_id)
        .ok_or_else(|| HeptaError(format!("scheduler job not found: {job_id}")))
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!(
            "scheduler store {label} must not be empty"
        )));
    }
    Ok(trimmed.to_string())
}

fn normalize_session_target(value: &str) -> Result<String, HeptaError> {
    let target = normalize_non_empty(value, "session target")?;
    if matches!(target.as_str(), "main" | "current" | "isolated")
        || target
            .strip_prefix("session:")
            .map(|custom| !custom.trim().is_empty())
            .unwrap_or(false)
    {
        return Ok(target);
    }
    Err(HeptaError(format!(
        "scheduler wake handoff session target {target} must be main, current, isolated, or session:<id>"
    )))
}

fn normalize_wake_mode(value: &str) -> Result<String, HeptaError> {
    let wake_mode = normalize_non_empty(value, "wake mode")?;
    if matches!(wake_mode.as_str(), "now" | "next-heartbeat") {
        return Ok(wake_mode);
    }
    Err(HeptaError(format!(
        "scheduler wake handoff wake mode {wake_mode} must be now or next-heartbeat"
    )))
}

fn normalize_timeout_seconds(value: Option<u64>) -> Result<Option<u64>, HeptaError> {
    if let Some(value) = value {
        if value > 7 * 24 * 60 * 60 {
            return Err(HeptaError(
                "scheduler explicit timeout seconds must be <= 604800".into(),
            ));
        }
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

fn policy_allows_handoff(policy_decision: &str) -> bool {
    let policy = policy_decision.to_ascii_lowercase();
    policy.contains("allow") || policy.contains("approved")
}

fn push_event(
    store: &mut SchedulerStoreFile,
    event_type: &str,
    job_id: &str,
    now_unix_ms: u64,
    summary: &str,
) {
    store.events.push(SchedulerStoreEvent {
        event_id: format!("schevt-{}-{}", now_unix_ms, store.events.len() + 1),
        event_type: event_type.into(),
        job_id: job_id.into(),
        occurred_at_unix_ms: now_unix_ms,
        summary: summary.into(),
    });
    store.events.truncate(1024);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ReadbackEvidenceLedger;

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hepta-scheduler-store-test-{}-{}-{name}.json",
            std::process::id(),
            current_unix_ms().unwrap_or(0)
        ))
    }

    fn sample_job(idempotency_key: &str) -> SchedulerJobInput {
        SchedulerJobInput {
            name: "morning digest".into(),
            schedule_kind: SchedulerScheduleKind::Cron,
            schedule_expr: "0 9 * * *".into(),
            timezone: Some("Asia/Shanghai".into()),
            payload_kind: "agentTurn".into(),
            payload_preview: "summarize inbox without sending external messages".into(),
            idempotency_key: idempotency_key.into(),
            explicit_timeout_seconds: None,
            session_bound_heartbeat_route: None,
            next_due_unix_ms: Some(current_unix_ms().unwrap_or(0).saturating_add(60_000)),
        }
    }

    #[test]
    fn scheduler_store_tracks_job_and_run_lifecycle() {
        let path = temp_file("run");
        let store = SchedulerStore::new(&path);
        let scheduled = store.schedule_job(sample_job("sched-idem-1")).unwrap();
        assert_eq!(scheduled.job.status, SchedulerJobStatus::Enabled);
        let duplicate = store.schedule_job(sample_job("sched-idem-1")).unwrap();
        assert!(duplicate.duplicate_idempotency_key);
        store.mark_due(&scheduled.job.job_id).unwrap();
        let run = store.start_run(&scheduled.job.job_id).unwrap();
        assert_eq!(run.job_status, SchedulerJobStatus::Running);
        let finished = store
            .finish_run(
                &run.run.run_id,
                true,
                "rb-scheduler-1",
                "run completed locally",
                Some(current_unix_ms().unwrap_or(0).saturating_add(120_000)),
            )
            .unwrap();
        assert_eq!(finished.run.status, SchedulerJobStatus::Completed);
        assert_eq!(finished.job_status, SchedulerJobStatus::Enabled);
        let report = store.report(None).unwrap();
        assert_eq!(report.enabled_count, 1);
        assert_eq!(report.store.runs.len(), 1);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn scheduler_store_reads_single_job_and_preserves_timeout_route_and_long_run_status() {
        let path = temp_file("readback-timeout-route");
        let store = SchedulerStore::new(&path);
        let mut input = sample_job("sched-idem-readback");
        input.explicit_timeout_seconds = Some(0);
        input.session_bound_heartbeat_route = Some("session:daily-digest".into());
        let scheduled = store.schedule_job(input).unwrap();
        let readback = store.get_job(&scheduled.job.job_id).unwrap();
        assert_eq!(readback.job.explicit_timeout_seconds, Some(0));
        assert_eq!(
            readback.job.session_bound_heartbeat_route.as_deref(),
            Some("session:daily-digest")
        );
        assert!(readback.explicit_timeout_intent_preserved);
        assert!(readback.session_bound_heartbeat_route_present);
        assert_eq!(readback.active_run_count, 0);
        store.mark_due(&scheduled.job.job_id).unwrap();
        let run = store
            .start_run_with_intent(
                &scheduled.job.job_id,
                SchedulerRunStartInput {
                    long_run_active: true,
                    active_in_task_registry: true,
                },
            )
            .unwrap();
        assert_eq!(run.run.timeout_seconds, Some(0));
        assert!(run.run.long_run_active);
        assert!(run.run.active_in_task_registry);
        let active = store.get_job(&scheduled.job.job_id).unwrap();
        assert_eq!(active.active_run_count, 1);
        let report = store.report(None).unwrap();
        assert!(report.store.events.iter().any(|event| {
            event.event_type == "job_run_started"
                && event.summary.contains("kept active in local task registry")
        }));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn scheduler_store_blocks_disabled_jobs_from_due_transition() {
        let path = temp_file("disabled");
        let store = SchedulerStore::new(&path);
        let scheduled = store
            .schedule_job(sample_job("sched-idem-disabled"))
            .unwrap();
        store.set_enabled(&scheduled.job.job_id, false).unwrap();
        assert!(store.mark_due(&scheduled.job.job_id).is_err());
        let report = store.report(None).unwrap();
        assert_eq!(report.disabled_count, 1);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn scheduler_store_gated_wake_handoff_records_readback_without_enqueuing_wake() {
        let path = temp_file("wake-handoff");
        let ledger_path = temp_file("wake-handoff-ledger");
        let store = SchedulerStore::new(&path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let scheduled = store
            .schedule_job(sample_job("sched-idem-wake"))
            .expect("job should schedule");
        let enabled_handoff = SchedulerWakeHandoffInput {
            run_id: None,
            session_target: "current".into(),
            wake_mode: "next-heartbeat".into(),
            payload_preview: "wake current session for scheduled digest".into(),
            policy_decision: "approved-wake".into(),
            operator_confirmed: true,
            idempotency_key: "wake-handoff-idem".into(),
        };
        assert!(
            store
                .gated_wake_session_handoff(&ledger, &scheduled.job.job_id, enabled_handoff)
                .is_err()
        );
        store.mark_due(&scheduled.job.job_id).unwrap();
        let unconfirmed = SchedulerWakeHandoffInput {
            run_id: None,
            session_target: "current".into(),
            wake_mode: "next-heartbeat".into(),
            payload_preview: "wake current session for scheduled digest".into(),
            policy_decision: "approved-wake".into(),
            operator_confirmed: false,
            idempotency_key: "wake-handoff-idem".into(),
        };
        assert!(
            store
                .gated_wake_session_handoff(&ledger, &scheduled.job.job_id, unconfirmed)
                .is_err()
        );
        let invalid_target = SchedulerWakeHandoffInput {
            run_id: None,
            session_target: "telegram:6476198178".into(),
            wake_mode: "next-heartbeat".into(),
            payload_preview: "wake current session for scheduled digest".into(),
            policy_decision: "approved-wake".into(),
            operator_confirmed: true,
            idempotency_key: "wake-handoff-idem".into(),
        };
        assert!(
            store
                .gated_wake_session_handoff(&ledger, &scheduled.job.job_id, invalid_target)
                .is_err()
        );
        let confirmed = SchedulerWakeHandoffInput {
            run_id: None,
            session_target: "session:morning-digest".into(),
            wake_mode: "now".into(),
            payload_preview: "wake named digest session after cron trigger".into(),
            policy_decision: "allow-wake-session-target".into(),
            operator_confirmed: true,
            idempotency_key: "wake-handoff-idem".into(),
        };
        let report = store
            .gated_wake_session_handoff(&ledger, &scheduled.job.job_id, confirmed.clone())
            .expect("confirmed handoff should record");
        assert!(report.scheduler_store_mutated_by_gate);
        assert!(!report.wake_enqueued_by_gate);
        assert!(!report.session_mutated_by_gate);
        assert_eq!(report.handoff.session_target, "session:morning-digest");
        assert_eq!(report.handoff.wake_mode, "now");
        assert!(report.handoff.readback_evidence_id.starts_with("rb-"));
        let duplicate = store
            .gated_wake_session_handoff(&ledger, &scheduled.job.job_id, confirmed)
            .expect("duplicate handoff should be idempotent");
        assert!(duplicate.duplicate_idempotency_key);
        assert!(!duplicate.scheduler_store_mutated_by_gate);
        let materialization_input = SchedulerWakeMaterializationInput {
            handoff_id: report.handoff.handoff_id.clone(),
            policy_decision: "approved-local-wake-queue".into(),
            operator_confirmed: true,
            idempotency_key: "wake-materialize-idem".into(),
        };
        let wake = store
            .materialize_wake_from_handoff(&ledger, materialization_input.clone())
            .expect("confirmed wake handoff should materialize into local queue");
        assert!(wake.scheduler_store_mutated_by_adapter);
        assert!(wake.wake_enqueued_by_adapter);
        assert!(!wake.session_mutated_by_adapter);
        assert!(!wake.gateway_rpc_performed_by_adapter);
        assert_eq!(wake.wake.handoff_id, report.handoff.handoff_id);
        assert_eq!(wake.wake.session_target, "session:morning-digest");
        assert_eq!(wake.wake.status, "queued");
        let duplicate_wake = store
            .materialize_wake_from_handoff(&ledger, materialization_input)
            .expect("duplicate wake materialization should be idempotent");
        assert!(duplicate_wake.duplicate_idempotency_key);
        assert!(!duplicate_wake.wake_enqueued_by_adapter);
        let scheduler_report = store.report(None).unwrap();
        assert_eq!(scheduler_report.store.wake_handoffs.len(), 1);
        assert_eq!(scheduler_report.queued_wake_count, 1);
        assert_eq!(scheduler_report.store.queued_wakes.len(), 1);
        assert!(scheduler_report.store.wake_handoffs[0].wake_enqueued_by_gate);
        assert!(scheduler_report.store.events.iter().any(|event| {
            event.event_type == "wake_handoff_recorded"
                && event.summary.contains("readback evidence")
        }));
        assert!(scheduler_report.store.events.iter().any(|event| {
            event.event_type == "wake_queued" && event.summary.contains("local wake queue")
        }));
        let ledger_report = ledger.report(None).unwrap();
        assert_eq!(ledger_report.evidence_count, 2);
        assert_eq!(
            ledger_report.ledger.entries[0].subject_kind,
            "scheduler_wake_handoff"
        );
        assert_eq!(
            ledger_report.ledger.entries[1].subject_kind,
            "scheduler_wake_queue"
        );
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }
}
