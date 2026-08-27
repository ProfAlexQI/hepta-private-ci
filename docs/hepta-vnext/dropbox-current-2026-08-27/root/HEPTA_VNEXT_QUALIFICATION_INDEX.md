# Hepta vNext qualification index — current developer binding (2026-08-27)

这是本轮独立 qualification 产物与当前开发文档绑定的索引。下方历史条目按原始时间和
provenance 保留；它们仍是 shadow/qualification-only，不改变 G4/G5、CALLERS、production
caller、operator acceptance 或 promotion。

当前开发绑定：`hepta-vnext-development-plan-final-2026-08-23.md`，canonical SHA-256
`f0a98ae0e6cb1ff3c4bb660bfe50ca921cda434926a755d127dcc7399c4a5620`，以 E.45 文档同步节为
当前入口；AuthBus 语义版本保持 `AUTHBUS.11 v1.3`，唯一 source 为
`AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry`，active selector 为
`AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map`。当前绑定仍是
`PLANNING_ONLY / IMPLEMENTATION_BACKLOG`，不是独立 runtime/production authority。

当前机器可读绑定：`HEPTA_DEVELOPMENT_DOCS_CURRENT_BINDING_V1.json`；变更收据：
`HEPTA-DEVELOPMENT-DOCS-SYNC-RECEIPT-2026-08-27.json`。二者只记录文档 digest 和负向
authority 边界；receipt/index 自身不互相纳入 digest，避免自引用。旧 E.20/E.42/E.43/E.44
receipt、镜像和 rebind package 仍是不可变历史或显式 stale/decode-only provenance。

| Slice | Worktree / commit | Result | Receipt SHA-256 | Tests |
|---|---|---|---|---:|
| G4 | `r2-g4-paired-exact-final40-20260823` | `qualified_exact` | `f36ce3f41cc8734f4392070a01ac53cbdf753dee5a1bb8b352feb1bc886e8064` | final40 |
| G5 | `qual-g5-bounded-aggregate-20260823` | `PASS_BOUNDED_AGGREGATE` (qualification only) | `7d178989d75ff4fe32c41fc38737a3d12a0ef0f3da30b6e84ff5c03bf11136a1` | 6 slices |
| H0/H1 | `qual-h0-protocol-20260823` / `74078ea…` | qualified shadow | `c515d55d65a4c5d62c3f171d4e2e5acbd9f95b5d2fc1fcb286ebe88ace15157d` | 36 |
| H2 | `qual-h2-workflow-20260823` / `fe9122c…` | qualified shadow | `a65d9805fa86db32667a464e115c0e1f7670d1e68b5e8d0abc49f9af0570d17e` | 41 (11 H2) |
| H3 | `qual-h3-taskflow-20260823` / `8cdd600…` | `PASS_H3_SHADOW` | `dde17a21ecb5bef1d712762c27c83e2bb3a18de40aedbec61cc2800e3f16ab0c` | 16 |
| H4 | `qual-h4-memory-compact-20260823` / `ab67e77…` | `PASS_H4_MEMORY_COMPACT` | `b066970c66b0efdc0baa7ceff5097fa89d1e41463375158ac4505d79aef6e936` | 10 |
| H5 | `qual-h5-neuron-group-20260823` / `5c69eee…` | `PASS_H5_NEURON_GROUP` | `c54fb6b66db9e600c74fb8afc405c28ab19af23624efa92fad05cd2e83655c22` | 11 |
| H6 | `qual-h6-intuition-policy-20260823` / `8807e17…` | `PASS_H6_INTUITION_POLICY` | `f471025ef0a6c9b907a0af51a154479dbdb5fa676ee1137980d94657066d4337` | 13 |
| H7 | `qual-h7-ndu-learning-20260823` / `7e84411…` | `PASS_H7_SHADOW` | `d562ca3b76f1b384979308e7c5df2aafb0fa56bd42b0e085419119382cde174e` | 10 |

## Implementation-contract qualification — 2026-08-23 20:31

These three lanes extend the H0–H7 semantic fixtures into explicit implementation contracts. They remain shadow-only and are bound to exact G4 parent `445d1cdc50c9e86d09041b17888245b8c5937bda`.

| Lane | Worktree / final commit | Result | Receipt SHA-256 | Tests |
|---|---|---|---|---:|
| P0 protocol contracts | `qual-protocol-contracts-20260823` / `0e84bd1f20d539eade367c3ffb41a063ad28696c` | `PASS_PROTOCOL_CONTRACTS_SHADOW` | `11f1f080161389b770bce226a94ea1a9a65bd127d36a5f144d9db48359475d64` | 12 |
| H4–H7 implementation contracts | `qual-contracts-h4-h7-20260823` / `ca2cf9fe69a54e1560484c430fa86094f48f132a` | `PASS_H4_H7_CONTRACTS_SHADOW` | `63184346bbbe3fdc62e099fef338dc5706897ec0fabfc04a96f1d8d85a016f28` | 12 |
| implementation readiness | `qual-implementation-readiness-20260823` / `ae5009358df7003d1abcf4b23ebc1db0612ddae9` | `PASS_IMPLEMENTATION_READINESS` | `42f1ab45bfc9a65f409e3ffb25d770cbdf863bf40f02c9a61b9284c24360a8f9` | 5 |

The new artifacts are stored in `protocol-contracts-qualification-20260823/`, `h4-h7-contracts-qualification-20260823/`, and `implementation-readiness-qualification-20260823/`. Each directory has a local `SHA256SUMS` manifest; independent remote reruns passed. The protocol and H4–H7 generators are idempotent across two consecutive runs.

Coverage includes:

- `RunStartSnapshot`, `LeaseFence` and epoch/CAS lifecycle;
- canonical event payload/rebuild, transactional outbox and Activity-vs-Effect intent split;
- GuardExpr/loop/fan-out/join/retry/approval semantics and migration CAS/abort;
- memory admission/forget lineage, compact CAS/rehydration, privacy-aware neuron fallback;
- DecisionReceipt hard safety veto/coverage floor, NDU propensity/support/OPE/CI and artifact rollback;
- 16-stage implementation order, real crate graph acyclicity, RACI/CI/resource budgets and position-specific fallback profiles.

All three receipts explicitly keep `production_writer`, `effect_dispatch`, `model_inference`, `npu_connected`, `operator_acceptance`, `callers_promoted`, `g5_allowed`, and `promotion` false. These artifacts are contract qualification, not implementation efficacy or canary evidence.

## Isolated implementation-spike qualification — 2026-08-23 21:05

These lanes push the remaining implementation blockers without entering a production caller. `PASS_H1_*_SHADOW` is an in-memory/standalone fixture result; `BLOCKED_*_PREREQUISITES` is an intentional, reproducible stop at the missing production seam.

| Lane | Worktree / final commit | Result | Receipt SHA-256 | Tests |
|---|---|---|---|---:|
| H1 authoritative writer/outbox | `qual-h1-authoritative-writer-20260823` / `42e958d4d083a0beb19f76e4e5ecaad7110f14ea` | `PASS_H1_AUTHORITATIVE_WRITER_SHADOW` | `168fe827b87d136edc70f555d88d0a57fddfaed7547f6b32f6cbd4b7135974cc` | 16 |
| H4a/H4b implementation prerequisites | `qual-h4-implementation-spike-20260823` / `0fd584455464071fbfc7d00517db39d9c2c6724d` | `BLOCKED_H4_IMPLEMENTATION_PREREQUISITES` | `b5ab99b0595b8f3ece7eb915f77f383a8194add090c4d79c501f1c9d79ad8342` | 12 |
| H5b/H6b/H7a runtime prerequisites | `qual-h5-h7-runtime-prereqs-20260823` / `d0e418e6f1d94b02576bd8dba8d73ef61f40909e` | `BLOCKED_H5_H7_IMPLEMENTATION_PREREQUISITES` | `52a46db61b2021de346859ec1ca156d9ea47240004e4353189698e133cfbcad5` | 8 |

Coverage includes H1 event/projection/outbox copy-on-write, command dedupe, CAS/fencing, fault rollback and deterministic rebuild; H4 typed admission/compact implementation semantics and static seam audit; and H5–H7 no-op runtime adapters plus Core ML/NPU/trajectory seam inventory. All new receipts are exact-G4 bound and keep production writer/effect/model/NPU/operator/promotion/G5 flags false.

H4 remains blocked because the exact G4 tree has no automatic MemoryAdmission writer or typed CognitiveRuntime compact hooks. H5–H7 remain blocked because there is no typed neuron/intuition/NDU/trajectory runtime seam, model/sidecar, or real NPU efficacy benchmark. These are not failures of the shadow semantics; they are the next implementation prerequisites.

## Negative authority aggregate

The H0–H7 receipts collectively assert:

```text
production_caller=false
production_writer=false
model_invocation=false (H5 shadow)
tool_effect=false
effect_authority=false
g5_allowed=false
operator_acceptance=false
promotion=false
CALLERS_touched=false
G4_exact_candidate_modified=false
```

## H8/H9

`HEPTA_H8_H9_GOVERNANCE_GATE.md` is the current gate contract. Its expected result is `BLOCKED_GOVERNANCE_PREREQUISITES` until an independent G5 operator acceptance/CALLERS/promotion receipt exists. No canary, legacy cutover, fleet propagation, or write-capable workflow was attempted.

## Reproduction rule

Every receipt must be independently verified from its sealed worktree and `SHA256SUMS`. A mismatch in source tree, parent exact head, artifact digest, authority flags, or receipt schema invalidates the result and stops promotion.

## E.20 delivery-consistency qualification — 2026-08-24

本条记录文档交付一致性，不提升任何 runtime 或 production 权限。canonical plan/index 位于 Dropbox 根目录；
`hepta-vnext-qualification-2026-08-23/` 中的同名文件是 byte-identical archival mirror。mirror 只有在
独立 verifier 通过后才标记 `MIRROR_SYNCED`，否则必须 `MIRROR_STALE`/fail-closed；历史 receipt、sealed
worktree 与各自 `SHA256SUMS` 不被覆盖。

| 对象 | canonical SHA-256 | mirror SHA-256 | 状态 |
|---|---|---|---|
| development plan | `25cf0123705f41a7808173f7232bd76f7a6cbb1c83482eac9733c065a1dd3e55` | `25cf0123705f41a7808173f7232bd76f7a6cbb1c83482eac9733c065a1dd3e55` | `MIRROR_SYNCED` |
| qualification index | 由外部 manifest 记录（避免自引用） | 由外部 manifest 记录（避免自引用） | pending verifier |

