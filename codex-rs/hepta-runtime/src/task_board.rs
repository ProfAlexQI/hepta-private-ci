use std::fs;
use std::path::PathBuf;

use hepta_core::HeptaError;
use serde::Deserialize;
use serde::Serialize;

use crate::ChannelSendHandoffInput;
use crate::ChannelSendHandoffReport;
use crate::DurableDeliveryQueue;
use crate::ReadbackEvidenceLedger;
use crate::WorkerTaskStatus;
use crate::current_unix_ms;
use crate::task_status_label;

pub const DEFAULT_TASK_BOARD_PATH: &str = ".hepta/task-board-v0.json";
pub const DEFAULT_TASK_BOARD_ID: &str = "hepta-development-lanes";
pub const DEFAULT_TASK_CLAIM_LEASE_MS: u64 = 15 * 60 * 1000;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskBoardFile {
    pub version: u32,
    pub board_id: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default)]
    pub tasks: Vec<TaskBoardTask>,
    #[serde(default)]
    pub workers: Vec<TaskBoardWorker>,
    #[serde(default)]
    pub events: Vec<TaskBoardEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskBoardTask {
    pub task_id: String,
    pub lane: String,
    pub worker_id: String,
    pub prompt: String,
    pub status: WorkerTaskStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claimed_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lease_expires_unix_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_heartbeat_unix_ms: Option<u64>,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    pub attempt_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskBoardWorker {
    pub worker_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_task_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_heartbeat_unix_ms: Option<u64>,
    pub updated_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskBoardEvent {
    pub event_id: String,
    pub event_type: String,
    pub task_id: String,
    pub worker_id: String,
    pub occurred_at_unix_ms: u64,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskBoardReport {
    pub board_path: String,
    pub board: TaskBoardFile,
    pub task_count: usize,
    pub ready_count: usize,
    pub running_count: usize,
    pub blocked_count: usize,
    pub stale_running_count: usize,
    pub worker_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskBoardAddReport {
    pub board_path: String,
    pub task: TaskBoardTask,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskBoardClaimReport {
    pub board_path: String,
    pub worker_id: String,
    pub task: TaskBoardTask,
    pub claim_token: String,
    pub lease_expires_unix_ms: u64,
    pub stale_reclaimed_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskBoardHeartbeatReport {
    pub board_path: String,
    pub worker_id: String,
    pub task_id: String,
    pub heartbeat_unix_ms: u64,
    pub lease_expires_unix_ms: u64,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskBoardReclaimReport {
    pub board_path: String,
    pub now_unix_ms: u64,
    pub reclaimed_count: usize,
    pub reclaimed_task_ids: Vec<String>,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskBoardDiagnosticsReport {
    pub board_path: String,
    pub task_count: usize,
    pub ready_count: usize,
    pub running_count: usize,
    pub blocked_count: usize,
    pub stale_running_count: usize,
    pub zombie_worker_count: usize,
    pub dependency_blocked_task_ids: Vec<String>,
    pub stale_task_ids: Vec<String>,
    pub zombie_worker_ids: Vec<String>,
    pub healthy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskBoardTerminalDeliveryInput {
    pub target: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskBoardTerminalDeliveryReport {
    pub board_path: String,
    pub task_id: String,
    pub task_status: WorkerTaskStatus,
    pub handoff: ChannelSendHandoffReport,
    #[serde(rename = "workGraphReportOnly")]
    pub work_graph_report_only: TaskBoardWorkGraphReportOnlyEmission,
    pub queue_mutated_by_gate: bool,
    pub external_send_performed_by_gate: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskBoardWorkGraphReportOnlyEmission {
    pub task_id: String,
    pub status: String,
    pub summary: String,
    pub artifacts: Vec<String>,
    pub evidence: Vec<String>,
    pub risks: Vec<String>,
    pub next_actions: Vec<String>,
    pub verifier: String,
    pub reducer: String,
    pub usage: TaskBoardWorkGraphReportOnlyUsage,
    pub trace_id: String,
    pub span_id: String,
    pub source_surface_id: &'static str,
    pub admission_decision: &'static str,
    pub feature_flag_id: &'static str,
    pub feature_flag_enabled: bool,
    pub canary_stage: &'static str,
    pub canary_traffic_ppm: u32,
    pub readback_required: bool,
    pub rollback_replay_required: bool,
    pub blocking_guardrail_preview: bool,
    pub live_blocking_enabled: bool,
    pub live_cutover_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskBoardWorkGraphReportOnlyUsage {
    pub model_tokens: u64,
    pub tool_calls: u64,
    pub command_count: u64,
    pub budget_state: &'static str,
}

pub struct TaskBoardStore {
    path: PathBuf,
}

impl TaskBoardStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_in_current_dir() -> Result<Self, HeptaError> {
        Ok(Self::new(crate::default_state_path(
            DEFAULT_TASK_BOARD_PATH,
        )?))
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }

    pub fn report(&self, now_unix_ms: Option<u64>) -> Result<TaskBoardReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let board = self.load_or_default(now)?;
        let diagnostics = diagnose_board(&board, now);
        Ok(TaskBoardReport {
            board_path: self.path_display(),
            task_count: diagnostics.task_count,
            ready_count: diagnostics.ready_count,
            running_count: diagnostics.running_count,
            blocked_count: diagnostics.blocked_count,
            stale_running_count: diagnostics.stale_running_count,
            worker_count: board.workers.len(),
            board,
            persisted: self.path.exists(),
        })
    }

    pub fn add_task(
        &self,
        lane: &str,
        worker_id: &str,
        prompt: &str,
        depends_on: Vec<String>,
    ) -> Result<TaskBoardAddReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut board = self.load_or_default(now)?;
        let lane = normalize_non_empty(lane, "lane")?;
        let worker_id = normalize_non_empty(worker_id, "worker id")?;
        let prompt = normalize_non_empty(prompt, "prompt")?;
        let depends_on = normalize_task_ids(depends_on)?;
        validate_dependencies_exist(&board, &depends_on)?;
        let task_id = format!("tb-{}-{}", now, board.tasks.len() + 1);
        let task = TaskBoardTask {
            task_id: task_id.clone(),
            lane,
            worker_id: worker_id.clone(),
            prompt,
            status: WorkerTaskStatus::Queued,
            depends_on,
            claimed_by: None,
            claim_token: None,
            lease_expires_unix_ms: None,
            last_heartbeat_unix_ms: None,
            created_at_unix_ms: now,
            updated_at_unix_ms: now,
            attempt_count: 0,
        };
        board.tasks.push(task.clone());
        upsert_worker(&mut board, &worker_id, None, now);
        push_event(
            &mut board,
            "task_added",
            &task_id,
            &worker_id,
            now,
            "task added to persistent board",
        );
        self.save(&mut board, now)?;
        Ok(TaskBoardAddReport {
            board_path: self.path_display(),
            task,
            persisted: true,
        })
    }

    pub fn claim(
        &self,
        worker_id: &str,
        task_id: Option<&str>,
        lease_ms: Option<u64>,
    ) -> Result<TaskBoardClaimReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut board = self.load_or_default(now)?;
        let worker_id = normalize_non_empty(worker_id, "worker id")?;
        let lease_ms = lease_ms.unwrap_or(DEFAULT_TASK_CLAIM_LEASE_MS).max(1_000);
        let reclaimed = reclaim_stale_in_board(&mut board, now);
        let selected_index = select_claimable_task(&board, &worker_id, task_id, now)?;
        let lease_expires_unix_ms = now.saturating_add(lease_ms);
        let claim_token = format!(
            "claim:{}:{}:{}",
            board.tasks[selected_index].task_id, worker_id, now
        );
        {
            let task = &mut board.tasks[selected_index];
            task.status = WorkerTaskStatus::Running;
            task.claimed_by = Some(worker_id.clone());
            task.claim_token = Some(claim_token.clone());
            task.lease_expires_unix_ms = Some(lease_expires_unix_ms);
            task.last_heartbeat_unix_ms = Some(now);
            task.updated_at_unix_ms = now;
            task.attempt_count = task.attempt_count.saturating_add(1);
        }
        let task = board.tasks[selected_index].clone();
        upsert_worker(&mut board, &worker_id, Some(task.task_id.clone()), now);
        push_event(
            &mut board,
            "task_claimed",
            &task.task_id,
            &worker_id,
            now,
            "task claimed by worker",
        );
        self.save(&mut board, now)?;
        Ok(TaskBoardClaimReport {
            board_path: self.path_display(),
            worker_id,
            task,
            claim_token,
            lease_expires_unix_ms,
            stale_reclaimed_count: reclaimed.reclaimed_count,
            persisted: true,
        })
    }

    pub fn heartbeat(
        &self,
        worker_id: &str,
        task_id: &str,
        lease_ms: Option<u64>,
    ) -> Result<TaskBoardHeartbeatReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut board = self.load_or_default(now)?;
        let worker_id = normalize_non_empty(worker_id, "worker id")?;
        let task_id = normalize_non_empty(task_id, "task id")?;
        let lease_ms = lease_ms.unwrap_or(DEFAULT_TASK_CLAIM_LEASE_MS).max(1_000);
        let lease_expires_unix_ms = now.saturating_add(lease_ms);
        let task = board
            .tasks
            .iter_mut()
            .find(|candidate| candidate.task_id == task_id)
            .ok_or_else(|| HeptaError(format!("unknown task-board task: {task_id}")))?;
        if task.claimed_by.as_deref() != Some(worker_id.as_str()) {
            return Err(HeptaError(format!(
                "heartbeat denied: task {task_id} is claimed by {:?}, not {}",
                task.claimed_by, worker_id
            )));
        }
        if task.status != WorkerTaskStatus::Running {
            return Err(HeptaError(format!(
                "heartbeat denied: task {task_id} is {}",
                task_status_label(task.status)
            )));
        }
        task.last_heartbeat_unix_ms = Some(now);
        task.lease_expires_unix_ms = Some(lease_expires_unix_ms);
        task.updated_at_unix_ms = now;
        upsert_worker(&mut board, &worker_id, Some(task_id.clone()), now);
        push_event(
            &mut board,
            "task_heartbeat",
            &task_id,
            &worker_id,
            now,
            "worker heartbeat refreshed task lease",
        );
        self.save(&mut board, now)?;
        Ok(TaskBoardHeartbeatReport {
            board_path: self.path_display(),
            worker_id,
            task_id,
            heartbeat_unix_ms: now,
            lease_expires_unix_ms,
            persisted: true,
        })
    }

    pub fn complete_task(
        &self,
        worker_id: &str,
        task_id: &str,
        claim_token: &str,
        readback_evidence_id: &str,
    ) -> Result<TaskBoardTask, HeptaError> {
        self.finish_claimed_task(
            worker_id,
            task_id,
            claim_token,
            WorkerTaskStatus::Completed,
            readback_evidence_id,
            "task completed with readback evidence",
        )
    }

    pub fn fail_task(
        &self,
        worker_id: &str,
        task_id: &str,
        claim_token: &str,
        readback_evidence_id: &str,
        error_summary: &str,
    ) -> Result<TaskBoardTask, HeptaError> {
        let error_summary = normalize_non_empty(error_summary, "error summary")?;
        self.finish_claimed_task(
            worker_id,
            task_id,
            claim_token,
            WorkerTaskStatus::Failed,
            readback_evidence_id,
            &format!("task failed with readback evidence: {error_summary}"),
        )
    }

    pub fn cancel_task(
        &self,
        task_id: &str,
        operator_id: &str,
        reason: &str,
    ) -> Result<TaskBoardTask, HeptaError> {
        let now = current_unix_ms()?;
        let mut board = self.load_or_default(now)?;
        let task_id = normalize_non_empty(task_id, "task id")?;
        let operator_id = normalize_non_empty(operator_id, "operator id")?;
        let reason = normalize_non_empty(reason, "cancel reason")?;
        let task_index = board
            .tasks
            .iter()
            .position(|candidate| candidate.task_id == task_id)
            .ok_or_else(|| HeptaError(format!("unknown task-board task: {task_id}")))?;
        if matches!(
            board.tasks[task_index].status,
            WorkerTaskStatus::Completed | WorkerTaskStatus::Failed | WorkerTaskStatus::Cancelled
        ) {
            return Err(HeptaError(format!(
                "task {task_id} is already terminal: {}",
                task_status_label(board.tasks[task_index].status)
            )));
        }
        let previous_worker = board.tasks[task_index].claimed_by.clone();
        {
            let task = &mut board.tasks[task_index];
            task.status = WorkerTaskStatus::Cancelled;
            task.claimed_by = None;
            task.claim_token = None;
            task.lease_expires_unix_ms = None;
            task.last_heartbeat_unix_ms = None;
            task.updated_at_unix_ms = now;
        }
        if let Some(worker_id) = previous_worker.as_deref() {
            clear_worker_task(&mut board, worker_id, &task_id, now);
        }
        push_event(
            &mut board,
            "task_cancelled",
            &task_id,
            &operator_id,
            now,
            &format!("task cancelled by operator: {reason}"),
        );
        let task = board.tasks[task_index].clone();
        self.save(&mut board, now)?;
        Ok(task)
    }

    pub fn reclaim_stale(
        &self,
        now_unix_ms: Option<u64>,
    ) -> Result<TaskBoardReclaimReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let mut board = self.load_or_default(now)?;
        let report = reclaim_stale_in_board(&mut board, now);
        self.save(&mut board, now)?;
        Ok(TaskBoardReclaimReport {
            board_path: self.path_display(),
            persisted: true,
            ..report
        })
    }

    pub fn diagnose(
        &self,
        now_unix_ms: Option<u64>,
    ) -> Result<TaskBoardDiagnosticsReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let board = self.load_or_default(now)?;
        let mut report = diagnose_board(&board, now);
        report.board_path = self.path_display();
        Ok(report)
    }

    pub fn handoff_terminal_delivery_summary(
        &self,
        queue: &DurableDeliveryQueue,
        evidence_ledger: &ReadbackEvidenceLedger,
        task_id: &str,
        input: TaskBoardTerminalDeliveryInput,
    ) -> Result<TaskBoardTerminalDeliveryReport, HeptaError> {
        let now = current_unix_ms()?;
        let board = self.load_or_default(now)?;
        let task_id = normalize_non_empty(task_id, "task id")?;
        let task = board
            .tasks
            .iter()
            .find(|candidate| candidate.task_id == task_id)
            .ok_or_else(|| HeptaError(format!("unknown task-board task: {task_id}")))?;
        if !matches!(
            task.status,
            WorkerTaskStatus::Completed | WorkerTaskStatus::Failed | WorkerTaskStatus::Cancelled
        ) {
            return Err(HeptaError(format!(
                "task-board delivery summary requires terminal task; {} is {}",
                task.task_id,
                task_status_label(task.status)
            )));
        }
        let latest_event = latest_task_event(&board, &task_id);
        let payload_preview = task_delivery_payload_preview(task, latest_event);
        let handoff = queue.gated_channel_send_handoff(
            evidence_ledger,
            ChannelSendHandoffInput {
                delivery_kind: "task-status-delivery".into(),
                target: input.target,
                payload_preview,
                policy_decision: input.policy_decision,
                operator_confirmed: input.operator_confirmed,
                idempotency_key: input.idempotency_key,
            },
        )?;
        let work_graph_report_only =
            task_board_work_graph_report_only_emission(task, latest_event, &handoff);
        Ok(TaskBoardTerminalDeliveryReport {
            board_path: self.path_display(),
            task_id,
            task_status: task.status,
            queue_mutated_by_gate: handoff.queue_mutated_by_gate,
            external_send_performed_by_gate: handoff.external_send_performed_by_gate,
            persisted: handoff.persisted,
            work_graph_report_only,
            handoff,
        })
    }

    fn finish_claimed_task(
        &self,
        worker_id: &str,
        task_id: &str,
        claim_token: &str,
        status: WorkerTaskStatus,
        readback_evidence_id: &str,
        summary: &str,
    ) -> Result<TaskBoardTask, HeptaError> {
        if !matches!(
            status,
            WorkerTaskStatus::Completed | WorkerTaskStatus::Failed
        ) {
            return Err(HeptaError(
                "task-board finish only supports completed/failed".into(),
            ));
        }
        let now = current_unix_ms()?;
        let mut board = self.load_or_default(now)?;
        let worker_id = normalize_non_empty(worker_id, "worker id")?;
        let task_id = normalize_non_empty(task_id, "task id")?;
        let claim_token = normalize_non_empty(claim_token, "claim token")?;
        let readback_evidence_id =
            normalize_non_empty(readback_evidence_id, "readback evidence id")?;
        let task_index = board
            .tasks
            .iter()
            .position(|candidate| candidate.task_id == task_id)
            .ok_or_else(|| HeptaError(format!("unknown task-board task: {task_id}")))?;
        {
            let task = &board.tasks[task_index];
            if task.status != WorkerTaskStatus::Running {
                return Err(HeptaError(format!(
                    "task {task_id} is not running; current status is {}",
                    task_status_label(task.status)
                )));
            }
            if task.claimed_by.as_deref() != Some(worker_id.as_str()) {
                return Err(HeptaError(format!(
                    "task {task_id} is claimed by {:?}, not {worker_id}",
                    task.claimed_by
                )));
            }
            if task.claim_token.as_deref() != Some(claim_token.as_str()) {
                return Err(HeptaError(format!(
                    "task {task_id} claim token mismatch for worker {worker_id}"
                )));
            }
        }
        {
            let task = &mut board.tasks[task_index];
            task.status = status;
            task.claimed_by = None;
            task.claim_token = None;
            task.lease_expires_unix_ms = None;
            task.last_heartbeat_unix_ms = None;
            task.updated_at_unix_ms = now;
        }
        clear_worker_task(&mut board, &worker_id, &task_id, now);
        push_event(
            &mut board,
            match status {
                WorkerTaskStatus::Completed => "task_completed",
                WorkerTaskStatus::Failed => "task_failed",
                _ => "task_finished",
            },
            &task_id,
            &worker_id,
            now,
            &format!("{summary}; readback_evidence_id={readback_evidence_id}"),
        );
        let task = board.tasks[task_index].clone();
        self.save(&mut board, now)?;
        Ok(task)
    }

    fn load_or_default(&self, now_unix_ms: u64) -> Result<TaskBoardFile, HeptaError> {
        if !self.path.exists() {
            return Ok(TaskBoardFile {
                version: 1,
                board_id: DEFAULT_TASK_BOARD_ID.into(),
                created_at_unix_ms: now_unix_ms,
                updated_at_unix_ms: now_unix_ms,
                tasks: Vec::new(),
                workers: Vec::new(),
                events: Vec::new(),
            });
        }
        let text = fs::read_to_string(&self.path).map_err(|err| {
            HeptaError(format!(
                "failed to read task-board {}: {err}",
                self.path.display()
            ))
        })?;
        let mut board: TaskBoardFile = serde_json::from_str(&text).map_err(|err| {
            HeptaError(format!(
                "failed to parse task-board {}: {err}",
                self.path.display()
            ))
        })?;
        if board.version != 1 {
            return Err(HeptaError(format!(
                "unsupported task-board version {} in {}",
                board.version,
                self.path.display()
            )));
        }
        board.events.truncate(512);
        Ok(board)
    }

    fn save(&self, board: &mut TaskBoardFile, now_unix_ms: u64) -> Result<(), HeptaError> {
        board.updated_at_unix_ms = now_unix_ms;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!(
                    "failed to create task-board directory {}: {err}",
                    parent.display()
                ))
            })?;
        }
        let text = serde_json::to_string_pretty(board)
            .map_err(|err| HeptaError(format!("failed to serialize task-board: {err}")))?;
        fs::write(&self.path, text).map_err(|err| {
            HeptaError(format!(
                "failed to write task-board {}: {err}",
                self.path.display()
            ))
        })
    }
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!("task-board {label} must not be empty")));
    }
    Ok(trimmed.to_string())
}

