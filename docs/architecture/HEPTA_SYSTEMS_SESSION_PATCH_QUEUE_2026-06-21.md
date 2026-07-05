# Hepta Systems Session Patch Queue - 2026-06-21

## Purpose

This note defines the ordered patch-queue layer above the session
reconstruction map. The map proves that recoverable Hepta patches exist in the
independent `hepta` agent session history; the queue orders those patch calls
and classifies them for Phase 0 recovery planning.

## Local Commands

- Report: `scripts/hepta-systems-session-patch-queue-report.sh`
- Gate: `scripts/hepta-systems-session-patch-queue-gate.sh`

The queue report scans `/Users/qianqi/.openclaw/agents/hepta/agent/codex-home/sessions`
for Hepta-targeted `apply_patch` calls, then emits call-level metadata:

- timestamp
- session path
- call id
- touched repository paths
- operation kind
- current checkout status
- Phase 0 anchor ids
- replay risk classification

By default, patch bodies are not emitted. Set
`HEPTA_SESSION_PATCH_QUEUE_FULL=true` only when a local review task needs the
full call-level queue in the report JSON.

## Phase 0 Anchors

The gate requires the ordered queue to contain all five recovery anchors:

- `plugin_contribution_point_abi`
- `tool_registry_router_lookup_shadow`
- `workflow_durable_store_replay_proof`
- `compact_capability_matrix`
- `scheduler_cutover_preview_chain`

## Replay Boundary

The queue does not apply patches. It is a planning and audit surface only.

Historical patch replay remains disabled until a later recovery step extracts a
small selected anchor patch body, checks it against the current checkout, and
applies it through normal reviewed edits.
