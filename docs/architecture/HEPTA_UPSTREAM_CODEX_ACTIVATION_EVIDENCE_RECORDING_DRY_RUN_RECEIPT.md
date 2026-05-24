# Hepta Upstream Codex Activation Evidence Recording Dry-Run Receipt

## Scope

This receipt gate defines the redacted evidence-recording packet that would be
required after the activation evidence completeness scoreboard. It proves the
receipt shape without recording evidence, writing the workspace, enabling active
wiring, or allowing public claims.

- Receipt id:
  `upstream-codex-activation-evidence-recording-dry-run-receipt`
- Source evidence completeness scoreboard gate:
  `scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh`
- Evidence recording dry-run receipt gate:
  `scripts/hepta-upstream-codex-activation-evidence-recording-dry-run-receipt.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## Receipt Status

- Source scoreboard ready: `true`
- Required receipt field count: `12`
- Recorded receipt field count: `0`
- Redacted or hashed field count: `10`
- Required evidence count: `8`
- Required trusted record count: `8`
- Accepted trusted record count: `0`
- Fresh trusted record count: `0`
- Operator approval recorded: `false`
- Activation request recorded: `false`
- Receipt schema ready: `true`
- Receipt recorded: `false`
- Real evidence recorded: `false`
- Trusted record materialized: `false`
- Public claim attempt blocked: `true`
- Release artifact write attempt blocked: `true`
- Evidence recording dry-run ready: `true`
- Activation blocked by receipt: `true`
- Activation allowed by receipt: `false`
- Active wiring allowed: `false`

## Required Receipt Fields

All fields are required and absent by default:

- `evidence_recording_receipt_id`
- `activation_request_id`
- `operator_approval_id`
- `operator_identity_hash`
- `accepted_trusted_record_ids`
- `fresh_trusted_record_ids`
- `active_binary_sha256`
- `route_or_status_hash_bundle`
- `artifact_sha256_or_redacted_path_bundle`
- `freshness_window_summary`
- `rollback_plan_id`
- `public_claim_and_artifact_decision`

The receipt schema requires redacted or hashed values for operator identity,
evidence ids, binary hashes, live route/status hashes, artifact hashes or
redacted local artifact paths, rollback plan id, and the receipt/request ids.

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## Receipt Invariants

- Receipt schema can be ready while no evidence is recorded.
- All receipt fields remain absent until a real activation request exists.
- Redacted or hashed fields prevent raw operator identity and artifact leakage.
- Dry-run receipt readiness does not permit active wiring or public claims.

## Side-Effect Boundary

- No upstream fetch
- No upstream merge
- No upstream checkout
- No workspace mutation by default
- No active service restart
- No credential value read
- No secret file read
- No provider invocation
- No channel delivery
- No gateway RPC
- No public release publication

## Required Next Gates

- Run `scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh`
  to prove partial, stale, and public-claim-shaped receipt attempts still route
  to the no-write sink.
- Bind receipt fields to a real activation request id only after an explicit
  operator-approved recording command exists.
- Record fresh trusted evidence ids only after live gate evidence is captured.
- Rerun scoreboard, receipt, and denial matrix gates before any active runtime
  wiring.
