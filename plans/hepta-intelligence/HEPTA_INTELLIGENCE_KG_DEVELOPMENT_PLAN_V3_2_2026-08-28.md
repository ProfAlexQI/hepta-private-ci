# Hepta Intelligence KG Development Plan v3.2

**日期**：2026-08-28  
**状态**：ACTIVE / FAIL-CLOSED PROMOTION  
**仓库**：`ProfAlexQI/hepta-private-ci`  
**当前 authoritative default branch**：`integration/vnext-main-20260811`  
**P0.3 source base**：`codex/hepta-intelligence-grounding-gate-v3-20260828@256a47d882413ac1f80892a80177419bb5d12c70`

## 1. 当前判断

KG 核心是 Rust 主导的 cognitive graph runtime，但生产可信闭环仍缺少：

```text
fact grounding authority
truth-status separation
semantic shadow/production parity
legacy governance
scalable projection
hybrid retrieval efficacy
```

v3.2 保持以下优先级：

```text
source witness
→ fact grounding
→ shared semantic projection
→ host-owned evidence resolution
→ legacy governance
→ truth status
→ projection scaling
→ hybrid retrieval
→ efficacy and promotion
```

任何 capability 均使用六阶段状态：

```text
implemented
wired
qualified
efficacy_proven
operator_accepted
promoted
```

上游状态为 false 时，下游必须为 false。

## 2. 不可破坏边界

- Agent-local writer 仍是唯一 mutation owner；
- source witness、fact grounding、truth status 必须正交；
- textual support 不得被解释为 external truth；
- legacy facts 不得自动升级为 grounded；
- shadow read 不得隐式 migration 或写 projection；
- production/shadow planner 必须共享 canonicalization、conflict 和 output digest；
- model 不得生成或声称 byte offset、digest 或 truth authority；
- physical-send 前必须复核 scope、generation、receipt 和 lifecycle；
- 所有 digest/binding mismatch fail closed；
- CI green 不等于 efficacy、operator acceptance 或 promotion。

## 3. P0.3 分解与当前状态

| Tranche | 内容 | 当前状态 | 激活依赖 |
|---|---|---|---|
| P0.3.1 | Shadow authorization、单 snapshot、read-only schema verify | patch 曾在外部准备；仓库未资格化 | P0.2 |
| P0.3.2 | Shared semantic ProjectionPlanner | patch 曾在外部准备；仓库未资格化 | P0.3.1 |
| P0.3.3 | HOST_OWNED_EVIDENCE_RESOLUTION | 本候选分支 source-only implemented | P0.3.2 qualified |
| P0.3.4 | Legacy inventory/backfill/quarantine | 未实现 | P0.3.3 |
| P0.3.5 | Dual pointer shadow/canary/rollback | 未实现 | P0.3.4 |
| P0.3.6 | Production grounded projection gate | 未实现 | P0.3.5 + truth gate |

“外部准备的 patch”不算 repository implementation，也不算 qualification evidence。v3.2 因此不会把 P0.3.1/P0.3.2 标记为完成。

## 4. P0.3.3 — Host-Owned Evidence Resolution

### 4.1 问题

v3 模型协议要求模型提交：

```text
start_byte
end_byte
sha256
```

该协议把机械、精确且容易因 UTF-8 出错的工作交给模型，导致：

- 中文和 emoji byte/character 偏差；
- 重复 quote 选择不确定；
- SHA-256 幻觉或格式错误；
- 高 invalid-call rate；
- 模型对 evidence binding 拥有了不必要的声明面。

### 4.2 新协议

模型只做语义选择：

```json
{"quote":"...","occurrence":0}
```

或引用 host 预先签发的 segment：

```json
{"segment_id":"source-segment:v1:..."}
```

Rust host 负责：

```text
exact witness lookup
→ deterministic occurrence resolution
→ UTF-8 byte boundaries
→ SHA-256
→ deterministic segment identity
→ per-fact duplicate detection
→ FactEvidenceSpanDraft
```

### 4.3 Contract invariants

- quote 是 exact bytes，不做 normalization；
- occurrence 是 zero-based、左到右，并以 UTF-8 字符步进支持确定性的重叠匹配；
- segment ID 绑定 start/end/evidence digest；
- host registry 中 ID 必须唯一；
- source/range/digest/ID 任一不匹配 fail closed；
- same fact 的重复 resolved span fail closed；
- different facts 可共享同一 source span；
- 输出按 range/digest 稳定排序；
- schema 不暴露 offsets/digest；
- v4 module 只编译，不进入 `ToolContributor`。

