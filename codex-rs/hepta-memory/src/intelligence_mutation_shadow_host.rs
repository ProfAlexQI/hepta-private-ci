//! P0.4c shadow host adapter over the typed mutation state and SQLite journal.
//!
//! The adapter accepts receipts that a qualification host has already
//! observed and records their causal order in the P0.4b journal. It never
//! performs a source append, memory/KG write, projection refresh, outbox
//! dispatch, tool call, physical send, or any other external effect.
//!
//! The public seam deliberately uses bounded JSON contracts so Agentd can
//! exercise it without exposing the private P0.4a/P0.4b implementation types.
//! The default `CognitiveStore::open` path remains unchanged.

use codex_hepta_contracts::Sha256Digest;
use codex_hepta_paths::HeptaAgentLayout;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use sha2::Digest;
use sha2::Sha256;

use crate::CognitiveStore;
use crate::CognitiveStoreError;

use super::frame_part;
use super::intelligence_mutation_journal::IntelligenceMutationJournalAppend;
use super::intelligence_mutation_journal::IntelligenceMutationJournalDisposition;
use super::intelligence_mutation_journal::IntelligenceMutationJournalError;
use super::intelligence_mutation_journal::IntelligenceMutationJournalFault;
use super::intelligence_mutation_state::IntelligenceMutationAction;
use super::intelligence_mutation_state::IntelligenceMutationBinding;
use super::intelligence_mutation_state::IntelligenceMutationState;
use super::intelligence_mutation_state::IntelligenceMutationTransitionRequest;

pub(crate) const SHADOW_INTELLIGENCE_MUTATION_HOST_SCHEMA_VERSION: u32 = 1;
pub(crate) const SHADOW_INTELLIGENCE_MUTATION_HOST_NAMESPACE: &str =
    "shadow_intelligence_mutation_host_v1";
pub(crate) const SHADOW_INTELLIGENCE_MUTATION_HOST_RUNTIME_WIRED: bool = false;
pub(crate) const SHADOW_INTELLIGENCE_MUTATION_HOST_DEFAULT_OPEN_WIRED: bool = false;
pub(crate) const SHADOW_INTELLIGENCE_MUTATION_HOST_MEMORY_WRITE_AUTHORITY: bool = false;
pub(crate) const SHADOW_INTELLIGENCE_MUTATION_HOST_PROJECTION_WRITE_AUTHORITY: bool = false;
pub(crate) const SHADOW_INTELLIGENCE_MUTATION_HOST_OUTBOX_DISPATCH_AUTHORITY: bool = false;
pub(crate) const SHADOW_INTELLIGENCE_MUTATION_HOST_EXTERNAL_EFFECTS: bool = false;
pub(crate) const SHADOW_INTELLIGENCE_MUTATION_HOST_PRODUCTION_AUTHORITY: bool = false;
pub(crate) const SHADOW_INTELLIGENCE_MUTATION_HOST_OPERATOR_ACCEPTANCE: bool = false;
pub(crate) const SHADOW_INTELLIGENCE_MUTATION_HOST_PROMOTION: bool = false;

const PREPARED_NAMESPACE: &str = "shadow_intelligence_mutation_prepared_observation_v1";
const MAX_JSON_BYTES: usize = 64 * 1024;
const MAX_ID_BYTES: usize = 256;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ShadowHostBindingDraft {
    operation_id: String,
    lease_id: String,
    lease_epoch: u64,
    expected_revision: Option<u64>,
    starting_projection_generation: u64,
    causal_root_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "observation", rename_all = "snake_case", deny_unknown_fields)]
enum ShadowHostObservation {
    SourceWitnessed {
        source_sha256: String,
    },
    GroundingValidated {
        grounding_receipt_sha256: String,
    },
    DurableIntentAppended {
        intent_sha256: String,
    },
    MemoryFactsCommitted {
        write_receipt_sha256: String,
    },
    ProjectionPublished {
        expected_previous_generation: u64,
        new_generation: u64,
        projection_receipt_sha256: String,
    },
    OutboxSettled {
        outcome_sha256: String,
    },
    Terminalized,
    Indeterminate {
        reason_sha256: String,
    },
    ReconciledApplied {
        outcome_sha256: String,
    },
    ReconciledNotApplied {
        outcome_sha256: String,
    },
    Quarantined {
        reason_sha256: String,
    },
}