Effective-version crosswalk is E.20.0 in the plan: v1.3 is historical baseline; H5/H6/H7 map to H5a/H5b,
H6a/H6b, H7a/H7b; H8 production maps to H8a + S5; H9 fleet and L3 topology are separate (S6). This index remains
shadow/qualification-only; `production_writer`, `effect_dispatch`, `model_inference`, `npu_connected`,
`operator_acceptance`, `callers_promoted`, `g5_allowed`, and `promotion` remain false.

Independent verifier/receipt/manifest:
`hepta-vnext-qualification-2026-08-23/delivery-consistency-qualification-20260824/`.

## E.20 lane registration and final-index freeze — 2026-08-24

本条把 v1.4 contract-closure 与 model/longitudinal protocol lane 登记到最终索引；它们的结果类别严格
分开，不能把 blocked prerequisite receipt 当作 efficacy。此 append 之后冻结 plan/index；任何 receipt
若绑定本条之前的 index digest，必须在冻结后的 digest 上独立重跑并重新生成，不得手改 receipt。

| Lane | Result class | Receipt / SHA-256 | Tests / evidence | Authority |
|---|---|---|---|---|
| `v1.4-contract-closure-qualification-20260824` | `PASS_CONTRACT_SEMANTICS_SHADOW`（contract semantics only） | `CONTRACT-CLOSURE-RECEIPT.json` / `856ebde826c51a697a3d0dcd437a2cdfb2d018d6c595d5ea7676d0fde2c63749` | 10 bounded checks；golden/verifier/SHA | production/effect/model/NPU/operator/promotion all false |
| `model-eval-longitudinal-qualification-20260824` | `BLOCKED_MODEL_AND_LONGITUDINAL_PREREQUISITES`（protocol fixture；not efficacy） | `MODEL-EVAL-LONGITUDINAL-RECEIPT.json` / `bfd1aa719639f3c03db354bd57a55a66e56ea2dd47d14cae910bd64c717d1264` | synthetic calibration/longitudinal accounting only；real model/corpus/NPU/OPE absent | production/effect/model/NPU/operator/promotion all false |
| `delivery-consistency-qualification-20260824` | `PASS_DELIVERY_CONSISTENCY_SHADOW`（document authority only） | `DELIVERY-CONSISTENCY-RECEIPT.json` / `7c94b42fdca530cf7b71d796a381a5784468ed0c7f7f27577f314e6b980292cb`（pre-freeze; rerun after append） | plan/index mirror byte equality; current compat manifest 9/9 | runtime authority false |

Pre-append canonical index SHA-256 was
`389e9b458425ffbaf1ffeac9e3e4b5ce0cab0ac874c3b2e5d15add40307cdcab`; the post-append SHA is emitted by the
external delivery verifier and becomes the sole index binding for subsequent receipt reruns. The plan SHA remains
`25cf0123705f41a7808173f7232bd76f7a6cbb1c83482eac9733c065a1dd3e55`. No source, G4/G5, CALLERS, production
writer/effect, model/NPU, operator acceptance or promotion was changed.

## E.20 lane digest correction — 2026-08-24

The receipt/hash values in the immediately preceding lane-registration table are retained as
**pre-final-index bootstrap references** for append-only lineage. They are not the current artifact
digests after the final-index binding and must not be used to assert a live receipt. Current
post-freeze receipt files, their SHA-256 values, source bindings, and verifier output are recorded
only in the external delivery-consistency manifest under
`hepta-vnext-qualification-2026-08-23/delivery-consistency-qualification-20260824/`.
This separation avoids a self-referential index/receipt hash cycle: the index is the document
authority, while the external manifest is the current artifact-digest authority. Any receipt whose
`source_binding.index_sha256` differs from the manifest's final index SHA is stale and fails closed.
No prior receipt, sealed worktree, or production authority flag is overwritten.

## Local-development profile — 2026-08-24

为避免把本地实现错误地卡在外部协作者输入上，新增一个与 production authority 分离的
`local_development` profile。它不需要 provider owner handoff、独立 SSHSIG、900 秒 challenge、
CALLERS production ratchet 或 promotion ceremony；这些只属于可选的 `production` profile。

| 字段 | local-development 值 |
|---|---|
| `g5_local_complete` | `true`（仅代表当前 bounded candidate 可继续开发） |
| `local_operator_acceptance` | `true`（交互用户确认，非独立签名） |
| `local_fleet_shadow_allowed` | `true` |
| `production_activation` | `false` |
| `g5_allowed` / `operator_acceptance` / `promotion` | `false` |
| `provider_physical_exactly_once` | `false` |
| `external_inputs_required_for_local_development` | `[]` |

本地 provider 语义仍是 at-least-once + `Indeterminate → reconcile`，未知结果不得盲重试，外部
effect 默认关闭。该 profile 是 declaration-only metadata，不是 runtime 开关；机器可读 profile：
`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-profile-v11-20260824/`；实现说明：
`G5_LOCAL_DEVELOPMENT_PROFILE_V1.md`。v11 profile SHA-256 为
`1d90cec75d2e1b5f12882e9f2424ac4055d4a6d25cfa2689544e4a592e8ff9fc`，绑定
`e14717e176f6e60976e63416052dda5e859ecad0` /
`fe303f40637e5902191a04daf76e6f5e43613530` /
`ce4d409607fd8d9aef0ae5d4b00cdd0cb7f03e92`（head/tree/parent）；v1–v10 receipts 不被覆盖，
只作为 superseded 历史/production profile 资料。

## Local-development implementation slice — 2026-08-24 01:xx

`local_development` detached candidate 已继续推进到：

| Slice | Candidate commit | Result | Tests |
|---|---|---|---:|
| H4 memory admission | `320d1f65284d2a1811fa13f9645f68601f4c802a` | provisional→verified/tombstone，显式证据 + CAS，KG write=false | 3/3 |
| H4 compact typed shadow | `f4b862be0b9a0a0c203ceeacc20f15b3e3e1b88f` | pre/post hook、fence/revision/loss/rehydration contract；只读 | 6/6 |
| H5 neuron proposal shadow | `140943d7ff9102b1b5b963e489bab006bd15fa0d` | deterministic proposal/abstain；无 KG/routing/effect consumer | 3/3 |
| provider reconcile hardening | `39b201beb4cf048fd804e5a73b625ec83548c16f` | late Accepted→Rejected 保持 Indeterminate，reopen fail-closed | 13/13 + 6/6 |
| automation recovery replay | `140943d7ff9102b1b5b963e489bab006bd15fa0d` | unknown quarantine/reopen 与 crash-after-acceptance | 1/1 + 1/1 |

统一 candidate head/tree/parent：
`140943d7ff9102b1b5b963e489bab006bd15fa0d` /
`197607f337543b88856ab50805ac2149fdd2193b` /
`f4b862be0b9a0a0c203ceeacc20f15b3e3e1b88f`。

slice receipt：`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v3-20260824/`，
SHA-256 `6406ee0d7127d75c7c81bef5acdb527eaf7a68be79fb70e2447e6620044037a0`。
此条目只记录 local/shadow qualification；`g5_allowed`、production writer/effect、CALLERS、promotion
仍为 false。H4 compact 的 authoritative lease/event/outbox、持久 checkpoint、rehydration executor 仍是下一阶段实现项。

## Local-development implementation slice v4 — 2026-08-24 01:xx

当前 local-development candidate 已推进到 H4 compact persistence 与 H6 intuition shadow：

| Slice | Candidate commit | Result | Tests |
|---|---|---|---:|
| H4 compact persistence shadow | `09414ff43deffae3a81dd9ccd9b9366a65e4df07` | append-only/CAS/fence/hash-chain/reconcile contract；无 authoritative writer | 5/5 |
| H6 intuition shadow + recomputation guard | `dbffda2d5edc74f1d83ca3346967dc5dc462dfd6` | deterministic suggest/abstain；拒绝错误自洽 receipt；无 runtime consumer | 7/7 |

统一 candidate head/tree/parent：
`dbffda2d5edc74f1d83ca3346967dc5dc462dfd6` /
`e18526e3a09140cba483659461ad2e35f96d00d3` /
`09414ff43deffae3a81dd9ccd9b9366a65e4df07`。

v4 receipt：`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v4-20260824/`，
SHA-256 `8d951b69a1a2b74a2c76a03b723d35b354a97cbf7d7879552c3cfe4d22e5b5e0`；完整 memory lib `59/59`，
H4 admission/compact/persistence `3/3 + 6/6 + 5/5`，H5 `3/3`，H6 `7/7`，provider `13/13 + 6/6`，
automation `1/1 + 1/1`。v4 supersedes v3 only for local/shadow implementation tracking;
all production authority/effect/promotion flags remain false. 下一阶段只接本地 authoritative lease/event/outbox、
checkpoint/reopen/rehydration executor 与只读 H5/H6 consumer，不开启 KG write、routing 或 external effect。

## H4 persistence integrity amendment / local slice v5 — 2026-08-24 02:xx

H4 persistence 的 amended candidate 已补全 checkpoint/parent digest binding：schema/namespace、context 与
parent event range、expected revision/state、authority/owner/generation/fencing token、protected refs 均纳入
哈希；late/unknown 结果仍走 `Indeterminate → reconcile`。当前 exact candidate：
`7215d4793aa741af5abfc4da1529125bbe430ce9` / `c3917450eb95dbc1d95a7cf9cd62e1ea918fb114` /
`dbffda2d5edc74f1d83ca3346967dc5dc462dfd6`（head/tree/parent）。

v5 receipt：`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v5-20260824/`，
SHA-256 `65655261ab5f447a518f20ec2a76e278b85a2e0156abe50b976bc0c4c16e8969`，supersedes v4/v3/v2。
focused H4 `5/5`、memory full `59/59`、provider `13/13 + 6/6`、automation `1/1 + 1/1`；生产 authority、
external effect、CALLERS 和 promotion 仍关闭。该 slice 尚未接入 authoritative SQLite/WAL/checkpoint executor。

## Local-development implementation slice v6 — 2026-08-24

| Slice | Candidate commit | Result | Receipt SHA-256 | Tests |
|---|---|---|---|---:|
| H5/H6 shadow advisory consumer | `37697b3ea3d14076aa7a6bd94f77521c59f221d1` | `SHADOW_QUALIFIED` | `9da7c9720c472292119c7a3337cafcfc382a3f2c06d97678136303b1f9b7f12c` | 5 |
| H7 local compact executor | `14e9e2d44333f4f1d686c2827dda2bb47070cbcf` | `LOCAL_PERSISTENCE_QUALIFIED` | `9da7c9720c472292119c7a3337cafcfc382a3f2c06d97678136303b1f9b7f12c` | 4 |

