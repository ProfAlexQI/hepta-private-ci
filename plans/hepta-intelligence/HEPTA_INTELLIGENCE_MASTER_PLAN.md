# Hepta Intelligence 统一主开发计划

## Governed Continual Learning, Adaptive Signals, Calibrated Intuition and Bio-inspired Mechanisms

**文档版本**：4.0.0  
**日期**：2026-08-28（Asia/Toronto）  
**状态**：`CANONICAL_CURRENT / PLAN_ONLY / FAIL_CLOSED`  
**仓库**：`ProfAlexQI/hepta-private-ci`  
**稳定路径**：`plans/hepta-intelligence/HEPTA_INTELLIGENCE_MASTER_PLAN.md`  
**机器入口**：`plans/hepta-intelligence/HEPTA_INTELLIGENCE_CURRENT_PLAN.json`

> 本文件是 Hepta Intelligence 唯一有效的人类可读开发计划。任何带日期或版本号的旧计划、tranche 文档、PR body、receipt、Dropbox snapshot、qualification index、实现说明或聊天结论均只保留为历史证据，不得作为当前实施顺序、能力状态或权限依据。所有 session 必须先读取机器入口，再读取本文件；二者不一致时立即 fail closed。

---

# 0. 文档权威与 Session 协议

## 0.1 唯一权威规则

当前开发事实只由以下两层表达：

1. `HEPTA_INTELLIGENCE_CURRENT_PLAN.json`：机器可读的当前版本、精确基线、阶段、claim、阻断与权限；
2. `HEPTA_INTELLIGENCE_MASTER_PLAN.md`：唯一的人类可读架构、路线图、合同、测试与 DoD。

历史文件可以被引用以追溯来源，但不得覆盖本文件。以后禁止再创建新的 `HEPTA_INTELLIGENCE_DEVELOPMENT_PLAN_V*.md`、日期版总计划或平行 roadmap。计划变更必须直接更新本稳定路径，并在同一 PR 中：

```text
更新 master plan
→ 更新 CURRENT_PLAN.json
→ 运行唯一 verifier
→ 生成 append-only plan receipt
→ 更新 PR body
```

## 0.2 每个 Session 的启动顺序

每个开发、审计、评审或研究 session 必须：

1. 读取 `HEPTA_INTELLIGENCE_CURRENT_PLAN.json`；
2. 读取本文件；
3. 核对 repository、branch、base/head、active phase、blocked phases；
4. 核对 `implemented/wired/qualified/efficacy_proven/operator_accepted/promoted` 六阶段状态；
5. 只执行 `next_actions` 中未被阻断的最前一项；
6. 不从旧 PR、旧 plan、源码命名或 source-only receipt 推导更高能力；
7. 结束时更新精确 head、验证结果、阻断原因和下一步，不创建第二份计划。

## 0.3 Claim 纪律

以下词语必须有对应 claim receipt 才可使用：

```text
自我进化
长期学习
闭环学习
经验直觉
仿生神经元
神经可塑性
结构生长
类脑
神经形态
自主进化
```

源码、schema、shadow、fixture、toy replay、loss 下降、source gate 或空 runner 均不足以支持这些声明。

---

# 1. 当前精确事实与最终判断

## 1.1 精确开发基线

当前计划继承的冻结实现基线：

```text
base PR: #23
base branch: codex/hepta-intelligence-shadow-host-adapter-v4c-20260828
base head: 7691978b786dd00c69477d1a3355be13db2c4d67
base tree: bc2342443fe28d2b803cf1c8273c5d3cd4171ced
P0.4c hardened source candidate: 7bb26ec016c2e2c83084756485ea324e79bcddbe
```

当前 P0.1–P0.4c 具备的主要 source surface：

```text
P0.1 source-span fact grounding
P0.2 durable fact-grounding ledger
P0.3 grounded tool/projection shadow gate
P0.4a typed intelligence mutation state machine
P0.4b SQLite transition journal and failpoints
P0.4c shadow host orchestration adapter
```

真实状态仍为：

```text
implemented=true
wired=false
qualified=false
efficacy_proven=false
operator_accepted=false
promoted=false
```

已观察到的 hosted jobs 为 `steps=[]`、`runner_id=0`，没有 checkout、fmt、test、clippy、SQLite、failpoint 或 runtime 命令执行。它们既不是 PASS，也不是代码失败。

## 1.2 当前能力结论

```text
较强：治理、权限边界、digest、receipt、CAS、SQLite、memory/KG/provenance 基础
部分：长期记忆、compaction、trajectory、artifact/rollback contract
未闭合：统一 LearningEpisode、durable causal learning ledger、真实 outcome/credit、trainer、OPE/CI、跨窗口 efficacy
H5：N0_METAPHORICAL_TYPED_PROPOSAL
H6：I0_DETERMINISTIC_SELECTIVE_POLICY
系统：L0_STATIC_SHADOW_WITH_PARTIAL_L1_FOUNDATIONS
```

因此当前必须保持：

```text
self_evolution=false
longitudinal_learning_efficacy=false
closed_loop_learning=false
structural_plasticity=false
neuromorphic_mechanism=false
biological_mechanism_replication=false
```

## 1.3 本计划允许的“自我改进”定义

允许目标是：

> **受治理、跨快照、可回放、可统计验证、可回滚的参数、校准、检索、路由和策略适应。**

永远禁止学习器直接修改：

```text
system goal
trust root
authority/capability
CALLERS
production writer ownership
safety invariant
workflow effect boundary
credential/provider policy
release policy
```

---

# 2. 必须修复的架构断点

