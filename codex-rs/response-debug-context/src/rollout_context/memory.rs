use codex_protocol::protocol::TurnContextManifestItem;
use codex_protocol::protocol::TurnContextMemoryFormationReceipt;
use codex_protocol::protocol::TurnContextMemoryTaxonomyBucket;
use codex_protocol::protocol::TurnContextMemoryTemporalFact;

use super::RolloutContextDebugFinding;
use super::RolloutContextDebugFindingSeverity;
use super::RolloutContextDebugSummary;

pub(super) fn summarize_manifest_memory_taxonomy(
    manifest: &TurnContextManifestItem,
    summary: &mut RolloutContextDebugSummary,
    findings: &mut Vec<RolloutContextDebugFinding>,
) {
    summary.latest_manifest_memory_taxonomy_count =
        u32::try_from(manifest.memory_taxonomy.len()).unwrap_or(u32::MAX);
    summary.latest_manifest_memory_taxonomy_schema_version = manifest
        .memory_taxonomy
        .iter()
        .find_map(TurnContextMemoryTaxonomyBucket::schema_version)
        .unwrap_or(0);
    summary.latest_manifest_memory_taxonomy_classes = manifest
        .memory_taxonomy
        .iter()
        .filter_map(|bucket| {
            (!bucket.class.is_unknown()).then_some(bucket.class.as_str().to_string())
        })
        .fold(Vec::<String>::new(), |mut classes, class| {
            if !classes.contains(&class) {
                classes.push(class);
            }
            classes
        });
    summary.latest_manifest_memory_taxonomy_source_count = manifest
        .memory_taxonomy
        .iter()
        .fold(0_u32, |count, bucket| {
            count.saturating_add(bucket.source_count)
        });
    summary.latest_manifest_memory_taxonomy_returned_count = manifest
        .memory_taxonomy
        .iter()
        .fold(0_u32, |count, bucket| {
            count.saturating_add(bucket.returned_count)
        });
    summary.latest_manifest_memory_taxonomy_available_count = manifest
        .memory_taxonomy
        .iter()
        .fold(0_u32, |count, bucket| {
            count.saturating_add(bucket.available_count)
        });
    summary.latest_manifest_memory_taxonomy_omitted_count = manifest
        .memory_taxonomy
        .iter()
        .fold(0_u32, |count, bucket| {
            count.saturating_add(bucket.omitted_count)
        });
    summary.latest_manifest_memory_taxonomy_provenance_span_count = manifest
        .memory_taxonomy
        .iter()
        .fold(0_u32, |count, bucket| {
            count.saturating_add(bucket.provenance_span_count)
        });
    summary.latest_manifest_memory_taxonomy_invalid = !manifest.memory_taxonomy_has_integrity();

    if summary.latest_manifest_memory_taxonomy_invalid {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "manifest_memory_taxonomy_invalid",
            message: "context manifest memory taxonomy report contains inconsistent payload-light counts",
        });
    }
}

pub(super) fn summarize_manifest_memory_formation_receipts(
    manifest: &TurnContextManifestItem,
    summary: &mut RolloutContextDebugSummary,
    findings: &mut Vec<RolloutContextDebugFinding>,
) {
    summary.latest_manifest_memory_formation_receipt_count =
        u32::try_from(manifest.memory_formation_receipts.len()).unwrap_or(u32::MAX);
    summary.latest_manifest_memory_formation_receipt_schema_version = manifest
        .memory_formation_receipts
        .iter()
        .find_map(TurnContextMemoryFormationReceipt::schema_version)
        .unwrap_or(0);
    summary.latest_manifest_memory_formation_receipt_candidate_types = manifest
        .memory_formation_receipts
        .iter()
        .filter_map(|receipt| {
            (!receipt.candidate_type.is_unknown())
                .then_some(receipt.candidate_type.as_str().to_string())
        })
        .fold(Vec::<String>::new(), |mut types, candidate_type| {
            if !types.contains(&candidate_type) {
                types.push(candidate_type);
            }
            types
        });
    summary.latest_manifest_memory_formation_receipt_privacy_classes = manifest
        .memory_formation_receipts
        .iter()
        .map(|receipt| receipt.privacy_class.clone())
        .fold(Vec::<String>::new(), |mut classes, privacy_class| {
            if !classes.contains(&privacy_class) {
                classes.push(privacy_class);
            }
            classes
        });
    summary.latest_manifest_memory_formation_receipt_transcript_span_count = manifest
        .memory_formation_receipts
        .iter()
        .fold(0_u32, |count, receipt| {
            count.saturating_add(receipt.transcript_span_count)
        });
    summary.latest_manifest_memory_formation_receipt_provenance_span_count = manifest
        .memory_formation_receipts
        .iter()
        .fold(0_u32, |count, receipt| {
            count.saturating_add(receipt.provenance_span_count)
        });
    summary.latest_manifest_memory_formation_receipt_confidence_basis_points = manifest
        .memory_formation_receipts
        .iter()
        .fold(0_u32, |count, receipt| {
            count.saturating_add(receipt.confidence_basis_points)
        });
    summary.latest_manifest_memory_formation_receipt_queued_count = u32::try_from(
        manifest
            .memory_formation_receipts
            .iter()
            .filter(|receipt| receipt.queued_for_background)
            .count(),
    )
    .unwrap_or(u32::MAX);
    summary.latest_manifest_memory_formation_receipt_production_write_count = u32::try_from(
        manifest
            .memory_formation_receipts
            .iter()
            .filter(|receipt| receipt.production_write)
            .count(),
    )
    .unwrap_or(u32::MAX);
    summary.latest_manifest_memory_formation_receipt_invalid =
        !manifest.memory_formation_receipts_have_integrity();

    if summary.latest_manifest_memory_formation_receipt_invalid {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "manifest_memory_formation_receipts_invalid",
            message:
                "context manifest memory formation receipts are not background-only or payload-light",
        });
    }
}

