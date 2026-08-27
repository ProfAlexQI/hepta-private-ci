# Hepta Intelligence Development Plan v2

**日期**：2026-08-28（Asia/Singapore）  
**状态**：ACTIVE DEVELOPMENT PLAN / FAIL-CLOSED PROMOTION  
**仓库**：`ProfAlexQI/hepta-private-ci`  
**默认分支**：`integration/vnext-main-20260811`  
**文档快照 head**：`fe0889ecd46a5fc89de7b1ff3f28158c133a3502`  
**canonical code parent**：`a85612afb43af722c61b54efe73570b25e9e4031`  
**本轮开发分支**：`codex/hepta-intelligence-grounding-v2-20260828`

> 本计划把 Hepta Intelligence 从“安全、可追溯的认知数据底座 + shadow qualification”推进为“事实可证、检索有效、决策可校准、反馈可归因、artifact 可治理晋升”的完整 intelligence plane。任何代码存在、测试通过或 shadow receipt 都不自动获得 production authority、operator acceptance、CALLERS promotion 或 external-effect 权限。

---

## 1. 执行摘要

当前 Hepta Intelligence 已经具备较强的 Agent-local SQLite durability、append-only lineage、scope/federation capability、CAS、generation fencing、physical-send revalidation、compact/outbox/recovery 和 H5–H7 shadow contracts。当前最主要的能力差距不是“有没有 memory/KG”，而是：

1. memory 的来源文本被逐字见证，不等于每条 caller-supplied KG fact 被该来源支撑；
2. 检索主路径仍以 lexical FTS、entity FTS、one-hop KG、recency 和固定 RRF 为主；
3. H5 neuron、H6 intuition、H7 learning 尚未组成可测量的在线闭环；
4. 安全退化大多 fail-open，但缺少可解释、隐私安全的 Intelligence telemetry；
5. 实现状态、运行接线、qualification、efficacy、operator acceptance 与 promotion 仍容易被长篇文档混淆。

本计划采用以下优先级：

```text
事实正确性
  → 状态机正确性
  → 检索与上下文质量
  → 可观测性与评测
  → 最小闭环学习
  → 统计安全晋升
  → production promotion
```

首个开发 tranche 已在本分支实现 qualification-only source-span fact grounding。它在现有 atomic memory/KG writer 之前校验每条 entity/relation 的精确证据区间，并生成 digest-bound、明确无 production authority 的 grounding receipt。它暂不持久化、不改变当前 projection authority，也不改变默认 read-only 产品配置。

---

## 2. 不可破坏的系统边界

### 2.1 Authority invariants

以下约束贯穿所有阶段：

- 唯一 authoritative mutation owner 仍是 Agent-local writer；
- NDU 只提出、评估和签署 artifact，不直接拥有 production execution authority；
- H5/H6 输出在获得单独晋升前保持 shadow/suggest-only；
- TaskFlow 负责可靠执行已经批准的图，不决定事实真伪；
- Codex / Agent execution spine 继续拥有实际 model/tool execution；
- federation 必须通过显式 capability、scope、generation、expiry 和 physical-send revalidation；
- compact summary 永远不是事实 admission；
- 任一 receipt、artifact 或 checkpoint 的 digest/binding 不匹配必须 fail closed；
- 默认 binary 保持认知写入关闭，除非显式 profile、host capability 与 authority verifier 同时成立；
- 任何阶段不得以“CI green”代替 operator acceptance、真实 efficacy 或 production promotion。

### 2.2 三种“正确性”必须分离

后续 schema 与 API 必须显式区分：

| 层级 | 证明内容 | 不证明内容 |
|---|---|---|
| `source_witness` | 来源文本与当前输入逐字一致 | 文本中的陈述真实 |
| `fact_grounding` | 某条结构化事实被来源中的精确证据区间支撑 | 外部世界中该事实为真 |
| `truth_status` | 经可信工具、用户确认、交叉来源或治理规则确认 | 永久不变或适用于所有时间 |

