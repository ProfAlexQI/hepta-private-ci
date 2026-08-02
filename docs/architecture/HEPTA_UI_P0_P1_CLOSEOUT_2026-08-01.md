# Hepta UI P0/P1 closeout — 2026-08-01

> **Historical, superseded architecture.** This closeout describes the earlier
> selective-six-module Native approach on `ui/native-productization`. The
> current Native direction is upstream-first Robrix and this report cannot
> promote current source. The canonical entry point is
> `scripts/hepta-ui-current-readiness.sh`.

This note freezes the current-source acceptance facts for the 2026 desktop and
mobile UI productization pass. It distinguishes real browser and Makepad-window
evidence from HTML fixtures, and it does not promote UI contracts to live
backend functionality or public-GA readiness.

## Source and decision record

- Branch: `ui/native-productization`.
- Audit base: `a0edebd37af6b9e8d4621c7f970a06af91bd2b23`.
- Implementation commit:
  `39a5f6abee0c78e77155d1a6e292a27e7cbebe11`.
- Design contract:
  `docs/architecture/HEPTA_UI_LIGHT_TEMPERED_GLASS_STANDARD_2026.md`.
- Canonical token source:
  `design-tokens/hepta-light-glass.tokens.json`.
- The Control surface remains Rust-rendered and JavaScript-free. No React or
  Next.js runtime was introduced.
- The accepted Native upstream is Robrix under MIT. Six modules were
  selectively adapted from exact commit
  `a5a664da569c577ab1a3e5a33f45dcc9364954a0`; no whole-tree merge was
  performed.

Apple calls its system language **Liquid Glass**. “Light tempered glass” is
Hepta's internal product language, not an Apple or independent-industry
certification. The implementation contract is therefore deliberately
measurable: stable content planes, restrained material on navigation and
floating controls, preserved contrast and focus, and no large-area white
clipping.

## Landed implementation

### Control UI

- Restored native `popover="auto"` disclosure for conversation actions,
  thread tools, composer tools, attachment, command and command-palette
  surfaces. Auto popovers provide mutual exclusion, light-dismiss and Escape
  behavior without a trusted client script.
- Removed the obsolete hash-driven command-palette path and retained
  `<details>` only for genuine diagnostic disclosure.
- Added native mobile Chats / Thread / Room fragment navigation with exactly
  one task pane visible at a time, usable Room content, focus routing, and no
  horizontal overflow at 500- and 320-pixel widths.
- Corrected conversation and popover semantics to native interactive elements
  with `list` / `listitem`, `aria-current`, `role="group"`, labels, titles and
  44-pixel targets.
- Split the active stylesheet into six ordered layers: generated tokens,
  legacy compatibility, foundation, components, responsive behavior and
  accessibility. The import manifest is `styles.css`; the runtime renderer
  concatenates the same six files.
- Restored `prefers-contrast`, `forced-colors`,
  `prefers-reduced-transparency` and `prefers-reduced-motion` handling.
- Removed the retired `assets/k.png` runtime reference. The old file may remain
  in the repository for history, but the active UI does not request it.

The large `styles.legacy.css` compatibility layer remains an explicit
maintenance boundary. This pass reduces and budgets active overrides without
claiming that every historical selector was clean-room rewritten.

### Shared design system

- `scripts/hepta-ui-light-glass-token-sync.rb` generates the Control CSS and
  Native Makepad/Rust token files from one JSON source and supports a strict
  `--check` mode.
- The current hard budgets are less than 300,000 runtime CSS bytes and no more
  than 2,100 `!important` occurrences, down from the audited 2,913 baseline.
- The screenshot census keeps a hard maximum of `0.75` for pixels with simple
  luminance at or above 245. This limit was not relaxed to make existing
  captures pass; Control and Native surfaces were retuned and recaptured.
- UI scripts select Rust `1.95.0` explicitly, avoiding the host's older default
  toolchain and the SQLx minimum-supported-Rust mismatch.
- Browser gates create isolated runtime outcome, preference and integrity
  fixtures, so current-source tests no longer depend on missing machine-local
  databases.

### Native and Robrix intake

The six selectively adapted modules are:

1. `slash_commands.rs`: `/html` and `/plain` parsing and Matrix message
   construction, not send dispatch.
2. `file_upload_modal.rs`: explicit local-path metadata inspection and a
   bounded 128 KiB text preview, not picker, upload or Matrix send.
3. `upload_progress.rs`: deterministic presentation state, not queue control.
4. `attachment_download.rs`: MXC and download/share presentation state, not
   file I/O, share sheet or worker submission.
5. `mention_popup.rs`: local vocabulary, insertion text and keyboard selection,
   not remote ranking, avatars or submission.
6. `room_input_popup_menu.rs`: registered 44-point Makepad popup widget,
   deliberately dormant until it can preserve Hepta's confirmation-first
   composer contract.