The exact detached candidate for this entry is head `14e9e2d44333f4f1d686c2827dda2bb47070cbcf`,
tree `f5f7143ac155aa8a1efe20529b434f843e11d794`, parent
`37697b3ea3d14076aa7a6bd94f77521c59f221d1`. Full memory regression is `68/68`; provider contracts/evidence
are `13/13 + 6/6`; automation unknown/crash recovery is `1/1 + 1/1`. The H7 executor is local
SQLite-only and read-only on rehydrate; it does not grant production writer, KG, routing, scheduler, or
external-effect authority. This entry supersedes v5 for local implementation tracking only; production
flags and canonical `main-integration` remain unchanged.

## Append-only document digest update — 2026-08-24

After the v6 append, the current plan document digest is
`e3a2065e8fa91b09c6f6e98f8f52fb4891e4d839c9cc80b39a3a2268b11ea7e0`.
The index digest after this entry is recorded by the filesystem hash; older header digests are historical
lineage values and are not overwritten. The qualification mirror remains dataless and is not an evidence source.

## Local-development implementation slice v7 — 2026-08-24

| Slice | Candidate commit | Result | Receipt SHA-256 | Tests |
|---|---|---|---|---:|
| H8 lease/fence + append-only event/outbox | `938174f615d19141588331b08624a30b23e0e925` | `LOCAL_PERSISTENCE_QUALIFIED` | `befcb4d1ee69923eb384db0e294a81d03c4d7e7fed822dfeb10fcda82f4b73bc` | 10 |

Exact candidate head/tree/parent:
`938174f615d19141588331b08624a30b23e0e925` /
`20e9d8d2b9afb8ba86a60c25546c083760e55f8d` /
`d55892f1617f292dadd97a7637fdf05a7d035e3f`.

The v7 receipt is
`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v7-20260824/LOCAL-DEVELOPMENT-SLICE-RECEIPT.json`.
Exact-head verification is memory `78/78`, H8 `10/10`, H7 `4/4`, H5/H6 advisory `5/5`, provider
contracts/evidence `13/13 + 6/6`, and automation crash/unknown `1/1 + 1/1`; fmt/diff checks pass.
The strict head-CAS API is `acquire_local_lease_after_head`; the compatibility generation-only API is
fail-closed. Terminal/unknown occurrence replay is fail-closed, and a queued receipt is never an external
effect receipt. This is local-development/shadow evidence only; production writer, KG/routing authority,
provider effects, CALLERS, promotion and fleet unfreeze remain false. v7 supersedes v6 for local implementation
tracking only.

## Append-only document digest update — 2026-08-24

After the v7 append, the plan and index hashes are recorded by the filesystem; prior digest values remain
historical lineage. The Dropbox qualification mirror is still dataless and is not an evidence source.

## Local-development implementation slice v9 — 2026-08-24

| Slice | Candidate commit | Result | Tests |
|---|---|---|---:|
| H9/H10 local lifecycle + status-aware crash replay | `cfcd2833ccbc73ed7a3e6a172cb400114d6d94e8` | `LOCAL_SHADOW_QUALIFIED` | 84/84 + 5/5 + 3/3 |
| H10 checkpoint rehydration witness | `cfcd2833ccbc73ed7a3e6a172cb400114d6d94e8` | `LOCAL_PERSISTENCE_QUALIFIED` | included above |
| H11 host-supplied H5/H6 digest-only observation | `cfcd2833ccbc73ed7a3e6a172cb400114d6d94e8` | `SHADOW_QUALIFIED` | extension 56/56 |

Exact candidate head/tree/parent:
`cfcd2833ccbc73ed7a3e6a172cb400114d6d94e8` /
`f7c3b739d9899ad85bba4b4a6f4357d86331347a` /
`a2ccd3616b817e90852a566b18f67de77da73594`.

Receipt:
`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v9-20260824/LOCAL-DEVELOPMENT-SLICE-RECEIPT.json`
SHA-256 `150f41ebc0fbadeff549df2103a0d060e9083c59b906081e82197601e7dce0cd`; supersedes v8/v7 for
local implementation tracking only. The candidate remains `local_development_only`: no runtime consumer,
KG writer, routing, scheduler, external effect, provider physical exactly-once, production caller, CALLERS,
promotion, or fleet unfreeze. Rehydration is a durable local witness/read-only plan, not complete state/KG
reconstruction; auto-compact IDs remain outside the durable core lifecycle. The qualification mirror remains
dataless/non-evidence and canonical `main-integration` remains unchanged.

## Local-development permission hardening / slice v10 — 2026-08-24

| Slice | Candidate commit | Result | Tests |
|---|---|---|---:|
| agentd cognitive/KG write boundary | `338c511da160ba341456d9fe8b7a27100030ae05` | `LOCAL_FAIL_CLOSED` | 3/3 |
| H9/H10/H11 local development regression | `338c511da160ba341456d9fe8b7a27100030ae05` | `LOCAL_SHADOW_QUALIFIED` | 84/84 + 56/56 + 5/5 |

Exact candidate head/tree/parent:
`338c511da160ba341456d9fe8b7a27100030ae05` /
`f2d5e6e8f46e4c0882334bc9ecdb11747ce36658` /
`cfcd2833ccbc73ed7a3e6a172cb400114d6d94e8`.

v10 receipt:
`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v10-20260824/LOCAL-DEVELOPMENT-SLICE-RECEIPT.json`
SHA-256 `87f3eed272f96121efeddf41ea5d5119a4baec73fc9842a0a60e623bb7df4ea2`, supersedes v9/v8/v7 for
local implementation tracking. Agentd now forces `features.hepta_cognitive_write=false`; the available local
store is used only for the local lifecycle journal. Production authority/effect flags remain false and the
Dropbox qualification mirror remains dataless/non-evidence.

## Local-development H12 rehydration hardening / slice v12 — 2026-08-24

| Slice | Candidate commit | Result | Receipt SHA-256 | Tests |
|---|---|---|---|---:|
| H12 pure read rehydration view | `f8e4657a8f0736b80ecf2eb342326b2a16300a6f` | `LOCAL_PERSISTENCE_QUALIFIED` | `9a7241d93b5fe0a0c50290c83e2427d67e873fdc419cdbbfff06ffeb9d8b052a` | 6 |
| CompactLease digest/ID revalidation | `f8e4657a8f0736b80ecf2eb342326b2a16300a6f` | `LOCAL_FAIL_CLOSED` | same receipt | 7 |

Exact candidate head/tree/parent:
`f8e4657a8f0736b80ecf2eb342326b2a16300a6f` /
`ed63ff87d35f30086e581617988f7087fdd09a9e` /
`61c0aa8cdfa0af570ef4afb9f9171f86be8ad480`.

Receipt:
`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v12-20260824/LOCAL-DEVELOPMENT-SLICE-RECEIPT.json`
(`9a7241d93b5fe0a0c50290c83e2427d67e873fdc419cdbbfff06ffeb9d8b052a`), append-only supersedes v11/v10/v9/v8
for local implementation tracking. Independent rerun totals are memory `86/86`, extension `56/56`, lifecycle
`5/5`, agentd `3/3`, lease/compact focused `7/7`, rehydration executor `6/6`; cargo check, fmt and git checks
also pass. The read API and lease validation are local-only, do not register a runtime consumer, do not write KG,
route, call a provider, or create an external effect. Production authority flags and canonical `main-integration`
remain unchanged/false. The Dropbox qualification mirror remains dataless/non-evidence.

## Local-development H13/H14 + fence/config hardening / slice v14 — 2026-08-24

| Slice | Candidate commit | Result | Receipt SHA-256 | Tests |
|---|---|---|---|---:|
| H13 explicit host read seam | `633ff76f1618c867f47aedfdfc0d4092e8accb2d` | `LOCAL_READ_ONLY_QUALIFIED` | `50884e1c6e6c0c98bd108ea8fe8d902c57ec4105beb5370d07491b63d6061d7f` | 3/3 |
| H14 local runtime read plan | `633ff76f1618c867f47aedfdfc0d4092e8accb2d` | `LOCAL_READ_ONLY_QUALIFIED` | same receipt | 4/4 |
| compact fence epoch integrity | `633ff76f1618c867f47aedfdfc0d4092e8accb2d` | `LOCAL_FAIL_CLOSED` | same receipt | 87/87 |
| effective cognitive-write config boundary | `633ff76f1618c867f47aedfdfc0d4092e8accb2d` | `LOCAL_FAIL_CLOSED` | same receipt | 1/1 + 3/3 |

Exact candidate head/tree/parent:
`633ff76f1618c867f47aedfdfc0d4092e8accb2d` /
`be55037d0a2779b877932ce03b7fb4d03a0a2d92` /
`31305d853235f810735e74589b6e1f8968728cf8`.

Receipt:
`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v14-20260824/LOCAL-DEVELOPMENT-SLICE-RECEIPT.json`
(`50884e1c6e6c0c98bd108ea8fe8d902c57ec4105beb5370d07491b63d6061d7f`), append-only supersedes v12/v11/v10/v9/v8
for local implementation tracking. Full exact-head checks include memory `87/87`, extension `63/63`, local
lifecycle `5/5`, H13 `3/3`, H14 `4/4`, agentd app-runtime `3/3`, effective-config app-server `1/1`, cargo
check, fmt, and git diff/show checks.

H13/H14 remain explicit, unregistered, read-only local seams: no witness/event/outbox/KG write, routing,
provider call, scheduler, or external effect. Fence epochs are persisted and legacy NULL rows fail closed;
managed/request config layers cannot reopen `hepta_cognitive_write`. The profile is
`local_development_only` with `planning_only=true`, `production_activation=false`, `provider_effects=false`,
and `kg_write_authority=false`; production authority flags, CALLERS, promotion, fleet unfreeze, and provider
physical exactly-once remain false. Canonical `main-integration` remains unchanged; the qualification mirror is
still dataless/non-evidence.

## Local-development bounded replay consumer / slice v15 — 2026-08-24

| Slice | Candidate commit | Result | Receipt SHA-256 | Tests |
|---|---|---|---|---:|
| H15 bounded local replay consumer | `1fb5208c6fe2ae9069223b8cbe03f49f930331e1` | `LOCAL_READ_ONLY_QUALIFIED` | `ac9906396a8cfa113f20ea46b6b07a2472a125865a56b42a6e1ae90ad396e971` | 4/4 |

