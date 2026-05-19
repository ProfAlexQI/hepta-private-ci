# Hepta Control UI P0-P29 Engineering Closure

Date: 2026-04-27
Status: local deterministic engineering hardening complete

## Scope

This closes the post-P21 engineering/productization lanes:

- P22: real ESM/build split advertised by `index.html`, imported by the deterministic build gate, and verified by `hepta-control-ui-build.mjs`.
- P23: smoke suite decomposition into contract, marker, browser/release/maturity/hardening/schema/soak/a11y suites.
- P24: cross-browser readiness matrix covering Chromium, Firefox, and WebKit candidates with deterministic local pass/skip accounting.
- P25: perceptual visual diff using screenshot average-hash baselines.
- P26: JSON schema contract gates for `/control-ui`, `/ui-contract-audit`, chat, task, and events surfaces.
- P27: UI soak/leak regression covering pane switching, endpoint chaos, typing preservation, DOM count, and localStorage growth.
- P28: task/artifact result drawer polish for the active task/result.
- P29: full accessibility rule snapshot: heading hierarchy, named controls, tablist state, focus/keyboard journey, and artifact keyboard path.

## Evidence

Run:

```bash
./scripts/hepta-control-ui-smoke.sh
```

Additional outputs:

- `target/hepta-control-ui-build/manifest.json`
- `target/hepta-control-ui-cross-browser-smoke/manifest.json`
- `target/hepta-control-ui-perceptual-smoke/manifest.json`
- `target/hepta-control-ui-soak-smoke/manifest.json`
- `target/hepta-control-ui-a11y-snapshot/snapshot.json`

## Result

Expected audit state:

```text
audit=100 p0=True ... p29=True p0p29=True convergence=100
```

Boundary remains unchanged: local-first, static, no hosted SaaS claim, and mutation paths remain confirm/dry-run gated.
