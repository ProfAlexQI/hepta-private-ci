# Hepta Upstream Codex Current Intake R9

## Observation

- The latest recorded upstream observation advanced from frozen R8
  `c8957bbf0f79` to `000d2540ad73`.
- The R9 delta contains 9 non-merge commits, 56 net changed paths, 2,439
  insertions, 303 deletions, and 68 commit-level file touches.
- R8 remains immutable. R9 records 4 candidates, 3 deferrals, 2 rejections,
  and zero imports at observation time.
- This is an offline exact-SHA intake. Network freshness remains a separate
  receipt and is not inferred from this document.
- Hepta and upstream have unrelated roots. Ordinary merge/rebase is forbidden;
  all integration requires selective semantic transplantation with behavioral
  evidence.

## Priority Assessment

- P0: executor-skill list/read authority (`7c71783135b0`, `fe8500c0a00e`) and
  current MCP elicitation authority (`000d2540ad73`).
- P1: fast-exit hook output preservation (`634a998d8aae`) and MCP refresh across
  the thread-start window (`3645a4397c48`).
- P2: extension warning delivery (`5dd992acd3f5`) and host skill-path
  compaction (`5a1c54fc2110`).
- P3: two upstream-only deterministic test repairs (`1a817bb95d94`,
  `5f6a2c3adb15`).

## Architectural Decision

1. Do not expose executor skills locally yet. The local tree has neither
   `ext/skills` nor per-step selected capability roots, so direct adoption
   would create session-global filesystem authority.
2. Absorb current MCP authority through the latest published local manager:
   policy, permission profile, and reviewer selection must not come from an
   older active turn.
3. Ignore only stdin `BrokenPipe` for hooks that already exited; preserve all
   other write failures and completed stdout.
4. Refresh loaded threads after successful OAuth, but defer the startup-race
   closure until a centralized MCP runtime/source epoch exists.
5. Retain the existing local host skill-path alias table as bounded semantic
   equivalence. Do not claim equivalence for the absent extension provider.

## Frozen Evidence

- Predecessor manifest:
  `docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R8.json`
  (`60833a3504bca61ed33f527d6bc9315193540b839650e92280eeedb6cf10dba3`).
- Required R9 frozen ref:
  `refs/remotes/upstream/hepta-intake-20260724-r9` at
  `000d2540ad73996f3589ae178bfe447bfd67cef2`.
- Range digest:
  `9b8b35976e482e8628f08ec1c40b412fc9f114dc80c577bb429d58ed2db960f8`.
- Net path-surface digest:
  `30c2d4cf04cd64d547843e61249f52ec5d505a4bc0f1746da8f48e0257fd3c2f`.
- Commit identity digest:
  `e420afa25ff20a16c40bb06abf43344644db97ad6b1459b74d881aa10e9cb1b3`.
- Normalized inventory digest:
  `18eb5611f0d7090aae76aa20278e033f65553d25d94250a4559c5cab8213a3a9`.
- Related-path inventory digest:
  `2f78b6bf411fa297a2850b52f3d197c985c34297f547dba72c9b3ad7fbdb4a17`.

## Non-Claims

- Candidate does not mean imported, fully absorbed, enabled, or
  production-ready.
- The executor-skill authority commits are not locally available.
- Loaded-thread OAuth refresh does not close the thread-start publication race.
- Local MCP authority binding does not implement upstream per-connector
  configuration-layer reviewer overrides.
- No merge, rebase, cherry-pick, deployment, restart, publication, or live
  enablement occurred during observation.
