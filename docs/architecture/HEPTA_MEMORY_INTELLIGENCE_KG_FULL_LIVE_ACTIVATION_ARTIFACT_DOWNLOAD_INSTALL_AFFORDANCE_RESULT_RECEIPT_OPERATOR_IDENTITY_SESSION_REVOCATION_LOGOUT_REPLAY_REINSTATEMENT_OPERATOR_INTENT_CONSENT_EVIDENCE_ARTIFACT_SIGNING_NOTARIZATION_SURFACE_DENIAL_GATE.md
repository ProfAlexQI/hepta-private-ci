# Hepta Memory/Intelligence/KG Artifact Install Receipt Operator Identity/Session Intent/Consent Evidence Artifact Signing/Notarization Surface Denial Gate

This gate extends the operator intent/consent evidence distribution
artifact/manifest status denial chain. It verifies that a denied artifact or
manifest status cannot be reframed as artifact signing, package signing,
signature manifest writing, notarization, stapling, provenance/SBOM
publication, package-channel publication, release authority, activation
authority, or live install status.

The gate is report-only. It consumes the operator intent/consent evidence
distribution artifact/manifest status denial report and requires that source
report to remain ready while all artifact signing, package signing, signature
manifest, notarization, stapling, provenance, SBOM, release-asset packaging,
CDN/update-feed write, package registry publication, authority, install,
active-binary, Memory/KG, provider/model, credential/secret, and external-send
effects remain zero or false.

## Covered Surfaces

The fixture models 18 artifact distribution signing/notarization surfaces:

- Source distribution artifact/manifest status report requirement.
- Artifact signing execution.
- Package signing execution.
- Signature manifest write and checksum binding.
- Notarization submission.
- Notarization ticket recording.
- Stapling execution.
- Installer signing execution.
- Provenance attestation publication.
- SBOM manifest publication.
- Release asset and artifact bundle packaging.
- CDN and update-feed artifact write.
- Package registry artifact publication.
- Dashboard/endpoint/query/export/observability signing status exposure.
- External/Telegram package-channel publication.
- Release-publication authority signing status.
- Activation/live install signing status.
- Install/restart/active-binary signing path.

Each surface is attempted in the fixture and must be denied as a no-op. The
report keeps acceptance, recording, persistence, materialization, filesystem
write, delivery, exposure, execution, signing, notarization, stapling,
provenance/SBOM publication, packaging, CDN/update-feed write, package-registry
publication, Telegram publication, public release/GA claim, operator
acceptance, operator approval, release-publication authority, activation
authority, install/restart, active-binary mutation, Memory/KG mutation,
provider/model invocation, credential/secret read, and external send at zero or
false.

## Contract

The emitted report must satisfy:

- `artifact_distribution_signing_notarization_surface_count == 18`
- `artifact_distribution_signing_notarization_surface_attempt_count == 18`
- `artifact_distribution_signing_notarization_surface_denied_count == 18`
- All artifact signing/notarization accepted, recorded, persisted,
  materialized, filesystem-written, delivered, exposed, and executed counters
  are zero.
- Artifact signing, package signing, signature manifest write, checksum
  binding, notarization submission/ticket, stapling, installer signing,
  provenance/SBOM publication, release asset/bundle packaging, CDN/update-feed
  write, package registry publication, dashboard/endpoint/query/export/
  observability exposure, external, and Telegram publication counters are zero.
- Release-publication authority, activation authority, install/restart, service
  restart, launchd mutation, and active-binary mutation counters are zero.
- All `side_effects` entries are false.

The only allowed next action is another report-only local denial slice for the
signing/notarization result receipt. This gate does not record acceptance,
persist signing evidence, publish artifacts, expose signing status externally,
install or restart services, mutate the active binary, write Memory/KG, invoke
providers/models, or read credentials/secrets.
