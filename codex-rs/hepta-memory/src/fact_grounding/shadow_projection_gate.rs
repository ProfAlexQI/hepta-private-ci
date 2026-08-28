//! P0.3 source-only grounded projection comparison.
//!
//! The module computes an alternative projection view containing only current
//! verified/active heads with complete durable grounding receipts. It never
//! updates `kg_projection`, never changes recall queries, and never grants
//! production or external-effect authority.

use codex_hepta_contracts::Sha256Digest;
use serde::Serialize;
use sha2::Digest;
use sha2::Sha256;
use sqlx::Row;

use crate::CognitiveScope;
use crate::CognitiveStore;
use crate::CognitiveStoreError;
use crate::StableMemoryId;
use crate::cognitive_store::unavailable;

const SHADOW_GATE_SCHEMA_VERSION: u32 = 1;
const SHADOW_GATE_MODE: &str = "grounded_projection_shadow_compare_v1";
const MAX_SHADOW_HEADS: usize = 10_000;

#[derive(Clone, Debug, Serialize)]
struct ShadowProjectionHead {
    memory_id: String,
    revision: u64,
    fact_set_sha256: String,
    grounding_receipt_sha256: String,
    entity_count: u64,
    relation_count: u64,
}

#[derive(Clone, Debug, Serialize)]
struct ExcludedProjectionHead {
    memory_id: String,
    revision: u64,
    reason: &'static str,
    entity_count: u64,
    relation_count: u64,
}

#[derive(Clone, Debug, Serialize)]
struct CurrentProjectionSnapshot {
    generation: u64,
    output_sha256: Option<String>,
    node_count: u64,
    edge_count: u64,
}

#[derive(Clone, Debug, Serialize)]
struct GroundedCandidateSnapshot {
    candidate_sha256: String,
    node_count: u64,
    edge_count: u64,
    included_heads: Vec<ShadowProjectionHead>,
}

#[derive(Clone, Debug, Serialize)]
struct ShadowProjectionComparison {
    schema_version: u32,
    mode: &'static str,
    projection_scope: String,
    current: CurrentProjectionSnapshot,
    grounded_candidate: GroundedCandidateSnapshot,
    excluded_heads: Vec<ExcludedProjectionHead>,
    zero_fact_heads: Vec<ExcludedProjectionHead>,
    node_count_delta: i64,
    edge_count_delta: i64,
    write_performed: bool,
    default_projection_pointer_changed: bool,
    default_recall_query_changed: bool,
    production_projection_gate: bool,
    production_authority: bool,
    external_effects: bool,
    operator_acceptance: bool,
    promotion: bool,
}

#[derive(Clone, Debug, Serialize)]
struct GroundingEvidenceSpan {
    fact_kind: String,
    fact_key: String,
    evidence_ordinal: u32,
    start_byte: u32,
    end_byte: u32,
    evidence_sha256: String,
}

#[derive(Clone, Debug, Serialize)]
struct ShadowGroundingExplanation {
    schema_version: u32,
    mode: &'static str,
    memory_id: String,
    revision: u64,
    projection_scope: String,
    content_sha256: String,
    verification: String,
    lifecycle: String,
    fact_set_sha256: String,
    fact_identity_sha256: Option<String>,
    grounding_receipt_sha256: Option<String>,
    grounding_status: &'static str,
    entity_count: u64,
    relation_count: u64,
    evidence: Vec<GroundingEvidenceSpan>,
    write_performed: bool,
    production_projection_gate: bool,
    production_authority: bool,
    external_effects: bool,
    operator_acceptance: bool,
    promotion: bool,
}

