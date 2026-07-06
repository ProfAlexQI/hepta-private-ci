# Hepta Systems Plugin Tool Invocation Read Only Status Dry Run Operator Evidence Acceptance Recording Persistence Shadow Write Rehearsal Readback

Status: ready-blocked, query-only.

This note documents the shadow-only rehearsal layer after the persistence-open preconditions readback. It consumes the two hepta-system candidate contributions and projects what an acceptance-record persistence write would need to look like without executing or materializing the write.

The readback projects a shadow acceptance record envelope, shadow write intent, shadow payload digest, shadow idempotency replay key, shadow receipt preview, shadow store target, and shadow replay result for the selected read-only status MCP dry-run path and the non-selected app connector preflight boundary.

The shadow store target is a logical target only. No `.hepta` store, runtime event-log, SQLite database, ledger, receipt store, idempotency index, test tmp path, connector, ToolRegistry, or live path is opened.

The boundary remains closed: no feature gate open, dry-run execution, operator evidence packet send, operator evidence packet persistence, operator evidence recording, operator acceptance recording, acceptance record persistence, shadow write execution, shadow write materialization, shadow store write, test tmp write, persistence open denial receipt persistence, persistence denial receipt persistence, non-recording denial receipt persistence, idempotency index write, ledger persistence, receipt persistence, ToolRegistry registration, ToolRegistry mutation, registry lookup execution, tool invocation, connector start, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, package/release, canary activation, Public GA promotion, or live execution.

The next migration step is `hepta_systems_tool_registry_shadow_registration_lookup_readback`, which should make in-memory ToolRegistry registration and lookup shadow state queryable without mutating the real registry or invoking any tool.
