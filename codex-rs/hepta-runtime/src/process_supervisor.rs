use std::{
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use hepta_core::HeptaError;
use serde::{Deserialize, Serialize};

use crate::{current_unix_ms, delivery_queue::ReadbackEvidenceLedger};

pub const DEFAULT_PROCESS_SUPERVISOR_PATH: &str = ".hepta/process-supervisor-v0.json";
pub const DEFAULT_PROCESS_SUPERVISOR_ID: &str = "hepta-native-process-supervisor";
pub const DEFAULT_PROCESS_LOG_LIMIT: usize = 256;
pub const DEFAULT_PROCESS_EXEC_TIMEOUT_MS: u64 = 10_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupervisedProcessStatus {
    Planned,
    Running,
    Exited,
    Failed,
    Killed,
    CleanupReady,
}

impl SupervisedProcessStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Running => "running",
            Self::Exited => "exited",
            Self::Failed => "failed",
            Self::Killed => "killed",
            Self::CleanupReady => "cleanup_ready",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessSupervisorFile {
    pub version: u32,
    pub supervisor_id: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default)]
    pub processes: Vec<SupervisedProcessRecord>,
    #[serde(default)]
    pub start_handoffs: Vec<ProcessStartHandoffRecord>,
    #[serde(default)]
    pub start_executions: Vec<ProcessStartExecutionRecord>,
    #[serde(default)]
    pub events: Vec<ProcessSupervisorEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupervisedProcessRecord {
    pub process_id: String,
    pub command_preview: String,
    pub cwd_preview: String,
    pub idempotency_key: String,
    pub status: SupervisedProcessStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_pid: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signal_plan: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readback_evidence_id: Option<String>,
    #[serde(default)]
    pub stdout_tail: Vec<String>,
    #[serde(default)]
    pub stderr_tail: Vec<String>,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessStartHandoffRecord {
    pub handoff_id: String,
    pub process_id: String,
    pub command_preview: String,
    pub cwd_preview: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub native_process_spawned_by_gate: bool,
    pub stdin_written_by_gate: bool,
    pub signal_sent_by_gate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessStartExecutionRecord {
    pub execution_id: String,
    pub handoff_id: String,
    pub process_id: String,
    pub command_preview: String,
    pub cwd_preview: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub native_pid: u32,
    pub exit_code: i32,
    pub status: SupervisedProcessStatus,
    pub stdout_tail: Vec<String>,
    pub stderr_tail: Vec<String>,
    pub timeout_killed_by_adapter: bool,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub finished_at_unix_ms: u64,
    pub native_process_spawned_by_adapter: bool,
    pub stdin_written_by_adapter: bool,
    pub signal_sent_by_adapter: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessSupervisorEvent {
    pub event_id: String,
    pub event_type: String,
    pub process_id: String,
    pub occurred_at_unix_ms: u64,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProcessSupervisorReport {
    pub supervisor_path: String,
    pub supervisor: ProcessSupervisorFile,
    pub planned_count: usize,
    pub running_count: usize,
    pub exited_count: usize,
    pub failed_count: usize,
    pub killed_count: usize,
    pub cleanup_ready_count: usize,
    pub start_execution_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProcessSupervisorPlanReport {
    pub supervisor_path: String,
    pub process: SupervisedProcessRecord,
    pub duplicate_idempotency_key: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProcessSupervisorTransitionReport {
    pub supervisor_path: String,
    pub process_id: String,
    pub status: SupervisedProcessStatus,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProcessStartHandoffInput {
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProcessStartHandoffReport {
    pub supervisor_path: String,
    pub evidence_ledger_path: String,
    pub handoff: ProcessStartHandoffRecord,
    pub duplicate_idempotency_key: bool,
    pub supervisor_mutated_by_gate: bool,
    pub native_process_spawned_by_gate: bool,
    pub stdin_written_by_gate: bool,
    pub signal_sent_by_gate: bool,
    pub process_status: SupervisedProcessStatus,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProcessStartExecutionInput {
    pub handoff_id: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProcessStartExecutionReport {
    pub supervisor_path: String,
    pub evidence_ledger_path: String,
    pub execution: ProcessStartExecutionRecord,
    pub duplicate_idempotency_key: bool,
    pub supervisor_mutated_by_adapter: bool,
    pub native_process_spawned_by_adapter: bool,
    pub stdin_written_by_adapter: bool,
    pub signal_sent_by_adapter: bool,
    pub timeout_killed_by_adapter: bool,
    pub process_status: SupervisedProcessStatus,
    pub persisted: bool,
}

pub struct ProcessSupervisor {
    path: PathBuf,
}

impl ProcessSupervisor {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_in_current_dir() -> Result<Self, HeptaError> {
        let cwd = std::env::current_dir().map_err(|err| {
            HeptaError(format!(
                "failed to resolve cwd for process-supervisor: {err}"
            ))
        })?;
        Ok(Self::new(cwd.join(DEFAULT_PROCESS_SUPERVISOR_PATH)))
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }

    pub fn report(&self, now_unix_ms: Option<u64>) -> Result<ProcessSupervisorReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let supervisor = self.load_or_default(now)?;
        Ok(ProcessSupervisorReport {
            supervisor_path: self.path_display(),
            planned_count: count_status(&supervisor, SupervisedProcessStatus::Planned),
            running_count: count_status(&supervisor, SupervisedProcessStatus::Running),
            exited_count: count_status(&supervisor, SupervisedProcessStatus::Exited),
            failed_count: count_status(&supervisor, SupervisedProcessStatus::Failed),
            killed_count: count_status(&supervisor, SupervisedProcessStatus::Killed),
            cleanup_ready_count: count_status(&supervisor, SupervisedProcessStatus::CleanupReady),
            start_execution_count: supervisor.start_executions.len(),
            persisted: self.path.exists(),
            supervisor,
        })
    }

    pub fn plan_process(
        &self,
        command_preview: &str,
        cwd_preview: &str,
        idempotency_key: &str,
    ) -> Result<ProcessSupervisorPlanReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut supervisor = self.load_or_default(now)?;
        let command_preview = normalize_non_empty(command_preview, "command preview")?;
        let cwd_preview = normalize_non_empty(cwd_preview, "cwd preview")?;
        let idempotency_key = normalize_non_empty(idempotency_key, "idempotency key")?;
        if let Some(existing) = supervisor
            .processes
            .iter()
            .find(|process| process.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(ProcessSupervisorPlanReport {
                supervisor_path: self.path_display(),
                process: existing,
                duplicate_idempotency_key: true,
                persisted: self.path.exists(),
            });
        }
        let process_id = format!("proc-{}-{}", now, supervisor.processes.len() + 1);
        let process = SupervisedProcessRecord {
            process_id: process_id.clone(),
            command_preview,
            cwd_preview,
            idempotency_key,
            status: SupervisedProcessStatus::Planned,
            native_pid: None,
            exit_code: None,
            signal_plan: None,
            readback_evidence_id: None,
            stdout_tail: Vec::new(),
            stderr_tail: Vec::new(),
            created_at_unix_ms: now,
            updated_at_unix_ms: now,
        };
        supervisor.processes.push(process.clone());
        push_event(
            &mut supervisor,
            "process_planned",
            &process_id,
            now,
            "process exact command preview registered; no spawn performed by supervisor store",
        );
        self.save(&mut supervisor, now)?;
        Ok(ProcessSupervisorPlanReport {
            supervisor_path: self.path_display(),
            process,
            duplicate_idempotency_key: false,
            persisted: true,
        })
    }

    pub fn mark_started(
        &self,
        process_id: &str,
        native_pid: u32,
    ) -> Result<ProcessSupervisorTransitionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut supervisor = self.load_or_default(now)?;
        let process_id = normalize_non_empty(process_id, "process id")?;
        let process = find_process_mut(&mut supervisor, &process_id)?;
        if process.status != SupervisedProcessStatus::Planned {
            return Err(HeptaError(format!(
                "process {process_id} cannot start from {}",
                process.status.label()
            )));
        }
        process.status = SupervisedProcessStatus::Running;
        process.native_pid = Some(native_pid);
        process.updated_at_unix_ms = now;
        push_event(
            &mut supervisor,
            "process_started",
            &process_id,
            now,
            "process marked running by native adapter readback",
        );
        self.save(&mut supervisor, now)?;
        Ok(ProcessSupervisorTransitionReport {
            supervisor_path: self.path_display(),
            process_id,
            status: SupervisedProcessStatus::Running,
            persisted: true,
        })
    }

    pub fn append_log_line(
        &self,
        process_id: &str,
        stream: &str,
        line: &str,
    ) -> Result<ProcessSupervisorTransitionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut supervisor = self.load_or_default(now)?;
        let process_id = normalize_non_empty(process_id, "process id")?;
        let stream = normalize_non_empty(stream, "stream")?;
        let line = normalize_non_empty(line, "log line")?;
        let process = find_process_mut(&mut supervisor, &process_id)?;
        match stream.as_str() {
            "stdout" => push_tail(&mut process.stdout_tail, line),
            "stderr" => push_tail(&mut process.stderr_tail, line),
            other => return Err(HeptaError(format!("unsupported process stream: {other}"))),
        }
        process.updated_at_unix_ms = now;
        let status = process.status;
        self.save(&mut supervisor, now)?;
        Ok(ProcessSupervisorTransitionReport {
            supervisor_path: self.path_display(),
            process_id,
            status,
            persisted: true,
        })
    }

    pub fn mark_exited(
        &self,
        process_id: &str,
        exit_code: i32,
        readback_evidence_id: &str,
    ) -> Result<ProcessSupervisorTransitionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut supervisor = self.load_or_default(now)?;
        let process_id = normalize_non_empty(process_id, "process id")?;
        let readback_evidence_id =
            normalize_non_empty(readback_evidence_id, "readback evidence id")?;
        let process = find_process_mut(&mut supervisor, &process_id)?;
        if process.status != SupervisedProcessStatus::Running {
            return Err(HeptaError(format!(
                "process {process_id} cannot exit from {}",
                process.status.label()
            )));
        }
        process.status = if exit_code == 0 {
            SupervisedProcessStatus::Exited
        } else {
            SupervisedProcessStatus::Failed
        };
        process.exit_code = Some(exit_code);
        process.readback_evidence_id = Some(readback_evidence_id);
        process.updated_at_unix_ms = now;
        let status = process.status;
        push_event(
            &mut supervisor,
            status.label(),
            &process_id,
            now,
            "process exit readback recorded",
        );
        self.save(&mut supervisor, now)?;
        Ok(ProcessSupervisorTransitionReport {
            supervisor_path: self.path_display(),
            process_id,
            status,
            persisted: true,
        })
    }

    pub fn plan_signal(
        &self,
        process_id: &str,
        signal_plan: &str,
    ) -> Result<ProcessSupervisorTransitionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut supervisor = self.load_or_default(now)?;
        let process_id = normalize_non_empty(process_id, "process id")?;
        let signal_plan = normalize_non_empty(signal_plan, "signal plan")?;
        let process = find_process_mut(&mut supervisor, &process_id)?;
        process.signal_plan = Some(signal_plan);
        process.updated_at_unix_ms = now;
        let status = process.status;
        push_event(
            &mut supervisor,
            "signal_planned",
            &process_id,
            now,
            "signal escalation plan staged; no signal sent by store",
        );
        self.save(&mut supervisor, now)?;
        Ok(ProcessSupervisorTransitionReport {
            supervisor_path: self.path_display(),
            process_id,
            status,
            persisted: true,
        })
    }

    pub fn mark_cleanup_ready(
        &self,
        process_id: &str,
    ) -> Result<ProcessSupervisorTransitionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut supervisor = self.load_or_default(now)?;
        let process_id = normalize_non_empty(process_id, "process id")?;
        let process = find_process_mut(&mut supervisor, &process_id)?;
        if !matches!(
            process.status,
            SupervisedProcessStatus::Exited
                | SupervisedProcessStatus::Failed
                | SupervisedProcessStatus::Killed
        ) {
            return Err(HeptaError(format!(
                "process {process_id} is not terminal; current status is {}",
                process.status.label()
            )));
        }
        process.status = SupervisedProcessStatus::CleanupReady;
        process.updated_at_unix_ms = now;
        push_event(
            &mut supervisor,
            "cleanup_ready",
            &process_id,
            now,
            "process marked cleanup-ready",
        );
        self.save(&mut supervisor, now)?;
        Ok(ProcessSupervisorTransitionReport {
            supervisor_path: self.path_display(),
            process_id,
            status: SupervisedProcessStatus::CleanupReady,
            persisted: true,
        })
    }

    pub fn gated_start_handoff(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        process_id: &str,
        input: ProcessStartHandoffInput,
    ) -> Result<ProcessStartHandoffReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut supervisor = self.load_or_default(now)?;
        let process_id = normalize_non_empty(process_id, "process id")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "process start handoff for {process_id} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "process start handoff for {process_id} requires allow/approved policy decision"
            )));
        }
        let process = supervisor
            .processes
            .iter()
            .find(|candidate| candidate.process_id == process_id)
            .cloned()
            .ok_or_else(|| HeptaError(format!("process not found: {process_id}")))?;
        if process.status != SupervisedProcessStatus::Planned {
            return Err(HeptaError(format!(
                "process start handoff for {process_id} requires planned process; current status is {}",
                process.status.label()
            )));
        }
        if let Some(existing) = supervisor
            .start_handoffs
            .iter()
            .find(|handoff| handoff.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(ProcessStartHandoffReport {
                supervisor_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                process_status: process.status,
                handoff: existing,
                duplicate_idempotency_key: true,
                supervisor_mutated_by_gate: false,
                native_process_spawned_by_gate: false,
                stdin_written_by_gate: false,
                signal_sent_by_gate: false,
                persisted: self.path.exists(),
            });
        }
        let handoff_id = format!("pstart-{}-{}", now, supervisor.start_handoffs.len() + 1);
        let evidence = evidence_ledger.append(
            "process_start_handoff",
            &handoff_id,
            "handoff_recorded",
            &format!(
                "process start handoff recorded for {process_id}; command preview already planned; native process spawn/stdin/signal not performed by this gate"
            ),
        )?;
        let handoff = ProcessStartHandoffRecord {
            handoff_id: handoff_id.clone(),
            process_id: process_id.clone(),
            command_preview: process.command_preview,
            cwd_preview: process.cwd_preview,
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            readback_evidence_id: evidence.entry.evidence_id,
            created_at_unix_ms: now,
            native_process_spawned_by_gate: false,
            stdin_written_by_gate: false,
            signal_sent_by_gate: false,
        };
        supervisor.start_handoffs.push(handoff.clone());
        supervisor.start_handoffs.truncate(1024);
        push_event(
            &mut supervisor,
            "process_start_handoff_recorded",
            &process_id,
            now,
            "process start handoff recorded with readback evidence; no native spawn performed",
        );
        self.save(&mut supervisor, now)?;
        Ok(ProcessStartHandoffReport {
            supervisor_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            handoff,
            duplicate_idempotency_key: false,
            supervisor_mutated_by_gate: true,
            native_process_spawned_by_gate: false,
            stdin_written_by_gate: false,
            signal_sent_by_gate: false,
            process_status: SupervisedProcessStatus::Planned,
            persisted: evidence.persisted,
        })
    }

    pub fn execute_start_handoff_once(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        input: ProcessStartExecutionInput,
    ) -> Result<ProcessStartExecutionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut supervisor = self.load_or_default(now)?;
        let handoff_id = normalize_non_empty(&input.handoff_id, "handoff id")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "process start execution for {handoff_id} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "process start execution for {handoff_id} requires allow/approved policy decision"
            )));
        }
        let timeout_ms = normalize_timeout_ms(input.timeout_ms)?;
        if let Some(existing) = supervisor
            .start_executions
            .iter()
            .find(|execution| execution.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(ProcessStartExecutionReport {
                supervisor_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                process_status: existing.status,
                execution: existing,
                duplicate_idempotency_key: true,
                supervisor_mutated_by_adapter: false,
                native_process_spawned_by_adapter: false,
                stdin_written_by_adapter: false,
                signal_sent_by_adapter: false,
                timeout_killed_by_adapter: false,
                persisted: self.path.exists(),
            });
        }
        let handoff_index = supervisor
            .start_handoffs
            .iter()
            .position(|handoff| handoff.handoff_id == handoff_id)
            .ok_or_else(|| HeptaError(format!("process start handoff not found: {handoff_id}")))?;
        let handoff = supervisor.start_handoffs[handoff_index].clone();
        if !handoff.operator_confirmed || !policy_allows_handoff(&handoff.policy_decision) {
            return Err(HeptaError(format!(
                "process start handoff {handoff_id} is not approved for execution"
            )));
        }
        if handoff.native_process_spawned_by_gate {
            return Err(HeptaError(format!(
                "process start handoff {handoff_id} has already been executed"
            )));
        }
        let process_index = supervisor
            .processes
            .iter()
            .position(|process| process.process_id == handoff.process_id)
            .ok_or_else(|| HeptaError(format!("process not found: {}", handoff.process_id)))?;
        if supervisor.processes[process_index].status != SupervisedProcessStatus::Planned {
            return Err(HeptaError(format!(
                "process {} cannot execute from {}",
                handoff.process_id,
                supervisor.processes[process_index].status.label()
            )));
        }
        let (executable, args) = parse_command_preview(&handoff.command_preview)?;
        let cwd = normalize_cwd_path(&handoff.cwd_preview)?;
        let spawned = spawn_and_wait_bounded(&executable, &args, &cwd, timeout_ms)?;
        let finished_at = current_unix_ms()?;
        let status = if spawned.timeout_killed {
            SupervisedProcessStatus::Killed
        } else if spawned.exit_code == 0 {
            SupervisedProcessStatus::Exited
        } else {
            SupervisedProcessStatus::Failed
        };
        let execution_id = format!(
            "pexec-{}-{}",
            finished_at,
            supervisor.start_executions.len() + 1
        );
        let evidence = evidence_ledger.append(
            "process_start_execution",
            &execution_id,
            status.label(),
            &format!(
                "process start executed for {}; exit_code={}; timeout_killed={}; stdin/signal not performed by adapter",
                handoff.process_id, spawned.exit_code, spawned.timeout_killed
            ),
        )?;
        let execution = ProcessStartExecutionRecord {
            execution_id: execution_id.clone(),
            handoff_id: handoff.handoff_id.clone(),
            process_id: handoff.process_id.clone(),
            command_preview: handoff.command_preview.clone(),
            cwd_preview: handoff.cwd_preview.clone(),
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            native_pid: spawned.native_pid,
            exit_code: spawned.exit_code,
            status,
            stdout_tail: spawned.stdout_tail.clone(),
            stderr_tail: spawned.stderr_tail.clone(),
            timeout_killed_by_adapter: spawned.timeout_killed,
            readback_evidence_id: evidence.entry.evidence_id,
            created_at_unix_ms: now,
            finished_at_unix_ms: finished_at,
            native_process_spawned_by_adapter: true,
            stdin_written_by_adapter: false,
            signal_sent_by_adapter: false,
        };
        {
            let process = &mut supervisor.processes[process_index];
            process.status = status;
            process.native_pid = Some(spawned.native_pid);
            process.exit_code = Some(spawned.exit_code);
            process.readback_evidence_id = Some(execution.readback_evidence_id.clone());
            process.stdout_tail = spawned.stdout_tail;
            process.stderr_tail = spawned.stderr_tail;
            process.updated_at_unix_ms = finished_at;
        }
        supervisor.start_handoffs[handoff_index].native_process_spawned_by_gate = true;
        supervisor.start_executions.push(execution.clone());
        supervisor.start_executions.truncate(1024);
        push_event(
            &mut supervisor,
            "process_start_executed",
            &handoff.process_id,
            finished_at,
            "process start executed by Hepta native adapter with exit readback evidence",
        );
        self.save(&mut supervisor, finished_at)?;
        Ok(ProcessStartExecutionReport {
            supervisor_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            execution,
            duplicate_idempotency_key: false,
            supervisor_mutated_by_adapter: true,
            native_process_spawned_by_adapter: true,
            stdin_written_by_adapter: false,
            signal_sent_by_adapter: false,
            timeout_killed_by_adapter: spawned.timeout_killed,
            process_status: status,
            persisted: evidence.persisted,
        })
    }

    fn load_or_default(&self, now_unix_ms: u64) -> Result<ProcessSupervisorFile, HeptaError> {
        if !self.path.exists() {
            return Ok(ProcessSupervisorFile {
                version: 1,
                supervisor_id: DEFAULT_PROCESS_SUPERVISOR_ID.into(),
                created_at_unix_ms: now_unix_ms,
                updated_at_unix_ms: now_unix_ms,
                processes: Vec::new(),
                start_handoffs: Vec::new(),
                start_executions: Vec::new(),
                events: Vec::new(),
            });
        }
        let text = fs::read_to_string(&self.path).map_err(|err| {
            HeptaError(format!(
                "failed to read process-supervisor {}: {err}",
                self.path.display()
            ))
        })?;
        let mut supervisor: ProcessSupervisorFile = serde_json::from_str(&text).map_err(|err| {
            HeptaError(format!(
                "failed to parse process-supervisor {}: {err}",
                self.path.display()
            ))
        })?;
        if supervisor.version != 1 {
            return Err(HeptaError(format!(
                "unsupported process-supervisor version {} in {}",
                supervisor.version,
                self.path.display()
            )));
        }
        supervisor.events.truncate(1024);
        supervisor.start_handoffs.truncate(1024);
        supervisor.start_executions.truncate(1024);
        Ok(supervisor)
    }

    fn save(
        &self,
        supervisor: &mut ProcessSupervisorFile,
        now_unix_ms: u64,
    ) -> Result<(), HeptaError> {
        supervisor.updated_at_unix_ms = now_unix_ms;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!(
                    "failed to create process-supervisor directory {}: {err}",
                    parent.display()
                ))
            })?;
        }
        let text = serde_json::to_string_pretty(supervisor)
            .map_err(|err| HeptaError(format!("failed to serialize process-supervisor: {err}")))?;
        fs::write(&self.path, text).map_err(|err| {
            HeptaError(format!(
                "failed to write process-supervisor {}: {err}",
                self.path.display()
            ))
        })
    }
}