Exact provenance, modification status and the current upstream copyright are
recorded in the Robrix copy manifest, notice and preserved MIT license. The
bounded intake adds no provider, gateway, runtime mutation or live backend
behavior.

## Candidate-project decision

- **Robrix** was selected for Native because Hepta already carries its Matrix
  heart and Makepad substrate. The intake is per-module and provenance-locked.
- **assistant-ui** remains the best interaction-spec reference for a future
  trusted-client Control experiment, but importing it would be a deliberate
  React architecture migration and was not mixed into this no-JavaScript pass.
- **Vercel Chatbot** and **NextChat** remain information-architecture and
  responsive-shell references; their Next.js/React stacks were not embedded in
  the Rust renderer.
- **liquid-glass-react** was not imported. The current deficit was hierarchy and
  interaction correctness, not a shortage of optical distortion, and a full
  surface effect would add browser and GPU risk.

## Current-source acceptance

`scripts/hepta-ui-current-source-readiness-gate.sh` is the authoritative UI-lane
finalizer for this pass. It verifies current source fingerprints, report hashes,
physical screenshot bytes and SHA-256 values, while keeping backend and release
boundaries hard-false. The final r3 evidence root is
`/Users/qianqi/.openclaw/tmp/hepta-ui-product-final-r3.20260801T1904`. A durable
review subset, report, manifest and checksum receipt are stored in
`docs/architecture/assets/hepta-ui-2026-08-01/`. The frozen readiness-report
SHA-256 is
`7d978b420c13a4cf1340d53f7beaa58719433ce636654ebce165921844698506`.

- Control browser coverage is ready at 1365×900, 768×900, 500×844 and 320×844.
  All four viewports satisfy the frozen eight-part shallow-light-glass contract.
- The Control v7 path used real browser clicks: 26 targets, 32 state screenshots,
  26 popover-switch steps, six mobile pane routes and zero failures. Fixture
  class/open injection is not accepted as proof for this result.
- Native fixture coverage is ready: 41 screenshots, 18 unique selected-row
  variants, four desktop plus four mobile route variants, 15 secondary scenes,
  57 action instances and a 4.84 minimum measured contrast ratio.
- Makepad current-source capture is ready across 20 physical window paths:
  two base windows, four desktop routes, four mobile routes, five desktop
  secondary surfaces and five mobile secondary surfaces. There are 19 unique
  image hashes because the base mobile Home window and the mobile Home-route
  capture intentionally select the same state.
- Mobile Makepad evidence honestly records a 390×844 host request producing a
  390×820 visible window. It proves the macOS-hosted narrow window layout, not a
  real-device safe area or software-keyboard result.
- The evidence manifest contains 65 screenshots, including 24 hard-validated
  key screenshots. Missing files, symlinks, out-of-root paths, byte drift and
  hash drift fail the finalizer.
- The design-system gate reports six ordered CSS layers, 287,610 runtime bytes,
  2,055 `!important` occurrences, zero active legacy texture references, all
  four accessibility media-query families, and a 4.841 shared dim-text minimum
  contrast ratio.
- The local unsigned macOS bundle is ready at 245,592,023 bytes. It is not a
  signed, notarized, stapled or public-distribution artifact.

The current-source report therefore records `status=ready`,
`ui_lane_ready=true`, `full_product_ready=false`,
`backend_live_adapter_ready=false` and `public_ga_ready=false`. The older
monolithic full-root chain remains `not_ready` because its top-design refresh
still requires retired extreme prismatic, caustic, pill and micro-glass fields.
Those old fields were not relabelled or visually reintroduced to manufacture a
green result.

## Verification performed

- Rust 1.95.0 Native `cargo check` passed; 12 focused Robrix-intake unit tests
  passed.
- Both focused `hepta-core` Control tests passed: asset-backed report
  completeness and current native-Popover/accessibility contract.
- `just fmt`, scoped `just fix -p hepta-core`, token generation `--check`, the
  Control Architecture V2 diagnostic smoke, design-system gate, current-source
  finalizer, all changed shell/Ruby syntax checks and `git diff --check` passed.
- A negative finalizer test with missing evidence failed closed as required.
- Independent evidence review confirmed 20 nonempty true-window files with
  report bytes/hashes matching disk and no unexplained duplicate paths or
  images.

## Explicitly open boundaries

- Search, upload, media, voice, account and related live adapters remain owned
  by the backend-contract lane. UI contract coverage and quarantined adapter
  models are not remote-function proof.
- VoiceOver/TalkBack traversal, system Dynamic Type, RTL mirroring, real mobile
  safe areas and keyboards, and low-power GPU, memory, frame-time and battery
  measurements require iOS/Android device-lab runs. Browser `dir="auto"` and
  simulated safe-area checks are prerequisites, not substitutes.
- The macOS package remains unsigned. Signing, notarization, stapling and public
  distribution require Apple credentials and explicit release authority;
  `public_ga_ready` remains false.
- The visual gates are frozen internal acceptance contracts, not independent
  third-party aesthetic certification.
