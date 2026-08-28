# HEPTA 本地大模型推理运行时开发计划 v2

> Plan ID: `HEPTA-INFERENCE-RUNTIME-V2`  
> Version: `2.0.0`  
> Date: `2026-08-28`  
> Base repository: `ProfAlexQI/hepta-private-ci`  
> Base branch: `integration/vnext-main-20260811`  
> Base commit: `fe0889ecd46a5fc89de7b1ff3f28158c133a3502`  
> Base tree: `636341eb865b7c6d669958a96e7959de74fee020`  
> Parent plan: `docs/hepta-vnext/dropbox-current-2026-08-27/root/hepta-inference-runtime-plan-2026-08-26.md`  
> Status: `SOURCE_BOUND_DEVELOPMENT_PLAN / QUALIFICATION_ONLY`  
> Authority: all production, effect, route, fleet, model/NPU, remote-inference and promotion authority remain `false`.

## 0. 定案

Hepta 的本地推理不能继续停留在“Codex 客户端直连 Ollama/LM Studio”的兼容模式，也不能把任意单一
native runtime 当成信任根。v2 采用三层结构：

```text
Hepta agents / app-server / agentd
              │
              │ versioned local inference protocol
              ▼
hepta-infer-core + hepta-inferd
  admission / identity / quota / scheduler / cache fence / cancellation
  model registry / lifecycle / observability / immutable receipt
              │
              │ backend-neutral worker protocol
              ▼
Ollama adapter | LM Studio adapter | llama.cpp worker | Core ML worker
OpenVINO worker | CUDA/vendor worker | audio/image workers
```

Rust 负责常驻控制面、合同和隔离；高速 kernel 由被锁定和实测过的 native runtime 执行。
任何 `model × tokenizer × artifact × backend × compiler × driver × device × quantization × context ×
batch/cache` 组合，没有独立 qualification receipt 就必须 `NotAdmitted`，不能静默切 CPU、远程、
另一量化或另一 provider。

本计划替代旧计划中“已经列出阶段命令但对应 crate 尚不存在”的歧义。每个阶段必须分别记录：

1. `planned`
2. `source_present`
3. `wired`
4. `executed`
5. `qualified`
6. `operator_accepted`
7. `promoted`

前一状态不能被文档描述自动提升为后一状态。

---

## 1. 当前事实基线

### 1.1 已存在

- `codex-rs/ollama`
- `codex-rs/lmstudio`
- `codex-rs/model-provider`
- `codex-rs/model-provider-info`
- `codex-rs/utils/oss`
- `codex-rs/responses-api-proxy`
- CLI/TUI/exec 的 `--oss` 与 `--local-provider`
- Ollama/LM Studio 的基本探测、模型列表、下载/拉取和 Responses API 兼容路径
- 旧版 inference performance、resource、model-eval 与 stage matrix 规划文档

### 1.2 尚不存在或尚未证明

- workspace 内的 `hepta-infer-core`
- 常驻 `hepta-inferd`
- 版本化 UDS/CBOR 协议
- native worker ABI
- request generation / cancel generation fence
- 多租户配额与调度
- warm model pool
- continuous/micro batching
- tenant-scoped KV/prefix cache
- 模型供应链 registry
- 真实模型 E2E
- 硬件 qualification receipt
- runner 实际执行证据
- 任何生产、NPU、远程、effect、promotion 权限

### 1.3 当前主要缺陷

1. 实际运行路径和目标架构双轨。
2. provider 抽象缺少模型生命周期、资源和取消合同。
3. readiness 存在 warning 后继续、后台裸 `spawn` 等 fail-open 行为。
4. capability 默认值过宽，不能由 provider 名称推断模型能力。
5. 没有 per-host 单实例控制面，多个 Agent 不能共享和隔离资源。
6. 没有 exact tuple 性能准入。
7. 模型下载、二进制、许可证、SBOM、签名和回滚链未闭环。
8. debug dump 可记录原始 prompt/response，隐私边界不符合生产要求。
9. CI 主要是模拟 HTTP；不能证明真实模型、取消、OOM、并发和硬件路径。
10. 旧阶段矩阵引用不存在的 crate，容易造成“计划命令等于已实施”的错误认知。

