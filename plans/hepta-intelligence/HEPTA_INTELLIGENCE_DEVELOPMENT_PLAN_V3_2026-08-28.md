# Hepta Intelligence Development Plan v3

## Causal Learning, Governed Adaptation and Bio-inspired Claim Discipline

**日期**：2026-08-28（Asia/Singapore）  
**状态**：ACTIVE SUPERSEDING DEVELOPMENT PLAN / PLAN-ONLY / FAIL-CLOSED  
**仓库**：`ProfAlexQI/hepta-private-ci`  
**继承计划**：`HEPTA_INTELLIGENCE_DEVELOPMENT_PLAN_V2_2026-08-28.md`  
**精确代码基线**：PR #23 head `7691978b786dd00c69477d1a3355be13db2c4d67`  
**P0.4c hardened source candidate**：`7bb26ec016c2e2c83084756485ea324e79bcddbe`  
**计划分支**：`codex/hepta-intelligence-plan-v3-20260828`  
**计划权限**：`production_authority=false`、`external_effects=false`、`operator_acceptance=false`、`promotion=false`、`callers_ratchet=false`

> 本计划把前一轮审计结论转化为可执行的工程合同。它不把任何 source-only、shadow、fixture、receipt 或 CI 配置提升为 runtime qualification、真实 efficacy、operator acceptance 或 production promotion。当前 P0.1–P0.4c 仍然没有获得可执行 runner 证据，P1/P2 运行接线继续冻结。

---

# 0. 执行摘要

当前 Hepta Intelligence 的真实能力不是“已经具备自我进化和仿生 Neuron/Intuition”，而是：

```text
强治理、强溯源、强事务边界
+ 较强长期记忆与 KG 基础
+ H5/H6/H7 shadow/qualification contracts
- 真实因果学习闭环
- 跨时间持续学习 efficacy
- 运行时 H5/H6 wiring
- 结构可塑性
- 神经动力学或神经形态机制
```

最新 P0 栈已经在 source 层实现：

```text
P0.1 source-span fact grounding
P0.2 durable fact-grounding ledger
P0.3 grounded tool v3 / projection shadow compare
P0.4a typed intelligence mutation state machine
P0.4b SQLite transition journal / failpoints
P0.4c shadow host orchestration adapter
```

但以上能力普遍仍为：

```text
implemented=true
wired=false
qualified=false
efficacy_proven=false
operator_accepted=false
promoted=false
```

因此 v3 的核心不是继续堆更多 shadow helper，而是按以下顺序关闭架构断点：

```text
冻结未 qualification 的代码栈
→ 建立唯一 current-plan / capability truth source
→ 提前拆出 learning data 与 policy runtime 边界
→ 建立统一 LearningEpisode 与 durable causal ledger
→ 完成 grounded hybrid retrieval、telemetry、semantic security
→ 把 H5 从参数提案器升级为可校准信号层
→ 把 H6 从确定性排序器升级为可选择性 abstain 的快速策略层
→ 把 H7 durable trajectory 与 feedback/OPE 合并为真实离线学习闭环
→ 经过 ESS/CI/subgroup/no-regression/retention 门
→ 仅在 next snapshot 加载 signed artifact
→ shadow → canary → operator → CALLERS
```

v3 对“自我进化”的允许范围限定为：

> **受治理、跨快照、可回放、可回滚的参数与策略适应。**

以下表面永远不允许由学习器直接修改：

```text
系统目标
authority / capability
trust root
safety invariant
workflow effect boundary
CALLERS
production writer ownership
provider credential policy
base release policy
```

---

# 1. 审计结论与必须修复的架构问题

## 1.1 Truth source 分散

当前架构事实分散在默认分支、Dropbox snapshot、V2 plan、tranche 文档、execution status、receipt、Draft PR body 和堆叠分支中。工程人员必须人工推导：

```text
哪份 plan 是当前有效解释
哪个 head 是实现基线
哪个 capability 只是 source-present
哪个 receipt 只是 shadow
哪个 branch 才是下一步 base
```

### v3 决策

新增唯一机器入口：

```text
plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json
plans/hepta-intelligence/HEPTA_INTELLIGENCE_EXECUTION_STATUS_V3.json
plans/hepta-intelligence/HEPTA_INTELLIGENCE_CLAIM_LADDER_V1.json
```

任何 README、PR、receipt 或 dashboard 的 capability 声明必须可追溯到该入口；历史文件保持 immutable lineage，但不再作为 current truth source。

---

## 1.2 未 qualification 的 PR 栈过深

P0.1 没有实际 runner 证据时，P0.2–P0.4c 已经继续堆叠。虽然所有 runtime/authority 保持关闭，但基础 API、migration 和 state-machine 一旦在后续 executable qualification 中失败，会引发整栈返工。

### v3 决策

实施 **Unqualified Stack Budget**：

```text
last_executable_qualified_base
  └─ 最多允许 2 层 source implementation Draft
```

例外仅包括：

```text
plan-only
schema-only
verifier-only
receipt-only
runner-infrastructure-only
```

当前 P0.1–P0.4c 已超过该预算，因此：

```text
P1.1 runtime/source implementation = frozen
新 H5/H6/H7 runtime code = frozen
新 migration = frozen
```

直到 P0.1–P0.4c 对同一冻结候选产生可读取的 executable qualification evidence。

---

## 1.3 当前形成的是治理闭环，不是学习闭环

Hepta 已能证明：

```text
谁在何时提出什么
绑定哪个 snapshot / lease / generation
是否越权
是否发生 durable write
能否 replay / reopen
receipt 是否被篡改
```

但尚不能证明：

```text
从哪条经验学到了什么
某个参数为何改变
改变后是否长期改善
旧能力是否遗忘
不同 workspace / language / risk subgroup 是否退化
用户 correction / forget 是否传播到后续 artifact
```

### v3 决策

P2 的前置条件不再只是 H5/H6/H7 类型存在，而是：

```text
LearningEpisodeV1
DurableLearningLedgerV1
OutcomeReceiptV1
CreditLedgerV1
EvaluationReceiptV2
DatasetSnapshotV1
PolicyArtifactManifestV2
```

全部形成一条可回放的因果链。

---

## 1.4 缺少统一 LearningEpisode

当前 Snapshot、NeuronProposal、IntuitionReceipt、Trajectory、Feedback、Artifact 可以分别自洽，却未必证明它们属于同一条经验。

### v3 决策

所有学习数据以 `LearningEpisodeV1` 为唯一顶层身份。禁止通过时间接近、字符串相似或日志拼接推断因果关系。

---

## 1.5 H7 durable trajectory 与 feedback/OPE 分裂

当前 durable trajectory 适合记录 qualification lifecycle，但 policy feedback、propensity、support 和 reward 仍被拒绝；纯 H7 feedback oracle 能计算固定点 OPE，却不是 durable ledger。

### v3 决策

