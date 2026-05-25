# Hepta Terminal Denial Index Gate

This document defines the report-only terminal denial index for Hepta live-mutation and upstream Codex activation safety.

## Purpose

The gate summarizes three already-bounded sources:

- `scripts/hepta-readiness-denial-review-acceptance-closure-summary-gate.sh`
- `scripts/hepta-upstream-codex-activation-readiness-closure.sh`
- `scripts/hepta-upstream-codex-sync-lane.sh`

The index is intentionally terminal and non-authorizing. It confirms that the live-mutation denial chain, upstream activation readiness closure, and upstream sync lane are all ready as audit contracts while activation, active wiring, upstream merge, public release claims, release artifact writes, and live mutation remain denied.

## Ready Criteria

The gate is ready only when all of the following are true:

- The readiness denial review acceptance closure summary is ready, terminal, schema-only, and activation-blocking.
- The upstream Codex activation readiness closure is ready and denies activation by default.
- Operator-approved activation is not ready.
- Active wiring is not allowed.
- All upstream active decisions are false.
- The upstream Codex sync lane is ready, classify-only, and does not fetch, merge, claim latest upstream, or auto-rebase active runtime code.
- The active `hepta-cli` dependency tree remains isolated from tracked Codex engine crates.
- The terminal index records three ready sources and six activation-blocking terminal families.
- The terminal denial list has 39 entries: four index-local denials, 23 inherited live-mutation summary denials, seven activation decision denials, and five sync-lane required next steps.
- All side-effect flags remain false.

## Boundary

This gate does not:

- fetch or merge upstream Codex
- write to the workspace as a runtime action
- restart launchd or active services
- write release artifacts or public claims
- persist terminal index records
- persist receipts, ledgers, review acceptances, scoreboards, readiness decisions, or denial summaries
- read credentials or secret files
- invoke providers or models
- send channel messages
- execute live mutation

## Contract

Successful execution emits `terminal_denial_index_v1` JSON with:

- `terminal_denial_index_ready=true`
- `terminal_index_mode=schema_only_terminal_index_activation_blocked`
- `terminal_index_decision=activation_and_sync_denial_indexed_without_activation`
- `required_source_count=3`
- `ready_source_count=3`
- `activation_blocking_source_count=3`
- `source_summary_denied_by_count=23`
- `source_activation_denied_decision_count=7`
- `source_sync_required_next_step_count=5`
- `source_sync_forbidden_codex_engine_crate_count=0`
- `terminal_denied_by_count=39`
- `activation_allowed=false`
- `active_wiring_allowed=false`
- `upstream_fetch_allowed=false`
- `upstream_merge_allowed=false`
- `live_mutation_execution_ready=false`
- every `side_effects.*` value set to `false`

Readiness means the denial index is closed as an audit contract. It does not authorize activation.