禁止继续把一个二元 `Verified` 同时解释为以上三层。

### 2.3 兼容与回滚原则

- 新能力优先通过新类型、新 wrapper 或新 migration 接入，不破坏现有 caller；
- legacy facts 在完成 backfill 前不得被自动升级为 grounded；
- 新 projection gate 必须支持 shadow compare、双写校验和一键回滚；
- migration 必须 append-only，并更新 migration ledger、required schema object oracle 与 reopen verifier；
- 任一新模型或 tokenizer 必须有 deterministic fallback；
- 新 telemetry 默认不记录 query、memory body、citation body、secret 或 PII。

---

## 3. 目标架构

```text
                          Governance / Safety Plane
          authority · capability · budget · receipts · signing · rollback
                                      │
                                      ▼
┌──────────────────────────────── Hepta Intelligence ────────────────────────────────┐
│                                                                                    │
│ Source Ledger                                                                      │
│   └─ SourceWitnessReceipt                                                          │
│          └─ MemoryCandidate / MemoryRevision                                       │
│                 └─ FactCandidate[]                                                  │
│                        └─ FactGroundingReceipt[]                                    │
│                               └─ TruthStatus / contradiction / temporal validity    │
│                                      └─ Grounded KG generations                    │
│                                                                                    │
│ Query Planner                                                                      │
│   ├─ lexical FTS                                                                   │
│   ├─ local embeddings / ANN                                                        │
│   ├─ KG entity + bounded multi-hop                                                 │
│   ├─ recency / lifecycle / source reliability                                      │
│   └─ contradiction and trust-zone filters                                          │
│                 └─ calibrated reranker                                             │
│                        └─ physical-send revalidation                                │
│                               └─ bounded context envelope                           │
│                                                                                    │
│ Feature Builder → H5 Neuron Signals → H6 DecisionReceipt → approved TaskFlow       │
│        ▲                                                        │                  │
│        └──── trajectory / outcome / credit / OPE / calibration / drift ────────────┘
│                                                                                    │
└────────────────────────────────────────────────────────────────────────────────────┘
                                      │
                                      ▼
                     TaskFlow Runtime / Codex Execution Spine
```

---

## 4. Capability 状态模型

所有 capability 必须使用统一六阶段状态，不再用一个“完成/未完成”字段：

1. `implemented`：代码或 schema 存在；
2. `wired`：进入明确的 runtime caller；
3. `qualified`：精确 head、测试、receipt 与 reopen/replay 证据通过；
4. `efficacy_proven`：真实、脱敏 corpus 或受控在线实验达到质量门；
5. `operator_accepted`：人工审阅和 rollback rehearsal 完成；
6. `promoted`：CALLERS / release authority 明确允许。

任一上游状态为 false 时，下游状态必须为 false。

---

## 5. Workstream 总览

| ID | Workstream | 当前 | 目标 | 依赖 |
|---|---|---|---|---|
| P0.1 | Source-span fact grounding contract | 本分支已实现，待 CI | qualification pass | 无 |
| P0.2 | Durable grounding ledger | 未实现 | append-only SQLite receipt/span tables | P0.1 |
| P0.3 | Tool protocol + projection grounding gate | 未实现 | 未 grounded facts 不进入 active projection | P0.2 |
| P0.4 | Intelligence mutation orchestration | 分散 helper | 单一 typed state machine | P0.2 |
| P1.1 | Hybrid retrieval | lexical/RRF | lexical + vector + KG + reranker | P0.3 |
| P1.2 | Intelligence telemetry | 局部/不足 | privacy-safe funnel + latency + quality | P0.3 |
| P1.3 | Federated recall scaling | 串行 fan-out | bounded concurrency + snapshot receipts | P1.2 |
| P1.4 | Memory semantic security | 基础 secret filter | semantic type/trust zone/instruction authority | P0.3 |
| P2.1 | Minimal H5/H6/H7 closed loop | shadow components | one low-risk end-to-end loop | P1.1–P1.4 |
| P2.2 | Statistical promotion gate | point estimates | CI/LCB/ESS/subgroup no-regression | P2.1 |
| P2.3 | Artifact reload/rollback | qualification artifact | governed next-snapshot reload | P2.2 |
| P3 | Intelligence crate decomposition | god crate | narrow façade + bounded crates | P0–P2 stabilized |