impl CognitiveStore {
    /// Computes a grounded-only candidate beside the current projection.
    ///
    /// The method verifies the durable ledger first and performs no projection
    /// pointer write. The returned JSON is a descriptive qualification receipt.
    pub async fn shadow_grounded_projection_compare(
        &self,
        scope: &CognitiveScope,
    ) -> Result<String, CognitiveStoreError> {
        self.ensure_durable_fact_grounding_schema().await?;
        self.verify_durable_fact_grounding_ledger().await?;

        let projection_scope = scope.projection_key();
        let current = read_current_projection(self, &projection_scope).await?;
        let (scope_kind, workspace_sha256) = scope.database_parts();
        let rows = sqlx::query(
            "SELECT r.memory_id, r.revision, r.verification, r.lifecycle,
                    f.fact_set_sha256, f.entity_count, f.relation_count,
                    g.receipt_sha256
             FROM memory_heads AS h
             JOIN memory_revisions AS r
               ON r.memory_id = h.memory_id AND r.revision = h.revision
             JOIN kg_revision_fact_sets AS f
               ON f.memory_id = r.memory_id AND f.memory_revision = r.revision
             LEFT JOIN kg_revision_fact_grounding_receipts AS g
               ON g.memory_id = r.memory_id AND g.memory_revision = r.revision
             WHERE r.owner_agent_id = ? AND r.scope_kind = ?
               AND r.workspace_sha256 IS ?
             ORDER BY r.memory_id LIMIT ?",
        )
        .bind(self.owner_agent_id.as_str())
        .bind(scope_kind)
        .bind(workspace_sha256)
        .bind(limit_plus_one(MAX_SHADOW_HEADS)?)
        .fetch_all(&self.pool)
        .await
        .map_err(unavailable)?;
        if rows.len() > MAX_SHADOW_HEADS {
            return Err(CognitiveStoreError::Corrupt(format!(
                "shadow grounded projection exceeds {MAX_SHADOW_HEADS} heads"
            )));
        }

        let mut included = Vec::new();
        let mut excluded = Vec::new();
        let mut zero_fact = Vec::new();
        let mut candidate_nodes = 0_u64;
        let mut candidate_edges = 0_u64;
        for row in rows {
            let memory_id: String = row.try_get("memory_id").map_err(unavailable)?;
            let revision_i64: i64 = row.try_get("revision").map_err(unavailable)?;
            let revision = positive_u64(revision_i64, "memory revision")?;
            let verification: String = row.try_get("verification").map_err(unavailable)?;
            let lifecycle: String = row.try_get("lifecycle").map_err(unavailable)?;
            let fact_set_sha256: String = row.try_get("fact_set_sha256").map_err(unavailable)?;
            Sha256Digest::parse(fact_set_sha256.clone()).map_err(CognitiveStoreError::Corrupt)?;
            let entity_count = nonnegative_u64(
                row.try_get("entity_count").map_err(unavailable)?,
                "entity count",
            )?;
            let relation_count = nonnegative_u64(
                row.try_get("relation_count").map_err(unavailable)?,
                "relation count",
            )?;
            let receipt_sha256: Option<String> =
                row.try_get("receipt_sha256").map_err(unavailable)?;

            if entity_count + relation_count == 0 {
                zero_fact.push(ExcludedProjectionHead {
                    memory_id,
                    revision,
                    reason: "zero_fact",
                    entity_count,
                    relation_count,
                });
                continue;
            }
            if verification != "verified" || lifecycle != "active" {
                excluded.push(ExcludedProjectionHead {
                    memory_id,
                    revision,
                    reason: "ineligible_head",
                    entity_count,
                    relation_count,
                });
                continue;
            }
            let Some(receipt_sha256) = receipt_sha256 else {
                excluded.push(ExcludedProjectionHead {
                    memory_id,
                    revision,
                    reason: "legacy_unreviewed",
                    entity_count,
                    relation_count,
                });
                continue;
            };
            Sha256Digest::parse(receipt_sha256.clone()).map_err(CognitiveStoreError::Corrupt)?;
            candidate_nodes = candidate_nodes.checked_add(entity_count).ok_or_else(|| {
                CognitiveStoreError::Corrupt("candidate node overflow".to_string())
            })?;
            candidate_edges = candidate_edges.checked_add(relation_count).ok_or_else(|| {
                CognitiveStoreError::Corrupt("candidate edge overflow".to_string())
            })?;
            included.push(ShadowProjectionHead {
                memory_id,
                revision,
                fact_set_sha256,
                grounding_receipt_sha256: receipt_sha256,
                entity_count,
                relation_count,
            });
        }

        let candidate_sha256 = grounded_candidate_digest(&projection_scope, &included);
        let node_count_delta = signed_delta(candidate_nodes, current.node_count)?;
        let edge_count_delta = signed_delta(candidate_edges, current.edge_count)?;
        serde_json::to_string(&ShadowProjectionComparison {
            schema_version: SHADOW_GATE_SCHEMA_VERSION,
            mode: SHADOW_GATE_MODE,
            projection_scope,
            current,
            grounded_candidate: GroundedCandidateSnapshot {
                candidate_sha256: candidate_sha256.as_str().to_string(),
                node_count: candidate_nodes,
                edge_count: candidate_edges,
                included_heads: included,
            },
            excluded_heads: excluded,
            zero_fact_heads: zero_fact,
            node_count_delta,
            edge_count_delta,
            write_performed: false,
            default_projection_pointer_changed: false,
            default_recall_query_changed: false,
            production_projection_gate: false,
            production_authority: false,
            external_effects: false,
            operator_acceptance: false,
            promotion: false,
        })
        .map_err(|error| CognitiveStoreError::Unavailable(error.to_string()))
    }