新增独立 `hepta-learning-ledger`，定义 canonical `LearningEventV1` wire contract。现有 durable trajectory 与纯 feedback oracle 只能作为 migration input，不能继续演化为两套互不兼容的长期真相。

---

## 1.6 H5 Neuron 不是 neuron runtime

当前 H5 是确定性、纯函数、shadow-only 参数提案 seam；其主要算法是 bounded feature aggregate 与 ±500 bps clipping。它没有 temporal state、recurrent dynamics、synapse、local plasticity 或模型推理。

### v3 决策

当前能力声明固定为：

```text
H5 claim = N0_METAPHORICAL_TYPED_PROPOSAL
```

后续只有完成 H5 signal runtime、temporal state、artifact update 和跨窗口 efficacy 后，才允许提升为 `N1_ADAPTIVE_SIGNAL_UNIT`。神经形态或 spiking 研究另开独立 track，不能自动继承 production authority。

---

## 1.7 H6 Intuition 不是经验形成的直觉

当前 H6 是：

```text
hard filter
→ deterministic score rank
→ tie-break
→ confidence threshold
→ suggest / abstain
```

score、risk、evidence 由调用者提供，模块自身没有经验学习、隐状态、OOD、distributional uncertainty 或 temporal context。

### v3 决策

当前能力声明固定为：

```text
H6 claim = I0_DETERMINISTIC_SELECTIVE_POLICY
```

后续 H6 必须输出 candidate distribution、calibrated uncertainty、abstain、OOD 与 reason attribution，才能提升为经验塑造的快速策略层。

---

## 1.8 Statistical promotion 设计强于当前实现

V2 已要求 ESS、CI、LCB/UCB、subgroup no-regression，但当前实现更接近 deterministic point estimate。

### v3 决策

任何 canary 之前必须至少实现：

```text
supported sample count
coverage
ESS
importance weight p50/p95/max
clipped ratio
IPS + SNIPS
doubly robust estimate
cluster/block confidence interval
candidate LCB
baseline UCB
critical subgroup no-regression
sequential peeking policy
7d / 30d retention
```

没有 CI 的 point estimate 只允许标记 `exploratory_offline_result`。

---

## 1.9 Recall eligibility 不能等于 training eligibility

有 evidence span 的 memory 只证明来源支撑，不证明 truth，也不证明适合作为训练信号。

### v3 决策

每条数据必须正交记录：

```text
recall_eligible
training_eligible
evaluation_eligible
promotion_eligible
```

任何一项都不能从另一项隐式推导。

---

## 1.10 稳定性—可塑性问题尚未解决

长期存储不等于长期学习。系统必须吸收新经验，同时保持旧能力、响应 correction/forget、控制 drift 和避免污染积累。

### v3 决策

采用双速架构：

```text
Fast Episodic System:
  immutable episode / feedback / correction / outcome

Slow Consolidation System:
  dataset freeze / replay / anti-forgetting / evaluation /
  signed artifact / next-snapshot reload
```

禁止单次 run 内更新 policy、graph、authority 或模型 artifact。

---

## 1.11 `codex-hepta-memory` god crate 拆分过晚

将 crate decomposition 推迟到 P0–P2 全部完成会让 ledger、KG、retrieval、learning、artifact 和 authority 继续耦合。

### v3 决策

在 P2.1 前完成最小提前拆分：

```text
hepta-cognitive-contracts
hepta-cognitive-ledger
hepta-learning-contracts
hepta-learning-ledger
hepta-policy-runtime
hepta-evaluation
hepta-intelligence        # narrow façade
```

KG/retrieval/federation 的完整拆分可继续分阶段，但 learning ledger 不得继续依赖 god crate 私有布局。

---

## 1.12 仿生与自我进化声明需要独立门

### v3 决策

任何下列表述必须绑定 claim receipt：

```text
自我进化
长期学习
经验直觉
仿生神经元
结构生长
神经可塑性
类脑
神经形态
```

源码存在、shadow test、fixture 或 toy replay 均不足以支持上述产品级声明。

---

# 2. 不可破坏的系统不变量

## 2.1 Authority invariants

1. 唯一 authoritative mutation owner 仍为 Agent-local writer。
2. H5/H6/H7 永远不直接拥有 provider、tool、KG、memory、outbox 或 CALLERS authority。
3. 学习器只能提出 artifact 或 policy proposal。
4. production action 必须由既有 authority plane、TaskFlow 与 execution spine 执行。
5. 同一 run 使用一个冻结 `RunStartSnapshot`。
6. 当前 run 内禁止更换 policy/model/head/calibration/graph artifact。
7. next-snapshot reload 必须原子化；加载失败回退上一 approved artifact。
8. receipt、dataset、artifact、snapshot 或 policy digest 不匹配必须 fail closed。
9. 任何 `steps=[]`、`runner_id=0`、无 checkout 的 CI 结果既不是 PASS，也不是 code failure。
10. source gate PASS 不能替代 Rust executable qualification。
11. executable qualification 不能替代真实 efficacy。
12. efficacy 不能替代 operator acceptance。
13. operator acceptance 不能替代 CALLERS/promotion。
14. topology proposal 永远不能绕过 compiler、shadow、canary 和 rollback。
15. trust root、authority schema、安全下限和 release policy 不在 learnable surface 中。

## 2.2 Data invariants

1. `source_witness`、`fact_grounding`、`truth_status` 必须分离。
2. `recall_eligible`、`training_eligible`、`evaluation_eligible`、`promotion_eligible` 必须分离。
3. model-generated 和 external content 默认 `training_eligible=false`。
4. remembered instruction 默认只作为 data；只有 explicit user/workspace policy 可以提升 instruction authority。
5. correction、forget、revocation 必须沿 lineage 传播到 dataset snapshot 和 future artifact。
6. 已删除或撤销数据不得在新 dataset snapshot 中出现。
7. 旧 artifact 若包含已撤销数据，必须进入 `revocation_pending`，不得扩大 canary。
8. telemetry 默认不存 query/body/citation/secret/PII。
9. raw learning payload 必须有 retention、encryption、scope 和 export policy。
10. 隐私过滤失败时，learning admission fail closed，不阻塞主回答。

## 2.3 Learning invariants

1. 没有真实 action/outcome/feedback 时，只能称 observational/shadow。
2. 模型自评不能作为唯一 reward。
3. behavior propensity 必须来自实际 behavior policy，不能事后猜测。
4. unsupported action 不进入 OPE。
5. delayed outcome 必须显式链接 episode/action。
6. credit 总量必须守恒并可验证。
7. missing outcome、censoring 和 timeout 必须有明确 policy。
8. 每次训练使用 immutable dataset snapshot。
9. 训练和评测数据必须按 episode/workspace/time 分组防止 leakage。
10. 任何 artifact 晋升必须有 baseline、rollback artifact 与 kill switch。
11. 学习失败不得阻塞 baseline runtime。
12. 不确定性高、OOD 或证据不足时优先 abstain。

