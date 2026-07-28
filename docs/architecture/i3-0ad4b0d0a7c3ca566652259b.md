# Hepta Artifact Distribution Signing/Notarization Receipt Retention/Expiry/Garbage-Collection Denial Gate

## Purpose

This report-only gate prevents a denied artifact distribution signing/notarization receipt audit/evidence claim from becoming a retention policy, TTL lease, expiry timestamp, expiry scheduler, garbage-collection queue, archive, compaction record, release-publication authority, activation authority, an install affordance, or a live binary mutation.

The script and document filenames use `artifact-signing-retention-gc` to stay under filesystem basename limits. The gate identifiers and report fields retain the full artifact distribution signing/notarization receipt retention/expiry/garbage-collection meaning.

## Source Contract

The gate consumes the artifact distribution signing/notarization receipt audit/evidence denial report and requires:

- runtime `hepta` and status `ready`
- the audit/evidence gate ready flag
- 18 signing receipt audit/evidence surfaces
- 18 attempted and 18 denied audit/evidence surfaces
- zero audit trail, immutable evidence, hash-chain, Merkle-root, attestation, witness/notary, ledger/index, delivery, query/export, observability, status/hash-status, release authority, activation authority, install, active-binary, Memory/KG, provider/model, credential/secret, and external-send counters
- all source side effects false
- the prior allowed next action pointing only to this retention/expiry/garbage-collection report-only slice

## Denied Surfaces

The retention/expiry/garbage-collection denial models 18 blocked surfaces:

1. source signing receipt audit/evidence report required
2. artifact signing audit-trail retention policy
3. package signing immutable-evidence TTL lease
4. signature manifest hash-chain expiry timestamp
5. notarization submission attestation retention ledger
6. notarization ticket witness/notary expiry scheduler
7. stapling tombstone garbage-collection queue
8. installer replacement evidence garbage-collection scan
9. provenance immutable-evidence archive
10. SBOM evidence compaction
11. release asset cancelled-query retention
12. CDN observability expiry acknowledgement
13. package registry replacement-status garbage-collection decision
14. dashboard/endpoint hash-status retention
15. external/Telegram retention delivery
16. release-publication authority retention
17. activation/live install garbage-collection evidence
18. install/restart/active-binary retention garbage-collection path

## Required No-Ops

Every surface keeps the following outcomes blocked or false:

- retention/expiry/garbage-collection acceptance, recording, persistence, materialization, and filesystem write
- retention policy recording/persistence and TTL lease recording/persistence
- expiry timestamp, scheduler, timer, acknowledgement, and state persistence
- garbage-collection queue, scan, candidate, decision, and state persistence
- tombstone/delete-marker garbage collection
- archive and compaction recording
- audit evidence, immutable evidence, hash/attestation, witness/notary, ledger/index, delivery evidence, and status evidence retention
- operator acceptance, operator approval, release-publication authority, and activation authority
- download link rendering, install command emission, install execution, service restart, launchd mutation, and active binary mutation
- Memory store mutation, live KG write, provider/model invocation, credential/secret reads, and external/Telegram/channel sends

## Allowed Next Action

The only allowed next action is another local report-only slice:

`prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_export_query_observability_denial_gate`

That next action is explicitly marked as not recording retention, not recording expiry, not recording garbage collection, not archiving, not compacting, not registering export/query, not recording observability, not deriving authority, not installing or restarting, not mutating active binaries, not mutating Memory/KG, not invoking providers, not reading credentials, and not sending externally.

## Verification

The gate is wired into `scripts/hepta-preflight.sh` and terminal coverage inventory/diagnostic gates. It is intended to run against a temporary current-worktree service such as `127.0.0.1:17373`; it must not touch the active live `127.0.0.1:7373` service.
