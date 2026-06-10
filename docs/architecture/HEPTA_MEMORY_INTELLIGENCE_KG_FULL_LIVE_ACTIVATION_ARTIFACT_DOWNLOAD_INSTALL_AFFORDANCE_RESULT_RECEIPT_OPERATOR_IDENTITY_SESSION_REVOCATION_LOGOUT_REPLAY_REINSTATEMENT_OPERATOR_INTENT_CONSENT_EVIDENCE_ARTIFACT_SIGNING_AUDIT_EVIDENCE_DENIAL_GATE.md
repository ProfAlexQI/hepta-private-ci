# Hepta Artifact Distribution Signing/Notarization Receipt Audit/Evidence Denial Gate

## Purpose

This report-only gate prevents a denied artifact distribution signing/notarization receipt cancellation or supersession claim from becoming an audit trail, immutable evidence packet, hash-chain root, attestation, witness/notary proof, ledger/index entry, delivery evidence, export/query/observability evidence, release-publication authority, activation authority, an install affordance, or a live binary mutation.

The script and document filenames use `artifact-signing-audit-evidence` to stay under filesystem basename limits. The gate identifiers and report fields retain the full artifact distribution signing/notarization receipt audit/evidence meaning.

## Source Contract

The gate consumes the artifact distribution signing/notarization receipt cancellation/supersession denial report and requires:

- runtime `hepta` and status `ready`
- the cancellation/supersession gate ready flag
- 18 signing receipt cancellation/supersession surfaces
- 18 attempted and 18 denied cancellation/supersession surfaces
- zero cancellation, supersession, replacement receipt, tombstone/delete-marker, lifecycle persistence, release authority, activation authority, install, active-binary, Memory/KG, provider/model, credential/secret, and external-send counters
- all source side effects false
- the prior allowed next action pointing only to this audit/evidence report-only slice

## Denied Surfaces

The audit/evidence denial models 18 blocked surfaces:

1. source signing receipt cancellation/supersession report required
2. artifact signing cancellation audit trail append
3. package signing supersession immutable evidence packet
4. signature manifest withdrawal hash chain
5. notarization submission cancellation attestation
6. notarization ticket supersession witness/notary
7. stapling tombstone ledger index
8. installer replacement evidence materialization
9. provenance latest replacement immutable evidence
10. SBOM supersession evidence export
11. release asset cancelled query evidence
12. CDN/update-feed superseded observability evidence
13. package registry replacement status evidence
14. dashboard/endpoint tombstone hash/status evidence
15. external/Telegram audit evidence delivery
16. release-publication authority audit evidence
17. activation/live install audit evidence
18. install/restart/active-binary audit path

## Required No-Ops

Every surface keeps the following outcomes blocked or false:

- audit/evidence acceptance, recording, persistence, materialization, and filesystem write
- audit trail recording/persistence
- immutable evidence recording/persistence
- hash-chain and Merkle-root recording
- attestation, witness, and notary recording
- ledger and index recording/persistence
- delivery evidence, query/export evidence, observability evidence, readback evidence, status evidence, and hash-status evidence
- signing cancellation audit, package supersession immutable evidence, signature-manifest withdrawal hash-chain, notarization attestation, ticket witness/notary, stapling ledger/index, installer replacement materialization, provenance replacement evidence, SBOM export, release-asset query, CDN observability, package-registry status, and dashboard hash-status evidence
- operator acceptance, operator approval, release-publication authority, and activation authority
- download link rendering, install command emission, install execution, service restart, launchd mutation, and active binary mutation
- Memory store mutation, live KG write, provider/model invocation, credential/secret reads, and external/Telegram/channel sends

## Allowed Next Action

The only allowed next action is another local report-only slice:

`prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_retention_expiry_garbage_collection_denial_gate`

That next action is explicitly marked as not recording audit evidence, not recording immutable evidence, not recording hash chains, not recording attestations, not recording witness/notary proofs, not persisting ledgers, not accepting retention or expiry, not performing garbage collection, not deriving authority, not installing or restarting, not mutating active binaries, not mutating Memory/KG, not invoking providers, not reading credentials, and not sending externally.

## Verification

The gate is wired into `scripts/hepta-preflight.sh` and terminal coverage inventory/diagnostic gates. It is intended to run against a temporary current-worktree service such as `127.0.0.1:17373`; it must not touch the active live `127.0.0.1:7373` service.
