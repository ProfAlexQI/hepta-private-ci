# Hepta Upstream Codex Current Intake R6

## Observation

- The latest recorded upstream observation advanced from frozen R5
  `81da9deb065d` to `6c729ef1c1dc`.
- The R6 delta contains 2 non-merge commits, 11 net changed paths, 465
  insertions, 47 deletions, and 14 commit-level file touches.
- R5 is preserved byte-for-byte. R6 is a new immutable observation and claims
  zero imports at observation time.
- This is an offline latest-recorded intake. It does not prove that the network
  head remains unchanged after `2026-07-24T10:01:59Z`.
- Hepta and upstream still have unrelated roots and no merge base. Ordinary
  merge/rebase remains forbidden; any integration requires selective semantic
  transplant with behavioral evidence.

## Classification

- `6c729ef1c1dc`: candidate P1. Session authentication changes should invalidate
  MCP runtime projections, but runtime construction must use one coherent
  authentication snapshot and startup must not publish stale credentials.
- `ef2d3edb959a`: candidate P1. Background MCP prewarm may reduce model-step
  latency, but exact model-step refresh remains the correctness path and stale
  queued work must never publish.

## Ordered Intake

1. Audit and selectively absorb authentication-state invalidation and
   single-snapshot runtime construction.
2. Only then adapt the coalescing prewarm worker with stale-publication,
   latest-state, exact-refresh, and shutdown-join evidence.

## Frozen Evidence

- Predecessor manifest:
  `docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R5.json`
  (`dd56ea3130f035714fe14dcf25584161e669bb441a946548273a2ddec741e18f`).
- Frozen R5 ref:
  `refs/remotes/upstream/hepta-intake-20260724-r5` at
  `81da9deb065d7adb283816b19b40f89bcc484276`.
- Required R6 frozen ref:
  `refs/remotes/upstream/hepta-intake-20260724-r6` at
  `6c729ef1c1dcfbcbe1bd9d0c2dddde24377ae899`.
- Range digest:
  `049e178e5bb6190f59776a7f3ef6bf924e9e1a5a3c3cc70ba02de4d1acfcfda5`.
- Net path-surface digest:
  `6402b9e303d68b212faa4162047bd635919051d0ebb7facac6ec5e77445de934`.
- Commit identity digest:
  `2cb86a8276e7831da2174db0d5873147a71419ac6adadd7dd5be6298581685d9`.
- Normalized commit-inventory digest:
  `62990e0160f88e604440469f2ea72fe2c517ecd337a5bca9dce516ebc6520d47`.
- Related-path inventory digest:
  `c39553d1d0715dccc7bd4a416c6eddcbe4a71b4c964ee1ed14d2e45991fbcf1f`.

## Non-Claims

- Candidate does not mean imported, integrated, enabled, or production-ready.
- Offline validation does not perform a network fetch or establish network
  freshness.
- No merge, rebase, cherry-pick, deployment, restart, publication, or live
  enablement occurred during observation.