Exact candidate head/tree/parent:
`1fb5208c6fe2ae9069223b8cbe03f49f930331e1` /
`d4298d043ff8d5a04633570ecde5ff3a7686fa2f` /
`633ff76f1618c867f47aedfdfc0d4092e8accb2d`.

Receipt:
`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v15-20260824/LOCAL-DEVELOPMENT-SLICE-RECEIPT.json`
(`ac9906396a8cfa113f20ea46b6b07a2472a125865a56b42a6e1ae90ad396e971`), append-only supersedes v14/v12/v11.
Exact-head checks: memory `87/87`, extension `67/67`, H15 focused `4/4`, local lifecycle `5/5`, agentd
app-runtime `3/3`, app-server effective-config `1/1`, cargo check, fmt, and git checks all pass.

H15 accepts only a validated, current H14 plan and returns a typed `NotStarted/Complete` observation. It has no
write-capable handle, is not registered or auto-called, and cannot append witness/event/outbox, write KG, route,
reconcile/release, call provider, perform an effect, or start a scheduler/state bus. It remains
`local_development_only`; production activation/effects, provider physical exactly-once, CALLERS, operator
acceptance, promotion, and fleet unfreeze remain false. Canonical `main-integration` is unchanged and the
qualification mirror remains dataless/non-evidence.

## Local-development explicit bounded replay lifecycle observer / slice v16 — 2026-08-24

| Slice | Candidate commit | Result | Receipt SHA-256 | Tests |
|---|---|---|---|---:|
| H16 explicit host-invoked replay lifecycle observer | `a2437ad8908877621aba932f7ba81a224970b6cf` | `LOCAL_READ_ONLY_QUALIFIED` (unregistered, no writes) | `dd6552c6bd7ed2ea86bb74230842a16c7c9f095c373e67c01e878a133683c721` | 8/8 |

Exact candidate head/tree/parent:
`a2437ad8908877621aba932f7ba81a224970b6cf` /
`a5a33df6d771796bf4a5e108695899fedcc5634f` /
`1fb5208c6fe2ae9069223b8cbe03f49f930331e1`.

Receipt:
`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v16-20260824/LOCAL-DEVELOPMENT-SLICE-RECEIPT.json`
(`dd6552c6bd7ed2ea86bb74230842a16c7c9f095c373e67c01e878a133683c721`), append-only supersedes v15/v14/v12/v11.
Exact-head checks: H16 focused `8/8`, extension `71/71`, memory `87/87`, local lifecycle `5/5`, agentd
app-runtime `3/3`, effective-config app-server `1/1`, related `cargo check`, nightly/stable fmt, and
`git diff/show --check` all pass.

H16 is an explicit host-invoked, SELECT/read-only observer. It reuses H14/H13 validation and H15 integrity
checks, returns only `NotStarted/Complete`, rejects auto-compact/payload rebinding and terminal leases, and does
not attach `ExtensionData`, write SQLite/event/outbox/witness/KG, reconcile/release, register a callback, start a
background retry/scheduler, route, call a provider, or create an external effect. The host owns timeout/budget;
the future witness writer remains a separate policy-gated E.16 seam. This entry is local-development evidence only:
production activation, provider physical exactly-once, CALLERS, operator acceptance, promotion and fleet unfreeze
remain false; canonical `main-integration` is unchanged and this qualification mirror remains dataless/non-evidence.

## Append-only document digest update — 2026-08-24 (after E.15)

The plan and qualification index filesystem digests immediately after the E.15 append are recorded as
`44e880c461626c70d5032059322a650fed11a5b126c27c9453ecbf6bdd97594f` and
`a9f98087f6df33da18092b76bb2a2f454ff2f54de7a4701afd933095b197b1cd`, respectively. This note is
append-only; historical values remain unchanged and the qualification mirror remains dataless/non-evidence.

## Local-development bounded lease/fence atomic witness writer / slice v17 — 2026-08-24

| Slice | Candidate commit | Result | Receipt SHA-256 | Tests |
|---|---|---|---|---:|
| E.16 bounded local atomic witness writer + host-owned wrapper | `cbd5175dce6f02e6bdfc2bf33ef900d2c3b07385` | `LOCAL_PARTIAL` (single-transaction local writer; protocol blockers open) | `88b605b7d619aea90cfd9758529972783e627d4ec756ad5cc8cff03fab94495c` | core 5/5; extension 2/2 |

Exact candidate head/tree/parent:
`cbd5175dce6f02e6bdfc2bf33ef900d2c3b07385` /
`1ba59cc75035a313826a5447370f839ae64a4c91` /
`a2437ad8908877621aba932f7ba81a224970b6cf`.

Receipt:
`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v17-20260824/LOCAL-DEVELOPMENT-SLICE-RECEIPT.json`
(`88b605b7d619aea90cfd9758529972783e627d4ec756ad5cc8cff03fab94495c`), append-only supersedes v16/v15/v14/v12/v11;
`SHA256SUMS` validates.

E.16 core opens one `BEGIN IMMEDIATE`, verifies the active Agent-local lease and event/outbox chains plus compact
fence/checkpoint binding, and appends at most one `Rehydrated` event. Fault injection proves rollback before commit;
replay is idempotent. The H16→writer extension wrapper is explicit host-invoked and repeats authoritative checks;
it does not register a callback, attach `ExtensionData`, release/reconcile the lease, write KG, route, call a provider,
or create an external effect. Receipt identity uses H14/H15 framing and malformed digest/NUL input fails closed.

This remains a bounded qualification slice, not a closed lifecycle owner. Open blockers are: lease schema lacks
persisted `authority_epoch/owner_epoch/lease_expires_at`; compact witness rows lack persisted `lease_id` and compact
head/event binding; core/app-server lifecycle owner and a positive policy gate are not registered; and this
qualification-only caller-zero surface adds no production `CALLERS` entry. Production activation, provider physical
exactly-once, operator acceptance, promotion, fleet unfreeze and all G5 authority flags remain false; canonical
`main-integration` is unchanged and the qualification mirror remains dataless/non-evidence.

## Append-only document digest update — 2026-08-24 (after E.16)

The new filesystem SHA-256 values after the E.16 append are recorded below; historical digest notes remain
unchanged.

- plan: `48313375383c548562d21b8da973249b30072ec5d8662bfad9461e3c73b34410`
- qualification index: `89ab03ba7c192864af44447a9bc290cc32950bb1b518367e09ec6223129a0894`

## Append-only digest correction — 2026-08-24

The preceding E.16 digest values identify the pre-value-line snapshot. The filesystem digests immediately before
this correction note (and therefore the values to use for the current append-only state) are:

- plan: `eff92be80b236d19e1340795e2f6ee6881fb0c3800aaed925bba30dae576ef7b`
- qualification index: `5fb4bc85e04a5eaa3c14bbde8270a1466a5f9e782d9190cb28affb5071746840`

## Local-development E.16 schema/policy closure / slice v18 — 2026-08-24

| Slice | Candidate commit | Result | Receipt SHA-256 | Tests |
|---|---|---|---|---:|
| E.16 schema-bound lease and compact witness binding | `540c57d59a` (included in final candidate) | `LOCAL_COMPLETE` | `d78e0db0e2a104f283a6409607336ab380c7116ba6154454d8af51cda6d0b7f1` | core 94/94; atomic 5/5 |
| E.16 explicit local policy and host lifecycle owner | `72ff78f1af` (included in final candidate) | `LOCAL_COMPLETE` | same receipt | ext 74/74; app-server 302/302; agentd 19/19; MCP 20/20 |
| G5 Python ledger formatter drift | `e34051e7ab` (format-only) | `FORMAT_GREEN` | same receipt | `just fmt-check` pass |

Exact final candidate head/tree/parent:
`e34051e7abcdf0560112131e9bddb40e3c4a8b8c` /
`21b139671cb1953ef11507d3fa5d976e4ae8bce5` /
`0d0a1bffcc09c1d24528414f8520712d079f0650`.

Receipt:
`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v18-20260824/LOCAL-DEVELOPMENT-SLICE-RECEIPT.json`
(`d78e0db0e2a104f283a6409607336ab380c7116ba6154454d8af51cda6d0b7f1`), SHA256SUMS verified, append-only
supersedes v17/v16/v15/v14/v12/v11. The schema-bound writer now persists and verifies lease epochs/expiry and
compact lease/head/event binding; legacy/unbound or expired paths fail closed. The policy is qualification-only,
caller-zero, and host-invoked; no callback/scheduler/provider/KG/routing/production caller is added.

All production authority flags remain false (`g5_complete`, `g5_allowed`, `operator_acceptance`, `promotion`,
`fleet_and_automation_unfrozen`, `provider_physical_exactly_once`); canonical `main-integration` is unchanged and
the qualification mirror remains dataless/non-evidence.

## Append-only qualification index digest update — 2026-08-24 (after v18)

The filesystem SHA-256 values after the v18 append are:

- plan: `c839edf98222e4e441162070ea87c8cab8a616314f7156ca0d54e1a0d7a87ea1`
- qualification index: `e113ec1af10447334dd93e7efb40f694413f8e49c8628516c5ea992fa7c8cb77`

This digest note is append-only; historical index entries and digest values are not overwritten.

## Append-only digest correction — 2026-08-24 (after v18 digest notes)

The filesystem digests including the immediately preceding digest notes are:

- plan: `a68e2b7325abf0432a51eaaaf1b065f293b56883045fb2c425fc2aabe6ca910a`
- qualification index: `d6952884c32612eba5b2e9fbe6e5c41da9efa0c5861c745fbd70bec7897afbf9`

## Append-only current-state digest — 2026-08-24

Immediately before this note, the filesystem digests were:

- plan: `dba02894545994e036c65b467a3bbadd06a1e0f683248f1da1b7c7e9a2b29703`
- qualification index: `1a1666328bd88fe1b0222a92876958510f49f01353acebef6634e8455bb83c10`

## E.19 / Architecture & Delivery Plan v1.4 amendment — 2026-08-24

The plan now has an append-only E.19 implementation-contract proposal consolidating the day's Neuron,
small-model, closed-loop learning, topology-plasticity and NDU-boundary decisions. It is a design/planning
amendment, not a qualification receipt and not a production promotion.

