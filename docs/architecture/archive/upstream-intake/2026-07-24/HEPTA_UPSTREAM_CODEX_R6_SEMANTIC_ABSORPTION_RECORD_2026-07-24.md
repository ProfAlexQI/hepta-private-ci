# Hepta Upstream Codex R6 Semantic Absorption Record

## Boundary

- Source intake: `HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R6.json` at local commit `846e2d4aa0f6`, with exact manifest SHA-256 `4e8993154f769ce2f4bdbd078816fe6ce193e64ca12913ca09acec073532bbdc`.
- Integration mode: fail-closed deferral pending Hepta-native prerequisites.
- No merge, rebase, cherry-pick, deployment, publication, or live enablement occurred.

## Deferred

- Upstream `ef2d3edb959a` (background MCP prewarm) is `deferred_prerequisite_unavailable`. The local baseline lacks a published immutable `McpRuntime`, serialized exact refresh ownership, authoritative dirty/generation transitions, latest desired-state and environment snapshots, stale-generation publish rejection, and joined shutdown semantics.
- Upstream `6c729ef1c1dc` (MCP refresh on session-auth changes) is also `deferred_prerequisite_unavailable`. Credential rotation is a high-priority security semantic, but safe absorption requires an authoritative auth generation publisher/watcher, plugin auth-mode binding, a published immutable auth/token snapshot, startup re-projection, and one generation binding both tool exposure and the executing MCP client.
- Neither item has a local commit or claimed test result.

## Required Safety Evidence

Before either item can be promoted to absorbed, Hepta must prove:

- an old-generation runtime is revoked and cannot expose or execute tools after auth rotation;
- a real MCP `Authorization` header uses the new credential generation;
- a cross-account transition cannot leak the prior account's tool catalog;
- a startup auth race cannot publish mixed credentials; and
- tool exposure and execution resolve through the same published runtime generation.

## Non-Claims

- R6 has zero absorbed commits in this receipt.
- This receipt does not claim that explicit MCP refresh is equivalent to automatic auth/config invalidation or background prewarm.
- It does not claim full upstream consumption, production readiness, controlled-live readiness, or network freshness.

Machine-readable provenance, exact commit IDs, prerequisite gaps, and required tests are in the adjacent JSON receipt.
