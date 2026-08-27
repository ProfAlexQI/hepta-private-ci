## INF-PLAN-2026-08-26 — hepta-infer-core / hepta-inferd 本地推理执行层（plan-only append）

> 状态：`PLANNING_ONLY / STALE_SOURCE_BINDING`
> 日期：2026-08-26（Asia/Shanghai）
> 计划类型：独立本地推理执行 lane；不改写既有 Neuron 语义合同，不授予模型、NPU、生产或 promotion 权限

## -1. 计划定位与源绑定

这份文档把 Hepta 的本地推理设计从“模型/语义评测合同”细化为可执行的 runtime 计划。
现有 `HEPTA_NEURON_RUNTIME_CONTRACT_V1_4.md` 继续是语义、隐私、校准、信号和证据合同；
`HEPTA_NEURON_MODEL_EVAL_MATRIX.md` 与 `HEPTA_RESOURCE_EVAL_MATRIX.md` 继续是候选模型和资源评测矩阵。
本计划只增加 `hepta-infer-core`、`hepta-inferd`、native worker、性能准入、调度和 receipt 层，
不覆盖或改写上述文件。

写入前重新读取的 canonical 主计划快照：

```yaml
plan_id: MIR-hepta-inference-runtime
plan_version: 0.1.0
parent_plan_id: hepta-vnext-development-plan-final-2026-08-23
parent_plan_digest: ba5008aa46b2f4e2ee3325deee197aa99d6997039bd5659db67f400c0c72c417
insert_after: "HNL.6 / EOF line 3479 (pre-append snapshot)"
documented_canonical_main_head: 7ed9c9a85fa65aa3cb26cf440a55028ce0b35079
canonical_main_tree: 7d4306273861564a62fa9614860bdc6239a065d0
canonical_main_dirty: true
implementation_branch: not-created
implementation_worktree: not-created
active_experimental_branch: hepta/integration-upstream-20260824
active_experimental_head: 54c733764be7c24ef928cfc689e09515273ff900
active_experimental_tree: ff902e217d857c3040516557305d0b93382afeae
active_experimental_dirty: true
binding_observed_at: 2026-08-26T02:32:40+08:00
binding_observed_via: ssh mac-ts read-only git inspection
parent_source_status: STALE_SOURCE_BINDING
mode: qualification_only
production_listener: false
production_writer: false
provider_effect: false
shared_kg_write: false
routing_write: false
fleet_write: false
model_npu: false
remote_inference: false
CALLERS_touched: false
operator_acceptance: false
promotion: false
```

`canonical_main_dirty=true` 是当前主集成树存在未纳入计划的临时文件；实验树也有未提交改动。
因此任何父 digest、head、tree、dirty、branch 或 worktree 变化都自动令本计划
`STALE_SOURCE_BINDING`，实现者必须停止并重新生成 binding receipt。附件中的命令均为 planned
commands；对应 crate、fixture 或硬件尚未存在时必须写 `NOT_TESTED`，不得伪造 PASS。

## 0. 速度优先的总决策

Rust 负责 Hepta 的常驻整体：模型注册、硬件探测、ModelRouter、权限、配额、调度、缓存、取消、
多 Agent 隔离、观测和 `ModelReceipt`。高速 kernel 不用纯 Rust 重写，而由锁定版本的 C/C++、GPU/NPU
runtime 执行。这样既保持 Hepta 的版本和安全边界，又不牺牲 Metal、CUDA、OpenVINO、厂商 NPU
等优化路径。

```text
Hepta agents / agentd
          │ versioned UDS RPC
hepta-infer-core (Rust facade + policy + router + receipt)
          │ one sidecar per physical host
hepta-inferd (Rust resident scheduler / model pool / quota broker)
          ├─ llama.cpp worker (LLM/VLM default)
          ├─ whisper.cpp worker (ASR default)
          ├─ sherpa-onnx worker (TTS/VAD/diarization/audio extension)
          ├─ stable-diffusion.cpp worker (image; locked commit)
          ├─ Core ML / OpenVINO / MNN / vendor NPU workers
          └─ optional mistral.rs / Candle (measured experimental backends)
```

