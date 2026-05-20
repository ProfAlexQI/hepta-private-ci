# Hepta Native Packaging Gate

Date: 2026-05-20
Scope: local Hepta Native packaging readiness inside `hepta-codex`
Status: local package metadata/source gate ready; signing, notarization, stapling, and public distribution remain intentionally disabled

## Purpose

Public GA needs a repeatable answer for the native desktop/mobile client before
any external release claim. This gate keeps that answer local and deterministic:

- endpoint: `/api/hepta-native-packaging-gate`
- source command label: `/hepta-native-packaging-gate --json`
- script gate: `scripts/hepta-codex-native-packaging-gate.sh`
- compatibility mode: `native_app_packaging_readiness_gate`

The gate validates that the transplanted `apps/hepta-native` source and package
metadata are present and represented in the live readiness matrix. It does not
sign, notarize, staple, publish, read Apple credentials, send Telegram messages,
invoke providers, or enable native POST mutation.

## Local Evidence

The gate records the current native app package surface:

- app source path: `apps/hepta-native`
- manifest: `apps/hepta-native/Cargo.toml`
- packaging path: `apps/hepta-native/packaging`
- resource path: `apps/hepta-native/resources`
- Rust source files: `125`
- packaging/resource files: `111`
- required metadata files: `9`

Required metadata files:

- `apps/hepta-native/Cargo.toml`
- `apps/hepta-native/Cargo.lock`
- `apps/hepta-native/README.md`
- `apps/hepta-native/LICENSE-MIT`
- `apps/hepta-native/License Attributions.md`
- `apps/hepta-native/packaging/Info.plist`
- `apps/hepta-native/packaging/Entitlements.plist`
- `apps/hepta-native/packaging/HeptaNative.icns`
- `apps/hepta-native/packaging/build-macos-dmg.sh`

## Script Checks

`scripts/hepta-codex-native-packaging-gate.sh` validates:

- live endpoint status and route/source-command counters
- required native app metadata files exist
- source/resource counts match the current transplant
- `cargo metadata --manifest-path apps/hepta-native/Cargo.toml --no-deps`
- `bash -n` for the DMG helper scripts
- `plutil -lint` for macOS plist/entitlements
- optional heavy cargo gate when `HEPTA_NATIVE_PACKAGING_RUN_CARGO=1`

The optional heavy gate runs with `CARGO_TARGET_DIR` outside `hepta-codex` by
default, preserving the repo from a large generated `apps/hepta-native/target`
tree.

## Current Boundary

This gate is enough to clear the local native packaging readiness blocker in the
aggregate public GA matrix. It is not a public distribution event.

Still intentionally blocked:

- Developer ID signing
- Apple notarization
- ticket stapling
- public artifact write
- mobile store release
- external public release notes

Those remain behind explicit release-artifact and external-release approvals in
`/api/hepta-public-ga-readiness`.
