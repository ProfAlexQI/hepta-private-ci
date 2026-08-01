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

1. Environment texture is atmospheric only: 20–30% visual opacity.
2. Content-bearing panels use 85–94% light-surface opacity.
3. Inputs and menus use 90–96% opacity so text remains stable over texture.
4. Navigation, controls, and floating menus may retain blur, cyan edge light,
   and a restrained inner highlight.
5. Content cards must not repeat a full-strength water or prismatic texture.
6. Cyan glow is a focus/edge cue, not a fill. Mobile glow and pill density
   should remain 20–30% below the July 2026 audit baseline.
7. At most one glass surface in a local group should be visually dominant;
   message content, chrome, composer, and status badges must not all compete.

## Shared semantic tokens

`design-tokens/hepta-light-glass.tokens.json` is the single source. Run
`scripts/hepta-ui-light-glass-token-sync.rb` after changing it, and use
`--check` in CI or local gates. Generated CSS and Rust files must not be edited
by hand.

| Semantic role | Control UI | Hepta Native |
| --- | --- | --- |
| Environment | `--hepta-glass-environment: #eef5f7ff` | `COLOR_HEPTA_GLASS_ENVIRONMENT: #EEF5F7FF` |
| Content panel | `--hepta-glass-panel: #e8eff1f0` | `COLOR_HEPTA_GLASS_PANEL: #E8EFF1F0` |
| Input/menu | `--hepta-glass-input: #f1f5f5f2` | `COLOR_HEPTA_GLASS_INPUT: #F1F5F5F2` |
| Primary text | `--hepta-glass-text: #142a32ff` | `COLOR_HEPTA_GLASS_TEXT: #142A32FF` |
| Muted text | `--hepta-glass-muted: #506575ff` | `COLOR_HEPTA_GLASS_MUTED: #506575FF` |
| Dim text | `--hepta-glass-dim: #566a78ff` | `COLOR_HEPTA_GLASS_DIM: #566A78FF` |
| Hairline | `--hepta-glass-hairline: #a5ccd7b8` | `COLOR_HEPTA_GLASS_HAIRLINE: #A5CCD7B8` |
| Focus/accent | `--hepta-glass-focus: #0f7290ff` | `COLOR_HEPTA_GLASS_FOCUS: #0F7290FF` |
| Secondary accent | `--hepta-glass-secondary-accent: #14b8a6ff` | `COLOR_HEPTA_GLASS_SECONDARY_ACCENT: #14B8A6FF` |
| Content shadow | `--hepta-glass-shadow: #1730471f` | `COLOR_HEPTA_GLASS_SHADOW: #1730471F` |

## Accessibility floor

- Body/message text: 13–14 px/pt minimum in primary reading paths.
- Timestamp and compact state text: 11 px/pt minimum.
- Interactive target: 44×44 px/pt minimum.
- Text contrast: WCAG AA, at least 4.5:1 for normal text; the shared visual
  acceptance chain keeps a 4.8:1 safety margin for its weakest sampled text,
  including the dim token against environment, panel, and input surfaces.
- Keyboard focus must remain visible.
- Control UI supports reduced motion, reduced transparency, increased contrast,
  and forced-colors modes.
- Root document direction must follow the active locale, and desktop/mobile layouts must remain
  usable when mirrored. `dir="auto"` is only a source-level prerequisite; it is not RTL evidence.
  Native timestamps use an unambiguous 24-hour numeric format until locale
  selection is wired through Makepad.

## Required evidence

- Control UI: 1365×900, 768×900, 500×844, and 320×844.
- Native fixture: desktop, mobile, phone, phone320, secondary surfaces, safe
  area, and keyboard-open states.
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
