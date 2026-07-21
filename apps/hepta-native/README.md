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
- iOS packaging gate: the checked-in productization snapshot records an iOS
  26.5 Simulator release-build smoke as complete. Re-run the commands below
  whenever the local Xcode/runtime installation changes.

The UI status/productization panes are reporting surfaces only. They must not run
installs, `adb`, simulators, signing, Gateway calls, Matrix sends, approvals,
tool execution, or task-registry writes.

## Local development gates

Run these from the Hepta repository root:

```sh
cargo check --manifest-path apps/hepta-native/Cargo.toml
cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture
./scripts/hepta-native-fixture-visual-smoke.sh
./scripts/hepta-native-packaging-gate.sh
./scripts/hepta-native-distribution-preflight-gate.sh
cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed -- --nocapture
./scripts/hepta-control-ui-smoke.sh
./scripts/hepta-ui-product-readiness-gate.sh
git diff --check
```

Run fixture mode without requiring a homeserver login:

```sh
HEPTA_NATIVE_FIXTURE_MODE=1 cargo run --manifest-path apps/hepta-native/Cargo.toml
```

Fixture route and selected-row variants are controllable without enabling
mutation:

```sh
HEPTA_NATIVE_FIXTURE_MODE=1 \
HEPTA_NATIVE_FIXTURE_ROUTE=Actions \
HEPTA_NATIVE_FIXTURE_ROW=2 \
cargo run --manifest-path apps/hepta-native/Cargo.toml
```

The fixture visual smoke is a deterministic desktop/mobile screenshot gate for
the local no-homeserver fixture surface. It verifies the current runtime bridge
event is visible in the cockpit card set and captures 1280x800 plus 500x844
screenshots without calling Matrix, Gateway, providers, or delivery channels.
The HTML preview also accepts `?route=Actions&row=0|1|2` so targeted visual
checks can cover selected-row detail, inline evidence, and inspector variants.
Inside the Makepad fixture, command/search result cards, route cards,
route-state cards, main rows, and the mobile dock also update the selected
route/row locally; the environment variables above remain the initial fixture
state and still do not enable mutation.

The Makepad fixture also mirrors the deterministic Telegram product-shell gate.
Default desktop layout is a two-pane chat surface: chat list, message thread,
and composer. The info panel, command palette, route cards, detailed row cards,
metrics, safety status, active-route previews, route-state cards, shell states,
review queue, and evidence timeline collapse out of the primary surface. Set
`HEPTA_NATIVE_FIXTURE_LAYOUT=desktop-full` to reopen that full diagnostic cockpit
locally.

The visual gate now treats that first reading path as a contract. Its JSON
summary reports `native_first_read_path_guard_ready=true`,
`native_desktop_first_read_path_ready=true`,
`native_mobile_first_read_path_ready=true`, and
`native_engineering_copy_hidden=true`; it fails if fixture/debug phrases such as
`Fixture mode`, `mutation=false`, `payload hash`, `renderer contract`, `old JS`,
`blank module fallback`, or `NO_REPLY` return to the served product surface.

For a product-team acceptance pass across both product surfaces, run:

```sh
./scripts/hepta-ui-product-readiness-gate.sh
```

That combined gate statically checks the Control UI product-first and Native
first-read markers, runs both screenshot gates, verifies the Native packaging
metadata plus local unsigned app-bundle probe against the current worktree's
local loopback server, runs the Native distribution preflight, and emits a single
`ui_product_readiness_gate_ready=true` JSON summary. It also writes
`readiness.json`, `static-contract.json`, `artifact-summary.json`,
`native-base-gap-drilldown.json`, `native-base-gap-work-queue.json`,
`native-base-gap-backend-handoff.json`, `screenshot-manifest.json`, and
`native-packaging-gate.json`, `native-distribution-preflight-gate.json`, and
`ui-plan-boundary-gate.json`, `ui-demo-evidence-gate.json`,
`ui-evidence-bundle-gate.json`, `ui-evidence-archive-gate.json`,
`ui-release-operator-dry-run-gate.json`, `ui-operator-briefing-gate.json`,
`ui-backend-contract-acceptance-gate.json`, and
`ui-backend-handoff-export-gate.json`, `ui-backend-dispatch-packet-gate.json`,
`ui-backend-receipt-intake-gate.json`,
`ui-backend-receipt-roundtrip-gate.json`, and
`ui-backend-receipt-refresh-lock-gate.json`, and
`ui-future-plan-refresh-gate.json`,
`ui-operator-briefing-refresh-gate.json`,
`ui-release-approval-intake-gate.json`,
`ui-top-design-referee-refresh-gate.json`,
`ui-release-artifact-boundary-gate.json`,
`ui-release-artifact-intake-gate.json`,
`ui-release-artifact-roundtrip-gate.json`,
`ui-current-plan-refresh-gate.json`,
`ui-blocker-closure-gate.json`,
`ui-backend-delivery-audit-gate.json`, and
`ui-root-report-replay-gate.json` with
the key Native and Control UI screenshot artifact paths, viewports, byte counts,
SHA-256 hashes, 12/12 Native base-gap drilldown state, per-gap acceptance
criteria, priority-ordered UI contract-ready backend next slices, backend
contract handoff, locked side-effect boundaries, synchronized packaging
readiness, a local unsigned `.app` bundle probe, static
signing/notarization workflow proof, explicit local-demo versus live-product
versus release/public-distribution claim boundaries, replayable demo evidence
source-report checks, screenshot dimension/hash verification, a local
self-contained evidence bundle manifest, a local compressed review archive with
extract-and-SHA replay, a release-operator dry-run manifest with a denial matrix
for unapproved signing/notarization/public-artifact requests, a local operator
briefing that names the critical risks/blockers/future plan, a critical-path
plan index for blocker/owner/next-lane alignment, a backend contract acceptance
handoff for the first five backend-owned items, a backend handoff export
markdown for the backend lane, a backend dispatch packet archive with manifest
and extract-and-SHA replay, a backend receipt intake template and optional
receipt validator, a release-artifact intake template, a current-plan refresh
receipt validator, a release-artifact intake template, a local release-artifact
roundtrip simulator that proves both waiting and valid-present intake branches
without promoting release/public claims, a current-plan refresh report that
preserves legacy r52 plan replay while promoting the current minimum UI demo
gate to r58 plus release artifact intake requirements, a blocker-closure report,
a backend delivery audit that separates local dispatch packet readiness from
actual backend-lane delivery receipt evidence, a root-report replay index for
plan/handoff/archive/release-dry-run/operator briefing/critical path/backend
acceptance/backend handoff export/backend dispatch packet/backend receipt
intake/backend receipt roundtrip/backend receipt refresh-lock/future-plan
refresh/operator briefing refresh/release approval/top-design/release artifact
boundary/release artifact intake/release artifact roundtrip/current-plan/
blocker-closure/backend-delivery audit alignment, and an opt-in true Makepad
window smoke report.

