# Hepta Control UI P0-P21 Hardening Closure

Date: 2026-04-27
Status: local deterministic hardening complete

## Scope

This closes the post-P13 hardening lanes:

- P14: visual diff baseline for rich desktop layout.
- P15: Workspace Room accordion/priority compression.
- P16: mobile compact composer.
- P17: browser performance budget for first render and command palette latency.
- P18: accessibility audit upgrade: keyboard focus, dialog focus trap, tab journey, contrast sanity.
- P19: real module split sidecars under `apps/hepta-control-ui/modules/`.
- P20: endpoint chaos regression: slow, partial failure, stale cache, retry affordances.
- P21: README/documentation density cleanup.

## Evidence

Run:

```bash
node scripts/hepta-control-ui-hardening-smoke.mjs
./scripts/hepta-control-ui-smoke.sh
```

The hardening smoke writes:

- `target/hepta-control-ui-hardening-smoke/manifest.json`

## Visual baseline

The layout baseline lives at:

- `apps/hepta-control-ui/baselines/visual-layout-baseline.json`

It tracks the rich desktop rail/thread/room panel geometry and key content counts with tolerances.

## Result

Expected audit state:

```text
audit=100 p0=True ... p21=True p0p21=True convergence=100
```

Boundary remains unchanged: local-first, static, no hosted SaaS claim, and mutation paths remain confirm/dry-run gated.