## 2.1 治理闭环不等于学习闭环

当前系统可以证明“谁提出了什么、绑定什么 snapshot、是否越权、是否持久化、receipt 是否可重算”，但还不能证明：

```text
从哪条经验学到了什么
哪个行为导致了哪个结果
参数为何改变
改变后是否长期改善
旧能力是否遗忘
correction/forget 是否进入后续 dataset/artifact
```

真实学习闭环必须是：

```text
state/candidate set
→ behavior policy and propensity
→ selected action
→ authorized execution/postcondition
→ immediate or delayed outcome
→ causal credit
→ immutable dataset snapshot
→ train candidate
→ independent evaluation
→ signed artifact
→ next-snapshot reload
→ retained improvement
```

## 2.2 长期存储不等于长期学习

Memory/KG 可以保存、验证、纠正、tombstone 和 recall，但这不自动改变 policy。长期学习必须增加独立的 learning eligibility、dataset lineage、anti-forgetting、evaluation 和 artifact promotion。

## 2.3 H7 双真相必须合并

现有 durable trajectory 偏 lifecycle，纯 feedback/OPE oracle 偏内存 replay。二者不得继续平行演化。必须迁移到唯一 `LearningEventV1` 和独立 `hepta-learning-ledger`，形成同一条 durable causal chain。

## 2.4 当前 OPE 只够 exploratory

任何把 supported sample 直接等同 sample、把 coverage 固定为 100%、只报告点估计或未计算 ESS/CI 的实现都不能用于 promotion。必须实现真实 support coverage、weight diagnostics、IPS/SNIPS/DR、clustered CI、LCB/UCB 和 subgroup gates。

## 2.5 确定性策略缺乏 counterfactual support

如果 behavior policy 永远 deterministic top-1，其他候选没有真实 propensity/support，OPE 无法识别替代动作。必须增加受治理的 `ExplorationPolicyReceiptV1`，且初期只允许无外部副作用的低风险 position。

## 2.6 H5/H6 当前不是本地小模型 runtime

当前 H5 是整数 feature aggregate 与 bounded delta proposal；当前 H6 是 hard filter、rank、tie-break 和 abstain。二者均不得声称已经调用本地小模型或形成经验直觉。

## 2.7 `codex-hepta-memory` god crate 必须提前拆边界

Learning ledger、policy runtime、evaluation 和 artifact 不得继续依赖 memory crate 私有布局。完整拆分可以分阶段，但 learning 边界必须在新增闭环 source 前完成。

## 2.8 未 qualification 栈必须冻结

最后一个 executable-qualified base 之后最多允许两层 source implementation Draft。当前栈已超过预算，因此 Q0 关闭前禁止：

```text
新 runtime source
新 migration
P1.1 activation
H5/H6/H7 runtime tranche
production caller
CALLERS ratchet
```

允许：plan、schema、verifier、receipt、runner infrastructure 和原 tranche 修复。

---

# 3. 不可破坏的不变量

## 3.1 Authority 不变量

1. Agent-local writer 是唯一 authoritative mutation owner。
2. H5/H6/H7 不拥有 provider、tool、memory、KG、outbox、credential 或 CALLERS authority。
3. 同一 run 只使用一个 immutable `RunStartSnapshotV2`。
4. 当前 run 内不得更换 policy/model/head/calibration/graph artifact。
5. 新 artifact 只能在 next snapshot 原子加载；失败回退上一 approved artifact。
6. TaskFlow/authority plane 执行动作；policy 只选择或 abstain。
7. topology 只能 proposal-only，经 compiler、shadow、canary、operator、rollback。
8. receipt、snapshot、dataset、artifact 或 policy digest 不匹配必须 fail closed。
9. source PASS、executable qualification、efficacy、operator acceptance、promotion 互不替代。

## 3.2 数据不变量

```text
source_witness != fact_grounding != truth_status
recall_eligible != training_eligible != evaluation_eligible != promotion_eligible
```

- model-generated/external content 默认 `training_eligible=false`；
- remembered instruction 默认只是 data，只有显式 policy 可提升 authority；
- correction、forget、revocation 必须传播至 dataset snapshot、cache、index、candidate 和 artifact state；
- telemetry 默认不存 raw query/body/citation/secret/PII；
- privacy filter 失败时 learning admission fail closed，但不能阻塞 baseline 回答。

## 3.3 Learning 不变量

1. 没有真实 action/outcome/feedback，只能称 observational/shadow。
2. 模型自评不能作为唯一 reward。
3. behavior propensity 必须来自实际 behavior policy，不得事后猜测。
4. unsupported action 不进入 IPS/SNIPS/DR。
5. delayed outcome 必须显式链接原 episode/action。
6. credit 总量必须守恒、可重算、可审计。
7. missing/censored/timeout 必须有明确 policy。
8. 每次训练使用 immutable dataset snapshot。
9. split 必须按 episode/workspace/time，防止 leakage。
10. 学习不可用时 deterministic baseline 必须继续运行。
11. OOD、证据不足或不确定性高时优先 abstain。

---

# 4. 目标架构

