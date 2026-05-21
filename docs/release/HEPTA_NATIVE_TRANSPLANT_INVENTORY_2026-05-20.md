# Hepta Native Transplant Inventory

Date: 2026-05-20
Scope: source-only inventory for bringing the standalone Native desktop/mobile
client into `hepta-codex`
Status: inventory complete; Patch 1 source-only app/docs import executed; Patch 2 isolated build/test gates passed; Patch 3 local bridge alignment complete for current bridge/fixture/status smoke; 2026-05-21 retirement pass moved active runtime state to `hepta-codex` and removed the old standalone Native residue

## Current Truth

`hepta-codex` has absorbed the Hepta control UI, core/runtime/gateway/intelligence/memory/plugins crates, native Telegram/POST surfaces, runtime state, and native desktop/mobile client. The active command entrypoint is `codex-rs` `codex-cli --bin hepta`; the old standalone `apps/hepta` wrapper was excluded because it depends on the old workspace layout. The repository did not yet contain the standalone desktop/mobile client before this transplant:

- active source now present in current repo: `/Users/qianqi/.openclaw/workspace/hepta-codex/apps/hepta-native`
- the old standalone Native source/target/runtime-state duplicate has been retired from the active workspace
- `hepta-codex` currently has no root Cargo workspace; its active Rust workspace is `codex-rs/Cargo.toml`

This means the native desktop/mobile client should be transplanted as a top-level app first, not added to the `codex-rs` workspace in the same patch.

## Source Payload

Source-only payload under `apps/hepta-native`, excluding `target/`:

- 252 source files currently present in `hepta-codex`, excluding `target/`
- ignored local `AGENTS.md` remains excluded from the reviewable transplant
- 125 Rust source files under `src/`
- 111 packaging/resource files under `packaging/` and `resources/`
- about 5.8 MB by `du -ck` source-file sum
- live build output present in source tree: `apps/hepta-native/target/` is about 31 GB and must not be copied

Top-level source groups:

- `.cargo/`: Makepad bundle/package defaults
- `.github/`: upstream Robrix/Makepad build workflows and helper action
- `Cargo.toml`, `Cargo.lock`, `build.rs`, `rust-toolchain.toml`, `rustfmt.toml`
- `LICENSE-MIT`, `License Attributions.md`, `README.md`, `SPLASH.md`
- `packaging/`: macOS, Linux desktop, iOS icons, DMG helpers, app metadata
- `resources/`: app icons, Android launcher resources, SVG UI icons, login/social images
- `src/`: Robrix Matrix-heart app plus Hepta-owned overlays and local bridge seams

## Functional Modules Not Yet In `hepta-codex`

These are the real missing native modules, not just docs:

- Matrix-heart substrate: `app.rs`, `sliding_sync.rs`, `space_service_sync.rs`, room list, timeline, composer, media/avatar/profile caches, login/logout, settings, persistence, TSP wallet scaffolding.
- Hepta custom event layer: `hepta_event.rs`, `event_preview.rs`, first-class `m.hepta.*` timeline recognition.
- Local fixture/no-homeserver boot: `hepta_fixture.rs` plus fixture cockpit widgets.
- Runtime bridge seams: `hepta_bridge.rs`, `hepta_action_bridge.rs`, `hepta_action_queue.rs`, local payload inspection and action staging.
- Composer command preview: `hepta_composer.rs`, `hepta_command_templates.rs`, `room/room_input_bar.rs` Hepta command path.
- Native status/productization panes: `hepta_runtime_status.rs`, `hepta_productization.rs`, `hepta_mobile_packaging.rs`, `home/hepta_*` panes.
- Desktop/mobile shell: `home/main_desktop_ui.rs`, `home/main_mobile_ui.rs`, `home/hepta_mobile_*`, Makepad widget wiring.
- Packaging metadata: app icons, macOS plist/DMG helper, Linux desktop entry, Android launcher resources, iOS icon assets.

## Docs And Attribution To Carry

The old repo has four relevant records that are absent from `hepta-codex` today:

- `docs/architecture/HEPTA_ROBRIX_DESKTOP_MOBILE_UI_DEVELOPMENT_2026-05-14.md`
- `docs/architecture/HEPTA_ROBRIX_MATRIX_HEART_PROGRESS_2026-05-14.md`
- `docs/architecture/third_party/ROBRIX_COPY_MANIFEST_2026-05-14.md`
- `docs/architecture/third_party/ROBRIX_NOTICE_2026-05-14.md`

These should be copied with the app source because the app incorporates Robrix / Project Robius code under MIT. The source repo and commit recorded there are:

- source repo: `https://github.com/project-robius/robrix`
- source commit: `b2bb6cf`
- source license: MIT

## Explicit Exclude List

Do not restore these retired standalone-repo artifacts into `hepta-codex`:

- old standalone Native `target/` build output
- old standalone repository `target/` build output
- old standalone Native Android SDK/toolchain cache
- old standalone release/test bundles
- old standalone rollback/checkpoint/test artifacts
- any `.hepta/` runtime state, Telegram ledgers, native-post stores, local-import stores, or watchdog state
- any generated screenshots, visual referee outputs, or temporary UI audit directories

