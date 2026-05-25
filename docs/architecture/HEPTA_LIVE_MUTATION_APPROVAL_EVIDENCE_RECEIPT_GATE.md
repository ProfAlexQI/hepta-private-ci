# Hepta Live Mutation Approval Evidence Receipt Gate

Date: 2026-05-25

This gate turns the live mutation governance and rollback drill outputs into a
candidate evidence receipt. It still does not approve, persist, or execute any
live mutation. The receipt exists only on stdout so that future activation work
has a concrete shape to bind approval, rollback, soak, and side-effect evidence
to.

## Contract

The gate requires:

- the live mutation governance gate is `ready`
- the rollback drill gate is `ready`
- memory/capability absorption remains `14/14`
- `live_mutation_enabled_count = 0`
- core fusion remains complete with active binary package `hepta-cli`
- active engine dependency closure remains complete with `0` remaining direct
  dependencies
- the governance installed SHA and rollback installed SHA match
- the rollback plan is dry-run only
- all side-effect maps remain false

## Receipt Shape

The emitted receipt has:

- `receipt_mode = candidate_no_write_no_activation`
- `approval_evidence_receipt_ready = true`
- `activation_allowed_by_receipt = false`
- `operator_approval_recorded = false`
- `trusted_evidence_recorded = false`
- `receipt_persisted = false`
- `receipt_persistence_enabled = false`
- `live_mutation_execution_ready = false`
- a deterministic `receipt_payload_sha256`

## Activation Boundary

This gate deliberately keeps activation blocked. Before any future live mutation
can be considered, a separate approval path must provide:

- explicit operator approval id
- single-surface activation scope
- fresh trusted evidence record
- current installed-binary backup after approval
- reviewed rollback plan
- minimum 24-sample pre-activation soak
- post-activation watchdog evidence
- post-activation minimum 24-sample soak
- side-effect receipt with no secret values

## Safety Boundary

The gate must not:

- write memory, skills, plugin registries, or capability registries
- invoke providers or models
- send channel messages
- mutate runtime stores or Gateway queues
- write receipt files or release artifacts
- restart launchd
- execute rollback
- read credentials

It may read live status reports, run the no-write governance gate, and run the
dry-run rollback drill gate.
