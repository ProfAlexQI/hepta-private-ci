# Hepta Terminal Publication Evidence Non-Persistence Summary Gate

`scripts/hepta-terminal-publication-evidence-non-persistence-summary-gate.sh` is a schema-only terminal summary over the public-distribution non-publication lock and watchdog evidence, including a green watchdog or a parseable known operator-security attention report.

The gate exists to prove that release health evidence can be observed without becoming a recorded operator approval, public GA claim, public release claim, publication artifact, external distribution, filesystem evidence receipt, install, restart, or live mutation.

## Inputs

The gate captures and hashes two source reports:

- `scripts/hepta-terminal-public-distribution-non-publication-lock-gate.sh`
- `scripts/hepta-watchdog.sh`

Both source reports must be ready. The public-distribution source must keep publication, public claims, artifact writes, operator approval recording, and lock persistence denied. The watchdog source must either be `ok` or be a known operator-security attention report (`operator_security_status=attention`, attention budget known, Telegram production attention budget not OK, `active_owner=conflict_risk`, and `double_poller_risk=true`) while still showing active binary SHA parity, route count `>=69`, zero missing routes, `full_fusion_complete=true`, and zero remaining Phase 4/Phase 5 closure surfaces.

## Output

The summary reports:

- `publication_evidence_non_persistence_summary_ready=true`
- `publication_evidence_summary_mode=schema_only_publication_evidence_summary_not_persisted`
- `publication_evidence_summary_decision=publication_evidence_summarized_without_claim_publication_artifact_write_or_runtime_mutation`
- `source_public_distribution_denied_by_count=99`
- `source_watchdog_status_known=true`
- `source_watchdog_status=ok`, or `source_watchdog_known_operator_security_attention=true`
- `source_watchdog_route_count>=69`
- `source_watchdog_full_fusion_complete=true`
- `publication_evidence_denied_by_count=111`

It also reports six activation-blocking families:

- `public-distribution-lock-source`
- `watchdog-observational-evidence-boundary`
- `publication-evidence-non-persistence-boundary`
- `public-claim-distribution-denial-boundary`
- `operator-approval-non-recording-boundary`
- `active-runtime-mutation-boundary`

## Locked Fields

The following fields must stay `false`:

- `public_distribution_publication_allowed`
- `public_distribution_artifact_write_allowed`
- `public_release_claim_allowed`
- `public_ga_claim_allowed`
- `public_release_published`
- `public_ga_claimed`
- `external_public_claim_performed`
- `external_public_distribution_performed`
- `publication_evidence_summary_recorded`
- `publication_evidence_summary_persisted`
- `publication_evidence_summary_materialized`
- `publication_evidence_summary_filesystem_written`
- `publication_evidence_receipt_persisted`
- `publication_evidence_ledger_persisted`
- `install_execution_allowed`
- `active_service_restart_allowed`
- `live_mutation_execution_ready`

Every `side_effects` field must also remain `false`.

## Non-Goals

This gate does not write publication evidence, persist receipts, materialize ledgers, record operator approval, publish release claims, write public artifacts, sign or notarize artifacts, install binaries, restart launchd, fetch upstream, merge upstream, invoke providers, send channel messages, or enable live mutation.

Watchdog `ok` and known watchdog attention are observational evidence only. Neither state authorizes publication evidence persistence, public release, public GA, artifact creation, install, restart, active wiring, or live mutation. This is an additional terminal denial summary, not an activation path.
