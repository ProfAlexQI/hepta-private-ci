# Hepta Upstream Codex Activation Evidence Denied Fixture

## Scope

This fixture proves that full-shaped evidence records are still denied when
they are placeholders rather than trusted operator-approved evidence. It does
not record real evidence and does not approve active wiring.

- Fixture id: `upstream-codex-activation-evidence-record-denied-fixture`
- Source binding manifest gate: `scripts/hepta-upstream-codex-activation-evidence-binding-record.sh`
- Denied fixture gate:
  `scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## Fixture Status

- Binding manifest ready: `true`
- Required evidence count: `8`
- Fixture record count: `8`
- Schema-complete fixture record count: `8`
- Trusted fixture record count: `0`
- Operator-approved fixture record count: `0`
- Request-binding verified record count: `0`
- Live gate hash verified record count: `0`
- Artifact hash verified record count: `0`
- Fresh fixture record count: `0`
- Blocked fixture record count: `8`
- Allowed fixture record count: `0`
- Denied fixture ready: `true`
- Activation blocked by denied fixture: `true`
- Activation allowed by denied fixture: `false`
- Fixture denial reason:
  `fixture evidence records are placeholders without operator approval or
  verified freshness`
- Active wiring allowed: `false`

## Placeholder Values

- Recorded at: `0`
- Active binary SHA: `placeholder-active-binary-sha256`
- Route/status hash: `placeholder-route-or-status-hash`
- Artifact hash or redacted path:
  `placeholder-artifact-hash-or-redacted-path`
- Activation request id binding: `placeholder-activation-request-id`

## Fixture Records

- `activation_request_id`
- `operator_approval_id`
- `operator_identity_hash`
- `live_dependency_isolation_evidence_id`
- `watchdog_evidence_id`
- `browser_smoke_evidence_id`
- `long_soak_evidence_id`
- `rollback_plan_id`

Each fixture record has a complete schema shape but remains blocked because the
operator approval, activation request binding, live gate hash, artifact hash,
freshness window, and trusted-source checks are not verified.

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## Fixture Invariants

- Full-shaped placeholder evidence records are not trusted evidence.
- Operator approval must verify every evidence record before activation review.
- Activation request binding must be verified rather than merely present.
- Live gate and artifact hashes must be verified before freshness can count.
- Denied fixtures keep active wiring, public release, and artifact writes false.

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

- Replace placeholder records with operator-approved evidence records.
- Verify activation request binding and live gate hashes for every record.
- Verify artifact hashes or redacted paths for browser, soak, and rollback
  records.
- Rerun freshness policy with trusted recorded evidence before any activation
  decision.