```text
┌──────────────────────── Authority / Governance Plane ────────────────────────┐
│ capability · lease · generation · policy · budget · approval · CALLERS     │
└──────────────────────────────────┬────────────────────────────────────────────┘
                                   │ RunStartSnapshotV2
                                   ▼
┌──────────────────────── Knowledge / Retrieval Plane ─────────────────────────┐
│ source → memory lifecycle → grounding → truth → lexical/ANN/KG/recency      │
└──────────────────────────────────┬────────────────────────────────────────────┘
                                   │ CandidateSetReceiptV1
                                   ▼
┌──────────────────────── Fast Decision Plane ─────────────────────────────────┐
│ FeatureBuilder → H5 Signal Network → H6 Selective Policy → abstain/choice   │
│ no authority · no mid-run artifact mutation · explicit OOD/uncertainty      │
└──────────────────────────────────┬────────────────────────────────────────────┘
                                   │ PolicyDecisionReceiptV2
                                   ▼
┌──────────────────────── TaskFlow / Execution Spine ──────────────────────────┐
│ approved workflow → step → effect intent/receipt → postcondition/reconcile  │
└──────────────────────────────────┬────────────────────────────────────────────┘
                                   │ immutable causal events
                                   ▼
┌──────────────────────── Learning Event Plane ─────────────────────────────────┐
│ Episode → candidates → decision → action → outcome → correction → credit    │
└──────────────────────────────────┬────────────────────────────────────────────┘
                                   │ DatasetSnapshotV1
                                   ▼
┌──────────────────────── Slow Consolidation Plane ─────────────────────────────┐
│ replay → train head/adapter/calibration → OPE/CI → retention/no-regression   │
└──────────────────────────────────┬────────────────────────────────────────────┘
                                   │ signed next-snapshot artifact
                                   ▼
┌──────────────────────── Artifact / Promotion Plane ───────────────────────────┐
│ registry → shadow → canary → operator → CALLERS → rollback/retire/revoke     │
└───────────────────────────────────────────────────────────────────────────────┘

Optional isolated research:
spike/event-time → LIF/STDP/homeostasis simulation → research receipts only
```

## 4.1 Fast loop

```text
one frozen snapshot
→ deterministic bounded features
→ H5 signals and temporal state
→ H6 distribution + OOD + abstain
→ baseline/shadow/canary choice
→ authority-controlled TaskFlow
→ postcondition/outcome
```

Fast loop不得训练、改 graph、改 authority、安装 artifact 或写长期 truth。

## 4.2 Slow loop

```text
immutable episodes
→ eligibility/privacy filter
→ dataset freeze
→ candidate training
→ replay/OPE/ESS/CI
→ subgroup/retention/forgetting
→ signed artifact
→ shadow/canary
→ next snapshot
```

---

# 5. Canonical Contracts

## 5.1 `RunStartSnapshotV2`

必须绑定：

```text
snapshot_id/run_id/turn_id
agent_id/spawn_generation/workspace_scope_digest
authority_epoch/owner_epoch/generation/fencing_token/lease_expiry
definition/graph/policy/capability digests
memory/KG projection generation
model registry/model/head/adapter/calibration/tokenizer digests
artifact manifest digest
privacy profile/execution scope/resource budget
logical clock/rng seed
```

## 5.2 `LearningEpisodeV1`

唯一顶层因果身份：

```text
episode_id/schema/namespace/snapshot_digest
agent/spawn/workspace/turn/task/risk
opened logical clock/terminal state/head seq/head digest
privacy class
four eligibility axes
```

状态：

```text
Opened → CandidateSetBound → DecisionBound → ActionObserved
→ OutcomePending → OutcomeObserved → CreditReady → Evaluated → Archived

exception: Indeterminate | Quarantined | RevocationPending | Revoked
```

## 5.3 `LearningEventV1`

事件至少包括：

```text
SnapshotStart QueryPlan CandidateSet H5Signal H6Decision BaselineDecision
CanaryDecision TaskFlowStart ActionPrepared EffectReceipt Postcondition
ImmediateOutcome DelayedOutcome UserCorrection ForgetRequest Revocation
SafetyObservation Terminal
```

每条事件带连续 sequence、精确 causal parent、state/policy/candidate/decision/action/outcome digests、propensity/support、eligibility、privacy/trust 和 event digest。

## 5.4 `CandidateSetReceiptV1`

必须保存完整候选集，而非只保存 chosen：

```text
candidate IDs/digests
availability mask
channel scores
grounding/truth/trust/risk/evidence status
behavior distribution/support
query/retrieval/tokenizer/model/index digests
```

## 5.5 `NeuronSignalReceiptV2`

```text
neuron/group/kernel/artifact IDs
feature vector + provenance + missingness mask
input/snapshot/state/output digests
signal value/distribution
uncertainty/calibration/attribution
activation/inhibition/homeostasis metadata
device/latency/resource/fallback
negative authority flags
```

## 5.6 `PlasticityStateV1`

区分 run 内短期状态与跨快照 artifact：

```text
temporal activation
eligibility traces
recent failure/correction state
refractory/suppression state
homeostatic target and budget
neuromodulator digest
TTL/snapshot/generation binding
rebuild digest
```

它不得包含 production weight mutation、authority 或 truth。

## 5.7 `PolicyDecisionReceiptV2`

```text
full candidate probability distribution
selected candidate or abstain
confidence/calibration/OOD/novelty
risk/evidence veto
behavior propensity
reason attribution
H5/candidate/snapshot/policy digests
mode: baseline | shadow | canary
negative authority flags
```

## 5.8 `ExplorationPolicyReceiptV1`

用于真实 support 获取：

```text
position/risk class
eligible candidate mask
safety mask
full behavior distribution
epsilon/temperature/method
exact RNG seed
selected action and propensity
exploration budget and expiry
kill switch
```

初期只允许 `MemoryRetrievalRank`、`ContextSalience` 等无外部副作用位置。

## 5.9 `OutcomeReceiptV1`

reward 必须拆分：

```text
task success
postcondition correctness
user correction
forget/revocation
tool/provider verified result
safety violation
latency/cost/resource
abstain appropriateness
delayed satisfaction
```

