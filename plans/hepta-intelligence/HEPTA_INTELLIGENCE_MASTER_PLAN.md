# Hepta Intelligence Master Development Plan

> **CANONICAL_CURRENT / PLAN_ONLY / FAIL_CLOSED**
>
> Plan ID: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4`
>
> Version: `4.3.0`
>
> Repository: `ProfHepta/hepta-private-ci`
>
> Current program phase: `A0_CANONICAL_CAPABILITY_AND_EVIDENCE_AUTHORITY`
>
> Current capability claims: `L0 / N0 / I0`
>
> 本文件是**唯一有效的人类可读开发计划**。任何 PR 正文、旧 status、workflow 日志、artifact、fixture、Draft 分支或附属执行规范都不得单独替代本文件，也不得授予 runtime、operator、production、promotion、release 或 `CALLERS` authority。

---

## 0. V4.3 变更摘要

V4.3 在 V4.2 的安全与证据框架之上，补齐此前仍然偏“路线图化”的部分，使后续每个工作包都能由不同 agent 按同一契约独立实现、验证、交接和恢复。

本版本新增或深化：

1. **状态漂移闭合规则**：checked-in `SOURCE_SNAPSHOT`、外部 `LIVE_EVIDENCE`、PR body、Git ref 与 synthetic merge candidate 的优先级和失效规则；
2. **A0 独立评审与 canonical selection 契约**：明确实现者、publisher、reviewer、selector、operator 五类身份不能互相替代；
3. **RepositoryCheckAttributionReceiptV1**：逐 check 归因、base/head/merge-candidate 三向比较、取消/过期/基础设施状态的处理；
4. **B0 九包边界的字段级 API/依赖/迁移计划**；
5. **C0/M0/J0 的存储所有权、事务边界、replay、checkpoint、归档和 E3 failure campaign**；
6. **R1 真实语义 artifact、host-owned evidence、产品 feature matrix 与多语言 efficacy 证据链**；
7. **N1/I1/L1/C1 的训练、propensity、credit、OPE、abstention、next-snapshot、canary 与 rollback 契约**；
8. **跨包 Definition of Ready / Definition of Done**；
9. **每个 blocker 的 stop/resume predicate、可伪造性边界和 closure evidence class**；
10. **开发顺序硬门**：A0 未完成 review/selection/merge admission 时，禁止 B0/runtime-adjacent source 落盘。

本版本仍不改变任何产品行为；`self_evolution=false`、`closed_loop_learning=false`、`neuromorphic_mechanism=false`。

---

## 1. 权威读取顺序与事实模型

每次开发、审计、资格化、恢复、restack、晋级或回滚必须依次读取：

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

事实分层：

```text
SOURCE_SNAPSHOT
  checked-in、deterministic、tree-bound
  只表达发布时已知且经 source publisher 落盘的事实

LIVE_EVIDENCE
  GitHub API / runner / artifact / external reviewer 的实时事实
  必须由 receipt 绑定 exact repository/branch/head/tree/parent/run/job/
  runner/steps/artifact/digest/expiry

SELECTION_TRUTH
  由独立 selector 绑定一个 exact candidate 与 rollback base
  不得由实现者、CI workflow 或 PR prose 自行生成

RUNTIME_TRUTH
  由 E3/E4 证据、operator acceptance 与独立 CALLERS receipt 共同形成
  不得从 source qualification 推导
