# Hepta UI v4 next tranche — platform/runtime compatibility

Date: 2026-08-27  
Status: `SOURCE_IMPLEMENTATION_IN_PROGRESS_RUNTIME_QUALIFICATION_REQUIRED`

## Source bindings

- Plan snapshot commit: `fe0889ecd46a5fc89de7b1ff3f28158c133a3502`
- Canonical source head: `a85612afb43af722c61b54efe73570b25e9e4031`
- Canonical source tree: `71026adff61523660d953867188f094184cee2e9`
- UI base: `647e294522a3b3341b4169e3f5a85f8f0df42cbe`
- Development branch: `codex/ui-light-glass-v4-20260827`

This tranche grants no production, effect, live-adapter, operator-acceptance,
promotion or release authority.

## Implemented in this tranche

### Control UI runtime compatibility

The Rust-served stylesheet aggregate historically concatenates the
accessibility tail directly after responsive rules. The critical v4 invariants
are therefore mirrored into the beginning of `styles.accessibility.css` as a
bounded compatibility bridge while all system preference rules remain final.

The bridge enforces:

- 15px body/message typography and 12px metadata;
- zero backdrop blur for stable reading and form content;
- 44px precise-pointer and 48px coarse-pointer/mobile targets;
- safe-area mobile bottom-sheet geometry;
- no background scrolling while an explicit v4 transient lifecycle is active;
- final reduced-transparency, reduced-motion, increased-contrast and
  forced-colors fallbacks.

The bridge is not a replacement for the canonical follow-up that adds
`styles.v4.css` as its own Rust aggregate input. That follow-up remains required
and should remove duplicated compatibility declarations after source-bound
browser evidence exists.

### Native platform material decision layer

`apps/hepta-native/src/shared/hepta_platform_material.rs` defines bounded
renderer intents:

| Platform | Environment | Chrome | Transient |
|---|---|---|---|
| Windows | Mica intent | Mica intent | Acrylic intent |
| macOS | Solid | system chrome material | system popover material |
| iOS | Solid | system glass intent | system sheet intent |
| Android | Solid | Material tonal chrome | Material tonal sheet |
| Web | Solid | bounded backdrop chrome | bounded backdrop transient |
| Linux/unknown | Solid | Solid | Solid |

Stable content is always `Solid`; disabling transparency returns a fully solid
profile; the maximum visible backdrop count is two. No native API is called by
this source decision layer.

### Migration and governance records

- `HEPTA_NATIVE_CONTROL_MIGRATION_V1.json` inventories concrete 44/45px controls
  still requiring direct migration and runtime measurement.
- `HEPTA_UI_Z_INDEX_REGISTRY_V1.json` defines the only approved elevation bands.
- `HEPTA_PLATFORM_MATERIAL_ADAPTER_MATRIX_V1.json` separates source intent from
  runtime adapter completion.
- `HEPTA_UI_V4_CI_BLOCKER_2026-08-27.json` records runner non-assignment without
  treating it as a test result.
- `scripts/hepta-ui-v4-next-source-gate` verifies this tranche while explicitly
  keeping browser, Native runtime and device validation false.

## Immediate remaining implementation

1. Add `styles.v4.css` as a dedicated input to `CONTROL_UI_STYLES_CSS`, then
   remove the compatibility duplication after equivalent screenshots pass.
2. Add digest-bound JavaScript lifecycle for focus entry/return, Tab containment,
   Escape/back dismissal and background scroll locking.
3. Migrate `RobrixTextInput` and mobile stack header concrete defaults to 48
   logical pixels and measure actual hit rectangles.
4. Implement platform API adapters behind the source profile without changing
   authority state.
5. Capture the source-bound desktop/mobile matrix before marking any visual row
   `PASS`.

## Exit criteria

This tranche may leave draft only when:

- source gates run on an assigned runner;
- the Rust aggregate and browser asset digests are exact;
- Windows/macOS/Android/iOS fallbacks are independently exercised;
- 320px reflow, 200% font scale, keyboard/safe-area and reduced-transparency
  evidence is attached;
- production/effect/promotion values remain false unless separately authorized.
