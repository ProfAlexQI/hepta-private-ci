# Hepta Artifact Distribution Signing/Notarization Receipt Summary/Briefing Denial Gate

## Purpose

This report-only gate prevents denied artifact distribution
signing/notarization receipt export, query, and observability claims from
becoming operator summaries, briefings, readbacks, delivery/status evidence,
release-publication authority, activation authority, install affordances, or
live binary mutation.

The script and document filenames use `artifact-signing-summary-briefing` to
stay under filesystem basename limits. The gate identifiers and report fields
retain the full artifact distribution signing/notarization receipt
operator-facing summary/briefing non-persistence meaning.

## Source Contract

The gate consumes the artifact distribution signing/notarization receipt
export/query/observability denial report and requires:

- runtime `hepta` and status `ready`
- the export/query/observability gate ready flag
- 18 signing receipt export/query/observability surfaces
- 18 attempted and 18 denied observability surfaces
- zero query, export, observability, readback, authority, install,
  active-binary, Memory/KG, provider/model, credential/secret, and
  external-send counters
- all source side effects false
- the prior allowed next action pointing only to this summary/briefing
  report-only slice

## Denied Surfaces

The summary/briefing denial models 18 blocked surfaces:

1. source signing receipt export/query/observability report required
2. artifact signing retention query operator summary
3. package signing TTL query operator briefing
4. signature manifest expiry query readback digest
5. notarization search-index status banner
6. witness/notary exported summary text
7. tombstone garbage-collection export briefing card
8. replacement garbage-collection notification timeline
9. provenance archive dashboard narrative
10. SBOM compaction audit narrative
11. release asset retention readback final summary
12. CDN expiry dashboard briefing panel
13. package registry trace/event operator memo
14. dashboard hash alert/SLO approval summary
15. external/Telegram observability briefing delivery
16. release-publication authority view briefing
17. activation/live install view status briefing
18. install/restart/active-binary view status briefing

## Required No-Ops

Every surface keeps the following outcomes blocked or false:

- operator summary, briefing, readback, status banner, exported summary,
  briefing card, timeline, dashboard narrative, audit narrative, and memo
  recording
- briefing delivery, external/Telegram delivery, authority briefing, and live
  status briefing recording
- summary/briefing-derived acceptance, operator approval,
  release-publication authority, and activation authority
- download link rendering, install command emission, install execution, service
  restart, launchd mutation, and active binary mutation
- Memory store mutation, live KG write, provider/model invocation,
  credential/secret reads, public artifact writes, and external/Telegram/channel
  sends

## Allowed Next Action

The only allowed next action is another local report-only slice:

`prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_final_operator_acknowledgement_non_acceptance_denial_gate`

That next action is explicitly marked as not recording summaries, briefings,
readbacks, delivery, or acknowledgements, not deriving authority, not installing
or restarting, not mutating active binaries, not mutating Memory/KG, not
invoking providers, not reading credentials, and not sending externally.

## Verification

The gate is wired into `scripts/hepta-preflight.sh` and terminal coverage
inventory/diagnostic gates. It is intended to run against a temporary
current-worktree service such as `127.0.0.1:17373`; it must not touch the active
live `127.0.0.1:7373` service.
