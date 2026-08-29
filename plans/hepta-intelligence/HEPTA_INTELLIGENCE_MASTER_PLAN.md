# Hepta Intelligence Master Development Plan

> **CANONICAL_CURRENT / PLAN_ONLY / FAIL_CLOSED**
>
> Plan ID: `HEPTA_INTELLIGENCE_MASTER_PLAN_V4`
>
> Version: `4.4.0`
>
> Repository: `ProfHepta/hepta-private-ci`
>
> Current program phase: `A0_CANONICAL_CAPABILITY_AND_EVIDENCE_AUTHORITY`
>
> Current capability claims: `L0 / N0 / I0 / MM0_SPECIFIED_ONLY`

本文件是**唯一有效的人类可读开发计划**。它把 V4.3 的证据、角色、边界、学习闭环治理与多模态 Memory 的 MM0–MM6 路线统一为一个 canonical plan。它不改变任何运行时行为，也不授予 runtime、operator、production、promotion、release 或 `CALLERS` authority。

---

## 0. V4.4 变更摘要

V4.4 在 V4.3 上完成以下 source-controlled 深化：

1. 把强制读取顺序统一为 8 个输入，机器 truth、AGENTS、master 与 verifier 不再分裂；
2. 将 subordinate execution specification 的 canonical parent 从 V4.2 修正为 V4.4；
3. 统一 `RepositoryCheckAttributionReceiptV1` 的分类词汇并要求 unknown fail closed；
4. 将 source snapshot 时间改为 commit/executable-receipt 绑定，不再在源码中预写未来 live 时间；
5. 把多模态 Memory 从外部提案纳入 canonical plan，并在现有 integration candidate 中注册独立 42 项 machine ledger；
6. 定义 MM0–MM6 的 package ownership、数据平面、契约、威胁模型、证据、停止条件与 handoff；
7. 保留 A0 runtime freeze：在独立 review、canonical selection 与 selected merge-candidate admission 前，禁止 B0/runtime-adjacent source 落盘。

本版本仍为 plan-only：

```text
self_evolution=false
closed_loop_learning=false
longitudinal_learning_efficacy=false
structural_plasticity=false
neuromorphic_mechanism=false
biological_mechanism_replication=false
native_media_memory_wired=false
cross_modal_retrieval_qualified=false
multimodal_efficacy_proven=false
runtime_wired=false
production_authority=false
```

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
  只表达发布时已知的 source contract；不嵌入尚未产生的 live run 结论

LIVE_EVIDENCE
  GitHub API / runner / artifact / external reviewer 的实时事实
  必须由 receipt 绑定 exact repository/branch/head/tree/parent/workflow/
  run/attempt/job/runner/steps/artifact/digest/expiry

SELECTION_TRUTH
  由独立 selector 绑定 exact candidate、review receipt 与 rollback base

RUNTIME_TRUTH
  由 E3/E4、operator acceptance 与独立 CALLERS receipt 共同形成
