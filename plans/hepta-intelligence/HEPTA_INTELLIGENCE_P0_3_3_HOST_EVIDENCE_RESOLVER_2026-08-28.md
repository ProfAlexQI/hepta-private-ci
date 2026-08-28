# Hepta Intelligence P0.3.3 — Host-Owned Evidence Resolver

**日期**：2026-08-28
**状态**：SOURCE-ONLY IMPLEMENTED / ACTIVATION BLOCKED
**基线分支**：`codex/hepta-intelligence-grounding-gate-v3-20260828`
**基线 commit**：`256a47d882413ac1f80892a80177419bb5d12c70`
**候选分支**：`codex/hepta-intelligence-evidence-resolver-v4-20260828`

## 1. 目标

P0.3 v3 contract 要求模型直接提交 `start_byte`、`end_byte` 和 `sha256`。这在安全上严格，但把 UTF-8 字节计算和摘要计算错误地交给了模型，中文、重复文本和长 source 下容易产生高失败率。

P0.3.3 改为 **HOST_OWNED_EVIDENCE_RESOLUTION**：

```text
model semantic selection
  → exact quote + zero-based occurrence
    or host-issued segment_id
  → Rust host exact witness resolution
  → UTF-8 byte range
  → SHA-256
  → duplicate/limit checks
  → FactEvidenceSpanDraft
  → existing grounding verifier / durable writer
```

模型不再拥有 byte offset 或 digest 的生成权。

## 2. 模型输入协议

### Quote selector

```json
{
  "quote": "Project Aurora uses Rust",
  "occurrence": 0
}
```

语义：在 exact witnessed UTF-8 source 中按左到右 exact match 选择；每次从上一个匹配起点的下一个 UTF-8 字符继续，因此重叠匹配也有确定序号第 `occurrence` 个匹配，`occurrence` 从 0 开始。

### Segment selector

```json
{
  "segment_id": "source-segment:v1:<64 lowercase hex>"
}
```

`segment_id` 必须由 Rust host 根据 exact source span、start/end 和 evidence digest 域分离计算。模型不能自行声明该 ID 的内部绑定。

### 禁止字段

模型-facing schema 不包含：

```text
start_byte
end_byte
sha256
verified=true
```

## 3. Rust contract

新增：

```text
GroundedToolV4Input
GroundedEntityV4
GroundedRelationV4
EvidenceLocatorV4
ExactQuoteLocatorV4
SourceSegmentLocatorV4
SourceSegmentDraftV1
SourceSegmentDescriptorV1
HostEvidenceResolverV1
HostEvidenceResolutionReceiptV1
prepare_grounded_tool_v4

implementation modules:
  evidence_resolver_v4.rs
  evidence_resolver_v4/receipt.rs
  evidence_resolver_v4/resolver.rs
  evidence_resolver_v4/schema.rs
  evidence_resolver_v4/support.rs
  evidence_resolver_v4/tests.rs
```

Host resolver 必须：

- 只接受 1..=256 KiB 的 exact UTF-8 source；
- quote 必须为 1..=4096 non-NUL bytes；
- quote occurrence 必须在 0..=1023；
- 每 fact 1..=4 selectors；
- 全输入最多 768 selectors；
- exact quote 不存在或 occurrence 越界时 fail closed；
- segment ID、range、UTF-8 boundary 和 digest 必须全部重算；
- 同一 fact 的 selectors 解析到重复 span 时 fail closed；
- entity/relation key 重复时 fail closed；
- relation endpoint 不存在时 fail closed；
- 输出先生成 host-owned v3 span contract，再复用 `prepare_grounded_tool_v3` 生成 `FactEvidenceSpanDraft`；
- 输出 span 按 start/end/digest 确定性排序。

## 4. Authority boundary

```text
tool_v4_registered=false
wired=false
qualified=false
p0_3_2_dependency_qualified=false
production_projection_gate=false
production_authority=false
external_effects=false
operator_acceptance=false
promotion=false
callers_ratchet=false
```

本 tranche 仅编译并测试 contract。不得：

- 注册 `remember_grounded_v4` / `correct_grounded_v4`；
- 替换现有 v3 tool schema；
- 修改默认 writer；
- 修改 projection pointer 或 recall query；
- 将 source-only gate 解释为 efficacy、operator acceptance 或 production promotion。

## 5. P0.3.2 依赖

最新计划要求 P0.3.3 在 P0.3.2 shared semantic ProjectionPlanner 之后激活。当前仓库没有已合并或 executable-qualified 的 P0.3.2 分支，因此本 tranche 只能作为 stacked source candidate：

```text
implemented=true
wired=false
qualified=false
activation_blocked_by=P0.3.2
```

P0.3.2 未资格化不妨碍编写和审阅纯 resolver contract，但阻止 runtime registration 和 production gate。

## 6. 测试矩阵

### Schema

- schema 包含 `quote`、`occurrence` 和 `segment_id`；
- schema 不包含 `start_byte`、`end_byte` 或 `sha256`；
- unknown fields 由 serde contract 拒绝；
- v4 registration/authority 常量保持 false。

### Quote resolution

- ASCII exact match；
- 中文/中英混合 UTF-8 boundary；
- 重复 quote 的 occurrence 0/1；
- quote 缺失；
- occurrence 越界；
- occurrence 上限；
- empty/NUL/oversized quote。

### Segment resolution

- deterministic segment ID；
- source-bound digest；
- tampered ID；
- tampered digest；
- invalid range；
- UTF-8 split；
- duplicate segment ID；
- unknown segment ID。

### Fact-set validation

- entity + relation happy path；
- duplicate entity key；
- duplicate relation key；
- unknown relation endpoint；
- missing selector；
- too many selectors；
- duplicate resolved span；
- total selector overflow。

## 7. Qualification

```bash
python3 scripts/verify-hepta-intelligence-evidence-resolver-v4.py

cd codex-rs
cargo fmt --all -- --check
cargo test -p codex-hepta-memory-extension evidence_resolver_v4 -- --nocapture
cargo test -p codex-hepta-memory fact_grounding -- --nocapture
cargo test -p codex-hepta-memory-extension
cargo test -p codex-hepta-memory
cargo clippy -p codex-hepta-memory-extension --all-targets -- -D warnings
cargo clippy -p codex-hepta-memory --all-targets -- -D warnings
```

Executable qualification receipt 必须绑定 exact commit/tree、所有命令、exit code、测试数量以及 authority flags。`steps=[]` 或 `runner_id=0` 仍分类为 `BLOCKED_RUNNER_NOT_ASSIGNED`，既不是失败也不是 PASS。

## 8. Exit gate

P0.3.3 只有在以下条件同时成立时才可标记 `qualified=true`：

- exact-head source gate PASS；
- Rust formatter、focused tests、full tests、strict clippy PASS；
- multilingual UTF-8 与 duplicate occurrence tests PASS；
- tampered segment tests PASS；
- schema 不暴露 model-owned offsets/digests；
- tool 仍未注册；
- P0.3.2 只作为独立依赖状态报告，不被伪造为 qualified；
- production authority、external effects、operator acceptance、promotion、CALLERS 均为 false。

## 9. 下一步

P0.3.3 资格化后，下一 tranche 是 P0.3.4：

```text
legacy inventory
→ grounded/backfilled/quarantined/zero_fact classification
→ deterministic backfill candidate generation
→ operator review receipts
→ coverage and false-admission evaluation
```