每项记录 source、confidence、observed_at、causal distance、missing/censored 和 digest。

## 5.10 `CreditLedgerV1`

- hierarchical credit；
- total conservation；
- bounded temporal decay；
- deterministic baseline first；
- learned credit 只能并行候选；
- no model-only reward。

## 5.11 `DatasetSnapshotV1`

```text
dataset_id/query/version
included/excluded episode manifests
eligibility/privacy/trust filters
correction/forget/revocation lineage
train/eval split by episode/workspace/time
feature/label schema digests
parent artifact and code/toolchain digests
```

## 5.12 `EvaluationReceiptV2`

必须包含：

```text
sample/supported count
coverage
ESS
weight p50/p95/max
clipped ratio
IPS/SNIPS/DR
cluster/block confidence interval
candidate LCB/baseline UCB
practical improvement
critical subgroup results
calibration/OOD/abstain
1d/7d/30d/90d retention
drift/forgetting/privacy/resource
```

## 5.13 `PolicyArtifactManifestV2`

```text
artifact/position/parent IDs
training dataset and code/toolchain digests
model/head/adapter/calibration/normalization digests
evaluation/safety/privacy receipts
compatibility matrix
expiry/revocation/rollback pointer
signature/trust anchor
```

## 5.14 `UnlearningComplianceReceiptV1`

forget lineage 不等于模型已遗忘。该 receipt 必须列出：

```text
affected episodes/datasets/caches/indexes/artifacts
immediate exclusion actions
revocation_pending artifacts
clean retrain or verified unlearning method
behavioral forget tests
completion/remaining contamination state
```

无法证明 weight-level unlearning 时，旧 artifact 必须 retire/revoke 并从 clean snapshot 重训。

## 5.15 `TopologyProposalV1`

仅 proposal-only：

```text
add/split/merge/retire expert
rewire bounded signal dependency
add retrieval channel/feature
change sparse routing
```

必须绑定 compiler、authority/cycle check、ablation、lesion、support-aware replay、shadow、canary、operator 和 rollback。

---

# 6. Knowledge、Memory 与 Eligibility

## 6.1 三层 truth

```text
SourceWitness: 原始来源是否存在且字节可验证
FactGrounding: 某 fact 是否被精确 span 支撑
TruthStatus: fact 是否经冲突、时效、scope、authority 验证
```

Grounded 不自动等于 true；可 recall 不自动可训练。

## 6.2 四轴 eligibility

每个 memory、event、episode、candidate 和 dataset member独立保存：

```text
recall_eligible
training_eligible
evaluation_eligible
promotion_eligible
```

默认：

- 用户明确事实/纠正：可进入审查；
- 模型输出：只作为 provisional proposal；
- 外部内容：默认不可训练；
- secret/PII/高熵 token：全部 false；
- forgotten/revoked：未来 snapshot 全部 false。

## 6.3 双速记忆与巩固

```text
Fast Episodic Store:
  exact episode/action/outcome/correction/forget

Slow Consolidation:
  dataset freeze/replay/anti-forgetting/eval/artifact
```

单次 run 内不更新长期 policy 或 truth。

---

# 7. H5 Neuron 深化计划

## 7.1 Claim ladder

```text
N0_METAPHORICAL_TYPED_PROPOSAL        当前
N1_ADAPTIVE_SIGNAL_UNIT               近期产品目标
N2_TEMPORAL_RECURRENT_SIGNAL_NETWORK  中期功能级仿生目标
N3_NEUROMORPHIC_RESEARCH              独立研究
N4_BIOLOGICAL_NEURON_REPLICA           永久禁止声明
```

## 7.2 H5.0 Claim closure

- 现有 `NeuronProposal` 标记 N0；
- receipt 增加 `bio_claim_level`；
- README/PR/verifier 禁止“仿生神经元已实现”；
- 所有 authority flags 编译期为 false。

## 7.3 H5.1 Deterministic Feature Builder

输入只能来自冻结 snapshot 和 bounded evidence：

```text
retrieval channels/grounding/truth/trust/freshness/contradiction
diversity/recent success-failure/latency/budget/risk/task/workspace
```

要求 canonical order、missingness mask、provenance、normalization digest、no secret/PII、overflow/adversarial tests。

## 7.4 H5.2 Interpretable Signal Kernel

第一版只使用：

```text
regularized linear scorer
small tree/ranker
small MLP or calibration head when justified
```

禁止第一版使用生成模型决定 signal。输出必须有 per-feature attribution、uncertainty、calibration、resource receipt 和 deterministic fallback。

## 7.5 H5.3 Temporal State

加入 bounded：

```text
exponential decay
recent failure/correction state
short recurrent state
eligibility trace
competition/inhibition proxy
homeostatic activation budget
```

状态 Agent-local、snapshot/generation-bound、bounded size/TTL、可 crash/rebuild、不写长期 truth、不跨 workspace。

## 7.6 H5.4 Shared Local Backbone

不是“每个 neuron 一个小模型”，而是：

```text
one shared frozen local encoder/backbone
+ task-specific small heads/adapters
+ independent typed schemas and temporal states
+ sparse routing and resource budgets
```

首批适合本地模型的任务：entity/relation proposal、semantic representation、risk/novelty、retrieval rerank。Authority、truth 和 effect approval 永远不用模型替代。

## 7.7 H5.5 Offline Artifact Update

可更新：weights、head/adapter、normalization、temperature、threshold。禁止更新 position registry、authority、graph topology、effect rules 和 safety floor。

## 7.8 H5.6 Calibration、Ablation、Lesion

