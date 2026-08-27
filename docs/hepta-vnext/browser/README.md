# Hepta Browser development bundle

This directory is the repository-native authority for the active Hepta browser
development lane. Historical Dropbox snapshots and earlier root-level browser
plans are provenance only; they cannot override this bundle or grant authority.

## Current posture

- Phase: `DEVELOPMENT`
- Claim: `L1_QUALIFICATION_ONLY`
- Canonical C1 pointer: `C1_CURRENT_V4.json`
- C0: typed contracts and negative authority implemented
- C1: exact source acquisition/verifier tooling, private worker protocol,
  build-input/preflight tooling, and a frozen Hepta-owned Servo source/API
  topology are implemented as qualification contracts and fixtures
- C1 embedder decision: build an out-of-tree Hepta worker directly against the
  pinned `components/servo` public embedding API; do not build or link
  `ports/servoshell` or `components/webdriver_server`
- C2: deterministic semantic fixture implemented
- C3: deterministic single-owner Actor fixture implemented
- Not accepted or implemented: an exact retained source bundle, source-topology
  receipt, Hepta worker crate, real build recipe, real Servo build, real artifact,
  Windows SID-restricted named pipe, real WebView, browser egress, production
  caller, durable recovery, operator acceptance, promotion, or release
- GitHub Actions currently fails before recording job steps; this is an
  environment/policy blocker, not a passing or failing code test result
- All runtime, effect, production, network, credential, operator, promotion and
  release authority flags remain false

## Normative files

- `CURRENT.yaml` — current machine-readable posture, open gates and authority
- `C1_CURRENT_V4.json` — canonical C1 pipeline, evidence slots and next actions
- `EXECUTION_PLAN.md` — architecture and C0-C7 execution plan
- `STAGE_MATRIX.yaml` — ordered entry/exit criteria and receipt kinds
- `TRACEABILITY_MATRIX.yaml` — requirement-to-code/test coverage
- `THREAT_MODEL.md` — assets, adversaries, trust boundaries and controls
- `STATE_MACHINES.md` — session, ownership and idempotency transitions
- `SERVO_UPSTREAM_PIN.json` — exact source candidate; pin only, not an import
- `SERVO_SOURCE_IMPORT_TOPOLOGY.yaml` — isolated checkout, worker artifact and
  patch-queue boundary
- `SERVO_WORKER_SOURCE_TOPOLOGY_V1.json` — exact selected/reference Servo blobs,
  public API anchors, feature posture, servoshell rejection and one-WebView rule
- `ADR-0003-hepta-owned-servo-embedder.md` — decision to own the minimal worker
  instead of linking servoshell/WebDriver
- `C1_WORKER_SOURCE_TOPOLOGY.md` — real verification procedure and receipt meaning
- `SERVO_PROVENANCE.md` — deterministic offline source receipt procedure
- `NEXT_WORK_QUEUE.yaml` and `NEXT_WORK_QUEUE_C1.json` — executable C1 ordering
- `hepta.browser.qualification_receipt.v1.schema.json` — browser qualification
  receipt schema
- `hepta.servo.source_receipt.v1.schema.json` — Servo source receipt schema
- `hepta.servo.worker_source_topology.v1.schema.json` — topology contract schema
- `hepta.servo.worker_source_topology_verification.v1.schema.json` — accepted
  source-bytes topology verification schema

## Repository entrypoints

```sh
python3 scripts/verify-hepta-browser-plan.py
python3 scripts/hepta-servo-worker-source-topology.py contract
python3 scripts/verify-hepta-servo-worker-source-topology.py
python3 scripts/tests/test_hepta_servo_worker_source_topology.py -v
```

The source-topology `contract` command verifies the repository-native frozen
selection only. It does not fetch, accept, build, link, launch, or execute Servo.

After an exact two-fetch source bundle has been independently accepted, the same
tool can reverify the retained source bytes and selected upstream blobs:

```sh
python3 scripts/hepta-servo-worker-source-topology.py verify \
  --bundle-dir /absolute/private/accepted-source-bundle \
  --output /absolute/private/output/worker-source-topology-receipt.json
```

A passing topology receipt still does not create a build recipe or authorize a
build. It only proves that the accepted source bytes retain the exact public API
and servoshell/WebDriver exclusion facts frozen by the plan.

## Non-claims

The current protocol, process, source, topology, build-input and preflight jobs
cover contracts and fixtures only. They are not a real Servo runtime and do not
satisfy WEB-C1. A later C1 receipt must bind exact accepted source, topology,
toolchain, recipe, build inputs, artifact, symbols, SBOM, reproducibility,
platform transport, sandbox, listener scan, external-egress denial and one real
local-fixture WebView.
