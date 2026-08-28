# Hepta Intelligence P1.1c.3 — Trusted Evidence and Real Corpus Intake Gateway

**Date:** 2026-08-29  
**Status:** source implementation / external evidence absent / no product authority  
**Parent:** `c3c42f60e5ca11fd49fe5d4d2013c5b21e183619`  
**Branch:** `codex/p1c3-evidence-intake-20260829`

## 1. Objective

P1.1c.3 closes the code-level trust gap left by P1.1c.1 and P1.1c.2. Earlier layers can validate deterministic review and efficacy mechanics, but they still accept a caller-constructed `DependencyState::qualified(...)` and digest commitments that do not prove the identity of a CI runner, reviewer, adjudicator, license approver, provenance approver, privacy approver or operator.

This tranche introduces a final fail-closed intake gateway. A corpus cannot become mechanically accepted unless all of the following are present and mutually bound:

```text
signed P1.1c exact-head executable qualification
signed P1.1c.1 exact-head executable qualification
P1.1c.1 accepted reviewed-corpus receipt
signed reviewer and adjudicator evidence
independent reviewer affiliations
independent adjudicator affiliation
signed license approval
signed provenance approval
signed privacy and redaction approval
signed operator approval
one exact dataset digest
one exact trust-store digest
one frozen intake policy digest
```

## 2. Trust model

The implementation uses Ed25519 signatures over framed SHA-256 payload digests. The trust store binds:

```text
key ID
public key
role
affiliation
locale authorization
validity interval
revocation state
trust domain
key digest
```

Supported roles are:

```text
ci_qualification
reviewer
adjudicator
license_approver
provenance_approver
privacy_approver
operator
```

A key tagged `qualification_fixture` cannot satisfy a policy requiring `external_attested` evidence.

## 3. Exact-head qualification evidence

A verified qualification receipt requires non-zero and exact:

```text
repository
source commit
source tree
workflow name
workflow run ID and attempt
job ID
runner ID and name
runner OS and architecture
step count
commands-executed state
success conclusion
Rust toolchain
passed test count
artifact ID and SHA-256
source/fmt/test/check/Clippy/reproducibility/redaction/clean-tree gates
CI signature
```

Queued jobs, `steps=[]`, `runner_id=0`, source-only scripts and unsigned JSON cannot produce a verified qualification receipt.

## 4. Review trust

Every review row is signed over the same canonical digest domain used by P1.1c.1. Reviewer commitments must resolve to trusted public keys, and every item must have two distinct reviewer affiliations. Adjudicators must use a trusted adjudicator key whose affiliation is independent from both reviewers for that item.

The review trust receipt binds the P1.1c.1 review-batch digest, adjudication-batch digest, reviewer-set digest, all verified signature receipts and the exact trust store.

## 5. Dataset governance

The exact accepted corpus digest must be shared by:

- the P1.1c.1 acceptance receipt;
- license evidence;
- provenance evidence;
- privacy evidence;
- operator approval subject;
- the final intake receipt.

License policy freezes the allowed SPDX identifiers and required evaluation, storage and derivative rights. Provenance evidence binds acquisition method, legal basis, source manifest and collection window. Privacy evidence binds secret scanning, PII assessment, redaction materialization and residual-risk approval.

## 6. Authority boundary

Even a mechanically accepted external intake receipt grants no product authority:

```text
product_workspace_member=false
product_module_registered=false
runtime_wired=false
default_recall_changed=false
federation_recall_changed=false
context_attachment=false
physical_send=false
external_effects=false
production_authority=false
efficacy_validation=false
efficacy_claim=false
operator_acceptance=false
promotion=false
callers_ratchet=false
```

P1.1c.3 proves that an offline corpus evidence bundle satisfies the frozen intake protocol. It does not activate retrieval, attach context, send data, claim efficacy or authorize promotion.

## 7. Checked-in fixture policy

The checked-in binary emits only a blocked fixture receipt. Tests may use deterministic signing keys to exercise the positive mechanics, but those keys and receipts are not real external evidence and are not committed as an accepted corpus.

Real closure still requires externally supplied, independently reviewable evidence:

```text
real P1.1c/P1.1c.1 exact-head signed CI receipts
real reviewer and adjudicator public keys and signatures
real affiliation declarations
real corpus bytes and immutable dataset digest
real license and provenance approvals
real privacy scan and redaction evidence
real operator approval
```

The software rejects missing or inconsistent evidence; it does not fabricate it.
