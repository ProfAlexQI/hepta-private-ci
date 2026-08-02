use std::collections::BTreeMap;

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
                "approval_digest_mismatch",
                "negative_export_digest_mismatch",
                "combined_digest_mismatch",
                "reordered_dependency_rows",
                "mixed_generation_sequence",
                "payload_write_activation_injection",
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
    CanonicalExportDigest,
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
            legacy_business_fields: BTreeMap::new(),
            production_authority_granted: false,
            write_authority_granted: false,
            ready_for_live_execution: false,
            mutation_enabled: false,
            side_effects: ContextPlaneCompatSideEffects::none(),
        };
        report.legacy_business_fields = expected_legacy_business_fields(&report)?;
        Ok(report)
    }

    fn render_legacy_line_protocol(&self) -> Result<String, String> {
        render_legacy_fields(&self.legacy_business_fields)
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
            && self.sources.iter().all(source_binding_has_shape_integrity)
            && expected_legacy_business_fields(self)
                .is_ok_and(|expected| expected == self.legacy_business_fields)
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
        if !matches!(
            &canonical.payload,
            ContextPlaneCompatPayload::CanonicalExportDigest
        ) || canonical.generation != CANONICAL_GENERATION
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
                    require_ok!(source_binding_from_serializable(
                        TAMPER_MATRIX_SOURCE_ID,
                        CANONICAL_GENERATION,
                        CANONICAL_GENERATION,
                        &tamper_matrix,
                    )),
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
    )?;
    let approval = ContextPlaneCompatReport::new(
        CONTEXT_PLANE_COMPAT_REPORT_IDS[1],
        APPROVAL_GENERATION,
        APPROVAL_GENERATION,
        vec![source_binding_from_report(&activation)?],
        ContextPlaneCompatPayload::OperatorApprovalPacket { packet },
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
    )?;
    let canonical = ContextPlaneCompatReport::new(
        CONTEXT_PLANE_COMPAT_REPORT_IDS[2],
        CANONICAL_GENERATION,
        CANONICAL_GENERATION,
        vec![
            source_binding_from_report(&approval)?,
            source_binding_from_report(&negative)?,
        ],
        ContextPlaneCompatPayload::CanonicalExportDigest,
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
            source_binding_from_serializable(
                TAMPER_MATRIX_SOURCE_ID,
                CANONICAL_GENERATION,
                CANONICAL_GENERATION,
                &tamper_matrix,
            )?,
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

fn source_binding_has_shape_integrity(source: &ContextPlaneCompatSourceBinding) -> bool {
    !source.report_id.is_empty()
        && source.line_count > 0
        && source.generation > 0
        && source.sequence > 0
        && source.sha256.len() == 64
        && source.sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn expected_legacy_business_fields(
    report: &ContextPlaneCompatReport,
) -> Result<BTreeMap<String, Value>, String> {
    let mut fields = BTreeMap::from([
        ("result".to_string(), Value::String("pass".to_string())),
        (
            "payload_light".to_string(),
            Value::String("pass".to_string()),
        ),
        (
            "generation".to_string(),
            Value::Number(report.generation.into()),
        ),
        (
            "sequence".to_string(),
            Value::Number(report.sequence.into()),
        ),
        ("runtime_activation".to_string(), Value::Bool(false)),
        ("operator_activation".to_string(), Value::Bool(false)),
        ("production_write".to_string(), Value::Bool(false)),
        ("graph_write".to_string(), Value::Bool(false)),
    ]);
    append_source_fields(&mut fields, &report.sources);
    match &report.payload {
        ContextPlaneCompatPayload::ActivationBlockerMatrix { status, matrix } => {
            fields.insert(
                "context_status_schema".to_string(),
                Value::Number(status.schema_version.into()),
            );
            fields.insert(
                "context_matrix_schema".to_string(),
                Value::Number(matrix.schema_version.into()),
            );
            fields.insert(
                "row_count".to_string(),
                Value::Number(matrix.rows.len().into()),
            );
            fields.insert(
                "threshold_satisfied_count".to_string(),
                Value::Number(matrix.satisfied_count().into()),
            );
            fields.insert(
                "blocker_count".to_string(),
                Value::Number(matrix.blocker_count.into()),
            );
            append_flattened(&mut fields, "matrix", matrix)?;
        }
        ContextPlaneCompatPayload::OperatorApprovalPacket { packet } => {
            fields.insert(
                "context_packet_schema".to_string(),
                Value::Number(packet.schema_version.into()),
            );
            fields.insert("dry_run_only".to_string(), Value::Bool(packet.dry_run_only));
            fields.insert(
                "approval_required".to_string(),
                Value::Bool(packet.approval_required),
            );
            fields.insert(
                "activation_command_present".to_string(),
                Value::Bool(packet.activation_command_present),
            );
            append_flattened(&mut fields, "packet", packet)?;
        }
        ContextPlaneCompatPayload::NegativeExport {
            payload_light,
            activation_command_present,
        } => {
            fields.insert("payload_light".to_string(), Value::Bool(*payload_light));
            fields.insert(
                "activation_command_present".to_string(),
                Value::Bool(*activation_command_present),
            );
        }
        ContextPlaneCompatPayload::CanonicalExportDigest => {
            fields.insert("source_count".to_string(), Value::Number(2_u64.into()));
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
            fields.insert(
                "approval_readiness_sequence".to_string(),
                Value::Number((*approval_readiness_sequence).into()),
            );
            fields.insert(
                "current_readiness_sequence".to_string(),
                Value::Number((*current_readiness_sequence).into()),
            );
            fields.insert(
                "expires_after_sequence".to_string(),
                Value::Number((*expires_after_sequence).into()),
            );
            fields.insert(
                "max_replay_age_sequences".to_string(),
                Value::Number((*max_replay_age_sequences).into()),
            );
            fields.insert(
                "stale_sequence_rejected".to_string(),
                Value::Bool(*stale_sequence_rejected),
            );
            fields.insert(
                "expired_sequence_rejected".to_string(),
                Value::Bool(*expired_sequence_rejected),
            );
            fields.insert(
                "future_sequence_rejected".to_string(),
                Value::Bool(*future_sequence_rejected),
            );
            fields.insert(
                "digest_replay_rejected".to_string(),
                Value::Bool(*digest_replay_rejected),
            );
        }
        ContextPlaneCompatPayload::FreshnessDependencyChain { .. }
        | ContextPlaneCompatPayload::FreshnessDependencyChainCanonicalDigest { .. }
        | ContextPlaneCompatPayload::FreshnessDependencyChainExpiryDrift { .. } => {
            append_flattened(&mut fields, "chain", &report.payload)?;
        }
    }
    Ok(fields)
}

fn append_source_fields(
    fields: &mut BTreeMap<String, Value>,
    sources: &[ContextPlaneCompatSourceBinding],
) {
    fields.insert(
        "source_count".to_string(),
        Value::Number(sources.len().into()),
    );
    for (index, source) in sources.iter().enumerate() {
        let prefix = format!("source.{index}");
        fields.insert(
            format!("{prefix}.report_id"),
            Value::String(source.report_id.clone()),
        );
        fields.insert(
            format!("{prefix}.line_count"),
            Value::Number(source.line_count.into()),
        );
        fields.insert(
            format!("{prefix}.sha256"),
            Value::String(source.sha256.clone()),
        );
        fields.insert(
            format!("{prefix}.generation"),
            Value::Number(source.generation.into()),
        );
        fields.insert(
            format!("{prefix}.sequence"),
            Value::Number(source.sequence.into()),
        );
    }
}

fn append_flattened<T: Serialize>(
    target: &mut BTreeMap<String, Value>,
    prefix: &str,
    value: &T,
) -> Result<(), String> {
    let source = serde_json::to_value(value)
        .map_err(|error| format!("cannot serialize context-plane typed source: {error}"))?;
    flatten_json(prefix, &source, target)
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