---

# 6. P0：事实与状态正确性

## 6.1 P0.1 — Qualification-only source-span fact grounding

### 目标

在不修改现有 durable schema 与 production authority 的前提下，建立第一条严格 contract：每条非空 KG entity/relation 都必须绑定来源文本中的 1–4 个精确 UTF-8 byte spans。

### 本分支实现

新增 `fact_grounding.rs`：

- `FactEvidenceSpanDraft`：fact kind/key、start/end byte、evidence SHA-256；
- `GroundedKgFactSetDraft`：原有 `KgFactSetDraft` + evidence spans；
- entity 支撑规则：证据合并文本必须包含规范化 entity label；
- relation 支撑规则：证据合并文本必须包含 source label、target label 和 predicate；
- 校验 source UTF-8、byte range、char boundary、digest、重复、孤儿、遗漏和数量上限；
- `FactGroundingReceipt` 绑定 memory revision、source revision、source digest、fact-set digest、fact identity digest 和全部 spans；
- `remember_with_grounded_kg` / `correct_with_grounded_kg` 在现有 writer 之前完成 grounding 校验；
- receipt 固定：`durable_persistence=false`、`production_authority=false`、`projection_gate=false`；
- 无效 grounding 在进入数据库事务前失败，不能产生 source/memory rows。

### 关键限制

- receipt 当前只在调用返回值中存在，尚未落 SQLite；
- 现有 `remember_with_kg` 仍可被 legacy caller 使用；
- 当前 active KG projection 尚未强制要求 persisted grounding；
- textual support 不等于 external truth；
- 当前语义匹配是确定性规范化包含关系，不是自然语言蕴含模型。

### 验收命令

```bash
cargo fmt --all -- --check
cargo test -p codex-hepta-memory fact_grounding -- --nocapture
cargo test -p codex-hepta-memory cognitive_intelligence_writer -- --nocapture
cargo test -p codex-hepta-memory cognitive_kg_store -- --nocapture
cargo clippy -p codex-hepta-memory --all-targets -- -D warnings
```

### Exit gate

- 所有新增测试通过；
- formatting/clippy 无告警；
- receipt tamper、missing evidence、digest drift、UTF-8 split、unknown fact 与 no-mutation 路径被覆盖；
- PR 明确标记 qualification-only；
- 不修改 production caller、feature default、CALLERS 或 promotion flag。

---

## 6.2 P0.2 — Durable grounding ledger

### 目标

将 P0.1 receipt 作为 Agent-local append-only evidence 持久化，并在数据库 reopen 时完整复核。

### Schema 设计

新增 migration `0011_fact_grounding.sql`，至少包含：

```text
kg_revision_fact_grounding_receipts
  memory_id
  memory_revision
  source_id
  source_revision
  grounding_contract
  source_content_sha256
  fact_set_sha256
  fact_identity_sha256
  evidence_count
  receipt_sha256
  recorded_at_unix_seconds

kg_revision_fact_grounding_spans
  memory_id
  memory_revision
  fact_kind
  fact_key
  evidence_ordinal
  start_byte
  end_byte
  evidence_sha256
```

约束：

- receipt/span 不允许 update/delete；
- span count 必须与 receipt 一致；
- source/memory/fact-set foreign key 精确绑定；
- byte range 非空、有序，ordinal 每 fact 从 0 连续；
- 不允许 duplicate span；
- grounding receipt 与 memory/KG facts 在同一 `BEGIN IMMEDIATE` transaction 中提交；
- crash 前后不会出现“facts 已投影但 grounding receipt 缺失”的半状态。

