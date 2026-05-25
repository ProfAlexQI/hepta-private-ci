# Hepta Terminal Governance Active-State Lock Gate

`scripts/hepta-terminal-governance-active-state-lock-gate.sh` is a schema-only active-state lock over the terminal governance closure summary, live watchdog, and active service dependency isolation gate.

The gate exists to prove that the terminal governance closure did not imply an install, release build, service restart, dependency mutation, upstream fetch or merge, public release claim, artifact write, rollback execution, or live mutation.

## Sources

The gate consumes exactly three source reports:

- `scripts/hepta-terminal-governance-closure-summary-gate.sh`
- `scripts/hepta-watchdog.sh`
- `scripts/hepta-active-service-dependency-isolation.sh` with `HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_LIVE=0`

Each source must be ready, side-effect-free, and compatible with the terminal non-activation boundary.

## Contract

The active-state lock reports:

- terminal governance closure readiness and `governance_closure_denied_by_count=65`
- watchdog health with `binary_sha_match=true`, `route_count=69`, and `full_fusion_complete=true`
- active dependency isolation with zero forbidden Codex engine crates in the active `hepta-cli --bin hepta` tree
- deterministic source hashes, active-state lock hash, policy hash, and side-effect hash
- six ready and blocked lock families:
  - `terminal-governance-closure-source-lock`
  - `watchdog-active-binary-integrity-lock`
  - `active-dependency-isolation-lock`
  - `install-restart-execution-boundary`
  - `active-state-lock-persistence-boundary`
  - `activation-public-claim-live-mutation-boundary`

## Non-Activation Boundary

The gate must keep all of these false:

- `readiness_allowed`
- `activation_allowed`
- `active_wiring_allowed`
- `active_runtime_auto_rebase_allowed`
- `active_runtime_codex_engine_dependency_allowed`
- `upstream_fetch_allowed`
- `upstream_merge_allowed`
- `install_execution_allowed`
- `release_build_required`
- `active_binary_mutation_allowed`
- `active_service_restart_allowed`
- `launchd_restart_allowed`
- `rollback_execution_allowed`
- `rollback_restore_allowed`
- `live_dependency_check_executed`
- `post_lock_soak_executed`
- `live_mutation_execution_ready`
- `public_release_claim_allowed`
- `public_ga_claim_allowed`
- `release_artifact_write_allowed`
- `public_artifact_write_allowed`
- `terminal_governance_active_state_lock_recorded`
- `terminal_governance_active_state_lock_persisted`
- `terminal_governance_active_state_lock_materialized`
- `terminal_governance_active_state_lock_filesystem_written`

## Side-Effect Boundary

The gate must not:

- mutate memory, capability, plugin, runtime, or gateway stores
- spawn coding agents
- invoke providers or models
- send channel messages
- fetch, merge, checkout, or rebase upstream Codex
- mutate active runtime dependencies
- build, install, mutate the active binary, restart launchd, or restart the active service
- execute rollback or restore
- run post-lock soak or live dependency checks
- write release or public artifacts
- write active-state lock records, receipts, ledgers, or filesystem evidence
- read credentials or secret files

The output is an active-state observation lock only. It does not approve, activate, persist, publish, install, restart, or execute anything.