---

# 3. Capability、Learning 与 Bio-inspired Claim Ladder

## 3.1 Delivery capability ladder

每个 capability 仍使用六阶段状态：

```text
implemented
wired
qualified
efficacy_proven
operator_accepted
promoted
```

任一上游为 false 时，下游必须 false。

## 3.2 Longitudinal learning claim ladder

| 等级 | 允许声明 | 必须具备 | 当前 |
|---|---|---|---|
| `L0_STATIC_SHADOW` | 静态/影子策略信号 | typed input/output、digest、abstain、no authority | 已有部分 |
| `L1_OBSERVATIONAL_CONTINUAL` | 从 observation/correction/drift 离线更新检索或校准 artifact | durable observation、dataset lineage、retention/forgetting、reload/rollback | 未闭合 |
| `L2_CLOSED_LOOP_POLICY_LEARNING` | 真实授权 action → outcome → causal credit → next policy | behavior propensity、support、effect/postcondition、OPE/CI、canary | 未实现 |
| `L3_GOVERNED_STRUCTURAL_PLASTICITY` | proposal-only graph add/split/merge/retire/rewire | topology proposal、compiler、ablation、shadow、operator、rollback | 仅规划 |
| `L4_AUTONOMOUS_GOAL_AUTHORITY_CHANGE` | 不允许 | 系统不得自行修改目标、权限或 trust root | 永久禁止 |

当前总体声明：

```text
L0_BASELINE
L1_PARTIAL_SHADOW_FOUNDATIONS
L2=false
L3=false
L4=prohibited
```

## 3.3 H5 bio-inspired claim ladder

| 等级 | 允许术语 | 条件 |
|---|---|---|
| `N0_METAPHORICAL_TYPED_PROPOSAL` | neuron-inspired parameter proposal | 纯 typed proposal，无 temporal dynamics |
| `N1_ADAPTIVE_SIGNAL_UNIT` | adaptive signal unit | stateful signal、offline artifact、calibration、跨窗口 efficacy |
| `N2_TEMPORAL_RECURRENT_SIGNAL_NETWORK` | recurrent bio-inspired signal network | temporal state、recurrent/competitive dynamics、ablation、stability evidence |
| `N3_NEUROMORPHIC_RESEARCH` | experimental spiking/neuromorphic kernel | spike/event time、local plasticity、homeostasis、硬件/仿真 evidence |
| `N4_BIOLOGICAL_NEURON_REPLICA` | 不允许 | 软件系统不得宣称生物神经元复现 |

当前 H5：`N0_METAPHORICAL_TYPED_PROPOSAL`。

## 3.4 H6 intuition claim ladder

| 等级 | 允许术语 | 条件 |
|---|---|---|
| `I0_DETERMINISTIC_SELECTIVE_POLICY` | deterministic selector with abstention | hard gate/rank/tie-break |
| `I1_CALIBRATED_FAST_POLICY` | calibrated fast policy | candidate distribution、ECE/Brier、OOD、selective risk |
| `I2_EXPERIENCE_SHAPED_FAST_POLICY` | experience-shaped intuition-like policy | durable episode、cross-window learning、retention/no-regression |
| `I3_HUMAN_LIKE_INTUITION` | 不允许 | 不宣称人类直觉复现 |

当前 H6：`I0_DETERMINISTIC_SELECTIVE_POLICY`。

---

# 4. 目标架构

```text
┌──────────────────────────────── Authority / Governance Plane ───────────────────────────────┐
│ capability · scope · lease · generation · policy · budget · approval · CALLERS · rollback │
└──────────────────────────────────────────┬───────────────────────────────────────────────────┘
                                           │ immutable RunStartSnapshot
                                           ▼
┌──────────────────────────── Knowledge & Retrieval Plane ────────────────────────────┐
│ source witness → memory lifecycle → fact grounding → truth status → grounded KG    │
│ lexical / alias / ANN / KG / recency / contradiction / trust filters → reranker    │
└───────────────────────────────┬──────────────────────────────────────────────────────┘
                                │ bounded candidate set + evidence
                                ▼
┌──────────────────────────── Fast Decision Plane ─────────────────────────────────────┐
│ FeatureBuilder → H5 SignalKernel → H6 SelectivePolicy → DecisionReceipt             │
│          no authority, no mid-run mutation, explicit abstain/OOD/uncertainty         │
└───────────────────────────────┬──────────────────────────────────────────────────────┘
                                │ approved choice
                                ▼
┌──────────────────────────── TaskFlow / Execution Spine ──────────────────────────────┐
│ approved workflow → step → effect → postcondition → terminal/reconcile               │
└───────────────────────────────┬──────────────────────────────────────────────────────┘
                                │ immutable causal events
                                ▼
┌──────────────────────────── Learning Event Plane ─────────────────────────────────────┐
│ LearningEpisode → CandidateSet → Decision → Action → Outcome → Feedback → Credit      │
│ correction / forget / revocation / delayed outcome / privacy & eligibility lineage   │
└───────────────────────────────┬──────────────────────────────────────────────────────┘
                                │ immutable dataset snapshot
                                ▼
┌──────────────────────────── Slow Consolidation Plane ─────────────────────────────────┐
│ replay → train head/adapter/calibration/routing → OPE/CI → no-regression → artifact   │
└───────────────────────────────┬──────────────────────────────────────────────────────┘
                                │ signed next-snapshot artifact
                                ▼
┌──────────────────────────── Artifact / Promotion Plane ───────────────────────────────┐
│ registry → shadow → canary → operator acceptance → CALLERS → rollback/retire          │
└───────────────────────────────────────────────────────────────────────────────────────┘
```

## 4.1 Fast loop

```text
one RunStartSnapshot
→ deterministic feature construction
→ H5 signal
→ H6 selective decision
→ baseline/shadow/canary policy
→ approved TaskFlow
→ effect/postcondition
```

Fast loop不得：

```text
train
change graph
change authority
install artifact
rewrite memory truth
mutate policy during run
```

## 4.2 Slow loop

```text
immutable episodes
→ eligibility filter
→ dataset snapshot
→ train candidate
→ replay/OPE/CI
→ retention/forgetting
→ subgroup/safety gates
→ signed artifact
→ shadow
→ bounded canary
→ next snapshot
```

---

# 5. Canonical Contracts

## 5.1 `RunStartSnapshotV2`

必须绑定：

```text
snapshot_id
run_id / turn_id
agent_id / spawn_generation
authority_epoch / owner_epoch / generation / fencing token / lease expiry
definition / graph / policy / capability digests
memory/KG projection generation
model registry / model / head / calibration / tokenizer digests
artifact manifest digest
privacy profile
execution scope
resource budget
logical clock
rng seed
```

规则：

