# Hepta Systems ToolRegistry Shadow Registration Lookup Readback

This note records the query-only ToolRegistry shadow registration and lookup
readback that consumes the persistence shadow write rehearsal boundary for the
local `hepta-system` fixture.

The readback projects a shadow registry registration plan, shadow registry entry key, shadow registration payload digest, shadow lookup query, shadow lookup result, shadow duplicate check, shadow idempotency replay anchor, and shadow approval ledger replay anchor for each candidate contribution. It keeps the selected read-only status MCP path and the non-selected app connector preflight path visible without registering either one.

The closed boundary is: no feature gate open, shadow write execution, shadow write materialization, shadow store write, test tmp write, shadow registry materialization, shadow lookup execution, ToolRegistry registration, ToolRegistry mutation, registry lookup execution, tool invocation, approval request, ledger persistence, receipt persistence, connector start, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, package/release, canary activation, Public GA promotion, or live execution.

The next migration step is
`close_controlled_live_evidence_before_status_canary_start`,
which should rehearse durable ledger/receipt writes from the shadow registry
readback without writing `.hepta`, opening a feature gate, invoking a tool, or
starting live execution.
