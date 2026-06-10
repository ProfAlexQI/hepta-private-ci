# Hepta Artifact Distribution Signing/Notarization Receipt Export/Query/Observability Denial Gate

## Purpose

This report-only gate prevents denied artifact distribution signing/notarization
receipt retention, expiry, garbage-collection, archive, or compaction claims
from becoming query results, export files, observability state, operator
readbacks, delivery evidence, release-publication authority, activation
authority, install affordances, or live binary mutation.

The script and document filenames use `artifact-signing-observability` to stay
under filesystem basename limits. The gate identifiers and report fields retain
the full artifact distribution signing/notarization receipt
export/query/observability meaning.

## Source Contract

The gate consumes the artifact distribution signing/notarization receipt
retention/expiry/garbage-collection denial report and requires:

- runtime `hepta` and status `ready`
- the retention/expiry/garbage-collection gate ready flag
- 18 signing receipt retention/expiry/garbage-collection surfaces
- 18 attempted and 18 denied lifecycle surfaces
- zero retention policy, TTL lease, expiry, garbage-collection, archive,
  compaction, evidence-retention, release authority, activation authority,
  install, active-binary, Memory/KG, provider/model, credential/secret, and
  external-send counters
- all source side effects false
- the prior allowed next action pointing only to this export/query/observability
  report-only slice

## Denied Surfaces

The export/query/observability denial models 18 blocked surfaces:

1. source signing receipt retention/expiry/garbage-collection report required
2. artifact signing audit-trail retention policy query registration
3. package signing immutable-evidence TTL lease query execution
4. signature manifest hash-chain expiry query result
5. notarization attestation retention search index
6. notarization ticket witness/notary export request
7. stapling tombstone garbage-collection export snapshot
8. installer replacement garbage-collection export file
9. provenance immutable-evidence archive export stream
10. SBOM evidence compaction observability metric/log
11. release asset cancelled-query retention readback
12. CDN observability expiry dashboard panel
13. package registry replacement-status trace/event
14. dashboard/endpoint hash-status alert/SLO
15. external/Telegram retention delivery observability
16. release-publication authority retention view
17. activation/live install garbage-collection view
18. install/restart/active-binary retention garbage-collection view

## Required No-Ops

Every surface keeps the following outcomes blocked or false:

- query registration, query execution, query result recording/persistence, and
  search-index recording/persistence
- export acceptance, export snapshot recording/persistence, export file write,
  and export stream opening
- observability metric/log/trace/event, dashboard panel, alert, SLO, ledger,
  index, and delivery evidence recording
- operator summary, readback surface, audit view, completion acknowledgement,
  and result receipt recording
- operator acceptance, operator approval, release-publication authority, and
  activation authority
- download link rendering, install command emission, install execution, service
  restart, launchd mutation, and active binary mutation
- Memory store mutation, live KG write, provider/model invocation,
  credential/secret reads, public artifact writes, and external/Telegram/channel
  sends

## Allowed Next Action

The only allowed next action is another local report-only slice:

`prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_non_persistence_denial_gate`

That next action is explicitly marked as not registering or executing queries,
not writing search indexes or exports, not recording observability, not recording
operator summaries, readbacks, audit views, or delivery evidence, not deriving
authority, not installing or restarting, not mutating active binaries, not
mutating Memory/KG, not invoking providers, not reading credentials, and not
sending externally.

## Verification

The gate is wired into `scripts/hepta-preflight.sh` and terminal coverage
inventory/diagnostic gates. It is intended to run against a temporary
current-worktree service such as `127.0.0.1:17373`; it must not touch the active
live `127.0.0.1:7373` service.