- run 开始后 immutable；
- 任一 digest 缺失或 schema mismatch fail closed；
- snapshot 只描述 authority，不授予 learning writer authority；
- new artifact 只能在下一个 snapshot 生效。

## 5.2 `LearningEpisodeV1`

顶层字段：

```text
episode_id
schema_version
namespace
snapshot_digest
agent_id
spawn_generation
workspace_scope_digest
turn_id
task_id
risk_class
opened_at_logical_clock
terminal_state
head_event_seq
head_event_digest
privacy_class
recall_eligible
training_eligible
evaluation_eligible
promotion_eligible
```

状态机：

```text
Opened
→ CandidateSetBound
→ DecisionBound
→ ActionObserved
→ OutcomePending
→ OutcomeObserved
→ CreditReady
→ Evaluated
→ Archived

异常：
Indeterminate
Quarantined
RevocationPending
Revoked
```

不变量：

- 一个 episode 只绑定一个 snapshot；
- decision/action/outcome 不得跨 spawn/generation；
- episode terminal 后 append 仅允许 delayed outcome、correction、forget 或 revocation lineage；
- changed replay 必须 conflict；
- episode id 不能由内容外推或复用。

## 5.3 `LearningEventV1`

事件种类：

```text
SnapshotStart
QueryPlan
CandidateSet
H5Signal
H6Decision
BaselineDecision
CanaryDecision
TaskFlowStart
ActionPrepared
EffectReceipt
Postcondition
ImmediateOutcome
DelayedOutcome
UserCorrection
ForgetRequest
Revocation
SafetyObservation
Terminal
```

每个事件绑定：

```text
episode_id
event_seq
event_id
event_kind
causal_parent_seq/digest
state_digest
policy_digest
candidate_set_digest
decision_digest
action_digest
effect/postcondition digest
behavior propensity
target propensity
support digest
outcome components
eligibility
privacy/trust metadata
event digest
```

## 5.4 `CandidateSetReceiptV1`

记录：

```text
candidate IDs/digests
channel scores
grounding/truth/trust status
risk/evidence readiness
availability mask
behavior-policy distribution
support set
retrieval/query planner/tokenizer/model digests
```

禁止只记录 selected candidate；否则无法做 counterfactual/OPE。

## 5.5 `PolicyDecisionReceiptV2`

输出必须包含：

```text
candidate probability distribution
selected candidate or abstain
confidence
calibration version
OOD/novelty score
risk veto reason
evidence veto reason
decision mode: baseline | shadow | canary
behavior propensity
policy digest
H5 signal digest
candidate set digest
snapshot digest
negative authority flags
```

## 5.6 `OutcomeReceiptV1`

reward 必须拆分为组件，禁止仅保存一个模型自评分：

```text
task_success
postcondition_correctness
user_correction
forget_requested
tool/provider verified result
safety violation
latency/cost
abstain appropriateness
delayed satisfaction
```

每个组件记录：

```text
source
confidence
observed_at
causal distance
missing/censored status
digest
```

## 5.7 `CreditLedgerV1`

要求：

- total credit conservation；
- hierarchical attribution：episode → decision → candidate/channel/neuron；
- delayed outcome 可追加但不可重写旧 credit；
- negative、zero、unknown credit 可区分；
- credit policy/version/digest 明确；
- correction/forget 可触发 future dataset exclusion，但不篡改历史 receipt。

## 5.8 `DatasetSnapshotV1`

必须包含：

```text
episode query/range
eligibility policy digest
privacy policy digest
included/excluded/quarantined counts
source episode/event digests
train/validation/test split digests
time/workspace grouping policy
revocation set digest
feature schema/tokenizer/model digests
created_by
expiry
```

禁止动态查询结果直接进入训练。

## 5.9 `EvaluationReceiptV2`

必须包含：

```text
baseline/candidate artifact digests
dataset snapshot digest
sample/supported/unsupported/censored counts
coverage
ESS
weight distribution and clipping
IPS/SNIPS/DR estimates
CI/LCB/UCB
subgroups
safety metrics
retention/forgetting windows
drift
negative controls
known limitations
promotion recommendation
```

## 5.10 `PolicyArtifactManifestV2`

artifact 初期只允许：

```text
retrieval weights
feature normalization
small head/adapter
temperature/calibration
abstain/risk threshold
model/router selection
```

禁止：

```text
authority
workflow effect topology
trust root
base safety invariant
provider credential policy
CALLERS
```

---

# 6. Data Eligibility、Truth、Privacy 与 Revocation

## 6.1 正交分类

每条 memory/episode/event 至少记录：

```text
semantic_type:
  fact | preference | instruction | observation | summary | external_content

instruction_authority:
  none | explicit_user | workspace_policy | system_prohibited

trust_zone:
  user | tool | repository | external | model_generated

truth_status:
  candidate | grounded | confirmed | disputed | contradicted | expired

eligibility:
  recall_eligible
  training_eligible
  evaluation_eligible
  promotion_eligible
```

## 6.2 默认策略

| 来源 | Recall | Train | Eval | Promote |
|---|---:|---:|---:|---:|
| explicit user correction | 条件允许 | 条件允许 | 条件允许 | 需支持集/隐私门 |
| verified tool postcondition | 允许 | 允许 | 允许 | 需统计门 |
| grounded repository fact | 允许 | 默认否 | 可用于 retrieval eval | 默认否 |
| external content | 条件允许 | 否 | 隔离评测 | 否 |
| model generated | 低信任 | 否 | 可作 proposal，不作 truth | 否 |
| summary/compact | 允许作上下文 | 否 | 否 | 否 |
| disputed/contradicted | 默认排除 | 否 | 可作负例 | 否 |
| secret/PII | 严格限制 | 否 | 否 | 否 |

## 6.3 Correction / Forget / Revocation

必须维护：

```text
source → memory revision → fact set → episode/event →
dataset snapshot → model artifact → canary assignment
```

规则：

- correction 生成新 revision，不覆盖历史；
- forget 生成 revocation lineage；
- 新 dataset snapshot 排除已撤销 lineage；
- artifact registry 标记受影响 artifact；
- 未扩大 canary 前必须评估污染程度；
- 高风险污染可直接 kill switch / rollback；
- 任何“删除后继续用于训练”必须可检测并作为 blocker。

---

# 7. H5 Neuron 深化计划

## 7.1 当前边界

当前 H5 仅作为 deterministic shadow proposal seam。v3 保留其安全边界，但停止把它描述为已实现的 neuron runtime。

## 7.2 目标接口

```rust
trait NeuronKernel {
    fn kernel_id(&self) -> KernelId;
    fn artifact_digest(&self) -> Sha256Digest;
    fn evaluate(
        &self,
        snapshot: &RunStartSnapshotV2,
        features: &FeatureVectorV2,
        state: Option<&NeuronTemporalStateV1>,
    ) -> Result<NeuronSignalReceiptV2, NeuronError>;
}
```

任何 kernel：

