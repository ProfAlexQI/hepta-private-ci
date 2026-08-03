use serde::Serialize;
use serde_json::{Map, Value};

pub const CONTROLLED_LIVE_TYPED_COMPAT_REPORT_IDS: &[&str] = &[
    "hepta-systems-controlled-canary-readiness-plan",
    "hepta-systems-controlled-live-operator-packet-non-send-readback",
    "hepta-systems-controlled-live-operator-packet-preview",
    "hepta-systems-controlled-live-operator-readiness-dashboard",
    "hepta-systems-controlled-live-readiness-audit",
    "hepta-systems-controlled-live-readiness-denial-readback-index",
    "hepta-systems-controlled-live-required-evidence-collection-plan",
    "hepta-systems-controlled-live-required-evidence-gap-diff-view",
    "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment",
    "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-credential-boundary-readback",
    "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-kill-switch-rehearsal-boundary-readback",
    "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-non-send-readback",
    "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-rollback-rehearsal-boundary-readback",
    "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-transport-boundary-readback",
    "hepta-systems-controlled-live-required-evidence-gap-operator-readback",
    "hepta-systems-controlled-live-required-evidence-gap-summary",
    "hepta-systems-controlled-live-required-evidence-readback-index",
];

const AUDIT_ID: &str = "hepta-systems-controlled-live-readiness-audit";
const DENIAL_ID: &str = "hepta-systems-controlled-live-readiness-denial-readback-index";
const PACKET_ID: &str = "hepta-systems-controlled-live-operator-packet-preview";
const NON_SEND_ID: &str = "hepta-systems-controlled-live-operator-packet-non-send-readback";
const COLLECTION_ID: &str = "hepta-systems-controlled-live-required-evidence-collection-plan";
const READBACK_ID: &str = "hepta-systems-controlled-live-required-evidence-readback-index";

const AUDIT_SCHEMA_V2: &str = "controlled_live_readiness_audit_v2";
const DENIAL_SCHEMA_V2: &str = "controlled_live_readiness_denial_readback_index_v2";
const PACKET_SCHEMA_V2: &str = "controlled_live_operator_packet_preview_v2";
const NON_SEND_SCHEMA_V2: &str = "controlled_live_operator_packet_non_send_readback_v2";
const COLLECTION_SCHEMA_V2: &str = "controlled_live_required_evidence_collection_plan_v2";
const READBACK_SCHEMA_V2: &str = "controlled_live_required_evidence_readback_index_v2";