fn count_status(supervisor: &ProcessSupervisorFile, status: SupervisedProcessStatus) -> usize {
    supervisor
        .processes
        .iter()
        .filter(|process| process.status == status)
        .count()
}

fn find_process_mut<'a>(
    supervisor: &'a mut ProcessSupervisorFile,
    process_id: &str,
) -> Result<&'a mut SupervisedProcessRecord, HeptaError> {
    supervisor
        .processes
        .iter_mut()
        .find(|process| process.process_id == process_id)
        .ok_or_else(|| HeptaError(format!("process not found: {process_id}")))
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!(
            "process supervisor {label} must not be empty"
        )));
    }
    Ok(trimmed.to_string())
}

fn policy_allows_handoff(policy_decision: &str) -> bool {
    let policy = policy_decision.to_ascii_lowercase();
    policy.contains("allow") || policy.contains("approved")
}

fn normalize_timeout_ms(value: u64) -> Result<u64, HeptaError> {
    if value == 0 {
        return Err(HeptaError(
            "process start execution timeout_ms must be greater than zero".into(),
        ));
    }
    Ok(value.min(DEFAULT_PROCESS_EXEC_TIMEOUT_MS))
}

fn parse_command_preview(command_preview: &str) -> Result<(String, Vec<String>), HeptaError> {
    let command_preview = normalize_non_empty(command_preview, "command preview")?;
    let tokens = command_preview
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<_>>();
    if tokens.is_empty() {
        return Err(HeptaError("process command preview is empty".into()));
    }
    for token in &tokens {
        if token.chars().any(|ch| {
            matches!(
                ch,
                '\n' | '\r' | ';' | '|' | '&' | '>' | '<' | '\u{60}' | '$'
            )
        }) {
            return Err(HeptaError(
                "process command preview must not contain shell metacharacters".into(),
            ));
        }
    }
    let executable = tokens[0].clone();
    let executable_path = PathBuf::from(&executable);
    if !executable_path.is_absolute() {
        return Err(HeptaError(
            "process live execution requires an absolute executable path".into(),
        ));
    }
    if !executable_path.is_file() {
        return Err(HeptaError(format!(
            "process executable does not exist or is not a file: {executable}"
        )));
    }
    Ok((executable, tokens.into_iter().skip(1).collect()))
}

