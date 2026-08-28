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
        """    pub acceptance_recomputed: bool,
    pub acceptance_receipt_matches: bool,
    pub reviewed_corpus_present: bool,
""",
        """    pub acceptance_recomputed: bool,
    pub acceptance_receipt_matches: bool,
    pub acceptance_policy_matches: bool,
    pub reviewed_corpus_present: bool,
""",
    )
    replace_once(
        path,
        """                || !self.acceptance_receipt_matches
                || !self.projection_complete
""",
        """                || !self.acceptance_receipt_matches
                || !self.acceptance_policy_matches
                || !self.projection_complete
""",
    )

    canonical_checks = r'''        if self
            .blocked_reasons
            .windows(2)
            .any(|pair| pair[0].as_str() >= pair[1].as_str())
        {
            return Err(ContractError::Corrupt(
                "blocked reasons must be strictly sorted and unique".to_string(),
            ));
        }
        let expected_status = if !self.reviewed_corpus_evaluated {
            "BLOCKED_P1_1C2_REVIEWED_CORPUS_DEPENDENCY"
        } else if self.efficacy_thresholds_passed {
            "PASS_P1_1C2_REVIEWED_CORPUS_EFFICACY_VALIDATION"
        } else {
            "FAIL_P1_1C2_EFFICACY_THRESHOLDS"
        };
        if self.status != expected_status {
            return Err(ContractError::Corrupt(
                "P1.1c.2 receipt status is not canonical".to_string(),
            ));
        }
        if !self.reviewed_corpus_evaluated && self.blocked_reasons.is_empty() {
            return Err(ContractError::Corrupt(
                "blocked reviewed corpus requires at least one blocker".to_string(),
            ));
        }
        let lane_order = self
            .lanes
            .iter()
            .map(|lane| lane.lane)
            .collect::<Vec<_>>();
        let expected_lane_order = AblationLane::ALL.iter().copied().collect::<Vec<_>>();
        if self.reviewed_corpus_evaluated && lane_order != expected_lane_order {
            return Err(ContractError::Corrupt(
                "evaluated lanes are not in canonical ablation order".to_string(),
            ));
        }
'''
    replace_once(
        path,
        """        for lane in &self.lanes {
            lane.validate()?;
        }
        if self.reviewed_corpus_evaluated {
""",
        """        for lane in &self.lanes {
            lane.validate()?;
        }
""" + canonical_checks + "        if self.reviewed_corpus_evaluated {\n",
    )

    replace_once(
        path,
        """        for (matches, blocker) in [
            (
                self.baseline_receipt_matches,
""",
        """        for (matches, blocker) in [
            (
                self.acceptance_policy_matches,
                "acceptance.reference_policy_mismatch",
            ),
            (
                self.baseline_receipt_matches,
""",
    )

    replace_once(
        path,
        """                "  \\\"acceptance_recomputed\\\": {},\\n",
                "  \\\"acceptance_receipt_matches\\\": {},\\n",
                "  \\\"reviewed_corpus_present\\\": {},\\n",
""",
        """                "  \\\"acceptance_recomputed\\\": {},\\n",
                "  \\\"acceptance_receipt_matches\\\": {},\\n",
                "  \\\"acceptance_policy_matches\\\": {},\\n",
                "  \\\"reviewed_corpus_present\\\": {},\\n",
""",
    )
    replace_once(
        path,
        """            self.acceptance_recomputed,
            self.acceptance_receipt_matches,
            self.reviewed_corpus_present,
""",
        """            self.acceptance_recomputed,
            self.acceptance_receipt_matches,
            self.acceptance_policy_matches,
            self.reviewed_corpus_present,
""",
    )
    replace_once(
        path,
        """                &[u8::from(self.acceptance_recomputed)],
                &[u8::from(self.acceptance_receipt_matches)],
                &[u8::from(self.reviewed_corpus_present)],
""",
        """                &[u8::from(self.acceptance_recomputed)],
                &[u8::from(self.acceptance_receipt_matches)],
                &[u8::from(self.acceptance_policy_matches)],
                &[u8::from(self.reviewed_corpus_present)],
""",
    )

    replace_once(
        path,
        """    let recomputed = evaluate_review_batch(
        request.review_batch,
        request.dependency,
        request.acceptance_policy,
    )?;
    let acceptance_receipt_matches = recomputed == *request.acceptance_receipt;
""",
        """    let reference_acceptance_policy = AcceptancePolicy::default();
    reference_acceptance_policy.validate()?;
    let acceptance_policy_matches = request.acceptance_policy == &reference_acceptance_policy;
    let recomputed = evaluate_review_batch(
        request.review_batch,
        request.dependency,
        &reference_acceptance_policy,
    )?;
    let acceptance_receipt_matches = recomputed == *request.acceptance_receipt;
""",
    )
    replace_once(
        path,
        """    let mut blockers = BTreeSet::new();
    blockers.extend(final_label_blockers);
    if !baseline_receipt_matches {
""",
        """    let mut blockers = BTreeSet::new();
    blockers.extend(final_label_blockers);
    if !acceptance_policy_matches {
        blockers.insert("acceptance.reference_policy_mismatch".to_string());
    }
    if !baseline_receipt_matches {
""",
    )
    replace_once(
        path,
        """        acceptance_recomputed: true,
        acceptance_receipt_matches,
        reviewed_corpus_present,
""",
        """        acceptance_recomputed: true,
        acceptance_receipt_matches,
        acceptance_policy_matches,
        reviewed_corpus_present,
""",
    )

    replace_once(
        path,
        """        if item.final_labels.privacy != PrivacyDecision::Allow {
            blockers.insert("acceptance.privacy_materialization_missing".to_string());
        }
""",
        """        match item.final_labels.privacy {
            PrivacyDecision::Allow => {}
            PrivacyDecision::Redact => {
                blockers.insert("acceptance.privacy_materialization_missing".to_string());
            }
            PrivacyDecision::Block => {
                blockers.insert("acceptance.privacy_blocked".to_string());
            }
        }
""",
    )
    replace_once(
        path,
        """    if contradiction > truth {
        ContradictionLabel::Confirmed
    } else {
        ContradictionLabel::None
    }
""",
        """    if contradiction == 0 {
        ContradictionLabel::None
    } else if contradiction > truth {
        ContradictionLabel::Confirmed
    } else {
        ContradictionLabel::Potential
    }
""",
    )


