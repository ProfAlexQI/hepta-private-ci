#!/usr/bin/env python3
from __future__ import annotations

import json
from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    target = Path(path)
    text = target.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one replacement target, found {count}")
    target.write_text(text.replace(old, new, 1), encoding="utf-8")


def patch_evaluation() -> None:
    path = "codex-rs/hepta-memory-p1-1c2-qualification/src/evaluation.rs"
    replace_once(
        path,
        """    P1_1C2_SCHEMA_VERSION, P1_1C2_SOURCE_QUALIFIED, ProjectionAudit, ReviewProjection,
    framed_digest, p1c1_digest, p1c_digest, validate_commit_oid,
""",
        """    P1_1C2_SCHEMA_VERSION, P1_1C2_SOURCE_QUALIFIED, ReviewProjection, framed_digest,
    p1c1_digest, p1c_digest, validate_commit_oid,
""",
    )
    replace_once(
        path,
        """    AblationLane, CalibrationContract, CandidateFeatures, CaseMetrics, CorpusProvenance,
    EvaluationReceipt, KgEdge, KgGraph, LaneMetrics, OfflineCorpus, PPM_DENOMINATOR,
""",
        """    AblationLane, CalibrationContract, CandidateFeatures, CandidateFixture, CaseMetrics,
    CorpusProvenance, EvaluationReceipt, KgEdge, KgGraph, LaneMetrics, OfflineCorpus,
    PPM_DENOMINATOR,
""",
    )
    replace_once(
        path,
        """    AcceptancePolicy, AcceptanceReceipt, DependencyState, ReviewBatch, evaluate_review_batch,
""",
        """    AcceptancePolicy, AcceptanceReceipt, CitationLabel, ContradictionLabel, DependencyState,
    PrivacyDecision, ReviewBatch, evaluate_review_batch,
""",
    )
    replace_once(
        path,
        "use std::collections::BTreeSet;\n",
        "use std::collections::{BTreeMap, BTreeSet};\n",
    )
    replace_once(
        path,
        """    pub reviewed_corpus_present: bool,
    pub projection_complete: bool,
    pub reviewed_corpus_evaluated: bool,
""",
        """    pub reviewed_corpus_present: bool,
    pub projection_complete: bool,
    pub final_label_bindings_match: bool,
    pub reviewed_corpus_evaluated: bool,
""",
    )
    replace_once(
        path,
        """                || !self.projection_complete
                || !self.blocked_reasons.is_empty()
""",
        """                || !self.projection_complete
                || !self.final_label_bindings_match
                || !self.blocked_reasons.is_empty()
""",
    )
    replace_once(
        path,
        """        if self.efficacy_validation
            != (self.reviewed_corpus_evaluated && self.efficacy_thresholds_passed)
""",
        """        if self.final_label_bindings_match && !self.projection_complete {
            return Err(ContractError::Corrupt(
                "final review labels cannot match an incomplete projection".to_string(),
            ));
        }
        if self.efficacy_validation
            != (self.reviewed_corpus_evaluated && self.efficacy_thresholds_passed)
""",
    )
    replace_once(
        path,
        """                "  \\\"projection_complete\\\": {},\\n",
                "  \\\"reviewed_corpus_evaluated\\\": {},\\n",
""",
        """                "  \\\"projection_complete\\\": {},\\n",
                "  \\\"final_label_bindings_match\\\": {},\\n",
                "  \\\"reviewed_corpus_evaluated\\\": {},\\n",
""",
    )
    replace_once(
        path,
        """            self.reviewed_corpus_present,
            self.projection_complete,
            self.reviewed_corpus_evaluated,
""",
        """            self.reviewed_corpus_present,
            self.projection_complete,
            self.final_label_bindings_match,
            self.reviewed_corpus_evaluated,
""",
    )
    replace_once(
        path,
        """                &[u8::from(self.reviewed_corpus_present)],
                &[u8::from(self.projection_complete)],
                &[u8::from(self.reviewed_corpus_evaluated)],
""",
        """                &[u8::from(self.reviewed_corpus_present)],
                &[u8::from(self.projection_complete)],
                &[u8::from(self.final_label_bindings_match)],
                &[u8::from(self.reviewed_corpus_evaluated)],
""",
    )
    replace_once(
        path,
        """    let projection_audit = request
        .projection
        .audit(request.review_batch, request.reviewed_corpus)?;
    let mut blockers = BTreeSet::new();

""",
        """    let projection_audit = request
        .projection
        .audit(request.review_batch, request.reviewed_corpus)?;
    let final_label_blockers = final_label_binding_blockers(
        request.acceptance_receipt,
        request.projection,
        request.reviewed_corpus,
    )?;
    let final_label_bindings_match = projection_audit.coverage_complete
        && projection_audit.bindings_match
        && final_label_blockers.is_empty();
    let mut blockers = BTreeSet::new();
    blockers.extend(final_label_blockers);

""",
    )
    replace_once(
        path,
        """        reviewed_corpus_present,
        projection_complete,
        reviewed_corpus_evaluated,
""",
        """        reviewed_corpus_present,
        projection_complete,
        final_label_bindings_match,
        reviewed_corpus_evaluated,
""",
    )
    helper = r'''
fn final_label_binding_blockers(
    acceptance: &AcceptanceReceipt,
    projection: &ReviewProjection,
    corpus: &OfflineCorpus,
) -> Result<BTreeSet<String>, ContractError> {
    let mut blockers = BTreeSet::new();
    let evaluation_candidate_count = corpus.cases.iter().try_fold(0_usize, |count, case| {
        count
            .checked_add(case.candidates.len())
            .ok_or(ContractError::Overflow)
    })?;
    if acceptance.items.len() != evaluation_candidate_count
        || projection.entries.len() != evaluation_candidate_count
    {
        blockers.insert("acceptance.final_label_coverage_incomplete".to_string());
    }

    let mut accepted_items = BTreeMap::new();
    for item in &acceptance.items {
        let item_id_sha256 = p1c1_digest(item.item_id_sha256)?;
        if accepted_items.insert(item_id_sha256, item).is_some() {
            return Err(ContractError::Corrupt(
                "duplicate accepted item digest in final-label projection".to_string(),
            ));
        }
    }
    let cases = corpus
        .cases
        .iter()
        .map(|case| (case.case_id.as_str(), case))
        .collect::<BTreeMap<_, _>>();

    for entry in &projection.entries {
        let item_id_sha256 = Digest32::for_bytes(entry.item_id.as_bytes());
        let Some(item) = accepted_items.get(&item_id_sha256) else {
            blockers.insert("acceptance.final_label_item_missing".to_string());
            continue;
        };
        let Some(case) = cases.get(entry.case_id.as_str()) else {
            continue;
        };
        let Some(candidate) = case
            .candidates
            .iter()
            .find(|candidate| candidate.candidate_id == entry.candidate_id)
        else {
            continue;
        };

        if !item.resolved || !item.accepted {
            blockers.insert("acceptance.final_label_item_unaccepted".to_string());
        }
        if item.locale != case.locale {
            blockers.insert("acceptance.final_label_locale_mismatch".to_string());
        }
        if item.final_labels.relevance != candidate.relevance_grade {
            blockers.insert("acceptance.relevance_label_mismatch".to_string());
        }
        match item.final_labels.citation {
            CitationLabel::Partial => {
                blockers.insert("acceptance.citation_label_not_representable".to_string());
            }
            CitationLabel::Supported if !candidate.citation_supported => {
                blockers.insert("acceptance.citation_label_mismatch".to_string());
            }
            CitationLabel::Unsupported if candidate.citation_supported => {
                blockers.insert("acceptance.citation_label_mismatch".to_string());
            }
            CitationLabel::Unsupported | CitationLabel::Supported => {}
        }
        if item.final_labels.contradiction != candidate_contradiction_label(candidate) {
            blockers.insert("acceptance.contradiction_label_mismatch".to_string());
        }
        if item.final_labels.privacy != PrivacyDecision::Allow {
            blockers.insert("acceptance.privacy_materialization_missing".to_string());
        }
    }
    Ok(blockers)
}

fn candidate_contradiction_label(candidate: &CandidateFixture) -> ContradictionLabel {
    let truth = u64::from(candidate.edge1_truth_ppm) + u64::from(candidate.edge2_truth_ppm);
    let contradiction = u64::from(candidate.edge1_contradiction_ppm)
        + u64::from(candidate.edge2_contradiction_ppm);
    if contradiction > truth {
        ContradictionLabel::Confirmed
    } else {
        ContradictionLabel::None
    }
}

'''
    replace_once(path, "fn run_seven_lanes(\n", helper + "fn run_seven_lanes(\n")


