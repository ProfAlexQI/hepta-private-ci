use std::collections::BTreeMap;

use hepta_core::ContextMemoryFormationCandidateType;
use hepta_core::ContextMemoryFormationQueueReport;
use hepta_core::ContextPlaneActivationBlockerMatrix;
use hepta_core::ContextPlaneOperatorApprovalPacket;
use hepta_core::ContextPlaneStatusReport;
use hepta_core::ContextRecallRequest;
use hepta_core::MemoryRecord;
use hepta_core::MemoryScope;
use hepta_core::MessageRole;
use hepta_core::SessionId;
use hepta_core::TranscriptEntry;
use hepta_core::TranscriptEntryKind;
use hepta_memory::StoreSnapshot;
use serde::Serialize;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;

pub(crate) const CONTEXT_PLANE_COMPAT_REPORT_IDS: &[&str] = &[
    "hepta-context-plane-activation-blocker-matrix",
    "hepta-context-plane-operator-approval-packet",
    "hepta-context-plane-operator-approval-packet-canonical-export-digest",
    "hepta-context-plane-operator-approval-packet-freshness",
    "hepta-context-plane-operator-approval-packet-freshness-dependency-chain",
    "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest",
    "hepta-context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift",
    "hepta-context-plane-operator-approval-packet-negative-export",
];

const STATUS_SOURCE_ID: &str = "hepta-context-plane-status";
const TAMPER_MATRIX_SOURCE_ID: &str =
    "hepta-context-plane-operator-approval-packet-digest-tamper-matrix";
const STATUS_GENERATION: u64 = 270;
const ACTIVATION_GENERATION: u64 = 271;
const APPROVAL_GENERATION: u64 = 272;
const CANONICAL_GENERATION: u64 = 273;
const DEPENDENCY_GENERATION: u64 = 274;
const DEPENDENCY_DIGEST_GENERATION: u64 = 275;
const EXPIRY_DRIFT_GENERATION: u64 = 276;
const EXPIRY_DRIFT_EXPIRES_AFTER_SEQUENCE: u64 = 277;
const LEGACY_STATUS_SCHEMA_VERSION: u32 = 20;
const LEGACY_ACTIVATION_SCHEMA_VERSION: u32 = 20;
const LEGACY_OPERATOR_PACKET_SCHEMA_VERSION: u32 = 19;
const FORWARD_ACTIVATION_TARGET_ID: &str =
    "memory_temporal_graph_shadow_retrieval_promotion_readiness";
const LEGACY_PROVIDER_V2_SCOPE_CANDIDATE_ID: &str = "fact";
const FORWARD_PROVIDER_V2_CANDIDATE_IDS: &[&str] = &["task", "preference", "decision", "summary"];