---

## 2. v2 目标与非目标

### 2.1 P0 目标

- 建立 source-bound、backend-neutral、fail-closed 的推理合同。
- 建立每主机一个 `hepta-inferd` 的最小控制面。
- 先支持文本 LLM 的一个真实、受控 backend。
- 兼容 Ollama/LM Studio，但将其降级为 adapter，而不是控制面。
- 支持请求身份、配额、deadline、取消、generation fence 和 terminal receipt。
- 支持一个模型 tuple 的 cold/warm/direct-native benchmark。
- 保证 raw prompt/media 不进入默认日志与 receipt。
- 所有 production/effect/remote/model-NPU authority 保持关闭。

### 2.2 P0 非目标

- 不实现视频生成。
- 不解冻远程推理。
- 不自动安装未知模型。
- 不允许 Agent 自行选 backend、模型路径、设备或 endpoint。
- 不实现跨主机 KV cache。
- 不将模型输出直接升级为 Memory、KG、effect、capability 或 route。
- 不以“Rust 实现”替代硬件性能证明。
- 不在没有 receipt 的情况下宣称 NPU/ANE/CUDA 路径已使用。

---

## 3. 信任与权限边界

### 3.1 全局负权限

```yaml
qualification_only: true
production_listener: false
production_writer: false
provider_effect: false
external_effect: false
shared_kg_write: false
memory_write: false
route_write: false
fleet_write: false
model_npu: false
remote_inference: false
automatic_model_install: false
operator_acceptance: false
promotion: false
release: false
```

### 3.2 所有请求必须绑定

```text
TenantId
WorkspaceId
AgentId
agent_generation
TaskId
RequestId
request_generation
cancel_generation
PolicyDigest
ResourceBudgetId
deadline_unix_ms
ModelTupleDigest
PromptDigest
PromptByteLength
OutputTokenLimit
```

任一身份、generation、policy、budget、deadline 或 tuple 缺失，返回稳定 typed error。

### 3.3 权威禁止

推理层只能产生：

- 流式推理事件；
- 资源事实；
- 不可变 execution receipt；
- typed proposal/signal。

推理层不能：

- 创建或修改 Hepta authority；
- 直接写 Memory/KG；
- 执行 provider effect；
- 修改 route/fleet；
- 将模型输出标记为事实；
- 自动批准模型或硬件 tuple；
- 自动切换到远程 endpoint。

---

## 4. 核心数据合同

### 4.1 Digest

所有安全相关 digest 使用：

```text
sha256:<64 lowercase hex>
```

空值、非小写、非 64 位、未知算法全部拒绝。初始 reference crate 只验证和传递 digest；真正对模型文件、
tokenizer、compiled artifact 和结果做 hash 的实现进入 INF-1/INF-2。

### 4.2 ModelTuple

```rust
struct ModelTuple {
    model_digest: Digest,
    tokenizer_digest: Digest,
    backend_id: String,
    backend_commit: String,
    backend_abi: String,
    compiled_artifact_digest: Digest,
    compiler_id: String,
    driver_runtime: String,
    device_profile_digest: Digest,
    quantization: String,
    context_tokens: u32,
    batch_size: u32,
    kv_cache_policy: String,
    prefix_cache_policy: String,
    input_output_shape: String,
    sbom_digest: Digest,
    license_digest: Digest,
}
```

任何字段变化都是不同 tuple，不能继承旧 receipt。

### 4.3 Request

请求对象不携带可被默认日志打印的 raw prompt 字段。控制面接收：

- shared-memory/byte-stream descriptor；
- `prompt_digest`；
- `prompt_byte_length`；
- privacy class；
- output limit；
- deadline 和 cancellation fence。

reference loopback 只接受 digest 和长度，不接收原始 prompt。

