# Hepta Upstream Codex Current Intake R5

## Observation

- `upstream/main` advanced from R4 `f61b51ddd924` to `81da9deb065d`.
- The R5 delta contains 2 non-merge commits, 28 changed paths, 1,191 insertions, and 87 deletions.
- Hepta and upstream still have unrelated roots and no merge base. Ordinary merge/rebase remains forbidden.
- R4 is unchanged. R5 is a new immutable observation and claims zero imports at observation time.

## Classification

- `81da9deb065d`: candidate P1. Bounded host customization for `wait_for_environment` can be adapted if default fallback, serialized-size limits, feature gating, and Hepta tool-exposure controls remain authoritative.
- `a28374e0dbb4`: candidate P1. Agent Plugins 1.0 support is valuable, but root-manifest recognition must remain inactive until Hepta's plugin trust, namespace, app/hook, legacy-precedence, and path-containment gates are closed.

## Ordered Intake

1. Absorb the bounded `wait_for_environment` description contract with default/oversized/feature-disabled tests.
2. Audit Agent Plugins schema and direct-child discovery against Hepta plugin trust before any activation.

## Non-Claims

- Candidate does not mean imported, integrated, enabled, or production-ready.
- No merge, rebase, cherry-pick, deployment, restart, publication, or live enablement occurred during observation.
