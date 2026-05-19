# Hepta Control UI P0-P13 Maturity Closure

Date: 2026-04-27
Status: local deterministic maturity complete

## Scope

This closes the seven post-P0-P6 optimization lanes as explicit gates:

- P7: content-rich seeded visual regression, not only empty-state screenshots.
- P8: mobile layered UX: Chats → Thread → Room instead of stacked cramped panels.
- P9: compact header/status chrome with overflow details popover.
- P10: consolidated empty Workspace Room onboarding state.
- P11: real user journey E2E: artifact citation, dry-run plan, endpoint retry.
- P12: module-boundary governance for the static bundle.
- P13: browser-level accessibility sanity checks.

## Evidence

Run:

```bash
node scripts/hepta-control-ui-maturity-smoke.mjs
./scripts/hepta-control-ui-smoke.sh
```

The maturity smoke writes:

- `target/hepta-control-ui-maturity-smoke/rich-desktop.png`
- `target/hepta-control-ui-maturity-smoke/rich-mobile-room.png`
- `target/hepta-control-ui-maturity-smoke/manifest.json`

## User journey covered

1. Seed a rich multi-agent room with messages, grouped replies, members, tasks, activity, endpoint degradation, and artifact preview.
2. Verify the desktop room shows members, tasks, artifact preview, activity log, compact header status, and detail popover.
3. Insert a task evidence citation into the composer.
4. Render and review a dry-run apply plan without executing mutation.
5. Open Developer and verify degraded endpoint retry affordance.
6. Collapse empty Workspace Room into one onboarding card plus boundary notes.
7. On mobile, switch between Chats, Thread, and Room layers.
8. Check visible buttons, dialog semantics, selected tab state, and reduced-motion CSS.

## Module boundary

See `apps/hepta-control-ui/modules/README.md` for static-bundle module ownership:

- chat-state
- chat-render
- workspace-room
- live-data
- task-actions
- browser-fixtures
- accessibility

## Result

Expected audit state:

```text
audit=100 p0=True ... p13=True p0p13=True convergence=100
```

Boundary remains unchanged: the UI is local-first and not a hosted SaaS surface.
