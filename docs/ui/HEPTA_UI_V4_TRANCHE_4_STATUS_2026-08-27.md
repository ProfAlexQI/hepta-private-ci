# Hepta UI v4 — Tranche 4 execution status

Date: 2026-08-27  
Branch: `codex/ui-light-glass-v4-20260827`  
Source candidate commit before this status record: `e692ebf98d297fc76e30f01f264a304a5f0348e6`  
Source candidate tree: `99a38378f59a8af58016cbcec929f4d8e419eba7`  
UI base: `647e294522a3b3341b4169e3f5a85f8f0df42cbe`  
Latest plan snapshot: `fe0889ecd46a5fc89de7b1ff3f28158c133a3502`

Status: `SERVED_RUNTIME_SOURCE_BOUND_COMPILE_AND_DEVICE_QUALIFICATION_REQUIRED`

This tranche closes the source-level defect recorded in Tranche 3: the v4
runtime controller is now part of the single JavaScript asset that Rust embeds
and the Native Gateway serves. This status does not claim that an executable
runner compiled or launched the candidate. Production, effect, live-adapter,
operator-acceptance, promotion and release authority remain false.

## 1. Canonical served JavaScript bundle

`hepta-core/build.rs` now constructs one deterministic Control UI bundle from:

1. `apps/hepta-control-ui/control-ui.js`;
2. the exact boundary `/* hepta-ui-v4-runtime-bundle-boundary */`;
3. `apps/hepta-control-ui/control-ui-v4-runtime.js`.

The order is fixed. The build script emits into `OUT_DIR`:

- `control-ui.bundle.js`;
- `control_ui_bundle_metadata.rs`.

The generated metadata exports:

- `CONTROL_UI_BASE_JS_SHA256`;
- `CONTROL_UI_V4_RUNTIME_JS_SHA256`;
- `CONTROL_UI_JS_SHA256`;
- `CONTROL_UI_JS_ETAG`;
- `CONTROL_UI_V4_RUNTIME_BOUND=true`.

Both source files are registered with `cargo:rerun-if-changed`. Empty source
files fail the build. No second HTTP script route is introduced, so the
browser still receives exactly one official `/control-ui.js` asset.

## 2. Exact digest and ETag binding

`hepta-core/src/control_ui_static_assets.rs` now embeds the generated bundle as
`CONTROL_UI_JS`, while retaining the two component sources as independently
inspectable constants.

The Native Gateway continues to expose only:

- `/assets/hepta-agent-logo.png`;
- `/control-ui.js`.

For `/control-ui.js`, both the response body and ETag are now generated from
the same bundle source set:

- body: `hepta_core::control_ui::CONTROL_UI_JS`;
- ETag: `hepta_core::control_ui::CONTROL_UI_JS_ETAG`.

This removes the previous risk that the v4 runtime could be present in the
repository but absent from served bytes, or that a hard-coded ETag could refer
to the pre-v4 controller.

## 3. Dependency-free SHA-256 generation

The bundle digest is computed inside the build script without adding a Cargo
build dependency or changing the workspace lockfile. The implementation runs a
known-answer test at build time:

- input: `abc`;
- expected SHA-256:
  `ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad`.

Failure of the known-answer test aborts the build before assets are generated.
The algorithm was additionally checked during implementation against standard
SHA-256 outputs for empty input, `abc`, a 1000-byte input and all byte values;
that local comparison is development evidence only and is not a repository CI
receipt.

## 4. Source gates and compile contracts

Added `scripts/hepta-ui-v4-served-bundle-gate`. It verifies:

- source existence and non-empty bytes;
- deterministic source order and boundary;
- generated bundle and metadata outputs;
- SHA-256 known-answer marker;
- lockfile-neutral build configuration;
- single official served JavaScript path;
- absence of a `/control-ui-v4-runtime.js` side route;
- generated body/ETag references in the Gateway;
- no fetch, XHR, WebSocket, EventSource, eval, Function or innerHTML capability
  in the v4 runtime controller;
- local-only runtime authority and bounded read-state contract;
- production/effect/operator/promotion values remain false.

Added `hepta-core/tests/control_ui_v4_bundle.rs`. After compilation it verifies:

- v4 binding is true;
- component and bundle digests are distinct and well formed;
- ETag matches the generated bundle digest;
- base controller precedes the boundary;
- the boundary precedes the v4 runtime controller;
- local-only runtime authority markers are present in served bytes.

The UI workflow now includes `cargo check` for `hepta-core` and
`hepta-native-gateway`, plus the focused bundle integration test.

## 5. CI infrastructure result

The GitHub Actions run for source candidate
`e692ebf98d297fc76e30f01f264a304a5f0348e6` was created as run
`33089048328`, but both executable jobs had:

- `steps=[]`;
- `runner_id=0`;
- no runner name;
- completion before checkout or command execution.

Therefore the red workflow result is classified as
`BLOCKED_RUNNER_NOT_ASSIGNED`, not as a Rust, JavaScript, CSS or test failure.
No compile, browser or device PASS is inferred from source inspection.

## 6. Current qualification state

| Lane | State |
|---|---|
| v4 runtime source | implemented |
| single canonical served bundle source | implemented |
| exact generated SHA-256 and ETag source | implemented |
| Gateway single-path binding | implemented |
| source gate | implemented, executable result pending runner |
| `hepta-core` bundle integration test | implemented, execution pending runner |
| Rust compile validation | `REQUIRED_NOT_RUN` / runner unavailable |
| Rust-served browser validation | `REQUIRED_NOT_RUN` |
| screenshot/accessibility receipt | `REQUIRED_NOT_RUN` |
| Windows/macOS/iOS/Android native adapters | source profiles only; system API binding incomplete |
| desktop/mobile device captures | `REQUIRED_NOT_RUN` |
| live mutation/effect | prohibited |
| production/operator acceptance/promotion | false |

## 7. Next executable tranche

1. Obtain an assigned runner or full local checkout and run all source gates,
   `cargo check` for `hepta-core`/Gateway and the bundle integration test.
2. Launch the Rust-served Control UI and verify that `/control-ui.js` contains
   both controllers, returns the generated ETag and changes ETag when either
   source changes.
3. Extend browser qualification from fixture HTML to the Rust-served endpoint,
   binding candidate commit/tree, bundle SHA-256, browser executable digest and
   screenshot digests.
4. Capture 320, 390, 412, 600, 768@200%, 980 and desktop scenarios, including
   focus trap/restore, Escape/back, safe area, reduced transparency, forced
   colors and GET-only network audit.
5. Compile Native and resolve any Makepad compatibility issue in the 48/56
   logical-pixel migration.
6. Implement and exercise real Windows Mica/Acrylic, macOS material, iOS
   navigation/sheet and Android dynamic-tonal adapters behind the existing
   fail-closed platform profiles.
7. Freeze a candidate only after source, compile and served-browser receipts
   refer to the same commit/tree; update visual matrix rows individually.