def patch_projection() -> None:
    replace_once(
        "codex-rs/hepta-memory-p1-1c2-qualification/src/projection.rs",
        "fn reviews_by_item<'a>(reviews: &'a ReviewBatch) -> BTreeMap<&'a str, Vec<&'a ReviewRecord>> {",
        "fn reviews_by_item(reviews: &ReviewBatch) -> BTreeMap<&str, Vec<&ReviewRecord>> {",
    )


def patch_tests() -> None:
    path = "codex-rs/hepta-memory-p1-1c2-qualification/tests/p1_1c2.rs"
    replace_once(
        path,
        """    AcceptancePolicy, AcceptanceReceipt, DependencyState, ReviewBatch, evaluate_review_batch,
""",
        """    AcceptancePolicy, AcceptanceReceipt, AdjudicationRecord, CitationLabel,
    DependencyState, Digest32 as ReviewDigest32, PrivacyDecision, ReviewBatch,
    evaluate_review_batch,
""",
    )
    replace_once(
        path,
        """    assert!(receipt.reviewed_corpus_evaluated);
    assert!(receipt.efficacy_thresholds_passed);
""",
        """    assert!(receipt.reviewed_corpus_evaluated);
    assert!(receipt.final_label_bindings_match);
    assert!(receipt.efficacy_thresholds_passed);
""",
    )
    replace_once(
        path,
        """    assert!(!receipt.reviewed_corpus_evaluated);
    assert!(!receipt.efficacy_validation);
""",
        """    assert!(!receipt.reviewed_corpus_evaluated);
    assert!(!receipt.final_label_bindings_match);
    assert!(!receipt.efficacy_validation);
""",
    )
    helper = r'''
fn recompute_acceptance(stack: &mut PositiveStack) {
    stack.acceptance = evaluate_review_batch(
        &stack.reviews,
        &stack.dependency,
        &stack.acceptance_policy,
    )
    .expect("recomputed acceptance");
    assert!(stack.acceptance.reviewed_corpus_accepted);
}

'''
    replace_once(path, "fn blocked_seed_receipt()", helper + "fn blocked_seed_receipt()")
    tests = r'''
#[test]
fn final_relevance_labels_must_match_evaluation_candidate() {
    let mut stack = positive_stack();
    let item_id = stack.reviews.reviews[0].item_id.clone();
    let original = stack.reviews.reviews[0].labels.relevance;
    let replacement = if original == 3 { 2 } else { original + 1 };
    for review in stack
        .reviews
        .reviews
        .iter_mut()
        .filter(|review| review.item_id == item_id)
    {
        review.labels.relevance = replacement;
    }
    recompute_acceptance(&mut stack);
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("blocked label drift");
    assert!(!receipt.final_label_bindings_match);
    assert!(!receipt.reviewed_corpus_evaluated);
    assert!(receipt.lanes.is_empty());
    assert!(receipt
        .blocked_reasons
        .contains(&"acceptance.relevance_label_mismatch".to_string()));
}

#[test]
fn partial_citation_label_requires_a_richer_evaluation_schema() {
    let mut stack = positive_stack();
    let item_id = stack.reviews.reviews[0].item_id.clone();
    for review in stack
        .reviews
        .reviews
        .iter_mut()
        .filter(|review| review.item_id == item_id)
    {
        review.labels.citation = CitationLabel::Partial;
    }
    recompute_acceptance(&mut stack);
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("blocked partial citation");
    assert!(!receipt.final_label_bindings_match);
    assert!(!receipt.reviewed_corpus_evaluated);
    assert!(receipt
        .blocked_reasons
        .contains(&"acceptance.citation_label_not_representable".to_string()));
}

#[test]
fn accepted_redaction_requires_materialized_redacted_corpus_bytes() {
    let mut stack = positive_stack();
    let item_id = stack.reviews.reviews[0].item_id.clone();
    for review in stack
        .reviews
        .reviews
        .iter_mut()
        .filter(|review| review.item_id == item_id)
    {
        review.labels.privacy = PrivacyDecision::Redact;
    }
    let labels = stack
        .reviews
        .reviews
        .iter()
        .find(|review| review.item_id == item_id)
        .expect("redacted review")
        .labels;
    stack.reviews.adjudications.push(AdjudicationRecord {
        item_id,
        adjudicator_commitment: ReviewDigest32::for_bytes(b"qualification-adjudicator"),
        labels,
        redaction_receipt_sha256: Some(ReviewDigest32::for_bytes(
            b"qualification-redaction-receipt",
        )),
        rationale_sha256: ReviewDigest32::for_bytes(b"qualification-redaction-rationale"),
    });
    recompute_acceptance(&mut stack);
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("blocked redaction materialization");
    assert!(!receipt.final_label_bindings_match);
    assert!(!receipt.reviewed_corpus_evaluated);
    assert!(receipt
        .blocked_reasons
        .contains(&"acceptance.privacy_materialization_missing".to_string()));
}

'''
    replace_once(
        path,
        "#[test]\nfn reviewed_evaluation_receipt_is_deterministic()",
        tests + "#[test]\nfn reviewed_evaluation_receipt_is_deterministic()",
    )