每台设备只运行一个受 `agentd`/fleet supervisor 管理的 `hepta-inferd`；不能每个 Agent 各自启动
推理 daemon，也不能在推理层另造 workflow、fleet scheduler、Memory/KG writer 或 effect executor。
大模型和不稳定 native backend 初期必须是隔离 worker 进程；小模型只有在 ABI 和性能 qualification
通过后才可申请 in-process fast path。

### 0.1 现有合同的权威关系

- `RunStartSnapshot`、`ArtifactSetDigest`、现有 `ModelReceipt` 和 `NeuronSignal` 仍是 Hepta 的事实入口。
- 推理 runtime 只能输出 typed signal、不可变执行 receipt 和受限 usage event；执行 receipt 通过 `model_receipt_ref` 关联既有语义 `ModelReceipt`，不替换或扩展其权威定义；不能直接写 Memory、KG、effect、
  capability、routing 或 fleet。
- `HEPTA_RESOURCE_EVAL_MATRIX` 中历史性的 `NPU → local CPU/GPU → remote → deterministic` 说明
 不是可执行的全局 fallback。执行只服从 `guard_deterministic`、`privacy_local`、`proposal_local` profile。
- 未测 tuple、隐私不允许的远程、资源超限和未知后端统一 `NotAdmitted`；禁止静默切换。

## 1. 目标与非目标

### 1.1 目标

1. 在不依赖 Ollama、HTTP/JSON 热路径或外部推理服务的条件下，先提供 Hepta 内置本地文本与转写能力，再按独立性能门槛扩展音频与图像能力。
2. 用同一 Rust facade 管理多 Agent 的模型准入、配额、取消、缓存、receipt 和隔离。
3. 以实测的“模型 × tokenizer × 量化 × backend × 编译/驱动 × 设备 × context/batch”合同保证速度。
4. 在 Mac、RTX、Intel、小型 ARM/Pocket4 profile 上允许不同 backend，不强求一个 runtime 覆盖所有设备。
5. 保持模型、tokenizer、编译产物、SBOM、许可证和回滚 digest 可验证。

### 1.2 非目标

- P0 不实现视频生成；视频先做抽帧/理解实验，生成另开 future lane。
- P0 不解冻真实 NPU、远程推理、生产模型安装、模型自动下载或跨设备模型分发。
- P0 不把 mistral.rs、Candle、EasyTier、Tailscale 或任何单一 runtime 作为 Hepta 信任根。
- 不把模型输出直接升级为 Memory/KG/effect/capability；仍需现有 typed admission 和 authority。
- 不用“语言是 Rust”替代硬件 benchmark；未实测性能不得进入默认 ModelRouter。

## 2. Backend capability matrix（规划基线）

| 任务 | P0 默认 | 对照/专用 | 约束 |
|---|---|---|---|
| LLM/VLM | `llama.cpp` C API | `mistral.rs`、厂商 backend | 按模型/设备 A/B；GGUF/量化组合需单独 receipt |
| ASR/VAD | `whisper.cpp` | `sherpa-onnx` | ASR 记录 RTF、首片延迟、分块和 VAD 行为 |
| TTS/说话人/分离 | `sherpa-onnx` | audio.cpp 等实验 | 不把快速演进 API 直接暴露给业务层 |
| 生图 | `stable-diffusion.cpp` 锁定 commit | 专用 vendor runtime | INF-5 受控 P1；模型权重许可证单独审查 |
| Intel CPU/GPU/NPU | OpenVINO GenAI | llama.cpp/ONNX Runtime | 只有设备 receipt 证明实际硬件路径 |
| ARM/Android/Pocket4 | 按 SoC 比较 llama.cpp、MNN、vendor NPU | Candle/custom | Pocket4 profile 未确认前只能 `KNOWN_GAP` |
| 视频生成 | 不进入 P0 | 独立 future lane | 不阻塞 service-stream/文本/音频 DoD |

