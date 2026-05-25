# Hepta Live Mutation Pre-Activation Soak Evidence Persistence Payload Redaction Acceptance Receipt Filesystem Persistence Dry-Run Ledger Gate

Date: 2026-05-25

This gate sits after the payload redaction acceptance receipt filesystem
persistence execution-denial matrix gate. It turns the four denied filesystem
persistence execution attempts into deterministic dry-run ledger entries while
still keeping the ledger report-only.

The gate does not record, materialize, persist, or write a ledger. It does not
select an output path, invoke or execute a command, execute materialization,
execute filesystem persistence, persist a receipt, write a file, inspect payload
plaintext, run a live secret scan, send a channel message, invoke a provider or
model, mutate Gateway/runtime stores, restart launchd, execute rollback, or
enable live mutation.

## Contract

The gate consumes:

- `scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-execution-denial-matrix-gate.sh`
- the source execution-denial matrix report hash
- the source sink write preview report hash
- the source output-path evidence binding report hash
- the source output-path allowlist report hash
- the source filesystem persistence approval-packet report hash
- the source materialization dry-run report hash
- the source write-enable fixture report hash
- the source no-write sink contract report hash
- the source invocation dry-run report hash
- the source receipt command contract report hash
- the source payload redaction acceptance matrix report hash
- the source payload redaction proof report hash
- the source no-secret payload review report hash
- the source operator-scope report hash
- the source approval-packet report hash
- the source approval receipt payload hash
- the source pre-activation soak report hash
- the source persistence-denial report hash
- the minimum 24-sample pre-activation soak requirement

It requires the source execution-denial matrix to be `ready`, but keeps these
values false or zero:

- `dry_run_ledger_recorded`
- `dry_run_ledger_persisted`
- `dry_run_ledger_materialized`
- `dry_run_ledger_filesystem_written`
- `dry_run_receipt_entry_materialized_count`
- `dry_run_receipt_entry_persisted_count`
- `receipt_persistence_allowed_count`
- `receipt_persistence_execution_performed_count`
- `filesystem_persistence_execution_performed`
- `filesystem_persistence_execution_performed_count`
- `filesystem_write_performed`
- `filesystem_write_performed_count`
- `workspace_write_performed`
- `workspace_write_performed_count`
- `command_invocation_performed_count`
- `command_execution_performed_count`
- `materialization_execution_performed_count`
- `receipt_materialized_count`
- `receipt_persisted_count`
- `selected_output_path_count`
- `recorded_output_path_count`
- `recorded_path_binding_count`
- `receipt_output_path_selected`
- `receipt_output_path_recorded`
- `active_binary_sha_bound_fixture_count`
- `trusted_source_bound_fixture_count`
- `operator_scope_bound_fixture_count`
- `accepted_redaction_proof_bound_fixture_count`
- `public_claim_allowed`
- `release_artifact_write_allowed`
- `raw_payload_plaintext_recorded`
- `raw_payload_plaintext_persisted`
- `raw_payload_inspected`
- `live_secret_scan_performed`
- `receipt_persistence_enabled`
- `activation_allowed`
- `live_mutation_execution_ready`

## Dry-Run Ledger Status

- Source denial fixture count: `4`
- Required dry-run ledger entry count: `4`
- Dry-run ledger entry count: `4`
- Dry-run ledger entry hash count: `4`
- Denied dry-run ledger entry count: `4`
- Allowed dry-run ledger entry count: `0`
- Dry-run receipt entry count: `4`
- Dry-run receipt entry materialized count: `0`
- Dry-run receipt entry persisted count: `0`
- Dry-run ledger recorded: `false`
- Dry-run ledger persisted: `false`
- Dry-run ledger materialized: `false`
- Dry-run ledger filesystem written: `false`
- Receipt persistence requested count: `4`
- Receipt persistence allowed count: `0`
- Filesystem persistence execution requested count: `4`
- Filesystem persistence execution performed count: `0`
- Filesystem write requested count: `4`
- Filesystem write performed count: `0`
- Workspace write performed count: `0`
- Command invocation requested count: `4`
- Command invocation performed count: `0`
- Materialization execution requested count: `4`
- Materialization execution performed count: `0`
- Receipt materialized count: `0`
- Receipt persisted count: `0`
- Activation blocked by dry-run ledger: `true`
- Activation allowed by dry-run ledger: `false`
- Live mutation execution ready: `false`

## Dry-Run Ledger Entries

The four ledger entries are deterministic, redacted, and blocked:

- `missing-persistence-approval-id-dry-run-ledger-entry`
- `stale-pre-activation-soak-evidence-dry-run-ledger-entry`
- `workspace-path-dry-run-ledger-entry`
- `public-artifact-dry-run-ledger-entry`

Each entry references the source execution-denial matrix report hash and a
deterministic ledger-entry hash. Each entry requests a hypothetical dry-run
receipt, receipt persistence, command invocation, materialization execution, and
filesystem persistence execution shape, but keeps command invocation, command
execution, materialization execution, filesystem persistence execution,
filesystem write, workspace write, receipt materialization, receipt persistence,
activation, and live mutation all false.

The ledger explicitly denies:

- missing persistence approval id
- stale or missing fresh pre-activation soak evidence
- source-tree and workspace output-path attempts
- public claim and release-artifact write attempts
- command invocation and execution
- materialization execution
- filesystem persistence execution
- filesystem writes
- workspace writes
- receipt persistence
- live mutation execution

## Safety Boundary

The gate must not:

- write memory, skills, plugin registries, or capability registries
- invoke providers or models
- send channel messages
- mutate runtime stores or Gateway queues
- invoke or execute a receipt command
- execute materialization
- execute filesystem persistence
- select a filesystem output path
- materialize, record, persist, or write dry-run ledger entries
- write sink previews, execution-denial matrices, output-path bindings,
  output-path allowlists, approval packets, operator-scope records, payload
  reviews, redaction proofs, acceptance matrices, no-write sink records,
  write-enable fixture records, materialization plans, receipt files, evidence
  files, or release artifacts
- persist plaintext payloads
- inspect raw payload plaintext
- run live secret scans
- persist pre-activation soak evidence
- restart launchd
- execute rollback
- read credentials or secret files

The next safe step is a redaction acceptance receipt filesystem persistence
ledger shape approval gate, still without command execution, materialization
execution, filesystem persistence execution, output-path selection, ledger
materialization, receipt persistence, filesystem writes, or live mutation.
