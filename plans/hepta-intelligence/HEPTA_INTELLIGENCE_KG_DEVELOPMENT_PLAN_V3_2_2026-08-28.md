# Hepta Intelligence KG Development Plan v3.12

**日期**：2026-08-29  
**仓库**：`ProfHepta/hepta-private-ci`（原 `ProfAlexQI/hepta-private-ci`）  
**状态**：ACTIVE / FAIL-CLOSED

## 精确状态

```text
P0.3.2 Shared Semantic ProjectionPlanner
branch = codex/hepta-intelligence-shared-projection-planner-v5-20260828
head   = fa59bb090043ba8d6fbf0991b167779d2385888c
qualified = true
exact Linux ARM run = 33190943793

P0.3.3 HOST_OWNED_EVIDENCE_RESOLUTION
branch = codex/hepta-intelligence-evidence-resolver-v4-20260828
base   = P0.3.2@fa59bb090043ba8d6fbf0991b167779d2385888c
implemented = true
restacked = true
qualified = false until the v5 exact-head receipt is all-green
```

## 已闭合的功能与治理 gap

- Product 与 grounded shadow 共用同一个 `ProjectionSemanticPlan`。
- Durable grounding ledger 与 shadow projection 在同一 SQLite read transaction 内复核。
- Current generation 在 shadow compare 前由 shared planner 重放。
- V4 模型协议仅接受 `quote` + `occurrence` 或 `segment_id`。
- UTF-8 byte offsets 与 SHA-256 由 Rust host 计算。
- P0.3.2 exact artifact、run、job、runner、steps、tree 与 authority flags 已硬绑定。
- P0.3.3 已 governed restack 到精确 P0.3.2 qualified head。
- 仓库 identity 已迁移到 `ProfHepta/hepta-private-ci`。
- P0.3.3 每个 exact head 使用独立 concurrency group，旧 pending run 不再串行阻塞新 head。

## P0.3.3 exact-head v5 admission

```text
python compile
source contract
exact P0.3.2 dependency binding
governed Rust rustfmt
extension all-target cargo check
core all-target cargo check
resolver focused tests
core grounding compatibility tests
extension full tests
core full tests
governed-file scoped Clippy
clean source tree
```

`qualified=true` 仅在全部 check 的 `exit_code=0` 且 receipt 绑定当前 exact head/tree 后成立。

## 下一唯一顺序

```text
P0.3.3 exact-head v5 executable all-green
→ P0.3.4 legacy inventory
→ bounded dry-run backfill
→ quarantine / corruption classification
→ shadow pointer/canary
→ production gate ratchet（需要独立 operator acceptance）
```

## Authority boundary

```text
tool_v4_registered=false
wired=false
default_projection_pointer_changed=false
default_recall_query_changed=false
production_projection_gate=false
production_authority=false
external_effects=false
operator_accepted=false
promoted=false
callers_ratchet=false
```
