# Hepta Intelligence Master Development Plan

> **CANONICAL_CURRENT / PLAN_ONLY / FAIL_CLOSED**  
> Plan ID: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4`  
> Version: `4.1.0`  
> Repository: `ProfHepta/hepta-private-ci`  
> Current program phase: `A0_CANONICAL_CAPABILITY_AND_EVIDENCE_AUTHORITY`  
> Current capability claims: `L0 / N0 / I0`  
> 本文件是**唯一有效的人类可读开发计划**。任何 PR 正文、旧 status、workflow 日志、artifact、fixture 或 Draft 分支均不得单独替代它。

---

## 0. 强制读取顺序与权威边界

每次开发、审计、资格化或晋级必须依次读取：

```text
HEPTA_INTELLIGENCE_CURRENT_PLAN.json
→ HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json
→ HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json
→ HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json
→ HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json
→ HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json
→ HEPTA_INTELLIGENCE_MASTER_PLAN.md
```

`CURRENT_PLAN` 是唯一聚合机器事实；本文件是唯一 current human plan。其他 registry 是被注册的规范输入，但都没有 production、operator、promotion、release 或 `CALLERS` authority。旧 `EXECUTION_STATUS_V2/V3` 与 tranche snapshots 只保留兼容性，不是 current truth。

任何 repository、branch、head、tree、parent、schema、digest、registered consumer、authority flag 或 read order 不一致都必须 `FAIL_CLOSED`。不得把 queued、`steps=[]`、`runner_id=0`、source-only gate、过期 artifact 或 PR prose 解释成 executable qualification。

---

## 1. 当前精确事实

### 1.1 Q0 exact candidate

```text
repository = ProfHepta/hepta-private-ci
repository_id = 1320694176
branch = codex/hepta-intelligence-plan-v3-20260828
head = c768bcbeb4c1168088d2499828c24da521a2a73a
tree = ca455a9ef797cd95164c880c7b8faba80b305589
parent = aeb8ac0bfb30d570a16c4914b6e4b31ce035dd62
workflow = .github/workflows/hepta-intelligence-q0-paired-candidate-v10.yml
run = 33252922404
attempt = 1
```

双架构执行事实：

```text
E1 x86_64: job 99101597686, runner 1000037745, success
E2 ARM64:  job 99101597800, runner 1000037696, success
pair:      job 99105393694, runner 1000037821, success
```

对应 artifacts：

```text
E1   id=9715334789 digest=sha256:e2356966e107c4cd5f00cc59d04520883f7fdf9a78b0b4c6cf6e83ce70c3b24b
E2   id=9715221566 digest=sha256:8c794d42726746e80ee71bbd938866bb1a2e7202d8ca1a22636aa1529c16dde3
pair id=9715623771 digest=sha256:3d225febc5931759cca61729fff492e9e45596c36cd2463524054afd4e21d606
```

Q0 结论：

```text
q0_executable_evidence_complete=true
q0_executable_qualified=true
qualified_candidate=true
runtime_wired=false
runtime_capability_qualified=false
full_repository_merge_green=false
production_authority=false
```

`qualified_candidate=true` 仅说明 exact source candidate 的双架构 admission matrix 已通过；它不等于产品 capability 已 wired、runtime-qualified、efficacy-proven、operator-accepted 或 promoted。

### 1.2 全仓边界

Q0 专项绿灯与 default-branch merge 资格分离。同一历史 head 的全仓 checks 曾出现 Bazel、cargo-deny、cargo-shear、repo-checks、macOS/Windows build/lint 等失败或未完成项。因此：

```text
full_repository_merge_green=false
merge_candidate_qualified=false
production_candidate=false
```

A0 只能记录该事实，不得把与 A0 无关的全仓失败伪装成 Hepta Intelligence runtime failure，也不得在 default-branch required checks 全绿前宣称可合并。

---

## 2. 能力成熟度与禁止夸大

当前真实声明：

```text
system_learning=L0_STATIC_SHADOW_WITH_PARTIAL_L1_FOUNDATIONS
H5=N0_METAPHORICAL_TYPED_PROPOSAL
H6=I0_DETERMINISTIC_SELECTIVE_POLICY
self_evolution=false
longitudinal_learning_efficacy=false
closed_loop_learning=false
structural_plasticity=false
neuromorphic_mechanism=false
biological_mechanism_replication=false
local_small_model_used_by_h5=false
local_small_model_used_by_h6=false
```

统一生命周期：

```text
implemented
→ candidate_qualified
→ wired
→ runtime_qualified
→ efficacy_proven
→ operator_accepted
→ promoted
```

禁止跳级。单一 source/test receipt 只能推进 `candidate_qualified`；只有真实 product graph、E3 runtime/restart/failpoint、E4 efficacy/retention、operator acceptance、rollback 与独立 `CALLERS` receipt 才能继续晋级。

---

## 3. Gap 分类与执行纪律

所有 gap 必须落入以下一类：

- `CLOSED_SOURCE_CONTROLLED`：已由 exact-head source 与 executable evidence 闭合。
- `OPEN_SOURCE_CONTROLLED`：可通过本仓代码、文档、测试和 CI 闭合。
- `BLOCKED_EXTERNAL_EVIDENCE`：真实 corpus、reviewer、hardware、soak 或 operator 才能闭合。
- `BLOCKED_UPSTREAM`：依赖未合并或未资格化的上游。
- `STOP_CONDITION`：安全、权限、身份或状态漂移要求立即停止。

外部证据缺失不得用 fixture、随机数据、测试密钥或模拟 reviewer 伪造闭合。应写入明确 blocker、所需输入、验收 schema、责任人类别和恢复条件。

运行期 source stack 在 A0 完成前冻结：不得新增 runtime source、SQL migration、product caller、H5/H6/H7 runtime、model download、provider dispatch、production authority、`CALLERS`、promotion 或 release 变更。

---

## 4. A0 Canonical Authority（当前活动阶段）

### 4.1 目标

A0 建立单一、确定性、可验证的 capability/evidence/PR-stack/integration authority，消除 PR body、机器状态和实时 Git 事实之间的漂移。

交付物：

1. `HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json`；
2. `HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json`；
3. `HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json`；
4. immutable Q0 evidence summary；
5. `hepta-intelligence-current-truth.py` 单命令输出；
6. fail-closed cross-verifier；
7. exact-parent changed-path allowlist；
8. independent read-only A0 workflow artifact。

A0 不接线、不执行 mutation、不新增数据库 schema、不注册 tool/model、不改变 default recall。所有 authority flag 必须为 false。

### 4.2 current-truth 输出

单命令必须产生 canonical、sorted、content-free JSON，至少包含：

```text
plan identity and digest
active phase/task
Q0 candidate/head/tree/run/artifacts
capability lifecycle
canonical PR stack
side stacks and external dependencies
integration candidate/rollback base
open/closed/external gaps
authority flags
next unblocked action
```

相同输入连续两次输出必须 byte-identical；任何未知 capability、重复 PR、错误 ancestry、digest drift、positive authority 或 register mismatch 必须失败。

### 4.3 A0 DoD

- candidate 必须是 `c768bcb...` 的恰好一个子提交；
- changed paths 必须等于 allowlist；
- master/current/registries 相互一致；
- Q0 summary 绑定 run/job/runner/artifact/digest；
- current-truth 两次 byte-identical；
- workflow 使用真实 runner 和非空 steps；
- artifact 可读并绑定 exact head；
- `a0_candidate_qualified` 只能由外部 exact-head receipt 得出；
- independent review 未完成前保持 Draft；
- no self-merge。

---

## 5. PR Stack 与 integration candidate

canonical Q0 stack：

```text
#7  P0.1 grounding contract
#13 P0.2 durable grounding ledger
#14 P0.3 grounded tool/shadow gate
#16 P0.4a typed mutation state
#21 P0.4b SQLite journal/failpoints
#23 P0.4c shadow host
#29 Q0 plan/compatibility/exact evidence carrier
```

P0.3.2/#40、P0.3.3/#30、P0.3.4/#64、P1.1a/#28、P1.1b/#34、P1.1c/#45、reviewed corpus/#49、efficacy rerun/#54、trusted intake/#58 都是 side stacks；除非明确 restack 到 selected qualified integration base 并重新运行完整 gates，否则不得自动纳入 canonical candidate。PR #53 仅作为 `EXTERNAL_UNMERGED_DEPENDENCY`，不授予本计划 authority。

下一阶段必须建立真正的 `IntegrationCandidateManifestV1`，绑定：

```text
base/head/tree/parent
ordered PR/commit stack
changed paths and file digests
dependency/feature graph
selected side-stack decisions
required full-repository checks
tranche receipts
runtime/authority flags
synthetic merge candidate
rollback base
```

source-head evidence与merge-candidate evidence不得互换。

---

## 6. 目标架构与 B0 boundary extraction

避免 `codex-hepta-memory` 继续演化为 god crate。B0 在 A0 资格化和 integration selection 后拆分边界：

```text
hepta-intelligence-contracts
hepta-grounding
hepta-mutation-core
hepta-mutation-journal
hepta-retrieval
hepta-policy-runtime
hepta-learning-ledger
hepta-intelligence-eval
```

qualification crates 必须成为生产实现的薄封装，禁止复制算法。每个 crate 需有 owner、public API、forbidden dependency、feature matrix、migration ownership、SLO 与 rollback contract。

共享模型策略采用 `shared frozen local encoder/backbone + small task heads/adapters`。在真实本地模型、tokenizer、artifact provenance、resource/energy receipts 与 efficacy 证据出现前，H5/H6 的 local model flag 保持 false。

---

## 7. 核心契约

必须逐步实现并版本化：

```text
RunStartSnapshotV2
LearningEpisodeV1
LearningEventV1
CandidateSetReceiptV1
NeuronSignalReceiptV2
PlasticityStateV1
PolicyDecisionReceiptV2
ExplorationPolicyReceiptV1
OutcomeReceiptV1
CreditLedgerV1
DatasetSnapshotV1
EvaluationReceiptV2
PolicyArtifactManifestV2
UnlearningComplianceReceiptV1
TopologyProposalV1
```

所有契约都需：bounded decoder、deny unknown fields、canonical serialization、schema/version、source/policy/model/tokenizer/dataset digests、privacy class、authority-negative flags、fuzz/property tests 与 migration story。

---

## 8. Mutation 原子闭环

现有 shadow journal 只能证明“观察到的 receipt 序列自洽”，不能证明真实 memory/KG/projection/outbox 副作用已发生。下一代 `IntelligenceMutationCoordinator` 必须由受信 host/store 生成事实 receipt：

```text
prepare
→ durable caller request
→ memory/KG transaction
→ mutation journal transition
→ transactional outbox intent
→ commit
→ asynchronous projection/send
→ store-derived acknowledgement
→ reconciliation
```

能位于同一数据库事务的写入必须原子提交；跨系统副作用采用 transactional outbox。普通 caller 不得自报 `MemoryWritten`、`ProjectionPublished` 或 `ReconcileApplied` digest。必须覆盖 pre-commit、post-commit ACK loss、retry/replay、lease drift、generation conflict、process kill、disk full、permission loss、bit corruption 和 recovery。

---

## 9. Journal 生命周期与恢复

append-only 不等于所有事件永久保留在一个在线表。生产 journal 必须增加：

- epoch/partition；
- terminal checkpoint；
- hash-chain/Merkle root；
- incremental recovery；
- background full scrub；
- verifiable archive；
- active/history separation；
- capacity admission/backpressure；
- recovery-time and storage-growth SLO；
- backup/restore rehearsal。

任何 checkpoint 都不得破坏 exact replay、correction、forget、revocation 或 audit provenance。

---

## 10. R1 Grounded Retrieval 与真实 efficacy

P1 qualification hash provider 不是语义模型；synthetic seed 不是真实 efficacy。进入产品前必须具备：

1. 真实本地 tokenizer 与 embedding/reranker artifact；
2. 模型、tokenizer、dimension、quantization、index generation 精确绑定；
3. reviewed multilingual corpus；
4. reviewer/adjudicator independence、license、provenance、privacy、redaction 与签名；
5. lexical/vector/KG/full rerank ablation；
6. Recall@k、nDCG@k、citation precision、contradiction、P50/P95、token/resource/energy；
7. locale/task/risk/privacy subgroup；
8. blind human review 与 shadow traffic；
9. regression corpus 与 rollback。

真实 corpus/reviewer/hardware 缺失时状态必须是 `BLOCKED_EXTERNAL_EVIDENCE`，不得由 fixture-only positive path 代替。

---

## 11. H5 Hepta Neuron

claim ladder：

```text
N0_METAPHORICAL_TYPED_PROPOSAL
→ N1_ADAPTIVE_SIGNAL_UNIT
→ N2_TEMPORAL_RECURRENT_SIGNAL_NETWORK
→ N3_ISOLATED_NEUROMORPHIC_RESEARCH
```

### N1

实现候选生成、sparse activation、lateral inhibition、homeostasis、bounded eligibility trace、calibrated confidence、abstention/OOD、adapter identity 和 `NeuronSignalReceiptV2`。参数更新只能生成 next-snapshot proposal，不能直接覆盖当前 artifact。

### N2

要求 temporal state、recurrent dynamics、neuromodulatory signal、lesion/ablation、graceful degradation、跨时间 retention 和可重复训练。必须明确这仍是 functional bio-inspired mechanism，不宣称复制生物神经元。

### N3

仅在隔离研究环境探索 event-driven/spiking/neuromorphic hardware；不得直接成为默认 production path。

---

## 12. H6 Hepta Intuition

claim ladder：

```text
I0_DETERMINISTIC_SELECTIVE_POLICY
→ I1_CALIBRATED_FAST_POLICY
→ I2_LONGITUDINALLY_VALIDATED_FAST_POLICY
```

I1 必须输出 `PolicyDecisionReceiptV2`，包含完整 candidate set、propensity、confidence、OOD、abstain/veto、risk class、policy/model artifact 与 latency/resource receipts。高风险 action 始终走慢路径或拒绝。

I2 必须通过长期 calibration、subgroup、retention、false-memory、correction 与 rollback 证明。不得以“human-like intuition”作为验收标准。

---

## 13. H7 长期学习闭环

long-term storage 不等于 long-term learning。H7 的唯一 durable causal spine 为：

```text
LearningEpisodeV1
  context snapshot
  complete candidates
  selected action
  logged propensity/support
  effect receipt
  postcondition
  delayed outcome
  correction/forget/revocation state
  credit assignment
  policy/model/dataset identities