    /// Returns bounded fact-grounding evidence metadata for one revision.
    /// Source content is never returned by this shadow explain path.
    pub async fn shadow_grounding_explain(
        &self,
        memory_id: &StableMemoryId,
        revision: u64,
    ) -> Result<String, CognitiveStoreError> {
        self.ensure_durable_fact_grounding_schema().await?;
        self.verify_durable_fact_grounding_ledger().await?;
        let revision_i64 = i64::try_from(revision)
            .map_err(|_| CognitiveStoreError::Invalid("memory revision exceeds i64".to_string()))?;
        let row = sqlx::query(
            "SELECT m.scope_kind, m.workspace_sha256, m.content_sha256,
                    m.verification, m.lifecycle, f.fact_set_sha256,
                    f.entity_count, f.relation_count,
                    g.fact_identity_sha256, g.receipt_sha256
             FROM memory_revisions AS m
             JOIN kg_revision_fact_sets AS f
               ON f.memory_id = m.memory_id AND f.memory_revision = m.revision
             LEFT JOIN kg_revision_fact_grounding_receipts AS g
               ON g.memory_id = m.memory_id AND g.memory_revision = m.revision
             WHERE m.memory_id = ? AND m.revision = ?",
        )
        .bind(memory_id.as_str())
        .bind(revision_i64)
        .fetch_optional(&self.pool)
        .await
        .map_err(unavailable)?
        .ok_or_else(|| {
            CognitiveStoreError::Invalid("memory revision does not exist".to_string())
        })?;
        let scope = CognitiveScope::parse(
            row.try_get("scope_kind").map_err(unavailable)?,
            row.try_get("workspace_sha256").map_err(unavailable)?,
        )
        .map_err(CognitiveStoreError::Corrupt)?;
        let entity_count = nonnegative_u64(
            row.try_get("entity_count").map_err(unavailable)?,
            "entity count",
        )?;
        let relation_count = nonnegative_u64(
            row.try_get("relation_count").map_err(unavailable)?,
            "relation count",
        )?;
        let receipt_sha256: Option<String> = row.try_get("receipt_sha256").map_err(unavailable)?;
        let status = if entity_count + relation_count == 0 {
            "zero_fact"
        } else if receipt_sha256.is_some() {
            "grounded_v1"
        } else {
            "legacy_unreviewed"
        };
        let span_rows = sqlx::query(
            "SELECT fact_kind, fact_key, evidence_ordinal,
                    start_byte, end_byte, evidence_sha256
             FROM kg_revision_fact_grounding_spans
             WHERE memory_id = ? AND memory_revision = ?
             ORDER BY fact_kind, fact_key, evidence_ordinal",
        )
        .bind(memory_id.as_str())
        .bind(revision_i64)
        .fetch_all(&self.pool)
        .await
        .map_err(unavailable)?;
        let evidence = span_rows
            .into_iter()
            .map(|span| {
                Ok(GroundingEvidenceSpan {
                    fact_kind: span.try_get("fact_kind").map_err(unavailable)?,
                    fact_key: span.try_get("fact_key").map_err(unavailable)?,
                    evidence_ordinal: u32::try_from(
                        span.try_get::<i64, _>("evidence_ordinal")
                            .map_err(unavailable)?,
                    )
                    .map_err(|_| {
                        CognitiveStoreError::Corrupt(
                            "negative grounding evidence ordinal".to_string(),
                        )
                    })?,
                    start_byte: u32::try_from(
                        span.try_get::<i64, _>("start_byte").map_err(unavailable)?,
                    )
                    .map_err(|_| {
                        CognitiveStoreError::Corrupt("negative grounding start byte".to_string())
                    })?,
                    end_byte: u32::try_from(
                        span.try_get::<i64, _>("end_byte").map_err(unavailable)?,
                    )
                    .map_err(|_| {
                        CognitiveStoreError::Corrupt("negative grounding end byte".to_string())
                    })?,
                    evidence_sha256: span.try_get("evidence_sha256").map_err(unavailable)?,
                })
            })
            .collect::<Result<Vec<_>, CognitiveStoreError>>()?;
        serde_json::to_string(&ShadowGroundingExplanation {
            schema_version: SHADOW_GATE_SCHEMA_VERSION,
            mode: "grounding_explain_shadow_v1",
            memory_id: memory_id.as_str().to_string(),
            revision,
            projection_scope: scope.projection_key(),
            content_sha256: row.try_get("content_sha256").map_err(unavailable)?,
            verification: row.try_get("verification").map_err(unavailable)?,
            lifecycle: row.try_get("lifecycle").map_err(unavailable)?,
            fact_set_sha256: row.try_get("fact_set_sha256").map_err(unavailable)?,
            fact_identity_sha256: row.try_get("fact_identity_sha256").map_err(unavailable)?,
            grounding_receipt_sha256: receipt_sha256,
            grounding_status: status,
            entity_count,
            relation_count,
            evidence,
            write_performed: false,
            production_projection_gate: false,
            production_authority: false,
            external_effects: false,
            operator_acceptance: false,
            promotion: false,
        })
        .map_err(|error| CognitiveStoreError::Unavailable(error.to_string()))
    }
}

