# Hepta Upstream Codex Current Intake — 2026-07-24 R4

## Boundary

- Range: `9fc715c0861c956c894a91890b78dc05b304ba29..f61b51ddd924643514b33234816a8a2772b1aec7` (cutoff exclusive, upstream head inclusive).
- Inventory: **97 commits**, **744 net changed paths**, **0 merge commits**; net diff `744 files changed, 33356 insertions(+), 18923 deletions(-)`.
- Local Hepta and upstream Codex histories have **no merge base**. Ordinary merge and rebase are forbidden; use selective semantic transplant only.
- Import evidence is conservative: no commit is marked imported. The exhaustive local stable patch-ID sweep could not be completed because local promisor history references missing objects, so unproven equivalence remains `not imported`.
- R3 remains immutable at `docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-22_R3.json`.

## Statistics

| Dimension | Counts |
|---|---|
| Status | `candidate` 41, `deferred` 34, `rejected` 22, `imported` 0 |
| Category | `tools/apps/runtime` 38, `TUI/other` 18, `protocol/app-server` 20, `security` 21 |
| V2 priority | `P0` 16, `P1` 23, `P2` 36, `none` 22 |

## Highest-Priority Candidates

- `P0` `32f4687b8c43` — Enable exec-server network policy callbacks (#34770) (security; 11 paths).
- `P0` `a59a419afa34` — Use path URIs in shell approval keys (#34806) (security; 2 paths).
- `P0` `946ed315a484` — Centralize SQLite connection configuration (#34808) (protocol/app-server; 35 paths).
- `P0` `ad65f016ed0c` — Honor disabled redirects in route-aware HTTP clients (#34978) (security; 3 paths).
- `P0` `5c94796dc9e8` — Enforce single-writer ownership for paginated threads (#34986) (security; 9 paths).
- `P0` `c769a0534069` — Honor the configured SQLite home across state consumers (#34994) (protocol/app-server; 76 paths).
- `P0` `265cd2e100ff` — Initialize execution environments with the final HTTP policy (#34995) (security; 31 paths).
- `P0` `b834702b27b3` — Support incremental replay of updated thread items (#35013) (protocol/app-server; 13 paths).
- `P0` `5bdbd3ee90d7` — Add trusted plugin script attribution (#35016) (security; 7 paths).
- `P0` `84fa68b429f1` — Attribute command executions to trusted plugin scripts (#35020) (security; 52 paths).
- `P0` `1ee8f49175a9` — Route exec-server HTTP through configured proxy policy (#35023) (security; 25 paths).
- `P0` `9fc4e5a7aaf0` — Preserve plugin attribution across command approvals (#35029) (security; 23 paths).
- `P0` `963316583b74` — Enforce writer ownership for thread archive and deletion (#35031) (security; 12 paths).
- `P0` `d45055ae58ef` — Route environment registry requests through the shared HTTP client (#35034) (security; 7 paths).
- `P0` `94ebae725e5e` — Route exec-server WebSockets through configured proxies (#35056) (security; 24 paths).
- `P0` `09241ae4db0f` — Decouple exec-server HTTP from reqwest types (#35059) (security; 18 paths).

## P1 Candidate Themes

- `84d2b203ed58` — Make MCP resource clients follow the latest runtime (#34733) (tools/apps/runtime).
- `fd51e505401b` — Remove step-scoped data from extension contributors (#34734) (tools/apps/runtime).
- `64dc1c7a01b2` — Retry websocket requests when the previous response is missing (#34763) (tools/apps/runtime).
- `80f3c3141e4d` — Include the final agent message in turn completion summaries (#34777) (protocol/app-server).
- `c5779ed6bb2a` — Use the live parent history mode when forking agents (#34779) (tools/apps/runtime).
- `08ae0fc0cef0` — Consolidate thread startup around `StartThreadOptions` (#34814) (protocol/app-server).
- `d7e8f4c3dccc` — Preserve user input when MCP startup is interrupted (#34839) (tools/apps/runtime).
- `400ee190c30d` — Add persisted thread pinning to the app server (#34840) (protocol/app-server).
- `0da13c6c993c` — Track multi-agent mode in world state (#34845) (tools/apps/runtime).
- `44d76c6a6dd0` — Wake sleeping threads for queued agent mail (#34852) (tools/apps/runtime).
- `e497325a6a17` — Centralize thread MCP state in `McpRuntime` (#34930) (tools/apps/runtime).
- `808d3c2702ce` — Keep session defaults static during config batch writes (#34940) (protocol/app-server).
- `e19e65317a33` — Reuse MCP connections across runtime refreshes (#34952) (tools/apps/runtime).
- `34b935e3e57f` — Replace closed MCP connections during reconciliation (#34957) (tools/apps/runtime).
- `205d37a20f74` — Keep the sleep tool outside code mode (#34969) (tools/apps/runtime).
- `0d4910331db5` — Preserve timestamps when importing external agent sessions (#34989) (protocol/app-server).
- `7bafdada8bea` — Separate Codex error details from retry metadata (#34996) (protocol/app-server).
- `091e4a5d7c7a` — Preserve refreshed Apps tools across MCP runtime updates (#35028) (tools/apps/runtime).
- `fb4e6ba2f492` — Allow disabling the update_plan tool (#35054) (tools/apps/runtime).
- `1d4b58f32d0b` — Track deferred tool namespaces in world state (#35063) (tools/apps/runtime).
- `3947f0d0c3e2` — Avoid duplicating deferred sources in tool search (#35065) (tools/apps/runtime).
- `0dfa778dae6a` — Add WebSocket transport to the code-mode host (#35078) (protocol/app-server).
- `f61b51ddd924` — Support remote code-mode hosts in app-server (#35098) (protocol/app-server).

## Disposition Guidance

- `candidate`: review and reimplement the semantic invariant inside the owning Hepta V2 layer, preserving fail-closed contracts and adding focused tests/receipts.
- `deferred`: keep as evidence-backed backlog; do not absorb until the corresponding Hepta ownership/composition slice is active.
- `rejected`: no standalone transplant for current V2; reconsider only against a concrete local requirement.
- `imported`: requires positive proof; R4 has none.

## Machine-Readable Source

The R4 manifest, ordered transplant slices, shard hashes, and validation contract are in `HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R4.json`. The complete 97-entry commit inventory is split across the four category shards, and the 744-path surface is stored in `HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R4_FILE_SURFACE.json`.

## Validation

- Range identity: `33d5eaeddf0f4e69919df5dde501747ca1abbe10ab30ad3a192a2c64441a6469`.
- File-surface identity: `edf243d8bcd9c047d8e03034dcc0090b83b996b6fadf8658fb5fc78e27fb5d15`.
- Normalized commit inventory SHA-256: `279eaf925d7a23470b6ad3a37d68e60f3ed4c81ef1bbb44c5f259a7985dc9f52`.
- Normalized file-surface SHA-256: `164b64cbc09bb08676974bac32877313b7f77f594d0fc848dfb2d3d9f67b612b`.
- Validated expected/actual commit count `97/97`, unique commits `97`, first/last range endpoints, required per-entry fields, non-empty path lists, category/status totals, and zero imported claims.