The plan-boundary report is local and non-mutating. It is intentionally green
only when local fixture/demo readiness is true while live-product and
release/public-distribution claims remain false until the backend-contract and
release-operator lanes provide their evidence.

The demo-evidence report is also local and non-mutating. In deterministic
no-window mode it verifies the fixture/browser evidence package; in full hard
mode it additionally requires the main, route, desktop-secondary, and
mobile-secondary true-window screenshots to be present, dimension-readable, and
byte-for-byte matched to the recorded SHA-256 values.

The evidence-bundle report copies the required demo-evidence reports and
screenshots into `evidence-bundle/` inside the same readiness directory, then
verifies copied file counts, JSON validity, and SHA-256 equality. It is a local
review/retention bundle only: it does not upload, sign, notarize, staple, or
write a public distribution artifact.

The evidence-archive report compresses that normalized `evidence-bundle/` plus
its bundle report into `evidence-archive/hepta-ui-evidence-bundle.tar.gz`, then
extracts the archive and verifies archive file counts, bundled file counts,
JSON validity, and per-file SHA-256 equality. It remains a local review artifact:
it is not signed, notarized, stapled, uploaded, or a public distribution build.

The root-report replay report reopens the readiness directory's root JSON
reports, verifies 41 report files are present, valid, SHA-addressed, and aligned
across backend handoff, future plan, demo evidence, bundle, archive, packaging,
distribution, release-operator dry-run, operator briefing, backend promotion
packet, backend alignment evidence, critical-path plan, backend contract
acceptance, backend handoff export, backend dispatch packet, backend receipt
intake, backend receipt roundtrip, backend receipt refresh lock, future-plan
refresh, operator briefing refresh, release approval intake, top-design
referee refresh, release artifact boundary, release artifact intake, release
artifact roundtrip, current-plan refresh, blocker closure, backend delivery
audit, and optional true-window reports. It keeps the same claim boundary:
local review evidence may be ready while live-product, release, and public
distribution claims remain false.

The release-operator dry-run report is local and non-mutating. It reads the
packaging, distribution preflight, plan-boundary, and evidence-archive reports,
writes a local dry-run manifest, and verifies that missing approval, remaining
backend contracts, missing credential/notary evidence, and public artifact write
attempts remain denied. It does not execute the DMG helper, read Apple
credentials, query the keychain, sign, notarize, staple, upload, or write a
public distribution artifact.

The operator-briefing report is local and non-mutating. It reads the plan,
demo-evidence, archive, release dry-run, backend handoff, distribution
preflight, and blocker rollup reports, then writes one machine-readable briefing
with exactly three top-level risks: 12 remaining live backend contracts,
release/public distribution not approved, and the hard true-window requirement
for public-demo evidence. It also hard-fails if the briefing claims
`live_product_ready`, `public_distribution_ready`, or `release_ready`.

The backend-promotion packet report is also local and non-mutating. It consumes
the backend handoff, backend contract wave gate, blocker rollup, plan boundary,
and operator briefing, then emits the first five backend-owned promotion
packets: message search, file upload/send, media download/playback,
notifications, and room settings. It records the exact backend adapter/readback
evidence needed before promotion and keeps active backend promotion, live
runtime mutation, and release/public claims disabled.

The backend-alignment evidence report is local and non-mutating. It consumes the
backend-promotion packet plus handoff, contract wave, fixture, demo evidence,
archive, plan, and operator reports, then emits one evidence map for
message search, file upload/send, media download/playback, notifications, and
room settings. Each map item ties the handoff priority to the backend contract
gate, required fixture markers, next backend slice, and promotion blocker while
keeping backend adapter promotion, live runtime mutation, and release/public
claims disabled.