def patch_workflow() -> None:
    path = ".github/workflows/hepta-intelligence-p1-1c2-reviewed-efficacy.yml"
    replace_once(
        path,
        'cargo fmt --manifest-path "$MANIFEST_PATH" --all -- --check \\\n',
        'cargo fmt --manifest-path "$MANIFEST_PATH" --package hepta-memory-p1-1c2-qualification -- --check \\\n',
    )
    text = Path(path).read_text(encoding="utf-8")
    text = text.replace("passed != 15", "passed != 18")
    text = text.replace("expected 15 P1.1c.2 tests", "expected 18 P1.1c.2 tests")
    text = text.replace('"tests_passed": 15', '"tests_passed": 18')
    text = text.replace("fmt / 15 tests / check", "fmt / 18 tests / check")
    marker = '          assert receipt["projection_complete"] is False\n'
    if text.count(marker) != 1:
        raise SystemExit("workflow projection assertion marker is not unique")
    text = text.replace(
        marker,
        marker + '          assert receipt["final_label_bindings_match"] is False\n',
        1,
    )
    Path(path).write_text(text, encoding="utf-8")


def patch_verifier() -> None:
    path = "scripts/verify-hepta-intelligence-p1-1c2-reviewed-efficacy.py"
    label_check = r'''    checks["evaluation.final_resolved_label_binding"] = contains_all(
        evaluation,
        (
            "final_label_bindings_match",
            "final_label_binding_blockers",
            "acceptance.final_label_coverage_incomplete",
            "acceptance.relevance_label_mismatch",
            "acceptance.citation_label_not_representable",
            "acceptance.contradiction_label_mismatch",
            "acceptance.privacy_materialization_missing",
        ),
    )
'''
    replace_once(
        path,
        '    checks["evaluation.recomputes_acceptance"] = contains_all(\n',
        label_check + '    checks["evaluation.recomputes_acceptance"] = contains_all(\n',
    )
    replace_once(
        path,
        '            "blocked_acceptance_receipt_cannot_be_reused_with_qualified_dependency",\n',
        '            "blocked_acceptance_receipt_cannot_be_reused_with_qualified_dependency",\n'
        '            "final_relevance_labels_must_match_evaluation_candidate",\n'
        '            "partial_citation_label_requires_a_richer_evaluation_schema",\n'
        '            "accepted_redaction_requires_materialized_redacted_corpus_bytes",\n',
    )
    replace_once(
        path,
        '            "nested, publish-disabled workspace",\n',
        '            "nested, publish-disabled workspace",\n'
        '            "final resolved review labels",\n'
        '            "privacy redaction requires materialized corpus bytes",\n',
    )
    replace_once(
        path,
        '        and status.get("implementation", {}).get("transitive_workspace_isolation") is True\n',
        '        and status.get("implementation", {}).get("transitive_workspace_isolation") is True\n'
        '        and status.get("implementation", {}).get("final_resolved_label_binding") is True\n'
        '        and status.get("implementation", {}).get("privacy_materialization_gate") is True\n',
    )
    replace_once(
        path,
        '        and receipt.get("claims", {}).get("transitive_workspace_isolation") is True\n',
        '        and receipt.get("claims", {}).get("transitive_workspace_isolation") is True\n'
        '        and receipt.get("claims", {}).get("final_resolved_label_binding") is True\n'
        '        and receipt.get("claims", {}).get("privacy_materialization_gate") is True\n',
    )
    replace_once(
        path,
        '            "cargo fmt --manifest-path",\n',
        '            "cargo fmt --manifest-path",\n'
        '            "--package hepta-memory-p1-1c2-qualification",\n',
    )
    replace_once(
        path,
        '            "BLOCKED_P1_1C2_REVIEWED_CORPUS_DEPENDENCY",\n',
        '            "BLOCKED_P1_1C2_REVIEWED_CORPUS_DEPENDENCY",\n'
        '            "final_label_bindings_match",\n'
        '            "expected 18 P1.1c.2 tests",\n',
    )


