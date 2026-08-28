#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path


def replace_once(path: str, old: str, new: str) -> None:
    target = Path(path)
    text = target.read_text(encoding="utf-8")
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{path}: expected one replacement target, found {count}")
    target.write_text(text.replace(old, new, 1), encoding="utf-8")


def main() -> None:
    replace_once(
        "codex-rs/hepta-memory-p1-1c3-qualification/src/review_trust.rs",
        'b"hepta:intelligence:p1.1c3:reviewer-set:v1"',
        'b"hepta:intelligence:p1.1c1:reviewer-set:v1"',
    )
    replace_once(
        "codex-rs/hepta-memory-p1-1c3-qualification/tests/p1_1c3.rs",
        '''#[test]
fn external_policy_rejects_qualification_fixture_key() {
    let (store, signers) = trust_fixture(TrustDomain::QualificationFixture);
    let (evidence, mut policy, _) = qualification_evidence(
        P1C_COMMIT,
        P1C_TREE,
        "fixture-workflow",
        &store,
        &signers,
    );
    policy.require_external_signer = true;
    policy.policy_sha256 = qualification_policy_digest(&policy);
    assert!(verify_qualification(&evidence, &policy, &store, NOW).is_err());
}
''',
        '''#[test]
fn external_policy_rejects_qualification_fixture_key() {
    let fixture = FullFixture::build();
    let mut store = fixture.store.clone();
    for key in &mut store.keys {
        key.domain = TrustDomain::QualificationFixture;
        key.key_sha256 = trusted_key_digest(key);
    }
    store.store_sha256 = trust_store_digest(&store);
    let mut policy = fixture.p1c_policy.clone();
    policy.expected_trust_store_sha256 = store.store_sha256;
    policy.policy_sha256 = qualification_policy_digest(&policy);
    assert!(verify_qualification(&fixture.p1c_evidence, &policy, &store, NOW).is_err());
}
''',
    )


if __name__ == "__main__":
    main()
