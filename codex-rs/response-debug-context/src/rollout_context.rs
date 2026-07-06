use codex_protocol::protocol::TurnContextAdaptiveBudgetAllocation;
use codex_protocol::protocol::TurnContextCompressionCandidate;
use codex_protocol::protocol::TurnContextDecisionKind;
use codex_protocol::protocol::TurnContextManifestItem;
use serde::Serialize;
use serde_json::Value;

mod memory;

use memory::summarize_manifest_memory_formation_receipts;
use memory::summarize_manifest_memory_taxonomy;
use memory::summarize_manifest_memory_temporal_facts;

pub const ROLLOUT_CONTEXT_DEBUG_EXPORT_VERSION: u32 = 1;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub struct RolloutContextDebugSummary {
    pub line_count: u32,
    pub parse_error_count: u32,
    pub manifest_count: u32,
    pub latest_manifest_present: bool,
    pub latest_manifest_version: u32,
    pub latest_manifest_estimated_tokens: u32,
    pub latest_manifest_budget_tokens: Option<u32>,
    pub latest_manifest_budget_exceeded: bool,
    pub latest_manifest_omitted_entries: u32,
    pub latest_manifest_truncated: bool,
    pub latest_manifest_decision_schema_version: u32,
    pub latest_manifest_decision_known_count: u32,
    pub latest_manifest_decision_unknown_count: u32,
    pub latest_manifest_decision_included_count: u32,
    pub latest_manifest_decision_policy_count: u32,
    pub latest_manifest_decision_candidate_omit_count: u32,
    pub latest_manifest_decision_candidate_truncate_count: u32,
    pub latest_manifest_decision_omitted_count: u32,
    pub latest_manifest_decision_truncated_count: u32,
    pub latest_manifest_compression_candidate_schema_version: u32,
    pub latest_manifest_compression_candidate_count: u32,
    pub latest_manifest_compression_candidate_stages: Vec<String>,
    pub latest_manifest_compression_candidate_tiers: Vec<String>,
    pub latest_manifest_compression_candidate_sources: Vec<String>,
    pub latest_manifest_compression_candidate_reasons: Vec<String>,
    pub latest_manifest_compression_candidate_input_tokens: u32,
    pub latest_manifest_compression_candidate_output_tokens: u32,
    pub latest_manifest_compression_candidate_tokens_saved: u32,
    pub latest_manifest_compression_candidate_affected_entries: u32,
    pub latest_manifest_compression_candidate_invalid: bool,
    pub latest_manifest_adaptive_budget_allocation_schema_version: u32,
    pub latest_manifest_adaptive_budget_allocation_count: u32,
    pub latest_manifest_adaptive_budget_allocation_sources: Vec<String>,
    pub latest_manifest_adaptive_budget_allocation_budget_classes: Vec<String>,
    pub latest_manifest_adaptive_budget_allocation_current_actions: Vec<String>,
    pub latest_manifest_adaptive_budget_allocation_proposed_actions: Vec<String>,
    pub latest_manifest_adaptive_budget_allocation_input_tokens: u32,
    pub latest_manifest_adaptive_budget_allocation_reserve_tokens: u32,
    pub latest_manifest_adaptive_budget_allocation_proposed_budget_tokens: u32,
    pub latest_manifest_adaptive_budget_allocation_overflow_tokens: u32,
    pub latest_manifest_adaptive_budget_allocation_would_drop_count: u32,
    pub latest_manifest_adaptive_budget_allocation_would_compress_count: u32,
    pub latest_manifest_adaptive_budget_allocation_invalid: bool,
    pub latest_manifest_compression_stage_schema_version: u32,
    pub latest_manifest_compression_stage_count: u32,
    pub latest_manifest_compression_stages: Vec<String>,
    pub latest_manifest_compression_loss_check_statuses: Vec<String>,
    pub latest_manifest_compression_rollback_source_text_hash_count: u32,
    pub latest_manifest_compression_protected_tier_invariants: Vec<String>,
    pub latest_manifest_compression_input_tokens: u32,
    pub latest_manifest_compression_output_tokens: u32,
    pub latest_manifest_compression_tokens_saved: u32,
    pub latest_manifest_compression_affected_entries: u32,
    pub latest_manifest_compression_invalid: bool,
    pub latest_manifest_truncated_decision_count: u32,
    pub latest_manifest_truncated_sources: Vec<String>,
    pub latest_manifest_truncation_evidence_present: bool,
    pub latest_manifest_truncation_evidence_invalid: bool,
    pub latest_manifest_tiers: Vec<String>,
    pub latest_manifest_sources: Vec<String>,
    pub latest_manifest_recall_selection_present: bool,
    pub latest_manifest_recall_selection_invalid: bool,
    pub latest_manifest_recall_returned_source_count: u32,
    pub latest_manifest_recall_selected_source_count: u32,
    pub latest_manifest_recall_ranked_source_count: u32,
    pub latest_manifest_recall_returned_unselected_source_count: u32,
    pub latest_manifest_recall_source_diversity_met: bool,
    pub latest_manifest_recall_source_diversity_target: u32,
    pub latest_manifest_recall_max_per_source: u32,
    pub latest_manifest_recall_ranked_item_count: u32,
    pub latest_manifest_recall_omitted_by_budget_count: u32,
    pub latest_manifest_recall_memory_control_omitted_count: u32,
    pub latest_manifest_recall_low_trust_ranked_item_count: u32,
    pub latest_manifest_recall_low_recency_ranked_item_count: u32,
    pub latest_manifest_recall_selected_snippets_present: bool,
    pub latest_manifest_recall_selected_snippets_invalid: bool,
    pub latest_manifest_recall_selected_snippet_count: u32,
    pub latest_manifest_recall_selected_snippet_omitted_count: u32,
    pub latest_manifest_recall_selected_snippet_redacted_count: u32,
    pub latest_manifest_recall_selected_snippet_truncated_count: u32,
    pub latest_manifest_recall_selected_snippet_max_snippets: u32,
    pub latest_manifest_recall_selected_snippet_max_chars: u32,
    pub latest_manifest_recall_selected_snippet_ready: bool,
    pub latest_manifest_recall_selected_snippet_bounded: bool,
    pub latest_manifest_memory_taxonomy_schema_version: u32,
    pub latest_manifest_memory_taxonomy_count: u32,
    pub latest_manifest_memory_taxonomy_classes: Vec<String>,
    pub latest_manifest_memory_taxonomy_source_count: u32,
    pub latest_manifest_memory_taxonomy_returned_count: u32,
    pub latest_manifest_memory_taxonomy_available_count: u32,
    pub latest_manifest_memory_taxonomy_omitted_count: u32,
    pub latest_manifest_memory_taxonomy_provenance_span_count: u32,
    pub latest_manifest_memory_taxonomy_invalid: bool,
    pub latest_manifest_memory_formation_receipt_schema_version: u32,
    pub latest_manifest_memory_formation_receipt_count: u32,
    pub latest_manifest_memory_formation_receipt_candidate_types: Vec<String>,
    pub latest_manifest_memory_formation_receipt_privacy_classes: Vec<String>,
    pub latest_manifest_memory_formation_receipt_transcript_span_count: u32,
    pub latest_manifest_memory_formation_receipt_provenance_span_count: u32,
    pub latest_manifest_memory_formation_receipt_confidence_basis_points: u32,
    pub latest_manifest_memory_formation_receipt_queued_count: u32,
    pub latest_manifest_memory_formation_receipt_production_write_count: u32,
    pub latest_manifest_memory_formation_receipt_invalid: bool,
    pub latest_manifest_memory_temporal_fact_schema_version: u32,
    pub latest_manifest_memory_temporal_fact_count: u32,
    pub latest_manifest_memory_temporal_fact_types: Vec<String>,
    pub latest_manifest_memory_temporal_fact_privacy_classes: Vec<String>,
    pub latest_manifest_memory_temporal_fact_provenance_span_count: u32,
    pub latest_manifest_memory_temporal_fact_confidence_basis_points: u32,
    pub latest_manifest_memory_temporal_fact_open_count: u32,
    pub latest_manifest_memory_temporal_fact_invalidated_count: u32,
    pub latest_manifest_memory_temporal_fact_supersedes_count: u32,
    pub latest_manifest_memory_temporal_fact_dry_run_count: u32,
    pub latest_manifest_memory_temporal_fact_production_write_count: u32,
    pub latest_manifest_memory_temporal_fact_invalid: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RolloutContextDebugAudit {
    pub ok: bool,
    pub findings: Vec<RolloutContextDebugFinding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RolloutContextDebugFinding {
    pub severity: RolloutContextDebugFindingSeverity,
    pub code: &'static str,
    pub message: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RolloutContextDebugFindingSeverity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RolloutContextDebugExport {
    pub version: u32,
    pub summary: RolloutContextDebugSummary,
    pub audit: RolloutContextDebugAudit,
}

pub fn summarize_rollout_context_debug_jsonl(input: &str) -> RolloutContextDebugExport {
    let mut summary = RolloutContextDebugSummary::default();
    let mut findings = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        summary.line_count = summary.line_count.saturating_add(1);
        let Ok(value) = serde_json::from_str::<Value>(line) else {
            summary.parse_error_count = summary.parse_error_count.saturating_add(1);
            findings.push(RolloutContextDebugFinding {
                severity: RolloutContextDebugFindingSeverity::Error,
                code: "jsonl_parse_error",
                message: "rollout JSONL contains a line that could not be parsed",
            });
            continue;
        };

        let Some(manifest_value) = extract_manifest_value(&value) else {
            continue;
        };
        summary.manifest_count = summary.manifest_count.saturating_add(1);
        match serde_json::from_value::<TurnContextManifestItem>(manifest_value.clone()) {
            Ok(manifest) => summarize_manifest(&manifest, &mut summary, &mut findings),
            Err(_) => findings.push(RolloutContextDebugFinding {
                severity: RolloutContextDebugFindingSeverity::Error,
                code: "manifest_parse_error",
                message: "context manifest could not be parsed",
            }),
        }
    }

    if summary.line_count > 0 && summary.manifest_count == 0 {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Warning,
            code: "missing_context_manifest",
            message: "rollout JSONL contained no context manifest",
        });
    }

    let ok = findings
        .iter()
        .all(|finding| finding.severity != RolloutContextDebugFindingSeverity::Error);

    RolloutContextDebugExport {
        version: ROLLOUT_CONTEXT_DEBUG_EXPORT_VERSION,
        summary,
        audit: RolloutContextDebugAudit { ok, findings },
    }
}

