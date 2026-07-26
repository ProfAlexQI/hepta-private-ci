# Hepta B Authority Closure

- Commit line: B-stage changes after the R11 A-stage baseline.
- Native Gateway: operator mutations require exact admission, effect planning, provider acknowledgement, and terminal receipts.
- App Server: process spawn is planned before execution, acknowledged after spawn, and terminally resolved on exit or failure.
- MCP Server: session start, reply, interrupt, and tool-runner handoff use the same lifecycle boundary.
- Telegram: the operator-authorized model request binds the durable request, session, update, frozen execution identity, runner, prompt digest, provider acknowledgement, and success or failure terminal receipt.
- CLI: `hepta-cli --bin hepta` is the only workspace owner of the `hepta` binary target. The old full Codex CLI is explicitly named `hepta-codex-compat` and cannot overwrite the active release artifact.
- Model provider: the active `hepta-cli` dependency tree contains no Codex model-provider, exec, MCP, app-server, plugin, protocol, state, sandbox, TUI, or legacy CLI crate. The gated Codex in-process runner is not compiled into the active service by default.
- Active model limitation: without an explicitly configured local MLX runner, the active service fails closed rather than silently falling back to the compatibility Codex execution stack.
- Verification: Native Gateway 399/399 tests passed; active and compatibility binary targets both compile; App Server, MCP, Telegram, Architecture V2 boundary, architecture budget, and active-service dependency-isolation gates passed in their focused B-stage runs.
- Release state: no deployment, service restart, controlled-live enablement, provider invocation, Telegram send, signing, publication, or external mutation occurred during B.
- Deferred: provider-specific tool effects may only be enabled after they receive their own exact EffectBroker lifecycle; compatibility execution is not evidence of active-service authority closure.