def patch_plan() -> None:
    path = "plans/hepta-intelligence/P1-1C2_REVIEWED_CORPUS_EFFICACY_PLAN.md"
    replace_once(
        path,
        """candidate projection digests match
locale bindings match
""",
        """candidate projection digests match
locale bindings match
final resolved review labels match the evaluation labels
partial citation labels are rejected until the evaluation schema can represent them
privacy redaction requires materialized corpus bytes bound to the acceptance evidence
""",
    )
    closure = """
### 4.1 Final-label materialization closure

Candidate-content equality is necessary but not sufficient. P1.1c.1 may conservatively resolve or adjudicate labels after the candidate bytes were reviewed. P1.1c.2 therefore binds every accepted `ResolvedItemReceipt` back to its projected evaluation candidate and fails closed unless:

```text
final relevance == evaluation relevance grade
final citation == the evaluation citation representation
final contradiction == the deterministic evaluation contradiction class
final privacy == allow
```

`citation=partial` is not representable by the current boolean P1.1c citation field and is blocked. `privacy=redact` is accepted by P1.1c.1 only with a redaction receipt, but P1.1c.2 still blocks it until the redacted corpus bytes and their digest are materialized. No lane may run from stale pre-adjudication labels or unmaterialized private content.

The machine receipt exposes only `final_label_bindings_match`; blocker codes contain no item, query, candidate or reviewer identifiers.

"""
    replace_once(path, "## 5. Seven-lane rerun\n", closure + "## 5. Seven-lane rerun\n")
    replace_once(
        path,
        "cargo fmt --manifest-path codex-rs/hepta-memory-p1-1c2-qualification/Cargo.toml --all -- --check",
        "cargo fmt --manifest-path codex-rs/hepta-memory-p1-1c2-qualification/Cargo.toml --package hepta-memory-p1-1c2-qualification -- --check",
    )
    replace_once(
        path,
        """projection_complete=false
reviewed_corpus_evaluated=false
""",
        """projection_complete=false
final_label_bindings_match=false
reviewed_corpus_evaluated=false
""",
    )


