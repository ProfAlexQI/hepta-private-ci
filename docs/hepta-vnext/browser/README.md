# Hepta Browser development bundle

This directory is the repository-native authority for the active Hepta browser lane. Historical Dropbox snapshots and older C1 pointers are provenance only.

## Current posture

- Phase: `DEVELOPMENT`
- Claim: `L1_QUALIFICATION_ONLY`
- Canonical C1 pointer: `C1_CURRENT_V7.json`
- Canonical aggregate: `.github/workflows/hepta-browser-next-required-v9.yml`
- Required future source check: `Source-only accepted pointer live review`
- Required future topology check: `Worker source/API topology accepted pointer live review`
- Source and topology live-review workflows execute trusted-base verifier code; PR-head code is fetched only as bounded data and is never executed
- C0: typed contracts and negative-authority defaults implemented
- C1: source acquisition, exact-source candidate, source acceptance, source/API topology verification, topology acceptance, private protocol, build-input and preflight tooling implemented as contracts and fixtures
- C2: deterministic semantic fixture implemented
- C3: deterministic single-owner Actor fixture implemented
- Exact Servo source accepted: `false`
- Worker source/API topology accepted: `false`
- Build authorized: `false`
- Servo built or runtime qualified: `false`
- All runtime, product, effect, network, credential, operator, execute, promotion and release authority remains false

## Normative files

- `CURRENT.yaml` — root machine-readable posture
- `C1_CURRENT_V7.json` — canonical C1 evidence sequence and next actions
- `SOURCE_ACCEPTANCE_REVIEW_POLICY_V1.json` — exact-source acceptance review policy
- `C1_EXACT_SOURCE_ACCEPTANCE_POINTER_V1.md` — source-only pointer contract
- `WORKER_SOURCE_TOPOLOGY_ACCEPTANCE_REVIEW_POLICY_V1.json` — topology acceptance review policy
- `C1_WORKER_SOURCE_TOPOLOGY_ACCEPTANCE_POINTER_V1.md` — topology-only pointer contract
- `C1_WORKER_SOURCE_TOPOLOGY_ACCEPTANCE_POINTER_V1_STATUS.json` — current topology acceptance evidence posture
- `SERVO_WORKER_SOURCE_TOPOLOGY_V1.json` — selected public Servo API and rejected servoshell/WebDriver surface
- `hepta.servo.worker_source_topology_acceptance_review_policy.v1.schema.json`
- `hepta.servo.worker_source_topology_acceptance_review_challenge.v1.schema.json`
- `hepta.servo.accepted_worker_source_topology_pointer.v1.schema.json`
- `EXECUTION_PLAN.md`, `STAGE_MATRIX.yaml`, `TRACEABILITY_MATRIX.yaml`, `THREAT_MODEL.md`, `STATE_MACHINES.md`

## Current entrypoints

```sh
python3 scripts/verify-hepta-browser-plan.py

python3 scripts/hepta-servo-exact-source-acceptance-pointer-v2.py contract \
  --policy "$(pwd)/docs/hepta-vnext/browser/SOURCE_ACCEPTANCE_REVIEW_POLICY_V1.json"

python3 scripts/hepta-servo-worker-source-topology-acceptance-pointer-v1.py contract \
  --policy "$(pwd)/docs/hepta-vnext/browser/WORKER_SOURCE_TOPOLOGY_ACCEPTANCE_REVIEW_POLICY_V1.json"

python3 scripts/tests/test_hepta_servo_worker_source_topology_acceptance_pointer_v1.py -v
python3 scripts/verify-hepta-servo-worker-source-topology-acceptance-pointer-v1.py
```

Neither acceptance tool has a command that creates or updates its final pointer. Source acceptance and topology acceptance must occur in separate dedicated PRs with deterministic snapshots, current-head review evidence and trusted-base status checks.

A future topology approval must contain exactly:

```text
HEPTA_WORKER_TOPOLOGY_ACCEPT_V1 <challenge_id>
```

A passing topology pointer still does not authorize a build. The Hepta-owned Worker crate, exact toolchain, reviewed recipe, sealed inputs, bounded offline build, artifact, symbols, SPDX SBOM, reproducibility, private launch and one-WebView qualification remain separate gates.