### 必须同步更新

- `MIGRATOR` exact lineage 从 1..10 更新为 1..11；
- `REQUIRED_SCHEMA_OBJECTS` 增加 tables、triggers、indexes；
- 更新 `REQUIRED_SCHEMA_ORACLE_SHA256`；
- `verify_store` 新增 receipt/span cardinality、digest、source-byte、fact identity 与 projection eligibility 校验；
- read-only federation store reopen 同样验证 grounding ledger；
- migration checksum 与历史数据库升级测试；
- corruption injection 测试：删 span、改 digest、改 ordinal、改 source binding 均 fail closed。

### Backfill 分类

legacy fact set 必须分类为：

- `grounded_v1`：新 writer 原生持久化；
- `legacy_unreviewed`：历史 caller-supplied facts，不可自动升级；
- `backfilled_grounded_v1`：经过离线 span extractor + deterministic verifier + operator review；
- `quarantined`：无足够 evidence；
- `zero_fact`：不需要 grounding。

### Exit gate

- 新数据库和 0001–0010 历史数据库均可升级并 reopen；
- failpoint 覆盖 insert receipt、insert spans、projection refresh、commit 前后；
- 任何有 facts 的 current head 都必须有完整 grounding status；
- legacy facts 尚未全部 backfill 时，production gate 保持关闭。

---

## 6.3 P0.3 — Tool protocol 与 projection grounding gate

### Tool contract

`hepta_cognitive.remember` / `correct` 的结构化 KG 输入升级为：

```json
{
  "entities": [{
    "key": "aurora",
    "entity_type": "project",
    "label": "Project Aurora",
    "evidence": [{"start_byte": 0, "end_byte": 14, "sha256": "..."}]
  }],
  "relations": [{
    "key": "aurora-uses-rust",
    "from_entity_key": "aurora",
    "to_entity_key": "rust",
    "relation": "uses",
    "evidence": [{"start_byte": 0, "end_byte": 24, "sha256": "..."}]
  }]
}
```

禁止模型只提供“已验证=true”布尔值。所有 span 必须由 host 对当前 exact witness 重算。

### Projection gate

active projection 只包含：

```text
memory.verification == verified
AND memory.lifecycle == active
AND grounding_status in {grounded_v1, backfilled_grounded_v1}
AND grounding receipt validates against current source/fact set
AND truth_status not in {disputed, contradicted, quarantined}
```

迁移期间采用 shadow compare：

- current legacy projection；
- grounded candidate projection；
- 记录差异，不影响当前回答；
- 完成 coverage/quality gate 后再切换 read pointer；
- 保留一键回滚到前一 generation。

### Exit gate

- 无 evidence 的模型输出不能写正式 KG；
- legacy facts 不会被静默解释为 grounded；
- physical-send attachment 记录 grounding receipt digest；
- explain API 返回 source span、grounding status 和 truth status；
- E2E 覆盖 remember/correct/forget/replay/restart/federation。

---

## 6.4 P0.4 — 单一 Intelligence mutation state machine

将 memory admission、fact grounding、projection、outbox、compact、rehydration 与 terminalization 的顺序收敛为宿主拥有的 typed orchestration：

```text
Planned
  → SourceWitnessed
  → GroundingValidated
  → DurableIntentAppended
  → MemoryFactsCommitted
  → ProjectionPublished
  → OutboxSettled
  → Terminal
```

异常状态：

```text
Indeterminate
  → ReconciledApplied
  → ReconciledNotApplied
  → Quarantined
```

必须：

