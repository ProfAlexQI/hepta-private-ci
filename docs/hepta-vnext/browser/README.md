# Hepta Browser development bundle

This directory is the repository-native authority for the active Hepta browser
development lane. Historical Dropbox snapshots and earlier root-level browser
plans are provenance only; they cannot override this bundle or grant authority.

## Current posture

- Phase: `DEVELOPMENT`
- Claim: `L1_QUALIFICATION_ONLY`
- C0: typed contracts and negative authority implemented
- C1: exact Servo source topology, empty governed patch queue, private worker
  protocol, offline provenance generator, and child-pipe qualification harness
  implemented but **not yet qualified**
- C2: deterministic semantic fixture implemented
- C3: deterministic single-owner Actor fixture implemented
- Not implemented: imported/built Servo, Unix inherited socketpair, Windows
  SID-restricted named pipe, real WebView, browser egress, production caller,
  durable recovery, operator acceptance, promotion, or release
- GitHub Actions currently fails before recording job steps; this is an
  environment/policy blocker, not a passing or failing code test result
- All runtime, effect, production, network, credential, operator, promotion and
  release authority flags remain false

## Normative files

- `CURRENT.yaml` — current machine-readable posture, open gates and authority
- `EXECUTION_PLAN.md` — architecture and C0-C7 execution plan
- `STAGE_MATRIX.yaml` — ordered entry/exit criteria and receipt kinds
- `TRACEABILITY_MATRIX.yaml` — requirement-to-code/test coverage
- `THREAT_MODEL.md` — assets, adversaries, trust boundaries and controls
- `STATE_MACHINES.md` — session, ownership and idempotency transitions
- `SERVO_UPSTREAM_PIN.json` — exact source candidate; pin only, not an import
- `SERVO_SOURCE_IMPORT_TOPOLOGY.yaml` — isolated checkout, worker artifact and
  patch-queue boundary
- `SERVO_PROVENANCE.md` — deterministic offline source receipt procedure
- `NEXT_WORK_QUEUE.yaml` — exact C1-001 through C1-008 implementation order
- `hepta.browser.qualification_receipt.v1.schema.json` — browser qualification
  receipt schema
- `hepta.servo.source_receipt.v1.schema.json` — Servo source receipt schema

## Repository entrypoints

```sh
python3 scripts/verify-hepta-browser-plan.py
python3 scripts/test_generate_hepta_servo_provenance.py
```

The first command is the canonical plan and implementation verifier. The
`verify-hepta-browser-plan-v2.py` file is its verifier core and should not be
called as an alternative authority entrypoint.

A Servo source receipt can be generated only from an independently prepared,
clean, exact source checkout:

```sh
python3 scripts/generate-hepta-servo-provenance.py \
  --servo-source /absolute/canonical/servo-checkout \
  --output /absolute/private/output/servo-source-receipt.json
```

Generating this receipt does not import, build, execute or qualify Servo.

## Non-claims

The current private stdio child-pipe harness proves protocol and process
behavior only. It is not the required production transport, is not a real Servo
runtime, and does not satisfy WEB-C1. A later C1 receipt must bind exact Servo
source, patches, toolchain, SBOM, artifact, platform, private OS transport,
sandbox, listener scan, external-egress denial and real one-WebView tests.
