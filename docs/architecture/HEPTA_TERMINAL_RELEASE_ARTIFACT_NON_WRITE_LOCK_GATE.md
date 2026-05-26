# Hepta Terminal Release Artifact Non-Write Lock Gate

`scripts/hepta-terminal-release-artifact-non-write-lock-gate.sh` is a schema-only release-artifact lock over the terminal active-state lock, native packaging readiness, and release-hardening status gate.

The gate exists to prove that package readiness, hardening readiness, and a healthy active service do not imply a release build, install, signing, notarization, public distribution write, launchd mutation, public release claim, or live mutation.

## Sources

The gate consumes exactly three source reports:

- `scripts/hepta-terminal-governance-active-state-lock-gate.sh`
- `scripts/hepta-native-packaging-gate.sh`
- `scripts/hepta-release-hardening-status-gate.sh`

Each source must be ready, synchronized, and side-effect-free. The active-state source must keep the active service locked without install, restart, dependency mutation, public claim, artifact write, or live mutation.

## Contract

The release-artifact non-write lock reports:

- active-state lock readiness and `active_state_lock_denied_by_count=73`
- native packaging readiness with signing and notarization deferred
- public distribution artifact state still unwritten
- release-hardening readiness with zero live execution enabled
- deterministic source hashes, lock hash, policy hash, and side-effect hash
- six ready and blocked lock families:
  - `active-state-lock-source`
  - `native-packaging-non-distribution-boundary`
  - `release-hardening-status-non-execution-boundary`
  - `release-artifact-write-boundary`
  - `signing-notarization-launchd-boundary`
  - `terminal-release-artifact-lock-persistence-boundary`

## Non-Write Boundary

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
- `release_build_executed`
- `active_binary_mutation_allowed`
- `active_service_restart_allowed`
- `launchd_restart_allowed`
- `rollback_execution_allowed`
- `rollback_restore_allowed`
- `native_packaging_execution_allowed`
- `native_signing_allowed`
- `native_notarization_allowed`
- `native_stapling_allowed`
- `public_distribution_artifact_write_allowed`
- `release_artifact_pack_execution_allowed`
- `recurring_watchdog_install_allowed`
- `live_mutation_execution_ready`
- `public_release_claim_allowed`
- `public_ga_claim_allowed`
- `release_artifact_write_allowed`
- `public_artifact_write_allowed`
- `terminal_release_artifact_non_write_lock_recorded`
- `terminal_release_artifact_non_write_lock_persisted`
- `terminal_release_artifact_non_write_lock_materialized`
- `terminal_release_artifact_non_write_lock_filesystem_written`

## Side-Effect Boundary

The gate must not:

- mutate memory, capability, plugin, runtime, gateway, launchd, or active binary state
- spawn coding agents
- invoke providers or models
- send channel messages
- fetch, merge, checkout, or rebase upstream Codex
- mutate active runtime dependencies
- build, install, sign, notarize, staple, package, or publish artifacts
- restart the active service or install recurring watchdogs
- execute rollback or restore
- write release, public, or distribution artifacts
- write release-artifact lock records, receipts, ledgers, or filesystem evidence
- read credentials or secret files

The output is a report-only non-write lock. It does not approve, activate, persist, publish, build, install, sign, notarize, restart, or execute anything.
