# Hepta Upstream Codex Activation Evidence Ledger

## Scope

This checklist defines the evidence slots required before any future upstream
Codex activation packet can be considered. It records no concrete evidence by
default and does not allow active runtime wiring.

- Ledger id: `upstream-codex-activation-evidence-ledger-checklist`
- Source dry-run gate: `scripts/hepta-upstream-codex-activation-packet-dry-run.sh`
- Evidence ledger gate: `scripts/hepta-upstream-codex-activation-evidence-ledger.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## Ledger Status

- Dry-run validator ready: `true`
- Activation packet recorded: `false`
- Required evidence count: `8`
- Recorded evidence count: `0`
- Fresh evidence count: `0`
- Evidence ledger ready: `true`
- Evidence recorded: `false`
- Active wiring allowed: `false`

## Required Evidence

- `activation_request_id`: unique activation request binding all evidence.
- `operator_approval_id`: explicit operator approval record.
- `operator_identity_hash`: hashed operator identity with no raw account or
  credential detail.
- `live_dependency_isolation_evidence_id`: fresh output from
  `scripts/hepta-active-service-dependency-isolation.sh`.
- `watchdog_evidence_id`: fresh output from `scripts/hepta-codex-watchdog.sh`.
- `browser_smoke_evidence_id`: fresh output from
  `scripts/hepta-codex-browser-visual-smoke.sh`.
- `long_soak_evidence_id`: fresh output from `scripts/hepta-codex-live-soak.sh`.
- `rollback_plan_id`: explicit rollback anchor for the requested activation.

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## Ledger Invariants

- Evidence ledger is a checklist only and records no concrete evidence by
  default.
- All eight required evidence slots are required but unrecorded.
- Freshness starts false until evidence ids are bound to current live gates.
- Activation packet remains unrecorded while evidence is missing.
- Public release and artifact-write decisions stay denied.

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

- Close the readiness summary with
  `scripts/hepta-upstream-codex-activation-readiness-closure.sh`.
- Add a full-shaped denied sample packet with
  `scripts/hepta-upstream-codex-activation-denied-sample.sh`.
- Define per-slot freshness policy with
  `scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh`.
- Define concrete evidence binding records with
  `scripts/hepta-upstream-codex-activation-evidence-binding-record.sh`.
- Record a concrete activation request id before any active wiring.
- Record an operator approval id and hashed operator identity before any active
  wiring.
- Bind live dependency-isolation, watchdog, browser-smoke, long-soak, and
  rollback evidence ids.
- Rerun the activation packet dry-run validator after concrete evidence is
  recorded.
- Rerun clean preflight and live gates before considering any operator-approved
  activation packet.