```

失效规则：

- head、tree、parent、workflow SHA、artifact digest、review target 任一变化，旧 evidence 立即 `STALE_SUPERSEDED`；
- PR head evidence 与 synthetic merge candidate evidence 不可互换；
- queued、pending、cancelled、`steps=[]`、`runner_id=0`、过期 artifact、source-only receipt 均不是 PASS；
- live API 观察不得由 CI 直接写回 candidate source；
- unknown field、unknown capability、unknown positive authority、unknown gap 均 fail closed；
- 同一 commit 被移动到新 branch 时，branch-bound receipt 不自动复用。

Evidence classes：

```text
E0 source/static/schema
E1 local executable
E2 independent runner/platform
E3 real process/restart/failpoint/durability/recovery
E4 reviewed corpus/longitudinal efficacy/resource/operator/canary
```

---

## 2. 当前精确基线、A0 replacement 与角色隔离

### 2.1 Q0 exact candidate

```text
repository = ProfHepta/hepta-private-ci
branch = codex/hepta-intelligence-plan-v3-20260828
head = c768bcbeb4c1168088d2499828c24da521a2a73a
tree = ca455a9ef797cd95164c880c7b8faba80b305589
parent = aeb8ac0bfb30d570a16c4914b6e4b31ce035dd62
run = 33252922404
qualified_candidate = true
runtime_capability_qualified = false
full_repository_merge_green = false
```

### 2.2 A0 replacement protocol

A0 candidate 必须是 Q0 head 的恰好一个直接子提交：

```text
revalidate Q0 exact head/tree
→ build complete replacement tree
→ create one sole-parent commit(parent=Q0 head)
→ atomically move the A0 branch
→ invalidate every prior A0 evidence/review
→ run fresh exact-head source/finalizer workflows
→ obtain independent review
→ obtain canonical selection
→ qualify the selected synthetic merge candidate
```

A0 允许：`DOCUMENTATION / REGISTRY / VERIFIER / READ_ONLY_WORKFLOW`。

A0 禁止：

```text
Rust runtime source
SQL migration
product caller
model/provider dispatch
H5/H6/H7 runtime
native-media intake
vector index or multimodal retrieval runtime
CALLERS
promotion
release
self-review
self-approval
self-merge
```

### 2.3 Separation of duty

| 角色 | 允许 | 不得替代 |
|---|---|---|
| implementer | 修改允许路径、构造 source candidate | reviewer、selector、operator |
| source publisher | 通过认证 Git write 发布 exact candidate | CI evidence workflow |
| CI evidence workflow | read-only 执行并产出 artifact | source writeback、selection |
| independent reviewer | 评审 exact source/evidence，签发 `A0IndependentReviewReceiptV1` | implementer/publisher |
| canonical selector | 选择唯一 integration candidate | PR 作者自选 |
| operator | 接受 runtime/rollback/production 风险 | fixture 或测试密钥 |

`最高权限` 不改变 separation of duty；它不允许伪造独立 reviewer、selector、operator、corpus、hardware 或 canary 事实。

---

## 3. Capability、claim ladder 与 lifecycle

统一生命周期：

```text
specified
→ implemented
→ candidate_qualified
→ selected
→ wired
→ runtime_qualified
→ efficacy_proven
→ operator_accepted
→ promoted
→ released
```

当前真实声明：

```text
system_learning=L0_STATIC_SHADOW_WITH_PARTIAL_L1_FOUNDATIONS
H5=N0_METAPHORICAL_TYPED_PROPOSAL
H6=I0_DETERMINISTIC_SELECTIVE_POLICY
multimodal_memory=MM0_SPECIFIED_ONLY
self_evolution=false
closed_loop_learning=false
longitudinal_learning_efficacy=false
structural_plasticity=false
neuromorphic_mechanism=false
biological_mechanism_replication=false
local_small_model_used_by_h5=false
local_small_model_used_by_h6=false
native_media_memory_wired=false
cross_modal_retrieval_qualified=false
multimodal_efficacy_proven=false
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

任何 source/test fixture 最多推进 `candidate_qualified`，不能推进 selected、wired、runtime-qualified、efficacy、operator 或 production。

---

## 4. Integration candidate 与 repository-check attribution

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

唯一合法 classification：

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

`UNKNOWN_FAIL_CLOSED` 阻塞 merge；专项 A0 绿灯不得覆盖 full-repository required checks。

---

## 5. B0 九包边界与 Field-level Causal Contracts

### 5.1 B0 九包边界

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

禁止 reverse dependency、qualification code copy、hidden default runtime feature，以及 `codex-hepta-memory` 继续吸收跨域 orchestration。

### 5.2 Field-level Causal Contracts

所有契约使用 `deny_unknown_fields`、canonical serialization、domain-separated SHA-256、显式版本、最大 encoded bytes、最大 cardinality、owner/tenant/agent/run/episode binding 与 authority-negative fields。

必需契约：

```text
RunStartSnapshotV2
LearningEpisodeV1
LearningEventV1
CandidateSetReceiptV1
PolicyDecisionReceiptV2
OutcomeReceiptV1
CreditLedgerV1
DatasetSnapshotV1
EvaluationReceiptV2
PolicyArtifactManifestV2
UnlearningComplianceReceiptV1
PackageHandoffReceiptV1
```

