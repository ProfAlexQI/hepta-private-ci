# Hepta Native

Hepta Native is the Hepta-owned desktop/mobile client built on the absorbed
Robrix Matrix-heart baseline. It keeps the proven Matrix SDK room list,
timeline, composer, Sliding Sync, and Makepad/Robius cross-platform shell, then
layers Hepta runtime collaboration surfaces on top.

Current Hepta surfaces include runtime status, task/tool/approval previews,
action outbox, exact payload inspection, context chips, quick commands, mobile
safety bars, productization status, and packaging gates.

Source baseline: `project-robius/robrix @ b2bb6cf` under MIT. Robrix-derived
portions remain attributed in license/copyright notices; product identity,
packaging metadata, visible app copy, and release commands now target
**Hepta Native**.

## Product posture

- Matrix-heart absorption: complete.
- Desktop cockpit: present.
- Mobile cockpit/detail surfaces: present.
- Live mutation classes: intentionally local-only / dry-run until exact payload
  confirmation, bridge policy gates, and readback evidence are enabled.
- Android packaging smoke: passed with Java-safe package name
  `ai.hepta.nativeapp`.
- iOS packaging gate: pending the matching Xcode iOS 26.5 simulator runtime.

The UI status/productization panes are reporting surfaces only. They must not run
installs, `adb`, simulators, signing, Gateway calls, Matrix sends, approvals,
tool execution, or task-registry writes.

## Local development gates

Run these from the Hepta repository root:

```sh
cargo check --manifest-path apps/hepta-native/Cargo.toml
cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture
cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed -- --nocapture
./scripts/hepta-control-ui-smoke.sh
git diff --check
```

Run fixture mode without requiring a homeserver login:

```sh
HEPTA_NATIVE_FIXTURE_MODE=1 cargo run --manifest-path apps/hepta-native/Cargo.toml
```

## Mobile packaging commands

Install the Makepad packaging CLI:

```sh
cargo install --force --git https://github.com/makepad/makepad.git --branch dev cargo-makepad
```

Android uses `ai.hepta.nativeapp`; do **not** use `ai.hepta.native` for Android
Java sources because `native` is reserved.

```sh
cargo makepad android \
  --abi=aarch64 \
  --package-name=ai.hepta.nativeapp \
  --app-label='Hepta Native' \
  --sdk-path=/Users/qianqi/.openclaw/workspace/Hepta/android_33_sdk \
  build -p hepta-native --release
```

iOS uses the Hepta app identifier surface:

```sh
xcodebuild -downloadPlatform iOS
xcrun simctl list runtimes
cargo makepad apple ios \
  --org=ai.hepta \
  --app=hepta-native \
  build -p hepta-native --release
```

If Xcode reports an unavailable simulator runtime, keep the iOS packaging gate
`Pending/Gated` until `xcrun simctl list runtimes` shows the matching runtime and
the `cargo makepad apple ios ... build` command passes.

## Desktop packaging

Hepta Native uses Cargo Packager metadata in `Cargo.toml` and a dedicated macOS
DMG helper:

```sh
cd apps/hepta-native
APPLE_ID=… APPLE_PASSWORD=… APPLE_TEAM_ID=… ./packaging/build-macos-dmg.sh
```

Expected product names:

- App bundle: `Hepta Native.app`
- Binary: `hepta-native`
- Bundle/package id: `ai.hepta.nativeapp`
- DMG background: `packaging/Hepta Native macOS dmg background.png`

If a mounted DMG blocks rebuilds, unmount the stale `Hepta Native` volume before
retrying. If macOS denies DMG/App bundle access, grant App Management permission
to the terminal application used for packaging.

## Hepta runtime safety boundary

The Hepta runtime bridge is deliberately staged:

- read-only status/query previews are allowed;
- draft commands may become local Matrix-shaped preview events;
- approvals/tool calls/tasks require exact payload inspection and confirmation;
- external mutation remains policy-blocked until a later phase provides readback
  evidence and explicit operator enablement.

This keeps the Robrix-derived Matrix timeline stable while Hepta productization
continues without accidental live side effects.

## Third-party basis and attribution

Hepta Native currently relies on:

- Robrix / Project Robius Matrix-heart baseline: <https://github.com/project-robius/robrix>
- Makepad UI toolkit: <https://github.com/makepad/makepad>
- Robius platform crates: <https://github.com/project-robius/robius>
- Matrix Rust SDK fork used by the baseline: <https://github.com/project-robius/matrix-rust-sdk>
- Ruma: <https://github.com/ruma/ruma>

See also:

- `docs/architecture/third_party/ROBRIX_COPY_MANIFEST_2026-05-14.md`
- `docs/architecture/third_party/ROBRIX_NOTICE_2026-05-14.md`
- `LICENSE-MIT`
- `License Attributions.md`