const ENVELOPE_FIELDS: &[&str] = &[
    "runtime",
    "product",
    "status",
    "gate",
    "schema_version",
    "plugin_id",
    "recommended_next_gate",
    "next_actions",
    "next_migration_step",
    "local_gate",
    "architecture_note",
    "side_effect_free",
    "side_effects",
    "compatibility_migration",
    "lib_export_present",
    "production_authority_granted",
    "write_authority_granted",
    "legacy_business_fields",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ControlledLiveWorktreeObservation {
    pub status_entry_count: usize,
    pub untracked_count: usize,
    pub tracked_change_count: usize,
}

impl ControlledLiveWorktreeObservation {
    pub fn from_porcelain_v1_z(input: &[u8]) -> Result<Self, String> {
        let observed = crate::DirtyWorktreeObservation::from_porcelain_v1_z(input)?;
        let untracked_count = observed
            .entries
            .iter()
            .filter(|entry| entry.status_code == "??")
            .count();
        let status_entry_count = observed.entries.len();
        let observation = Self {
            status_entry_count,
            untracked_count,
            tracked_change_count: status_entry_count.saturating_sub(untracked_count),
        };
        observation.validate()?;
        Ok(observation)
    }

    pub const fn clean() -> Self {
        Self {
            status_entry_count: 0,
            untracked_count: 0,
            tracked_change_count: 0,
        }
    }

    pub const fn is_clean(self) -> bool {
        self.status_entry_count == 0
    }

    fn validate(self) -> Result<(), String> {
        if self.status_entry_count != self.untracked_count + self.tracked_change_count {
            return Err("controlled-live worktree observation counts do not reconcile".to_string());
        }
        Ok(())
    }
}

struct ControlledLiveCompatChain {
    audit: Value,
    denial: Value,
    packet: Value,
    non_send: Value,
    collection: Value,
    readback: Value,
}

pub fn is_controlled_live_typed_compat_report(id: &str) -> bool {
    CONTROLLED_LIVE_TYPED_COMPAT_REPORT_IDS.contains(&id)
}

pub fn controlled_live_typed_compat_report(
    id: &str,
    observation: &ControlledLiveWorktreeObservation,
) -> Result<Value, String> {
    if !is_controlled_live_typed_compat_report(id) {
        return Err(format!(
            "unknown controlled-live typed compatibility report: {id}"
        ));
    }
    observation.validate()?;
    if matches!(
        id,
        AUDIT_ID | DENIAL_ID | PACKET_ID | NON_SEND_ID | COLLECTION_ID | READBACK_ID
    ) {
        let chain = controlled_live_compat_chain(observation)?;
        return Ok(match id {
            AUDIT_ID => chain.audit,
            DENIAL_ID => chain.denial,
            PACKET_ID => chain.packet,
            NON_SEND_ID => chain.non_send,
            COLLECTION_ID => chain.collection,
            READBACK_ID => chain.readback,
            _ => {
                return Err(format!(
                    "controlled-live report id escaped the validated dynamic set: {id}"
                ));
            }
        });
    }

    let (value, ready_field) = match id {
        "hepta-systems-controlled-canary-readiness-plan" => (
            serialize(crate::controlled_canary_readiness_plan_report())?,
            "controlled_canary_readiness_plan_ready",
        ),
        "hepta-systems-controlled-live-operator-readiness-dashboard" => (
            serialize(crate::controlled_live_operator_readiness_dashboard_report())?,
            "dashboard_ready",
        ),
        "hepta-systems-controlled-live-required-evidence-gap-diff-view" => (
            serialize(crate::controlled_live_required_evidence_gap_diff_view_report())?,
            "diff_view_ready",
        ),
        "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment" => (
            serialize(
                crate::controlled_live_required_evidence_gap_operator_packet_attachment_report(),
            )?,
            "operator_packet_attachment_ready",
        ),
        "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-credential-boundary-readback" => (
            serialize(
                crate::controlled_live_required_evidence_gap_operator_packet_attachment_credential_boundary_readback_report(),
            )?,
            "credential_boundary_readback_ready",
        ),
        "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-kill-switch-rehearsal-boundary-readback" => (
            serialize(
                crate::controlled_live_required_evidence_gap_operator_packet_attachment_kill_switch_rehearsal_boundary_readback_report(),
            )?,
            "kill_switch_rehearsal_boundary_readback_ready",
        ),
        "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-non-send-readback" => (
            serialize(
                crate::controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_report(),
            )?,
            "non_send_readback_ready",
        ),
        "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-rollback-rehearsal-boundary-readback" => (
            serialize(
                crate::controlled_live_required_evidence_gap_operator_packet_attachment_rollback_rehearsal_boundary_readback_report(),
            )?,
            "rollback_rehearsal_boundary_readback_ready",
        ),
        "hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-transport-boundary-readback" => (
            serialize(
                crate::controlled_live_required_evidence_gap_operator_packet_attachment_transport_boundary_readback_report(),
            )?,
            "transport_boundary_readback_ready",
        ),
        "hepta-systems-controlled-live-required-evidence-gap-operator-readback" => (
            serialize(crate::controlled_live_required_evidence_gap_operator_readback_report())?,
            "operator_readback_ready",
        ),
        "hepta-systems-controlled-live-required-evidence-gap-summary" => (
            serialize(crate::controlled_live_required_evidence_gap_summary_report())?,
            "gap_summary_ready",
        ),
        _ => return Err(format!("missing controlled-live typed report owner: {id}")),
    };
    finalize(value, id, ready_field)
}

fn controlled_live_compat_chain(
    observation: &ControlledLiveWorktreeObservation,
) -> Result<ControlledLiveCompatChain, String> {
    let audit = controlled_live_readiness_audit_report_from_observation(observation)?;
    let denial =
        controlled_live_readiness_denial_readback_index_report_from_sources(observation, &audit)?;
    let packet = controlled_live_operator_packet_preview_report_from_sources(&denial)?;
    let non_send = controlled_live_operator_packet_non_send_readback_report_from_sources(&packet)?;
    let collection =
        controlled_live_required_evidence_collection_plan_report_from_sources(&denial, &non_send)?;
    let readback =
        controlled_live_required_evidence_readback_index_report_from_sources(&collection)?;
    Ok(ControlledLiveCompatChain {
        audit,
        denial,
        packet,
        non_send,
        collection,
        readback,
    })
}

pub fn controlled_live_readiness_audit_report_from_observation(
    observation: &ControlledLiveWorktreeObservation,
) -> Result<Value, String> {
    observation.validate()?;
    let clean = observation.is_clean();
    let mut value = serialize(crate::controlled_live_readiness_audit_report())?;
    let object = object_mut(&mut value, "controlled-live readiness audit")?;
    set_string(object, "schema_version", AUDIT_SCHEMA_V2);
    set_string(
        object,
        "compatibility_migration",
        "clean_worktree_boundary_v2",
    );
    set_bool(object, "source_plugin_lifecycle_ready", true);
    set_bool(object, "source_tool_dispatch_ready", true);
    set_bool(object, "source_workflow_adapter_ready", true);
    set_bool(object, "source_read_only_e2e_ready", true);
    set_bool(object, "source_single_source_of_truth_ready", true);
    set_bool(object, "replay_validation_metadata_present", true);
    set_bool(object, "rollback_metadata_present", true);
    set_bool(object, "live_readiness_runtime_source_present", true);
    set_string(object, "git_status_source", "actual_repository");
    set_usize(
        object,
        "git_status_entry_count",
        observation.status_entry_count,
    );
    set_usize(object, "git_untracked_count", observation.untracked_count);
    set_usize(
        object,
        "git_tracked_change_count",
        observation.tracked_change_count,
    );
    set_bool(object, "worktree_boundary_observed", true);
    set_string(
        object,
        "worktree_boundary_state",
        if clean { "clean" } else { "dirty" },
    );
    set_bool(object, "dirty_worktree_boundary_tracked", !clean);
    set_bool(object, "operator_live_approval_recorded", false);
    set_bool(object, "fresh_soak_readback_evidence_recorded", false);
    set_bool(object, "credential_boundary_attestation_recorded", false);
    set_bool(
        object,
        "gateway_native_telegram_post_boundary_approval_recorded",
        false,
    );
    set_bool(object, "rollback_rehearsal_evidence_recorded", false);
    set_bool(object, "kill_switch_rehearsal_evidence_recorded", false);

    let preconditions = array_mut(object, "preconditions", "controlled-live readiness audit")?;
    let clean_precondition = object_in_array_mut(
        preconditions,
        "id",
        "clean_worktree_required",
        "controlled-live readiness audit precondition",
    )?;
    set_bool(clean_precondition, "satisfied", clean);
    set_bool(clean_precondition, "blocks_cutover", !clean);

    let blockers = array_mut(object, "blockers", "controlled-live readiness audit")?;
    blockers.retain(|blocker| {
        !clean || blocker.get("id").and_then(Value::as_str) != Some("dirty_worktree_boundary")
    });
    let satisfied = if clean { 6 } else { 5 };
    let blocked = if clean { 6 } else { 7 };
    set_usize(object, "satisfied_precondition_count", satisfied);
    set_usize(object, "blocking_precondition_count", blocked);
    set_usize(object, "blocker_count", blocked);
    set_bool(object, "controlled_live_audit_ready", true);
    set_string(object, "status", "ready_blocked");
    add_closed_side_effects(object, &["report_written", "git_index_mutated"])?;
    finalize(value, AUDIT_ID, "controlled_live_audit_ready")
}

pub fn controlled_live_readiness_denial_readback_index_report_from_sources(
    observation: &ControlledLiveWorktreeObservation,
    audit: &Value,
) -> Result<Value, String> {
    verify_source(
        audit,
        "controlled_live_audit_ready",
        AUDIT_SCHEMA_V2,
        "controlled-live readiness audit",
    )?;
    observation.validate()?;
    let clean = observation.is_clean();
    let blocker_count = if clean { 6 } else { 7 };
    let satisfied_count = if clean { 1 } else { 0 };
    let mut value = serialize(crate::controlled_live_readiness_denial_readback_index_report())?;
    let object = object_mut(&mut value, "controlled-live denial index")?;
    object.remove("source_audit_ready");
    set_string(object, "schema_version", DENIAL_SCHEMA_V2);
    set_string(
        object,
        "compatibility_migration",
        "clean_worktree_boundary_v2",
    );
    set_bool(object, "source_controlled_live_audit_ready", true);
    set_bool(object, "source_controlled_live_cutover_ready", false);
    set_bool(object, "source_cutover_blocked", true);
    set_string(
        object,
        "source_worktree_boundary_state",
        if clean { "clean" } else { "dirty" },
    );
    set_bool(object, "source_dirty_worktree_boundary_tracked", !clean);
    set_usize(object, "source_blocker_count", blocker_count);
    set_usize(object, "source_blocking_precondition_count", blocker_count);
    set_string(object, "source_audit_status", "ready_blocked");

    let entries = array_mut(object, "entries", "controlled-live denial index")?;
    update_dynamic_entries(entries, clean, "current_state")?;
    set_usize(object, "active_blocker_count", blocker_count);
    set_usize(object, "blocking_entry_count", blocker_count);
    set_usize(object, "satisfied_entry_count", satisfied_count);
    set_bool(object, "readback_index_ready", true);
    set_string(object, "status", "ready_blocked");
    add_closed_side_effects(object, &["report_written", "git_index_mutated"])?;
    finalize(value, DENIAL_ID, "readback_index_ready")
}

pub fn controlled_live_operator_packet_preview_report_from_sources(
    denial: &Value,
) -> Result<Value, String> {
    verify_source(
        denial,
        "readback_index_ready",
        DENIAL_SCHEMA_V2,
        "controlled-live denial index",
    )?;
    let clean = source_is_clean(denial)?;
    let blocker_count = if clean { 6 } else { 7 };
    let satisfied_count = if clean { 1 } else { 0 };
    let denial_entries = required_array(denial, "entries", "controlled-live denial index")?;
    let mut value = serialize(crate::controlled_live_operator_packet_preview_report())?;
    let object = object_mut(&mut value, "controlled-live packet preview")?;
    set_string(object, "schema_version", PACKET_SCHEMA_V2);
    set_string(
        object,
        "compatibility_migration",
        "clean_worktree_boundary_v2_propagation",
    );
    set_string(
        object,
        "source_worktree_boundary_state",
        if clean { "clean" } else { "dirty" },
    );
    set_bool(object, "source_dirty_worktree_boundary_tracked", !clean);
    set_usize(object, "source_blocker_count", blocker_count);
    set_usize(object, "source_index_entry_count", 7);
    set_usize(object, "stable_blocker_slot_count", 7);
    set_usize(object, "active_blocker_count", blocker_count);
    set_usize(object, "satisfied_entry_count", satisfied_count);
    set_usize(object, "required_evidence_count", blocker_count);

    let readbacks = array_mut(
        object,
        "blocker_readbacks",
        "controlled-live packet preview",
    )?;
    for readback in readbacks {
        let readback_object = readback.as_object_mut().ok_or_else(|| {
            "controlled-live packet blocker readback must be an object".to_string()
        })?;
        let source_id = required_string(
            readback_object,
            "source_blocker_id",
            "controlled-live packet blocker readback",
        )?;
        let source = object_in_array(
            denial_entries,
            "source_blocker_id",
            source_id,
            "controlled-live denial entry",
        )?;
        copy_fields(
            readback_object,
            source,
            &[
                "current_state",
                "active_blocker",
                "evidence_required",
                "blocks_cutover",
            ],
        )?;
    }
    set_bool(object, "operator_packet_preview_ready", true);
    set_string(object, "status", "ready_blocked");
    finalize(value, PACKET_ID, "operator_packet_preview_ready")
}

pub fn controlled_live_operator_packet_non_send_readback_report_from_sources(
    packet: &Value,
) -> Result<Value, String> {
    verify_source(
        packet,
        "operator_packet_preview_ready",
        PACKET_SCHEMA_V2,
        "controlled-live packet preview",
    )?;
    let clean = source_is_clean(packet)?;
    let blocker_count = if clean { 6 } else { 7 };
    let satisfied_count = if clean { 1 } else { 0 };
    let mut value = serialize(crate::controlled_live_operator_packet_non_send_readback_report())?;
    let object = object_mut(&mut value, "controlled-live non-send readback")?;
    set_string(object, "schema_version", NON_SEND_SCHEMA_V2);
    set_string(
        object,
        "compatibility_migration",
        "clean_worktree_boundary_v2_propagation",
    );
    set_string(
        object,
        "source_worktree_boundary_state",
        if clean { "clean" } else { "dirty" },
    );
    set_bool(object, "source_dirty_worktree_boundary_tracked", !clean);
    set_usize(object, "source_blocker_count", blocker_count);
    set_usize(object, "source_blocker_readback_count", 7);
    set_usize(object, "source_active_blocker_count", blocker_count);
    set_usize(object, "source_satisfied_entry_count", satisfied_count);
    set_usize(object, "stable_blocker_slot_count", 7);
    set_bool(object, "packet_visible_to_operator", true);
    set_bool(object, "non_send_readback_ready", true);
    set_string(object, "status", "ready_blocked");
    finalize(value, NON_SEND_ID, "non_send_readback_ready")
}

pub fn controlled_live_required_evidence_collection_plan_report_from_sources(
    denial: &Value,
    non_send: &Value,
) -> Result<Value, String> {
    verify_source(
        denial,
        "readback_index_ready",
        DENIAL_SCHEMA_V2,
        "controlled-live denial index",
    )?;
    verify_source(
        non_send,
        "non_send_readback_ready",
        NON_SEND_SCHEMA_V2,
        "controlled-live non-send readback",
    )?;
    let clean = source_is_clean(denial)?;
    if clean != source_is_clean(non_send)? {
        return Err("controlled-live collection sources disagree on worktree state".to_string());
    }
    let blocker_count = if clean { 6 } else { 7 };
    let satisfied_count = if clean { 1 } else { 0 };
    let denial_entries = required_array(denial, "entries", "controlled-live denial index")?;
    let mut value = serialize(crate::controlled_live_required_evidence_collection_plan_report())?;
    let object = object_mut(&mut value, "controlled-live evidence collection plan")?;
    set_string(object, "schema_version", COLLECTION_SCHEMA_V2);
    set_string(
        object,
        "compatibility_migration",
        "clean_worktree_boundary_v2_propagation",
    );
    set_string(
        object,
        "source_worktree_boundary_state",
        if clean { "clean" } else { "dirty" },
    );
    set_bool(object, "source_dirty_worktree_boundary_tracked", !clean);
    set_usize(object, "source_blocker_count", blocker_count);
    set_usize(object, "stable_blocker_slot_count", 7);
    set_usize(object, "active_blocker_count", blocker_count);
    set_usize(object, "satisfied_entry_count", satisfied_count);
    set_usize(object, "required_evidence_count", blocker_count);

    let entries = array_mut(
        object,
        "entries",
        "controlled-live evidence collection plan",
    )?;
    for entry in entries {
        let entry_object = entry.as_object_mut().ok_or_else(|| {
            "controlled-live evidence collection entry must be an object".to_string()
        })?;
        let source_id = required_string(
            entry_object,
            "source_blocker_id",
            "controlled-live evidence collection entry",
        )?;
        let source = object_in_array(
            denial_entries,
            "source_blocker_id",
            source_id,
            "controlled-live denial entry",
        )?;
        copy_fields(
            entry_object,
            source,
            &[
                "current_state",
                "active_blocker",
                "evidence_required",
                "blocks_cutover",
            ],
        )?;
    }
    set_bool(object, "evidence_collection_plan_ready", true);
    set_string(object, "status", "ready_blocked");
    finalize(value, COLLECTION_ID, "evidence_collection_plan_ready")
}

pub fn controlled_live_required_evidence_readback_index_report_from_sources(
    plan: &Value,
) -> Result<Value, String> {
    verify_source(
        plan,
        "evidence_collection_plan_ready",
        COLLECTION_SCHEMA_V2,
        "controlled-live evidence collection plan",
    )?;
    let clean = source_is_clean(plan)?;
    let blocker_count = if clean { 6 } else { 7 };
    let satisfied_count = if clean { 1 } else { 0 };
    let plan_entries = required_array(plan, "entries", "controlled-live evidence collection plan")?;
    let mut value = serialize(crate::controlled_live_required_evidence_readback_index_report())?;
    let object = object_mut(&mut value, "controlled-live evidence readback index")?;
    set_string(object, "schema_version", READBACK_SCHEMA_V2);
    set_string(
        object,
        "compatibility_migration",
        "clean_worktree_boundary_v2_propagation",
    );
    set_string(
        object,
        "source_worktree_boundary_state",
        if clean { "clean" } else { "dirty" },
    );
    set_bool(object, "source_dirty_worktree_boundary_tracked", !clean);
    set_usize(object, "source_blocker_count", blocker_count);
    set_usize(object, "source_active_blocker_count", blocker_count);
    set_usize(object, "source_plan_entry_count", 7);
    set_usize(object, "stable_blocker_slot_count", 7);
    set_usize(object, "active_blocker_count", blocker_count);
    set_usize(object, "missing_evidence_count", blocker_count);
    set_usize(object, "satisfied_entry_count", satisfied_count);

    let entries = array_mut(object, "entries", "controlled-live evidence readback index")?;
    for entry in entries {
        let entry_object = entry.as_object_mut().ok_or_else(|| {
            "controlled-live evidence readback entry must be an object".to_string()
        })?;
        let source_id = required_string(
            entry_object,
            "source_blocker_id",
            "controlled-live evidence readback entry",
        )?;
        let source = object_in_array(
            plan_entries,
            "source_blocker_id",
            source_id,
            "controlled-live evidence collection entry",
        )?;
        copy_field_as(entry_object, source, "current_state", "evidence_state")?;
        copy_fields(
            entry_object,
            source,
            &["active_blocker", "evidence_required", "blocks_cutover"],
        )?;
    }
    set_bool(object, "readback_index_ready", true);
    set_string(object, "status", "ready_blocked");
    finalize(value, READBACK_ID, "readback_index_ready")
}

fn serialize<T: Serialize>(report: T) -> Result<Value, String> {
    serde_json::to_value(report)
        .map_err(|error| format!("cannot serialize controlled-live report: {error}"))
}

fn finalize(mut value: Value, id: &str, ready_field: &str) -> Result<Value, String> {
    let projection = legacy_business_projection(&value)?;
    let object = object_mut(&mut value, id)?;
    object.insert("product".to_string(), Value::String("Hepta".to_string()));
    object.insert("side_effect_free".to_string(), Value::Bool(true));
    object.insert(
        "production_authority_granted".to_string(),
        Value::Bool(false),
    );
    object.insert("write_authority_granted".to_string(), Value::Bool(false));
    object.insert("legacy_business_fields".to_string(), projection);
    verify_integrity(&value, id, ready_field)?;
    Ok(value)
}

fn legacy_business_projection(value: &Value) -> Result<Value, String> {
    let object = value
        .as_object()
        .ok_or_else(|| "controlled-live report must be an object".to_string())?;
    let projection = object
        .iter()
        .filter(|(key, _)| !ENVELOPE_FIELDS.contains(&key.as_str()))
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect::<Map<_, _>>();
    if projection.is_empty() {
        return Err("controlled-live business projection cannot be empty".to_string());
    }
    Ok(Value::Object(projection))
}

fn verify_integrity(value: &Value, id: &str, ready_field: &str) -> Result<(), String> {
    let object = value
        .as_object()
        .ok_or_else(|| format!("{id} must render a JSON object"))?;
    if object.get("runtime").and_then(Value::as_str) != Some("hepta") {
        return Err(format!("{id} runtime drifted"));
    }
    if object.get(ready_field) != Some(&Value::Bool(true)) {
        return Err(format!("{id} integrity flag {ready_field} is not true"));
    }
    let side_effects = object
        .get("side_effects")
        .and_then(Value::as_object)
        .ok_or_else(|| format!("{id} must expose side_effects"))?;
    if side_effects.is_empty()
        || side_effects
            .values()
            .any(|value| value != &Value::Bool(false))
    {
        return Err(format!("{id} side effects are not closed"));
    }
    if object.get("production_authority_granted") != Some(&Value::Bool(false))
        || object.get("write_authority_granted") != Some(&Value::Bool(false))
    {
        return Err(format!("{id} authority boundary is not closed"));
    }
    verify_sensitive_fields(value, id)
}

fn verify_sensitive_fields(value: &Value, context: &str) -> Result<(), String> {
    const FORBIDDEN_TRUE_FIELDS: &[&str] = &[
        "production_authority_granted",
        "write_authority_granted",
        "controlled_live_cutover_ready",
        "controlled_canary_activation_ready",
        "ready_for_live_execution",
        "live_execution_allowed",
        "live_execution_enabled",
        "live_activation_allowed",
        "activation_allowed",
        "approval_request_ready",
        "approval_request_sent",
        "approval_acceptance_ready",
        "approval_accepted",
        "approval_recorded",
        "approval_broker_write_allowed",
        "evidence_recording_allowed",
        "evidence_persisted",
        "credential_read_allowed",
        "credential_material_load_allowed",
        "credential_value_exposure_allowed",
        "credential_handle_resolution_allowed",
        "gateway_or_auth_mutation_allowed",
        "native_post_mutation_allowed",
        "telegram_transport_mutation_allowed",
        "channel_send_allowed",
        "transport_mutation_allowed",
        "persistence_allowed",
        "packet_persisted",
        "attachment_persisted",
        "readback_persisted",
        "canary_persistence_allowed",
        "canary_receipt_persisted",
        "workflow_event_log_write_allowed",
        "sqlite_write_allowed",
        "rollback_rehearsal_allowed",
        "rollback_rehearsal_executed",
        "rollback_execution_allowed",
        "rollback_executed",
        "kill_switch_rehearsal_allowed",
        "kill_switch_rehearsal_executed",
        "kill_switch_mutation_allowed",
        "kill_switch_mutated",
        "provider_invocation_allowed",
        "model_invocation_allowed",
        "package_or_release_allowed",
        "public_ga_allowed",
    ];
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                if FORBIDDEN_TRUE_FIELDS.contains(&key.as_str()) && value == &Value::Bool(true) {
                    return Err(format!("{context} unexpectedly enables {key}"));
                }
                verify_sensitive_fields(value, context)?;
            }
        }
        Value::Array(values) => {
            for value in values {
                verify_sensitive_fields(value, context)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn verify_source(
    value: &Value,
    ready_field: &str,
    schema: &str,
    context: &str,
) -> Result<(), String> {
    let object = value
        .as_object()
        .ok_or_else(|| format!("{context} source must be an object"))?;
    if object.get(ready_field) != Some(&Value::Bool(true)) {
        return Err(format!("{context} source is not ready"));
    }
    if object.get("schema_version").and_then(Value::as_str) != Some(schema) {
        return Err(format!("{context} source schema drifted"));
    }
    if object.get("production_authority_granted") != Some(&Value::Bool(false))
        || object.get("write_authority_granted") != Some(&Value::Bool(false))
    {
        return Err(format!("{context} source authority boundary is open"));
    }
    Ok(())
}

fn source_is_clean(value: &Value) -> Result<bool, String> {
    match value
        .get("source_worktree_boundary_state")
        .or_else(|| value.get("worktree_boundary_state"))
        .and_then(Value::as_str)
    {
        Some("clean") => Ok(true),
        Some("dirty") => Ok(false),
        _ => Err("controlled-live source lacks a valid worktree state".to_string()),
    }
}

fn update_dynamic_entries(
    entries: &mut [Value],
    clean: bool,
    state_field: &str,
) -> Result<(), String> {
    if entries.len() != 7 {
        return Err(
            "controlled-live dynamic entry inventory must expose seven stable slots".to_string(),
        );
    }
    for entry in entries {
        let object = entry
            .as_object_mut()
            .ok_or_else(|| "controlled-live dynamic entry must be an object".to_string())?;
        let dirty_slot = required_string(object, "source_blocker_id", "controlled-live entry")?
            == "dirty_worktree_boundary";
        let active = !clean || !dirty_slot;
        set_string(
            object,
            state_field,
            if active { "missing" } else { "satisfied" },
        );
        set_bool(object, "active_blocker", active);
        set_bool(object, "evidence_required", active);
        set_bool(object, "blocks_cutover", active);
    }
    Ok(())
}

fn add_closed_side_effects(object: &mut Map<String, Value>, keys: &[&str]) -> Result<(), String> {
    let effects = object
        .get_mut("side_effects")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "controlled-live report must expose side_effects".to_string())?;
    for key in keys {
        effects.insert((*key).to_string(), Value::Bool(false));
    }
    Ok(())
}

fn object_mut<'a>(
    value: &'a mut Value,
    context: &str,
) -> Result<&'a mut Map<String, Value>, String> {
    value
        .as_object_mut()
        .ok_or_else(|| format!("{context} must be a JSON object"))
}

