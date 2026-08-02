# Hepta Native Robrix-main product shell closeout — 2026-08-01

> **Historical, pre-rebase evidence.** This document and its linked assets are
> retained to reproduce the 2026-08-01 run only. Its implementation/evidence
> commits are not ancestors of the current branch HEAD, so its `ready` result
> must not be used for current-source promotion. Run
> `scripts/hepta-ui-current-readiness.sh` for the only active readiness truth.

## Outcome

At the historical implementation identified below, Hepta Native used the
complete Robrix `a5a664da569c…` product shell as its default desktop/mobile
application surface. The following statements are past-tense evidence claims,
not assertions about the current worktree.

The historical Native UI source lane reported `ready`. It did not make the full
product or public release ready: no live Matrix account was used, the Hepta
runtime bridge remained disabled by default, no real iOS/Android device lab was
run, and no Developer ID signing, notarization, staple, upload, PR, or public
release was performed.

## Source and provenance

- Frozen previous UI: `cd1834c579c33431cc5086e7619d87c1501f7285`
- Pristine Robrix import: `7ac362f9690aa870591f4edcf533934af18921cb`
- Hepta implementation: `3e0031afc4ab7ddb44e29ab6545a7b1b05f681e0`
- Upstream: `project-robius/robrix` commit `a5a664da569c577ab1a3e5a33f45dcc9364954a0`
- Upstream tree: `e620da0561b6632e85eed31008f811bf94c4c24a`
- 242 upstream files were inventoried, 232 imported, and 10 policy exclusions recorded.
- The strict sync gate accounts for all 72 downstream drift paths; the upstream remote push URL is disabled.

The default product keeps Robrix's real room list, timeline, composer, desktop dock, mobile navigation stack, uploads, mentions, replies, media surfaces, settings, and Matrix SDK integration. Hepta changes are recorded in `apps/hepta-native/DOWNSTREAM_PATCHES.md` so future upstream updates can be reviewed rather than overlaid blindly.

## Product and visual changes

- Replaced the test-cockpit information architecture with Robrix's real desktop and mobile chat shell.
- Added a restrained Hepta light-glass system: stable content surfaces; glass limited to chrome, navigation, composer, and popovers.
- Removed residual purple branding, restored visible keyboard focus, normalized 44pt controls, raised muted-text contrast, and restyled popovers.
- Compacted the login surface so both 1200×720 desktop and 390×828 macOS-hosted mobile windows fit without clipping.
- Kept the diagnostic GPU-frame capture path feature-gated; it is absent from default builds.

Representative current-source-equivalent Makepad/Metal frames:

- [Desktop login, logical 1200×720](assets/hepta-native-robrix-main-2026-08-01/desktop-login-1200x720@2x.png)
- [Mobile login, requested 390×844 / host-visible 390×828](assets/hepta-native-robrix-main-2026-08-01/mobile-login-390x828@2x.png)

Only the logged-out route is visually proven. Without credentials, this run does not claim runtime screenshots for the room list, timeline, composer, settings, safe-area, or software keyboard states.

## Security boundaries

- `hepta_bridge` is a thin, side-effect-free contract/presentation layer and is forced disabled in production builds.
- Cross-session, origin, correlation, redaction, payload-size, and Unicode control checks are enforced.
- Matrix session persistence uses a generation-specific keyring master key and a ChaCha20-Poly1305 envelope, with keyed BLAKE3 binding across identity and database metadata.
- Homeservers must use HTTPS; corrupted, missing, mismatched, or tampered state fails closed.
- Legacy plaintext migration retires plaintext only after a committed secure generation.
- Logout cleanup covers canonical and legacy generations and cannot report success after incomplete credential cleanup.
- Android/OpenHarmony secure persistence remains fail-closed/re-login until a supported credential-store path is proven.

## Verification

- Rust `1.95.0`
- `cargo test --locked --lib`: 105 passed, 0 failed
- `cargo check --locked --no-default-features`: passed
- `cargo check --locked --all-features`: passed
- Default release build: passed
- `cargo packager --release --formats app`: passed after normalizing `icon.ico` to a multi-resolution RGBA-compatible asset
- Persistence tests: 24/24; Matrix state tests: 3/3; Hepta bridge tests: 14/14
- Product-shell v2 gate, strict upstream-sync v2 gate, gate self-test, shell syntax, and diff checks: passed
- Independent security review: PASS

Detailed machine-readable reports:

- [Evidence manifest](assets/hepta-native-robrix-main-2026-08-01/evidence-manifest.json)
- [Strict upstream sync](assets/hepta-native-robrix-main-2026-08-01/upstream-sync.json)
- [Source product-shell gate](assets/hepta-native-robrix-main-2026-08-01/product-shell-source-gate.json)
- [Local macOS package report](assets/hepta-native-robrix-main-2026-08-01/macos-local-package.json)

## Local macOS package

`apps/hepta-native/dist/Hepta.app` was generated from implementation commit `3e0031afc4ab…`:

- Bundle ID: `ai.hepta.nativeapp`
- Bundle icon: `Hepta.icns` (declared and present)
- Architecture: arm64
- Bundle size: 138,480 KiB
- Executable SHA-256: `2c8e16b1e5920bcf1f53986bf4044cda896907d5c58ec38cce9d3fb4b89c3445`
- The embedded About metadata resolves to the implementation commit.

The bundle is not release-valid: it has only the linker-generated ad-hoc executable signature, no Team ID, unsealed bundle resources, no notarization ticket, and no staple. `codesign --verify --deep --strict` and `spctl` therefore fail as expected. The package was not launched because the default product build intentionally ignores the isolated diagnostics data-directory override; launching could have touched an existing local Matrix session.

## Explicit remaining boundaries

- Live Matrix login, Sliding Sync, room list, timeline, upload, media, reply, mention, and composer flows are not exercised in this run.
- The live Hepta runtime/task/tool/approval adapter is not connected.
- Real iOS/Android safe-area, keyboard, VoiceOver/TalkBack, RTL, dynamic type, low-power GPU, memory, and power tests are not complete.
- Developer ID signing, notarization, staple, store/public distribution, public GA, and full-product readiness remain false.