pub(super) fn summarize_manifest_memory_temporal_facts(
    manifest: &TurnContextManifestItem,
    summary: &mut RolloutContextDebugSummary,
    findings: &mut Vec<RolloutContextDebugFinding>,
) {
    summary.latest_manifest_memory_temporal_fact_count =
        u32::try_from(manifest.memory_temporal_facts.len()).unwrap_or(u32::MAX);
    summary.latest_manifest_memory_temporal_fact_schema_version = manifest
        .memory_temporal_facts
        .iter()
        .find_map(TurnContextMemoryTemporalFact::schema_version)
        .unwrap_or(0);
    summary.latest_manifest_memory_temporal_fact_types = manifest
        .memory_temporal_facts
        .iter()
        .filter_map(|fact| {
            (!fact.fact_type.is_unknown()).then_some(fact.fact_type.as_str().to_string())
        })
        .fold(Vec::<String>::new(), |mut types, fact_type| {
            if !types.contains(&fact_type) {
                types.push(fact_type);
            }
            types
        });
    summary.latest_manifest_memory_temporal_fact_privacy_classes = manifest
        .memory_temporal_facts
        .iter()
        .map(|fact| fact.privacy_class.clone())
        .fold(Vec::<String>::new(), |mut classes, privacy_class| {
            if !classes.contains(&privacy_class) {
                classes.push(privacy_class);
            }
            classes
        });
    summary.latest_manifest_memory_temporal_fact_provenance_span_count = manifest
        .memory_temporal_facts
        .iter()
        .fold(0_u32, |count, fact| {
            count.saturating_add(fact.provenance_span_count)
        });
    summary.latest_manifest_memory_temporal_fact_confidence_basis_points = manifest
        .memory_temporal_facts
        .iter()
        .fold(0_u32, |count, fact| {
            count.saturating_add(fact.confidence_basis_points)
        });
    summary.latest_manifest_memory_temporal_fact_open_count = u32::try_from(
        manifest
            .memory_temporal_facts
            .iter()
            .filter(|fact| fact.invalid_at_sequence.is_none())
            .count(),
    )
    .unwrap_or(u32::MAX);
    summary.latest_manifest_memory_temporal_fact_invalidated_count = u32::try_from(
        manifest
            .memory_temporal_facts
            .iter()
            .filter(|fact| fact.invalid_at_sequence.is_some())
            .count(),
    )
    .unwrap_or(u32::MAX);
    summary.latest_manifest_memory_temporal_fact_supersedes_count = u32::try_from(
        manifest
            .memory_temporal_facts
            .iter()
            .filter(|fact| fact.supersedes_fact_hash.is_some())
            .count(),
    )
    .unwrap_or(u32::MAX);
    summary.latest_manifest_memory_temporal_fact_dry_run_count = u32::try_from(
        manifest
            .memory_temporal_facts
            .iter()
            .filter(|fact| fact.dry_run_only)
            .count(),
    )
    .unwrap_or(u32::MAX);
    summary.latest_manifest_memory_temporal_fact_production_write_count = u32::try_from(
        manifest
            .memory_temporal_facts
            .iter()
            .filter(|fact| fact.production_write)
            .count(),
    )
    .unwrap_or(u32::MAX);
    summary.latest_manifest_memory_temporal_fact_invalid =
        !manifest.memory_temporal_facts_have_integrity();

    if summary.latest_manifest_memory_temporal_fact_invalid {
        findings.push(RolloutContextDebugFinding {
            severity: RolloutContextDebugFindingSeverity::Error,
            code: "manifest_memory_temporal_facts_invalid",
            message: "context manifest memory temporal facts are not dry-run-only or payload-light",
        });
    }
}