```

评价采用 support coverage、ESS、IPS、SNIPS、DR、cluster/bootstrap CI、candidate LCB 与 baseline UCB。晋级规则必须是：

```text
candidate LCB > baseline UCB
```

同时满足 safety、subgroup、retention、privacy、resource 与 rollback gates。参数、policy 或 topology 更新只能生成签名的 **next-snapshot** artifact；当前运行快照永不原地自改。

closed loop 最初仅允许 `MemoryRetrievalRank` 等低风险只读位置，禁止 tool/provider/auth/credential/topology/high-risk action 作为首个闭环。

---

## 14. Security、privacy 与 unlearning

威胁模型覆盖 prompt injection、memory poisoning、citation laundering、embedding inversion、membership inference、artifact substitution、receipt replay、cross-agent confusion、privilege escalation 和 side-channel leakage。

最小控制：

- source trust/grounding 与 truth status 分离；
- secret/PII redaction before persistence；
- tenant/agent/scope binding；
- signed artifact and revocation；
- training eligibility 独立于 retrieval eligibility；
- high-risk abstention/veto；
- no raw content in telemetry；
- key rotation/expiry/nonces/replay domain。

`UnlearningComplianceReceiptV1` 必须绑定 correction、forget、revocation 到所有派生 dataset、index、adapter、policy、evaluation 与 artifact。不可变审计保留无原文 digest/provenance；应删除内容通过 tombstone 或 crypto-shredding 失效。

---

## 15. 验证矩阵

### Source/contract

- schema/required/forbidden fields；
- digest/current pointer；
- single-plan uniqueness；
- registry cross references；
- exact parent/path allowlist；
- unknown positive authority fail-closed；
- deterministic current-truth。

### Rust/product

- fmt、focused/full tests、strict Clippy；
- feature default-off 与 no-cycle；
- qualification wrapper directly uses production implementation；
- product workspace feature matrix；
- cross-platform deterministic fixtures；
- fuzz/property/adversarial tests。

### Runtime/recovery

- process kill and restart；
- `BEGIN IMMEDIATE` contention；
- pre/post-commit failpoints；
- WAL/checkpoint/backup/restore；
- disk full/permission loss/page corruption；
- bounded startup/recovery/growth；
- archive verification。

### Learning/efficacy

- true candidate support and propensity；
- missing/censored outcome handling；
- IPS/SNIPS/DR/ESS/CI；
- subgroup and safety no-regression；
- 1d/7d/30d/90d retention；
- correction/forget/revocation；
- latency/resource/energy；
- negative controls and leakage tests。

---

## 16. Roadmap 与依赖

```text
Q0 Qualification Debt Closure              // exact candidate complete
→ A0 Canonical Capability/Evidence Authority // current active
→ B0 Learning Boundary Extraction
→ C0 Durable LearningEpisode Ledger
→ R1 Grounded Retrieval/Telemetry/Security
→ N1 H5 Adaptive Signal Unit
→ I1 H6 Calibrated Fast Policy
→ L1 H7 Causal Evaluation/Artifact
→ C1 Low-risk Closed Loop
→ N2 Temporal Recurrent Signal Network
→ S1 Governed Structural Plasticity
→ N3 Isolated Neuromorphic Research
```

每个阶段使用独立 Draft PR、exact qualified parent、changed-path allowlist、immutable receipt 和 rollback base。side stack 必须先 restack/rebase 到唯一 selected integration candidate 并重新资格化。

---

## 17. Pilot 阈值

低风险 pilot 的最低预注册建议：

```text
minimum episodes >= 5,000
supported episodes >= 2,000
ESS >= 500
support coverage >= 80%
clipped ratio <= 5%
candidate LCB improvement >= pre-registered threshold
critical safety regression = 0
false-memory attachment not worse than baseline
p95 overhead <= 20% of baseline or bounded absolute budget
7d retention before canary expansion
30d retention before broad promotion
```

阈值必须在结果前冻结，不能事后选择。

---

## 18. Definition of Done

### A0 DoD

- registries 与 current truth 相互一致；
- Q0 exact evidence 被完整绑定；
- workflow 在 real runner 上执行 non-empty steps；
- artifact exact-head 可验证；
- all authority=false；
- independent review；
- no self-merge；
- full-repository merge-green 仍独立要求。

### Runtime capability DoD

- selected integration candidate；
- explicit wiring；
- E3 runtime/restart/failpoint evidence；
- full product checks；
- bounded recovery and rollback；
- no hidden authority；
- operator acceptance 独立签名。

### L2 closed-loop DoD

- durable causal episodes；
- complete candidate/propensity/support；
- delayed outcomes and credit conservation；
- OPE/ESS/CI/subgroup/retention；
- signed next-snapshot artifact；
- shadow/canary and rollback rehearsal；
- independent `CALLERS` promotion receipt。

---

## 19. 最终执行顺序

```text
1. 固化 Q0 paired evidence summary
2. 完成 A0 registries/current-truth/verifiers/workflow
3. 获取 A0 exact-head executable artifact
4. 独立 review，禁止 self-merge
5. 选择唯一 integration candidate
6. 修复该候选全部 required checks
7. 启动 B0 boundary extraction
8. 实现 C0 LearningEpisode ledger
9. 完成 R1 real retrieval corpus/evaluation
10. 依次推进 N1 → I1 → L1 → C1
11. 只有 L2 DoD 完整后才允许 N2/S1
12. N3 永久隔离研究
```

任何阶段都不得自行把 `candidate_qualified` 提升为 runtime、efficacy、operator、promotion 或 production authority。缺少真实外部输入的 gap 必须保持 `BLOCKED_EXTERNAL_EVIDENCE`，不能以 fixture 或文案闭合。
