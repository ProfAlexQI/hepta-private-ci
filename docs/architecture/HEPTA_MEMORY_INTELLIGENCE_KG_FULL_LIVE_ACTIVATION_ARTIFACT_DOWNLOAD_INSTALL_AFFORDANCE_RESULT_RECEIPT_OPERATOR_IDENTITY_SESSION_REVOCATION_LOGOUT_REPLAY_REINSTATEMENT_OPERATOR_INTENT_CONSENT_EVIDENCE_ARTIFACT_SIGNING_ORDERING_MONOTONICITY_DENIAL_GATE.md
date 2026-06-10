# Hepta Artifact Distribution Signing/Notarization Receipt Ordering/Monotonicity Denial Gate

## Purpose

This report-only gate prevents a denied artifact distribution signing/notarization receipt replay or idempotency claim from becoming ordered state, monotonic cursor state, latest-wins status, release-publication authority, activation authority, an install affordance, or a live binary mutation.

The script filename uses `artifact-signing-ordering-monotonicity` to stay under filesystem basename limits. The gate identifiers and report fields retain the full artifact distribution signing/notarization receipt ordering/monotonicity meaning.

## Source Contract

The gate consumes the artifact distribution signing/notarization receipt replay/idempotency denial report and requires:

- runtime `hepta` and status `ready`
- the replay/idempotency gate ready flag
- 18 signing receipt replay/idempotency surfaces
- 18 attempted and 18 denied replay/idempotency surfaces
- zero replay, duplicate, idempotency-key, idempotency-state, nonce, cross-scope reuse, status-upgrade, acknowledgement-replay, ledger/index/delivery, export/query/observability, hash/status rebind, release authority, activation authority, install, active-binary, Memory/KG, provider/model, credential/secret, and external-send counters
- all source side effects false
- the prior allowed next action pointing only to this ordering/monotonicity report-only slice

## Denied Surfaces

The ordering/monotonicity denial models 18 blocked surfaces:

1. source signing receipt replay/idempotency report required
2. duplicate signing receipt sequence
3. stale package signing receipt sequence
4. signature manifest receipt late arrival
5. notarization submission receipt future gap
6. notarization ticket timestamp rollback
7. stapling receipt epoch rollback
8. installer signing same-sequence different hash
9. provenance receipt latest-wins overwrite
10. SBOM receipt monotonic cursor
11. release asset bundle receipt ordered query/export
12. CDN/update-feed receipt ordered observability
13. package registry ordered status
14. dashboard/endpoint ordered hash/status
15. external/Telegram ordered delivery
16. release-publication authority ordering
17. activation/live install ordering
18. install/restart/active-binary ordering path

## Required No-Ops

Every surface keeps the following outcomes blocked or false:

- ordering acceptance, recording, persistence, materialization, and filesystem write
- sequence cursor recording and persistence
- monotonicity state recording, persistence, and materialization
- duplicate sequence acceptance, stale sequence acceptance, late arrival, future gap, timestamp rollback, epoch rollback, same-sequence different hash, and latest-wins overwrite
- ordered query/export, observability, delivery, status, and hash/status rebind
- operator acceptance, operator approval, release-publication authority, and activation authority
- download link rendering, install command emission, install execution, service restart, launchd mutation, and active binary mutation
- Memory store mutation, live KG write, provider/model invocation, credential/secret reads, and external/Telegram/channel sends

## Allowed Next Action

The only allowed next action is another local report-only slice:

`prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denial_gate`

That next action is explicitly marked as not accepting ordering, not recording sequence cursors, not persisting monotonicity state, not accepting cancellation or supersession, not deriving authority, not installing or restarting, not mutating active binaries, not mutating Memory/KG, not invoking providers, not reading credentials, and not sending externally.

## Verification

The gate is wired into `scripts/hepta-preflight.sh` and terminal coverage inventory/diagnostic gates. It is intended to run against a temporary current-worktree service such as `127.0.0.1:17373`; it must not touch the active live `127.0.0.1:7373` service.