The critical-path plan report is local and non-mutating. It consumes the plan,
demo evidence, evidence archive, release dry-run, operator briefing, backend
promotion packet, and backend-alignment evidence, then emits one blocker and
future-plan index for the current UI lane position. It keeps the first backend
handoff set fixed to message search, file upload/send, media download/playback,
notifications, and room settings, while recording that live-product,
release/public-distribution, backend adapter promotion, and live runtime
mutation claims remain false.

The backend-contract acceptance report is local and non-mutating. It consumes
the critical-path plan, backend-alignment evidence, backend-promotion packet,
operator briefing, and plan-boundary reports, then emits an acceptance checklist
for message search, file upload/send, media download/playback, notifications,
and room settings. Each checklist item requires a backend adapter contract,
operation id/source hash, backend readback evidence, retry/cancel/idempotency
policy, stale-target guard, side-effect review, and refreshed no-window plus
full-hard UI readiness artifacts before any backend promotion claim can become
true.

The backend-handoff export report is local and non-mutating. It consumes the
plan-boundary, operator briefing, backend-promotion packet, backend-alignment
evidence, critical-path plan, and backend-contract acceptance reports, then
writes `backend-handoff-export/backend-handoff-export.md` for the backend lane.
The export keeps the first five ids fixed to message search, file upload/send,
media download/playback, notifications, and room settings, and records the
required UI refresh commands after backend changes. It does not perform backend
adapter promotion, readback recording, live runtime mutation, external dispatch,
signing, notarization, upload, or public distribution.

The backend-dispatch packet report is local and non-mutating. It consumes the
backend-handoff export report and markdown plus the plan-boundary, operator
briefing, backend-promotion packet, backend-alignment evidence, critical-path
plan, and backend-contract acceptance reports. It copies those files into
`backend-dispatch-packet/payload/`, writes a manifest, compresses the payload to
`backend-dispatch-packet/backend-dispatch-packet.tar.gz`, extracts it, and
replays every payload SHA. It records that no backend agent dispatch, backend
adapter promotion, readback evidence, live runtime mutation, external dispatch,
signing, notarization, upload, or public distribution occurred in the UI lane.

The backend-receipt intake report is local and non-mutating. It consumes the
backend dispatch packet report, manifest, and archive, then writes
`backend-receipt-intake/backend-receipt-template.json` plus
`backend-receipt-intake/backend-receipt-intake.md` for the backend lane to fill.
When `HEPTA_UI_BACKEND_RECEIPT_INPUT_PATH` is unset, the gate stays green in an
explicit waiting-for-receipt state. When it is set, the receipt must cover the
same first-five ids, match the dispatch packet archive SHA, and include backend
adapter contract, operation id/source hash, readback, retry/cancel/idempotency,
stale-target, and side-effect evidence for every item. It still does not make
live-product, public-distribution, or release claims ready.

The backend-receipt roundtrip report is a local simulator for the receipt intake
validator. It generates
`backend-receipt-roundtrip/simulated-backend-receipt.json` bound to the current
dispatch packet archive SHA, then reruns the receipt intake gate into
`backend-receipt-roundtrip/ui-backend-receipt-intake-present-gate.json` without
overwriting the default waiting-for-receipt intake report. The combined
readiness and root replay gates require both branches to stay green: waiting for
a real backend receipt and a local simulated valid receipt. This is simulation
only; it records no real backend completion, backend adapter promotion, live
runtime mutation, public distribution, or release readiness.

The backend-receipt refresh-lock report is local and non-mutating. It consumes
the waiting intake report, simulated roundtrip report, dispatch packet, and
optional true-window reports, then writes
`backend-receipt-refresh-lock/backend-receipt-refresh-lock.md`. It keeps the
waiting-for-real-receipt state green, proves the simulated branch is not
promoted to a real backend receipt, and records the required no-window plus
full-hard UI refresh commands for the first real backend receipt. Live-product,
public-distribution, and release claims remain false until a real receipt and a
fresh hard UI pass exist together.

The future-plan refresh report is local and non-mutating. It consumes the
critical-path plan, backend acceptance/export/dispatch packet, receipt intake,
receipt roundtrip, receipt refresh-lock, and optional true-window reports, then
emits a refreshed machine-readable plan. This supersedes the legacy
plan-boundary `next_plan` with an r52-level minimum gate:
`r52_minimum_ui_demo_gate`, `backend_real_receipt_return`, and
`ui_refresh_after_real_receipt`. It keeps live-product, public-distribution,
and release claims false.

The operator-briefing refresh report is local and non-mutating. It consumes the
current operator briefing, future-plan refresh, backend dispatch packet,
receipt refresh-lock, release dry-run, and evidence archive reports, then writes
`operator-briefing-refresh/operator-briefing-refresh.md`. The combined root
replay now requires 33 JSON reports, including the refreshed operator briefing,
so the current critical risk list and machine plan cannot drift after the
future-plan refresh. It keeps real backend receipt, live-product,
public-distribution, and release claims false.