所有 native backend 必须锁 commit/tag、编译器和驱动矩阵；上游 API 通过 Hepta 自己的 facade/ABI
隔离。模型运行时许可证与权重/数据许可证分开记录。

## 3. PerformanceContract（ModelRouter 的硬准入键）

每一条允许进入 `RunStartSnapshot` 的性能合同必须绑定以下完整 tuple：

```yaml
model_digest: sha256
tokenizer_digest: sha256
backend_id: string
backend_commit: git-or-release-digest
compiled_artifact_digest: sha256
compiler: string
driver_runtime: string
device_profile: string
quantization: string
context_tokens: integer
batch_size: integer
kv_cache_policy: string
prefix_cache_policy: string
input_output_shape: canonical descriptor
```

准入规则：

1. tuple 有独立 benchmark receipt、artifact digest、资源读数和结果校验，才可 `Admitted`。
2. 任一字段缺失、过期、跨设备复用或与当前 policy digest 不一致，返回 `NotAdmitted`。
3. 失败组合记 `KNOWN_GAP` 并从默认路由移除；不能静默切 CPU、远程、另一量化或未测试 backend。
4. direct-native 基线必须与 Hepta 使用相同模型、artifact、设备、context、batch 和 warm/cold 条件。
5. 初始相对门槛（INF-0 前为 provisional）：warm p95 end-to-end overhead ≤ `max(10 ms, 10%)`，
   warm throughput ≥ direct-native 的 90%；ASR RTF、图像延迟、峰值资源同样不得超过 direct-native 的 1.1x。
   实机数据完成后才冻结为正式门槛。

## 4. Rust facade、worker ABI 与热路径

### 4.1 稳定业务接口

```text
start(NodeConfig) -> InferHandle
register_model(ModelManifest) -> ModelLease
inspect_capability(device_profile, task) -> CapabilityReport
submit(InferRequest) -> InferStream
cancel(request_id, generation) -> CancelReceipt
revoke(subject, epoch)
stats() -> InferStats
events() -> InferEventStream
```

业务代码不能出现 `llama_context`、`OrtSession`、Core ML 对象或任意上游结构体；所有错误、取消、
流式 token、媒体 descriptor 和资源结果使用 Hepta 自己的版本化类型。

### 4.2 `hepta_backend_v1`（计划中的 opaque C ABI）

- 固定 ABI version、endianness、alignment、allocator ownership 和 opaque handle 生命周期。
- capability query 返回 backend/device/model digest、支持的 quant/context/batch、stream/cancel 能力。
- request/result/error 使用长度显式的 byte slice；worker 不拥有 Rust heap 指针。
- 统一 `load / warm / submit / poll / cancel / drain / unload / health` 函数表，结果必须带 generation。
- backend 只能返回计算结果和资源事实，不能创建 Hepta authority、写共享状态或选择 remote endpoint。

### 4.3 进程和数据边界

- 控制面：版本化 UDS + canonical CBOR；身份由 Unix credential、Windows SID 或 macOS audit token 校验。
- 大文本、音频、图像、视频：shared memory/memfd descriptor + `fd/offset/length/digest/owner/lifetime`；发送端使用 memfd seal，接收端做 bounds、digest、owner 和 TOCTOU 校验；禁止 Base64、HTTP/JSON 进入热路径。
- native worker 默认独立进程，崩溃、OOM、死锁或 ABI 不兼容只熔断该 backend；不得拖垮 agentd。worker 崩溃、OOM、thermal trip、取消或 generation fence 必须释放 reservation、使旧 cache/receipt 失效，并发出终态事件；旧 worker 不得继续发布结果。
- 观测只记录 digest、shape、size、延迟和资源，不记录原始 prompt、音频、图像或隐私内容。

## 5. Multi-Agent 隔离、配额与调度

