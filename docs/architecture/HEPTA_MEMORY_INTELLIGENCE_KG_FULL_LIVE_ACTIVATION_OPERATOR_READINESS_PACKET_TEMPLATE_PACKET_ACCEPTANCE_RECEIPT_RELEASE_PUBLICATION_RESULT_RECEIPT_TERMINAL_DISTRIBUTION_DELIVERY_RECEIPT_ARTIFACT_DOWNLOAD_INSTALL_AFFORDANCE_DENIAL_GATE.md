# Hepta Memory/Intelligence/KG Packet Acceptance Receipt Release/Publication Result Receipt Terminal Distribution Delivery Receipt Artifact Download/Install Affordance Denial Gate

This gate extends the release/publication result receipt terminal distribution
delivery receipt artifact distribution signing/notarization surface denial
chain. It verifies that a denied signing, notarization, stapling, provenance,
SBOM, release asset, CDN/update-feed, or package-channel surface cannot be
reframed as a user-facing download or install affordance.

The gate is intentionally report-only. It consumes the artifact distribution
signing/notarization denial report and requires that source report to remain
ready while all signing, notarization, package publication, release/publication
authority, activation authority, live execution, install/restart, and
active-binary mutation counts remain zero.

## Covered Surfaces

The fixture models 18 artifact download/install affordance surfaces:

- Artifact download button.
- Direct download URL.
- Checksum copy prompt.
- Package manager install command.
- Curl-pipe-shell snippet.
- Installer launch prompt.
- Auto-update offer.
- Release-channel subscribe prompt.
- Update-feed consumer hint.
- Package registry install badge.
- CDN mirror download link.
- SBOM/provenance download link.
- Notarization ticket download link.
- Signature verification command.
- One-click install deep link.
- External/Telegram install message.
- Release/publication authority install affordance.
- Activation/live/install/restart/active-binary affordance.

Each surface is attempted in the fixture and must be denied as a no-op. The
report keeps request acceptance, acceptance, recording, persistence,
materialization, filesystem write, delivery, exposure, publication, execution,
download or install UI rendering, external/Telegram sends, authority derivation,
activation command derivation, live execution, install/restart, launchd
mutation, active-binary mutation, Memory/KG mutation, provider/model invocation,
credential/secret read, and external send at zero or false.

## Contract

The emitted report must satisfy:

- `release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_count == 18`
- `release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_attempt_count == 18`
- All artifact download/install affordance accepted, recorded, persisted,
  materialized, filesystem-written, delivered, exposed, published, and executed
  counters are zero.
- Download buttons, direct URLs, install commands, shell snippets, installer
  prompts, update offers, package badges, mirror links, SBOM/provenance links,
  notarization-ticket links, signature-verification commands, one-click install
  links, and external/Telegram install messages all remain zero.
- Release/publication authority, activation authority, activation command, live
  execution, install/restart, service restart, launchd mutation, and
  active-binary mutation counters are zero.
- All `side_effects` entries are false.

The only allowed next action is another report-only local denial slice. This
gate does not render download links, emit install commands, prompt installers,
publish update offers, send install messages, install or restart services,
mutate the active binary, write Memory/KG, invoke providers/models, or read
credentials/secrets.