The release-approval intake report is local and non-mutating. It writes
`release-approval-intake/release-approval-template.json` and records that the UI
lane is waiting for explicit release approval. Approval alone still cannot make
release/public-distribution claims true without a signed/notarized/stapled
artifact gate.

The top-design referee refresh report is local and non-mutating. It consumes
the current full-hard visual evidence, including the persisted Control UI 320px
system-Chrome screenshot plus true-window route and secondary mobile probes, and
writes `top-design-referee-refresh/top-design-referee-refresh.md`.

The release artifact boundary report is local and non-mutating. It consumes
packaging, distribution preflight, release dry-run, release approval intake,
top-design refresh, and archive reports, then writes
`release-artifact-boundary/release-artifact-boundary.md`. It locks the current
state as unsigned/local only: no signed, notarized, stapled, public, uploaded,
or externally distributed artifact exists, and the next required artifact gate is
`signed_notarized_stapled_artifact_gate`.

The release artifact intake report is also local and non-mutating. Its default
branch stays green while waiting for a signed/notarized/stapled artifact. Its
artifact-present branch accepts a release-operator artifact receipt only when it
records signed, notarized, stapled, public distribution artifact, and SHA-256
evidence; even then it keeps release/public/live claims false until backend
receipt and post-artifact UI refresh evidence exist.

The release artifact roundtrip report is a local simulator for the release
artifact intake validator. It generates
`release-artifact-roundtrip/simulated-signed-artifact.json`, reruns release
artifact intake into
`release-artifact-roundtrip/ui-release-artifact-intake-present-gate.json`, and
requires both the default waiting branch and the simulated valid-present branch
to remain ready. The simulator writes only local proof files: it does not read
Apple credential values, query the keychain, sign, notarize, staple, upload,
publish, or make release/public/live claims ready.

By default the combined gate stays deterministic and does not require an
unlocked macOS desktop. To also collect true Makepad desktop/mobile window
screenshots in the combined artifact, run:

```sh
HEPTA_UI_PRODUCT_READINESS_INCLUDE_NATIVE_WINDOW_SMOKE=1 \
  ./scripts/hepta-ui-product-readiness-gate.sh
```

If the desktop is locked or local macOS capture permission is unavailable, keep
the combined artifact green only when the goal is to record that explicit local
blocker rather than claim true-window screenshots passed:

```sh
HEPTA_UI_PRODUCT_READINESS_INCLUDE_NATIVE_WINDOW_SMOKE=1 \
HEPTA_UI_PRODUCT_READINESS_ALLOW_NATIVE_WINDOW_BLOCKED=1 \
  ./scripts/hepta-ui-product-readiness-gate.sh
```

For the Native packaging metadata plus local unsigned app-bundle probe, run:

```sh
./scripts/hepta-native-packaging-gate.sh
```

By default the gate starts the current worktree's `hepta --serve-ui` loopback
server on a free local port before reading packaging, GA readiness, and merge
completion endpoints. It also builds the local `hepta-native` binary and stages
an unsigned `Hepta Native.app` under the gate artifact, then checks the
Info.plist, icon, bundle identifier, URL schemes, Mach-O binary, and hashes.
This is local packaging evidence only: it does not sign, notarize, staple,
publish, read Apple credentials, or create a public distribution artifact. Set
`HEPTA_LIVE_URL=http://127.0.0.1:7373` only when you intentionally want to
validate an already-running server instead.

For the Native distribution preflight, run:

```sh
./scripts/hepta-native-distribution-preflight-gate.sh
```

The preflight is intentionally static and local: it lints `Info.plist` and
`Entitlements.plist`, checks Cargo Packager metadata, verifies the DMG helper
contains Developer ID signing, hardened runtime, entitlement, timestamp,
notarytool, stapler, spctl, and DMG repack steps, and can consume the packaging
gate's unsigned `.app` probe. It does not read Apple credential values, query the
keychain, submit to notary, sign, staple, publish, or write a public artifact.
If `cargo-packager` is not on `PATH`, the preflight still passes the static
workflow contract and records `cargo_packager_missing` as the remaining local
release-tooling blocker.

When the available widget width is 620px or narrower, or when
`HEPTA_NATIVE_FIXTURE_LAYOUT=mobile` is set, it switches to a task-first mobile
chat layout: mobile chat header, message thread, and composer remain visible
while command palette, desktop route cards, detailed row cards, metrics, safety
status, active-route previews, route-state cards, shell states, review queue,
and evidence timeline collapse out of the first screen.

For a true macOS app-window capture, run the Peekaboo-backed gate:

```sh
./scripts/hepta-native-window-smoke.sh
```