```text
no store write
no KG write
no workflow routing authority
no external effect
no mid-run artifact mutation
```

## 7.3 H5.0 — Claim and naming closure

交付：

- 将现有 `NeuronProposal` 明确标记 `N0`；
- receipt 增加 `bio_claim_level`；
- README/PR 禁止“仿生神经元已实现”；
- source verifier 检查 negative authority 与 claim boundary。

## 7.4 H5.1 — Deterministic Feature Builder

Feature schema 仅来自冻结 snapshot 与 bounded evidence：

```text
retrieval channel scores
grounding/truth/trust
freshness
contradiction
candidate diversity
recent success/failure
latency/budget
workspace/task/risk class
```

要求：

- canonical feature order；
- missingness mask；
- feature provenance；
- normalization artifact digest；
- no raw secret/PII；
- deterministic fallback；
- adversarial range/overflow tests。

## 7.5 H5.2 — Linear/Tree Signal Kernel

第一条真实 kernel 使用可解释、低风险模型：

```text
regularized linear scorer
small tree/ranker
temperature/calibration head
```

禁止第一版使用生成模型决定 signal。

交付：

- candidate vector → typed signal；
- per-feature attribution；
- uncertainty/calibration；
- signed artifact；
- baseline shadow compare；
- latency/resource receipt。

## 7.6 H5.3 — Temporal State Kernel

只有 H5.2 通过后才允许：

```text
bounded exponential decay
recent failure state
short recurrent state
competition / inhibition proxy
homeostatic activation budget
```

状态必须：

- Agent-local；
- snapshot/generation-bound；
- bounded size/TTL；
- crash/restart 可重建；
- 不写长期 truth；
- 不跨 workspace 泄漏。

## 7.7 H5.4 — Offline Update Artifact

允许更新：

```text
weights
head/adapter
normalization
temperature
threshold
```

禁止更新：

```text
position registry
authority
graph topology
effect rules
base safety floor
```

## 7.8 H5.5 — Calibration、Ablation 与 Lesion

必须评测：

- signal calibration；
- feature ablation；
- lesion 某 neuron/kernel 后的影响；
- dominant feature 检查；
- shortcut/leakage 检查；
- 7d/30d retention；
- cross-language/workspace subgroup；
- OOD abstain。

## 7.9 H5.6 — Optional Neuromorphic Research Track

独立、非生产 track 可研究：

```text
event/spike representation
LIF-style temporal integration
STDP-like local update
homeostatic scaling
sparse competition
```

要求：

- 与 H5 production artifact registry 完全隔离；
- 不共享 authority；
- 必须明确“research approximation”；
- 没有独立 efficacy/energy/hardware evidence时不得提升 claim。

---

# 8. H6 Intuition 深化计划

## 8.1 当前边界

当前 H6 是 `I0` deterministic selective policy。它保留 hard veto 和 abstain，但不再被描述成已形成经验直觉。

## 8.2 输入

H6 V2 输入：

```text
RunStartSnapshot
CandidateSetReceipt
H5 SignalReceipt[]
risk/evidence policy
recent failure digest
OOD reference artifact
calibration artifact
mode
```

## 8.3 输出

`PolicyDecisionReceiptV2` 输出：

```text
probability distribution
selected candidate / abstain
confidence
risk veto
evidence veto
OOD score
calibration digest
behavior propensity
reason attribution
negative authority flags
```

## 8.4 H6.0 — Receipt V2 and recomputation

- receipt 必须可由 immutable inputs 完全重算；
- self-consistent but wrong decision 必须被拒绝；
- candidate set、H5 signal、policy、calibration、snapshot 全绑定；
- changed candidate order 不改变 canonical result；
- stale/cross-policy replay fail closed。

## 8.5 H6.1 — Distributional Selective Policy

不只输出 top-1 score，而是输出完整候选概率和 abstain mass。

第一阶段可采用：

```text
softmax / Platt / isotonic / calibrated tree
```

必须保留 hard veto 在 rank 之前执行。

## 8.6 H6.2 — Uncertainty and OOD

最少包含：

```text
epistemic proxy
aleatoric/score margin
candidate coverage
feature missingness
distance-to-training-support
novelty/OOD
```

OOD 高时必须提高 abstain，不允许以高 confidence 掩盖无支持区域。

## 8.7 H6.3 — Temporal Context

只允许使用 bounded、可解释的近期上下文：

```text
recent failure count
recent correction
provider indeterminate rate
retrieval instability
budget pressure
```

不得把整段未经治理的 memory 直接作为 hidden policy prompt。

## 8.8 H6.4 — Calibration and Selective Risk Gates

指标：

```text
ECE
Brier score
negative log likelihood
coverage
selective risk
abstain precision/recall
OOD AUROC/AUPRC
decision regret
safety veto false-negative rate
```

必须按 language、workspace、task、risk、backend 分组。

## 8.9 H6.5 — Shadow and Canary

阶段：

```text
SuggestOnly
→ ShadowCompare
→ PrepareOnly
→ LowRiskCanarySelect
```

即使 `LowRiskCanarySelect`，effect authority 仍由 TaskFlow/authority plane 持有。

## 8.10 H6.6 — Claim promotion

只有满足以下条件才允许 `I2_EXPERIENCE_SHAPED_FAST_POLICY`：

- durable closed-loop episodes；
- next-snapshot artifact update；
- 7d/30d retention；
- no critical subgroup regression；
- calibrated abstain；
- operator-reviewed rollback。

---

# 9. H7 Learning 深化计划

## 9.1 统一目标

```text
durable causal episode
+ policy action/propensity/support
+ outcome/feedback
+ credit
+ dataset snapshot
+ OPE/CI
+ artifact
+ reload/rollback
```

必须成为一条 schema 和一套 ledger，而不是 durable lifecycle 与 pure feedback oracle 两条平行真相。

## 9.2 H7.0 — Contract reconciliation

交付：

- `LearningEventV1` canonical wire schema；
- existing trajectory → canonical migration adapter；
- existing feedback → canonical migration adapter；
- collision/cross-scope/stale fence tests；
- deprecated dual-write 状态清单；
- no new production caller。

## 9.3 H7.1 — `hepta-learning-ledger`

独立 SQLite migration 和 crate：

```text
learning_episodes
learning_events
learning_outcomes
learning_credit_entries
learning_revocations
learning_dataset_membership
```

不变量：

- append-only；
- episode/event hash chain；
- exact replay；
- changed replay conflict；
- one snapshot/spawn/generation；
- causal parent contiguous；
- terminal + delayed outcome 规则；
- no external effect authority；
- read-only verifier；
- corruption injection；
- WAL/crash/reopen qualification。

## 9.4 H7.2 — Policy Action、Propensity 与 Support

对于可评估 action：

```text
behavior_propensity > 0
target_propensity >= 0
selected action in behavior support
candidate set digest exact
availability mask exact
```

禁止：

