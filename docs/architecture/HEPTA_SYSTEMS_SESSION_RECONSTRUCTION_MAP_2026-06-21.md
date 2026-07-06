# Hepta Systems Session Reconstruction Map - 2026-06-21

## Purpose

This note defines the Phase 0.0 recovery surface for Hepta systems work that is
present in the `hepta` agent Codex session JSONL history but absent from the
current `main` checkout.

The current source of truth remains the git checkout at:

- `/Users/qianqi/.openclaw/workspace/Hepta`

The recovery evidence source is the independent agent session store at:

- `/Users/qianqi/.openclaw/agents/hepta/agent/codex-home/sessions`

## Local Commands

- Report: `scripts/hepta-systems-session-reconstruction-map-report.sh`
- Gate: `scripts/hepta-systems-session-reconstruction-map-gate.sh`

Both commands are local and report-only. They read session JSONL files, extract
Hepta-targeted `apply_patch` metadata, compare touched paths with the current
checkout, and produce a reconstruction candidate map.

## Recovery Anchors

The gate requires evidence for these anchors before any future patch replay is
considered:

- `plugin_contribution_point_abi`
- `tool_registry_router_lookup_shadow`
- `workflow_durable_store_replay_proof`
- `compact_capability_matrix`
- `scheduler_cutover_preview_chain`

These anchors correspond to the missing systems surfaces observed during the
2026-06-21 audit.

## Boundaries

This recovery map does not:

- apply historical patches
- mutate the git index
- commit, push, tag, release, or deploy
- read credentials
- invoke providers or models
- perform gateway, Native POST, Telegram, SQLite, event-log, WorkGraph, tool, or
  ledger mutations

The next local step is to extract an ordered Hepta-only patch queue and replay
only selected Phase 0 recovery candidates after reviewing current checkout
compatibility.
