# Hepta Browser development bundle

This directory is the repository-native authority for the active Hepta browser development lane.

## Current posture

- Phase: `DEVELOPMENT`
- Claim: `L1_QUALIFICATION_ONLY`
- Implemented: C0 contracts, C2 deterministic semantic fixture, C3 deterministic single-owner Actor fixture
- Not implemented: real Servo C1, real browser networking, production caller, durable recovery, operator acceptance and promotion
- All runtime/effect/production/network/credential/promotion authority flags remain false

## Normative files

- `CURRENT.yaml` — current machine-readable posture and authority flags
- `EXECUTION_PLAN.md` — architecture and C0–C7 execution plan
- `STAGE_MATRIX.yaml` — ordered entry/exit criteria and receipt kinds
- `TRACEABILITY_MATRIX.yaml` — requirement-to-code/test coverage
- `THREAT_MODEL.md` — assets, adversaries, trust boundaries and required controls
- `SERVO_UPSTREAM_PIN.json` — source candidate for C1; pin only, not an import
- `hepta.browser.qualification_receipt.v1.schema.json` — strict qualification receipt schema

`python3 scripts/verify-hepta-browser-plan-v2.py` validates this bundle in CI. JSON is used inside the `.yaml` files so the verifier requires only the Python standard library while the files remain valid YAML.

Historical Dropbox snapshots and earlier root-level browser planning documents remain provenance inputs only. They do not override `CURRENT.yaml` and do not grant runtime or release authority.
