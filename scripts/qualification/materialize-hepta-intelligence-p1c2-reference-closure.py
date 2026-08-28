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
        """    PPM_DENOMINATOR,
    RankedCandidate,
""",
        """    PPM_DENOMINATOR, RankedCandidate, evaluate_corpus,
""",
    )
    replace_once(
        path,
        """    pub projection_complete: bool,
    pub final_label_bindings_match: bool,
    pub reviewed_corpus_evaluated: bool,
""",
        """    pub projection_complete: bool,
    pub final_label_bindings_match: bool,
    pub baseline_receipt_matches: bool,
    pub calibration_contract_matches: bool,
    pub efficacy_policy_matches: bool,
    pub reviewed_corpus_evaluated: bool,
""",
    )
    replace_once(
        path,
        """                || !self.final_label_bindings_match
                || !self.blocked_reasons.is_empty()
""",
        """                || !self.final_label_bindings_match
                || !self.baseline_receipt_matches
                || !self.calibration_contract_matches
                || !self.efficacy_policy_matches
                || !self.blocked_reasons.is_empty()
""",
    )
    replace_once(
        path,
        """        if self.final_label_bindings_match && !self.projection_complete {
            return Err(ContractError::Corrupt(
                "final review labels cannot match an incomplete projection".to_string(),
            ));
        }
        if self.efficacy_validation
""",
        """        if self.final_label_bindings_match && !self.projection_complete {
            return Err(ContractError::Corrupt(
                "final review labels cannot match an incomplete projection".to_string(),
            ));
        }
        let blocker_set = self
            .blocked_reasons
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        for (matches, blocker) in [
            (
                self.baseline_receipt_matches,
                "baseline.reference_receipt_mismatch",
            ),
            (
                self.calibration_contract_matches,
                "calibration.reference_contract_mismatch",
            ),
            (
                self.efficacy_policy_matches,
                "efficacy_policy.reference_policy_mismatch",
            ),
        ] {
            if matches == blocker_set.contains(blocker) {
                return Err(ContractError::Corrupt(format!(
                    "reference evidence match state disagrees with blocker {blocker}"
                )));
            }
        }
        if self.efficacy_validation
""",
    )
    replace_once(
        path,
        """                "  \\\"final_label_bindings_match\\\": {},\\n",
                "  \\\"reviewed_corpus_evaluated\\\": {},\\n",
""",
        """                "  \\\"final_label_bindings_match\\\": {},\\n",
                "  \\\"baseline_receipt_matches\\\": {},\\n",
                "  \\\"calibration_contract_matches\\\": {},\\n",
                "  \\\"efficacy_policy_matches\\\": {},\\n",
                "  \\\"reviewed_corpus_evaluated\\\": {},\\n",
""",
    )
    replace_once(
        path,
        """            self.projection_complete,
            self.final_label_bindings_match,
            self.reviewed_corpus_evaluated,
""",
        """            self.projection_complete,
            self.final_label_bindings_match,
            self.baseline_receipt_matches,
            self.calibration_contract_matches,
            self.efficacy_policy_matches,
            self.reviewed_corpus_evaluated,
""",
    )
    replace_once(
        path,
        """                &[u8::from(self.projection_complete)],
                &[u8::from(self.final_label_bindings_match)],
                &[u8::from(self.reviewed_corpus_evaluated)],
""",
        """                &[u8::from(self.projection_complete)],
                &[u8::from(self.final_label_bindings_match)],
                &[u8::from(self.baseline_receipt_matches)],
                &[u8::from(self.calibration_contract_matches)],
                &[u8::from(self.efficacy_policy_matches)],
                &[u8::from(self.reviewed_corpus_evaluated)],
""",
    )
    replace_once(
        path,
        """    let mut blockers = BTreeSet::new();
    blockers.extend(final_label_blockers);

    if request.p1_1c1_source_commit != P1_1C1_SOURCE_COMMIT {
""",
        """    let reference_calibration = CalibrationContract::qualification_reference()?;
    let reference_baseline = reference_seed_baseline(&reference_calibration)?;
    let reference_efficacy_policy = EfficacyPolicy::default();
    reference_efficacy_policy.validate()?;
    let baseline_receipt_matches = request.baseline_receipt == &reference_baseline;
    let calibration_contract_matches = request.calibration == &reference_calibration;
    let efficacy_policy_matches = request.efficacy_policy == &reference_efficacy_policy;

    let mut blockers = BTreeSet::new();
    blockers.extend(final_label_blockers);
    if !baseline_receipt_matches {
        blockers.insert("baseline.reference_receipt_mismatch".to_string());
    }
    if !calibration_contract_matches {
        blockers.insert("calibration.reference_contract_mismatch".to_string());
    }
    if !efficacy_policy_matches {
        blockers.insert("efficacy_policy.reference_policy_mismatch".to_string());
    }

    if request.p1_1c1_source_commit != P1_1C1_SOURCE_COMMIT {
""",
    )
    replace_once(
        path,
        """        let reviewed_lanes = run_seven_lanes(request.reviewed_corpus, request.calibration)?;
        lanes = build_lane_deltas(&reviewed_lanes, request.baseline_receipt)?;
""",
        """        let reviewed_lanes = run_seven_lanes(
            request.reviewed_corpus,
            &reference_calibration,
        )?;
        lanes = build_lane_deltas(&reviewed_lanes, &reference_baseline)?;
""",
    )
    replace_once(
        path,
        """        efficacy_thresholds_passed = request
            .efficacy_policy
            .permits(&full.reviewed, case_count, locale_count);
""",
        """        efficacy_thresholds_passed = reference_efficacy_policy
            .permits(&full.reviewed, case_count, locale_count);
""",
    )
    replace_once(
        path,
        """        projection_complete,
        final_label_bindings_match,
        reviewed_corpus_evaluated,
""",
        """        projection_complete,
        final_label_bindings_match,
        baseline_receipt_matches,
        calibration_contract_matches,
        efficacy_policy_matches,
        reviewed_corpus_evaluated,
""",
    )
    helper = r'''
fn reference_seed_baseline(
    calibration: &CalibrationContract,
) -> Result<EvaluationReceipt, ContractError> {
    const SEED: &str = include_str!(
        "../../hepta-memory-p1-1c-qualification/fixtures/p1_1c_multilingual_seed.tsv"
    );
    let corpus = OfflineCorpus::parse_tsv(SEED)?;
    Ok(evaluate_corpus(&corpus, calibration)?)
}

'''
    replace_once(path, "fn final_label_binding_blockers(\n", helper + "fn final_label_binding_blockers(\n")