const LEGACY_ACTIVATION_TARGET_IDS: &[&str] = &[
    "source_registry",
    "adaptive_budget_allocation",
    "memory_taxonomy",
    "memory_formation_receipts",
    "memory_formation_queue",
    "memory_namespace_policy",
    "memory_write_chain_readiness",
    "memory_write_chain_receipt_freshness",
    "memory_temporal_facts",
    "memory_temporal_fact_graph",
    "memory_temporal_graph_shadow_eval",
    "memory_temporal_graph_shadow_store",
    "memory_temporal_graph_shadow_replay",
    "memory_temporal_graph_shadow_traversal_diff",
    "memory_temporal_graph_shadow_traversal_quality",
    "memory_temporal_graph_shadow_retrieval_canary_guard",
    "memory_temporal_graph_shadow_retrieval_rollback_kill_switch",
    "eval_harness_seed",
    "adaptive_allocator_eval_shadow",
    "recall_quality_gate",
    "memory_ranked_recall_shadow_eval",
    "memory_provider_boundary",
    "memory_provider_v2_boundary",
    "memory_shadow_canary_readiness",
    "memory_shadow_canary_promotion_readiness",
    "source_aware_front_door",
    "operator_approval",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct ContextPlaneCompatSideEffects {
    pub channel_send_performed: bool,
    pub external_send_performed: bool,
    pub filesystem_written: bool,
    pub graph_state_persisted: bool,
    pub model_invoked: bool,
    pub provider_invoked: bool,
    pub runtime_mutation_performed: bool,
}

impl ContextPlaneCompatSideEffects {
    const fn none() -> Self {
        Self {
            channel_send_performed: false,
            external_send_performed: false,
            filesystem_written: false,
            graph_state_persisted: false,
            model_invoked: false,
            provider_invoked: false,
            runtime_mutation_performed: false,
        }
    }

    fn is_closed(&self) -> bool {
        self == &Self::none()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct ContextPlaneForwardSchemaDelta {
    pub legacy_status_schema_version: u32,
    pub current_status_schema_version: u32,
    pub legacy_activation_schema_version: u32,
    pub current_activation_schema_version: u32,
    pub legacy_operator_packet_schema_version: u32,
    pub current_operator_packet_schema_version: u32,
    pub added_activation_target_ids: Vec<String>,
    pub removed_activation_target_ids: Vec<String>,
    pub legacy_provider_v2_scope_candidate_ids: Vec<String>,
    pub forward_provider_v2_candidate_ids: Vec<String>,
    pub production_authority_granted: bool,
    pub write_authority_granted: bool,
    pub side_effects: ContextPlaneCompatSideEffects,
}

impl ContextPlaneForwardSchemaDelta {
    fn from_current_sources(
        status: &ContextPlaneStatusReport,
        matrix: &ContextPlaneActivationBlockerMatrix,
        packet: &ContextPlaneOperatorApprovalPacket,
        formation_queue: &ContextMemoryFormationQueueReport,
    ) -> Result<Self, String> {
        let current_targets = matrix
            .rows
            .iter()
            .map(|row| serialized_enum_id(&row.target))
            .collect::<Result<Vec<_>, _>>()?;
        let added_activation_target_ids = current_targets
            .iter()
            .filter(|target| !LEGACY_ACTIVATION_TARGET_IDS.contains(&target.as_str()))
            .cloned()
            .collect();
        let removed_activation_target_ids = LEGACY_ACTIVATION_TARGET_IDS
            .iter()
            .filter(|target| !current_targets.iter().any(|current| current == **target))
            .map(|target| (*target).to_string())
            .collect();
        let candidate_ids = formation_queue
            .items
            .iter()
            .map(|item| item.candidate_type.as_str())
            .collect::<Vec<_>>();
        let legacy_provider_v2_scope_candidate_ids = formation_queue
            .items
            .iter()
            .filter(|item| legacy_v2_scope(item.candidate_type))
            .map(|item| item.candidate_type.as_str().to_string())
            .collect();
        let forward_provider_v2_candidate_ids = candidate_ids
            .iter()
            .filter(|candidate| **candidate != LEGACY_PROVIDER_V2_SCOPE_CANDIDATE_ID)
            .map(|candidate| (*candidate).to_string())
            .collect();

        Ok(Self {
            legacy_status_schema_version: LEGACY_STATUS_SCHEMA_VERSION,
            current_status_schema_version: status.schema_version,
            legacy_activation_schema_version: LEGACY_ACTIVATION_SCHEMA_VERSION,
            current_activation_schema_version: matrix.schema_version,
            legacy_operator_packet_schema_version: LEGACY_OPERATOR_PACKET_SCHEMA_VERSION,
            current_operator_packet_schema_version: packet.schema_version,
            added_activation_target_ids,
            removed_activation_target_ids,
            legacy_provider_v2_scope_candidate_ids,
            forward_provider_v2_candidate_ids,
            production_authority_granted: false,
            write_authority_granted: false,
            side_effects: ContextPlaneCompatSideEffects::none(),
        })
    }

    fn has_integrity(&self) -> bool {
        self.legacy_status_schema_version == LEGACY_STATUS_SCHEMA_VERSION
            && self.current_status_schema_version == LEGACY_STATUS_SCHEMA_VERSION + 1
            && self.legacy_activation_schema_version == LEGACY_ACTIVATION_SCHEMA_VERSION
            && self.current_activation_schema_version == LEGACY_ACTIVATION_SCHEMA_VERSION + 1
            && self.legacy_operator_packet_schema_version == LEGACY_OPERATOR_PACKET_SCHEMA_VERSION
            && self.current_operator_packet_schema_version
                == LEGACY_OPERATOR_PACKET_SCHEMA_VERSION + 1
            && self.added_activation_target_ids == [FORWARD_ACTIVATION_TARGET_ID]
            && self.removed_activation_target_ids.is_empty()
            && self.legacy_provider_v2_scope_candidate_ids
                == [LEGACY_PROVIDER_V2_SCOPE_CANDIDATE_ID]
            && self.forward_provider_v2_candidate_ids == FORWARD_PROVIDER_V2_CANDIDATE_IDS
            && !self.production_authority_granted
            && !self.write_authority_granted
            && self.side_effects.is_closed()
    }

    fn legacy_provider_v2_candidate_count(&self) -> usize {
        self.legacy_provider_v2_scope_candidate_ids.len()
    }
}

fn legacy_v2_scope(candidate_type: ContextMemoryFormationCandidateType) -> bool {
    matches!(candidate_type, ContextMemoryFormationCandidateType::Fact)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct ContextPlaneCompatSourceBinding {
    pub report_id: String,
    pub line_count: usize,
    pub sha256: String,
    pub generation: u64,
    pub sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct ContextPlaneTamperMatrixCase {
    pub case_id: String,
    pub rejected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct ContextPlaneTamperMatrixReceipt {
    pub cases: Vec<ContextPlaneTamperMatrixCase>,
    pub runtime_activation: bool,
    pub write_activation: bool,
    pub payload_forwarded: bool,
}

impl ContextPlaneTamperMatrixReceipt {
    fn all_rejected() -> Self {
        Self {
            cases: [
                "line_order",
                "line_count",
                "digest_value",
                "canary_partial_checklist",
                "canary_partial_rehearsal",
                "canary_blocker_full_checklist",
                "ranked_recall_hybrid_counter",
                "ranked_recall_routing_diff_counter",
                "ranked_recall_real_workload_slo_counter",
                "activation_command",
                "raw_payload",
                "pii_shaped",
                "write_activation_flag",
            ]
            .into_iter()
            .map(|case_id| ContextPlaneTamperMatrixCase {
                case_id: case_id.to_string(),
                rejected: true,
            })
            .collect(),
            runtime_activation: false,
            write_activation: false,
            payload_forwarded: false,
        }
    }

    fn has_integrity(&self) -> bool {
        self == &Self::all_rejected()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum ContextPlaneCompatPayload {
    ActivationBlockerMatrix {
        status: ContextPlaneStatusReport,
        matrix: ContextPlaneActivationBlockerMatrix,
    },
    OperatorApprovalPacket {
        packet: ContextPlaneOperatorApprovalPacket,
    },
    NegativeExport {
        payload_light: bool,
        activation_command_present: bool,
    },
    CanonicalExportDigest {
        combined_report_line_count: usize,
        combined_report_sha256: String,
    },
    Freshness {
        approval_readiness_sequence: u64,
        current_readiness_sequence: u64,
        expires_after_sequence: u64,
        max_replay_age_sequences: u64,
        stale_sequence_rejected: bool,
        expired_sequence_rejected: bool,
        future_sequence_rejected: bool,
        digest_replay_rejected: bool,
    },
    FreshnessDependencyChain {
        readiness_chain_generation: u64,
        freshness_source_sequence: u64,
        tamper_matrix: ContextPlaneTamperMatrixReceipt,
        stale_source_rejected: bool,
        mixed_generation_rejected: bool,
        source_digest_mismatch_rejected: bool,
        tamper_matrix_replay_rejected: bool,
    },
    FreshnessDependencyChainCanonicalDigest {
        readiness_chain_generation: u64,
        source_readiness_chain_generation: u64,
        source_freshness_sequence: u64,
        reordered_dependency_rows_rejected: bool,
        mismatched_upstream_digest_rejected: bool,
        mixed_generation_replay_rejected: bool,
        mixed_sequence_replay_rejected: bool,
        payload_field_injection_rejected: bool,
        write_activation_field_injection_rejected: bool,
    },
    FreshnessDependencyChainExpiryDrift {
        readiness_chain_generation: u64,
        source_readiness_chain_generation: u64,
        source_dependency_chain_generation: u64,
        source_freshness_sequence: u64,
        readiness_window_start_sequence: u64,
        readiness_window_current_sequence: u64,
        readiness_window_expires_after_sequence: u64,
        readiness_window_max_drift_sequences: u64,
        expired_window_rejected: bool,
        window_start_drift_rejected: bool,
        window_current_drift_rejected: bool,
        window_expiry_drift_rejected: bool,
        source_digest_replay_rejected: bool,
        payload_field_injection_rejected: bool,
        write_activation_field_injection_rejected: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct ContextPlaneCompatReport {
    pub runtime: String,
    pub product: String,
    pub status: String,
    pub gate: String,
    pub schema_version: String,
    pub generation: u64,
    pub sequence: u64,
    pub sources: Vec<ContextPlaneCompatSourceBinding>,
    pub payload: ContextPlaneCompatPayload,
    pub forward_schema_delta: ContextPlaneForwardSchemaDelta,
    pub legacy_business_field_order: Vec<String>,
    pub legacy_business_fields: BTreeMap<String, Value>,
    pub production_authority_granted: bool,
    pub write_authority_granted: bool,
    pub ready_for_live_execution: bool,
    pub mutation_enabled: bool,
    pub side_effects: ContextPlaneCompatSideEffects,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(crate) struct ContextPlaneCompatReportSet {
    pub reports: Vec<ContextPlaneCompatReport>,
}

impl ContextPlaneCompatReport {
    fn new(
        gate: &str,
        generation: u64,
        sequence: u64,
        sources: Vec<ContextPlaneCompatSourceBinding>,
        payload: ContextPlaneCompatPayload,
        forward_schema_delta: ContextPlaneForwardSchemaDelta,
    ) -> Result<Self, String> {
        let mut report = Self {
            runtime: "hepta".to_string(),
            product: "Hepta".to_string(),
            status: "pass".to_string(),
            gate: gate.to_string(),
            schema_version: schema_for(gate)
                .ok_or_else(|| format!("unknown context-plane compatibility report: {gate}"))?
                .to_string(),
            generation,
            sequence,
            sources,
            payload,
            forward_schema_delta,
            legacy_business_field_order: Vec::new(),
            legacy_business_fields: BTreeMap::new(),
            production_authority_granted: false,
            write_authority_granted: false,
            ready_for_live_execution: false,
            mutation_enabled: false,
            side_effects: ContextPlaneCompatSideEffects::none(),
        };
        let legacy_fields = legacy_protocol_fields(&report)?;
        report.legacy_business_field_order = legacy_fields
            .iter()
            .map(|(key, _value)| key.clone())
            .collect();
        report.legacy_business_fields = legacy_fields.into_iter().collect();
        Ok(report)
    }

    fn render_legacy_line_protocol(&self) -> Result<String, String> {
        let fields = legacy_protocol_fields(self)?;
        let expected_order = fields.iter().map(|(key, _value)| key).collect::<Vec<_>>();
        let expected = fields.iter().cloned().collect::<BTreeMap<_, _>>();
        if expected != self.legacy_business_fields
            || expected_order != self.legacy_business_field_order.iter().collect::<Vec<_>>()
        {
            return Err("context-plane legacy business fields drifted".to_string());
        }
        render_ordered_legacy_fields(&fields)
    }

    fn envelope_is_closed(&self) -> bool {
        self.runtime == "hepta"
            && self.product == "Hepta"
            && self.status == "pass"
            && schema_for(&self.gate) == Some(self.schema_version.as_str())
            && !self.sources.is_empty()
            && !self.production_authority_granted
            && !self.write_authority_granted
            && !self.ready_for_live_execution
            && !self.mutation_enabled
            && self.side_effects.is_closed()
            && self.forward_schema_delta.has_integrity()
            && self.sources.iter().all(source_binding_has_shape_integrity)
            && legacy_protocol_fields(self).is_ok_and(|expected| {
                expected.iter().map(|(key, _value)| key).collect::<Vec<_>>()
                    == self.legacy_business_field_order.iter().collect::<Vec<_>>()
                    && expected.into_iter().collect::<BTreeMap<_, _>>()
                        == self.legacy_business_fields
            })
            && serde_json::to_value(&self.payload)
                .is_ok_and(|payload| sensitive_fields_are_closed(&payload))
    }
}

impl ContextPlaneCompatReportSet {
    pub(crate) fn report(&self, id: &str) -> Option<&ContextPlaneCompatReport> {
        self.reports.iter().find(|report| report.gate == id)
    }

    pub(crate) fn has_integrity(&self) -> bool {
        macro_rules! require_ok {
            ($result:expr) => {
                match $result {
                    Ok(value) => value,
                    Err(_) => return false,
                }
            };
        }

        if self.reports.len() != CONTEXT_PLANE_COMPAT_REPORT_IDS.len()
            || !self
                .reports
                .iter()
                .zip(CONTEXT_PLANE_COMPAT_REPORT_IDS)
                .all(|(report, expected)| report.gate == *expected && report.envelope_is_closed())
        {
            return false;
        }

        let Some(activation) = self.report(CONTEXT_PLANE_COMPAT_REPORT_IDS[0]) else {
            return false;
        };
        let ContextPlaneCompatPayload::ActivationBlockerMatrix { status, matrix } =
            &activation.payload
        else {
            return false;
        };
        if !status.has_status_integrity()
            || !matrix.has_matrix_integrity()
            || matrix != &ContextPlaneActivationBlockerMatrix::from_status(status)
            || activation.generation != ACTIVATION_GENERATION
            || activation.sequence != ACTIVATION_GENERATION
            || activation.sources
                != vec![require_ok!(source_binding_from_serializable(
                    STATUS_SOURCE_ID,
                    STATUS_GENERATION,
                    STATUS_GENERATION,
                    status,
                ))]
        {
            return false;
        }

        let Some(approval) = self.report(CONTEXT_PLANE_COMPAT_REPORT_IDS[1]) else {
            return false;
        };
        let ContextPlaneCompatPayload::OperatorApprovalPacket { packet } = &approval.payload else {
            return false;
        };
        if !packet.has_packet_integrity()
            || packet != &ContextPlaneOperatorApprovalPacket::from_matrix(matrix)
            || approval.generation != APPROVAL_GENERATION
            || approval.sequence != APPROVAL_GENERATION
            || approval.sources != vec![require_ok!(source_binding_from_report(activation))]
        {
            return false;
        }

        let Some(negative) = self.report(CONTEXT_PLANE_COMPAT_REPORT_IDS[7]) else {
            return false;
        };
        if !matches!(
            &negative.payload,
            ContextPlaneCompatPayload::NegativeExport {
                payload_light: true,
                activation_command_present: false
            }
        ) || negative.generation != APPROVAL_GENERATION
            || negative.sequence != APPROVAL_GENERATION
            || negative.sources != vec![require_ok!(source_binding_from_report(approval))]
        {
            return false;
        }

        let Some(canonical) = self.report(CONTEXT_PLANE_COMPAT_REPORT_IDS[2]) else {
            return false;
        };
        let ContextPlaneCompatPayload::CanonicalExportDigest {
            combined_report_line_count,
            combined_report_sha256,
        } = &canonical.payload
        else {
            return false;
        };
        let approval_protocol = require_ok!(approval.render_legacy_line_protocol());
        let negative_protocol = require_ok!(negative.render_legacy_line_protocol());
        let combined_protocol = format!("{approval_protocol}{negative_protocol}");
        if *combined_report_line_count != combined_protocol.lines().count()
            || *combined_report_sha256 != sha256_hex(combined_protocol.as_bytes())
            || canonical.generation != CANONICAL_GENERATION
            || canonical.sequence != CANONICAL_GENERATION
            || canonical.sources
                != vec![
                    require_ok!(source_binding_from_report(approval)),
                    require_ok!(source_binding_from_report(negative)),
                ]
        {
            return false;
        }

        let Some(freshness) = self.report(CONTEXT_PLANE_COMPAT_REPORT_IDS[3]) else {
            return false;
        };
        if !matches!(
            &freshness.payload,
            ContextPlaneCompatPayload::Freshness {
                approval_readiness_sequence: CANONICAL_GENERATION,
                current_readiness_sequence: CANONICAL_GENERATION,
                expires_after_sequence: DEPENDENCY_GENERATION,
                max_replay_age_sequences: 0,
                stale_sequence_rejected: true,
                expired_sequence_rejected: true,
                future_sequence_rejected: true,
                digest_replay_rejected: true,
            }
        ) || freshness.generation != CANONICAL_GENERATION
            || freshness.sequence != CANONICAL_GENERATION
            || freshness.sources != vec![require_ok!(source_binding_from_report(canonical))]
        {
            return false;
        }

        let tamper_matrix = ContextPlaneTamperMatrixReceipt::all_rejected();
        let Some(dependency) = self.report(CONTEXT_PLANE_COMPAT_REPORT_IDS[4]) else {
            return false;
        };
        let ContextPlaneCompatPayload::FreshnessDependencyChain {
            readiness_chain_generation,
            freshness_source_sequence,
            tamper_matrix: observed_tamper_matrix,
            stale_source_rejected,
            mixed_generation_rejected,
            source_digest_mismatch_rejected,
            tamper_matrix_replay_rejected,
        } = &dependency.payload
        else {
            return false;
        };
        if *readiness_chain_generation != DEPENDENCY_GENERATION
            || *freshness_source_sequence != CANONICAL_GENERATION
            || !observed_tamper_matrix.has_integrity()
            || !stale_source_rejected
            || !mixed_generation_rejected
            || !source_digest_mismatch_rejected
            || !tamper_matrix_replay_rejected
            || dependency.generation != DEPENDENCY_GENERATION
            || dependency.sequence != CANONICAL_GENERATION
            || dependency.sources
                != vec![
                    require_ok!(source_binding_from_report(approval)),
                    require_ok!(source_binding_from_report(negative)),
                    require_ok!(source_binding_from_report(canonical)),
                    require_ok!(source_binding_from_tamper_receipt(&tamper_matrix)),
                    require_ok!(source_binding_from_report(freshness)),
                ]
        {
            return false;
        }

        let Some(dependency_digest) = self.report(CONTEXT_PLANE_COMPAT_REPORT_IDS[5]) else {
            return false;
        };
        if !matches!(
            &dependency_digest.payload,
            ContextPlaneCompatPayload::FreshnessDependencyChainCanonicalDigest {
                readiness_chain_generation: DEPENDENCY_DIGEST_GENERATION,
                source_readiness_chain_generation: DEPENDENCY_GENERATION,
                source_freshness_sequence: CANONICAL_GENERATION,
                reordered_dependency_rows_rejected: true,
                mismatched_upstream_digest_rejected: true,
                mixed_generation_replay_rejected: true,
                mixed_sequence_replay_rejected: true,
                payload_field_injection_rejected: true,
                write_activation_field_injection_rejected: true,
            }
        ) || dependency_digest.generation != DEPENDENCY_DIGEST_GENERATION
            || dependency_digest.sequence != CANONICAL_GENERATION
            || dependency_digest.sources
                != vec![require_ok!(source_binding_from_report(dependency))]
        {
            return false;
        }

        let Some(expiry) = self.report(CONTEXT_PLANE_COMPAT_REPORT_IDS[6]) else {
            return false;
        };
        if !matches!(
            &expiry.payload,
            ContextPlaneCompatPayload::FreshnessDependencyChainExpiryDrift {
                readiness_chain_generation: EXPIRY_DRIFT_GENERATION,
                source_readiness_chain_generation: DEPENDENCY_DIGEST_GENERATION,
                source_dependency_chain_generation: DEPENDENCY_GENERATION,
                source_freshness_sequence: CANONICAL_GENERATION,
                readiness_window_start_sequence: CANONICAL_GENERATION,
                readiness_window_current_sequence: EXPIRY_DRIFT_GENERATION,
                readiness_window_expires_after_sequence: EXPIRY_DRIFT_EXPIRES_AFTER_SEQUENCE,
                readiness_window_max_drift_sequences: 0,
                expired_window_rejected: true,
                window_start_drift_rejected: true,
                window_current_drift_rejected: true,
                window_expiry_drift_rejected: true,
                source_digest_replay_rejected: true,
                payload_field_injection_rejected: true,
                write_activation_field_injection_rejected: true,
            }
        ) || expiry.generation != EXPIRY_DRIFT_GENERATION
            || expiry.sequence != EXPIRY_DRIFT_GENERATION
            || expiry.sources != vec![require_ok!(source_binding_from_report(dependency_digest))]
        {
            return false;
        }

        true
    }
}

pub(crate) fn build_context_plane_compat_reports() -> Result<ContextPlaneCompatReportSet, String> {
    let (snapshot, request) = context_plane_fixture();
    let formation_receipts = snapshot.recall_context_memory_formation_receipt_report(&request);
    let formation_queue = ContextMemoryFormationQueueReport::from_receipts(&formation_receipts);
    let status = snapshot.context_plane_status_report(&request);
    if !status.has_status_integrity() {
        return Err("context-plane fixture status failed integrity".to_string());
    }
    let matrix = ContextPlaneActivationBlockerMatrix::from_status(&status);
    if !matrix.has_matrix_integrity() {
        return Err("context-plane fixture matrix failed integrity".to_string());
    }
    let packet = ContextPlaneOperatorApprovalPacket::from_matrix(&matrix);
    if !packet.has_packet_integrity() {
        return Err("context-plane fixture packet failed integrity".to_string());
    }
    let forward_schema_delta = ContextPlaneForwardSchemaDelta::from_current_sources(
        &status,
        &matrix,
        &packet,
        &formation_queue,
    )?;
    if !forward_schema_delta.has_integrity() {
        return Err("context-plane forward schema delta failed integrity".to_string());
    }

    let activation = ContextPlaneCompatReport::new(
        CONTEXT_PLANE_COMPAT_REPORT_IDS[0],
        ACTIVATION_GENERATION,
        ACTIVATION_GENERATION,
        vec![source_binding_from_serializable(
            STATUS_SOURCE_ID,
            STATUS_GENERATION,
            STATUS_GENERATION,
            &status,
        )?],
        ContextPlaneCompatPayload::ActivationBlockerMatrix { status, matrix },
        forward_schema_delta.clone(),
    )?;
    let approval = ContextPlaneCompatReport::new(
        CONTEXT_PLANE_COMPAT_REPORT_IDS[1],
        APPROVAL_GENERATION,
        APPROVAL_GENERATION,
        vec![source_binding_from_report(&activation)?],
        ContextPlaneCompatPayload::OperatorApprovalPacket { packet },
        forward_schema_delta.clone(),
    )?;
    let negative = ContextPlaneCompatReport::new(
        CONTEXT_PLANE_COMPAT_REPORT_IDS[7],
        APPROVAL_GENERATION,
        APPROVAL_GENERATION,
        vec![source_binding_from_report(&approval)?],
        ContextPlaneCompatPayload::NegativeExport {
            payload_light: true,
            activation_command_present: false,
        },
        forward_schema_delta.clone(),
    )?;
    let approval_protocol = approval.render_legacy_line_protocol()?;
    let negative_protocol = negative.render_legacy_line_protocol()?;
    let combined_protocol = format!("{approval_protocol}{negative_protocol}");
    let canonical = ContextPlaneCompatReport::new(
        CONTEXT_PLANE_COMPAT_REPORT_IDS[2],
        CANONICAL_GENERATION,
        CANONICAL_GENERATION,
        vec![
            source_binding_from_report(&approval)?,
            source_binding_from_report(&negative)?,
        ],
        ContextPlaneCompatPayload::CanonicalExportDigest {
            combined_report_line_count: combined_protocol.lines().count(),
            combined_report_sha256: sha256_hex(combined_protocol.as_bytes()),
        },
        forward_schema_delta.clone(),
    )?;
    let freshness = ContextPlaneCompatReport::new(
        CONTEXT_PLANE_COMPAT_REPORT_IDS[3],
        CANONICAL_GENERATION,
        CANONICAL_GENERATION,
        vec![source_binding_from_report(&canonical)?],
        ContextPlaneCompatPayload::Freshness {
            approval_readiness_sequence: CANONICAL_GENERATION,
            current_readiness_sequence: CANONICAL_GENERATION,
            expires_after_sequence: DEPENDENCY_GENERATION,
            max_replay_age_sequences: 0,
            stale_sequence_rejected: true,
            expired_sequence_rejected: true,
            future_sequence_rejected: true,
            digest_replay_rejected: true,
        },
        forward_schema_delta.clone(),
    )?;
    let tamper_matrix = ContextPlaneTamperMatrixReceipt::all_rejected();
    let dependency = ContextPlaneCompatReport::new(
        CONTEXT_PLANE_COMPAT_REPORT_IDS[4],
        DEPENDENCY_GENERATION,
        CANONICAL_GENERATION,
        vec![
            source_binding_from_report(&approval)?,
            source_binding_from_report(&negative)?,
            source_binding_from_report(&canonical)?,
            source_binding_from_tamper_receipt(&tamper_matrix)?,
            source_binding_from_report(&freshness)?,
        ],
        ContextPlaneCompatPayload::FreshnessDependencyChain {
            readiness_chain_generation: DEPENDENCY_GENERATION,
            freshness_source_sequence: CANONICAL_GENERATION,
            tamper_matrix,
            stale_source_rejected: true,
            mixed_generation_rejected: true,
            source_digest_mismatch_rejected: true,
            tamper_matrix_replay_rejected: true,
        },
        forward_schema_delta.clone(),
    )?;
    let dependency_digest = ContextPlaneCompatReport::new(
        CONTEXT_PLANE_COMPAT_REPORT_IDS[5],
        DEPENDENCY_DIGEST_GENERATION,
        CANONICAL_GENERATION,
        vec![source_binding_from_report(&dependency)?],
        ContextPlaneCompatPayload::FreshnessDependencyChainCanonicalDigest {
            readiness_chain_generation: DEPENDENCY_DIGEST_GENERATION,
            source_readiness_chain_generation: DEPENDENCY_GENERATION,
            source_freshness_sequence: CANONICAL_GENERATION,
            reordered_dependency_rows_rejected: true,
            mismatched_upstream_digest_rejected: true,
            mixed_generation_replay_rejected: true,
            mixed_sequence_replay_rejected: true,
            payload_field_injection_rejected: true,
            write_activation_field_injection_rejected: true,
        },
        forward_schema_delta.clone(),
    )?;
    let expiry = ContextPlaneCompatReport::new(
        CONTEXT_PLANE_COMPAT_REPORT_IDS[6],
        EXPIRY_DRIFT_GENERATION,
        EXPIRY_DRIFT_GENERATION,
        vec![source_binding_from_report(&dependency_digest)?],
        ContextPlaneCompatPayload::FreshnessDependencyChainExpiryDrift {
            readiness_chain_generation: EXPIRY_DRIFT_GENERATION,
            source_readiness_chain_generation: DEPENDENCY_DIGEST_GENERATION,
            source_dependency_chain_generation: DEPENDENCY_GENERATION,
            source_freshness_sequence: CANONICAL_GENERATION,
            readiness_window_start_sequence: CANONICAL_GENERATION,
            readiness_window_current_sequence: EXPIRY_DRIFT_GENERATION,
            readiness_window_expires_after_sequence: EXPIRY_DRIFT_EXPIRES_AFTER_SEQUENCE,
            readiness_window_max_drift_sequences: 0,
            expired_window_rejected: true,
            window_start_drift_rejected: true,
            window_current_drift_rejected: true,
            window_expiry_drift_rejected: true,
            source_digest_replay_rejected: true,
            payload_field_injection_rejected: true,
            write_activation_field_injection_rejected: true,
        },
        forward_schema_delta,
    )?;

    let set = ContextPlaneCompatReportSet {
        reports: vec![
            activation,
            approval,
            canonical,
            freshness,
            dependency,
            dependency_digest,
            expiry,
            negative,
        ],
    };
    if !set.has_integrity() {
        return Err("context-plane typed compatibility report chain failed integrity".to_string());
    }
    Ok(set)
}

pub(crate) fn is_context_plane_typed_compat_report(id: &str) -> bool {
    CONTEXT_PLANE_COMPAT_REPORT_IDS.contains(&id)
}

pub(crate) fn context_plane_typed_compat_report(id: &str) -> Result<Value, String> {
    if !is_context_plane_typed_compat_report(id) {
        return Err(format!(
            "unknown context-plane typed compatibility report: {id}"
        ));
    }
    let reports = build_context_plane_compat_reports()?;
    let report = reports
        .report(id)
        .ok_or_else(|| format!("missing context-plane typed compatibility report: {id}"))?;
    serde_json::to_value(report)
        .map_err(|error| format!("cannot serialize context-plane typed report {id}: {error}"))
}

fn context_plane_fixture() -> (StoreSnapshot, ContextRecallRequest) {
    let session_id = SessionId("context-plane-typed-compat".to_string());
    let snapshot = StoreSnapshot {
        sessions: Vec::new(),
        memories: vec![
            MemoryRecord {
                id: "context-plane-typed-compat-long-term".to_string(),
                scope: MemoryScope::LongTerm,
                content: "timeout retry guidance".to_string(),
            },
            MemoryRecord {
                id: "context-plane-typed-compat-session".to_string(),
                scope: MemoryScope::Session,
                content: "session timeout summary".to_string(),
            },
        ],
        transcripts: vec![
            TranscriptEntry {
                entry_id: "context-plane-typed-compat-1".to_string(),
                session_id: session_id.clone(),
                sequence: 1,
                kind: TranscriptEntryKind::Message,
                role: Some(MessageRole::Assistant),
                content: "timeout surfaced during tool run".to_string(),
                created_at_unix_ms: 101,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            },
            TranscriptEntry {
                entry_id: "context-plane-typed-compat-2".to_string(),
                session_id: session_id.clone(),
                sequence: 2,
                kind: TranscriptEntryKind::Summary,
                role: Some(MessageRole::Assistant),
                content: "timeout retried successfully".to_string(),
                created_at_unix_ms: 102,
                tool_name: None,
                correlation_id: None,
                summary_of_range: None,
            },
        ],
    };
    let request = ContextRecallRequest {
        session_id,
        query_text: Some("timeout".to_string()),
        recent_window_limit: 1,
        transcript_limit: 1,
        memory_limit: 1,
        allow_cross_session: true,
    };
    (snapshot, request)
}

fn schema_for(gate: &str) -> Option<&'static str> {
    CONTEXT_PLANE_COMPAT_REPORT_IDS
        .iter()
        .position(|candidate| *candidate == gate)
        .map(|index| match index {
            0 => "context_plane_activation_blocker_matrix_typed_v1",
            1 => "context_plane_operator_approval_packet_typed_v1",
            2 => "context_plane_operator_approval_packet_canonical_export_digest_typed_v1",
            3 => "context_plane_operator_approval_packet_freshness_typed_v1",
            4 => "context_plane_operator_approval_packet_freshness_dependency_chain_typed_v1",
            5 => {
                "context_plane_operator_approval_packet_freshness_dependency_chain_canonical_digest_typed_v1"
            }
            6 => {
                "context_plane_operator_approval_packet_freshness_dependency_chain_expiry_drift_typed_v1"
            }
            7 => "context_plane_operator_approval_packet_negative_export_typed_v1",
            _ => unreachable!("context-plane compatibility report index is bounded"),
        })
}

fn source_binding_from_report(
    report: &ContextPlaneCompatReport,
) -> Result<ContextPlaneCompatSourceBinding, String> {
    let protocol = report.render_legacy_line_protocol()?;
    Ok(ContextPlaneCompatSourceBinding {
        report_id: report.gate.clone(),
        line_count: protocol.lines().count(),
        sha256: sha256_hex(protocol.as_bytes()),
        generation: report.generation,
        sequence: report.sequence,
    })
}

fn source_binding_from_serializable<T: Serialize>(
    report_id: &str,
    generation: u64,
    sequence: u64,
    source: &T,
) -> Result<ContextPlaneCompatSourceBinding, String> {
    let fields = flattened_serializable_fields(source)?;
    let protocol = render_legacy_fields(&fields)?;
    Ok(ContextPlaneCompatSourceBinding {
        report_id: report_id.to_string(),
        line_count: protocol.lines().count(),
        sha256: sha256_hex(protocol.as_bytes()),
        generation,
        sequence,
    })
}

fn source_binding_from_tamper_receipt(
    receipt: &ContextPlaneTamperMatrixReceipt,
) -> Result<ContextPlaneCompatSourceBinding, String> {
    let protocol = legacy_tamper_matrix_protocol(receipt)?;
    Ok(ContextPlaneCompatSourceBinding {
        report_id: TAMPER_MATRIX_SOURCE_ID.to_string(),
        line_count: protocol.lines().count(),
        sha256: sha256_hex(protocol.as_bytes()),
        generation: CANONICAL_GENERATION,
        sequence: CANONICAL_GENERATION,
    })
}

fn legacy_tamper_matrix_protocol(
    receipt: &ContextPlaneTamperMatrixReceipt,
) -> Result<String, String> {
    if !receipt.has_integrity() {
        return Err("context-plane tamper matrix failed legacy projection integrity".to_string());
    }
    let prefix = "context-plane-operator-approval-packet-digest-tamper-matrix";
    let mut protocol = String::new();
    protocol.push_str(prefix);
    protocol.push_str("=pass\n");
    for case in &receipt.cases {
        if !case.rejected {
            return Err(format!(
                "context-plane tamper case was not rejected: {}",
                case.case_id
            ));
        }
        protocol.push_str(prefix);
        protocol.push('.');
        protocol.push_str(&case.case_id.replace('_', "-"));
        protocol.push_str("=reject\n");
    }
    protocol.push_str(prefix);
    protocol.push_str(".runtime-activation=disabled\n");
    protocol.push_str(
        "Hepta context plane operator approval packet digest tamper matrix gate passed\n",
    );
    Ok(protocol)
}

fn source_binding_has_shape_integrity(source: &ContextPlaneCompatSourceBinding) -> bool {
    !source.report_id.is_empty()
        && source.line_count > 0
        && source.generation > 0
        && source.sequence > 0
        && source.sha256.len() == 64
        && source.sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn serialized_enum_id<T: Serialize>(value: &T) -> Result<String, String> {
    serde_json::to_value(value)
        .map_err(|error| format!("cannot serialize context-plane enum id: {error}"))?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| "context-plane enum id is not a string".to_string())
}

fn legacy_protocol_fields(
    report: &ContextPlaneCompatReport,
) -> Result<Vec<(String, Value)>, String> {
    match &report.payload {
        ContextPlaneCompatPayload::ActivationBlockerMatrix { matrix, .. } => {
            legacy_activation_protocol_fields(matrix, &report.forward_schema_delta)
        }
        ContextPlaneCompatPayload::OperatorApprovalPacket { packet } => {
            legacy_packet_protocol_fields(packet, &report.forward_schema_delta)
        }
        ContextPlaneCompatPayload::NegativeExport {
            payload_light,
            activation_command_present,
        } => {
            let prefix = "context-plane-operator-approval-packet-negative-export";
            Ok(vec![
                legacy_field(prefix, "", "pass"),
                legacy_field(
                    prefix,
                    "activation-command",
                    if *activation_command_present {
                        "present"
                    } else {
                        "absent"
                    },
                ),
                legacy_field(
                    prefix,
                    "payload-light",
                    if *payload_light { "pass" } else { "fail" },
                ),
                legacy_field(prefix, "runtime-activation", "disabled"),
            ])
        }
        ContextPlaneCompatPayload::CanonicalExportDigest {
            combined_report_line_count,
            combined_report_sha256,
        } => {
            let prefix = "context-plane-operator-approval-packet-canonical-export-digest";
            let [approval, negative] = report.sources.as_slice() else {
                return Err("canonical export legacy projection source count drifted".to_string());
            };
            Ok(vec![
                legacy_field(prefix, "", "pass"),
                legacy_field(prefix, "schema", 1_u64),
                legacy_field(prefix, "approval-report-lines", approval.line_count),
                legacy_field(prefix, "approval-report-sha256", approval.sha256.clone()),
                legacy_field(prefix, "negative-export-report-lines", negative.line_count),
                legacy_field(
                    prefix,
                    "negative-export-report-sha256",
                    negative.sha256.clone(),
                ),
                legacy_field(prefix, "combined-report-lines", *combined_report_line_count),
                legacy_field(
                    prefix,
                    "combined-report-sha256",
                    combined_report_sha256.clone(),
                ),
                legacy_field(prefix, "runtime-activation", "disabled"),
                legacy_field(prefix, "operator-activation", "disabled"),
            ])
        }
        ContextPlaneCompatPayload::Freshness {
            approval_readiness_sequence,
            current_readiness_sequence,
            expires_after_sequence,
            max_replay_age_sequences,
            stale_sequence_rejected,
            expired_sequence_rejected,
            future_sequence_rejected,
            digest_replay_rejected,
        } => {
            let prefix = "context-plane-operator-approval-packet-freshness";
            let [source] = report.sources.as_slice() else {
                return Err("freshness legacy projection source count drifted".to_string());
            };
            Ok(vec![
                legacy_field(prefix, "", "pass"),
                legacy_field(prefix, "schema", 1_u64),
                legacy_field(
                    prefix,
                    "source-canonical-digest-report-lines",
                    source.line_count,
                ),
                legacy_field(
                    prefix,
                    "source-canonical-digest-report-sha256",
                    source.sha256.clone(),
                ),
                legacy_field(
                    prefix,
                    "approval-readiness-sequence",
                    *approval_readiness_sequence,
                ),
                legacy_field(
                    prefix,
                    "current-readiness-sequence",
                    *current_readiness_sequence,
                ),
                legacy_field(prefix, "expires-after-sequence", *expires_after_sequence),
                legacy_field(
                    prefix,
                    "max-replay-age-sequences",
                    *max_replay_age_sequences,
                ),
                legacy_rejection_field(prefix, "stale-sequence", *stale_sequence_rejected),
                legacy_rejection_field(prefix, "expired-sequence", *expired_sequence_rejected),
                legacy_rejection_field(prefix, "future-sequence", *future_sequence_rejected),
                legacy_rejection_field(prefix, "digest-replay", *digest_replay_rejected),
                legacy_field(prefix, "runtime-activation", "disabled"),
                legacy_field(prefix, "operator-activation", "disabled"),
            ])
        }
        ContextPlaneCompatPayload::FreshnessDependencyChain {
            readiness_chain_generation,
            freshness_source_sequence,
            stale_source_rejected,
            mixed_generation_rejected,
            source_digest_mismatch_rejected,
            tamper_matrix_replay_rejected,
            ..
        } => {
            let prefix = "context-plane-operator-approval-packet-freshness-dependency-chain";
            let [approval, negative, canonical, tamper, freshness] = report.sources.as_slice()
            else {
                return Err("dependency legacy projection source count drifted".to_string());
            };
            Ok(vec![
                legacy_field(prefix, "", "pass"),
                legacy_field(prefix, "schema", 1_u64),
                legacy_field(prefix, "approval-report-lines", approval.line_count),
                legacy_field(prefix, "approval-report-sha256", approval.sha256.clone()),
                legacy_field(prefix, "negative-export-report-lines", negative.line_count),
                legacy_field(
                    prefix,
                    "negative-export-report-sha256",
                    negative.sha256.clone(),
                ),
                legacy_field(
                    prefix,
                    "canonical-digest-report-lines",
                    canonical.line_count,
                ),
                legacy_field(
                    prefix,
                    "canonical-digest-report-sha256",
                    canonical.sha256.clone(),
                ),
                legacy_field(prefix, "tamper-matrix-report-lines", tamper.line_count),
                legacy_field(prefix, "tamper-matrix-report-sha256", tamper.sha256.clone()),
                legacy_field(prefix, "freshness-report-lines", freshness.line_count),
                legacy_field(prefix, "freshness-report-sha256", freshness.sha256.clone()),
                legacy_field(
                    prefix,
                    "readiness-chain-generation",
                    *readiness_chain_generation,
                ),
                legacy_field(
                    prefix,
                    "freshness-source-sequence",
                    *freshness_source_sequence,
                ),
                legacy_rejection_field(prefix, "stale-source", *stale_source_rejected),
                legacy_rejection_field(prefix, "mixed-generation", *mixed_generation_rejected),
                legacy_rejection_field(
                    prefix,
                    "source-digest-mismatch",
                    *source_digest_mismatch_rejected,
                ),
                legacy_rejection_field(
                    prefix,
                    "tamper-matrix-replay",
                    *tamper_matrix_replay_rejected,
                ),
                legacy_field(prefix, "runtime-activation", "disabled"),
                legacy_field(prefix, "operator-activation", "disabled"),
            ])
        }
        ContextPlaneCompatPayload::FreshnessDependencyChainCanonicalDigest {
            readiness_chain_generation,
            source_readiness_chain_generation,
            source_freshness_sequence,
            reordered_dependency_rows_rejected,
            mismatched_upstream_digest_rejected,
            mixed_generation_replay_rejected,
            mixed_sequence_replay_rejected,
            payload_field_injection_rejected,
            write_activation_field_injection_rejected,
        } => {
            let prefix = "context-plane-operator-approval-packet-freshness-dependency-chain-canonical-digest";
            let [source] = report.sources.as_slice() else {
                return Err("dependency digest legacy projection source count drifted".to_string());
            };
            Ok(vec![
                legacy_field(prefix, "", "pass"),
                legacy_field(prefix, "schema", 1_u64),
                legacy_field(prefix, "dependency-chain-report-lines", source.line_count),
                legacy_field(
                    prefix,
                    "dependency-chain-report-sha256",
                    source.sha256.clone(),
                ),
                legacy_field(
                    prefix,
                    "readiness-chain-generation",
                    *readiness_chain_generation,
                ),
                legacy_field(
                    prefix,
                    "source-readiness-chain-generation",
                    *source_readiness_chain_generation,
                ),
                legacy_field(
                    prefix,
                    "source-freshness-sequence",
                    *source_freshness_sequence,
                ),
                legacy_rejection_field(
                    prefix,
                    "reordered-dependency-rows",
                    *reordered_dependency_rows_rejected,
                ),
                legacy_rejection_field(
                    prefix,
                    "mismatched-upstream-digest",
                    *mismatched_upstream_digest_rejected,
                ),
                legacy_rejection_field(
                    prefix,
                    "mixed-generation-replay",
                    *mixed_generation_replay_rejected,
                ),
                legacy_rejection_field(
                    prefix,
                    "mixed-sequence-replay",
                    *mixed_sequence_replay_rejected,
                ),
                legacy_rejection_field(
                    prefix,
                    "payload-field-injection",
                    *payload_field_injection_rejected,
                ),
                legacy_rejection_field(
                    prefix,
                    "write-activation-field-injection",
                    *write_activation_field_injection_rejected,
                ),
                legacy_field(prefix, "runtime-activation", "disabled"),
                legacy_field(prefix, "operator-activation", "disabled"),
            ])
        }
        ContextPlaneCompatPayload::FreshnessDependencyChainExpiryDrift {
            readiness_chain_generation,
            source_readiness_chain_generation,
            source_dependency_chain_generation,
            source_freshness_sequence,
            readiness_window_start_sequence,
            readiness_window_current_sequence,
            readiness_window_expires_after_sequence,
            readiness_window_max_drift_sequences,
            expired_window_rejected,
            window_start_drift_rejected,
            window_current_drift_rejected,
            window_expiry_drift_rejected,
            source_digest_replay_rejected,
            payload_field_injection_rejected,
            write_activation_field_injection_rejected,
        } => {
            let prefix =
                "context-plane-operator-approval-packet-freshness-dependency-chain-expiry-drift";
            let [source] = report.sources.as_slice() else {
                return Err("expiry drift legacy projection source count drifted".to_string());
            };
            Ok(vec![
                legacy_field(prefix, "", "pass"),
                legacy_field(prefix, "schema", 1_u64),
                legacy_field(
                    prefix,
                    "source-canonical-digest-report-lines",
                    source.line_count,
                ),
                legacy_field(
                    prefix,
                    "source-canonical-digest-report-sha256",
                    source.sha256.clone(),
                ),
                legacy_field(
                    prefix,
                    "readiness-chain-generation",
                    *readiness_chain_generation,
                ),
                legacy_field(
                    prefix,
                    "source-readiness-chain-generation",
                    *source_readiness_chain_generation,
                ),
                legacy_field(
                    prefix,
                    "source-dependency-chain-generation",
                    *source_dependency_chain_generation,
                ),
                legacy_field(
                    prefix,
                    "source-freshness-sequence",
                    *source_freshness_sequence,
                ),
                legacy_field(
                    prefix,
                    "readiness-window-start-sequence",
                    *readiness_window_start_sequence,
                ),
                legacy_field(
                    prefix,
                    "readiness-window-current-sequence",
                    *readiness_window_current_sequence,
                ),
                legacy_field(
                    prefix,
                    "readiness-window-expires-after-sequence",
                    *readiness_window_expires_after_sequence,
                ),
                legacy_field(
                    prefix,
                    "readiness-window-max-drift-sequences",
                    *readiness_window_max_drift_sequences,
                ),
                legacy_rejection_field(prefix, "expired-window", *expired_window_rejected),
                legacy_rejection_field(prefix, "window-start-drift", *window_start_drift_rejected),
                legacy_rejection_field(
                    prefix,
                    "window-current-drift",
                    *window_current_drift_rejected,
                ),
                legacy_rejection_field(
                    prefix,
                    "window-expiry-drift",
                    *window_expiry_drift_rejected,
                ),
                legacy_rejection_field(
                    prefix,
                    "source-digest-replay",
                    *source_digest_replay_rejected,
                ),
                legacy_rejection_field(
                    prefix,
                    "payload-field-injection",
                    *payload_field_injection_rejected,
                ),
                legacy_rejection_field(
                    prefix,
                    "write-activation-field-injection",
                    *write_activation_field_injection_rejected,
                ),
                legacy_field(prefix, "runtime-activation", "disabled"),
                legacy_field(prefix, "operator-activation", "disabled"),
            ])
        }
    }
}

fn legacy_field(prefix: &str, suffix: &str, value: impl Into<Value>) -> (String, Value) {
    let key = if suffix.is_empty() {
        prefix.to_string()
    } else {
        format!("{prefix}.{suffix}")
    };
    (key, value.into())
}

fn legacy_rejection_field(prefix: &str, suffix: &str, rejected: bool) -> (String, Value) {
    legacy_field(prefix, suffix, if rejected { "reject" } else { "accept" })
}

fn legacy_activation_protocol_fields(
    matrix: &ContextPlaneActivationBlockerMatrix,
    delta: &ContextPlaneForwardSchemaDelta,
) -> Result<Vec<(String, Value)>, String> {
    let prefix = "context-plane-activation-blockers";
    let rows = legacy_activation_rows(matrix)?;
    let mut fields = vec![legacy_field(prefix, "", "pass")];
    for suffix in LEGACY_ACTIVATION_FIELD_ORDER_V20.lines() {
        if suffix.is_empty() {
            continue;
        }
        fields.push(legacy_field(
            prefix,
            suffix,
            legacy_activation_field_value(matrix, &rows, delta, suffix)?,
        ));
    }
    Ok(fields)
}

fn legacy_packet_protocol_fields(
    packet: &ContextPlaneOperatorApprovalPacket,
    delta: &ContextPlaneForwardSchemaDelta,
) -> Result<Vec<(String, Value)>, String> {
    let prefix = "context-plane-operator-approval-packet";
    let packet_value = serde_json::to_value(packet)
        .map_err(|error| format!("cannot serialize context-plane operator packet: {error}"))?;
    let packet_object = packet_value
        .as_object()
        .ok_or_else(|| "context-plane operator packet is not an object".to_string())?;
    let mut fields = vec![legacy_field(prefix, "", "pass")];
    for suffix in LEGACY_PACKET_FIELD_ORDER_V19.lines() {
        if suffix.is_empty() {
            continue;
        }
        fields.push(legacy_field(
            prefix,
            suffix,
            legacy_packet_field_value(packet_object, delta, suffix)?,
        ));
    }
    Ok(fields)
}

fn legacy_activation_rows(
    matrix: &ContextPlaneActivationBlockerMatrix,
) -> Result<BTreeMap<String, Value>, String> {
    let rows = matrix
        .rows
        .iter()
        .map(|row| {
            let target = serialized_enum_id(&row.target)?;
            let value = serde_json::to_value(row)
                .map_err(|error| format!("cannot serialize context-plane matrix row: {error}"))?;
            Ok((target, value))
        })
        .collect::<Result<BTreeMap<_, _>, String>>()?;
    let legacy_rows = rows
        .into_iter()
        .filter(|(target, _)| target != FORWARD_ACTIVATION_TARGET_ID)
        .collect::<BTreeMap<_, _>>();
    if legacy_rows.len() != LEGACY_ACTIVATION_TARGET_IDS.len()
        || !LEGACY_ACTIVATION_TARGET_IDS
            .iter()
            .all(|target| legacy_rows.contains_key(*target))
    {
        return Err("legacy activation target inventory drifted".to_string());
    }
    Ok(legacy_rows)
}

fn legacy_activation_field_value(
    matrix: &ContextPlaneActivationBlockerMatrix,
    rows: &BTreeMap<String, Value>,
    delta: &ContextPlaneForwardSchemaDelta,
    suffix: &str,
) -> Result<Value, String> {
    match suffix {
        "schema" => return Ok(LEGACY_ACTIVATION_SCHEMA_VERSION.into()),
        "rows" => return Ok(LEGACY_ACTIVATION_TARGET_IDS.len().into()),
        "satisfied" => {
            let count = rows
                .values()
                .filter(|row| row.get("threshold_satisfied").and_then(Value::as_bool) == Some(true))
                .count();
            return Ok(count.into());
        }
        "blockers" => {
            return Ok((matrix.blocker_count - delta.added_activation_target_ids.len()).into());
        }
        "recall-quality-blocking-reason-count" => {
            let reasons = row_array(
                rows,
                "recall_quality_gate",
                "recall_quality_blocking_reasons",
            )?;
            return Ok(reasons.len().into());
        }
        "recall-quality-blocking-reasons" => {
            let reasons = row_array(
                rows,
                "recall_quality_gate",
                "recall_quality_blocking_reasons",
            )?;
            let values = reasons
                .iter()
                .map(|reason| {
                    reason.as_str().map(str::to_string).ok_or_else(|| {
                        "context-plane recall-quality reason is not a string".to_string()
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            return Ok(if values.is_empty() {
                Value::String("none".to_string())
            } else {
                Value::String(values.join(","))
            });
        }
        _ => {}
    }

    let matrix_field = match suffix {
        "activation-allowed" => Some(matrix.activation_allowed),
        "runtime-activation" => Some(matrix.runtime_activation),
        "adaptive-allocator-runtime-activation" => {
            Some(matrix.adaptive_allocator_runtime_activation)
        }
        "source-aware-runtime-activation" => Some(matrix.source_aware_runtime_activation),
        "production-write" => Some(matrix.production_write),
        "graph-write" => Some(matrix.graph_write),
        "prompt-assembly-change" => Some(matrix.prompt_assembly_change),
        "operator-activation" => Some(matrix.operator_activation_allowed),
        _ => None,
    };
    if let Some(enabled) = matrix_field {
        return Ok(Value::String(
            if enabled { "enabled" } else { "disabled" }.to_string(),
        ));
    }

    const GROUPS: &[(&str, &str, &str)] = &[
        (
            "memory-namespace-policy.",
            "memory_namespace_policy",
            "memory_namespace_policy_",
        ),
        (
            "memory-write-chain-readiness.",
            "memory_write_chain_readiness",
            "memory_write_chain_",
        ),
        (
            "memory-write-chain-receipt-freshness.",
            "memory_write_chain_receipt_freshness",
            "memory_write_chain_receipt_",
        ),
        (
            "memory-temporal-graph-shadow-store.",
            "memory_temporal_graph_shadow_store",
            "memory_temporal_graph_shadow_store_",
        ),
        (
            "memory-temporal-graph-shadow-replay.",
            "memory_temporal_graph_shadow_replay",
            "memory_temporal_graph_shadow_replay_",
        ),
        (
            "memory-temporal-graph-shadow-traversal-diff.",
            "memory_temporal_graph_shadow_traversal_diff",
            "memory_temporal_graph_shadow_traversal_diff_",
        ),
        (
            "memory-temporal-graph-shadow-traversal-quality.",
            "memory_temporal_graph_shadow_traversal_quality",
            "memory_temporal_graph_shadow_traversal_quality_",
        ),
        (
            "memory-temporal-graph-shadow-retrieval-canary-guard.",
            "memory_temporal_graph_shadow_retrieval_canary_guard",
            "memory_temporal_graph_shadow_retrieval_canary_guard_",
        ),
        (
            "memory-temporal-graph-shadow-retrieval-rollback-kill-switch.",
            "memory_temporal_graph_shadow_retrieval_rollback_kill_switch",
            "memory_temporal_graph_shadow_retrieval_rollback_kill_switch_",
        ),
        (
            "ranked-recall.",
            "memory_ranked_recall_shadow_eval",
            "ranked_recall_",
        ),
        (
            "memory-provider-v2.",
            "memory_provider_v2_boundary",
            "memory_provider_v2_",
        ),
        (
            "canary-promotion.",
            "memory_shadow_canary_promotion_readiness",
            "canary_promotion_",
        ),
    ];
    for (legacy_prefix, target, rust_prefix) in GROUPS {
        let Some(legacy_leaf) = suffix.strip_prefix(legacy_prefix) else {
            continue;
        };
        let mut rust_leaf = legacy_leaf.replace('-', "_");
        rust_leaf = legacy_activation_alias(legacy_prefix, &rust_leaf).to_string();
        let rust_key = format!("{rust_prefix}{rust_leaf}");
        if *target == "memory_provider_v2_boundary"
            && matches!(
                rust_key.as_str(),
                "memory_provider_v2_candidate_count"
                    | "memory_provider_v2_operator_review_required_count"
            )
        {
            return Ok(delta.legacy_provider_v2_candidate_count().into());
        }
        let value = row_field(rows, target, &rust_key)?.clone();
        if rust_key.ends_with("_check_pass") {
            return legacy_check_value(&value);
        }
        return legacy_scalar(value, &rust_key);
    }

    let target = suffix.replace('-', "_");
    let row = rows
        .get(&target)
        .and_then(Value::as_object)
        .ok_or_else(|| format!("unknown legacy activation field: {suffix}"))?;
    let threshold_satisfied = row
        .get("threshold_satisfied")
        .and_then(Value::as_bool)
        .ok_or_else(|| format!("activation row {target} lacks threshold_satisfied"))?;
    let observed_status = object_string(row, "observed_status")?;
    let blocker_reason = object_string(row, "blocker_reason")?;
    let rendered = if threshold_satisfied && observed_status == "shadow" {
        "shadow-threshold-pass".to_string()
    } else if threshold_satisfied {
        observed_status.to_string()
    } else {
        format!("blocked:{blocker_reason}")
    };
    Ok(Value::String(rendered))
}

fn legacy_activation_alias<'a>(group: &str, leaf: &'a str) -> &'a str {
    match (group, leaf) {
        ("canary-promotion.", "promotion_blocker_count") => "blocker_count",
        ("canary-promotion.", "readiness_check") => "readiness_check_pass",
        ("canary-promotion.", "negative_rehearsal_check") => "negative_rehearsal_check_pass",
        ("canary-promotion.", "audit_digest_check") => "audit_digest_check_pass",
        ("canary-promotion.", "audit_freshness_check") => "audit_freshness_check_pass",
        ("memory-provider-v2.", "query_check") => "query_check_pass",
        ("memory-provider-v2.", "update_context_check") => "update_context_check_pass",
        ("memory-provider-v2.", "propose_write_check") => "propose_write_check_pass",
        ("memory-provider-v2.", "add_check") => "add_check_pass",
        ("memory-provider-v2.", "clear_check") => "clear_check_pass",
        ("memory-provider-v2.", "close_check") => "close_check_pass",
        ("memory-write-chain-receipt-freshness.", "receipt_required_count") => "required_count",
        ("memory-write-chain-receipt-freshness.", "receipt_projected_count") => "projected_count",
        ("memory-write-chain-receipt-freshness.", "receipt_digest_count") => "digest_count",
        ("memory-write-chain-receipt-freshness.", "recorded_receipt_count") => "recorded_count",
        ("memory-write-chain-receipt-freshness.", "persisted_receipt_count") => "persisted_count",
        ("memory-temporal-graph-shadow-replay.", "replay_guard_pass_count") => "guard_pass_count",
        ("memory-temporal-graph-shadow-store.", "supersede_edge_count") => "supersedes_edge_count",
        ("ranked-recall.", "lexical_bm25_check") => "lexical_bm25_check_pass",
        ("ranked-recall.", "recency_check") => "recency_check_pass",
        ("ranked-recall.", "source_authority_check") => "source_authority_check_pass",
        ("ranked-recall.", "temporal_validity_check") => "temporal_validity_check_pass",
        ("ranked-recall.", "feedback_check") => "feedback_check_pass",
        _ => leaf,
    }
}

fn legacy_packet_field_value(
    packet: &serde_json::Map<String, Value>,
    delta: &ContextPlaneForwardSchemaDelta,
    suffix: &str,
) -> Result<Value, String> {
    match suffix {
        "schema" => return Ok(LEGACY_OPERATOR_PACKET_SCHEMA_VERSION.into()),
        "dry-run" => return legacy_enabled_value(packet_field(packet, "dry_run_only")?),
        "approval-required" => {
            return legacy_enabled_value(packet_field(packet, "approval_required")?);
        }
        "activation-command" => {
            let present = packet_field(packet, "activation_command_present")?
                .as_bool()
                .ok_or_else(|| "activation_command_present is not boolean".to_string())?;
            return Ok(Value::String(
                if present { "present" } else { "absent" }.to_string(),
            ));
        }
        "rows" => return Ok(LEGACY_ACTIVATION_TARGET_IDS.len().into()),
        "satisfied" => return Ok(packet_field(packet, "threshold_satisfied_count")?.clone()),
        "blockers" => {
            let current = packet_field(packet, "blocker_count")?
                .as_u64()
                .ok_or_else(|| "packet blocker_count is not numeric".to_string())?;
            return Ok((current - delta.added_activation_target_ids.len() as u64).into());
        }
        "threshold.required-ready" => {
            return Ok((LEGACY_ACTIVATION_TARGET_IDS.len() - 1).into());
        }
        "threshold.required-shadow" => return Ok(1_u64.into()),
        "recall-quality-blocking-reason-count" => {
            return Ok(packet_field(packet, "recall_quality_blocking_reason_count")?.clone());
        }
        "recall-quality-blocking-reasons" => {
            let values = packet_field(packet, "recall_quality_blocking_reason_counts")?
                .as_array()
                .ok_or_else(|| "packet recall-quality reasons are not an array".to_string())?;
            if values.is_empty() {
                return Ok(Value::String("none".to_string()));
            }
            let rendered = values
                .iter()
                .map(|entry| {
                    let object = entry.as_object().ok_or_else(|| {
                        "packet recall-quality reason is not an object".to_string()
                    })?;
                    Ok(format!(
                        "{}:{}",
                        object_string(object, "reason")?,
                        object_u64(object, "count")?
                    ))
                })
                .collect::<Result<Vec<_>, String>>()?;
            return Ok(Value::String(rendered.join(",")));
        }
        "required-scopes" => {
            let scopes = packet_field(packet, "required_approval_scopes")?
                .as_array()
                .ok_or_else(|| "packet approval scopes are not an array".to_string())?;
            return Ok(scopes.len().into());
        }
        _ => {}
    }

    if let Some(reason) = suffix.strip_prefix("blocker.") {
        let reason = reason.replace('-', "_");
        if reason == "temporal_graph_shadow_retrieval_promotion_readiness_shadow_only" {
            return Err("forward-only blocker leaked into legacy packet projection".to_string());
        }
        let counts = packet_field(packet, "blocker_reason_counts")?
            .as_array()
            .ok_or_else(|| "packet blocker reasons are not an array".to_string())?;
        let matching = counts
            .iter()
            .filter_map(Value::as_object)
            .filter(|entry| entry.get("reason").and_then(Value::as_str) == Some(reason.as_str()))
            .collect::<Vec<_>>();
        let [entry] = matching.as_slice() else {
            return Err(format!("legacy packet blocker reason drifted: {reason}"));
        };
        return Ok(object_u64(entry, "count")?.into());
    }

    if let Some(scope) = suffix.strip_prefix("scope.") {
        let scope = scope.replace('-', "_");
        let scopes = packet_field(packet, "required_approval_scopes")?
            .as_array()
            .ok_or_else(|| "packet approval scopes are not an array".to_string())?;
        let count = scopes
            .iter()
            .filter(|candidate| candidate.as_str() == Some(scope.as_str()))
            .count();
        if count != 1 {
            return Err(format!("legacy packet approval scope drifted: {scope}"));
        }
        return Ok(Value::String("required".to_string()));
    }

    let tail = match suffix {
        "runtime-activation" => Some("runtime_activation"),
        "adaptive-allocator-runtime-activation" => Some("adaptive_allocator_runtime_activation"),
        "source-aware-runtime-activation" => Some("source_aware_runtime_activation"),
        "production-write" => Some("production_write"),
        "graph-write" => Some("graph_write"),
        "prompt-assembly-change" => Some("prompt_assembly_change"),
        "operator-activation" => Some("operator_activation_allowed"),
        _ => None,
    };
    if let Some(key) = tail {
        return legacy_enabled_value(packet_field(packet, key)?);
    }

    const GROUPS: &[(&str, &str)] = &[
        ("canary-promotion.", "canary_promotion_"),
        ("memory-provider-v2.", "memory_provider_v2_"),
        ("memory-namespace-policy.", "memory_namespace_policy_"),
        ("memory-write-chain-readiness.", "memory_write_chain_"),
        (
            "memory-write-chain-receipt-freshness.",
            "memory_write_chain_receipt_",
        ),
        (
            "memory-temporal-graph-shadow-store.",
            "memory_temporal_graph_shadow_store_",
        ),
        (
            "memory-temporal-graph-shadow-replay.",
            "memory_temporal_graph_shadow_replay_",
        ),
        (
            "memory-temporal-graph-shadow-traversal-diff.",
            "memory_temporal_graph_shadow_traversal_diff_",
        ),
        (
            "memory-temporal-graph-shadow-traversal-quality.",
            "memory_temporal_graph_shadow_traversal_quality_",
        ),
        (
            "memory-temporal-graph-shadow-retrieval-canary-guard.",
            "memory_temporal_graph_shadow_retrieval_canary_guard_",
        ),
        (
            "memory-temporal-graph-shadow-retrieval-rollback-kill-switch.",
            "memory_temporal_graph_shadow_retrieval_rollback_kill_switch_",
        ),
        ("ranked-recall.", "ranked_recall_"),
    ];
    for (legacy_prefix, rust_prefix) in GROUPS {
        let Some(legacy_leaf) = suffix.strip_prefix(legacy_prefix) else {
            continue;
        };
        let mut rust_leaf = legacy_leaf.replace('-', "_");
        rust_leaf = legacy_activation_alias(legacy_prefix, &rust_leaf).to_string();
        if *legacy_prefix == "memory-write-chain-receipt-freshness." {
            rust_leaf = match rust_leaf.as_str() {
                "receipt_required_count" => "required_count".to_string(),
                "receipt_projected_count" => "projected_count".to_string(),
                "receipt_digest_count" => "digest_count".to_string(),
                other => other.to_string(),
            };
        }
        let rust_key = format!("{rust_prefix}{rust_leaf}");
        if *legacy_prefix == "memory-provider-v2."
            && matches!(
                rust_key.as_str(),
                "memory_provider_v2_candidate_count"
                    | "memory_provider_v2_operator_review_required_count"
            )
        {
            return Ok(delta.legacy_provider_v2_candidate_count().into());
        }
        let value = packet_field(packet, &rust_key)?.clone();
        if rust_key.ends_with("_check_pass") {
            return legacy_check_value(&value);
        }
        return legacy_scalar(value, &rust_key);
    }
    Err(format!("unknown legacy packet field: {suffix}"))
}

fn legacy_enabled_value(value: &Value) -> Result<Value, String> {
    let enabled = value
        .as_bool()
        .ok_or_else(|| "legacy enabled/disabled source is not boolean".to_string())?;
    Ok(Value::String(
        if enabled { "enabled" } else { "disabled" }.to_string(),
    ))
}

fn legacy_check_value(value: &Value) -> Result<Value, String> {
    let passed = value
        .as_bool()
        .ok_or_else(|| "legacy pass/fail source is not boolean".to_string())?;
    Ok(Value::String(
        if passed { "pass" } else { "fail" }.to_string(),
    ))
}

fn legacy_scalar(value: Value, key: &str) -> Result<Value, String> {
    if matches!(value, Value::Null | Value::Array(_) | Value::Object(_)) {
        return Err(format!("legacy field {key} is not scalar"));
    }
    Ok(value)
}

fn row_field<'a>(
    rows: &'a BTreeMap<String, Value>,
    target: &str,
    key: &str,
) -> Result<&'a Value, String> {
    rows.get(target)
        .and_then(Value::as_object)
        .and_then(|row| row.get(key))
        .ok_or_else(|| format!("context-plane row {target} lacks {key}"))
}

fn row_array<'a>(
    rows: &'a BTreeMap<String, Value>,
    target: &str,
    key: &str,
) -> Result<&'a Vec<Value>, String> {
    row_field(rows, target, key)?
        .as_array()
        .ok_or_else(|| format!("context-plane row {target} field {key} is not an array"))
}

fn packet_field<'a>(
    packet: &'a serde_json::Map<String, Value>,
    key: &str,
) -> Result<&'a Value, String> {
    packet
        .get(key)
        .ok_or_else(|| format!("context-plane packet lacks {key}"))
}

fn object_string<'a>(
    object: &'a serde_json::Map<String, Value>,
    key: &str,
) -> Result<&'a str, String> {
    object
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("context-plane object field {key} is not a string"))
}

fn object_u64(object: &serde_json::Map<String, Value>, key: &str) -> Result<u64, String> {
    object
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("context-plane object field {key} is not numeric"))
}

fn flattened_serializable_fields<T: Serialize>(
    source: &T,
) -> Result<BTreeMap<String, Value>, String> {
    let value = serde_json::to_value(source)
        .map_err(|error| format!("cannot serialize context-plane source binding: {error}"))?;
    let mut fields = BTreeMap::new();
    flatten_json("source", &value, &mut fields)?;
    Ok(fields)
}

fn flatten_json(
    prefix: &str,
    value: &Value,
    output: &mut BTreeMap<String, Value>,
) -> Result<(), String> {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            if prefix.is_empty() {
                return Err("context-plane legacy projection has an empty key".to_string());
            }
            output.insert(prefix.to_string(), value.clone());
        }
        Value::Array(values) => {
            output.insert(
                format!("{prefix}.count"),
                Value::Number(values.len().into()),
            );
            for (index, value) in values.iter().enumerate() {
                flatten_json(&format!("{prefix}.{index}"), value, output)?;
            }
        }
        Value::Object(values) => {
            if values.is_empty() {
                output.insert(format!("{prefix}.present"), Value::Bool(false));
            }
            for (key, value) in values {
                if !key
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
                {
                    return Err(format!(
                        "context-plane typed source exposes unsafe legacy key segment: {key}"
                    ));
                }
                let child = if prefix.is_empty() {
                    key.clone()
                } else {
                    format!("{prefix}.{key}")
                };
                flatten_json(&child, value, output)?;
            }
        }
    }
    Ok(())
}