pub fn rollout_context_debug_export_json(input: &str) -> serde_json::Result<String> {
    serde_json::to_string_pretty(&summarize_rollout_context_debug_jsonl(input))
}

fn extract_manifest_value(value: &Value) -> Option<&Value> {
    let item_type = value.get("type").and_then(Value::as_str)?;
    let payload = value.get("payload")?;
    match item_type {
        "turn_context_manifest" => Some(payload),
        "turn_context" => payload.get("context_manifest"),
        _ => None,
    }
}

fn summarize_manifest(
    manifest: &TurnContextManifestItem,
    summary: &mut RolloutContextDebugSummary,
    findings: &mut Vec<RolloutContextDebugFinding>,
) {
    summary.latest_manifest_present = true;
    summary.latest_manifest_version = manifest.version;
    summary.latest_manifest_estimated_tokens = manifest.estimated_tokens;
    summary.latest_manifest_budget_tokens = manifest.budget_tokens;
    summary.latest_manifest_budget_exceeded = manifest
        .budget_tokens
        .is_some_and(|budget| manifest.estimated_tokens > budget);
    summary.latest_manifest_omitted_entries = manifest.omitted_entries;
    summary.latest_manifest_truncated = manifest.truncated;
    summary.latest_manifest_sources = manifest
        .entries
        .iter()
        .map(|entry| entry.source.clone())
        .collect();
    summary.latest_manifest_tiers = manifest
        .entries
        .iter()
        .filter_map(|entry| (!entry.tier.is_unknown()).then_some(entry.tier.as_str().to_string()))
        .fold(Vec::<String>::new(), |mut tiers, tier| {
            if !tiers.contains(&tier) {
                tiers.push(tier);
            }
            tiers
        });
    summarize_manifest_decision_schema(manifest, summary);
    summarize_manifest_truncation(manifest, summary, findings);

    if let Some(recall_selection) = &manifest.recall_selection {
        summary.latest_manifest_recall_selection_present = true;
        summary.latest_manifest_recall_selection_invalid = !recall_selection.has_count_integrity();
        summary.latest_manifest_recall_returned_source_count =
            recall_selection.returned_source_count;
        summary.latest_manifest_recall_selected_source_count =
            recall_selection.selected_source_count;
        summary.latest_manifest_recall_ranked_source_count = recall_selection.ranked_source_count;
        summary.latest_manifest_recall_returned_unselected_source_count =
            recall_selection.returned_unselected_source_count;
        summary.latest_manifest_recall_source_diversity_met = recall_selection.source_diversity_met;
        summary.latest_manifest_recall_source_diversity_target =
            recall_selection.source_diversity_target;
        summary.latest_manifest_recall_max_per_source = recall_selection.max_per_source;
        summary.latest_manifest_recall_ranked_item_count = recall_selection.ranked_item_count;
        summary.latest_manifest_recall_omitted_by_budget_count =
            recall_selection.omitted_by_budget_count;
        summary.latest_manifest_recall_memory_control_omitted_count =
            recall_selection.memory_control_omitted_count;
        summary.latest_manifest_recall_low_trust_ranked_item_count =
            recall_selection.low_trust_ranked_item_count;
        summary.latest_manifest_recall_low_recency_ranked_item_count =
            recall_selection.low_recency_ranked_item_count;
    } else {
        summary.latest_manifest_recall_selection_present = false;
        summary.latest_manifest_recall_selection_invalid = false;
        summary.latest_manifest_recall_returned_source_count = 0;
        summary.latest_manifest_recall_selected_source_count = 0;
        summary.latest_manifest_recall_ranked_source_count = 0;
        summary.latest_manifest_recall_returned_unselected_source_count = 0;
        summary.latest_manifest_recall_source_diversity_met = false;
        summary.latest_manifest_recall_source_diversity_target = 0;
        summary.latest_manifest_recall_max_per_source = 0;
        summary.latest_manifest_recall_ranked_item_count = 0;
        summary.latest_manifest_recall_omitted_by_budget_count = 0;
        summary.latest_manifest_recall_memory_control_omitted_count = 0;
        summary.latest_manifest_recall_low_trust_ranked_item_count = 0;
        summary.latest_manifest_recall_low_recency_ranked_item_count = 0;
    }

    if let Some(selected_snippets) = &manifest.recall_selected_snippets {
        summary.latest_manifest_recall_selected_snippets_present = true;
        summary.latest_manifest_recall_selected_snippets_invalid =
            !selected_snippets.has_shadow_integrity();
        summary.latest_manifest_recall_selected_snippet_count =
            selected_snippets.selected_snippet_count;
        summary.latest_manifest_recall_selected_snippet_omitted_count =
            selected_snippets.omitted_snippet_count;
        summary.latest_manifest_recall_selected_snippet_redacted_count =
            selected_snippets.redacted_snippet_count;
        summary.latest_manifest_recall_selected_snippet_truncated_count =
            selected_snippets.truncated_snippet_count;
        summary.latest_manifest_recall_selected_snippet_max_snippets =
            selected_snippets.max_snippets;
        summary.latest_manifest_recall_selected_snippet_max_chars =
            selected_snippets.max_snippet_chars;
        summary.latest_manifest_recall_selected_snippet_ready =
            selected_snippets.safety.ready_for_shadow_handoff;
        summary.latest_manifest_recall_selected_snippet_bounded = selected_snippets.safety.bounded;
    } else {
        summary.latest_manifest_recall_selected_snippets_present = false;
        summary.latest_manifest_recall_selected_snippets_invalid = false;
        summary.latest_manifest_recall_selected_snippet_count = 0;
        summary.latest_manifest_recall_selected_snippet_omitted_count = 0;
        summary.latest_manifest_recall_selected_snippet_redacted_count = 0;
        summary.latest_manifest_recall_selected_snippet_truncated_count = 0;
        summary.latest_manifest_recall_selected_snippet_max_snippets = 0;
        summary.latest_manifest_recall_selected_snippet_max_chars = 0;
        summary.latest_manifest_recall_selected_snippet_ready = false;
        summary.latest_manifest_recall_selected_snippet_bounded = false;
    }

    if !manifest.has_supported_version() {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "unsupported_manifest_version",
            message: "context manifest version is unsupported",
        });
    }
    if !manifest.entries_have_replay_integrity() {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "manifest_entries_invalid",
            message: "context manifest entries do not satisfy replay identity requirements",
        });
    }
    if !manifest.ledger_hash_is_compatible() {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "manifest_ledger_hash_mismatch",
            message: "context manifest ledger hash does not match payload-light fields",
        });
    }
    if !manifest.decision_ledger_has_integrity() || !manifest.decision_ledger_hash_is_compatible() {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "manifest_decision_ledger_invalid",
            message: "context manifest decision ledger does not satisfy hash requirements",
        });
    }
    if !manifest.recall_selection_has_integrity() {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "manifest_recall_selection_invalid",
            message: "context manifest recall selection rollup contains inconsistent payload-light counts",
        });
    }
    if !manifest.recall_selected_snippets_have_integrity() {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "manifest_recall_selected_snippets_invalid",
            message:
                "context manifest recall selected-snippet envelope is not bounded or source-safe",
        });
    }
    summarize_manifest_memory_taxonomy(manifest, summary, findings);
    summarize_manifest_memory_formation_receipts(manifest, summary, findings);
    summarize_manifest_memory_temporal_facts(manifest, summary, findings);
    summarize_manifest_compression_candidates(manifest, summary, findings);
    summarize_manifest_adaptive_budget_allocations(manifest, summary, findings);
    summarize_manifest_compression(manifest, summary, findings);
}

