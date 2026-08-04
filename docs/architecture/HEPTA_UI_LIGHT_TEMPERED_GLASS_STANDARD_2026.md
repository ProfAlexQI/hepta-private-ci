# Hepta light tempered-glass standard (2026)

This document is the shared visual contract for Control UI and Hepta Native.
It describes product UI, not runtime or backend behavior.

Hepta's “light tempered glass” is a project design language, not an external
certification. Its current platform reference is Apple's Liquid Glass guidance
and current design resources; Hepta deliberately uses a quieter material on
content-bearing chat surfaces than on navigation, controls, and popovers.

- <https://developer.apple.com/documentation/TechnologyOverviews/adopting-liquid-glass>
- <https://developer.apple.com/design/resources/>

## Material hierarchy

1. Environment is a fully opaque atmospheric substrate; texture remains a
   restrained 20–30% visual modulation inside that substrate.
2. Stable Content is 98% opaque, never blurred, and owns messages, lists, and
   ordinary information cards.
3. Glass Chrome is 88% opaque with a 14 px blur and is reserved for rails,
   headers, and the composer.
4. Floating Glass is 94% opaque with a 20 px blur and is reserved for popovers
   and transient controls.
5. Content cards must not repeat a full-strength water or prismatic texture.
6. Cyan glow is a focus/edge cue, not a fill. Mobile glow and pill density
   should remain 20–30% below the July 2026 audit baseline.
7. At most two backdrop-filter layers may be simultaneously visible, and
   Stable Content contributes zero backdrop layers. At most one glass surface
   in a local group should be visually dominant;
   message content, chrome, composer, and status badges must not all compete.

## Shared semantic tokens

`design-tokens/hepta-light-glass.tokens.json` schema v3 is the single source.
The default command, `scripts/hepta-ui-light-glass-token-sync.rb`, is read-only
and equivalent to `--check`. Regeneration is an explicit
`scripts/hepta-ui-light-glass-token-sync.rb --write`; generated CSS and Rust
files must not be edited by hand.

Schema v3 adds the normative Environment / Stable Content / Glass Chrome /
Floating Glass material contract to the shared color roles. Text,
focus, and the secondary accent are shared. Native and Control retain separate
surface roles because their Makepad and browser compositors have different
opacity and layering requirements. A renderer-specific value is therefore not
silent drift: it is named and reviewed in the canonical source.

| Semantic role | Control UI | Hepta Native |
| --- | --- | --- |
| Environment | `--hepta-glass-environment: #eef5f7ff` | `COLOR_HEPTA_ENVIRONMENT: #EEF3F5FF` |
| Content surface | `--hepta-glass-panel: #e8eff1f0` | `COLOR_HEPTA_CONTENT: #FCFDFEFF` |
| Glass chrome | `--hepta-glass-panel: #e8eff1f0` | `COLOR_HEPTA_GLASS: #EAF1F3EC` |
| Input/menu | `--hepta-glass-input: #f1f5f5f2` | `COLOR_HEPTA_INPUT: #F8FAFBF8` |
| Primary text (shared) | `--hepta-glass-text: #142a32ff` | `COLOR_HEPTA_TEXT: #142A32FF` |
| Muted text | `--hepta-glass-muted: #506575ff` | `COLOR_HEPTA_MUTED: #566A78FF` |
| Dim text | `--hepta-glass-dim: #566a78ff` | `COLOR_HEPTA_DIM: #5C6E79FF` |
| Hairline | `--hepta-glass-hairline: #a5ccd7b8` | `COLOR_HEPTA_HAIRLINE: #A7C5CF99` |
| Focus/accent (shared) | `--hepta-glass-focus: #0f7290ff` | `COLOR_HEPTA_FOCUS: #0F7290FF` |
| Success | `--hepta-glass-success: #128a61ff` | `COLOR_HEPTA_SUCCESS: #137A5AFF` |
| Shadow | `--hepta-glass-shadow: #1730471f` | `COLOR_HEPTA_SHADOW: #1730471A` |

## Accessibility floor

- Body/message text: 13–14 px/pt minimum in primary reading paths.
- Timestamp and compact state text: 11 px/pt minimum.
- Interactive target: 44×44 px/pt minimum.
- Text contrast: WCAG AA, at least 4.5:1 for normal text. Control keeps a
  4.8:1 safety margin for its dim token against environment, panel, and input;
  Native's renderer-specific dim/surface combinations remain at or above 4.5:1.
- Keyboard focus must remain visible.
- Control UI supports reduced motion, reduced transparency, increased contrast,
  and forced-colors modes.
- Root document direction must follow the active locale, and desktop/mobile layouts must remain
  usable when mirrored. `dir="auto"` is only a source-level prerequisite; it is not RTL evidence.
  Native timestamps use an unambiguous 24-hour numeric format until locale
  selection is wired through Makepad.

## Required evidence

- Control UI: 1365×900, 768×900, 500×844, and 320×844.
- Native: current-HEAD Makepad GPU frames for desktop and mobile, plus explicit
  safe-area and keyboard-open evidence on the relevant platform. HTML fixtures
  may diagnose layout, but cannot promote Native product readiness.
- Open menus must be produced by native user activation, not by injecting
  classes or `open` attributes in the visual referee.
- Opening a second auto popover closes the first; Escape closes the current
  popover and restores a usable focus path.
- A missing historical `/tmp` screenshot must never block current-source
  readiness. Current true-window evidence is either ready, explicitly disabled,
  or an allowed local lock/permission block.

## External, device, and release validation boundary (not claimed)

Full runtime locale selection, RTL layout mirroring in Makepad, platform screen
reader node exposure, real-device dynamic type and low-power performance,
signed/notarized/stapled distribution, and backend-owned live mutations remain
open promotion boundaries. They are not completed by this UI implementation or
by fixture/desktop-window evidence and must not be reported as closed until the
corresponding device, credentialed release, or backend lane executes them.