- 根据 action 结果反推 propensity；
- 将 deterministic top-1 默认写成 1.0，除非 behavior policy 真正确定；
- unsupported episode 进入 IPS/DR；
- 丢失未选择 candidates。

## 9.5 H7.3 — Delayed Outcome

支持：

```text
immediate postcondition
same-turn correction
next-turn correction
task completion
7d delayed satisfaction
forget/revocation
```

每个 delayed outcome 绑定原 action/episode，记录 observation delay、censoring 和 confidence。

## 9.6 H7.4 — Credit Assignment

第一阶段采用保守、可解释策略：

```text
terminal reward decomposition
bounded temporal decay
hierarchical conservation
no model-only reward
```

第二阶段才允许 learned credit model，并且 learned credit 只能作为候选，与 deterministic baseline 并行。

## 9.7 H7.5 — OPE

最少实现：

```text
IPS
self-normalized IPS
doubly robust
weight clipping diagnostics
support coverage
negative control
```

禁止只报告 offline loss。

## 9.8 H7.6 — ESS、CI 与 Sequential Policy

必须计算：

```text
ESS = (sum w)^2 / sum(w^2)
clustered/bootstrap CI
candidate LCB
baseline UCB
practical improvement
```

样本聚类至少按：

```text
episode
workspace
time window
```

禁止按 event 行独立 bootstrap。

连续观察 dashboard 时必须使用预注册 sequential policy，防止反复窥视造成假阳性。

## 9.9 H7.7 — Retention、Drift 与 Forgetting

窗口：

```text
1d smoke
7d short retention
30d medium retention
90d long retention
```

评测：

- old-task retention；
- new-task gain；
- correction adoption；
- forget compliance；
- truth/grounding pollution；
- calibration drift；
- subgroup drift；
- artifact age decay。

## 9.10 H7.8 — Artifact Registry

状态：

```text
Draft
→ Trained
→ OfflineEvaluated
→ ShadowApproved
→ CanaryApproved
→ OperatorAccepted
→ Promoted
→ RolledBack / Retired / RevocationPending
```

artifact 与 approval/trust anchor 分离；当前 run 不允许 mid-flight replacement。

---

# 10. 双速长期学习与抗遗忘

## 10.1 Fast Episodic Store

职责：

```text
capture exact experience
preserve correction/forget
bind action/outcome
support audit/replay
```

不负责：

```text
修改 policy
生成 truth
直接训练
获得 authority
```

## 10.2 Slow Consolidation

流程：

```text
episode eligibility
→ immutable dataset snapshot
→ train candidate
→ replay
→ anti-forgetting
→ OPE/CI
→ retention
→ signed artifact
```

## 10.3 Anti-forgetting 初始策略

按风险从简单到复杂：

1. 固定旧任务 replay set；
2. stratified rehearsal；
3. regularization against previous artifact；
4. protected feature/head subset；
5. distillation from approved baseline；
6. only after evidence: adapter isolation / modular routing。

第一阶段不更新 base LLM 权重，只更新小 head、adapter、calibration、threshold 和 routing。

## 10.4 Consolidation Schedule

```text
event append: near-real-time
dataset freeze: daily/weekly
candidate train: scheduled
shadow eval: per artifact
canary: bounded window
long retention: 7d/30d/90d
```

低样本 workspace 不单独训练，使用 shared artifact + local calibration，避免过拟合和隐私泄漏。

---

# 11. Revised Roadmap and Dependency DAG

## 11.1 Phase Q0 — Qualification Debt Closure

### Q0.1 Freeze exact stack

冻结：

```text
P0.1 PR #7
P0.2 PR #13
P0.3 PR #14
P0.4a PR #16
P0.4b PR #21
P0.4c PR #23
```

记录 exact heads、trees、toolchain、migration、workflow 和 blocker receipts。

### Q0.2 Evidence classes

| Class | 证据 | 可支持声明 |
|---|---|---|
| `E0_SOURCE` | schema/static/source gate | implemented/source-present |
| `E1_LOCAL_EXECUTABLE` | exact-head fmt/test/clippy/SQLite in reproducible env | developer executable confidence |
| `E2_INDEPENDENT_RUNNER` | independent runner exact-head artifacts | qualified candidate |
| `E3_RUNTIME` | loopback/real runtime/restart/failpoint | wired/runtime behavior |
| `E4_EFFICACY` | corpus/online controlled experiment | efficacy_proven |
| `E5_GOVERNANCE` | operator/rollback/CALLERS | accepted/promoted |

P0 关闭至少需要 E1+E2；production wiring 还需 E3，P2 promotion 还需 E4+E5。

### Q0.3 Stack budget gate

在 Q0 完成前：

```text
new runtime code = blocked
new migration = blocked
new H5/H6/H7 source tranche = blocked
plan/verifier/runner repair = allowed
```

### Q0 Exit Gate

- P0.1–P0.4c exact-head executable evidence；
- fmt/test/clippy/source gates；
- migration/reopen/corruption/failpoint；
- Agentd feature remains default-off；
- candidate freeze；
- no production authority。

---

## 11.2 Phase A0 — Canonical Architecture Authority

交付：

```text
CURRENT_PLAN
EXECUTION_STATUS_V3
CLAIM_LADDER
CAPABILITY_REGISTRY
EVIDENCE_REGISTRY
PR_STACK
```

Exit：

- 一个命令可得 current truth；
- historical plan 不再被误当 current；
- status 与 PR/branch/head 自动核对；
- authority flags machine-enforced false。

---

## 11.3 Phase B0 — Early Boundary Extraction

在 P2 前最少拆出：

```text
hepta-learning-contracts
hepta-learning-ledger
hepta-policy-runtime
hepta-evaluation
```

迁移原则：

- 先复制 contract + compatibility adapter；
- 双读验证；
- no circular dependency；
- public API compatibility receipt；
- god crate façade 只保留 bounded delegation；
- 每步可回滚。

---

## 11.4 Phase C0 — LearningEpisode and Eligibility

### C0.1 Contract

`LearningEpisodeV1`、`LearningEventV1`、eligibility/trust/privacy contracts。

### C0.2 Durable Ledger

SQLite append-only、reopen、hash chain、failpoint、revocation。

### C0.3 Agentd Shadow Host

feature-default-off host 只写 learning ledger；不得写 memory/KG/outbox。

### C0 Exit

- one complete synthetic episode durable/reopen/replay；
- changed replay conflict；
- correction/forget propagation；
- policy action remains unqualified；
- no external effects。

---

## 11.5 Phase R1 — Grounded Retrieval and Telemetry

保留 V2 P1.1–P1.4，但增加依赖：

```text
P0 exact qualification
A0 current authority
B0 contract boundary
C0 episode/eligibility
```

### R1.1 Hybrid Retrieval

lexical + alias + ANN + bounded KG + calibrated reranker。

### R1.2 Telemetry

