# Hepta Core Activation Request Monotonic Single-Use Approval Nonce Denial Gate

Status: gated, read-only, non-activation.

Gate:

`scripts/hepta-core-activation-request-monotonic-single-use-approval-nonce-denial-gate.sh`

## Purpose

This gate adds a replay and ordering boundary after the operator approval /
fresh evidence supersession-expiry gate.

The previous gate proves that evidence and approval must bind the same current
activation request, be unexpired, and be unsuperseded. This gate adds that the
activation request itself must be monotonic and every request, fresh evidence,
operator approval, and evidence/approval pair nonce must be single use.

That prevents a future positive record from being replayed through generation
rollback, generation skip, duplicate nonce, concurrent "current request"
ambiguity, or stale downstream ledger/receipt/terminal closure state.

## Source Gate

The gate captures:

- `scripts/hepta-core-activation-operator-approval-fresh-evidence-supersession-expiry-denial-gate.sh`

The source gate already captures the freshness-denial and operator approval
packet sources. This gate consumes that result and narrows the future positive
path with monotonic activation request and nonce constraints.

## Monotonic And Single-Use Contract

The report is ready only when these policy edges remain denied:

- activation request without a generation
- activation request generation rollback
- activation request generation skip without predecessor terminal closure
- duplicate activation request nonce
- fresh evidence nonce reused across activation requests
- operator approval nonce reused across activation requests
- approval/evidence pair replay after terminal denial
- concurrent current activation request ambiguity
- ledger, receipt, or terminal closure from stale generation state
- current request paired with previous-generation fresh evidence
- current request paired with previous-generation operator approval
- public, artifact, install, restart, or live mutation attempts

The expected decision is:

`blocked_until_current_activation_request_generation_and_single_use_nonces_bind_fresh_evidence_operator_approval_ledger_receipt_terminal_closure`

## Denied Surfaces

The gate keeps these surfaces false:

- activation execution
- active runtime mutation
- active binary mutation
- service install or restart
- release artifact write
- public release claim
- provider or model invocation
- channel delivery
- credential or secret read
- upstream fetch or merge
- nonce registry persistence
- idempotency state persistence
- approval packet persistence
- evidence record persistence
- ledger, receipt, or terminal closure persistence

Only local source report reads and inherited watchdog/soak observations from
the source gate are allowed.

## Preflight Position

`scripts/hepta-preflight.sh` runs this gate immediately after the operator
approval / fresh evidence supersession-expiry denial gate and before the fresh
long-soak evidence ledger receipt gate.

That placement means later ledger, receipt, acceptance, or terminal closure
gates cannot consume a hypothetical positive approval/evidence pair unless it
also satisfies monotonic request generation and single-use nonce constraints.
