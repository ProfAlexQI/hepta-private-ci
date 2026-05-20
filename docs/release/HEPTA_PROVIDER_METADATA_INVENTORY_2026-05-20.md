# Hepta Provider Metadata Inventory

Date: 2026-05-20
Scope: old standalone Hepta provider/search ops modules versus current `hepta-codex`
Status: metadata-only inventory landed; no provider or search live calls enabled

## Summary

The first CLI breadth inventory identified provider/search bridges as the next
safe migration slice. This slice exposes that family as a read-only native route:

- `/api/hepta-provider-metadata-inventory`
- source-command equivalent: `/hepta-provider-metadata-inventory --json`
- validation script: `scripts/hepta-codex-provider-metadata-inventory.sh`

The route is intentionally metadata-only. It does not read provider credentials,
invoke model providers, perform search/network reads, call Telegram, send
messages, mutate native POST state, or write files.

## Inventory Counts

- old provider ops files: `15`
- adjacent search/readability ops files: `3`
- provider adapters inventoried: `15`
- adjacent search adapters inventoried: `3`
- current `hepta-codex` scripts: `7`
- current native gateway source commands: `54`
- Control UI route parity after this slice: `54/54`, missing `0`

## Provider Files Covered

- `anthropic_ops.rs`
- `deepinfra_ops.rs`
- `google_antigravity_ops.rs`
- `google_gemini_cli_ops.rs`
- `google_ops.rs`
- `google_vertex_ops.rs`
- `media_generation_ops.rs`
- `mistral_ops.rs`
- `native_model_provider_ops.rs`
- `ollama_ops.rs`
- `openai_codex_ops.rs`
- `openai_ops.rs`
- `openrouter_ops.rs`
- `provider_registration_ops.rs`
- `xai_ops.rs`

Adjacent search/readability files covered:

- `native_search_provider_ops.rs`
- `search_tools_ops.rs`
- `web_readability_ops.rs`

## Remaining Blockers

- provider prompt/API smoke is not operator-approved
- provider credentials are not read by the inventory
- search live network smoke is not operator-approved
- old CLI invocation compatibility is not claimed

## Safe Next Slice

Continue with runtime-event/task/session surfaces as local dry-run planners.
Keep channel adapters disabled/live-gated until a separate explicit operator
request exists. Credentialed provider smokes remain blocked.