fn summarize_manifest_decision_schema(
    manifest: &TurnContextManifestItem,
    summary: &mut RolloutContextDebugSummary,
) {
    let decision_summary = manifest.decision_ledger_summary();
    summary.latest_manifest_decision_schema_version = decision_summary.schema_version;
    summary.latest_manifest_decision_known_count = decision_summary.known_count();
    summary.latest_manifest_decision_unknown_count = decision_summary.unknown_count;
    summary.latest_manifest_decision_included_count = decision_summary.included_count;
    summary.latest_manifest_decision_policy_count = decision_summary.policy_count;
    summary.latest_manifest_decision_candidate_omit_count = decision_summary.candidate_omit_count;
    summary.latest_manifest_decision_candidate_truncate_count =
        decision_summary.candidate_truncate_count;
    summary.latest_manifest_decision_omitted_count = decision_summary.omitted_count;
    summary.latest_manifest_decision_truncated_count = decision_summary.truncated_count;
}

fn summarize_manifest_compression(
    manifest: &TurnContextManifestItem,
    summary: &mut RolloutContextDebugSummary,
    findings: &mut Vec<RolloutContextDebugFinding>,
) {
    summary.latest_manifest_compression_stage_count =
        u32::try_from(manifest.compression_stages.len()).unwrap_or(u32::MAX);
    summary.latest_manifest_compression_stage_schema_version = manifest
        .compression_stages
        .iter()
        .find_map(|stage| stage.kind.schema_version())
        .unwrap_or(0);
    summary.latest_manifest_compression_stages = manifest
        .compression_stages
        .iter()
        .filter_map(|stage| (!stage.kind.is_unknown()).then_some(stage.kind.as_str().to_string()))
        .fold(Vec::<String>::new(), |mut stages, stage| {
            if !stages.contains(&stage) {
                stages.push(stage);
            }
            stages
        });
    summary.latest_manifest_compression_loss_check_statuses = manifest
        .compression_stages
        .iter()
        .filter_map(|stage| {
            stage
                .loss_check_status
                .filter(|status| !status.is_unknown())
                .map(|status| status.as_str().to_string())
        })
        .fold(Vec::<String>::new(), |mut statuses, status| {
            if !statuses.contains(&status) {
                statuses.push(status);
            }
            statuses
        });
    summary.latest_manifest_compression_rollback_source_text_hash_count = manifest
        .compression_stages
        .iter()
        .filter(|stage| stage.rollback_source_text_hash.is_some())
        .count()
        .try_into()
        .unwrap_or(u32::MAX);
    summary.latest_manifest_compression_protected_tier_invariants = manifest
        .compression_stages
        .iter()
        .filter_map(|stage| {
            stage
                .protected_tier_invariant
                .filter(|invariant| !invariant.is_unknown())
                .map(|invariant| invariant.as_str().to_string())
        })
        .fold(Vec::<String>::new(), |mut invariants, invariant| {
            if !invariants.contains(&invariant) {
                invariants.push(invariant);
            }
            invariants
        });
    summary.latest_manifest_compression_input_tokens = manifest
        .compression_stages
        .iter()
        .fold(0_u32, |tokens, stage| {
            tokens.saturating_add(stage.input_tokens)
        });
    summary.latest_manifest_compression_output_tokens = manifest
        .compression_stages
        .iter()
        .fold(0_u32, |tokens, stage| {
            tokens.saturating_add(stage.output_tokens)
        });
    summary.latest_manifest_compression_tokens_saved = manifest
        .compression_stages
        .iter()
        .fold(0_u32, |tokens, stage| {
            tokens.saturating_add(stage.tokens_saved())
        });
    summary.latest_manifest_compression_affected_entries = manifest
        .compression_stages
        .iter()
        .fold(0_u32, |entries, stage| {
            entries.saturating_add(stage.affected_entries)
        });
    summary.latest_manifest_compression_invalid = !manifest.compression_stages_have_integrity();

    if summary.latest_manifest_compression_invalid {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "manifest_compression_stages_invalid",
            message: "context manifest compression stages are not payload-light or token-safe",
        });
    }
}

