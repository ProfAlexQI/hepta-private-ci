# Hepta Upstream Codex Activation Trusted Record Shape Validator

## Scope

This validator proves that partially verified evidence records and public-claim
attempts remain denied even when the records have the trusted-evidence shape. It
does not record trusted evidence and does not approve active wiring, public
release claims, or release artifact writes.

- Validator id:
  `upstream-codex-activation-trusted-record-shape-validator`
- Source trusted acceptance matrix gate:
  `scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh`
- Trusted record shape validator gate:
  `scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## Validator Status

- Source trusted acceptance matrix ready: `true`
- Required evidence count: `8`
- Fixture count: `2`
- Partial trusted fixture count: `1`
- Public claim attempt fixture count: `1`
- Blocked fixture count: `2`
- Allowed fixture count: `0`
- Required verification count per record: `7`
- Total required verification count per fixture: `56`
- Max satisfied verification count: `48`
- Trusted record shape validator ready: `true`
- Activation blocked by shape validator: `true`
- Activation allowed by shape validator: `false`
- Shape denial reason:
  `partial or public-claim trusted-record shapes stay blocked until every record
  is fresh, bound, trusted, and operator-approved`
- Active wiring allowed: `false`

## Fixtures

### `partial-trusted-records`

- Fixture kind: `partial_trusted_records`
- Evidence record count: `8`
- Schema-complete record count: `8`
- Required verification count per record: `7`
- Total required verification count: `56`
- Total satisfied verification count: `32`
- Operator approval verified record count: `8`
- Request-binding verified record count: `8`
- Active binary SHA verified record count: `8`
- Route/status hash verified record count: `8`
- Artifact hash verified record count: `0`
- Freshness window satisfied record count: `0`
- Trusted source verified record count: `0`
- Accepted record count: `0`
- Blocked record count: `8`
- Validation status: `blocked`
- Active wiring allowed: `false`
- Public release claim allowed: `false`
- Release artifact write allowed: `false`

### `public-claim-attempt-with-trusted-shape`

- Fixture kind: `public_claim_attempt`
- Evidence record count: `8`
- Schema-complete record count: `8`
- Required verification count per record: `7`
- Total required verification count: `56`
- Total satisfied verification count: `48`
- Operator approval verified record count: `8`
- Request-binding verified record count: `8`
- Active binary SHA verified record count: `8`
- Route/status hash verified record count: `8`
- Artifact hash verified record count: `8`
- Freshness window satisfied record count: `0`
- Trusted source verified record count: `8`
- Accepted record count: `0`
- Blocked record count: `8`
- Public release claim requested: `true`
- Release artifact write requested: `true`
- Validation status: `blocked`
- Active wiring allowed: `false`
- Public release claim allowed: `false`
- Release artifact write allowed: `false`

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## Shape Invariants

- Partially verified evidence records are not trusted evidence.
- Public release claims stay blocked when any evidence record is incomplete.
- Release artifact writes stay blocked when freshness is missing.
- Active wiring stays false for every trusted-record shape fixture.
- Shape validation is report-only and performs no upstream or runtime mutation.

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

- Record a real operator-approved activation request before replacing fixtures.
- Verify all seven checks for every required evidence record.
- Rerun clean preflight, live gates, browser smoke, and long soak after evidence
  recording.
- Require a separate explicit operator decision before any public claim or
  artifact write.
