# Hepta Upstream Codex Activation Evidence Freshness Policy

## Scope

This policy defines the freshness requirements for every evidence slot required
by an upstream Codex activation request. It records no evidence by default and
does not convert the denied activation sample into an approval.

- Policy id: `upstream-codex-activation-evidence-freshness-policy`
- Source denied sample gate: `scripts/hepta-upstream-codex-activation-denied-sample.sh`
- Freshness policy gate:
  `scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh`
- Active dependency gate: `scripts/hepta-active-service-dependency-isolation.sh`
- Candidate diff range:
  `108234b5ebe6941764a6b8edbb37b2aa04369f07..7d47056ea42636271ac020b86347fbbef49490aa`

## Policy Status

- Denied sample ready: `true`
- Required evidence count: `8`
- Policy entry count: `8`
- Missing evidence count: `8`
- Fresh evidence count: `0`
- Expired evidence count: `0`
- Stale evidence count: `0`
- Freshness policy ready: `true`
- Activation blocked by freshness policy: `true`
- Activation allowed by freshness policy: `false`
- Freshness denial reason: `all required activation evidence slots are absent
  from the denied sample`
- Active wiring allowed: `false`

## Evidence Freshness Entries

- `activation_request_id`
  - Source gate: `scripts/hepta-upstream-codex-activation-request-packet.sh`
  - Freshness anchor: `candidate diff range and requested activation scope`
  - Max age policy: `same activation request`
  - Denial reason: `activation request id is absent`
- `operator_approval_id`
  - Source gate: `scripts/hepta-codex-public-ga-operator-approval-packet.sh`
  - Freshness anchor: `explicit operator approval record`
  - Max age policy: `same activation request`
  - Denial reason: `operator approval id is absent`
- `operator_identity_hash`
  - Source gate: `scripts/hepta-codex-public-ga-operator-approval-packet.sh`
  - Freshness anchor: `redacted operator identity bound to approval id`
  - Max age policy: `same activation request`
  - Denial reason: `operator identity hash is absent`
- `live_dependency_isolation_evidence_id`
  - Source gate: `scripts/hepta-active-service-dependency-isolation.sh`
  - Freshness anchor: `active binary sha and live dependency-closure route`
  - Max age policy: `30 minutes`
  - Denial reason: `live dependency isolation evidence is absent`
- `watchdog_evidence_id`
  - Source gate: `scripts/hepta-codex-watchdog.sh`
  - Freshness anchor: `active binary sha and live watchdog route matrix`
  - Max age policy: `30 minutes`
  - Denial reason: `watchdog evidence is absent`
- `browser_smoke_evidence_id`
  - Source gate: `scripts/hepta-codex-browser-visual-smoke.sh`
  - Freshness anchor: `desktop and mobile screenshot hashes`
  - Max age policy: `30 minutes`
  - Denial reason: `browser smoke evidence is absent`
- `long_soak_evidence_id`
  - Source gate: `scripts/hepta-codex-live-soak.sh`
  - Freshness anchor: `24/24 live soak sample report`
  - Max age policy: `120 minutes`
  - Denial reason: `long soak evidence is absent`
- `rollback_plan_id`
  - Source gate: `docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_DENIED_SAMPLE.md`
  - Freshness anchor: `candidate diff range and active binary rollback anchor`
  - Max age policy: `same activation request`
  - Denial reason: `rollback plan id is absent`

## Denied Active Decisions

- Active runtime code wiring allowed: `false`
- Active runtime dependency allowed: `false`
- Active runtime auto-rebase allowed: `false`
- Active Codex engine dependency allowed: `false`
- Public release claim allowed: `false`
- Public GA claim allowed: `false`
- Release artifact write allowed: `false`

## Policy Invariants

- Freshness policy defines evidence requirements but records no evidence.
- Missing evidence is a denial reason even when packet shape is complete.
- Freshness is evaluated per evidence slot before active wiring can be
  reconsidered.
- Operator approval, public release claims, and artifact writes remain denied.

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

- Define concrete evidence binding records with
  `scripts/hepta-upstream-codex-activation-evidence-binding-record.sh`.
- Timestamp and hash every live dependency, watchdog, browser, soak, and
  rollback evidence record only after an operator-approved activation request.
- Rerun the denied-sample gate after replacing absence with concrete evidence.
- Rerun clean preflight and live gates before any active wiring decision.
