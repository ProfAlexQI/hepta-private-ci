# Hepta Upstream Codex Activation Trusted Evidence Acceptance Matrix

## Scope

This matrix defines the verification checks required before placeholder
activation evidence can become trusted evidence. It does not record trusted
evidence and does not approve active wiring.

- Matrix id:
  `upstream-codex-activation-trusted-evidence-acceptance-matrix`
- Source denied fixture gate:
  `scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh`
- Trusted acceptance matrix gate:
  `scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## Matrix Status

- Source denied fixture ready: `true`
- Required evidence count: `8`
- Verification entry count: `8`
- Schema-complete verification entry count: `8`
- Required verification count per record: `7`
- Total required verification count: `56`
- Total satisfied verification count: `0`
- Operator approval verified record count: `0`
- Request-binding verified record count: `0`
- Active binary SHA verified record count: `0`
- Route/status hash verified record count: `0`
- Artifact hash verified record count: `0`
- Freshness window satisfied record count: `0`
- Trusted source verified record count: `0`
- Accepted record count: `0`
- Blocked record count: `8`
- Trusted evidence acceptance matrix ready: `true`
- Activation blocked by trusted evidence acceptance matrix: `true`
- Activation allowed by trusted evidence acceptance matrix: `false`
- Acceptance denial reason:
  `trusted evidence acceptance requires operator approval, request binding,
  hashes, freshness, and trusted source verification`
- Active wiring allowed: `false`

## Required Verification Checks

Every evidence record must satisfy all seven checks before it can be accepted:

1. Operator approval verified.
2. Activation request binding verified.
3. Active binary SHA verified.
4. Route/status hash verified.
5. Artifact hash or redacted path verified.
6. Freshness window satisfied.
7. Trusted source verified.

## Evidence Records

- `activation_request_id`
- `operator_approval_id`
- `operator_identity_hash`
- `live_dependency_isolation_evidence_id`
- `watchdog_evidence_id`
- `browser_smoke_evidence_id`
- `long_soak_evidence_id`
- `rollback_plan_id`

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## Matrix Invariants

- Schema-complete fixture records are not trusted evidence.
- Operator approval must be verified for every evidence record.
- Activation request binding, active binary SHA, and route/status hash must all
  verify.
- Artifact hash or redacted path and freshness window must verify before
  acceptance.
- Trusted source verification is required before active wiring can be
  reconsidered.

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

- Replace placeholders with operator-approved evidence records.
- Bind every evidence record to the activation request id and active binary SHA.
- Verify route/status and artifact hashes for live dependency, watchdog,
  browser, soak, and rollback evidence.
- Rerun freshness policy and clean preflight after trusted evidence is recorded.