def patch_machine_documents() -> None:
    status_path = Path("plans/hepta-intelligence/P1-1C2_EXECUTION_STATUS.json")
    status = json.loads(status_path.read_text(encoding="utf-8"))
    implementation = status["implementation"]
    implementation.update(
        {
            "package_scoped_rustfmt": True,
            "final_resolved_label_binding": True,
            "relevance_label_binding": True,
            "citation_representation_gate": True,
            "contradiction_label_binding": True,
            "privacy_materialization_gate": True,
        }
    )
    for blocker in (
        "final resolved-label equality for every evaluation candidate",
        "materialized digest-bound corpus bytes for any accepted redaction",
    ):
        if blocker not in status["blockers"]:
            status["blockers"].append(blocker)
    status_path.write_text(json.dumps(status, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    receipt_path = Path("plans/hepta-intelligence/P1-1C2_IMPLEMENTATION_RECEIPT.json")
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    receipt["claims"].update(
        {
            "package_scoped_rustfmt": True,
            "final_resolved_label_binding": True,
            "relevance_label_binding": True,
            "citation_representation_gate": True,
            "contradiction_label_binding": True,
            "privacy_materialization_gate": True,
        }
    )
    receipt_path.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> None:
    patch_evaluation()
    patch_projection()
    patch_tests()
    patch_workflow()
    patch_verifier()
    patch_plan()
    patch_machine_documents()


if __name__ == "__main__":
    main()
