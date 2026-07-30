use std::fs;
use std::path::PathBuf;

use hepta_core::HeptaError;
use serde::Deserialize;
use serde::Serialize;

use crate::current_unix_ms;
use crate::delivery_queue::ReadbackEvidenceLedger;

pub const DEFAULT_AGENT_HARNESS_LEDGER_PATH: &str = ".hepta/agent-harness-ledger-v0.json";
pub const DEFAULT_AGENT_HARNESS_LEDGER_ID: &str = "hepta-native-agent-harness-ledger";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentHarnessKind {
    NativeSubagent,
    AcpHarness,
}

impl AgentHarnessKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::NativeSubagent => "native_subagent",
            Self::AcpHarness => "acp_harness",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentHarnessSessionClassification {
    TopLevel,
    Isolated,
    Named,
    SpawnChild,
}

impl Default for AgentHarnessSessionClassification {
    fn default() -> Self {
        Self::TopLevel
    }
}

impl AgentHarnessSessionClassification {
    pub const fn label(self) -> &'static str {
        match self {
            Self::TopLevel => "top_level",
            Self::Isolated => "isolated",
            Self::Named => "named",
            Self::SpawnChild => "spawn_child",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentHarnessRunStatus {
    Planned,
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl AgentHarnessRunStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentHarnessLedgerFile {
    pub version: u32,
    pub ledger_id: String,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
    #[serde(default)]
    pub runs: Vec<AgentHarnessRunRecord>,
    #[serde(default)]
    pub start_handoffs: Vec<AgentHarnessStartHandoffRecord>,
    #[serde(default)]
    pub local_executions: Vec<AgentHarnessLocalExecutionRecord>,
    #[serde(default)]
    pub events: Vec<AgentHarnessEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentHarnessRunRecord {
    pub run_id: String,
    pub agent_id: String,
    pub session_key: String,
    #[serde(default)]
    pub session_classification: AgentHarnessSessionClassification,
    pub harness_kind: AgentHarnessKind,
    pub status: AgentHarnessRunStatus,
    pub prompt_preview: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_run_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_session_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lineage_root_run_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_id_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subagent_model_source: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub acp_fallback_runtime_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub acp_runtime_attempt_order: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_registry_task_id: Option<String>,
    #[serde(default)]
    pub output_emitted_before_fallbacks_exhausted: bool,
    #[serde(default)]
    pub spawn_child_session: bool,
    #[serde(default)]
    pub lineage_metadata_recorded: bool,
    #[serde(default)]
    pub task_registry_mirrored: bool,
    #[serde(default)]
    pub in_process_completion_handoff: bool,
    #[serde(default)]
    pub ping_pong_chain_depth: u32,
    #[serde(default)]
    pub ping_pong_chain_cap: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readback_evidence_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_preview: Option<String>,
    pub external_process_started: bool,
    pub provider_invoked: bool,
    pub created_at_unix_ms: u64,
    pub updated_at_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentHarnessEvent {
    pub event_id: String,
    pub event_type: String,
    pub run_id: String,
    pub occurred_at_unix_ms: u64,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentHarnessStartHandoffRecord {
    pub handoff_id: String,
    pub run_id: String,
    pub agent_id: String,
    pub session_key: String,
    pub harness_kind: AgentHarnessKind,
    pub prompt_preview: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub agent_queued_by_gate: bool,
    #[serde(default)]
    pub local_agent_executed_by_gate: bool,
    pub external_process_started_by_gate: bool,
    pub provider_invoked_by_gate: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentHarnessLocalExecutionRecord {
    pub execution_id: String,
    pub handoff_id: String,
    pub run_id: String,
    pub agent_id: String,
    pub session_key: String,
    pub harness_kind: AgentHarnessKind,
    pub result_preview: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_registry_mirror_id: Option<String>,
    #[serde(default)]
    pub in_process_completion_handoff: bool,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
    pub readback_evidence_id: String,
    pub created_at_unix_ms: u64,
    pub local_agent_executed_by_adapter: bool,
    pub external_process_started_by_adapter: bool,
    pub provider_invoked_by_adapter: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentHarnessLedgerReport {
    pub ledger_path: String,
    pub ledger: AgentHarnessLedgerFile,
    pub run_count: usize,
    pub planned_count: usize,
    pub completed_count: usize,
    pub failed_count: usize,
    pub local_execution_count: usize,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentHarnessPlanReport {
    pub ledger_path: String,
    pub run: AgentHarnessRunRecord,
    pub duplicate_run_id: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentHarnessSpawnChildInput {
    pub run_id: String,
    pub agent_id: String,
    pub parent_run_id: String,
    pub parent_session_key: String,
    pub child_session_key: String,
    pub harness_kind: AgentHarnessKind,
    pub prompt: String,
    pub primary_runtime_id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub acp_fallback_runtime_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_subagent_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_agent_primary_model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_registry_task_id: Option<String>,
    pub ping_pong_chain_depth: u32,
    pub ping_pong_chain_cap: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentHarnessSpawnChildReport {
    pub ledger_path: String,
    pub run: AgentHarnessRunRecord,
    pub duplicate_run_id: bool,
    pub fallback_order: Vec<String>,
    pub selected_model_source: String,
    pub spawn_child_classification: bool,
    pub lineage_metadata_recorded: bool,
    pub task_registry_mirror_required: bool,
    pub in_process_dispatch_required: bool,
    pub output_emitted_before_fallbacks_exhausted: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentHarnessTransitionReport {
    pub ledger_path: String,
    pub run_id: String,
    pub status: AgentHarnessRunStatus,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentHarnessStartHandoffInput {
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentHarnessStartHandoffReport {
    pub ledger_path: String,
    pub evidence_ledger_path: String,
    pub run_status: AgentHarnessRunStatus,
    pub handoff: AgentHarnessStartHandoffRecord,
    pub duplicate_idempotency_key: bool,
    pub ledger_mutated_by_gate: bool,
    pub agent_queued_by_gate: bool,
    pub external_process_started_by_gate: bool,
    pub provider_invoked_by_gate: bool,
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentHarnessLocalExecutionInput {
    pub handoff_id: String,
    pub policy_decision: String,
    pub operator_confirmed: bool,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AgentHarnessLocalExecutionReport {
    pub ledger_path: String,
    pub evidence_ledger_path: String,
    pub run_status: AgentHarnessRunStatus,
    pub execution: AgentHarnessLocalExecutionRecord,
    pub duplicate_idempotency_key: bool,
    pub ledger_mutated_by_adapter: bool,
    pub local_agent_executed_by_adapter: bool,
    pub external_process_started_by_adapter: bool,
    pub provider_invoked_by_adapter: bool,
    pub persisted: bool,
}

pub struct AgentHarnessLedger {
    path: PathBuf,
}

impl AgentHarnessLedger {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn default_in_current_dir() -> Result<Self, HeptaError> {
        Ok(Self::new(crate::default_state_path(
            DEFAULT_AGENT_HARNESS_LEDGER_PATH,
        )?))
    }

    pub fn path_display(&self) -> String {
        self.path.display().to_string()
    }

    pub fn report(&self, now_unix_ms: Option<u64>) -> Result<AgentHarnessLedgerReport, HeptaError> {
        let now = now_unix_ms.unwrap_or(current_unix_ms()?);
        let ledger = self.load_or_default(now)?;
        Ok(AgentHarnessLedgerReport {
            ledger_path: self.path_display(),
            run_count: ledger.runs.len(),
            planned_count: count_status(&ledger, AgentHarnessRunStatus::Planned),
            completed_count: count_status(&ledger, AgentHarnessRunStatus::Completed),
            failed_count: count_status(&ledger, AgentHarnessRunStatus::Failed),
            local_execution_count: ledger.local_executions.len(),
            persisted: self.path.exists(),
            ledger,
        })
    }

    pub fn plan_run(
        &self,
        run_id: &str,
        agent_id: &str,
        session_key: &str,
        harness_kind: AgentHarnessKind,
        prompt: &str,
        resume_session_id: Option<&str>,
    ) -> Result<AgentHarnessPlanReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut ledger = self.load_or_default(now)?;
        let run_id = normalize_scoped_id(run_id, "run id")?;
        let agent_id = normalize_scoped_id(agent_id, "agent id")?;
        let session_key = normalize_scoped_id(session_key, "session key")?;
        let prompt_preview = redact_preview(&normalize_non_empty(prompt, "prompt")?);
        let resume_session_id = resume_session_id
            .map(|value| normalize_scoped_id(value, "resume session id"))
            .transpose()?;
        if let Some(existing) = ledger.runs.iter().find(|run| run.run_id == run_id).cloned() {
            return Ok(AgentHarnessPlanReport {
                ledger_path: self.path_display(),
                run: existing,
                duplicate_run_id: true,
                persisted: self.path.exists(),
            });
        }
        let run = AgentHarnessRunRecord {
            run_id: run_id.clone(),
            agent_id,
            session_key,
            session_classification: AgentHarnessSessionClassification::TopLevel,
            harness_kind,
            status: AgentHarnessRunStatus::Planned,
            prompt_preview,
            parent_run_id: None,
            parent_session_key: None,
            lineage_root_run_id: None,
            runtime_id: None,
            runtime_id_source: None,
            selected_model: None,
            subagent_model_source: None,
            acp_fallback_runtime_ids: Vec::new(),
            acp_runtime_attempt_order: Vec::new(),
            resume_session_id,
            task_registry_task_id: None,
            output_emitted_before_fallbacks_exhausted: false,
            spawn_child_session: false,
            lineage_metadata_recorded: false,
            task_registry_mirrored: false,
            in_process_completion_handoff: false,
            ping_pong_chain_depth: 0,
            ping_pong_chain_cap: 0,
            readback_evidence_id: None,
            result_preview: None,
            external_process_started: false,
            provider_invoked: false,
            created_at_unix_ms: now,
            updated_at_unix_ms: now,
        };
        ledger.runs.push(run.clone());
        push_event(
            &mut ledger,
            "run_planned",
            &run_id,
            now,
            "agent harness run planned locally; no ACP/subagent process started",
        );
        self.save(&mut ledger, now)?;
        Ok(AgentHarnessPlanReport {
            ledger_path: self.path_display(),
            run,
            duplicate_run_id: false,
            persisted: true,
        })
    }

    pub fn plan_spawn_child_run(
        &self,
        input: AgentHarnessSpawnChildInput,
    ) -> Result<AgentHarnessSpawnChildReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut ledger = self.load_or_default(now)?;
        let run_id = normalize_scoped_id(&input.run_id, "run id")?;
        let agent_id = normalize_scoped_id(&input.agent_id, "agent id")?;
        let parent_run_id = normalize_scoped_id(&input.parent_run_id, "parent run id")?;
        let parent_session_key =
            normalize_scoped_id(&input.parent_session_key, "parent session key")?;
        let child_session_key = normalize_scoped_id(&input.child_session_key, "child session key")?;
        let prompt_preview = redact_preview(&normalize_non_empty(&input.prompt, "prompt")?);
        let primary_runtime_id = normalize_scoped_id(&input.primary_runtime_id, "runtime id")?;
        let acp_fallback_runtime_ids =
            normalize_unique_ids(&input.acp_fallback_runtime_ids, "fallback runtime id")?;
        let acp_runtime_attempt_order =
            runtime_attempt_order(&primary_runtime_id, &acp_fallback_runtime_ids);
        let (selected_model, selected_model_source) = select_subagent_model(
            input.default_subagent_model.as_deref(),
            input.target_agent_primary_model.as_deref(),
        )?;
        let task_registry_task_id = input
            .task_registry_task_id
            .as_deref()
            .map(|value| normalize_scoped_id(value, "task registry task id"))
            .transpose()?;
        let ping_pong_chain_cap = input.ping_pong_chain_cap.clamp(1, 64);
        if input.ping_pong_chain_depth > ping_pong_chain_cap {
            return Err(HeptaError(format!(
                "spawn-child ping-pong depth {} exceeds cap {}",
                input.ping_pong_chain_depth, ping_pong_chain_cap
            )));
        }
        let parent = ledger
            .runs
            .iter()
            .find(|candidate| candidate.run_id == parent_run_id)
            .cloned()
            .ok_or_else(|| {
                HeptaError(format!(
                    "spawn-child run {run_id} requires existing parent run {parent_run_id}"
                ))
            })?;
        if parent.session_key != parent_session_key {
            return Err(HeptaError(format!(
                "spawn-child parent session mismatch for {parent_run_id}: expected {}, got {}",
                parent.session_key, parent_session_key
            )));
        }
        if let Some(existing) = ledger.runs.iter().find(|run| run.run_id == run_id).cloned() {
            return Ok(AgentHarnessSpawnChildReport {
                ledger_path: self.path_display(),
                fallback_order: existing.acp_runtime_attempt_order.clone(),
                selected_model_source: existing
                    .subagent_model_source
                    .clone()
                    .unwrap_or_else(|| "unknown".into()),
                spawn_child_classification: existing.session_classification
                    == AgentHarnessSessionClassification::SpawnChild,
                lineage_metadata_recorded: existing.lineage_metadata_recorded,
                task_registry_mirror_required: existing.task_registry_task_id.is_some(),
                in_process_dispatch_required: existing.spawn_child_session,
                output_emitted_before_fallbacks_exhausted: existing
                    .output_emitted_before_fallbacks_exhausted,
                run: existing,
                duplicate_run_id: true,
                persisted: self.path.exists(),
            });
        }
        let lineage_root_run_id = parent
            .lineage_root_run_id
            .clone()
            .unwrap_or_else(|| parent.run_id.clone());
        let run = AgentHarnessRunRecord {
            run_id: run_id.clone(),
            agent_id,
            session_key: child_session_key,
            session_classification: AgentHarnessSessionClassification::SpawnChild,
            harness_kind: input.harness_kind,
            status: AgentHarnessRunStatus::Planned,
            prompt_preview,
            parent_run_id: Some(parent_run_id.clone()),
            parent_session_key: Some(parent_session_key),
            lineage_root_run_id: Some(lineage_root_run_id),
            runtime_id: Some(primary_runtime_id),
            runtime_id_source: Some("session_lineage_metadata".into()),
            selected_model: Some(selected_model),
            subagent_model_source: Some(selected_model_source.clone()),
            acp_fallback_runtime_ids,
            acp_runtime_attempt_order,
            resume_session_id: None,
            task_registry_task_id,
            output_emitted_before_fallbacks_exhausted: false,
            spawn_child_session: true,
            lineage_metadata_recorded: true,
            task_registry_mirrored: false,
            in_process_completion_handoff: false,
            ping_pong_chain_depth: input.ping_pong_chain_depth,
            ping_pong_chain_cap,
            readback_evidence_id: None,
            result_preview: None,
            external_process_started: false,
            provider_invoked: false,
            created_at_unix_ms: now,
            updated_at_unix_ms: now,
        };
        let fallback_order = run.acp_runtime_attempt_order.clone();
        let task_registry_mirror_required = run.task_registry_task_id.is_some();
        ledger.runs.push(run.clone());
        push_event(
            &mut ledger,
            "spawn_child_planned",
            &run_id,
            now,
            "spawn-child agent run planned with lineage metadata, fallback order, model precedence, and no output emitted before fallback exhaustion",
        );
        self.save(&mut ledger, now)?;
        Ok(AgentHarnessSpawnChildReport {
            ledger_path: self.path_display(),
            run,
            duplicate_run_id: false,
            fallback_order,
            selected_model_source,
            spawn_child_classification: true,
            lineage_metadata_recorded: true,
            task_registry_mirror_required,
            in_process_dispatch_required: true,
            output_emitted_before_fallbacks_exhausted: false,
            persisted: true,
        })
    }

    pub fn mark_completed(
        &self,
        run_id: &str,
        readback_evidence_id: &str,
        result_preview: &str,
    ) -> Result<AgentHarnessTransitionReport, HeptaError> {
        self.finish_run(
            run_id,
            AgentHarnessRunStatus::Completed,
            readback_evidence_id,
            result_preview,
        )
    }

    pub fn mark_failed(
        &self,
        run_id: &str,
        readback_evidence_id: &str,
        result_preview: &str,
    ) -> Result<AgentHarnessTransitionReport, HeptaError> {
        self.finish_run(
            run_id,
            AgentHarnessRunStatus::Failed,
            readback_evidence_id,
            result_preview,
        )
    }

    pub fn gated_start_handoff(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        run_id: &str,
        input: AgentHarnessStartHandoffInput,
    ) -> Result<AgentHarnessStartHandoffReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut ledger = self.load_or_default(now)?;
        let run_id = normalize_scoped_id(run_id, "run id")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "agent harness start handoff for {run_id} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "agent harness start handoff for {run_id} requires allow/approved policy decision"
            )));
        }
        let run = ledger
            .runs
            .iter()
            .find(|candidate| candidate.run_id == run_id)
            .cloned()
            .ok_or_else(|| HeptaError(format!("agent harness run not found: {run_id}")))?;
        if let Some(existing) = ledger
            .start_handoffs
            .iter()
            .find(|handoff| handoff.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(AgentHarnessStartHandoffReport {
                ledger_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                run_status: run.status,
                handoff: existing,
                duplicate_idempotency_key: true,
                ledger_mutated_by_gate: false,
                agent_queued_by_gate: false,
                external_process_started_by_gate: false,
                provider_invoked_by_gate: false,
                persisted: self.path.exists(),
            });
        }
        if run.status != AgentHarnessRunStatus::Planned {
            return Err(HeptaError(format!(
                "agent harness start handoff for {run_id} requires planned run; current status is {}",
                run.status.label()
            )));
        }
        let handoff_id = format!(
            "agentstarthandoff-{}-{}",
            now,
            ledger.start_handoffs.len() + 1
        );
        let evidence = evidence_ledger.append(
            "agent_harness_start_handoff",
            &handoff_id,
            "queued",
            &format!(
                "agent harness start handoff queued for run {run_id}; external process/provider invocation not performed by this gate"
            ),
        )?;
        let handoff = AgentHarnessStartHandoffRecord {
            handoff_id: handoff_id.clone(),
            run_id: run_id.clone(),
            agent_id: run.agent_id.clone(),
            session_key: run.session_key.clone(),
            harness_kind: run.harness_kind,
            prompt_preview: run.prompt_preview.clone(),
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            readback_evidence_id: evidence.entry.evidence_id,
            created_at_unix_ms: now,
            agent_queued_by_gate: true,
            local_agent_executed_by_gate: false,
            external_process_started_by_gate: false,
            provider_invoked_by_gate: false,
        };
        if let Some(run) = ledger
            .runs
            .iter_mut()
            .find(|candidate| candidate.run_id == run_id)
        {
            run.status = AgentHarnessRunStatus::Queued;
            run.readback_evidence_id = Some(handoff.readback_evidence_id.clone());
            run.updated_at_unix_ms = now;
        }
        ledger.start_handoffs.push(handoff.clone());
        ledger.start_handoffs.truncate(1024);
        push_event(
            &mut ledger,
            "start_handoff_recorded",
            &run_id,
            now,
            "agent harness start handoff recorded with readback evidence; no process/provider invocation performed",
        );
        self.save(&mut ledger, now)?;
        Ok(AgentHarnessStartHandoffReport {
            ledger_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            run_status: AgentHarnessRunStatus::Queued,
            handoff,
            duplicate_idempotency_key: false,
            ledger_mutated_by_gate: true,
            agent_queued_by_gate: true,
            external_process_started_by_gate: false,
            provider_invoked_by_gate: false,
            persisted: true,
        })
    }

    pub fn execute_local_handoff(
        &self,
        evidence_ledger: &ReadbackEvidenceLedger,
        input: AgentHarnessLocalExecutionInput,
    ) -> Result<AgentHarnessLocalExecutionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut ledger = self.load_or_default(now)?;
        let handoff_id = normalize_non_empty(&input.handoff_id, "handoff id")?;
        let policy_decision = normalize_non_empty(&input.policy_decision, "policy decision")?;
        let idempotency_key = normalize_non_empty(&input.idempotency_key, "idempotency key")?;
        if !input.operator_confirmed {
            return Err(HeptaError(format!(
                "agent harness local execution for {handoff_id} requires explicit operator confirmation"
            )));
        }
        if !policy_allows_handoff(&policy_decision) {
            return Err(HeptaError(format!(
                "agent harness local execution for {handoff_id} requires allow/approved policy decision"
            )));
        }
        if let Some(existing) = ledger
            .local_executions
            .iter()
            .find(|execution| execution.idempotency_key == idempotency_key)
            .cloned()
        {
            return Ok(AgentHarnessLocalExecutionReport {
                ledger_path: self.path_display(),
                evidence_ledger_path: evidence_ledger.path_display(),
                run_status: AgentHarnessRunStatus::Completed,
                execution: existing,
                duplicate_idempotency_key: true,
                ledger_mutated_by_adapter: false,
                local_agent_executed_by_adapter: false,
                external_process_started_by_adapter: false,
                provider_invoked_by_adapter: false,
                persisted: self.path.exists(),
            });
        }
        let handoff_index = ledger
            .start_handoffs
            .iter()
            .position(|handoff| handoff.handoff_id == handoff_id)
            .ok_or_else(|| HeptaError(format!("agent harness handoff not found: {handoff_id}")))?;
        let handoff = ledger.start_handoffs[handoff_index].clone();
        if !handoff.operator_confirmed || !policy_allows_handoff(&handoff.policy_decision) {
            return Err(HeptaError(format!(
                "agent harness handoff {handoff_id} is not approved for local execution"
            )));
        }
        if handoff.local_agent_executed_by_gate {
            return Err(HeptaError(format!(
                "agent harness handoff {handoff_id} has already been locally executed"
            )));
        }
        let run_index = ledger
            .runs
            .iter()
            .position(|run| run.run_id == handoff.run_id)
            .ok_or_else(|| {
                HeptaError(format!("agent harness run not found: {}", handoff.run_id))
            })?;
        if ledger.runs[run_index].status != AgentHarnessRunStatus::Queued {
            return Err(HeptaError(format!(
                "agent harness run {} cannot execute from {}",
                handoff.run_id,
                ledger.runs[run_index].status.label()
            )));
        }
        let in_process_completion_handoff = ledger.runs[run_index].spawn_child_session;
        let task_registry_mirror_id =
            ledger.runs[run_index]
                .task_registry_task_id
                .as_ref()
                .map(|_| {
                    format!(
                        "taskregistrymirror-{}-{}",
                        now,
                        ledger.local_executions.len() + 1
                    )
                });
        let result_preview = local_agent_result_preview(&handoff);
        let execution_id = format!(
            "agentlocalexec-{}-{}",
            now,
            ledger.local_executions.len() + 1
        );
        let evidence = evidence_ledger.append(
            "agent_harness_local_execution",
            &execution_id,
            "completed",
            &format!(
                "agent harness local execution completed for run {}; external process/provider invocation not performed",
                handoff.run_id
            ),
        )?;
        let execution = AgentHarnessLocalExecutionRecord {
            execution_id: execution_id.clone(),
            handoff_id: handoff.handoff_id.clone(),
            run_id: handoff.run_id.clone(),
            agent_id: handoff.agent_id.clone(),
            session_key: handoff.session_key.clone(),
            harness_kind: handoff.harness_kind,
            result_preview: result_preview.clone(),
            task_registry_mirror_id: task_registry_mirror_id.clone(),
            in_process_completion_handoff,
            policy_decision,
            operator_confirmed: input.operator_confirmed,
            idempotency_key,
            readback_evidence_id: evidence.entry.evidence_id,
            created_at_unix_ms: now,
            local_agent_executed_by_adapter: true,
            external_process_started_by_adapter: false,
            provider_invoked_by_adapter: false,
        };
        {
            let run = &mut ledger.runs[run_index];
            run.status = AgentHarnessRunStatus::Completed;
            run.readback_evidence_id = Some(execution.readback_evidence_id.clone());
            run.result_preview = Some(result_preview);
            run.external_process_started = false;
            run.provider_invoked = false;
            if task_registry_mirror_id.is_some() {
                run.task_registry_mirrored = true;
            }
            if in_process_completion_handoff {
                run.in_process_completion_handoff = true;
            }
            run.updated_at_unix_ms = now;
        }
        ledger.start_handoffs[handoff_index].local_agent_executed_by_gate = true;
        ledger.local_executions.push(execution.clone());
        ledger.local_executions.truncate(1024);
        push_event(
            &mut ledger,
            "local_execution_completed",
            &handoff.run_id,
            now,
            if in_process_completion_handoff {
                "agent harness local execution completed with same-process completion handoff and readback evidence"
            } else {
                "agent harness local execution completed with readback evidence"
            },
        );
        self.save(&mut ledger, now)?;
        Ok(AgentHarnessLocalExecutionReport {
            ledger_path: self.path_display(),
            evidence_ledger_path: evidence_ledger.path_display(),
            run_status: AgentHarnessRunStatus::Completed,
            execution,
            duplicate_idempotency_key: false,
            ledger_mutated_by_adapter: true,
            local_agent_executed_by_adapter: true,
            external_process_started_by_adapter: false,
            provider_invoked_by_adapter: false,
            persisted: evidence.persisted,
        })
    }

    fn finish_run(
        &self,
        run_id: &str,
        status: AgentHarnessRunStatus,
        readback_evidence_id: &str,
        result_preview: &str,
    ) -> Result<AgentHarnessTransitionReport, HeptaError> {
        let now = current_unix_ms()?;
        let mut ledger = self.load_or_default(now)?;
        let run_id = normalize_scoped_id(run_id, "run id")?;
        let readback_evidence_id =
            normalize_scoped_id(readback_evidence_id, "readback evidence id")?;
        let result_preview =
            redact_preview(&normalize_non_empty(result_preview, "result preview")?);
        let run = ledger
            .runs
            .iter_mut()
            .find(|run| run.run_id == run_id)
            .ok_or_else(|| HeptaError(format!("agent harness run not found: {run_id}")))?;
        if !matches!(
            run.status,
            AgentHarnessRunStatus::Planned
                | AgentHarnessRunStatus::Queued
                | AgentHarnessRunStatus::Running
        ) {
            return Err(HeptaError(format!(
                "agent harness run {run_id} cannot finish from {}",
                run.status.label()
            )));
        }
        run.status = status;
        run.readback_evidence_id = Some(readback_evidence_id);
        run.result_preview = Some(result_preview);
        run.updated_at_unix_ms = now;
        push_event(
            &mut ledger,
            status.label(),
            &run_id,
            now,
            "agent harness run finished from local readback evidence",
        );
        self.save(&mut ledger, now)?;
        Ok(AgentHarnessTransitionReport {
            ledger_path: self.path_display(),
            run_id,
            status,
            persisted: true,
        })
    }

    fn load_or_default(&self, now_unix_ms: u64) -> Result<AgentHarnessLedgerFile, HeptaError> {
        if !self.path.exists() {
            return Ok(AgentHarnessLedgerFile {
                version: 1,
                ledger_id: DEFAULT_AGENT_HARNESS_LEDGER_ID.into(),
                created_at_unix_ms: now_unix_ms,
                updated_at_unix_ms: now_unix_ms,
                runs: Vec::new(),
                start_handoffs: Vec::new(),
                local_executions: Vec::new(),
                events: Vec::new(),
            });
        }
        let text = fs::read_to_string(&self.path).map_err(|err| {
            HeptaError(format!(
                "failed to read agent-harness ledger {}: {err}",
                self.path.display()
            ))
        })?;
        let mut ledger: AgentHarnessLedgerFile = serde_json::from_str(&text).map_err(|err| {
            HeptaError(format!(
                "failed to parse agent-harness ledger {}: {err}",
                self.path.display()
            ))
        })?;
        if ledger.version != 1 {
            return Err(HeptaError(format!(
                "unsupported agent-harness ledger version {} in {}",
                ledger.version,
                self.path.display()
            )));
        }
        ledger.events.truncate(1024);
        ledger.start_handoffs.truncate(1024);
        ledger.local_executions.truncate(1024);
        Ok(ledger)
    }

    fn save(
        &self,
        ledger: &mut AgentHarnessLedgerFile,
        now_unix_ms: u64,
    ) -> Result<(), HeptaError> {
        ledger.updated_at_unix_ms = now_unix_ms;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|err| {
                HeptaError(format!(
                    "failed to create agent-harness ledger directory {}: {err}",
                    parent.display()
                ))
            })?;
        }
        let text = serde_json::to_string_pretty(ledger).map_err(|err| {
            HeptaError(format!("failed to serialize agent-harness ledger: {err}"))
        })?;
        fs::write(&self.path, text).map_err(|err| {
            HeptaError(format!(
                "failed to write agent-harness ledger {}: {err}",
                self.path.display()
            ))
        })
    }
}

fn count_status(ledger: &AgentHarnessLedgerFile, status: AgentHarnessRunStatus) -> usize {
    ledger
        .runs
        .iter()
        .filter(|run| run.status == status)
        .count()
}

fn normalize_scoped_id(value: &str, label: &str) -> Result<String, HeptaError> {
    let value = normalize_non_empty(value, label)?;
    if value.contains('\n') || value.contains('\r') || value.contains("..") {
        return Err(HeptaError(format!(
            "agent harness {label} must be single-line and scoped"
        )));
    }
    Ok(value)
}

fn normalize_unique_ids(values: &[String], label: &str) -> Result<Vec<String>, HeptaError> {
    let mut out = Vec::new();
    for value in values {
        let value = normalize_scoped_id(value, label)?;
        if !out.iter().any(|existing| existing == &value) {
            out.push(value);
        }
    }
    Ok(out)
}

fn runtime_attempt_order(primary_runtime_id: &str, fallback_runtime_ids: &[String]) -> Vec<String> {
    let mut out = vec![primary_runtime_id.to_string()];
    for runtime_id in fallback_runtime_ids {
        if !out.iter().any(|existing| existing == runtime_id) {
            out.push(runtime_id.clone());
        }
    }
    out
}

fn select_subagent_model(
    default_subagent_model: Option<&str>,
    target_agent_primary_model: Option<&str>,
) -> Result<(String, String), HeptaError> {
    if let Some(model) = default_subagent_model {
        let model = normalize_scoped_id(model, "default subagent model")?;
        return Ok((model, "agents.defaults.subagents.model".into()));
    }
    if let Some(model) = target_agent_primary_model {
        let model = normalize_scoped_id(model, "target agent primary model")?;
        return Ok((model, "target-agent.primary_model".into()));
    }
    Ok(("hepta-local-default".into(), "hepta-local-default".into()))
}

fn normalize_non_empty(value: &str, label: &str) -> Result<String, HeptaError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(HeptaError(format!(
            "agent harness {label} must not be empty"
        )));
    }
    Ok(trimmed.to_string())
}

fn policy_allows_handoff(policy_decision: &str) -> bool {
    let policy = policy_decision.to_ascii_lowercase();
    policy.contains("allow") || policy.contains("approved")
}

fn redact_preview(value: &str) -> String {
    value
        .split_whitespace()
        .map(|part| {
            if part.len() > 56 || part.contains("token=") || part.contains("secret") {
                "<redacted>"
            } else {
                part
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn local_agent_result_preview(handoff: &AgentHarnessStartHandoffRecord) -> String {
    let mut preview = format!(
        "hepta-local-agent-result run={} agent={} kind={} prompt_chars={}",
        handoff.run_id,
        handoff.agent_id,
        handoff.harness_kind.label(),
        handoff.prompt_preview.chars().count()
    );
    if preview.chars().count() > 240 {
        preview = preview.chars().take(240).collect::<String>();
        preview.push_str("...");
    }
    preview
}

fn push_event(
    ledger: &mut AgentHarnessLedgerFile,
    event_type: &str,
    run_id: &str,
    now_unix_ms: u64,
    summary: &str,
) {
    ledger.events.push(AgentHarnessEvent {
        event_id: format!(
            "agentharnessevt-{}-{}",
            now_unix_ms,
            ledger.events.len() + 1
        ),
        event_type: event_type.into(),
        run_id: run_id.into(),
        occurred_at_unix_ms: now_unix_ms,
        summary: summary.into(),
    });
    ledger.events.truncate(1024);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_file(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "hepta-agent-harness-test-{}-{}-{name}.json",
            std::process::id(),
            current_unix_ms().unwrap_or(0)
        ))
    }

    #[test]
    fn agent_harness_plans_acp_and_native_runs_without_external_process() {
        let path = temp_file("plan");
        let ledger = AgentHarnessLedger::new(&path);
        let native = ledger
            .plan_run(
                "run-native-1",
                "native-agent",
                "session-main",
                AgentHarnessKind::NativeSubagent,
                "summarize token=secret",
                None,
            )
            .unwrap();
        assert_eq!(native.run.harness_kind, AgentHarnessKind::NativeSubagent);
        assert!(native.run.prompt_preview.contains("<redacted>"));
        assert!(!native.run.external_process_started);
        assert!(!native.run.provider_invoked);
        let acp = ledger
            .plan_run(
                "run-acp-1",
                "codex",
                "session-code",
                AgentHarnessKind::AcpHarness,
                "apply patch",
                Some("resume-1"),
            )
            .unwrap();
        assert_eq!(acp.run.resume_session_id.as_deref(), Some("resume-1"));
        let completed = ledger
            .mark_completed("run-acp-1", "rb-agent-1", "done")
            .unwrap();
        assert_eq!(completed.status, AgentHarnessRunStatus::Completed);
        let report = ledger.report(None).unwrap();
        assert_eq!(report.run_count, 2);
        assert_eq!(report.completed_count, 1);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn agent_harness_rejects_bad_ids_and_terminal_refinish() {
        let path = temp_file("reject");
        let ledger = AgentHarnessLedger::new(&path);
        assert!(
            ledger
                .plan_run(
                    "../bad",
                    "agent",
                    "session",
                    AgentHarnessKind::NativeSubagent,
                    "prompt",
                    None,
                )
                .is_err()
        );
        ledger
            .plan_run(
                "run-1",
                "agent",
                "session",
                AgentHarnessKind::NativeSubagent,
                "prompt",
                None,
            )
            .unwrap();
        ledger.mark_failed("run-1", "rb-1", "failed").unwrap();
        assert!(ledger.mark_completed("run-1", "rb-2", "done").is_err());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn agent_harness_gated_start_handoff_queues_without_process_or_provider() {
        use crate::ReadbackEvidenceLedger;

        let path = temp_file("start-handoff");
        let ledger_path = temp_file("start-handoff-ledger");
        let ledger = AgentHarnessLedger::new(&path);
        let evidence = ReadbackEvidenceLedger::new(&ledger_path);
        ledger
            .plan_run(
                "run-start-1",
                "codex",
                "session-code",
                AgentHarnessKind::AcpHarness,
                "fix issue token=secret",
                Some("resume-1"),
            )
            .unwrap();
        let input = AgentHarnessStartHandoffInput {
            policy_decision: "approved-agent-start".into(),
            operator_confirmed: true,
            idempotency_key: "agent-start-handoff-1".into(),
        };
        let unconfirmed = AgentHarnessStartHandoffInput {
            operator_confirmed: false,
            ..input.clone()
        };
        assert!(
            ledger
                .gated_start_handoff(&evidence, "run-start-1", unconfirmed)
                .is_err()
        );
        let handoff = ledger
            .gated_start_handoff(&evidence, "run-start-1", input.clone())
            .unwrap();
        assert!(handoff.agent_queued_by_gate);
        assert!(!handoff.external_process_started_by_gate);
        assert!(!handoff.provider_invoked_by_gate);
        assert_eq!(handoff.run_status, AgentHarnessRunStatus::Queued);
        let duplicate = ledger
            .gated_start_handoff(&evidence, "run-start-1", input)
            .unwrap();
        assert!(duplicate.duplicate_idempotency_key);
        assert!(!duplicate.ledger_mutated_by_gate);
        let report = ledger.report(None).unwrap();
        assert_eq!(report.ledger.start_handoffs.len(), 1);
        assert_eq!(report.ledger.runs[0].status, AgentHarnessRunStatus::Queued);
        let readback = evidence.report(None).unwrap();
        assert_eq!(readback.evidence_count, 1);
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn agent_harness_executes_local_handoff_with_completion_readback() {
        use crate::ReadbackEvidenceLedger;

        let path = temp_file("local-exec");
        let ledger_path = temp_file("local-exec-ledger");
        let ledger = AgentHarnessLedger::new(&path);
        let evidence = ReadbackEvidenceLedger::new(&ledger_path);
        ledger
            .plan_run(
                "run-local-1",
                "hepta-local-agent",
                "session-main",
                AgentHarnessKind::NativeSubagent,
                "summarize task state",
                None,
            )
            .unwrap();
        let handoff = ledger
            .gated_start_handoff(
                &evidence,
                "run-local-1",
                AgentHarnessStartHandoffInput {
                    policy_decision: "allow-agent-start".into(),
                    operator_confirmed: true,
                    idempotency_key: "agent-local-handoff-idem".into(),
                },
            )
            .unwrap();
        let unconfirmed = AgentHarnessLocalExecutionInput {
            handoff_id: handoff.handoff.handoff_id.clone(),
            policy_decision: "allow-local-agent".into(),
            operator_confirmed: false,
            idempotency_key: "agent-local-exec-idem".into(),
        };
        assert!(
            ledger
                .execute_local_handoff(&evidence, unconfirmed)
                .is_err()
        );
        let input = AgentHarnessLocalExecutionInput {
            handoff_id: handoff.handoff.handoff_id.clone(),
            policy_decision: "approved-local-agent".into(),
            operator_confirmed: true,
            idempotency_key: "agent-local-exec-idem".into(),
        };
        let execution = ledger
            .execute_local_handoff(&evidence, input.clone())
            .expect("approved local agent handoff should complete");
        assert!(execution.ledger_mutated_by_adapter);
        assert!(execution.local_agent_executed_by_adapter);
        assert!(!execution.external_process_started_by_adapter);
        assert!(!execution.provider_invoked_by_adapter);
        assert_eq!(execution.run_status, AgentHarnessRunStatus::Completed);
        assert!(
            execution
                .execution
                .result_preview
                .contains("hepta-local-agent-result")
        );
        let duplicate = ledger
            .execute_local_handoff(&evidence, input)
            .expect("duplicate local agent execution should be idempotent");
        assert!(duplicate.duplicate_idempotency_key);
        assert!(!duplicate.local_agent_executed_by_adapter);
        let report = ledger.report(None).unwrap();
        assert_eq!(report.completed_count, 1);
        assert_eq!(report.local_execution_count, 1);
        assert!(report.ledger.start_handoffs[0].local_agent_executed_by_gate);
        assert_eq!(report.ledger.local_executions.len(), 1);
        assert!(!report.ledger.runs[0].external_process_started);
        assert!(!report.ledger.runs[0].provider_invoked);
        assert!(report.ledger.events.iter().any(|event| {
            event.event_type == "local_execution_completed"
                && event.summary.contains("readback evidence")
        }));
        let readback = evidence.report(None).unwrap();
        assert_eq!(readback.evidence_count, 2);
        assert_eq!(
            readback.ledger.entries[1].subject_kind,
            "agent_harness_local_execution"
        );
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }

    #[test]
    fn agent_harness_plans_spawn_child_with_lineage_fallbacks_and_model_precedence() {
        let path = temp_file("spawn-child");
        let ledger = AgentHarnessLedger::new(&path);
        ledger
            .plan_run(
                "run-parent-1",
                "parent-agent",
                "session-parent",
                AgentHarnessKind::NativeSubagent,
                "coordinate child",
                None,
            )
            .unwrap();
        let report = ledger
            .plan_spawn_child_run(AgentHarnessSpawnChildInput {
                run_id: "run-child-1".into(),
                agent_id: "child-agent".into(),
                parent_run_id: "run-parent-1".into(),
                parent_session_key: "session-parent".into(),
                child_session_key: "session-child-1".into(),
                harness_kind: AgentHarnessKind::AcpHarness,
                prompt: "child prompt secret=abc".into(),
                primary_runtime_id: "codex-acp".into(),
                acp_fallback_runtime_ids: vec![
                    "pi-acp".into(),
                    "codex-acp".into(),
                    "local-acp".into(),
                ],
                default_subagent_model: Some("gpt-5.4-mini".into()),
                target_agent_primary_model: Some("gpt-5.4".into()),
                task_registry_task_id: Some("task-123".into()),
                ping_pong_chain_depth: 3,
                ping_pong_chain_cap: 8,
            })
            .unwrap();
        assert_eq!(
            report.run.session_classification,
            AgentHarnessSessionClassification::SpawnChild
        );
        assert!(report.spawn_child_classification);
        assert_eq!(report.run.parent_run_id.as_deref(), Some("run-parent-1"));
        assert_eq!(
            report.run.lineage_root_run_id.as_deref(),
            Some("run-parent-1")
        );
        assert_eq!(report.run.runtime_id.as_deref(), Some("codex-acp"));
        assert_eq!(
            report.run.runtime_id_source.as_deref(),
            Some("session_lineage_metadata")
        );
        assert_eq!(
            report.fallback_order,
            vec!["codex-acp", "pi-acp", "local-acp"]
        );
        assert_eq!(
            report.selected_model_source,
            "agents.defaults.subagents.model"
        );
        assert_eq!(report.run.selected_model.as_deref(), Some("gpt-5.4-mini"));
        assert!(report.lineage_metadata_recorded);
        assert!(report.task_registry_mirror_required);
        assert!(report.in_process_dispatch_required);
        assert!(!report.output_emitted_before_fallbacks_exhausted);
        let duplicate = ledger
            .plan_spawn_child_run(AgentHarnessSpawnChildInput {
                run_id: "run-child-1".into(),
                agent_id: "child-agent".into(),
                parent_run_id: "run-parent-1".into(),
                parent_session_key: "session-parent".into(),
                child_session_key: "session-child-1".into(),
                harness_kind: AgentHarnessKind::AcpHarness,
                prompt: "child prompt".into(),
                primary_runtime_id: "codex-acp".into(),
                acp_fallback_runtime_ids: vec!["pi-acp".into()],
                default_subagent_model: None,
                target_agent_primary_model: Some("gpt-5.4".into()),
                task_registry_task_id: None,
                ping_pong_chain_depth: 1,
                ping_pong_chain_cap: 8,
            })
            .unwrap();
        assert!(duplicate.duplicate_run_id);
        assert_eq!(duplicate.fallback_order, report.fallback_order);
        let too_deep = ledger.plan_spawn_child_run(AgentHarnessSpawnChildInput {
            run_id: "run-child-too-deep".into(),
            agent_id: "child-agent".into(),
            parent_run_id: "run-parent-1".into(),
            parent_session_key: "session-parent".into(),
            child_session_key: "session-child-2".into(),
            harness_kind: AgentHarnessKind::NativeSubagent,
            prompt: "child prompt".into(),
            primary_runtime_id: "local-acp".into(),
            acp_fallback_runtime_ids: Vec::new(),
            default_subagent_model: None,
            target_agent_primary_model: None,
            task_registry_task_id: None,
            ping_pong_chain_depth: 9,
            ping_pong_chain_cap: 8,
        });
        assert!(too_deep.is_err());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn agent_harness_spawn_child_completion_mirrors_task_registry_without_gateway_loopback() {
        use crate::ReadbackEvidenceLedger;

        let path = temp_file("spawn-child-complete");
        let ledger_path = temp_file("spawn-child-complete-ledger");
        let ledger = AgentHarnessLedger::new(&path);
        let evidence = ReadbackEvidenceLedger::new(&ledger_path);
        ledger
            .plan_run(
                "run-parent-2",
                "parent-agent",
                "session-parent",
                AgentHarnessKind::NativeSubagent,
                "coordinate child",
                None,
            )
            .unwrap();
        ledger
            .plan_spawn_child_run(AgentHarnessSpawnChildInput {
                run_id: "run-child-2".into(),
                agent_id: "child-agent".into(),
                parent_run_id: "run-parent-2".into(),
                parent_session_key: "session-parent".into(),
                child_session_key: "session-child-2".into(),
                harness_kind: AgentHarnessKind::NativeSubagent,
                prompt: "child prompt".into(),
                primary_runtime_id: "hepta-local".into(),
                acp_fallback_runtime_ids: vec!["backup-local".into()],
                default_subagent_model: None,
                target_agent_primary_model: Some("gpt-5.4".into()),
                task_registry_task_id: Some("task-child-2".into()),
                ping_pong_chain_depth: 2,
                ping_pong_chain_cap: 8,
            })
            .unwrap();
        let handoff = ledger
            .gated_start_handoff(
                &evidence,
                "run-child-2",
                AgentHarnessStartHandoffInput {
                    policy_decision: "allow-agent-start".into(),
                    operator_confirmed: true,
                    idempotency_key: "spawn-child-start".into(),
                },
            )
            .unwrap();
        let execution = ledger
            .execute_local_handoff(
                &evidence,
                AgentHarnessLocalExecutionInput {
                    handoff_id: handoff.handoff.handoff_id,
                    policy_decision: "approved-local-agent".into(),
                    operator_confirmed: true,
                    idempotency_key: "spawn-child-exec".into(),
                },
            )
            .unwrap();
        assert!(execution.local_agent_executed_by_adapter);
        assert!(execution.execution.in_process_completion_handoff);
        assert!(execution.execution.task_registry_mirror_id.is_some());
        assert!(!execution.external_process_started_by_adapter);
        assert!(!execution.provider_invoked_by_adapter);
        let report = ledger.report(None).unwrap();
        let child = report
            .ledger
            .runs
            .iter()
            .find(|run| run.run_id == "run-child-2")
            .unwrap();
        assert!(child.spawn_child_session);
        assert!(child.task_registry_mirrored);
        assert!(child.in_process_completion_handoff);
        assert_eq!(child.status, AgentHarnessRunStatus::Completed);
        assert!(report.ledger.events.iter().any(|event| {
            event.event_type == "local_execution_completed"
                && event.summary.contains("same-process completion handoff")
        }));
        let _ = fs::remove_file(path);
        let _ = fs::remove_file(ledger_path);
    }
}
