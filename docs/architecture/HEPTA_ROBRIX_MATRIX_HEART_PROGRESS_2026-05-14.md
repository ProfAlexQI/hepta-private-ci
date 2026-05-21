# Hepta Robrix/Matrix-heart UI Progress

Date: 2026-05-14
Branch: `ui/robrix-matrix-heart-20260514`

## Current milestone

M1: Hepta-branded Robrix fork boots with Matrix heart intact.

## Completed in this pass

- Created implementation branch: `ui/robrix-matrix-heart-20260514`.
- Added Matrix-heart development plan:
  - `docs/architecture/HEPTA_ROBRIX_DESKTOP_MOBILE_UI_DEVELOPMENT_2026-05-14.md`
- Directly copied Robrix baseline into:
  - `apps/hepta-native/`
- Preserved source attribution:
  - source repo: `project-robius/robrix`
  - source commit: `b2bb6cf`
  - license: MIT
- Added third-party records:
  - `docs/architecture/third_party/ROBRIX_COPY_MANIFEST_2026-05-14.md`
  - `docs/architecture/third_party/ROBRIX_NOTICE_2026-05-14.md`
- Excluded `apps/hepta-native` from the root Hepta workspace for now so existing Hepta workspace gates remain isolated.
- Renamed initial app/package identity:
  - Cargo package: `hepta-native`
  - Makepad bundle identifier: `ai.hepta.native`
  - project directory constants: `ai.hepta.hepta-native`
  - initial visible labels: `Hepta Native`
- Added first Hepta Matrix-style event helper module:
  - `apps/hepta-native/src/hepta_event.rs`
- Added first low-risk renderer hook:
  - `event_preview.rs` now labels known `m.hepta.*` custom message-like events as Hepta event previews instead of generic unknown events.

## Checks started

- `cargo check --manifest-path apps/hepta-native/Cargo.toml` is running; it is currently downloading/building Robrix/Matrix/Makepad dependency graph.
- `cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed -- --nocapture` is running for existing Control UI regression.
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_event -- --nocapture` is queued/running behind Cargo package-cache locks.

## Next implementation steps

1. Wait for the native cargo check/test results and fix first compile blockers.
2. Add fixture/local mode for Hepta Native if Matrix login blocks UI boot.
3. Add custom event renderers beyond preview labels:
   - `m.hepta.runtime_event`
   - `m.hepta.tool_call`
   - `m.hepta.tool_result`
   - `m.hepta.approval_request`
   - `m.hepta.task`
4. Add first Hepta Matrix bridge fixture that injects a static Hepta runtime conversation into the Robrix timeline.
5. Keep existing `hepta-core` Control UI gates green throughout.

## M1 verification update

Completed after the first Matrix-heart transplant patch:

- `cargo check --manifest-path apps/hepta-native/Cargo.toml` ✅
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` ✅
  - 5 Hepta custom/fixture tests passed
- `cargo check -q -p hepta-core -p hepta-cli` ✅
- `git diff --check` ✅

Additional implementation progress:

- Added `.gitattributes` to preserve upstream Robrix whitespace inside `apps/hepta-native/**` while keeping normal Hepta diff hygiene elsewhere.
- Added `HEPTA_NATIVE_FIXTURE_MODE=1` boot path: Hepta Native can skip Matrix SDK startup and show the desktop/mobile shell for local UI development.
- Added visible welcome/cockpit copy for the Matrix-heart route, listing the first-class `m.hepta.*` event families.

## M2 custom event renderer update

Completed the first real Hepta timeline renderer pass:

- `m.hepta.*` custom message-like events now route out of Robrix's generic `OtherMessageLike` small-state path and into a first-class `HeptaEventCard` timeline widget.
- `HeptaEventEnvelope` now validates `hepta.event.v1` content, derives display text, status badges, titles, bodies, and redaction-aware metadata.
- The local fixture now emits Matrix-shaped custom event JSON (`type`, `room_id`, `event_id`, `sender`, `origin_server_ts`, `content`) for the Hepta Runtime Cockpit path.
- This keeps the Matrix heart intact while making runtime/task/tool/approval/memory events visibly Hepta-native in the timeline.

Verification:

