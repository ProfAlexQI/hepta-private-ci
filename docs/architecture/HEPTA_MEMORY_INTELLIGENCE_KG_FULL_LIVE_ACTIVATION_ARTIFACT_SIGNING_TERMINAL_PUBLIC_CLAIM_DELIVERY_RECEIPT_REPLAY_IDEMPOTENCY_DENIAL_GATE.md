# Hepta Memory/Intelligence/KG Artifact Signing Terminal Public Claim Delivery Receipt Replay/Idempotency Denial Gate

This gate follows the artifact signing terminal public claim delivery receipt
non-persistence denial gate. It proves that a non-persistent delivery receipt
cannot be replayed, deduplicated into a durable receipt, accepted through an
idempotency key, rebound to status, or used to derive authority.

The gate is report-only. It consumes the delivery receipt non-persistence denial
report and requires the source report to keep receipt recording, persistence,
materialization, filesystem, ledger, index, query, export, observability, status,
acknowledgement, authority, install, active-binary, Memory/KG, provider/model,
credential/secret, and external-send effects at zero or false.

## Covered Surfaces

The fixture models 18 replay/idempotency attempts:

- Source delivery receipt non-persistence report requirement.
- Duplicate delivery receipt identity.
- Delivery receipt replay acceptance.
- Delivery receipt idempotency key.
- Delivery receipt idempotency state.
- Stale nonce replay.
- Cross-scope reuse.
- Status upgrade.
- Completed status replay.
- Acknowledgement replay.
- Ledger/index replay.
- Query/export/observability replay.
- Hash/status rebind.
- Readback receipt backfill replay.
- External/Telegram delivery receipt replay.
- Release-publication authority from delivery receipt replay.
- Activation/live install from delivery receipt replay.
- Install/restart/active-binary replay from delivery receipt.

Every surface is attempted in the fixture and must be denied as a no-op. The
report keeps replay acceptance, duplicate acceptance, idempotency key/state,
nonce, cross-scope reuse, status upgrade, acknowledgement replay, ledger/index,
query/export/observability, hash rebind, external/Telegram replay,
release-publication authority, activation authority, install/restart,
active-binary mutation, Memory/KG mutation, provider/model invocation,
credential/secret read, and external send at zero or false.

## Contract

The emitted report must satisfy:

- `artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_surface_count == 18`
- `artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_attempt_count == 18`
- `artifact_signing_terminal_public_claim_delivery_receipt_replay_idempotency_denied_count == 18`
- Replay, duplicate, idempotency key/state, nonce, cross-scope, status-upgrade,
  acknowledgement, ledger/index, query/export/observability, and hash-rebind
  counters are zero.
- Replay-derived operator approval, release-publication authority, activation
  authority, download link, install command, install/restart, and active-binary
  counters are zero.
- All `side_effects` entries are false.

The only allowed next action is another report-only local denial slice for
delivery receipt ordering/monotonicity. This gate does not record or replay a
delivery receipt, accept an idempotency key, persist idempotency state, expose
receipt status, install or restart services, mutate the active binary, write
Memory/KG, invoke providers/models, read credentials/secrets, or send external
messages.

Script:
`scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-replay-idempotency-denial-gate.sh`