fn array_mut<'a>(
    object: &'a mut Map<String, Value>,
    field: &str,
    context: &str,
) -> Result<&'a mut Vec<Value>, String> {
    object
        .get_mut(field)
        .and_then(Value::as_array_mut)
        .ok_or_else(|| format!("{context} must expose array {field}"))
}

fn required_array<'a>(value: &'a Value, field: &str, context: &str) -> Result<&'a [Value], String> {
    value
        .get(field)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| format!("{context} must expose array {field}"))
}

fn object_in_array_mut<'a>(
    values: &'a mut [Value],
    field: &str,
    expected: &str,
    context: &str,
) -> Result<&'a mut Map<String, Value>, String> {
    values
        .iter_mut()
        .find(|value| value.get(field).and_then(Value::as_str) == Some(expected))
        .and_then(Value::as_object_mut)
        .ok_or_else(|| format!("{context} {expected} is missing"))
}

fn object_in_array<'a>(
    values: &'a [Value],
    field: &str,
    expected: &str,
    context: &str,
) -> Result<&'a Map<String, Value>, String> {
    values
        .iter()
        .find(|value| value.get(field).and_then(Value::as_str) == Some(expected))
        .and_then(Value::as_object)
        .ok_or_else(|| format!("{context} {expected} is missing"))
}