- `cargo check --manifest-path apps/hepta-native/Cargo.toml` ✅
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` ✅
  - 8 Hepta event/fixture tests passed
- `cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed -- --nocapture` ✅
- `git diff --check` ✅

## M2 local cockpit screen update

Completed a visible local fixture cockpit for desktop/mobile shell development:

- Added `home/hepta_fixture_cockpit.rs` with a reusable `HeptaFixtureCockpit` widget.
- The welcome screen now embeds the fixture cockpit and renders seven sample Matrix-shaped `m.hepta.*` events as Hepta mini-cards.
- This gives the native app a no-homeserver UI surface for runtime/tool/result/approval/task/agent/memory cards while Hepta's own native OpenClaw-parity runtime is still being implemented.

Verification:

- `cargo check --manifest-path apps/hepta-native/Cargo.toml` ✅
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` ✅
- `git diff --check` ✅

## M2 approval action bridge guardrail

Added the first approval action affordance without sending external mutations yet:

- `HeptaEventCard` now shows Approve / Reject buttons for `m.hepta.approval_request` envelopes.
- Button clicks are handled inside `RoomScreen` and produce a local warning popup only.
- This proves the UI action path and preserves the safety boundary until Hepta's native execution adapters, policy gates, persistence, and readback evidence are explicitly wired.

Verification:

- `cargo check --manifest-path apps/hepta-native/Cargo.toml` ✅
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` ✅
- `cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed -- --nocapture` ✅
- `git diff --check` ✅

## M2 composer action bridge guardrail

Added the first explicit composer bridge command path:

- Added `hepta_composer.rs` with a bounded parser for `/hepta ...` commands.
- Supported local commands now include:
  - `/hepta task <summary>`
  - `/hepta tool <name> [args...]`
  - `/hepta approve <target>`
  - `/hepta reject <target>`
- `RoomInputBar` intercepts only explicit `/hepta` commands, stages a local popup, clears the composer, and does **not** send Matrix messages or perform external mutations yet.
- Normal Matrix messages and non-Hepta slash commands still follow the original Robrix send path.

Verification:

- `cargo check --manifest-path apps/hepta-native/Cargo.toml` ✅
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` ✅
  - 11 Hepta parser/event/fixture tests passed
- `cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed -- --nocapture` ✅
- `git diff --check` ✅

## M2 Hepta workspace chrome pass

Started converting the Robrix shell language from generic Matrix rooms toward the Hepta cockpit/workspace model while keeping the Matrix-heart underneath:

- Navigation home tooltip now reads `Agent Cockpit`.
- Add/join tooltip now reads `Connect Workspace`.
- Rooms-list header defaults to `Agent Cockpit`.
- Add-room screen now explains that Matrix aliases/IDs/links are the transport surface for Hepta bridge workspaces.
- Composer placeholder now advertises the staged `/hepta task|tool|approve|reject ...` command path.

Verification:

- `cargo check --manifest-path apps/hepta-native/Cargo.toml` ✅
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` ✅
  - 11 Hepta parser/event/fixture tests passed
- `cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed -- --nocapture` ✅
- `git diff --check` ✅

## M3 native runtime event shape seam

Added a local, side-effect-free Hepta-runtime-to-Matrix event shape seam for the native OpenClaw-parity runtime:

- New `hepta_bridge.rs` converts Hepta runtime inputs into Matrix-shaped `m.hepta.*` custom timeline events.
- Unknown Hepta event kinds are rejected before timeline injection.
- Generated bridge events preserve Matrix event fields: `type`, `room_id`, `event_id`, `sender`, `origin_server_ts`, and typed Hepta `content`.
- The event seam does not talk to the OpenClaw Gateway, does not send Matrix events, and does not mutate external state yet; it is capability-replication scaffolding, not Gateway integration.

Verification:

- `cargo check --manifest-path apps/hepta-native/Cargo.toml` ✅
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` ✅
  - 14 Hepta bridge/parser/event/fixture tests passed
- `cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed -- --nocapture` ✅
- `git diff --check` ✅

## M3 no-homeserver fixture workspace injection

The local fixture path now enters the real Robrix room-list model instead of only rendering a static welcome preview:

- `HEPTA_NATIVE_FIXTURE_MODE` still bypasses Matrix SDK startup.
- Startup now enqueues a local `Hepta Runtime Cockpit` joined workspace into `RoomsListUpdate`.
- The fixture workspace has a valid Matrix room id (`!hepta-runtime-fixture:local`), avatar, latest event preview, unread state, and loaded status.
- This makes the fixture cockpit selectable through the existing desktop/mobile Matrix-heart shell while runtime wiring remains local and side-effect-free.

