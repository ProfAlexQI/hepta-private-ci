# Hepta Live Mutation Pre-Activation Soak Evidence Gate

Date: 2026-05-25

This gate binds the live mutation approval evidence receipt to the required
pre-activation long-soak evidence. It keeps the default execution mode cheap and
safe: the gate validates the required soak command and receipt binding, but it
does not run a 24-sample soak unless explicitly asked through an environment
flag.

## Contract

The gate requires:

- the approval evidence receipt gate is `ready`
- the receipt remains `candidate_no_write_no_activation`
- `activation_allowed = false`
- `live_mutation_execution_ready = false`
- `receipt_persistence_enabled = false`
- `receipt_persisted = false`
- `operator_approval_recorded = false`
- the minimum pre-activation soak requirement is at least 24 samples
- all side-effect maps remain false

When `HEPTA_LIVE_MUTATION_PRE_ACTIVATION_SOAK_RUN=1` is set, the gate runs:

```bash
HEPTA_SOAK_SAMPLES=24 HEPTA_SOAK_INTERVAL_SECONDS=5 scripts/hepta-live-soak.sh
```

and requires `status=ready`, `ok=samples`, and `fail=0`. The resulting evidence
is still emitted only on stdout and is not persisted by this gate.

## Default Mode

Default mode does not execute the long soak. It records:

- `long_soak_executed_by_this_gate = false`
- `long_soak_execution_default_enabled = false`
- `long_soak_evidence_persisted = false`
- `activation_allowed = false`

This prevents the normal preflight from becoming a hidden mutation-approval or
long-running activation path.

## Activation Boundary

Before live mutation can be considered, a later operator-approved path must bind
all of these:

- explicit operator approval id
- single-surface activation scope
- fresh trusted evidence record
- current installed-binary backup after approval
- reviewed rollback plan
- fresh 24-sample pre-activation soak evidence
- persisted no-secret side-effect receipt
- post-activation watchdog evidence
- post-activation minimum 24-sample soak

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

The optional soak execution reads live readiness status only; it still does not
enable or perform live mutation.
