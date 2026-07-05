# Hepta Systems Compact Capability Matrix Restore Preflight - 2026-06-21

This note records the local-only Compact Capability Matrix Restore Preflight.
It consumes the tool execution canonical summary attachment index and performs a
manual apply check over the selected historical compact capability/canonical
patch shape.

The preflight intentionally does not replay the historical patch.

## Current Checkout Reality

The selected historical candidate is `call_rFtWhyTEAmT4jByPkr8d7L3f`. It is
useful as shape evidence, and every path it touched was absent in the historical
snapshot evidence:

- `scripts/hepta-systems-canonical-gate.sh`
- `scripts/hepta-systems-p0-local-gate.sh`
- `docs/architecture/HEPTA_SYSTEMS_CANONICAL_GATE_MATRIX_2026-06-12.md`
- `docs/architecture/HEPTA_SYSTEMS_PLUGINS_TOOLS_WORKFLOW_PLAN_2026-06-12.md`
- `plugins/hepta-system/skills/hepta-system-status/SKILL.md`

That means the selected patch is not safe to apply mechanically. The plugin
skill path is also treated as non-fabrication evidence: this lane should not
invent a plugin fixture merely to satisfy the historical gate shape.

Current report facts:

- `source_attachment_ready=true`
- `source_current_checkout_missing_canonical_summary=true`
- `source_historical_snapshot_missing_canonical_summary=true`
- `source_canonical_summary_probe_basis=historical_snapshot_evidence`
- `source_canonical_summary_current_filesystem_probe_used=false`
- `source_snapshot_ready=true`
- `source_canonical_summary_present_count=0`
- `source_historical_compact_capability_matrix_patch_call_count>=1`
- `selected_patch_call_id=call_rFtWhyTEAmT4jByPkr8d7L3f`
- `selected_patch_replay_risk=requires_missing_base_path_reconstruction`
- `selected_patch_touched_path_count=5`
- `selected_patch_missing_path_count=5`
- `selected_patch_missing_path_count_basis=historical_snapshot_evidence`
- `manual_apply_check_entry_count=5`
- `manual_apply_check_missing_count=5`
- `manual_apply_check_basis=historical_snapshot_evidence`
- `manual_apply_check_current_filesystem_probe_used=false`
- `manual_apply_check_missing_at_snapshot_count=5`
- `historical_patch_replay_allowed=false`
- `patch_body_emission_allowed=false`
- `plugin_fixture_fabrication_allowed=false`
- `canonical_summary_mutation_allowed=false`
- `canonical_gate_invocation_allowed=false`
- `capability_matrix_gate_invocation_allowed=false`
- `restore_preflight_ready=true`
- `tool_execution_live_cutover_allowed=false`
- `tool_execution_public_ga_allowed=false`

## Restore Rules

- Use the selected historical patch only as shape evidence.
- Use historical snapshot evidence, not live filesystem absence, for missing
  path checks.
- Do not emit patch bodies from this preflight.
- Do not replay the historical patch.
- Do not fabricate `plugins/hepta-system` or `.agents` fixtures.
- Do not mutate a canonical/capability summary in this preflight.
- Do not invoke canonical, capability, terminal live, live URL, or long-soak
  gates.
- Keep tool execution live cutover and Public GA disallowed.

## Guardrails

- No historical patch replay.
- No patch body emission.
- No plugin fixture fabrication.
- No canonical summary mutation.
- No canonical gate invocation.
- No capability matrix gate invocation.
- No terminal live gate invocation.
- No live URL contact.
- No long soak start.
- No ToolRegistry registration.
- No execution adapter dispatch.
- No tool invocation.
- No ledger write.
- No ApprovalBroker request.
- No approval request send.
- No operator cutover acceptance record.
- No live cutover start.
- No rollback execution.
- No rollback receipt write.
- No result receipt write.
- No MCP server or app connector startup.
- No workflow event log, SQLite, local storage, WorkGraph, provider/model,
  gateway/auth, Native POST, channel send, package, release, Public GA, or
  external live action.

## Files

- Report:
  `scripts/hepta-systems-compact-capability-matrix-restore-preflight-report.sh`
- Gate:
  `scripts/hepta-systems-compact-capability-matrix-restore-preflight-gate.sh`
- Source:
  `scripts/hepta-systems-tool-execution-canonical-summary-attachment-index-gate.sh`

## Next Move

Restore a current-checkout compact capability summary without plugin fixture
fabrication. The restored summary should consume the current tool execution
closure/bridge/attachment/preflight sources and represent current facts rather
than the absent historical package fixture.