### 4.4 Event

```text
Accepted
Queued
Loading
Warming
Running
TokenDelta
Usage
Draining
Completed
Cancelled
Rejected
FailedClosed
```

每个事件携带 `request_id`、`request_generation`、`backend_generation` 和单调 `sequence`。
旧 generation 的事件不能发布。

### 4.5 Receipt

终态 receipt 至少包含：

- source commit/tree；
- request identity and generations；
- model tuple digest；
- policy and budget digest；
- queue/load/TTFT/prefill/decode/total timing；
- token count；
- RSS/VRAM/CPU/GPU/NPU/power/temperature；
- cache outcome；
- cancellation/restart outcome；
- result digest；
- fallback attempts；
- authority snapshot。

raw prompt、音频、图像和私有代码不得写入 receipt。

---

## 5. Admission 与状态机

### 5.1 准入顺序

```text
validate schema
→ validate closed authority
→ validate tenant/workspace/agent/task identity
→ validate generations
→ validate deadline
→ validate resource budget
→ exact tuple lookup
→ exact policy binding
→ reserve resources
→ enqueue
```

任何失败都在 reserve/load 前终止。

### 5.2 生命周期

```text
Admit
  → Reserved
  → Queued
  → Loading
  → Warming
  → Running
  → Draining
  → Completed

Admit/Reserved/Queued/Loading/Warming/Running/Draining
  → Cancelled

任意非终态
  → FailedClosed

Admit
  → Rejected
```

终态不可再次转换；worker/backend generation 变化使所有旧 request 和 cache lease 失效。

### 5.3 取消

取消必须同时满足：

- request identity 匹配；
- request generation 匹配；
- cancel generation 严格递增；
- backend generation 未被替换。

收到取消后，旧 token/event 即使晚到也被 controller 丢弃。取消 receipt 必须说明：

- controller 是否接受；
- backend 是否确认；
- 最后发布 sequence；
- 是否发生 forced worker termination；
- reservation 是否释放。

---

## 6. 多 Agent 调度与资源治理

### 6.1 队列

- 交互式请求：EDF，deadline 越近优先。
- 后台请求：weighted fair queue。
- 每 tenant/workspace/agent 有独立并发和排队上限。
- 单个 Agent 不能占满所有 stream slot。
- queue wait 超限必须拒绝或取消，不能无限等待。

### 6.2 资源预算

初始预算维度：

- CPU milliseconds；
- GPU/NPU milliseconds；
- RSS；
- VRAM；
- context tokens；
- output tokens；
- concurrent streams；
- queue length；
- model residency；
- file descriptors；
- worker count；
- temperature/power envelope。

### 6.3 Cache

KV/prefix cache key 必须包含：

```text
TenantId
WorkspaceId
ModelTupleDigest
PolicyDigest
privacy_class
backend_generation
```

默认禁止跨 tenant/workspace 共享。cache hit 必须写 receipt；generation、policy 或 tuple 变化立即失效。

---

## 7. Backend 与 worker 设计

### 7.1 Backend 分层

1. `ReferenceLoopbackBackend`：无网络、无模型，仅验证合同。
2. `OllamaAdapter`：兼容和开发。
3. `LMStudioAdapter`：兼容和开发。
4. `LlamaCppWorker`：P0 原生文本默认候选。
5. `CoreML/OpenVINO/CUDA/vendor workers`：按设备 qualification。
6. 音频/图像 worker：P1。

### 7.2 `hepta_backend_v1`

计划 ABI：

```text
abi_version
backend_id
backend_generation
capabilities
load
warm
submit
poll
cancel
drain
unload
health
stats
```

使用 opaque handle、显式长度 byte slice 和明确 allocator ownership。worker 不能持有 Rust heap 指针，
不能写 Hepta 状态，也不能选择远程 endpoint。

### 7.3 进程隔离

不稳定 native backend 默认独立进程：

