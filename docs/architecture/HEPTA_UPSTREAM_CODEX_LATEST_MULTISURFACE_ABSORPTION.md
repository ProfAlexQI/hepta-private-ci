# Hepta Upstream Codex Latest Multi-Surface Absorption Gate

`scripts/hepta-upstream-codex-latest-multisurface-absorption.sh` covers the
latest observed Codex delta from:

- baseline: `9f42c89c0112771dc29100a6f3fc904049b2655f`
- target: `8a94430bb273623be42b68f144f1ab1df343bb53`

The delta contains 12 commits and 57 changed Codex files. It is not a direct
runtime adoption. The gate classifies the delta as upstream-intake oracle
material and keeps active Hepta wiring denied.

## Covered Families

The gate requires all five families to be present:

- `doctor-thread-inventory-audit`
- `appserver-remote-status`
- `tui-markdown-status-stderr`
- `tui-config-trust-cleanup`
- `process-hardening-macos-malloc-diagnostics`

It also binds the existing diff ledger for the narrow range and requires the
expected bucket shape:

- provider/security changed files: `0`
- runtime/app-server changed files: `11`
- legacy CLI/TUI changed files: `47`
- product-governance changed files: `2`

The provider/security bucket is intentionally empty for this narrow delta; the
delta still needs runtime, compatibility, and product-governance classification.

## Denied Actions

The gate keeps all of these disabled:

- direct upstream merge, checkout, or active runtime auto-rebase
- active Hepta dependency mutation
- active binary mutation, install, or service restart
- Gateway mutation
- live doctor thread inventory query exposure
- remote-status active wiring
- TUI compatibility promotion
- launchd environment mutation for process-hardening diagnostics
- provider/model invocation
- channel delivery
- public release or GA claim
- release artifact write
- evidence persistence

## Preflight Position

`scripts/hepta-preflight.sh` runs this gate after the doctor diagnostics
absorption gate and before the broader product-governance/runtime compatibility
gates. This keeps the newer upstream head visible without letting it bypass the
existing Hepta-native promotion and denial sequence.