def patch_tests() -> None:
    path = "codex-rs/hepta-memory-p1-1c2-qualification/tests/p1_1c2.rs"
    replace_once(
        path,
        """    CalibrationContract, EvaluationReceipt, OfflineCorpus, evaluate_corpus,
""",
        """    CalibrationContract, Digest32 as P1Digest32, EvaluationReceipt, OfflineCorpus,
    evaluate_corpus,
""",
    )
    replace_once(
        path,
        """    assert!(receipt.final_label_bindings_match);
    assert!(receipt.efficacy_thresholds_passed);
""",
        """    assert!(receipt.final_label_bindings_match);
    assert!(receipt.baseline_receipt_matches);
    assert!(receipt.calibration_contract_matches);
    assert!(receipt.efficacy_policy_matches);
    assert!(receipt.efficacy_thresholds_passed);
""",
    )
    helpers = r'''
fn calibration_digest(contract: &CalibrationContract) -> P1Digest32 {
    let payload = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
        contract.contract_id,
        contract.lexical_weight_ppm,
        contract.vector_weight_ppm,
        contract.kg_weight_ppm,
        contract.grounding_weight_ppm,
        contract.truth_weight_ppm,
        contract.citation_weight_ppm,
        contract.contradiction_penalty_ppm,
        contract.learned_weights,
        contract.source_reviewed,
        contract.production_calibrated
    );
    P1Digest32::for_bytes(payload.as_bytes())
}

fn efficacy_policy_digest(policy: &EfficacyPolicy) -> Digest32 {
    let minimum_cases = policy.minimum_cases.to_be_bytes();
    let minimum_locales = policy.minimum_locales.to_be_bytes();
    let minimum_recall = policy.minimum_full_recall_at_4_ppm.to_be_bytes();
    let minimum_ndcg = policy.minimum_full_ndcg_at_4_ppm.to_be_bytes();
    let minimum_citation = policy
        .minimum_full_citation_precision_ppm
        .to_be_bytes();
    let maximum_latency = policy.maximum_full_p95_latency_micros.to_be_bytes();
    let maximum_token_cost = policy.maximum_full_mean_token_cost.to_be_bytes();
    let calibrated = [u8::from(policy.production_calibrated)];
    let mut bytes = Vec::new();
    for value in [
        b"hepta:intelligence:p1.1c2:efficacy-policy:v1".as_slice(),
        minimum_cases.as_slice(),
        minimum_locales.as_slice(),
        minimum_recall.as_slice(),
        minimum_ndcg.as_slice(),
        minimum_citation.as_slice(),
        maximum_latency.as_slice(),
        maximum_token_cost.as_slice(),
        calibrated.as_slice(),
    ] {
        bytes.extend_from_slice(&u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
        bytes.extend_from_slice(value);
    }
    Digest32::for_bytes(&bytes)
}

'''
    replace_once(path, "fn recompute_acceptance(stack: &mut PositiveStack)", helpers + "fn recompute_acceptance(stack: &mut PositiveStack)")
    tests = r'''
#[test]
fn alternate_valid_calibration_contract_is_blocked() {
    let mut stack = positive_stack();
    stack.calibration.lexical_weight_ppm += 10_000;
    stack.calibration.vector_weight_ppm -= 10_000;
    stack.calibration.contract_sha256 = calibration_digest(&stack.calibration);
    stack.calibration.validate().expect("alternate valid calibration");
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("blocked calibration substitution");
    assert!(!receipt.calibration_contract_matches);
    assert!(!receipt.reviewed_corpus_evaluated);
    assert!(receipt.lanes.is_empty());
    assert!(receipt
        .blocked_reasons
        .contains(&"calibration.reference_contract_mismatch".to_string()));
}

#[test]
fn alternate_valid_seed_baseline_is_blocked() {
    let mut stack = positive_stack();
    let drifted_seed = P1_1C_SEED.replacen(
        "\t650000\t900000\ttrue\t120\t",
        "\t640000\t900000\ttrue\t120\t",
        1,
    );
    let drifted_corpus = OfflineCorpus::parse_tsv(&drifted_seed).expect("drifted seed corpus");
    stack.baseline = evaluate_corpus(&drifted_corpus, &stack.calibration)
        .expect("alternate valid baseline");
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("blocked baseline substitution");
    assert!(!receipt.baseline_receipt_matches);
    assert!(!receipt.reviewed_corpus_evaluated);
    assert!(receipt.lanes.is_empty());
    assert!(receipt
        .blocked_reasons
        .contains(&"baseline.reference_receipt_mismatch".to_string()));
}

#[test]
fn alternate_valid_efficacy_policy_is_blocked() {
    let mut stack = positive_stack();
    stack.efficacy_policy.minimum_full_recall_at_4_ppm = 740_000;
    stack.efficacy_policy.policy_sha256 = efficacy_policy_digest(&stack.efficacy_policy);
    stack.efficacy_policy.validate().expect("alternate valid policy");
    let receipt = evaluate_reviewed_corpus(&stack.request()).expect("blocked policy substitution");
    assert!(!receipt.efficacy_policy_matches);
    assert!(!receipt.reviewed_corpus_evaluated);
    assert!(receipt.lanes.is_empty());
    assert!(receipt
        .blocked_reasons
        .contains(&"efficacy_policy.reference_policy_mismatch".to_string()));
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
    text = text.replace("passed != 18", "passed != 21")
    text = text.replace("expected 18 P1.1c.2 tests", "expected 21 P1.1c.2 tests")
    text = text.replace('"tests_passed": 18', '"tests_passed": 21')
    text = text.replace("fmt / 18 tests / check", "fmt / 21 tests / check")
    marker = '          assert receipt["final_label_bindings_match"] is False\n'
    if text.count(marker) != 1:
        raise SystemExit("workflow final-label assertion marker is not unique")
    text = text.replace(
        marker,
        marker
        + '          assert receipt["baseline_receipt_matches"] is True\n'
        + '          assert receipt["calibration_contract_matches"] is True\n'
        + '          assert receipt["efficacy_policy_matches"] is True\n',
        1,
    )
    Path(path).write_text(text, encoding="utf-8")


def patch_verifier() -> None:
    path = "scripts/verify-hepta-intelligence-p1-1c2-reviewed-efficacy.py"
    reference_check = r'''    checks["evaluation.reference_evidence_binding"] = contains_all(
        evaluation,
        (
            "reference_seed_baseline",
            "baseline_receipt_matches",
            "calibration_contract_matches",
            "efficacy_policy_matches",
            "baseline.reference_receipt_mismatch",
            "calibration.reference_contract_mismatch",
            "efficacy_policy.reference_policy_mismatch",
        ),
    )