- crash/OOM/deadlock 只熔断该 backend；
- supervisor 重启后 generation 递增；
- 旧 lease、cache 和 receipt 草稿全部失效；
- worker 不监听 TCP；
- worker 不能读取超出模型和受控 shared-memory descriptor 的文件。

---

## 8. 模型供应链

### 8.1 ModelManifest

必须包含：

- model/tokenizer/compiled artifact digest；
- format 与 quantization；
- backend commit 和 ABI；
- compiler/build flags；
- driver/runtime；
- device profile；
- SBOM；
- license；
- signature；
- source allowlist；
- privacy classification；
- rollback digest；
- allowed policy profiles；
- benchmark fixture digest；
- capability declaration。

### 8.2 安装与加载

- 下载、验证、安装、注册、加载是不同动作。
- 普通推理请求不能触发隐式下载。
- 自动下载在 P0 保持关闭。
- 未签名、许可证未知、digest 不符或 provenance 不完整的模型不得注册。
- PATH 中任意同名 CLI 不能自动成为可信安装器。

---

## 9. 隐私、日志与调试

默认日志只能记录：

- request/digest/shape/size；
- queue/load/runtime timing；
- resource metrics；
- typed error code；
- backend generation。

禁止默认记录：

- Authorization/Cookie/secret headers；
- raw prompt；
- source code；
- model full output；
- audio/image bytes；
- tool arguments。

任何 debug body dump 必须：

1. 显式 unsafe-development 开关；
2. owner-only 文件权限；
3. 独立目录；
4. TTL 与大小上限；
5. 默认结构化脱敏；
6. receipt 标记；
7. 生产构建不可用或 fail-closed。

---

## 10. 性能合同

### 10.1 exact tuple

每条性能资格必须绑定完整 tuple、source、fixture 和环境。不得将不同设备、量化、context 或 batch 的
结果混合。

### 10.2 provisional gates

在 INF-2 实机基线冻结前：

- warm sidecar p95 overhead ≤ `max(10 ms, 10%)`；
- warm throughput ≥ direct-native 的 90%；
- cold-start 单独报告；
- TTFT 单独报告；
- RSS/VRAM ≤ direct-native 的 1.1×；
- ASR RTF、图像延迟 ≤ direct-native 的 1.1×；
- thermal throttle 不能隐藏；
- 失败 tuple 标记 `KNOWN_GAP` 并从默认 router 移除。

### 10.3 结果正确性

性能测试必须同时校验：

- deterministic fixture/result digest；
- token count；
- stop reason；
- tool-call schema；
- cancellation terminality；
- no stale event after restart。

只测 tok/s 不构成资格认证。

---

## 11. 测试架构

### 11.1 L0 source gate

- JSON/YAML/Markdown binding；
- authority 全 false；
- stage/status 一致；
- crate/files 存在；
- no accidental network/listener；
- no raw prompt field；
- source receipt 绑定 exact parent commit/tree。

### 11.2 L1 reference tests

- digest validation；
- authority escalation rejection；
- unknown tuple rejection；
- policy mismatch rejection；
- stale request/cancel/backend generation rejection；
- cross-tenant cache rejection；
- lifecycle invalid transition rejection；
- terminal state immutability；
- deterministic loopback event ordering；
- receipt 不包含 raw prompt。

### 11.3 L2 software E2E

固定版本 Ollama/LM Studio 与 tiny model：

- startup；
- model discovery；
- explicit load；
- minimal streaming；
- tool call；
- cancellation；
- restart；
- malformed event；
- non-2xx；
- version unknown；
- timeout；
- no implicit download。

### 11.4 L3 native worker

- ABI sanitizer；
- crash/OOM/deadlock；
- generation fence；
- shared-memory bounds/TOCTOU；
- direct-native parity；
- concurrent streams；
- backpressure。

### 11.5 L4 hardware qualification

按设备独立运行：

- macOS Apple Silicon；
- Windows/Linux CUDA；
- Intel CPU/GPU/NPU；
- low-memory CPU；
- Android/ARM/Pocket profile。