fn normalize_task_ids(values: Vec<String>) -> Result<Vec<String>, HeptaError> {
    let mut result = Vec::new();
    for value in values {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !result.iter().any(|existing| existing == trimmed) {
            result.push(trimmed.to_string());
        }
    }
    Ok(result)
}

fn validate_dependencies_exist(
    board: &TaskBoardFile,
    depends_on: &[String],
) -> Result<(), HeptaError> {
    if let Some(missing) = depends_on
        .iter()
        .find(|dependency| !board.tasks.iter().any(|task| task.task_id == **dependency))
    {
        return Err(HeptaError(format!(
            "task-board dependency does not exist: {missing}"
        )));
    }
    Ok(())
}

fn select_claimable_task(
    board: &TaskBoardFile,
    worker_id: &str,
    requested_task_id: Option<&str>,
    now_unix_ms: u64,
) -> Result<usize, HeptaError> {
    let requested_task_id = requested_task_id
        .map(str::trim)
        .filter(|value| !value.is_empty());
    for (index, task) in board.tasks.iter().enumerate() {
        if requested_task_id.is_some_and(|requested| requested != task.task_id) {
            continue;
        }
        if task.worker_id != worker_id && task.worker_id != "any" {
            continue;
        }
        if !matches!(
            task.status,
            WorkerTaskStatus::Queued | WorkerTaskStatus::Scheduled
        ) {
            continue;
        }
        if dependencies_completed(board, task) {
            return Ok(index);
        }
    }
    if let Some(requested) = requested_task_id {
        return Err(HeptaError(format!(
            "task-board task {requested} is not claimable for worker {worker_id} at {now_unix_ms}"
        )));
    }
    Err(HeptaError(format!(
        "no claimable task-board task for worker {worker_id} at {now_unix_ms}"
    )))
}

