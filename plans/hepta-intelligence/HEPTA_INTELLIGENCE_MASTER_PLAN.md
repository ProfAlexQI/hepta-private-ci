# Hepta Intelligence Master Development Plan

> **CANONICAL_CURRENT / PLAN_ONLY / FAIL_CLOSED**
>
> Plan ID: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4`
>
> Version: `4.2.0`
>
> Repository: `ProfHepta/hepta-private-ci`
>
> Current program phase: `A0_CANONICAL_CAPABILITY_AND_EVIDENCE_AUTHORITY`
>
> Current capability claims: `L0 / N0 / I0`
>
> 本文件是**唯一有效的人类可读开发计划**。任何 PR 正文、旧 status、workflow 日志、artifact、fixture、Draft 分支或本文件的附属执行规范都不得单独替代它。

---

## 0. 权威读取顺序与事实分层

每次开发、审计、资格化、恢复或晋级必须依次读取：

```text
HEPTA_INTELLIGENCE_CURRENT_PLAN.json
→ HEPTA_INTELLIGENCE_DOCUMENT_AUTHORITY_REGISTRY_V1.json
→ HEPTA_INTELLIGENCE_EVIDENCE_INDEX_V1.json
→ HEPTA_INTELLIGENCE_CAPABILITY_REGISTRY_V1.json
→ HEPTA_INTELLIGENCE_PR_STACK_REGISTRY_V1.json
→ HEPTA_INTELLIGENCE_INTEGRATION_CANDIDATE_V1.json
→ HEPTA_INTELLIGENCE_MASTER_PLAN.md
→ HEPTA_INTELLIGENCE_CONTROLLED_GAP_CLOSURE_EXECUTION_SPEC_V1.md
```

`CURRENT_PLAN` 是唯一聚合机器权威；本文件是唯一 current human plan。registry、执行规范、PR 正文、CI 日志与 artifact 均没有 production、operator、promotion、release 或 `CALLERS` authority。

事实必须分成两层：

```text
SOURCE_SNAPSHOT
  checked-in, deterministic, exact-tree-bound
  只表达最后一次合法发布时已知的事实

LIVE_EVIDENCE
  GitHub API / runner / artifact 的实时观察
  必须在 receipt 中绑定 repository/head/tree/run/job/runner/steps/artifact