必须评测 signal calibration、feature ablation、neuron/group lesion、shortcut/leakage、dominant feature、OOD abstain、cross-language/workspace/risk subgroup 和 7d/30d retention。

## 7.9 H5.7 N2 Temporal Recurrent Network

只有 N1 稳定后才加入：

```text
k-of-N sparse activation
lateral inhibition
winner/coalition formation
bounded recurrent refinement
oscillation detection
homeostatic scaling
neuromodulator-controlled plasticity
```

固定最大迭代、总激活、latency/energy budget；不收敛时 abstain 或回 deterministic baseline。

## 7.10 H5.8 N3 Neuromorphic Research

独立非生产 track 可研究 event/spike time、LIF、STDP-like local update、homeostasis 和 sparse competition。必须与 production registry、authority 和 promotion 完全隔离；只有仿真/硬件、能耗、稳定性和 efficacy receipts 后才可称 experimental neuromorphic research。

---

# 8. H6 Intuition 深化计划

## 8.1 当前边界

当前 H6 只是 `I0_DETERMINISTIC_SELECTIVE_POLICY`：hard filter → rank → tie-break → threshold → suggest/abstain。调用者提供 score/risk，它没有经验学习、hidden state、OOD 或概率分布。

## 8.2 Claim ladder

```text
I0_DETERMINISTIC_SELECTIVE_POLICY  当前
I1_CALIBRATED_FAST_POLICY          近期目标
I2_EXPERIENCE_SHAPED_FAST_POLICY   闭环与跨窗口验证后
I3_HUMAN_LIKE_INTUITION            永久禁止声明
```

## 8.3 H6.0 Receipt V2

receipt 必须可由 immutable inputs 完全重算；candidate order 不影响 canonical result；self-consistent but wrong decision、stale/cross-policy replay 必须拒绝。

## 8.4 H6.1 Distributional Selective Policy

输出完整候选概率与 abstain mass。第一阶段可用 softmax、Platt、isotonic、calibrated tree；hard veto 永远在 rank 前。

## 8.5 H6.2 Uncertainty/OOD

至少包含：

```text
epistemic proxy
score margin/aleatoric proxy
candidate coverage
feature missingness
distance-to-training-support
novelty/OOD
```

OOD 高必须提高 abstain，不允许以高 confidence 掩盖无支持区域。

## 8.6 H6.3 Bounded Temporal Context

只允许 recent failure、correction、provider indeterminate、retrieval instability 和 budget pressure 等可解释状态；不得把未经治理的整段 memory 作为 hidden prompt。

## 8.7 H6.4 Calibration Gates

指标：ECE、Brier、NLL、coverage、selective risk、abstain precision/recall、OOD AUROC/AUPRC、regret、safety veto false-negative；按 language/workspace/task/risk/backend 分组。

## 8.8 H6.5 Safe Exploration

阶段：

```text
SuggestOnly
→ ShadowCompare
→ PrepareOnly
→ LowRiskExplorationShadow
→ LowRiskCanarySelect
```

探索由 `ExplorationPolicyReceiptV1` 控制。tool/provider/auth/credential/high-risk branch 禁止作为第一闭环。

## 8.9 H6.6 I2 晋升

需要 durable closed-loop episodes、真实 behavior propensity/support、next-snapshot artifact update、7d/30d retention、无 critical subgroup regression、calibrated abstain 和 operator-reviewed rollback。

---

# 9. H7 统一长期学习计划

## 9.1 唯一目标

```text
durable causal episode
+ full candidate set
+ behavior action/propensity/support
+ execution/postcondition
+ immediate/delayed outcome
+ correction/forget/revocation
+ conserved credit
+ immutable dataset
+ OPE/ESS/CI
+ signed artifact
+ next-snapshot reload/rollback
```

## 9.2 H7.0 Contract Reconciliation

- 定义唯一 `LearningEventV1`；
- 现有 durable trajectory 和 pure feedback oracle 只作为 migration input；
- 禁止新 dual-write；
- collision/cross-scope/stale-fence/property tests；
- 无新 production caller。

## 9.3 H7.1 `hepta-learning-ledger`

独立 crate/SQLite：

```text
learning_episodes
learning_events
learning_outcomes
learning_credit_entries
learning_corrections
learning_revocations
learning_dataset_membership
```

要求 append-only、hash chain、exact replay、changed replay conflict、one snapshot/spawn/generation、contiguous parent、terminal + delayed outcome、WAL/crash/reopen/corruption/failpoints、read-only verifier、no effect authority。

## 9.4 H7.2 Action、Propensity、Support

```text
behavior_propensity > 0
target_propensity >= 0
selected action in support
candidate set and availability mask exact
```

禁止事后猜 propensity、丢失 unselected candidates、让 unsupported episode 进入 OPE。

## 9.5 H7.3 Delayed Outcome

支持 immediate postcondition、same/next-turn correction、task completion、7d satisfaction、forget/revocation；记录 delay、censoring、confidence 和 causal binding。

## 9.6 H7.4 Credit

先采用可解释 terminal decomposition、bounded decay、hierarchical conservation。learned credit model 后置且只作为候选，与 deterministic baseline 并行。

## 9.7 H7.5 OPE

最少：IPS、SNIPS、doubly robust、weight clipping diagnostics、support coverage、negative control。只报告 offline loss 或点估计禁止晋升。

## 9.8 H7.6 ESS、CI、Sequential Policy

```text
ESS = (sum w)^2 / sum(w^2)
clustered/bootstrap CI by episode/workspace/time
candidate LCB
baseline UCB
practical improvement
pre-registered sequential peeking policy
```