async fn read_current_projection(
    store: &CognitiveStore,
    projection_scope: &str,
) -> Result<CurrentProjectionSnapshot, CognitiveStoreError> {
    let row = sqlx::query(
        "SELECT p.generation, r.output_sha256
         FROM kg_projection AS p
         LEFT JOIN kg_projection_generation_receipts AS r
           ON r.projection_scope = p.projection_scope
          AND r.generation = p.generation
         WHERE p.projection_scope = ?",
    )
    .bind(projection_scope)
    .fetch_optional(&store.pool)
    .await
    .map_err(unavailable)?;
    let Some(row) = row else {
        return Ok(CurrentProjectionSnapshot {
            generation: 0,
            output_sha256: None,
            node_count: 0,
            edge_count: 0,
        });
    };
    let generation_i64: i64 = row.try_get("generation").map_err(unavailable)?;
    let generation = nonnegative_u64(generation_i64, "projection generation")?;
    let node_count = nonnegative_u64(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM kg_nodes
             WHERE projection_scope = ? AND generation = ?",
        )
        .bind(projection_scope)
        .bind(generation_i64)
        .fetch_one(&store.pool)
        .await
        .map_err(unavailable)?,
        "current node count",
    )?;
    let edge_count = nonnegative_u64(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM kg_edges
             WHERE projection_scope = ? AND generation = ?",
        )
        .bind(projection_scope)
        .bind(generation_i64)
        .fetch_one(&store.pool)
        .await
        .map_err(unavailable)?,
        "current edge count",
    )?;
    Ok(CurrentProjectionSnapshot {
        generation,
        output_sha256: row.try_get("output_sha256").map_err(unavailable)?,
        node_count,
        edge_count,
    })
}