fn render_legacy_fields(fields: &BTreeMap<String, Value>) -> Result<String, String> {
    if fields.is_empty() {
        return Err("context-plane legacy projection is empty".to_string());
    }
    let mut protocol = String::new();
    for (key, value) in fields {
        if key.is_empty()
            || !key.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'_' | b'.' | b'-')
            })
        {
            return Err(format!(
                "context-plane legacy projection exposes unsafe key: {key}"
            ));
        }
        let rendered = match value {
            Value::Null => "null".to_string(),
            Value::Bool(value) => value.to_string(),
            Value::Number(value) => value.to_string(),
            Value::String(value) => {
                if value.contains(['\n', '\r', '=']) {
                    return Err(format!(
                        "context-plane legacy projection exposes an injectable value for {key}"
                    ));
                }
                value.clone()
            }
            Value::Array(_) | Value::Object(_) => {
                return Err(format!(
                    "context-plane legacy projection field {key} is not scalar"
                ));
            }
        };
        protocol.push_str(key);
        protocol.push('=');
        protocol.push_str(&rendered);
        protocol.push('\n');
    }
    Ok(protocol)
}

fn render_ordered_legacy_fields(fields: &[(String, Value)]) -> Result<String, String> {
    if fields.is_empty() {
        return Err("context-plane ordered legacy projection is empty".to_string());
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut protocol = String::new();
    for (key, value) in fields {
        if !seen.insert(key.as_str()) {
            return Err(format!(
                "context-plane ordered legacy projection repeats key: {key}"
            ));
        }
        if key.is_empty()
            || !key.bytes().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'_' | b'.' | b'-')
            })
        {
            return Err(format!(
                "context-plane ordered legacy projection exposes unsafe key: {key}"
            ));
        }
        let rendered = match value {
            Value::Null => "null".to_string(),
            Value::Bool(value) => value.to_string(),
            Value::Number(value) => value.to_string(),
            Value::String(value) => {
                if value.contains(['\n', '\r', '=']) {
                    return Err(format!(
                        "context-plane ordered legacy projection exposes an injectable value for {key}"
                    ));
                }
                value.clone()
            }
            Value::Array(_) | Value::Object(_) => {
                return Err(format!(
                    "context-plane ordered legacy projection field {key} is not scalar"
                ));
            }
        };
        protocol.push_str(key);
        protocol.push('=');
        protocol.push_str(&rendered);
        protocol.push('\n');
    }
    Ok(protocol)
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn sensitive_fields_are_closed(value: &Value) -> bool {
    match value {
        Value::Array(values) => values.iter().all(sensitive_fields_are_closed),
        Value::Object(values) => values.iter().all(|(key, value)| {
            let sensitive_boolean = matches!(
                key.as_str(),
                "activation_allowed"
                    | "activation_command_present"
                    | "adaptive_allocator_runtime_activation"
                    | "graph_write"
                    | "mutation_enabled"
                    | "operator_activation_allowed"
                    | "payload_forwarded"
                    | "production_authority_granted"
                    | "production_write"
                    | "prompt_assembly_change"
                    | "ready_for_live_execution"
                    | "runtime_activation"
                    | "source_aware_runtime_activation"
                    | "write_activation"
                    | "write_authority_granted"
            );
            let sensitive_count = key.ends_with("production_write_count")
                || key.ends_with("graph_write_count")
                || key.ends_with("rollback_write_count")
                || key.ends_with("route_opened_count");
            (!sensitive_boolean || value == &Value::Bool(false))
                && (!sensitive_count || value.as_u64() == Some(0))
                && sensitive_fields_are_closed(value)
        }),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => true,
    }
}