That gate launches `hepta-native` in fixture mode, resizes the Makepad window to
desktop and mobile bounds, captures real window screenshots, and requires both
captures to enter the Hepta Native product shell, hide the raw Matrix composer,
and trigger the expected desktop/mobile layout markers. It requires an unlocked
macOS desktop plus Screen Recording and Accessibility permission for
Peekaboo/the invoking terminal. Set `HEPTA_NATIVE_WINDOW_SMOKE_PREBUILD=1` to
compile before the app launch so cold Rust builds are not counted as window
startup time. Each smoke artifact uses an isolated Cargo target directory by
default (`$OUT_DIR/cargo-target`, or `$OUT_DIR/cargo-target/hepta-ui-native` for
the combined product-readiness gate) so concurrent UI lanes do not contend for a
shared build directory; set `HEPTA_NATIVE_WINDOW_SMOKE_CARGO_TARGET_DIR` or the
older `HEPTA_NATIVE_CARGO_TARGET_DIR` only when intentionally reusing a target.
The route, desktop-secondary, and mobile-secondary wrappers prebuild once by
default and then set `HEPTA_NATIVE_WINDOW_SMOKE_SKIP_PREFLIGHT=1` for child
captures after their shared Rust preflight suite has passed. The route wrapper
also requires each Home, Actions, Approvals, and Inspector capture to pass the
route content probe, which samples the workspace plus upper and lower route
regions. The mobile-secondary wrapper requires the selected secondary surface
to emit a content-visible app marker before capture and requires each 390px
phone-window screenshot to pass the first-viewport mobile content probe. These
gates prove more than uniqueness/nonblank pixels. On locked or
permission-blocked machines, use the blocked-report mode only to record the
explicit local blocker without claiming a visual pass:

```sh
HEPTA_NATIVE_WINDOW_SMOKE_ALLOW_BLOCKED=1 ./scripts/hepta-native-window-smoke.sh

# Backward-compatible alias for local permission blockers:
HEPTA_NATIVE_WINDOW_SMOKE_ALLOW_PERMISSION_SKIP=1 ./scripts/hepta-native-window-smoke.sh
```

## Latest product-readiness proof

The latest UI-lane full-hard product-readiness artifact is:

```text
/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.top-design-v3-r65-full-hard-20260619
```

The r65 full-hard run passed with `status=ready`, 60 total screenshots, root
report replay 41/41, release artifact roundtrip ready, current-plan refresh
ready, blocker closure ready, backend delivery audit ready, and top-design
referee refresh `refresh_version=3`. The v3 referee expands the r64 standard
alignment into a control-level matrix for desktop/mobile modules, small
buttons, and opened submenu surfaces. It requires Control UI primary controls
across 4 viewports, 18 selected-row variants across Actions/Approvals/Inspector
and desktop/mobile fixture widths, 15 secondary-surface cases across
search/settings/attachment/voice/modal and desktop/390/320 widths, 57 total
secondary-surface actions, true-window desktop/mobile submenu coverage, and
`clipping_failure_count=0`.

The true-window set includes main 2, route 4 unique with route content probe
ready, desktop secondary 5 unique, and mobile secondary 5 unique with mobile
content probe ready and visible count 10. The top-design referee, root replay,
and final readiness all use the current machine plan:
`r62_minimum_ui_demo_gate`, `backend_real_receipt_return`,
`ui_refresh_after_real_receipt`, and
`release_artifact_roundtrip_and_signed_artifact_gate`. Blocker closure critical
blockers remain 5, and backend delivery audit critical blockers remain 6
because the backend delivery receipt is still missing.

Key r65 full-hard hashes: readiness report
`052f4cf42dd8c2a09a254e579b240caab02be3c119aac846c3b9e197f766c08c`,
top-design referee refresh report
`5a7779daafc84700d36a6e80dd05c5ff3fd89af4f8610660b225ace8dd4ec484`,
root replay report
`9e0d2f048f8e2cce79396470c2179e3b1c5ee8c71f3ccdc2f35ff1bed6071071`,
Control UI `phone320.png`
`978a1d00a04bc44454cae7b2e806c9d509a139977db54bcb3c875549de24b6f2`,
selected-row manifest
`c0dfae0a6cf375c4aaf1d61868634f7df84fcbe64a4008363a26395b03432efb`,
backend dispatch packet archive
`91664373d30fab21f5e870443a5409be6d08f43b6f376fd20c5823b95139c079`,
and evidence archive
`1050b4a265bbe2a509835d92bd1b47a81a871af759f6b038af3a47e28793ac8c`.
No real backend receipt, backend delivery receipt, signing, notarization,
stapling, upload, Gateway/provider/channel call, or external mutation was
performed.

The v3 referee still uses the official-source calibration added after r64:
current Apple HIG, Material I/O 2026, and W3C WCAG 2.2 mobile guidance.
Targeted reference-calibration report:
`/Users/qianqi/.openclaw/tmp/hepta-ui-top-design-referee-r64b-2026-reference-calibration.json`,
SHA-256
`f3d92a760f8eb905c68a11fa8311f46d221ca2f77e55b88b8c7679e8c86c3833`.

The latest no-window productization replay artifact is:

```text
/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.top-design-v3-r65-nowindow-20260619
```

The r65 no-window run passed with `status=ready`, 44 deterministic screenshots,
root report replay 41/41, top-design referee refresh `refresh_version=3`,
backend delivery audit ready, and release artifact roundtrip ready. It is the
deterministic companion proof for the r65 full-hard visual baseline. The
readiness report SHA-256 is
`6f1f74fe3c6ff00524903cb3db7329e6b5c6037edd36844fa78f02e6b02fcaf3`, the
top-design referee refresh report SHA-256 is
`64d75726b1a135354cf10ce83937f59106b0ab8cb512281616e3161fdf6f1ace`, the root
replay report SHA-256 is
`43d37d3e7a1926831758a0c7bdffc56535f2f8612d2c3e726a3bcd062ccfad20`, the
backend dispatch packet archive SHA-256 is
`93e7fb8f977577cd7d78647f89a205eed5df331e5290ded640a744a36e5486c8`, and the
no-window evidence archive SHA-256 is
`d428e99c8f3b4ac38935f8d52aa6229b036c166f77afd42863e4d02a4777c3ad`.