```

失效规则：

- head、tree、parent、workflow SHA、artifact digest、review target 任一变化，旧 evidence 立即 `STALE_SUPERSEDED`；
- PR head evidence 与 synthetic merge candidate evidence 不可互换；
- queued、pending、cancelled、`steps=[]`、`runner_id=0`、过期 artifact、source-only receipt 均不是 PASS；
- live API 观察不得直接改写 canonical source；
- 未知字段、未知 capability、未知 positive authority、未登记 side stack 均 fail closed；
- same commit 被移动到新 branch 时，branch-bound receipt 不自动复用。

---

## 2. 当前精确基线与 A0 边界

### 2.1 Q0 exact candidate

```text
repository = ProfHepta/hepta-private-ci
branch = codex/hepta-intelligence-plan-v3-20260828
head = c768bcbeb4c1168088d2499828c24da521a2a73a
tree = ca455a9ef797cd95164c880c7b8faba80b305589
parent = aeb8ac0bfb30d570a16c4914b6e4b31ce035dd62
run = 33252922404
E1 job = 99101597686
E2 job = 99101597800
pair job = 99105393694
qualified_candidate = true
runtime_capability_qualified = false
full_repository_merge_green = false
```

### 2.2 A0 replacement protocol

A0 candidate 必须是 Q0 head 的恰好一个直接子提交。任何文档、registry、verifier 或 read-only workflow 修复都必须：

```text
revalidate Q0 exact head/tree
→ build replacement tree
→ create one sole-parent commit(parent=Q0 head)
→ atomically move the A0 branch
→ invalidate all prior A0 evidence
→ run fresh exact-head source/finalizer workflows
→ obtain independent review
→ obtain canonical selection
→ qualify the selected synthetic merge candidate
```

A0 当前允许的变更面只有：

```text
DOCUMENTATION
REGISTRY
VERIFIER
READ_ONLY_WORKFLOW
```

禁止：

```text
Rust runtime source
SQL migration
product caller
model/provider dispatch
H5/H6/H7 runtime
CALLERS
promotion
release
self-review
self-approval
self-merge
```

### 2.3 A0 角色隔离

| 角色 | 允许 | 禁止替代 |
|---|---|---|
| implementer | 修改允许路径、生成 source candidate | reviewer、selector、operator |
| source publisher | 通过受认证 Git write 发布 exact candidate | CI evidence workflow |
| CI evidence workflow | read-only 执行与产出 artifact | source writeback、selection |
| independent reviewer | 评审 exact source/evidence，签发 Q1 | implementer/publisher |
| canonical selector | 选择唯一 integration candidate | 由 PR 作者自行选择 |
| operator | 接受 runtime/rollback/production 风险 | 由测试密钥或 fixture 替代 |

`最高权限` 不改变 separation-of-duty；它只允许执行 repository-controlled 工作，不允许伪造独立事实。

---

## 3. Capability 与 Claim Ladder

统一生命周期：

```text
implemented
→ candidate_qualified
→ selected
→ wired
→ runtime_qualified
→ efficacy_proven
→ operator_accepted
→ promoted
```

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

H5 ladder：

```text
N0_METAPHORICAL_TYPED_PROPOSAL
→ N1_ADAPTIVE_SIGNAL_UNIT
→ N2_TEMPORAL_RECURRENT_SIGNAL_NETWORK
→ N3_ISOLATED_NEUROMORPHIC_RESEARCH
```

H6 ladder：

```text
I0_DETERMINISTIC_SELECTIVE_POLICY
→ I1_CALIBRATED_FAST_POLICY
→ I2_LONGITUDINALLY_VALIDATED_FAST_POLICY
```

任何 source/test fixture 最多推进 `candidate_qualified`，不能推进 selected、wired、runtime-qualified 或 efficacy。

---

## 4. Integration Candidate 与 Repository Check Attribution

`IntegrationCandidateManifestV1` 必须绑定：

```text
repository/repository_id
base/head/tree/parent
ordered commit list
selected side-stack decisions
changed path + blob SHA + content SHA-256
Cargo/Bazel/feature dependency graph
toolchain and lock identities
required source/head/merge-candidate checks
tranche receipts and expiry
synthetic merge candidate
authority flags
rollback base
```

`RepositoryCheckAttributionReceiptV1` 每个 check 至少包含：

```text
check_name
workflow_path/workflow_sha
base_conclusion
head_conclusion
merge_candidate_conclusion
run_id/job_id/runner_id
steps_non_empty
first_failure_step
annotation_digest
log_digest
classification
owner_class
repair_commit
retest_run
```

合法 classification：

```text
PASS
INTRODUCED_BY_CANDIDATE
PRE_EXISTING_ON_BASE
MERGE_INTERACTION
RUNNER_OR_PLATFORM_INFRA
CANCELLED_OR_SUPERSEDED
NOT_REQUIRED_BY_SELECTED_POLICY
UNKNOWN_FAIL_CLOSED
```

规则：

- `UNKNOWN_FAIL_CLOSED` 阻塞 merge；
- target exclusion 必须由独立 policy authority 签名；
- 不得用专项 Q0/A0 绿灯覆盖全仓 required check；
- 不得把 base 已存在失败伪装成本 tranche runtime failure；
- 修复只允许针对选定 candidate，不得在未选择前无限追 side-stack。

---

## 5. B0 九包边界与所有权

目标 crate：

```text
codex-rs/hepta-intelligence-contracts
codex-rs/hepta-grounding
codex-rs/hepta-mutation-core
codex-rs/hepta-mutation-journal
codex-rs/hepta-mutation-coordinator
codex-rs/hepta-retrieval
codex-rs/hepta-policy-runtime
codex-rs/hepta-learning-ledger
codex-rs/hepta-intelligence-eval
```

允许 DAG：

```text
contracts
├─ grounding
├─ mutation-core
├─ learning-ledger
├─ retrieval ─→ grounding
├─ mutation-journal ─→ mutation-core
├─ policy-runtime ─→ retrieval + contracts
├─ mutation-coordinator ─→ grounding + mutation-core + mutation-journal
└─ intelligence-eval ─→ retrieval + policy-runtime + learning-ledger
```

禁止：

- runtime → eval；
- contracts → 任何实现 crate；
- journal → coordinator；
- retrieval → policy-runtime；
- qualification workspace 复制生产算法；
- `codex-hepta-memory` 继续吸收跨域 orchestration；
- hidden default feature 启用 runtime authority。

B0 包顺序及最小切片：

| 包 | 输入 | 产物 | 行为变化 | 最低证据 |
|---|---|---|---|---|
| B0.1 | Q0/A0 selected base | contracts crate | 无 | E1/E2 |
| B0.2 | B0.1 | pure mutation state | 无 | property tests |
| B0.3 | B0.1 | grounding crate | 无 | compatibility |
| B0.4 | B0.2 | journal adapter | 无 | replay parity |
| B0.5 | B0.1/B0.3 | retrieval contracts | 无 | deterministic fixtures |
| B0.6 | B0.1 | learning ledger contracts | 无 | schema/fuzz |
| B0.7 | B0.5/B0.6 | policy/eval boundary | 无 | no reverse deps |
| B0.8 | 全部 | Cargo/Bazel parity | 无 | full workspace/merge |

每个包必须有 `B0ContractsExtractionReceiptV1` 或对应 handoff，绑定 old/new API surface、moved symbols、consumer inventory、Cargo/Bazel graph、no-behavior-change tests 和 rollback。

---

## 6. Field-level Causal Contracts

所有契约使用 `deny_unknown_fields`、canonical serialization、domain-separated SHA-256、显式版本、最大 encoded bytes、最大 cardinality、owner/tenant/agent/run/episode binding 和 authority-negative fields。

### 6.1 RunStartSnapshotV2

```text
snapshot_id
run_id
agent_id / tenant_id
code_commit/tree
configuration_digest
memory_revision
projection_generation
retrieval_index_generation
policy_artifact_id/digest
model/tokenizer/adapter identities
dataset eligibility policy
started_at_monotonic_counter
schema_version
```

上限：128 KiB；不可包含 raw secret、credential 或未脱敏 context。

### 6.2 LearningEpisodeV1 / LearningEventV1

```text
episode_id
snapshot_id
event_sequence
event_kind
context_digest
candidate_set_digest
decision_digest
effect_intent_digest
effect_receipt_digest
outcome_digest
credit_digest
correction/forget/revocation lineage
privacy_eligibility
training_eligibility
retention_class
causal_parent_digest
event_digest
```

事件严格 append-only；sequence 连续；同 episode 单一 owner；任何 correction 不重写历史，而生成 superseding event。

### 6.3 CandidateSetReceiptV1

```text
query/scope/risk digests
complete candidate inventory
candidate source channel
grounding/truth/lifecycle state
feature vector digest
eligibility/rejection reason
candidate count and truncation proof
selection support
```

没有完整 candidate set 就不得计算 propensity、IPS 或 DR。

### 6.4 PolicyDecisionReceiptV2

```text
policy/model/tokenizer/adapter identity
candidate_set_digest
selected action
logged propensity
support floor
confidence
calibration bucket
OOD score
abstain/veto/slow-path reason
risk class
latency/resource counters
decision digest
```

高风险 action 必须 abstain、reject 或走慢路径；`propensity=0` 的 action 不得进入离线反事实估计。

### 6.5 OutcomeReceiptV1 / CreditLedgerV1

Outcome 支持 delayed、censored、corrected、revoked；Credit 必须守恒：

```text
sum(assigned_credit) == bounded_episode_outcome
```

credit 分配绑定 policy snapshot，不能事后把新策略解释为旧策略行为。

### 6.6 DatasetSnapshotV1 / EvaluationReceiptV2

绑定：

```text
immutable row manifest
source episode/event digests
privacy/training eligibility
split strategy
leakage controls
negative controls
metrics/subgroups
support/ESS/IPS/SNIPS/DR
cluster/bootstrap CI
candidate LCB / baseline UCB
resource/energy
```

晋级必要条件保留：

```text
candidate LCB > baseline UCB
```

### 6.7 PolicyArtifactManifestV2

```text
artifact bytes/digest
base model/backbone
adapter/head
training code/tree
dataset snapshot
hyperparameters
evaluation receipt
signature/role/scope/expiry/revocation
rollback predecessor
```

只允许生成 **next-snapshot**；当前运行快照永不原地自改。

### 6.8 UnlearningComplianceReceiptV1

覆盖 source、memory、KG、index、dataset、adapter、policy、evaluation 派生链。审计表只保留无原文 digest/provenance；删除通过 tombstone、key destruction 或 crypto-shredding 失效。

---

## 7. C0 — Durable Learning Ledger

### Definition of Ready

- A0 selected merge candidate admitted；
- B0.1/B0.6 完成；
- schema、size/cardinality、retention、unlearning policy 冻结；
- no runtime writer default enabled。

### 存储所有权

`hepta-learning-ledger` 是唯一 causal episode/event writer；memory、retrieval、policy 只能提交 typed append request，不能直接写内部表。

### 不变量

- operation/episode/event identity 不复用；
- causal parent 精确；
- duplicate exact retry 返回 Replay；
- changed retry 冲突；
- outcome/credit/correction/revocation 可重放；
- raw content 不进入 telemetry；
- reopen verification 能从 genesis/checkpoint 重建。

### 证据

```text
schema oracle
migration checksum
append/replay property tests
cross-agent/tenant confusion tests
size/cardinality bounds
fuzz decoder
correction/forget/revocation lineage
clean Cargo/Bazel dependency graph
```

### DoD

只推进 `C0 candidate_qualified`。没有 runtime writer、E3 recovery、operator 或 `CALLERS` authority。

---

## 8. M0 — Transactional Mutation Coordinator

现有 shadow journal 只能证明 receipt 序列自洽，不能证明真实 memory/KG/projection/outbox 已发生。M0 采用 host-owned producer：

```text
prepare
→ durable caller request
→ BEGIN transaction
→ source/memory/KG mutation
→ mutation transition
→ transactional outbox intent
→ COMMIT
→ async projection/send
→ store-derived acknowledgement
→ reconciliation
```

关键规则：

- 普通 caller 不得自报 `MemoryWritten`、`ProjectionPublished`、`ReconcileApplied`；
- 同 SQLite/Postgres 的 write/journal/outbox 必须同事务；
- 外部副作用只通过 transactional outbox；
- post-commit ACK loss 用 exact prepared envelope 采纳；
- lease、revision、generation、owner、tenant、causal root 全绑定；
- convenience `prepare+append` 只允许测试，不允许生产调用。

E3 failure matrix：

```text
before intent insert
after intent before memory write
after memory write before journal
after journal before outbox
after outbox before commit
after commit before return
dispatcher before send
send success before ack persist
reconciliation during owner/lease change
process kill / disk full / permission loss / page corruption
```

DoD 要求每个窗口都有零/一次副作用证明、reopen replay、source-derived receipt、duplicate delivery 和 rollback rehearsal。

---

## 9. J0 — Journal Lifecycle, Recovery and Capacity

设计：

```text
active epoch
→ terminal checkpoint
→ immutable checkpoint manifest
→ archive pack
→ verified restore
→ active index compaction without history rewrite
```

每个 checkpoint 绑定：

```text
epoch_id
first/last operation
operation/event counts
prior checkpoint digest
Merkle/hash-chain root
schema/migration identities
archive digest/location class
retention/unlearning policy
created_by software identity
```

必须支持：

- incremental startup recovery；
- background full scrub；
- corruption quarantine；
- backup/restore；
- capacity admission/backpressure；
- active/history 分离；
- archive verification；
- bounded startup/recovery/storage growth。

预注册设计目标：

```text
active operations per epoch <= 8,000
transitions per operation <= 64
incremental recovery <= 5 s / 10,000 active operations
checkpoint verify <= 30 s / 1,000,000 archived transitions
critical replay mismatch = 0
```

这些是设计预算，不是性能达标事实；真实硬件与 soak 属于 E4。

---

## 10. R1 — Grounded Hybrid Retrieval

产品化顺序：

```text
host-owned source span resolution
→ grounded/truth/lifecycle/risk eligibility
→ lexical candidates
→ local semantic candidates
→ bounded KG traversal
→ deterministic fusion
→ optional calibrated reranker
→ context budget compiler
→ shadow comparison
→ canary
```

真实 semantic artifact package 必须包含 `SemanticArtifactEvidencePackageV1`：

```text
model/tokenizer/config bytes + SHA-256
license/provenance
dimension/metric/quantization
max batch/input/output
hardware/runtime compatibility
signature/role/scope/expiry/revocation
deterministic smoke vectors
resource envelope
```

禁止把 hash-one-hot、qualification tokenizer 或 synthetic seed 当作语义 efficacy。

真实 corpus package 使用 `ReviewedCorpusEvidencePackageV1`：

```text
immutable corpus manifest/digest
query/candidate/locale/task/risk labels
two independent reviews per candidate
adjudicator independence
license/provenance
PII/secret/redaction evidence
reviewer keys/signatures/revocation
complete 1:1 projection into evaluation rows
```

评估 lanes：

```text
lexical
vector
KG
lexical+vector
lexical+KG
vector+KG
full fusion
full fusion + reranker
```

指标：

```text
Recall@k / nDCG@k
citation precision/coverage
contradiction and false-memory rate
P50/P95/P99 latency
token/CPU/RAM/GPU/energy
locale/task/risk/privacy subgroup
regression corpus
blind review
```

R1 源码必须进入 product workspace feature matrix，qualification wrapper 只可薄封装生产实现。

---

## 11. N1 — H5 Adaptive Signal Unit

N1 不是名称升级，而是可训练、可校准、可回滚的 signal unit：

```text
shared frozen local encoder/backbone
+ small task head/adapter
+ sparse activation
+ lateral inhibition
+ homeostasis
+ bounded eligibility trace
+ calibrated confidence
+ OOD/abstention
```

输入必须来自 C0 snapshot/episode 和 R1 grounded candidates；输出 `NeuronSignalReceiptV2`，不得直接执行 tool/provider/mutation。

必要试验：

- lesion/ablation；
- frozen-backbone vs head-only；
- sparse density/homeostasis stability；
- calibration/OOD；
- subgroup/safety；
- next-snapshot determinism；
- rollback predecessor；
- long-horizon drift。

达到 N1 仍不能声称 biological replication 或 neuromorphic mechanism。

---

## 12. I1 — H6 Calibrated Fast Policy

I1 只对低风险、可逆、只读 action 提供快速选择。必须记录完整 candidate set 与 propensity。

硬 veto：

```text
credential/auth mutation
external physical send
irreversible tool
high-risk provider effect
topology mutation
unknown/OOD input
missing grounding/truth
```

必要证据：

- calibration error；
- abstention precision/coverage；
- OOD detection；
- propensity support；
- slow-path parity；
- latency/resource；
- subgroup false-positive；
- adversarial prompt/memory poisoning；
- rollback。

没有 C0 causal ledger 和 N1/R1 input，不得从 I0 升级。

---

## 13. L1 — H7 Causal Evaluation and Artifact Pipeline

离线评价必须同时报告：

```text
support coverage
ESS
IPS
SNIPS
doubly robust
cluster/bootstrap CI
candidate LCB
baseline UCB
subgroup/safety/retention/privacy/resource
```

处理：

- delayed/censored outcomes；
- repeated-user/episode clustering；
- clipping 与 clipped ratio；
- zero-support rejection；
- multiple testing；
- pre-registered thresholds；
- negative controls；
- leakage tests；
- correction/forget/revocation 重算；
- artifact signature/expiry/revocation。

最低 pilot 建议：

```text
episodes >= 5,000
supported episodes >= 2,000
ESS >= 500
support coverage >= 80%
clipped ratio <= 5%
critical safety regression = 0
candidate LCB > baseline UCB
7d retention before canary expansion
30d retention before broad promotion
```

阈值必须在结果之前冻结。

---

## 14. C1 — Low-risk Closed Loop

首个闭环固定为可逆、只读 `MemoryRetrievalRank`，禁止 tool/provider/auth/credential/topology/high-risk action。

C1 路径：

```text
frozen baseline
→ offline candidate
→ shadow replay
→ canary with hard quota
→ delayed outcome
→ causal evaluation
→ independent operator acceptance
→ separate CALLERS receipt
→ bounded promotion
```

必须具备：

- kill switch；
- canary tenant/user/rate/duration 上限；
- rollback rehearsal；
- baseline artifact 永久可用；
- no in-place self modification；
- fault isolation；
- privacy/unlearning propagation；
- audit without raw content。

---

## 15. Security, Privacy and Supply-chain

威胁模型：

```text
prompt injection
memory/KG poisoning
citation laundering
cross-agent/tenant confusion
embedding inversion
membership inference
artifact substitution
receipt replay
reviewer-key compromise
malicious corpus intake
telemetry leakage
rollback artifact deletion
```

控制：

- trust / grounding / truth 分离；
- signed/scoped/expiring/revocable artifacts；
- nonce/replay domain；
- software identity 与 human acceptance 分离；
- secret/PII redaction before persistence；
- training eligibility 与 retrieval eligibility 分离；
- provenance/SBOM/lock/toolchain binding；
- commit/tag or merge-queue identity；
- artifact attestation；
- key rotation/revocation；
- high-risk abstention/veto；
- no raw content in telemetry；
- unlearning lineage。

---

## 16. Cross-package Validation Matrix

### E0 Source/static

```text
schema
required/forbidden fields
size/cardinality constants
digest/read-order/canonical uniqueness
Cargo/Bazel DAG
default-off feature
unknown authority fail closed
```

### E1 Local executable

```text
fmt
focused/full tests
strict Clippy
property/fuzz/adversarial
deterministic receipt twice
clean tree
```

### E2 Independent runner/platform

```text
x86_64 + ARM64
Linux plus selected macOS/Windows support policy
toolchain/lock identity
artifact attestation
cross-platform deterministic fixtures
```

### E3 Runtime/recovery

```text
real process kill/restart
BEGIN IMMEDIATE contention
pre/post-commit windows
WAL/checkpoint
disk full/permission loss
page/bit corruption
duplicate delivery
clock regression
owner/tenant confusion
backup/restore/archive
```

### E4 Longitudinal/operator

```text
soak
retention
efficacy
resource/energy
subgroups
unlearning
operator acceptance
rollback rehearsal
CALLERS promotion
```

---

## 17. Package Handoff and Gap Loop

每个工作单元输出 `PackageHandoffReceiptV1`：

```text
package_id/version
repository/branch/head/tree/parent
dependency receipts
changed paths/blob/content digests
commands/exits
artifacts/digests/expiry
closed/open/external gaps
authority flags
rollback pointer
resume predicate
stop reason
```

Gap loop：

```text
REVALIDATE
→ CLASSIFY
→ IMPLEMENT smallest coherent source-controlled slice
→ RUN exact gates
→ EMIT receipt
→ REVALIDATE dependencies
→ CLOSE or BLOCK with machine-readable predicate
```

合法 terminal classification：

```text
CLOSED_SOURCE_CONTROLLED
OPEN_SOURCE_CONTROLLED
BLOCKED_EXTERNAL_EVIDENCE
BLOCKED_UPSTREAM
STOP_CONDITION
```

Fixture 只能证明 mechanics，不能关闭真实 corpus、reviewer、hardware、soak、operator、CALLERS、promotion、release 或 production fact。

---

## 18. Stage Definition of Ready / Done

### A0 DoR

- Q0 exact paired receipt valid；
- A0 sole-parent protocol；
- 17-path allowlist；
- runtime freeze；
- all authority=false。

### A0 DoD

- exact-head source/finalizer PASS；
- unique unexpired artifacts；
- independent Q1 review；
- canonical selection；
- selected synthetic merge check attribution；
- repository-controlled required checks all green or independently approved exclusion；
- no self-merge。

### B0 DoD

- nine-package boundary manifest；
- no cycles/reverse dependency；
- qualification wrapper parity；
- Cargo/Bazel parity；
- no behavior change；
- Q0 compatibility regression green。

### C0/M0/J0 DoD

- store-owned facts；
- append/replay and exact retry；
- atomic transaction/outbox；
- E3 failure matrix；
- checkpoint/archive/restore；
- bounded recovery；
- rollback。

### R1/N1/I1/L1/C1 DoD

- real artifacts/corpus；
- product workspace integration；
- calibrated abstention/OOD；
- complete support/propensity；
- causal metrics and CIs；
- subgroup/safety/retention/unlearning；
- next-snapshot signature；
- canary/kill switch/rollback；
- independent operator and CALLERS。

---

## 19. Final Execution Order

```text
1. Publish V4.3 as one exact-parent A0 replacement
2. Obtain fresh A0 exact-head executable evidence
3. Obtain independent review and canonical selection
4. Build selected synthetic merge candidate
5. Attribute and repair all repository-controlled required checks
6. Extract B0.1 → B0.8
7. Implement C0 durable causal ledger
8. Implement M0 transactional coordinator
9. Implement J0 lifecycle/recovery
10. Restack and integrate R1
11. Implement N1
12. Implement I1
13. Implement L1
14. Pilot C1 MemoryRetrievalRank
15. Only after longitudinal DoD: N2/S1
16. N3 remains isolated research
```

在第 3–5 步完成前，不得落盘 B0/runtime-adjacent source。任何阶段都不得自行把 `candidate_qualified` 提升为 runtime、efficacy、operator、promotion 或 production authority。缺少真实外部输入的 gap 必须保持 `BLOCKED_EXTERNAL_EVIDENCE`。
