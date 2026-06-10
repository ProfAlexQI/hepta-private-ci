# Hepta Artifact Distribution Signing/Notarization Receipt Cancellation/Supersession Denial Gate

## Purpose

This report-only gate prevents a denied artifact distribution signing/notarization receipt ordering or monotonicity claim from becoming cancellation state, supersession state, replacement receipt state, tombstone/delete-marker state, release-publication authority, activation authority, an install affordance, or a live binary mutation.

The script and document filenames use `artifact-signing-cancel-supersession` to stay under filesystem basename limits. The gate identifiers and report fields retain the full artifact distribution signing/notarization receipt cancellation/supersession meaning.

## Source Contract

The gate consumes the artifact distribution signing/notarization receipt ordering/monotonicity denial report and requires:

- runtime `hepta` and status `ready`
- the ordering/monotonicity gate ready flag
- 18 signing receipt ordering/monotonicity surfaces
- 18 attempted and 18 denied ordering/monotonicity surfaces
- zero ordering, sequence-cursor, monotonicity-state, duplicate/stale/late/future/rollback/same-sequence/latest-wins, ordered query/export/observability/delivery/status/hash-status, release authority, activation authority, install, active-binary, Memory/KG, provider/model, credential/secret, and external-send counters
- all source side effects false
- the prior allowed next action pointing only to this cancellation/supersession report-only slice

## Denied Surfaces

The cancellation/supersession denial models 18 blocked surfaces:

1. source signing receipt ordering/monotonicity report required
2. duplicate signing receipt cancellation
3. stale package signing receipt cancellation
4. signature manifest late-arrival withdrawal
5. notarization submission future-gap cancellation
6. notarization ticket rollback supersession
7. stapling epoch-rollback tombstone
8. installer same-sequence hash replacement
9. provenance latest-wins cancellation
10. SBOM monotonic-cursor supersession
11. release asset bundle cancelled query/export
12. CDN/update-feed superseded observability
13. package registry replacement status
14. dashboard/endpoint tombstone hash/status
15. external/Telegram supersession delivery
16. release-publication authority cancellation/supersession
17. activation/live install supersession
18. install/restart/active-binary cancellation path

## Required No-Ops

Every surface keeps the following outcomes blocked or false:

- cancellation/supersession acceptance, recording, persistence, materialization, and filesystem write
- cancellation recording/persistence and withdrawal acceptance
- supersession recording/persistence
- replacement receipt acceptance, recording, and persistence
- tombstone/delete-marker recording and persistence
- latest replacement, acknowledgement replacement, query/export replacement, and observability replacement
- lifecycle cancellation/supersession persistence
- operator acceptance, operator approval, release-publication authority, and activation authority
- download link rendering, install command emission, install execution, service restart, launchd mutation, and active binary mutation
- Memory store mutation, live KG write, provider/model invocation, credential/secret reads, and external/Telegram/channel sends

## Allowed Next Action

The only allowed next action is another local report-only slice:

`prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_audit_evidence_denial_gate`

That next action is explicitly marked as not accepting cancellation, not accepting supersession, not recording replacement receipts, not recording tombstones or delete markers, not persisting lifecycle state, not recording audit evidence, not deriving authority, not installing or restarting, not mutating active binaries, not mutating Memory/KG, not invoking providers, not reading credentials, and not sending externally.

## Verification

The gate is wired into `scripts/hepta-preflight.sh` and terminal coverage inventory/diagnostic gates. It is intended to run against a temporary current-worktree service such as `127.0.0.1:17373`; it must not touch the active live `127.0.0.1:7373` service.