impl ShadowHostObservation {
    const fn kind(&self) -> &'static str {
        match self {
            Self::SourceWitnessed { .. } => "source_witnessed",
            Self::GroundingValidated { .. } => "grounding_validated",
            Self::DurableIntentAppended { .. } => "durable_intent_appended",
            Self::MemoryFactsCommitted { .. } => "memory_facts_committed",
            Self::ProjectionPublished { .. } => "projection_published",
            Self::OutboxSettled { .. } => "outbox_settled",
            Self::Terminalized => "terminalized",
            Self::Indeterminate { .. } => "indeterminate",
            Self::ReconciledApplied { .. } => "reconciled_applied",
            Self::ReconciledNotApplied { .. } => "reconciled_not_applied",
            Self::Quarantined { .. } => "quarantined",
        }
    }

    fn to_action(&self) -> Result<IntelligenceMutationAction, CognitiveStoreError> {
        Ok(match self {
            Self::SourceWitnessed { source_sha256 } => {
                IntelligenceMutationAction::WitnessSource {
                    source_sha256: parse_digest(source_sha256, "source digest")?,
                }
            }
            Self::GroundingValidated {
                grounding_receipt_sha256,
            } => IntelligenceMutationAction::ValidateGrounding {
                grounding_receipt_sha256: parse_digest(
                    grounding_receipt_sha256,
                    "grounding receipt digest",
                )?,
            },
            Self::DurableIntentAppended { intent_sha256 } => {
                IntelligenceMutationAction::AppendDurableIntent {
                    intent_sha256: parse_digest(intent_sha256, "intent digest")?,
                }
            }
            Self::MemoryFactsCommitted {
                write_receipt_sha256,
            } => IntelligenceMutationAction::CommitMemoryFacts {
                write_receipt_sha256: parse_digest(
                    write_receipt_sha256,
                    "write receipt digest",
                )?,
            },
            Self::ProjectionPublished {
                expected_previous_generation,
                new_generation,
                projection_receipt_sha256,
            } => IntelligenceMutationAction::PublishProjection {
                expected_previous_generation: *expected_previous_generation,
                new_generation: *new_generation,
                projection_receipt_sha256: parse_digest(
                    projection_receipt_sha256,
                    "projection receipt digest",
                )?,
            },
            Self::OutboxSettled { outcome_sha256 } => {
                IntelligenceMutationAction::SettleOutbox {
                    outcome_sha256: parse_digest(outcome_sha256, "outcome digest")?,
                }
            }
            Self::Terminalized => IntelligenceMutationAction::Terminalize,
            Self::Indeterminate { reason_sha256 } => {
                IntelligenceMutationAction::MarkIndeterminate {
                    reason_sha256: parse_digest(reason_sha256, "reason digest")?,
                }
            }
            Self::ReconciledApplied { outcome_sha256 } => {
                IntelligenceMutationAction::ReconcileApplied {
                    outcome_sha256: parse_digest(outcome_sha256, "outcome digest")?,
                }
            }
            Self::ReconciledNotApplied { outcome_sha256 } => {
                IntelligenceMutationAction::ReconcileNotApplied {
                    outcome_sha256: parse_digest(outcome_sha256, "outcome digest")?,
                }
            }
            Self::Quarantined { reason_sha256 } => {
                IntelligenceMutationAction::Quarantine {
                    reason_sha256: parse_digest(reason_sha256, "reason digest")?,
                }
            }
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct PreparedShadowObservation {
    schema_version: u32,
    namespace: String,
    binding: ShadowHostBindingDraft,
    sequence: u64,
    causal_parent_sha256: Option<String>,
    observation: ShadowHostObservation,
    prepared_sha256: String,
    runtime_wired: bool,
    default_open_wired: bool,
    memory_write_authority: bool,
    projection_write_authority: bool,
    outbox_dispatch_authority: bool,
    external_effects: bool,
    production_authority: bool,
}

#[derive(Debug, Serialize)]
struct ShadowHostBeginReceipt {
    schema_version: u32,
    namespace: &'static str,
    operation_id: String,
    owner_agent_id: String,
    binding_sha256: String,
    journal_disposition: &'static str,
    sqlite_journal_write_performed: bool,
    runtime_wired: bool,
    default_open_wired: bool,
    memory_write_performed_by_adapter: bool,
    projection_write_performed_by_adapter: bool,
    outbox_dispatch_performed_by_adapter: bool,
    external_effects: bool,
    production_authority: bool,
    operator_acceptance: bool,
    promotion: bool,
}

#[derive(Debug, Serialize)]
struct ShadowHostAppendReceipt {
    schema_version: u32,
    namespace: &'static str,
    operation_id: String,
    sequence: u64,
    observation: &'static str,
    from_phase: String,
    to_phase: String,
    prepared_sha256: String,
    request_sha256: String,
    causal_parent_sha256: Option<String>,
    transition_sha256: String,
    journal_disposition: &'static str,
    observed_durable_intent_appended: bool,
    observed_durable_intent_settled: bool,
    observed_memory_write_count: u8,
    observed_projection_publish_count: u8,
    observed_last_published_generation: u64,
    sqlite_journal_write_performed: bool,
    runtime_wired: bool,
    default_open_wired: bool,
    memory_write_performed_by_adapter: bool,
    projection_write_performed_by_adapter: bool,
    outbox_dispatch_performed_by_adapter: bool,
    external_effects: bool,
    production_authority: bool,
    operator_acceptance: bool,
    promotion: bool,
}

#[derive(Debug, Serialize)]
struct ShadowHostSnapshot {
    schema_version: u32,
    namespace: &'static str,
    owner_agent_id: String,
    operation_id: String,
    phase: String,
    next_sequence: u64,
    causal_parent_sha256: Option<String>,
    starting_projection_generation: u64,
    runtime_wired: bool,
    default_open_wired: bool,
    memory_write_authority: bool,
    projection_write_authority: bool,
    outbox_dispatch_authority: bool,
    external_effects: bool,
    production_authority: bool,
    operator_acceptance: bool,
    promotion: bool,
}

impl CognitiveStore {
    /// Opens the ordinary cognitive store plus the opt-in P0.4b journal.
    /// Nothing calls this method from the default runtime.
    pub async fn open_with_shadow_intelligence_mutation_host(
        layout: &HeptaAgentLayout,
    ) -> Result<Self, CognitiveStoreError> {
        Self::open_with_intelligence_mutation_journal(layout)
            .await
            .map_err(map_journal_error)
    }

    /// Creates or exact-replays one immutable shadow-operation binding.
    pub async fn begin_shadow_intelligence_mutation(
        &self,
        binding_json: &str,
    ) -> Result<String, CognitiveStoreError> {
        let draft: ShadowHostBindingDraft = parse_bounded_json(binding_json, "shadow binding")?;
        let binding = binding_from_draft(&draft)?;
        let disposition = self
            .begin_intelligence_mutation_journal(&binding)
            .await
            .map_err(map_journal_error)?;
        serialize(&ShadowHostBeginReceipt {
            schema_version: SHADOW_INTELLIGENCE_MUTATION_HOST_SCHEMA_VERSION,
            namespace: SHADOW_INTELLIGENCE_MUTATION_HOST_NAMESPACE,
            operation_id: binding.operation_id.clone(),
            owner_agent_id: self.owner_agent_id().as_str().to_string(),
            binding_sha256: shadow_binding_digest(&binding).as_str().to_string(),
            journal_disposition: journal_disposition(disposition),
            sqlite_journal_write_performed: disposition
                == IntelligenceMutationJournalDisposition::Applied,
            runtime_wired: SHADOW_INTELLIGENCE_MUTATION_HOST_RUNTIME_WIRED,
            default_open_wired: SHADOW_INTELLIGENCE_MUTATION_HOST_DEFAULT_OPEN_WIRED,
            memory_write_performed_by_adapter: false,
            projection_write_performed_by_adapter: false,
            outbox_dispatch_performed_by_adapter: false,
            external_effects: SHADOW_INTELLIGENCE_MUTATION_HOST_EXTERNAL_EFFECTS,
            production_authority: SHADOW_INTELLIGENCE_MUTATION_HOST_PRODUCTION_AUTHORITY,
            operator_acceptance: SHADOW_INTELLIGENCE_MUTATION_HOST_OPERATOR_ACCEPTANCE,
            promotion: SHADOW_INTELLIGENCE_MUTATION_HOST_PROMOTION,
        })
    }

    /// Freezes the next sequence and causal parent for an observed receipt.
    /// The returned JSON is reusable for an exact retry after acknowledgement
    /// loss; callers must not regenerate it with changed evidence.
    pub async fn prepare_shadow_intelligence_mutation_observation(
        &self,
        operation_id: &str,
        observation_json: &str,
    ) -> Result<String, CognitiveStoreError> {
        validate_id(operation_id, "operation id")?;
        let observation: ShadowHostObservation =
            parse_bounded_json(observation_json, "shadow observation")?;
        let _ = observation.to_action()?;
        let state = self
            .replay_intelligence_mutation_operation(operation_id)
            .await
            .map_err(map_journal_error)?;
        if state.binding().operation_id != operation_id {
            return Err(CognitiveStoreError::Corrupt(
                "shadow replay returned a different operation binding".to_string(),
            ));
        }
        let binding = binding_to_draft(state.binding());
        let causal_parent_sha256 = state
            .causal_parent_sha256()
            .map(|digest| digest.as_str().to_string());
        let mut prepared = PreparedShadowObservation {
            schema_version: SHADOW_INTELLIGENCE_MUTATION_HOST_SCHEMA_VERSION,
            namespace: PREPARED_NAMESPACE.to_string(),
            binding,
            sequence: state.next_sequence(),
            causal_parent_sha256,
            observation,
            prepared_sha256: Sha256Digest::for_bytes(b"uncomputed")
                .as_str()
                .to_string(),
            runtime_wired: false,
            default_open_wired: false,
            memory_write_authority: false,
            projection_write_authority: false,
            outbox_dispatch_authority: false,
            external_effects: false,
            production_authority: false,
        };
        prepared.prepared_sha256 = prepared_digest(&prepared)?.as_str().to_string();
        serialize(&prepared)
    }

    /// Appends one previously prepared observation to the qualification-only
    /// journal. No product mutation is executed by this method.
    pub async fn append_shadow_intelligence_mutation_observation(
        &self,
        prepared_json: &str,
    ) -> Result<String, CognitiveStoreError> {
        self.append_shadow_intelligence_mutation_observation_with_fault(
            prepared_json,
            IntelligenceMutationJournalFault::None,
        )
        .await
    }

    /// Convenience path for one prepare+append cycle. The response includes
    /// the exact prepared request so a caller can persist it before retrying.
    pub async fn observe_shadow_intelligence_mutation(
        &self,
        operation_id: &str,
        observation_json: &str,
    ) -> Result<String, CognitiveStoreError> {
        let prepared = self
            .prepare_shadow_intelligence_mutation_observation(operation_id, observation_json)
            .await?;
        let append = self
            .append_shadow_intelligence_mutation_observation(&prepared)
            .await?;
        let prepared_value: Value = serde_json::from_str(&prepared).map_err(json_error)?;
        let append_value: Value = serde_json::from_str(&append).map_err(json_error)?;
        serialize(&serde_json::json!({
            "schema_version": SHADOW_INTELLIGENCE_MUTATION_HOST_SCHEMA_VERSION,
            "namespace": SHADOW_INTELLIGENCE_MUTATION_HOST_NAMESPACE,
            "prepared": prepared_value,
            "append": append_value,
            "runtime_wired": false,
            "default_open_wired": false,
            "memory_write_performed_by_adapter": false,
            "projection_write_performed_by_adapter": false,
            "outbox_dispatch_performed_by_adapter": false,
            "external_effects": false,
            "production_authority": false,
            "operator_acceptance": false,
            "promotion": false
        }))
    }

    /// Returns the causal cursor for one operation without returning source or
    /// memory content and without performing any product write.
    pub async fn inspect_shadow_intelligence_mutation(
        &self,
        operation_id: &str,
    ) -> Result<String, CognitiveStoreError> {
        validate_id(operation_id, "operation id")?;
        let state = self
            .replay_intelligence_mutation_operation(operation_id)
            .await
            .map_err(map_journal_error)?;
        serialize(&ShadowHostSnapshot {
            schema_version: SHADOW_INTELLIGENCE_MUTATION_HOST_SCHEMA_VERSION,
            namespace: SHADOW_INTELLIGENCE_MUTATION_HOST_NAMESPACE,
            owner_agent_id: self.owner_agent_id().as_str().to_string(),
            operation_id: state.binding().operation_id.clone(),
            phase: state.phase().as_str().to_string(),
            next_sequence: state.next_sequence(),
            causal_parent_sha256: state
                .causal_parent_sha256()
                .map(|digest| digest.as_str().to_string()),
            starting_projection_generation: state.binding().starting_projection_generation,
            runtime_wired: false,
            default_open_wired: false,
            memory_write_authority: false,
            projection_write_authority: false,
            outbox_dispatch_authority: false,
            external_effects: false,
            production_authority: false,
            operator_acceptance: false,
            promotion: false,
        })
    }

    async fn append_shadow_intelligence_mutation_observation_with_fault(
        &self,
        prepared_json: &str,
        fault: IntelligenceMutationJournalFault,
    ) -> Result<String, CognitiveStoreError> {
        let prepared: PreparedShadowObservation =
            parse_bounded_json(prepared_json, "prepared shadow observation")?;
        validate_prepared(&prepared)?;
        let binding = binding_from_draft(&prepared.binding)?;
        let request = IntelligenceMutationTransitionRequest {
            binding,
            sequence: prepared.sequence,
            causal_parent_sha256: prepared
                .causal_parent_sha256
                .clone()
                .map(|digest| parse_digest(&digest, "causal parent digest"))
                .transpose()?,
            action: prepared.observation.to_action()?,
        };
        let append = self
            .append_intelligence_mutation_transition(request, fault)
            .await
            .map_err(map_journal_error)?;
        serialize(&append_receipt(&prepared, append))
    }
}

fn append_receipt(
    prepared: &PreparedShadowObservation,
    append: IntelligenceMutationJournalAppend,
) -> ShadowHostAppendReceipt {
    ShadowHostAppendReceipt {
        schema_version: SHADOW_INTELLIGENCE_MUTATION_HOST_SCHEMA_VERSION,
        namespace: SHADOW_INTELLIGENCE_MUTATION_HOST_NAMESPACE,
        operation_id: append.receipt.operation_id.clone(),
        sequence: append.receipt.sequence,
        observation: prepared.observation.kind(),
        from_phase: append.receipt.from_phase.as_str().to_string(),
        to_phase: append.receipt.to_phase.as_str().to_string(),
        prepared_sha256: prepared.prepared_sha256.clone(),
        request_sha256: append.receipt.request_sha256.as_str().to_string(),
        causal_parent_sha256: append
            .receipt
            .causal_parent_sha256
            .as_ref()
            .map(|digest| digest.as_str().to_string()),
        transition_sha256: append.receipt.transition_sha256.as_str().to_string(),
        journal_disposition: journal_disposition(append.disposition),
        observed_durable_intent_appended: append.receipt.durable_intent_appended,
        observed_durable_intent_settled: append.receipt.durable_intent_settled,
        observed_memory_write_count: append.receipt.memory_write_count,
        observed_projection_publish_count: append.receipt.projection_publish_count,
        observed_last_published_generation: append.receipt.last_published_generation,
        sqlite_journal_write_performed: append.disposition
            == IntelligenceMutationJournalDisposition::Applied,
        runtime_wired: false,
        default_open_wired: false,
        memory_write_performed_by_adapter: false,
        projection_write_performed_by_adapter: false,
        outbox_dispatch_performed_by_adapter: false,
        external_effects: false,
        production_authority: false,
        operator_acceptance: false,
        promotion: false,
    }
}

fn validate_prepared(prepared: &PreparedShadowObservation) -> Result<(), CognitiveStoreError> {
    if prepared.schema_version != SHADOW_INTELLIGENCE_MUTATION_HOST_SCHEMA_VERSION
        || prepared.namespace != PREPARED_NAMESPACE
    {
        return Err(CognitiveStoreError::Invalid(
            "unsupported prepared shadow-observation contract".to_string(),
        ));
    }
    if prepared.runtime_wired
        || prepared.default_open_wired
        || prepared.memory_write_authority
        || prepared.projection_write_authority
        || prepared.outbox_dispatch_authority
        || prepared.external_effects
        || prepared.production_authority
    {
        return Err(CognitiveStoreError::AccessDenied(
            "prepared shadow observation crosses the authority boundary".to_string(),
        ));
    }
    parse_digest(&prepared.prepared_sha256, "prepared observation digest")?;
    let expected = prepared_digest(prepared)?;
    if expected.as_str() != prepared.prepared_sha256 {
        return Err(CognitiveStoreError::Conflict(
            "prepared shadow observation digest does not match its contents".to_string(),
        ));
    }
    let _ = binding_from_draft(&prepared.binding)?;
    let _ = prepared.observation.to_action()?;
    Ok(())
}

fn binding_from_draft(
    draft: &ShadowHostBindingDraft,
) -> Result<IntelligenceMutationBinding, CognitiveStoreError> {
    validate_id(&draft.operation_id, "operation id")?;
    validate_id(&draft.lease_id, "lease id")?;
    let binding = IntelligenceMutationBinding {
        operation_id: draft.operation_id.clone(),
        lease_id: draft.lease_id.clone(),
        lease_epoch: draft.lease_epoch,
        expected_revision: draft.expected_revision,
        starting_projection_generation: draft.starting_projection_generation,
        causal_root_sha256: parse_digest(&draft.causal_root_sha256, "causal root digest")?,
    };
    binding
        .validate()
        .map_err(|error| CognitiveStoreError::Invalid(error.to_string()))?;
    Ok(binding)
}

fn binding_to_draft(binding: &IntelligenceMutationBinding) -> ShadowHostBindingDraft {
    ShadowHostBindingDraft {
        operation_id: binding.operation_id.clone(),
        lease_id: binding.lease_id.clone(),
        lease_epoch: binding.lease_epoch,
        expected_revision: binding.expected_revision,
        starting_projection_generation: binding.starting_projection_generation,
        causal_root_sha256: binding.causal_root_sha256.as_str().to_string(),
    }
}

fn prepared_digest(
    prepared: &PreparedShadowObservation,
) -> Result<Sha256Digest, CognitiveStoreError> {
    let mut hasher = Sha256::new();
    frame_part(
        &mut hasher,
        b"hepta:intelligence:shadow-host-prepared-observation:v1",
    );
    frame_part(&mut hasher, &prepared.schema_version.to_be_bytes());
    frame_part(&mut hasher, prepared.namespace.as_bytes());
    frame_binding(&mut hasher, &prepared.binding);
    frame_part(&mut hasher, &prepared.sequence.to_be_bytes());
    match &prepared.causal_parent_sha256 {
        Some(digest) => frame_part(&mut hasher, digest.as_bytes()),
        None => frame_part(&mut hasher, b""),
    }
    let observation = serde_json::to_vec(&prepared.observation).map_err(json_error)?;
    frame_part(&mut hasher, &observation);
    frame_part(&mut hasher, &[0, 0, 0, 0, 0, 0, 0]);
    Ok(Sha256Digest::from_sha256_output(hasher.finalize()))
}

fn shadow_binding_digest(binding: &IntelligenceMutationBinding) -> Sha256Digest {
    let draft = binding_to_draft(binding);
    let mut hasher = Sha256::new();
    frame_part(
        &mut hasher,
        b"hepta:intelligence:shadow-host-binding:v1",
    );
    frame_binding(&mut hasher, &draft);
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn frame_binding(hasher: &mut Sha256, binding: &ShadowHostBindingDraft) {
    frame_part(hasher, binding.operation_id.as_bytes());
    frame_part(hasher, binding.lease_id.as_bytes());
    frame_part(hasher, &binding.lease_epoch.to_be_bytes());
    match binding.expected_revision {
        Some(revision) => frame_part(hasher, &revision.to_be_bytes()),
        None => frame_part(hasher, b""),
    }
    frame_part(
        hasher,
        &binding.starting_projection_generation.to_be_bytes(),
    );
    frame_part(hasher, binding.causal_root_sha256.as_bytes());
}

fn journal_disposition(disposition: IntelligenceMutationJournalDisposition) -> &'static str {
    match disposition {
        IntelligenceMutationJournalDisposition::Applied => "applied",
        IntelligenceMutationJournalDisposition::Replay => "replay",
    }
}

fn parse_bounded_json<T>(value: &str, label: &str) -> Result<T, CognitiveStoreError>
where
    T: for<'de> Deserialize<'de>,
{
    if value.is_empty() || value.len() > MAX_JSON_BYTES || value.as_bytes().contains(&0) {
        return Err(CognitiveStoreError::Invalid(format!(
            "{label} must contain 1..={MAX_JSON_BYTES} non-NUL bytes"
        )));
    }
    serde_json::from_str(value).map_err(|error| {
        CognitiveStoreError::Invalid(format!("invalid {label} JSON: {error}"))
    })
}

fn parse_digest(value: &str, label: &str) -> Result<Sha256Digest, CognitiveStoreError> {
    Sha256Digest::parse(value.to_string())
        .map_err(|error| CognitiveStoreError::Invalid(format!("invalid {label}: {error}")))
}

fn validate_id(value: &str, label: &str) -> Result<(), CognitiveStoreError> {
    if value.trim().is_empty() || value.len() > MAX_ID_BYTES || value.as_bytes().contains(&0) {
        return Err(CognitiveStoreError::Invalid(format!(
            "{label} must contain 1..={MAX_ID_BYTES} non-NUL bytes"
        )));
    }
    Ok(())
}

fn serialize<T>(value: &T) -> Result<String, CognitiveStoreError>
where
    T: Serialize,
{
    serde_json::to_string(value).map_err(json_error)
}

fn json_error(error: serde_json::Error) -> CognitiveStoreError {
    CognitiveStoreError::Unavailable(format!("serialize shadow host receipt: {error}"))
}

fn map_journal_error(error: IntelligenceMutationJournalError) -> CognitiveStoreError {
    match error {
        IntelligenceMutationJournalError::Store(error) => error,
        IntelligenceMutationJournalError::State(error) => {
            CognitiveStoreError::Conflict(error.to_string())
        }
        IntelligenceMutationJournalError::Injected(label) => {
            CognitiveStoreError::Unavailable(format!("shadow journal fault: {label}"))
        }
        IntelligenceMutationJournalError::Indeterminate(digest) => {
            CognitiveStoreError::Unavailable(format!(
                "shadow journal committed but acknowledgement is indeterminate: {}",
                digest.as_str()
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cognitive_test_support::agent_id;
    use crate::cognitive_test_support::layout;
    use tempfile::TempDir;

    fn digest(label: &str) -> String {
        Sha256Digest::for_bytes(label.as_bytes())
            .as_str()
            .to_string()
    }

    fn binding() -> String {
        serde_json::json!({
            "operation_id": "shadow-operation-1",
            "lease_id": "shadow-lease-1",
            "lease_epoch": 1,
            "expected_revision": null,
            "starting_projection_generation": 4,
            "causal_root_sha256": digest("causal-root")
        })
        .to_string()
    }

    async fn observe(store: &CognitiveStore, observation: Value) -> Value {
        let receipt = store
            .observe_shadow_intelligence_mutation("shadow-operation-1", &observation.to_string())
            .await
            .expect("shadow observation");
        serde_json::from_str(&receipt).expect("receipt json")
    }

    #[tokio::test]
    async fn shadow_host_records_full_path_without_product_effects() {
        let temp = TempDir::new().expect("temp");
        let owner = agent_id(241);
        let store = CognitiveStore::open_with_shadow_intelligence_mutation_host(&layout(&temp, &owner))
            .await
            .expect("store");
        let begin = store
            .begin_shadow_intelligence_mutation(&binding())
            .await
            .expect("begin");
        let begin: Value = serde_json::from_str(&begin).expect("begin json");
        assert_eq!(begin["journal_disposition"], "applied");
        assert_eq!(begin["production_authority"], false);

        observe(
            &store,
            serde_json::json!({
                "observation": "source_witnessed",
                "source_sha256": digest("source")
            }),
        )
        .await;
        observe(
            &store,
            serde_json::json!({
                "observation": "grounding_validated",
                "grounding_receipt_sha256": digest("grounding")
            }),
        )
        .await;
        observe(
            &store,
            serde_json::json!({
                "observation": "durable_intent_appended",
                "intent_sha256": digest("intent")
            }),
        )
        .await;
        observe(
            &store,
            serde_json::json!({
                "observation": "memory_facts_committed",
                "write_receipt_sha256": digest("write")
            }),
        )
        .await;
        observe(
            &store,
            serde_json::json!({
                "observation": "projection_published",
                "expected_previous_generation": 4,
                "new_generation": 5,
                "projection_receipt_sha256": digest("projection")
            }),
        )
        .await;
        observe(
            &store,
            serde_json::json!({
                "observation": "outbox_settled",
                "outcome_sha256": digest("outcome")
            }),
        )
        .await;
        let terminal = observe(
            &store,
            serde_json::json!({"observation": "terminalized"}),
        )
        .await;
        assert_eq!(terminal["append"]["to_phase"], "terminal");
        assert_eq!(terminal["append"]["observed_memory_write_count"], 1);
        assert_eq!(terminal["append"]["observed_projection_publish_count"], 1);
        assert_eq!(terminal["append"]["memory_write_performed_by_adapter"], false);
        assert_eq!(terminal["append"]["projection_write_performed_by_adapter"], false);
        assert_eq!(terminal["append"]["outbox_dispatch_performed_by_adapter"], false);

        let source_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM source_ledger")
            .fetch_one(&store.pool)
            .await
            .expect("source count");
        let memory_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM memory_revisions")
            .fetch_one(&store.pool)
            .await
            .expect("memory count");
        let projection_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM kg_projection")
            .fetch_one(&store.pool)
            .await
            .expect("projection count");
        let transition_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM cognitive_intelligence_mutation_transitions",
        )
        .fetch_one(&store.pool)
        .await
        .expect("transition count");
        assert_eq!(source_count, 0);
        assert_eq!(memory_count, 0);
        assert_eq!(projection_count, 0);
        assert_eq!(transition_count, 7);

        let snapshot = store
            .inspect_shadow_intelligence_mutation("shadow-operation-1")
            .await
            .expect("snapshot");
        let snapshot: Value = serde_json::from_str(&snapshot).expect("snapshot json");
        assert_eq!(snapshot["phase"], "terminal");
        assert_eq!(snapshot["runtime_wired"], false);
    }

    #[tokio::test]
    async fn prepared_request_supports_exact_retry_and_rejects_tamper() {
        let temp = TempDir::new().expect("temp");
        let owner = agent_id(242);
        let store = CognitiveStore::open_with_shadow_intelligence_mutation_host(&layout(&temp, &owner))
            .await
            .expect("store");
        store
            .begin_shadow_intelligence_mutation(&binding())
            .await
            .expect("begin");
        let prepared = store
            .prepare_shadow_intelligence_mutation_observation(
                "shadow-operation-1",
                &serde_json::json!({
                    "observation": "source_witnessed",
                    "source_sha256": digest("source")
                })
                .to_string(),
            )
            .await
            .expect("prepare");
        let first = store
            .append_shadow_intelligence_mutation_observation(&prepared)
            .await
            .expect("append");
        let replay = store
            .append_shadow_intelligence_mutation_observation(&prepared)
            .await
            .expect("replay");
        let first: Value = serde_json::from_str(&first).expect("first json");
        let replay: Value = serde_json::from_str(&replay).expect("replay json");
        assert_eq!(first["journal_disposition"], "applied");
        assert_eq!(replay["journal_disposition"], "replay");
        assert_eq!(first["transition_sha256"], replay["transition_sha256"]);

        let mut tampered: Value = serde_json::from_str(&prepared).expect("prepared json");
        tampered["sequence"] = serde_json::json!(9);
        assert!(
            store
                .append_shadow_intelligence_mutation_observation(&tampered.to_string())
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn postcommit_ack_loss_is_adopted_by_exact_retry() {
        let temp = TempDir::new().expect("temp");
        let owner = agent_id(243);
        let store = CognitiveStore::open_with_shadow_intelligence_mutation_host(&layout(&temp, &owner))
            .await
            .expect("store");
        store
            .begin_shadow_intelligence_mutation(&binding())
            .await
            .expect("begin");
        let prepared = store
            .prepare_shadow_intelligence_mutation_observation(
                "shadow-operation-1",
                &serde_json::json!({
                    "observation": "source_witnessed",
                    "source_sha256": digest("source")
                })
                .to_string(),
            )
            .await
            .expect("prepare");
        assert!(
            store
                .append_shadow_intelligence_mutation_observation_with_fault(
                    &prepared,
                    IntelligenceMutationJournalFault::AfterCommitBeforeReturn,
                )
                .await
                .is_err()
        );
        let replay = store
            .append_shadow_intelligence_mutation_observation(&prepared)
            .await
            .expect("replay");
        let replay: Value = serde_json::from_str(&replay).expect("replay json");
        assert_eq!(replay["journal_disposition"], "replay");
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM cognitive_intelligence_mutation_transitions",
        )
        .fetch_one(&store.pool)
        .await
        .expect("count");
        assert_eq!(count, 1);
    }
}