`CandidateSetReceiptV1` 必须包含完整合法 action inventory；没有 complete candidate set 就不得计算 propensity、IPS、SNIPS 或 DR。

`PolicyDecisionReceiptV2` 必须绑定 policy/model/tokenizer/adapter、candidate-set digest、selected action、logged propensity、support floor、confidence、calibration、OOD、abstain/veto/slow-path、risk 与 resource counters。

`UnlearningComplianceReceiptV1` 覆盖 source、memory、KG、index、dataset、adapter、policy、evaluation 派生链；审计只保留 content-free digest/provenance。

---

## 6. C0、M0、J0、R1、N1、I1、L1、C1

### 6.1 C0 — Durable learning episode ledger

`hepta-learning-ledger` 是唯一 causal episode/event writer。事件 append-only，correction/forget/revocation 生成 superseding event，不重写历史。必须证明 append/replay、exact retry、owner/tenant 隔离、credit conservation、schema migration、reopen determinism 与 bounded storage。C0 source qualification 不授予 training 或 product-write authority。

### 6.2 M0 — Transactional mutation coordinator

Host-owned producer sequence：

```text
prepare immutable request
→ persist binding
→ BEGIN transaction
→ verify lease/revision/generation
→ write source/memory/KG facts
→ append mutation transition
→ append transactional outbox intent
→ COMMIT
→ asynchronous dispatch
→ store/provider acknowledgement
→ reconciliation
```

只有观察事实的 store/adapter 可签发 `MemoryWritten`、`ProjectionPublished`、`OutboxDispatched` 或 `ReconcileApplied`。E3 覆盖 pre/post-commit、ACK loss、duplicate request、lease race、disk full、process kill 与 restart。

### 6.3 J0 — Journal lifecycle, recovery and capacity

```text
active epoch
→ terminal checkpoint
→ immutable checkpoint manifest
→ create-only archive pack
→ verified restore
→ active-index compaction without history rewrite
```

必须支持 incremental startup、background scrub、corruption quarantine、backup/restore、capacity admission、bounded recovery 与 rollback rehearsal。

### 6.4 R1 — Grounded hybrid retrieval

```text
host-owned source-span resolution
→ grounding/truth/lifecycle/risk eligibility
→ lexical candidates
→ local semantic candidates
→ bounded KG traversal
→ deterministic fusion
→ optional calibrated reranker
→ context budget compiler
→ shadow comparison
→ canary
```

真实 `SemanticArtifactEvidencePackageV1` 必须包含 model/tokenizer/config bytes+SHA-256、license/provenance、dimension/metric/quantization、hardware/runtime compatibility、signature/expiry/revocation、smoke vectors 与 resource envelope。Hash-one-hot 或 synthetic seed 只能证明 mechanics。

### 6.5 N1 — H5 adaptive signal unit

N1 使用 `shared frozen local encoder/backbone` 加 small task head/adapter、sparse activation、lateral inhibition、homeostasis、bounded eligibility trace、calibrated confidence 与 OOD/abstention。输出仅为 `NeuronSignalReceiptV2` proposal，不直接执行 effect。

### 6.6 I1 — H6 calibrated fast policy

I1 仅处理低风险、可逆、只读 action；auth/credential/provider/tool/topology/high-risk action 硬 veto。必须记录完整 candidate set 与 propensity，并证明 calibration、OOD、abstention、slow-path parity、subgroup safety 与 rollback。

### 6.7 L1 — H7 causal evaluation and artifact pipeline

离线评价必须报告 support coverage、ESS、IPS、SNIPS、doubly robust、cluster/bootstrap CI、subgroup/safety/retention/privacy/resource。Promotion 的必要条件之一是：

```text
candidate LCB > baseline UCB
```

阈值必须在结果之前冻结；只允许生成 signed next-snapshot artifact，不允许当前运行原地自改。

### 6.8 C1 — Low-risk closed loop

首个闭环固定为可逆、只读 `MemoryRetrievalRank`：

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

---

## 7. Multimodal Memory target architecture

### 7.1 三平面模型

#### Evidence asset plane