`apps/hepta-native/AGENTS.md` exists in the source tree but is ignored by the old repo's `.git/info/exclude`. Treat it as local development guidance, not part of the reviewable transplant unless explicitly needed.

## Patch 1 Executed

Executed source-only import:

- copied `apps/hepta-native/` excluding `target/` and ignored local `AGENTS.md`
- copied Robrix attribution docs listed above
- kept `apps/hepta-native` outside `codex-rs/Cargo.toml`
- verified `apps/hepta-native/target/` does not exist in `hepta-codex`
- verified manifest parsing with `cargo metadata --manifest-path apps/hepta-native/Cargo.toml --no-deps --format-version 1`
- verified whitespace with `git diff --check`

## Patch 2 Executed

Executed isolated build/test gates with `CARGO_TARGET_DIR=apps/hepta-native/target`:

- `cargo check --manifest-path apps/hepta-native/Cargo.toml` passed
- `cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` passed: 47 tests, 0 failed
- verified `apps/hepta-native/target/` still does not exist in `hepta-codex`

## Patch 3 Started

Executed first local bridge alignment slice:

- added `hepta-core` and `hepta-runtime` path dependencies to `apps/hepta-native/Cargo.toml`
- added `apps/hepta-native/src/hepta_runtime_bridge.rs`
- exported `hepta_runtime_bridge` from `apps/hepta-native/src/lib.rs`
- the bridge reads current `codex-rs/hepta-core` absorption metadata and `codex-rs/hepta-runtime` readiness / live-adapter discipline reports
- the bridge projects those reports into an existing Matrix-shaped `m.hepta.runtime_event` fixture input
- all bridge side-effect flags remain false: no Gateway call, no provider invocation, no channel delivery, no process spawn, no Matrix send

Verification for this slice:

- `CARGO_TARGET_DIR=apps/hepta-native/target cargo check --manifest-path apps/hepta-native/Cargo.toml` passed
- `CARGO_TARGET_DIR=apps/hepta-native/target cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_runtime_bridge -- --nocapture` passed: 2 tests, 0 failed
- `CARGO_TARGET_DIR=apps/hepta-native/target cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` passed: 49 tests, 0 failed
- `rustfmt --check apps/hepta-native/src/hepta_runtime_bridge.rs` passed with existing stable-channel warnings about ignored unstable rustfmt options
- `git diff --check` passed
- verified `apps/hepta-native/target/` still does not exist in `hepta-codex`

## Patch 3 Continued

Executed second, third, and fourth local bridge alignment slices:

- wired the current `codex-rs/hepta-*` runtime bridge event into `hepta_fixture::sample_conversation()`
- the native fixture timeline now includes a live-readiness-shaped `m.hepta.runtime_event` with id `current-codex-runtime-bridge`
- added fixture tests for bridge source, conversation id, and side-effect flags
- added `hepta_fixture_smoke`, a bounded local smoke report that checks fixture timeline count, known Matrix event types, redaction, current runtime bridge presence, bridge source, and false side-effect flags
- surfaced the fixture smoke report in the native runtime status model and `HeptaRuntimeStatusPane` as `Current codex-rs fixture smoke`
- kept the event path local and serialized only; it does not send Matrix, call Gateway, deliver Telegram, invoke providers, or spawn processes

Verification for this slice:

- `CARGO_TARGET_DIR=apps/hepta-native/target cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_fixture -- --nocapture` passed: 6 tests, 0 failed
- `CARGO_TARGET_DIR=apps/hepta-native/target cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_fixture_smoke -- --nocapture` passed: 1 test, 0 failed
- `CARGO_TARGET_DIR=apps/hepta-native/target cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_runtime_status -- --nocapture` passed: 5 tests, 0 failed
- `CARGO_TARGET_DIR=apps/hepta-native/target cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture` passed: 52 tests, 0 failed
- `CARGO_TARGET_DIR=apps/hepta-native/target cargo check --manifest-path apps/hepta-native/Cargo.toml` passed
- targeted `rustfmt --check` on the new/changed native bridge, fixture, smoke, runtime-status, and runtime-status-pane files passed with existing stable-channel warnings about ignored unstable rustfmt options
- trailing-whitespace scan on changed native bridge/fixture/doc files passed
- `git diff --check` passed
- verified `apps/hepta-native/target/` still does not exist in `hepta-codex`

## Audit Correction

The first package pass missed the `apps/hepta-native/resources/icons/` SVG set because the repo-root `.gitignore` had a broad `Icon?` OS-file rule. The directory existed locally, so isolated native checks could pass, but the files were not reviewable/tracked. The post-package audit fixed this by adding explicit `.gitignore` exceptions for `apps/hepta-native/resources/icons/**` and including the 57 SVG resources in the review package.

## Remaining Patch Shape

Patch 3: bridge alignment with current `hepta-codex`

- Continue replacing static local status claims with read-only adapters over current `codex-rs/hepta-*` reports where that does not pull in live side effects.
- Package the native transplant workset into reviewable commits after a full untracked-file inventory.
- Keep live Matrix, Telegram, provider, task mutation, install, and deploy paths gated until an explicit activation request.

## Safety Boundary

This inventory does not change runtime ownership. Old OpenClaw Telegram remains the active owner unless explicitly changed later. No live Telegram poll/send, native POST activation, install, launchd reload, or release build is part of this transplant inventory.