fn summarize_manifest_compression_candidates(
    manifest: &TurnContextManifestItem,
    summary: &mut RolloutContextDebugSummary,
    findings: &mut Vec<RolloutContextDebugFinding>,
) {
    summary.latest_manifest_compression_candidate_count =
        u32::try_from(manifest.compression_candidates.len()).unwrap_or(u32::MAX);
    summary.latest_manifest_compression_candidate_schema_version = manifest
        .compression_candidates
        .iter()
        .find_map(TurnContextCompressionCandidate::schema_version)
        .unwrap_or(0);
    summary.latest_manifest_compression_candidate_stages = manifest
        .compression_candidates
        .iter()
        .filter_map(|candidate| {
            (!candidate.kind.is_unknown()).then_some(candidate.kind.as_str().to_string())
        })
        .fold(Vec::<String>::new(), |mut stages, stage| {
            if !stages.contains(&stage) {
                stages.push(stage);
            }
            stages
        });
    summary.latest_manifest_compression_candidate_tiers = manifest
        .compression_candidates
        .iter()
        .filter_map(|candidate| {
            (!candidate.tier.is_unknown()).then_some(candidate.tier.as_str().to_string())
        })
        .fold(Vec::<String>::new(), |mut tiers, tier| {
            if !tiers.contains(&tier) {
                tiers.push(tier);
            }
            tiers
        });
    summary.latest_manifest_compression_candidate_sources = manifest
        .compression_candidates
        .iter()
        .map(|candidate| candidate.source_id.clone())
        .fold(Vec::<String>::new(), |mut sources, source| {
            if !sources.contains(&source) {
                sources.push(source);
            }
            sources
        });
    summary.latest_manifest_compression_candidate_reasons = manifest
        .compression_candidates
        .iter()
        .filter_map(|candidate| {
            (!candidate.not_executed_reason.is_unknown())
                .then_some(candidate.not_executed_reason.as_str().to_string())
        })
        .fold(Vec::<String>::new(), |mut reasons, reason| {
            if !reasons.contains(&reason) {
                reasons.push(reason);
            }
            reasons
        });
    summary.latest_manifest_compression_candidate_input_tokens = manifest
        .compression_candidates
        .iter()
        .fold(0_u32, |tokens, candidate| {
            tokens.saturating_add(candidate.input_tokens)
        });
    summary.latest_manifest_compression_candidate_output_tokens = manifest
        .compression_candidates
        .iter()
        .fold(0_u32, |tokens, candidate| {
            tokens.saturating_add(candidate.estimated_output_tokens)
        });
    summary.latest_manifest_compression_candidate_tokens_saved = manifest
        .compression_candidates
        .iter()
        .fold(0_u32, |tokens, candidate| {
            tokens.saturating_add(candidate.estimated_tokens_saved())
        });
    summary.latest_manifest_compression_candidate_affected_entries = manifest
        .compression_candidates
        .iter()
        .fold(0_u32, |entries, candidate| {
            entries.saturating_add(candidate.affected_entries)
        });
    summary.latest_manifest_compression_candidate_invalid =
        !manifest.compression_candidates_have_integrity();

    if summary.latest_manifest_compression_candidate_invalid {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "manifest_compression_candidates_invalid",
            message: "context manifest compression candidates are not payload-light or token-safe",
        });
    }
}