每个 request 必须绑定：`TenantId`、`WorkspaceId`、`AgentId`、`agent_generation`、`task_id`、
`policy_digest`、`resource_budget_id`、`deadline`、`cancel_generation` 和 `model_tuple_digest`。

一个 `hepta-inferd` 管理多个 Agent；Agent 无权自行选择 backend、设备、远程 endpoint、模型路径、
AllowedIPs 或 route。默认配额包括：并发 stream、队列长度、CPU/GPU/NPU 毫秒、RSS/VRAM、context token、
图像像素、扩散步数、音频/视频秒数和 deadline。

调度状态机：

```text
Admit → Reserved → Queued → Loading/Warm → Running → Draining → Completed
                  └──────────────→ Rejected/NotAdmitted
Running ──cancel/generation fence──→ Cancelled
Running ──OOM/thermal/worker crash──→ FailedClosed
```

- 交互请求使用 deadline/EDF；后台请求使用 weighted-fair queue；任何 Agent 不能饥饿其他租户。
- LLM 使用 capability-gated continuous/micro-batching 与 tenant-scoped KV/prefix cache。
- KV/prefix cache 只能在 `TenantId/WorkspaceId/model_tuple_digest/policy_digest` 全部匹配时复用；默认跨租户、跨 workspace 禁止共享。
- ASR 使用 bounded chunk + VAD；扩散图像使用独立队列，避免阻塞 token 流。
- cancellation 必须有 generation fence；worker 重启后旧 request、旧 cache 和旧 receipt 全部失效。
- OOM、温度、功耗或 backpressure 超限时释放 reservation 并返回可解释错误，不隐式换后端。

## 6. ModelRegistry 与供应链

`ModelManifest` 至少包含：格式（GGUF/safetensors/ONNX 等 allowlist）、model/tokenizer digest、
quantization、compiled artifact digest、backend commit、compiler/driver、device profile、许可证、
SBOM、签名、来源、校准/安全 schema、回滚 digest 和允许的 policy profile。

Core ML/厂商 NPU 转换成功不等于真实走 ANE/NPU；只有设备 receipt 的 hardware path、driver、温度和功耗
证据能使该 tuple 获得 `Admitted`。模型下载、转换和安装均是独立受控动作，不由普通 Agent 或推理请求隐式触发。

## 7. Receipt、指标与验收

附件 `hepta.infer_receipt.v1.schema.json` 定义统一 receipt。必须记录：

- source：repo、worktree、branch、head/tree/dirty、parent plan digest；
- inputs：model/tokenizer/backend/compiler/driver/device/quant/context/batch/artifact/SBOM digest；
- latency：queue、load、TTFT、prefill/decode tok/s、ASR RTF、image seconds/step、total p50/p95；
- resource：RSS、VRAM、CPU/GPU/NPU、功耗、温度、thermal throttle、线程和 FD；
- behavior：fallback attempts、cancel/restart generation、result digest、error code、cache hit；
- authority：所有生产、effect、shared-KG、route、fleet、model-NPU、remote、CALLERS、promotion flags。

raw prompt/media 永不写入 receipt，只写 digest、size、shape 和脱敏错误。

最低验收：未知 tuple/旧 epoch/旧 generation/跨租户 cache 全拒绝；worker 抓包无明文；取消后不得有旧
结果发布；crash/restart 后不得复用旧 reservation；direct-native 性能门槛按硬件 profile 单独出具 receipt。

## 8. 分期开发计划

