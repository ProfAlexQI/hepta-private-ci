# Hepta Upstream Codex R5 Semantic Absorption Record

## Boundary

- Source intake: `HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R5.json` at local commit `a53794d561b0`, with exact manifest SHA-256 `dd56ea3130f035714fe14dcf25584161e669bb441a946548273a2ddec741e18f`.
- Integration mode: selective semantic transplant with fail-closed deferral.
- No merge, rebase, cherry-pick, deployment, publication, or live enablement occurred.

## Absorbed

- Tools/apps/runtime: upstream `a28374e0dbb4` maps to Hepta `cb366251686a`.
- Hepta recognizes supported Agent Plugin root manifests while preserving legacy precedence; freezes manifest namespace and provenance; restricts Agent overlays to apps, hooks, and interface metadata; and gives DirectChildren discovery local filesystem authority with canonical containment and no-symlink checks for consumed skill resources.
- Focused package results: `codex-plugin` 3/3, `codex-core-skills` 103/103, `codex-core-plugins` 265/265, and `codex-utils-plugins` 8/8.
- Package-scoped fixes passed for `codex-plugin`, `codex-core-skills`, `codex-core-plugins`, and `codex-utils-plugins`.

## Deferred

- Upstream `81da9deb065d` (`wait_for_environment` host descriptions) is `deferred_prerequisite_unavailable`.
- The local baseline lacks the DeferredExecutor feature gate, wait handler, router wiring, starting-environment state machine, and turn-frozen description snapshot/StepContext binding needed to absorb that change without widening the tool surface.
- No local commit or test result is claimed for this deferred item.

## Non-Claims

- Remote plugin sharing still requires `.codex-plugin/plugin.json`.
- Apps, hooks, and MCP component contents are read later from frozen paths rather than immutable byte snapshots.
- The generic namespace validator is not yet fully unified with the Agent Plugin name validator.
- This receipt proves one selected R5 semantic absorption, not full consumption of the two-commit R5 range.

Machine-readable provenance, exact commit IDs, representative files, evidence, and deferral prerequisites are in the adjacent JSON receipt.