def patch_tests() -> None:
    path = "codex-rs/hepta-memory-p1-1c2-qualification/tests/p1_1c2.rs"
    replace_once(
        path,
        """    AcceptancePolicy, AcceptanceReceipt, AdjudicationRecord, CitationLabel,
    DependencyState, Digest32 as ReviewDigest32, PrivacyDecision, ReviewBatch,
""",
        """    AcceptancePolicy, AcceptanceReceipt, AdjudicationRecord, CitationLabel,
    ContradictionLabel, DependencyState, Digest32 as ReviewDigest32, PrivacyDecision,
    ReviewBatch,
""",
    )
    replace_once(
        path,
        """            let contradiction_label = if contradiction > truth {
                "confirmed"
            } else {
                "none"
            };
""",
        """            let contradiction_label = if contradiction == 0 {
                "none"
            } else if contradiction > truth {
                "confirmed"
            } else {
                "potential"
            };
""",
    )

    tests = r'''
#[test]
fn potential_contradiction_labels_are_preserved() {
    let stack = positive_stack();
    assert!(stack.reviews.reviews.iter().any(|review| {
        review.labels.contradiction == ContradictionLabel::Potential
    }));
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("potential labels");
    assert!(receipt.final_label_bindings_match);
    assert!(receipt.reviewed_corpus_evaluated);
}

#[test]
fn privacy_block_uses_a_distinct_fail_closed_blocker() {
    let mut stack = positive_stack();
    let item_id = stack.reviews.reviews[0].item_id.clone();
    for review in stack
        .reviews
        .reviews
        .iter_mut()
        .filter(|review| review.item_id == item_id)
    {
        review.labels.privacy = PrivacyDecision::Block;
    }
    stack.acceptance = evaluate_review_batch(
        &stack.reviews,
        &stack.dependency,
        &stack.acceptance_policy,
    )
    .expect("privacy-blocked acceptance receipt");
    assert!(!stack.acceptance.reviewed_corpus_accepted);
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("blocked privacy item");
    assert!(!receipt.final_label_bindings_match);
    assert!(!receipt.reviewed_corpus_evaluated);
    assert!(receipt
        .blocked_reasons
        .contains(&"acceptance.privacy_blocked".to_string()));
}

#[test]
fn alternate_valid_acceptance_policy_is_blocked() {
    let mut stack = positive_stack();
    stack.acceptance_policy.minimum_exact_tuple_agreement_ppm -= 1;
    stack.acceptance_policy.validate().expect("alternate valid acceptance policy");
    stack.acceptance = evaluate_review_batch(
        &stack.reviews,
        &stack.dependency,
        &stack.acceptance_policy,
    )
    .expect("alternate-policy acceptance receipt");
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("blocked acceptance policy");
    assert!(!receipt.acceptance_policy_matches);
    assert!(!receipt.reviewed_corpus_evaluated);
    assert!(receipt.lanes.is_empty());
    assert!(receipt
        .blocked_reasons
        .contains(&"acceptance.reference_policy_mismatch".to_string()));
}

#[test]
fn receipt_rejects_noncanonical_status_lane_order_and_blockers() {
    let stack = positive_stack();
    let mut status_receipt =
        evaluate_reviewed_corpus(&stack.request()).expect("positive receipt");
    status_receipt.status = "BLOCKED_P1_1C2_REVIEWED_CORPUS_DEPENDENCY".to_string();
    assert!(status_receipt
        .validate()
        .expect_err("noncanonical status")
        .to_string()
        .contains("status is not canonical"));

    let mut lane_receipt =
        evaluate_reviewed_corpus(&stack.request()).expect("positive lane receipt");
    lane_receipt.lanes.swap(0, 1);
    assert!(lane_receipt
        .validate()
        .expect_err("noncanonical lane order")
        .to_string()
        .contains("canonical ablation order"));

    let mut blocker_receipt = blocked_seed_receipt();
    let duplicate = blocker_receipt.blocked_reasons[0].clone();
    blocker_receipt.blocked_reasons.push(duplicate);
    blocker_receipt.blocked_reasons.sort();
    assert!(blocker_receipt
        .validate()
        .expect_err("duplicate blocker")
        .to_string()
        .contains("strictly sorted and unique"));
}

'''
    replace_once(
        path,
        "#[test]\nfn reviewed_evaluation_receipt_is_deterministic()",
        tests + "#[test]\nfn reviewed_evaluation_receipt_is_deterministic()",
    )