'''
    replace_once(
        path,
        '    checks["evaluation.final_resolved_label_binding"] = contains_all(\n',
        reference_check + '    checks["evaluation.final_resolved_label_binding"] = contains_all(\n',
    )
    replace_once(
        path,
        '            "accepted_redaction_requires_materialized_redacted_corpus_bytes",\n',
        '            "accepted_redaction_requires_materialized_redacted_corpus_bytes",\n'
        '            "alternate_valid_calibration_contract_is_blocked",\n'
        '            "alternate_valid_seed_baseline_is_blocked",\n'
        '            "alternate_valid_efficacy_policy_is_blocked",\n',
    )
    replace_once(
        path,
        '            "privacy redaction requires materialized corpus bytes",\n',
        '            "privacy redaction requires materialized corpus bytes",\n'
        '            "exact reference baseline, calibration and efficacy policy",\n',
    )
    replace_once(
        path,
        '        and status.get("implementation", {}).get("privacy_materialization_gate") is True\n',
        '        and status.get("implementation", {}).get("privacy_materialization_gate") is True\n'
        '        and status.get("implementation", {}).get("reference_seed_baseline_binding") is True\n'
        '        and status.get("implementation", {}).get("reference_calibration_binding") is True\n'
        '        and status.get("implementation", {}).get("reference_efficacy_policy_binding") is True\n',
    )
    replace_once(
        path,
        '        and receipt.get("claims", {}).get("privacy_materialization_gate") is True\n',
        '        and receipt.get("claims", {}).get("privacy_materialization_gate") is True\n'
        '        and receipt.get("claims", {}).get("reference_seed_baseline_binding") is True\n'
        '        and receipt.get("claims", {}).get("reference_calibration_binding") is True\n'
        '        and receipt.get("claims", {}).get("reference_efficacy_policy_binding") is True\n',
    )
    replace_once(
        path,
        '            "expected 18 P1.1c.2 tests",\n',
        '            "expected 21 P1.1c.2 tests",\n'
        '            "baseline_receipt_matches",\n'
        '            "calibration_contract_matches",\n'
        '            "efficacy_policy_matches",\n',
    )


def patch_plan() -> None:
    path = "plans/hepta-intelligence/P1-1C2_REVIEWED_CORPUS_EFFICACY_PLAN.md"
    replace_once(
        path,
        """privacy redaction requires materialized corpus bytes bound to the acceptance evidence
""",
        """privacy redaction requires materialized corpus bytes bound to the acceptance evidence
baseline receipt equals the exact embedded P1.1c seed baseline
calibration equals `CalibrationContract::qualification_reference()`
efficacy policy equals the exact source-only default policy
""",
    )
    section = """
### 4.2 Frozen reference-evidence closure

P1.1c.2 does not trust caller-selected baseline metrics, reranker weights or efficacy thresholds. It deterministically reconstructs all three from the exact checked-in source:

```text
P1.1c multilingual seed fixture -> exact baseline receipt
CalibrationContract::qualification_reference() -> exact calibration digest
EfficacyPolicy::default() -> exact source-only threshold policy
```

A structurally valid but different baseline, calibration contract or policy emits a dedicated blocker and zero lane evidence. This prevents a caller from changing deltas, rankings or pass/fail thresholds while retaining superficially valid digests.

The submitted evidence digests remain in the machine receipt for audit; `baseline_receipt_matches`, `calibration_contract_matches` and `efficacy_policy_matches` prove equality to the frozen references bound by the exact source commit.

"""
    replace_once(path, "## 5. Seven-lane rerun\n", section + "## 5. Seven-lane rerun\n")
    replace_once(
        path,
        """final_label_bindings_match=false
reviewed_corpus_evaluated=false
""",
        """final_label_bindings_match=false
baseline_receipt_matches=true
calibration_contract_matches=true
efficacy_policy_matches=true
reviewed_corpus_evaluated=false
""",
    )


def patch_machine_documents() -> None:
    status_path = Path("plans/hepta-intelligence/P1-1C2_EXECUTION_STATUS.json")
    status = json.loads(status_path.read_text(encoding="utf-8"))
    status["implementation"].update(
        {
            "reference_seed_baseline_binding": True,
            "reference_calibration_binding": True,
            "reference_efficacy_policy_binding": True,
            "caller_reference_substitution_blocked": True,
        }
    )
    status_path.write_text(json.dumps(status, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    receipt_path = Path("plans/hepta-intelligence/P1-1C2_IMPLEMENTATION_RECEIPT.json")
    receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    receipt["claims"].update(
        {
            "reference_seed_baseline_binding": True,
            "reference_calibration_binding": True,
            "reference_efficacy_policy_binding": True,
            "caller_reference_substitution_blocked": True,
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