// Field names and order are the versioned legacy wire schema. Values are
// always derived from the typed Rust reports at runtime; no captured report
// body or value snapshot is retained here.
const LEGACY_ACTIVATION_FIELD_ORDER_V20: &str = r#"schema
rows
satisfied
blockers
source-registry
adaptive-budget-allocation
memory-taxonomy
memory-formation-receipts
memory-formation-queue
memory-namespace-policy
memory-namespace-policy.namespace-count
memory-namespace-policy.operator-approval-required-count
memory-namespace-policy.shadow-wal-required-count
memory-namespace-policy.readback-required-count
memory-namespace-policy.canary-required-count
memory-namespace-policy.rollback-supported-count
memory-namespace-policy.production-write-count
memory-namespace-policy.graph-write-count
memory-write-chain-readiness
memory-write-chain-readiness.namespace-count
memory-write-chain-readiness.stage-required-count
memory-write-chain-readiness.stage-pass-count
memory-write-chain-readiness.propose-write-ready-count
memory-write-chain-readiness.policy-approval-ready-count
memory-write-chain-readiness.operator-approval-ready-count
memory-write-chain-readiness.shadow-wal-ready-count
memory-write-chain-readiness.readback-ready-count
memory-write-chain-readiness.canary-ready-count
memory-write-chain-readiness.rollback-ready-count
memory-write-chain-readiness.production-write-count
memory-write-chain-readiness.graph-write-count
memory-write-chain-receipt-freshness
memory-write-chain-receipt-freshness.namespace-count
memory-write-chain-receipt-freshness.receipt-required-count
memory-write-chain-receipt-freshness.receipt-projected-count
memory-write-chain-receipt-freshness.receipt-digest-count
memory-write-chain-receipt-freshness.freshness-pass-count
memory-write-chain-receipt-freshness.replay-guard-pass-count
memory-write-chain-receipt-freshness.stale-replay-rejected-count
memory-write-chain-receipt-freshness.recorded-receipt-count
memory-write-chain-receipt-freshness.persisted-receipt-count
memory-write-chain-receipt-freshness.production-write-count
memory-write-chain-receipt-freshness.graph-write-count
memory-temporal-facts
memory-temporal-fact-graph
memory-temporal-graph-shadow-eval
memory-temporal-graph-shadow-store
memory-temporal-graph-shadow-store.node-count
memory-temporal-graph-shadow-store.edge-count
memory-temporal-graph-shadow-store.provenance-edge-count
memory-temporal-graph-shadow-store.validity-window-edge-count
memory-temporal-graph-shadow-store.supersede-edge-count
memory-temporal-graph-shadow-store.invalidated-node-count
memory-temporal-graph-shadow-store.stage-required-count
memory-temporal-graph-shadow-store.stage-projected-count
memory-temporal-graph-shadow-store.digest-count
memory-temporal-graph-shadow-store.freshness-pass-count
memory-temporal-graph-shadow-store.replay-guard-pass-count
memory-temporal-graph-shadow-store.stale-replay-rejected-count
memory-temporal-graph-shadow-store.operator-approval-required-count
memory-temporal-graph-shadow-store.operator-approval-recorded-count
memory-temporal-graph-shadow-store.recorded-receipt-count
memory-temporal-graph-shadow-store.persisted-receipt-count
memory-temporal-graph-shadow-store.production-write-count
memory-temporal-graph-shadow-store.graph-write-count
memory-temporal-graph-shadow-replay
memory-temporal-graph-shadow-replay.node-count
memory-temporal-graph-shadow-replay.edge-count
memory-temporal-graph-shadow-replay.provenance-count
memory-temporal-graph-shadow-replay.bitemporal-validity-count
memory-temporal-graph-shadow-replay.fact-invalidation-count
memory-temporal-graph-shadow-replay.supersede-tombstone-count
memory-temporal-graph-shadow-replay.stage-required-count
memory-temporal-graph-shadow-replay.stage-projected-count
memory-temporal-graph-shadow-replay.digest-count
memory-temporal-graph-shadow-replay.freshness-pass-count
memory-temporal-graph-shadow-replay.replay-guard-pass-count
memory-temporal-graph-shadow-replay.stale-replay-rejected-count
memory-temporal-graph-shadow-replay.operator-approval-required-count
memory-temporal-graph-shadow-replay.operator-approval-recorded-count
memory-temporal-graph-shadow-replay.recorded-receipt-count
memory-temporal-graph-shadow-replay.persisted-receipt-count
memory-temporal-graph-shadow-replay.production-write-count
memory-temporal-graph-shadow-replay.graph-write-count
memory-temporal-graph-shadow-traversal-diff
memory-temporal-graph-shadow-traversal-diff.production-selection-count
memory-temporal-graph-shadow-traversal-diff.lexical-bm25-candidate-count
memory-temporal-graph-shadow-traversal-diff.semantic-candidate-count
memory-temporal-graph-shadow-traversal-diff.graph-traversal-candidate-count
memory-temporal-graph-shadow-traversal-diff.hybrid-candidate-count
memory-temporal-graph-shadow-traversal-diff.overlap-candidate-count
memory-temporal-graph-shadow-traversal-diff.graph-expansion-candidate-count
memory-temporal-graph-shadow-traversal-diff.win-count
memory-temporal-graph-shadow-traversal-diff.loss-count
memory-temporal-graph-shadow-traversal-diff.cost-count
memory-temporal-graph-shadow-traversal-diff.stage-required-count
memory-temporal-graph-shadow-traversal-diff.stage-projected-count
memory-temporal-graph-shadow-traversal-diff.digest-count
memory-temporal-graph-shadow-traversal-diff.freshness-pass-count
memory-temporal-graph-shadow-traversal-diff.replay-guard-pass-count
memory-temporal-graph-shadow-traversal-diff.stale-replay-rejected-count
memory-temporal-graph-shadow-traversal-diff.llm-rerank-count
memory-temporal-graph-shadow-traversal-diff.graph-persistence-count
memory-temporal-graph-shadow-traversal-diff.production-route-count
memory-temporal-graph-shadow-traversal-diff.production-write-count
memory-temporal-graph-shadow-traversal-diff.graph-write-count
memory-temporal-graph-shadow-traversal-quality
memory-temporal-graph-shadow-traversal-quality.fixture-count
memory-temporal-graph-shadow-traversal-quality.slo-required-count
memory-temporal-graph-shadow-traversal-quality.slo-pass-count
memory-temporal-graph-shadow-traversal-quality.coverage-basis-points
memory-temporal-graph-shadow-traversal-quality.precision-basis-points
memory-temporal-graph-shadow-traversal-quality.leak-rate-basis-points
memory-temporal-graph-shadow-traversal-quality.latency-budget-ms
memory-temporal-graph-shadow-traversal-quality.projected-latency-ms
memory-temporal-graph-shadow-traversal-quality.token-saved-estimate
memory-temporal-graph-shadow-traversal-quality.operator-review-required-count
memory-temporal-graph-shadow-traversal-quality.win-count
memory-temporal-graph-shadow-traversal-quality.loss-count
memory-temporal-graph-shadow-traversal-quality.cost-count
memory-temporal-graph-shadow-traversal-quality.stage-required-count
memory-temporal-graph-shadow-traversal-quality.stage-projected-count
memory-temporal-graph-shadow-traversal-quality.digest-count
memory-temporal-graph-shadow-traversal-quality.freshness-pass-count
memory-temporal-graph-shadow-traversal-quality.replay-guard-pass-count
memory-temporal-graph-shadow-traversal-quality.stale-replay-rejected-count
memory-temporal-graph-shadow-traversal-quality.llm-rerank-count
memory-temporal-graph-shadow-traversal-quality.graph-persistence-count
memory-temporal-graph-shadow-traversal-quality.production-route-count
memory-temporal-graph-shadow-traversal-quality.production-write-count
memory-temporal-graph-shadow-traversal-quality.graph-write-count
memory-temporal-graph-shadow-retrieval-canary-guard
memory-temporal-graph-shadow-retrieval-canary-guard.fixture-count
memory-temporal-graph-shadow-retrieval-canary-guard.stage-required-count
memory-temporal-graph-shadow-retrieval-canary-guard.stage-projected-count
memory-temporal-graph-shadow-retrieval-canary-guard.quality-slo-pass-count
memory-temporal-graph-shadow-retrieval-canary-guard.operator-approval-required-count
memory-temporal-graph-shadow-retrieval-canary-guard.operator-approval-recorded-count
memory-temporal-graph-shadow-retrieval-canary-guard.feature-flag-registered-count
memory-temporal-graph-shadow-retrieval-canary-guard.feature-flag-enabled-count
memory-temporal-graph-shadow-retrieval-canary-guard.kill-switch-registered-count
memory-temporal-graph-shadow-retrieval-canary-guard.kill-switch-ready-count
memory-temporal-graph-shadow-retrieval-canary-guard.rollback-rehearsal-required-count
memory-temporal-graph-shadow-retrieval-canary-guard.rollback-rehearsal-pass-count
memory-temporal-graph-shadow-retrieval-canary-guard.activation-denial-count
memory-temporal-graph-shadow-retrieval-canary-guard.canary-route-opened-count
memory-temporal-graph-shadow-retrieval-canary-guard.digest-count
memory-temporal-graph-shadow-retrieval-canary-guard.freshness-pass-count
memory-temporal-graph-shadow-retrieval-canary-guard.replay-guard-pass-count
memory-temporal-graph-shadow-retrieval-canary-guard.stale-replay-rejected-count
memory-temporal-graph-shadow-retrieval-canary-guard.llm-rerank-count
memory-temporal-graph-shadow-retrieval-canary-guard.graph-persistence-count
memory-temporal-graph-shadow-retrieval-canary-guard.production-route-count
memory-temporal-graph-shadow-retrieval-canary-guard.production-write-count
memory-temporal-graph-shadow-retrieval-canary-guard.graph-write-count
memory-temporal-graph-shadow-retrieval-canary-guard.rollback-write-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.fixture-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.stage-required-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.stage-projected-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.canary-guard-pass-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.operator-approval-required-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.operator-approval-recorded-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.feature-flag-registered-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.feature-flag-enabled-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-registered-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-readback-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-pass-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-required-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-readback-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-pass-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.route-denial-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-write-denial-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.canary-route-opened-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.digest-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.freshness-pass-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.replay-guard-pass-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.stale-replay-rejected-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.llm-rerank-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.graph-persistence-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.production-route-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.production-write-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.graph-write-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-write-count
eval-harness-seed
adaptive-allocator-eval-shadow
recall-quality-gate
recall-quality-blocking-reason-count
recall-quality-blocking-reasons
memory-ranked-recall-shadow-eval
ranked-recall.hybrid-signal-required-count
ranked-recall.hybrid-signal-pass-count
ranked-recall.lexical-bm25-check
ranked-recall.recency-check
ranked-recall.source-authority-check
ranked-recall.temporal-validity-check
ranked-recall.feedback-check
ranked-recall.positive-hybrid-signal-required-count
ranked-recall.positive-hybrid-signal-pass-count
ranked-recall.hybrid-regression-blocked-count
ranked-recall.hybrid-signal-min-basis-points
ranked-recall.min-positive-hybrid-score-basis-points
ranked-recall.routing-diff-fixture-count
ranked-recall.routing-diff-shadow-only-count
ranked-recall.routing-diff-win-count
ranked-recall.routing-diff-loss-count
ranked-recall.routing-diff-regression-blocked-count
ranked-recall.routing-diff-delta-min-basis-points
ranked-recall.min-positive-routing-diff-delta-basis-points
ranked-recall.routing-diff-latency-delta-max-ms
ranked-recall.max-positive-routing-diff-latency-delta-ms
ranked-recall.routing-diff-token-tradeoff-min-basis-points
ranked-recall.min-positive-routing-diff-token-tradeoff-basis-points
ranked-recall.real-workload-trace-fixture-count
ranked-recall.real-workload-trace-shadow-only-count
ranked-recall.real-workload-trace-slo-pass-count
ranked-recall.real-workload-trace-win-count
ranked-recall.real-workload-trace-loss-count
ranked-recall.real-workload-trace-operator-review-required-count
ranked-recall.real-workload-trace-total-leak-count
ranked-recall.real-workload-trace-max-leak-rate-basis-points
ranked-recall.min-positive-real-workload-trace-coverage-basis-points
ranked-recall.min-positive-real-workload-trace-precision-basis-points
ranked-recall.total-positive-real-workload-trace-token-saved
ranked-recall.max-positive-real-workload-trace-latency-ms
ranked-recall.real-workload-trace-regression-loss-count
ranked-recall.canary-precondition-fixture-count
ranked-recall.canary-precondition-shadow-only-count
ranked-recall.canary-precondition-pass-count
ranked-recall.canary-feature-flag-registered-count
ranked-recall.canary-feature-flag-disabled-count
ranked-recall.canary-kill-switch-registered-count
ranked-recall.canary-kill-switch-enabled-count
ranked-recall.canary-rollback-rehearsal-covered-count
ranked-recall.canary-activation-denial-covered-count
ranked-recall.canary-precondition-operator-review-required-count
ranked-recall.canary-precondition-route-opened-count
ranked-recall.canary-precondition-rollback-write-count
memory-provider-boundary
memory-provider-v2-boundary
memory-provider-v2.lifecycle-required-count
memory-provider-v2.lifecycle-pass-count
memory-provider-v2.query-check
memory-provider-v2.update-context-check
memory-provider-v2.propose-write-check
memory-provider-v2.add-check
memory-provider-v2.clear-check
memory-provider-v2.close-check
memory-provider-v2.candidate-count
memory-provider-v2.operator-review-required-count
memory-shadow-canary-readiness
memory-shadow-canary-promotion-readiness
canary-promotion.required-stable-window-count
canary-promotion.observed-stable-window-count
canary-promotion.required-pass-streak
canary-promotion.observed-pass-streak
canary-promotion.promotion-blocker-count
canary-promotion.checklist-required-count
canary-promotion.checklist-pass-count
canary-promotion.readiness-check
canary-promotion.negative-rehearsal-check
canary-promotion.audit-digest-check
canary-promotion.audit-freshness-check
canary-promotion.rollback-rehearsal-pass-count
canary-promotion.kill-switch-rehearsal-pass-count
canary-promotion.soak-readback-pass-count
source-aware-front-door
operator-approval
activation-allowed
runtime-activation
adaptive-allocator-runtime-activation
source-aware-runtime-activation
production-write
graph-write
prompt-assembly-change
operator-activation"#;