- 使用一个显式 transition table；
- 所有 transition 绑定 operation id、lease/generation、expected revision 与 causal parent；
- terminalize 前必须 settle durable intent；
- property tests 生成 duplicate/reorder/crash/restart；
- SQLite failpoint 覆盖每个持久化边界；
- 编写轻量 TLA+/PlusCal 或等价状态模型，验证无双写、无 stranded intent、无 stale generation publish。

---

# 7. P1：检索、上下文与可观测性

## 7.1 P1.1 — Hybrid retrieval v2

### Candidate channels

- exact lexical / FTS5；
- normalized alias/entity lookup；
- local embedding ANN；
- KG one-hop + bounded two-hop；
- recency、lifecycle、source reliability；
- contradiction/truth-status filter；
- optional small calibrated reranker。

### Query planner

输入不再直接等同于整段用户文本。planner 输出：

```text
intent
entities
lexical_terms
semantic_query
required_time_range
workspace/scope
risk_class
retrieval_budget
```

planner 必须有 deterministic fallback，并记录 planner/model/tokenizer digest。

### Rank contract

每个 candidate 保留：

- per-channel rank/score；
- fused score；
- freshness；
- grounding/truth status；
- source reliability；
- contradiction flag；
- revalidation binding；
- optional reranker receipt。

### Token budgeting

- 增加 tokenizer registry；
- tokenizer digest 进入 RunStartSnapshot；
- prepare 阶段可保守估计；
- physical send 前按实际 model/tokenizer 精确重算；
- tokenizer 不可用时回退 bytes + safety margin；
- receipt 记录 estimate、actual、estimator type。

### Efficacy gate

真实脱敏 corpus 至少覆盖中文、英文、中英混合、同义改写、时间冲突、撤销、跨 workspace 和 federation。指标：

- Recall@4、nDCG@4；
- citation precision；
- false-memory attachment rate；
- stale/contradicted attachment rate；
- task success delta；
- p50/p95/p99 latency；
- context token cost；
- no-memory baseline 对照。

---

## 7.2 P1.2 — Privacy-safe Intelligence telemetry

### Funnel events

```text
recall_attempted
candidate_found
attachment_prepared
revalidation_passed
attachment_sent
attachment_abstained
```

### Reason taxonomy

`no_match`、`stale`、`revoked`、`expired`、`secret`、`budget`、`timeout`、`corrupt`、`grounding_missing`、`contradicted`、`tokenizer_unavailable`。

### Privacy rules

- 不记录 query、memory body、citation body；
- 只记录枚举、计数、延迟桶、匿名 digest；
- workspace/agent 维度需最小聚合阈值；
- telemetry schema/version 与 retention policy 可审计；
- 任何 exporter 失败不得阻塞主 turn。

---

## 7.3 P1.3 — Federated recall scaling

- owner source 间有界并发；
- 每 source 一个只读 snapshot transaction；
- 总 deadline 分配 source 子预算；
- batch revalidation 返回 source snapshot digest；
- source circuit breaker 与短期健康缓存；
- merge receipt 明确 complete/partial/timed-out sources；
- 高风险任务可声明 all-required-sources；普通 recall 允许 partial；
- capability 撤销、generation drift 与 expiry 在发送前再次检查。

Exit gate：source 数量扩展时 p99 不随 N 线性恶化，且慢/坏 owner 不拖垮本地回答。

---

## 7.4 P1.4 — Memory semantic security

每条 memory 增加正交分类：

```text
semantic_type:
  fact | preference | instruction | observation | summary | external_content

instruction_authority:
  none | explicit_user | workspace_policy | system_prohibited

trust_zone:
  user | tool | repository | external | model_generated

truth_status:
  candidate | grounded | confirmed | disputed | contradicted | expired
```

规则：

- recalled content 默认作为数据引用，不作为指令执行；
- 长期 instruction 只允许 explicit user 或 trusted workspace policy；
- external/model-generated text 不能提升 instruction authority；
- DLP 扩展到 PII、编码 secret、高熵 token 与 provider-specific credentials；
- explain 输出必须可见 semantic/trust/authority 分类。

