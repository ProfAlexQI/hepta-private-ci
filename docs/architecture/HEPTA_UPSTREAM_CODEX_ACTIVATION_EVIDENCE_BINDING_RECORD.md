# Hepta Upstream Codex Activation Evidence Binding Record Manifest

## Scope

This manifest defines the record shape required before any activation evidence
slot can count as recorded or fresh. It records zero concrete evidence by
default and does not approve active wiring.

- Manifest id: `upstream-codex-activation-evidence-binding-record-manifest`
- Source freshness policy gate: `scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh`
- Binding manifest gate:
  `scripts/hepta-upstream-codex-activation-evidence-binding-record.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## Manifest Status

- Freshness policy ready: `true`
- Required evidence count: `8`
- Binding record count: `8`
- Missing binding record count: `8`
- Recorded binding record count: `0`
- Required record schema field count: `7`
- Recorded record schema field count: `0`
- Timestamped record count: `0`
- Binary SHA bound record count: `0`
- Route or status hash bound record count: `0`
- Artifact hash or redacted path bound record count: `0`
- Activation request id bound record count: `0`
- Binding manifest ready: `true`
- Activation blocked by binding manifest: `true`
- Activation allowed by binding manifest: `false`
- Binding denial reason:
  `all evidence binding records are schema-only and unrecorded`
- Active wiring allowed: `false`

## Required Record Fields

- `evidence_record_id`
  - Purpose: stable id for the evidence record.
- `source_gate`
  - Purpose: gate or document that produced the evidence.
- `recorded_at_unix_ms`
  - Purpose: timestamp used for freshness evaluation.
- `active_binary_sha256`
  - Purpose: active Hepta binary SHA bound to live evidence.
- `route_or_status_hash`
  - Purpose: hash of the route response or status payload used as evidence.
- `artifact_sha256_or_redacted_path`
  - Purpose: artifact hash or redacted local path for browser, soak, or
    rollback evidence.
- `activation_request_id_binding`
  - Purpose: activation request id that this evidence record authorizes.

## Binding Records

- `activation_request_id`
  - Source gate: `scripts/hepta-upstream-codex-activation-request-packet.sh`
  - Evidence recorded: `false`
- `operator_approval_id`
  - Source gate: `scripts/hepta-codex-public-ga-operator-approval-packet.sh`
  - Evidence recorded: `false`
- `operator_identity_hash`
  - Source gate: `scripts/hepta-codex-public-ga-operator-approval-packet.sh`
  - Evidence recorded: `false`
- `live_dependency_isolation_evidence_id`
  - Source gate: `scripts/hepta-active-service-dependency-isolation.sh`
  - Evidence recorded: `false`
- `watchdog_evidence_id`
  - Source gate: `scripts/hepta-codex-watchdog.sh`
  - Evidence recorded: `false`
- `browser_smoke_evidence_id`
  - Source gate: `scripts/hepta-codex-browser-visual-smoke.sh`
  - Evidence recorded: `false`
- `long_soak_evidence_id`
  - Source gate: `scripts/hepta-codex-live-soak.sh`
  - Evidence recorded: `false`
- `rollback_plan_id`
  - Source gate: `docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_DENIED_SAMPLE.md`
  - Evidence recorded: `false`

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## Binding Invariants

- Binding manifest defines record shape without recording evidence.
- Every evidence record must bind to an activation request id.
- Live evidence records must bind active binary SHA and route or status hash.
- Artifact-bearing records must use artifact hash or redacted artifact path.
- Schema-only binding records keep active wiring, public release, and artifact
  writes denied.

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

- Materialize concrete evidence records only after operator approval.
- Populate timestamp, active binary SHA, route/status hash, artifact hash or
  redacted path, and activation request binding for every evidence slot.
- Rerun freshness policy against recorded evidence before allowing activation
  review.
- Rerun clean preflight and live gates before any active wiring decision.