Verification:

- `cargo check --manifest-path apps/hepta-native/Cargo.toml` ✅
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` ✅
  - 15 Hepta bridge/parser/event/fixture tests passed
- `cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed -- --nocapture` ✅
- `git diff --check` ✅

## M3 selectable fixture timeline surface

The no-homeserver fixture workspace now has a real room-screen surface:

- Selecting `Hepta Runtime Cockpit` switches `RoomScreen` into a local fixture timeline surface.
- The fixture surface renders `HeptaFixtureCockpit` inside the normal room screen shell, keeping the composer/action bar visible.
- The Matrix timeline widget is hidden only for the fixture workspace; normal Matrix rooms continue through the untouched Robrix timeline path.
- Fixture hide/drop paths avoid Matrix subscription/read-receipt side effects.

Verification:

- `cargo check --manifest-path apps/hepta-native/Cargo.toml` ✅
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` ✅
  - 15 Hepta bridge/parser/event/fixture tests passed
- `cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed -- --nocapture` ✅
- `git diff --check` ✅

## M3 workspace loading language pass

Continued replacing user-visible generic Matrix room language with Hepta workspace language:

- Rooms list loading label now says `Loading workspaces...`.
- Homeserver wait status now says `Loading workspaces (waiting for Matrix transport)...`.
- Space lobby loading state now says `Loading bridge workspaces...`.

Verification:

- `cargo check --manifest-path apps/hepta-native/Cargo.toml` ✅
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` ✅
  - 15 Hepta bridge/parser/event/fixture tests passed
- `cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed -- --nocapture` ✅
- `git diff --check` ✅

## M4-M6 desktop/mobile cockpit completion pass

Continued from the Matrix-heart fast path into the shared desktop/mobile Hepta cockpit surfaces:

- Desktop Inspector and mobile detail now include shared read-only panes for runtime status, context chips, quick command templates, action outbox, action drill-down, and mobile packaging gates.
- `hepta_action_queue` now exposes `HeptaActionDetail`, selected-action drill-down, compact exact payload preview, target display, confirmation copy, execution guard, result readback note, and redacted evidence note.
- `HeptaActionDetailPane` renders the same selected action evidence on desktop and mobile, so staged task/tool/approval lanes can be inspected before any controlled mutation is ever enabled.
- `hepta_mobile_packaging` records Phase 6 packaging gate status as a local model only; it never shells out, installs toolchains, runs `adb`, starts a simulator, signs packages, sends Matrix events, calls Gateway, or mutates task/tool/approval state.
- Root `.gitignore` now excludes `/android_33_sdk/` so generated SDK/NDK/JDK artifacts are not tracked.
- Android package command changed to `ai.hepta.nativeapp` because `ai.hepta.native` creates invalid Java package sources (`native` is a reserved keyword).

Packaging evidence captured in UI/docs:

- `cargo-makepad v1.0.0` installed from Makepad `dev` branch.
- Android SDK/NDK/JDK local workaround materialized under `/Users/qianqi/.openclaw/workspace/hepta-codex/android_33_sdk`.
- Android APK build smoke passed and produced `apps/hepta-native/target/android/makepad-android-apk/hepta_native/apk/heptanative.apk`.
- iOS release build reached the asset/runtime phase; current host blocker is `No simulator runtime version from ["23C54"] available to use with iphonesimulator SDK version 23F73`.

Verification:

- `rustfmt --edition 2024 --check apps/hepta-native/src/hepta_action_queue.rs apps/hepta-native/src/hepta_mobile_packaging.rs apps/hepta-native/src/hepta_runtime_status.rs apps/hepta-native/src/home/hepta_action_detail.rs apps/hepta-native/src/home/hepta_inspector.rs apps/hepta-native/src/home/hepta_mobile_detail.rs apps/hepta-native/src/home/hepta_mobile_packaging.rs` ✅ with known stable-only rustfmt config warnings
- `cargo check --manifest-path apps/hepta-native/Cargo.toml` ✅
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` ✅
  - 40 Hepta tests passed after action detail + packaging gate status

## M7 Hepta-native product shell closure