fn summarize_manifest_adaptive_budget_allocations(
    manifest: &TurnContextManifestItem,
    summary: &mut RolloutContextDebugSummary,
    findings: &mut Vec<RolloutContextDebugFinding>,
) {
    summary.latest_manifest_adaptive_budget_allocation_count =
        u32::try_from(manifest.adaptive_budget_allocations.len()).unwrap_or(u32::MAX);
    summary.latest_manifest_adaptive_budget_allocation_schema_version = manifest
        .adaptive_budget_allocations
        .iter()
        .find_map(TurnContextAdaptiveBudgetAllocation::schema_version)
        .unwrap_or(0);
    summary.latest_manifest_adaptive_budget_allocation_sources = manifest
        .adaptive_budget_allocations
        .iter()
        .map(|allocation| allocation.source_id.clone())
        .fold(Vec::<String>::new(), |mut sources, source| {
            if !sources.contains(&source) {
                sources.push(source);
            }
            sources
        });
    summary.latest_manifest_adaptive_budget_allocation_budget_classes = manifest
        .adaptive_budget_allocations
        .iter()
        .map(|allocation| allocation.budget_class.clone())
        .fold(Vec::<String>::new(), |mut classes, budget_class| {
            if !classes.contains(&budget_class) {
                classes.push(budget_class);
            }
            classes
        });
    summary.latest_manifest_adaptive_budget_allocation_current_actions = manifest
        .adaptive_budget_allocations
        .iter()
        .map(|allocation| allocation.current_heuristic_action.as_str().to_string())
        .fold(Vec::<String>::new(), |mut actions, action| {
            if !actions.contains(&action) {
                actions.push(action);
            }
            actions
        });
    summary.latest_manifest_adaptive_budget_allocation_proposed_actions = manifest
        .adaptive_budget_allocations
        .iter()
        .map(|allocation| allocation.proposed_action.as_str().to_string())
        .fold(Vec::<String>::new(), |mut actions, action| {
            if !actions.contains(&action) {
                actions.push(action);
            }
            actions
        });
    summary.latest_manifest_adaptive_budget_allocation_input_tokens = manifest
        .adaptive_budget_allocations
        .iter()
        .fold(0_u32, |tokens, allocation| {
            tokens.saturating_add(allocation.input_tokens)
        });
    summary.latest_manifest_adaptive_budget_allocation_reserve_tokens = manifest
        .adaptive_budget_allocations
        .iter()
        .fold(0_u32, |tokens, allocation| {
            tokens.saturating_add(allocation.reserve_tokens)
        });
    summary.latest_manifest_adaptive_budget_allocation_proposed_budget_tokens = manifest
        .adaptive_budget_allocations
        .iter()
        .fold(0_u32, |tokens, allocation| {
            tokens.saturating_add(allocation.proposed_budget_tokens)
        });
    summary.latest_manifest_adaptive_budget_allocation_overflow_tokens = manifest
        .adaptive_budget_allocations
        .iter()
        .fold(0_u32, |tokens, allocation| {
            tokens.saturating_add(allocation.overflow_tokens)
        });
    summary.latest_manifest_adaptive_budget_allocation_would_drop_count = manifest
        .adaptive_budget_allocations
        .iter()
        .filter(|allocation| allocation.would_drop)
        .count()
        .try_into()
        .unwrap_or(u32::MAX);
    summary.latest_manifest_adaptive_budget_allocation_would_compress_count = manifest
        .adaptive_budget_allocations
        .iter()
        .filter(|allocation| allocation.would_compress)
        .count()
        .try_into()
        .unwrap_or(u32::MAX);
    summary.latest_manifest_adaptive_budget_allocation_invalid =
        !manifest.adaptive_budget_allocations_have_integrity();

    if summary.latest_manifest_adaptive_budget_allocation_invalid {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "manifest_adaptive_budget_allocations_invalid",
            message:
                "context manifest adaptive budget allocations are not payload-light or token-safe",
        });
    }
}