fn dependencies_completed(board: &TaskBoardFile, task: &TaskBoardTask) -> bool {
    task.depends_on.iter().all(|dependency| {
        board.tasks.iter().any(|candidate| {
            candidate.task_id == *dependency && candidate.status == WorkerTaskStatus::Completed
        })
    })
}

fn upsert_worker(
    board: &mut TaskBoardFile,
    worker_id: &str,
    active_task_id: Option<String>,
    now_unix_ms: u64,
) {
    if let Some(worker) = board
        .workers
        .iter_mut()
        .find(|candidate| candidate.worker_id == worker_id)
    {
        if active_task_id.is_some() {
            worker.active_task_id = active_task_id;
        }
        worker.last_heartbeat_unix_ms = Some(now_unix_ms);
        worker.updated_at_unix_ms = now_unix_ms;
    } else {
        board.workers.push(TaskBoardWorker {
            worker_id: worker_id.into(),
            active_task_id,
            last_heartbeat_unix_ms: Some(now_unix_ms),
            updated_at_unix_ms: now_unix_ms,
        });
    }
}

fn clear_worker_task(board: &mut TaskBoardFile, worker_id: &str, task_id: &str, now_unix_ms: u64) {
    if let Some(worker) = board
        .workers
        .iter_mut()
        .find(|candidate| candidate.worker_id == worker_id)
    {
        if worker.active_task_id.as_deref() == Some(task_id) {
            worker.active_task_id = None;
        }
        worker.last_heartbeat_unix_ms = Some(now_unix_ms);
        worker.updated_at_unix_ms = now_unix_ms;
    }
}

