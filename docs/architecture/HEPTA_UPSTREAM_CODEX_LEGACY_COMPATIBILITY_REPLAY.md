# Hepta Upstream Codex Legacy Compatibility Replay

This packet turns the `legacy-cli-tui-compatibility` intake bucket into a
bounded Hepta replay contract. It is report-only evidence for retained Codex
CLI, TUI, and code-mode compatibility surfaces; it does not promote any of
those surfaces into the active Hepta runtime.

Selected changed paths: `128`

## Replay Surfaces

- CLI command shape and argument contract replay: retained upstream CLI deltas
  must be mapped to explicit Hepta command contracts before any behavior is
  exposed.
- TUI presentation and snapshot compatibility replay: retained TUI deltas stay
  as compatibility snapshots until Hepta-native presentation parity gates exist.
- code-mode runtime callback and module-loader replay: retained code-mode
  callbacks stay behind compatibility-only replay and do not become active
  runtime hooks.
- terminal detection, PTY, and utils CLI helper replay: terminal and helper
  behavior must be replayed as bounded contracts before promotion.
- Active dependency boundary and no-promotion replay: active `hepta-cli` remains
  free of tracked Codex engine crates.

## Promotion Boundary

- No active CLI/TUI promotion.
- No active runtime code wiring.
- No active Codex engine dependency.
- No provider invocation.
- No channel delivery.
- No gateway RPC.
- No public release claim.

## Gate

```bash
scripts/hepta-upstream-codex-legacy-compatibility-replay.sh
```

The gate runs the Rust report contract tests, verifies this packet, and emits a
JSON report with five replay surfaces and all side-effect flags false.
