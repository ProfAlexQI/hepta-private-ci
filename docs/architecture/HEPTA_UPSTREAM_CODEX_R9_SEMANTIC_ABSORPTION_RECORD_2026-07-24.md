# Hepta Upstream Codex R9 Semantic Absorption Record

## Source

- Intake:
  `docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R9.json`
- Manifest SHA-256:
  `e8b28c20790abe02a9af0f59fb48eec172298799427d655e5ac23ed3fff64564`
- Upstream range:
  `c8957bbf0f79fa29c5e08b8c0b942c12ea3893f2..000d2540ad73996f3589ae178bfe447bfd67cef2`
- Intake commit:
  `5495b0af073d7b236e532794b4d0ce4a2de57c19`
- Semantic transplant commit:
  `fb25fbaba289b354b508ad30aa2d78eac200cade`

No merge, rebase, or cherry-pick was performed.

## Absorbed

- `634a998d8aae`: hook stdin `BrokenPipe` no longer discards stdout from a
  fast-exiting hook. All other stdin write failures remain errors.
- `000d2540ad73`: a reused MCP elicitation reviewer reads the latest published
  global policy, effective permission profile, and reviewer selection rather
  than the authority of an older active turn.
- `3645a4397c48`: successful explicit and silent plugin OAuth now queue
  best-effort refresh across loaded threads.
- `5a1c54fc2110`: the local host renderer already uses bounded root aliases when
  they improve skill metadata coverage.

Focused core refresh, app-server refresh, hook, and core-skills alias tests all
passed. Panic debt and architecture budgets remained within their locked
baselines.

## Deferred

- Executor skill list/read and resource authority (`7c71783135b0`,
  `fe8500c0a00e`) remains P0-deferred. The local tree has no `ext/skills`, no
  per-step selected capability roots, and no package-confined bounded read
  contract.
- Extension warning delivery (`5dd992acd3f5`) remains deferred until a local
  typed skill-extension producer exists.
- The extension-provider portion of path compaction (`5a1c54fc2110`) remains
  deferred; only host rendering is equivalent.
- Thread-start MCP invalidation (`3645a4397c48`) remains deferred until a
  centralized runtime/source epoch exists.
- Per-connector configuration-layer reviewer resolution (`000d2540ad73`)
  remains deferred; the local absorption binds current global authority only.

## Rejected

- `1a817bb95d94` and `5f6a2c3adb15` repair upstream-only tests and fixtures
  that do not exist locally. No synthetic test surface was created.

## Non-Claims

- Executor skill authority is not available or enabled.
- Loaded-thread OAuth refresh does not close the thread-start race.
- Current global MCP authority does not imply per-connector layer parity.
- Upstream is not fully consumed.
- No deployment, restart, publication, signing, push, or live enablement was
  performed.
