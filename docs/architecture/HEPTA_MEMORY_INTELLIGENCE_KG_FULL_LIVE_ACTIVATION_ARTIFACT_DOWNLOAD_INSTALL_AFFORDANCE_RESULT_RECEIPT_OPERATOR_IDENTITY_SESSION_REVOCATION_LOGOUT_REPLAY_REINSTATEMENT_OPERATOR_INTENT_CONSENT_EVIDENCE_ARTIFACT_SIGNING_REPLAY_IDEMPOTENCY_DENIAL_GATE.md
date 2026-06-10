# Hepta Memory/Intelligence/KG Artifact Install Receipt Operator Identity/Session Intent/Consent Evidence Artifact Signing Receipt Replay/Idempotency Denial Gate

This gate extends the operator intent/consent evidence artifact distribution
signing/notarization result receipt no-persistence chain. It verifies that a
denied signing receipt cannot be replayed, duplicated, cached as idempotent
state, rebound to status, or promoted into release/activation authority.

The gate is report-only. It consumes the artifact distribution
signing/notarization result receipt no-persistence denial report and requires
that source report to remain ready while all replay, duplicate, idempotency,
nonce, cross-scope reuse, completion acknowledgement, ledger/index/delivery,
export/query/observability, status rebind, authority, install, active-binary,
Memory/KG, provider/model, credential/secret, and external-send effects remain
zero or false.

## Covered Surfaces

The fixture models 18 artifact distribution signing/notarization receipt
replay/idempotency surfaces:

- Source signing/notarization result receipt no-persistence report requirement.
- Duplicate artifact signing receipt identity.
- Package signing receipt replay acceptance.
- Signature manifest receipt idempotency key.
- Notarization submission receipt idempotency state.
- Notarization ticket stale nonce replay.
- Stapling receipt cross-scope reuse.
- Installer signing receipt out-of-order replay.
- Provenance receipt completion acknowledgement replay.
- SBOM receipt ledger/index replay.
- Release asset and bundle receipt export/query replay.
- CDN and update-feed receipt observability replay.
- Package registry receipt status rebind.
- Dashboard/endpoint receipt hash/status replay.
- External/Telegram receipt delivery replay.
- Release-publication authority replay from signing receipt.
- Activation/live install replay from signing receipt.
- Install/restart/active-binary replay path.

Each surface is attempted in the fixture and must be denied as a no-op. The
report keeps replay, duplicate acceptance, duplicate recording/persistence,
idempotency key/state recording, idempotency persistence/materialization,
filesystem write, nonce recording, cross-scope reuse, status upgrade, completed
status, acknowledgement replay, ledger/index/delivery replay, export/query/
observability replay, hash/status rebind, external/Telegram replay,
release-publication authority, activation authority, download link, install
command, install/restart, active-binary mutation, Memory/KG mutation,
provider/model invocation, credential/secret read, and external send at zero or
false.

## Contract

The emitted report must satisfy:

- `artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count == 18`
- `artifact_distribution_signing_notarization_receipt_replay_idempotency_attempt_count == 18`
- `artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count == 18`
- All signing receipt replay, duplicate, idempotency key/state, nonce,
  cross-scope reuse, status upgrade, completed status, acknowledgement,
  ledger/index/delivery, export/query/observability, and hash/status rebind
  counters are zero.
- Artifact signing receipt replay, package signing receipt replay, signature
  manifest receipt idempotency, notarization receipt idempotency/nonce,
  stapling cross-scope reuse, installer out-of-order replay, provenance
  acknowledgement replay, SBOM ledger/index replay, release asset/bundle
  export/query replay, CDN/update-feed observability replay, package registry
  status rebind, dashboard/endpoint status replay, external receipt replay, and
  Telegram receipt replay counters are zero.
- Release-publication authority, activation authority, download link, install
  command, install/restart, service restart, launchd mutation, and active-binary
  mutation counters are zero.
- All `side_effects` entries are false.

The only allowed next action is another report-only local denial slice for
signing/notarization receipt ordering/monotonicity. This gate does not replay
signing receipts, record idempotency state, expose signing receipt status,
install or restart services, mutate the active binary, write Memory/KG, invoke
providers/models, or read credentials/secrets.