The latest full-hard local risk/future-plan replay is:

```text
/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.risk-future-plan-r66d-full-hard-20260619
```

The r66d full-hard run passed with `status=ready`, 60 screenshots, root report
replay 42/42, and `ui-risk-future-plan-gate.json` ready. It records
`current_evidence_mode=full_hard_true_window`,
`current_full_hard_evidence_ready=true`, route content probe ready, desktop and
mobile secondary smoke ready, and mobile secondary content probe ready with 10
visible menu/action items. The latest plan ids are
`r65_top_design_v3_full_hard_minimum_ui_demo_gate`,
`backend_delivery_receipt_return`, `backend_real_receipt_return`,
`ui_refresh_after_real_receipt`, and
`release_artifact_roundtrip_and_signed_artifact_gate`. Critical blockers remain
6 because backend delivery receipt, real backend receipt, backend first-five
execution, release approval, and a real signed/notarized/stapled artifact are
still external/cross-lane blockers.

Key r66d hashes: readiness report
`e74ddfd41b3b182c5138aeee831f8532f77dc45d52712b88ea1b5926efe1bbd4`,
risk/future-plan report
`38168eae027c0ffad2861582b00417a27939a3e169fda62757a7db481f0f6490`,
root replay report
`c2f2a496d79dcd7b76dc0fedba80b383b02cca7861dac07ae2e14a80d6c34c7a`,
top-design report
`d5e59647967f4f2fbba618227112cca8f81dd112f315a83e640f1afdf3e30d54`,
and evidence archive
`20112d23fd457bc15a1cc38c8a01e2e46a9944c861bcbe9a0446398d43cdba2e`.

The deterministic no-window companion replay is:

```text
/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.risk-future-plan-r66c-nowindow-20260619
```

The r66c no-window run passed with `status=ready`, 44 screenshots, root report
replay 42/42, and `ui-risk-future-plan-gate.json` ready. It keeps the r65
top-design v3 full-hard proof as the minimum demo baseline while recording the
current no-window artifact as a companion replay only:
`current_evidence_mode=no_window_fixture`, `current_artifact_evidence_ready=true`,
`current_full_hard_evidence_ready=false`, and `no_window_companion_ready=true`.
The latest plan ids are `r65_top_design_v3_full_hard_minimum_ui_demo_gate`,
`backend_delivery_receipt_return`, `backend_real_receipt_return`,
`ui_refresh_after_real_receipt`, and
`release_artifact_roundtrip_and_signed_artifact_gate`. Critical blockers remain
6 because backend delivery receipt, real backend receipt, backend first-five
execution, release approval, and a real signed/notarized/stapled artifact are
still external/cross-lane blockers.

Key r66c hashes: readiness report
`ae95d67c9a0298c085d7d71f9047a5181c30ec7c55a719241cec888f3a64cdfb`,
risk/future-plan report
`0ab67314b904ffc091a1fc269551d5a17e8e0da0ab0cd8ff7e2af5ca9805e187`,
and root replay report
`27f6a185be4caadd82cfd6142e69f5a475581deeb944408f91582e955b20bdde`.
Targeted replay against copied r65 full-hard evidence also passed at
`/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.risk-future-targeted-r66c-fullhard-20260619`,
with `current_evidence_mode=full_hard_true_window`, 60 screenshots, and root
report replay 42/42. That targeted full-hard risk/future-plan report SHA-256 is
`ac7d11b1147478a596eca2564503a03d4af94f238f2880e6178e23224e8cbade`.

## Backend delivery receipt roundtrip

The latest deterministic backend-delivery receipt roundtrip proof is:

```text
/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.backend-delivery-receipt-roundtrip-r67d-nowindow-20260619
```

The r67d no-window run passed with `status=ready`, 44 screenshots, root report
replay 43/43, and `scripts/hepta-ui-backend-delivery-receipt-roundtrip-gate.sh`
included in product readiness. The new gate generates a local simulated
`backend_dispatch_packet_delivery_receipt`, reruns the backend delivery audit in
the valid-present branch, and verifies that the delivery-claim branch can become
ready while real backend receipt, backend receipt, live product, public
distribution, and release claims stay false.

Key r67d hashes: readiness report
`fda6bbc4dc2ffaecd3af62190d3c1fad5a0025081943c74e0bd4e95398f049f7`,
backend delivery receipt roundtrip report
`52569e55ad10d437e89ef9b4890f19e692d20ba833f9e82066753dcc6309398b`,
simulated delivery receipt
`79f8ebcd130f55640f273722d6dc0aa0b84e2087c3cd45adabf99d3604072c8c`,
present-branch delivery audit
`a78a74ac2601b83cb545a9a7b1c8459d51292cbfda6fad23e0031ac278a59352`,
risk/future-plan report
`fd9fe011e717e97ccbe8e1033392d6f5781718aa447e85c04caeb4728cfcecf1`, and root
replay report
`df92b547be1067a7bf44aee3546e56deeed9f5ee69170c9c5775cb6cf1aaad80`.