def patch_workflow() -> None:
    path = ".github/workflows/hepta-intelligence-p1-1c2-reviewed-efficacy.yml"
    text = Path(path).read_text(encoding="utf-8")
    replacements = {
        "passed != 21": "passed != 25",
        "expected 21 P1.1c.2 tests": "expected 25 P1.1c.2 tests",
        '"tests_passed": 21': '"tests_passed": 25',
        "fmt / 21 tests / check": "fmt / 25 tests / check",
    }
    for old, new in replacements.items():
        count = text.count(old)
        if count == 0:
            raise SystemExit(f"{path}: missing workflow test-count marker {old!r}")
        text = text.replace(old, new)
    marker = '          assert receipt["efficacy_policy_matches"] is True\n'
    if text.count(marker) != 1:
        raise SystemExit("workflow efficacy-policy assertion marker is not unique")
    text = text.replace(
        marker,
        marker + '          assert receipt["acceptance_policy_matches"] is True\n',
        1,
    )
    Path(path).write_text(text, encoding="utf-8")


def patch_verifier() -> None:
    path = "scripts/verify-hepta-intelligence-p1-1c2-reviewed-efficacy.py"
    replace_once(
        path,
        '            "reference_seed_baseline",\n'
        '            "baseline_receipt_matches",\n',
        '            "reference_seed_baseline",\n'
        '            "acceptance_policy_matches",\n'
        '            "acceptance.reference_policy_mismatch",\n'
        '            "baseline_receipt_matches",\n',
    )
    replace_once(
        path,
        '            "alternate_valid_efficacy_policy_is_blocked",\n',
        '            "alternate_valid_efficacy_policy_is_blocked",\n'
        '            "alternate_valid_acceptance_policy_is_blocked",\n'
        '            "potential_contradiction_labels_are_preserved",\n'
        '            "privacy_block_uses_a_distinct_fail_closed_blocker",\n'
        '            "receipt_rejects_noncanonical_status_lane_order_and_blockers",\n',
    )
    replace_once(
        path,
        '            "expected 21 P1.1c.2 tests",\n',
        '            "expected 25 P1.1c.2 tests",\n'
        '            "acceptance_policy_matches",\n',
    )
    replace_once(
        path,
        '        and status.get("implementation", {}).get("caller_reference_substitution_blocked") is True\n',
        '        and status.get("implementation", {}).get("caller_reference_substitution_blocked") is True\n'
        '        and status.get("implementation", {}).get("reference_acceptance_policy_binding") is True\n'
        '        and status.get("implementation", {}).get("potential_contradiction_preserved") is True\n'
        '        and status.get("implementation", {}).get("privacy_block_distinguished") is True\n'
        '        and status.get("implementation", {}).get("receipt_canonicality_enforced") is True\n',
    )
    replace_once(
        path,
        '        and receipt.get("claims", {}).get("caller_reference_substitution_blocked") is True\n',
        '        and receipt.get("claims", {}).get("caller_reference_substitution_blocked") is True\n'
        '        and receipt.get("claims", {}).get("reference_acceptance_policy_binding") is True\n'
        '        and receipt.get("claims", {}).get("potential_contradiction_preserved") is True\n'
        '        and receipt.get("claims", {}).get("privacy_block_distinguished") is True\n'
        '        and receipt.get("claims", {}).get("receipt_canonicality_enforced") is True\n',
    )