fn summarize_manifest_truncation(
    manifest: &TurnContextManifestItem,
    summary: &mut RolloutContextDebugSummary,
    findings: &mut Vec<RolloutContextDebugFinding>,
) {
    let mut truncated_sources = Vec::new();
    let mut truncated_decision_count = 0_u32;
    let mut truncation_evidence_invalid = false;

    for decision in &manifest.decision_ledger {
        let decision_kind = decision.kind();
        if !decision_kind.is_truncation() {
            continue;
        }
        truncated_decision_count = truncated_decision_count.saturating_add(1);
        if !truncated_sources.contains(&decision.source) {
            truncated_sources.push(decision.source.clone());
        }
        if !truncation_decision_is_manifest_safe(manifest, &decision.source, &decision_kind) {
            truncation_evidence_invalid = true;
        }
    }

    summary.latest_manifest_truncated_decision_count = truncated_decision_count;
    summary.latest_manifest_truncated_sources = truncated_sources;
    summary.latest_manifest_truncation_evidence_present = truncated_decision_count > 0;
    summary.latest_manifest_truncation_evidence_invalid = truncation_evidence_invalid;

    if manifest.truncated && truncated_decision_count == 0 {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "manifest_truncation_evidence_missing",
            message: "context manifest claims truncation without truncated decision evidence",
        });
    }
    if !manifest.truncated && truncated_decision_count > 0 {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "manifest_truncation_evidence_unexpected",
            message: "context manifest carries truncated decision evidence without truncated=true",
        });
    }
    if truncation_evidence_invalid {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "manifest_truncation_evidence_invalid",
            message:
                "context manifest truncated decision evidence is malformed or source-disconnected",
        });
    }
}

fn truncation_decision_is_manifest_safe(
    manifest: &TurnContextManifestItem,
    source: &str,
    decision_kind: &TurnContextDecisionKind,
) -> bool {
    if !manifest.entries.iter().any(|entry| entry.source == source) {
        return false;
    }

    matches!(
        decision_kind,
        TurnContextDecisionKind::Truncated {
            source_id,
            original_tokens,
            tokens,
        } if !source_id.is_empty() && tokens < original_tokens
    )
}
