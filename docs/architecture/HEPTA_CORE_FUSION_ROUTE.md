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
- The workspace directory cutover is intentionally still pending. An existing
  `/Users/qianqi/.openclaw/workspace/Hepta` checkout is present, so the active
  repository path must be migrated in a later controlled slice rather than by a
  blind rename.

No public release publication, credential read, model invocation, channel
delivery, or gateway mutation is implied by this document.