| Item | Status |
|---|---|
| Canonical baseline | v1.3 historical sections and all prior receipts remain unchanged |
| New proposal | `PROPOSED_IMPLEMENTATION_CONTRACT_V1_4 / SHADOW_PLANNING_ONLY` |
| Claim ladder | L0 static shadow → L1 observational continual → L2 closed-loop policy learning → L3 governed structural plasticity |
| First implementation slice | `salience_neuron_closed_loop` in a local, reversible sandbox only |
| TopologyProposal | proposal-only; no graph change, permission change or online self-rewrite |
| H5/H6/H7 | existing semantic/fixture/shadow evidence only; runtime prerequisites remain blocked |
| Authority flags | `production_activation=false`, `g5_allowed=false`, `operator_acceptance=false`, `promotion=false`, fleet/effect/KG/model/NPU authority false |

The E.19 plan covers `NeuronSpec/NeuronGraph`, typed causal adapters, model/backend selection, the fast and
slow loops, H5a–H9 sub-gates, support-aware OPE, longitudinal evaluation, TopologyProposal compiler gates,
rollback and claim-language policy. No new runtime artifact or model was installed by this amendment.

Current plan SHA-256 after the E.19 content and correction append:
`1774aa7fcc5451dd8f8f4723eb25a520e1c2e3eb5f957932e8d9770cb2d3906b`.
The index digest after this append is computed externally below; historical digest notes remain unchanged.

## Append-only qualification index digest update — 2026-08-24 (after E.19 / v1.4 amendment)

The qualification index SHA-256 immediately before this E.19 append was
`31dfc29d77ab231e6f74413e13810006a11b8fbfe03ddda4947629c398c09d75`.
The final post-note index digest is delivered alongside the plan SHA and is not self-referenced here.

## Append-only digest correction — 2026-08-24 (after E.19 wording review)

The plan received an append-only contract-wording correction after the initial E.19 digest note. The current
plan SHA-256 is now:

`586b79cae510e51e3fba2d8d7b0560267425d01e04d40283ccd3f31fb3019903`

The index SHA immediately before this correction was
`16ddc2049ffc48aef79b15d344b7ae09d14a031bd40b8fbad01e1df3be00df77`.
The final index SHA is emitted by the external delivery check; historical values remain unchanged.

## Append-only digest correction — 2026-08-24 (after E.19 authority/loop wording review)

The plan's final post-correction SHA-256 is:

`b90bbe670a00f36d284221a8fcb75acad328cf49d5f4087c8d860347f20ced90`

The index digest immediately before this correction was
`b89c6d13a3e506fb02b4b060ecce41530407d9a8c234a944d80bca271d59b516`.
The final index SHA is verified externally; historical entries are not overwritten.

## E.21 Contract-hardening and safe-blocker qualification — 2026-08-24

This append registers the E.21 shadow lanes and their claim boundaries. The
external E21 delivery manifest is the only current digest authority; receipt
hashes are intentionally not copied into this index because they bind to the
current plan/index snapshot and must be regenerated after an append.

Current effective plan pointer (before this index append):
`810ff86215772d6bd8df248401aac313e0dbdd937a50861a1bc0ec9ce05c71b5`

| Lane | Directory | Result | Scope | Production authority |
|---|---|---|---|---|
| E21 canonical contract | `e21-contract-hardening-qualification-20260824` | `PASS_E21_CANONICAL_CONTRACT_HARDENING_SHADOW` | strict Event/Trajectory union, authority, CAS/fence, digest, reconciliation, graph and hash-chain semantics; 22 unit + 18 mutation checks | false |
| E21 model verifier | `e21-model-verifier-qualification-20260824` | `PASS_MODEL_VERIFIER_HARDENING_SHADOW` | full nested Draft 2020-12 validation; 4 nested mutations; synthetic ECE negative classification | false |
| E21 S3a runtime seam | `e21-s3a-runtime-qualification-20260824` | `PASS_E21_S3A_RUNTIME_SHADOW` | deterministic typed input→NeuronSignal→advisory DecisionReceipt→read-only ActivityReceipt→observation trajectory; bounded graph planning | false |
| E21 delivery hardening | `e21-delivery-hardening-qualification-20260824` | `PASS_E21_DELIVERY_HARDENING_SHADOW` | canonical/mirror, attachment, history, authority and lane-manifest audit | false |

The effective-version machine index is
`HEPTA_EFFECTIVE_VERSION_INDEX_V1_4.json`; directory authority is
`HEPTA_QUALIFICATION_AUTHORITY.json`. The current external manifest is
`e21-delivery-hardening-qualification-20260824/E21-CURRENT-MANIFEST.json`.
The package mirror remains archival-only and must be byte-identical to the
canonical plan/index. `freeze_for_receipt_binding_at` is an append-only
receipt snapshot, not a ban on later append-only corrections.

E21 remains `L0_BASELINE_L1_SHADOW_ONLY`. It does not prove production runtime,
L2 closed-loop efficacy, L3 topology plasticity, real model/NPU execution,
operator acceptance, CALLERS promotion, G5 unfreeze, or biomimetic neural
mechanisms. The previously listed bootstrap receipt rows are immutable
`HISTORICAL_BOOTSTRAP_ONLY` references; machines must use the external current
manifest pointer above.

### E.21 current digest pointer

After this append, the canonical plan/index SHA values and all E21 receipt
bindings are recomputed by the external delivery verifier. Any receipt whose
`source_binding.plan_sha256` or `source_binding.index_sha256` differs from that
manifest is stale and fails closed. No historical receipt or sealed worktree is
rewritten.

## E.22 Runtime-integration safe closure — 2026-08-24

本条登记一个隔离的、无生产权限的 runtime-integration qualification lane，目标是把 E.21 之后仍可安全
推进的 SQLite trajectory/outbox 与 H5→H6 typed causal adapter 做成可重放证据。它不是产品 runtime 注册，
也不改变当前能力声明。

| Lane | Directory | Result | Scope | Production authority |
|---|---|---|---|---|
| E22 SQLite/causal adapter | `e22-runtime-integration-qualification-20260824` | `PASS_E22_SQLITE_CAUSAL_ADAPTER_SHADOW` | file-backed SQLite WAL/FULL、atomic event+outbox rollback、CAS/fence/dedupe/reopen-rebuild、typed H5 Signal/ModelReceipt→H6 adapter；10 tests + strict nested schema/mutations | false |

E22 的 adapter 只从已验证的 `NeuronSignal + ModelReceipt` 推导 H6 features，拒绝 caller-supplied synthetic
features；DecisionReceipt 仍 `advisory/execute_allowed=false`，ActivityReceipt 仍 read-only/observe-only。
它没有接入产品源码、Codex/App Server、CALLERS、scheduler、CognitiveStore、KG、provider、tool 或 effect。
临时 SQLite 文件是 qualification 实现，不是 authoritative Agent-local production writer。

E22 同时把以下外部门明确保留为 blocked：真实产品 writer/migration/multi-process crash recovery、真实模型
sidecar/Core ML/CPU/NPU、H4 MemoryAdmission outbox/Saga、真实 effect/postcondition/feedback、跨周/月 corpus/OPE/
retention/no-regression、operator/CALLERS/G5/promotion，以及 L3 topology compiler/ablation/lesion。机器当前入口
仍只有 E21 external current manifest；历史 V3/H5–H7 receipts 不得覆盖新 wrapper。

整体 claim 继续为 `L0_BASELINE_L1_SHADOW_ONLY`；E22 不证明 L2 感知—行动 efficacy、长期学习、自我进化、L3
结构生长或仿生神经机制。所有 production writer/effect/model/NPU/operator/CALLERS/G5/promotion flags 保持
false。

## E.23 Upstream Codex integration / local blocker closure — 2026-08-24

本条是 append-only 的 current local integration pointer，不是 production、H8 或 H9 promotion。它绑定根计划
追加 E.23 后的 plan SHA，并把当前 source/head/tree、上游提交和可复核 receipt 指向同一个 exact artifact。

| Lane | Directory | Result | Scope | Production authority |
|---|---|---|---|---|
| E23 upstream Codex integration | `hepta-upstream-integration-846229-20260824` | `LOCAL_INTEGRATION_ONLY` | upstream `e3609f2d02` merged into isolated integration lane; queue recovery, semantic memory-review, provider payload binding, deterministic SQLite reopen stress, and macOS stable authorized-read seam | false |

Current source binding:

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head/tree: `8462290cba24e3f0b7d9b5ee7a9118091b2ba5ff` /
  `3951803160eec7ca6b0b9fc2f6708c1615274d51`
- parent: `2e6697caf1e562e465c57e568dbdb969a51e1a3b`
- upstream main: `e3609f2d02a5896c391fa4c07335165c9272b686`
- exact receipt: `hepta-upstream-integration-846229-20260824/INTEGRATION-RECEIPT.json`
- binary SHA-256: `e1c7d7b641aa374e631bbe7c0ef3eebb8e831b33e0603482b65e4333fd61d4ee`
- current root plan SHA-256 after E23 append: `0319dcaf0c73ec9545d68e9118e74dbfd002c69ab7f4532126fd4235653cee42`
- index SHA-256 immediately before this E23 append: `2104bac0705f510956ddddc41eafd824f5ace8b260b1663455ddf6cf49e4ce18`

Verification summary: app-server 308/308, MCP 20/20, memory 108 + 1 ignored, memory extension 76/76, agentd
memory-review 1/1, two-agent recovery 1/1, queue unit/integration 6/6 and 35/35, provider-effect 7/7,
exec-server 265/265, stable authorized-read 11/11, Unix authorized-read 4/4, fmt/diff checks passed. The
deterministic 1,000-operation SQLite stress is not evidence of 1,000 real host restarts.

All production/effect/KG/model/NPU/operator/CALLERS/promotion/fleet flags remain false. The package mirror,
`HEPTA_EFFECTIVE_VERSION_INDEX_V1_4.json`, and `HEPTA_QUALIFICATION_AUTHORITY.json` are still dataless locally;
until hydration is independently verified, they are `MIRROR_STALE/non-evidence`. Historical `MIRROR_SYNCED`,
old digest rows, E21 pointers, and prior receipts remain immutable historical records and must not be used as the
current source binding. Upstream-deleted authorized-read RPC/capability/remote provenance, physical provider
exactly-once/status reconcile, independent trust/operator acceptance, real Agent writer/H4-H7 runtime, H8 and H9
remain blocked.

## Append-only E23 digest correction — 2026-08-24

The root plan received the E23.4 external evidence digest note. Its current post-append SHA-256 is
`fd96aa651cdc2db719044d5116a59628d0ae4dae653f6ef3a8832e9e4c6201d9`.
The E23 receipt SHA-256 is
`5a7e2859e6ffc14adf313e9796ae06b29486fe8a90f91dd69b66912be3ab73f6` and its `SHA256SUMS` SHA-256 is
`4c5cd986668dab293a6c1c0a74365ee668b61e02f12a8f4143f3298bc30e65e6`.
The index SHA immediately before this correction was
`ebcf93b1c78d0aacd47a5e477e9c6e7c3845ff73dcf70c43a116e94eca682da6`; the post-correction index digest is
emitted externally and is not self-referenced here.

