# Hepta Upstream Codex Sync Lane

This lane defines how Hepta consumes upstream `openai/codex` changes after
active service fusion.

The retained `codex-rs` compatibility surface is an intake and regression
oracle, not the active production engine. Upstream changes must move through a
bounded path:

1. Record the observed upstream head and diff range.
2. Classify deltas into provider/credential/security, runtime/session/tool,
   MCP/app-server, sandbox/exec, TUI/legacy command, and product/documentation
   groups.
3. Materialize Hepta absorption contracts for the selected deltas.
4. Keep `scripts/hepta-active-service-dependency-isolation.sh` green so the
   active `hepta-cli --bin hepta` service does not regain direct Codex engine
   dependencies.
5. Run preflight, watchdog, soak, and release-governance gates before any public
   release claim.

The local sync-lane gate is:

```bash
scripts/hepta-upstream-codex-sync-lane.sh
```

Set `HEPTA_UPSTREAM_CODEX_SYNC_REQUIRE_LIVE=1` to require live route
confirmation from the running service as well as the offline cargo-tree
isolation check.

This lane does not fetch upstream, merge commits, restart the active service,
read credentials, invoke providers, deliver channel messages, or publish a
release by itself.