def patch_plan() -> None:
    path = "plans/hepta-intelligence/P1-1C2_REVIEWED_CORPUS_EFFICACY_PLAN.md"
    replace_once(
        path,
        """efficacy policy equals the exact source-only default policy
""",
        """efficacy policy equals the exact source-only default policy
acceptance policy equals `AcceptancePolicy::default()`
""",
    )
    replace_once(
        path,
        """EfficacyPolicy::default() -> exact source-only threshold policy
""",
        """EfficacyPolicy::default() -> exact source-only threshold policy
AcceptancePolicy::default() -> exact reviewed-corpus acceptance threshold policy
""",
    )
    replace_once(
        path,
        """A structurally valid but different baseline, calibration contract or policy emits a dedicated blocker and zero lane evidence. This prevents a caller from changing deltas, rankings or pass/fail thresholds while retaining superficially valid digests.
""",
        """A structurally valid but different baseline, calibration contract, efficacy policy or acceptance policy emits a dedicated blocker and zero lane evidence. This prevents a caller from changing deltas, rankings, corpus admission or pass/fail thresholds while retaining superficially valid digests.
""",
    )
    replace_once(
        path,
        """baseline_receipt_matches=true
calibration_contract_matches=true
efficacy_policy_matches=true
reviewed_corpus_evaluated=false
""",
        """baseline_receipt_matches=true
calibration_contract_matches=true
efficacy_policy_matches=true
acceptance_policy_matches=true
reviewed_corpus_evaluated=false
""",
    )
    section = """
### 4.3 Canonical receipt and label-state closure

The evaluator preserves all three contradiction states (`none`, `potential`, `confirmed`), distinguishes privacy `redact` from privacy `block`, and rejects noncanonical receipt status, blocker ordering, duplicate blockers and lane ordering before digest acceptance.

`DependencyState::qualified(...)` remains an input contract rather than cryptographically verified exact-head evidence. Therefore this tranche may qualify the deterministic evaluation mechanics only; it must keep `source_qualified=false`, `efficacy_claim=false`, `production_authority=false`, and cannot activate product recall. Exact dependency qualification-receipt binding is a separate prerequisite before any external efficacy claim.

"""
    replace_once(path, "## 5. Seven-lane rerun\n", section + "## 5. Seven-lane rerun\n")


def patch_machine_documents() -> None:
    status_path = Path("plans/hepta-intelligence/P1-1C2_EXECUTION_STATUS.json")
    status = json.loads(status_path.read_text(encoding="utf-8"))
    status["implementation"].update(
        {
            "reference_acceptance_policy_binding": True,
            "potential_contradiction_preserved": True,
            "privacy_block_distinguished": True,
            "receipt_canonicality_enforced": True,
        }
    )
    status.setdefault("blockers", []).append(
        "exact dependency qualification-receipt binding"
    )
    status["blockers"] = sorted(set(status["blockers"]))
    status_path.write_text(json.dumps(status, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    receipt_path = Path("plans/hepta-intelligence/P1-1C2_IMPLEMENTATION_RECEIPT.json")
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    receipt["claims"].update(
        {
            "reference_acceptance_policy_binding": True,
            "potential_contradiction_preserved": True,
            "privacy_block_distinguished": True,
            "receipt_canonicality_enforced": True,
        }
    )
    receipt_path.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> None:
    patch_evaluation()
    patch_tests()
    patch_workflow()
    patch_verifier()
    patch_plan()
    patch_machine_documents()


if __name__ == "__main__":
    main()
