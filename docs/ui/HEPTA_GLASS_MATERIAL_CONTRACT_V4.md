# Hepta Glass Material Contract v4

Status: `IMPLEMENTATION_CONTRACT_NO_RELEASE_AUTHORITY`  
Date: 2026-08-27  
Machine source: `design-tokens/hepta-material-v4.contract.json`

## Contract

Hepta uses translucency as a functional hierarchy, not as decoration. The
interface has five material roles:

1. **Environment** — the window/page field. Opaque, no blur.
2. **Content** — messages, tables, logs, forms, evidence and long-form reading.
   Opaque or nearly opaque, no backdrop blur.
3. **Chrome** — navigation rails, top bars and composer chrome. At most one
   persistent glass layer at a pixel.
4. **Transient** — one open popover, menu, command palette or sheet. It may use
   stronger blur and shadow, but may not contain another glass surface.
5. **Fallback** — fully opaque replacement whenever platform support,
   accessibility preferences, performance or contrast require it.

## Hard limits

- Visible backdrop layers: `<= 2`.
- Stable-content backdrop layers: `0`.
- Stable scroll containers may not use backdrop blur.
- Nested transient glass is forbidden.
- A mobile transient is a bottom sheet by default.
- A wide-screen transient may be an anchored popover.
- Hard-coded screen coordinates are not an accepted positioning contract.
- Message/body text is at least 15 logical pixels.
- Metadata is at least 12 logical pixels.
- Touch targets are at least 48 logical pixels.
- Reducing font size is not an allowed overflow fix.

## Web mapping

Web may use bounded `backdrop-filter` for chrome and the active transient only.
Messages, thread content, forms, tables and inputs use stable no-blur surfaces.
`prefers-reduced-transparency`, forced colors, increased contrast and reduced
motion each have a complete fallback. Browser support for blur is optional;
layout, state and contrast are not optional.

## Windows mapping

Use one Mica-like application backdrop for the window environment. Use
Acrylic-like material only for temporary menus or flyouts. Do not layer
persistent Acrylic panels. Transparency Effects off and High Contrast must
produce solid surfaces without changing information architecture.

## macOS mapping

Use system window/sidebar/titlebar material for chrome where appropriate and
system popover material for transient UI. Content remains stable. Reduce
Transparency and Increase Contrast must be honored without an application
restart.

## iOS mapping

Use system navigation/control glass only where the platform supports it and
only for functional chrome. Do not reproduce Liquid Glass by stacking custom
blurred cards. Use system sheets and menus for transient UI and grouped solid
surfaces for content. Dynamic Type 200%, Reduce Transparency and Reduce Motion
are qualification targets.

## Android mapping

Use Material 3 dynamic/tonal surfaces and tonal elevation. Android does not
copy iOS glass. Navigation and app bars may use tonal chrome; transient actions
use modal or bottom-sheet elevation; content is solid. Minimum touch target is
48dp and Remove Animations must preserve meaning.

## Authority boundary

Material choice never grants capability or authority. A primary-colored,
glass, raised, or animated control remains disabled or prepare-only unless the
state/authority matrix grants the required capability and an independently
verified receipt grants the effect. This contract keeps production, effect,
live-adapter, operator-acceptance, promotion and release flags false.