| 阶段 | 内容 | 依赖 | 退出条件 | 状态 |
|---|---|---|---|---|
| INF-0 | 合同、威胁模型、PerformanceContract、ABI、模型 manifest、硬件基线、golden vectors | 无 | schema/协议/预算/供应链字段冻结 | plan-only |
| INF-1 | backend-neutral deterministic loopback + UDS；身份、ACL、配额、取消、重启、generation fence 与 shadow typed-output/receipt fence | INF-0 | 两 node/多 Agent 隔离和 generation fence 可重放；无 authority 越界 | qualification-only |
| INF-2A | 锁定 llama.cpp worker；一组 GGUF tuple 的 cold/warm/direct-native benchmark | INF-1 | 独立性能 receipt；失败组合 `KNOWN_GAP` | controlled fixture |
| INF-2B | Core ML tiny neuron + whisper.cpp/sherpa-onnx | INF-1 | 实际硬件路径和 ASR receipt；不解冻生产 NPU | controlled fixture |
| INF-3 | warm model pool、EDF/WFQ、continuous batching、KV/prefix cache、背压、OOM/thermal breaker | INF-2A/B | 配额、公平性、取消和缓存隔离 receipt | qualification-only |
| INF-4 | Mac M5、RTX4060、j3160、Pocket4（profile 到位后）性能 qualification | INF-3 | 每设备 tuple allowlist、p50/p95 和资源 receipt | benchmark-only |
| INF-5 | stable-diffusion.cpp 图像；受控音频扩展 | INF-4 | 图像/音频 receipt；视频生成不纳入 P0 | future P1 |
| INF-6 | 接入 RunStartSnapshot、ArtifactSetDigest、ModelReceipt、NeuronSignal、TaskFlow | INF-3, INF-4 | typed signal/receipt 端到端，零 authority 越界 | future P1 |
| INF-7 | 视频生成、远程/联邦推理、多运营方模型市场 | INF-6 | 单独安全、隐私、成本和治理评审 | future P2 |

INF-2A 与 INF-2B 可并行；Pocket4 未知不能阻塞 INF-1。任何 backend 失败只影响自身 tuple，不能阻塞
backend-neutral core；但没有新 receipt 就不能进入默认路由。

## 9. 机器可读附件、权限与回滚

同一变更集附加：

- `OpenClaw/hepta-inference-runtime-plan-2026-08-26.md`
- `OpenClaw/INF_STAGE_MATRIX_v1.yaml`
- `OpenClaw/hepta.infer_receipt.v1.schema.json`
- `OpenClaw/HEPTA_INFERENCE_PERFORMANCE_CONTRACT_V1.md`
- `OpenClaw/INF-PLAN-APPEND-RECEIPT-2026-08-26.json`

全局负权限固定为：

```yaml
qualification_only: true
production_listener: false
production_writer: false
provider_effect: false
shared_kg_write: false
route_write: false
fleet_write: false
model_npu: false
remote_inference: false
CALLERS_touched: false
operator_acceptance: false
promotion: false
```

每阶段 rollback 只能关闭新 feature、移除新 worker/tuple、清理新缓存或回到旧 semantic contract；
不得修改 canonical main、旧 receipt、effective-index、authority、manifest 或 live binary。父 digest/head/tree/
dirty 变化、schema 漂移、artifact 签名失败或跨租户泄漏均立即 `STALE_SOURCE_BINDING`/`FAIL_CLOSED`。

## 10. 明确剩余 blocker

1. Pocket4 的 SoC、RAM、NPU/GPU、OS、目标 workload 尚未确认，不能冻结其模型大小、量化和 tok/s/延迟门槛。
2. 需要在 INF-0 冻结 UDS CBOR profile、ABI version、shared-memory descriptor、canonical error/cancel semantics。
3. 需要锁定各 backend commit、compiler/driver 矩阵、direct-native benchmark harness、脱敏 corpus 和结果校验。
4. 需要建立模型/权重/转换 artifact/SBOM/license/signature/rollback registry。
5. 当前 external effective-index、qualification mirror、authority 和 manifest 仍指向旧 digest；fresh binding receipt
   前只能做 RFC、schema、golden vector、deterministic loopback 和 shadow design。
6. 当前实验树存在未提交 lifecycle 改动；推理 lane 不得从该树派生实现，也不得把推理代码塞进 `codex-core` blocker 分支。

一句话定案：**Rust 形成 Hepta 的常驻推理控制整体，native runtime 负责速度；只有被
PerformanceContract 实测证明的 tuple 才能路由，所有未知组合 fail-closed。**
