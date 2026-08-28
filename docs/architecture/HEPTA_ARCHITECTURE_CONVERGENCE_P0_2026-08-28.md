# Architecture Convergence P0 — 2026-08-28

## Scope

This slice implements the first executable convergence step on the default integration baseline:

- one canonical architecture entry and machine product graph;
- one closed-world runtime authority kernel in `codex-hepta-contracts`;
- one code-level single-writer data authority map;
- a thinner Agentd startup path through `AgentRuntimeComposition`;
- a narrow Memory runtime facade for store open and federation discovery;
- an exact-head qualification job that compiles and tests the real product modules and opens real
  Agent-private Cognitive/Automation SQLite stores.

## Code changes

| Area | P0 result |
|---|---|
| Authority | `AuthorityGrant`, typed capability markers, non-serializable `Authorized<C>` tokens |
| Product graph | Deterministic acyclic graph and unique writer validation |
| Memory | `CognitiveRuntime::open_agent_owned` and `with_discovered_federation` |
| Agentd | Startup composition extracted from the task supervision loop |
| App Server | Cognitive write feature is derived from the exact authority grant |
| Qualification | Real product composition test; no nested shadow product implementation |

## Explicit non-goals

- No new listener or public route.
- No model or provider call.
- No external effect.
- No production cognitive writer.
- No fleet mutation outside the existing supervisor.
- No operator acceptance, CALLERS ratchet, merge, or promotion.
- No physical Cognitive SQLite table split in this slice.

## Exit gate

The slice remains `SOURCE_PRESENT_QUALIFICATION_PENDING` until the exact branch head has non-empty
runner steps and passes:

```text
architecture source verifier
cargo fmt --check
codex-hepta-contracts tests
codex-hepta-memory tests
codex-hepta-agentd default tests
codex-hepta-agentd qualification-cognitive-write tests
cargo check --all-targets
clippy -D warnings
```

A queued job, source-only inspection, partial step list, or generated artifact is not a qualification
receipt.