fn push_event(
    board: &mut TaskBoardFile,
    event_type: &str,
    task_id: &str,
    worker_id: &str,
    now_unix_ms: u64,
    summary: &str,
) {
    board.events.push(TaskBoardEvent {
        event_id: format!("evt-{}-{}", now_unix_ms, board.events.len() + 1),
        event_type: event_type.into(),
        task_id: task_id.into(),
        worker_id: worker_id.into(),
        occurred_at_unix_ms: now_unix_ms,
        summary: summary.into(),
    });
}

fn latest_task_event<'a>(board: &'a TaskBoardFile, task_id: &str) -> Option<&'a TaskBoardEvent> {
    board
        .events
        .iter()
        .rev()
        .find(|event| event.task_id == task_id)
}

fn task_delivery_payload_preview(task: &TaskBoardTask, event: Option<&TaskBoardEvent>) -> String {
    let mut parts = vec![
        format!("task_id={}", task.task_id),
        format!("status={}", task_status_label(task.status)),
        format!("lane={}", task.lane),
        format!("worker={}", task.worker_id),
        format!("prompt={}", compact_preview(&task.prompt, 120)),
    ];
    if let Some(event) = event {
        parts.push(format!("event={}", event.event_type));
        parts.push(format!("summary={}", compact_preview(&event.summary, 160)));
    }
    parts.join("; ")
}