---

# 8. P2：最小闭环学习与安全晋升

## 8.1 P2.1 — 一个低风险 position 的完整闭环

优先选择 `MemoryRetrievalRank` 或 `ContextSalience`，禁止第一步就接高风险 tool execution。

```text
RunStartSnapshot
  → deterministic feature builder
  → H5 neuron signal
  → H6 SuggestOnly DecisionReceipt
  → shadow/controlled attachment choice
  → outcome/postcondition
  → durable H7 trajectory
  → credit assignment
  → offline evaluation
```

要求：

- behavior propensity 与 target propensity 明确记录；
- outcome 不使用模型自评作为唯一真值；
- 支持用户 correction、forget、task completion、tool postcondition 等客观信号；
- shadow 与 baseline 同时跑，生产输出仍由 baseline 决定；
- 只有低风险、可回滚 canary 才允许影响选择。

---

## 8.2 P2.2 — OPE / CI / no-regression

promotion receipt 必须包含：

- sample count、supported count、coverage；
- effective sample size；
- importance-weight max/p95、clipped ratio；
- point estimate；
- confidence interval / lower confidence bound；
- baseline upper bound；
- minimum practical improvement；
- workspace、任务类型、语言、模型 backend、风险等级 subgroup；
- safety floor 与 critical subgroup no-regression；
- 多窗口 drift/retention/forgetting；
- dataset/query/artifact/policy/model/tokenizer digests。

晋升条件：

```text
candidate LCB > baseline UCB + minimum practical improvement
AND ESS >= threshold
AND support coverage >= floor
AND clipped ratio <= ceiling
AND all critical safety metrics pass
AND no critical subgroup regression
```

---

## 8.3 P2.3 — Governed artifact reload 与 rollback

- artifact 签名与 trust anchor 分离；
- artifact 只在 next RunStartSnapshot 生效；
- 当前 run 不允许 mid-flight policy mutation；
- canary 比例、scope、expiry 与 kill switch 显式；
- rollback artifact 已预生成并演练；
- reload 失败回退上一 approved artifact；
- promotion receipt 不等于 production authority，仍需 operator acceptance 与 CALLERS ratchet。

---

# 9. P3：crate 与 API 边界优化

`codex-hepta-memory` 当前同时承担 ledger、KG、retrieval、federation、compact、outbox、H5–H7、artifact 和 production writer。稳定 P0–P2 后按以下顺序拆分：

```text
hepta-cognitive-contracts
hepta-cognitive-ledger
hepta-kg
hepta-retrieval
hepta-federation
hepta-compact
hepta-policy-runtime
hepta-learning
hepta-authority-runtime
hepta-intelligence      # narrow façade
```

拆分原则：

- 先 internalize qualification-only API；
- 再移动纯 contract；
- 再移动 storage/retrieval；
- 最后保留窄 façade；
- 每一步保持 crate graph 无环和 public API compatibility receipt。

---

# 10. 测试与 qualification 矩阵

| 类型 | 必须覆盖 |
|---|---|
| Unit | canonicalization、digest、span、state transition、rank fusion、OPE math |
| Property | duplicate/reorder/crash/restart、arbitrary UTF-8、tamper、overflow |
| SQLite | migration、reopen、foreign key、triggers、quick_check、corruption injection |
| E2E | remember/correct/forget/recall/explain/federation/compact/recovery |
| Security | prompt injection、scope escape、stale capability、secret/PII、receipt forgery |
| Efficacy | multilingual corpus、contradiction、temporal drift、task-success delta |
| Performance | p50/p95/p99、DB growth、compaction loss、federation scaling |
| Rollback | schema rollback strategy、projection pointer、artifact rollback、kill switch |

Qualification receipt 至少绑定：

