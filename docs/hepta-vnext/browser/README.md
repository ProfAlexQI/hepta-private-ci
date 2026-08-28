# Hepta Browser development bundle

This directory is the repository-native authority for the active Hepta browser lane. Historical Dropbox snapshots and older C1 pointers are provenance only.

## Current posture

- Phase: `DEVELOPMENT`
- Claim: `L1_QUALIFICATION_ONLY`
- Canonical C1 pointer: `C1_CURRENT_V6.json`
- Canonical aggregate: `.github/workflows/hepta-browser-next-required-v8.yml`
- Required future live check: `Source-only accepted pointer live review`
- C0: typed contracts and negative-authority defaults implemented
- C1: source acquisition, exact-source candidate, source-only review fence, private protocol, source/API topology, build-input and preflight tooling implemented as contracts and fixtures
- C2: deterministic semantic fixture implemented
- C3: deterministic single-owner Actor fixture implemented
- Exact Servo source accepted: `false`
- Worker source/API topology accepted: `false`
- Build authorized: `false`
- Servo built or runtime qualified: `false`
- All runtime, product, effect, network, credential, operator, execute, promotion and release authority remains false

## Normative files

- `CURRENT.yaml` — root machine-readable posture
- `C1_CURRENT_V6.json` — canonical C1 evidence sequence and next actions
- `SOURCE_ACCEPTANCE_REVIEW_POLICY_V1.json` — self-bound dedicated-review policy
- `C1_EXACT_SOURCE_ACCEPTANCE_POINTER_V1.md` — source-only pointer and live-review contract
- `C1_EXACT_SOURCE_ACCEPTANCE_POINTER_V1_STATUS.json` — current evidence posture
- `hepta.servo.source_acceptance_review_policy.v1.schema.json`
- `hepta.servo.source_acceptance_review_challenge.v1.schema.json`
- `hepta.servo.accepted_source_pointer.v1.schema.json`
- `SERVO_WORKER_SOURCE_TOPOLOGY_V1.json` — selected public Servo API and rejected servoshell/WebDriver surface
- `EXECUTION_PLAN.md`, `STAGE_MATRIX.yaml`, `TRACEABILITY_MATRIX.yaml`, `THREAT_MODEL.md`, `STATE_MACHINES.md`

## Current entrypoints

```sh
python3 scripts/verify-hepta-browser-plan.py
python3 scripts/hepta-servo-exact-source-review-candidate-v2.py contract
python3 scripts/hepta-servo-exact-source-acceptance-pointer-v2.py contract \
  --policy "$(pwd)/docs/hepta-vnext/browser/SOURCE_ACCEPTANCE_REVIEW_POLICY_V1.json"
python3 scripts/verify-hepta-servo-exact-source-acceptance-pointer-v2.py
python3 scripts/tests/test_hepta_servo_exact_source_acceptance_pointer_v2.py -v
```

The acceptance tool can create a review challenge and verify a manually authored pointer. It has no command that creates or updates `source-acceptance/ACCEPTED_SOURCE_POINTER.json`.

A real proposal retains deterministic candidate and challenge snapshots, binds the assigned PR number and exact head ref in the final pointer commit, changes only policy-allowed files, receives a distinct current-head trusted-collaborator approval containing:

```text
HEPTA_SOURCE_ACCEPT_V1 <challenge_id>
```

and passes `.github/workflows/hepta-servo-exact-source-acceptance-live-review.yml`.

The verifier intentionally does not equate `author_association` with CODEOWNER identity. Repository rules may add CODEOWNER review independently.

A passing source pointer still does not authorize a build. A separate source/API topology pointer, exact toolchain, reviewed recipe, sealed build inputs, bounded offline build, artifact, symbols, SPDX SBOM, reproducibility and one-WebView qualification remain required.