fn task_board_work_graph_report_only_emission(
    task: &TaskBoardTask,
    event: Option<&TaskBoardEvent>,
    handoff: &ChannelSendHandoffReport,
) -> TaskBoardWorkGraphReportOnlyEmission {
    let event_summary = event
        .map(|event| compact_preview(&event.summary, 160))
        .unwrap_or_else(|| "terminal task has no event summary".into());
    let event_ref = event
        .map(|event| event.event_id.clone())
        .unwrap_or_else(|| "event:missing".into());

    TaskBoardWorkGraphReportOnlyEmission {
        task_id: task.task_id.clone(),
        status: task_status_label(task.status).into(),
        summary: format!("task-board terminal report-only TaskResultEnvelope preview: {event_summary}"),
        artifacts: vec![format!("task-board-terminal-ref:{}", task.task_id)],
        evidence: vec![
            event_ref,
            format!("delivery_id:{}", handoff.delivery_id),
            format!("readback_evidence_id:{}", handoff.readback_evidence_id),
        ],
        risks: vec![
            "report-only emission is returned with terminal summary and is not persisted to WorkGraph event store"
                .into(),
            "external delivery remains governed by the existing gated queue handoff".into(),
        ],
        next_actions: vec![
            "project terminal event into append-only WorkGraph shadow path before live cutover".into(),
            "require approval and readback replay evidence before external delivery promotion".into(),
        ],
        verifier: "task_board_terminal_event_report_only_verifier".into(),
        reducer: "task_board_terminal_result_reducer".into(),
        usage: TaskBoardWorkGraphReportOnlyUsage {
            model_tokens: 0,
            tool_calls: 0,
            command_count: 0,
            budget_state: "not_debited_report_only",
        },
        trace_id: format!("trace-task-board-report-only-{}", task.task_id),
        span_id: format!("span-task-board-terminal-{}", task.task_id),
        source_surface_id: "hepta_runtime_task_board",
        admission_decision: "allow_report_only_no_live_blocking",
        feature_flag_id: "work_graph_task_board_non_blocking_canary",
        feature_flag_enabled: false,
        canary_stage: "shadow_0ppm_report_only",
        canary_traffic_ppm: 0,
        readback_required: true,
        rollback_replay_required: true,
        blocking_guardrail_preview: true,
        live_blocking_enabled: false,
        live_cutover_enabled: false,
    }
}

