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
- current `hepta-codex` scripts: `8`
- current native gateway source commands: `55`
- Control UI route parity after runtime/session dry-run continuation: `55/55`, missing `0`

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

Continue with channel adapters as disabled live-gated status reports. Inventory
local tooling/content surfaces before any process execution, filesystem
mutation, network read, or channel delivery smoke. Credentialed provider smokes
remain blocked.