## E.24 Upstream latest + local blocker hardening — 2026-08-24

| Lane | Directory | Result | Scope | Production authority |
|---|---|---|---|---|
| E24 upstream/latest integration | `hepta-upstream-integration-de1c1-20260824` | `LOCAL_INTEGRATION_ONLY` | upstream `80cce09d05`; provider late-uncertainty reopen guard; qualification-only CognitiveRuntime available-store gate; local gated authorized-read RPC; exact release artifact | false |

Current source binding:

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head/tree: `de1c1c4d6eefa619d720f9c160eca461586f79d7` /
  `bb753af815c81a0d78f3ff110e7a39aae93c7a2d1`
- parent: `d6283990d6f811a647b0232f7d24a41818e6d6b1`
- upstream main: `80cce09d059780528e59353ab3d87e4c97d1e944`
- exact receipt: `hepta-upstream-integration-de1c1-20260824/INTEGRATION-RECEIPT.json`
- release binary SHA-256: `65cc95a7eb6de6ab806b427110f6b84135a100cbd9637ab1783dcdf4c38e3dc5`
- receipt SHA-256: `e418be379aadd59a7e70c22160f01b7074f3308559d0523dd55c9d33b33bbcdd`
- `SHA256SUMS` SHA-256: `04323e0377b0b256739d81e8d194763ee0bc1384b3c0f97de1ad8f70dea25ce5`

Verification summary: memory `108 + 1 ignored`, extension `76/76`, app-server `308/308`, MCP `20/20`, evidence
`59/59`, automation `15/15`, exec protocol `21/21`, exec-server `265/265`, authorized-read RPC `1/1`, accepted
websocket `5/5`, Guardian V2 `13/13`, default memory-review `1/1`, qualification writer/isolation `1/1`, runtime
profile symmetry/fail-closed gate `1/1 + 1/1`, queue `6/6 + 35/35`, provider effect `7/7`, stable handle `11/11`,
Unix authorized-read `4/4`, local lease/outbox `17/17`, lifecycle owner `5/5`, extension registration `1/1`, selected
plugin projection/binding `1/1` each, two-agent recovery `1/1`, and deterministic SQLite stress `1/1`. Formatting and
diff checks pass with only existing nightly warnings.

E24 remains local qualification only. The authorized-read RPC/capability is local-only; remote filesystem/skills and
remote hardlink provenance remain separate. Provider physical exactly-once/status reconciliation, independent trust
signer/operator/CALLERS/promotion/root seal, host-bound authority/owner-epoch turn lifecycle, real Agent writer/outbox
and Codex hook, H4–H7 runtime closure, 1,000 real host restarts, H8 and H9 remain blocked. All production/effect/KG/
model/NPU/fleet flags remain false. The package mirror and authority/effective-index placeholders remain
`MIRROR_STALE/non-evidence` until hydration is independently verified.

## E.25 Semantic tombstone propagation — 2026-08-24

| Lane | Directory | Result | Scope | Production authority |
|---|---|---|---|---|
| E25 read-only semantic memory | `hepta-upstream-integration-15bf080-20260824` | `LOCAL_INTEGRATION_ONLY` | host-owned tombstone propagation through real Agentd/App Server/SQLite before and after reopen; no cognitive write tools | false |

Current source binding:

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head/tree: `15bf080523b430c1c472b69557a6cc0a7e82d519` /
  `e81ad773a0c064e27e7c7372a671d7d1d67ae7ec`
- parent: `de1c1c4d6eefa619d720f9c160eca461586f79d7`
- upstream main: `80cce09d059780528e59353ab3d87e4c97d1e944`
- exact receipt: `hepta-upstream-integration-15bf080-20260824/INTEGRATION-RECEIPT.json`
- release binary SHA-256: `65cc95a7eb6de6ab806b427110f6b84135a100cbd9637ab1783dcdf4c38e3dc5`
- receipt SHA-256: `5e4ff0f7155f1c9aa1fcc1e0f5418afce4ae15d9134955773757528b688e4635`
- `SHA256SUMS` SHA-256: `dc2b1c0b72bbeee8a115b95ef56617240432b9439487d8b10a119bc91474f3bd`

Verification: the exact-head default-profile tombstone/reopen E2E is `1/1` in `16.51s`; it verifies the initial
assistant response/citation/source/revision, host-owned tombstone revision `1 → 2`, absence of all three cognitive
write tools, no attachment after withdrawal, and no attachment after Agentd kill/reopen. The E24 package gates and
focused checks remain valid on the parent chain; this test-only slice leaves the release binary hash unchanged.

This is not production/H8/H9 evidence. Host-bound authority/owner-epoch turn context, authoritative writer/outbox
and Codex hook, resource/duplicate-callback soak, H4–H7 runtime closure, provider physical exactly-once/status
reconcile/provenance, independent trust/operator/CALLERS/promotion/root seal, 1,000 real host restarts, remote
authorized-read provenance, H8 and H9 remain blocked. The mirror and authority/effective-index placeholders remain
`MIRROR_STALE/non-evidence`; all production/effect/KG/model/NPU/fleet flags remain false.

## E.26 Active lineage and normalized-wrapper pointer — 2026-08-24

This append is the sole active source pointer after E25. Earlier top-level SHA values and `MIRROR_SYNCED` rows are
historical bootstrap records and must be interpreted in a `HISTORICAL/SUPERSEDED` namespace; they are not edited in
place.

| Field | Current value |
|---|---|
| `claim_level` | `L0_BASELINE_L1_SHADOW_ONLY` |
| `evidence_class` | `LOCAL_INTEGRATION_ONLY` |
| `runtime_authority` | `false` |
| `efficacy_status` | `QUALIFICATION_SEMANTIC_ONLY` |
| `approval_state` | `NOT_APPROVED` |
| `source_binding.head` | `15bf080523b430c1c472b69557a6cc0a7e82d519` |
| `source_binding.tree` | `e81ad773a0c064e27e7c7372a671d7d1d67ae7ec` |
| `source_binding.artifact` | `hepta-upstream-integration-15bf080-20260824/` |
| `receipt_sha256` | `5e4ff0f7155f1c9aa1fcc1e0f5418afce4ae15d9134955773757528b688e4635` |
| `sha256sums_sha256` | `dc2b1c0b72bbeee8a115b95ef56617240432b9439487d8b10a119bc91474f3bd` |
| `canonical_plan_sha256` | `1f7ea043155abae2b8154368878af46461570f0fbe764edc5a93385dd1d1e923` |
| `mirror_status` | `MIRROR_STALE/non-evidence` |

The intended local Dropbox root is `/Users/qianqi/Library/CloudStorage/Dropbox/OpenClaw`; Linux `/home/qian-qi/...`
paths and the non-hyphenated `hepta-vnext-qualification-20260823` name are historical/non-authoritative references.
The authority/effective-index/current-manifest files remain dataless, so this pointer must fail closed if those files
cannot be hydrated. E25's parent-chain package gates are `REUSED_PARENT_EVIDENCE`; the new E25 tombstone/reopen
semantic test is the only fresh check on the 15bf head. All production/effect/KG/model/NPU/fleet flags remain false.

## E.27 Active current candidate — 2026-08-25 (Asia/Shanghai)

E27 supersedes E26 as the sole active local pointer. E20/E21 historical `MIRROR_SYNCED` and stale SHA rows remain
append-only historical records and must not be selected by a machine reader.

| Field | Current value |
|---|---|
| `claim_level` | `L0_BASELINE_L1_SHADOW_ONLY` |
| `evidence_class` | `LOCAL_INTEGRATION_ONLY` |
| `runtime_authority` | `false` |
| `efficacy_status` | `QUALIFICATION_SEMANTIC_AND_RESTART_SOAK_ONLY` |
| `approval_state` | `NOT_APPROVED` |
| `source_binding.head` | `fad3be113b48382102f5b375c894c77758860984` |
| `source_binding.tree` | `a4a76c65cb2ed747b7cb9af2eddf47a50c885490` |
| `source_binding.parent` | `95423c4e76ba5c8f19f229d106e214e42f5d0c98` |
| `source_binding.upstream_main` | `80cce09d059780528e59353ab3d87e4c97d1e944` (verified local) |
| `source_binding.artifact` | `hepta-upstream-integration-06ecdf-20260824/` |
| `receipt_file_sha256` | `47e70c1b10437c45ef1c7d8bfd51a00562a9a38ba97366cd76295f4fd7649264` |
| `receipt_sha256` (canonical bytes with self-field null) | `c537204499b1bd6501fc34457dc676f54cadadb88a1f948e2970caa8c7974bbc` |
| `sha256sums_sha256` | `037a627fabb3ecd459bbc68cfea706c54b803d021907ff4b1c20f30441bea9b8` |
| `binary_sha256` | `50b8a9ce9e7bfac2cac28b1bd9dea293e76bcd341395f87568eb69bc6532a70c` |
| `canonical_main` | `7ed9c9a85fa65aa3cb26cf440a55028ce0b35079` (unchanged) |
| `mirror_status` | `MIRROR_STALE/non-evidence` |

Fresh focused gates on E27: H4 `1/1`; H5→H6 `1/1`; H7 artifact reload/rollback `2/2`; duplicate replay `1/1`;
remote authorized-read `14/14`; GPT-5.3-Codex-Spark read-only transport `1/1` (28.40s); and a real sequential
child-process 1,000-operation kill/reopen/replay soak `1/1` (647.59s, four kill stages, 3,000 events/hash-chain,
1,000 committed-state witnesses). The final memory package is `116 passed, 2 ignored`; other parent package counts are
marked `REUSED_PARENT_EVIDENCE`, not a fresh whole-tree claim. The standing Spark authorization is bounded to development/qualification tests and does not open tools,
effects, shared KG, routing, fleet, CALLERS, promotion, or production.

E27 remains blocked on provider physical exactly-once/status reconciliation/effect provenance, independent trust and
operator/promotion/root-seal authority, production host-bound epoch/power-loss semantics, authoritative writer/outbox
and Codex hook, H4–H7 production closure, remote provenance beyond fail-closed transport, H8, and H9. All negative
production/effect/KG/model/NPU/routing/fleet flags remain false.

## E.41 current phase crosswalk — 2026-08-26

