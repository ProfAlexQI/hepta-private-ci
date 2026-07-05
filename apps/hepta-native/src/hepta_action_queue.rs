//! Local action outbox model for Hepta Native.
//!
//! This is a side-effect-free staging model used by the desktop/mobile UI while
//! native delivery, Matrix-send, tool-approval, and task-registry mutations remain
//! policy-gated. It turns composer/approval intents into inspectable queue items so
//! the native client can display what would happen without executing it.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::{
    hepta_action_bridge::{
        decide_hepta_action, HeptaActionBridgeDecision, HeptaActionBridgeRequest,
        HeptaActionDisposition, MUTATION_APPROVE_TOOL_EXEC,
    },
    hepta_bridge::HeptaBridgeEventInput,
    hepta_composer::HeptaComposerPlan,
};
use serde_json::{json, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeptaActionQueueStage {
    LocalPreview,
    AwaitingConfirmation,
    PolicyBlocked,
    ConfirmedPreviewOnly,
    Dismissed,
}

impl HeptaActionQueueStage {
    pub fn label(self) -> &'static str {
        match self {
            Self::LocalPreview => "local_preview",
            Self::AwaitingConfirmation => "awaiting_confirmation",
            Self::PolicyBlocked => "policy_blocked",
            Self::ConfirmedPreviewOnly => "confirmed_preview_only",
            Self::Dismissed => "dismissed",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeptaActionQueueItem {
    pub id: String,
    pub title: String,
    pub event_kind: String,
    pub mutation_class: String,
    pub stage: HeptaActionQueueStage,
    pub created_at_ms: u64,
    pub requires_confirmation: bool,
    pub external_mutation_enabled: bool,
    pub exact_payload: Value,
    pub bridge_decision: HeptaActionBridgeDecision,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaActionDetail {
    pub item_id: String,
    pub title: String,
    pub stage: &'static str,
    pub mutation_class: String,
    pub event_kind: String,
    pub target_display: String,
    pub payload_preview: String,
    pub confirmation_summary: String,
    pub execution_guard: String,
    pub result_readback: String,
    pub redacted_evidence: String,
    pub payload_inspection_badge: String,
    pub mobile_safe: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaActionPayloadInspection {
    pub item_id: String,
    pub stage: &'static str,
    pub mutation_class: String,
    pub exact_payload_hash: String,
    pub payload_preview: String,
    pub external_mutation_enabled: bool,
    pub exact_payload_preview_required: bool,
    pub result_readback_required: bool,
    pub policy_decision_label: &'static str,
    pub safe_to_show_mobile: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HeptaActionOutboxPersistedSnapshot {
    pub revision: u64,
    pub item_count: usize,
    pub items: Vec<Value>,
    pub payload_hashes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeptaActionOutboxPersistenceReport {
    pub revision: u64,
    pub item_count: usize,
    pub serialized_item_count: usize,
    pub exact_payload_hash_count: usize,
    pub roundtrip_ok: bool,
    pub desktop_inspection_badges_visible: bool,
    pub mobile_inspection_badges_visible: bool,
    pub external_mutation_enabled: bool,
    pub live_mutation_performed: bool,
    pub readback_evidence_id: String,
}

impl HeptaActionPayloadInspection {
    pub fn badge_line(&self) -> String {
        format!(
            "{} · {} · payload={} · mobile_safe={} · external_mutation_enabled={}",
            self.mutation_class,
            self.policy_decision_label,
            self.exact_payload_hash,
            self.safe_to_show_mobile,
            self.external_mutation_enabled,
        )
    }
}

impl HeptaActionOutboxPersistedSnapshot {
    pub fn to_payload_value(&self) -> Value {
        json!({
            "revision": self.revision,
            "item_count": self.item_count,
            "payload_hashes": self.payload_hashes,
            "items": self.items,
        })
    }
}

impl HeptaActionDetail {
    pub fn operator_line(&self) -> String {
        format!(
            "{} · {} · {} · mobile_safe={} · {}",
            self.title, self.stage, self.mutation_class, self.mobile_safe, self.execution_guard,
        )
    }
}

impl HeptaActionQueueItem {
    pub fn from_composer_plan(plan: &HeptaComposerPlan) -> Self {
        let preview = plan.to_bridge_input();
        let bridge_decision = HeptaActionBridgeDecision::preview_only(
            plan.mutation_class(),
            plan.requires_confirmation(),
        );
        Self::from_preview_event(
            preview,
            plan.mutation_class(),
            plan.requires_confirmation(),
            plan.external_mutation_enabled,
            bridge_decision,
        )
    }

    pub fn blocked_tool_approval_preview(
        approval_id: impl Into<String>,
        exact_payload: Value,
        created_at_ms: u64,
    ) -> Self {
        let approval_id = approval_id.into();
        let bridge_decision = decide_hepta_action(HeptaActionBridgeRequest {
            mutation_class: MUTATION_APPROVE_TOOL_EXEC,
            requires_confirmation: true,
            external_mutation_enabled: false,
            confirmed: false,
        });
        Self {
            id: format!("approval-outbox-{approval_id}"),
            title: format!("Approval preview · {approval_id}"),
            event_kind: "approval_result".to_string(),
            mutation_class: MUTATION_APPROVE_TOOL_EXEC.to_string(),
            stage: stage_for_decision(&bridge_decision),
            created_at_ms,
            requires_confirmation: true,
            external_mutation_enabled: false,
            exact_payload,
            bridge_decision,
        }
    }

    fn from_preview_event(
        preview: HeptaBridgeEventInput,
        mutation_class: &str,
        requires_confirmation: bool,
        external_mutation_enabled: bool,
        bridge_decision: HeptaActionBridgeDecision,
    ) -> Self {
        let title = preview
            .payload
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or(&preview.fallback_body)
            .to_string();
        Self {
            id: preview.id,
            title,
            event_kind: preview.event_kind,
            mutation_class: mutation_class.to_string(),
            stage: stage_for_decision(&bridge_decision),
            created_at_ms: preview.created_at_ms,
            requires_confirmation,
            external_mutation_enabled,
            exact_payload: preview.payload,
            bridge_decision,
        }
    }

    pub fn confirm_preview_only(mut self) -> Self {
        if matches!(
            self.stage,
            HeptaActionQueueStage::AwaitingConfirmation | HeptaActionQueueStage::LocalPreview
        ) {
            self.stage = HeptaActionQueueStage::ConfirmedPreviewOnly;
        }
        self.external_mutation_enabled = false;
        self
    }

    pub fn dismiss(mut self) -> Self {
        self.stage = HeptaActionQueueStage::Dismissed;
        self.external_mutation_enabled = false;
        self
    }

    pub fn as_payload_value(&self) -> Value {
        json!({
            "id": self.id,
            "title": self.title,
            "event_kind": self.event_kind,
            "mutation_class": self.mutation_class,
            "stage": self.stage.label(),
            "created_at_ms": self.created_at_ms,
            "requires_confirmation": self.requires_confirmation,
            "external_mutation_enabled": self.external_mutation_enabled,
            "bridge_decision": self.bridge_decision.as_payload_value(),
            "exact_payload": self.exact_payload,
        })
    }

    pub fn detail(&self) -> HeptaActionDetail {
        HeptaActionDetail {
            item_id: self.id.clone(),
            title: self.title.clone(),
            stage: self.stage.label(),
            mutation_class: self.mutation_class.clone(),
            event_kind: self.event_kind.clone(),
            target_display: target_display_for_payload(&self.exact_payload),
            payload_preview: compact_payload_preview(&self.exact_payload),
            confirmation_summary: confirmation_summary_for_item(self),
            execution_guard: execution_guard_for_item(self),
            result_readback: result_readback_for_item(self),
            redacted_evidence: redacted_evidence_for_item(self),
            payload_inspection_badge: inspection_badge_for_item(self),
            mobile_safe: !self.external_mutation_enabled
                && !matches!(self.stage, HeptaActionQueueStage::ConfirmedPreviewOnly),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HeptaActionQueueSummary {
    pub local_preview: usize,
    pub awaiting_confirmation: usize,
    pub policy_blocked: usize,
    pub confirmed_preview_only: usize,
    pub dismissed: usize,
}

impl HeptaActionQueueSummary {
    pub fn pending_total(&self) -> usize {
        self.local_preview + self.awaiting_confirmation + self.policy_blocked
    }
}

pub fn summarize_action_queue(items: &[HeptaActionQueueItem]) -> HeptaActionQueueSummary {
    let mut summary = HeptaActionQueueSummary::default();
    for item in items {
        match item.stage {
            HeptaActionQueueStage::LocalPreview => summary.local_preview += 1,
            HeptaActionQueueStage::AwaitingConfirmation => summary.awaiting_confirmation += 1,
            HeptaActionQueueStage::PolicyBlocked => summary.policy_blocked += 1,
            HeptaActionQueueStage::ConfirmedPreviewOnly => summary.confirmed_preview_only += 1,
            HeptaActionQueueStage::Dismissed => summary.dismissed += 1,
        }
    }
    summary
}

pub fn sample_action_queue_items() -> Vec<HeptaActionQueueItem> {
    use crate::hepta_composer::plan_hepta_composer_command;

    let status = HeptaActionQueueItem::from_composer_plan(
        &plan_hepta_composer_command("/status session:current", 1_764_000_010_000)
            .expect("sample status command should parse"),
    );
    let task = HeptaActionQueueItem::from_composer_plan(
        &plan_hepta_composer_command(
            "/task verify mobile confirmation UX #task-mobile @main",
            1_764_000_011_000,
        )
        .expect("sample task command should parse"),
    );
    let tool = HeptaActionQueueItem::from_composer_plan(
        &plan_hepta_composer_command(
            "/tool exec cargo check --manifest-path apps/hepta-native/Cargo.toml",
            1_764_000_012_000,
        )
        .expect("sample tool command should parse"),
    );
    let approval = HeptaActionQueueItem::blocked_tool_approval_preview(
        "approval-install-cargo-makepad",
        json!({
            "decision": "approve",
            "tool": "exec",
            "command": "cargo install cargo-makepad",
            "safety": "blocked until explicit external mutation gate is enabled",
        }),
        1_764_000_013_000,
    );
    vec![status, task, tool, approval]
}

pub fn selected_action_detail(items: &[HeptaActionQueueItem]) -> Option<HeptaActionDetail> {
    items
        .iter()
        .find(|item| item.stage == HeptaActionQueueStage::AwaitingConfirmation)
        .or_else(|| {
            items
                .iter()
                .find(|item| item.stage == HeptaActionQueueStage::PolicyBlocked)
        })
        .or_else(|| items.first())
        .map(HeptaActionQueueItem::detail)
}

pub fn inspect_action_payload(item: &HeptaActionQueueItem) -> HeptaActionPayloadInspection {
    let payload_preview = compact_payload_preview(&item.exact_payload);
    HeptaActionPayloadInspection {
        item_id: item.id.clone(),
        stage: item.stage.label(),
        mutation_class: item.mutation_class.clone(),
        exact_payload_hash: stable_payload_hash(&item.exact_payload),
        payload_preview,
        external_mutation_enabled: item.external_mutation_enabled,
        exact_payload_preview_required: item.bridge_decision.exact_payload_preview_required,
        result_readback_required: item.bridge_decision.result_readback_required,
        policy_decision_label: item.bridge_decision.disposition.label(),
        safe_to_show_mobile: !item.external_mutation_enabled
            && !item.bridge_decision.result_readback_required,
    }
}

pub fn inspect_action_outbox(items: &[HeptaActionQueueItem]) -> Vec<HeptaActionPayloadInspection> {
    items.iter().map(inspect_action_payload).collect()
}

pub fn persist_action_outbox_snapshot(
    items: &[HeptaActionQueueItem],
    revision: u64,
) -> HeptaActionOutboxPersistedSnapshot {
    HeptaActionOutboxPersistedSnapshot {
        revision,
        item_count: items.len(),
        items: items
            .iter()
            .map(HeptaActionQueueItem::as_payload_value)
            .collect(),
        payload_hashes: items
            .iter()
            .map(|item| stable_payload_hash(&item.exact_payload))
            .collect(),
    }
}

pub fn action_outbox_persistence_report(
    items: &[HeptaActionQueueItem],
    revision: u64,
) -> HeptaActionOutboxPersistenceReport {
    let snapshot = persist_action_outbox_snapshot(items, revision);
    let serialized = snapshot.to_payload_value().to_string();
    let parsed: Value = serde_json::from_str(&serialized).unwrap_or(Value::Null);
    let serialized_items = parsed
        .get("items")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let exact_payload_hash_count = parsed
        .get("payload_hashes")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let roundtrip_ok = serialized_items == items.len()
        && exact_payload_hash_count == items.len()
        && snapshot
            .payload_hashes
            .iter()
            .zip(items.iter())
            .all(|(hash, item)| *hash == stable_payload_hash(&item.exact_payload));
    let inspections = inspect_action_outbox(items);
    let desktop_inspection_badges_visible = !inspections.is_empty()
        && inspections
            .iter()
            .all(|inspection| inspection.badge_line().contains("payload="));
    let mobile_inspection_badges_visible = !inspections.is_empty()
        && inspections
            .iter()
            .all(|inspection| inspection.safe_to_show_mobile);
    let external_mutation_enabled = items.iter().any(|item| item.external_mutation_enabled);
    let readback_evidence_id = format!(
        "action-outbox-readback-{}",
        stable_payload_hash(&snapshot.to_payload_value())
    );

    HeptaActionOutboxPersistenceReport {
        revision,
        item_count: items.len(),
        serialized_item_count: serialized_items,
        exact_payload_hash_count,
        roundtrip_ok,
        desktop_inspection_badges_visible,
        mobile_inspection_badges_visible,
        external_mutation_enabled,
        live_mutation_performed: false,
        readback_evidence_id,
    }
}

fn stage_for_decision(decision: &HeptaActionBridgeDecision) -> HeptaActionQueueStage {
    match decision.disposition {
        HeptaActionDisposition::BlockedUntilPolicyGate => HeptaActionQueueStage::PolicyBlocked,
        _ if decision.requires_confirmation => HeptaActionQueueStage::AwaitingConfirmation,
        _ => HeptaActionQueueStage::LocalPreview,
    }
}

fn compact_payload_preview(payload: &Value) -> String {
    let mut text = prioritized_payload_preview(payload).unwrap_or_else(|| payload.to_string());
    text = text.replace("/Users/qianqi", "<home>");
    text = text.replace("\n", " ");
    const MAX_PREVIEW_CHARS: usize = 420;
    if text.chars().count() > MAX_PREVIEW_CHARS {
        let mut preview: String = text.chars().take(MAX_PREVIEW_CHARS).collect();
        preview.push_str("…");
        preview
    } else {
        text
    }
}

fn prioritized_payload_preview(payload: &Value) -> Option<String> {
    let object = payload.as_object()?;
    if object.contains_key("command_payload") {
        return Some(
            json!({
                "title": object.get("title"),
                "mutation_class": object.get("mutation_class"),
                "command_payload": object.get("command_payload"),
                "context": object.get("context"),
            })
            .to_string(),
        );
    }
    if object.contains_key("command") || object.contains_key("decision") {
        return Some(
            json!({
                "decision": object.get("decision"),
                "tool": object.get("tool"),
                "command": object.get("command"),
                "safety": object.get("safety"),
            })
            .to_string(),
        );
    }
    None
}

fn target_display_for_payload(payload: &Value) -> String {
    let Some(object) = payload.as_object() else {
        return "target=local preview queue".to_string();
    };

    let command_payload = object.get("command_payload").and_then(Value::as_object);
    let context = object.get("context").and_then(Value::as_object);
    let tool = object
        .get("tool")
        .and_then(Value::as_str)
        .or_else(|| {
            command_payload
                .and_then(|object| object.get("tool"))
                .and_then(Value::as_str)
        })
        .or_else(|| {
            command_payload
                .and_then(|object| object.get("tool_name"))
                .and_then(Value::as_str)
        });
    let command = object.get("command").and_then(Value::as_str).or_else(|| {
        command_payload
            .and_then(|object| object.get("command"))
            .and_then(Value::as_str)
    });
    let agent = context
        .and_then(|object| object.get("agents"))
        .and_then(Value::as_array)
        .and_then(|values| values.first())
        .and_then(Value::as_str);
    let task = context
        .and_then(|object| object.get("tasks"))
        .and_then(Value::as_array)
        .and_then(|values| values.first())
        .and_then(Value::as_str);

    match (tool, command, agent, task) {
        (Some(tool), _, _, _) => format!("tool draft target={tool}"),
        (_, Some(command), _, _) => format!("command target={command}"),
        (_, _, Some(agent), Some(task)) => format!("agent={agent} · task={task}"),
        (_, _, Some(agent), _) => format!("agent={agent}"),
        (_, _, _, Some(task)) => format!("task={task}"),
        _ => "target=local preview queue".to_string(),
    }
}

fn confirmation_summary_for_item(item: &HeptaActionQueueItem) -> String {
    match item.stage {
        HeptaActionQueueStage::AwaitingConfirmation => format!(
            "Exact payload is staged for local confirmation only; accepting must keep {} disabled.",
            item.mutation_class,
        ),
        HeptaActionQueueStage::PolicyBlocked => format!(
            "{} is policy-blocked until class-by-class mutation gates and result readback exist.",
            item.mutation_class,
        ),
        HeptaActionQueueStage::LocalPreview => {
            "Read-only local preview; no confirmation or external send is required.".to_string()
        }
        HeptaActionQueueStage::ConfirmedPreviewOnly => {
            "Confirmation was recorded as local evidence only; no live mutation followed."
                .to_string()
        }
        HeptaActionQueueStage::Dismissed => {
            "Draft was dismissed locally; no live mutation followed.".to_string()
        }
    }
}

fn execution_guard_for_item(item: &HeptaActionQueueItem) -> String {
    format!(
        "external_mutation_enabled={} · disposition={} · exact_payload_preview_required={}",
        item.external_mutation_enabled,
        item.bridge_decision.disposition.label(),
        item.bridge_decision.exact_payload_preview_required,
    )
}

fn result_readback_for_item(item: &HeptaActionQueueItem) -> String {
    if item.bridge_decision.result_readback_required {
        "Result readback must be captured before the UI can mark this mutation complete."
            .to_string()
    } else {
        format!(
            "No live result expected: {} remains local preview evidence only.",
            item.stage.label(),
        )
    }
}

fn redacted_evidence_for_item(item: &HeptaActionQueueItem) -> String {
    if item.bridge_decision.redacted_evidence_required {
        "Evidence lane: exact payload preview + redacted target/readback; secrets and home paths stay hidden."
            .to_string()
    } else {
        "Evidence lane disabled for this local-only item.".to_string()
    }
}

fn inspection_badge_for_item(item: &HeptaActionQueueItem) -> String {
    inspect_action_payload(item).badge_line()
}

fn stable_payload_hash(payload: &Value) -> String {
    let mut hasher = DefaultHasher::new();
    payload.to_string().hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hepta_composer::plan_hepta_composer_command;

    #[test]
    fn task_draft_enters_confirmation_queue_without_live_mutation() {
        let plan = plan_hepta_composer_command("/task close mobile M5 #task-m5", 123).unwrap();
        let item = HeptaActionQueueItem::from_composer_plan(&plan);
        assert_eq!(item.id, "composer-draft-123-task");
        assert_eq!(item.stage, HeptaActionQueueStage::AwaitingConfirmation);
        assert_eq!(item.mutation_class, "draft_task_plan");
        assert!(!item.external_mutation_enabled);
        assert_eq!(
            item.exact_payload
                .get("command_payload")
                .and_then(|value| value.get("summary"))
                .and_then(Value::as_str),
            Some("close mobile M5 #task-m5"),
        );
    }

    #[test]
    fn status_read_only_command_stays_local_preview() {
        let plan = plan_hepta_composer_command("/status session:current", 7).unwrap();
        let item = HeptaActionQueueItem::from_composer_plan(&plan);
        assert_eq!(item.stage, HeptaActionQueueStage::LocalPreview);
        assert!(!item.requires_confirmation);
        assert_eq!(item.event_kind, "runtime_event");
    }

    #[test]
    fn tool_approval_execution_remains_policy_blocked() {
        let item = HeptaActionQueueItem::blocked_tool_approval_preview(
            "approval-42",
            json!({"tool":"exec","command":"cargo check"}),
            42,
        );
        assert_eq!(item.stage, HeptaActionQueueStage::PolicyBlocked);
        assert_eq!(item.mutation_class, MUTATION_APPROVE_TOOL_EXEC);
        assert!(!item.external_mutation_enabled);
        assert!(item.bridge_decision.exact_payload_preview_required);
    }

    #[test]
    fn sample_queue_covers_safe_desktop_and_mobile_lanes() {
        let items = sample_action_queue_items();
        let summary = summarize_action_queue(&items);
        assert!(summary.local_preview >= 1);
        assert!(summary.awaiting_confirmation >= 1);
        assert!(summary.policy_blocked >= 1);
        assert!(items.iter().all(|item| !item.external_mutation_enabled));
        assert!(
            items
                .iter()
                .any(|item| item.id.contains("approval-install-cargo-makepad"))
        );
    }

    #[test]
    fn summary_counts_pending_and_terminal_preview_states() {
        let task = HeptaActionQueueItem::from_composer_plan(
            &plan_hepta_composer_command("/task close M5", 1).unwrap(),
        );
        let status = HeptaActionQueueItem::from_composer_plan(
            &plan_hepta_composer_command("/status", 2).unwrap(),
        );
        let blocked = HeptaActionQueueItem::blocked_tool_approval_preview(
            "approval-1",
            json!({"decision":"approve"}),
            3,
        );
        let dismissed = status.clone().dismiss();
        let confirmed = task.clone().confirm_preview_only();
        let summary = summarize_action_queue(&[task, status, blocked, dismissed, confirmed]);
        assert_eq!(summary.local_preview, 1);
        assert_eq!(summary.awaiting_confirmation, 1);
        assert_eq!(summary.policy_blocked, 1);
        assert_eq!(summary.dismissed, 1);
        assert_eq!(summary.confirmed_preview_only, 1);
        assert_eq!(summary.pending_total(), 3);
    }

    #[test]
    fn selected_detail_prefers_confirmation_and_compacts_payload() {
        let items = sample_action_queue_items();
        let detail = selected_action_detail(&items).expect("sample queue has a selected detail");
        assert_eq!(detail.stage, "awaiting_confirmation");
        assert_eq!(detail.mutation_class, "draft_task_plan");
        assert!(detail.mobile_safe);
        assert!(detail.target_display.contains("task=#task-mobile"));
        assert!(detail.payload_preview.contains("command_payload"));
        assert!(
            detail
                .confirmation_summary
                .contains("local confirmation only")
        );
        assert!(
            detail
                .execution_guard
                .contains("external_mutation_enabled=false")
        );
        assert!(detail.result_readback.contains("No live result expected"));
        assert!(detail.redacted_evidence.contains("Evidence lane"));
        assert!(detail.payload_inspection_badge.contains("payload="));
        assert!(detail.payload_inspection_badge.contains("mobile_safe=true"));
    }

    #[test]
    fn action_detail_extracts_operator_target_without_live_lookup() {
        let tool = sample_action_queue_items()
            .into_iter()
            .find(|item| item.mutation_class == "draft_tool_call")
            .expect("sample queue contains a tool draft")
            .detail();
        assert!(tool.target_display.contains("tool draft target"));
        assert!(!tool.target_display.contains("/Users/qianqi"));
        assert!(tool.result_readback.contains("local preview evidence"));
    }

    #[test]
    fn compact_payload_preview_redacts_home_and_bounds_length() {
        let payload = json!({
            "a_path": "/Users/qianqi/.openclaw/workspace/hepta-codex/very/long/path",
            "body": "x".repeat(500),
        });
        let preview = compact_payload_preview(&payload);
        assert!(!preview.contains("/Users/qianqi"));
        assert!(preview.contains("<home>"));
        assert!(preview.ends_with('…'));
        assert!(preview.chars().count() <= 421);
    }

    #[test]
    fn payload_inspection_hashes_exact_payload_without_enabling_mutation() {
        let items = sample_action_queue_items();
        let inspections = inspect_action_outbox(&items);
        assert_eq!(inspections.len(), items.len());
        assert!(
            inspections
                .iter()
                .all(|inspection| !inspection.external_mutation_enabled)
        );
        let approval = inspections
            .iter()
            .find(|inspection| inspection.mutation_class == MUTATION_APPROVE_TOOL_EXEC)
            .expect("approval inspection exists");
        assert_eq!(approval.policy_decision_label, "blocked_until_policy_gate");
        assert!(approval.exact_payload_preview_required);
        assert!(!approval.payload_preview.contains("/Users/qianqi"));
        assert!(approval.badge_line().contains("payload="));
    }

    #[test]
    fn persisted_outbox_roundtrips_with_payload_hash_readback() {
        let items = sample_action_queue_items();
        let snapshot = persist_action_outbox_snapshot(&items, 7);
        assert_eq!(snapshot.revision, 7);
        assert_eq!(snapshot.item_count, items.len());
        assert_eq!(snapshot.items.len(), items.len());
        assert_eq!(snapshot.payload_hashes.len(), items.len());
        assert!(
            snapshot
                .items
                .iter()
                .all(|item| item.get("exact_payload").is_some())
        );

        let report = action_outbox_persistence_report(&items, 7);
        assert_eq!(report.item_count, items.len());
        assert_eq!(report.serialized_item_count, items.len());
        assert_eq!(report.exact_payload_hash_count, items.len());
        assert!(report.roundtrip_ok);
        assert!(report.desktop_inspection_badges_visible);
        assert!(report.mobile_inspection_badges_visible);
        assert!(!report.external_mutation_enabled);
        assert!(!report.live_mutation_performed);
        assert!(
            report
                .readback_evidence_id
                .starts_with("action-outbox-readback-")
        );
    }
}