```

source snapshot 不得伪装成实时 CI。实时观察不得直接改写 canonical source。任何 repository、branch、head、tree、parent、schema、digest、registered consumer、authority flag、read order 或 evidence identity 不一致都必须 `FAIL_CLOSED`。

不得把 queued、`steps=[]`、`runner_id=0`、source-only gate、过期 artifact、synthetic merge ref 或 PR prose 解释为 executable qualification。

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

`qualified_candidate=true` 只说明 exact source candidate 的双架构 admission matrix 已通过；它不等于产品 capability 已 wired、runtime-qualified、efficacy-proven、operator-accepted 或 promoted。

### 1.2 A0 current source candidate

A0 必须始终是 Q0 head 的**恰好一个直接子提交**。修复不得追加第二个 A0 commit；必须生成 replacement tree、创建 sole-parent commit 并原子移动 A0 branch。每次 replacement 都使旧 head 的 evidence 失效。

A0 当前工作单元：

```text
A0.3_REPLACE_BOT_HEAD_AND_OBTAIN_EXACT_HEAD_EXECUTABLE_EVIDENCE
```

A0 只允许 documentation、registry、verifier 和 read-only workflow。Rust runtime、SQL migration、product caller、model/provider dispatch、H5/H6/H7 runtime、`CALLERS`、promotion 与 release 均冻结。

### 1.3 全仓边界

Q0/A0 专项绿灯与 default-branch merge 资格分离。历史候选曾出现 Bazel、cargo-deny、cargo-shear、repo-checks、macOS/Windows build/lint 等失败或未完成项。因此：

```text
full_repository_merge_green=false
merge_candidate_qualified=false
production_candidate=false
```

只能在 canonical selection 后，通过 `RepositoryCheckAttributionReceiptV1` 对选定 synthetic merge candidate 进行 attribution：区分本 tranche 引入、base 已存在、runner/platform 基础设施和取消/过期状态。不得把无关全仓失败伪装成 Hepta Intelligence runtime failure，也不得忽略真实归因失败。

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

禁止跳级。source/test receipt 最多推进 `candidate_qualified`。真实 product graph、E3 runtime/restart/failpoint、E4 efficacy/retention/resource、operator acceptance、rollback rehearsal 与独立 `CALLERS` receipt 才能推进后续状态。

---

## 3. Gap 分类、闭合规则与恢复协议

每个 gap 必须具有：

```text
gap_id
classification
owner_class
status
exact dependencies
closure evidence schema
next action
authority effect
resume predicate
```

合法分类：

- `CLOSED_SOURCE_CONTROLLED`
- `OPEN_SOURCE_CONTROLLED`
- `BLOCKED_EXTERNAL_EVIDENCE`
- `BLOCKED_UPSTREAM`
- `STOP_CONDITION`

外部证据缺失不得用 fixture、随机数据、测试密钥、模拟 reviewer 或自签 operator 伪造闭合。阻塞必须落盘为可机器读取的 required-input contract。

每个工作单元退出时必须生成 `PackageHandoffReceiptV1`：

```text
repository / branch / head / tree / parent
changed-path manifest
commands and exits
artifact and receipt digests
open/closed/external gaps
authority flags
resume predicate
rollback pointer
```

---

## 4. A0 Canonical Authority（当前活动阶段）

### 4.1 目标

A0 建立单一、确定性、可验证的 capability/evidence/PR-stack/integration authority，消除 PR body、机器状态、checked-in snapshot 和实时 Git 事实之间的漂移。

交付物：

1. capability registry；
2. PR-stack registry；
3. integration candidate 与 gap ledger；
4. immutable Q0 evidence summary；
5. deterministic current-truth command；
6. fail-closed cross-verifiers；
7. exact-parent changed-path allowlist；
8. source snapshot/live evidence 分层；
9. package-level implementation blueprint；
10. independent read-only exact-head artifacts。

### 4.2 A0 DoD

- candidate 是 `c768bcb...` 的恰好一个直接子提交；
- changed paths 与 allowlist 完全相等；
- master/current/registries/spec 相互一致；
- Q0 summary 绑定 run/job/runner/artifact/digest；
- current-truth 连续两次 byte-identical；
- workflow 使用真实 runner 和非空成功 steps；
- artifact 可读、未过期、唯一且绑定 exact head；
- source snapshot 不声明 live pass；
- `a0_candidate_qualified` 只能由外部 exact-head executable receipt 得出；
- independent review 与 canonical selection 独立完成；
- no self-merge；
- all authority=false。

---

## 5. PR stack、side stack 与 integration selection

Canonical Q0 stack：

```text
#7  P0.1 grounding contract
#13 P0.2 durable grounding ledger
#14 P0.3 grounded tool/shadow gate
#16 P0.4a typed mutation state
#21 P0.4b SQLite journal/failpoints
#23 P0.4c shadow host
#29 Q0 plan/compatibility/exact evidence carrier
```

Side-stack 初步处理决策：

| PR | 决策 | 可复用内容 | 不可直接继承内容 |
|---|---|---|---|
| #40 | `INCLUDE_AFTER_RESTACK` | deterministic projection planner、same-snapshot proof | 原分支 ancestry 与独立 authority |
| #30 | `INCLUDE_AFTER_RESTACK` | host-owned evidence resolution、UTF-8 range binding | 原 exact-head receipt 只能作 provenance |
| #64 | `SPLIT_AND_REWRITE` | legacy inventory/quarantine schema | 未完成 staging payload |
| #28 | `REWRITE_AS_PRODUCT_MODULE` | query/fusion contract | integration-test-only source isolation |
| #34 | `SPLIT_PROVIDER_INDEX_CONTRACTS` | bounded provider/index contracts | hash-one-hot semantic claim |
| #45 | `RETAIN_EVAL_MECHANICS_ONLY` | multilingual ablation harness | synthetic efficacy claim |
| #49 | `RETAIN_GOVERNANCE_MECHANICS_ONLY` | reviewer/adjudication validation | fixture reviewer fact |
| #54 | `RETAIN_BLOCKED_RERUN_MECHANICS` | complete coverage gate | 8/48 fixture projection |
| #58 | `RETAIN_TRUST_GATE_MECHANICS` | signature/trust-store validation | test keys/external acceptance |

每个 selected side stack 必须 restack 到唯一 selected integration base，生成 changed-file/dependency/behavior manifest，重跑完整 source/Rust/product/security/repository checks，并保持 production authority=false。

未来 `IntegrationCandidateManifestV1` 必须绑定：

```text
base/head/tree/parent
ordered commits and selected side-stack decisions
changed paths and file digests
Cargo/Bazel/feature dependency graph
required repository checks and attribution
tranche receipts
runtime/authority flags
synthetic merge candidate
rollback base
```

source-head evidence 与 merge-candidate evidence 不得互换。

---

## 6. B0 package boundary extraction

目标路径与 crate：

```text
codex-rs/hepta-intelligence-contracts    / codex-hepta-intelligence-contracts
codex-rs/hepta-grounding                 / codex-hepta-grounding
codex-rs/hepta-mutation-core             / codex-hepta-mutation-core
codex-rs/hepta-mutation-journal          / codex-hepta-mutation-journal
codex-rs/hepta-mutation-coordinator      / codex-hepta-mutation-coordinator
codex-rs/hepta-retrieval                 / codex-hepta-retrieval
codex-rs/hepta-policy-runtime            / codex-hepta-policy-runtime
codex-rs/hepta-learning-ledger           / codex-hepta-learning-ledger
codex-rs/hepta-intelligence-eval         / codex-hepta-intelligence-eval
```

允许依赖 DAG：

```text
contracts
├─ grounding
├─ mutation-core
├─ learning-ledger
├─ retrieval ─→ grounding
├─ mutation-journal ─→ mutation-core
├─ policy-runtime ─→ retrieval
├─ mutation-coordinator ─→ grounding + mutation-core + mutation-journal
└─ intelligence-eval ─→ retrieval + policy-runtime + learning-ledger
```

禁止反向依赖、循环依赖、eval/runtime 写 authority 和 qualification 算法复制。qualification workspace 只能做薄封装并直接调用生产实现。

共享模型策略固定为 `shared frozen local encoder/backbone + small task heads/adapters`；真实 artifact provenance、资源和 efficacy 证据缺失时，本地模型能力标志保持 false。

B0 提取顺序：

```text
B0.1 contracts
→ B0.2 pure mutation core
→ B0.3 grounding
→ B0.4 journal adapter
→ B0.5 retrieval contracts
→ B0.6 learning ledger contracts
→ B0.7 policy/eval boundaries
→ B0.8 Cargo/Bazel parity and removal of duplicate implementations
```

每一步独立 Draft PR、exact qualified parent、default-off feature、rollback pointer 和 no-behavior-change receipt。

---

## 7. 核心契约及统一工程标准

必须版本化：

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

统一要求：

- `deny_unknown_fields`；
- canonical serialization；
- domain-separated SHA-256；
- schema/version/migration/downgrade；
- owner/tenant/agent/run/episode binding；
- source/model/tokenizer/policy/dataset/code identity；
- privacy、retention、retrieval/training eligibility；
- maximum encoded bytes 与 collection cardinality；
- authority-negative fields；
- fuzz/property/adversarial tests；
- producer 必须是观察事实的组件，而不是从该声明获益的 caller。

字段级表、精确上限、producer/consumer、error taxonomy、命令与 receipt 在附属执行规范的 **Implementation Blueprint** 中定义。

---

## 8. C0 durable causal learning ledger

Long-term storage 不等于 long-term learning。唯一 durable causal spine：

```text
RunStartSnapshotV2
→ LearningEpisodeV1
→ CandidateSetReceiptV1
→ PolicyDecisionReceiptV2
→ effect/postcondition
→ OutcomeReceiptV1
→ CreditLedgerV1
→ DatasetSnapshotV1
→ EvaluationReceiptV2
→ PolicyArtifactManifestV2
```

每个 episode 必须绑定完整 candidates、logged propensity/support、context snapshot、selected action、policy/model/tokenizer/dataset identity、effect receipt、postcondition、delayed/censored outcome、correction/forget/revocation、credit 和 privacy/training eligibility。

参数、policy、adapter 或 topology 更新只能生成签名的 **next-snapshot** artifact；当前运行快照永不原地自改。

---

## 9. M0 mutation 原子闭环

现有 shadow journal 只能证明“观察到的 receipt 序列自洽”，不能证明真实 memory/KG/projection/outbox 副作用已发生。`IntelligenceMutationCoordinator` 必须由受信 host/store 生成事实 receipt：

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

同库写入必须原子提交；跨系统副作用使用 transactional outbox。普通 caller 不得自报 `MemoryWritten`、`ProjectionPublished` 或 `ReconcileApplied` digest。

必须覆盖 pre-commit、post-commit ACK loss、retry/replay、lease drift、generation conflict、process kill、disk full、permission loss、page/bit corruption、duplicate delivery、owner confusion 和 recovery。

---

## 10. J0 journal 生命周期与恢复

生产 journal 必须增加：

- active/history epoch partition；
- terminal checkpoint；
- hash-chain/Merkle checkpoint；
- incremental startup recovery；
- background full scrub；
- verifiable archive/restore；
- capacity admission/backpressure；
- corruption quarantine；
- backup/restore rehearsal；
- recovery-time 与 storage-growth SLO。

append-only 不等于所有事件永久保留在同一个在线表。checkpoint/archival 不得破坏 exact replay、correction、forget、revocation 或 audit provenance。

---

## 11. R1 grounded retrieval 与真实 efficacy

进入产品前必须具备：

1. 真实本地 tokenizer、embedding/reranker artifacts；
2. model/tokenizer/dimension/quantization/index generation 精确绑定；
3. host-owned evidence range resolution；
4. grounded/truth/risk filter before scoring；
5. bounded lexical/vector/KG candidates 和 deterministic fusion；
6. reviewed multilingual corpus；
7. reviewer/adjudicator independence、license、provenance、privacy、redaction、signatures；
8. lexical/vector/KG/full rerank ablation；
9. Recall@k、nDCG@k、citation precision、contradiction、P50/P95/P99、token/resource/energy；
10. locale/task/risk/privacy subgroup；
11. blind human review、shadow traffic、regression corpus 与 rollback。

Qualification hash provider 不是语义模型；synthetic seed 不是真实 efficacy。真实 corpus/reviewer/model/hardware 缺失时必须 `BLOCKED_EXTERNAL_EVIDENCE`。

---

## 12. H5 Hepta Neuron

Claim ladder：

```text
N0_METAPHORICAL_TYPED_PROPOSAL
→ N1_ADAPTIVE_SIGNAL_UNIT
→ N2_TEMPORAL_RECURRENT_SIGNAL_NETWORK
→ N3_ISOLATED_NEUROMORPHIC_RESEARCH
```

N1 要求 sparse activation、lateral inhibition、homeostasis、bounded eligibility trace、calibrated confidence、abstention/OOD、frozen backbone/adapter identity、next-snapshot proposal、lesion/ablation 和 rollback。

N2 要求 temporal state、recurrent dynamics、neuromodulatory signal、跨时间 retention 与 graceful degradation。它仍只能声明 functional bio-inspired mechanism，不能声称复制生物神经元。

N3 永久隔离于研究环境，不得由 ancestry 自动成为 production path。

---

## 13. H6 Hepta Intuition

Claim ladder：

```text
I0_DETERMINISTIC_SELECTIVE_POLICY
→ I1_CALIBRATED_FAST_POLICY
→ I2_LONGITUDINALLY_VALIDATED_FAST_POLICY
```

I1 的 `PolicyDecisionReceiptV2` 必须包含 complete candidate set、propensity、confidence、OOD、abstain/veto、risk class、policy/model artifact、latency/resource。高风险 action 始终 abstain、reject 或转慢路径。

I2 必须通过长期 calibration、subgroup、retention、false-memory、correction、unlearning 和 rollback。不得以“human-like intuition”作为验收标准。

---

## 14. H7 evaluation、promotion 与低风险闭环

评价采用：

```text
support coverage
ESS
IPS
SNIPS
doubly robust
cluster/bootstrap CI
candidate LCB
baseline UCB
subgroup/safety/retention/privacy/resource gates
```

晋级必要条件：

```text
candidate LCB > baseline UCB
```

首个 closed loop 只允许可逆、只读的 `MemoryRetrievalRank`；tool/provider/auth/credential/topology/high-risk action 不得作为首个闭环位置。

C1 需要 shadow replay、canary limits、kill switch、frozen baseline、rollback rehearsal、delayed outcome、credit conservation、independent operator approval 与独立 `CALLERS` receipt。

---

## 15. Security、privacy 与 unlearning

威胁模型覆盖 prompt injection、memory poisoning、citation laundering、embedding inversion、membership inference、artifact substitution、receipt replay、cross-agent confusion、privilege escalation、malicious corpus intake 和 telemetry leakage。

最小控制：

- trust、grounding 与 truth 分离；
- secret/PII redaction before persistence；
- tenant/agent/scope binding；
- signed/scoped/expiring/revocable artifacts；
- retrieval eligibility 与 training eligibility 分离；
- high-risk abstention/veto；
- no raw content in telemetry；
- nonce/replay domain；
- key rotation/expiry/revocation；
- derived dataset/index/adapter/policy/evaluation 的 unlearning lineage。

`UnlearningComplianceReceiptV1` 绑定 correction、forget、revocation 到所有派生 artifact。不可变审计只保留无原文 digest/provenance；删除内容通过 tombstone 或 crypto-shredding 失效。

---

## 16. 初始设计上限与 SLO 预注册

以下是进入实现前的默认设计上限，不是性能达标声明；任何调整必须在测量结果前通过 plan receipt：

```text
single contract encoded bytes          <= 128 KiB
LearningEpisode aggregate bytes        <= 1 MiB
candidate inventory                    <= 1,024
scored candidates                      <= 128
returned candidates                    <= 32
KG hops                                <= 2
KG nodes / edges                       <= 256 / 1,024
policy decision deadline               <= 50 ms local p95 design budget
retrieval added latency                <= 20% baseline or frozen absolute budget
journal active partition               <= 8,000 operations before rollover
single operation transitions           <= 64
incremental recovery target            <= 5 s per 10,000 active operations
critical safety regression             = 0
```

真实资源、能耗、硬件与 soak 仍是 E4 外部证据。

---

## 17. 验证矩阵

### Source/contract

- schema/required/forbidden fields；
- digest/current pointer；
- single-plan uniqueness；
- registry cross references；
- exact parent/path allowlist；
- source snapshot/live evidence separation；
- unknown positive authority fail-closed；
- deterministic current-truth。

### Rust/product

- fmt、focused/full tests、strict Clippy；
- Cargo/Bazel parity；
- feature default-off、no-cycle、unused dependency；
- qualification wrapper directly uses production implementation；
- product workspace feature matrix；
- cross-platform deterministic fixtures；
- fuzz/property/adversarial tests。

### Runtime/recovery

- real process kill/restart；
- `BEGIN IMMEDIATE` contention；
- pre/post-commit failpoints；
- WAL/checkpoint/backup/restore；
- disk full/permission loss/page corruption；
- duplicate delivery、clock regression、owner confusion；
- bounded startup/recovery/growth；
- archive verification。

### Learning/efficacy

- true candidate support and propensity；
- missing/censored outcome handling；
- IPS/SNIPS/DR/ESS/CI；
- subgroup/safety no-regression；
- 1d/7d/30d/90d retention；
- correction/forget/revocation；
- latency/token/memory/CPU/GPU/energy；
- negative controls、leakage tests 和 rollback rehearsal。

Evidence classes：

```text
E0 source/static
E1 local executable
E2 independent runner/platform
E3 runtime/restart/failpoint
E4 soak/retention/efficacy/energy/operator
```

---

## 18. Roadmap 与 work-unit gates

```text
Q0 Qualification Debt Closure                 // exact candidate complete
→ A0 Canonical Capability/Evidence Authority   // current active
→ B0 Package Boundary Extraction
→ C0 Durable LearningEpisode Ledger
→ M0 Transactional Mutation Coordinator
→ J0 Journal Lifecycle/Recovery
→ R1 Grounded Retrieval/Telemetry/Security
→ N1 H5 Adaptive Signal Unit
→ I1 H6 Calibrated Fast Policy
→ L1 H7 Causal Evaluation/Artifact
→ C1 Low-risk Closed Loop
→ N2 Temporal Recurrent Signal Network
→ S1 Governed Structural Plasticity
→ N3 Isolated Neuromorphic Research
```

每阶段必须具有 exact qualified parent、changed-path allowlist、commands、expected receipts、E-level、rollback pointer、stop condition 和 handoff receipt。side stack 必须先 restack 到 selected integration candidate 并重新资格化。

---

## 19. Pilot 阈值

最低预注册建议：

```text
minimum episodes >= 5,000
supported episodes >= 2,000
ESS >= 500
support coverage >= 80%
clipped ratio <= 5%
candidate LCB improvement >= frozen threshold
critical safety regression = 0
false-memory attachment not worse than baseline
p95 overhead <= 20% baseline or bounded absolute budget
7d retention before canary expansion
30d retention before broad promotion
```

阈值必须在结果前冻结，不能事后选择。

---

## 20. Definition of Done

### A0 DoD

- registries、current truth、master、spec 相互一致；
- Q0 exact evidence 完整绑定；
- real runner/non-empty successful steps；
- exact-head unique unexpired artifacts；
- source snapshot/live evidence 分离；
- all authority=false；
- independent review；
- no self-merge；
- full-repository merge-green 独立要求。

### Runtime capability DoD

- selected integration candidate；
- explicit wiring；
- E3 runtime/restart/failpoint；
- full product checks；
- bounded recovery/rollback；
- no hidden authority；
- independent operator signature。

### L2 closed-loop DoD

- durable causal episodes；
- complete candidates/propensity/support；
- delayed outcomes/credit conservation；
- OPE/ESS/CI/subgroup/retention；
- signed next-snapshot artifact；
- shadow/canary/rollback；
- independent `CALLERS` promotion receipt。

---

## 21. 最终执行顺序

```text
1. 固化 Q0 paired evidence summary
2. 深化并资格化 A0 master/spec/registries/current-truth
3. 获取 A0 exact-head executable artifact
4. 独立 review 与 canonical selection；禁止 self-merge
5. 构建唯一 integration candidate 和 check attribution
6. 修复该候选全部 repository-controlled required checks
7. 启动 B0 package extraction
8. 实现 C0 LearningEpisode ledger
9. 实现 M0 coordinator 与 J0 recovery
10. 集成 R1 real retrieval
11. 依次推进 N1 → I1 → L1 → C1
12. 只有 longitudinal DoD 完整后才允许 N2/S1
13. N3 永久隔离研究
```

任何阶段都不得自行把 `candidate_qualified` 提升为 runtime、efficacy、operator、promotion 或 production authority。缺少真实外部输入的 gap 必须保持 `BLOCKED_EXTERNAL_EVIDENCE`。