no-content funnel、reason taxonomy、latency/quality、minimum aggregation。

### R1.3 Federation

bounded concurrency、snapshot merge receipt、partial semantics、physical-send revalidation。

### R1.4 Semantic Security

truth/trust/instruction/eligibility + DLP。

### R1.5 Retrieval Efficacy Dataset

多语言、同义改写、冲突、时效、forget、workspace/federation。

---

## 11.6 Phase N1 — H5 Adaptive Signal Layer

顺序：

```text
N1.0 claim closure
N1.1 feature builder
N1.2 linear/tree signal kernel
N1.3 temporal state
N1.4 artifact update
N1.5 calibration/ablation
```

Exit：

- `N1_ADAPTIVE_SIGNAL_UNIT` 最多；
- latency/resource gate；
- no authority；
- 7d retention；
- no critical subgroup regression。

---

## 11.7 Phase I1 — H6 Calibrated Fast Policy

顺序：

```text
I1.0 DecisionReceiptV2
I1.1 probability distribution
I1.2 abstain/OOD
I1.3 temporal context
I1.4 calibration/selective risk
I1.5 shadow compare
```

Exit：

- `I1_CALIBRATED_FAST_POLICY`；
- baseline remains product decision；
- no external effect；
- canary not yet allowed。

---

## 11.8 Phase L1 — H7 Evaluation and Artifact

顺序：

```text
L1.0 feedback/trajectory reconciliation
L1.1 durable policy action/outcome
L1.2 credit
L1.3 OPE
L1.4 ESS/CI
L1.5 retention/forgetting
L1.6 signed artifact
```

Exit：

- closed offline causal chain；
- no production canary；
- artifact reload only in local qualification snapshot；
- rollback verified。

---

## 11.9 Phase C1 — Low-risk Closed Loop

第一 position：

```text
MemoryRetrievalRank
```

第二候选：

```text
ContextSalience
```

禁止首个闭环：

```text
tool execution
provider mutation
credential
auth
workflow topology
high-risk branch
```

阶段：

```text
baseline + shadow
→ local controlled choice
→ 1% low-risk canary
→ bounded workspace canary
→ operator review
```

晋升条件：

```text
candidate LCB > baseline UCB + practical improvement
ESS >= configured floor
coverage >= floor
clipped ratio <= ceiling
critical safety pass
no subgroup regression
7d retention pass
rollback rehearsal pass
```

---

## 11.10 Phase S1 — Governed Structural Plasticity

仅 proposal-only：

```text
add/split/merge/retire neuron
rewire signal dependency
add retrieval feature/channel
change routing module
```

每个 proposal 必须经过：

```text
GraphCompiler
schema/invariant
cycle/authority check
ablation/lesion
support-aware replay
shadow
canary
operator
rollback
```

运行中 graph 永不改变；新 graph 仅进入 next snapshot。

---

# 12. PR and Commit Sequence

## Plan Tranche — 本 PR

1. `docs(plan): add Hepta Intelligence development plan v3`
2. `docs(status): add canonical current-plan and claim ladder`
3. `ci(plan): verify v3 plan and authority boundaries`
4. append-only plan review receipt

该 tranche 只修改 plan/status/verifier/workflow，不修改 runtime、migration、CALLERS 或 production caller。

## Qualification Tranche

1. `ci(intelligence): freeze P0 exact stack manifest`
2. `test(intelligence): add reproducible local exact-head qualification runner`
3. `test(memory): execute grounding/mutation SQLite failpoint matrix`
4. `test(agentd): execute default-off P0.4c host matrix`
5. append-only executable receipts

## Boundary Tranche

1. `refactor(intelligence): add hepta-learning-contracts`
2. `refactor(intelligence): add hepta-learning-ledger`
3. `refactor(intelligence): add hepta-policy-runtime`
4. `refactor(intelligence): add hepta-evaluation`
5. compatibility and no-cycle receipts

## Causal Ledger Tranche

1. `feat(learning): add LearningEpisodeV1`
2. `feat(learning): add append-only event chain`
3. `feat(learning): add outcome/correction/forget/revocation`
4. `test(learning): add crash/reopen/replay/corruption matrix`
5. no-authority Agentd shadow host

## Retrieval Tranche

1. tokenizer/query plan contracts；
2. channel registry；
3. local embedding adapter + deterministic fallback；
4. candidate set receipt；
5. telemetry；
6. efficacy dataset and receipt。

## H5/H6/H7 Tranches

每个子阶段独立 PR，且不得超过 unqualified stack budget。

---

# 13. Qualification and Test Matrix

## 13.1 Source / Contract

- JSON/Schema parse；
- canonical serialization；
- unknown field rejection；
- negative authority flags；
- current-plan pointer；
- plan/status/claim consistency；
- no forbidden product claims。

## 13.2 Rust

```text
cargo fmt
focused unit tests
full crate tests
strict clippy
feature-default-off checks
dependency graph/no-cycle
```

## 13.3 SQLite

- migration checksum；
- 0001..current upgrade；
- BEGIN IMMEDIATE boundaries；
- pre/post commit failpoints；
- changed replay；
- corruption injection；
- reopen verification；
- WAL checkpoint/restart；
- bounded growth；
- revocation lineage。

## 13.4 Property / Model

- duplicate/reorder；
- arbitrary UTF-8；
- hash collision assumptions not bypassed；
- sequence overflow；
- stale generation；
- cross-agent/spawn/workspace；
- terminal/delayed outcome；
- credit conservation；
- propensity range/support；
- OPE fixed-point overflow。

## 13.5 Security / Privacy

- prompt injection memory；
- external/model-generated training admission；
- instruction authority escalation；
- scope escape；
- stale capability；
- secret/PII/high-entropy token；
- receipt forgery；
- dataset leakage；
- correction/forget bypass；
- artifact revocation。

## 13.6 Efficacy

Retrieval：

```text
Recall@4
nDCG@4
citation precision
false-memory attachment
stale/contradicted attachment
task success
token cost
p95/p99 latency
```

H5/H6：

```text
ECE
Brier
selective risk
OOD AUROC/AUPRC
abstain quality
regret
feature ablation
```

H7：

```text
coverage
ESS
weight clipping
IPS/SNIPS/DR
CI/LCB/UCB
subgroup
retention/forgetting
drift
```

## 13.7 Initial Pilot Threshold Registry

阈值必须预注册并可按 risk class 配置。初始低风险 pilot 建议默认：

```text
minimum episodes >= 5,000
supported episodes >= 2,000
ESS >= 500
support coverage >= 80%
clipped ratio <= 5%
candidate LCB improvement >= 2% absolute or pre-registered practical threshold
critical safety regression = 0 tolerated
false-memory attachment not worse than baseline
p95 decision overhead <= 20% of baseline or bounded absolute budget
7d retention pass before canary expansion
30d retention pass before broad promotion
```