The canonical plan's E.41 is the latest scheduling interpretation for this
index. Until an explicit final-release admission, use:

```text
active_profile=development_internal_test
phase=DEVELOPMENT
allowed_phases=DEVELOPMENT,INTERNAL_TEST,RELEASE_PREP,FINAL_RELEASE,POST_RELEASE
development_blockers=implementation_backlog_only
external_inputs_required_for_development=[]
provider_provenance=release_only_deferred
external_trust_and_signer=release_only_deferred
h4_physical_power_and_media=release_only_deferred
h8_h9_production_governance=release_only_deferred
decision=CONTINUE_PRE_RELEASE
```

The `BLOCKED_*_PREREQUISITES` labels in the historical E.23–E.27 entries are
qualification or release-readiness observations; they do not block another
unreleased feature head, local sandbox, internal test, or crash/reopen work.
Only `FINAL_RELEASE` may convert the four external categories into
`BLOCKED_FINAL_RELEASE`. All production-negative flags remain false, and old
receipts/hashes stay immutable.

## E.42 AuthBus/Basil phase crosswalk — 2026-08-26

`AUTHBUS-PLAN-2026-08-26` registers the new Hepta AuthBus implementation lane. It is a thin
downstream Basil fork plan, not a runtime authority claim and not a rebind of the E.41 current
package.

| Field | Current value |
|---|---|
| `canonical_module` | `AUTHBUS-PLAN-2026-08-26` |
| `parent_plan_sha256_before_append` | `ca5e28fc5b6a7bac8ca4c156dc7ce389f9320161081910b42f9ec01007ba244c` |
| `fork_model` | `thin_downstream_fork` |
| `upstream_repo` | `https://github.com/openbasil/basil` |
| `upstream_commit` | `1fd29adb8e7356968eacbff9309e056cec9bafd7` |
| `upstream_workspace` | `0.7.2 main snapshot; public latest v0.7.1` |
| `upstream_license` | `Apache-2.0` |
| `lane_status` | `PLANNING_ONLY_IMPLEMENTATION_BACKLOG` |
| `active_profile` | `development_internal_test` |
| `phase` | `DEVELOPMENT` |
| `decision` | `CONTINUE_PRE_RELEASE` |
| `stages` | `B0–B10 implementation overlay / AUTH-0–AUTH-10` |
| `implementation_stage_source` | `AUTHBUS.10 + AUTHBUS_STAGE_MATRIX_v1.yaml:implementation_overlay` |
| `external_inputs_required_for_development` | `[]` |
| `release_only_deferred` | provider contract, KMS/HSM, physical H4, operator/public settlement |

Current source/attachment digests:

| File | SHA-256 |
|---|---|
| `hepta-vnext-development-plan-final-2026-08-23.md` | `2616e5aadbc789bd95af0d2ed65f4086f3dcc467e404fa1c45cb05c513d48f80` |
| `hepta-authbus-basil-fork-plan-2026-08-26.md` | `2a5d0952b63e4d208ceed9ac8b945adaf36ba6f39e4afe0d637f16e09efe7d6d` |
| `AUTHBUS_STAGE_MATRIX_v1.yaml` | `84a09bec4a81001a7858ed38daa933e9a8d6b8f51bd945c8ae4dc4189e44c4f2` |
| `hepta.authbus_receipt.v1.schema.json` | `5074dccc09e221e9c7806be51075bf284fc8977dc32ddbfcd4973aca4f349a3a` |
| `HEPTA_AUTHBUS_BASIL_UPSTREAM_POLICY_V1.md` | `c7b13f770d1547c6b10900956ed0ceb6c3752d2cc52214f0291124547903e72e` |
| `AUTHBUS_TRUST_MODE_MATRIX_v1.yaml` | `023ced6c982e83941663c9a086307a61c0fec6a3d6e585f297618d26b8b6531f` |
| `AUTHBUS_RESOURCE_QUOTA_METERING_CONTRACT_v1.yaml` | `8522265b77677388bd4d326173b619620a2d9a144292d74382f7ec496d6c161a` |
| `AUTHBUS_FAILURE_RECONCILE_STATE_MACHINE_v1.yaml` | `18e1bf82066db366d9e95def9dbb45c2cdd9f9db0a821ccf0fdd4736147582e4` |
| `AUTHBUS_ABUSE_DISPUTE_POLICY_v1.yaml` | `085d727956a1e03fc52706445639cb36266fb3afcf8e6a900e456d838be729a6` |
| `AUTHBUS-PLAN-APPEND-RECEIPT-2026-08-26.json` | `2f1e1c6aa86132cbd956e8045834e1224322e728f5fc577fedf20cf327c6059d` |

E.41 remains the global phase policy. B0–B9 and B10 are implementation backlog and may proceed with
synthetic or loopback resources. Only an explicit `FINAL_RELEASE` turns the listed external
inputs into release gates; missing inputs do not block feature development or internal testing.

The fork keeps Basil's host-local UDS/caller-identity/default-deny/backend/audit boundary and
places Hepta resource scheduling, HNL usage-rights, gateway and market seams in separate crates.
Raw credentials are not exchanged, and OpenBao remains the Go SecretBackend.

Attachments:

- `hepta-authbus-basil-fork-plan-2026-08-26.md`
- `AUTHBUS_STAGE_MATRIX_v1.yaml`
- `hepta.authbus_receipt.v1.schema.json`
- `HEPTA_AUTHBUS_BASIL_UPSTREAM_POLICY_V1.md`
- `AUTHBUS-PLAN-APPEND-RECEIPT-2026-08-26.json`
- `AUTHBUS_TRUST_MODE_MATRIX_v1.yaml`
- `AUTHBUS_RESOURCE_QUOTA_METERING_CONTRACT_v1.yaml`
- `AUTHBUS_FAILURE_RECONCILE_STATE_MACHINE_v1.yaml`
- `AUTHBUS_ABUSE_DISPUTE_POLICY_v1.yaml`

`AUTHBUS_STAGE_MATRIX_v1.yaml` retains its original rows as a historical summary; its
`implementation_overlay` and the AUTHBUS.10 DAG are normative for new implementation work.
This crosswalk is `STALE_SOURCE_BINDING` until a future reviewed implementation head is selected.
It does not modify `HEPTA_EFFECTIVE_VERSION_INDEX_V1_4.json`,
`HEPTA_QUALIFICATION_AUTHORITY.json`, or historical receipts.

## E.43 AUTHBUS.11 contract-crosswalk amendment — 2026-08-26

E.43 is the append-only implementation amendment for `AUTHBUS.11 Contract Crosswalk & CI Rules
v1.2`. It supersedes the implementation interpretation of AUTHBUS.10 while leaving E.41, E.42,
the original append receipt, and all earlier qualification records immutable. This is a
`PLANNING_ONLY / IMPLEMENTATION_BACKLOG` document binding; it is not a runtime authority or a
release approval.

| Field | Current value |
|---|---|
| `amendment` | `AUTHBUS.11` |
| `revision` | `1.2` |
| `supersedes` | `AUTHBUS.10` (implementation interpretation only) |
| `canonical_anchor` | `authbus11-contract-crosswalk-ci-rules-v12` |
| `receipt` | `OpenClaw/AUTHBUS-PLAN-AMENDMENT-RECEIPT-2026-08-26.json` |
| `receipt_sha256` | `2951072b7c032b6dbbd4e0dfec92b3202f4471b9e5b0c260984e1af3bce60775` |
| `receipt_schema` | `OpenClaw/hepta.authbus_amendment_receipt.v1.schema.json` |
| `receipt_schema_sha256` | `376a946ca262272378e7e648ed9ec580de2b9caeda66807d12e2fd18e77da94a` |
| `status` | `PLANNING_ONLY_IMPLEMENTATION_BACKLOG` |
| `index_hash_scope` | index records receipt/post-bindings; index bytes are excluded from receipt hash to avoid a cycle |

The amendment closes the implementation-level consistency gaps found in the review:

- `codex-hepta-contracts.v1.4`/E21 remains the sole public contract owner. The closed E21
  `EffectReceipt` required set is explicit; AuthBus fields live in the canonical envelope/payload
  or the existing TaskFlow-owned `ReconcileDecision` sidecar, never in an added top-level receipt
  field.
- Per-host `hepta-authbusd` is the durable writer for resource/quota/lease/permit and dispatch
  references. `DispatchAttemptStarted` is fsynced before an adapter call; a post-call crash is
  `DispatchUnknownRef` → public `Indeterminate`, followed by lookup-only reconcile. TaskFlow owns
  `EffectIntent`/`EffectReceipt`/`ReconcileDecision`; `ReconcileEvidenceRef` is only an AuthBus
  proposal.
- `UsageRightCounter` has one durable writer (`market-adapter`) with
  `reserved_uses/consumed_uses/remaining_uses`, fenced `ReserveUse`/`CommitUse`/`ReleaseUse`, and
  before/after counter digest and revision witnesses. Walletd/TRNM remains the escrow and
  settlement authority.
- HNL mappings are role-qualified and shared across the four machine attachments:
  `NodeDescriptor.node_id→descriptor_node_id`, `AgentDelegation.parent_node_id→issuer_node_id`,
  `ConnectIntent.initiator_node→local_node_id`, `ConnectIntent.remote_node→remote_node_id`;
  missing, swapped, or widened fields reject.
- Basil remains a thin downstream fork with the `hepta-basil-host-minimal-v1` compile,
  registration, and runtime deny profile. Raw secret get/set/import/export, key creation/private
  key return, unscoped mint, admin/NATS/SDS/SPIFFE, remote invocation, and unknown future RPCs are
  denied; the separate-process bridge trusts only the AuthBus service identity plus an attenuated
  capability.

### E.43 post-amendment bindings

The following digests are the exact bytes bound by the amendment receipt. The old E.42 digests are
the parent snapshot and remain unchanged; the amendment receipt and this index are not included in
`post_bindings` so their mutual reference cannot create a hash cycle.

