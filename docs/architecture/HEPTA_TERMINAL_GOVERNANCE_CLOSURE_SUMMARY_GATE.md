# Hepta Terminal Governance Closure Summary Gate

`scripts/hepta-terminal-governance-closure-summary-gate.sh` is the final schema-only governance closure summary over the terminal denial, release-claim denial, and operator-readiness non-approval indexes.

The gate exists to prove that the terminal governance chain is closed without granting activation, public release claims, artifact writes, rollback execution, launchd restart, or live mutation.

## Sources

The gate consumes exactly three source reports:

- `scripts/hepta-terminal-denial-index-gate.sh`
- `scripts/hepta-terminal-non-activation-release-claim-index-gate.sh`
- `scripts/hepta-terminal-operator-readiness-non-approval-index-gate.sh`

Each source must be `ready`, activation-blocking, and side-effect-free.

## Contract

The summary reports:

- terminal denial index readiness and `terminal_denied_by_count=39`
- release-claim index readiness and `release_claim_denied_by_count=47`
- operator-readiness index readiness and `operator_readiness_denied_by_count=57`
- a final governance denied-by set with `governance_closure_denied_by_count=65`
- deterministic source hashes, summary hash, policy hash, and side-effect hash
- seven ready and blocked closure families:
  - `terminal-denial-index-closure`
  - `release-claim-index-closure`
  - `operator-readiness-index-closure`
  - `operator-execution-boundary`
  - `active-binary-integrity-non-activation-boundary`
  - `terminal-governance-summary-persistence-boundary`
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
- `rollback_execution_allowed`
- `rollback_restore_allowed`
- `launchd_restart_allowed`
- `post_restore_soak_executed`
- `live_mutation_execution_ready`
- `public_release_claim_allowed`
- `public_ga_claim_allowed`
- `release_artifact_write_allowed`
- `public_artifact_write_allowed`
- `terminal_governance_closure_summary_recorded`
- `terminal_governance_closure_summary_persisted`
- `terminal_governance_closure_summary_materialized`
- `terminal_governance_closure_summary_filesystem_written`

## Side-Effect Boundary

The gate must not:

- mutate memory, capability, plugin, runtime, or gateway stores
- spawn coding agents
- invoke providers or models
- send channel messages
- fetch, merge, checkout, or rebase upstream Codex
- mutate active runtime dependencies
- restart launchd or the active service
- execute rollback
- write release or public artifacts
- write governance closure records, indexes, receipts, ledgers, or filesystem evidence
- read credentials or secret files

The output is a report-only closure summary. It does not approve, activate, persist, publish, or execute anything.