fn compact_preview(value: &str, max_chars: usize) -> String {
    let compact = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.chars().count() <= max_chars {
        compact
    } else {
        format!(
            "{}...",
            compact
                .chars()
                .take(max_chars.saturating_sub(3))
                .collect::<String>()
        )
    }
}

fn reclaim_stale_in_board(board: &mut TaskBoardFile, now_unix_ms: u64) -> TaskBoardReclaimReport {
    let mut reclaimed_task_ids = Vec::new();
    for task in &mut board.tasks {
        let stale = task.status == WorkerTaskStatus::Running
            && task
                .lease_expires_unix_ms
                .is_some_and(|expires| expires <= now_unix_ms);
        if stale {
            reclaimed_task_ids.push(task.task_id.clone());
            task.status = WorkerTaskStatus::Queued;
            task.claimed_by = None;
            task.claim_token = None;
            task.lease_expires_unix_ms = None;
            task.last_heartbeat_unix_ms = None;
            task.updated_at_unix_ms = now_unix_ms;
        }
    }
    for task_id in &reclaimed_task_ids {
        for worker in &mut board.workers {
            if worker.active_task_id.as_deref() == Some(task_id.as_str()) {
                worker.active_task_id = None;
                worker.updated_at_unix_ms = now_unix_ms;
            }
        }
    }
    let reclaimed_count = reclaimed_task_ids.len();
    for task_id in &reclaimed_task_ids {
        push_event(
            board,
            "task_reclaimed",
            task_id,
            "task-board",
            now_unix_ms,
            "stale task claim reclaimed",
        );
    }
    TaskBoardReclaimReport {
        board_path: String::new(),
        now_unix_ms,
        reclaimed_count,
        reclaimed_task_ids,
        persisted: false,
    }
}

