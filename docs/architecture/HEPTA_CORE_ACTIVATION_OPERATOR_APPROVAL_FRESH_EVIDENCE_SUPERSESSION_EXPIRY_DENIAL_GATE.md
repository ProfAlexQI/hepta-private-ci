# Hepta Core Activation Operator Approval Fresh Evidence Supersession Expiry Denial Gate

Status: gated, read-only, non-activation.

Gate:

`scripts/hepta-core-activation-operator-approval-fresh-evidence-supersession-expiry-denial-gate.sh`

## Purpose

This gate prevents future fresh evidence or operator approval records from
being reused outside the activation request they were issued for.

It exists because freshness alone is not enough. A valid activation decision
must bind fresh evidence and operator approval to the same current activation
request. Both records must be unexpired and unsuperseded. An older operator
approval cannot be reused after a new activation request appears, even when new
fresh evidence later exists.

## Source Gates

The gate captures two read-only source gates:

- `scripts/hepta-core-activation-long-soak-observation-freshness-denial-gate.sh`
- `scripts/hepta-core-activation-long-soak-operator-approval-packet-gate.sh`

The first source proves that an observed long soak is not a fresh trusted
evidence record. The second source defines the operator approval packet schema
and proves that no approval packet, activation request, or approval record has
been recorded.

## Supersession And Expiry Contract

The report is ready only when these policy edges remain denied:

- fresh evidence without an activation request binding
- fresh evidence bound to a different activation request
- expired fresh evidence
- superseded fresh evidence
- operator approval without fresh evidence binding
- operator approval bound to a different activation request
- expired operator approval
- superseded operator approval
- old operator approval reused after a new activation request
- current fresh evidence paired with old approval
- ledger, receipt, or terminal closure from a superseded pair
- public, artifact, install, restart, or live mutation attempts

The expected decision is:

`blocked_until_operator_approval_and_fresh_evidence_bind_same_current_activation_request_and_are_unexpired_unsuperseded`

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
- approval packet persistence
- evidence record persistence
- ledger, receipt, or terminal closure persistence

Only local source report reads and the inherited watchdog/soak observations from
the source gates are allowed.

## Preflight Position

`scripts/hepta-preflight.sh` runs this gate immediately after the core
activation long-soak operator approval packet gate and before the fresh
long-soak evidence ledger receipt gate.

That placement makes the activation request binding, expiry, and supersession
rules explicit before any later ledger, receipt, acceptance, or terminal
closure gate can consume a hypothetical positive evidence or approval record.