每个设备只承认自己的 receipt。

---

## 12. CI 证据分类

允许状态：

```text
NOT_STARTED
SOURCE_PRESENT_NOT_RUN
BLOCKED_RUNNER_NOT_ASSIGNED
EXECUTED_FAILED
EXECUTED_PASSED_QUALIFICATION_ONLY
OPERATOR_ACCEPTED
PROMOTED
```

`steps=[]`、`runner_id=0`、checkout 未执行均是 `BLOCKED_RUNNER_NOT_ASSIGNED`，不是 PASS，也不是源码失败。
本地等价测试可记录为 `LOCAL_SOURCE_EQUIVALENT_PASS`，但不能替代 exact-head CI。

---

## 13. 分期计划

### INF-0A — source rebind 与计划冻结

Deliverables:

- 本 v2 计划；
- stage matrix v2；
- implementation status；
- threat/authority contract；
- source gate；
- exact source receipt。

Exit:

- exact base head/tree；
- authority 全 false；
- 所有阶段状态可机器读取；
- 旧 stale plan 不再被误读为已实施。

### INF-0B — backend-neutral reference contracts

Deliverables:

- standalone qualification crate；
- Digest/IDs/Authority/ModelTuple/Request/Admission/Lifecycle/CacheFence/Receipt；
- deterministic loopback；
- strict unit tests；
- no network/no filesystem/no unsafe。

Exit:

- source present；
- fmt/test/clippy 被 exact-head runner 执行；
- 未知 tuple、旧 generation、跨租户 cache 全部拒绝。

### INF-0C — compatibility readiness hardening

Deliverables:

- LM Studio readiness 等待真实 load 完成；
- Ollama 模型探测/版本探测 fail-closed；
- unknown provider fail-closed；
- typed errors；
- 不隐式把 warning 当 ready。

Exit:

- 真实软件 E2E；
- 无后台裸 spawn；
- 无未知 provider silent success。

### INF-1 — minimal `hepta-inferd`

Deliverables:

- versioned UDS protocol；
- peer identity；
- bounded queue；
- exact tuple registry；
- cancel/backend generation；
- terminal receipt；
- one controller instance per host。

Exit:

- two tenant deterministic replay；
- crash/restart；
- no stale event；
- no authority crossing；
- no TCP listener。

### INF-2A — Ollama/LM Studio adapters behind daemon

Deliverables:

- adapter trait；
- explicit model lifecycle；
- event normalization；
- timeout/cancel；
- capability probe；
- no implicit download。

Exit:

- tiny model E2E；
- tool-call fixture；
- restart and malformed stream tests。

### INF-2B — llama.cpp native worker

Deliverables:

- locked upstream commit；
- `hepta_backend_v1`；
- GGUF allowlist；
- one qualified model/device/quant tuple；
- direct-native benchmark。

Exit:

- sanitizer/fault tests；
- cold/warm receipts；
- performance gate or `KNOWN_GAP`。

### INF-3 — scheduler/cache/resource control

Deliverables:

- EDF/WFQ；
- warm pool；
- continuous/micro batching；
- KV/prefix cache；
- OOM/thermal/backpressure breaker；
- tenant quotas。

Exit:

- fairness；
- cache isolation；
- cancellation under load；
- reservation recovery。

### INF-4 — multi-device qualification

Deliverables:

- device profiles；
- signed receipts；
- router allowlist；
- per-device rollback。

Exit:

- router only contains measured tuple；
- unknown devices fail closed。

### INF-5 — semantic integration

Deliverables:

- RunStartSnapshot binding；
- ModelReceipt reference；
- NeuronSignal/TaskFlow typed adapter；
- no direct Memory/KG/effect write。

Exit:

- end-to-end typed output；
- authority negative tests；
- replay/rollback。

### INF-6 — audio/image

ASR/VAD first, image second. Video remains out of scope。

### INF-7 — operator acceptance and promotion

Only after exact-head CI, hardware receipts, security review and independent operator acceptance。