Original text、image、audio、video、document 与批准的 sensor evidence 作为不可变、content-addressed asset。大对象存入 encrypted object store；SQLite/Postgres 只保存 bounded manifest、lifecycle、segment、transaction/outbox state。

每个 asset 绑定：

```text
asset id/revision/content SHA-256
canonical MemoryScopeV2 digest
sniffed and declared MIME
modality/codec/bytes/pixels/pages/frames/samples/duration
object key/encryption-key identity
capture/import time and trust class
retention/legal-hold/redaction policy
```

#### Derived artifact plane

OCR、ASR、caption、scene summary、crop、frame、thumbnail、redacted copy、feature artifact 均为 `DerivedArtifactV1`，绑定 exact asset/segment、extractor/model/preprocessor/prompt/output digest，并永久保持：

```text
source_truth=false
```

#### Semantic memory plane

Fact、episode、entity、relation、procedure、preference 仍为 compact textual/structured memory；每个 assertion 引用 exact asset/derived segment 的支持与反证，并区分 proposed、verified、contradicted、revoked 状态。

### 7.2 Scope 与 segment

`MemoryScopeV2` 统一 installation/workspace/tenant/principal/thread 与显式 delegation。Asset、segment、derivative、embedding、index、query、citation、deletion 与 training lineage 全部绑定同一 canonical scope digest。

`MediaSegmentV1` 支持 bounded locator：byte range、page range、page rectangle、audio/video time range、frame range、OCR/text span。整段视频/PDF 的模糊 citation 不能替代精确 segment。

### 7.3 Storage 和 transaction

- 原始媒体禁止进入 `memory_revisions.content`；
- object write、manifest transition、mutation journal 与 outbox 使用 M0/J0 contract；
- same-store facts 原子提交，cross-system effects 通过 transactional outbox；
- temporary lease、orphan GC、key lifecycle、capacity admission、checkpoint/archive/restore 均需 receipt；
- database-only backup 不构成完整媒体 Memory backup。

---

## 8. Required multimodal contracts

```text
MemoryScopeV2
MemoryAssetManifestV1
MediaSegmentV1
AssetLifecycleV1
AssetIntakeReceiptV1
DerivedArtifactV1
EvidenceSetV1
ModelArtifactManifestV1
EmbeddingSpaceManifestV1
EmbeddingManifestV1
CrossModalQueryV1
MultimodalCandidateReceiptV1
MultimodalRerankReceiptV1
MultimodalRetrievalReceiptV1
RevalidationBindingV1
ContextCompilationReceiptV1
DeletionPropagationReceiptV1
MultimodalEfficacyReceiptV1
MultimodalSecurityReceiptV1
MultimodalResourceReceiptV1
```

所有契约必须版本化、bounded decode、canonical serialization、domain-separated digest、exact scope/source/model/preprocessor/index/policy binding、unknown-field rejection、property/fuzz/corruption/replay tests，并显式保持所有 authority false。

---

## 9. MM0–MM6 roadmap

### MM0 — Contract and authority foundation

MM0-A：`MemoryScopeV2`、`MemoryAssetManifestV1`、`MediaSegmentV1`、lifecycle/intake receipt。  
MM0-B：derivative、model、embedding-space、query、revalidation、context、deletion contracts。

DoD：E1/E2 compile/fmt/test/strict Clippy、canonical digest vectors、bounds/property/fuzz/cross-scope rejection、Cargo/Bazel parity。仍 `wired=false`。

### MM1 — Encrypted asset, segment and derivation ledger

Object store 保存 encrypted original/derived bytes；database 保存 manifest、segment、key reference、retention/legal hold、transactional outbox。Intake 流程必须 stream/hash/bound/quarantine/encrypt/commit/finalize/reconcile，并在 crash、duplicate、stale CAS、disk full、missing object、key unavailable、restart、concurrent forget 下保持 exactly-once logical effect。

### MM2 — Sandboxed decode, extraction, redaction and quarantine

Decoder worker 无 network、credential、repository write 或 production token，并限制 decompressed bytes、pixels、pages、frames、duration、CPU、RSS/VRAM、temp disk、derivative count。安全 corpus 覆盖 malformed/polyglot/bomb/external reference/codec hang/QR secret/prompt injection/adversarial media。

