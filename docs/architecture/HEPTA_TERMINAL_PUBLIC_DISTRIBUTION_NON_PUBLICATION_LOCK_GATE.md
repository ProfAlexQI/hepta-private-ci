# Hepta Terminal Public Distribution Non-Publication Lock Gate

`scripts/hepta-terminal-public-distribution-non-publication-lock-gate.sh` is a schema-only terminal lock over release-artifact non-write state, public GA readiness, and the operator approval packet.

The gate exists to prove that a ready public GA matrix and a ready release-artifact boundary still do not imply public publication, public GA claim, external distribution, release artifact writes, signing, notarization, install, restart, or live mutation.

## Sources

The gate consumes exactly three source reports:

- `scripts/hepta-terminal-release-artifact-non-write-lock-gate.sh`
- `scripts/hepta-public-ga-readiness.sh`
- `scripts/hepta-public-ga-operator-approval-packet.sh`

Each source must be ready, synchronized, and side-effect-free. The public GA source may report readiness, but it must keep `public_ga_claimed=false`. The operator packet may be ready, but it remains a plan-only checklist and not a recorded approval.

## Contract

The public-distribution non-publication lock reports:

- release-artifact non-write readiness and `release_artifact_non_write_denied_by_count=87`
- public GA readiness with no GA claim
- operator packet readiness with no recorded operator approval
- native packaging public distribution artifact state still unwritten
- deterministic source hashes, lock hash, policy hash, and side-effect hash
- six ready and blocked lock families:
  - `release-artifact-non-write-source`
  - `public-ga-readiness-non-claim-boundary`
  - `operator-packet-non-approval-boundary`
  - `public-distribution-artifact-write-boundary`
  - `publication-external-send-boundary`
  - `terminal-public-distribution-lock-persistence-boundary`

## Non-Publication Boundary

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
- `public_distribution_publication_allowed`
- `public_distribution_artifact_write_allowed`
- `release_artifact_pack_execution_allowed`
- `recurring_watchdog_install_allowed`
- `live_mutation_execution_ready`
- `public_release_claim_allowed`
- `public_ga_claim_allowed`
- `release_artifact_write_allowed`
- `public_artifact_write_allowed`
- `public_release_published`
- `public_ga_claimed`
- `external_public_claim_performed`
- `external_public_distribution_performed`
- `terminal_public_distribution_non_publication_lock_recorded`
- `terminal_public_distribution_non_publication_lock_persisted`
- `terminal_public_distribution_non_publication_lock_materialized`
- `terminal_public_distribution_non_publication_lock_filesystem_written`

## Side-Effect Boundary

The gate must not:

- mutate memory, capability, plugin, runtime, gateway, launchd, or active binary state
- spawn coding agents
- invoke providers or models
- send channel messages
- fetch, merge, checkout, or rebase upstream Codex
- mutate active runtime dependencies
- build, install, sign, notarize, staple, package, or publish artifacts
- publish public release or public GA claims
- send external public distribution messages
- restart the active service or install recurring watchdogs
- execute rollback or restore
- write release, public, or distribution artifacts
- write public-distribution lock records, receipts, ledgers, or filesystem evidence
- read credentials or secret files

The output is a report-only non-publication lock. It does not approve, activate, persist, publish, build, install, sign, notarize, restart, distribute, or execute anything.