### INF-8 — remote/federated future lane

Requires separate privacy, cost, policy, routing and governance review. It is not inherited from local inference。

---

## 14. Immediate implementation tranche

本次下一步开发固定为 `INF-0A + INF-0B + INF-0C(source subset)`：

1. 创建 source-bound plan/stage/status。
2. 创建 standalone `tools/hepta-inference-inf0` reference crate。
3. 覆盖 admission、generation、cache isolation、lifecycle、receipt 的 deterministic tests。
4. 增加 source gate 与专用 workflow。
5. 将 LM Studio model load 从后台裸 spawn 改为 await。
6. unknown OSS provider 改为 fail-closed。
7. Ollama readiness 的模型查询和版本未知改为 fail-closed。
8. 不创建 daemon、不监听端口、不加载真实模型、不授予任何 authority。
9. CI 未执行前状态只能是 `SOURCE_PRESENT_NOT_RUN`。
10. exact source candidate 由后续 append-only receipt 绑定。

---

## 15. 文件级工作分解

```text
docs/hepta-vnext/inference/
  HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V2.md
  HEPTA_INFERENCE_STAGE_MATRIX_V2.json
  HEPTA_INFERENCE_IMPLEMENTATION_STATUS_V1.json
  HEPTA_INFERENCE_INF0_SOURCE_RECEIPT_2026-08-28.json

tools/hepta-inference-inf0/
  Cargo.toml
  Cargo.lock
  README.md
  src/lib.rs

scripts/
  hepta-inference-inf0-source-gate.py

.github/workflows/
  hepta-inference-inf0.yml

compatibility hardening:
  codex-rs/lmstudio/src/lib.rs
  codex-rs/ollama/src/lib.rs
  codex-rs/utils/oss/src/lib.rs
```

---

## 16. 风险与控制

| 风险 | 控制 |
|---|---|
| 新 crate 尚未进入主 workspace | INF-0 使用独立 workspace，资格通过后再进入 product workspace |
| CI runner 未分配 | 明确标记 blocker，禁止 PASS |
| adapter 行为改变影响用户 | 独立 Draft PR，真实 E2E 前不合并 |
| 模型下载供应链不可信 | P0 默认禁止隐式下载，后续 manifest/signature lane |
| 本地 HTTP 泄漏 prompt | daemon 后端仅 loopback/UDS，日志默认 digest-only |
| native crash 拖垮 agentd | 独立 worker 和 generation fence |
| cache 跨租户泄漏 | exact scoped cache key + negative tests |
| 性能优化破坏正确性 | correctness digest 与 terminality 同时成为 gate |
| 文档超前于代码 | machine status 与 source receipt 分离 |
| 旧计划 stale binding | v2 exact base binding，旧文档保留为历史上下文 |

---

## 17. Rollback

- INF-0：删除新 plan/status/reference tool/workflow；不影响 product runtime。
- INF-0C：恢复三处 compatibility helper 的旧行为；不更改模型或 state。
- INF-1：关闭 inference feature，停止新 daemon，删除 socket，不迁移现有状态。
- INF-2+：从 router 移除 tuple、drain worker、清理仅属于该 generation 的 cache。
- 回滚不得修改历史 receipt、已有 Memory/KG、authority 或 canonical source snapshot。

---

## 18. Definition of Done

本次 tranche 只有同时满足下列条件才可称为 `EXECUTED_PASSED_QUALIFICATION_ONLY`：

- exact candidate 的 source receipt 校验通过；
- source gate 实际执行；
- `cargo fmt --check` 实际执行；
- standalone crate tests 实际执行；
- strict clippy 实际执行；
- compatibility crates 至少完成 workspace check/test；
- CI steps 非空且 runner_id 非 0；
- 所有 authority 仍为 false；
- 没有真实模型、NPU、远程或 production claim。

在此之前，本分支只能声明：

```text
SOURCE_PRESENT_NOT_RUN
qualified=false
promotion=false
```