## 9.9 H7.7 Retention、Drift、Forgetting

窗口：1d、7d、30d、90d。评测 old-task retention、new-task gain、correction adoption、forget compliance、truth pollution、calibration/subgroup drift、artifact age decay。

## 9.10 H7.8 Artifact Registry

```text
Draft → Trained → OfflineEvaluated → ShadowApproved
→ CanaryApproved → OperatorAccepted → Promoted
→ RolledBack | Retired | RevocationPending | Revoked
```

artifact 与 approval/trust anchor 分离；当前 run 禁止 mid-flight replacement。

---

# 10. 本地模型与资源架构

## 10.1 当前事实

本地 inference 分支目前是 source-present/not-run；没有 real-model E2E、hardware/NPU receipt、hepta-inferd 或 H5/H6 runtime wiring。因此当前 H5/H6 不得声称使用本地小模型。

## 10.2 目标执行方式

```text
Rust host
→ typed UDS/IPC request
→ shared local model pool/sidecar
→ typed result + confidence + attribution + ModelReceipt
```

- Mac：Core ML/ANE 优先，CPU/MLX 作为显式 fallback；
- RTX 4060：较重 batch/embedding/rerank qualification；
- j3160：deterministic rule/remote only，不加载本地 model pool；
- Ollama/LM Studio 只能通过 loopback、exact model manifest、no proxy/redirect、无隐式安装的资格路径。

## 10.3 Position-specific fallback

```text
guard_deterministic: rule only, remote forbidden
privacy_local: NPU → local CPU/GPU → rule, remote forbidden
proposal_local: NPU → local → policy-allowed remote → rule, proposal only
```

每次 fallback 生成 receipt，不得静默改变 policy。资源超限只停止当前 invocation/run，不能破坏 CognitiveStore、TaskFlow 或 peer Agent。

## 10.4 资源指标

每次 invocation 记录 p50/p95/p99、peak RSS/VRAM、queue wait、fallback ratio、error/abstain、thermal/throttle、energy proxy、artifact/device digests。每个 group 有 concurrency、memory、latency、energy 和 circuit-breaker budget。

---

# 11. 功能级仿生机制

## 11.1 不追求生物复制

Hepta 的目标不是模拟完整生物脑，而是实现可验证的功能级机制：

```text
temporal state
sparse activation
competition/lateral inhibition
homeostasis
eligibility traces
neuromodulation
multi-timescale adaptation
offline replay/consolidation
lesion/ablation
graceful degradation
proposal-only structural plasticity
```

## 11.2 Neuromodulator Vector

独立信号：

```text
reward prediction error
novelty/surprise
uncertainty
user correction
safety risk
memory pollution
resource pressure
latency/cost
```

用于控制 episodic admission、trace TTL、探索预算、abstain、slow consolidation 和 topology proposal；hard safety veto 永远不能被 reward 抵消。

## 11.3 多时间尺度

```text
milliseconds/seconds: activation, inhibition, recurrent refinement
minutes/hours: episodic state, eligibility, local calibration observation
days/weeks: dataset freeze, artifact training, replay, retention, promotion
```

## 11.4 Graceful Degradation

必须通过 group/feature/model lesion 证明：移除某单元后系统能回 deterministic baseline、冗余 expert 或 abstain，不产生 authority escape 或 silent quality collapse。

---

# 12. 路线图与依赖 DAG

```text
Q0 Qualification Debt Closure
→ A0 Canonical Capability/Evidence Authority
→ B0 Learning Boundary Extraction
→ C0 LearningEpisode + Durable Ledger
→ R1 Grounded Retrieval/Telemetry/Security
→ N1 H5 Adaptive Signal Unit
→ I1 H6 Calibrated Fast Policy
→ L1 H7 Causal Evaluation/Artifact
→ C1 Low-risk Closed Loop
→ N2 Temporal Recurrent Signal Network
→ S1 Governed Structural Plasticity
→ N3 Isolated Neuromorphic Research
```

## 12.1 Q0 — Qualification Debt Closure

**Entry**：当前。  
**交付**：冻结 P0.1–P0.4c exact heads/trees/toolchain/migrations；E1 local executable；E2 independent runner；source/fmt/focused/full tests/clippy；SQLite upgrade/reopen/corruption/failpoints；Agentd default-off；可读 artifacts。  
**Exit**：同一冻结候选通过；所有 runtime/production authority 仍 false。  
**Blocked**：新 runtime、migration、P1/H5/H6/H7 source。

## 12.2 A0 — Canonical Authority

交付 capability/evidence/PR-stack registry；一个命令输出 current truth；自动核对 branch/head/status/authority。历史文件不得作为 current。

## 12.3 B0 — Boundary Extraction

最少拆出：

```text
hepta-learning-contracts
hepta-learning-ledger
hepta-policy-runtime
hepta-evaluation
hepta-intelligence narrow façade
```

先 compatibility adapter、双读、no-cycle、API receipt，再切 owner；每步可回滚。

## 12.4 C0 — Episode and Eligibility

实现 contracts、durable ledger、correction/forget/revocation、feature-default-off Agentd host。Exit：完整 synthetic episode 可 reopen/replay，changed replay conflict，无 memory/KG/outbox/effect authority。

## 12.5 R1 — Grounded Retrieval and Data Quality

lexical/alias/ANN/bounded KG/rerank、no-content telemetry、federation snapshot merge、semantic security、multilingual efficacy dataset。没有数据质量和 eligibility，禁止 policy learning。

## 12.6 N1 — H5 Adaptive Signal