OCR、ASR、caption、scene segmentation、entity extraction、PII/secret/face/document redaction 输出 `DerivedArtifactV1`；provider failure 不得伪造 empty success。

### MM3 — Model registry, embedding spaces and immutable indexes

`ModelArtifactManifestV1` 绑定 provider/model/revision、weights digest/size、tokenizer/preprocessor、license、quantization/backend、supported modalities、hardware compatibility、resource envelope 与 security review。禁止 runtime download。

`EmbeddingSpaceManifestV1` 绑定 space ID/generation/model/preprocessor/dimension/metric/encoding/normalization/task mode。不同 space/generation 的 vectors 禁止直接比较；reindex 创建新 immutable generation；dual-read shadow 后再 cutover/GC。

初始 benchmark candidates（不是预定 winner）：Qwen3-VL-Embedding-2B、Nomic vision/text、SigLIP 2 与一个 visual-document multi-vector baseline。音频先采用 ASR+text，独立 audio space 作为后续受控 lane。

### MM4 — Cross-modal retrieval and exact revalidation

候选 channels：memory FTS、OCR/transcript FTS、entity/KG/temporal、recency、text/image/document/audio/video embeddings、perceptual duplicate index。每个 channel 有 quota/deadline；fusion deterministic、stable tie-break、bounded cap；optional reranker 不得引入 candidate-set 外对象。

返回前必须重验证：

```text
scope/delegation
asset lifecycle/revision/content digest
segment locator/digest
derivative lineage/source_truth=false
model/preprocessor generation
memory head/citations
KG generation
embedding space/vector/index generation
policy/redaction/deletion state
```

任何 stale/revoked/deleted/mismatched candidate 必须 drop 并记录 reason。

### MM5 — Semantic consolidation and bounded context

Source、derivative、proposed assertion、verified assertion、contradicted、revoked 分离。高风险 identity/biometric/credential/financial/medical/legal assertion 必须人工 review 或 abstain；model confidence 不能授予 memory-write authority。

Context compiler 仅输出 bounded text/OCR/transcript span、approved thumbnail/crop、timestamped frames、受控 audio excerpt 或 content-free reference，并绑定 exact citation、risk、token/pixel/frame/byte budget 与 `ContextCompilationReceiptV1`。

### MM6 — Deletion, security, efficacy, hardware, soak and canary

删除/Unlearning 必须沿：

```text
source asset → key/object → segment → derivative → embedding/vector
→ index generation → cache/context → dataset → adapter/checkpoint/policy/evaluation
```

安全 campaign 覆盖 cross-scope ANN、embedding inversion/membership、multimodal prompt injection、adversarial media、parser/decoder crash、path/object-key traversal、forged digest/receipt、stale generation、secret/PII/biometric leakage 与 quota DoS。

Efficacy corpus 必须具备 consent/license/provenance/privacy review、immutable digest、independent labels/adjudication、modality/query/locale/risk/task balance、exact temporal/page/rectangle/frame grounding、correction/forget/contradiction/OOD cases。

指标包括 Recall@k/nDCG@k、citation/grounding precision、OCR/ASR/caption quality、contradiction/stale/deleted retrieval、calibration/abstention、privacy leakage、latency/throughput/recovery、CPU/GPU/NPU/RAM/VRAM/disk/energy、index growth/rebuild/deletion lag。Aggregate win 不得掩盖 subgroup 或 privacy failure。

---

## 10. Security, privacy and supply-chain

Threat model：

```text
prompt injection and memory/KG poisoning
citation laundering and generated-content promotion
cross-agent/tenant/scope confusion
embedding inversion and membership inference
artifact/model/preprocessor substitution
receipt replay and reviewer-key compromise
malicious corpus/media intake and codec exploitation
telemetry/content leakage
rollback artifact deletion
backup resurrection after forget
```

Controls：trust/grounding/truth separation；signed/scoped/expiring/revocable artifacts；nonce/replay domain；secret/PII redaction before persistence；training eligibility 与 retrieval eligibility 分离；SBOM/lock/toolchain binding；high-risk abstention；no raw media/content in telemetry；full deletion lineage。

