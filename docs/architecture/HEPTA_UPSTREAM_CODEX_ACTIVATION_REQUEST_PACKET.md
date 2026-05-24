# Hepta Upstream Codex Activation Request Packet

## Scope

This packet defines the required schema for any future upstream Codex active
wiring request. It is a schema packet only: no activation packet is recorded,
and active wiring remains disallowed by default.

- Packet id: `upstream-codex-activation-request-packet-schema`
- Source precondition gate:
  `scripts/hepta-upstream-codex-active-wiring-precondition.sh`
- Packet schema gate:
  `scripts/hepta-upstream-codex-activation-request-packet.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## Source Preconditions

- Active wiring precondition ready: `true`
- Active wiring allowed by precondition: `false`
- Operator approval required: `true`
- Operator approval recorded: `false`
- Activation request id required: `true`
- Activation request id recorded: `false`

## Schema Status

- Required schema field count: `14`
- Recorded required schema field count: `0`
- Schema field count: `14`
- Activation packet schema ready: `true`
- Activation packet recorded: `false`
- Active wiring allowed: `false`

## Required Fields

- `activation_request_id`
- `operator_approval_id`
- `operator_identity_hash`
- `approved_bucket_ids`
- `approved_surface_ids`
- `requested_runtime_wiring_scope`
- `requested_dependency_change_set`
- `live_dependency_isolation_evidence_id`
- `watchdog_evidence_id`
- `browser_smoke_evidence_id`
- `long_soak_evidence_id`
- `rollback_plan_id`
- `public_release_claim_decision`
- `release_artifact_write_decision`

`operator_identity_hash` must be redacted or hashed. No raw credential,
account, provider, or channel secret may be recorded in the packet.

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## Packet Invariants

- Packet schema is ready but no activation packet is recorded.
- Operator approval must be explicit and is not recorded by default.
- Activation request id must be concrete and is not recorded by default.
- Live dependency isolation, watchdog, browser smoke, long soak, and rollback
  evidence are required fields.
- Public release and artifact decisions stay false in the schema packet.

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

- Record a concrete `activation_request_id` before any active wiring.
- Record an operator approval id and hashed operator identity before any active
  wiring.
- Attach fresh live dependency isolation, watchdog, browser smoke, long-soak,
  and rollback evidence ids.
- Keep active Codex engine dependency and release artifact decisions false
  unless separately approved.
- Rerun clean preflight and live gates after any future activation packet is
  recorded.
