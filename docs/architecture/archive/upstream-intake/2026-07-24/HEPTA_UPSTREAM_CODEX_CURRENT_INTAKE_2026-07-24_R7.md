# Hepta Upstream Codex Current Intake R7

## Observation

- The latest recorded upstream observation advanced from frozen R6
  `6c729ef1c1dc` to `f201c30c52a3`.
- The R7 delta contains 1 non-merge commit, 3 net changed paths, 65 insertions,
  8 deletions, and 3 commit-level file touches.
- R6 is preserved byte-for-byte. R7 is a new immutable observation and claims
  zero imports at observation time.
- This is an offline latest-recorded intake. It does not prove that the network
  head remains unchanged after `2026-07-24T10:39:16Z`.
- Hepta and upstream still have unrelated roots and no merge base. Ordinary
  merge/rebase remains forbidden; any integration requires selective semantic
  transplant with behavioral evidence.

## Classification

- `f201c30c52a3`: candidate P1. Explicit MCP refresh should reconnect every
  configured server, preserve the reconnect request across cancelled runtime
  replacement, and retain connection reuse for ordinary runtime updates.

## Ordered Intake

1. First establish the R6 immutable published-runtime, serialized refresh,
   desired-state, environment, and authentication-generation prerequisites.
2. Then adapt an explicit force-reconnect request that is generation-bound,
   cancellation-safe, and covered by same-generation exposure/execution tests.

## Frozen Evidence

- Predecessor manifest:
  `docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R6.json`
  (`4e8993154f769ce2f4bdbd078816fe6ce193e64ca12913ca09acec073532bbdc`).
- Frozen R6 ref:
  `refs/remotes/upstream/hepta-intake-20260724-r6` at
  `6c729ef1c1dcfbcbe1bd9d0c2dddde24377ae899`.
- Required R7 frozen ref:
  `refs/remotes/upstream/hepta-intake-20260724-r7` at
  `f201c30c52a35f819262865a53df94b6f4ea7a50`.
- Range digest:
  `58b43c4389ea3adf336023a05d08b6b6a4708d4c89eb49b4f2529d83ea1cbc4e`.
- Net path-surface digest:
  `7db8ff45019f54816b0175ae01cb4a4bd0d09d8886fc7ed93515bcbe27c1a03e`.
- Commit identity digest:
  `fb0f8b5c3dbc294a0cdda6c452c249646084d82cf3b33465dc3b116a60e3b40d`.
- Normalized commit-inventory digest:
  `151ffc8a8db68378663d69b0e28f9de3e442456f28a92fa525114e8603c82209`.
- Related-path inventory digest:
  `4610d4f464aa739db64c87ed819562d335b6cebb807815b496e9ec4d550449a8`.

## Non-Claims

- Candidate does not mean imported, integrated, enabled, or production-ready.
- Offline validation does not perform a network fetch or establish network
  freshness.
- No merge, rebase, cherry-pick, deployment, restart, publication, or live
  enablement occurred during observation.