- exact commit/tree；
- dirty-state false；
- command list与 exit code；
- test counts；
- schema/migration digests；
- feature/CALLERS state；
- authority flags；
- known limitations；
- operator acceptance/promotion false，除非另有独立授权。

---

# 11. 风险登记

| 风险 | 影响 | 缓解 |
|---|---|---|
| textual support 被误当作 truth | 错误知识进入决策 | 三层状态分离、truth gate、contradiction handling |
| legacy facts 无 grounding | projection 可信度不一 | quarantine/backfill/shadow compare |
| migration/oracle 漂移 | 数据库无法 reopen 或错误接受 | exact ledger、schema oracle、历史升级测试 |
| grounding 过严导致低 recall | 正确事实被拒绝 | 多 span、alias policy、离线误拒分析；不放宽 authority |
| grounding 过松 | 幻觉事实进入 KG | deterministic support + optional NLI 只能作辅助，不替代 spans |
| hybrid retrieval 增加延迟 | turn p99 上升 | bounded budget、fallback、deadline、cache |
| federation fan-out | 尾延迟/部分状态 | bounded concurrency、snapshot receipt、partial semantics |
| telemetry 泄露 | 隐私风险 | no-content schema、aggregation、retention、audit |
| H7 selection bias | 错误晋升 | propensity/support/ESS/CI/subgroup gates |
| god crate 持续膨胀 | 审计和维护成本 | 稳定后分层拆 crate，窄 façade |

---

# 12. 具体提交序列

## Tranche A — 当前分支

1. `feat(memory): add qualification fact-grounding contract`
   - 新增 `fact_grounding.rs`；
   - 更新 `lib.rs` exports；
   - 内联 unit/E2E tests；
   - 不改 migration 和 production caller。
2. `docs(plan): activate Hepta Intelligence development plan v2`
   - 本计划；
   - machine-readable execution status。
3. CI 修复提交
   - 只修 fmt、clippy、test 与 contract 缺陷；
   - 不扩大权限。

## Tranche B — durable ledger

1. migration 0011；
2. atomic writer persistence；
3. reopen verifier/oracle/ledger update；
4. crash/corruption/backward-upgrade qualification；
5. legacy scanner 与 status inventory。

## Tranche C — tool/projection gate

1. tool schema v3 evidence spans；
2. grounded writer wiring；
3. projection shadow compare；
4. explain/recall grounding metadata；
5. product E2E 与 feature-gated canary。

## Tranche D — retrieval/telemetry/federation

1. tokenizer registry；
2. retrieval channel contract；
3. embedding adapter与 deterministic fallback；
4. telemetry funnel；
5. bounded-concurrent federation；
6. multilingual efficacy receipt。

## Tranche E — H5/H6/H7 loop

1. feature builder；
2. low-risk position shadow decision；
3. durable outcome/trajectory；
4. CI/ESS evaluation；
5. signed next-snapshot artifact；
6. canary/rollback rehearsal；
7. operator acceptance request。

---

# 13. 当前 Definition of Done

本轮只在以下条件同时满足时可声明 P0.1 “qualified”：

- branch head 绑定明确；
- 新 module 与 exports 已提交；
- focused tests、full crate tests、fmt、clippy 通过；
- CI artifacts 可读取；
- invalid grounding zero-mutation 测试通过；
- receipt tamper 与 authority-boundary 测试通过；
- plan/status 文件与代码一致；
- `durable_persistence=false`；
- `production_authority=false`；
- `projection_gate=false`；
- operator acceptance、promotion、CALLERS ratchet 均保持 false。

若任何条件未满足，状态必须是 `implemented_on_branch` 或 `qualification_failed`，不得写成 `qualified`。

---

# 14. 决策

下一步开发以 P0.1 → P0.2 → P0.3 顺序推进。禁止跳过 durable grounding ledger 直接把 caller-supplied KG facts 宣称为生产可信；禁止在真实 efficacy 与统计晋升门完成前放开 H5/H6/H7 production decision authority。
