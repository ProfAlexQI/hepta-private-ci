# Hepta UI P0/P1 closeout — 2026-07-11

This note freezes the repeatable acceptance facts for the UI/product lane after
the 2026 shallow light tempered-glass pass. It does not promote fixture-only
contracts to live backend functionality and it does not claim public-GA
readiness.

## Source state

- Branch: `ui/native-productization`
- Audit base commit: `32451d0479701b89191f9a65a5fe656550a579c0`
- Design contract: `HEPTA_UI_LIGHT_TEMPERED_GLASS_STANDARD_2026.md`
- Full local evidence directory:
  `/Users/qianqi/.openclaw/tmp/hepta-ui-audit.20260711T1220`

## Current-source acceptance

- Control UI v3: 4 viewports, 56 screenshots, 26 popover targets, 0 failures.
- Control UI v8: 130 lifecycle steps, 104 screenshots, 0 failures.
- Control UI v9: 44 cross-popover transitions, 0 failures.
- Control UI v30: 21 semantic audits, 0 failures.
- Control UI v34: 30 full-frame screenshots and 509 optical crops, 0 failures.
- Control UI v41: 509 total crops, 325 interactive crops and 128
  cross-viewport groups, 0 failures.
- Native fixture: `ready`, 41 screenshots, 15 secondary-surface cases,
  57 action instances, minimum contrast ratio 4.89.
- Native `cargo check`: passed.
- Shell syntax checks and `git diff --check`: passed.
- Productization blocker rollup: `ready` with in-progress artifacts.

## True Makepad window evidence

The current Makepad binary was captured from real macOS windows with the
`product-shell` fixture route. This replaces the deleted June screenshot
dependency.

| Window | Size | Light-glass probe | SHA-256 |
| --- | ---: | --- | --- |
| Desktop | 1200×720 | luminance 244.85; mid 0.0332; cyan accent 0.0095 | `d537e2fbedfd5dae23614f12ae3f6fa77bcb03bbac61aa88f784f152ecb1c447` |
| Mobile | 500×720 | luminance 244.01; mid 0.0325; cyan accent 0.0074 | `adb7641b2b2cf0b66311d37c31f4de240ac75be78064b9db40de5f0327a60fce` |

The window report records `true_window_capture_performed: true`, `status:
ready`, and no Matrix login, gateway call, provider invocation, channel
delivery, or external mutation.

## Intentionally open boundaries

- Search, upload, media, voice, account and other live adapters remain owned by
  the backend-contract lane. UI contract coverage is not live-function proof.
- Signing, notarization, stapling and public distribution require Apple
  credentials plus an explicit release decision. `public_ga_ready` remains
  false until that workflow is executed and verified.
- VoiceOver/TalkBack, RTL, platform dynamic type and low-power device GPU,
  memory, frame-time and power measurements still require real-device runs.
- The v41 design gate is an internal frozen contract, not an independent
  third-party aesthetic certification.

## Maintenance follow-up

The visual and interaction gates are green, but the Control stylesheet still
contains historical override layers and the Control/Native token mapping is
not yet generated from a single source. Those are architecture improvements,
not reasons to falsify the current UI readiness result; they should be handled
as reviewable, behavior-preserving follow-up changes with the same screenshot
matrix.