---

## 11. Cross-package validation matrix

### E0 Source/static

Schema、required/forbidden fields、size/cardinality、digest/read-order/canonical uniqueness、Cargo/Bazel DAG、default-off feature、unknown authority fail closed。

### E1 Local executable

fmt、focused/full tests、strict Clippy、property/fuzz/adversarial、deterministic receipt twice、clean tree。

### E2 Independent platform

x86_64 + ARM64、selected OS policy、toolchain/lock identity、artifact attestation、cross-platform deterministic fixtures。

### E3 Runtime/recovery

real process kill/restart、transaction contention、pre/post-commit windows、WAL/checkpoint、disk full/permission loss/corruption、duplicate delivery、backup/restore/archive、object/key mismatch。

### E4 Longitudinal/operator

reviewed corpus、soak、retention、efficacy、resource/energy、subgroups、unlearning、operator acceptance、rollback rehearsal、CALLERS promotion。

---

## 12. Gap loop、handoff 与停止条件

每个 package 输出 `PackageHandoffReceiptV1`：

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

合法 repository classification：

```text
CLOSED_SOURCE_CONTROLLED
OPEN_SOURCE_CONTROLLED
BLOCKED_EXTERNAL_EVIDENCE
BLOCKED_UPSTREAM
STOP_CONDITION
```

Stop immediately on base/head/tree/parent drift、path outside allowlist、positive authority、source-writeback、stale evidence reuse、scope ambiguity、missing deletion lineage、model/license mismatch、decoder network/credential access 或 corpus consent failure。

---

## 13. Stage DoR/DoD

### A0 DoD

- exact-head source/finalizer PASS；
- unique unexpired artifacts；
- independent Q1 review；
- canonical selection；
- selected synthetic merge check attribution；
- repository-controlled required checks all green or independently approved exclusion；
- no self-merge。

### B0/MM0 DoD

- package ownership/DAG frozen；
- bounded versioned contracts；
- Cargo/Bazel parity；
- no product behavior change；
- x86_64+ARM64 exact-head receipts；
- all authority false。

### MM1–MM5 DoD

- store-owned facts and exact lineage；
- atomic transaction/outbox；
- sandbox/resource limits；
- immutable model/index generations；
- exact revalidation and bounded context；
- E3 failure/reopen/restore/deletion evidence；
- no caller or production authority unless separately authorized。

### MM6/C1 DoD

- real reviewed artifacts/corpus；
- efficacy and subgroup guardrails；
- hardware/resource/energy/soak；
- deletion/unlearning non-resurrection；
- canary/kill switch/rollback；
- independent operator and CALLERS receipts。

---

## 14. Final execution order

```text
1. Publish V4.4 as one exact-parent A0 replacement
2. Obtain fresh A0 exact-head source and executable evidence
3. Obtain independent review and canonical selection
4. Build selected synthetic merge candidate
5. Attribute and repair repository-controlled required checks
6. Extract B0.1 → B0.8
7. Implement MM0-A → MM0-B contracts
8. Implement C0 durable causal ledger
9. Implement M0 transactional coordinator
10. Implement J0 lifecycle/recovery
11. Restack and integrate R1 grounded retrieval
12. Implement MM1 asset/derivation ledger
13. Implement MM2 sandbox/extraction/redaction
14. Implement MM3 artifact/embedding/index generations
15. Implement MM4 cross-modal retrieval/revalidation
16. Implement MM5 consolidation/context compiler
17. Execute MM6 deletion/security/efficacy/hardware/soak
18. Implement N1 → I1 → L1
19. Pilot C1 MemoryRetrievalRank and multimodal retrieval in shadow/canary
20. Only after longitudinal DoD: N2/S1; N3 remains isolated research
```

在第 3–5 步完成前，不得落盘 B0、MM0 或 runtime-adjacent source。缺少真实 reviewer、selector、corpus、model/license approval、hardware、soak、operator、CALLERS、promotion 或 release 输入的 gap 必须保持 `BLOCKED_EXTERNAL_EVIDENCE` 或 `BLOCKED_UPSTREAM`，不得由 prose/fixture 伪关闭。
