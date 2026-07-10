# Hepta Systems Controlled Live Fresh Soak Preflight Source Cache

## Purpose

This source-cache makes `fresh_soak_readback_missing` a short, reusable
controlled-live fact before any status canary path can open. It consumes the
existing readiness audit, denial readback index, required evidence readback
index, required evidence gap summary, and canary readiness plan.

It does not run a soak, collect probes, record evidence, persist readbacks,
request approval, or unlock canary/live execution.

Controlled Live Fresh Soak Preflight Source Cache

## Sources

- `scripts/hepta-systems-controlled-live-readiness-audit-report.sh`
- `scripts/hepta-systems-controlled-live-readiness-denial-readback-index-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-readback-index-report.sh`
- `scripts/hepta-systems-controlled-live-required-evidence-gap-summary-report.sh`
- `scripts/hepta-systems-controlled-canary-readiness-plan-report.sh`

## Contract

The report must prove:

- `fresh_soak_readback_missing` remains queryable, operator-visible, and high
  risk under `runtime_soak_owner`
- readiness audit still records no fresh soak/readback evidence for this
  cutover
- required evidence readback and gap summary both keep the fresh soak evidence
  state as `missing`
- canary readiness still requires soak/readback and keeps the start guard
  blocked
- evidence recording, evidence persistence, readback persistence, approval,
  canary start, live execution, and Public GA all remain blocked

## Side-Effect Boundary

Closed boundary summary: no fresh soak execution, probe collection, evidence recording, readback persistence, approval request, approval acceptance, canary start, live execution, or Public GA.

This source-cache performs no filesystem write, git index mutation, approval
request, approval acceptance, approval recording, evidence recording, evidence
persistence, blocker waiver, credential read, readback persistence, fresh soak
start, fresh soak probe collection, fresh soak probe persistence, ledger write,
receipt persistence, runtime event-log write, workflow event-log write, SQLite
write, Native POST mutation, Gateway/Auth mutation, Telegram transport
mutation, channel send, provider/model invocation, rollback execution,
kill-switch mutation, package/release write, canary start, live execution, or
Public GA promotion.