### 4.4 Source-only authority

```text
implemented=true
wired=false
qualified=false
efficacy_proven=false
operator_accepted=false
promoted=false
tool_v4_registered=false
production_authority=false
external_effects=false
```

## 5. P0.3.4 — Legacy Governance

P0.3.3 后立即执行：

### Inventory schema

```text
memory_id
revision
scope
fact_set_sha256
grounding_status
entity_count
relation_count
source_kind
review_state
inventory_receipt_sha256
```

状态：

```text
grounded_v1
backfilled_grounded_v1
legacy_unreviewed
quarantined
zero_fact
```

### Backfill pipeline

```text
legacy immutable fact set
→ quote/segment candidates
→ host resolver
→ deterministic grounding verifier
→ review bundle
→ operator accept/reject
→ append-only backfill receipt
```

不得自动修改原 fact rows；backfill 只增加 evidence/decision ledger。

### Exit metrics

- legacy fact-set coverage；
- candidate resolution success rate；
- operator acceptance rate；
- false-support sample rate；
- multilingual subgroup；
- unresolved/quarantined count；
- DB/reopen cost。

## 6. P0.5 — Claim-Centric Truth Model

Grounding 后仍需独立 truth layer：

```text
ClaimRevision
  subject
  predicate
  object_entity | object_value
  qualifiers
  valid_time
  system_time
  grounding_status
  truth_status
  confidence/calibration
  evidence_bindings[]
  contradicts/retracts/supersedes
```

truth status 最少包括：

```text
candidate
grounded
confirmed
disputed
contradicted
expired
retracted
quarantined
```

Projection eligibility 必须同时检查 grounding 与 truth，而非继续滥用二元 `Verified`。

## 7. P1.0 — Projection Scalability

当前完整 generation materialization 会造成写放大和历史数据接近 O(N²) 增长。稳定 correctness 后改为：

```text
immutable fact log
→ projection delta
→ rebuildable current index
→ signed checkpoint
→ governed historical GC
```

性能门：

- 10k heads mutation p95/p99；
- DB growth per mutation；
- reopen verification time；
- current-index rebuild time；
- checkpoint/rollback correctness；
- no lineage loss。

## 8. P1.1 — Hybrid Retrieval

在 grounded/truth gate 后加入：

```text
lexical FTS
+ alias/entity lookup
+ local embedding ANN
+ bounded 2-hop KG
+ recency/source reliability
+ contradiction filter
+ calibrated reranker
```

每个 candidate receipt 绑定 channel scores、grounding/truth status、source reliability、planner/model/tokenizer digest 和 physical-send revalidation。

## 9. Qualification 总矩阵

| 类别 | 必须覆盖 |
|---|---|
| Unit | selector validation、UTF-8、occurrence、segment ID、digest、planner |
| Property | arbitrary UTF-8、duplicate/reorder、overflow、tamper |
| SQLite | migration、reopen、corruption、snapshot、failpoint |
| E2E | remember/correct/forget/restart/replay/federation/explain |
| Security | scope escape、prompt injection、secret/PII、receipt forgery |
| Performance | p50/p95/p99、DB growth、rebuild、federation scaling |
| Efficacy | multilingual recall、citation precision、false attachment |
| Rollback | pointer、schema strategy、artifact rollback、kill switch |

## 10. 当前执行序列

```text
P0.3.3 source candidate
  → exact-head source gate
  → fmt/focused/full/clippy
  → P0.3.2 independent qualification
  → P0.3.3 dependency rebase and requalification
  → P0.3.4 legacy inventory
  → P0.3.5 dual pointer canary
  → P0.3.6 production gate
```

P0.3.3 可以在 P0.3.2 尚未资格化时并行编写，但不得 wired 或 promoted。

## 11. 当前 Definition of Done

本轮只能声明 `P0.3.3 implemented_on_branch`，并且必须同时满足：

- v4 contract 和 host resolver 已提交；
- model-facing schema 无 offsets/digest；
- source gate 可执行；
- dedicated workflow 存在；
- tool_v4_registered=false；
- P0.3.2 qualified=false 被明确记录；
- production authority/external effects/operator acceptance/promotion=false。

只有 exact-head Rust qualification 全通过后，才能把 `qualified` 改为 true；只有 P0.3.2 也独立通过后，才允许讨论 runtime wiring。