| File | SHA-256 |
|---|---|
| `OpenClaw/hepta-vnext-development-plan-final-2026-08-23.md` | `4fb8cc7b5d20d9bd07e3aed6352d9457a3f2fa754c6361842922d71538077967` |
| `OpenClaw/hepta-authbus-basil-fork-plan-2026-08-26.md` | `5133e9aecda40f75291df94d52fa9685fd3cb5994f93945b3fe43b12081b7465` |
| `OpenClaw/AUTHBUS_STAGE_MATRIX_v1.yaml` | `25a97bf088ac4ec384131d10a81c99e7d7f2ea14f049388a8f19d5e4a82267fb` |
| `OpenClaw/HEPTA_AUTHBUS_BASIL_UPSTREAM_POLICY_V1.md` | `a1209078e287b9d879fe783fb2bbc37d574eef78a3384efdf44e6ec6859070c6` |
| `OpenClaw/AUTHBUS_TRUST_MODE_MATRIX_v1.yaml` | `7d8db70c06c775b354eb89876d313d549fc2faf7cb402ba46ee4146e6e9ff3af` |
| `OpenClaw/AUTHBUS_RESOURCE_QUOTA_METERING_CONTRACT_v1.yaml` | `565f8df52a7d7217083f532cd33f3b41d19c5adccd03a1a42513c3aa1e9dcd08` |
| `OpenClaw/AUTHBUS_FAILURE_RECONCILE_STATE_MACHINE_v1.yaml` | `f1a993adfa20eb9b66c7610980b4a0361020c4fc31804cde8d216f6c2980c969` |
| `OpenClaw/AUTHBUS_ABUSE_DISPUTE_POLICY_v1.yaml` | `9ea57b07a007ea883a3d87e507251477c0a159f7562862e97a8a7b19a4344883` |
| `OpenClaw/hepta.authbus_receipt.v1.schema.json` | `5074dccc09e221e9c7806be51075bf284fc8977dc32ddbfcd4973aca4f349a3a` |
| `OpenClaw/hepta.authbus_amendment_receipt.v1.schema.json` | `376a946ca262272378e7e648ed9ec580de2b9caeda66807d12e2fd18e77da94a` |

### E.43 validation and phase policy

- strict YAML parsing with duplicate-key rejection: **PASS (5/5)**;
- shared crosswalk projection (`id`, owner, namespace, causal lineage, base mutation fields and
  aliases): **PASS (5/5)**;
- role-qualified HNL source-path mapping: **PASS (4/4; identical mappings)**;
- market lifecycle projection and failure-owner projection: **PASS (exact transition equality)**;
- B0→B10 dependency DAG, active stage reference and pointer resolution: **PASS**;
- amendment receipt Draft-07 validation: **PASS**;
- E.41 prefix byte comparison against the pre-amendment mirror: **PASS** (prefix SHA-256
  `d0a810a5eedea516f5ae85b95a960d32d11d15d522006677a4d7288f9d316038`);
- running AuthBus/OpenBao, effective/authority JSON, and all production/effect/promotion flags:
  **UNCHANGED**.

Development, internal test and release-prep continue under E.41. Provider provenance, external
KMS/HSM, physical H4/media, operator acceptance and public settlement remain
`DEFERRED_PRE_RELEASE`; only an explicit `FINAL_RELEASE` may evaluate them as release gates. No
production key was generated, no real provider effect was called, and no physical power-loss test
was performed as part of this amendment.

<a id="authbus11-execution-closure-v13"></a>
## E.44 AUTHBUS.11 v1.3 execution closure — 2026-08-27

E.44 records the implementation-ready closure for `AUTHBUS.11 v1.3`. It supersedes only the
implementation interpretation of E.43; E.41, E.42, E.43, their receipts, and all runtime
authority state remain immutable. This is a `PLANNING_ONLY / IMPLEMENTATION_BACKLOG` binding,
not behavioral evidence, provider evidence, or release approval.

| Field | Current value |
|---|---|
| `canonical_registry` | `OpenClaw/AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry` |
| `registry_status` | `REQUIRED_AT_B2` / `DESIGN_ONLY` until B2 binds the registry digest |
| `amendment` | `AUTHBUS.11` |
| `revision` | `1.3` |
| `supersedes` | `AUTHBUS.11-v1.2` |
| `receipt` | `OpenClaw/AUTHBUS-PLAN-AMENDMENT-RECEIPT-2026-08-26-v1.3.json` |
| `receipt_sha256` | `2b8e68a3917314795abb7e0f71d39b62859477c315c14dfd807ecc954b296e7e` |
| `receipt_schema` | `OpenClaw/hepta.authbus_amendment_receipt.v1_3.schema.json` |
| `receipt_schema_sha256` | `bc518877a81cc49bfe7f964392beadaf0ebb0ca9bd581f5b82b99b391db42ce1` |
| `status` | `PLANNING_ONLY_IMPLEMENTATION_BACKLOG` |
| `hash_scope` | receipt binds 16 post-state files; the receipt and this index are excluded to avoid a cycle |

The registry is the sole normative v1.3 status/error, E21, serialization, fence, OAuth,
reconcile, outbox, remote-reservation and gateway source. These four narrow contracts are
generated projections and cannot rename, drop or reinterpret registry semantics:

- `OpenClaw/AUTHBUS_OAUTH_SECRETREF_CONTRACT_v1.yaml#/contract`
- `OpenClaw/AUTHBUS_RECONCILE_E21_CONTRACT_v1.yaml#/contract`
- `OpenClaw/AUTHBUS_OUTBOX_DISPATCH_CONTRACT_v1.yaml#/contract`
- `OpenClaw/AUTHBUS_REMOTE_RESERVATION_GATEWAY_CONTRACT_v1.yaml#/contract`

The four existing trust/quota/failure/abuse files remain domain projections. The active stage
selector is `OpenClaw/AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map` and the
implementation order is `B0 → B1 → B2 → B3 → B4 → B5 → B6 → B7 → B8 → B9 → B10`.
`HNL-GATE0-DECISIONS` is explicitly `NOT_READY_FAIL_CLOSED` for B7 and federated B8/B9/B10;
local loopback lanes remain available.

The closure freezes the following implementation rules: opaque `SecretRef` refresh with
lookup-only response-loss recovery; E21 hashed initial receipt/decision sentinels and
`PROPOSED|COMMITTED` NoEffectProof; `DispatchAttemptStarted` fsync before adapter call,
`DirectTerminalAckRef` and `DispatchUnknownRef` lookup-only recovery; `ManualRequired`,
`ReconcileBlocked` and `DispatchUnknownRef` are non-terminal; Basil's exact descriptor routes
and compile/registration/runtime deny profile including key-generation prohibition; componentwise
six-dimensional `UsageVector`; fenced remote reservation; and gateway PoP/replay/SSRF rules.

Static validation for E.44 is recorded in the receipt: strict YAML duplicate-key checking,
JSON-schema validation, projection/status/error/reconcile/outbox/stage crosswalks and historical
terminal guards are asserted PASS; behavioral implementation evidence is `NOT_RUN`.
`E.41` prefix comparison, runtime service state and all production/effect/promotion flags are
unchanged. Provider contracts, external KMS/HSM, physical power/media, operator acceptance and
public settlement remain `DEFERRED_PRE_RELEASE` and are evaluated only by an explicit
`FINAL_RELEASE` selector.

### E.44 post-state bindings

| File | SHA-256 |
|---|---|
| `OpenClaw/hepta-vnext-development-plan-final-2026-08-23.md` | `1ff0f8aabf733e1d43a59e3b8f30281dba4f6814f20d3759b909816f207becce` |
| `OpenClaw/hepta-authbus-basil-fork-plan-2026-08-26.md` | `ba2a6779865ce3a2fccc592bfc17cb688b112e9e4e42940254d58c9818fa6c23` |
| `OpenClaw/AUTHBUS_STAGE_MATRIX_v1.yaml` | `5d98b261e3cd803fd52a06427e341b3fae9269495099bfd6c43b96b3f554b707` |
| `OpenClaw/HEPTA_AUTHBUS_BASIL_UPSTREAM_POLICY_V1.md` | `70ae381bdcbfd0655d392e6ca9ca15728aeef2e44c487a87c135f8a85d4a29f6` |
| `OpenClaw/AUTHBUS_TRUST_MODE_MATRIX_v1.yaml` | `d95c0cbd0162156baa3a6b8b4b6bd5423826c99a90b06c596ffedaa9ed153309` |
| `OpenClaw/AUTHBUS_RESOURCE_QUOTA_METERING_CONTRACT_v1.yaml` | `aa90dbe0c63826dcf951783d858ce2e7ec05a0f554ec119cbc6b2b4a72a0ddf8` |
| `OpenClaw/AUTHBUS_FAILURE_RECONCILE_STATE_MACHINE_v1.yaml` | `c726f52e0109763ed876d23468c4e5552ff27da97f0c86c1e4c6380475a8f30a` |
| `OpenClaw/AUTHBUS_ABUSE_DISPUTE_POLICY_v1.yaml` | `0e92726dab7777b1904e4369c92d506e391765a278757fe0bc6a22e1ae8ada14` |
| `OpenClaw/HEPTA_VNEXT_PROTOCOL_SPEC.md` | `58862b5bb0dc02d2bbad7d74a70efd8a281c3672dd6ae70ea7e88f78cd9361e6` |
| `OpenClaw/HEPTA_TASKFLOW_VERTICAL_SLICE.md` | `0b5f5c35ad9bd454b3304233a7f0270c44437f6644edbaba666474d125c7076b` |
| `OpenClaw/hepta.authbus_amendment_receipt.v1_3.schema.json` | `bc518877a81cc49bfe7f964392beadaf0ebb0ca9bd581f5b82b99b391db42ce1` |
| `OpenClaw/AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml` | `cda83c4776d4c2b3c2851474e476e775d6ca26fa815373083aac47fdfd0c89f5` |
| `OpenClaw/AUTHBUS_OAUTH_SECRETREF_CONTRACT_v1.yaml` | `bd82b1380497de39f484b2f071f611247752459ad1aaf1d8d112fad8680e2d92` |
| `OpenClaw/AUTHBUS_RECONCILE_E21_CONTRACT_v1.yaml` | `fff6dbee24303b8333b23ba084e73d0863a8ef6d8e5f629ae11bbdecca50f5e4` |
| `OpenClaw/AUTHBUS_OUTBOX_DISPATCH_CONTRACT_v1.yaml` | `ff84c2de1b6e8e7197a34252ef7bee524c6691c5c1e8d688985f7932e001995f` |
| `OpenClaw/AUTHBUS_REMOTE_RESERVATION_GATEWAY_CONTRACT_v1.yaml` | `693119ead3c3ed6b8c6027260020374a49ccf56356e99756235524a9f2fc9887` |

The E.44 receipt is the machine-checkable binding for this table. Its `source_closure` keeps
Basil at research-capture status (`1.94.0` capture versus required `1.98.0` B0 toolchain), and
its authority flags remain false. No key was generated, no running AuthBus/OpenBao service was
changed, and no external effect was invoked by this documentation amendment.