claim closure → feature builder → linear/tree kernel → temporal state → offline artifact → calibration/ablation。Exit 最多 N1；7d retention、资源预算、无 subgroup regression、no authority。

## 12.7 I1 — H6 Calibrated Fast Policy

DecisionReceiptV2 → distribution → OOD/abstain → bounded temporal context → calibration/selective risk → shadow compare。baseline 仍是产品 decision；尚不 canary。

## 12.8 L1 — H7 Evaluation and Artifact

trajectory/feedback reconciliation → durable action/outcome → credit → OPE → ESS/CI → retention/unlearning → signed artifact。只允许 local qualification snapshot reload/rollback。

## 12.9 C1 — Low-risk Closed Loop

第一 position：`MemoryRetrievalRank`；第二候选：`ContextSalience`。阶段：baseline+shadow → governed exploration → 1% low-risk canary → bounded workspace → operator。禁止 tool/provider/auth/credential/topology/high-risk branch。

Exit：candidate LCB > baseline UCB + practical threshold；ESS/coverage/clip/safety/subgroup/7d retention/rollback 全通过。

## 12.10 N2 — Recurrent Functional Bio-inspiration

实现 sparse competition、lateral inhibition、homeostasis、eligibility、bounded recurrent loop、lesion 和 graceful degradation。仍无 authority。

## 12.11 S1 — Structural Plasticity

仅 TopologyProposal；GraphCompiler、cycle/authority、ablation/lesion、support replay、shadow/canary/operator/rollback。运行中 graph 永不改变。

## 12.12 N3 — Neuromorphic Research

LIF/STDP/event-time/hardware-simulation 独立 registry 和 CI；不能继承产品 authority 或 claim。

---

# 13. PR 与提交顺序

任何阶段不得超过两层未 qualification source stack。

## Q0 tranche

1. freeze exact stack manifest；
2. reproducible local runner；
3. grounding/mutation SQLite matrix；
4. Agentd default-off matrix；
5. append-only E1/E2 receipts。

## Boundary tranche

1. learning contracts；
2. learning ledger；
3. policy runtime；
4. evaluation；
5. façade compatibility/no-cycle receipts。

## Causal ledger tranche

1. LearningEpisode/Event；
2. outcome/correction/forget/revocation；
3. crash/reopen/replay/corruption；
4. no-authority host。

## Retrieval tranche

query/tokenizer/channel registry → local embedding adapter/fallback → candidate set receipt → telemetry/security → efficacy receipt。

## H5/H6/H7 tranche

每个编号子阶段独立 PR；未通过前一 exact-head gate，不得启动下一 runtime tranche。

---

# 14. Evidence 与 Qualification 等级

| Class | 证据 | 可支持声明 |
|---|---|---|
| E0_SOURCE | schema/static/source verifier | source-present/implemented |
| E1_LOCAL_EXECUTABLE | exact-head local fmt/test/clippy/SQLite | developer executable confidence |
| E2_INDEPENDENT_RUNNER | independent exact-head logs/artifacts | qualified candidate |
| E3_RUNTIME | real loopback/runtime/restart/failpoint/hardware | wired runtime behavior |
| E4_EFFICACY | pre-registered corpus/controlled experiment | efficacy proven |
| E5_GOVERNANCE | operator/rollback/CALLERS | accepted/promoted |

每个 capability 使用：

```text
implemented → wired → qualified → efficacy_proven → operator_accepted → promoted
```

任一上游 false，下游必须 false。

---

# 15. 测试与评测矩阵

## 15.1 Source/Contract

JSON/Schema、canonical serialization、unknown fields、digest、negative authority、current pointer、single-plan uniqueness、forbidden claims。

## 15.2 Rust

fmt、focused/full tests、strict clippy、feature-default-off、dependency/no-cycle、property/fuzz、cross-platform deterministic fixtures。

## 15.3 SQLite

migration checksum、upgrade、BEGIN IMMEDIATE、pre/post commit failpoint、exact/changed replay、corruption、reopen、WAL checkpoint/restart、bounded growth、revocation lineage。

## 15.4 Fault Injection

kill/timeout/disconnect/restart/disk full/corruption/generation rollover/stale owner/duplicate callback，在 intent 前后、dispatch、result/receipt、postcondition、commit、compact、approval、artifact reload 各窗口都有明确 terminal/recovery/indeterminate。

## 15.5 Security/Privacy

prompt-injection memory、external/model training admission、instruction escalation、scope escape、stale capability、secret/PII/high entropy、receipt forgery、dataset leakage、forget bypass、artifact revocation。

## 15.6 Retrieval

Recall@4、nDCG@4、citation precision、false/stale/contradicted memory attachment、task success、token cost、p95/p99。

## 15.7 H5/H6

ECE、Brier、NLL、selective risk、OOD AUROC/AUPRC、abstain quality、regret、feature ablation、group lesion、activation sparsity、homeostatic stability、resource/energy。

## 15.8 H7

coverage、ESS、weight clipping、IPS/SNIPS/DR、CI/LCB/UCB、negative controls、subgroup、retention/forgetting、drift、unlearning compliance。

---

# 16. 初始 Pilot 阈值注册

低风险 pilot 默认建议：

```text
minimum episodes >= 5,000
supported episodes >= 2,000
ESS >= 500
support coverage >= 80%
clipped ratio <= 5%
candidate LCB improvement >= 2% absolute or pre-registered practical threshold
critical safety regression = 0
false-memory attachment not worse than baseline
p95 decision overhead <= 20% of baseline or bounded absolute budget
7d retention before canary expansion
30d retention before broad promotion
```