fn normalize_cwd_path(cwd_preview: &str) -> Result<PathBuf, HeptaError> {
    let cwd_preview = normalize_non_empty(cwd_preview, "cwd preview")?;
    if cwd_preview.contains('\n') || cwd_preview.contains('\r') || cwd_preview.contains("..") {
        return Err(HeptaError(
            "process cwd preview must be single-line and scoped".into(),
        ));
    }
    let path = PathBuf::from(&cwd_preview);
    if !path.is_dir() {
        return Err(HeptaError(format!(
            "process cwd does not exist or is not a directory: {cwd_preview}"
        )));
    }
    Ok(path)
}

#[derive(Debug)]
struct BoundedProcessOutput {
    native_pid: u32,
    exit_code: i32,
    stdout_tail: Vec<String>,
    stderr_tail: Vec<String>,
    timeout_killed: bool,
}

fn spawn_and_wait_bounded(
    executable: &str,
    args: &[String],
    cwd: &PathBuf,
    timeout_ms: u64,
) -> Result<BoundedProcessOutput, HeptaError> {
    let mut child = Command::new(executable)
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| HeptaError(format!("failed to spawn process {executable}: {err}")))?;
    let native_pid = child.id();
    let deadline = Instant::now() + Duration::from_millis(timeout_ms);
    let mut timeout_killed = false;
    loop {
        if child
            .try_wait()
            .map_err(|err| HeptaError(format!("failed to poll process {native_pid}: {err}")))?
            .is_some()
        {
            break;
        }
        if Instant::now() >= deadline {
            timeout_killed = true;
            let _ = child.kill();
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    let output = child
        .wait_with_output()
        .map_err(|err| HeptaError(format!("failed to collect process {native_pid}: {err}")))?;
    let exit_code = output
        .status
        .code()
        .unwrap_or(if timeout_killed { -9 } else { -1 });
    Ok(BoundedProcessOutput {
        native_pid,
        exit_code,
        stdout_tail: output_tail(&output.stdout),
        stderr_tail: output_tail(&output.stderr),
        timeout_killed,
    })
}

fn output_tail(bytes: &[u8]) -> Vec<String> {
    let mut lines = String::from_utf8_lossy(bytes)
        .lines()
        .map(|line| {
            if line.chars().count() > 240 {
                let mut preview = line.chars().take(240).collect::<String>();
                preview.push_str("...");
                preview
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>();
    if lines.len() > DEFAULT_PROCESS_LOG_LIMIT {
        let overflow = lines.len() - DEFAULT_PROCESS_LOG_LIMIT;
        lines.drain(0..overflow);
    }
    lines
}

fn push_tail(lines: &mut Vec<String>, line: String) {
    lines.push(line);
    if lines.len() > DEFAULT_PROCESS_LOG_LIMIT {
        let overflow = lines.len() - DEFAULT_PROCESS_LOG_LIMIT;
        lines.drain(0..overflow);
    }
}

fn push_event(
    supervisor: &mut ProcessSupervisorFile,
    event_type: &str,
    process_id: &str,
    now_unix_ms: u64,
    summary: &str,
) {
    supervisor.events.push(ProcessSupervisorEvent {
        event_id: format!("psevt-{}-{}", now_unix_ms, supervisor.events.len() + 1),
        event_type: event_type.into(),
        process_id: process_id.into(),
        occurred_at_unix_ms: now_unix_ms,
        summary: summary.into(),
    });
    supervisor.events.truncate(1024);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ReadbackEvidenceLedger;

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hepta-process-supervisor-test-{}-{}-{name}.json",
            std::process::id(),
            current_unix_ms().unwrap_or(0)
        ))
    }

    #[test]
    fn process_supervisor_tracks_plan_start_logs_and_exit() {
        let path = temp_file("lifecycle");
        let supervisor = ProcessSupervisor::new(&path);
        let planned = supervisor
            .plan_process(
                "cargo test -q -p hepta-runtime",
                "/workspace/hepta-codex",
                "proc-idem-1",
            )
            .unwrap();
        assert_eq!(planned.process.status, SupervisedProcessStatus::Planned);
        let duplicate = supervisor
            .plan_process(
                "cargo test -q -p hepta-runtime",
                "/workspace/hepta-codex",
                "proc-idem-1",
            )
            .unwrap();
        assert!(duplicate.duplicate_idempotency_key);
        let started = supervisor
            .mark_started(&planned.process.process_id, 4242)
            .unwrap();
        assert_eq!(started.status, SupervisedProcessStatus::Running);
        supervisor
            .append_log_line(&planned.process.process_id, "stdout", "running tests")
            .unwrap();
        let exited = supervisor
            .mark_exited(&planned.process.process_id, 0, "rb-process-1")
            .unwrap();
        assert_eq!(exited.status, SupervisedProcessStatus::Exited);
        let cleanup = supervisor
            .mark_cleanup_ready(&planned.process.process_id)
            .unwrap();
        assert_eq!(cleanup.status, SupervisedProcessStatus::CleanupReady);
        let report = supervisor.report(None).unwrap();
        assert_eq!(report.cleanup_ready_count, 1);
        assert_eq!(
            report.supervisor.processes[0].stdout_tail,
            vec!["running tests"]
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn process_supervisor_rejects_invalid_transitions_and_streams() {
        let path = temp_file("invalid");
        let supervisor = ProcessSupervisor::new(&path);
        let planned = supervisor
            .plan_process("echo hello", "/tmp", "proc-idem-invalid")
            .unwrap();
        assert!(
            supervisor
                .mark_exited(&planned.process.process_id, 0, "rb-too-soon")
                .is_err()
        );
        supervisor
            .mark_started(&planned.process.process_id, 123)
            .unwrap();
        assert!(
            supervisor
                .append_log_line(&planned.process.process_id, "stdin", "not-supported")
                .is_err()
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn process_supervisor_gated_start_handoff_records_readback_without_spawning() {
        let path = temp_file("start-handoff");
        let ledger_path = temp_file("start-handoff-ledger");
        let supervisor = ProcessSupervisor::new(&path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let planned = supervisor
            .plan_process(
                "cargo test -q -p hepta-runtime --lib",
                "/workspace/hepta-codex",
                "proc-idem-start-handoff",
            )
            .unwrap();
        let unconfirmed = ProcessStartHandoffInput {
            policy_decision: "approved-exec".into(),
            operator_confirmed: false,
            idempotency_key: "proc-start-handoff-idem".into(),
        };
        assert!(
            supervisor
                .gated_start_handoff(&ledger, &planned.process.process_id, unconfirmed)
                .is_err()
        );
        let denied = ProcessStartHandoffInput {
            policy_decision: "deny-exec".into(),
            operator_confirmed: true,
            idempotency_key: "proc-start-handoff-idem".into(),
        };
        assert!(
            supervisor
                .gated_start_handoff(&ledger, &planned.process.process_id, denied)
                .is_err()
        );
        let confirmed = ProcessStartHandoffInput {
            policy_decision: "allow-exec-start".into(),
            operator_confirmed: true,
            idempotency_key: "proc-start-handoff-idem".into(),
        };
        let report = supervisor
            .gated_start_handoff(&ledger, &planned.process.process_id, confirmed.clone())
            .expect("confirmed process start handoff should record");
        assert!(report.supervisor_mutated_by_gate);
        assert!(!report.native_process_spawned_by_gate);
        assert!(!report.stdin_written_by_gate);
        assert!(!report.signal_sent_by_gate);
        assert_eq!(report.process_status, SupervisedProcessStatus::Planned);
        assert_eq!(report.handoff.process_id, planned.process.process_id);
        assert!(report.handoff.readback_evidence_id.starts_with("rb-"));
        let duplicate = supervisor
            .gated_start_handoff(&ledger, &planned.process.process_id, confirmed)
            .expect("duplicate process start handoff should be idempotent");
        assert!(duplicate.duplicate_idempotency_key);
        assert!(!duplicate.supervisor_mutated_by_gate);
        let supervisor_report = supervisor.report(None).unwrap();
        assert_eq!(supervisor_report.planned_count, 1);
        assert_eq!(supervisor_report.supervisor.start_handoffs.len(), 1);
        assert!(supervisor_report.supervisor.events.iter().any(|event| {
            event.event_type == "process_start_handoff_recorded"
                && event.summary.contains("no native spawn")
        }));
        let ledger_report = ledger.report(None).unwrap();
        assert_eq!(ledger_report.evidence_count, 1);
        assert_eq!(
            ledger_report.ledger.entries[0].subject_kind,
            "process_start_handoff"
        );
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn process_supervisor_executes_confirmed_start_handoff_with_bounded_readback() {
        let path = temp_file("start-execution");
        let ledger_path = temp_file("start-execution-ledger");
        let supervisor = ProcessSupervisor::new(&path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let cwd = std::env::temp_dir();
        let planned = supervisor
            .plan_process(
                "/bin/echo hepta-process-live",
                &cwd.display().to_string(),
                "proc-idem-start-execution",
            )
            .unwrap();
        let handoff = supervisor
            .gated_start_handoff(
                &ledger,
                &planned.process.process_id,
                ProcessStartHandoffInput {
                    policy_decision: "allow-exec-start".into(),
                    operator_confirmed: true,
                    idempotency_key: "proc-start-exec-handoff-idem".into(),
                },
            )
            .unwrap();
        let unconfirmed = ProcessStartExecutionInput {
            handoff_id: handoff.handoff.handoff_id.clone(),
            policy_decision: "allow-exec-start".into(),
            operator_confirmed: false,
            idempotency_key: "proc-start-exec-idem".into(),
            timeout_ms: 1_000,
        };
        assert!(
            supervisor
                .execute_start_handoff_once(&ledger, unconfirmed)
                .is_err()
        );
        let input = ProcessStartExecutionInput {
            handoff_id: handoff.handoff.handoff_id.clone(),
            policy_decision: "approved-live-exec".into(),
            operator_confirmed: true,
            idempotency_key: "proc-start-exec-idem".into(),
            timeout_ms: 1_000,
        };
        let execution = supervisor
            .execute_start_handoff_once(&ledger, input.clone())
            .expect("approved handoff should spawn and collect readback");
        assert!(execution.supervisor_mutated_by_adapter);
        assert!(execution.native_process_spawned_by_adapter);
        assert!(!execution.stdin_written_by_adapter);
        assert!(!execution.signal_sent_by_adapter);
        assert!(!execution.timeout_killed_by_adapter);
        assert_eq!(execution.process_status, SupervisedProcessStatus::Exited);
        assert_eq!(execution.execution.exit_code, 0);
        assert!(
            execution
                .execution
                .stdout_tail
                .iter()
                .any(|line| line.contains("hepta-process-live"))
        );
        let duplicate = supervisor
            .execute_start_handoff_once(&ledger, input)
            .expect("duplicate execution should be idempotent");
        assert!(duplicate.duplicate_idempotency_key);
        assert!(!duplicate.native_process_spawned_by_adapter);
        let report = supervisor.report(None).unwrap();
        assert_eq!(report.exited_count, 1);
        assert_eq!(report.start_execution_count, 1);
        assert!(report.supervisor.start_handoffs[0].native_process_spawned_by_gate);
        assert_eq!(report.supervisor.start_executions.len(), 1);
        assert!(report.supervisor.events.iter().any(|event| {
            event.event_type == "process_start_executed"
                && event.summary.contains("exit readback evidence")
        }));
        let ledger_report = ledger.report(None).unwrap();
        assert_eq!(ledger_report.evidence_count, 2);
        assert_eq!(
            ledger_report.ledger.entries[1].subject_kind,
            "process_start_execution"
        );
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }
}