fn grounded_candidate_digest(
    projection_scope: &str,
    heads: &[ShadowProjectionHead],
) -> Sha256Digest {
    let mut hasher = Sha256::new();
    super::frame_part(
        &mut hasher,
        b"hepta:cognitive:grounded-projection-shadow:v1",
    );
    super::frame_part(&mut hasher, projection_scope.as_bytes());
    super::frame_part(
        &mut hasher,
        &u64::try_from(heads.len()).unwrap_or(u64::MAX).to_be_bytes(),
    );
    for head in heads {
        super::frame_part(&mut hasher, head.memory_id.as_bytes());
        super::frame_part(&mut hasher, &head.revision.to_be_bytes());
        super::frame_part(&mut hasher, head.fact_set_sha256.as_bytes());
        super::frame_part(&mut hasher, head.grounding_receipt_sha256.as_bytes());
        super::frame_part(&mut hasher, &head.entity_count.to_be_bytes());
        super::frame_part(&mut hasher, &head.relation_count.to_be_bytes());
    }
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn positive_u64(value: i64, label: &str) -> Result<u64, CognitiveStoreError> {
    let value = u64::try_from(value)
        .map_err(|_| CognitiveStoreError::Corrupt(format!("negative {label}")))?;
    if value == 0 {
        return Err(CognitiveStoreError::Corrupt(format!("zero {label}")));
    }
    Ok(value)
}

fn nonnegative_u64(value: i64, label: &str) -> Result<u64, CognitiveStoreError> {
    u64::try_from(value).map_err(|_| CognitiveStoreError::Corrupt(format!("negative {label}")))
}

fn signed_delta(candidate: u64, current: u64) -> Result<i64, CognitiveStoreError> {
    let candidate = i128::from(candidate);
    let current = i128::from(current);
    i64::try_from(candidate - current)
        .map_err(|_| CognitiveStoreError::Corrupt("shadow projection delta overflow".to_string()))
}

fn limit_plus_one(value: usize) -> Result<i64, CognitiveStoreError> {
    value
        .checked_add(1)
        .and_then(|next| i64::try_from(next).ok())
        .ok_or_else(|| {
            CognitiveStoreError::Corrupt("shadow projection head limit exceeds i64".to_string())
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CognitiveAccess;
    use crate::FactEvidenceSpanDraft;
    use crate::GroundedFactKind;
    use crate::GroundedKgFactSetDraft;
    use crate::KgEntityFactDraft;
    use crate::KgFactSetDraft;
    use crate::LedgerSourceKind;
    use crate::MemoryDraft;
    use crate::MemoryLifecycleState;
    use crate::MemoryRevisionDraft;
    use crate::MemoryVerification;
    use crate::SourceDraft;
    use crate::cognitive_test_support::agent_id;
    use crate::cognitive_test_support::layout;
    use tempfile::TempDir;

    fn source(text: &str, key: &str) -> SourceDraft {
        SourceDraft {
            scope: CognitiveScope::AgentPrivate,
            kind: LedgerSourceKind::ExplicitMemoryDirective,
            event_key: key.to_string(),
            content: text.as_bytes().to_vec(),
            observed_at_unix_seconds: 100,
        }
    }

    fn memory(text: &str, key: &str) -> MemoryDraft {
        MemoryDraft {
            stable_key: key.to_string(),
            revision: MemoryRevisionDraft {
                scope: CognitiveScope::AgentPrivate,
                content: text.to_string(),
                verification: MemoryVerification::Verified,
                lifecycle: MemoryLifecycleState::Active,
                valid_from_unix_seconds: 100,
                valid_to_unix_seconds: None,
                citations: Vec::new(),
            },
        }
    }

    fn facts(label: &str) -> KgFactSetDraft {
        KgFactSetDraft {
            entities: vec![KgEntityFactDraft {
                key: label.to_ascii_lowercase(),
                entity_type: "project".to_string(),
                label: label.to_string(),
            }],
            relations: Vec::new(),
        }
    }

    fn grounded(text: &str, label: &str) -> GroundedKgFactSetDraft {
        let start = text.find(label).expect("label");
        let end = start + label.len();
        GroundedKgFactSetDraft {
            facts: facts(label),
            evidence: vec![
                FactEvidenceSpanDraft::from_source_text(
                    GroundedFactKind::Entity,
                    label.to_ascii_lowercase(),
                    text,
                    start,
                    end,
                )
                .expect("evidence"),
            ],
        }
    }

    #[tokio::test]
    async fn shadow_compare_excludes_legacy_without_changing_projection() {
        let temp = TempDir::new().expect("temp");
        let owner = agent_id(231);
        let store = CognitiveStore::open_with_durable_fact_grounding(&layout(&temp, &owner))
            .await
            .expect("store");
        let access = CognitiveAccess::agent_private(owner.clone());
        let grounded_text = "Project Aurora is active.";
        store
            .remember_with_durable_grounded_kg(
                &access,
                &source(grounded_text, "shadow:grounded"),
                &memory(grounded_text, "shadow-grounded"),
                &grounded(grounded_text, "Project Aurora"),
            )
            .await
            .expect("grounded write");
        let legacy_text = "Project Borealis is active.";
        store
            .remember_with_kg(
                &access,
                &source(legacy_text, "shadow:legacy"),
                &memory(legacy_text, "shadow-legacy"),
                &facts("Project Borealis"),
            )
            .await
            .expect("legacy write");
        let generation_before: i64 = sqlx::query_scalar(
            "SELECT generation FROM kg_projection WHERE projection_scope = 'agent_private'",
        )
        .fetch_one(&store.pool)
        .await
        .expect("generation");
        let receipt = store
            .shadow_grounded_projection_compare(&CognitiveScope::AgentPrivate)
            .await
            .expect("shadow compare");
        let value: serde_json::Value = serde_json::from_str(&receipt).expect("json");
        assert_eq!(value["grounded_candidate"]["node_count"], 1);
        assert_eq!(value["current"]["node_count"], 2);
        assert_eq!(value["excluded_heads"][0]["reason"], "legacy_unreviewed");
        assert_eq!(value["write_performed"], false);
        assert_eq!(value["production_projection_gate"], false);
        let generation_after: i64 = sqlx::query_scalar(
            "SELECT generation FROM kg_projection WHERE projection_scope = 'agent_private'",
        )
        .fetch_one(&store.pool)
        .await
        .expect("generation");
        assert_eq!(generation_before, generation_after);
    }

    #[tokio::test]
    async fn shadow_explain_returns_digests_and_not_source_content() {
        let temp = TempDir::new().expect("temp");
        let owner = agent_id(232);
        let store = CognitiveStore::open_with_durable_fact_grounding(&layout(&temp, &owner))
            .await
            .expect("store");
        let text = "Project Aurora is active.";
        let write = store
            .remember_with_durable_grounded_kg(
                &CognitiveAccess::agent_private(owner),
                &source(text, "shadow:explain"),
                &memory(text, "shadow-explain"),
                &grounded(text, "Project Aurora"),
            )
            .await
            .expect("write");
        let receipt = store
            .shadow_grounding_explain(&write.memory.id.memory_id, write.memory.id.revision)
            .await
            .expect("explain");
        let value: serde_json::Value = serde_json::from_str(&receipt).expect("json");
        assert_eq!(value["grounding_status"], "grounded_v1");
        assert!(value["grounding_receipt_sha256"].is_string());
        assert!(value["evidence"][0]["evidence_sha256"].is_string());
        assert!(!receipt.contains(text));
        assert_eq!(value["production_authority"], false);
    }
}
