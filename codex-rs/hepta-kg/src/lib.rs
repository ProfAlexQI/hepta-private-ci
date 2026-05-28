//! Hepta-native knowledge graph contracts.
//!
//! This crate defines the boundary between Hepta memory/intelligence and any
//! durable graph backend. It intentionally starts as a dry-run and review-only
//! layer: callers can build write candidates and audit plans, but the default
//! gate refuses live writes until provenance, redaction, idempotency, rollback,
//! and post-write validation are wired by an adapter.

use serde::{Deserialize, Serialize};

pub const DEFAULT_KG_SCHEMA_VERSION: &str = "hepta-kg-v0";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KgEntity {
    pub id: String,
    pub kind: KgEntityKind,
    pub label: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub source_spans: Vec<KgSourceSpan>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KgEntityKind {
    Person,
    Project,
    Task,
    CodeArtifact,
    Decision,
    Preference,
    Capability,
    Memory,
    Session,
    Document,
    ExternalSystem,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KgRelation {
    pub id: String,
    pub kind: KgRelationKind,
    pub from_entity_id: String,
    pub to_entity_id: String,
    pub confidence: KgConfidence,
    #[serde(default)]
    pub temporal: KgTemporalValidity,
    #[serde(default)]
    pub source_spans: Vec<KgSourceSpan>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KgRelationKind {
    Mentions,
    RelatedTo,
    DependsOn,
    Decided,
    Supersedes,
    ConflictsWith,
    Supports,
    DerivedFrom,
    TriggeredBy,
    TemporalContinuation,
    Causal,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct KgConfidence {
    pub basis_points: u16,
}

impl KgConfidence {
    pub const ZERO: Self = Self { basis_points: 0 };
    pub const CERTAIN: Self = Self {
        basis_points: 10_000,
    };

    pub fn new(basis_points: u16) -> Self {
        Self {
            basis_points: basis_points.min(10_000),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct KgTemporalValidity {
    #[serde(default)]
    pub observed_at_unix_ms: Option<i64>,
    #[serde(default)]
    pub valid_from_unix_ms: Option<i64>,
    #[serde(default)]
    pub valid_to_unix_ms: Option<i64>,
    #[serde(default)]
    pub superseded_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KgEpisode {
    pub id: String,
    pub kind: KgEpisodeKind,
    pub summary: String,
    #[serde(default)]
    pub occurred_at_unix_ms: Option<i64>,
    #[serde(default)]
    pub source_spans: Vec<KgSourceSpan>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KgEpisodeKind {
    ConversationTurn,
    TaskResult,
    CodeChange,
    DocumentIngest,
    OperatorDecision,
    ExternalObservation,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KgSourceSpan {
    pub source_id: String,
    pub source_kind: KgSourceKind,
    #[serde(default)]
    pub uri: Option<String>,
    #[serde(default)]
    pub start_offset: Option<usize>,
    #[serde(default)]
    pub end_offset: Option<usize>,
    #[serde(default)]
    pub excerpt_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KgSourceKind {
    Transcript,
    MemoryRecord,
    File,
    RuntimeReport,
    ToolResult,
    OperatorInput,
    ExternalDocument,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KgProvenance {
    pub producer: String,
    #[serde(default = "default_schema_version")]
    pub schema_version: String,
    #[serde(default)]
    pub source_spans: Vec<KgSourceSpan>,
    #[serde(default)]
    pub redaction: KgRedactionState,
    #[serde(default)]
    pub operator_review: KgOperatorReviewState,
}

impl KgProvenance {
    pub fn has_source_evidence(&self) -> bool {
        !self.source_spans.is_empty()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KgRedactionState {
    #[default]
    NotReviewed,
    Redacted,
    NoSensitiveContent,
}

impl KgRedactionState {
    pub fn is_cleared(&self) -> bool {
        matches!(self, Self::Redacted | Self::NoSensitiveContent)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KgOperatorReviewState {
    #[default]
    NotReviewed,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KgWriteCandidate {
    pub id: String,
    #[serde(default = "default_schema_version")]
    pub schema_version: String,
    pub episode: KgEpisode,
    #[serde(default)]
    pub entities: Vec<KgEntity>,
    #[serde(default)]
    pub relations: Vec<KgRelation>,
    pub provenance: KgProvenance,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

impl KgWriteCandidate {
    pub fn has_graph_payload(&self) -> bool {
        !self.entities.is_empty() || !self.relations.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KgWritePolicy {
    pub mode: KgWriteMode,
    pub allow_external_side_effects: bool,
    pub require_operator_approval: bool,
    pub require_redaction_clearance: bool,
    pub require_idempotency_key: bool,
    pub require_post_write_validation: bool,
}

impl Default for KgWritePolicy {
    fn default() -> Self {
        Self {
            mode: KgWriteMode::DryRun,
            allow_external_side_effects: false,
            require_operator_approval: true,
            require_redaction_clearance: true,
            require_idempotency_key: true,
            require_post_write_validation: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KgWriteMode {
    DryRun,
    ReviewRequired,
    LiveWrite,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KgWritePlan {
    pub candidate_id: String,
    pub mode: KgWriteMode,
    pub live_write_allowed: bool,
    pub review_required: bool,
    pub external_side_effects_allowed: bool,
    #[serde(default)]
    pub blockers: Vec<KgWriteBlocker>,
}

impl KgWritePlan {
    pub fn ready_for_live_write(&self) -> bool {
        self.live_write_allowed && self.blockers.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KgWriteBlocker {
    DryRunGate,
    ReviewGate,
    MissingGraphPayload,
    MissingSourceProvenance,
    RedactionNotCleared,
    OperatorApprovalMissing,
    IdempotencyKeyMissing,
    ExternalSideEffectsDisabled,
    PostWriteValidationRequired,
}

pub const KG_EXTERNAL_ADAPTER_DRY_RUN_CONTRACT: &str = "hepta-kg-external-adapter-dry-run-v0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KgExternalAdapterKind {
    Graphiti,
    Neo4j,
    CocoIndex,
}

impl KgExternalAdapterKind {
    pub const ALL: [Self; 3] = [Self::Graphiti, Self::Neo4j, Self::CocoIndex];

    pub fn id(self) -> &'static str {
        match self {
            Self::Graphiti => "graphiti",
            Self::Neo4j => "neo4j",
            Self::CocoIndex => "cocoindex",
        }
    }

    pub fn projection_family(self) -> &'static str {
        match self {
            Self::Graphiti => "episode-entity-relation-temporal-memory",
            Self::Neo4j => "node-relationship-property-graph",
            Self::CocoIndex => "document-chunk-entity-index",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KgExternalAdapterDryRunPlan {
    pub adapter: KgExternalAdapterKind,
    pub adapter_id: String,
    pub projection_family: String,
    pub contract: String,
    pub candidate_id: String,
    pub projected_episode_records: usize,
    pub projected_entity_records: usize,
    pub projected_relation_records: usize,
    pub projected_total_records: usize,
    pub network_call_allowed: bool,
    pub external_write_allowed: bool,
    pub live_write_allowed: bool,
    #[serde(default)]
    pub blockers: Vec<KgExternalAdapterBlocker>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KgExternalAdapterBlocker {
    AdapterDryRunGate,
    NetworkDisabled,
    ExternalWriteDisabled,
    AdapterReviewRequired,
    SourceWritePlanNotLiveReady,
    MissingGraphPayload,
    MissingSourceProvenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KgWriteAudit {
    pub candidate_id: String,
    pub planned_mode: KgWriteMode,
    pub live_write_performed: bool,
    pub persisted_records: usize,
    #[serde(default)]
    pub blockers: Vec<KgWriteBlocker>,
}

pub trait KnowledgeGraphStore {
    type Error;

    fn preview_write(&self, candidate: &KgWriteCandidate, policy: &KgWritePolicy) -> KgWritePlan;

    fn commit_write(
        &self,
        candidate: &KgWriteCandidate,
        policy: &KgWritePolicy,
    ) -> Result<KgWriteAudit, Self::Error>;
}

#[derive(Debug, Clone, Default)]
pub struct DryRunKnowledgeGraphStore;

impl KnowledgeGraphStore for DryRunKnowledgeGraphStore {
    type Error = KgWriteError;

    fn preview_write(&self, candidate: &KgWriteCandidate, policy: &KgWritePolicy) -> KgWritePlan {
        plan_kg_write(candidate, policy)
    }

    fn commit_write(
        &self,
        candidate: &KgWriteCandidate,
        policy: &KgWritePolicy,
    ) -> Result<KgWriteAudit, Self::Error> {
        let plan = self.preview_write(candidate, policy);
        Err(KgWriteError::LiveWriteDisabled(KgWriteAudit {
            candidate_id: plan.candidate_id,
            planned_mode: plan.mode,
            live_write_performed: false,
            persisted_records: 0,
            blockers: plan.blockers,
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KgWriteError {
    LiveWriteDisabled(KgWriteAudit),
}

pub fn plan_kg_write(candidate: &KgWriteCandidate, policy: &KgWritePolicy) -> KgWritePlan {
    let mut blockers = Vec::new();
    let review_required = policy.mode != KgWriteMode::DryRun;

    if policy.mode == KgWriteMode::DryRun {
        blockers.push(KgWriteBlocker::DryRunGate);
    }
    if policy.mode == KgWriteMode::ReviewRequired {
        blockers.push(KgWriteBlocker::ReviewGate);
    }
    if !candidate.has_graph_payload() {
        blockers.push(KgWriteBlocker::MissingGraphPayload);
    }
    if !candidate.provenance.has_source_evidence() {
        blockers.push(KgWriteBlocker::MissingSourceProvenance);
    }
    if policy.require_redaction_clearance && !candidate.provenance.redaction.is_cleared() {
        blockers.push(KgWriteBlocker::RedactionNotCleared);
    }
    if policy.require_operator_approval
        && candidate.provenance.operator_review != KgOperatorReviewState::Approved
    {
        blockers.push(KgWriteBlocker::OperatorApprovalMissing);
    }
    if policy.require_idempotency_key && candidate.idempotency_key.is_none() {
        blockers.push(KgWriteBlocker::IdempotencyKeyMissing);
    }
    if !policy.allow_external_side_effects {
        blockers.push(KgWriteBlocker::ExternalSideEffectsDisabled);
    }
    if policy.require_post_write_validation {
        blockers.push(KgWriteBlocker::PostWriteValidationRequired);
    }

    KgWritePlan {
        candidate_id: candidate.id.clone(),
        mode: policy.mode,
        live_write_allowed: policy.mode == KgWriteMode::LiveWrite && blockers.is_empty(),
        review_required,
        external_side_effects_allowed: policy.allow_external_side_effects,
        blockers,
    }
}

pub fn plan_external_adapter_dry_run(
    candidate: &KgWriteCandidate,
    source_plan: &KgWritePlan,
    adapter: KgExternalAdapterKind,
) -> KgExternalAdapterDryRunPlan {
    let mut blockers = vec![
        KgExternalAdapterBlocker::AdapterDryRunGate,
        KgExternalAdapterBlocker::NetworkDisabled,
        KgExternalAdapterBlocker::ExternalWriteDisabled,
        KgExternalAdapterBlocker::AdapterReviewRequired,
    ];

    if !source_plan.ready_for_live_write() {
        blockers.push(KgExternalAdapterBlocker::SourceWritePlanNotLiveReady);
    }
    if !candidate.has_graph_payload() {
        blockers.push(KgExternalAdapterBlocker::MissingGraphPayload);
    }
    if !candidate.provenance.has_source_evidence() {
        blockers.push(KgExternalAdapterBlocker::MissingSourceProvenance);
    }

    let projected_episode_records = usize::from(!candidate.episode.id.trim().is_empty());
    let projected_entity_records = candidate.entities.len();
    let projected_relation_records = candidate.relations.len();
    let projected_total_records =
        projected_episode_records + projected_entity_records + projected_relation_records;

    KgExternalAdapterDryRunPlan {
        adapter,
        adapter_id: adapter.id().to_string(),
        projection_family: adapter.projection_family().to_string(),
        contract: KG_EXTERNAL_ADAPTER_DRY_RUN_CONTRACT.to_string(),
        candidate_id: candidate.id.clone(),
        projected_episode_records,
        projected_entity_records,
        projected_relation_records,
        projected_total_records,
        network_call_allowed: false,
        external_write_allowed: false,
        live_write_allowed: false,
        blockers,
    }
}

fn default_schema_version() -> String {
    DEFAULT_KG_SCHEMA_VERSION.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_is_dry_run_and_blocks_live_write() {
        let candidate = sample_candidate();
        let policy = KgWritePolicy::default();

        let plan = plan_kg_write(&candidate, &policy);

        assert_eq!(plan.mode, KgWriteMode::DryRun);
        assert!(!plan.live_write_allowed);
        assert!(plan.blockers.contains(&KgWriteBlocker::DryRunGate));
        assert!(
            plan.blockers
                .contains(&KgWriteBlocker::ExternalSideEffectsDisabled)
        );
    }

    #[test]
    fn provenance_redaction_approval_and_idempotency_are_required_for_live_plans() {
        let mut candidate = sample_candidate();
        candidate.provenance.redaction = KgRedactionState::NotReviewed;
        candidate.provenance.operator_review = KgOperatorReviewState::NotReviewed;
        candidate.idempotency_key = None;

        let policy = KgWritePolicy {
            mode: KgWriteMode::LiveWrite,
            allow_external_side_effects: true,
            require_operator_approval: true,
            require_redaction_clearance: true,
            require_idempotency_key: true,
            require_post_write_validation: false,
        };

        let plan = plan_kg_write(&candidate, &policy);

        assert!(!plan.live_write_allowed);
        assert!(plan.blockers.contains(&KgWriteBlocker::RedactionNotCleared));
        assert!(
            plan.blockers
                .contains(&KgWriteBlocker::OperatorApprovalMissing)
        );
        assert!(
            plan.blockers
                .contains(&KgWriteBlocker::IdempotencyKeyMissing)
        );
    }

    #[test]
    fn live_plan_can_become_ready_only_when_all_gates_are_explicitly_cleared() {
        let candidate = sample_candidate();
        let policy = KgWritePolicy {
            mode: KgWriteMode::LiveWrite,
            allow_external_side_effects: true,
            require_operator_approval: true,
            require_redaction_clearance: true,
            require_idempotency_key: true,
            require_post_write_validation: false,
        };

        let plan = plan_kg_write(&candidate, &policy);

        assert!(plan.ready_for_live_write());
        assert!(plan.blockers.is_empty());
    }

    #[test]
    fn dry_run_store_never_commits_even_when_plan_is_live_ready() {
        let candidate = sample_candidate();
        let policy = KgWritePolicy {
            mode: KgWriteMode::LiveWrite,
            allow_external_side_effects: true,
            require_operator_approval: true,
            require_redaction_clearance: true,
            require_idempotency_key: true,
            require_post_write_validation: false,
        };

        let store = DryRunKnowledgeGraphStore;
        let err = store
            .commit_write(&candidate, &policy)
            .expect_err("dry-run store must not perform live writes");

        let KgWriteError::LiveWriteDisabled(audit) = err;
        assert!(!audit.live_write_performed);
        assert_eq!(audit.persisted_records, 0);
    }

    #[test]
    fn write_candidate_roundtrips_through_json() {
        let candidate = sample_candidate();

        let json = serde_json::to_string(&candidate).expect("candidate should serialize");
        let parsed: KgWriteCandidate =
            serde_json::from_str(&json).expect("candidate should deserialize");

        assert_eq!(parsed, candidate);
        assert_eq!(parsed.schema_version, DEFAULT_KG_SCHEMA_VERSION);
    }

    #[test]
    fn external_adapter_dry_run_projects_without_network_or_writes() {
        let candidate = sample_candidate();
        let write_plan = plan_kg_write(&candidate, &KgWritePolicy::default());

        let plan =
            plan_external_adapter_dry_run(&candidate, &write_plan, KgExternalAdapterKind::Graphiti);

        assert_eq!(plan.adapter_id, "graphiti");
        assert_eq!(
            plan.projection_family,
            "episode-entity-relation-temporal-memory"
        );
        assert_eq!(plan.projected_episode_records, 1);
        assert_eq!(plan.projected_entity_records, 1);
        assert_eq!(plan.projected_relation_records, 1);
        assert_eq!(plan.projected_total_records, 3);
        assert!(!plan.network_call_allowed);
        assert!(!plan.external_write_allowed);
        assert!(!plan.live_write_allowed);
        assert!(
            plan.blockers
                .contains(&KgExternalAdapterBlocker::AdapterDryRunGate)
        );
        assert!(
            plan.blockers
                .contains(&KgExternalAdapterBlocker::NetworkDisabled)
        );
        assert!(
            plan.blockers
                .contains(&KgExternalAdapterBlocker::ExternalWriteDisabled)
        );
    }

    #[test]
    fn external_adapter_dry_run_covers_graphiti_neo4j_and_cocoindex() {
        let candidate = sample_candidate();
        let write_plan = plan_kg_write(&candidate, &KgWritePolicy::default());
        let plans = KgExternalAdapterKind::ALL
            .into_iter()
            .map(|adapter| plan_external_adapter_dry_run(&candidate, &write_plan, adapter))
            .collect::<Vec<_>>();

        assert_eq!(plans.len(), 3);
        assert!(plans.iter().all(|plan| !plan.network_call_allowed));
        assert!(plans.iter().all(|plan| !plan.external_write_allowed));
        assert!(plans.iter().all(|plan| !plan.live_write_allowed));
        assert!(plans.iter().any(|plan| plan.adapter_id == "graphiti"));
        assert!(plans.iter().any(|plan| plan.adapter_id == "neo4j"));
        assert!(plans.iter().any(|plan| plan.adapter_id == "cocoindex"));
    }

    fn sample_candidate() -> KgWriteCandidate {
        let source = KgSourceSpan {
            source_id: "turn:21179".to_string(),
            source_kind: KgSourceKind::Transcript,
            uri: Some("telegram:6476198178/21179".to_string()),
            start_offset: None,
            end_offset: None,
            excerpt_hash: Some("sha256:sample".to_string()),
        };

        KgWriteCandidate {
            id: "kg-write:hepta-kg-skeleton".to_string(),
            schema_version: DEFAULT_KG_SCHEMA_VERSION.to_string(),
            episode: KgEpisode {
                id: "episode:hepta-kg-skeleton".to_string(),
                kind: KgEpisodeKind::OperatorDecision,
                summary: "Introduce a Hepta-native KG dry-run boundary".to_string(),
                occurred_at_unix_ms: Some(1_779_942_000_000),
                source_spans: vec![source.clone()],
            },
            entities: vec![KgEntity {
                id: "project:hepta".to_string(),
                kind: KgEntityKind::Project,
                label: "Hepta".to_string(),
                aliases: vec!["OpenClaw Hepta".to_string()],
                source_spans: vec![source.clone()],
            }],
            relations: vec![KgRelation {
                id: "relation:hepta-has-kg-boundary".to_string(),
                kind: KgRelationKind::Supports,
                from_entity_id: "project:hepta".to_string(),
                to_entity_id: "capability:knowledge-graph".to_string(),
                confidence: KgConfidence::new(9_500),
                temporal: KgTemporalValidity {
                    observed_at_unix_ms: Some(1_779_942_000_000),
                    valid_from_unix_ms: None,
                    valid_to_unix_ms: None,
                    superseded_by: None,
                },
                source_spans: vec![source.clone()],
            }],
            provenance: KgProvenance {
                producer: "hepta-intelligence".to_string(),
                schema_version: DEFAULT_KG_SCHEMA_VERSION.to_string(),
                source_spans: vec![source],
                redaction: KgRedactionState::NoSensitiveContent,
                operator_review: KgOperatorReviewState::Approved,
            },
            idempotency_key: Some("kg-write:hepta-kg-skeleton:v1".to_string()),
        }
    }
}
