# Hepta light tempered-glass standard (2026)

This document is the shared visual contract for Control UI and Hepta Native.
It describes product UI, not runtime or backend behavior.

## Material hierarchy

1. Environment texture is atmospheric only: 20–30% visual opacity.
2. Content-bearing panels use 85–94% light-surface opacity.
3. Inputs and menus use 90–96% opacity so text remains stable over texture.
4. Navigation, controls, and floating menus may retain blur, cyan edge light,
   and a restrained inner highlight.
5. Content cards must not repeat a full-strength water or prismatic texture.

## Shared semantic tokens

| Semantic role | Control UI | Hepta Native |
| --- | --- | --- |
| Environment | `--bg: #eef5f7` | `COLOR_TELEGRAM_BG: #F4F9FCE8` |
| Content panel | `--card: #ffffffe8` | `COLOR_TELEGRAM_PANEL: #FFFFFFE8` |
| Input/menu | `--input: #fffffff0` | `COLOR_TELEGRAM_INPUT: #FFFFFFF0` |
| Primary text | `--text: #142a32` | `COLOR_TELEGRAM_TEXT: #132332` |
| Muted text | `--muted: #506575` | `COLOR_TELEGRAM_MUTED: #506575` |
| Hairline | `--border: #b9dce6cc` | `COLOR_TELEGRAM_GLASS_HAIRLINE: #B9DCE6CC` |
| Focus/accent | `#006f86 / #14b8a6` | `COLOR_ROBRIX_CYAN: #14B8A6` |

## Accessibility floor

- Body/message text: 13–14 px/pt minimum in primary reading paths.
- Timestamp and compact state text: 11 px/pt minimum.
- Interactive target: 44×44 px/pt minimum.
- Text contrast: WCAG AA, at least 4.5:1 for normal text.
- Keyboard focus must remain visible.
- Control UI supports reduced motion, reduced transparency, increased contrast,
  and forced-colors modes.
- Root document direction is automatic. Native timestamps use an unambiguous
  24-hour numeric format until locale selection is wired through Makepad.

## Required evidence

- Control UI: 1365×900, 768×900, 500×844, and 320×844.
- Native fixture: desktop, mobile, phone, phone320, secondary surfaces, safe
  area, and keyboard-open states.
- Open menus must be produced by native user activation, not by injecting
  classes or `open` attributes in the visual referee.
- A missing historical `/tmp` screenshot must never block current-source
  readiness. Current true-window evidence is either ready, explicitly disabled,
  or an allowed local lock/permission block.

## Deferred beyond P1

Full runtime locale selection, RTL layout mirroring in Makepad, platform screen
reader node exposure, signed/notarized distribution, and backend-owned live
mutations remain separate promotion work.