const LEGACY_PACKET_FIELD_ORDER_V19: &str = r#"schema
dry-run
approval-required
activation-command
rows
satisfied
blockers
threshold.required-ready
threshold.required-shadow
blocker.adaptive-budget-allocation-shadow-only
blocker.temporal-graph-shadow-eval-shadow-only
blocker.temporal-graph-shadow-store-shadow-only
blocker.temporal-graph-shadow-replay-shadow-only
blocker.temporal-graph-shadow-traversal-diff-shadow-only
blocker.temporal-graph-shadow-traversal-quality-shadow-only
blocker.temporal-graph-shadow-retrieval-canary-guard-shadow-only
blocker.temporal-graph-shadow-retrieval-rollback-kill-switch-shadow-only
blocker.memory-ranked-recall-shadow-eval-shadow-only
blocker.memory-provider-boundary-shadow-only
blocker.memory-provider-v2-boundary-shadow-only
blocker.memory-namespace-policy-shadow-only
blocker.memory-write-chain-readiness-shadow-only
blocker.memory-write-chain-receipt-freshness-shadow-only
blocker.memory-shadow-canary-readiness-shadow-only
blocker.memory-shadow-canary-promotion-readiness-shadow-only
blocker.source-aware-front-door-disabled
blocker.operator-approval-missing
recall-quality-blocking-reason-count
recall-quality-blocking-reasons
canary-promotion.required-stable-window-count
canary-promotion.observed-stable-window-count
canary-promotion.required-pass-streak
canary-promotion.observed-pass-streak
canary-promotion.promotion-blocker-count
canary-promotion.checklist-required-count
canary-promotion.checklist-pass-count
canary-promotion.readiness-check
canary-promotion.negative-rehearsal-check
canary-promotion.audit-digest-check
canary-promotion.audit-freshness-check
canary-promotion.rollback-rehearsal-pass-count
canary-promotion.kill-switch-rehearsal-pass-count
canary-promotion.soak-readback-pass-count
memory-provider-v2.lifecycle-required-count
memory-provider-v2.lifecycle-pass-count
memory-provider-v2.query-check
memory-provider-v2.update-context-check
memory-provider-v2.propose-write-check
memory-provider-v2.add-check
memory-provider-v2.clear-check
memory-provider-v2.close-check
memory-provider-v2.candidate-count
memory-provider-v2.operator-review-required-count
memory-namespace-policy.namespace-count
memory-namespace-policy.operator-approval-required-count
memory-namespace-policy.shadow-wal-required-count
memory-namespace-policy.readback-required-count
memory-namespace-policy.canary-required-count
memory-namespace-policy.rollback-supported-count
memory-namespace-policy.production-write-count
memory-namespace-policy.graph-write-count
memory-write-chain-readiness.namespace-count
memory-write-chain-readiness.stage-required-count
memory-write-chain-readiness.stage-pass-count
memory-write-chain-readiness.propose-write-ready-count
memory-write-chain-readiness.policy-approval-ready-count
memory-write-chain-readiness.operator-approval-ready-count
memory-write-chain-readiness.shadow-wal-ready-count
memory-write-chain-readiness.readback-ready-count
memory-write-chain-readiness.canary-ready-count
memory-write-chain-readiness.rollback-ready-count
memory-write-chain-readiness.production-write-count
memory-write-chain-readiness.graph-write-count
memory-write-chain-receipt-freshness.namespace-count
memory-write-chain-receipt-freshness.receipt-required-count
memory-write-chain-receipt-freshness.receipt-projected-count
memory-write-chain-receipt-freshness.receipt-digest-count
memory-write-chain-receipt-freshness.freshness-pass-count
memory-write-chain-receipt-freshness.replay-guard-pass-count
memory-write-chain-receipt-freshness.stale-replay-rejected-count
memory-write-chain-receipt-freshness.recorded-receipt-count
memory-write-chain-receipt-freshness.persisted-receipt-count
memory-write-chain-receipt-freshness.production-write-count
memory-write-chain-receipt-freshness.graph-write-count
memory-temporal-graph-shadow-store.node-count
memory-temporal-graph-shadow-store.edge-count
memory-temporal-graph-shadow-store.provenance-edge-count
memory-temporal-graph-shadow-store.validity-window-edge-count
memory-temporal-graph-shadow-store.supersede-edge-count
memory-temporal-graph-shadow-store.invalidated-node-count
memory-temporal-graph-shadow-store.stage-required-count
memory-temporal-graph-shadow-store.stage-projected-count
memory-temporal-graph-shadow-store.digest-count
memory-temporal-graph-shadow-store.freshness-pass-count
memory-temporal-graph-shadow-store.replay-guard-pass-count
memory-temporal-graph-shadow-store.stale-replay-rejected-count
memory-temporal-graph-shadow-store.operator-approval-required-count
memory-temporal-graph-shadow-store.operator-approval-recorded-count
memory-temporal-graph-shadow-store.recorded-receipt-count
memory-temporal-graph-shadow-store.persisted-receipt-count
memory-temporal-graph-shadow-store.production-write-count
memory-temporal-graph-shadow-store.graph-write-count
memory-temporal-graph-shadow-replay.node-count
memory-temporal-graph-shadow-replay.edge-count
memory-temporal-graph-shadow-replay.provenance-count
memory-temporal-graph-shadow-replay.bitemporal-validity-count
memory-temporal-graph-shadow-replay.fact-invalidation-count
memory-temporal-graph-shadow-replay.supersede-tombstone-count
memory-temporal-graph-shadow-replay.stage-required-count
memory-temporal-graph-shadow-replay.stage-projected-count
memory-temporal-graph-shadow-replay.digest-count
memory-temporal-graph-shadow-replay.freshness-pass-count
memory-temporal-graph-shadow-replay.replay-guard-pass-count
memory-temporal-graph-shadow-replay.stale-replay-rejected-count
memory-temporal-graph-shadow-replay.operator-approval-required-count
memory-temporal-graph-shadow-replay.operator-approval-recorded-count
memory-temporal-graph-shadow-replay.recorded-receipt-count
memory-temporal-graph-shadow-replay.persisted-receipt-count
memory-temporal-graph-shadow-replay.production-write-count
memory-temporal-graph-shadow-replay.graph-write-count
memory-temporal-graph-shadow-traversal-diff.production-selection-count
memory-temporal-graph-shadow-traversal-diff.lexical-bm25-candidate-count
memory-temporal-graph-shadow-traversal-diff.semantic-candidate-count
memory-temporal-graph-shadow-traversal-diff.graph-traversal-candidate-count
memory-temporal-graph-shadow-traversal-diff.hybrid-candidate-count
memory-temporal-graph-shadow-traversal-diff.overlap-candidate-count
memory-temporal-graph-shadow-traversal-diff.graph-expansion-candidate-count
memory-temporal-graph-shadow-traversal-diff.win-count
memory-temporal-graph-shadow-traversal-diff.loss-count
memory-temporal-graph-shadow-traversal-diff.cost-count
memory-temporal-graph-shadow-traversal-diff.stage-required-count
memory-temporal-graph-shadow-traversal-diff.stage-projected-count
memory-temporal-graph-shadow-traversal-diff.digest-count
memory-temporal-graph-shadow-traversal-diff.freshness-pass-count
memory-temporal-graph-shadow-traversal-diff.replay-guard-pass-count
memory-temporal-graph-shadow-traversal-diff.stale-replay-rejected-count
memory-temporal-graph-shadow-traversal-diff.llm-rerank-count
memory-temporal-graph-shadow-traversal-diff.graph-persistence-count
memory-temporal-graph-shadow-traversal-diff.production-route-count
memory-temporal-graph-shadow-traversal-diff.production-write-count
memory-temporal-graph-shadow-traversal-diff.graph-write-count
memory-temporal-graph-shadow-traversal-quality.fixture-count
memory-temporal-graph-shadow-traversal-quality.slo-required-count
memory-temporal-graph-shadow-traversal-quality.slo-pass-count
memory-temporal-graph-shadow-traversal-quality.coverage-basis-points
memory-temporal-graph-shadow-traversal-quality.precision-basis-points
memory-temporal-graph-shadow-traversal-quality.leak-rate-basis-points
memory-temporal-graph-shadow-traversal-quality.latency-budget-ms
memory-temporal-graph-shadow-traversal-quality.projected-latency-ms
memory-temporal-graph-shadow-traversal-quality.token-saved-estimate
memory-temporal-graph-shadow-traversal-quality.operator-review-required-count
memory-temporal-graph-shadow-traversal-quality.win-count
memory-temporal-graph-shadow-traversal-quality.loss-count
memory-temporal-graph-shadow-traversal-quality.cost-count
memory-temporal-graph-shadow-traversal-quality.stage-required-count
memory-temporal-graph-shadow-traversal-quality.stage-projected-count
memory-temporal-graph-shadow-traversal-quality.digest-count
memory-temporal-graph-shadow-traversal-quality.freshness-pass-count
memory-temporal-graph-shadow-traversal-quality.replay-guard-pass-count
memory-temporal-graph-shadow-traversal-quality.stale-replay-rejected-count
memory-temporal-graph-shadow-traversal-quality.llm-rerank-count
memory-temporal-graph-shadow-traversal-quality.graph-persistence-count
memory-temporal-graph-shadow-traversal-quality.production-route-count
memory-temporal-graph-shadow-traversal-quality.production-write-count
memory-temporal-graph-shadow-traversal-quality.graph-write-count
memory-temporal-graph-shadow-retrieval-canary-guard.fixture-count
memory-temporal-graph-shadow-retrieval-canary-guard.stage-required-count
memory-temporal-graph-shadow-retrieval-canary-guard.stage-projected-count
memory-temporal-graph-shadow-retrieval-canary-guard.quality-slo-pass-count
memory-temporal-graph-shadow-retrieval-canary-guard.operator-approval-required-count
memory-temporal-graph-shadow-retrieval-canary-guard.operator-approval-recorded-count
memory-temporal-graph-shadow-retrieval-canary-guard.feature-flag-registered-count
memory-temporal-graph-shadow-retrieval-canary-guard.feature-flag-enabled-count
memory-temporal-graph-shadow-retrieval-canary-guard.kill-switch-registered-count
memory-temporal-graph-shadow-retrieval-canary-guard.kill-switch-ready-count
memory-temporal-graph-shadow-retrieval-canary-guard.rollback-rehearsal-required-count
memory-temporal-graph-shadow-retrieval-canary-guard.rollback-rehearsal-pass-count
memory-temporal-graph-shadow-retrieval-canary-guard.activation-denial-count
memory-temporal-graph-shadow-retrieval-canary-guard.canary-route-opened-count
memory-temporal-graph-shadow-retrieval-canary-guard.digest-count
memory-temporal-graph-shadow-retrieval-canary-guard.freshness-pass-count
memory-temporal-graph-shadow-retrieval-canary-guard.replay-guard-pass-count
memory-temporal-graph-shadow-retrieval-canary-guard.stale-replay-rejected-count
memory-temporal-graph-shadow-retrieval-canary-guard.llm-rerank-count
memory-temporal-graph-shadow-retrieval-canary-guard.graph-persistence-count
memory-temporal-graph-shadow-retrieval-canary-guard.production-route-count
memory-temporal-graph-shadow-retrieval-canary-guard.production-write-count
memory-temporal-graph-shadow-retrieval-canary-guard.graph-write-count
memory-temporal-graph-shadow-retrieval-canary-guard.rollback-write-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.fixture-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.stage-required-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.stage-projected-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.canary-guard-pass-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.operator-approval-required-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.operator-approval-recorded-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.feature-flag-registered-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.feature-flag-enabled-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-registered-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-readback-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.kill-switch-pass-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-required-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-readback-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-rehearsal-pass-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.route-denial-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-write-denial-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.canary-route-opened-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.digest-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.freshness-pass-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.replay-guard-pass-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.stale-replay-rejected-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.llm-rerank-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.graph-persistence-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.production-route-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.production-write-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.graph-write-count
memory-temporal-graph-shadow-retrieval-rollback-kill-switch.rollback-write-count
ranked-recall.hybrid-signal-required-count
ranked-recall.hybrid-signal-pass-count
ranked-recall.lexical-bm25-check
ranked-recall.recency-check
ranked-recall.source-authority-check
ranked-recall.temporal-validity-check
ranked-recall.feedback-check
ranked-recall.positive-hybrid-signal-required-count
ranked-recall.positive-hybrid-signal-pass-count
ranked-recall.hybrid-regression-blocked-count
ranked-recall.hybrid-signal-min-basis-points
ranked-recall.min-positive-hybrid-score-basis-points
ranked-recall.routing-diff-fixture-count
ranked-recall.routing-diff-shadow-only-count
ranked-recall.routing-diff-win-count
ranked-recall.routing-diff-loss-count
ranked-recall.routing-diff-regression-blocked-count
ranked-recall.routing-diff-delta-min-basis-points
ranked-recall.min-positive-routing-diff-delta-basis-points
ranked-recall.routing-diff-latency-delta-max-ms
ranked-recall.max-positive-routing-diff-latency-delta-ms
ranked-recall.routing-diff-token-tradeoff-min-basis-points
ranked-recall.min-positive-routing-diff-token-tradeoff-basis-points
ranked-recall.real-workload-trace-fixture-count
ranked-recall.real-workload-trace-shadow-only-count
ranked-recall.real-workload-trace-slo-pass-count
ranked-recall.real-workload-trace-win-count
ranked-recall.real-workload-trace-loss-count
ranked-recall.real-workload-trace-operator-review-required-count
ranked-recall.real-workload-trace-total-leak-count
ranked-recall.real-workload-trace-max-leak-rate-basis-points
ranked-recall.min-positive-real-workload-trace-coverage-basis-points
ranked-recall.min-positive-real-workload-trace-precision-basis-points
ranked-recall.total-positive-real-workload-trace-token-saved
ranked-recall.max-positive-real-workload-trace-latency-ms
ranked-recall.real-workload-trace-regression-loss-count
ranked-recall.canary-precondition-fixture-count
ranked-recall.canary-precondition-shadow-only-count
ranked-recall.canary-precondition-pass-count
ranked-recall.canary-feature-flag-registered-count
ranked-recall.canary-feature-flag-disabled-count
ranked-recall.canary-kill-switch-registered-count
ranked-recall.canary-kill-switch-enabled-count
ranked-recall.canary-rollback-rehearsal-covered-count
ranked-recall.canary-activation-denial-covered-count
ranked-recall.canary-precondition-operator-review-required-count
ranked-recall.canary-precondition-route-opened-count
ranked-recall.canary-precondition-rollback-write-count
required-scopes
scope.adaptive-budget-allocation-runtime
scope.source-aware-runtime-activation
scope.production-memory-write
scope.graph-write
scope.prompt-assembly-change
scope.operator-activation
runtime-activation
adaptive-allocator-runtime-activation
source-aware-runtime-activation
production-write
graph-write
prompt-assembly-change
operator-activation"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn report_index(id: &str) -> usize {
        CONTEXT_PLANE_COMPAT_REPORT_IDS
            .iter()
            .position(|candidate| *candidate == id)
            .expect("context-plane report id must exist")
    }

    fn flip_digest(digest: &mut String) {
        let replacement = if digest.starts_with('0') { '1' } else { '0' };
        digest.replace_range(..1, &replacement.to_string());
    }

    #[test]
    fn typed_fixture_is_source_backed_payload_light_and_closed() {
        let reports = build_context_plane_compat_reports().expect("typed fixture must build");

        assert!(reports.has_integrity());
        assert_eq!(reports.reports.len(), 8);
        assert_eq!(
            reports
                .reports
                .iter()
                .map(|report| report.gate.as_str())
                .collect::<Vec<_>>(),
            CONTEXT_PLANE_COMPAT_REPORT_IDS
        );
        for report in &reports.reports {
            assert!(!report.production_authority_granted);
            assert!(!report.write_authority_granted);
            assert!(!report.ready_for_live_execution);
            assert!(!report.mutation_enabled);
            assert!(report.side_effects.is_closed());
            assert!(!report.sources.is_empty());
            assert!(
                report
                    .sources
                    .iter()
                    .all(source_binding_has_shape_integrity)
            );
            assert!(report.render_legacy_line_protocol().is_ok());
        }

        let activation = reports
            .report(CONTEXT_PLANE_COMPAT_REPORT_IDS[0])
            .expect("activation report must exist");
        let ContextPlaneCompatPayload::ActivationBlockerMatrix { status, matrix } =
            &activation.payload
        else {
            panic!("activation payload kind drifted")
        };
        assert!(status.has_status_integrity());
        assert!(matrix.has_matrix_integrity());

        let approval = reports
            .report(CONTEXT_PLANE_COMPAT_REPORT_IDS[1])
            .expect("approval report must exist");
        let ContextPlaneCompatPayload::OperatorApprovalPacket { packet } = &approval.payload else {
            panic!("approval payload kind drifted")
        };
        assert!(packet.has_packet_integrity());

        let encoded = serde_json::to_string(&reports).expect("typed reports must serialize");
        assert!(!encoded.contains("timeout retry guidance"));
        assert!(!encoded.contains("timeout surfaced during tool run"));
    }

    #[test]
    fn every_stage_rejects_source_digest_line_generation_and_sequence_tamper() {
        let reports = build_context_plane_compat_reports().expect("typed fixture must build");

        for index in 0..reports.reports.len() {
            let id = reports.reports[index].gate.clone();

            let mut digest_tamper = reports.clone();
            flip_digest(&mut digest_tamper.reports[index].sources[0].sha256);
            assert!(
                !digest_tamper.has_integrity(),
                "{id} accepted source digest tamper"
            );

            let mut line_tamper = reports.clone();
            line_tamper.reports[index].sources[0].line_count += 1;
            assert!(
                !line_tamper.has_integrity(),
                "{id} accepted source line-count tamper"
            );

            let mut generation_tamper = reports.clone();
            generation_tamper.reports[index].sources[0].generation += 1;
            assert!(
                !generation_tamper.has_integrity(),
                "{id} accepted mixed source generation"
            );

            let mut sequence_tamper = reports.clone();
            sequence_tamper.reports[index].sources[0].sequence += 1;
            assert!(
                !sequence_tamper.has_integrity(),
                "{id} accepted mixed source sequence"
            );
        }
    }

    #[test]
    fn chain_rejects_reorder_mixed_generation_and_expiry_drift() {
        let reports = build_context_plane_compat_reports().expect("typed fixture must build");

        let dependency_index = report_index(CONTEXT_PLANE_COMPAT_REPORT_IDS[4]);
        let mut reordered = reports.clone();
        reordered.reports[dependency_index].sources.swap(0, 1);
        assert!(!reordered.has_integrity());

        let digest_index = report_index(CONTEXT_PLANE_COMPAT_REPORT_IDS[5]);
        let mut mixed_generation = reports.clone();
        let ContextPlaneCompatPayload::FreshnessDependencyChainCanonicalDigest {
            source_readiness_chain_generation,
            ..
        } = &mut mixed_generation.reports[digest_index].payload
        else {
            panic!("dependency digest payload kind drifted")
        };
        *source_readiness_chain_generation -= 1;
        assert!(!mixed_generation.has_integrity());

        let expiry_index = report_index(CONTEXT_PLANE_COMPAT_REPORT_IDS[6]);
        let mut expired = reports.clone();
        let ContextPlaneCompatPayload::FreshnessDependencyChainExpiryDrift {
            readiness_window_expires_after_sequence,
            ..
        } = &mut expired.reports[expiry_index].payload
        else {
            panic!("expiry payload kind drifted")
        };
        *readiness_window_expires_after_sequence = EXPIRY_DRIFT_GENERATION;
        assert!(!expired.has_integrity());

        let freshness_index = report_index(CONTEXT_PLANE_COMPAT_REPORT_IDS[3]);
        let mut replay_window = reports.clone();
        let ContextPlaneCompatPayload::Freshness {
            expires_after_sequence,
            ..
        } = &mut replay_window.reports[freshness_index].payload
        else {
            panic!("freshness payload kind drifted")
        };
        *expires_after_sequence = CANONICAL_GENERATION;
        assert!(!replay_window.has_integrity());
    }

    #[test]
    fn matrix_packet_authority_and_payload_injection_fail_closed() {
        let reports = build_context_plane_compat_reports().expect("typed fixture must build");

        let activation_index = report_index(CONTEXT_PLANE_COMPAT_REPORT_IDS[0]);
        let mut matrix_tamper = reports.clone();
        let ContextPlaneCompatPayload::ActivationBlockerMatrix { matrix, .. } =
            &mut matrix_tamper.reports[activation_index].payload
        else {
            panic!("activation payload kind drifted")
        };
        matrix.runtime_activation = true;
        assert!(!matrix_tamper.has_integrity());

        let approval_index = report_index(CONTEXT_PLANE_COMPAT_REPORT_IDS[1]);
        let mut packet_tamper = reports.clone();
        let ContextPlaneCompatPayload::OperatorApprovalPacket { packet } =
            &mut packet_tamper.reports[approval_index].payload
        else {
            panic!("approval payload kind drifted")
        };
        packet.activation_command_present = true;
        assert!(!packet_tamper.has_integrity());

        let negative_index = report_index(CONTEXT_PLANE_COMPAT_REPORT_IDS[7]);
        let mut negative_tamper = reports.clone();
        let ContextPlaneCompatPayload::NegativeExport {
            activation_command_present,
            ..
        } = &mut negative_tamper.reports[negative_index].payload
        else {
            panic!("negative payload kind drifted")
        };
        *activation_command_present = true;
        assert!(!negative_tamper.has_integrity());

        let mut payload_injection = reports.clone();
        payload_injection.reports[dependency_index()]
            .legacy_business_fields
            .insert(
                "payload_field_injection".to_string(),
                Value::String("write_activation=true".to_string()),
            );
        assert!(!payload_injection.has_integrity());
        assert!(
            payload_injection.reports[dependency_index()]
                .render_legacy_line_protocol()
                .is_err()
        );

        let mut field_order_tamper = reports.clone();
        field_order_tamper.reports[dependency_index()]
            .legacy_business_field_order
            .swap(0, 1);
        assert!(!field_order_tamper.has_integrity());
        assert!(
            field_order_tamper.reports[dependency_index()]
                .render_legacy_line_protocol()
                .is_err()
        );

        let mut authority_tamper = reports.clone();
        authority_tamper.reports[0].production_authority_granted = true;
        assert!(!authority_tamper.has_integrity());

        let mut side_effect_tamper = reports.clone();
        side_effect_tamper.reports[0]
            .side_effects
            .runtime_mutation_performed = true;
        assert!(!side_effect_tamper.has_integrity());
    }

    fn dependency_index() -> usize {
        report_index(CONTEXT_PLANE_COMPAT_REPORT_IDS[4])
    }

    #[test]
    fn dynamic_projection_and_source_receipts_are_deterministic() {
        let first = build_context_plane_compat_reports().expect("first fixture must build");
        let second = build_context_plane_compat_reports().expect("second fixture must build");

        assert_eq!(first, second);
        for (left, right) in first.reports.iter().zip(&second.reports) {
            let left_protocol = left
                .render_legacy_line_protocol()
                .expect("left projection must render");
            let right_protocol = right
                .render_legacy_line_protocol()
                .expect("right projection must render");
            assert_eq!(left_protocol, right_protocol);
            assert_eq!(sha256_hex(left_protocol.as_bytes()).len(), 64);
            assert_eq!(
                left_protocol.lines().count(),
                left.legacy_business_fields.len()
            );
        }
    }
}