fn required_string<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    context: &str,
) -> Result<&'a str, String> {
    object
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("{context} must expose string {field}"))
}

fn copy_fields(
    target: &mut Map<String, Value>,
    source: &Map<String, Value>,
    fields: &[&str],
) -> Result<(), String> {
    for field in fields {
        copy_field_as(target, source, field, field)?;
    }
    Ok(())
}

fn copy_field_as(
    target: &mut Map<String, Value>,
    source: &Map<String, Value>,
    source_field: &str,
    target_field: &str,
) -> Result<(), String> {
    let value = source
        .get(source_field)
        .ok_or_else(|| format!("controlled-live source lacks field {source_field}"))?;
    target.insert(target_field.to_string(), value.clone());
    Ok(())
}

fn set_string(object: &mut Map<String, Value>, field: &str, value: &str) {
    object.insert(field.to_string(), Value::String(value.to_string()));
}

fn set_bool(object: &mut Map<String, Value>, field: &str, value: bool) {
    object.insert(field.to_string(), Value::Bool(value));
}

fn set_usize(object: &mut Map<String, Value>, field: &str, value: usize) {
    object.insert(field.to_string(), Value::from(value));
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    fn dirty_observation() -> ControlledLiveWorktreeObservation {
        ControlledLiveWorktreeObservation::from_porcelain_v1_z(
            b" M codex-rs/hepta-runtime/src/lib.rs\0?? scripts/controlled-live-oracle.txt\0",
        )
        .expect("dirty fixture should parse")
    }

    #[test]
    fn controlled_live_registry_is_exact_and_unique() {
        assert_eq!(CONTROLLED_LIVE_TYPED_COMPAT_REPORT_IDS.len(), 17);
        assert_eq!(
            CONTROLLED_LIVE_TYPED_COMPAT_REPORT_IDS
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .len(),
            17
        );
    }

    #[test]
    fn controlled_live_observation_is_explicit_and_reconciled() {
        let clean = ControlledLiveWorktreeObservation::from_porcelain_v1_z(b"")
            .expect("clean fixture should parse");
        assert!(clean.is_clean());
        let dirty = dirty_observation();
        assert_eq!(dirty.status_entry_count, 2);
        assert_eq!(dirty.tracked_change_count, 1);
        assert_eq!(dirty.untracked_count, 1);
        assert!(!dirty.is_clean());
        assert!(
            ControlledLiveWorktreeObservation {
                status_entry_count: 2,
                tracked_change_count: 0,
                untracked_count: 1,
            }
            .validate()
            .is_err()
        );
    }

    #[test]
    fn six_report_chain_propagates_clean_and_dirty_slots() {
        for (observation, expected_active, expected_satisfied) in [
            (ControlledLiveWorktreeObservation::clean(), 6, 1),
            (dirty_observation(), 7, 0),
        ] {
            let chain = controlled_live_compat_chain(&observation)
                .expect("controlled-live chain should render");
            assert_eq!(chain.audit["blocker_count"], Value::from(expected_active));
            assert_eq!(
                chain.denial["active_blocker_count"],
                Value::from(expected_active)
            );
            assert_eq!(
                chain.denial["satisfied_entry_count"],
                Value::from(expected_satisfied)
            );
            assert_eq!(
                chain.packet["active_blocker_count"],
                Value::from(expected_active)
            );
            assert_eq!(
                chain.non_send["source_active_blocker_count"],
                Value::from(expected_active)
            );
            assert_eq!(
                chain.collection["required_evidence_count"],
                Value::from(expected_active)
            );
            assert_eq!(
                chain.readback["missing_evidence_count"],
                Value::from(expected_active)
            );
            for report in [
                &chain.audit,
                &chain.denial,
                &chain.packet,
                &chain.non_send,
                &chain.collection,
                &chain.readback,
            ] {
                assert_eq!(report["production_authority_granted"], false);
                assert_eq!(report["write_authority_granted"], false);
                assert_eq!(report["controlled_live_cutover_ready"], false);
                assert!(report["legacy_business_fields"].is_object());
            }
        }
    }

    #[test]
    fn all_seventeen_reports_are_typed_and_read_only() {
        for observation in [
            ControlledLiveWorktreeObservation::clean(),
            dirty_observation(),
        ] {
            for id in CONTROLLED_LIVE_TYPED_COMPAT_REPORT_IDS {
                let report = controlled_live_typed_compat_report(id, &observation)
                    .unwrap_or_else(|error| panic!("{id} failed: {error}"));
                let effects = report["side_effects"]
                    .as_object()
                    .unwrap_or_else(|| panic!("{id} lacks side effects"));
                assert!(!effects.is_empty(), "{id} has empty side effects");
                assert!(
                    effects.values().all(|value| value == &Value::Bool(false)),
                    "{id} exposes an enabled side effect"
                );
                assert_eq!(report["production_authority_granted"], false, "{id}");
                assert_eq!(report["write_authority_granted"], false, "{id}");
            }
        }
    }

    #[test]
    fn dynamic_sources_fail_closed_on_schema_readiness_and_authority_drift() {
        let observation = ControlledLiveWorktreeObservation::clean();
        let audit = controlled_live_readiness_audit_report_from_observation(&observation)
            .expect("audit should render");
        for (field, value) in [
            ("controlled_live_audit_ready", Value::Bool(false)),
            ("schema_version", Value::String("wrong".to_string())),
            ("production_authority_granted", Value::Bool(true)),
        ] {
            let mut drifted = audit.clone();
            drifted[field] = value;
            assert!(
                controlled_live_readiness_denial_readback_index_report_from_sources(
                    &observation,
                    &drifted,
                )
                .is_err(),
                "source drift in {field} must fail closed"
            );
        }
    }

    #[test]
    fn enabled_side_effect_or_mutation_authority_is_rejected() {
        let mut report = serialize(crate::controlled_live_required_evidence_gap_summary_report())
            .expect("gap summary should serialize");
        report["side_effects"]["provider_invoked"] = Value::Bool(true);
        assert!(finalize(report, "gap-summary-negative", "gap_summary_ready").is_err());

        let mut report = serialize(crate::controlled_live_required_evidence_gap_summary_report())
            .expect("gap summary should serialize");
        report["live_execution_allowed"] = Value::Bool(true);
        assert!(finalize(report, "gap-summary-negative", "gap_summary_ready").is_err());
    }
}
