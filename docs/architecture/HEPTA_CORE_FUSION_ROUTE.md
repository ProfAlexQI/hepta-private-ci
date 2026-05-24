# Hepta Core Fusion Route

This is the Hepta-named canonical route note for the core-fusion closure work.
The dated `HEPTA_CODEX_CORE_FUSION_ROUTE_2026-05-23.md` document is retained as
a transition history and compatibility reference.

Current state:

- The active service binary is `/Users/qianqi/.local/opt/hepta/bin/hepta`.
- The active release package is `hepta-cli --bin hepta`.
- The canonical engine-adapter route is `/api/hepta-engine-adapter-boundary`.
- The transition route `/api/hepta-codex-engine-adapter-boundary` remains as a
  compatibility alias.
- The canonical release gate script family is `scripts/hepta-*.sh`.
- The transition script family `scripts/hepta-codex-*.sh` remains callable as a
  compatibility target.
- Runtime reports now expose `runtime="hepta"` while retaining compatibility
  names only where they identify old paths, old scripts, or old route aliases.
- The active workspace directory is `/Users/qianqi/.openclaw/workspace/Hepta`.
  The old `/Users/qianqi/.openclaw/workspace/hepta-codex` path is retained only
  as a rollback/compatibility alias.
- The Phase 5 dependency closure gate is
  `/api/hepta-engine-dependency-closure`.
- The active `hepta-cli --bin hepta` service binary is now isolated from the
  tracked direct Codex engine crates. `full_fusion_complete=true` means active
  service closure; `codex-cli` remains only as a legacy compatibility package.
- `scripts/hepta-active-service-dependency-isolation.sh` makes the active
  service isolation check repeatable with an offline `hepta-cli` cargo-tree
  gate and an optional live route contract check.
- `scripts/hepta-upstream-codex-snapshot.sh` records the local Hepta head,
  `codex-rs` compatibility tree, optional read-only upstream Codex HEAD, and
  the risk classification buckets that must exist before absorption work.
- `scripts/hepta-upstream-codex-diff-ledger.sh` records the local upstream
  baseline-to-target range and classifies the first `codex-rs` upstream delta
  inventory before any absorption patch is allowed.
- `scripts/hepta-upstream-codex-product-governance-absorption.sh` selects the
  first bounded absorption family from that ledger and requires Hepta-specific
  product/release-governance translation before promotion.
- `scripts/hepta-upstream-codex-product-governance-translation.sh` verifies the
  Hepta-owned translation packet for that selected product/release-governance
  family, keeping upstream docs/package wording out of active runtime claims.
- `scripts/hepta-upstream-codex-legacy-compatibility-absorption.sh` selects the
  P1 CLI/TUI/code-mode compatibility family as an intake contract while keeping
  Codex CLI/TUI behavior out of the active Hepta runtime.
- `scripts/hepta-upstream-codex-legacy-compatibility-replay.sh` verifies the P1
  CLI/TUI/code-mode command-contract replay packet while keeping CLI command
  shape, TUI presentation, code-mode callbacks, terminal helpers, and dependency
  boundaries report-only with no active CLI/TUI promotion.
- `scripts/hepta-upstream-codex-provider-security-absorption.sh` selects the P0
  provider/auth/config/exec/sandbox/network-proxy family as an intake contract
  while keeping provider and security-policy behavior out of the active runtime
  until Hepta-native replay, redaction, operator, soak, and dependency gates
  pass.
- `scripts/hepta-upstream-codex-provider-security-replay.sh` verifies the P0
  provider/security replay packet: redacted provider contracts, credential
  redaction, approval-policy replay, sandbox/exec replay, network-proxy replay,
  and side-effect boundaries, still with no active provider/security promotion.
- `scripts/hepta-upstream-codex-runtime-appserver-absorption.sh` selects the P0
  app-server/session/thread/tool/MCP/hooks/exec family as an intake contract
  while keeping runtime behavior out of the active service until route/event,
  replay, shadow, and dependency gates pass.
- `scripts/hepta-upstream-codex-runtime-appserver-replay.sh` verifies the P0
  runtime/app-server replay packet for app-server protocol, daemon/transport,
  session/thread-store, tool/MCP, exec/hook, and side-effect boundaries, still
  with no active runtime, app-server, or tool/MCP promotion.
- `scripts/hepta-upstream-codex-sync-lane.sh` defines the upstream Codex intake
  lane: upstream changes are classified, absorbed through Hepta contracts, and
  gated by active-service dependency isolation before any active runtime
  promotion.

No public release publication, credential read, model invocation, channel
delivery, or gateway mutation is implied by this document.
