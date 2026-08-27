# Hepta UI v4 Tranche 6 — Native host consolidation

Date: 2026-08-28, Asia/Singapore  
Branch: `codex/ui-v4-native-host-consolidation-20260828`  
Stacked parent: PR #6 / `codex/ui-v4-native-material-runtime-20260828`  
Parent commit: `5f8f99a95c087b1a2e0987cde2f7a455a9c9eeee`  
Latest plan snapshot: `fe0889ecd46a5fc89de7b1ff3f28158c133a3502`

Status: `SOURCE_CONSOLIDATED_RUNTIME_VALIDATION_REQUIRED`

## Purpose

PR #5 and PR #6 were sibling branches on the same canonical bundle guard. PR #5
contained the Native material host lifecycle and mobile stack compatibility
migration. PR #6 contained the system-preference probes, capability-aware
material resolution, and the Windows DWM adapter. Leaving them as siblings
would allow either merge order to discard or conflict with the other half.

This tranche stacks on PR #6 and carries forward the non-duplicated, still-valid
parts of PR #5. PR #5 is therefore superseded once this candidate is opened.

## Delivered

### Consolidated fail-closed material host

`hepta_platform_material_host.rs` now composes the PR #6 runtime contracts:

- accepts a verified `HeptaSystemPreferenceSnapshot`;
- requires an explicit host capability before calling an OS adapter;
- requires both Android preference evidence and Activity capability evidence for
  dynamic color;
- uses the canonical `bind_material_runtime` validator;
- rejects an `Ok` receipt that did not bind a system material;
- rejects platform, renderer, or authority mismatches;
- unbinds before semantic-only, fallback, suspend, and shutdown states;
- makes shutdown terminal;
- keeps stable content solid and the backdrop budget bounded;
- keeps every authority field false.

Host phases are:

1. `SolidFallback`;
2. `SemanticIntentOnly`;
3. `SystemMaterialBound`;
4. `Suspended`;
5. `Shutdown`.

A host snapshot is evidence about material state only. It does not grant effect,
live-adapter, production, operator, promotion, or release authority.

### Mobile StackNavigation migration

The existing HomeScreen-compatible prototype remains owned by
`home_screen.rs`. A post-registration override now changes the compatibility
prototype to:

- 56 logical-pixel header;
- 48 logical-pixel back target;
- 20 logical-pixel back icon;
- body offset derived from the same 56px token;
- existing safe-inset padding retained.

This avoids copying the full HomeScreen tree and keeps the migration bounded.

### Control and layout token publication

The Rust constants now have script-side equivalents:

- `HEPTA_V4_CONTROL_MIN_HEIGHT = 48`;
- `HEPTA_V4_ICON_HIT_TARGET = 48`;
- `HEPTA_V4_MOBILE_HEADER_HEIGHT = 56`;
- `HEPTA_V4_MOBILE_BACK_TARGET = 48`;
- `HEPTA_V4_DESKTOP_RAIL_MIN_WIDTH = 280`;
- `HEPTA_V4_COMPACT_DESKTOP_BREAKPOINT = 980`;
- `HEPTA_V4_MOBILE_BREAKPOINT = 700`.

`HeptaV4RobrixTextInput` is provided as an explicit migration target. The legacy
`RobrixTextInput` name is deliberately **not** globally rebound because some
consumers still live in fixed 35–45px rows; rebinding only the child would create
clipping and false source compliance.

## Remaining component work

The machine-readable control matrix intentionally leaves these rows open:

- shared legacy `RobrixTextInput` at 44px;
- `RoomFilterInputBar` at 35px;
- its desktop 39px and mobile 45px containing rows;
- runtime hit-rectangle evidence for the migrated mobile stack;
- 320/360/390/412/600 visual captures;
- Dynamic Type / 200% text, keyboard-open, and landscape evidence.

The next component tranche must migrate each fixed-height row together with its
input and clear action; it must not use a global template rebind as a shortcut.

## Platform adapter boundary

Inherited from PR #6:

- Windows preference probe source;
- iOS accessibility preference probe source;
- Android dynamic-color semantic gate;
- Windows explicit-handle Mica/Acrylic adapter with rollback.

Still required:

- actual Makepad host object extraction;
- App startup/resume/background/shutdown wiring;
- macOS AppKit material adapter;
- iOS view/controller material adapter;
- Android Activity/JNI bridge;
- device screenshots and performance evidence.

`system_material_bound=true` remains impossible without a concrete adapter,
required host objects, successful system calls, and a validated receipt.

## CI and evidence boundary

Parent PR #6 workflow run `33101656274` and superseded PR #5 workflow run
`33097734095` returned jobs with `steps=[]` and no assigned runner. They are
classified as `BLOCKED_RUNNER_NOT_ASSIGNED`, not source failures and not PASS.

The new gate can emit only:

`PASS_NATIVE_HOST_CONSOLIDATION_SOURCE_ONLY`

Even on source-gate success, all of the following remain false:

- Native compile validation;
- Native runtime validation;
- Windows/macOS/iOS/Android runtime validation;
- device validation;
- production/effect/live-adapter authority;
- operator acceptance;
- promotion and release.

## Review order

1. PR #5/#6 consolidation and absence of duplicate lifecycle owners.
2. host receipt validation and rollback paths.
3. Android dual-evidence dynamic-color gate.
4. HomeScreen post-registration ordering.
5. 56px header and 48px back-target compatibility override.
6. deliberate non-rebinding of legacy fixed-height text inputs.
7. control/material matrices and source-only claim boundary.
