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
- `scripts/hepta-upstream-codex-sync-lane.sh` defines the upstream Codex intake
  lane: upstream changes are classified, absorbed through Hepta contracts, and
  gated by active-service dependency isolation before any active runtime
  promotion.

No public release publication, credential read, model invocation, channel
delivery, or gateway mutation is implied by this document.