Closed the first productization shell pass on top of the fully absorbed Robrix Matrix-heart baseline:

- Added `hepta_productization` model + desktop/mobile pane so the app reports the difference between Matrix-heart absorption, Hepta cockpit completion, branding, runtime bridge policy, mobile release gates, and release-candidate evidence.
- Wired `mod.widgets.HeptaProductizationPane` into both the desktop inspector and mobile detail surface.
- Productized packaging metadata and app assets:
  - app name: `Hepta Native`
  - Android package: `ai.hepta.nativeapp`
  - bundle/app id: `ai.hepta.nativeapp`
  - package id/binary: `hepta-native`
  - desktop template: `packaging/hepta-native.desktop`
  - icon assets regenerated from the Hepta logo, including `HeptaNative.icns`, `icon.ico`, PNG app icons, and Google Play 512 icon.
  - macOS DMG background switched to `Hepta Native macOS dmg background.png`.
- Updated macOS packaging scripts to expect `Hepta Native.app` and the `hepta-native` binary rather than `Robrix.app` / `robrix`.
- Cleaned remaining user-visible restart/auth comments that still said Robrix in the TSP/iOS SSO paths; Robrix references that remain are attribution, implementation lineage, internal widget names, or intentionally preserved Matrix-heart substrate notes.
- Initially corrected the iOS packaging gate to stay `Pending` while the Xcode `iOS 26.5 Simulator (23F77)` runtime download was still in progress; later verification completed the runtime install and release simulator build smoke.

Commits:

- `c613315 feat: productize Hepta native shell`
- `3cbdc07 chore: finish Hepta native visible naming`

Verification:

- `git diff --check` ✅
- `bash -n apps/hepta-native/packaging/build-macos-dmg.sh apps/hepta-native/packaging/fix-dmg-applications-icon.sh` ✅
- `cargo check --manifest-path apps/hepta-native/Cargo.toml` ✅
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` ✅
  - 43 Hepta tests passed after the productization pane and packaging updates
- `cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed -- --nocapture` ✅
- `cargo check -q -p hepta` ✅
- `./scripts/hepta-control-ui-smoke.sh` ✅
  - `Hepta Control UI hardening smoke passed (Rust-native retired Node suite)`
  - `Hepta Control UI Rust/no-JS contract smoke passed`

Mobile release packaging follow-up closure:

- Xcode `iOS 26.5 Simulator (23F77)` runtime download finished and installed successfully.
- `cargo makepad apple ios --org=ai.hepta --app=hepta-native build -p hepta-native --release` passed from `apps/hepta-native`, so the local iOS simulator build smoke is now complete.
- The root-level invocation still fails because `hepta-native` is not a root workspace package; the validated invocation is from `apps/hepta-native`.

M7 follow-up polish:

- `1a87bea style: align Hepta native accent colors` switched the carried Matrix-heart color slots to Hepta primary/secondary accents while keeping legacy internal constant names stable.
- `bc3390b chore: polish Hepta native user-facing copy` replaced remaining product-facing Matrix/Robrix copy in permission strings, welcome/cockpit panes, mobile safety/detail text, and workspace attach wording with Hepta-first language while preserving Matrix as the explicit transport/substrate where needed.

Additional verification:

- `git diff --check` ✅
- `cargo check --manifest-path apps/hepta-native/Cargo.toml` ✅
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` ✅
  - 43 Hepta tests passed after the follow-up copy/theme polish

### 2026-05-14 productization cleanup addendum

- Rewrote `apps/hepta-native/README.md` as a Hepta Native product README; removed obsolete upstream Robrix app commands, `rs.robius` / `--app=robrix` examples, and `Robrix.app` desktop packaging references.
- Removed the unused tracked `packaging/Robrix macOS dmg background.png`; the active DMG asset is now `packaging/Hepta Native macOS dmg background.png`.
- Updated the third-party copy manifest so the Robrix desktop file and DMG background are recorded as renamed/heavily modified Hepta Native product assets rather than active Robrix-named files.
- Scope remains productization/status-model-only: no OpenClaw Gateway calls, Matrix sends, approval/tool execution, task registry mutation, device install, signing, `adb`, or simulator actions were added to UI status surfaces. Hepta is tracking native OpenClaw-parity capability replication, not a Gateway-backed integration path.