Targeted full-hard-aligned replay against copied r66d full-hard evidence also
passed at
`/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.backend-delivery-receipt-roundtrip-r67-targeted-fullhard-20260619`,
with the roundtrip, risk/future-plan, and root replay reports ready at 43/43.

This is still a local branch replay, not a backend-lane receipt. Remaining
blockers are the actual backend delivery receipt, real backend receipt and
first-five readback, release approval, and the real signed/notarized/stapled
artifact.

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
  --sdk-path=/Users/qianqi/.openclaw/workspace/hepta-codex/android_33_sdk \
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
HEPTA_NATIVE_NOTARYTOOL_PROFILE=hepta ./packaging/build-macos-dmg.sh
APPLE_ID=... APPLE_PASSWORD=... APPLE_TEAM_ID=... ./packaging/build-macos-dmg.sh
```

Prefer a `notarytool` keychain profile so Apple credential values do not have to
be passed directly through the shell. When a release approval is already valid,
the helper can also emit the machine-readable artifact receipt consumed by the
release artifact intake gate:

```sh
HEPTA_NATIVE_NOTARYTOOL_PROFILE=hepta \
HEPTA_NATIVE_RELEASE_APPROVAL_VALID=1 \
HEPTA_NATIVE_RELEASE_ARTIFACT_RECEIPT_PATH=/tmp/hepta-native-release-artifact.json \
./packaging/build-macos-dmg.sh
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

## r68 tempered-glass top-design gate

The current desktop/mobile design referee baseline is r68
`2026_tempered_glass_liquid_glass`.

- Primary full product proof:
  `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.tempered-glass-r68-nowindow-20260619`
- Targeted full-hard replay:
  `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.tempered-glass-r68-targeted-fullhard-20260619`
- Top-design referee `refresh_version=4`.
- Risk/future minimum gate:
  `r68_tempered_glass_v4_minimum_ui_demo_gate`.
- Root report replay remains 43/43.
- The tempered-glass matrix requires translucent panels, glass hairlines,
  backdrop blur, dark-surface readability, primary button coverage, secondary
  submenu coverage, mobile 320px readiness, touch target readiness, contrast
  `>= 4.5`, and `clipping_failure_count=0`.
- Latest no-window readiness SHA:
  `b4a41db93abda5f3c04a2e70ff0ffd1ce1862408801ec828b019c33b015581e0`.
- No backend mutation, Gateway/provider/channel call, Matrix login, signing,
  notarization, stapling, upload, or external mutation is part of this UI gate.

## r69 semantic submenu action-matrix gate

The current strongest desktop/mobile design referee baseline is r69b
`2026_tempered_glass_liquid_glass` with top-design referee
`refresh_version=5`.

- Full-hard product proof:
  `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.tempered-glass-action-matrix-r69b-fullhard-20260619`
- No-window companion:
  `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.tempered-glass-action-matrix-r69b-nowindow-20260619`
- Risk/future minimum gate:
  `r69_tempered_glass_v5_action_matrix_minimum_ui_demo_gate`.
- Root report replay remains 43/43.
- The r69 matrix keeps the r68 tempered-glass checks and adds an exact
  semantic secondary-submenu action matrix: 15 cases, 57 action instances,
  stable action ids, semantic `button` controls, aria labels, touch target
  readiness, and zero clipping.
- Full-hard readiness SHA:
  `ec16a657b9ce28a44b8036f81223ab8bf21220a024919004b47bca0840bec827`.
- No-window readiness SHA:
  `836458b1aa432560b5c4a820f35e9ac695ef7beee493a992f5a1e9229dd94eb0`.
- No backend mutation, Gateway/provider/channel call, Matrix login, signing,
  notarization, stapling, upload, or external mutation is part of this UI gate.

## r70 cross-lane receipt and release approval replay

r70 keeps the r69b full-hard UI baseline as the latest desktop/mobile design
proof and records the first real cross-lane blocker reductions.

- Targeted replay artifact:
  `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.crosslane-r70-targeted-from-r69b-20260619`
- Summary report:
  `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.crosslane-r70-targeted-from-r69b-20260619/ui-crosslane-r70-summary.json`
- Summary SHA:
  `5a345b62eb7a0f906c683cf1c119c005d986cb2f6cbd8794fe9849dbf4068f17`
- Real backend delivery receipt SHA:
  `457c7dc3b0d6935ba148e6d13863c9bfa05accd54ba845a17a35a00558d4eda4`
- Release approval receipt SHA:
  `171ef22165452bdd254ce40d1825f712728297e495409310e83966f9e8de5410`
- Delivery receipt valid: true; backend delivery claim ready: true.
- Release approval valid: true; release approval claim ready: true.
- Remaining blocker count: 4.
- Remaining blockers: current-session backend agent dispatch visibility, real
  backend receipt missing, backend first-five not executed, and real
  signed/notarized/stapled artifact missing.