阈值按 position/risk 预注册，不能从结果反向选择；它们不自动授予 production authority。

---

# 17. Rollback、Kill Switch 与 Unlearning

每个 artifact/canary 预生成 previous artifact、rollback artifact/transaction、scope、expiry、kill switch、recovery target、schema compatibility。

触发：signature/digest、OOD/abstain collapse、safety/subgroup/calibration、false-memory、revocation contamination、latency/resource、operator stop。

Rollback 必须幂等、离线可执行、不依赖 candidate、生成 receipt、下一 snapshot 验证、保留历史。

Forget/revocation：立即阻止未来 recall/training/eval/promotion；污染 artifact 进入 `RevocationPending`；没有可信 unlearning evidence 时 retire 并 clean retrain。

---

# 18. Observability

只记录 no-content metrics：episode state、candidate size、abstain/veto、support/coverage、missing/censored outcome、correction/forget/revocation、dataset include/exclude、artifact transitions、latency/resource buckets。

Dashboard：capability truth、qualification、learning data health、support/OPE、calibration/OOD、retention/forgetting、privacy/revocation、artifact/canary/rollback。禁止展示 raw memory/query/secret。

---

# 19. 风险清单

| 风险 | 严重度 | 硬缓解 |
|---|---:|---|
| source receipt 被当 qualification | Critical | evidence classes + verifier |
| 未 qualification 栈增长 | High | two-layer budget + Q0 freeze |
| grounding 被当 truth | Critical | three-layer truth |
| recall 被当 training eligibility | Critical | four axes |
| H7 双真相 | Critical | canonical learning ledger |
| propensity 事后猜测 | Critical | behavior/exploration receipt |
| support 不足仍 OPE | High | hard support/ESS gate |
| point estimate 假阳性 | High | CI/LCB/UCB/sequential policy |
| delayed outcome 错归因 | High | explicit causal/censoring |
| forget 未传播 | Critical | lineage + unlearning receipt |
| god crate 耦合 | High | early boundary extraction |
| H5/H6 过度宣传 | High | claim ladder + forbidden terms |
| catastrophic forgetting | High | replay/retention/no-regression |
| telemetry 泄露 | Critical | no-content schemas |
| mid-run artifact replacement | Critical | next-snapshot-only |
| topology 越权 | Critical | proposal/compiler/operator |
| low-sample overfit | High | shared artifact/local calibration |
| runner blocker掩盖 defect | High | E1 local + E2 independent |
| 本地模型资源失控 | High | shared pool/budget/circuit breaker |
| reward hacking | Critical | decomposed outcome + hard safety |

---

# 20. Definition of Done

## 20.1 统一计划 DoD

- 稳定 master 是唯一 current human plan；
- CURRENT_PLAN、verifier、session AGENTS 指向同一文件；
- 旧版本计划标记 historical；
- authority flags false；
- 计划不声称已实现自我进化、长期 efficacy 或仿生机制。

## 20.2 Q0 DoD

exact frozen heads、non-empty steps、Rust exact、fmt/test/clippy、migration/reopen/corruption/failpoint、zero-write negatives、Agentd default-off、readable artifacts、qualified receipt；operator/promotion false。

## 20.3 L1 Observational Continual DoD

durable observations、eligibility/lineage、immutable dataset、offline calibration/retrieval update、next-snapshot reload/rollback、7d/30d retention、forget compliance。

## 20.4 L2 Closed-loop DoD

LearningEpisode durable、真实 propensity/support、effect/postcondition/outcome、delayed correction/forget、credit conservation、OPE/ESS/CI、subgroup/no-regression、signed artifact、shadow/canary、rollback rehearsal、operator、独立 CALLERS promotion。

## 20.5 N2 Functional Bio-inspired DoD

temporal state、sparse competition、lateral inhibition、homeostasis、neuromodulation、lesion/ablation、graceful degradation、multi-timescale retention，且无 authority escape。

---

# 21. 当前唯一执行顺序

严格执行：

1. 冻结 P0.1–P0.4c exact stack；
2. 修复/建立本地可复现 executable runner，产出 E1；
3. 恢复独立 runner，产出 E2；
4. 对冻结候选执行 source/fmt/test/clippy/SQLite/reopen/failpoint/default-off；
5. 失败则只修原 tranche，不继续上堆；
6. Q0 通过后建立 A0 capability/evidence registries；
7. 提前拆出 learning contracts/ledger/policy/evaluation；
8. 实现 LearningEpisode、eligibility、correction/forget/revocation；
9. 完成 grounded retrieval、telemetry、security 和 efficacy；
10. 启动 H5 N1；
11. 启动 H6 I1；
12. 统一 H7 durable learning、trainer、OPE/CI；
13. 在 MemoryRetrievalRank 做受治理低风险探索与 canary；
14. 证明 L2 后再做 N2/S1；
15. N3 永远是隔离研究 track。

---

# 22. 最终架构决策

Hepta 的最佳方向不是把每个所谓 Neuron 包装成一个小语言模型，也不是模拟完整生物神经系统，而是建设：

> **事实可证、因果可回放、信号有状态、策略可校准、探索受治理、学习可统计验证、遗忘可追踪、artifact 可晋升、失败可回滚的长期智能系统。**

产品级演进顺序必须严格区分：

```text
metaphorical typed proposal
→ adaptive signal unit
→ calibrated fast policy
→ observational continual learning
→ governed closed-loop learning
→ functional temporal/recurrent bio-inspiration
→ proposal-only structural plasticity
→ isolated neuromorphic research
```

任何阶段都不能自动继承下一阶段的 claim 或 authority。