fn diagnose_board(board: &TaskBoardFile, now_unix_ms: u64) -> TaskBoardDiagnosticsReport {
    let mut ready_count = 0;
    let mut running_count = 0;
    let mut blocked_count = 0;
    let mut stale_task_ids = Vec::new();
    let mut dependency_blocked_task_ids = Vec::new();

    for task in &board.tasks {
        match task.status {
            WorkerTaskStatus::Queued | WorkerTaskStatus::Scheduled => {
                if dependencies_completed(board, task) {
                    ready_count += 1;
                } else {
                    blocked_count += 1;
                    dependency_blocked_task_ids.push(task.task_id.clone());
                }
            }
            WorkerTaskStatus::Running => {
                running_count += 1;
                if task
                    .lease_expires_unix_ms
                    .is_some_and(|expires| expires <= now_unix_ms)
                {
                    stale_task_ids.push(task.task_id.clone());
                }
            }
            WorkerTaskStatus::Paused => blocked_count += 1,
            WorkerTaskStatus::Completed
            | WorkerTaskStatus::Failed
            | WorkerTaskStatus::Cancelled
            | WorkerTaskStatus::Interrupted => {}
        }
    }

    let zombie_worker_ids = board
        .workers
        .iter()
        .filter(|worker| {
            worker.active_task_id.as_ref().is_some_and(|task_id| {
                !board.tasks.iter().any(|task| {
                    task.task_id == *task_id
                        && task.status == WorkerTaskStatus::Running
                        && task.claimed_by.as_deref() == Some(worker.worker_id.as_str())
                })
            })
        })
        .map(|worker| worker.worker_id.clone())
        .collect::<Vec<_>>();

    let stale_running_count = stale_task_ids.len();
    let zombie_worker_count = zombie_worker_ids.len();
    TaskBoardDiagnosticsReport {
        board_path: String::new(),
        task_count: board.tasks.len(),
        ready_count,
        running_count,
        blocked_count,
        stale_running_count,
        zombie_worker_count,
        dependency_blocked_task_ids,
        stale_task_ids,
        zombie_worker_ids,
        healthy: stale_running_count == 0 && zombie_worker_count == 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DurableDeliveryQueue;
    use crate::ReadbackEvidenceLedger;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    fn temp_board_path(name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should work")
            .as_nanos();
        std::env::temp_dir().join(format!("hepta-task-board-{name}-{unique}.json"))
    }

    #[test]
    fn task_board_persists_claim_heartbeat_and_reclaims_stale_task() {
        let path = temp_board_path("claim");
        let store = TaskBoardStore::new(&path);
        let added = store
            .add_task("runtime", "builder", "implement task board", Vec::new())
            .expect("task should be added");
        assert!(path.exists());
        let claimed = store
            .claim("builder", Some(&added.task.task_id), Some(1_000))
            .expect("task should be claimed");
        assert_eq!(claimed.task.status, WorkerTaskStatus::Running);
        assert_eq!(claimed.task.claimed_by.as_deref(), Some("builder"));
        let heartbeat = store
            .heartbeat("builder", &added.task.task_id, Some(2_000))
            .expect("heartbeat should extend lease");
        assert_eq!(heartbeat.task_id, added.task.task_id);
        let reclaimed = store
            .reclaim_stale(Some(heartbeat.lease_expires_unix_ms + 1))
            .expect("stale task should be reclaimed");
        assert_eq!(
            reclaimed.reclaimed_task_ids,
            vec![added.task.task_id.clone()]
        );
        let report = store
            .report(Some(heartbeat.lease_expires_unix_ms + 2))
            .unwrap();
        assert_eq!(report.ready_count, 1);
        assert_eq!(report.running_count, 0);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn task_board_diagnoses_dependency_blocks_and_zombies() {
        let path = temp_board_path("diagnose");
        let store = TaskBoardStore::new(&path);
        let first = store
            .add_task("runtime", "builder", "first", Vec::new())
            .expect("first task should add");
        let second = store
            .add_task(
                "runtime",
                "builder",
                "second",
                vec![first.task.task_id.clone()],
            )
            .expect("dependent task should add");
        let diag = store.diagnose(None).expect("diagnostics should load");
        assert_eq!(diag.ready_count, 1);
        assert_eq!(diag.blocked_count, 1);
        assert_eq!(diag.dependency_blocked_task_ids, vec![second.task.task_id]);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn task_board_completion_records_readback_and_unblocks_dependents() {
        let path = temp_board_path("complete");
        let store = TaskBoardStore::new(&path);
        let first = store
            .add_task("runtime", "builder", "first", Vec::new())
            .expect("first task should add");
        let second = store
            .add_task(
                "runtime",
                "builder",
                "second",
                vec![first.task.task_id.clone()],
            )
            .expect("dependent task should add");
        let claimed = store
            .claim("builder", Some(&first.task.task_id), Some(60_000))
            .expect("first task should be claimed");
        let completed = store
            .complete_task(
                "builder",
                &first.task.task_id,
                &claimed.claim_token,
                "rb-task-1",
            )
            .expect("claimed task should complete");
        assert_eq!(completed.status, WorkerTaskStatus::Completed);
        let report = store.report(None).expect("report should load");
        assert_eq!(report.ready_count, 1);
        assert_eq!(report.running_count, 0);
        assert!(report.board.events.iter().any(|event| {
            event.event_type == "task_completed" && event.summary.contains("rb-task-1")
        }));
        let next = store
            .claim("builder", Some(&second.task.task_id), Some(60_000))
            .expect("dependent task should be claimable after first completes");
        assert_eq!(next.task.task_id, second.task.task_id);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn task_board_terminal_delivery_summary_uses_confirmed_handoff() {
        let board_path = temp_board_path("terminal-delivery-board");
        let queue_path = temp_board_path("terminal-delivery-queue");
        let ledger_path = temp_board_path("terminal-delivery-ledger");
        let board = TaskBoardStore::new(&board_path);
        let queue = DurableDeliveryQueue::new(&queue_path);
        let ledger = ReadbackEvidenceLedger::new(&ledger_path);
        let added = board
            .add_task("lane-a", "worker-a", "run tests", vec![])
            .unwrap();
        assert!(
            board
                .handoff_terminal_delivery_summary(
                    &queue,
                    &ledger,
                    &added.task.task_id,
                    TaskBoardTerminalDeliveryInput {
                        target: "telegram:chat".into(),
                        policy_decision: "allow-send".into(),
                        operator_confirmed: true,
                        idempotency_key: "task-terminal".into(),
                    },
                )
                .is_err()
        );
        let claimed = board
            .claim("worker-a", Some(&added.task.task_id), Some(10_000))
            .unwrap();
        let completed = board
            .complete_task(
                "worker-a",
                &claimed.task.task_id,
                &claimed.claim_token,
                "rb-task-complete",
            )
            .unwrap();
        assert_eq!(completed.status, WorkerTaskStatus::Completed);
        assert!(
            board
                .handoff_terminal_delivery_summary(
                    &queue,
                    &ledger,
                    &completed.task_id,
                    TaskBoardTerminalDeliveryInput {
                        target: "telegram:chat".into(),
                        policy_decision: "allow-send".into(),
                        operator_confirmed: false,
                        idempotency_key: "task-terminal".into(),
                    },
                )
                .is_err()
        );
        let report = board
            .handoff_terminal_delivery_summary(
                &queue,
                &ledger,
                &completed.task_id,
                TaskBoardTerminalDeliveryInput {
                    target: "telegram:chat".into(),
                    policy_decision: "approved-send".into(),
                    operator_confirmed: true,
                    idempotency_key: "task-terminal".into(),
                },
            )
            .unwrap();
        assert_eq!(report.task_status, WorkerTaskStatus::Completed);
        assert!(report.queue_mutated_by_gate);
        assert!(!report.external_send_performed_by_gate);
        assert_eq!(report.work_graph_report_only.task_id, completed.task_id);
        assert_eq!(report.work_graph_report_only.status, "completed");
        assert!(
            report
                .work_graph_report_only
                .summary
                .contains("TaskResultEnvelope")
        );
        assert!(
            report
                .work_graph_report_only
                .evidence
                .iter()
                .any(|evidence| evidence.contains("readback_evidence_id:"))
        );
        assert!(report.work_graph_report_only.blocking_guardrail_preview);
        assert_eq!(
            report.work_graph_report_only.feature_flag_id,
            "work_graph_task_board_non_blocking_canary"
        );
        assert!(!report.work_graph_report_only.feature_flag_enabled);
        assert_eq!(
            report.work_graph_report_only.canary_stage,
            "shadow_0ppm_report_only"
        );
        assert_eq!(report.work_graph_report_only.canary_traffic_ppm, 0);
        assert!(report.work_graph_report_only.readback_required);
        assert!(report.work_graph_report_only.rollback_replay_required);
        assert!(!report.work_graph_report_only.live_blocking_enabled);
        assert!(!report.work_graph_report_only.live_cutover_enabled);
        let queue_report = queue.report(None).unwrap();
        assert_eq!(queue_report.queued_count, 1);
        assert_eq!(
            queue_report.queue.items[0].delivery_kind,
            "task-status-delivery"
        );
        assert!(
            queue_report.queue.items[0]
                .payload_preview
                .contains("task_id=")
        );
        assert!(
            queue_report.queue.items[0]
                .payload_preview
                .contains("status=completed")
        );
        let ledger_report = ledger.report(None).unwrap();
        assert_eq!(ledger_report.evidence_count, 1);
        let _ = fs::remove_file(board_path);
        let _ = fs::remove_file(queue_path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn task_board_rejects_finish_with_wrong_claim_token() {
        let path = temp_board_path("wrong-token");
        let store = TaskBoardStore::new(&path);
        let added = store
            .add_task("runtime", "builder", "guard token", Vec::new())
            .expect("task should add");
        store
            .claim("builder", Some(&added.task.task_id), Some(60_000))
            .expect("task should claim");
        assert!(
            store
                .complete_task("builder", &added.task.task_id, "wrong", "rb-task-wrong")
                .is_err()
        );
        let _ = fs::remove_file(path);
    }
}