这些是 pilot 默认，不是自动 production authority；任何修改必须在 experiment registry 中预注册。

---

# 14. Rollback and Kill-switch Contract

每个 artifact/canary 必须预生成：

```text
previous approved artifact
rollback artifact
rollback command/transaction
scope
expiry
kill switch
expected recovery time
data/schema compatibility
```

触发条件：

```text
digest/signature mismatch
OOD/abstain collapse
safety floor failure
critical subgroup regression
calibration drift
false-memory increase
revocation contamination
latency/resource ceiling
operator stop
```

rollback 必须：

- 幂等；
- 不依赖 candidate artifact；
- 可在 provider/network 不可用时本地执行；
- 生成 receipt；
- 下一 snapshot 验证已回退；
- 保留历史，不重写 evidence。

---

# 15. Observability

## 15.1 No-content metrics

```text
episode opened/completed/indeterminate/quarantined
candidate set size
abstain/veto reason
support/coverage
outcome missing/censored
correction/forget/revocation
dataset include/exclude
artifact shadow/canary/rollback
latency/resource buckets
```

## 15.2 Required dashboards

1. Capability truth dashboard；
2. qualification/evidence dashboard；
3. learning data health；
4. support/propensity/OPE；
5. calibration/OOD/abstain；
6. retention/forgetting；
7. privacy/revocation；
8. artifact/canary/rollback。

禁止 dashboard 直接展示 raw memory/query/secret。

---

# 16. Risk Register

| ID | 风险 | 严重度 | 主要缓解 |
|---|---|---:|---|
| R1 | source receipt 被误当 qualification | Critical | evidence class + machine gate |
| R2 | 未 qualification 栈继续增长 | High | stack budget / freeze |
| R3 | grounding 被误当 truth | Critical | three-layer truth model |
| R4 | recalled external content 进入训练 | Critical | four eligibility axes |
| R5 | H7 durable/feedback 双真相 | Critical | canonical LearningEvent |
| R6 | propensity 事后猜测 | Critical | behavior policy receipt |
| R7 | support 不足仍做 OPE | High | hard support gate |
| R8 | point estimate 假阳性 | High | ESS/CI/sequential policy |
| R9 | delayed outcome 错归因 | High | explicit causal link/censoring |
| R10 | correction/forget 未传播 | Critical | lineage/revocation |
| R11 | god crate 耦合扩大 | High | early boundary extraction |
| R12 | Neuron/Intuition 过度宣传 | High | claim ladder |
| R13 | catastrophic forgetting | High | replay/retention/no-regression |
| R14 | telemetry 泄露 | Critical | no-content schema/aggregation |
| R15 | artifact reload mid-run | Critical | next-snapshot-only |
| R16 | topology proposal 越权 | Critical | compiler/operator/rollback |
| R17 | low-sample workspace overfit | High | shared artifact/local calibration |
| R18 | runner blocker长期掩盖 source defect | High | local executable E1 + independent E2 |

---

# 17. Current Exact Status

基于 PR #23 head：

```text
base branch:
  codex/hepta-intelligence-shadow-host-adapter-v4c-20260828

base head:
  7691978b786dd00c69477d1a3355be13db2c4d67

P0.4c hardened source candidate:
  7bb26ec016c2e2c83084756485ea324e79bcddbe

hosted workflow:
  33144320922

source job:
  steps=[]
  runner_id=0

Rust job:
  steps=[]
  runner_id=0
```

当前结论：

```text
P0.1–P0.4c source implemented
P0.1–P0.4c wired=false
P0.1–P0.4c qualified=false
P1.1 activation=blocked
H5 runtime=false
H6 runtime=false
H7 closed_loop=false
self_evolution=false
longitudinal_learning_efficacy=false
bio_mechanism=false
production_authority=false
```

---

# 18. Immediate Next Actions

严格顺序：

1. 合入或评审本 plan-only Draft，不改变 PR #23 runtime posture。
2. 冻结 P0 stack exact heads 与 tree manifest。
3. 建立可复现本地 executable qualification 环境，产出 E1 receipt。
4. 恢复独立 runner，产出 E2 receipt。
5. 对 P0.1–P0.4c 执行 source/fmt/test/clippy/SQLite/reopen/failpoint/Agentd default-off gates。
6. 不通过则修复原 tranche，不继续 P1 stack。
7. 通过后建立 A0 current capability/evidence registry。
8. 提前拆出 learning contracts/ledger/policy/evaluation 边界。
9. 实现 LearningEpisode/eligibility/revocation。
10. 再启动 grounded hybrid retrieval 和 telemetry。
11. 完成 P1 efficacy 后，启动 H5 N1。
12. H5 通过后启动 H6 I1。
13. H6 通过后统一 H7 durable learning ledger/OPE。
14. 只有完整 L2 evidence 才允许 low-risk canary。
15. 结构可塑性保持 proposal-only 独立 track。

---

# 19. Definition of Done

## 19.1 Plan v3 DoD

- 本计划绑定 exact base head；
- current-plan pointer、status、claim ladder 一致；
- verifier 可执行且 fail closed；
- 所有 authority flags false；
- P0 runner blocker 被准确记录；
- P1/P2 runtime activation false；
- plan 不声称已实现自我进化、长期学习 efficacy 或仿生机制。

## 19.2 P0 Qualification DoD

- exact frozen heads；
- non-empty executable steps；
- Rust 1.95.0 exact；
- fmt/test/clippy；
- migration/reopen/corruption/failpoint；
- source/memory/KG zero-write negative tests；
- Agentd feature default-off；
- readable artifacts；
- qualified receipt；
- operator/promotion false。

## 19.3 L2 Closed-loop DoD

- LearningEpisode durable；
- actual behavior propensity/support；
- effect/postcondition/outcome；
- correction/forget/revocation；
- credit conservation；
- immutable dataset snapshot；
- OPE/ESS/CI；
- subgroup/no-regression；
- 7d/30d retention；
- signed next-snapshot artifact；
- shadow + bounded canary；
- rollback rehearsal；
- operator acceptance；
- separate CALLERS promotion。

---

# 20. Final Decision

Hepta 的最佳方向不是模拟完整生物神经系统，而是建设：

> **一个事实可证、因果可回放、策略可校准、学习可统计验证、artifact 可治理晋升、失败可回滚的长期智能系统。**

H5 和 H6 可以逐步具备功能层面的 bio-inspired 特征，但必须严格区分：

```text
metaphor
adaptive signal
calibrated fast policy
closed-loop learning
structural plasticity
neuromorphic research
```

当前计划的执行决策是：

```text
STOP unqualified implementation stacking
CLOSE P0 executable qualification debt
ESTABLISH canonical plan/capability truth
SEPARATE learning ledger from memory
BUILD LearningEpisode before H5/H6/H7 activation
PROVE retrieval and data quality before policy learning
PROMOTE only through CI/ESS/subgroup/retention/rollback/operator/CALLERS
```
