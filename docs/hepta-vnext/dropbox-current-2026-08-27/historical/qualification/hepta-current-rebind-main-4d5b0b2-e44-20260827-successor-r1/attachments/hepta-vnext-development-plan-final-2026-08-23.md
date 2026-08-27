# Hepta vNext 全新开发计划

## Architecture & Delivery Plan v1.3（implementation-spike closure）

**日期**：2026-08-23（Asia/Shanghai）  
**状态**：Architecture v1.1 基线 + Qualification contracts v0.1 + Implementation spikes v0.1 + Learning spec v0.3；本文已加入实施合同与隔离 implementation-spike 收口状态，不等于生产 promotion。  
**目标**：在不破坏 Codex/Hepta vNext 现有安全与验收边界的前提下，形成可持续记忆、上下文压缩、neuron group、NDU 长期学习、稳定 intuition 和 Agent-local TaskFlow 的完整开发路线。  
**基线仓库**：/Volumes/T5/hepta-vnext  
**最新显式计划**：artifacts/roadmaps/r2-final-development-plan-20260821.md  
**本次核对的 exact worktree**：worktrees/r2-g4-matrix-robrix-detached37，HEAD 445d1cdc50c9，工作树 clean。

> 本文是对现有 R2 计划的重新设计，不是把旧模块整棵树合并。所有“已具备”均来自 2026-08-23 的只读核对；所有“目标/新增”均必须经过本文的阶段门、精确 head receipt、CALLERS ratchet 和 operator acceptance。qualification 产物可以证明边界和语义，不自动获得生产权限。

## 0. 本轮 blocker-closure 状态（2026-08-23 19:41）

本轮优先收口的是协议、权威和证据链，而不是提前解冻 production。当前结果：

| 门 | 当前结果 | 权威证据 | 仍未授权 |
|---|---|---|---|
| R2-G4 exact | `qualified_exact`；`g4_complete=true`、CALLERS ratchet 已应用 | `/Volumes/T5/hepta-vnext/artifacts/r2-g4-paired-exact-final40-20260823/PAIR-RECEIPT.json`，SHA-256 `f36ce3f41cc8734f4392070a01ac53cbdf753dee5a1bb8b352feb1bc886e8064` | `promotion=false`、`operator_acceptance=false`、`g5_allowed=false` |
| R2-G5 bounded slices | 六个 slice 已统一到 head `73ff3b438a25…` / tree `4070f421a633…`，聚合验证 `PASS_BOUNDED_AGGREGATE` | `qual-g5-bounded-aggregate-20260823/G5-BOUNDED-AGGREGATE-RECEIPT.json`，SHA-256 `7d178989d75ff4fe32c41fc38737a3d12a0ef0f3da30b6e84ff5c03bf11136a1` | 未 ratchet CALLERS；`g5_complete=false`、fleet/automation 仍冻结 |
| H0/H1 protocol | 独立 shadow qualification fixture 已通过 36 tests、clippy、metadata、fmt、diff-check；不进入 product caller | `qual-h0-protocol-20260823`，最终 receipt commit `74078ea3390bf55bc2ea46fc6e2eb043107319e5`；receipt SHA `c515d55d65a4c5d62c3f171d4e2e5acbd9f95b5d2fc1fcb286ebe88ace15157d` | 仅 schema/fixture；无 authoritative SQLite writer、真实 adapter、migration、promotion |
| H2 workflow | 独立 workflow registry/compiler fixture 已通过 41/41（H2 专测 11/11）、clippy/fmt；`qualified_shadow_only` | `qual-h2-workflow-20260823`，commit `fe9122cf0bff4adc62c321f7591cb0096673c5cc`；receipt SHA `a65d9805fa86db32667a464e115c0e1f7670d1e68b5e8d0abc49f9af0570d17e` | 仅编译/校验；无 scheduler、executor、effect 或 production caller |
| H4 memory/compact | 独立 reference model、JSON Schema、golden、verifier 已通过 10/10；`PASS_H4_MEMORY_COMPACT` | `qual-h4-memory-compact-20260823`，最终 commit `ab67e772c75fc2fb27ba6606ea3aa667d8dd5338`；receipt SHA `b066970c66b0efdc0baa7ceff5097fa89d1e41463375158ac4505d79aef6e936` | 仅 schema/fixture；未接自动 admission 或 Codex compact hook |
| H3 TaskFlow kernel | 独立 deterministic shadow kernel、golden trace、fault/recovery verifier 通过 16/16；`PASS_H3_SHADOW` | `qual-h3-taskflow-20260823`，receipt `PASS_H3_SHADOW.json` SHA `dde17a21ecb5bef1d712762c27c83e2bb3a18de40aedbec61cc2800e3f16ab0c`；tree `86fedfaaf808fc488bf0559df1c4eefb7a389395` | 仅 reference/projection-only；无 Agent-local production writer、真实 effect 或 promotion |
| H5 neuron group | typed group、共享池/fallback、预算和 provisional fence 通过 11/11；`PASS_H5_NEURON_GROUP` | `qual-h5-neuron-group-20260823`，commit `5c69eeeb10c77fedd2a22eddda180025ce191a3a`；receipt SHA `c54fb6b66db9e600c74fb8afc405c28ab19af23624efa92fad05cd2e83655c22` | 未安装模型、未连接真实 NPU；仅 isolated shadow |
| H6 intuition | hard-filter→rank→tie-break、calibration、abstain、stale receipt gate 通过 13/13；`PASS_H6_INTUITION_POLICY` | `qual-h6-intuition-policy-20260823`，commit `8807e17b24dc2a977a1c0b57f3cea03352b949c9`；receipt SHA `f471025ef0a6c9b907a0af51a154479dbdb5fa676ee1137980d94657066d4337` | 仅 SuggestOnly/PrepareOnly；`execute_allowed=false` |
| H7 NDU learning | position/trajectory/credit/OPE/calibration/drift/forgetting shadow 通过 10/10；`PASS_H7_SHADOW` | `qual-h7-ndu-learning-20260823`，commit `7e8441183cb31a196210300261d0bde22bcbce1b`；receipt SHA `d562ca3b76f1b384979308e7c5df2aafb0fa56bd42b0e085419119382cde174e` | 仅离线 artifact；base weights/topology/authority/promotion 均冻结 |

本状态表将“证明 bounded 语义”和“获得 release authority”明确分开；后者必须另有 CALLERS qualification、operator acceptance 和 promotion receipt。

## 0.1 实施合同 qualification 收口（2026-08-23 20:31）

本轮按上一轮复审提出的 P0/P1 清单，完成了三条完全隔离的 qualification lane。它们只证明协议、schema、阶段依赖和 shadow fixture 的内部一致性；没有把任何产物接入生产 caller，也没有把 H8/H9 从治理阻断状态中解冻。

| lane | exact G4 parent | 结果 | 独立证据 | 权限状态 |
|---|---|---|---|---|
| P0 protocol contracts | `445d1cdc50c9e86d09041b17888245b8c5937bda`；commit `0e84bd1f20d539eade367c3ffb41a063ad28696c` | `PASS_PROTOCOL_CONTRACTS_SHADOW`；12/12；generator 双跑幂等 | `protocol-contracts-qualification-20260823/PROTOCOL-CONTRACTS-RECEIPT.json`；receipt SHA `11f1f080161389b770bce226a94ea1a9a65bd127d36a5f144d9db48359475d64`；golden SHA `41e885337e5094323812a91834345469da4ea7f3772788e401f16a3087407cc6` | production writer/effect/scheduler/model/tool/operator/promotion/G5 全部 false |
| H4–H7 implementation contracts | 同上；commit `ca2cf9fe69a54e1560484c430fa86094f48f132a` | `PASS_H4_H7_CONTRACTS_SHADOW`；12/12；generator/verifier 双跑稳定 | `h4-h7-contracts-qualification-20260823/CONTRACTS-RECEIPT.json`；receipt SHA `63184346bbbe3fdc62e099fef338dc5706897ec0fabfc04a96f1d8d85a016f28`；golden SHA `be6e53ef6a92dfa039f8145169eb2c6f8444f41308bff3d5bd840e3c9bad096e` | memory/KG/model/NPU/production mutation/promotion/operator/G5 全部 false |
| implementation readiness | 同上；commit `ae5009358df7003d1abcf4b23ebc1db0612ddae9` | `PASS_IMPLEMENTATION_READINESS`；5/5；crate graph 无环、SHA 8/8 | `implementation-readiness-qualification-20260823/IMPLEMENTATION-READINESS-RECEIPT.json`；receipt SHA `42f1ab45bfc9a65f409e3ffb25d770cbdf863bf40f02c9a61b9284c24360a8f9` | shadow-only；H8/H9a/H9b/H9c governance blocked |

新增协议合同覆盖：唯一 Agent-local authority/outbox、`RunStartSnapshot`、`LeaseFence`/epoch 生命周期、`ActivityIntent` 与 `EffectIntent` 分型、canonical event payload/rebuild、唯一 wakeup owner、受限 workflow 语义、Indeterminate reconciliation、migration CAS/abort。H4–H7 合同覆盖 memory admission/forget lineage、compact CAS/rehydration、privacy-aware neuron fallback、DecisionReceipt hard veto/coverage floor、NDU propensity/support/OPE/CI 和 artifact rollback。

本轮 qualification 使用 Python/schema verifier，未运行全 workspace cargo build，以避免 Mac 外置盘低空间（约 8 GiB free）造成新的资源风险；该资源限制已写入 readiness receipt。上述 receipts 不能替代后续 Agent-local writer、Codex hook、真实 Core ML/NPU benchmark、脱敏 corpus efficacy、operator acceptance 或 CALLERS promotion。

### 0.2 交付一致性收口

- Dropbox 本地同步状态：`Up to date`。
- 主计划 SHA-256：以 Dropbox 交付时计算的外部 manifest 为准（避免文档自引用）。
- qualification index SHA-256：以 Dropbox 交付时计算的外部 manifest 为准（避免文档交叉自引用）。
- 主计划与 qualification 目录镜像、主索引与 qualification 目录镜像的逐字节关系由 E.20 delivery manifest 验证；manifest 为 `MIRROR_SYNCED` 时才可声明一致，任何同步中断均标记 `MIRROR_STALE` 并 fail-closed。本轮新增的六条 qualification/implementation 目录均有通过的 `SHA256SUMS`（H1/H4/H5–H7 三条 implementation lane 加此前三条 contract lane）。
- 本收口只更新文档/qualification artifact，不改变 canonical G4/G5、`CALLERS.toml`、production caller、模型/NPU 或任何 effect authority。

## 0.3 隔离 implementation-spike 收口（2026-08-23 21:05）

在 0.1 合同 qualification 之后，继续对真正剩余的实现 blocker 做了三条独立、exact-G4 绑定的最小切片。它们仍然不进入 production caller；`PASS_*_SHADOW` 只表示 fixture/接口语义通过，`BLOCKED_*_PREREQUISITES` 则是可复现的生产接入阻断证据。

| lane | exact G4 parent | 结果 | 独立证据 | 当前阻断/权限 |
|---|---|---|---|---|
| H1 Agent-local writer/outbox | `445d1cdc50c9e86d09041b17888245b8c5937bda`；commit `42e958d4d083a0beb19f76e4e5ecaad7110f14ea`；tree `221c2e9b57c9a4f76f80649be97cf295950dd3e7` | `PASS_H1_AUTHORITATIVE_WRITER_SHADOW`；16/16；fmt/clippy/verifier 通过 | `h1-authoritative-writer-qualification-20260823/H1-AUTHORITATIVE-WRITER-RECEIPT.json`；receipt SHA `168fe827b87d136edc70f555d88d0a57fddfaed7547f6b32f6cbd4b7135974cc`；golden SHA `51e523c6ad2bd9dc8dd58c047b6392db9623338461f13aa68c2e03e49e0aae45` | standalone in-memory shadow；real SQLite/production writer/effect authority 仍 false |
| H4a/H4b memory/compact implementation | 同上；commit `0fd584455464071fbfc7d00517db39d9c2c6724d`；tree `b532ce285fd224ccb32eaa25962b98b1fb90663b` | `BLOCKED_H4_IMPLEMENTATION_PREREQUISITES`；12/12；schema/verifier/SHA 通过 | `h4-implementation-qualification-20260823/H4-IMPLEMENTATION-RECEIPT.json`；receipt SHA `b5ab99b0595b8f3ece7eb915f77f383a8194add090c4d79c501f1c9d79ad8342`；golden SHA `9eeaf043118c9e75519fcff9f405ff7ab3d85bcc2ba5be96a32a2b056ce0f5a8` | 缺自动 MemoryAdmission、CognitiveRuntime pre/post compact hook、CognitiveStore candidate writer |
| H5b/H6b/H7a runtime prerequisites | 同上；commit `d0e418e6f1d94b02576bd8dba8d73ef61f40909e`；tree `ab637139d20e4ca9d703b6eae86b6cc9b3f3bc28` | `BLOCKED_H5_H7_IMPLEMENTATION_PREREQUISITES`；8/8；schema/verifier/双跑/SHA 通过 | `h5-h7-runtime-prerequisites-qualification-20260823/BLOCKED_H5_H7_IMPLEMENTATION_PREREQUISITES.json`；receipt SHA `52a46db61b2021de346859ec1ca156d9ea47240004e4353189698e133cfbcad5`；golden SHA `25af7cf8db7d286aa784481439489242ec0eaa967d9cba7ed42669b6e41efb02` | exact G4 无 typed neuron/intuition/NDU/trajectory seam；无模型、sidecar、真实 NPU benchmark |

H1 的 fixture 明确验证了 `RunStartSnapshot`、command dedupe、expected-revision CAS、authority/owner/generation fencing、event/projection/outbox 原子性、故障回滚、at-least-once ack、sticky cancel 和 deterministic rebuild；它没有把 in-memory 语义冒充 SQLite durability。H4 与 H5–H7 的 blocked receipt 把缺口固定成下一批实现 PR 的前置条件，并保持所有 production/authority flags 为 false。

---

## 1. 执行摘要

### 1.1 最终判断

Hepta vNext 应采用五个相互独立、通过 typed contract 连接的平面：

1. **Governance / Safety Plane**：权限、能力、预算、证据、签名、回滚和威胁模型。它是横切硬约束。
2. **NDU Control Plane**：慢速 replay、评测、credit assignment、校准、shadow、canary 和 artifact promotion。NDU 只提出并发布已批准的 policy/model/artifact，不拥有生产执行权。
3. **Hepta Intelligence Plane**：持久 memory、KG、context state、neuron group 和 intuition policy。它负责理解状态、召回证据和选择合法 workflow。
4. **Hepta TaskFlow Runtime Plane**：版本化 TaskFlow 图的编译、持久运行、等待、重试、join、补偿、恢复和 reconciliation。它负责可靠地执行已批准图，不负责事实真伪，也不直接拥有模型/工具权限。
5. **Codex / Agent Execution Spine**：Codex 继续拥有 ThreadManager、CodexThread、ToolRouter、session、rollout、thread store 和实际 model/tool 执行。Hepta 通过 typed extension 和 activity adapter 接入。

### 1.2 关键设计决策

- **TaskFlow 可以内化，但只内化语义和内核，不搬 OpenClaw 实现。** OpenClaw TaskFlow 是 durable flow ledger；Lobster 才是 pipeline DSL/runtime。Hepta 应拥有自己的 Rust TaskFlow kernel，OpenClaw 只保留兼容 adapter 和状态投影。
- **Intuition 不是自由生成的 skill。** Intuition 是“固定、版本化、可验证的 TaskFlow 图”之上的 workflow 选择、分支、阈值、abstain 和恢复策略。
- **Neuron 是 group，不是孤立小模型。** 一个 group 可以由多个 typed neuron、共享模型池和轻量 head/adapter 组成；逻辑上分别绑定 task，物理上共享模型权重和 NPU/CPU 资源。
- **NDU 不做在线自改。** 可学习的是选择、排序、检索、阈值、head/adapter 和模型路由；workflow 拓扑、权限、invariant、CAS、effect receipt 和安全下限必须冻结。
- **长期学习依赖统一 trajectory。** turn → memory → neuron → intuition → workflow → step → effect → feedback 必须形成可追加、可重放、可审计的事件链。
- **稳定性来自固定图、硬约束、可回放、canary 和 rollback，而不是模型更大。**

### 1.3 当前状态的一句话

当前 vNext 已有较强的 Codex 执行基础、治理/evidence、CognitiveStore/KG、bounded recall、Agent generation fencing 和 per-Agent automation lease；G4 final40 已达到 `qualified_exact`，G5 目前只有六个 bounded qualification slice 聚合证据；尚未有进入生产 caller 的 hepta-taskflow、hepta-neuron、hepta-intuition、hepta-ndu 或统一 trajectory/event-ledger。fleet/automation 仍被冻结。

---

## 2. 当前基线审计

### 2.1 现有 R2 计划必须保留的发布脊柱

最新显式计划 artifacts/roadmaps/r2-final-development-plan-20260821.md 规定：

1. G1：Codex execution substrate；
2. G2：两个独立 Agent 进程；
3. G3：Intelligence/KG product loop；
4. G4：Matrix/Robrix；
5. G5：fleet/automation。

这是**发布资格的前置依赖链**，不能因为新增 Intelligence/TaskFlow 设计而跳过。新的架构工作可以在独立分支、shadow 或 qualification lane 中提前准备，但不得在前置 exact receipt 之前合并、推广或宣称完成。

### 2.2 2026-08-23 的实际状态

| 区域 | 已观察到的状态 | 计划含义 |
|---|---|---|
| canonical integration | main-integration 仍是 7ed9c9a85f | 不把 detached candidate 当生产基线 |
| G4 backend/UI | backend exact head 445d1cdc50c9…、UI f818e130f8…，final40 paired receipt 已生成 | `status=qualified_exact`；`g4_complete=true`、caller ratchet 已应用；promotion/operator acceptance=false |
| G4 safety claims | runner revalidation required；kernel execve atomic binding 明确不作强保证 | 这是 fail-closed 安全声明，不是失败；不得把它改成可晋级的假阳性 |
| G5 bounded aggregate | 六个 slice 均为 `qualified_bounded_slice`，common head/tree 一致 | 仅 bounded evidence；无 CALLERS ratchet、无 fleet/automation unfreeze、无 promotion |
| hepta-automation | per-Agent schedule/lease/admission → owning App Server thread/queue/add | 不是 DAG/TaskFlow executor，不直接调用 model/tool/Core |
| hepta-runtime | 最小 live shell；既有 schema-v5 只读 adapter | model invocation、automatic transition、operator mutation、promotion、retirement 默认均关闭 |
| hepta-contracts | identity、governance receipt/decision、memory/provenance、provider intent/terminal | 尚无 workflow/neuron/intuition/NDU/trajectory contract |
| hepta-evidence | append-only governance/provider/memory/channel evidence | 尚未串联完整 turn→memory→neuron→workflow→effect→feedback 轨迹 |
| hepta-memory | CognitiveStore、source ledger、memory revision/CAS、KG facts/projection、FTS、federation、revalidation | 已有持久记忆基础，但无自动 admission/consolidation 与 compact checkpoint/rehydration |
| ext/hepta-memory | bounded recall 和 physical send 前 revalidate；故障移除可选 context | 是 context attachment，不等于 Hepta 专用 compaction |
| agentd | one-process-per-workspace、generation/home/workspace fencing、现有 App Server | 不能再造第二 session/runtime kernel/fleet bus |
| NDU | 当前工作树未接入；旧归档中有 shadow/replay-only continual-learning 设计 | 作为后置 control plane 接入，不能成为 runtime 单点依赖 |

### 2.3 不应被误解为已完成的部分

- G4 的 test_assertions_passed=true 只代表当前 candidate 的测试断言通过，不代表 G4 complete 或 promotion。
- hepta-memory 的 recall attachment 不代表自动把每轮对话写入长期 memory。
- Codex core 的 context compaction 不代表 Hepta 已保存 compact checkpoint、压缩损失报告或可验证 rehydration。
- hepta-automation 的 durable schedule/run 表不代表已有 workflow DAG executor。
- 旧版 hepta-intelligence 归档中的 intuition/neuron/NDU 类型不能直接视为 vNext 生产模块。

---

## 3. 不可破坏的基线与权威边界

以下约束是所有新模块的编译、测试和 review 前置条件。

### 3.1 Codex 是唯一执行脊柱

Codex 继续拥有：

- ThreadManager、CodexThread、session、turn、rollout、thread store；
- ToolRegistry/ToolRouter 和实际 tool dispatch；
- provider/model invocation 和 provider terminal；
- App Server 生命周期、Agent generation 和恢复 authority。

Hepta 只能通过 typed extension、activity adapter、evidence hook 和 feature gate 接入。不得在 Hepta 再造第二套 session、router、generic state store 或隐式模型执行器。

### 3.2 单一权威原则

每一类状态必须只有一个写权威：

| 状态 | 唯一权威 | 其他层只能做什么 |
|---|---|---|
| Thread/turn/session | Codex | 读取 typed facts、提交受控请求 |
| 事实/KG/memory head | CognitiveStore + deterministic validator | 提议、召回、解释 |
| Flow definition | Workflow Registry + governance promotion | 读取已批准版本、提出离线 proposal |
| Flow run / step state | Agent-local TaskFlow store | OpenClaw 只能投影 |
| External effect | Effect intent/receipt + owning executor | 重试、对账、补偿 |
| Policy/model artifact | Model/Policy Registry + NDU promotion | runtime 只加载已签名版本 |
| Fleet lifecycle | fleet supervisor/control state | TaskFlow 不得变成 fleet scheduler |

禁止 OpenClaw TaskFlow、Hepta TaskFlow 和某个 agent 同时写同一 flow。

### 3.3 失败语义

- 内部状态转移使用 expected revision/CAS，成功提交一次。
- 外部 side effect 只承诺 at-least-once；不声称 provider/tool 的物理 exactly-once。
- 每个 side effect 先持久化 EffectIntent，完成后写 EffectReceipt；超时或崩溃导致结果不明时写 Indeterminate，进入 reconciliation。
- lease expiry 不是 takeover 权威；必须有 owner/generation fencing 和正面的旧 owner 死亡证据。
- cancel 是 sticky；补偿是显式 Saga step；不得依赖隐式回滚。

### 3.4 NDU 的权限

NDU 可以：

- 读取脱敏、可重放 trajectory；
- 计算 utility、credit、calibration 和 drift；
- 生成 bounded proposal、adapter、policy artifact；
- 运行 paired replay、shadow、canary；
- 请求 promotion 或 rollback。

NDU 不可以：

- 直接写 production memory/KG；
- 直接调用 tool、provider 或外部 effect；
- 修改 workflow topology、权限、invariants 或 safety budget；
- 在运行中悄悄修改 base model 或 neuron 权重；
- 绕过 Hepta governance 或取得执行 credential。

---

## 4. 目标架构

### 4.1 平面关系

~~~text
                         ┌─────────────────────────┐
                         │ Governance / Threat     │
                         │ model / authority /     │
                         │ budget / signatures     │
                         └────────────┬────────────┘
                                      │ hard gates
                   approved artifacts│
                         ┌────────────▼────────────┐
                         │ NDU Control Plane        │
                         │ replay / credit / eval   │
                         │ shadow / canary / revert │
                         └────────────┬────────────┘
                                      │ policy/model digest
                         ┌────────────▼────────────┐
                         │ Hepta Intelligence       │
                         │ memory / KG / context    │
                         │ neuron groups / intuition│
                         └────────────┬────────────┘
                                      │ WorkflowDecision
                         ┌────────────▼────────────┐
                         │ Hepta TaskFlow Kernel    │
                         │ compile / run / wait     │
                         │ retry / join / recovery  │
                         └────────────┬────────────┘
                                      │ typed activity
                         ┌────────────▼────────────┐
                         │ Codex + Agentd spine     │
                         │ model / tool / thread    │
                         │ App Server / generation  │
                         └────────────┬────────────┘
                                      │ receipts
             ┌────────────────────────▼────────────────────────┐
             │ CognitiveStore / KG / Evidence / Trajectory     │
             │ state, provenance, compact checkpoint, feedback  │
             └────────────────────────┬────────────────────────┘
                                      └──────────────► NDU replay
~~~

### 4.2 依赖方向

~~~text
hepta-contracts
   ├── hepta-evidence / hepta-trajectory
   ├── hepta-memory / hepta-context
   └── hepta-taskflow-contracts
            └── hepta-taskflow
                    ├── hepta-intuition (traits only)
                    ├── agentd/Codex activity adapters
                    └── hepta-automation wakeup adapter

hepta-neuron ──► typed signal/model receipt ──► intelligence/taskflow
hepta-ndu-adapter ──► read trajectory; emit signed artifact ──► registry
OpenClaw/Lobster adapters ──► edge compatibility only
~~~

关键点：hepta-taskflow 与 hepta-intelligence 平级，位于 runtime 层；TaskFlow 不直接依赖具体 neuron、provider 或 OpenClaw session。hepta-intuition 只依赖 workflow registry 的 typed view，不把执行器塞进 cognition。

---

## 5. 冻结面、可学习面与观察面

| 类别 | 冻结/治理内容 | 可学习内容 | 仅观察内容 |
|---|---|---|---|
| Workflow | 节点拓扑、schema、capability、前后置条件、invariant、终态 | 合法分支排序、workflow 选择、阈值、abstain | 实际耗时、失败、补偿 |
| Memory | scope、provenance、事实验证、tombstone/forget、最低保留集 | admission/retrieval/decay 权重、salience | recall 命中、用户纠正、污染信号 |
| Context | compact 安全下限、保留区、schema、恢复规则 | token budget 分配、摘要粒度、召回排序 | loss report、rehydration 结果 |
| Neuron | 输入输出 schema、模型签名、隐私级别、资源上限 | head/adapter、temperature、置信度校准 | NPU 使用率、CPU fallback、latency |
| Execution | CAS、lease/fence、effect receipt、权限、取消/补偿语义 | retry/abstain/escalate 的策略参数 | activity outcome、indeterminate |
| NDU | promotion/rollback、数据脱敏、artifact 签名 | policy/model/head/adapter | replay/eval 统计 |

任何 topology proposal 都必须经过离线 schema 编译、独立评测、shadow、canary 和 rollback；不能由 neuron 或 NDU 在生产中动态改图。

---

## 6. 模块与 crate 总体设计

下面的 crate 名称是建议名，统一遵循当前 codex-hepta-* 命名；是否拆成独立 crate 由依赖环和 CALLERS ratchet 决定。

### 6.1 codex-hepta-contracts：公共类型与摘要

**当前状态**：已有 Agent identity、memory/provenance、governance/provider receipt；缺少 workflow/neuron/intuition/NDU/trajectory 类型。

**新增内容**：

- FlowId、WorkflowId、WorkflowVersion、RunId、StepId、AttemptId、PositionId、TrajectoryId；
- SchemaDigest、DefinitionDigest、PolicyDigest、ModelDigest、StateDigest；
- 统一 bounded string/bytes、canonical serialization、schema version；
- CapabilityId、Authority、SideEffectClass、FailureClass、IndeterminateReason；
- typed terminal/receipt envelope，禁止把 raw prompt、token 或 secret 写入 evidence。

**实现细节**：

1. 所有 public type 使用显式 schema version；
2. digest 采用 domain-separated SHA-256；
3. serde_json 只作为边缘投影/兼容格式，权威状态使用 typed Rust structs；
4. 每个新 public surface 必须加入 CALLERS.toml，qualification harness 不算产品 caller；
5. contracts crate 不依赖具体 runtime、provider、OpenClaw 或 neuron 模型。

**验收**：canonical serialization 稳定、非法长度/版本 fail-closed、digest golden fixtures、跨平台 round-trip、无依赖环。

### 6.2 codex-hepta-evidence + codex-hepta-trajectory：证据与运行轨迹

**当前状态**：evidence 已有 append-only governance/provider/memory/channel 表，但没有完整统一轨迹。

**设计**：

- hepta-evidence 继续保存不可变 authority/effect/memory receipts；
- 新的 hepta-trajectory 保存可重放的 observation lineage，或作为 evidence 的 typed family；
- 两者共享 Agent/scope/workspace digest、generation、source provenance 和 idempotency；
- 不另造跨 Agent event bus；Agent-local SQLite/WAL 作为初始 backend；
- materialized run view 可以重建，不得成为唯一真相。

**事件顺序**：

~~~text
turn_observed
  → memory_recall / memory_candidate
  → neuron_signal
  → intuition_decision
  → workflow_started
  → step_ready
  → effect_intent
  → dispatch
  → effect_receipt | indeterminate
  → postcondition
  → step_committed
  → flow_terminal
  → feedback / consolidation
~~~

**故障要求**：

- append-first + expected sequence/CAS；
- kill/restart 后可从事件重建 frontier；
- 重复事件由 trajectory_id、sequence、event_digest 幂等；
- 发现 digest/sequence 冲突立即 fail-closed；
- raw user content、token、credential 只保留必要摘要或受保护引用。

**验收**：随机 kill/restart/replay 后 state digest 相同；无丢事件、无双 terminal、无跨 Agent 读写；event ledger 与既有 evidence receipt 可互相追溯。

### 6.3 codex-hepta-state（逻辑层，不新增万能数据库）

不建议新建一个脱离 Codex 的通用 state store。以 trait 方式定义 Agent-owned durable state seam，底层复用现有 Agent 私有 SQLite/sqlx 和 generation fencing：

- flow run projection；
- typed snapshots；
- leases/owner/revision；
- compact checkpoint；
- pending approval/wait；
- reconciliation queue。

OpenClaw 的 stateJson 只作为 projection，不是 authority。snapshot 必须带 schema version、parent event sequence、state digest、policy/definition digest 和 creation generation。

### 6.4 codex-hepta-memory：持久记忆与 CognitiveStore 扩展

**已具备**：

- AgentPrivate/WorkspacePrivate scope；
- source ledger、stable memory ID、revision/CAS；
- verified/provisional memory；
- KG fact-set、immutable projection、FTS/recall、federation；
- remember/correct/forget 的原子写和 tombstone；
- physical send 前 revalidate；
- KG 不从 prose 推断事实。

**新增模块**：

1. **Memory Admission**：接收 neuron group、turn_end、task_end、tool receipt、用户纠正生成的 MemoryCandidate；先 provisional，再经过 provenance/conflict/scope/freshness/authority 校验。
2. **Consolidator**：把 episodic candidate 合并为 semantic fact、preference、procedural workflow prior、open loop；每次合并产生 receipt。
3. **分层 memory**：
   - episodic：原始事件摘要和可追溯引用；
   - semantic：验证过的实体/关系/事实；
   - procedural：已批准 workflow 的先验和成功轨迹；
   - preference：用户明确偏好；
   - open-loop：未完成任务、等待和风险。
4. **Decay/forget/retire**：时间有效性、访问统计、冲突、用户 forget、策略版本均可解释；删除使用 tombstone，不物理复活。
5. **Federated recall**：维持显式 grant/revoke；NDU 只能观察 retrieval utility，不直接扩大 scope。

**验收**：自动 admission 不会绕过 deterministic validator；provisional 不进入 KG；引用、scope、revision、forget 后都可重放验证；记忆故障只移除可选 context，不拖垮 turn。

### 6.5 codex-hepta-context：context compact 与 rehydration

当前 Codex 有自己的 compact，但 Hepta 需要独立的语义 checkpoint。

**组件**：

- CompactionOrchestrator：pre-compact 收集保留区，post-compact 写入 checkpoint；
- RetentionSet：身份、硬约束、用户明确偏好、未完成 open loop、未解决冲突、workflow invariants；
- SummaryLayers：turn、task、episode、long-term 四层摘要；
- CompressionPolicy：版本化、带最低安全保留集；
- LossReport：被压缩的实体/约束/引用、token、latency、confidence；
- Rehydrator：新 context 建立时按 scope/revision/definition/policy digest 重建；
- Poison/Drift Validator：检查摘要是否引入未验证事实、过期版本或越权 context。

**生命周期**：

~~~text
turn boundary
  → pre-compact recall + protected facts
  → bounded summary / loss report
  → append checkpoint + state digest
  → Codex compact or new context
  → rehydrate + exact-head revalidation
  → post-compact observation + feedback
~~~

**验收指标**：相对于当前 baseline，关键实体、约束、open loop 的召回率不下降；token/latency 有明确预算；checkpoint/restart/rehydrate 后 digest 和权限一致；压缩失败时保留原 context 或安全降级。

### 6.6 codex-hepta-taskflow-contracts：版本化 workflow 图

建议模型：

~~~rust
WorkflowDefinition {
    workflow_id,
    version,
    input_schema,
    output_schema,
    nodes: Vec<NodeSpec>,
    edges: Vec<EdgeSpec>,
    invariants,
    capability_set,
    policy_digest,
    definition_digest,
}

NodeSpec {
    node_id,
    kind,
    input_schema,
    output_schema,
    authority,
    side_effect_class,
    retry_policy,
    timeout,
    idempotency_template,
    preconditions,
    postconditions,
    compensation,
}

FlowRun {
    run_id,
    definition_digest,
    owner_agent_id,
    thread_id,
    status,
    revision,
    current_frontier,
    state_snapshot_digest,
    cancel_intent,
    wait_spec,
}
~~~

**节点类型**：Recall、NeuronGroup、Model、Tool、Subflow、Approval/Input、Timer、Fanout、Join、Validate、Commit、Compensate、Feedback。

**编译器校验**：

- 无孤立节点、无非法 cycle、至少一个成功/失败终态；
- edge input/output schema 可兼容；
- capability 与 authority 不越权；
- side effect 必须有 idempotency/receipt/compensation 策略；
- wait、retry、join、cancel 都有恢复路径；
- definition digest 对同一输入稳定；
- workflow 版本不可变，修改生成新版本。

### 6.7 codex-hepta-taskflow：Agent-local durable kernel

这是 Hepta 内化 TaskFlow 的核心，但必须与 hepta-intelligence 平级、位于 runtime 层。

**它负责**：

- 加载已批准 definition；
- 创建/推进/暂停/恢复/cancel flow；
- durable frontier、checkpoint、lease、retry、timer、join；
- activity dispatch 和 receipt matching；
- crash recovery、reconciliation、Saga compensation；
- 产出每一步的 trajectory/evidence。

**它不负责**：

- 解析自然语言事实；
- 直接写 CognitiveStore/KG；
- 直接调用 model/tool/provider；
- 修改 workflow graph/权限；
- 变成 fleet-wide scheduler；
- 取代 Codex ThreadManager 或 App Server。

**运行循环**：

~~~text
load run projection
  → validate definition/policy/generation
  → claim ready node with owner fence
  → append EffectIntent (if side effect)
  → dispatch typed Activity through Codex/Agent adapter
  → persist Receipt or Indeterminate
  → validate postcondition
  → CAS commit next frontier
  → emit trajectory + schedule next wakeup
~~~

**现有 automation 的接法**：

- hepta-automation 继续负责 once/fixed-interval due wakeup、lease 和 generation fencing；
- TaskFlow kernel 通过 trait 接收 timer/event，不再新建 250ms 的第二 scheduler；
- 初期 activity adapter 仍可使用 thread/queue/add；
- 后续增加 typed App Server activity seam，但不绕过 Codex execution spine；
- hepta-fleet 只做 supervisor-owned lifecycle，不承载 flow execution。

**状态机**：

queued → running → waiting | blocked → running → succeeded | failed | cancelled | indeterminate

每个 transition 都带 expected revision；sticky cancel 拒绝新增 child activity；indeterminate 只能经 reconciliation 或人工批准继续。

### 6.8 codex-hepta-neuron：Neuron group 与本地 NPU

**group 分层**：

1. Perception：intent、topic、对象、约束、紧急度；
2. Memory：salience、remember-vs-ignore、entity linking、相似案例、freshness；
3. Workflow/Intuition：workflow candidate、合法 branch score、abstain；
4. Risk/Validation：冲突、风险、postcondition anomaly、prompt injection；
5. Feedback：结果分类、用户纠正、utility、drift。

**本地执行原则**：

- 逻辑上每个 neuron 绑定 task/model/schema/threshold/fallback；
- 物理上由共享 model pool/sidecar 加载权重；
- Mac M5 首选 Core ML CPU+Neural Engine；MLX 作为 GPU/CPU fallback，不假定其直接调用 ANE；
- Rust 通过 Unix socket/IPC 获取 typed result、confidence、latency、model receipt；
- 不支持的算子、NPU 忙或超时，按 rule → CPU/MLX → remote 的 fallback 链降级；
- j3160 不承担本地小模型推理；必须有资源预算和 circuit breaker。

**NeuronResult 最小字段**：

neuron_id、group_id、model_id、model_version、model_digest、input_digest、output_schema、output_digest、confidence、calibration_version、device、latency_ms、fallback_used、provisional、receipt_digest

neuron 输出只能是 typed proposal/signal，不能直接写 memory、KG 或执行 tool。

**第一批 smoke test**：salience、intent、entity/relation；测 p50/p95、内存、NPU/CPU fallback、schema 合法率和 confidence calibration。

### 6.9 codex-hepta-intuition：稳定的 workflow policy

Intuition 的定义：

> 在当前 memory/context/neuron state 下，从已批准的 workflow registry 中选择一个 workflow/version，选择一个合法分支或 abstain，并给出可解释的 DecisionReceipt。

推理顺序：

~~~text
recall protected state
  → run perception/memory/risk groups
  → filter by hard preconditions/capabilities
  → score approved workflow candidates
  → choose branch | ask | abstain | escalate
  → emit DecisionReceipt
  → TaskFlow executes only the receipt-authorized graph
~~~

Intuition 可以学习：

- workflow/skill 选择；
- 合法分支排序；
- retry/abstain/escalate 阈值；
- memory retrieval prior；
- confidence calibration。

Intuition 不能学习或在线改变：

- graph topology；
- permission/capability；
- KG fact validation；
- effect receipt semantics；
- context safety lower bound。

与普通 skill 的差异是：skill 是能力原语；TaskFlow 是固定执行图；intuition 是图上的稳定选择 policy，拥有 precondition、postcondition、回退、审批和验收。

### 6.10 codex-hepta-ndu-adapter：慢速学习与治理

每个 NDU position 都必须声明：

position_id / owner / phase / input_schema_digest / output_schema_digest / authority / learnable_parameters / budget / feedback_dimensions / cadence / artifact_digest / promotion_gate / rollback_gate

推荐 positions：

| position | 输入 | 学习面 | 生产权限 |
|---|---|---|---|
| memory-admission | candidate/provenance/outcome | admission score、decay | observe/propose |
| memory-retrieval | query、scope、recall result | ranking、freshness | observe/propose |
| context-budget | token/loss/rehydration | budget allocation | observe/propose |
| workflow-select | state + candidate workflows | ranking/policy | propose |
| branch-route | legal branch features | threshold/order | propose |
| retry-abstain | failure/risk/cost | retry/escalate/abstain | propose |
| neuron-group-head | typed features | head/adapter/calibration | shadow/canary |
| postcondition-check | effect receipt/state | anomaly threshold | observe/propose |

**训练方式**：

- 连续 signal/head：offline supervised/replay backprop；
- 离散 workflow/branch：imitation、ranking、contextual bandit 或 constrained offline policy gradient；
- 不对真实 network/tool side effect 直接反向传播；
- 使用 hierarchical credit assignment，把 terminal utility 归因到 group、policy、memory candidate、step；
- base model、权限和 graph topology 冻结；
- 所有 artifact 必须有 dataset/replay manifest、digest、签名、independent eval、shadow/canary 和 rollback。

建议 utility vector 包括：成功率、人工纠正、回滚、记忆污染、关键事实保持、延迟、成本、资源峰值、abstain 安全性。不要在数据不足时先拍绝对阈值；先比较相对 baseline，再固化门槛。

### 6.11 Model/Policy Registry 与 Core ML sidecar

Registry 管理：

- model/adapter/head/policy 的 immutable artifact；
- schema、runtime、device、license、privacy、resource budget；
- digest/signature、训练数据 manifest、评测 receipt；
- compatibility matrix、fallback chain、promotion/rollback state。

Mac sidecar：

- Swift/Core ML 进程，统一 model pool；
- Unix socket framing、长度上限、request id、timeout、generation；
- 返回 typed JSON/CBOR envelope，不返回未验证 prose 作为事实；
- 资源超限自动降级或停止 sidecar，不拖垮 agent；
- 每次调用生成 ModelReceipt，供 trajectory 和 NDU replay。

### 6.12 OpenClaw/Lobster/Agentd adapters

- OpenClawTaskFlowAdapter：映射 flowId、status、revision、wait、cancel、child task；OpenClaw stateJson 只是 projection。
- LobsterAdapter：导入/导出受限 pipeline、approval、resume token；不把 Node runtime 当 Hepta authority。
- AutomationWakeupAdapter：复用现有 schedule/lease。
- CodexActivityAdapter：将 model/tool/memory activity 送入既有 App Server/Thread spine。
- Matrix/Robrix projection adapter：只读呈现 flow/agent status；不绕过 G4 backend/UI authority。

迁移期间禁止双写。每个 flow 只有一个 owner；旧 flow 可以留在 legacy mode，新 flow 由 Hepta kernel 创建，OpenClaw 读取 projection。

---

## 7. 统一数据模型与 API 草案

以下是实现方向，不要求一次性全部落地。

~~~rust
pub struct WorkflowDefinition {
    pub workflow_id: WorkflowId,
    pub version: u32,
    pub input_schema: SchemaDigest,
    pub output_schema: SchemaDigest,
    pub nodes: Vec<NodeSpec>,
    pub edges: Vec<EdgeSpec>,
    pub invariants: Vec<Invariant>,
    pub capability_set: Vec<CapabilityId>,
    pub policy_digest: PolicyDigest,
    pub definition_digest: DefinitionDigest,
}

pub struct FlowRun {
    pub run_id: RunId,
    pub definition_digest: DefinitionDigest,
    pub owner_agent_id: AgentId,
    pub thread_id: ThreadId,
    pub status: FlowStatus,
    pub revision: u64,
    pub current_frontier: Vec<StepId>,
    pub state_snapshot_digest: StateDigest,
    pub wait: Option<WaitSpec>,
    pub cancel_requested: bool,
}

pub struct EffectIntent {
    pub run_id: RunId,
    pub step_id: StepId,
    pub attempt: AttemptId,
    pub idempotency_key: String,
    pub capability: CapabilityId,
    pub input_digest: Sha256Digest,
}

pub struct EffectReceipt {
    pub intent_digest: Sha256Digest,
    pub outcome: EffectOutcome,
    pub external_ref_digest: Option<Sha256Digest>,
    pub observed_at: Timestamp,
}

pub struct DecisionReceipt {
    pub policy_digest: PolicyDigest,
    pub position_ids: Vec<PositionId>,
    pub candidates: Vec<CandidateScore>,
    pub chosen: DecisionChoice,
    pub confidence: f32,
    pub abstain_reason: Option<String>,
}

pub struct NduObservation {
    pub trajectory_id: TrajectoryId,
    pub position_id: PositionId,
    pub input_digest: Sha256Digest,
    pub output_digest: Sha256Digest,
    pub reward_vector: RewardVector,
    pub safety_flags: Vec<SafetyFlag>,
}
~~~

### 7.1 状态与事件的一致性

权威顺序为：

intent → dispatch → receipt/indeterminate → postcondition → commit

如果 snapshot 与 event ledger 不一致，优先以完整性检查和 event sequence 判定；不能从 prompt、摘要或任意 JSON 猜测已发生的 external effect。

### 7.2 OpenClaw projection

OpenClaw 兼容 projection 只公开最小字段：

- flow id / status / current step；
- revision；
- wait/blocked summary；
- child task health；
- state digest、definition digest；
- last receipt class。

不把 raw state、credentials、模型输出或未验证 memory 复制进 OpenClaw state database。

---

## 8. Rust 项目借鉴与取舍

### 8.1 推荐结论

没有一个 Rust 项目可以无修改地作为 Hepta 核心。建议：

1. 用 Duroxide 做 durable history/replay 语义 spike；
2. 用 Sayiir 做 checkpoint/continuation 语义对照；
3. 生产实现写 Hepta-native 薄 kernel，复用现有 lease/CAS/evidence/Codex seam；
4. petgraph 只用于图拓扑校验，不承担 durable execution。

### 8.2 候选比较

| 项目 | 适合借鉴 | 主要问题 | 结论 |
|---|---|---|---|
| Duroxide（Microsoft，MIT，preview） | embedded Tokio、SQLite、history/replay、timer/event、retry、fan-out/join、compensation | preview；deterministic orchestration 约束严格；需接 Hepta authority/evidence | 最佳隔离 spike/语义参考 |
| Sayiir（MIT，年轻） | checkpoint、branch/loop/fork/join/wait/cancel、无 server | 明确不做 deterministic replay；审计/权限需自补 | 快速原型对照 |
| Obelisk（AGPL-3、WASM、pre-release） | WIT schema、sandbox、replay、structured concurrency | AGPL、单独 server/WASM 运行时 | 只参考设计 |
| Temporal Rust Core | 成熟 durable workflow 语义 | 依赖 Temporal Server；不是 embedded | 有外部服务时再评估 |
| Restate/Resonate | durable async/service 模式 | 偏外部服务 | 不作为当前内核 |
| Dagrs | DAG/条件节点 | 已迁移/归档，非 durable authority | 仅看图算法 |
| rp-engine/Oocana | AI workflow/manifest/connector 想法 | 版本早、provider/外部 executor 耦合 | 仅做概念参考 |

第三方依赖必须做 license、MSRV、SQLite/async runtime、determinism、security audit；不可因为 demo 可运行就进入生产 authority path。

---

## 9. 开发阶段与硬验收门

现有 R2 G1–G5 是发布脊柱；以下 H0–H9 是新的 Intelligence/TaskFlow 产品链。H0/H1/H2/H4 可以在 G4/G5 期间以设计、schema、shadow 和隔离测试推进，但任何生产启用仍服从 exact receipt、CALLERS、operator acceptance 和 promotion 前置门。

### H0 — Architecture / threat model freeze

**目标**：冻结平面、authority matrix、依赖方向、schema/version、隐私和回滚规则。

**交付**：

- ADR-001 平面与 ownership；
- ADR-002 TaskFlow/OpenClaw/Lobster 边界；
- ADR-003 event/effect/reconciliation 语义；
- ADR-004 NDU positions 和 promotion；
- crate graph 与 CALLERS 更新；
- threat model、数据保留/脱敏策略。

**硬门**：无循环依赖；无第二 session/router/scheduler/generic store；所有 authority 都有唯一 owner；property tests 编译通过。

### H1 — Contracts / trajectory / durable state

**目标**：建立 typed IDs、schema digest、event ledger、snapshot/CAS、effect intent/receipt。

**交付**：

- hepta-contracts 扩展；
- hepta-trajectory schema/migrations；
- Agent-local store adapter；
- kill/restart/rebuild 工具；
- evidence lineage 查询。

**硬门**：随机 fault injection 后 state digest 一致；重复 effect 不产生第二个可观察 external effect；indeterminate 不被误报成功；跨 Agent 访问 fail-closed。

### H2 — Workflow registry / compiler

**目标**：固定、可编译、可审计的 TaskFlow definition。

**交付**：

- registry/version/digest；
- NodeSpec/EdgeSpec schema；
- graph validator/compiler；
- golden workflow definitions；
- invalid graph corpus。

**硬门**：非法 cycle、无终态、schema mismatch、越权 capability、无幂等 effect、无 recovery path 全部 fail-closed；同 definition digest 稳定。

### H3 — TaskFlow kernel shadow/read-only

**目标**：在不执行写副作用的情况下验证 durable orchestrator。

**首个流程**：memory-review：

Recall → evidence validate → workflow select → read-only inspect → postcondition verify → report

**交付**：

- run lifecycle；
- wait/resume/approval；
- retry/backoff；
- fanout/join；
- cancellation；
- crash recovery/reconciliation；
- OpenClaw/Lobster projection；
- existing automation wakeup adapter。

**硬门**：shadow 不产生 production effect；旧 runtime 与新 kernel paired replay 一致；Agent fault 不拖垮 peer；owner/generation fence 可靠。

### H4 — Memory admission + context compact

**目标**：把已有 CognitiveStore/KG/recall 连接成持久记忆和 context loop。

**交付**：

- MemoryCandidate/admission/consolidation；
- turn_end/task_end/tool-result capture；
- compact pre/post hooks；
- protected retention set；
- compact checkpoint/loss report/rehydrator；
- memory/context trajectory。

**硬门**：未验证事实不进 KG；forget/correct 后不会复活；compact 后关键实体/约束/open loop 不劣于 baseline；失败只安全降级，不丢 authority。

### H5 — Neuron groups / local NPU

**目标**：接入首批小型 typed neuron，先 shadow。

**交付**：

- NeuronGroup contract/scheduler；
- shared model pool；
- Mac Core ML sidecar smoke；
- salience/intent/entity/risk groups；
- CPU/MLX/remote/rule fallback；
- model receipt、confidence calibration、resource budget。

**硬门**：无直写 memory/KG、无直执行 tool；p95/内存/NPU fallback 均有 receipt；模型 schema/digest 可回放；sidecar 故障不影响主 Agent。

### H6 — Intuition policy

**目标**：在冻结 workflow 图上形成稳定的 workflow/branch/abstain policy。

**交付**：

- IntuitionPolicy；
- candidate filtering/precondition；
- DecisionReceipt/explanation；
- heuristic baseline；
- paired replay 和 counterfactual evaluation；
- SuggestOnly → Prepare 的 feature gate（暂不 ExecuteAllowed）。

**硬门**：不劣于 heuristic；安全拒绝/abstain 不下降；不可绕过 invariant、approval 或 capability；连续 replay 窗口无显著遗忘回归。

### H7 — NDU offline learning / promotion

**目标**：让 neuron group 和 intuition 通过轨迹长期变好。

**交付**：

- NduPositionRegistry；
- trajectory extractor/segmenter；
- hierarchical credit assignment；
- calibrator/head/adapter trainer；
- replay manifest、artifact registry；
- shadow → canary → promote/rollback；
- drift/forgetting monitor。

**硬门**：NDU 不可用时 deterministic baseline 继续运行；任何 artifact 可重建、签名、回滚；base model/authority/topology 不被在线修改；独立安全评测通过。

### H8 — Agent-local production canary

**目标**：把一条只读流程逐步变成受控 production flow。

**交付**：

- 单 Agent、单 workflow、低流量 canary；
- OpenClaw projection-only bridge；
- approval/commit/compensation；
- reconciliation/operator UI；
- exact receipt、CALLERS truth ratchet；
- rollback rehearsal。

**硬门**：明确单一 authority；故障可一键回到 legacy path；无跨 Agent 泄漏、无重复外部 effect、无资源失控；operator acceptance 后才扩大范围。

### H9 — 扩展 workflow 与 fleet

只有 H8 稳定且 R2-G5 完成后，才扩展：

- engineering-change；
- provider/auth fallback；
- cross-Agent federation；
- fleet-wide lifecycle；
- 写入型 workflow；
- 更复杂的 planner/adapter。

TaskFlow 仍是 Agent-local executor；fleet 不变成 workflow state bus。

---

## 10. 第一条垂直切片：memory-review

选择 memory-review 而非通用 agent 的理由：边界清晰、可只读、已有 CognitiveStore/KG 基础、容易做 oracle/replay、不会先碰外部副作用。

### 10.1 流程图

~~~text
input turn
  → recall bounded memories/KG
  → salience + intent neuron group
  → validate provenance/scope/revision
  → choose memory-review workflow version
  → inspect conflicts/staleness/open loops
  → produce typed report
  → emit postcondition + feedback
  → NDU shadow learns retrieval/select/abstain
~~~

### 10.2 第一版不做

- 自动写入未经用户确认的长期事实；
- 自动修改 workflow topology；
- 直接执行 tool/网络副作用；
- 在线 adapter training；
- 跨 Agent 自动扩大 memory grant；
- 让通用聊天模型决定 authority。

### 10.3 通过标准

- 同一输入、同一 definition/policy digest 可重放；
- 所有报告结论可追溯到 source/citation/revision；
- stale/conflict 时能够 abstain；
- compact 前后 protected facts 一致；
- shadow policy 至少不差于现有 deterministic heuristic；
- 任何失败均可回到 legacy read-only path。

---

## 11. 测试、评测与运维

### 11.1 单元与属性测试

- canonical serialization/digest；
- graph topology/schema/capability；
- CAS/revision/lease/generation；
- event append/rebuild/idempotency；
- memory provenance/forget/tombstone；
- compact retention/loss report；
- neuron schema/confidence/fallback；
- intuition candidate filtering/abstain。

### 11.2 故障注入矩阵

在每个边界注入 kill、timeout、disconnect、process restart、disk full、SQLite corruption、generation rollover、stale owner、duplicate callback：

1. intent 前；
2. intent 后、dispatch 前；
3. dispatch 中；
4. external result 到达后、receipt 前；
5. receipt 后、postcondition 前；
6. commit 前；
7. compact checkpoint 前后；
8. approval 等待期间。

每个格子都必须有预期 terminal、recovery 或 indeterminate 结果，不能靠日志推测。

### 11.3 长期学习评测

- paired replay：current heuristic vs candidate；
- no-regression/forgetting window；
- calibration（confidence 与实际成功率）；
- memory pollution/false fact rate；
- compact semantic preservation；
- latency/cost/resource；
- abstain safety；
- counterfactual branch evaluation；
- artifact reproducibility。

建议先使用相对指标：候选不得显著劣于 baseline，关键安全指标不得下降；在有足够 replay 样本后再冻结绝对阈值。

### 11.4 运维观测

所有 flow/run/step/neuron/policy 事件至少带：

agent_id、generation、workspace_digest、thread_id、run_id、step_id、attempt_id、definition_digest、policy_digest、model_digest、state_digest、event_seq、latency、resource、outcome

敏感内容只留 digest/引用。j3160 等小内存 Agent 需要：

- TaskFlow concurrency cap；
- compact/trajectory retention cap；
- NPU/model circuit breaker；
- gateway memory/Task limits；
- queue backpressure；
- failure isolation（停 workflow，不停整机）。

---

## 12. 迁移策略与回滚

### 12.1 迁移顺序

1. **Observe**：旧 OpenClaw/Lobster 继续是 authority，Hepta 只记录 observation。
2. **Shadow**：Hepta compiler/kernel 对同一输入做 paired replay，不执行副作用。
3. **Projection**：Hepta 成为新 flow authority，OpenClaw 只读 projection；旧 flow 保持 legacy owner。
4. **Canary**：单 Agent、单只读 workflow、低流量。
5. **Prepare**：允许生成 effect intent/approval，但仍需显式 operator。
6. **ExecuteAllowed**：仅在独立 acceptance、reconciliation 和 rollback rehearsal 后逐步开放。

### 12.2 回滚点

- definition/policy/model artifact 版本；
- schema migration；
- adapter feature flag；
- flow owner；
- Agent generation；
- side effect capability；
- NDU artifact promotion；
- OpenClaw projection bridge。

回滚只切换已签名 artifact/feature gate，不重写历史事件，不删除 evidence，不把 indeterminate 改成成功。

### 12.3 兼容规则

- 不共享 OpenClaw openclaw.sqlite；
- 不让 OpenClaw 和 Hepta 双写 flow；
- 不把 Lobster resume token 当 Hepta effect credential；
- legacy task 可通过 adapter 映射为 read-only observation；
- projection schema 变化必须带版本和 digest；
- 每次迁移都要有 old-vs-new canonical fixture。

---

## 13. 风险清单

| 风险 | 具体表现 | 缓解 |
|---|---|---|
| 重复 authority | OpenClaw/Hepta/Agent 同时推进 flow | ownership matrix、单 writer、projection-only |
| workflow 与 session 循环依赖 | taskflow 取代 Codex thread spine | activity adapter、Codex 唯一 execution spine |
| external effect 重试 | 重复发送/写入/扣费 | intent、idempotency、receipt、Indeterminate、reconciliation |
| event/snapshot 分叉 | 重启后状态不一致 | append-first、CAS、digest、rebuild |
| memory poisoning | 模型把 prose 当事实 | structured caller facts、provenance、provisional、validator |
| compact 丢关键事实 | 新 context 忘记约束/open loop | protected retention、loss report、rehydration eval |
| NDU 越权 | 在线改权重/图/权限 | proposal-only、签名 artifact、canary、rollback |
| 稀疏反馈/奖励投机 | policy 学会绕过检查 | hierarchical credit、安全 utility、独立 eval |
| 灾难遗忘/drift | 新 adapter 破坏旧 workflow | replay manifest、no-regression window、freeze base |
| NPU/资源耗尽 | Mac CPU fallback、j3160 OOM | shared pool、budget、circuit breaker、remote/rule fallback |
| deadlock/孤儿 lease | flow 永久 waiting | 静态图检查、watchdog、generation fencing、operator reconciliation |
| 第三方依赖风险 | license/preview/API 变化 | spike 隔离、trait boundary、license audit、可替换实现 |
| G4/G5 越级 | candidate 被误报 production | exact-head receipt、CALLERS ratchet、fail-closed promotion |

---

## 14. 交付物目录与建议命名

### 14.1 设计与契约

- docs/architecture/HEPTA_VNEXT_PLAN_V1.md
- docs/architecture/HEPTA_AUTHORITY_MATRIX.md
- docs/architecture/HEPTA_TASKFLOW_ADR.md
- docs/architecture/HEPTA_NDU_POSITION_CONTRACT.md
- docs/architecture/HEPTA_THREAT_MODEL.md
- schemas/hepta-taskflow/*.json（只做 projection/schema fixture，不替代 Rust authority）

### 14.2 建议 crate

~~~text
codex-rs/hepta-contracts/
codex-rs/hepta-trajectory/
codex-rs/hepta-taskflow-contracts/
codex-rs/hepta-taskflow/
codex-rs/hepta-neuron/
codex-rs/hepta-intuition/
codex-rs/hepta-ndu-adapter/
codex-rs/ext/hepta-taskflow-openclaw/
codex-rs/ext/hepta-taskflow-lobster/
~~~

hepta-context 可以先作为 hepta-memory/extension 的 typed module，只有在依赖和 CALLERS 清晰后才拆 crate。不要为了“模块数量”制造第二个 state store。

### 14.3 每个新 public surface 的最低交付

1. typed contract；
2. named product caller in CALLERS.toml；
3. unit/property/fault tests；
4. canonical fixture/oracle；
5. evidence/telemetry；
6. feature gate；
7. shadow/canary/rollback path；
8. exact-head receipt；
9. operator acceptance（若涉及 promotion/effect）。

---

## 15. Definition of Done

只有同时满足以下条件，才能称为“Hepta vNext Intelligence/TaskFlow product loop 完成”：

1. R2 G1→G5 前置链完成并有 exact receipts；
2. TaskFlow kernel 在 Agent-local runtime 中运行，且没有第二 session/router/scheduler/generic store；
3. workflow graph、权限、invariants、effect semantics 都是版本化、可审计、不可在线越权修改；
4. memory admission/consolidation、context checkpoint/rehydration、trajectory ledger 已闭环；
5. neuron group 有 typed output、model receipt、资源预算和 fallback；
6. intuition 只选择 approved workflow/branch，并可安全 abstain；
7. NDU 能离线 replay、credit、calibrate、shadow、canary、promote、rollback；
8. OpenClaw/Lobster 只做兼容 adapter/projection，不存在双 authority；
9. 失败、重启、重复 callback、external indeterminate 都有可重放的处理；
10. independent eval 证明长期 memory、compact preservation、workflow stability 和资源隔离没有显著回归；
11. operator 能看见、暂停、对账、回滚，而不能绕过 typed authority；
12. 每个 public surface 有 caller、receipt 和可验证的 provenance。

---

## 16. 当前立即执行顺序

在不改变当前 production authority 的前提下，建议按以下顺序开始：

1. 保留 G4 final40 `qualified_exact`，禁止重复改 exact candidate；
2. 对 G5 六个 bounded slice 使用 aggregate receipt 做独立 review；未满足 operator acceptance 前不 ratchet CALLERS、不解冻 fleet/automation；
3. 在独立 lane 完成 H0/H1 protocol fixture（event/CAS/lease/effect/trajectory）并通过 offline tests；**已完成**；
4. 完成 H2 workflow registry/compiler golden corpus；**已完成**；
5. 完成 H4 memory candidate/forget propagation 与 compact checkpoint/rehydration schema；**已完成**；
6. 完成 P0 protocol-contracts、H4–H7 implementation-contracts 和 implementation-readiness matrix 的隔离 qualification；**已完成**；
7. 以 Duroxide 做隔离 replay spike、Sayiir 做 checkpoint 对照，不直接引入生产依赖；该 spike 不阻塞 Agent-local kernel 实现；
8. 按 readiness matrix 先完成 H1 Agent-local writer/outbox 的 standalone shadow（**已完成**），再把它逐字段映射到真实 Agent-local SQLite/outbox；H4a/H4b 已生成 implementation-blocked receipt，待 hook/writer seam 明确后再接入；真实 effect seam 仍由既有 Codex/App Server 控制；
9. H5b/H6b/H7a 已完成 runtime prerequisite audit（当前 blocked + no-op only）；下一步只做受控 implementation PR、receipt aggregation、独立评审和 H8 governance rehearsal，不能直接解冻 production。

**第一条可交付产品切片不是“通用自主 Agent”，而是可审计、只读、可恢复的 memory-review。**

---

## 附录 A：现有源码/证据索引

- 当前显式计划：/Volumes/T5/hepta-vnext/artifacts/roadmaps/r2-final-development-plan-20260821.md
- 当前 exact worktree：/Volumes/T5/hepta-vnext/worktrees/r2-g4-matrix-robrix-detached37
- 当前 HEAD：445d1cdc50c9（clean detached candidate）
- G4 authoritative final40 receipt：/Volumes/T5/hepta-vnext/artifacts/r2-g4-paired-exact-final40-20260823/PAIR-RECEIPT.json（SHA-256 `f36ce3f41cc8734f4392070a01ac53cbdf753dee5a1bb8b352feb1bc886e8064`）
- Hepta migration contract：codex-rs/hepta-contracts/README.md
- CognitiveStore：codex-rs/hepta-memory/src/cognitive_store.rs
- Cognitive runtime：codex-rs/hepta-memory/src/cognitive_runtime.rs
- Memory extension：codex-rs/ext/hepta-memory/src/extension.rs
- Automation contract：codex-rs/hepta-automation/src/lib.rs
- Automation scheduler：codex-rs/hepta-automation/src/scheduler.rs
- Agentd automation adapter：codex-rs/hepta-agentd/src/automation.rs
- Live runtime shell：codex-rs/hepta-runtime/src/lib.rs
- Existing evidence migrations：codex-rs/hepta-evidence/migrations/

## 附录 B：外部项目参考

- Duroxide：https://github.com/microsoft/duroxide
- Sayiir：https://github.com/sayiir/sayiir
- Obelisk：https://github.com/obeli-sk/obelisk
- Temporal Rust/Core：https://github.com/temporalio/sdk-rust
- Restate Rust SDK：https://github.com/restatedev/sdk-rust
- Dagrs：https://github.com/dagrs-dev/dagrs
- Lobster：https://github.com/openclaw/lobster

本文不授予任何第三方项目生产依赖资格；依赖引入仍需独立 license、security、MSRV、determinism 和 reproducibility 审计。

## 附录 D：执行合同与评测附件

本计划的事件/CAS/lease/effect/migration/compact 边界已另存为可评审合同：`HEPTA_VNEXT_PROTOCOL_SPEC.md`。NDU 的 position/trajectory/OPE/promotion 规则另存为 `HEPTA_NDU_LEARNING_SPEC.md`；Mac M5/RTX4060/j3160 的资源预算另存为 `HEPTA_RESOURCE_EVAL_MATRIX.md`；第一条 H3 `memory-review` 垂直切片的 PR、状态机、故障矩阵和 DoD 另存为 `HEPTA_TASKFLOW_VERTICAL_SLICE.md`；H8/H9 的迁移、canary、rollback 和 fleet 阻断门另存为 `HEPTA_H8_H9_GOVERNANCE_GATE.md`。协议合同明确：

- Agent-local authoritative SQLite + transactional outbox 是首版事务域；digest 不能替代原子提交；
- command 去重、expected-revision CAS、owner epoch/generation/fencing token 必须 fail-closed；
- `DispatchAccepted/QueuedReceipt` 永远不等于 external effect terminal；不确定结果必须进入 `Indeterminate` reconciliation；
- GuardExpr、loop/fan-out/join/retry budget、replay clock/random 和 legacy migration handshake 都是 typed/受限语义；
- memory/compact 摘要是 provisional evidence，forget 必须传播到 derived indexes/checkpoints/replay/training artifacts；
- qualification receipt 不自动改变 CALLERS、`g5_allowed`、`promotion` 或 `operator_acceptance`。

本轮新增的隔离 qualification 目录位于 Dropbox qualification index 下：

- `protocol-contracts-qualification-20260823/`：四份 P0 实施合同、JSON schema、golden、in-memory model、12 项测试和幂等 verifier；receipt SHA `11f1f080161389b770bce226a94ea1a9a65bd127d36a5f144d9db48359475d64`。
- `h4-h7-contracts-qualification-20260823/`：memory/compact、neuron runtime、NDU replay/OPE、artifact bundle 合同；receipt SHA `63184346bbbe3fdc62e099fef338dc5706897ec0fabfc04a96f1d8d85a016f28`。
- `implementation-readiness-qualification-20260823/`：16-stage 顺序、真实 crate graph、RACI/CI/资源预算和 fallback profile；receipt SHA `42f1ab45bfc9a65f409e3ffb25d770cbdf863bf40f02c9a61b9284c24360a8f9`。

协议文件本身不引入 runtime、database writer 或 production authority；进入 H1/H3 实现时必须逐项映射到代码与 fault tests。

### D.1 实施合同与 readiness qualification

#### D.1.1 P0 protocol contracts

- worktree：`/Volumes/T5/hepta-vnext/worktrees/qual-protocol-contracts-20260823`
- branch：`qualification/qual-protocol-contracts-20260823`
- commit/tree：`0e84bd1f20d539eade367c3ffb41a063ad28696c` / `dd843f759603b70575c38ef7fe5f8061e307a6f4`
- receipt：`protocol-contracts-qualification-20260823/PROTOCOL-CONTRACTS-RECEIPT.json`
- result：`PASS_PROTOCOL_CONTRACTS_SHADOW`；12/12；generator 连续两次输出完全相同；`SHA256SUMS` 全部通过。
- scope：仅 in-memory/schema/contract fixture；`production_writer/effect_authority/scheduler/model/tool/operator_acceptance/promotion/g5_allowed=false`。

#### D.1.2 H4–H7 implementation contracts

- worktree：`/Volumes/T5/hepta-vnext/worktrees/qual-contracts-h4-h7-20260823`
- branch：`qualification/qual-contracts-h4-h7-20260823`
- commit/tree：`ca2cf9fe69a54e1560484c430fa86094f48f132a` / `ee29dcde8fa7bd2e4427405742cfca25e50686c1`
- receipt：`h4-h7-contracts-qualification-20260823/CONTRACTS-RECEIPT.json`
- result：`PASS_H4_H7_CONTRACTS_SHADOW`；12/12；generator/verifier 双跑稳定；`SHA256SUMS` 全部通过。
- scope：memory/compact/neuron/NDU contract only；未安装模型、未连接 NPU、未写 memory/KG、未生成 production artifact。

#### D.1.3 Implementation readiness matrix

- worktree：`/Volumes/T5/hepta-vnext/worktrees/qual-implementation-readiness-20260823`
- branch：`qualification/qual-implementation-readiness-20260823`
- commit/tree：`ae5009358df7003d1abcf4b23ebc1db0612ddae9` / `83e0327fee5db3060c7fb294e1b3ed8914392b48`
- receipt：`implementation-readiness-qualification-20260823/IMPLEMENTATION-READINESS-RECEIPT.json`
- result：`PASS_IMPLEMENTATION_READINESS`；5/5；16 stages；crate graph acyclic；`SHA256SUMS` 8/8。
- stage order：`H0 → H1 → H2 → H4a → H3 → H4b → H5a → H5b → H6a → H6b → H7a → H7b → H8 → H9a/H9b/H9c`。
- explicit claims：`shadow_not_efficacy=true`、`real_npu_verified=false`、`production_authority=false`；H8/H9a/H9b/H9c remain governance-blocked。

这些产物把“协议已定义”与“实现已上线”分开；后续 PR 必须逐行把 contract 字段映射到 Agent-local writer、Codex hook、sidecar、真实 corpus 和 operator gate，不能仅因 qualification receipt 存在而宣称 production complete。

### D.2 隔离 implementation-spike receipts

#### D.2.1 H1 Agent-local writer/outbox shadow

- worktree：`/Volumes/T5/hepta-vnext/worktrees/qual-h1-authoritative-writer-20260823`
- branch：`qualification/qual-h1-authoritative-writer-20260823`
- commit/tree：`42e958d4d083a0beb19f76e4e5ecaad7110f14ea` / `221c2e9b57c9a4f76f80649be97cf295950dd3e7`
- receipt：`h1-authoritative-writer-qualification-20260823/H1-AUTHORITATIVE-WRITER-RECEIPT.json`
- result：`PASS_H1_AUTHORITATIVE_WRITER_SHADOW`；16/16；standalone Rust crate；fmt/clippy/verifier/SHA 全通过。
- scope：copy-on-write event/projection/outbox、command dedupe、CAS、owner/generation fence、fault rollback、at-least-once ack、sticky cancel；real SQLite、production writer/effect、CALLERS/G5/operator/promotion 全 false。

#### D.2.2 H4a/H4b implementation prerequisites

- worktree：`/Volumes/T5/hepta-vnext/worktrees/qual-h4-implementation-spike-20260823`
- branch：`qualification/qual-h4-implementation-spike-20260823`
- commit/tree：`0fd584455464071fbfc7d00517db39d9c2c6724d` / `b532ce285fd224ccb32eaa25962b98b1fb90663b`
- receipt：`h4-implementation-qualification-20260823/H4-IMPLEMENTATION-RECEIPT.json`
- result：`BLOCKED_H4_IMPLEMENTATION_PREREQUISITES`；12/12；typed in-memory MemoryAdmission/CompactAdapter fixture 与静态 seam audit 通过。
- missing prerequisites：自动 MemoryAdmission hook、CognitiveRuntime typed pre/post compact hook、CognitiveStore candidate admission writer。

#### D.2.3 H5b/H6b/H7a runtime prerequisites

- worktree：`/Volumes/T5/hepta-vnext/worktrees/qual-h5-h7-runtime-prereqs-20260823`
- branch：`qualification/qual-h5-h7-runtime-prereqs-20260823`
- commit/tree：`d0e418e6f1d94b02576bd8dba8d73ef61f40909e` / `ab637139d20e4ca9d703b6eae86b6cc9b3f3bc28`
- receipt：`h5-h7-runtime-prerequisites-qualification-20260823/BLOCKED_H5_H7_IMPLEMENTATION_PREREQUISITES.json`
- result：`BLOCKED_H5_H7_IMPLEMENTATION_PREREQUISITES`；8/8；schema/verifier/双跑/SHA 全通过；仅 feature-off no-op adapters。
- missing prerequisites：typed neuron/intuition/NDU/trajectory runtime seam、Core ML model/sidecar、真实 NPU benchmark、TaskFlow DecisionReceipt 绑定和 trajectory/OPE adapter。
- Mac capability note：Core ML `cpuAndNeuralEngine` 编译能力存在；MLX 可用但未暴露 ANE；没有模型、`coremltools`、sidecar 或真实 NPU efficacy 证据。

## 附录 C：本轮 qualification receipt 索引与 blocker ledger

### C.1 G5 bounded aggregate

| 产物 | 路径/commit | SHA-256 | 结论 |
|---|---|---|---|
| aggregate receipt | `/Volumes/T5/hepta-vnext/worktrees/qual-g5-bounded-aggregate-20260823/G5-BOUNDED-AGGREGATE-RECEIPT.json`；commit `7ff977aa1771c725fda4d0eaa24657c5c94826d8` | `7d178989d75ff4fe32c41fc38737a3d12a0ef0f3da30b6e84ff5c03bf11136a1` | `PASS_BOUNDED_AGGREGATE`，六 slice/common head/G4 binding 均通过 |
| verifier | 同一 worktree `verify_g5_bounded_aggregate.py` | `8d1745cf5f09af79f6a2dc595e2efe531ecb6dfb3912e519cabfa99a95c5232b` | 负权限门通过 |
| verify result | `VERIFY-RESULT.json` | `e861a65d616dc840d83560d768d8c1fac67539495aa2b4c36da0c4f748a66c59` | 未修改 CALLERS；未授权 promotion |

复核记录：本轮曾观察到一次工作树瞬态 hash drift，verifier 当时按设计 fail-closed；随后未改 receipt 内容，`r2-g5-fleet-isolation` 恢复 clean，common head/tree 与输入 receipt 再次一致，独立 verifier 重新得到 `PASS_BOUNDED_AGGREGATE`。这次瞬态不构成 production 变更，也没有因此解冻 G5。

六个输入 receipt 固定绑定 common head `73ff3b438a25d88201169aed7c7c79cf5d9644a8`、tree `4070f421a63311c66a77d08491c4a9ab1fd52c65`：

- generation/CAS target-only：`3d3423fca60e2206f9153e5a54c69df40b1368980a26bb586768d66dcb8109df`
- five-agent isolation：`17f43db073e65bce2728d2ff11b14b5137a120c64b29df0f1f779a6797f35ab9`
- memory grant/revoke：`4d3cd7a396c2d291c0b467161562d6e5c05a83aef0df2b4355f530add64b6d15`
- automation occurrence idempotency：`1cdc7b4aa2750581f0aec949a1c84acf7b18a0527cc3caa1fa492988e3617dc1`
- stale lease/no resurrection：`feba182f47b7d9273d304a8d55a9c32667f610615347d069a807495703cd649b`
- store-failure peer liveness：`ee3b488434daf998f3d44c2ee5049e71ebff09535c436a0b561e62295bfde79a`

### C.2 Remaining blockers and owner

| 优先级 | blocker | 当前处理 | 完成条件 |
|---|---|---|---|
| P0 | G5 CALLERS/production authority | aggregate 已完成，仍 qualification-only | 独立 G5 exact-head review、CALLERS qualification entry、operator acceptance、promotion receipt |
| P0 | 单一 authoritative DB + outbox/lineage | H0/H1 contract 与 H1 standalone writer shadow 已交付；真实 Agent-local SQLite writer/outbox 仍未实现 | 将 H1 字段逐项接入 Agent-local DB，完成 crash/restart/reconcile、outbox delivery 与单一 authority receipt；不能双写 |
| P1 | H2 workflow compiler | shadow compiler/golden corpus 已通过；仍未进入 production registry | 受限 GuardExpr、cycle/terminal/capability/idempotency/recovery 全部 fail-closed，并接入 governed registry |
| P1 | H3 durable TaskFlow kernel | shadow/reference kernel 已通过；production writer/effect seam 仍未实现 | Agent-local authoritative store、唯一 wakeup owner、wait/retry/join/reconcile；不执行真实 effect |
| P1 | H4 memory admission/compact | reference contract 与 implementation shadow 已通过；当前 `BLOCKED_H4_IMPLEMENTATION_PREREQUISITES` | candidate writer、自动 admission、CognitiveRuntime pre/post hook、forget 传播与 checkpoint/rehydration 接入 Agent authority |
| P2 | H5/H6/H7 neuron/intuition/NDU | semantic shadow 已通过；runtime prerequisite audit 为 `BLOCKED_H5_H7_IMPLEMENTATION_PREREQUISITES` | typed runtime seam、Core ML sidecar/model receipt、真实 NPU benchmark、trajectory/OPE adapter，再走 shadow→canary→rollback |
| P2 | H8/H9 canary/fleet | 已补 governance gate contract；当前按预期 `BLOCKED_GOVERNANCE_PREREQUISITES` | H4–H7 独立门、G5 operator/promotion、legacy quiesce/CAS transfer 和 rollback rehearsal 全部通过 |

任何 blocker 的 qualification 通过，都不能单独改变 `g5_allowed`、`promotion`、`operator_acceptance` 或 production CALLERS；这些字段必须由独立治理仪式产生。

### C.3 H0/H1 protocol qualification

- worktree：`/Volumes/T5/hepta-vnext/worktrees/qual-h0-protocol-20260823`
- protocol implementation commit：`8abeb59ddab8c9b68c9a81f34d9887fa84bd9bcc`
- final receipt commit：`74078ea3390bf55bc2ea46fc6e2eb043107319e5`
- receipt：`codex-rs/hepta-shadow-qualification/receipts/h0_h1_protocol_qualification_receipt_v1.json`
- receipt SHA-256：`c515d55d65a4c5d62c3f171d4e2e5acbd9f95b5d2fc1fcb286ebe88ace15157d`
- test output SHA-256：`cb80a26184261410f8ba632237a3219af15813c98cb579487c59453506357b6b`
- result：36 passed；clippy `-D warnings`、locked metadata、fmt、diff-check passed；`production_caller/writer=false`、`promotion=false`、`g5_unfrozen=false`。

### C.4 H2 workflow compiler qualification

- worktree：`/Volumes/T5/hepta-vnext/worktrees/qual-h2-workflow-20260823`
- commit：`fe9122cf0bff4adc62c321f7591cb0096673c5cc`；tree `f29527cd79bbe8189d80edac41f554ca7d0d1fed`
- golden：`codex-rs/hepta-shadow-qualification/fixtures/h2_memory_review_v1.json`，SHA-256 `5350b7aefe057fd11df817c5624f04f0d9bc5bc35d5d647e83029c42fe58b092`
- receipt：`codex-rs/hepta-shadow-qualification/fixtures/h2_workflow_qualification_receipt.json`，SHA-256 `a65d9805fa86db32667a464e115c0e1f7670d1e68b5e8d0abc49f9af0570d17e`
- result：全 crate 41/41、H2 专测 11/11、clippy `-D warnings`、fmt check、exact-G4 ancestry 和 forbidden-path scan 全通过；guard DSL、acyclic graph、terminal coverage、capability scope、idempotency、recovery path、stable digest 均有检查；`qualification_only=true`、`production_effects=false`、`promoted=false`。
- 未解决：尚无 Agent-local durable kernel、唯一 wakeup owner、真实 effect/reconciliation 或 production registry caller。

### C.5 H4 memory/compact qualification

- worktree：`/Volumes/T5/hepta-vnext/worktrees/qual-h4-memory-compact-20260823`
- commit：`ab67e772c75fc2fb27ba6606ea3aa667d8dd5338`；tree `ce9082ebbc90d88590978ff1569b4f5b8c0504b5`
- receipt：`h4-memory-compact-qualification/H4-MEMORY-COMPACT-RECEIPT.json`
- receipt SHA-256：`b066970c66b0efdc0baa7ceff5097fa89d1e41463375158ac4505d79aef6e936`
- golden fixture SHA-256：`e9f86b64c629c0e60cf8c0f5008366342745ae1c2aa017a17e49382f6cd7298b`
- result：Python unittest 10/10、JSON Schema Draft 2020-12、verifier、SHA256SUMS 全通过；candidate state/forget propagation/compact pre-post/loss/rehydration/range binding 均有绑定；`production_mutation=false`、`promotion=false`。
- 未解决：尚未把 reference model 接到 CognitiveStore admission、真实 Codex compact hook 或 Agent-local authoritative DB。

### C.5a Test-harness canonical-root qualification

- worktree：`/Volumes/T5/hepta-vnext/worktrees/qual-g4-memory-temp-canonical-20260823`
- commit：`c83fbcc1312dbfbdfd2a4eddfbf1c67119f5123a`；tree `1fc4eb088f69a3ef9724d8cc724a8a946c4010e5`
- 唯一变更：`hepta-memory/src/cognitive_test_support.rs` 将测试 `TempDir` 根先 canonicalize；生产 `cognitive_store.rs` 的 symlink 防护未放宽。
- receipt：`/Volumes/T5/hepta-vnext/artifacts/qualification/r2-g4-memory-temp-canonical-20260823/G4-MEMORY-TEST-HARNESS-RECEIPT.json`，SHA-256 `ea9b12429d33fb263505d28a532f9f8c4eea984fad854207bcd1e0e90ff62d25`
- 结果：原 exact test helper 在 macOS `/var` symlink 下 13/35；隔离 test-only patch 后同一默认 TMPDIR **35/35**，canonical TMPDIR 亦 35/35；`production_mutation=false`、`promotion=false`。

### C.7 H3 TaskFlow shadow qualification

- worktree：`/Volumes/T5/hepta-vnext/worktrees/qual-h3-taskflow-20260823`
- implementation commit：`ab528f32f7aa99cde3cef921a75e3be07ea0fd72`
- sealed receipt commit：`8cdd600d50a3d2cb0228b568771ccea24c298388`；tree `86fedfaaf808fc488bf0559df1c4eefb7a389395`
- receipt：`h3-taskflow-qualification/receipts/PASS_H3_SHADOW.json`；SHA-256 `dde17a21ecb5bef1d712762c27c83e2bb3a18de40aedbec61cc2800e3f16ab0c`
- golden trace：`h3-taskflow-qualification/golden/h3_golden_trace_v1.json`；SHA-256 `cb347bb95493a11d48f22a532c0c11d3aadcd2428c0a976df75e90c88d8aa38f`
- result：16/16 tests、verifier、SHA256SUMS 全通过；覆盖 CAS/dedupe、owner/generation fence、wait/resume、bounded retry、fan-out/join、sticky cancel、crash→indeterminate、projection-only；`production_caller/writer/model/tool/scheduler/effect=false`、`CALLERS_touched=false`、`promotion=false`。
- 未解决：这是 deterministic reference kernel；尚未进入 Agent-local authoritative DB、真实 App Server seam 或 production registry。

### C.8 H5 neuron-group shadow qualification

- worktree：`/Volumes/T5/hepta-vnext/worktrees/qual-h5-neuron-group-20260823`
- commit：`5c69eeeb10c77fedd2a22eddda180025ce191a3a`；tree `1caa892dc113031bedfbbc236503a0905222e23c`
- receipt：`h5-neuron-group-qualification/H5-NEURON-GROUP-RECEIPT.json`；SHA-256 `c54fb6b66db9e600c74fb8afc405c28ab19af23624efa92fad05cd2e83655c22`
- golden：`h5-neuron-group-qualification/golden/h5_neuron_group_v1.json`；SHA-256 `dac9254cb6688fb4b5176d2e6514ecc63814e346c4c938122be23bcfc72d5418`
- result：11/11 tests、verifier、SHA256SUMS 全通过；覆盖 typed signal/group state/model receipt、fixed/learnable position、shared pool、fairness/budget、NPU→CPU→remote→rule fallback、calibration 和 provisional effect fence；`model_installed=false`、`real_npu_connected=false`、`production_mutation=false`、`promotion=false`。

### C.9 H6 intuition-policy shadow qualification

- worktree：`/Volumes/T5/hepta-vnext/worktrees/qual-h6-intuition-policy-20260823`
- commit：`8807e17b24dc2a977a1c0b57f3cea03352b949c9`；tree `34a0719650b1104b6d5f269ba53e241d80e9f28a`
- receipt：`h6-intuition-policy-qualification/H6-INTUITION-POLICY-RECEIPT.json`；SHA-256 `f471025ef0a6c9b907a0af51a154479dbdb5fa676ee1137980d94657066d4337`
- golden：`h6-intuition-policy-qualification/golden/h6_intuition_policy_v1.json`；SHA-256 `92005cfb28d13b47e52015a5cf0a3a3d94174582d5537915813955c807991bc4`
- result：13/13 tests、Draft2020 schema、verifier、SHA256SUMS 全通过；hard-filter→learned-rank→deterministic tie-break、hysteresis/debounce、branch/retry budget、distribution-shift reject、stale receipt reject 和 paired replay safety floor 均有测试；`execute_allowed=false`、`production_mutation=false`、`promotion=false`。

### C.10 H7 NDU offline-learning shadow qualification

- worktree：`/Volumes/T5/hepta-vnext/worktrees/qual-h7-ndu-learning-20260823`
- commit：`7e8441183cb31a196210300261d0bde22bcbce1b`；tree `49cb55377c4dbf0e17d40c4f1986c091e6e840ce`
- receipt：`h7-ndu-learning-qualification/H7-NDU-LEARNING-SHADOW-RECEIPT.json`；SHA-256 `d562ca3b76f1b384979308e7c5df2aafb0fa56bd42b0e085419119382cde174e`
- golden：`h7-ndu-learning-qualification/golden/h7_ndu_learning_v1.json`；SHA-256 `2b3f114b65b1a5a3a9142704d2ba0c5654f16ab885da5550eea177e3dad209a3`
- result：10/10 tests、verifier、SHA256SUMS 全通过；覆盖 NduPosition registry、immutable causal trajectory、hierarchical credit、bounded objective、paired replay/IPS-OPE、calibration、drift/forgetting/tombstone lineage、artifact digest 和 promotion reject/rollback baseline；`production_mutation=false`、`promotion=false`、base weights/topology/permissions 不可在线修改。

### C.11 H8/H9 governance gate

- contract：`HEPTA_H8_H9_GOVERNANCE_GATE.md`
- 当前结果：`BLOCKED_GOVERNANCE_PREREQUISITES`（预期 fail-closed，不是 production failure）
- 阻断原因：`g5_allowed=false`、尚无独立 operator acceptance/CALLERS promotion receipt；因此不得执行 legacy cutover、production canary、fleet propagation 或写入型 workflow。
- 已定义：prepare→quiesce→ownership CAS→canary→rollback 状态机、单一 authority、indeterminate reconciliation、duplicate-effect/stale-owner fault gates；所有 authority flags 保持 false。

### C.6 Exact baseline regression note

- `codex-hepta-contracts`：25 tests passed。
- `codex-hepta-automation`：6 tests passed。
- `codex-hepta-memory`：在隔离 test-only canonicalization lane 后默认 `/var` TMPDIR **35/35 passed**；生产 canonical-root 防护保持不变。
- `codex-hepta-supervisor`：补齐 `borrow-or-share v0.2.2` 本地缓存后，`--locked --offline` 全套 unit/product/projection 通过（26 unit、1 daemon product、2 process product、2 projection，1 controlled fixture ignored）。
- 统一 exact G4 worktree（只读、canonical TMPDIR、已补齐 cache）四包回归：contracts 25/25、automation 6/6、supervisor 全部通过、memory 35/35；未修改 exact source。

## 附录 E：local-development profile（2026-08-24，替代本地开发的外部输入阻断）

上一版把 provider owner 契约、独立 SSHSIG/trust owner、900 秒 challenge、CALLERS
production ratchet 和 promotion ceremony 作为所有后续工作的共同前置。这会把“本地开发资格”
错误地绑定到尚未接入的生产协作方。本附录对 **local-development / shadow / sandbox** 路径作
明确 supersede：这些外部输入不再是本地开发 blocker，也不要求开发者去寻找外部 owner。

### E.1 两种 profile，不混淆

| profile | 用途 | 外部输入 | 生产 effect / promotion |
|---|---|---|---|
| `local_development` | 本地实现、回放、shadow、sandbox、bounded fleet 开发 | 无 | 始终关闭 |
| `production` | 将来若明确需要真实外部 effect 或正式发布 | 另行接入 provider 契约与 trust policy | 需要单独显式开启 |

`local_development` 的完成状态使用独立字段：
`g5_local_complete`、`local_operator_acceptance`、`local_fleet_shadow_allowed`。
它们不得被代码解释为 `g5_allowed`、`operator_acceptance`、`promotion` 或生产 CALLERS 权限。
这里的 profile 是 declaration-only qualification metadata，不是 runtime 开关；实际 entrypoint
仍必须保持治理启用、Shadow 模式和所有 production/effect flags 关闭。

### E.2 本地 effect 语义

本地路径统一采用 `at_least_once_indeterminate_reconcile`：

`EffectIntent → DispatchAccepted → EffectReceipt` 或 `Indeterminate → reconcile`。

当前 provider 没有远端 occurrence/status/effect ACK 时，adapter 继续
`Unsupported`/`Indeterminate`/fail-closed；不再把这个事实当作本地开发的阻断，也不把它改写成
physical exactly-once。外部 effect 默认关闭；需要 effect 的测试只能使用显式 sandbox harness。

### E.3 删除的本地流程步骤

- 不再为本地工作生成或等待外部 provider handoff；
- 不再生成 900 秒 head-scoped challenge、SSHSIG 或独立 trust-owner acceptance；
- 不再为了 local shadow 触碰 production CALLERS ratchet、promotion 或 deployment；
- 不再重复封装同一批只读 assessor/lineage receipt；由一个自动生成的 local profile manifest
  记录 exact head、证据路径和负权限字段即可。

历史 receipt、旧 blocker handoff 和生产 profile 文档保持 append-only，作为历史/生产路径资料，
不再阻塞 local-development。新的机器可读 profile 位于：
`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-profile-v11-20260824/`；实现说明位于
detached worktree 的 `docs/hepta-vnext/G5_LOCAL_DEVELOPMENT_PROFILE_V1.md`。
当前 v11 profile SHA256 为 `1d90cec75d2e1b5f12882e9f2424ac4055d4a6d25cfa2689544e4a592e8ff9fc`，
绑定 head/tree/parent `e14717e176f6e60976e63416052dda5e859ecad0` /
`fe303f40637e5902191a04daf76e6f5e43613530` /
`ce4d409607fd8d9aef0ae5d4b00cdd0cb7f03e92`；v1–v10 仅作 superseded 历史记录。

### E.4 当前默认出口

```text
g5_local_complete=true
local_operator_acceptance=true
local_fleet_shadow_allowed=true
production_activation=false
g5_allowed=false
operator_acceptance=false
promotion=false
fleet_and_automation_unfrozen=false
provider_physical_exactly_once=false
external_inputs_required_for_local_development=[]
```

后续开发按原产品顺序继续：Codex execution base → independent dual Agent → Intelligence/KG
true closure → Matrix/Robrix → fleet/automation；只是在 local-development profile 下不再等待
外部签字或 provider owner。只有将来明确选择 `production` profile 时，才重新评估真实 provider
契约与生产 trust policy。

### E.5 Local-development implementation slice — 2026-08-24 01:xx

在 `local_development` 路径下继续推进真实代码切片，不等待外部 provider 或 signer：

- H4 memory admission：候选记忆只能进入 `provisional`，必须有显式 directive、内容绑定证据和 CAS
  才能 `verified`；不写 KG，不把 candidate origin 当作 explicit directive。
- H4 compact seam：加入 typed `pre_compact`/`post_compact`、parent revision/state digest、generation/fence、
  protected-reference/loss 检查和只读 rehydration plan；当前仍没有 authoritative lease/event/outbox、持久 checkpoint
  或 executor，因此不会自动取得 runtime authority。
- H5 neuron shadow：state/policy snapshot 绑定、确定性 proposal ID、position scope、±500bps 上限和显式
  `NoChange` abstain；不写 KG、不改 workflow/routing、不执行 effect。
- provider reconcile：unknown quarantine 后禁止把 late `Accepted→Rejected` 当作合法终态；SQLite reopen 继续
  fail-closed。真实 provider 仍按 `Unsupported/Indeterminate → reconcile`，外部 effect 关闭。

统一 detached candidate：
`140943d7ff9102b1b5b963e489bab006bd15fa0d` / tree
`197607f337543b88856ab50805ac2149fdd2193b` / parent
`f4b862be0b9a0a0c203ceeacc20f15b3e3e1b88f`，worktree clean。

本轮 slice receipt：
`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v3-20260824/`，
SHA256 `6406ee0d7127d75c7c81bef5acdb527eaf7a68be79fb70e2447e6620044037a0`。
验证为 H4 `3/3 + 6/6`、H5 `3/3`、provider `13/13 + 6/6`、automation `1/1 + 1/1`；
`cargo fmt --check` 通过（仅 stable 对 nightly import 配置给出 warning）。
该 receipt 只授予 local-development/shadow 资格；production flags、CALLERS、promotion、真实外部 effect 继续关闭。

### E.6 Local-development implementation slice v4 — 2026-08-24 01:xx

在 v3 之后继续收敛两个本地实现切片，并补上 H6 receipt 完整性修补：

- H4 compact persistence shadow：append-only intent、CAS parent/generation/fence、幂等 replay、hash-chain
  reopen/tamper 检查和 `Indeterminate → reconcile` quarantine；focused `5/5`。这仍是纯
  `local_development_only` contract，未接入 authoritative SQLite/WAL writer、checkpoint 或 rehydration executor。
- H6 intuition shadow：snapshot/schema/policy/epoch 绑定、hard filter、确定性 tie-break、Suggest/Abstain，
  并在 receipt 校验时重新计算 decision，拒绝自洽但错误的伪造结果；focused `7/7`。没有 runtime consumer、
  KG write、routing、model invocation 或 effect。

候选已合并为单一 detached clean head：
`dbffda2d5edc74f1d83ca3346967dc5dc462dfd6` / tree
`e18526e3a09140cba483659461ad2e35f96d00d3` / parent
`09414ff43deffae3a81dd9ccd9b9366a65e4df07`。

v4 slice receipt：
`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v4-20260824/`，
SHA-256 `8d951b69a1a2b74a2c76a03b723d35b354a97cbf7d7879552c3cfe4d22e5b5e0`。
合并回归为 memory `59/59`，H4 admission `3/3`、compact `6/6`、persistence `5/5`、H5 `3/3`、
H6 `7/7`、provider contracts/evidence `13/13 + 6/6`、automation fault/reopen `1/1 + 1/1`，
fmt/check 通过（stable 对 nightly-only import 配置仅有 warning）。

该 receipt supersedes v3 作为当前 local-development 实现证据，但不改变生产 authority：
`g5_complete=false`、`g5_allowed=false`、`operator_acceptance=false`、`promotion=false`、
`fleet_and_automation_unfrozen=false`、`provider_physical_exactly_once=false`。下一刀是把 compact
contract 接入唯一 authoritative lease/event/outbox、持久 checkpoint/reopen/rehydration executor；再把
H5/H6 接入只读 shadow consumer，仍不得取得 KG write、runtime routing 或 external effect 权限。

### E.7 H4 persistence integrity amendment — 2026-08-24 02:xx

复核 H4 persistence 后发现 checkpoint digest 若只绑定摘要字段，会留下 lease/fence/schema/namespace/
protected-reference 错绑空间。已把 amended persistence commit 纳入同一 detached candidate：

- 完整 checkpoint digest 现在绑定 schema version、namespace、context/parent event range、expected revision/state、
  authority/owner/generation/fencing token，以及排序后的 protected refs；parent digest 也绑定 authority/owner epoch。
- H4 focused `5/5`、memory 全量 `59/59`、provider `13/13 + 6/6`、automation `1/1 + 1/1`、fmt/check 全部通过。
- 当前 candidate：`7215d4793aa741af5abfc4da1529125bbe430ce9` / tree
  `c3917450eb95dbc1d95a7cf9cd62e1ea918fb114` / parent
  `dbffda2d5edc74f1d83ca3346967dc5dc462dfd6`，worktree clean。

最新 v5 receipt：
`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v5-20260824/`，
SHA-256 `65655261ab5f447a518f20ec2a76e278b85a2e0156abe50b976bc0c4c16e8969`，supersedes v4/v3/v2。
这仍是 local-development-only contract；没有把 persistence 宣称成 SQLite/WAL authoritative writer，
下一步仍是接入唯一 lease/event/outbox 与持久 checkpoint/reopen/rehydration executor。

### E.8 Local-development implementation slice v6 — 2026-08-24

在不等待 provider owner、外部 signer 或 production ceremony 的前提下，继续把本地 shadow
从 contract 推进到真实 Agent-local SQLite 持久化：

- H5/H6 shadow advisory consumer：`37697b3ea3d14076aa7a6bd94f77521c59f221d1`；只组合已绑定的
  neuron/intuition 决策，保持 deterministic、read-only、`runtime_consumer=false`，不改变 KG、routing
  或 effect authority。
- H7 local compact executor：candidate commit `14e9e2d44333f4f1d686c2827dda2bb47070cbcf`，source
  commit `9374160e22c2e11d51e93ede8eedbd021be2c4ae`。新增 migration `0004_local_compact_events`，
  用 Agent-local SQLite append-only event journal、immutable triggers、`BEGIN IMMEDIATE`、generation/fence
  校验实现 intent→commit、`Indeterminate→reconcile`、reopen/hash-chain 校验和 durable-commit-backed
  read-only rehydrate。production effect、KG writer、scheduler、routing 均未接入。
- 统一 detached candidate：head `14e9e2d44333f4f1d686c2827dda2bb47070cbcf`，tree
  `f5f7143ac155aa8a1efe20529b434f843e11d794`，parent
  `37697b3ea3d14076aa7a6bd94f77521c59f221d1`，worktree clean。
- v6 receipt：`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v6-20260824/`，
  SHA-256 `9da7c9720c472292119c7a3337cafcfc382a3f2c06d97678136303b1f9b7f12c`，supersedes v5/v4/v3/v2。
- exact-head verification：memory `68/68`、H7 executor `4/4`、H5/H6 advisory `5/5`、provider
  contracts/evidence `13/13 + 6/6`、automation unknown/crash `1/1 + 1/1`、fmt/diff check 通过。

本 slice 仍只授予 `local_development_only` 的实现/回放资格；`g5_allowed`、production writer、KG
write、routing、external effect、CALLERS、promotion 均保持关闭。下一刀是补齐同一 Agent-local DB
中的 lease/fence ownership 与 append-only event/outbox admission，再做 bounded replay；不把它接成
production caller，也不引入第二 scheduler/state bus。

### E.9 Local-development implementation slice v7 — 2026-08-24

H8 已在同一 detached candidate 上收口为可回放的本地持久化边界：

- `cognitive_local_leases`、`cognitive_local_events`、`cognitive_local_outbox` 使用 append-only
  journal、immutable trigger 与 `BEGIN IMMEDIATE`；event+outbox admission 在一个事务内完成。
- 终态或 `Indeterminate` occurrence 重试不会再返回 queued receipt；跨 generation 的旧 occurrence
  返回 `StaleFence`，不能被新 lease 重新派发。
- 新增 `acquire_local_lease_after_head`，把 lease id/owner/sequence/state/generation/fence/previous
  digest/head digest 作为严格 CAS witness；旧 generation-only API 保留为兼容入口但显式 fail-closed。
- lease history 绑定 event/outbox 的 generation/fence，禁止 fencing-token 重用；512-byte lease id
  生成的 row id 使用 digest fallback，保持数据库边界。
- 两个事务故障点（event 写入后、outbox 写入后）均验证为无部分提交；未知结果仍是
  `Indeterminate → reconcile`，queued receipt 永远不是 external effect。

统一 detached candidate：head
`938174f615d19141588331b08624a30b23e0e925`，tree
`20e9d8d2b9afb8ba86a60c25546c083760e55f8d`，parent
`d55892f1617f292dadd97a7637fdf05a7d035e3f`，worktree clean。

v7 receipt：`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v7-20260824/`，
SHA-256 `befcb4d1ee69923eb384db0e294a81d03c4d7e7fed822dfeb10fcda82f4b73bc`，supersedes v6/v5/v4/v3/v2。
exact-head verification：memory `78/78`、H8 `10/10`、H7 `4/4`、H5/H6 advisory `5/5`、provider
contracts/evidence `13/13 + 6/6`、automation crash/unknown `1/1 + 1/1`、fmt/diff check 通过。

该 slice 仍仅授予 `local_development_only` 的实现/回放资格；生产 writer、KG write、routing、真实
provider effect、CALLERS、promotion 与 fleet unfreeze 均关闭。下一刀是把 lease/outbox 接到
Codex/CognitiveRuntime 的 local-only lifecycle consumer，再做持久 checkpoint/rehydration 的 bounded
shadow replay；不得引入第二 scheduler/state bus。

### E.10 Local-development implementation slice v9 — 2026-08-24

H9/H10/H11 已在一个统一、干净的 detached candidate 上收口；这条记录只授予本地开发、回放和
shadow 资格，不授予生产 authority：

- candidate head/tree/parent：`cfcd2833ccbc73ed7a3e6a172cb400114d6d94e8` /
  `f7c3b739d9899ad85bba4b4a6f4357d86331347a` /
  `a2ccd3616b817e90852a566b18f67de77da73594`；worktree clean。
- H9/H10 local lifecycle：lease/event/outbox 仍由 Agent-local SQLite 唯一写入者维护；重复 start
  去重，acquire-before-admit 崩溃重放识别为 `NotAdmitted` 后才 admit，Queued 重放复用原 receipt，
  已终态/Indeterminate 重放以一个事务原子释放；外部 effect=false。
- H10 checkpoint/rehydration：新增 append-only `Rehydrated` witness，`BEGIN IMMEDIATE`、重开和
  hash-chain 校验通过；当前是只读 state-reconstruction plan/witness，不宣称完整状态或 KG 重建。
- H11 shadow observation：host 显式提供 typed H5/H6 input+receipt；extension 只保存 digest-only
  observation，缺字段/错 turn/坏 receipt/authority flag 会拒绝；没有 runtime registration、KG
  snapshot、routing、execute 或 provider consumer。
- exact-head verification：memory `84/84`、extension `56/56`、local lifecycle `5/5`、agentd
  app-runtime `3/3`、相关 crate `cargo check`、`cargo fmt --check`、`git diff/show --check` 均通过。

v9 receipt：`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v9-20260824/`，
`LOCAL-DEVELOPMENT-SLICE-RECEIPT.json` SHA-256
`150f41ebc0fbadeff549df2103a0d060e9083c59b906081e82197601e7dce0cd`，append-only supersedes v8/v7。
已知边界：auto-compact occurrence id 仍排除在 durable core lifecycle 之外；H11 不是 runtime consumer；
rehydrate 只写 witness/只读计划；真实 provider physical exactly-once、外部 effect、KG write、生产
caller、CALLERS、promotion 和 fleet unfreeze 仍关闭。canonical `main-integration` 未改。

### E.11 Local-development permission hardening / slice v10 — 2026-08-24

独立复核发现 v9 所继承的 agentd 启动覆盖仍把 `features.hepta_cognitive_write` 强制设为 true；这与
local-development 的 KG-write 关闭边界不一致。已在新 detached head 修正为显式 false，同时保留
Available cognitive store 仅供本地 lease/event/outbox 生命周期 journal 使用：

- candidate head/tree/parent：`338c511da160ba341456d9fe8b7a27100030ae05` /
  `f2d5e6e8f46e4c0882334bc9ecdb11747ce36658` /
  `cfcd2833ccbc73ed7a3e6a172cb400114d6d94e8`；worktree clean。
- `app_runtime` 的 agentd profile 回归 `3/3`；memory `84/84`、extension `56/56`、lifecycle `5/5`、
  相关 crate `cargo check`、`cargo fmt --check`、`git diff/show --check` 均通过。
- v10 receipt：`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v10-20260824/`，
  `LOCAL-DEVELOPMENT-SLICE-RECEIPT.json` SHA-256
  `87f3eed272f96121efeddf41ea5d5119a4baec73fc9842a0a60e623bb7df4ea2`，append-only supersedes v9/v8/v7。

该修复只收紧本地运行时权限，不把本地资格变成生产资格：`g5_complete/g5_allowed/operator_acceptance/
promotion/fleet_and_automation_unfrozen/provider_physical_exactly_once` 仍为 false；external effect、
KG writer、routing、production caller 和 CALLERS 仍关闭。canonical `main-integration` 未改。

### E.12 Local-development H12 rehydration hardening / slice v12 — 2026-08-24

继续沿 `local_development_only` 路径推进，不等待 provider owner、外部 signer、challenge 或 promotion：

- H12 提供纯读 `LocalRehydrationRead` / `LocalCompactExecutor::read_rehydration(...)`；它只验证
  append-only journal、Committed intent、精确 intent digest/revision 和 witness 状态。无 witness 返回
  `NotStarted`，有 witness 返回 `Complete`；不会追加事件、写 KG、路由、调用 provider 或产生外部 effect。
- 追加 lease 完整性硬化：反序列化后的 `CompactLease` 在 compact/checkpoint 使用前重新计算 snapshot
  digest 与 `ctxlease:v1:<digest>` ID；digest 或 ID 被篡改时 typed error、fail-closed。该校验不是 authority
  grant，也不改变 production flags。
- 最新 detached candidate：head `f8e4657a8f0736b80ecf2eb342326b2a16300a6f`，tree
  `ed63ff87d35f30086e581617988f7087fdd09a9e`，parent
  `61c0aa8cdfa0af570ef4afb9f9171f86be8ad480`，worktree clean。
- v12 receipt：`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v12-20260824/`，
  `LOCAL-DEVELOPMENT-SLICE-RECEIPT.json` SHA-256
  `9a7241d93b5fe0a0c50290c83e2427d67e873fdc419cdbbfff06ffeb9d8b052a`，append-only supersedes v11/v10/v9/v8。
- exact-head 独立复跑：memory `86/86`、extension `56/56`、local lifecycle `5/5`、agentd app-runtime
  `3/3`、lease/compact focused `7/7`、rehydration executor `6/6`、相关 crate `cargo check`、fmt 和
  git checks 均通过。

该 slice 仍只授予本地实现/回放/shadow 资格：`planning_only=true`、`production_activation=false`、
`provider_effects=false`、`kg_write_authority=false`；采用 `at-least-once + Indeterminate → reconcile`。
没有 runtime 自动注册、KG writer、routing、生产 caller、provider physical exactly-once、CALLERS、promotion
或 fleet unfreeze。managed/request config 的最终有效配置审计仍是后续硬化项；本条不把 caller-supplied
receipt metadata 当作独立信任证明。canonical `main-integration` 未改。

### E.13 Local-development H13/H14 + fence/config hardening / slice v14 — 2026-08-24

继续沿 `local_development_only` 推进；provider owner、外部 signer、challenge、CALLERS 和 promotion
不再阻塞本地实现。v14 将 H12 的纯读重建能力接到显式、仍未注册的本地 runtime read seam，并收紧
compact fence 与最终有效配置边界：

- H13 提供 typed host read seam；它只校验 turn binding 并读取 H12 的 append-only 状态，不追加
  witness、不写 KG、不路由、不调用 provider/effect，也不自动安装 runtime caller。
- H14 提供 `LocalRehydrationRuntimePlan`；在读取前后验证 lease generation/token 和 checkpoint fence，
  只返回 `NotStarted/Complete` 计划与 binding digest。它不 admit、rehydrate、reconcile、release，
  不写 event/outbox/KG，不产生外部 effect。
- compact fence 的 `authority_epoch` 与 `owner_epoch` 已持久化并参与事件 JSON、摘要、SQLite 行和
  reopen/replay 校验；旧 v1 NULL 行明确 `Corrupt`，不猜测回填或跨 epoch 复用。
- managed/request/CLI 配置层之后重新施加 agentd 的 `hepta_cognitive_write=false` 约束；effective
  config 读取也反映最终值，避免后续层重新打开 KG/cognitive write。

统一 detached candidate：head `633ff76f1618c867f47aedfdfc0d4092e8accb2d`，tree
`be55037d0a2779b877932ce03b7fb4d03a0a2d92`，parent
`31305d853235f810735e74589b6e1f8968728cf8`，worktree clean；它以 local profile base
`e14717e176f6e60976e63416052dda5e859ecad0` 为祖先，未改 canonical。

v14 receipt：`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v14-20260824/`，
`LOCAL-DEVELOPMENT-SLICE-RECEIPT.json` SHA-256
`50884e1c6e6c0c98bd108ea8fe8d902c57ec4105beb5370d07491b63d6061d7f`，append-only supersedes
v12/v11/v10/v9/v8 for local implementation tracking. Exact-head verification passed: memory `87/87`,
extension `63/63`（H13 focused `3/3`、H14 focused `4/4`）、local lifecycle `5/5`、agentd app-runtime
`3/3`、effective-config app-server `1/1`，相关 crates `cargo check`、fmt、diff/show checks 全通过。

边界保持明确：`planning_only=true`、`production_activation=false`、`provider_effects=false`、
`kg_write_authority=false`；采用 `at-least-once + Indeterminate → reconcile`。生产 writer、KG writer、
routing、真实 provider effect、provider physical exactly-once、CALLERS、operator acceptance、promotion
和 fleet unfreeze 均关闭。下一刀是继续把本地 read plan 接到显式 bounded replay/rehydration consumer；仍不得
引入第二 scheduler/state bus，亦不得把 local receipt 当成生产 authority。

### E.14 Local-development bounded replay consumer / slice v15 — 2026-08-24

在 v14 的显式 read seam 之上继续推进一个纯本地、纯只读的 bounded replay consumer：

- H15 `consume_local_rehydration_runtime_plan` 先强制执行 H14 `validate()`，再重算 H13/H14 的 turn、
  lease、fencing-token、epoch/generation 与 compact/host binding；自洽但过期、篡改或跨 turn 的 plan
  一律拒绝。
- 输出 typed `LocalRehydrationReplayPlan` 与 `NotStarted/Complete` disposition；`NotStarted` 不会隐式
  执行 rehydrate，`Complete` 只表示已经观察到本地 witness。所有 effect/KG/routing/provider/production/
  runtime-registration/replay-performed flags 固定为 false。
- 提供的 async convenience 只调用 H14 的只读准备路径；不写 `ExtensionData`、SQLite、event/outbox、
  witness 或 KG，不 reconcile/release，不注册 runtime，不启动 scheduler/state bus。

统一 detached candidate 已更新为 head `1fb5208c6fe2ae9069223b8cbe03f49f930331e1`，tree
`d4298d043ff8d5a04633570ecde5ff3a7686fa2f`，parent
`633ff76f1618c867f47aedfdfc0d4092e8accb2d`，worktree clean；local profile base
`e14717e176f6e60976e63416052dda5e859ecad0` 仍为祖先，canonical `main-integration` 未改。

v15 receipt：`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v15-20260824/`，
`LOCAL-DEVELOPMENT-SLICE-RECEIPT.json` SHA-256
`ac9906396a8cfa113f20ea46b6b07a2472a125865a56b42a6e1ae90ad396e971`，append-only supersedes v14/v12/v11。
Exact-head verification passed: memory `87/87`、extension `67/67`、H15 focused `4/4`、local lifecycle
`5/5`、agentd app-runtime `3/3`、effective-config app-server `1/1`、相关 crates `cargo check`、fmt 和
git checks 全通过。

本 slice 仍只授予 `local_development_only` 的回放/观察资格；production activation、provider effects、KG
write、routing、CALLERS、operator acceptance、promotion、fleet unfreeze 与 provider physical exactly-once
继续关闭。真实 provider/trust blocker 已移到 production-only 路径，不得用本地 replay receipt 伪造闭合。

### E.15 Local-development explicit bounded replay lifecycle observer / slice v16 — 2026-08-24

在 H14/H15 的只读边界上补齐一个显式 host-invoked lifecycle seam；本条不是自动 runtime 接线，也不把
local receipt 变成 production authority：

- 新增 `LocalRehydrationReplayLifecycleInput` 与 `observe_local_rehydration_replay`。唯一 host owner 显式
  提供当前 turn store、checkpoint、lease 和 compact executor；空 turn、错 turn、`auto-compact-*`、错
  operation/revision/checkpoint/fence 或已 terminal 的 lease 均 fail-closed。
- observer 复用 H14 的前后 lease 校验与 H13 的纯读，再执行 H15 digest-bound plan integrity validation，
  只返回 stack-owned `NotStarted/Complete`。它只做 bounded SELECT/read；host 负责外层 timeout/budget，
  helper 不创建 background task、retry 或第二 scheduler/state bus。
- 本 commit 不注册 lifecycle callback；install registry 保持原有 seams，`LIFECYCLE_REGISTERED=false`。
  observer 不 attach `ExtensionData`，不写 SQLite/event/outbox/witness/KG，不 reconcile/release，不路由、
  调 provider 或产生 external effect。真实 witness writer 留给后续独立、显式 policy-gated E.16。
- terminal lease guard、NotStarted/Complete 幂等、auto-compact/payload rebinding 与 read-only snapshot/witness
  回归均通过；provider owner、外部 signer、challenge、CALLERS、promotion 仍只属于 production-only 路径。

统一 detached candidate：head `a2437ad8908877621aba932f7ba81a224970b6cf`，tree
`a5a33df6d771796bf4a5e108695899fedcc5634f`，parent
`1fb5208c6fe2ae9069223b8cbe03f49f930331e1`，worktree clean；local profile base
`e14717e176f6e60976e63416052dda5e859ecad0` 仍为祖先，canonical `main-integration` 未改。

v16 receipt：`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v16-20260824/`，
`LOCAL-DEVELOPMENT-SLICE-RECEIPT.json` SHA-256
`dd6552c6bd7ed2ea86bb74230842a16c7c9f095c373e67c01e878a133683c721`，SHA256SUMS 已验证，append-only
supersedes v15/v14/v12/v11。Exact-head verification passed: H16 focused `8/8`、extension `71/71`、
memory `87/87`、local lifecycle `5/5`、agentd app-runtime `3/3`、effective-config app-server `1/1`、
相关 crates `cargo check`、nightly/stable fmt、`git diff/show --check` 均通过。

本 slice 仍只授予 `local_development_only` 的显式观察/回放资格；`planning_only=true`、
`production_activation=false`、`provider_effects=false`、`kg_write_authority=false`，以及
`g5_complete/g5_allowed/operator_acceptance/promotion/fleet_and_automation_unfrozen/
provider_physical_exactly_once` 全部保持 false。qualification mirror 仍为 dataless/non-evidence。

## Append-only document digest update — 2026-08-24 (after E.15)

After the E.15 append, the plan filesystem SHA-256 is
`44e880c461626c70d5032059322a650fed11a5b126c27c9453ecbf6bdd97594f` and the qualification index
filesystem SHA-256 is `a9f98087f6df33da18092b76bb2a2f454ff2f54de7a4701afd933095b197b1cd`.
These values identify the documents immediately after the E.15 content append; this digest note is itself
append-only. Historical header/digest values are not overwritten, and the qualification mirror remains
dataless/non-evidence.

### E.16 Local-development bounded lease/fence atomic witness writer / slice v17 — 2026-08-24

在 E.15 的只读 observer 之后落一个可复现、仍限于 `local_development_only` 的写切片；本条明确是
`LOCAL_PARTIAL`，不是完整 E.16 lifecycle closure，也不把本地 receipt 当成 production authority：

- core `write_local_rehydration_witness` 在同一 `BEGIN IMMEDIATE` 中重新校验当前 lease、event/outbox
  链、compact journal、generation/fencing token 与 checkpoint digest/revision，然后最多追加一个
  `Rehydrated` compact event。重复调用返回同一 witness（Replay）；故障注入在 commit 前退出时，compact
  row 与 witness 一并回滚，精确重试不会产生第二条 witness。writer 不释放或 reconcile lease，亦不写 KG、
  projection、provider 或 external effect。
- extension 提供显式 host-owned `write_local_rehydration_witness_at_lifecycle`：先观察 H16，再调用
  core writer；writer 在自己的事务中再次做权威检查，因此观察结果不是写授权。它不注册 callback、
  不 attach `ExtensionData`、不启动 retry/scheduler/state bus，也没有 production caller。
- receipt 的 lease/fencing identity 已与 H14/H15 使用相同 domain/framing，并对 digest、NUL/长度和
  boundary flags 做 fail-closed 校验；`lease_epoch_bound=false`、`lease_expiry_bound=false`、
  `policy_gate_bound=false` 是有意报告的当前 schema/owner 缺口，而不是猜测补齐。

统一 detached candidate：head `cbd5175dce6f02e6bdfc2bf33ef900d2c3b07385`，tree
`1ba59cc75035a313826a5447370f839ae64a4c91`，parent
`a2437ad8908877621aba932f7ba81a224970b6cf`，worktree clean；local profile base
`e14717e176f6e60976e63416052dda5e859ecad0` 仍为祖先，canonical `main-integration` 未改。

v17 receipt：`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v17-20260824/`，
`LOCAL-DEVELOPMENT-SLICE-RECEIPT.json` SHA-256
`88b605b7d619aea90cfd9758529972783e627d4ec756ad5cc8cff03fab94495c`，SHA256SUMS 已验证，append-only
supersedes v16/v15/v14/v12/v11。Exact-head verification passed：core `92/92`、extension `73/73`、
E.16 core focused `5/5`、extension focused `2/2`、agentd app-runtime `3/3`、effective-config
app-server `1/1`、相关 crates `cargo check`、Rust nightly/stable fmt 和 git checks 均通过。

未闭 blocker 保持显式：当前 lease migration 没有持久化 `authority_epoch/owner_epoch/lease_expires_at`；
compact witness row 没有持久化 `lease_id` 与 compact head/event binding；core/app-server lifecycle owner
和正面的 policy gate 尚未注册；该 qualification-only caller-zero surface 不新增 production `CALLERS`。
因此 `production_activation=false`、`provider_effects=false`、`kg_write_authority=false`、
`g5_complete/g5_allowed/operator_acceptance/promotion/fleet_and_automation_unfrozen/
provider_physical_exactly_once` 全部保持 false，qualification mirror 仍为 dataless/non-evidence。

## Append-only document digest update — 2026-08-24 (after E.16)

This digest note is appended after the E.16 content above; the prior E.15 digest remains historical and is not
overwritten. The new filesystem SHA-256 values are:

- plan: `48313375383c548562d21b8da973249b30072ec5d8662bfad9461e3c73b34410`
- qualification index: `89ab03ba7c192864af44447a9bc290cc32950bb1b518367e09ec6223129a0894`

## Append-only digest correction — 2026-08-24

The preceding E.16 digest values identify the pre-value-line snapshot. The filesystem digests immediately before
this correction note (and therefore the values to use for the current append-only state) are:

- plan: `eff92be80b236d19e1340795e2f6ee6881fb0c3800aaed925bba30dae576ef7b`
- qualification index: `5fb4bc85e04a5eaa3c14bbde8270a1466a5f9e782d9190cb28affb5071746840`

### E.17 E.16 schema/policy closure and local-development receipt v18 — 2026-08-24

本条是 E.16 `LOCAL_PARTIAL` 后续的本地实现收口；它只闭合可由 Agent-local code、schema 与 host-owned
policy 实际证明的部分，不把 qualification receipt 解释成 production authority：

- migration `0007_local_lease_compact_binding` 将 `authority_epoch`、`owner_epoch`、
  `lease_expires_at_unix_seconds` 持久化到 lease journal，并将 bound lease 字段纳入 v2 hash framing。
  legacy H8 unbound API 仍可用于旧的 local admission seam，但 schema-bound witness writer 对 NULL、部分
  binding、过期 lease、digest/owner/generation/fencing 不一致一律 fail closed；不会自动 takeover 或续租。
- compact event row 现在持久化 `lease_id`、`lease_head_sha256`、`compact_previous_sha256` 与
  `compact_event_binding_sha256`。bound executor 在打开与写入时都重算并校验这些值；atomic witness writer
  在同一个 `BEGIN IMMEDIATE` 内再次核对当前 lease head、epochs、expiry、compact fence、event/outbox chain
  与 checkpoint digest/revision。重复调用仍是 Replay，commit 前故障仍完整回滚。
- 新增 closed-world `local_development_only` policy：必须 qualification-only、caller-zero、显式 writer bit，
  且 automatic lifecycle registration、production activation、provider/external effect、KG、routing 与
  production caller 全部为 false。app-server 导出显式 host-owned lifecycle owner；agentd 只有在显式 policy
  下启用本地资格，普通 app-server/MCP caller 保持 None。owner 不创建 callback、scheduler、retry 或 provider
  effect；host 负责 timeout/budget 与后续 lease terminal decision。
- 两个既有 G5 Python ledger 脚本只做 ruff 格式化，无逻辑/authority 变化；`just fmt-check` 随之恢复绿色。

统一 detached candidate：head `e34051e7abcdf0560112131e9bddb40e3c4a8b8c`，tree
`21b139671cb1953ef11507d3fa5d976e4ae8bce5`，parent
`0d0a1bffcc09c1d24528414f8520712d079f0650`，worktree clean；canonical `main-integration` 未改。

v18 receipt：`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v18-20260824/`，
`LOCAL-DEVELOPMENT-SLICE-RECEIPT.json` SHA-256
`d78e0db0e2a104f283a6409607336ab380c7116ba6154454d8af51cda6d0b7f1`，`SHA256SUMS` 已验证，append-only
supersedes v17/v16/v15/v14/v12/v11。Exact-head verification passed：core `94/94`、extension `74/74`、
app-server `302/302`、agentd `19/19`、MCP server `20/20`、`just fmt-check`、Rust nightly/stable fmt、
`git diff --check` 全部通过。

本地 slice 现为 `LOCAL_COMPLETE`（仅指上述 local-development implementation surface）。production
provider/trust、external signer/ceremony、CALLERS、operator acceptance、promotion、fleet unfreeze 与
provider physical exactly-once 仍是独立治理/authority blocker，且没有被本条或 receipt 伪造闭合：
`production_activation=false`、`provider_effects=false`、`kg_write_authority=false`、
`g5_complete/g5_allowed/operator_acceptance/promotion/fleet_and_automation_unfrozen/
provider_physical_exactly_once` 继续全部为 false；qualification mirror 仍为 dataless/non-evidence。

## Append-only document digest update — 2026-08-24 (after E.17 / v18)

The filesystem SHA-256 values after the E.17/v18 append are:

- plan: `c839edf98222e4e441162070ea87c8cab8a616314f7156ca0d54e1a0d7a87ea1`
- qualification index: `e113ec1af10447334dd93e7efb40f694413f8e49c8628516c5ea992fa7c8cb77`

This digest note is append-only; all historical plan/index values remain unchanged.

## Append-only digest correction — 2026-08-24 (after v18 digest notes)

The filesystem digests including the immediately preceding digest notes are:

- plan: `a68e2b7325abf0432a51eaaaf1b065f293b56883045fb2c425fc2aabe6ca910a`
- qualification index: `d6952884c32612eba5b2e9fbe6e5c41da9efa0c5861c745fbd70bec7897afbf9`

## Append-only current-state digest — 2026-08-24

Immediately before this note, the filesystem digests were:

- plan: `dba02894545994e036c65b467a3bbadd06a1e0f683248f1da1b7c7e9a2b29703`
- qualification index: `1a1666328bd88fe1b0222a92876958510f49f01353acebef6634e8455bb83c10`

### E.18 Bound compact mutation revalidation / slice v19 — 2026-08-24

本条是对 E.17/v18 审计发现的真实 correctness gap 的 append-only 修订，不把旧 receipt 静默改写：

- `LocalCompactExecutor` 现在持有打开时使用的精确 `LocalLeaseOutbox` handle。所有 bound mutation（append、commit、
  `mark_indeterminate`、reconcile、rehydrate）在同一 `BEGIN IMMEDIATE` 内重新验证 active state、lease head、
  authority/owner epoch、expiry、event/outbox chain，并与 open-time binding 比对；release、terminal、stale 或 expiry
  后一律 `StaleFence` fail closed，compact journal 的 entries/head/SQL row count 保持不变。旧 unbound API 仍仅作
  shadow/compatibility seam，不获得 E.18 授权。
- 新增 active replay、lease release/no-side-effect、expiry-after-open 回归，覆盖五类 mutation 与幂等 witness replay；
  证明事务内校验确实关闭 observation/open 后的 TOCTOU 窗口。该修订不注册 callback/scheduler，不写 KG、provider 或
  external effect，也不自动 release/reconcile lease。
- policy 语义保持 closed-world：`automatic_lifecycle_registration=false` 表示 policy 本身绝不自动注册；只有既有
  explicit host embedding opt-in 且 policy 完整有效时，才可启用 local contributor。它不是 background registration。

最终 detached candidate：head `c1ef681e5fc2d1066a3e02fc36a9b020ea3bfd93`，tree
`e1dd337ecb1e90d94056659578e916f52010a15f`，parent
`e34051e7abcdf0560112131e9bddb40e3c4a8b8c`，worktree clean；canonical `main-integration` 仍未改。

v19 receipt：`/Volumes/T5/hepta-vnext/artifacts/r2-g5-local-development-slice-v19-20260824/`，
`LOCAL-DEVELOPMENT-SLICE-RECEIPT.json` SHA-256
`d191f2c897c9a87112d22ce9176e263436fa3ca2c6d31d9b6cfb98539ce0f9fd`，`SHA256SUMS` 已验证，append-only
supersedes v18/v17/v16/v15/v14/v12/v11。Exact-head verification：core `97/97`、extension `74/74`、
app-server `302/302`、agentd `19/19`、MCP `20/20`、dependent cargo check、`just fmt-check`、Rust
nightly/stable fmt、`git diff --check` 全部通过。

截至本条，所有可由本地代码、schema、事务语义与 host-owned policy 完成的开发计划 slice 均已闭合为
`LOCAL_COMPLETE`。仍保持独立且不可伪造的 production authority blockers：provider/trust、external signer ceremony、
production `CALLERS`、operator acceptance、promotion、fleet unfreeze 与 provider physical exactly-once；所有
production flags（`production_activation`、`provider_effects`、`kg_write_authority`、`g5_complete`、`g5_allowed`、
`operator_acceptance`、`promotion`、`fleet_and_automation_unfrozen`、`provider_physical_exactly_once`）继续为 false。

## Append-only document digest update — 2026-08-24 (after E.18 / v19)

The filesystem SHA-256 values after the E.18/v19 append are recorded in the companion qualification index digest note;
historical values above remain unchanged and this plan remains append-only.

---

## E.19 Architecture & Delivery Plan v1.4 amendment — Neuron runtime, closed-loop learning and governed plasticity (2026-08-24)

### E.19.0 版本语义、适用范围与当前状态

本节是对今日讨论的整理，以及面向下一批实现 PR 的 **Implementation Contract v1.4 提案**。它采用 append-only 方式追加，**不改写 v1.3 的历史文字、receipt 或 digest**。

- v1.3 仍是 canonical architecture baseline；E.19 是 Neuron/longitudinal-learning 的 superseding implementation interpretation。
- E.19 的 `设计完成` 不等于 `runtime implementation`、`canary` 或 `production promotion`。
- H5/H6/H7 现有 receipt 仍分别属于 semantic qualification、fixture 或 shadow；不得把它们解释为真实模型推理、真实 NPU efficacy、真实 action feedback 或长期学习证明。
- 生产 `CALLERS`、`g5_allowed`、`operator_acceptance`、`promotion`、fleet/automation、provider/effect authority、KG writer 和模型/NPU 接入继续保持 false。
- 当前 canonical 文件没有把 `NeuronGraph` / `TopologyProposal` 当作已实现产物；它们在 E.19 中是 proposal-only contract，须经过后续独立 PR、verifier、shadow、canary 和 rollback。

### E.19.1 今日讨论的统一结论

| 主题 | E.19 决策 | 当前证据状态 |
|---|---|---|
| Neuron 的角色 | Intelligence Plane 的快速、受约束 typed perception/scoring provider；只产生 `NeuronSignal`/`ModelReceipt`，不拥有事实、权限、workflow 或 effect | H5 isolated shadow；无生产 runtime seam |
| 单个 neuron 的定义 | `PositionContract/NeuronSpec + trigger/state + typed ports + approved artifact + calibration + gate + receipt`；模型是可替换 backend，不是功能定义 | 合同/fixture 已有部分字段；trigger、typed edges、真实 state seam 待实现 |
| 物理实现 | 一个 group 可共享 encoder/model pool；多个逻辑 neuron 使用 task-specific head/adapter/calibration；不采用“一 neuron 一独立模型”硬规则 | H5 仅证明共享池元数据，未证明真实推理 |
| 小模型 | 窄任务优先使用 tiny classifier、embedding、reranker 或线性/树 ranker；规则保留硬安全门；生成模型只作 teacher/proposal/fallback | 候选已研究；模型转换、M5 latency/ANE 仍未实测 |
| 快环 | `Event/State → Neuron → Intuition → DecisionReceipt → TaskFlow → Effect/Postcondition`；一次运行内不改权重、图、权限或目标 | 目前 H5/H6 advisory/no-op，H6 `execute_allowed=false` |
| 慢环 | `Trajectory/Feedback → NDU credit/replay/OPE → artifact 或 TopologyProposal → eval/shadow/canary/rollback → Registry → next snapshot` | H7 仅 offline shadow；无真实 causal consumer |
| 参数塑性 | 允许 head/adapter/temperature/calibration/threshold/routing 的离线有界更新 | v1.3 已定义 learnable surface；未达 production efficacy |
| 图塑性 | 只允许 proposal-only 的 add/split/merge/retire/rewire；运行中不换图，下一 snapshot 原子加载 | `TopologyProposal` 尚未实现 |
| 长期学习声明 | 没有真实 action→outcome→feedback→trajectory 只能称 observational/qualification；L2 才可称 closed-loop policy learning；L3 另称 governed structural plasticity | 当前约 L0，带 L1 shadow evidence；L2/L3 blocked |

### E.19.2 能力与声明等级（Claim Ladder）

所有交付物、receipt、README 和对外说明必须标注以下等级，不得把较低等级的 evidence 升格：

| 等级 | 能力定义 | 必要证据 | 当前状态 |
|---|---|---|---|
| L0 | 静态/影子推理：输入产生 typed signal，但没有真实 action/outcome | schema、receipt、fallback、validator、golden replay | H5/H6 shadow，当前基线 |
| L1 | Observational continual learning：memory/retrieval/calibration/forgetting 从 observation、纠正和 drift 中离线更新 | 脱敏 trajectory、跨窗口 retention/forgetting、calibration/no-regression | H7 fixture/shadow，未生产化 |
| L2 | Closed-loop policy learning：真实授权 action、effect/postcondition、feedback 和 causal credit 归因到下一版 policy/artifact | Agent-local writer、typed adapters、EffectReceipt、propensity/support、OPE/CI、artifact reload/rollback、跨周/月 efficacy | 尚未实现；H5b/H6b/H7a prerequisites blocked |
| L3 | Governed structural plasticity：版本化 graph proposal 经过 compiler、ablation/lesion、shadow、canary、operator 和 rollback | `TopologyProposal`、GraphCompiler、graph lineage、support-aware evaluation、canary/rollback | 仅 E.19 提案 |

任何“自我进化”“长期学习”“仿生结构生长”的产品级措辞，必须引用对应等级的 receipt；L0/L1 不得宣称 L2，L2 不得自动宣称 L3 或生物机制复现。

### E.19.3 双环架构与 authority 边界

```text
FAST RUNTIME LOOP (one RunStartSnapshot; no online mutation)
TypedEvent / StateSnapshot
  → ActivationGate / FeatureBuilder
  → Neuron groups (shared backend + task heads | deterministic guards)
  → Calibration / Validator / Veto
  → provisional NeuronSignal + ModelReceipt
  → Intuition hard-filter / rank / choose | ask | abstain | escalate
  → DecisionReceipt
  → TaskFlow authorized activity/effect
  → ActivityIntent/ActivityReceipt (internal/read-only)
  OR EffectIntent → EffectReceipt
  (Indeterminate → reconciliation → Postcondition | terminal)
  → append immutable trajectory/event

SLOW LEARNING LOOP (cross-run/week/month; proposal-only)
Immutable trajectory + user correction + utility + drift
  → NDU hierarchical credit / replay / support-aware OPE
  → signed head/adapter/calibration artifact OR TopologyProposal
  → independent eval + ablation/lesion + no-regression
  → shadow → single-Agent canary → operator/signature
  → Registry promotion / rollback
  → next RunStartSnapshot loads approved digest
```

硬边界：

1. Neuron 不直接写 memory、KG、provider、tool 或 external effect；它只输出 typed proposal/signal。
2. Intuition 只能在 approved workflow registry 的固定图上作选择；TaskFlow 才拥有 action/effect 语义和执行 CAS/lease/fence。
3. NDU 是 slow control plane，不是 runtime driver；NDU 不可用时，runtime 必须保留旧 artifact、旧 graph 和 deterministic baseline。
4. 每个 RunStartSnapshot 固定 graph digest、model/head/calibration/policy digest、generation、authority epoch、owner fence、privacy profile 和 resource budget；运行中不得更换。
5. 任何 feedback 必须通过 immutable trajectory，并带 causal parent、receipt digest 和适用的 provenance。涉及 policy-action/OPE 的事件必须带 propensity/support；纯 observation、用户纠正、drift 或 memory 事件允许 `null`，但必须附 `reason=not_applicable`，未绑定的 signal 不得进入 NDU credit。
6. 所有 command/transition/DecisionReceipt/ActivityReceipt/EffectReceipt 继续沿用既有 `command_id + expected_revision + authority_epoch + owner_epoch + generation + fencing_token` CAS/fence 语义；E.19 不创建第二套 neuron-specific authority protocol。

### E.19.4 NeuronSpec / NeuronGraph v1.4 合同

`NeuronSpec` 最少包含：

```text
identity: neuron_id, group_id, position_id, version, owner, purpose
contract: input_ports, output_schema, schema_digests, upstream/downstream typed ports
trigger: event_types, activation_predicate, debounce, min_interval, priority, cooldown
state: stateless | windowed | bounded_recurrent; state_schema, state_digest, max_bytes, TTL, tick_budget, causal_parent, reset_fence, reset conditions
artifact: model_digest, head/adapter_digest, calibration_digest, tokenizer/schema digest
runtime: allowed_devices, fallback_profile, timeout, latency/RSS/energy budget, concurrency
privacy: data class, redaction, remote_allowed, retention and evidence scope
authority: fixed | observe_only | proposal_only | runtime_read; provisional/effect flags
gate: threshold, hysteresis, abstain policy, validator/veto and coverage floor
receipt: input/output/model/policy/calibration/evidence/fence/attempt-chain digests; command_id, expected_revision, authority_epoch, owner_epoch, generation, fencing_token
feedback: reward dimensions, trajectory refs, update cadence, NDU position
lifecycle: phase, expiry/revoke, parent lineage, promotion and rollback predecessor
```

`NeuronGraph` 的每条 edge 必须包含 source/target port、input/output schema digest、causal parent、privacy/capability boundary、fan-in/fan-out、depth/cycle/recurrence budget、resource cost 和 graph version。所有 edge 是 typed dataflow，不把共享模型权重误称为突触连接。

在线执行顺序固定为：

`EventOwner → RunStartSnapshot → FeatureBuilder(evidence/state digest) → backend selector → inference → calibration → schema/evidence/fence/privacy/budget validator → NeuronSignal/ModelReceipt → GroupAggregator/Intuition`。

H5 fixture 的 `request.output`、scripted backend plan、fixture confidence 或 group threshold 不得被当作真实 inference、真实 gate 或长期 efficacy。fallback 必须按 position/privacy profile 选择，并在每次 backend/device 变化后按 `(position_id, model_digest, backend/device, schema_version)` 重新校准或强制 abstain；`ModelReceipt` 记录完整 attempt chain。

### E.19.5 Model / backend selection matrix

模型候选只作为可替换 artifact，必须固定 commit/digest、tokenizer、许可证、转换产物和 compatibility matrix；模型卡属性不等于 Hepta efficacy。

| Tier / candidate | 适合的 neuron 功能 | 约束与状态 |
|---|---|---|
| Tier 0：规则、状态机、linear/GBDT、统计监控 | capability/scope/schema/provenance/tombstone/forget、hard risk veto、postcondition、workflow tie-break、drift | 永远保留 deterministic baseline；不得用模型解除硬 veto；不远程 |
| `BAAI/bge-small-zh-v1.5`（约 24M、4-layer、512d、MIT） | 中文 salience/novelty/entity linking/retrieval 特征；首个本地候选 | 冻结 encoder，训练 Hepta task head + calibration；中文 efficacy 需脱敏 corpus 验证；[model card](https://huggingface.co/BAAI/bge-small-zh-v1.5) |
| `sentence-transformers/all-MiniLM-L6-v2`（约 22.7M、384d、6-layer、Apache-2.0） | 英文 intent/topic/similarity/retrieval | 主要英文、短输入；不得直接宣称中文能力；[model card](https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2) |
| `intfloat/multilingual-e5-small`（12-layer、384 hidden、多语、MIT） | 中英混合 embedding、跨语言 recall/entity candidate | 词表/内存较重，只有在 M5 benchmark 后才注册第二 pool；[model card](https://huggingface.co/intfloat/multilingual-e5-small) |
| `cross-encoder/ms-marco-MiniLM-L6-v2`（约 22.7M、Apache-2.0） | top-k pair rerank | 英文训练；中文必须另训/验证，不能直接作为中文 Memory authority；[model card](https://huggingface.co/cross-encoder/ms-marco-MiniLM-L6-v2) |
| Qwen2.5-0.5B / SmolLM2-135M 等生成模型 | 离线 teacher、proposal、解释或显式允许的 fallback | 不得担任 risk/permission/schema/effect authority；生成 prose 不能直接成为事实或 NeuronSignal |

部署 profile 统一为：`guard_deterministic`（规则/本地执行）、`privacy_local`（Core ML/NPU → CPU/MLX → rule/abstain）和 `proposal_local`（在显式脱敏与 policy 允许时才可 remote）。计划中的旧式 `rule → CPU/MLX → remote` 与 H5 fixture 的 `NPU → CPU → remote → rule` 不再作为全局链；以 position-specific profile 为准。

Core ML conversion 必须单独验收 operator coverage、M5 p50/p95、RSS、能耗、量化误差、ECE/Brier、coverage/abstain 和 fallback；MLX 只作为 CPU/GPU fallback，不假定直接使用 ANE。H5 的 scripted 40ms/80ms/32KB 预算不可外推为真实硬件性能。

### E.19.6 Functional neuron implementation matrix

| group / position | trigger | typed input → output | backend | 可学习面 | 冻结硬门 | NDU position / phase |
|---|---|---|---|---|---|---|
| Perception / intent-topic-urgency | `turn_observed` | bounded text、locale、context digest → labels、constraints、urgency、confidence、evidence | BGE-zh/MiniLM/E5 shared encoder + multitask heads | head/adapter/temp/calibration | schema/privacy/fence/coverage | `perception-head`，H5a/H5b |
| Entity / relation | `turn_observed`、`memory_candidate` | spans、scope、candidate entities → entity/link score/relation proposal/evidence | embedding + ANN + light linker/token head | linker/ranker/calibration | KG authority、provenance、tombstone | `entity-link`，H5b |
| Memory / salience-admission | `turn_observed`、`task_end`、`tool_receipt`、`user_correction` | novelty、recency、goal relevance、recall digest、utility → salience、remember candidate、freshness、confidence | shared encoder + tiny MLP/linear scorer；novelty/freshness rule | salience head、decay/retrieval prior/calibration | Memory Admission 决定写入；scope/provenance/forget 固定 | `memory-admission`，H5b→H7a |
| Risk / validation | every signal/effect/postcondition | proposal、capability、scope、evidence、receipt → risk/anomaly/injection、veto/abstain reason | deterministic guard first + optional tiny classifier/one-class detector | threshold/calibration/observe anomaly | hard veto、permission、invariant、effect semantics 固定；不远程 | `risk-threshold` / `postcondition-check`，H5b→H6b |
| Workflow / Intuition | after group signals | protected recall、approved candidates、preconditions、resource state → rank/branch/ask/abstain/escalate + DecisionReceipt | linear/GBDT/pairwise ranker/contextual constrained policy | ranking、threshold、retry/abstain/calibration | graph/capability/effect semantics 固定；不用 free-form LLM | `workflow-select`/`branch-route`，H6a/H6b |
| Feedback Observer / utility / drift | `flow_terminal`、effect receipt、periodic eval | outcome、correction、latency、cost、retention → reward vector、drift/forget/calibration signal | Brier/ECE/PSI/KS/change-point/statistics；不是 neuron backend | NDU credit/drift/forget artifacts | online authority=observe only | `feedback-credit`，H7a/H7b |
| Postcondition | effect receipt/state change | typed receipt + expected state → validated/indeterminate/anomaly | deterministic checker + optional one-class detector | anomaly threshold only | no effect authority | `postcondition-check`，H6b/H7b |

所有行的 output 只能是有限 schema 的 label/score/candidate/reason/evidence refs；不得输出未验证 prose 作为事实。H5→H6 必须使用真实 typed adapter，禁止手工 synthetic feature 作为闭环证据。

### E.19.7 TopologyProposal：受治理的图塑性

参数塑性和图塑性必须分开：

- 参数塑性：head/adapter/temperature/calibration/threshold/routing；固定 graph 上由 NDU 离线提出，属于 L1/L2 路径。
- 图塑性：`add | split | merge | retire | rewire`；只允许 `authority=proposal_only` 的 `TopologyProposal`，属于 L3 路径。
- 在线自由改代码、拓扑、权限、base model、目标、invariant 或 effect semantics：永不支持。

`TopologyProposal` 必须带：`graph_version`、`parent_graph_digest`、`position_id`、Node/Edge schema、causal lineage、novelty/coverage/drift/abstain/error-cluster evidence、privacy/capability、resource/latency/RSS/energy budget、预期 utility、registry migration、checkpoint/trajectory schema compatibility、retired-node tombstone/lineage、兼容/迁移计划、rollback predecessor、proposal signer 和 expiry。

`NeuronGraphCompiler` 必须拒绝 schema/port 不兼容、无界环/递归、fan-out/depth 超限、隐私/权限越界、effect boundary 穿透、资源超限或旧 artifact 不兼容。候选图依次经过 paired replay/support audit、ablation/lesion、no-regression、shadow、single-Agent canary、operator acceptance、签名和 rollback；既有 run 不换图，新图只在下一 `RunStartSnapshot` 原子加载。若新节点/新边没有历史 action support，OPE 只能保持 `research_only`，必须先生成受限 shadow 数据。

NDU 或 governance 不可用时，保留旧 graph、旧 artifact 和 deterministic baseline；不得因 proposal 失败而自动扩图、自选权限或触发 effect。

### E.19.8 L2 closed-loop implementation gates

下一批实现必须拆成可独立验收的子门，不得以 H5/H6/H7 总 receipt 代替：

| 子门 | 必须实现 | 硬失败条件 |
|---|---|---|
| H5a model bake-off | 固定 tokenizer/model digest、Core ML/CPU conversion、M5 benchmark、privacy/fallback receipt | 无真实 model artifact、无 operator coverage 或只用 scripted latency |
| H5b typed neuron runtime | EventOwner、NeuronSpec、FeatureBuilder、sidecar、ModelReceipt、calibration/validator | `request.output` 直透、无 causal parent、无 attempt-chain 或可直写 effect |
| H6a causal adapter | H5 Signal/Receipt → H6 typed input，保留 schema/model/policy/fence/calibration digest | synthetic feature、未绑定 Signal、过期/stale receipt 被接受 |
| H6b action seam | DecisionReceipt → TaskFlow authorized `ActivityIntent/ActivityReceipt`；外部副作用另走 `EffectIntent/EffectReceipt/Indeterminate`，统一 reconciliation/postcondition | `execute_allowed=false` 仍被解释为 action efficacy，action 无 receipt，或 sandbox 绕过 indeterminate reconciliation |
| H7a trajectory writer | Agent-local authoritative writer/outbox，完整 turn→feedback causal chain | 只写 fixture、缺 feedback/propensity/support/terminal state |
| H7b NDU reload | replay/OPE/credit、cross-window eval、signed artifact、next-snapshot reload、rollback | 无 CI/support/no-regression，或在线改权重/图/权限 |
| H8a closed-loop sandbox canary | local sandbox action、operator acceptance、rollback rehearsal | 任何 production caller/provider/KG/effect 越权 |
| H9/L3 topology | TopologyProposal/compiler/ablation/lesion/canary/lineage | 运行中换图、无 support、无旧图 rollback |

第一条垂直切片固定为 `salience_neuron_closed_loop`：

`turn/task/tool receipt → BGE-small-zh encoder → salience head → calibration → validator → MemoryAdmission → authorized local sandbox action → EffectReceipt/Postcondition → user correction → immutable trajectory → NDU shadow`。

该 sandbox action 只能是可回滚、无 provider/KG/生产 effect 的本地动作；它用于验证 causal chain，不授予 production authority。

### E.19.9 长期学习评测与 claim policy

每一组必须预注册 metric、样本数、置信区间、baseline、no-regression 规则和 rollback gate；不得只报告 offline loss 或单次 toy trajectory。最低指标包括：

- action utility/goal success、postcondition correctness、人工纠正与安全 abstain；
- propensity/support、IPS/OPE CI、hierarchical credit completeness；
- retention/forgetting、memory pollution、tombstone/forget lineage；
- calibration ECE/Brier、coverage、fallback 分布和 stale receipt rejection；
- p50/p95 latency、RSS、能耗、Core ML operator coverage、remote/privacy violation；
- 跨周/跨月 drift、no-regression、seed/hardware ablation 和 lesion；
- graph lineage、artifact compatibility、rollback/recovery time（L3）。

没有 action receipt、postcondition、feedback 和 causal support 时，只能声明 `observational correlation / qualification`；有完整 L2 证据后才可声明 `bounded closed-loop policy learning`；L3 必须另有 topology proposal 证据，仍不能自动称为生物神经机制。

### E.19.10 阶段、交付物与回滚顺序

E.19 的 S0–S6 顺序是对旧 §16 neuron/H5b/H6b/H7a 排序的 superseding implementation order；旧顺序保留为历史 qualification 记录。既有 `memory-review` vertical slice 仍是 L0 read-only qualification slice，不得与后续 L2 sandbox 混称为同一闭环。

1. **S0 Model bake-off**：BGE-small-zh、multilingual-e5、规则 baseline；产出模型评测 receipt，不装生产模型。
2. **S1 Contract v1.4**：`NeuronSpec`、`NeuronGraph`、typed adapter、receipt schema、fallback profile；独立 schema/verifier。
3. **S2 Runtime shadow**：Core ML/CPU sidecar、一个 salience slice、calibration/fallback/validator；不写 memory/KG/effect。
4. **S3 Sandbox closed loop**：`salience_neuron_closed_loop` 绑定 DecisionReceipt/TaskFlow/ActivityReceipt 或 EffectReceipt/reconciliation/postcondition/trajectory seam；只允许 local sandbox，完整 rollback rehearsal。
5. **S4 Longitudinal efficacy**：脱敏跨周/月 corpus、NDU replay/OPE/credit、artifact reload、retention/forgetting/no-regression。
6. **S5 Governance canary**：独立 operator acceptance、CALLERS qualification、single-Agent canary、签名 promotion；在此之前 `g5_allowed=false`。
7. **S6 L3 topology proposal**：只在 L2 稳定后开放 TopologyProposal；新图有 lineage/support/ablation/rollback，失败保留旧图。

新增实施合同（规划交付物，不代表本条已生成文件）：

- `HEPTA_NEURON_RUNTIME_CONTRACT_V1_4.md`
- `HEPTA_NEURON_MODEL_EVAL_MATRIX.md`
- `HEPTA_TOPOLOGY_PROPOSAL_CONTRACT.md`
- `HEPTA_SENSORIMOTOR_LOOP_PROTOCOL.md`
- `HEPTA_LONGITUDINAL_EVAL_PROTOCOL.md`

### E.19.11 风险与禁止项

- 小模型过度信任、中文/跨域分布漂移、synthetic-to-real gap；
- fallback/device 之间 calibration 不兼容，远端 fallback 泄漏隐私；
- feedback attribution、propensity/support 不足导致 OPE/credit 假阳性；
- closed-loop reward hacking、duplicate effect、indeterminate 误重试；
- graph churn、资源爆炸、旧 trajectory/artifact 不兼容、灾难遗忘；
- 将共享 model pool 当作 neuron connectivity，把 fixture/no-op 当作 inference，把 ANE capability 当作 efficacy；
- 将 L0/L1 evidence 宣称为 L2/L3，或将工程类比宣称为生物机制复现。

以上风险一律采用 fail-closed、deterministic baseline、版本化 receipt、shadow/canary 和 rollback 处理；不得通过放宽权限或删除证据来“修复”指标。

### E.19.12 本条交付边界

本 E.19 只更新开发计划的设计、依赖、验收和治理说明；没有修改 Hepta 源码、canonical G4/G5、CALLERS、production writer/effect、模型、NPU、operator acceptance 或 promotion。E.19 本身的状态为 `PROPOSED_IMPLEMENTATION_CONTRACT_V1_4 / SHADOW_PLANNING_ONLY`。

## Append-only document digest update — 2026-08-24 (after E.19 / v1.4 amendment)

The filesystem SHA-256 immediately before this note was
`4cdc85beb175200878cf69b2e50bdf9d86a12123940ac7da4faa0a0096c43ba7`.
The final post-note plan digest is recorded in the companion qualification index and delivery manifest;
this note is append-only and does not rewrite any historical digest.

## Append-only digest correction — 2026-08-24 (after E.19 contract wording review)

The plan digest immediately before this correction was
`1774aa7fcc5451dd8f8f4723eb25a520e1c2e3eb5f957932e8d9770cb2d3906b`.
The final post-correction digest is recorded externally in the qualification index update; no prior digest
or receipt is overwritten.

## Append-only digest correction — 2026-08-24 (after E.19 authority/loop wording review)

The plan digest immediately before this correction was
`9623cd8b426b882321078ec1c1540ff83b81fbadf0c87b769d7a8cbf7f470c29`.
The final post-correction plan digest is recorded in the final qualification-index delivery check.

## E.20 Effective-version, mirror authority and delivery-manifest closure (2026-08-24)

本条是交付一致性 blocker 的 append-only 收口。它不改写历史 receipt、旧 digest 或生产权限；只定义
当前有效版本、章节优先级、Dropbox 文档的 authority 关系和可复核的文件交付不变量。

### E.20.0 当前有效版本与优先级

从本条起，实施团队使用 `E.19 + E.20 / v1.4 implementation-contract proposal` 作为 Neuron、
closed-loop learning、NDU adapter 和 topology-plasticity 的有效解释。文件头的 v1.3 是历史架构基线，
不是另一套并行实现指令。若历史段与 E.19/E.20 冲突，按以下优先级解释：

1. G4/G5、protocol/CAS/fence/effect/governance 的硬安全边界；
2. E.19/E.20 的 v1.4 implementation contract 与本条 crosswalk；
3. 已批准的独立 contract、receipt、verifier 和 registry manifest；
4. v1.3 历史章节，仅在不与以上条款冲突时作为背景和 lineage。

| 历史语义 | v1.4 有效入口 | 说明 |
|---|---|---|
| H5 neuron group | H5a model bake-off → H5b typed neuron runtime | H5 receipt 仍是 isolated shadow，不是 runtime inference |
| H6 intuition policy | H6a typed causal adapter → H6b action seam | `execute_allowed=false` 不得解释成 action efficacy |
| H7 NDU shadow | H7a authoritative trajectory writer → H7b NDU reload | 先有 action/postcondition/feedback，才可做 policy credit |
| H8 production/canary | H8a local sandbox canary → S5 governance canary | sandbox acceptance 与 production operator acceptance 分离 |
| H9 fleet/automation | S5 production governance；S6/L3 topology proposal | fleet promotion 与图塑性不再复用同一门语义 |
| 旧 §16 阶段顺序 | E.19.10 S0–S6 | 旧顺序保留为历史 qualification 记录 |
| 旧 §6.8 feedback group | E.19.6 Feedback Observer（observe-only） | 不是可直接执行的 neuron backend |
| 旧 §6.9/§6.10 NDU/topology | E.19.7 + E.19.8 + S4–S6 | NDU 只做慢环；图变更 proposal-only |

### E.20.1 文档 authority 与镜像语义

- canonical plan：`/home/qian-qi/Dropbox/OpenClaw/hepta-vnext-development-plan-final-2026-08-23.md`；
- canonical qualification index：`/home/qian-qi/Dropbox/OpenClaw/HEPTA_VNEXT_QUALIFICATION_INDEX.md`；
- qualification package mirror：`hepta-vnext-qualification-2026-08-23/hepta-vnext-development-plan-final-2026-08-23.md`
  与同目录 `HEPTA_VNEXT_QUALIFICATION_INDEX.md`。

canonical plan/index 是唯一文档 authority。qualification package 中的两个同名文件是为离线审计和外置盘
交付提供的 byte-identical archival mirror；它们不是另一份 authority，也不单独授予任何 receipt、模型、
NPU、writer、effect、CALLERS 或 promotion 权限。镜像必须与 canonical 文件逐字节一致；若同步中断，
manifest 状态必须是 `MIRROR_STALE`，不得再声明 `byte_identical`。历史 qualification receipt 不移动、
不覆盖、不重算；它们的 sealed worktree 与各自 `SHA256SUMS` 仍是证据来源。

### E.20.2 Digest 与 manifest 规则

文档不嵌入自身最终 SHA，也不把旧 header digest 当作当前值。交付 manifest 在
`hepta-vnext-qualification-2026-08-23/delivery-consistency-qualification-20260824/` 记录：

`canonical plan/index SHA`、`mirror plan/index SHA`、byte equality、文件尺寸、生成时间、verifier 版本、
authority 状态和所有 production-negative flags。manifest 的 own SHA 只写入外部 `SHA256SUMS`，避免
自引用。任何 hash mismatch、长度 mismatch、镜像缺失或主索引指向非当前主计划，均为 fail-closed
`DELIVERY_MIRROR_MISMATCH`，不得进入 H8/S5。

### E.20.3 交付验收与当前边界

交付验收至少执行：canonical/mirror 四文件 SHA 与 byte-compare、index→plan digest 指向、E.19/E.20
存在性与 crosswalk 检查、历史 receipt 文件未被改写的路径清单检查，以及 production-negative flag 检查。
当前交付即使 `MIRROR_SYNCED`，也只说明文档与证据入口一致；能力声明仍为 L0 baseline + L1 shadow，
L2 closed-loop efficacy、L3 topology plasticity、真实 NPU、operator acceptance、CALLERS qualification
和 production promotion 继续 blocked/false。

本条状态：`PASS_DELIVERY_CONSISTENCY_SHADOW`（文档/qualification artifact only）；不改变源码、
canonical G4/G5、production writer/effect、模型/NPU、CALLERS、operator acceptance 或 promotion。

## E.21 Contract-hardening and safe-blocker closure (2026-08-24)

本条是 E.20 之后的 append-only 收口。它把“文档/fixture 已通过”和“可以进入真实 runtime”再次分开，
并把本轮能够在无生产权限、无真实模型、无源代码 worktree 的条件下安全完成的 blocker 固化为独立
qualification lanes。任何 lane 的 `PASS_*_SHADOW` 都不能升级 claim level、解冻 G5、修改 CALLERS 或
获得 memory/KG/effect authority。

### E.21.0 当前有效入口、冻结和机器可读 authority

由于本文首个 header 的 `v1.3` 是历史架构基线，机器读取者必须先读取外部有效版本索引：

`/home/qian-qi/Dropbox/OpenClaw/HEPTA_EFFECTIVE_VERSION_INDEX_V1_4.json`

该索引声明 `effective_version=1.4`、`supersedes=1.3`、E.20.0/E.21.0 precedence、H5→H5a/H5b、
H6→H6a/H6b、H7→H7a/H7b、H8→H8a/S5、H9→S5、L3→S6 的 crosswalk。历史 header 和旧章节保留为
lineage，不再作为与 E.19–E.21 并行的实施指令。

计划和索引继续采用 append-only：

- `freeze_for_receipt_binding_at` 只表示某次 receipt 绑定的快照，不禁止之后的 append-only 修订；
- 新修订必须更新外部 manifest 的 `canonical_plan_sha256`、`canonical_index_sha256` 和
  `current_index_digest_pointer`，并重跑受影响 lane；
- 任何 receipt 若绑定旧 digest，标记 `STALE_SOURCE_BINDING` 并 fail-closed；
- 当前机器入口只能是外部 current manifest，历史 bootstrap SHA 只属于
  `HISTORICAL_BOOTSTRAP_ONLY` namespace。

目录 authority 由以下机器文件声明：

`/home/qian-qi/Dropbox/OpenClaw/HEPTA_QUALIFICATION_AUTHORITY.json`

`hepta-vnext-qualification-20260823` 是当前 qualification package；同级的
`hepta-vnext-qualification-20260823`（无日期连字符）是 `NON_AUTHORITATIVE_LEGACY_PACKAGE`，
`hepta-vnext-qualification-20260824` 是 `EMPTY_STAGING`。glob auditor 不得把后两者当作当前证据。

### E.21.1 Canonical contract hardening lane

新增独立目录：

`hepta-vnext-qualification-2026-08-23/e21-contract-hardening-qualification-20260824/`

`PASS_E21_CANONICAL_CONTRACT_HARDENING_SHADOW` 覆盖：

1. EventEnvelope 六类 typed payload 的 discriminated union，不接受宽 optional payload；
2. EventEnvelope、Trajectory 和 NDU/OPE 字段统一，包含 `artifact_digest`、`support_set_digest`、
   `reward_vector`、`outcome`、`snapshot_digest`、`position_id`、`causal_parent`；
3. `RunStartSnapshot`、`LeaseFence`、command/CAS/fencing 字段和 snapshot↔graph↔artifact digest compatibility；
4. `NeuronSignal` 固定 `authority=advisory`、`execution_scope=none`、`provisional=true`；
5. `DecisionReceipt` 在 shadow lane 固定 `execute_allowed=false`，不能转授 effect authority；
6. `EffectReceipt` 的 execution scope、provider/reconciliation lineage、Indeterminate 强制对账和
   Reconciled 外部状态 digest；
7. hash-chain/prev digest/sequence/fence、graph endpoint/topological order/fan-out/depth/cycle/resource
   budget 的独立 semantic validator；
8. 22 个测试（含 18 个 negative mutation）和严格 Draft 2020-12 nested schema。

这条 lane 证明的是 contract semantics 和 fail-closed rejection；它不是 GraphExecutor runtime，也不是
真实 action/effect 证据。

### E.21.2 Model verifier and calibration-claim hardening lane

新增独立目录：

`hepta-vnext-qualification-2026-08-23/e21-model-verifier-qualification-20260824/`

`PASS_MODEL_VERIFIER_HARDENING_SHADOW` 对既有 model protocol fixture 做完整 Draft 2020-12 nested
validation，并加入 4 个 nested mutation。synthetic ECE `0.075` 高于协议门槛 `0.05`，因此机器 claim
固定为 `NEGATIVE_MATH_FIXTURE_NOT_CALIBRATION_PASS`；不得再把 arithmetic fixture 写成 calibration pass。
该 lane 不安装模型、不连接 Core ML/NPU、不附 corpus、不执行训练或长期 efficacy。

### E.21.3 S3a salience causal seam and bounded graph planning lane

新增独立目录：

`hepta-vnext-qualification-2026-08-23/e21-s3a-runtime-qualification-20260824/`

`PASS_E21_S3A_RUNTIME_SHADOW` 只验证以下 sandbox/read-only 数据形状：

```text
TypedInput/RedactionReceipt
  → NeuronSignal + ModelReceipt (deterministic baseline, provisional)
  → DecisionReceipt (advisory, execute_allowed=false)
  → ActivityReceipt (read-only/observe-only)
  → observation trajectory (causal parent + hash chain)
```

它同时验证 snapshot/graph/model/head/calibration digest compatibility、stale fence、bounded graph replay、
TopologyProposal 的 `proposal_only/research_only/unsigned/next_snapshot_only` 约束，以及 fault/replay
negative cases。它明确不写 Memory/CognitiveStore/KG/SQLite/outbox，不调用模型、网络、sidecar、provider、
tool 或 effect；因此它不能称为 L2 感知—行动闭环，只是 S3a sandbox causal seam。

### E.21.4 H8 gate、附件和 manifest 收口

根级 Resource、TaskFlow、H8/H9 文档已追加 E.21 crosswalk，并与 qualification mirror 做 byte-identical
同步。H8 gate 的输入现在必须列出 v1.4 contract/model/delivery lanes、E.21 canonical/model-verifier lanes
以及 S3a runtime seam lane，并携带：

`claim_level, evidence_class, runtime_authority, efficacy_status, approval_state, source_binding, receipt_sha256`

缺失、过期或 digest 不一致均 fail-closed。`APPROVED_RUNTIME` 只表示治理批准状态，不能由
`QUALIFIED_SHADOW` 自动转换。

当前外部 manifest：

`hepta-vnext-qualification-2026-08-23/e21-delivery-hardening-qualification-20260824/E21-CURRENT-MANIFEST.json`

负责记录 canonical/mirror digest、附件 pair、历史 allowlist、重复目录 authority、lane receipt 和生产负标志。

### E.21.5 当前仍真实 blocked 的门

本轮没有越过以下门，且它们不能由文档或 fixture 冒充完成：

- Agent-local authoritative SQLite writer/outbox 与 Codex hook 的真实接入；
- H5 Signal/ModelReceipt→H6 typed causal adapter→TaskFlow action seam 的生产实现；
- 真实小模型安装、Core ML/CPU/NPU conversion、operator coverage、M5 latency/RSS/energy benchmark；
- 脱敏跨周/月 corpus、locked calibration、真实 feedback/postcondition、support-aware OPE/CI、
  retention/forgetting/no-regression；
- H4 MemoryAdmission/CognitiveStore Pending/Applied/Rejected/Revoked 的真实 outbox/Saga；
- 独立 production operator acceptance、CALLERS qualification/promotion、G5 解冻和 rollback rehearsal；
- L3 TopologyProposal/GraphCompiler/ablation/lesion 只能在 L2 稳定后推进；运行中仍禁止改图、权限、代码、
  base model 或目标；
- 若未来要宣称“仿生”，必须另开 L4 neuromorphic track（时序动力学、局部 STDP/Hebbian 塑性、homeostasis/
  neuromodulation、sensorimotor timing、lesion/ablation）。

### E.21.6 Claim boundary

本条最终状态为：

`PASS_E21_CONTRACT_HARDENING_SHADOW / PASS_E21_MODEL_VERIFIER_SHADOW / PASS_E21_S3A_RUNTIME_SHADOW`

但整体能力声明仍严格为 `L0_BASELINE_L1_SHADOW_ONLY`。本条不改变 canonical G4/G5、CALLERS、production
writer/effect、模型/NPU、operator acceptance、promotion 或任何外部权限。

## E.22 Runtime-integration safe closure — SQLite trajectory/outbox and typed causal adapter (2026-08-24)

本条在 E.21 contract hardening 之后，只推进当前环境中可独立验证、可逆且无生产权限的实现 blocker。
它不改变 L0–L3 claim ladder，也不把 qualification code 注册到 Codex/App Server/Agent runtime。

### E.22.0 Isolated qualification lane

新增：

`hepta-vnext-qualification-2026-08-23/e22-runtime-integration-qualification-20260824/`

状态为 `PASS_E22_SQLITE_CAUSAL_ADAPTER_SHADOW`。该 lane 实际执行并验证：

1. file-backed SQLite trajectory/outbox，`journal_mode=WAL`、`synchronous=FULL`；
2. event + outbox + run revision 的单事务提交，注入故障时全部回滚；
3. `command_id` 去重、`expected_revision` CAS、authority/owner epoch、generation 和 fencing token 拒绝；
4. 关闭并重新打开数据库后重建 4-event causal/hash chain，outbox ack/pending 状态保持一致；
5. H5 `NeuronSignal + ModelReceipt` → H6 typed causal adapter；adapter 的 confidence/salience/novelty/goal
   relevance 必须从已验证 signal/receipt 推导，拒绝 caller-supplied synthetic features；
6. 下游 `DecisionReceipt` 保持 advisory、`execute_allowed=false`，`ActivityReceipt` 保持 read-only/
   observe-only；不生成 provider EffectReceipt；
7. 严格 Draft 2020-12 nested schema、fault replay 和 10 个单元测试。

这里的 SQLite writer 是 qualification 临时文件中的真实事务实现，但不是 Agent-local authoritative production
writer：当前环境没有挂载产品源码 worktree，也没有把它注册到 Codex、App Server、CALLERS、scheduler、
CognitiveStore、KG、provider 或 tool/effect seam。它只关闭“SQLite/outbox 与 causal adapter 是否可实现、是否能
fail closed”的本地可执行证据缺口。

### E.22.1 Delivery and H8 binding

H8 shadow prerequisites 现在必须同时绑定：E.21 canonical/model/named/delivery contract lanes、E.21 S3a
read-only causal seam 和本 E22 SQLite/adapter lane。所有 current wrapper 必须携带：

`claim_level, evidence_class, runtime_authority, efficacy_status, approval_state, source_binding, receipt_sha256`

旧 V3 delivery receipt 与历史 H5/H6/H7 receipts 仅作 immutable historical source；机器当前入口是 E.21
external manifest 中的 normalized wrapper。缺字段、source binding 过期、SHA 漂移或 production-negative
flag 非 false 均 fail closed。

### E.22.2 Remaining external/product blockers

完成 E.22 后仍真实 blocked，且不得由本地 fixture 冒充完成：

- 将 SQLite schema/migration/outbox 接入真实 Agent-local owner，并完成多进程锁、crash recovery、磁盘满/
  corruption、backup/restore 和产品回滚测试；
- 将 typed adapter 接入真实 H5 model sidecar、H6 Intuition 与 TaskFlow action seam；
- 真实小模型 artifact、tokenizer/quantization/operator/SBOM、Core ML/CPU/NPU conversion 和 M5
  latency/RSS/energy benchmark；
- H4 MemoryAdmission 的 Pending/Applied/Rejected/Revoked、target ack 及 CognitiveStore/KG outbox/Saga；
- 真实 EffectReceipt/Postcondition/Indeterminate reconciliation、用户纠正和 delayed/out-of-order feedback；
- 脱敏跨周/月 corpus、locked calibration、support-aware OPE/cluster CI、retention/forgetting/pollution/
  no-regression；
- independent production operator acceptance、CALLERS qualification/promotion、G5 解冻和 rollback rehearsal；
- L3 TopologyProposal compiler/dual-run/ablation/lesion/canary 仍须等待 L2 稳定；L4 neuromorphic claim 仍是
  独立未来 track。

### E.22.3 Claim boundary

整体能力声明继续严格为 `L0_BASELINE_L1_SHADOW_ONLY`。E.22 证明本地 SQLite 事务底座与 typed causal
adapter 可以实现并通过 fault replay；它不证明真实感知—行动闭环、长期学习 efficacy、自我进化、结构生长
或仿生机制。所有 production writer/effect/model/NPU/operator/CALLERS/G5/promotion flags 保持 false。

## E.23 Upstream Codex integration / local blocker closure — 2026-08-24

本条登记一条独立的上游同步与本地 blocker closure lane。它只绑定精确 source head、tree、上游提交和可重放
测试证据；不改变 canonical main-integration、当前真机候选或任何 production/H8/H9 权限。

### E.23.0 Exact source and build binding

`result = LOCAL_INTEGRATION_ONLY`

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head: `8462290cba24e3f0b7d9b5ee7a9118091b2ba5ff`
- tree: `3951803160eec7ca6b0b9fc2f6708c1615274d51`
- parent: `2e6697caf1e562e465c57e568dbdb969a51e1a3b`
- upstream main: `e3609f2d02a5896c391fa4c07335165c9272b686`
- qualified candidate ancestor: `1cff3bc8aeb58f1e2cc36a954fd9a9725168cb1e`
- worktree: clean; canonical `main-integration` and listener `7373`: unchanged
- local release binary: `hepta-upstream-integration-846229-20260824/codex-hepta-agentd`
- binary SHA-256: `e1c7d7b641aa374e631bbe7c0ef3eebb8e831b33e0603482b65e4333fd61d4ee`
- build profile: release with `CARGO_PROFILE_RELEASE_LTO=off`, `CARGO_BUILD_JOBS=4`; this is an integration
  build, not a production release profile

### E.23.1 What was actually closed

- latest upstream Codex main was merged conservatively through `e3609f2d02`; MCP selected-plugin projection still
  binds to the exact session source snapshot and focused projection tests pass;
- `ToolCallSource` compatibility, including Hepta's `DirectPlaintextMessage` governance mapping, was reconciled;
- durable queue recovery now requires matching turn/generation/fingerprint/history boundaries and fails closed on
  malformed or cross-turn tails;
- semantic read-only memory-review now checks actual assistant response/citation/source across agentd reopen;
- provider-effect tests enforce occurrence-key and payload-bound ACK/replay semantics (physical provider exactly-once
  is not claimed);
- SQLite deterministic 1,000-operation close/reopen/replay stress passes; this is not 1,000 real host restarts;
- local macOS authorized-read stable-handle seam restores `O_NOFOLLOW_ANY`/`O_UNIQUE`, nlink/dev/ino and F_GETPATH
  provenance, parent/path replacement and hardlink fail-closed checks, and bounded no-prefix reads.

### E.23.2 Verification

The exact-head receipt is `hepta-upstream-integration-846229-20260824/INTEGRATION-RECEIPT.json` with companion
`SHA256SUMS`. Verified counts include: app-server 308/308, MCP 20/20, memory 108 passed + 1 ignored, memory
extension 76/76, agentd memory-review 1/1, two-agent recovery 1/1, selected-plugin projection 1/1, selected-plugin
binding 1/1, strict queue unit 6/6, queue integration 35/35, provider-effect focused 7/7, exec-server 265/265,
stable authorized-read 11/11, Unix authorized-read 4/4, `cargo fmt --check` and `git diff --check` passed (only
existing nightly formatter warnings).

### E.23.3 Claim boundary and remaining blockers

All authority flags remain false: `production_ready`, `h8_allowed`, `h9_allowed`, physical provider exactly-once,
operator acceptance, promotion, fleet/effect/KG/model/NPU authority. This lane does not close:

- provider-owned durable occurrence-key exactly-once, status lookup/reconcile, and key/payload-bound effect
  provenance;
- independent trust signer, operator acceptance, CALLERS ratchet, promotion, and root-owned writer seal;
- real Agent-local authoritative writer/outbox + Codex hook, multi-process crash/restore, and H4 admission/Saga /
  forget-tombstone product path;
- H5-H7 real model/typed action seam, model/NPU/resource benchmark, trajectory/eval/reload loop;
- 1,000 real host kill/restart/replay cycles (only deterministic SQLite reopen stress is evidenced);
- the upstream-deleted authorized-read RPC/protocol/capability advertisement and remote hardlink provenance;
- H8 single-agent production canary and H9 fleet.

The Dropbox package mirror and authority/effective-index placeholders remain dataless/non-evidence. Historical
`MIRROR_SYNCED` and old digest lines are retained append-only as historical records; machines must use the exact
receipt above and treat the mirror as `MIRROR_STALE/non-evidence` until hydration is independently verified.

### E.23.4 Evidence digest note

The exact E23 receipt SHA-256 is
`5a7e2859e6ffc14adf313e9796ae06b29486fe8a90f91dd69b66912be3ab73f6`.
Its companion `SHA256SUMS` SHA-256 is
`4c5cd986668dab293a6c1c0a74365ee668b61e02f12a8f4143f3298bc30e65e6`.
These values are external artifact digests; the plan does not self-reference its own post-append digest.

## E.24 Upstream latest + local blocker hardening — 2026-08-24

本条是新的 append-only current local integration pointer。它先吸收远端 Codex `upstream/main`
最新可见 head `80cce09d05`，再在独立 integration lane 完成可安全本地闭合的两个 correctness slice：provider
evidence reopen 时拒绝 terminal ACK 之后的 late uncertainty，以及仅对显式
`qualification-cognitive-write` profile 启用 CognitiveRuntime available-only 启动硬门。两项都不打开默认/生产
writer、shared KG、routing、effect、fleet 或 promotion 权限。

### E.24.0 Exact source and artifact binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head: `de1c1c4d6eefa619d720f9c160eca461586f79d7`
- tree: `bb753af815c81a0d78f3ff110e7a39aae93c7a2d1`
- parent: `d6283990d6f811a647b0232f7d24a41818e6d6b1`
- upstream main: `80cce09d059780528e59353ab3d87e4c97d1e944`
- qualified candidate ancestor: `1cff3bc8aeb58f1e2cc36a954fd9a9725168cb1e`
- exact release artifact: `hepta-upstream-integration-de1c1-20260824/`
- release binary SHA-256: `65cc95a7eb6de6ab806b427110f6b84135a100cbd9637ab1783dcdf4c38e3dc5`
- receipt SHA-256: `e418be379aadd59a7e70c22160f01b7074f3308559d0523dd55c9d33b33bbcdd`
- `SHA256SUMS` SHA-256: `04323e0377b0b256739d81e8d194763ee0bc1384b3c0f97de1ad8f70dea25ce5`
- worktree clean; canonical `main-integration` and listener `7373` unchanged

### E.24.1 Verification

Exact-head gates pass: hepta-memory `108 passed + 1 ignored`, memory extension `76/76`, app-server `308/308`,
MCP `20/20`, evidence `59/59` (including late-uncertainty reopen regression), automation `15/15`, exec-server
protocol `21/21`, exec-server `265/265`, authorized-read RPC `1/1`, accepted websocket `5/5`, Guardian V2 `13/13`,
default read-only memory-review `1/1`, qualification writer/isolation E2E `1/1`, default runtime symmetry `1/1`,
qualification unavailable-store gate `1/1`, queue `6/6 + 35/35`, provider-effect focused `7/7`, stable handle `11/11`,
Unix authorized-read `4/4`, local lease/outbox `17/17`, lifecycle owner `5/5`, extension registration `1/1`, selected
plugin projection/binding `1/1` each, two-agent recovery `1/1`, and deterministic SQLite stress `1/1`. `cargo fmt`
and `git diff --check` pass; remaining output is existing warning/no-nightly-config noise.

### E.24.2 Claim boundary

The gated `fs/readFileAuthorized` RPC and `stableHandleAuthorizedRead` capability are local-only; remote filesystem/
skills follow-on and remote hardlink provenance remain separate. The provider change hardens local evidence replay,
not physical provider exactly-once. The qualification startup gate hardens an isolated writer profile, not the real
Agent-local authoritative writer/outbox or Codex hook. The receipt remains `LOCAL_INTEGRATION_ONLY` with
`production_ready=false`, `h8_allowed=false`, `h9_allowed=false`, and all effect/KG/model/NPU/operator/CALLERS/
promotion flags false. The Dropbox mirror and authority/effective-index placeholders remain
`MIRROR_STALE/non-evidence` until independently hydrated and verified.

E.24 does not close the provider owner, independent trust signer/operator acceptance, host-bound authority/owner epoch
turn lifecycle seam, H4 Saga/forget path, H5–H7 real model/action/resource/eval loop, 1,000 real host restart soak, H8,
or H9. These remain the next development blockers.

## E.25 Semantic tombstone propagation on the exact integration head — 2026-08-24

本条继续沿着上游同步后的 exact integration head 推进一个最小、可复核的产品语义切片：host-owned
`forget_memory` tombstone 必须穿过真实 Agentd/App Server/SQLite read-only memory-review 路径；当前 turn
和 Agentd kill/reopen 后的下一 turn 都不得再附加被撤回记忆。该切片不授予 live Agent writer/tool、provider
effect、shared KG、routing、fleet 或 production authority。

### E.25.0 Exact source and artifact binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head: `15bf080523b430c1c472b69557a6cc0a7e82d519`
- tree: `e81ad773a0c064e27e7c7372a671d7d1d67ae7ec`
- parent: `de1c1c4d6eefa619d720f9c160eca461586f79d7`
- upstream main: `80cce09d059780528e59353ab3d87e4c97d1e944`
- exact release artifact: `hepta-upstream-integration-15bf080-20260824/`
- release binary SHA-256: `65cc95a7eb6de6ab806b427110f6b84135a100cbd9637ab1783dcdf4c38e3dc5`
- receipt SHA-256: `5e4ff0f7155f1c9aa1fcc1e0f5418afce4ae15d9134955773757528b688e4635`
- `SHA256SUMS` SHA-256: `dc2b1c0b72bbeee8a115b95ef56617240432b9439487d8b10a119bc91474f3bd`
- worktree clean; canonical `main-integration` and listener `7373` unchanged

### E.25.1 Verification

The new default-profile real-process E2E passes `1/1` (`16.51s`): it verifies assistant response/citation/source and
revision before withdrawal, host-owned revision `1 → 2` with `Tombstoned` lifecycle, no `remember`/`correct`/`forget`
tools exposed, no cognitive attachment after the tombstone, and the same no-attachment result after Agentd
kill/reopen. The focused test was run on the exact head with `--no-default-features`; file formatting and diff checks
pass. The prior E24 package gates remain bound to the unchanged parent chain (memory `108 + 1 ignored`, extension
`76/76`, app-server `308/308`, MCP `20/20`, evidence `59/59`, automation `15/15`, exec-server `265/265`, and the
other E24 focused gates). The release rebuild completed with the same binary hash because this slice changes only
the E2E test surface.

### E.25.2 Claim boundary and next blockers

E25 closes only default read-only tombstone visibility and reopen replay. It does not claim host-bound authority/owner
epoch turn context, real Agent-local authoritative writer/outbox or Codex hook, resource-budget/duplicate-callback
soak, H4 Saga, H5–H7 runtime closure, provider physical exactly-once/status reconcile/provenance, independent trust
signer/operator acceptance/CALLERS/promotion/root seal, 1,000 real host restarts, remote authorized-read provenance,
H8, or H9. All production/effect/KG/model/NPU/fleet flags remain false.

## E.26 Active lineage pointer repair — 2026-08-24

本条只修复当前 source/evidence 的机器入口语义，不提升任何 runtime、production、H8 或 H9 权限。此前
E20/E21 的旧 SHA、`MIRROR_SYNCED`、Linux 路径和目录名继续作为 append-only 历史保留；机器读取者不得把
它们当作 active pointer。

### E.26.0 Current active pointer

- active local candidate: `15bf080523b430c1c472b69557a6cc0a7e82d519`
- active tree: `e81ad773a0c064e27e7c7372a671d7d1d67ae7ec`
- active artifact: `hepta-upstream-integration-15bf080-20260824/`
- active receipt SHA-256: `5e4ff0f7155f1c9aa1fcc1e0f5418afce4ae15d9134955773757528b688e4635`
- active `SHA256SUMS` SHA-256: `dc2b1c0b72bbeee8a115b95ef56617240432b9439487d8b10a119bc91474f3bd`
- active plan/index snapshot before this append: plan `d4f2f1157a0aa2cf56807b9500b12a09a75a6e6a62ad2372eb516dcb636040e3`, index `fed7d2f762c9a91850fe73ad53cfabe44d48562bec230c08309cc80a581ffa07`
- active mirror status: `MIRROR_STALE/non-evidence`; only the hydrated root plan/index and the exact artifact above are current evidence

The local Dropbox root is `/Users/qianqi/Library/CloudStorage/Dropbox/OpenClaw`. The names
`hepta-vnext-qualification-2026-08-23` (hyphenated) and its current artifact subdirectories are the intended
historical package names; the non-hyphenated `hepta-vnext-qualification-20260823` is not an active authority.
The machine-readable authority/effective-index/current-manifest files are currently dataless and therefore fail
closed rather than serving as evidence.

### E.26.1 Normalized receipt contract

Every new current wrapper must expose, at top level, `claim_level`, `evidence_class`, `runtime_authority`,
`efficacy_status`, `approval_state`, `source_binding`, and `receipt_sha256`. Nested receipt fields may remain for
compatibility, but a missing normalized field, stale source/tree, unreadable manifest, or any production-negative
flag mismatch is `STALE_SOURCE_BINDING` and cannot enter H8 or promotion. E25 remains a local integration/read-only
semantic slice; its parent-chain gates are explicitly `REUSED_PARENT_EVIDENCE`, not a fresh whole-tree pass.

## E.27 Final local integration qualification — 2026-08-25 (Asia/Shanghai)

本条将当前可复核的 local integration candidate 前移到 host-bound、Agent-private writer/outbox、H4、H5/H6、H7、
restart/reopen 和 authorized-read fail-closed 的组合 exact head。它仍然是 qualification/shadow 证据，不是
production authority；E26 及更早的 current pointer 只保留在 `HISTORICAL/SUPERSEDED` namespace。

### E.27.0 Exact source and artifact binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head: `fad3be113b48382102f5b375c894c77758860984`
- tree: `a4a76c65cb2ed747b7cb9af2eddf47a50c885490`
- parent: `95423c4e76ba5c8f19f229d106e214e42f5d0c98`
- upstream main (verified local): `80cce09d059780528e59353ab3d87e4c97d1e944`
- exact artifact: `hepta-upstream-integration-06ecdf-20260824/`
- release binary SHA-256: `50b8a9ce9e7bfac2cac28b1bd9dea293e76bcd341395f87568eb69bc6532a70c`
- `INTEGRATION-RECEIPT.json` file SHA-256: `47e70c1b10437c45ef1c7d8bfd51a00562a9a38ba97366cd76295f4fd7649264`
- receipt canonical digest (self-field null): `c537204499b1bd6501fc34457dc676f54cadadb88a1f948e2970caa8c7974bbc`
- `SHA256SUMS` SHA-256: `037a627fabb3ecd459bbc68cfea706c54b803d021907ff4b1c20f30441bea9b8`
- worktree clean; canonical `main-integration` and listener `7373` unchanged

### E.27.1 Normalized claim and standing test provider

- `claim_level`: `L0_BASELINE_L1_SHADOW_ONLY`
- `evidence_class`: `LOCAL_INTEGRATION_ONLY`
- `runtime_authority`: `false`
- `efficacy_status`: `QUALIFICATION_SEMANTIC_AND_RESTART_SOAK_ONLY`
- `approval_state`: `NOT_APPROVED`
- mirror: `MIRROR_STALE/non-evidence`; dataless authority/effective-index/current-manifest fail closed
- GPT-5.3-Codex-Spark is now the standing bounded test provider. This standing authorization covers development and
  qualification transport/semantic/replay tests only; it does not authorize production activation, tools, external
  effects, shared KG, routing, fleet, CALLERS, promotion, or credential embedding.

### E.27.2 Fresh and reused verification

Fresh focused evidence on this head:

- H4 admission/Saga/forget/tombstone qualification: `1/1`
- H5→H6 typed handoff and stale-generation fence: `1/1`
- H7 trajectory→replay-only evaluation→approved artifact→reload/rollback/tamper fence: `2/2`
- duplicate read-only replay binding: `1/1`
- real sequential child-process restart/reopen/replay soak: `1/1`, 1,000 operations, 647.59s; stages
  `clean=249`, `kill-after-intent=236`, `kill-after-commit=254`, `kill-after-rehydrate=261`; final audit covers
  3,000 events/hash-chain and 1,000 committed-state rehydration witnesses
- remote authorized-read fail-closed and path-identity regression: `14/14`; skills executor focused: `2/2`
- GPT-5.3-Codex-Spark real Agentd→App Server→ChatGPT backend read-only smoke: `1/1`, 28.40s; proves transport and
  turn completion only, not assistant quality, tool trace, memory efficacy, or provider exactly-once
- release build, `cargo fmt --all -- --check`, and `git diff --check`: passed (existing warnings only)

The final head memory package itself passes `116 passed, 2 ignored` (the two ignored tests are stress tests); the other
package counts remain explicitly reused rather than relabeled as a fresh whole-tree pass: extension `81/81`, app-server
`308/308`, MCP `20/20`, evidence `59/59`, automation `15/15`, exec protocol `21/21`, and exec-server full `265/265`.

### E.27.3 What remains blocked

E27 still does not close provider-owned physical exactly-once/status lookup/reconcile/effect provenance; independent
trust signer, operator acceptance, CALLERS ratchet, promotion, or root-owned writer seal; host-supplied production
authority/owner epoch and power-loss/supervisor semantics; authoritative Agent-local writer/outbox + Codex hook;
H4 production Saga; H5/H6 real model/Core ML/NPU and resource benchmarks; H7 production artifact governance; remote
filesystem/skills provenance beyond fail-closed transport; H8 single-agent production canary; or H9 fleet. All
production/effect/shared-KG/model/NPU/routing/fleet flags remain false. The 1,000-operation process soak is bounded
local qualification, not power-loss, supervisor, fleet, or production evidence.

## E.28 Agentd qualification writer host integration — 2026-08-25 (Asia/Shanghai)

本条把 E27 已存在但尚未接入真实 Agentd/App Server turn lifecycle 的 host-bound writer seam 接上；仍严格限于
`qualification-cognitive-write` 编译 profile 与 `local_development_only` SQLite。它不授予 production authority、
provider effect、KG writer、routing、fleet 或 promotion 权限。

### E.28.0 Exact source binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head: `120437d6585c3b90c0e6cbd3040e381c8a7b35fd`
- tree: `5f2377c43b978d80066315690395bd3e08518d8f`
- parent: `d85ebc5bb943549daecd0cf7f45738d3d4fbf2c7`
- commit: `feat(agentd): attach qualification turn writer host`
- E27 receipt head/tree are historical and stale for this section; no E27 digest is reused as fresh E28 evidence.

### E.28.1 Implemented bounded seam

- Agentd now constructs an explicit `QualificationTurnWriterHost` only in the opt-in
  `qualification-cognitive-write` build and passes it through App Server runtime options.
- Each turn is bound to an Agent-private lease, bound compact executor, and
  `LocalTurnLifecycleBinding`; admission remains local event+outbox bookkeeping with
  `external_effects=false`, `kg_write_authority=false`, and `production_caller=false`.
- Fleet lifecycle generation is checked before and after durable setup and the writer requires Running + App Server
  readiness; local lease generation is kept distinct (fresh per-turn lease starts at generation 1).
- `LocalLeaseAcquire::into_handle()` exposes only the already-owned handle and does not weaken current-head checks.
- Failure paths release a partially prepared lease and map generation/clock failures to a bounded input error.

### E.28.2 Fresh verification

- feature-gated Agentd unit suite: `21/21`
- host binding/admission/replay unit: `1/1`; duplicate occurrence remains `1 event / 1 outbox`, then local release
- default Agentd unit suite: `20/20`
- real Agentd → App Server → turn lifecycle cognitive product E2E
  (`real_agentd_remember_recall_correct_and_forget_revalidate_physical_sends`): `1/1`, `17.72s`
- exact changed-file rustfmt check and `git diff --check`: passed; repository-wide rustfmt still has unrelated
  pre-existing baseline diffs outside this slice and is not relabeled as an E28 pass.

### E.28.3 Explicit remaining blockers

E28 does not claim cross-process exactly-once replay: the current qualification host deliberately namespaces lease,
journal, and occurrence ids by process spawn generation, so a restart cannot be treated as a provider-grade replay
receipt. It also does not implement production supervisor-issued authority epochs, power-loss recovery, root-owned
writer seal, provider physical exactly-once/status reconcile/effect provenance, H4 production Saga, H5/H6 real model
or Core ML/NPU/resource benchmarks, H7 durable signed artifact governance, remote provenance, H8 canary, or H9 fleet.
All production/effect/shared-KG/model/NPU/routing/fleet/promotion flags remain false. GPT-5.3-Codex-Spark remains the
standing bounded test provider under the E27 authorization; no new per-run authorization is required for these tests.

## E.29 H7-LQ durable trajectory and provider terminal reconcile hardening — 2026-08-25 (Asia/Shanghai)

本条继续在 E28 exact integration lane 推进两个可本地闭合的 blocker：把真实 qualification turn 的
`turn_start → terminal` 生命周期写入 Agent-local immutable causal trajectory，并修复 durable provider
effect 在终态后收到迟到 lookup 时被错误降级为 `Indeterminate` 的问题。两项都不授予 production authority、
provider physical effect、shared KG、routing、fleet 或 promotion 权限。

### E.29.0 Exact source binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head: `cac8cee49d3f6e8dc196b5f05748600b7c28ef7e`
- tree: `ee977c9ece923eb9b1db006777abbfc434862daf`
- parent: `cad5614f092f27ee74c4d41d8044a4de2d605e0d`
- H7 commit: `5d9deba268` (`feat(hepta-memory): persist qualification trajectory lifecycle evidence`)
- provider terminal guard commit: `cad5614f09` (`fix(hepta-evidence): preserve terminal provider effect state on late lookup`)
- rejected-terminal regression commit: `cac8cee49d` (`test(hepta-evidence): cover rejected terminal lookup replay`)

### E.29.1 H7-LQ implemented boundary

- Added Agent-local `cognitive_h7_trajectory_events` immutable hash-chain storage with lease/fence, state/policy/
  model-receipt digest fields, causal parent sequence/digest, append-only triggers and schema oracle coverage.
- Migration `0009_h7_trajectory_observation_guard.sql` preserves SQLx migration checksums and rejects direct SQL rows
  that would masquerade as typed feedback/reward before that contract exists. The typed API and reopen verifier also
  reject `Feedback`, non-zero reward, policy propensity/support, and any external/KG/production authority flags in
  this observation-only slice.
- The opt-in `qualification-cognitive-write` turn writer now records start and terminal lifecycle observations against
  the exact host lease/compact binding, bounds terminal occurrence/reason values, and keeps the local receipt digest
  explicitly provenance-only (never a provider/effect receipt). Terminal append remains a recoverable two-step local
  lifecycle, not an atomic production Saga.
- Real Agentd → App Server qualification E2E now reads and verifies the persisted two-event chain after Agentd stop;
  the checked turn has contiguous causal parent data and all three authority/effect flags false.

### E.29.2 Provider durable terminal guard

- `HeptaEvidenceStore::reconcile_provider_effect_lookup` now returns a durable `Completed`/`Rejected` state unchanged
  for late `Unknown`/`NotFound`/`Conflict` lookups, including unsupported capability, rather than appending a late
  uncertainty and downgrading local state.
- An ACK lookup still validates exact key/payload binding; identical terminal ACKs replay idempotently, while a
  conflicting operation/status remains an `IdempotencyConflict`. Direct quarantine and reopen tamper guards remain
  strict.

### E.29.3 Fresh verification

- `codex-hepta-memory`: `124 passed, 2 ignored` (full library; ignored tests are explicit stress tests)
- H7 trajectory focused tests: `3/3`; schema/SQL guard test: `1/1`
- `codex-hepta-memory-extension`: `84/84`
- `codex-hepta-evidence`: `61/61`; provider-effect focused: `10/10`
- `codex-hepta-contracts`: `43/43`
- real feature-gated Agentd → App Server cognitive product E2E with H7 chain assertion: `1/1`, `18.56s`
- changed-file rustfmt and `git diff --check`: passed; unrelated repository baseline warning/no-nightly-config output is
  not relabeled as a whole-tree formatting pass.

### E.29.4 Explicit remaining blockers

This is H7-LQ observation evidence, not full E19.8 H7a: typed policy-action feedback, propensity/support, provider
receipt provenance, counterfactual/OPE/credit evidence, and H7b durable artifact evaluation/approval/CAS/reload/
rollback/signer/CI governance remain unimplemented. Cross-process exactly-once replay, supervisor-issued authority
epochs, power-loss semantics, and root-owned writer seal remain open. The provider change is only a local durable
state-machine correction; provider-owned physical exactly-once, status lookup contract, payload/key-bound external
effect provenance, and independent owner evidence remain false. H4 production Saga, H5/H6 real model/Core ML/NPU and
resource benchmarks, remote provenance, independent trust signer/operator/CALLERS/promotion, H8 canary, and H9 fleet
remain closed. All production/effect/shared-KG/model/NPU/routing/fleet/promotion flags remain false. GPT-5.3-Codex-
Spark remains the standing bounded test provider under the existing authorization; no per-run reauthorization is needed.

## E.30 Local qualification provider facade and H7 terminal-replay crash-window closure — 2026-08-25 (Asia/Shanghai)

本条把 E29 仍可在 Agent-local qualification lane 闭合的两组边界收口：durable provider effect 的 adapter-backed
dispatch/reconcile facade，以及 H7 terminal 已落盘但 local outcome/release 尚未完成时的 reopen recovery。它仍
不接入 Agentd/App Server runtime/provider routing，不授予 provider physical effect、production authority、共享
KG、fleet 或 promotion 权限。

### E.30.0 Exact source binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head: `865bc4925dc2cb9dee419093f8144cff05f0809d`
- tree: `53878e2b3e4d418ccd9895b9b5fbc0949b4266db`
- parent: `cac8cee49d3f6e8dc196b5f05748600b7c28ef7e`
- commit: `feat(hepta): close qualification provider and H7 replay boundaries`
- worktree clean after commit; E29 head/tree remain historical for this section.

### E.30.1 Provider qualification facade

- `HeptaEvidenceStore::dispatch_provider_effect_qualification` persists the intent, writes a durable
  `provider_dispatch_boundary_pending` quarantine before the adapter call, and lets only the durable claim winner
  invoke the adapter. A reopened or already-uncertain effect never blindly dispatches again; it must use lookup
  reconciliation.
- `reconcile_provider_effect_with_adapter` is lookup-only, short-circuits durable `Completed`/`Rejected` states, and
  quarantines malformed or conflicting lookup ACKs. ACK key/payload validation occurs before persistence, including
  the cross-key ACK case. The same-opened-store facade operations are serialized through a shared async boundary lock.
- The verifier no longer compares AUTOINCREMENT `seq` values across the independent ACK and uncertainty tables; it
  uses the per-key strictly increasing recorded timestamp as the cross-table ordering witness.
- Qualification metadata constants are descriptive registration conventions only (`local_qualification_only`, no
  external-effects/production-caller claim); an injected adapter is not thereby made harmless.

### E.30.2 H7 reopen/crash-window guard

- Added an atomic `read_h7_trajectory_bound` transaction: `BEGIN IMMEDIATE`, exact lease/fence verification, and
  trajectory read happen under one snapshot, removing the read/verify TOCTOU window.
- `QualificationTurnLifecycleContributor::admit` now detects a durable H7 terminal before attempting another
  `turn_start`; it closes the leftover local lease and preserves the existing stop projection semantics. A terminal
  `turn_indeterminate` also restores the local indeterminate outcome. No second H7 event or outbox admission is made.
- A trajectory's immutable event chain is now required to keep one complete binding tuple (lease/head/epochs/
  generation/token); a successor generation must use a new trajectory identity. Old-generation terminal heads and
  mixed-generation append attempts fail closed rather than closing a new lease.

### E.30.3 Fresh verification

- `codex-hepta-evidence`: `69/69`
- `codex-hepta-contracts` provider-effect tests: `15/15`
- `codex-hepta-memory-extension`: `85/85`
- `codex-hepta-memory`: `126 passed, 2 ignored` (the ignored tests are explicit stress tests)
- H7 trajectory/schema focused slice: `7/7` (including prior-generation head and mixed-generation-chain guards)
- `cargo clippy -p codex-hepta-evidence -p codex-hepta-contracts --lib --tests -- -D warnings`: passed
- changed-file rustfmt checks and `git diff --check`: passed; repository-wide rustfmt still reports unrelated baseline
  diffs/no-nightly import-order warnings and is not relabeled as a whole-tree formatting pass.

### E.30.4 Explicit remaining blockers

The facade lock is shared only by clones of one opened store. Public low-level append/mark/reconcile APIs and separate
opens/processes can bypass it; dispatch↔reconcile races outside that seam and provider-owned physical exactly-once,
status lookup, receipt/provenance, and power-loss durability remain open. A fresh imported Pending lookup whose
quarantine write itself fails is still a durability limitation, not a replay guarantee.

H7 remains observation-only: no typed policy feedback/reward/propensity/support, provider receipt provenance,
counterfactual/OPE/credit evidence, or H7b signed artifact/evaluation/approval/CAS/reload/rollback/CI governance is
claimed. Cross-spawn stable logical-turn replay/takeover is intentionally not inferred from spawn-scoped v1 IDs; it
needs a new stable logical-turn registry and atomic takeover CAS while preserving attempt-scoped lease/journal/H7
bindings. Do not relax current generation/fence checks or reuse old trajectories.

The SQLx migration checksum boundary remains explicit: historical 0008 edits must be frozen or followed by a deliberate
checksum/migration policy before claiming portable reopen of databases that already applied an earlier 0008. H4
production Saga, H5/H6 real model/Core ML/NPU/resource benchmarks, independent signer/operator/CALLERS/promotion,
H8 canary, H9 fleet, remote provenance, and all production/effect/shared-KG/model/NPU/routing/fleet/promotion flags
remain closed. GPT-5.3-Codex-Spark remains the standing bounded test provider under the existing authorization; no
per-run reauthorization is required.

## E.31 Provider ACK race guard and read-only local lease-head inspection — 2026-08-25 (Asia/Shanghai)

本条继续处理 E30 之后仍可在 Agent-local qualification lane 安全闭合的两个小 blocker：独立 store
reconcile 观察到 ACK 后，dispatch 不得再越过已完成的状态检查调用物理 adapter；以及为后续 stable
logical-turn registry/takeover 设计提供一个经过完整链校验的只读 lease-head witness。第二项刻意不做
mutation、不把过期观测升级为 takeover 权限，也不改变现有 generation/fence 语义。

### E.31.0 Exact source binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head: `c7a6df63b038fe4f3217fd6c2a4078aa65f1d2cd`
- tree: `851cbf5d7532aa6fa01c10d38936c13dc0d271da`
- parent: `25dc3c4b2d11dbc2b4514c455d7b5d4997a2dc71`
- provider race-guard commit: `25dc3c4b2d11dbc2b4514c455d7b5d4997a2dc71`
  (`fix(hepta-evidence): stop dispatch after observed provider ACK`)
- lease inspection commit: `c7a6df63b0` (`feat(hepta-memory): expose read-only lease head inspection`)
- worktree clean after commit; E30 head/tree remain historical for this section.

### E.31.1 Provider ACK race guard

- `dispatch_provider_effect_qualification` now treats any ACK observed after the durable claim (including one
  written by a separate-open reconciler) as a no-dispatch outcome. The guard is in addition to the existing
  `Completed`/`Rejected` and uncertainty checks, so a late ACK cannot be followed by a second physical adapter call.
- Exact key/payload validation, conflicting-ACK quarantine, and terminal short-circuit semantics remain unchanged;
  this is still a local qualification facade and does not establish provider-owned physical exactly-once or receipt
  provenance.

### E.31.2 Read-only lease-head witness

- Added public `CognitiveStore::inspect_local_lease_head`, `LocalLeaseHeadInspection`, and
  `LocalLeaseHeadDisposition::{Missing,Active,ExpiredActive,Released,RolledBack}`.
- The read transaction verifies the complete append-only lease hash chain, returns the exact current `LocalLease`
  head as a witness, and classifies bound expiry without appending rows. `ExpiredActive` is observation only; the
  API cannot release, roll back, dispatch, or take over a lease and callers must revalidate the witness in a future
  explicit CAS.
- Regression coverage proves missing/active/released/expired/rolled-back classification and unchanged event/outbox
  counts. Existing generation/token/head fencing remains intact.

### E.31.3 Fresh verification

- `codex-hepta-memory`: `127 passed, 2 ignored` (full library; ignored tests are explicit stress tests)
- read-only classifier focused test: `1/1`
- `codex-hepta-memory-extension`: `85/85`
- provider-effect focused tests after the race guard: `18/18`
- `cargo clippy -p codex-hepta-evidence --lib --tests -- -D warnings`: passed after the provider guard
- `git diff --check`: passed; direct changed-source rustfmt check passed. The repository's baseline test-module
  import-order diff and whole-memory `clippy -D warnings` failures (pre-existing dead-code/H7/API lint backlog)
  were not relabeled as E31 regressions and no unrelated lint cleanup was mixed into this commit.

### E.31.4 Explicit remaining blockers

The inspection API is a read-only foundation, not stable cross-spawn replay. Current qualification lease/journal/
occurrence/H7 identities remain spawn-generation and attempt scoped; `acquire_*_after_head` only creates a successor
after a terminal head and is not active lease takeover. Closing this blocker requires a new durable stable
logical-turn registry with canonical payload/hash, current attempt/head, and a single-transaction takeover CAS
restricted to an unadmitted expired attempt, plus dual-spawn one-winner, payload-mismatch, stale-writer, and rollback
tests. Do not stabilize IDs or relax mixed-generation H7/compact fences.

Provider physical exactly-once/status/receipt provenance, separate-process/power-loss semantics, imported-Pending
quarantine failure, H7a/H7b typed feedback/OPE and signed artifact governance, H4 production Saga, H5/H6 real
model/Core ML/NPU/resource benchmarks, independent signer/operator/CALLERS/promotion, H8 canary, H9 fleet, and
historical 0008 SQLx checksum policy remain open. GPT-5.3-Codex-Spark remains the standing bounded test provider
under the existing authorization; no per-run reauthorization, production/effect/shared-KG/routing/fleet/
promotion authority, or credential embedding is enabled.

追加审计边界：当前 H7 terminal recovery 仍经由 `verify_current_in_transaction` 的 active+TTL 检查；若进程
死亡晚于 lease expiry，即使 terminal observation 已 durable，现有 admit 也不能读取它来做收尾，只能由外部
显式 expire。另有同一 action 的 terminal retry 若 reason 改变，会因固定 event id 触发 CasConflict。两项都
应在后续只读/收尾 slice 中解决或保持明确的 fail-closed 限制，不能通过放宽现有 writer fence 规避。

## E.32 Expired qualification terminal recovery fence — 2026-08-25 (Asia/Shanghai)

本条闭合 E31 追加审计中两个可在 Agent-local qualification lane 安全收尾的 crash window：lease 到期后仍可
只读验证已落盘的完整 H7 terminal；以及 terminal event 已提交但 local outcome/release 尚未提交时，重试复用
durable reason 而不以固定 event id 触发冲突。所有动作仍是 observation/timeout-only，不授予 takeover、provider
physical effect、production authority 或跨 spawn replay。

### E.32.0 Exact source binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head: `47a0a95e7f5f7f7173718b914940b55d20bbc945`
- tree: `a6f2f759098ad6a5e111f44d98acbfca36b6f46d`
- parent: `c7a6df63b038fe4f3217fd6c2a4078aa65f1d2cd`
- commit: `fix(hepta): fence qualification terminal recovery`
- worktree clean after commit.

### E.32.1 Recovery implementation

- Added a read-only same-process recovery read that verifies exact lease identity, event/outbox chains, H7 binding,
  and observes expiry without opening a writable expired executor.
- Added restart-safe `inspect_expired_terminal_h7`: one `BEGIN IMMEDIATE` rechecks exact active head/expiry, local
  admission/outbox receipt binding, optional compact-journal binding, and an immutable exact two-event
  `turn_start -> terminal` trajectory. It returns only a witness; Agentd then reopens the exact head and uses the
  existing timeout CAS. Missing, nonterminal, malformed, foreign, or unbound evidence remains untouched.
- Added a pure H7 terminal-shape predicate (including bounded terminal occurrence key) reused by the extension writer;
  expired same-process terminal closure now requires that shape, while an expired attempt without terminal evidence
  remains fail-closed. Durable terminal retries reuse the first persisted outcome/reason.
- Agentd same-spawn prepare now inspects and reopens the persisted active head/expiry/token instead of recomputing a
  new TTL or lease row; an expired head is terminalized only after the restart-safe H7 gate. IDs remain spawn-scoped.

### E.32.2 Fresh verification

- `codex-hepta-memory`: `130 passed, 2 ignored`
- H7 trajectory/recovery focused tests: `8/8`
- `codex-hepta-memory-extension`: `86/86` (local turn writer `9/9`)
- feature-gated Agentd qualification tests: `4/4`
- `cargo check -p codex-hepta-memory --lib`: passed; only existing dead-code warnings
- `git diff --check`: passed; repository/test-module import-order rustfmt baseline remains separate.

### E.32.3 Explicit remaining blockers

This slice does not create a stable logical-turn registry or active-lease takeover. Qualification lease/journal/
occurrence/H7 identities remain spawn-generation and attempt scoped; `ExpiredActive` is still observation-only unless
the exact old attempt has the verified terminal shape above. The next bounded blocker is a new append-only stable
logical-turn identity/attempt registry and one-transaction takeover CAS restricted to an expired, unadmitted attempt;
do not stabilize existing IDs or relax mixed-generation fences. Provider physical exactly-once/status/receipt
provenance, H7a typed feedback/OPE and H7b signed artifact governance, H4/H5/H6, signer/operator/CALLERS/promotion,
H8/H9, historical 0008 checksum policy, and all production/effect/shared-KG/model/NPU/routing/fleet/promotion flags
remain closed. GPT-5.3-Codex-Spark remains the standing bounded test provider under the existing authorization; no
per-run reauthorization is needed.

## E.33 Stable Agent-local logical-turn registry and one-winner takeover — 2026-08-25 (Asia/Shanghai)

本条闭合 E32 之后的 durable identity/CAS 基础切片：在不稳定现有 spawn-scoped lease/journal/H7 ID、也不
放宽 mixed-generation fence 的前提下，新增 stable logical-turn identity 与 attempt transition stream。它只
服务 `local_qualification_only`；不注册 Agentd runtime、不调用 provider、不写 shared KG、不授予 production
ownership。TTL + zero-evidence takeover 是本地资格启发式，不是 OS/process death proof。

### E.33.0 Exact source binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head: `f2d91ca54bde082c16e86e7c60b840374a7f58ef`
- tree: `db669b82441feb81bab1f72e23b52e85412586b2`
- parent: `47a0a95e7f5f7f7173718b914940b55d20bbc945`
- commit: `feat(hepta): add stable logical turn registry`
- worktree clean after commit.

### E.33.1 Durable registry and CAS implementation

- Migration `0010_logical_turn_registry.sql` adds immutable `cognitive_logical_turns` identities and an append-only
  `cognitive_logical_turn_attempts` transition/hash chain. Identity hash binds owner, logical turn, scope, and logical
  payload digest; each attempt hash binds its fresh physical IDs, exact historical lease sequence/head digest,
  epochs/fence/expiry, and predecessor hash. Schema objects, exact 0001..0010 ledger, and SQLx schema oracle are
  checked on every `CognitiveStore::open`.
- `reserve_or_replay_logical_turn` runs under one `BEGIN IMMEDIATE`. Exact live replays return the durable attempt;
  a different live attempt returns `ExistingInFlight`; scope/payload rebinding returns `Conflict`. An expired attempt
  can be superseded only if the complete registry and exact current active lease head verify and no local
  event/outbox/compact/H7 evidence exists for any of its attempt-scoped IDs. Foreign-owner ID reuse also blocks.
- A successful takeover appends the exact old `superseded` witness, an append-only `rolled_back` lease terminal, and
  a fresh generation-one attempt/lease in the same transaction. Failure at any intermediate write rolls back every
  lease and registry row. Loader/reopen verification rejects gaps, active-to-active rows, malformed supersession,
  non-fresh successor IDs, tail superseded rows, missing/tampered historical leases, foreign/orphan rows, and digest
  mismatch. Old handles lose the lease-head fence immediately; old journal/H7 records are never rebound or continued.

### E.33.2 Fresh verification

- logical-turn registry focused tests: `11/11`, including two independent store handles with one winner, exact replay,
  live conflict, scope/payload mismatch, expired zero-evidence takeover, evidence block, old-handle fencing, clean
  reopen, tamper rejection, and injected mid-transaction rollback/retry.
- `codex-hepta-memory`: `141 passed, 2 ignored`
- `codex-hepta-memory-extension`: `86/86`
- feature-gated `codex-hepta-agentd`: `23/23`
- `cargo check -p codex-hepta-memory --tests`: passed; `git diff --check` and changed-source rustfmt check passed.
  Whole-memory `clippy -D warnings` still reports the established dead-code/H7/API lint backlog plus non-semantic
  registry style lints; no unrelated crate-wide lint rewrite was mixed into this commit.

### E.33.3 Explicit remaining blockers

The registry is not yet called by Agentd, so current runtime turn preparation still derives spawn-scoped IDs and does
not consume the stable winner across a new spawn. E34 must bind an explicit host-supplied logical-turn key/payload
digest into qualification prepare, translate registry `Acquired/Replayed/Takeover/ExistingInFlight/Blocked` outcomes
without reusing old journal/H7 IDs, and prove restart/two-spawn one-winner behavior. The local TTL+zero-evidence rule
does not prove owner death, cross-process power-loss exactly-once, or production supervisor authority.

Provider physical exactly-once/status/receipt provenance, imported-Pending quarantine failure, H7a typed feedback/
propensity/OPE and H7b signed artifact/evaluation/approval governance, H4 production Saga, H5/H6 real model/Core ML/
NPU/resource qualification, historical 0008 checksum portability, independent signer/operator/CALLERS/promotion,
H8 canary, H9 fleet, and all production/effect/shared-KG/model/NPU/routing/fleet/promotion flags remain closed.
GPT-5.3-Codex-Spark remains the standing bounded test provider; no per-run reauthorization is required, and no
credential or stale prior-head receipt is embedded or reused.

## E.34 Agentd logical-turn admission binding and attempt-scoped replay — 2026-08-25 (Asia/Shanghai)

本条把 E33 registry 接到真实 qualification turn 的本地 admission seam。它只在有明确的 user/client
admission identity 时启用；automatic、mailbox、recovery、缺少 client message id 或多输入 turn 保持 inert。
所有物理 lease/journal/trajectory/occurrence 仍是 attempt-scoped，旧 spawn 不续写旧 H7/compact 链；本条不
接 Agentd 生产 listener、不调用 provider、不写 shared KG、不改变 routing/fleet/promotion。

### E.34.0 Exact source binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head: `a4a0328b9d83cb60a64667d390ed02ba72489cdf`
- tree: `fc9aeecd3631a9923da0c7ff2429e3d6e1715bed`
- parent: `f2d91ca54bde082c16e86e7c60b840374a7f58ef`
- commit: `feat(hepta): bind qualification turns to logical registry`
- worktree clean after commit.

### E.34.1 Binding and fencing implementation

- Core derives a bounded `QualificationTurnAdmissionIdentity` only for exactly one non-empty user input with a
  client message id; payload SHA-256 is retained as a binding digest, never as an authority or raw prompt.
- Extension prepare receives a length-framed stable logical-turn id/scope/binding. The stable id excludes payload so
  a changed payload is a durable conflict; scope is hashed to remain within the registry's 512-byte cap.
- Agentd reserves first, then reopens the exact returned lease head and mints fresh physical attempt/lease/journal/
  trajectory/occurrence IDs. `Acquired`, exact `Replayed`, and zero-evidence `Takeover` are accepted; a different
  live attempt, payload conflict, or evidence block fails closed without mutating the old witness.
- Attempt digest framing includes agent, logical id, physical turn id, fleet generation, and spawn generation.
  Public host callback fields are revalidated at the callback boundary. Post-reservation materialization failures
  intentionally retain the committed active witness because schema 0010 has no abort projection.
- Equal owner epochs are allowed only for an expired, zero-evidence same-generation retry; lower epochs are rejected.
  Attempt-scoped lease, compact, H7, and lifecycle fencing remains unchanged. A 3600-second TTL is a local heuristic,
  not a process-death proof; a new spawn may wait for expiry rather than blindly re-dispatch.

### E.34.2 Fresh verification

- `codex-hepta-memory`: `143 passed, 0 failed, 2 ignored`
- `codex-hepta-memory-extension`: `88/88`
- feature-gated `codex-hepta-agentd`: `23/23`
- Core admission identity regression: `1/1`
- feature-on product E2E (`real_agentd_remember_recall_correct_and_forget_revalidate_physical_sends`): `1/1`
- feature-on cargo check for extension, memory, Core, and Agentd: passed; `git diff --check`: passed. Remaining
  compiler output is pre-existing dead-code warnings.

### E.34.3 Explicit remaining blockers

This slice proves local registry-first qualification replay, not provider physical exactly-once, provider status/receipt
provenance, imported-Pending quarantine, or supervisor/power-loss recovery. H7 remains observation-only: H7a typed
feedback/action receipts, propensity/support, OPE, and H7b signed durable artifact/eval/approval/CAS/reload/rollback/
signer/CI are not implemented. H4/H5/H6, signer/operator/CALLERS/promotion, H8/H9, historical 0008 checksum
portability, and every production/effect/shared-KG/model/NPU/routing/fleet flag remain closed. Recovery callbacks with
no durable client identity remain intentionally inert. GPT-5.3-Codex-Spark remains the standing bounded test provider;
no per-run authorization is needed, but this commit does not activate it in production or persist credentials.

## E.35 Imported-Pending provider-effect quarantine — 2026-08-25 (Asia/Shanghai)

本条补上 E30/E34 明确列出的 imported-Pending local blocker：一个 intent 已经存在、但当前 dispatch
facade 没有本地 `provider_dispatch_boundary_pending` claim 时，不能把 `Pending` 当成“可安全重发”。这只
强化 Agent-local evidence 的 fail-closed 行为；不把 uncertainty 当 provider receipt，不改变 provider
physical exactly-once，也不注册 Agentd/App Server/automation 或生产 caller。

### E.35.0 Exact source binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head: `7fe92cc3fe6baf1e0c47ff76cd50b26a00fd8b85`
- tree: `21a6609dfb6a0792885a03e2554be6682054f899`
- parent: `a4a0328b9d83cb60a64667d390ed02ba72489cdf`
- commit: `fix(hepta-evidence): quarantine imported pending effects`
- worktree clean after commit.

### E.35.1 Behavior

- `dispatch_provider_effect_qualification` now records the intent insertion disposition. If the intent was already
  durable while its state is still bare `Pending` (no ACK and no uncertainty), it appends the bounded reason
  `provider_imported_pending`, returns `Indeterminate`, and never invokes `adapter.dispatch`.
- A later `reconcile_provider_effect_with_adapter` may perform key/payload-bound lookup and append a valid ACK;
  exact terminal replay remains short-circuited and malformed/conflicting ACKs remain quarantined.
- The first-insert path retains the existing pre-dispatch claim and process-local serialization. Separate stores,
  processes, provider physical idempotency, status/receipt provenance, and power-loss semantics remain outside this
  local facade's authority.

### E.35.2 Fresh verification

- provider-effect focused tests: `19/19`
- full `codex-hepta-evidence` lib: `70/70`
- `cargo clippy -p codex-hepta-evidence --lib -- -D warnings`: passed
- `git diff --check`: passed; worktree clean.

### E.35.3 Explicit remaining blockers

Imported-Pending is now quarantined locally, but provider-owned physical exactly-once, durable provider operation
status/receipt provenance, separate-process races, and supervisor/power-loss recovery remain open. H7a typed
feedback/action/propensity/OPE and H7b signed artifact/eval/approval/CAS/reload/rollback/CI, H4/H5/H6, independent
signer/operator/CALLERS/promotion, H8/H9, checksum portability, and all production/effect/shared-KG/model/NPU/
routing/fleet flags remain closed. GPT-5.3-Codex-Spark remains the standing bounded test provider with no per-run
reauthorization; no production dispatch or credential persistence was enabled.

## E.36 Read-only logical-turn registry inspection — 2026-08-25 (Asia/Shanghai)

E34 的 registry-first prepare 已能完成 reserve/replay/takeover，但跨 spawn 的诊断/显式 recovery seam 仍需要
先构造一个新的物理 attempt 才能观察 durable head。本条新增纯读观测 API，不偷偷加入 abort、renew、release
或自动 Agentd recovery，也不改变 attempt-scoped lease/H7/compact fencing。

### E.36.0 Exact source binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- head: `9a3ba67a5a2383e743f8faf28292063aa94fb4fb`
- tree: `119ee73fe7759a054f6c7e964a1eb362ed4780de`
- parent: `7fe92cc3fe6baf1e0c47ff76cd50b26a00fd8b85`
- commit: `feat(hepta-memory): inspect logical turn registry read-only`
- worktree clean after commit.

### E.36.1 Inspection seam

- `CognitiveStore::inspect_logical_turn` uses a read transaction and never inserts or updates rows. It checks the
  immutable identity digest, full registry transition/hash chain, historical lease witnesses, current lease head,
  and conservative counts of event/outbox/compact/H7 rows bound to the current attempt.
- Typed dispositions are `Missing`, `Conflict`, `Active`, `ExpiredZeroEvidence`, `ExpiredWithEvidence`, and
  `TerminalPhysicalLease`. The result carries only scope/digest material and exact attempt/lease witnesses; raw
  prompt/provider payload is not returned.
- The snapshot is explicitly stale-able and diagnostic. `ExpiredZeroEvidence` remains only a witness; takeover still
  requires the existing locked CAS with a fresh physical tuple. No Agentd automatic caller, recovery authority,
  provider dispatch, shared KG, routing, fleet, or production flag was added.

### E.36.2 Fresh verification

- logical-turn registry focused tests: `17/17`
- `codex-hepta-memory`: `147 passed, 0 failed, 2 ignored`
- feature-on extension/Agentd cargo check: passed; `git diff --check`: passed; worktree clean.
- Tests cover missing read-only row stability, active/expired zero-evidence/expired evidence, scope/payload conflict,
  terminal lease, two-store inspection, and stale old head after takeover.

### E.36.3 Explicit remaining blockers

Inspection does not solve HeptaTurnRecovery's missing client-message/payload identity, post-reservation materialization
stranded heads, or provider-before/after lifecycle gating; those require a separately approved admission/recovery
protocol and must not be faked by a read API. H7 typed feedback/action/propensity/OPE and H7b signed artifact/eval/
approval/CAS/reload/rollback/CI, provider physical exactly-once/status/receipt provenance and power-loss semantics,
H4/H5/H6, signer/operator/CALLERS/promotion, H8/H9, checksum portability, and every production/effect/shared-KG/
model/NPU/routing/fleet flag remain closed. GPT-5.3-Codex-Spark remains the standing bounded test provider with no
per-run reauthorization; no credentials or production authority were enabled.

## E.37 — bounded turn-start cleanup and lease-head drift guard

- Committed `dee714733375d7d81c2b4b7dbfff163cdf3c97ba` (tree
  `91553b03ae43ec21ccd99fd2d6cbe44f801334d7`, parent
  `9a3ba67a5a2383e743f8faf28292063aa94fb4fb`) on the upstream-sync lane; worktree clean.
- Qualification `turn_start` failure cleanup is now bounded by the existing I/O timeout. If SQLite is locked or
  unavailable, the cleanup future is cancelled and the exact active lease witness is retained for explicit
  inspection/recovery; no fence, generation, or attempt-scoped H7/compact binding is weakened.
- `inspect_logical_turn` now verifies the exact active attempt lease sequence/digest witness. For terminal inspection,
  it requires the physical lease identity (generation, fencing token, authority/owner epochs, and expiry) to remain
  bound to the registry head. A later successor generation under the same lease id is reported as corruption rather
  than a false Active/Expired/Terminal state.
- Verification: extension lib `89 passed, 0 failed`; memory lib `147 passed, 0 failed, 2 ignored`; feature-on cargo
  check for extension + Agentd passed; `git diff --check` passed. The repository-wide stable `cargo fmt --all
  -- --check` still reports pre-existing formatting/config diffs and was not used to rewrite unrelated files.
- Remaining blockers are deliberately unchanged: cleanup-specific `terminal_requested`/start race and abort projection,
  durable recovery identity, provider physical exactly-once/status/receipt provenance and power-loss semantics, H7a
  typed feedback/action/propensity/OPE, H7b signed artifact/eval/approval/CAS/reload/rollback/CI, H4/H5/H6,
  signer/operator/CALLERS/promotion, H8/H9, migration checksum portability, and all production/effect/shared-KG/
  model/NPU/routing/fleet flags. GPT-5.3-Codex-Spark remains the standing bounded test provider with no per-run
  reauthorization; no production credential or authority was enabled.

## E.38 — fail-closed qualification provider boundary — 2026-08-25 (Asia/Shanghai)

### E.38.0 Exact source binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- commit: `b7c0b1bffbcb24b3bb5f74199207162eb9ca29ac`
- tree: `a2e996892ab522ea037a091978be7ffee5ad8bd1`
- parent: `dee714733375d7d81c2b4b7dbfff163cdf3c97ba`
- message: `fix(hepta): fail closed before qualification provider dispatch`
- worktree was clean after commit.

### E.38.1 Boundary behavior

- Extension API adds an in-memory, turn-scoped `TurnStartGate` schema v1 with
  `Pending`/`Allowed`/`Blocked` states and a `NewTurn`/`Recovery` origin. A gate is fail-closed until the
  qualification writer has durably admitted and appended its local H7 start; prepare, attach, validation, duplicate,
  lock, admission, or terminal races leave it `Blocked`.
- Core forwards the origin through the lifecycle seam, checks the gate at the common task spawn boundary for every
  task kind, and keeps RegularTask's pre-`TurnStarted` plus `run_turn` defense-in-depth checks. A blocked gate ends
  as `TurnAborted` before TurnStarted/prewarm/provider work. Recovery, automatic, direct, and no-client turns remain
  inert until a separately versioned recovery identity exists. An inert contributor does not remove a gate owned by a
  different contributor.
- Admission identity derivation now rejects mixed inter-agent/mailbox and client-user input, while preserving
  additional response-item context as same-turn context. The qualification writer accepts an explicitly attached
  host input for local tests; the Agentd host still requires durable admission identity and remains qualification-only.
- The slice does not add Agentd listeners, production callers, provider effects, shared KG/routing/fleet authority, or
  credential persistence. Gate schema v1 is independent from writer/topic/payload v1, user-payload digest v1,
  logical/binding/attempt domains v3, fence v2, and registry storage v1; no historical migration checksum was edited.

### E.38.2 Fresh verification

- `codex-extension-api --test state`: `6/6`
- `codex-hepta-memory-extension --lib`: `90/90`; focused writer module: `13/13`
- Core gate/lifecycle smoke: `2/2`; admission identity (including mixed mailbox rejection): `1/1`
- Goal extension backend compatibility: `22/22`
- feature-on Agentd library: `23/23`
- feature-on product E2E `real_agentd_remember_recall_correct_and_forget_revalidate_physical_sends`: `1/1`
- feature-on extension/Agentd cargo check: passed; `git diff --check`: passed. Existing dead-code warnings remain.
  Repository-wide stable/nightly rustfmt still reports unrelated baseline/import-order/config diffs and was not used
  to rewrite other lanes.

### E.38.3 Explicit remaining blockers

This closes only the local pre-provider fail-open seam. Recovery still lacks a durable client/payload identity and a
governed predecessor-to-fresh-attempt transition, so recovery remains intentionally inert. `start_task` still has a
reserved `task=None` window during lifecycle callbacks where concurrent abort can be lost; registry reservation
materialization failures still strand an active head because schema 0010 has no abort projection and its 3600-second
TTL is a wall-clock heuristic. Gate reason codes are bounded local diagnostics and currently surface as generic
`TurnAborted`; the gate is not a physical exactly-once claim or a cross-process seal. Provider physical
exactly-once/status/receipt provenance and power-loss/supervisor semantics, H7a typed feedback/action/propensity/OPE,
H7b signed artifact/eval/approval/CAS/reload/rollback/CI, H4/H5/H6, signer/operator/CALLERS/promotion, H8/H9,
migration checksum portability, and every production/effect/shared-KG/model/NPU/routing/fleet flag remain closed.
GPT-5.3-Codex-Spark remains the standing bounded test/qualification provider with no per-run reauthorization; no
production activation or credential embedding was performed.

## E.39 — fence aborts during the host-owned turn-start transition — 2026-08-25 (Asia/Shanghai)

### E.39.0 Exact source binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- commit: `0a9f4530afe80c5b49bed7fe25ec292d39d4cd08`
- tree: `85102dd35c2979f1a448e3c9ba2b2830219044ce`
- parent: `b7c0b1bffbcb24b3bb5f74199207162eb9ca29ac`
- message: `fix(core): fence turn-start transition aborts`
- worktree was clean after commit.

### E.39.1 Boundary behavior

- Core `ActiveTurn` now carries a host-owned `StartTransition` marker with a unique `Arc` identity and first-wins
  abort reason. `abort_turn_if_active`/`abort_all_tasks` record an accepted abort while `task=None`; the start owner
  performs lifecycle and terminal work only after the in-flight `on_turn_start` callback returns. Identity-fenced CAS
  prevents a stale continuation from attaching, clearing, publishing recovery, or waking a later turn.
- Pre-run abort reuses the existing `handle_task_abort` terminal path through a pre-signalled placeholder, preserving
  the interrupted marker, `TurnAborted` persistence/flush, abort hook, admission completion, and cleanup ordering;
  the real task's provider-facing `run` is never entered. `Replaced` remains first-wins and is not overwritten by a
  later `Interrupted`/`BudgetLimited` request.
- Recovery history rewind is deferred until the start marker is installed. A turn-scoped rollback witness restores the
  pre-rewind `ContextManager` after an accepted pre-attach abort, keeping in-memory history and durable cold
  reconstruction aligned when an orphan replay binding has no `TurnStarted` proof.
- This is Core-only local behavior. No Agentd listener, production caller, provider effect, shared KG/routing/fleet
  authority, deployment, release, or credential persistence was added.

### E.39.2 Fresh verification

- Core transition/first-wins/race tests: `3/3`; all `session::tests::abort_` tests: `6/6`
- blocked turn-start gate: `1/1`; recovery rollback witness: `1/1`
- the deterministic start-race test repeated `50/50` without failure
- `cargo check -p codex-core --lib`: passed; feature-on extension + Agentd cargo check: passed
- feature-on Agentd library: `23/23`
- feature-on product E2E `real_agentd_remember_recall_correct_and_forget_revalidate_physical_sends`: `1/1`
- `git diff --check`: passed. Repository-wide rustfmt still reports pre-existing baseline/config diffs; no unrelated
  formatting rewrite was made.

### E.39.3 Explicit remaining blockers

The caller-owned reservation/preamble before `start_task` installs its marker still has a `task=None`/marker-none
window; aborts in that window can remain a no-op and require a separate ownership slice. A dropped/panicking start
future or permanently hung lifecycle contributor can retain the marker (fail-closed but stranded), replacement while
Starting keeps the existing busy contract, and guardian `abort_turn_if_active` can perform an immediate idle check
before the owner finishes (no deferred idle emission yet). The rollback witness intentionally replaces the whole
`ContextManager`; any future side-channel history writer during the marker phase must be fenced before this is broadened.
If an abort is recorded while `on_turn_start` is already executing, the remaining lifecycle contributors still finish
their callback sequence; the common gate prevents task/provider `run`, but this slice is not a cancellation protocol for
arbitrary contributor side effects and does not claim physical dispatch suppression inside a contributor.
Provider physical exactly-once/status/receipt provenance and power-loss/supervisor semantics, H7a typed
feedback/action/propensity/OPE, H7b signed artifact/eval/approval/CAS/reload/rollback/CI, H4/H5/H6,
signer/operator/CALLERS/promotion, H8/H9, migration checksum portability, and all production/effect/shared-KG/
model/NPU/routing/fleet flags remain closed. GPT-5.3-Codex-Spark remains the standing bounded test/qualification
provider with no per-run reauthorization; no production activation or credential embedding was performed.

## E.40 — fence caller-owned start reservations and hand off to host start — 2026-08-25 (Asia/Shanghai)

### E.40.0 Exact source binding

- worktree: `/Volumes/T5/hepta-vnext/worktrees/upstream-sync-20260824`
- branch: `hepta/integration-upstream-20260824`
- commit: `e669ceaf6a85f587a4618d2c8b4af16a5cd1d58d`
- tree: `b29c1c0d80cf74e9718344022cd39f8f17c1b6f2`
- parent: `0a9f4530afe80c5b49bed7fe25ec292d39d4cd08`
- message: `fix(core): fence caller-owned start reservations`
- worktree is clean after commit.

### E.40.1 Boundary behavior

- Core now keeps three phases separate: ordinary idle/history reservation, caller-owned
  `StartReservation`, and host-owned `StartTransition`. A reservation carries an opaque `Arc` identity,
  turn id, and exact turn-state identity; only that handle can promote or release it. Promotion is a
  single active-turn-lock CAS and transfers the first abort reason without allowing a stale owner to
  touch a later attempt.
- `spawn_task`, `start_or_steer`, `start_if_idle`, and pending-mail wakeup reserve before their caller-owned
  preamble and pass the same handle into host start. Plain legacy start cannot steal a plain history
  reservation. Owned starts return `Attached`/`Aborted`/`Stale`; input admission completes as not-submitted
  on a non-attached outcome, so a fenced start cannot report `Started`.
- Abort recognizes both caller reservations and host transitions. A pre-context abort is returned to the
  owner for explicit cancellation cleanup; a post-context abort retains E39's deferred terminalization,
  identity fence, first-wins reason, and `TurnAborted` ordering. Attempt-scoped lease, H7 trajectory, and
  compact fencing remain unchanged.
- This is Core-only local behavior. No Agentd listener, production/canonical runtime registration, provider
  effect, shared KG/routing/fleet authority, deployment, release, push, or external send was added.

### E.40.2 Fresh verification

- reservation state tests: `2/2`
- deterministic spawn-preamble abort test: `1/1`; E39 transition race: `1/1`; blocked gate: `1/1`; recovery
  rollback witness: `1/1`
- Guardian strict auto-review regression (dedicated 8 MiB test stack): `1/1`
- large-stack Core library run: `2394 passed / 5 failed`; each failure was reproduced unchanged on the E39
  baseline (`config_schema_matches_fixture` fixture drift, `turn_aborted_flushes_terminal_event_after_delivery`
  flush-count timing, and `abort_turn_if_active_publishes_unchanged_in_flight_ready` Notify lost-wakeup;
  the other two E40 run failures passed when isolated). These are not attributed to E40.
- `cargo check -p codex-core --lib`, `cargo check -p codex-hepta-agentd --lib --features qualification-cognitive-write`,
  and `cargo check -p codex-app-server --lib`: passed with existing dead-code warnings. Changed-file diff check
  passed; repository-wide rustfmt still reports the known `tasks/mod_tests.rs`/baseline formatting drift.

### E.40.3 Explicit remaining blockers

The pre-reservation checks/awaits in some callers still precede ownership capture; a dropped, panicking, or
permanently hung start future can retain a reservation/transition (fail-closed but stranded). Recovery
candidate consumption still holds the active lock across its durable mutation, and guardian idle emission after
deferred start completion remains a later slice. The handoff is at-least-once local lifecycle fencing, not
provider physical exactly-once, power-loss/supervisor proof, or a cancellation protocol for arbitrary lifecycle
contributors. Provider receipt provenance/status reconcile, H7a typed feedback/action/propensity/OPE, H7b
signed artifact/eval/approval/CAS/reload/rollback/CI, H4/H5/H6, governance/signer/operator/CALLERS/promotion,
H8/H9, migration checksum portability, and all production/effect/shared-KG/model/NPU/routing/fleet flags remain
closed. GPT-5.3-Codex-Spark remains the standing bounded test/qualification provider with no per-run
reauthorization; no production activation or credential embedding was performed.

## HNL-PLAN-2026-08-26 — hepta-net / hepta-node-link 常驻 Rust 网络层（plan-only append）

本节是对 Hepta/Pocket4 多节点与 Multi-Agent 方向的独立开发计划追加。它不改写 E.1–E.40
历史记录，也不把 TRNM 链、市场撮合或结算逻辑塞进网络热路径。

### HNL.0 追加绑定与状态

本次追加前 canonical 主计划快照：

- canonical plan：`/home/qian-qi/Dropbox/OpenClaw/hepta-vnext-development-plan-final-2026-08-23.md`
- parent plan SHA-256（追加前）：`07dbbd0fb36a9d82d039275a9819e6c18b133248a2864470f7ffa17ef6cda718`
- append anchor：`E.40.3` 结束处，EOF line `3346`
- 主计划中记录的 canonical main-integration head：`7ed9c9a85fa65aa3cb26cf440a55028ce0b35079`
- `/Volumes/T5` 在本次写入环境未挂载，故 head/tree/dirty 必须在实现前重新验证
- external effective-index/manifest 仍指向旧 mirror digest；本节状态：`STALE_SOURCE_BINDING`
- 运行模式：`qualification_only` / `PLANNING_ONLY`
- implementation branch/worktree：`not-created`
- `production_listener=false`、`production_writer=false`、`provider_effect=false`、`shared_kg_write=false`、
  `routing_write=false`、`fleet_write=false`、`model_npu=false`、`CALLERS_touched=false`、
  `operator_acceptance=false`、`promotion=false`

因此，本节和三个附件只能授权 RFC、schema、golden vector、deterministic loopback 与 shadow design；
不能授权实现合并、生产部署、TUN/default-route、跨租户网络或任何 TRNM 真实链 SDK 接入。外部
effective-index/manifest 完成重新绑定并生成新 receipt 之前，任何机器读取者都必须 fail-closed。

### HNL.1 设计决策

Hepta 需要的是与主系统同版本演进的常驻 Rust 子系统，而不是把 Tailscale daemon 原样嵌入：

```text
TRNM（未来外部市场/结算）
        │ signed Lease / Capability / Revoke / Receipt（异步）
hepta-market-adapter（未来、链无关）
        │
hepta-net-core → hepta-node-linkd（每台受管物理安装一个 sidecar）
  ├─ Host/Agent/Service identity、密钥轮换、epoch 撤销
  ├─ PeerRegistry / signed discovery / trust registry
  ├─ AgentRouter / Service ACL / quota / backpressure
  ├─ SessionBroker / multiplex / transcript binding
  ├─ PathManager / direct → NAT → replaceable relay → migration
  └─ immutable NetworkEvent/UsageEvent outbox（不作结算）
        │ backend trait
  native QUIC/iroh（原生服务通道）
  tailscale-rs（Tailscale/Headscale compatibility）
  boringtun/WireGuard、EasyTier（后置 L3/对照）
```

硬边界：一台物理设备只有一个 Host `NodeId`；本地每个 Agent 都使用带
`TenantId/WorkspaceId/AgentId/ServiceId/agent_generation` 的受限委托身份；不能让每个 Agent
各自启动 VPN 节点。现有 `agentd`/fleet supervisor 保持生命周期与调度唯一权威，node-linkd
不得新造 session kernel、fleet bus、共享 KG writer 或隐式 model/tool executor。

### HNL.2 身份、握手与协议合同

- Host 根：Ed25519；`NodeId=hash(public_key)`。Host transport key、Agent delegation key、Service key
  和 ephemeral session key 分层，密钥轮换由单调 `epoch` 与 continuity certificate 约束。
- 远端握手必须同时绑定 `local_node_id`、`remote_node_id`、`tenant_id`、`workspace_id`、
  `agent_id`、`service_id`、`agent_generation`、`capability_hash/intent_hash`、`expiry`、`epoch`、
  `nonce` 与 endpoint/path challenge；只验证 Tailscale/WireGuard 主机公钥不算通过。
- HNL-0 冻结 canonical CBOR/COSE（或经安全审查批准的等价 profile）、domain separation、固定 ALPN、
  transcript/channel-exporter binding、clock-skew、TTL/replay window、feature/downgrade refusal、
  rekey 和 canonical error codes。安全字段必须进入 transcript，不能只放在首个应用包里。
- relay 只转发端到端密文，不能签发 capability、改写 route/计量或提升 Agent scope；协调器离线时已有
  会话受 `max_offline_grace` 与 `max_session_lifetime` 双上限，禁止静默无限续租。
- 本地 UDS 必须校验 Unix credential / Windows SID / macOS audit token，固定 socket ACL、防 symlink、
  request-id 幂等、cursor 去重和 per-Agent quota/backpressure；Agent 不能自行指定 backend、relay、
  AllowedIPs 或主机 route。

### HNL.3 稳定 facade（业务只依赖 Hepta 类型）

```text
start(NodeConfig) -> NodeHandle
register_agent(AgentRegistration) -> AgentLease
advertise(agent_id, ServiceAdvertisement) -> ServiceId
discover_peer(query) -> Vec<PeerDescriptor>
connect(ConnectIntent) -> SessionHandle
open_stream(session, agent_id, service_id, StreamPolicy) -> Stream
revoke(subject, epoch)
rotate_key(subject)
stats() -> NetStats
events() -> EventStream
```

`Session` 不暴露 `tailscale::Device`、EasyTier 内部结构体、底层私钥或上游 socket。网络层只产生
不可变 `NetworkEvent`/受限 `UsageEvent`，不写 TRNM 账本；未来 adapter 才把 TRNM finality 转成
短时签名 `Lease/Capability/Revoke`。链上交易号只能是 `settlement_ref`，不能成为握手密钥或路由条件。

### HNL.4 分期与依赖

| 阶段 | 内容 | 进入/退出条件 | 允许的状态 |
|---|---|---|---|
| HNL-0 | facade、wire schema、密钥层级、威胁模型、trust registry、golden vectors、资源预算冻结 | parent binding 可验证；协议/安全 review 通过 | RFC/schema/shadow |
| HNL-1 | `hepta-net-core` + `hepta-node-linkd` + UDS；deterministic loopback vertical slice | 两 node、每 node 两 Agent；ACL/撤销/crash-restart 可重放 | qualification-only |
| HNL-2A | native QUIC/iroh service backend、relay fallback、path migration | HNL-1 receipt；端到端加密与 transcript binding receipt | controlled fixture |
| HNL-2B | 锁定 commit 的 `tailscale-rs` Headscale/Go Tailscale 互操作 | 独立 compatibility receipt；WIP 缺口不阻塞 HNL-1/2A | compatibility known-gap |
| HNL-3 | cross-backend Multi-Agent multiplex、租户隔离、单 Agent revoke、key rotation | 对抗/故障/重放矩阵和 signed receipt 完成 | qualification-only |
| HNL-4 | boringtun/WireGuard、EasyTier 隔离 L3 对照 | 独立 privileged helper/netns、MTU/功耗/路由冲突报告 | future P1 |
| HNL-5 | TRNM signed-event simulator + `hepta-market-adapter` | 网络 qualification 后；只验证 Lease/Revoke/Receipt 适配 | future P1 |
| HNL-6 | 多 relay/operator、联邦 discovery、离线加入、阈值治理 | 多区域演练与治理 review | future P2 |

HNL-2A 与 HNL-2B 可并行，但两者各自出 receipt；`tailscale-rs` 的 WIP 互操作失败只能记
`KNOWN_GAP`，不能阻塞 backend-neutral core/native service。HNL-4 的 L3/TUN/default route、DHT、
EasyTier 深度集成、真实 TRNM SDK 和多国治理不属于 P0 service-stream DoD。

### HNL.5 统一 receipt 与验收门槛

每一阶段必须绑定 repo/head/tree/dirty、parent plan digest、Cargo.lock/backend commit、配置与 feature
flags、命令/超时、测试计数、耗时、artifact SHA-256、RSS/CPU/磁盘/功耗读数和 negative-authority
flags，使用附件 `hepta.hnl_receipt.v1.schema.json`。阶段矩阵在 `HNL_STAGE_MATRIX_v1.yaml`；对应
crate/fixture 未创建前只能填 `NOT_TESTED`，不能伪造 PASS。

最低硬门槛：cross-Agent/tenant 明文或权限泄漏为 0；relay 抓包应用明文为 0；旧 epoch/重复 nonce/
旧 lease 接受率为 0；撤销收敛不超过 TTL/clock-skew 上限；direct/relay 建链与重连 p95、idle/peak
RSS、CPU、FD/stream/peer 上限和 Pocket4 ARM/macOS/Linux 功耗必须以实机 receipt 冻结。确定性测试
不得访问公网；互操作、relay 和性能测试单独计分。

### HNL.6 Dropbox 附件与执行规则

本次追加的详细附件（与本节同一变更集）：

- `OpenClaw/hepta-net-node-link-plan-2026-08-26.md`
- `OpenClaw/HNL_STAGE_MATRIX_v1.yaml`
- `OpenClaw/hepta.hnl_receipt.v1.schema.json`

附件中的 parent digest 与本节一致；附件状态也明确为 `STALE_SOURCE_BINDING`/
`bound_planning_only_stale_external_pointer`。在 canonical index/effective-index/manifest 重新绑定并
生成 fresh receipt 前，Mac 主 agent 只能把 HNL 视为独立 RFC/qualification lane；不得把它塞进当前
`codex-core` execution-substrate blocker 分支，不得部署或 promotion。

一句话定案：`tailscale-rs` 负责兼容现有网络，native QUIC 负责 Hepta 原生服务连接，WireGuard/EasyTier
只在后置 L3 lane 评估；真正的常驻核心是 Hepta 自己的 `hepta-net/hepta-node-linkd`，TRNM 未来只
通过签名租约适配接入，agentd/fleet supervisor 仍拥有唯一的多智能体生命周期与调度权。

## INF-PLAN-2026-08-26 — hepta-infer-core / hepta-inferd 本地推理执行层（plan-only append）

> 状态：`PLANNING_ONLY / STALE_SOURCE_BINDING`。本节是独立的本地推理执行 lane，不是当前
> `codex-core` blocker 的实现授权；不修改既有 Neuron 语义合同，不解冻生产模型、NPU、远程推理或 promotion。

### 决策与边界

现有 `HEPTA_NEURON_RUNTIME_CONTRACT_V1_4` 继续负责 NeuronSpec、隐私、校准、`ModelReceipt`、
`NeuronSignal` 和证据语义。本 lane 只增加 Rust 常驻执行整体：每台物理设备一个由
`agentd`/fleet supervisor 管理的 `hepta-inferd`，以及供现有 agent 使用的
`hepta-infer-core` facade。Rust 负责 ModelRegistry、硬件能力探测、ModelRouter、权限与多 Agent
隔离、配额、调度、缓存、取消和执行 receipt；高速 kernel 复用锁定版本的 native runtime，不以纯
Rust 重写 kernel，也不引入 Ollama 或 HTTP/JSON 热路径。

```text
Hepta agents/agentd
        │ versioned UDS + canonical CBOR
hepta-infer-core (Rust facade / policy / router / receipt)
        │ one sidecar per physical host
hepta-inferd (Rust scheduler / quota broker / model pool)
        ├─ llama.cpp       LLM/VLM default
        ├─ whisper.cpp     ASR default
        ├─ sherpa-onnx     TTS/VAD/audio extension
        ├─ stable-diffusion.cpp  controlled image P1
        └─ Core ML/OpenVINO/MNN/vendor NPU workers (profile-gated)
```

P0 先收敛到文本/LLM/VLM 和 ASR；音频扩展、生图列入后续受控阶段，视频生成、远程/联邦推理和
多运营方模型市场另开 P2 lane。`mistral.rs`、Candle 等仅作逐模型/逐设备实测对照，不能因语言
是 Rust 就进入默认路由。

### 速度准入合同

任何组合只有在独立 direct-native 对照 receipt 通过后才能进入 `RunStartSnapshot` 和默认
ModelRouter。准入键必须完整匹配：

`model digest + tokenizer digest + quantization + backend/commit + ABI + compiler/build flags +
driver + device profile + thermal/power mode + context + batch/threads + KV/prefix-cache policy +
compiled artifact/SBOM/license digest`。

未知、过期、未签名或未实测 tuple 统一返回 `NotAdmitted`，禁止静默切 CPU、远程、另一量化或
未测试 backend。INF-0 前的 provisional review gate 是 warm sidecar p95 额外开销不超过
`max(10 ms, 10%)`、warm throughput 至少为 direct-native 的 90%；ASR RTF、图像延迟和峰值资源
按设备单独比较，实机 receipt 后才冻结正式门槛。Pocket4 的 SoC、RAM、GPU/NPU、OS 和目标 workload
尚未给定，在 profile 到位前只能标 `KNOWN_GAP`。

### 热路径、隔离与证据

- 控制面使用版本化 UDS + canonical CBOR；文本、音频、图像和视频使用 shared-memory/memfd，
  descriptor 必须含 `fd/offset/length/digest/owner/lifetime`，并执行 seal、bounds、digest 和 TOCTOU 校验。
- native worker 初期独立进程；崩溃、OOM、thermal trip、取消或 generation fence 必须释放 reservation、
  使旧 cache/receipt 失效并发出终态事件。worker 不得拥有 Hepta authority 或自行选择 endpoint、route、
  模型路径。
- 每个请求绑定 `TenantId/WorkspaceId/AgentId/agent_generation/task_id/policy_digest/resource_budget`
  和 model tuple digest。KV/prefix cache 默认不得跨租户或跨 workspace 复用，除非上述身份和 digest
  全部匹配。
- INF-1 先验收 deterministic loopback、UDS 身份/ACL、配额、取消、重启/generation fence 和 shadow
  typed-output/authority fence；执行 receipt 通过 `model_receipt_ref` 关联既有语义 `ModelReceipt`，
  不替换其权威定义。完整 `RunStartSnapshot → ModelReceipt → NeuronSignal → TaskFlow` 接入后置到 INF-6。
- receipt 只记录 digest、shape、大小、延迟、吞吐、RSS/VRAM、CPU/GPU/NPU、功耗、温度、缓存命中、
  取消/重启和错误码，不记录原始 prompt、音频或图像；所有 production/effect/shared-KG/route/fleet/
  model-NPU/remote/CALLERS/promotion 权限固定为 `false`。

### 分期与依赖

`INF-0` 合同/威胁模型/PerformanceContract/ABI/硬件与模型基线 → `INF-1` loopback+UDS →
`INF-2A` llama.cpp 与 `INF-2B` Core ML tiny/whisper/sherpa 并行 → `INF-3` warm pool、EDF/WFQ、
continuous batching、KV/prefix cache、背压和 OOM/thermal breaker → `INF-4` Mac/RTX/j3160/Pocket4
实机 qualification → `INF-5` stable-diffusion.cpp 图像与受控音频 → `INF-6` 接入现有语义合同。
`INF-2B` 在不适用的设备上为 `KNOWN_GAP` 时不得阻塞 backend-neutral core；没有对应 receipt 的
backend 不得进入默认 router。

### 本次源绑定、附件与执行限制

本次 append 前 canonical 主计划 SHA-256 为
`ba5008aa46b2f4e2ee3325deee197aa99d6997039bd5659db67f400c0c72c417`，锚点为
`HNL.6 / EOF line 3479 (pre-append snapshot)`。只读复核得到 canonical main HEAD/tree
`7ed9c9a85fa65aa3cb26cf440a55028ce0b35079` / `7d4306273861564a62fa9614860bdc6239a065d0`，
主树 dirty；实验树为 `hepta/integration-upstream-20260824`，HEAD/tree
`54c733764be7c24ef928cfc689e09515273ff900` / `ff902e217d857c3040516557305d0b93382afeae`，
同样 dirty。实验树的未提交 lifecycle 改动不属于 inference lane，推理实现不得从该树派生，也不得
塞入当前 `codex-core` blocker 分支。

详细执行矩阵、receipt schema 和性能合同见以下同一变更集附件：

- `OpenClaw/hepta-inference-runtime-plan-2026-08-26.md`
- `OpenClaw/INF_STAGE_MATRIX_v1.yaml`
- `OpenClaw/hepta.infer_receipt.v1.schema.json`
- `OpenClaw/HEPTA_INFERENCE_PERFORMANCE_CONTRACT_V1.md`
- `OpenClaw/INF-PLAN-APPEND-RECEIPT-2026-08-26.json`

在 effective-index、qualification mirror、authority 和 manifest 重新绑定并生成 fresh receipt
前，本节及附件只能作为 RFC/qualification-only 计划；不得实现、安装模型、触碰 NPU、启动生产
listener、写 shared KG/effect/route/fleet、远程推理、接受 operator 或 promotion。任何 parent digest、
HEAD/tree/dirty、schema、artifact 签名或权限旗标漂移立即标记 `STALE_SOURCE_BINDING` 并 fail-closed。

## E.41 Phase-gated delivery policy — 2026-08-26（当前阶段解释）

本节是总 runbook 的最新阶段解释，专门修正 E.23–E.40 中把“发布前置条件”写成“所有开发工作的
全局 blocker”的歧义。它不删除、重写或改变任何历史 receipt、测试数字、负权限字段或生产安全合同。
它只改变调度语义：**在明确执行最终发布之前，外部 provider/KMS/签名/物理断电证据不得阻塞功能开发、
内测或未发布版本的继续迭代。**

### E.41.0 唯一当前 profile 与优先级

默认阶段为：

```text
active_profile=development_internal_test
phase=DEVELOPMENT
allowed_phases=DEVELOPMENT,INTERNAL_TEST,RELEASE_PREP,FINAL_RELEASE,POST_RELEASE
external_inputs_required=[]
decision=CONTINUE_PRE_RELEASE
```

阶段顺序为 `DEVELOPMENT → INTERNAL_TEST → RELEASE_PREP（可含 RC）→ FINAL_RELEASE → POST_RELEASE`。
只有用户/发布负责人明确选择 `FINAL_RELEASE`（或等价的 production release manifest）时，
provider provenance/status、独立 H7/H8/H9 trust material、H4 physical/media claim、operator acceptance、
CALLERS/promotion 和 H8/H9 production canary/fleet 才进入硬门评估。

E.41 **仅 supersede 早期文字的阶段/调度解释**：E.23–E.40 里出现的 “remaining/next development
blocker” 对外部证据的描述保留为历史审计事实，但在当前 profile 下统一解释为
`release_only_deferred` 或 `implementation_backlog`，不解释为 `DEVELOPMENT_BLOCKED`。

### E.41.1 阶段行为

| 阶段 | 允许继续的工作 | provider / trust / H4 physical / H8-H9 外部门 | 权限姿态 |
|---|---|---|---|
| `DEVELOPMENT` | 未完成功能、代码 seam、unit/compile、fixture、local daemon smoke | `DEFERRED_PRE_RELEASE`；不请求生产 key，不做真实 effect/断电 | 所有 production/effect/promotion flags=false |
| `INTERNAL_TEST` | 本地集成、sandbox、回放、crash/reopen、projection-only supervisor | `ADVISORY_MISSING` 或 `DEFERRED_PRE_RELEASE`；缺失不停止内测 | 仍全部 false；sandbox 必须显式标注 |
| `RELEASE_PREP` / RC | 冻结候选、整理 release manifest、可选 staging 预检 | 可收集/核验 external receipt，但缺失只标 `PENDING_FINAL_RELEASE`；不阻塞继续修代码和内测 | mutation/effect 仍关闭 |
| `FINAL_RELEASE` | 仅执行正式发布准入 | 严格硬门；缺失/过期/冲突为 `BLOCKED_FINAL_RELEASE`，只阻止发布动作 | 只有独立验证的 grant/policy 才能开启 |
| `POST_RELEASE` | 监控、对账、回滚 | 失效/撤销触发回滚；新版本回到 DEVELOPMENT | 由已批准 release authority 控制 |

`BLOCKED_RC`（若旧工具仍输出）只表示该候选尚未具备发布资格，**不得**传播为
`DEVELOPMENT_BLOCKED` 或 `INTERNAL_TEST_BLOCKED`，也不得阻止另一个未发布 head 继续开发。
当前 policy 对外部证据优先使用 `PENDING_FINAL_RELEASE`，避免把 RC 误解成发布前硬阻断。

### E.41.2 四类事项的具体处理

1. **Provider provenance/status**：开发/内测可编译真实 adapter、跑 fixture、跑明确的非生产 sandbox，
   记录 digest/replay/conflict/lookup 的工程结果；没有 provider-owned effect-key/status-by-key、终态
   语义、签名 ACK 或独立 attestation 时只记 `DEFERRED_PRE_RELEASE`。不得把 202/200/409、WireMock、
   本地 key 或 queue receipt 写成 production exactly-once。
2. **H4 durable writer/outbox/power-loss**：WAL/FULL、integrity、crash/reopen、replay、reconcile 和
   rollback 属于开发/内测验收；SIGKILL/重启仍只叫 crash。物理断电与介质耐久只在最终 release manifest
   声明该能力时成为硬门；否则是可选 qualification 记录，不挡功能开发。
3. **H7 signed artifact/OPE、H8/H9 supervisor/rollback**：开发/内测允许 local/staging artifact 做
   verifier-negative、CAS、epoch、expiry、reload、rollback 和 recovery 测试；daemon mutation、
   production caller、governance bypass、promotion 始终关闭。独立 signer/KMS、公钥 pin、fresh grant/OPE、
   operator acceptance、canary/fleet 只在 `FINAL_RELEASE` 评估。

### E.41.3 不可放松的运行时不变量

- 阶段选择只能影响 runbook 的“是否继续工作/是否允许发布”，不能添加 bypass 或把 false authority
  flag 改成 true。
- `production_caller`、`production_writer`、`effect_authority`、`operator_acceptance`、`promotion`、
  `g5_allowed`、`execute_allowed` 在 DEVELOPMENT/INTERNAL_TEST/RELEASE_PREP 均保持 false。
- sandbox/fixture 不能满足 production `ProviderEffectAck`；`Queued/Accepted` 不是 terminal success，
  不明结果必须 `Indeterminate → reconcile`。
- 物理断电声明必须有 operator-confirmed cut、独立掉电/上电监控、boot/replay/integrity 证据；SIGKILL、
  clean reboot 和本地 smoke 不得升级为 physical-power-loss。
- FINAL_RELEASE 的 grant 必须绑定精确 source/tree/artifact/policy/CAS/epoch digest，过期/撤销/换 head
  自动失效并要求新的 release review。

### E.41.4 开发期 DoD 与机器记录

开发/内测 receipt 最少记录：`phase`、source head/tree、测试窗口、`claim_level`、外部门状态、
`decision=CONTINUE_PRE_RELEASE`，以及以下负权限字段全部为 false：

```text
production_caller=false
production_writer=false
effect_authority=false
operator_acceptance=false
promotion=false
g5_allowed=false
external_effect=false
```

开发期真正可以阻塞工作的只包括代码/测试/资源/数据等实现问题（`implementation_backlog`）；
provider owner、独立 KMS/HSM、签名 ceremony、物理断电窗口等外部输入统一列入
`release_inputs_pending`。旧 receipt 继续 immutable，新的证据按当前 head 追加，不得回写成发布凭证。

一句话规则：**功能没做完、内测没完成、没有明确最终发布意图时，继续开发；外部 authority 只做记录，
不做 blocker。最终发布命令才打开严格门，且严格门只阻止发布，不阻止开发。**

## AUTHBUS-PLAN-2026-08-26 — Basil-derived hepta-authbusd / capability-resource control plane（plan-only append）

本节把 AuthBus 设计落到一个可持续吸收 Basil upstream 的实现路线。它是 E.41 当前 DEVELOPMENT/INTERNAL_TEST 调度规则下的 planning-only 追加：不生成生产密钥，不迁移运行中的 Node/Python 服务，不开启 provider/effect/gateway/public listener，不改变任何 false authority flag。

### AUTHBUS.0 结论：应该 fork Basil，但不能把 Hepta 变成 Basil 的长期硬分叉

结论是“fork + upstream remote + additive Hepta crates”，而不是复制源码后任意魔改：

- 上游基线：openbasil/basil，main 当前 commit 1fd29adb8e7356968eacbff9309e056cec9bafd7，workspace version 0.7.2（截至本计划时尚未形成公开 0.7.2 release；公开 latest 为 v0.7.1），Apache-2.0，pre-1.0；仓库声明 Linux x86_64/aarch64 为主要生产目标，macOS/aarch64 主要用于开发。
- 名为 `v0.7.2` 的另一条 upstream 分支实际是 `0.8.0-pre.1` 且与 main 分叉；在其正式兼容性审查前不把它当作同步目标。
- Hepta fork 暂名 hepta-authbus-basil；保留 upstream remote，Hepta fork 作为 origin；所有同步都经过 upstream-sync 分支和兼容性 CI，禁止直接在生产分支拉取 main。
- upstream 层尽量只保留 basil-core、basil-client、basil-proto、basil-keystore-backend 和必要的 transport/backend；Hepta 业务放在独立 crate，不把 AuthResource/Quota/Offer/Wallet/TaskFlow 逻辑塞进 Basil core。
- upstream wire/API 不做无记录的修改。Hepta 使用自己的 versioned auth contracts 和 adapter；若必须扩展 Basil proto，新增 hepta.auth.v1 namespace，保留 basil.broker.v1 兼容面。
- Basil 的 SO_PEERCRED、default-deny、policy matcher、in-place backend、zeroized secret path、append-only audit、reload generation 和 explain/decision 语义直接复用或包裹；OAuth refresh owner、quota scheduler、lease epoch、HNL market、gateway 和 agent projection 由 Hepta 自有层实现。
- Basil 上游安全修复可以快速吸收；功能更新先在隔离 sync lane 回归，再由 Hepta 版本门控合并。不存在“自动跟随 upstream 直接上线”。

因此 Hepta 获得两个好处：第一天拥有成熟的 host-local secret/sign boundary；长期仍能吃到 Basil 的安全修复、backend 和协议改进。代价是必须维护 patch stack、兼容测试和许可证/来源记录，这个代价小于重写 OpenBao 或复制一个无法更新的 secrets daemon。

### AUTHBUS.1 upstream 同步与 fork 治理

固定同步规则：

1. 建立三类 ref：upstream/basil-main、hepta/basil-base、hepta/authbus-integration；发布只从带 source digest 的 Hepta tag 构建。
2. 每次同步生成 upstream commit、tag/version、Cargo.lock、工具链、SBOM、许可证清单和变更分类 receipt。
3. 变更分类为 security、bugfix、non-breaking feature、wire/API change、backend/packaging change；security 可进入加急 review，其余按月或按 Basil release 窗口同步。
4. 同步顺序固定为 fetch → 基线构建 → Basil 原生 test/clippy/audit → Hepta contract/adapter tests → J3160 Linux smoke → Mac development smoke → review → merge。
5. 任何 Basil proto/wire、policy semantics、key handling 或 default-deny 行为变化都需要 compat report；不能只看 cargo test 绿色。
6. 上游 breaking API 只允许在 Hepta 自己的 minor migration 中切换；旧 adapter 保留一个可回滚窗口，禁止双写。
7. Hepta 自有修改必须落在独立 crate 或清晰标记的 patch 文件；每个 patch 有原因、上游 issue/commit、删除条件和回归测试。
8. 每次同步都运行 secret-byte scan、license/REUSE check、unsafe/panic policy check、protocol golden vectors 和 crash/reopen；失败就停在 sync branch，不污染当前运行服务。

Basil README 声明 0.7.x patch 更新应保持非 breaking，0.7.x→0.8.0 才承担 breaking API/wire 变化。Hepta 可以利用该承诺，但仍以现场 exact-version 回归为准，不把上游语义当作自己的 release authority。

### AUTHBUS.2 Hepta fork 的分层设计

```text
basil-core / basil-client / basil-proto / backend
        │  upstream-compatible host secret/sign boundary
hepta-basil-adapter
        │  SecretRef, typed errors, backend health, process-bound use
hepta-auth-contracts
        │
hepta-authbus-core ── hepta-authbus-scheduler ── hepta-authbus-metering
        │                         │
        │                         ├─ hepta-authbus-hnl → hepta-node-linkd
        │                         ├─ hepta-gatewayd → hepta-inferd/provider adapter
        │                         └─ hepta-market-adapter → walletd/TRNM
        │
hepta-authbusd (one managed Rust sidecar per physical host)
        │
agentd/fleet supervisor + Codex/TaskFlow callers
```

**Peer-credential boundary decision（不可省略）**：`SO_PEERCRED` 只证明直接连接
`authbusd` 的本地进程。如果 Basil 作为独立进程运行，内核不会把原始 agent UID/GID
透传给 Basil；因此 `authbusd` 必须在第一跳完成端到端 SubjectRef/tenant policy，Basil
socket 只能信任固定的 `authbusd` service UID。跨进程调用必须携带显式的 capability
attenuation（subject digest、operation、SecretRef allowlist、epoch、TTL 和 fence），
且 Basil 不得声称自己识别了原始 agent。长期目标是在同一个 Rust `hepta-authbusd`
进程内嵌入 Basil broker state/service，或提交一个最小、可回 upstream 的 additive custom
service hook；在此之前 `hepta.auth.v1` 使用独立 UDS，禁止用双层 SO_PEERCRED 掩盖
confused-deputy 风险。

各层唯一责任：

| 层 | 负责 | 明确不负责 |
|---|---|---|
| Basil upstream layer | kernel caller identity、policy、secret/sign/decrypt in place、audit、backend | Hepta quota、market、TaskFlow、wallet |
| hepta-basil-adapter | OpenBao/Vault/KMS typed mapping、SecretRef、错误/health/reconcile | 暴露 raw secret、替代 OpenBao server |
| hepta-authbus-core | AuthResource、lease/permit、epoch/revoke、owner/CAS、refresh owner | 模型执行、余额、fleet lifecycle |
| authbus-scheduler | RPM/TPM/window、semaphore、WFQ/EDF、cooldown、fair admission | 第二个 TaskFlow/fleet scheduler |
| authbusd | UDS/SO_PEERCRED、tenant/agent policy、request id、audit bridge | 直接接受公网请求、修改钱包 |
| HNL adapter | signed capability channel、peer/session binding、跨 node revoke | 路由授权、价格、结算 |
| gatewayd | virtual API auth、stream/backpressure、外接 ingress | 持有 upstream token、接管 inferd pool |
| market-adapter/walletd | offer/order/escrow/nonce/settlement/dispute | 把 raw credential 变成商品 |

### AUTHBUS.3 从 Basil API 到 Hepta API 的映射

| Basil 能力 | Hepta 暴露方式 | 迁移规则 |
|---|---|---|
| kernel-attested local caller | SubjectRef（tenant/workspace/agent/service/generation） | UDS caller 先过 Basil，再过 Hepta scope/fence |
| policy allow/deny/explain | AdmissionDecision + deny reason | 共用 default-deny，Hepta 只收窄不放宽 |
| sign/decrypt/mint-jwt | SecretUse/SignIntent/VirtualCredential | 返回 reference/receipt，不返回长期私钥 |
| OpenBao/Vault/KMS in-place backend | SecretBackend trait | OpenBao 保持 Go server，Rust 只做 typed adapter |
| audit decision | AuthDecision/UsageReceipt projection | 不把 raw prompt/token 写入 audit |
| broker generation/reload | policy_epoch/authority_epoch | epoch bump 使旧 lease/permit 失效 |
| Basil proto | hepta-auth contracts + adapter | 不把 Hepta market 字段偷偷塞入 basil.broker.v1 |

### AUTHBUS.4 Hepta 自有核心合同

首批合同必须冻结以下对象：AuthResource、SecretRef、QuotaContract、QuotaSnapshot、AuthRequest、ResourceLease、UsagePermit、UsageReceipt、VirtualCredential、Revoke、Offer、UsageRight、DispatchPermit、SettlementRef。权威编码使用版本化 canonical bytes、domain-separated digest 和明确 signing_bytes；JSON/HTTP 只做边缘投影。

硬性语义：

- raw access/refresh/token/key 只存在 SecretBackend 或受控 adapter 的短暂内存路径；不进 agent SQLite、Basil audit、HNL wire、gateway log、market receipt。
- quota 未知、OpenBao sealed/standby、provider 429/timeout、响应丢失和状态不明必须 conservative/Indeterminate；不得用换账号规避限制。
- lease/permit 绑定 owner、subject、resource digest、payload/model digest、epoch、generation、nonce、audience、nbf/exp、max usage 和 policy digest。
- Resource owner 单写；远程 node 通过 HNL reserve，owner 不可达时停止新 lease；不做未经证明的 multi-primary CRDT。
- TaskFlow 只提交 admission request；agentd/fleet 仍掌管生命周期；inferd 仍掌管模型运行；walletd 仍掌管结算。

### AUTHBUS.5 Basil 代码增量和 fork 目录约定

建议 fork 后保持以下目录稳定：

```text
crates/basil-bin            # upstream CLI/daemon，少改
crates/basil-client         # upstream Rust client，少改
crates/basil-core           # upstream policy/backend/transport，安全同步
crates/basil-proto          # upstream wire，禁止无记录改字段
crates/basil-keystore-backend
crates/hepta-basil-adapter  # Hepta-owned OpenBao/SecretRef bridge
crates/hepta-auth-contracts
crates/hepta-authbus-core
crates/hepta-authbus-scheduler
crates/hepta-authbusd
crates/hepta-authbus-hnl
crates/hepta-authbus-metering
crates/hepta-gatewayd
crates/hepta-market-adapter
```

不要把 Hepta 代码直接改写进 basil-core 的 policy matcher、crypto envelope 或 transport，除非上游缺陷修复必须如此；必要 patch 要能单独 revert，并带 upstream-compatible test。这样以后同步 Basil 时主要是合并新增 crate 和有限 adapter patch，而不是解决巨大冲突。

### AUTHBUS.6 细化开发阶段（历史摘要；已由 AUTHBUS.11/Stage Matrix overlay supersede）

> 本表仅供历史回放。不得作为当前执行输入；实现顺序、阶段依赖和验收归属以
> `AUTHBUS.11 Contract Crosswalk & CI Rules v1.2` 与 `AUTHBUS_STAGE_MATRIX_v1.yaml#/implementation_overlay/phase_map`
> 为唯一规范。

| 阶段 | 交付 | 上游/Hepta 验收 |
|---|---|---|
| B0 provenance | fork、Apache notices、SPDX/REUSE、upstream remote、commit pin、SBOM | Basil 0.7.2 main snapshot 原生 build/test；source receipt 完整 |
| B1 base import | basil-core/client/proto/backend 编译包装、feature profile、J3160 target；明确同进程/受限 bridge 边界 | policy/UDS/backend/audit golden tests；不接旧生产 caller；peer identity 不丢失 |
| B2 SecretRef adapter | OpenBao typed adapter、404/401/timeout 分类、process-bound use | no raw secret projection；sealed/standby fail-closed；reconcile tests |
| B3 AuthBus contracts | Resource/Quota/Lease/Permit/Receipt、CAS/epoch/generation | state-machine/property/golden vectors；Basil policy 只能收窄权限 |
| B4 local scheduler | semaphore/token bucket/WFQ/EDF/cooldown/singleflight | 16 agent/3 resource synthetic；无超卖/重复 permit/starvation 漏洞 |
| B5 authbusd | Rust UDS、SO_PEERCRED、tenant/agent policy、WAL/audit | crash/reopen、fsync、stale owner、legacy observe bridge |
| B6 HNL federation | peer/session binding、signed usage-right、revoke/expiry、remote reserve | 3-node loopback/QUIC；抓包 raw secret=0；partition/replay reject |
| B7 gateway | virtual credential、OpenAI-compatible API、inferd/provider path、metering | stream/backpressure/cancel、quota/revoke、upstream token exposure=0 |
| B8 market sandbox | Offer/Order/UsageRight/escrow/dispute、wallet/TRNM adapter | virtual credits；double settle/double spend=0；无真实资金 |
| B9 migration/release | JS/Python observe→shadow→projection→canary；upstream sync CI | single writer、rollback；FINAL_RELEASE 才评估外部 provider/KMS/H4/H8-H9 |

B1–B8 属于实现 backlog，可在 E.41 DEVELOPMENT/INTERNAL_TEST 中推进；B9 的 external evidence 是 release_inputs_pending，不得变成开发阻断。本段及上表为历史摘要，不是当前执行 DAG。

### AUTHBUS.7 上游更新流水线

```text
upstream release/security notice
         ↓
fetch + source/license/SBOM receipt
         ↓
Basil native tests/clippy + protocol vectors
         ↓
Hepta adapter/contracts/scheduler/gateway tests
         ↓
J3160 Linux smoke + Mac development smoke
         ↓
security/review/compat report
         ↓
hepta-authbus-basil tag → canary → promote or rollback
```

更新策略：security fix 进入专门 fast lane；普通 patch 在固定窗口同步；minor/breaking 版本先建立 compatibility branch。每个 Hepta release 记录 upstream commit、Hepta patch list、Cargo.lock、toolchain、tests、SBOM、license 和 known gaps。任何 upstream 更新失败只影响 sync lane，不影响当前开发版继续运行。

### AUTHBUS.8 验收与发布边界

必须同时通过三组验收才可称为“Basil-derived Hepta AuthBus implementation”：

1. **Upstream parity**：Basil 原生测试、协议 fixture、policy/audit/crypto/backend 回归不退化；
2. **Hepta semantics**：quota/concurrency/fairness、lease/CAS/epoch/reconcile、HNL permit、gateway virtual auth、wallet boundary 和 single-writer 通过；
3. **Security/operations**：secret-byte scan、default-deny、rotation/revoke、crash/reopen、resource cap、J3160 service supervision、rollback 和 source/license receipts 通过。

以下不属于实现阶段 B0–B9/B10 的提前硬门：真实 provider exactly-once、独立 production signer/KMS、物理断电介质证据、operator acceptance、公开市场资金。它们按 E.41 只在 FINAL_RELEASE 评估；fixture、WireMock、SIGKILL 和本地生成 key 不得冒充这些证据。

### AUTHBUS.9 当前决定和下一步（历史摘要；已由 AUTHBUS.11 supersede）

> 本节是历史回放，不能驱动代码生成、CI 或当前阶段选择。实现队列只读取
> `AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map`，并以
> `AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry/projection_contract_map` 解析合同投影。

决定：采用 Basil 作为 Hepta AuthBus 的 host-local broker 基座，建立 hepta-authbus-basil fork；不采用 RustyVault 替换 OpenBao，不把 Codex CredentialBroker 当 secrets authority，不把 Basil API 当完整市场协议。

历史记录中的立即队列曾写作 B0 provenance、B1 base import、B2 SecretRef adapter、B3 contracts、B4 local scheduler；当前规范队列以 B0→B1→B2 contracts→B3 adapter→B4 scheduler 为准。第一批只使用合成 provider/inferd 和现有 J3160 loopback；不改当前 AuthBus/OpenBao 运行态，不生成生产 key，不接真实交易资金。

本 append 的详细合同、阶段矩阵、receipt schema 和同步记录见：

- OpenClaw/hepta-authbus-basil-fork-plan-2026-08-26.md
- OpenClaw/AUTHBUS_STAGE_MATRIX_v1.yaml
- OpenClaw/hepta.authbus_receipt.v1.schema.json
- OpenClaw/HEPTA_AUTHBUS_BASIL_UPSTREAM_POLICY_V1.md
- OpenClaw/AUTHBUS-PLAN-APPEND-RECEIPT-2026-08-26.json

本节状态为 PLANNING_ONLY；E.41 的 DEVELOPMENT/INTERNAL_TEST/RELEASE_PREP 规则继续有效，所有 production_caller、production_writer、effect_authority、operator_acceptance、promotion、g5_allowed、execute_allowed 保持 false。

### AUTHBUS.10 Implementation Clarifications v1.1（implementation input；不是新增 release gate）

**状态：HISTORICAL / SUPERSEDED_BY_AUTHBUS.11 v1.3。** 本页保留用于 provenance 和旧实现
回放；实现、代码生成和 CI 必须读取 AUTHBUS.11 v1.3 的 canonical registry projection map（`AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry/projection_contract_map`）及其已声明的
`execution_closure_v1_3` 投影，不得把本页的旧字段、profile 或阶段映射当作当前规范。

本页把 AUTHBUS-PLAN-2026-08-26 从架构意图收敛为可编码的接口、边界和验收输入。它只覆盖
实现排序和合同细节；不修改 E.41，不把 provider/KMS/HSM、物理断电、公开结算或 operator
acceptance 提前成开发门槛。旧的 B0–B9 表仍是摘要；本页的依赖 DAG 是实现时的规范覆盖，避免
旧表中 B2/AUTH-3、B3/AUTH-1、B9 混合编号造成误排。

#### 10.1 权威边界与唯一 durable writer

- AuthBus 属于 Governance/Safety 的 resource-admission 实现，不是 Hepta 的第六个 authority
  plane。每个物理 Host 运行一个由 `agentd/fleet supervisor` 管理的 Rust `hepta-authbusd`。
- `authbusd` 是 `AuthResource`、`ComputeResource`、quota snapshot、lease、permit reservation
  和 reconcile 的唯一 durable writer；使用 SQLite WAL（`synchronous=FULL`，平台可用时
  `fullfsync`）并在提交后同步目录。Basil audit 只是决策投影，不能代替业务 WAL。
- Agent-local SQLite 只保存 projection/cursor/只读缓存，不保存 token 或 reservation authority。
  每个 mutation 必须带 `command_id`、`expected_revision`、`authority_epoch`、`owner_epoch`、
  `agent_generation` 和 `fencing_token`；重复命令返回原 receipt，冲突拒绝。
- `agentd` 负责 launch/lease handshake、generation 和 owner transfer；分区或 owner 不可达时
  停止新 lease，旧 permit 只能按 TTL drain。TaskFlow 只提交 admission request，不能成为第二
  scheduler；inferd 掌管执行，walletd/TRNM 掌管余额/escrow/settlement。

#### 10.2 进程拓扑与调用者身份

- 首选：将 Basil broker state/service 嵌入同一 `hepta-authbusd` 进程，统一第一跳授权和审计。
- 过渡：Basil 独立进程只接受固定 service UID 的本地连接；`SO_PEERCRED` 不会透传原始 agent
  UID/GID。桥接信封必须含 `subject_digest`、tenant、operation、SecretRef allowlist、
  audience、nonce、TTL、policy/epoch/fence，并拒绝转发或自报的 UID。
- 同一 OS UID 不能区分 Agent A/B；Linux 使用 agentd generation + pidfd/starttime/launch nonce
  绑定，macOS 使用 audit-token 或等价平台适配器。无法提供绑定时，明确把同 UID 视为一个
  trust domain，而不是伪称 per-agent 隔离。
- 仅开放两个本地面：`authbusd-agent.sock`（admission/permit/watch）和
  `authbusd-control.sock`（agentd/operator lifecycle）；不开放 Basil/OpenBao 公网端口。

#### 10.3 Basil 最小 service profile（default-deny）

Hepta 编译/注册的 Basil profile 默认只允许：`Sign`、`Verify`、`PublicKey`、`Health`
以及显式的 process-bound virtual-capability issue。`Decrypt` 默认关闭，只有显式
`process_bound_decrypt` feature、allowlisted SecretRef 和 active permit 同时满足时才可加入。
默认拒绝并做负向 RPC/raw-byte 测试：

`GetSecret`、`SetSecret`、`ImportSecret`、`ListSecret`、`NewKey`、导出私钥、证书私钥返回、
`Mint`、NATS/SDS/SPIFFE、Admin、远程 invocation 和未知新增 RPC 均不得注册或调用。Basil 的
zeroized/in-place 路径不等于“绝不产生字节”；短暂 bearer bytes 只可在隔离 adapter 内存中存在，
不得进入 `String`、env、SQLite、日志、wire、swap、fork 或 receipt。

#### 10.4 合同、状态机和 RPC

复用 canonical `CommandEnvelope`、`EffectIntent`、`ActivityReceipt`、`EffectReceipt`、
`FailureClass`、`IndeterminateReason` 和 `CapabilityId`，不另造平行协议。AUTHBUS 合同补齐：
`AdmissionDecision`、`QuotaReservation`、`ProviderQuotaDomain`、`ProviderStatus`、
`EffectAck`、`OperationRef`、`CapabilityAttenuation`、`PeerSession`、`ClockSnapshot`、
`ResourceAdvertisement`。

规范状态链为：`Admission → Reservation → EffectIntent → DispatchAccepted → Ack | Indeterminate
→ Reconcile → Release | Refund`；取消、部分 stream、terminal immutable、重放和超时都必须有
明确事件。最小 RPC 为：`RegisterResource/Describe`、`AcquirePermit`、`RenewLease`、`Release`、
`ReportUsage`、`ReportProviderFeedback`、`Reconcile`、`Revoke`、`WatchEvents`（sequence/hash
 cursor 可恢复）、`IssueVirtualCredential`、`Health/Ready`。

#### 10.5 旧 OAuth 生命周期与 SecretRef 迁移

实现 `AuthProfile`（`profile_id`、`provider_id`、`secret_ref`、scope/quota digest、
`max_concurrency`、rpm/tpm window、expiry、`last_good`、cooldown、health、quarantine、
revision、authority_epoch）和 `ProviderSession/TokenFamily/RefreshAttempt`。refresh owner
按 profile 做 CAS + singleflight；`invalid_grant` 进入 quarantine/backoff，响应丢失进入
reconcile，不盲目重发。

现有约 151 个 profile 只迁移 metadata/`SecretRef`、order/health 和 revision；先做 observe→
projection→canary，旧 Node/Python 仅只读观察，不与 Rust 双写。迁移 receipt 必须证明 raw
access/refresh/token/key 没有进入 agent SQLite、日志或 projection；回滚切换 authority flag，
不得把 indeterminate 改写成 success。

#### 10.6 quota、调度与计量

使用 `QuotaVector=(rpm,tpm,concurrency,day_budget,context)`，同时约束 provider/account/org/IP/
model/endpoint 聚合域。请求先估算成本，再在安全余量下 reservation；quota observation 带
`source`、`confidence`、`observed_at`、clock uncertainty，未知值按 conservative 处理，429/
`Retry-After` 只降低可用容量。调度器组合 per-resource semaphore、token bucket、WFQ/EDF、
cooldown 和 deadline；公平/饥饿/超卖指标写入实现 receipt。

`MeteringContract` 固定 unit、preauth/hold、estimate/final、rounding、cancel/partial-stream
和 refund 语义；UsageReceipt 只含 digest、数量和引用，不含 prompt/token。P0 只实现 Auth 与
Compute 两类资源；Egress 先保留 trait，不把网络路由塞入 AuthBus。

#### 10.7 HNL、usage-right 与 gateway

HNL 复用既有 HNL-0..6 的 NodeId、Principal、session 和 capability facade，不新造 root 或
handshake。跨节点只传签名 `Offer/UsageRight/DispatchPermit/UsageReceipt/Revoke`；“二手 auth”
在协议中命名为可撤销、有限 scope 的 delegated capacity/usage-right，`transferable=false`
为默认，provider/owner consent 与 ToS 是业务输入。Offer/Order/Escrow/Settlement/Dispute
由 market-adapter/walletd/TRNM 负责，结算引用永远不能授权调用。

`hepta-gatewayd` 仅接受短期 virtual credential（kid、tenant、aud、scope、model、TTL、jti、
nonce、policy digest）；入口的 `Authorization: Bearer <virtual-credential>` 只用于提取和
验证虚拟凭据。调用者不得选择上游 URL，也不得转发任意上游 headers；真实上游
`Authorization` 由 allowlisted adapter 在网关边界之后构造并留在 adapter/OpenBao。入口做
registry allowlist、SSRF/DNS-rebinding/CONNECT 防护、body/stream 上限、backpressure/cancel、
tenant quota 和 PII/retention policy；模型输出不得铸造 key、grant、wallet asset 或 command authority。

#### 10.8 故障、模式和审计

必须区分 sealed/standby、401/403、404（不存在 vs mount misconfiguration）、429、timeout/reset、
disk-full、WAL corruption、OOM、clock rollback 和 network partition；不确定结果统一进入
`Indeterminate → Reconcile`。readiness 表示可接收新命令，liveness 不代表可发新 permit；支持
drain/kill switch、schema migration、目录 fsync、cursor resume 和 audit retention。

模式固定为 `DEVELOPMENT`（synthetic/loopback）、`INTERNAL_TEST`（显式 LAN、无真实资金）、
`RELEASE_PREP`（批准的 sandbox/virtual credits）和 `FINAL_RELEASE`（外部证据后才可评估）。
前三种模式的外部 provider、KMS/HSM、物理 H4、公开 gateway/settlement 仍按 E.41 deferred；
本页不改变任何 production/effect/promotion false flag。

#### 10.9 规范实现 DAG、DoD 与附件

实现顺序固定为：`B0 provenance → B1 topology/profile/identity → B2 contracts → B3 adapter →
B4 scheduler → B5 authbusd-WAL/reconcile → B6 legacy migration → B7 HNL → B8 gateway → B9
market sandbox → B10 release-prep`。量化目标是实现回归目标而非当前发布门：调度 0 oversell/
duplicate permit、Jain fairness ≥0.95、B5 fault-injection ≥1000 次且 fsync-complete RPO=0、
B7 partition/replay old-permit acceptance=0、B8 cancel ≤1s、B9 ledger conservation/double-settle=0。

本页的四个实现输入附件为：

- `OpenClaw/AUTHBUS_TRUST_MODE_MATRIX_v1.yaml`
- `OpenClaw/AUTHBUS_RESOURCE_QUOTA_METERING_CONTRACT_v1.yaml`
- `OpenClaw/AUTHBUS_FAILURE_RECONCILE_STATE_MACHINE_v1.yaml`
- `OpenClaw/AUTHBUS_ABUSE_DISPUTE_POLICY_v1.yaml`

这些附件是编码和测试输入，不是独立 authority receipt；plan append receipt 也不等同于 stage
receipt。B0–B9/B10 失败只影响 implementation backlog 或 sync lane，不改变 E.41 的开发连续性。

<a id="authbus11-contract-crosswalk-ci-rules-v12"></a>
### AUTHBUS.11 Contract Crosswalk & CI Rules v1.2（实现修订；不新增 release gate）

本页是 AUTHBUS-PLAN-2026-08-26 的规范实现修订。它把上一页的架构约束落成唯一的
contract/state/error registry、写者边界、dispatch 崩溃语义、Basil profile 和阶段验收归属。
它只改变 AuthBus 的实现输入，不改 E.41，不把 provider、KMS/HSM、物理断电、公开结算或
operator acceptance 提前成开发阻断。旧附件中的 v1 字段和状态保留作 decode-only/historical
兼容输入；本 v1.2 页现已是历史记录，新代码和 CI 必须读取 AUTHBUS.11 v1.3 的 canonical
registry projection map（`AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry/projection_contract_map`）及其已声明的
`execution_closure_v1_3` 投影。

#### 11.1 唯一 canonical contract owner

- `codex-hepta-contracts` 是 Hepta 已有的公共合同 owner，继续拥有
  `CommandEnvelope`、`EffectIntent`、`ActivityReceipt`、`EffectReceipt`、`FailureClass`、
  `IndeterminateReason`、`CapabilityId`、既有 `ReconcileDecision`、CAS/fence 字段和 v1.4 状态集合。
- 新建的 `hepta-auth-contracts` 只拥有 AuthBus 专属类型：`AuthResource`、`ComputeResource`、
  `ProviderQuotaDomain`、`QuotaSnapshot`、`QuotaReservation`、`ResourceLease`、`UsagePermit`、
  `EffectAck`、`OperationRef`、`CapabilityAttenuation`、`PeerSession`、`ClockSnapshot`、
  `ResourceAdvertisement`。它必须 import/re-export 公共合同，不能复制公共 enum 或 envelope。
- AuthBus wire namespace 固定为 `hepta.auth.v1`；Basil 的 `basil.broker.v1` 只作为上游兼容
  transport。任何边缘 JSON/HTTP/CBOR 都是 projection，不能产生第二套状态或错误语义。
- 权威 bytes 由公共合同的 versioned canonical serialization 产生，digest 使用 domain-separated
  SHA-256。本地 UDS 可使用 Basil protobuf/length-delimited transport，但必须携带相同 canonical
  bytes；HNL 使用既有 HNL-0..6 的 canonical CBOR/COSE_Sign1。未知 schema/字段在权威 decode
  时拒绝，边缘 projection 不得反向升级权限。

对象写权威固定如下；projection、audit 和 outbox 永远不能反向写 owner：

| 对象 | 唯一 durable writer | 只读/协作层 |
|---|---|---|
| `AuthResource`/`ComputeResource`/`QuotaSnapshot` | 每 Host 的 `hepta-authbusd` | agentd 注册握手、Basil audit projection |
| `Lease`/`Permit`/`QuotaReservation`/`ReconcileQueue`/`EffectDispatchRef` | `hepta-authbusd` WAL | agent、gateway、HNL adapter 提交 typed command |
| `FlowRun`/`Step`/`EffectIntent`/`ActivityReceipt`/`EffectReceipt` | owning Agent 的 TaskFlow DB | AuthBus 通过 command/outbox 交互，不改 flow ledger |
| `Offer`/`UsageRight`/`UsageRightCounter`/`Dispute` | market-adapter 的交易状态与 entitlement counter | AuthBus 只提交 fenced `ReserveUse/CommitUse/ReleaseUse` command 并保存引用 |
| escrow/balance/nonce/settlement | `walletd`/TRNM | AuthBus 只保存 `SettlementRef` |
| raw provider secret/private key | OpenBao/SecretBackend 内部 | AuthBus 只保存 opaque `SecretRef` |

跨 owner 的唯一通道是带 `command_id`、`idempotency_key`、`expected_revision` 和 causal
lineage 的 typed command + transactional outbox；不允许共享万能数据库或双写。

#### 11.2 字段、状态和错误 crosswalk

基础合同与 AuthBus 扩展必须显式分层：`codex-hepta-contracts.v1.4`/E21 仍是公开
`CommandEnvelope`、event 和 `EffectReceipt` 的基础合同；`hepta-auth-contracts.v1` 只能
追加资源/租约/配额字段并 re-export 基础类型。v1.4 的 `EffectReceipt` 是闭合对象；其精确
required set 固定为
`effect_id,effect_intent_id,decision_id,run_id,schema_digest,status,terminal,
reconciliation_required,idempotency_key,attempt,authority_scope,execution_scope,
effect_authority,production_authority,snapshot_digest,graph_digest,policy_digest,provider_id,
reconciliation_id,reconciles_receipt_id,external_state_digest,resolver_attempt,current_revision,cas`。
事件 envelope 另有 `schema_version,event_id,event_kind,aggregate_id,event_seq,expected_revision,
authority_epoch,owner_epoch,generation,fencing_token,logical_clock,causal_parent_event_id,payload,
payload_digest,prev_event_digest,event_digest,hash_algorithm` 等字段；二者不可混成一套可变
receipt。AuthBus 扩展不得删除、改名或向闭合的 E21 `EffectReceipt` 顶层添加字段；生成器先
校验基础合同，再叠加对象专属字段到 envelope/payload 或 sidecar，最后按 `hepta.auth.v1`
canonical bytes 投影，避免只凭 AuthBus fence 列表构造出不合法的 v1.4 receipt。

所有写 mutation 至少带：
`schema_version, command_id, run_id, aggregate_id, expected_revision, authority_epoch,
owner_epoch, generation, fencing_token, logical_clock, causal_parent_event_id,
payload_digest, policy_digest, resource_digest`。需要幂等的写操作还必须带
`idempotency_key`（health/ready/watch/verify 等纯读或观察操作例外）。Effect 还必须带
`effect_id, effect_intent_id, idempotency_key, attempt`；reservation 还必须带
`reservation_id, resource_id, quota_domain, expires_at`；event 还必须带
`event_id, event_seq, prev_event_digest, event_digest`。历史别名只在 decode 接受并在 encode
时输出 canonical 名称：`revision→expected_revision`、`agent_generation→generation`、
`fence→fencing_token`、`payload_sha256→payload_digest`、`operation_id→provider_operation_id`。
E21 v1.4 的公开 `CommandEnvelope`/event wire canonical 字段是
`causal_parent_event_id`；既有 event-store 内部序列仍使用 `causal_parent_seq`。
二者不是同义别名：adapter 必须用同一 `event_seq → event_id` 索引做显式双向映射，
并在 canonical bytes 中只按当前 serialization context 发出一个字段。公开 E21/event
serialization context 发出 `causal_parent_event_id`；内部 WAL/event-store context 发出
`causal_parent_seq`，并同时保留可查的 parent event id。旧输入可按上下文
兼容解码，但不得把整数序列直接当作事件 ID。

公共 v1.4 status 集合是唯一可发出的 wire status：

- `ActivityReceipt`：`Queued | Running | Succeeded | Failed | Cancelled | Indeterminate`；
  `Succeeded/Failed/Cancelled` 才是 direct terminal，`effect_authority=false`。
- `EffectReceipt`：`Queued | DispatchAccepted | Running | Succeeded | Failed | Cancelled |
  Indeterminate | Reconciled`；`DispatchAccepted` 是 queue acknowledgement，不是成功；
  `Indeterminate` 必须 `reconciliation_required=true`；`Reconciled` 只能由已验证的
  terminal external state、terminal failure 或 non-effect proof 产生。其 terminal outcome
  不扩展 E21 `effectReceipt` 顶层，也不塞入闭合的 E21 payload。沿用既有
  `HEPTA_EFFECT_RECONCILIATION` 命名的 `ReconcileDecision`：authbusd 只在自己的 WAL 写
  `ReconcileEvidenceRef` proposal，TaskFlow 验证后在 effect ledger 提交
  `ReconcileDecision(reconciliation_id, effect_id, outcome, evidence_digest, provider_query_receipt_digest,
  decided_at, authority_epoch, owner_epoch, generation, fencing_token)`，并写闭合的
  `EffectReceipt.status=Reconciled`。这不是第二个 receipt/status protocol；没有 evidence 时 outcome
  只能是 `ManualRequired`，且不能关闭 `Indeterminate`。
- `QuotaReservation`：`Proposed | Held | DispatchAccepted | PartiallyConsumed | Indeterminate |
  Released | Refunded | Expired`。它是 quota 结果，不是 EffectReceipt；`Released/Refunded`
  不能编码成 effect success。
- market `UsageRight` 的唯一 lifecycle owner 是
  `AUTHBUS_ABUSE_DISPUTE_POLICY_v1.market_lifecycle`。`escrow_confirmed`、
  `terminal_receipt`、`approved_refund` 等是 canonical event；`escrow_held`、
  `authorized_settlement`、`authorized_refund` 只允许 decode-only alias。
  `PARTIALLY_CONSUMED + usage_reported` 必须支持幂等 self-loop（同一 `usage_id` 不得重复计费），
  terminal `SETTLED | REFUNDED | CLOSED` 无出边且 immutable。
- canonical failure classes 为 `AUTHENTICATION`、`AUTHORIZATION_OR_SCOPE`、
  `BACKEND_MISCONFIGURED`、`NOT_FOUND`、`QUOTA_EXHAUSTED`、`BACKEND_SEALED`、`THROTTLED`、
  `TIMEOUT`、`CONNECTION_RESET`、`LOST_RESPONSE`、`LEASE_STALE`、`PARTITION`、`CLOCK_SKEW`、
  `INVALID_GRANT`、`DISK_FULL`、`WAL_CORRUPT`、`OOM`、`OWNER_EPOCH_MISMATCH`、
  `RESOURCE_REVOKED`、`PROVIDER_5XX`、`QUOTA_UNKNOWN`。`AUTHZ_DENIED` 只作为 decode alias，
  输出 `AUTHORIZATION_OR_SCOPE`；`TRANSPORT_UNKNOWN`/`BACKEND_UNAVAILABLE` 只作为旧输入
  alias，必须映射到 canonical class。
- `IndeterminateReason` 至少覆盖 `Timeout`、`ConnectionReset`、`LostResponse`、
  `NetworkPartition`、`Provider5xx`、`BackendSealed`、`DiskFull`、`WalCorrupt`、`Oom`、
  `OwnerEpochMismatch`、`UnknownDispatch`、`RefreshResponseUnknown`、`StatusLookupUnavailable`；
  未知结果不得直接映射成 `Failed`、`Released` 或 `Refunded`。

#### 11.3 Dispatch WAL 和崩溃窗口

`authbusd` 的唯一 writer WAL 使用 `synchronous=FULL`（平台支持时 `fullfsync`）并在提交后
同步目录。规范顺序固定为：

```text
TaskFlow EffectIntent committed + outbox
  → EffectDispatchRefDurable (fsync; authbusd internal ref)
  → DispatchAttemptStarted (fsync; internal subrecord)
  → adapter call
  → DispatchAcceptedRef OR DispatchUnknownRef (fsync)
  → EffectAckRef | IndeterminateRef
  → TaskFlow EffectReceipt via outbox
  → Reconcile
  → Release | Refund
```

`DispatchAttemptStarted` 是内部 WAL subrecord，不是新的公共 `EffectReceipt` status；其字段必须
包含 `effect_id, effect_intent_id, idempotency_key, attempt, operation_key, call_digest,
adapter_id, command_id, run_id, aggregate_id, expected_revision, authority_epoch, owner_epoch,
generation, fencing_token, logical_clock, payload_digest, policy_digest, resource_digest`。
adapter 调用前必须 fsync 该 marker。进程若在调用后、写入 response 前崩溃，只能进入
内部 `DispatchUnknownRef`，并向公共 `EffectReceipt` 投影为 `Indeterminate`；恢复时按
`effect_key`、provider idempotency key 或已绑定 `OperationRef` 查询；不得盲目重发。只有
verified-not-found、provider idempotency scope、non-effect proof、当前 epoch/fence 全部成立时
才允许创建新 attempt。`EffectDispatchRefDurable` 不拥有或替代 TaskFlow 的 canonical
`EffectIntent`/`EffectReceipt`；它只保存 digest、operation ref 和 dispatch 状态。

owner/fence 算法：`authority_epoch` 只在 authority/policy transfer 时递增并先持久化；每次
writer claim/transfer 原子递增 `owner_epoch` 且不复用旧值；`generation` 由 agentd 在 launch
时发放；每个 `(owner_epoch,generation)` 使用新 fencing token。旧 epoch、旧 generation、旧
callback 或过期 lease 一律拒绝且不改 quota。`event_seq/prev_event_digest/payload_digest`
形成可重建链；projection cursor 不是 authority。

#### 11.4 Basil 精确最小 profile 与身份边界

上游 pinned source capture 是 `openbasil/basil@1fd29adb8e7356968eacbff9309e056cec9bafd7`，
workspace `0.7.2` main snapshot（公开 release `v0.7.1`），Apache-2.0。B0 只记录 source/tree、
archive、Cargo.lock 和 toolchain receipt；未完成 source capture 不能声称 fork 已导入。

Hepta profile 固定为 `hepta-basil-host-minimal-v1`：

- 编译/注册默认只启用 `Sign`、`Verify`、`PublicKey`、`Health` 和 Hepta wrapper 的
  `IssueProcessBoundCapability`；`Decrypt` 只有显式 `process_bound_decrypt` feature 才可加入，
  且明文只在 adapter 进程内存短暂存在。
- 不构建或暴露 `basil-bin` 的公网/独立 host listener。上游源码观测到的 `Signing`、`Aead`、
  `Secret`、`Minting`、`Nats`、`Admin`、`Invocation`、`SPIFFE/SDS/NixCache` 服务组中，
  `GetSecret/SetSecret/ImportSecret/ListSecret/NewKey`、私钥导出/证书私钥返回、unscoped mint、
  NATS/SDS/SPIFFE、Admin、远程 invocation 和未知 future RPC 必须在 compile、registration、
  runtime 三层拒绝。
- `SO_PEERCRED` 只证明直接 socket peer。首选 Basil broker 嵌入 `hepta-authbusd`；分进程过渡
  方案只信任固定 service UID，并通过 COSE/CBOR 或 mTLS bridge 传递衰减 capability
  （`subject_digest, tenant_id, workspace_id, agent_id, service_id, node_id, session_id,
  operation, SecretRef allowlist, audience, nonce, issued_at, expires_at, policy_digest,
  intent_digest, transcript_digest, authority_epoch, owner_epoch, generation, fencing_token`）。
  不得把转发的 UID 声称为内核证明。
- Linux 同 UID 的 Agent A/B 需要 agentd `generation + launch_nonce + pidfd/starttime` 绑定；
  macOS 使用 audit-token 适配器，当前只列为 development target。无法提供绑定时按一个
  trust domain 处理，不伪称 per-agent 隔离。
- 只开放 `authbusd-agent.sock`（admission/permit/renew/release/usage/watch）和
  `authbusd-control.sock`（register/owner-transfer/agentd lifecycle/drain/policy/revoke/health/ready），均为本机 ACL；Basil/OpenBao
  不开放公网端口。`walletd` 只能提交 settlement reference，永不能授权 dispatch。

#### 11.5 RPC/actor/字段映射

| 本地面 | 允许调用者 | 规范操作与关键绑定 |
|---|---|---|
| `control.sock` | agentd（全部 lifecycle/owner mutation）、受审计 operator（仅 Drain/PolicyReload/Health/Ready，或 `emergency=true` 的 bounded Revoke） | agentd: `RegisterResource/OwnerTransfer/AgentdLifecycle/Drain/PolicyReload/Revoke/Health/Ready`; operator: `Drain/PolicyReload/Revoke/Health/Ready`；`authority_epoch/owner_epoch` |
| `agent.sock` | generation-bound agent、gateway、TaskFlow adapter | `AcquirePermit/RenewLease/Release/ReportUsage/WatchEvents`；tenant/workspace/agent/generation/fence |
| adapter-only | authbusd worker | `ReportProviderFeedback/Dispatch/StatusByEffectKey`；effect key、operation ref、payload digest |
| reconcile worker | authbusd + verified adapter/wallet proof | `Reconcile`；只允许 lookup/verified terminal/non-effect proof，不能盲 retry |
| gateway seam | hepta-gatewayd | `IssueVirtualCredential/AcquirePermit/ReportUsage`；audience、scope、TTL、PoP/replay |

`owner` 没有直连特权 socket。所有 owner 的 Register/Delegate/Revoke 意图都由 agentd 在
`SubmitOwnerCommand` 通道代提交，并携带 `owner_attestation_digest`、`consent_digest`、
tenant/resource 绑定、epoch/fence 和 `idempotency_key`；AuthBus 只验证已衰减的 owner
命令。operator 的 `Revoke` 仅限 `emergency=true`、incident/reason/bounded-scope，不能
注册资源、转移 owner、启动/停止 agent 或改 market outcome。`authbusd_worker` 与
`verified_adapter` 是内部 actor，前者可 dispatch/feedback，后者只可按 effect key lookup
和返回绑定 ACK；两者均不能直接写 TaskFlow ledger。

HNL 字段采用角色限定的唯一 crosswalk：`NodeDescriptor.node_id → descriptor_node_id`；
`AgentDelegation.parent_node_id → issuer_node_id`；`ConnectIntent.initiator_node → local_node_id`、
`ConnectIntent.remote_node → remote_node_id`；`agent_generation → generation`、
`capability_hash → capability_digest`、`purpose_digest → intent_digest`、`session_nonce/nonce → nonce`、
`audience → aud`、`max_depth → depth`、`scope → scope_digest`、`expiry → expires_at`。
`epoch/policy_digest/record_seq/key_id/signature/transport_endpoint_id` 分别保留为
`epoch/policy_digest/record_seq/key_id/signature/endpoint_id`；`scope_digest` 由 canonical scope bytes
计算，raw scope 不具 authority。未映射字段、角色互换和任何权限 widening 一律拒绝。该映射由
四个机器附件共享同一 `source_path_mappings`、golden vector 与 CI 检查，禁止按字段名静默丢弃。

所有 mutation 的结果统一为公共 command result：`Accepted`、`AlreadyApplied`、`Conflict` 或
`StaleGeneration`。`Rejected` 仅是 envelope-level no-state-change 响应，必须附带 canonical
`FailureClass`，不是第五个业务状态。业务失败通过 canonical `FailureClass` 表达，不得用 HTTP
202/queued 伪造 terminal receipt。Gateway ingress credential（`kid, tenant, aud, scope, model,
iat/nbf/exp, jti, nonce, policy_digest`）与 adapter 看到的 upstream `Authorization` 分离；后者
永不进入 gateway wire、日志、receipt 或 agent projection。TaskFlow lineage（`run_id,
step_id, attempt_id`）必须保留。

#### 11.6 quota、metering、HNL、gateway 和 market 的实现归属

- quota 使用 `QuotaVector=(rpm,tpm,concurrency,day_budget,context)`，按 provider/account/org/
  IP/model/endpoint 逐域原子 hold；先算 `available_before_margin = limit - used - held`，再要求
  `estimate + safety_margin <= available_before_margin`（安全余量只计算一次），未知值
  conservative。调度器由 per-resource semaphore、token bucket、WFQ/EDF、cooldown 和 deadline
  组成。`B4` 负责 oversell/duplicate permit/Jain fairness；`B5` 负责 fsync/RPO；`B6` 只负责
  legacy writer cutover；`B7` 负责 HNL partition/replay；`B8` 负责 stream cancel；`B9` 负责
  ledger conservation/double settlement。
- metering 的 estimate/hold/final/refund 使用整数最小单位和单 writer 事务；partial stream
  delta 按 `usage_id + operation_ref` 单调幂等，UsageReceipt 只有 digest、数量、unit、引用和
  timestamps，不含 prompt/token/header/private key。
- `UsageRightCounter` 的唯一 durable writer 是 market-adapter，字段为
  `max_uses/reserved_uses/consumed_uses/remaining_uses/counter_revision/counter_state_digest`。
  AuthBus 在 `AcquirePermit` 前以 typed CAS command 调用 `ReserveUse`，成功后只保存带 digest/fence
  的 `UseReservationRef`；verified consumption 走 `CommitUse`，确认无 effect 或 dispatch 前取消走
  `ReleaseUse`，unknown/partial 保持 reserved 直到 reconcile。`remaining_uses = max_uses -
  reserved_uses - consumed_uses`，三者由 market-adapter 在同一 revision/fence 事务更新；首次可验证
  consumption 才把一个 reserved use 原子移入 consumed，重复 `usage_id` 幂等。每个 counter event
  和 UsageReceipt 绑定前后 counter digest/revision，防止未签名 mutable counter 被替换。不同
  `usage_id` 不能绕过上限，cancel/no-effect 不永久烧掉 use；overspend=0 是 B9 回归目标。
- HNL 复用既有 NodeId/AgentId/ServiceId/TenantId/WorkspaceId、session、epoch、nonce、key_id、
  capability/intent digest；`ConnectIntent.initiator_node/remote_node` 分别映射
  `local_node_id/remote_node_id`，`AgentDelegation.parent_node_id` 映射 `issuer_node_id`，
  `NodeDescriptor.node_id` 仅映射 `descriptor_node_id`，避免把 descriptor 主体误当成交易主体。
  `expiry/epoch/policy_digest/record_seq/key_id/signature/transport_endpoint_id` 分别保留为
  `expires_at/epoch/policy_digest/record_seq/key_id/signature/endpoint_id`；raw scope 只产生
  `scope_digest`。跨节点只传签名 `Offer/UsageRight/DispatchPermit/UsageReceipt/Revoke`。
  `transferable=false`、depth=0、scope/audience/expiry 是默认；交易价格、escrow、nonce、争议
  由 market-adapter/walletd/TRNM 负责，AuthBus 不持余额。
- gateway 默认 loopback/显式 LAN allowlist，registry allowlist、SSRF/DNS rebinding/CONNECT
  防护、body/stream 上限、backpressure、cancel、tenant quota 和 retention/PII policy 必须先于
  public bind。模型输出不得铸造 key、grant、wallet asset 或 command authority。

#### 11.7 阶段、CI 和量化 DoD

本历史页的 DAG 仅作 decode summary；规范实现 DAG 为
`B0 provenance → B1 topology/profile/identity → B2 contracts → B3 adapter
→ B4 scheduler → B5 authbusd-WAL/reconcile → B6 legacy-migration → B7 HNL → B8 gateway →
B9 market-sandbox → B10 release-prep`。机器消费者必须读取
`AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map`，并验证
`stage_resolution.active_stage_ref`、DAG/phase 一致性和唯一 normative stage set。

实现回归目标（不是 release gate）固定归属：

| 阶段 | 目标 |
|---|---|
| B4 | oversell=0、duplicate active permit=0、Jain fairness≥0.95、starvation 有界 |
| B5 | fault-injection≥1000、fsync-complete RPO=0、dispatch unknown 只 lookup |
| B6 | cutover 后 legacy writer acceptance=0、projection 无 raw secret |
| B7 | partition 后 old permit acceptance=0、HNL replay reject |
| B8 | cancel latency≤1000ms、stream/backpressure 无泄漏 |
| B9 | double settlement=0、ledger conservation delta=0 |

CI 必须执行：严格 YAML duplicate-key/schema parse；公共 v1.4 status/error 与
`AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry/projection_contract_map` 及四个领域附件
`execution_closure_v1_3` 投影的共享投影（owner、namespace、base fence、alias、状态子集和生命周期
digest）一致性检查，允许各域保留明确声明的 domain-specific keys；legacy alias decode-only 检查；所有 mutation fence
字段检查；`DispatchAttemptStarted` fsync-before-call 和 recovery-no-blind-retry 测试；RPC/actor/
HNL 字段 crosswalk golden vectors；stage active-ref/metric ownership 检查；Basil compile/register/
runtime deny-list 与 raw-byte scan；property/fuzz、disk-full/OOM/clock-rollback/partition/reopen
矩阵；UsageRight counter `reserved+consumed<=max`、no-effect release、counter digest/revision
conservation；dispute wallet disposition conservation；locked build、SBOM、SPDX/REUSE 和
x86_64/aarch64（Mac 仅 development）矩阵。

本修订仍是 `PLANNING_ONLY / IMPLEMENTATION_BACKLOG`。E.41 的
`DEVELOPMENT / INTERNAL_TEST / RELEASE_PREP` 连续开发规则不变；provider provenance、外部
signer/KMS、物理 H4、公开结算和 H8/H9 production evidence 继续 `DEFERRED_PRE_RELEASE`，只在
明确 `FINAL_RELEASE` 时评估。`authority`、`production_caller`、`production_writer`、
`effect_authority`、`operator_acceptance`、`promotion`、`g5_allowed`、`execute_allowed` 均保持
`false`；本页不修改运行中的 AuthBus/OpenBao 或任何 effective/authority JSON。

本次 v1.2 实现修订的 provenance 单独记录在
`OpenClaw/hepta.authbus_amendment_receipt.v1.schema.json` 与
`OpenClaw/AUTHBUS-PLAN-AMENDMENT-RECEIPT-2026-08-26.json`；原始 E.42 交叉索引和
`AUTHBUS-PLAN-APPEND-RECEIPT-2026-08-26.json` 保持不可变。网关入口允许
`Authorization: Bearer <virtual-credential>` 仅用于凭据提取/验证；调用者不能指定上游
URL 或透传任意上游 header，真实上游授权由 allowlisted adapter 在边界后构造。

<a id="authbus11-execution-closure-v13"></a>
### AUTHBUS.11 Contract Crosswalk & Execution Closure v1.3（编码闭环；不新增 release gate）

本页是 AUTHBUS.11 v1.2 的实现闭环修订。v1.2、AUTHBUS.10 和 E.42 仍保留为
decode-only/historical provenance；新代码生成器、stage resolver 和 CI 只能读取本页、
`AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry` 及其 projection map，再读取四个窄合同
和四个领域附件的 `execution_closure_v1_3` 投影。本页只收敛实现合同，不改变 E.41，不把 provider、
外部 KMS/HSM、物理断电、公开结算或 operator acceptance 提前成开发阻断。

#### 11.8 唯一合同、编码和时间上下文

1. `codex-hepta-contracts.v1.4_e21`/E21 继续拥有 CommandEnvelope、EffectIntent、ActivityReceipt、
   EffectReceipt、FailureClass、IndeterminateReason、CapabilityId 和 ReconcileDecision。
   `hepta-auth-contracts.v1` 只拥有 AuthResource、ComputeResource、QuotaSnapshot、
   QuotaReservation、ResourceLease、UsagePermit、EffectAck、OperationRef、
   CapabilityAttenuation、PeerSession、ClockSnapshot 及其 domain-specific payload；不得复制
   公共枚举或闭合 E21 receipt。
   E21 active schema is pinned to
   `OpenClaw/hepta-vnext-qualification-2026-08-23/e21-contract-hardening-qualification-20260824/schema/canonical_contract_v1_4_e21.schema.json`
   with SHA-256 `0e712e91cd188a150b8349391d9043c4ca61cef4798a091f36cd46e675fd6955`;
   similarly named historical/runtime copies are decode-only and cannot be selected by codegen.
2. 每个领域只有一个 canonical state/error registry。四个 YAML 的 execution_closure_v1_3
   只能是从 registry 生成的投影；CI 必须检查状态、事件、错误和 terminal 标志的一一映射，
   禁止手工维护第二套状态机。
3. 序列化上下文必须显式声明：E21/public envelope、AuthBus canonical object、Basil transport、
   HNL COSE/CBOR、gateway JWT/COSE。每个上下文固定 version、domain label、UTF-8 字符串、
   整数单位、字段排序、未知字段处理、null/缺省规则和 golden vector。Basil protobuf
   只承载 length-delimited canonical bytes 与其 digest，不能替换 canonical preimage。
   AuthBus/HNL/gateway extension objects 使用
   `SHA-256(domain_label || 0x00 || version || 0x00 || canonical_bytes)`；E21 自身继续使用
   其已发布的 `sha256-canonical-json`/schema digest 规则。crosswalk 必须记录 serialization
   context，禁止把 AuthBus domain digest 冒充 E21 schema digest；domain label 也不得复用
   provider 或 wallet domain。
   所有 `reconciliation_id`, `refresh_operation_key`, `window_digest`, `usage_vector_digest`,
   `NoEffectProof` 和 capability digest 统一引用 `preimage_v1`：domain/version 以长度前缀编码，
   后续字段按固定顺序编码为 `(field_name_len,field_name,field_value_len,field_value)`，整数使用
   canonical unsigned big-endian，空值使用显式 tagged-null；禁止无长度的字符串拼接。每个跨语言
   实现必须通过同一组 golden vectors。
4. 时间字段不是全局别名：lease/resource 使用 RFC3339 UTC 加 monotonic snapshot/uncertainty；
   JWT `iat`/`nbf`/`exp` 使用 NumericDate；HNL expiry 使用既有 HNL 单位。`expires_at→exp`
   等别名只在声明的 serialization context 解码，编码输出必须使用该 context 的唯一名称。
5. 所有 mutation 继承公共字段
   `schema_version,command_id,run_id,aggregate_id,expected_revision,authority_epoch,owner_epoch,
   generation,fencing_token,logical_clock,causal_parent_event_id,payload_digest,policy_digest,
   resource_digest`；同一 idempotency key 换 payload digest 必须返回 Conflict 且不改状态。

#### 11.9 OAuth/SecretRef 闭环（不泄漏原始 token）

v1 选择在 `hepta-basil-adapter` 内实现 process-bound `RefreshWithSecretRef` 和
`RotateSecretRef`，由 authbusd 持有 TokenFamily writer；只有在代码审查证明 adapter
隔离不可行时，才拆成独立 `hepta-auth-refreshd`，并沿用同一合同。两种形态不能同时写。

请求固定包含 `schema_version,operation_id,refresh_operation_key,command_id,run_id,profile_id,
provider_id,token_family_id,secret_ref,expected_secret_revision,idempotency_key,payload_digest,
policy_digest,scope_digest,authority_epoch,owner_epoch,generation,fencing_token,logical_clock,
causal_parent_event_id,deadline_at,purpose_digest,audience`；其中
`refresh_operation_key` 必须在 provider call 前 durable 且非空。adapter 在边界内短暂解析
SecretRef 并调用 provider；响应只包含新 SecretRef、版本、operation key 和 digest，绝不返回
access/refresh 原文、Authorization header 或 provider response body。成功时
`access_secret_ref/refresh_secret_ref` 必须非空；invalid-grant、transient 或 indeterminate
时这些新 SecretRef 必须为空/禁止，operation key 仍必须可 lookup。原始字节只存在 adapter
进程内存，按 provider response 完成/失败、超时或取消立即 zeroize；不得写日志、WAL、
SQLite、wire、receipt 或 core dump。

`RefreshWithSecretRef`/`RotateSecretRef` 的 response required/nullable/conditional 字段唯一
来源是 canonical registry `AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry/contracts/oauth_secretref`；
`AUTHBUS_RESOURCE_QUOTA_METERING_CONTRACT_v1.yaml#/execution_closure_v1_3/opaque_secret_operations`
只是生成投影，可补充其明确拥有的 adapter conditional guard，但不得删改 canonical 字段、状态或含义。
`RefreshWithSecretRef` 的 required response 是
`schema_version,response_id,operation_id,provider_id,profile_id,token_family_id,outcome,
access_secret_ref,refresh_secret_ref,secret_revision,refresh_operation_key,provider_status,
response_digest,idempotency_key,payload_digest,expected_secret_revision,authority_epoch,owner_epoch,
generation,fencing_token`；`RotateSecretRef` 的 required response 是
`schema_version,response_id,operation_id,provider_id,profile_id,token_family_id,outcome,
new_refresh_secret_ref,secret_revision,refresh_operation_key,response_digest,idempotency_key,
payload_digest,expected_secret_revision,authority_epoch,owner_epoch,generation,fencing_token`。
各 outcome 再按附件的 conditional/non-null/forbidden 规则处理。`deadline_at` 是请求的唯一时间字段；`deadline` 仅为
decode-only 历史别名，不能由新编码器输出。该响应合同不得另造第二套 refresh schema。

refresh 状态和恢复规则固定为：

`IDLE → CLAIMED → IN_FLIGHT → SUCCEEDED | TRANSIENT_FAILURE | INDETERMINATE → RECONCILING
→ SUCCEEDED | TRANSIENT_FAILURE | BACKOFF | MANUAL_REQUIRED`；`INVALID_GRANT` 仅是分类事件，
不是 durable state；`TRANSIENT_FAILURE → BACKOFF`（预算未耗尽）或
`TRANSIENT_FAILURE → MANUAL_REQUIRED`（无安全重试）；`MANUAL_REQUIRED →
RECONCILING` 仅在新证据和当前 fence 到达时允许。StatusByOperationKey 的 verified lookup
结果固定映射为 `ROTATED→SUCCEEDED`（新 SecretRef/CAS）、`INVALID_GRANT→QUARANTINED`、
`TRANSIENT_FAILURE→BACKOFF`、`UNKNOWN→MANUAL_REQUIRED`；没有独立证据时不能关闭
`INDETERMINATE`。`SUCCEEDED` 与 `QUARANTINED` 是唯一 refresh terminal state。

- `CLAIMED` 使用 (provider_id,profile_id,token_family_id) singleflight key、CAS、claim TTL；
  claim TTL 到期可由新 owner 接管，但旧 fencing callback 必须被拒绝。
- `INVALID_GRANT` 只允许一次 token-family quarantine 和显式 re-authorize；不得循环刷新，也不得写入独立的 INVALID_GRANT 状态。
- 超时、连接重置或响应丢失进入 `INDETERMINATE`，必须按 refresh id/effect key 查询或
  provider reconciliation；不能盲重试或并行旋转。
- `MANUAL_REQUIRED`/`RECONCILE_BLOCKED` 是非终态 hold，不得被包装成成功、Failed、
  Reconciled、Release 或 Refund；证据到达后才由 TaskFlow-owned decision 关闭。
- legacy AuthProfile/ProviderSession 迁移只带 metadata + SecretRef；旧 raw 副本在 cutover
  后进入可验证 scrub 阶段，scrub 未完成不得声称迁移完成。

#### 11.10 E21 EffectReceipt、reconcile evidence 和 direct ACK

E21 EffectReceipt 的 required set 保持闭合：
`effect_id,effect_intent_id,decision_id,run_id,schema_digest,status,terminal,
reconciliation_required,idempotency_key,attempt,authority_scope,execution_scope,effect_authority,
production_authority,snapshot_digest,graph_digest,policy_digest,provider_id,reconciliation_id,
reconciles_receipt_id,external_state_digest,resolver_attempt,current_revision,cas`。

为使 queued、direct terminal 和 no-effect 场景均可编码，ID 规则固定如下：

- `reconciliation_id` 使用 `recon:v1:<sha256(preimage_v1(domain=hepta.auth.reconciliation.v1;
  effect_id; effect_intent_id; attempt; authority_epoch; owner_epoch; generation))>`；
  同一 attempt 重试复用该 ID，新的 attempt 或 owner epoch 生成新 ID；初始 receipt 也必须填值。
- 尚无 reconcile 的 receipt 使用 registry 定义的哈希 sentinel：
  `reconciles_receipt_id = receipt-ref:v1:na:<sha256(preimage_v1(domain=hepta.auth.receipt-ref.na.v1;
  effect_id; attempt; authority_epoch; owner_epoch; generation))>`；进入 reconcile 后改为实际
  decision receipt ref，旧值保留在 immutable history。初始 decision id 同样使用
  `decision-ref:v1:na:<sha256(preimage_v1(domain=hepta.auth.decision-ref.na.v1; effect_id; attempt; status; current_revision))>`。
- direct terminal ACK 的 `external_state_digest` 是 provider terminal ACK canonical bytes 的
  digest；`cancel_without_dispatch` 必须生成 `NoEffectProof`（effect id、attempt、dispatch
  marker digest、outbox sequence、owner/generation、observed clock）并把其 digest 写入该字段。
- `NoEffectProof` 唯一字段为
  `proof_id,proof_stage,reconciliation_id,provider_id,provider_namespace,effect_id,effect_intent_id,effect_key,
  attempt,idempotency_key,operation_ref,proof_kind,dispatch_attempted,dispatch_marker_state,
  dispatch_marker_digest,outbox_delivery_seq,writer_record_fsync_confirmed,writer_fsync_witness_digest,
  provider_query_receipt_digest,external_state_digest,evidence_digest,observed_at,verified_at,decided_at,
  taskflow_decision_id,taskflow_decision_digest,authority_epoch,owner_epoch,generation,fencing_token,
  payload_digest,current_revision,cas`；`writer_fsync_witness_digest` 必须解析到包含
  `source_owner,wal_segment,wal_offset,event_seq,commit_digest,directory_fsync,writer_boot_id,
  writer_generation,verified_at` 的持久 witness，单独的布尔标记无效。`operation_ref` 对
  `pre_dispatch_cancel`/`verified_not_found` 使用
  `operation-ref:v1:not-applicable:<provider_namespace>:<effect_id>:<effect_key>:<attempt>`；
  provider explicit no-effect 才可使用真实 operation ref，且必须绑定 namespace/effect_id/effect_key/
  payload_digest/idempotency_key/attempt。`pre_dispatch_cancel` 的 query digest 必须是 canonical
  no-query marker，`dispatch_marker_state=absent`，且 `outbox_delivery_seq` 必须绑定 TaskFlow
  enqueue 序列，不能伪造或留空。
  pre-dispatch cancellation 还必须由 TaskFlow 提交带当前 authority/owner/generation/fence 的
  `ReconcileDecision`; authbusd 只能提出 NoEffectProof，不能自行把 RESERVED 关闭为 Reconciled。
- `ReconcileEvidenceRef` 唯一字段为
  `reconciliation_id,effect_id,effect_intent_id,reconciles_receipt_id,idempotency_key,attempt,
  operation_ref,outcome,evidence_kind,evidence_digest,external_state_digest,
  provider_query_receipt_digest,verified_at,decided_at,resolver_attempt,authority_epoch,
  owner_epoch,generation,fencing_token,payload_digest,policy_digest,current_revision,cas`；
  `verified_at` 是证据被验证的时间，`decided_at` 是 TaskFlow 决策提交时间，二者不可互换。
  旧的单字段变体仅 decode；无来源的 `external_state_digest` 变体拒绝。
- `ManualRequired`、`ReconcileBlocked`、`DispatchUnknownRef` 均非 terminal；它们不能产生
  `Reconciled`，不能释放 quota/escrow，也不能进入 market terminal state。

Dispatch 状态允许两条合法路径：

`DispatchAttemptStarted → DispatchAcceptedRef → EffectAckRef`

或同步 adapter 返回时：

`DispatchAttemptStarted → DirectTerminalAck`

实现可以在同一 writer 事务中生成 synthetic DispatchAcceptedRef，再提交 terminal ACK，
但该 marker 只能是内部、同一 writer 事务、明确标记 synthetic，且绝不能向公共 wire 或
receipt 单独投影；公共 `DispatchAccepted` 永远非 terminal。所有 ACK 必须绑定
`effect_id,effect_intent_id,idempotency_key,attempt,effect_key,provider_operation_id,
provider_operation_id_sha256,payload_digest,authority_epoch,owner_epoch,generation,fencing_token`
并验证签名/audience/issuer/key_epoch；synthetic loopback 只能使用本地 marker+mode attestation，
external-attested 才可使用签名/key_id/issued_at/expires_at，且签名 preimage 必须覆盖
provider namespace、effect key、payload digest、audience 和 key epoch，不能以“provider contract 支持”省略证据 profile。

`StatusByEffectKey` 请求/响应冻结为：
`schema_version,request_id,provider_id,provider_namespace,effect_key,payload_digest,
idempotency_key,attempt,authority_epoch,owner_epoch,generation,fencing_token,audience,
  request_nonce,deadline_at,query_revision,retention_hint,expected_execution_mode,policy_digest` →
  `schema_version,response_id,provider_id,provider_namespace,effect_key,payload_digest,
  terminal_state,operation_ref,provider_operation_id,provider_operation_id_sha256,ack_digest,
  observed_at,status_revision,binding_digest,provider_query_receipt_digest,not_found_kind,evidence_profile,
  execution_mode,mode_attestation_digest,policy_digest,signature,key_id,issued_at,expires_at`。
  `not_found_kind ∈ {none,verified,unavailable,expired}`；非 `none` 时 `terminal_state` 必须为
  `UNKNOWN` 且 operation/ack refs 禁止，`verified` 只能经 NoEffectProof 关闭，`unavailable/expired`
  永远保持 Indeterminate。`evidence_profile`、execution_mode、mode_attestation_digest 和 policy_digest
  必须由本地 mode/policy registry 验证，不能接受远端自报；external-attested 的签名必须覆盖
  audience、key_epoch、provider namespace、effect key 和 payload digest。
effect-key collision、provider namespace 不匹配、过期签名或 unknown status 一律
`Indeterminate/ManualRequired`；不得新建 attempt。

#### 11.11 WAL、outbox 和跨 owner 交付

authbusd 是每 Host 的 AuthResource、Quota、Lease、Permit、reservation 和 dispatch-reference
唯一 durable writer；TaskFlow DB 是 EffectIntent、EffectReceipt、ReconcileDecision 唯一 writer。
两者不是跨数据库事务，而是各自本地原子提交加 at-least-once typed bridge：

`local_commit + outbox → deliver(attempt,lease) → ack_digest/dedupe → cursor`

outbox 必须使用 `hepta.auth.outbox-delivery.v1` 的完整字段：
`schema_version,outbox_id,source_owner,destination,event_type,event_id,aggregate_id,command_id,
causal_parent_event_id,delivery_seq,delivery_attempt,provider_attempt,resolver_attempt,payload_ref,
payload_encoding,payload_digest,source_record_digest,idempotency_key,lease_owner,lease_expires_at,
next_retry_at,ack_digest,created_at,updated_at,status,authority_epoch,owner_epoch,generation,
fencing_token,writer_record_fsync_confirmed,writer_fsync_witness_digest,dead_letter_reason`，并允许
nullable `lease_owner,lease_expires_at,next_retry_at,ack_digest,dead_letter_reason`。同一 bridge delivery
重放必须按 `(idempotency_key,payload_digest)` 幂等；
不同 digest 冲突即停。bridge 交付重试可复用同一 key，provider effect 在 DispatchUnknown 后
禁止新 dispatch，reconcile resolver 只能按有限预算 lookup/retry。磁盘满、WAL corruption、OOM、
clock rollback、owner transfer 和 partition 的 crash matrix 要覆盖每个 marker 窗口。

dispatch 的 durable 顺序固定为：
`EffectIntentDurable → DispatchAttemptStarted(fsync) → adapter call →
DispatchAcceptedRef|DirectTerminalAckRef|DispatchUnknownRef(fsync) → EffectAckRef|IndeterminateRef →
TaskFlow receipt → lookup-only Reconcile → Release|Refund`。
调用后崩溃只能生成 DispatchUnknownRef；恢复不得盲重发。authbusd 的 ReconcileQueue 只是
lookup worklist/projection，TaskFlow DB 仍拥有 canonical reconcile queue/decision 和唤醒；
历史 v1.2 中“ReconcileQueue 由 authbusd 持有”的行仅供 decode，主动实现不得采用，不得形成第二 scheduler。

#### 11.12 Basil fork 的真实最小 profile、key generation 和工具链

当前 Basil 源码没有按服务组的 Cargo feature；`basil-core` service modules、proto descriptor
和 `basil-bin` 默认 workspace 成员会编译/注册 raw Secret、NewKey、Admin、NATS、SDS、SPIFFE、
remote invocation 等面。故 B1 必须提交可审计 fork patch inventory，而不能只写
`--no-default-features`：

1. 从 workspace members/exclude 和构建 target 中排除 basil-bin、basil-nats、basil-nats-bridge、
   basil-https-courier 及其默认 keystore/unlock/keygen 依赖；
2. 删除/重生成 proto service/descriptor 中的 Get/Set/Import/List/NewKey、private-key return、
   unscoped mint、Admin/NATS/SDS/SPIFFE/Invocation 和未知 future RPC，并记录 descriptor digest；
3. registration allowlist 与 runtime default-deny 的 canonical route set 只保留完整限定 gRPC path
   `/basil.broker.v1.SigningService/Sign`, `/basil.broker.v1.SigningService/Verify`,
   `/basil.broker.v1.SigningService/GetPublicKey`, `/basil.broker.v1.AdminService/Health`
   及 Hepta process-bound capability；`/basil.broker.v1.AeadService/Decrypt` 仅显式
   process-bound feature；`/basil.broker.v1.SigningService/NewKey`, `/basil.broker.v1.SigningService/Import`,
   `/basil.broker.v1.SigningService/ImportSet`, `/basil.broker.v1.SecretService/GetSecret`,
   `/basil.broker.v1.SecretService/SetSecret`, `/basil.broker.v1.SecretService/RotateSecret`,
   `/basil.broker.v1.SecretService/ListCatalog`, `/basil.broker.v1.AeadService/Encrypt`,
   `/basil.broker.v1.AeadService/WrapEnvelope`, `/basil.broker.v1.AeadService/UnwrapEnvelope`,
   `/basil.broker.v1.AeadService/UnsealCose`, all `/basil.broker.v1.MintingService/*`,
   `/basil.broker.v1.NatsService/*`, `/basil.broker.v1.NixCacheService/*`,
   `/SpiffeWorkloadAPI/*`, `/envoy.service.secret.v3.SecretDiscoveryService/*`,
   and `/basil.broker.v1.AdminService/*` (Health excluded) must be rejected at descriptor,
   registration and runtime layers;
   抽象别名仅 decode-only；
4. `LocalIdentity::open_or_create`、`missing=generate` 和 keystore generate path 改为
   `key_generation=forbidden`：没有外部登记的 key ref 必须 fail-closed，启动不得自动生成
   软件 P-256 key；加入启动、RPC、raw-byte 和 binary-symbol negative tests；
5. B0 receipt 记录确切 Basil tree/archive/Cargo.lock/rust-toolchain、rustc/cargo/libprotoc、
   cargo metadata、SBOM/SPDX/REUSE 工具版本和命令 digest。当前 upstream 要求 Rust 1.98，
   现场观察 1.94 只能算 research capture，不能冒充 native build pass。

#### 11.13 quota、远程 reservation、HNL、gateway 和 market

- QuotaSnapshot 必须带 `observation_id,resource_id,domain_id,window_id,window_kind,
  window_start,window_end,source_seq,observed_at,source,confidence,component_states,
  limit_vector,used_vector,held_vector,remaining_vector,reset_at,revision,authority_epoch,
  owner_epoch,generation,fencing_token,stale_after_ms,clock_uncertainty_ms`；每个向量维度都
  明示 KNOWN(value,unit) 或 UNKNOWN(reason,last_known_digest)，缺失不等于无限。source_seq
  在 (resource,domain,window,source) 内单调，过期/乱序 observation 不能覆盖新容量，
  429 Retry-After 只是 cooldown 下界；窗口 rollover 必须生成新 window_id，禁止跨窗口猜测抵扣。
- 每个 provider domain 的不变量为
  `sum(local_held + remote_held) + consumed <= limit`。seller owner 串行处理
  `ReserveRemote → signed DispatchPermit → counter commit → ACK`；分区期间不接新 reservation，
  unknown hold 到 reconcile，旧 permit 永不恢复。
  `ReserveRemote` 的 command、signed usage lease 和 durable reservation 必须共同携带
  `window_id,window_kind,window_start,window_end,reset_at,source_seq,window_digest`；
  `window_digest=SHA-256(preimage_v1(domain=hepta.auth.quota-window.v1; window_id; window_kind;
  window_start; window_end; reset_at; source_seq))`，
  窗口轮换视为新 allocation，不能复用旧 idempotency key。
- `UsageVector=(request_count,rpm,tpm,concurrency,day_budget,context)`，每一项声明
  unit、估算、最终量和 rounding。若市场 v1 只交易 request-count，必须在产品文案和
  contract 中明确禁止宣称 TPM/算力额度；只有在其它维度为零且显式声明
  `request_count_only` 时，才允许把 legacy scalar `max_uses` 投影到
  `UsageVector.request_count`，禁止静默映射成向量。
- B7 及 federated B8/B9/B10 必须显式依赖 `HNL-GATE0-DECISIONS`，记录 source digest/status；
  Gate 0 未冻结时 federation 只能 fail-closed loopback；local B8/B9/B10 可在 B4/B5/B6
  依赖满足后用 synthetic/loopback 继续。复用既有 HNL trust root 和 Node/Agent/Service/
  Tenant/Workspace 字段，不创建第二 root；NodeDescriptor、AgentDelegation、ConnectIntent、
  RevocationRecord 的字段映射必须 lossless（含 endpoint_candidates、allowed_relays、capabilities、
  delegation/parent/recipient、tenant/workspace/agent/service、epoch/revocation_seq/reason/key/signature）；
  Linux pidfd/starttime/launch nonce 与 macOS audit-token vector 必须各有 golden test。
- gateway 冻结 virtual credential 的 COSE/JWT 算法、issuer/JWKS/rotation、PoP、TTL、
  replay/revocation、clock skew，以及 Responses/Chat Completions、SSE、cancel、error、
  idempotency 的确切子集。`IssueVirtualCredential` 不能自授权；ingress credential 与
  adapter 构造的 upstream Authorization 永远分离，SSRF/CONNECT/DNS-rebinding 默认拒绝。
  credential claims 必须绑定 `tenant,workspace,agent,service,node,agent_generation,authority_epoch,
  owner_epoch,generation,fencing_token,resource,provider,permit,payload_digest,aud,jti,cnf`；
  remote 请求强制 PoP，replay key 为 `(iss,kid,jti,resource_id,permit_id,payload_digest,nonce)`，
  gateway 只能验证/请求 authbusd 发行的短期 credential，不能自行发行或升权。
- raw token 永不 transferable；只有 owner/provider consent 的 bounded delegated usage-right
  可在显式 mode 下交易，带 depth、recipient、scope、expiry、provider_terms_digest。价格、
  currency、fee、quote expiry、escrow timeout、partial settlement、refund conservation、
  dispute/appeal SLA 必须是整数和可审计事件；AuthBus 不持 wallet ledger。
  market lifecycle 的 ACTIVE/PARTIALLY_CONSUMED expiry/cancel/refund 只能先进入 DISPUTED；
  terminal market 状态必须有唯一 wallet disposition join 与 conservation proof，wallet RELEASED
  不得隐式投影为 SETTLED/REFUNDED。v1 scalar `max_uses` 只能在显式 request-count-only 模式
  映射到 UsageVector.request_count。

#### 11.14 B0–B10 实施闭环和可量化 DoD

规范 DAG 的全局顺序保持 `B0 provenance → B1 topology/profile/identity → B2 contracts →
B3 adapter → B4 scheduler → B5 authbusd-WAL/reconcile → B6 legacy-migration → B7 HNL →
B8 gateway → B9 market-sandbox → B10 release-prep`，但 B8/B9/B10 由 stage resolver 分为
`local` 与 `federated` lane：local B8/B9/B10 只需本地 synthetic/loopback 依赖，federated
B7/B8/B9/B10 才要求 HNL-GATE0-DECISIONS；不得用聚合 status 阻塞已就绪的 local lane。
每阶段的 normative source 只能由 `AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map`
解析，历史 overlay 与 v1.2 crosswalk 一律 decode-only。

| 阶段 | 必须通过的实现目标 |
|---|---|
| B0 | source/toolchain/license/SBOM receipt 完整；研究 capture 与 imported fork 分离 |
| B1 | Basil patch inventory、descriptor/binary/raw-byte deny、keygen-forbidden 启动测试 |
| B2 | canonical registry、serialization vectors、E21 required set、状态/错误生成投影 |
| B3 | SecretRef refresh/rotation、401/invalid_grant、direct ACK、StatusByEffectKey、fail-closed |
| B4 | oversell=0、duplicate active permit=0、Jain fairness≥0.95（固定 workload/weights/seed）、p95 admission target、starvation 有界 |
| B5 | fault injection≥1000、fsync-complete RPO=0、unknown lookup-only、outbox dedupe |
| B6 | single-writer cutover、legacy writer acceptance=0、raw scrub receipt、rollback/callback fence |
| B7 | HNL Gate0 dependency、partition/replay old-permit acceptance=0、identity vectors |
| B8 | cancel≤1000ms、stream/backpressure/SSRF/replay negative matrix |
| B9 | UsageVector/counter conservation、double-settlement=0、dispute/appeal/timeout conservation |
| B10 | reproducible build/canary/rollback and FINAL_RELEASE handoff；不自动 promotion |

CI 必须运行 duplicate-key/schema 检查、canonical-byte/golden vectors、crosswalk generation
diff、historical enum rejection、ManualRequired non-terminal guard、direct ACK/no-effect proof、
DispatchAttemptStarted fsync-before-call、lookup-only recovery、refresh claim takeover、
quota stale observation/remote reservation、HNL Gate0 dependency、gateway SSRF/PoP/replay、
market counter/escrow conservation、disk-full/OOM/clock rollback/partition/reopen、locked build、
SBOM/license、x86_64/aarch64（macOS 为 development-only）矩阵。每个 validation boolean
必须绑定脚本路径、命令 digest、fixture digest 和 timestamp，不能只手写 PASS。

本页仍为 `PLANNING_ONLY / IMPLEMENTATION_BACKLOG`。E.41 的开发连续性和所有 negative
authority flags 保持原值；provider/KMS/HSM、物理 H4、公开结算、operator/legal acceptance
继续 `DEFERRED_PRE_RELEASE`，只有显式 `FINAL_RELEASE` 才评估。运行中的 AuthBus/OpenBao、
effective/authority JSON、production/effect/promotion flags 未在本修订中改变。

<a id="authbus11-artifact-closure-v13"></a>
### AUTHBUS.11 v1.3 artifact closure（canonical registry + generated projections）

本附录把上面的实现规则绑定到实际文件，解决“文字规范”和“生成输入”分离的问题。唯一
可生成 source 是：

`OpenClaw/AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry`

它的 `status_error_registry`、E21 required set、序列化上下文、identity fence 和四个领域
合同是唯一语义来源。以下四个窄合同是可审查的 generated projection，不能独立改写语义，
四个既有大附件（trust/quota/failure/abuse）只保留为领域投影：

| 领域 | canonical contract | projection |
|---|---|---|
| OAuth/SecretRef | `hepta.auth.opaque-secret-operation.v1` | `AUTHBUS_OAUTH_SECRETREF_CONTRACT_v1.yaml#/contract` |
| E21 reconcile | `hepta.auth.reconcile-evidence.v1` | `AUTHBUS_RECONCILE_E21_CONTRACT_v1.yaml#/contract` |
| outbox/dispatch | `hepta.auth.outbox-delivery.v1` | `AUTHBUS_OUTBOX_DISPATCH_CONTRACT_v1.yaml#/contract` |
| remote reservation + gateway | `hepta.auth.remote-reservation.v1` + `hepta.gateway.credential.v1` | `AUTHBUS_REMOTE_RESERVATION_GATEWAY_CONTRACT_v1.yaml#/contract` |

projection 只能做 root metadata wrapper、领域分组和 decode-only alias；不得删字段、换状态
含义、替换 UsageVector 维度或把 `operation_id` 改名为 `provider_operation_id`。B2 生成器
必须先解析 registry，再验证每个 projection 的 `source_registry_ref`/digest，任何 stale
或未绑定 projection 都 fail closed。

#### 运行前必须冻结的编码闭环

1. OAuth 只接受 opaque `SecretRef`。`RefreshWithSecretRef`/`RotateSecretRef` 的 durable
   outcome 仅为 `SUCCEEDED|TRANSIENT_FAILURE|INDETERMINATE|QUARANTINED`；`INVALID_GRANT`
   是错误分类，不是状态。原始 token 只在 adapter 内存中短暂存在，绝不进入 wire/WAL/SQLite/
   receipt/log。响应丢失必须按 operation key lookup，不能盲重试。
2. E21 receipt 的 24 个 required fields 闭合；初始 receipt 与 decision 使用 registry 的
   hashed `receipt-ref:v1:na`/`decision-ref:v1:na` sentinel，不能把 effect id 直接插值成未
   绑定字符串。`NoEffectProof` 分为 `PROPOSED|COMMITTED`：authbusd 只能提议，TaskFlow
   带当前 fence 提交后才允许 release/refund；`ManualRequired`、`ReconcileBlocked`、
   `DispatchUnknownRef` 永远不是 terminal。
3. outbox 的唯一 writer 仍按 owner 分开：authbusd 只写 AuthResource/quota/lease/permit
   和 dispatch reference，TaskFlow 只写 EffectIntent/EffectReceipt/ReconcileDecision。
   `EffectIntentDurable → DispatchAttemptStarted(fsync) → AdapterCall →
   DispatchAcceptedRef|DirectTerminalAckRef|DispatchUnknownRef`；unknown 恢复只能
   `StatusByEffectKey` lookup。bridge 是 at-least-once，按 `(idempotency_key,payload_digest)`
   去重，冲突停机，不产生第二 scheduler。
4. Basil profile 的 route 集合以 pinned descriptor 为准：允许仅
   `/basil.broker.v1.SigningService/{Sign,Verify,GetPublicKey}`、
   `/basil.broker.v1.AdminService/Health`，`AeadService/Decrypt` 仅显式 process-bound
   feature；拒绝真实存在的 Signing import/new-key、Secret get/set/rotate/ListCatalog、
   Invocation（含 challenge/capabilities）、Minting、Admin 非 Health（含
   ListConnections/DropConnections）、NATS、NixCache、`/SpiffeWorkloadAPI/*`、
   `/envoy.service.secret.v3.SecretDiscoveryService/*` 和未知 RPC。B1 必须在 compile、
   registration、runtime 三层裁剪，并禁止 `LocalIdentity.open_or_create`、
   `missing=generate` 和所有 keystore 自动 key generation；缺少已登记 key 就 fail closed。
5. `UsageVector` 固定为 `request_count,rpm,tpm,concurrency,day_budget,context` 六维；provider
   token/compute/bytes 只能是 context extension。市场 v1 若只交易 request-count，必须显式
   `request_count_only`，不得宣传为 TPM/算力额度。remote window 绑定
   `window_id/window_kind/start/end/reset/source_seq/window_digest`，跨节点不变量逐维保持
   `local_held + remote_held + consumed <= limit`。

#### stage、source 和外部依赖选择器

执行器只读 `AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map` 与
`AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry/projection_contract_map`；旧 `stages`、
`implementation_overlay`、各附件顶层 `contract_crosswalk_v1_3` 和 v1.2 crosswalk 仅
decode。active stage refs 必须直接指向上表 registry/窄合同或四个领域附件的
`execution_closure_v1_3`；B8/B9/B10 的 local lane 可用
synthetic/loopback，federated lane 继续要求 `HNL-GATE0-DECISIONS`，其 stale receipt 只会
产生 `NOT_READY_FAIL_CLOSED`。本附录的执行闭环记录在
`OpenClaw/AUTHBUS-PLAN-AMENDMENT-RECEIPT-2026-08-26-v1.3.json`；receipt 与 E.44 索引
绑定的文件集合不含自身和索引，避免哈希环。

上述内容是实现输入和静态 CI 规则，不是行为测试通过、生产 authority 或 release approval。
E.41 不变：开发、内测和 release-prep 不被 provider/KMS/HSM、物理断电或公开结算证据
阻塞；这些只在明确 `FINAL_RELEASE` 时成为发布门。