- Signing capability audit SHA:
  `ba23ebe58b2b1e56e2e68b583020abb35bc1b000afc5871239bf4056258bea54`.
  `notarytool`, `stapler`, and `cargo-packager` are present, but there are zero
  valid code signing identities and no Apple notary credentials in env.
- Backend first-five readback attempt SHA:
  `170b0f70ed31387731e9744fdbe9d86fe16890e83e01cdb521ece656c8b64bc2`.
  It records that `room_settings`, `notifications`, and
  `media_download_playback` still have backend-side not-implemented/not-supported
  evidence in `/Users/qianqi/.openclaw/workspace/Hepta`.
- r70 did not perform signing, notarization, stapling, public upload, Matrix
  login, Gateway/provider/channel calls, backend repo writes, live runtime
  mutation, or external mutation.

## r71 cross-lane full product readiness

r71 promotes the r70 targeted receipt/approval work into a full no-window
product-readiness pass. The current strongest visual proof remains the r69b
full-hard top-design v5 artifact; r71 verifies the cross-lane receipt state
inside the full product gate.

- Full product artifact:
  `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.crosslane-r71-product-from-r70-20260619`
- Status: `ready`
- Screenshot total: 44
- Root report replay: 43/43
- Product readiness SHA:
  `fe8f58e85aef21518745d45f0c1ddf38f24b24b6234c93cfd38b9e8b470ef07e`
- Root replay SHA:
  `02b71c813e73a921b89b157514d0814740a984fc2af9c9bc2fce475d6e159407`
- Risk/future plan SHA:
  `e99c367cac23b163ff6012064d96be6cd070ff2fa4c09a1b20d54b4cd92e47af`
- Backend delivery audit SHA:
  `376d2b742f8dd2cccb0777e55fd87b6517a9a11ea3f07f8b7d45e3657ea4e1b4`
- Release approval intake SHA:
  `fdd61f009edcf83c7c849dda45a712af994889d9d222d32278fe3de0b05d4294`
- r71 backend delivery receipt SHA:
  `255874065fa7b3ed88ead1a4ed2a2551774796f8b07994f15e4304b56de1624d`
- r71 release approval SHA:
  `7df6bca5d79b50b2cb8b79e184255e650edd86b8590ddcaa1600a5769f4d3c11`
- Delivery receipt valid: true; backend delivery claim ready: true.
- Release approval valid: true; release approval claim ready: true.
- Remaining risk/future blocker count: 4.
- Release claim blockers: signed/notarized/stapled artifact missing, public
  distribution artifact not written, and real backend receipt missing.
- OpenClaw cross-agent send to `hepta-backend` remained forbidden by
  `tools.sessions.visibility=tree`; visible agents still only include
  `hepta-ui`.
- Signing capability remains blocked: zero valid code signing identities, no
  Apple notary env credentials, `notarytool` and `cargo-packager` available.
- Verification passed: full product gate, final readiness self-check, strict
  jq, `bash -n`, `git diff --check`, trailing-whitespace scan, OpenClaw
  visibility probe, and signing/notary capability probe.
- r71 did not perform signing, notarization, stapling, public upload, Matrix
  login, Gateway/provider/channel calls, backend repo writes, live runtime
  mutation, or external mutation.

## r72 cross-agent visibility and signing capability preflight

r72 adds a targeted local preflight for the remaining cross-lane blockers. It
does not replace the r71 full product gate or the r69b full-hard visual proof.

- Targeted artifact:
  `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.cross-agent-visibility-r72-20260619`
- Gate report:
  `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.cross-agent-visibility-r72-20260619/ui-cross-agent-visibility-gate.json`
- Gate status: `ready`
- Gate SHA:
  `0a8692139cd83987bae904f1974d4c1acf7cd6e99c9c824be8a51cd8d88d86cf`
- Markdown SHA:
  `3762b32a9ac375220c676ec2244419335c5bafca980baef965f78bcd4bc8a256`
- Local all-agent session store SHA:
  `0f9308fcbc836cec939f6aba2d9ddf0d34c9af4e4e037d90a3d03a068d589005`
- `openclaw sessions --all-agents` sees at least one `hepta-backend` session,
  but the controlled tool surface still exposes only `hepta-ui`.
- `sessions_send` to `hepta-backend` remains `forbidden` because the current
  session tool visibility is `tree`; the captured evidence SHA is
  `942eaf1ec963fd49cc43fc8a607030db54aa2fb6e00abd47f81e8726f01c3e8d`.
- The attempted config patch for `tools.sessions.visibility=all` and
  `tools.agentToAgent.allow=["hepta-backend"]` was rejected as protected config;
  the captured evidence SHA is
  `f86c2481bdacd6acc352f6914ab35d72e4da87fd07f366d659a3bfc31fef94c8`.
- Signing/notary capability remains blocked: zero valid code signing
  identities, no Apple notary/signing env readiness, `notarytool`, `stapler`,
  and `cargo-packager` available.
- Remaining true blockers are host-level OpenClaw cross-agent visibility,
  real backend receipt, signed/notarized/stapled artifact, and public
  distribution artifact.
- r72 did not send a backend session message successfully, mutate a backend
  repo, sign, notarize, staple, upload, publish, log into Matrix, call
  provider/channel surfaces, perform live runtime mutation, or perform external
  mutation.

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
