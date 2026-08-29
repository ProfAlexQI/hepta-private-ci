# HEPTA 本地大模型推理运行时开发计划 v3

> Plan ID: `HEPTA-INFERENCE-RUNTIME-V3`  
> Version: `3.0.0`  
> Date: `2026-08-30`  
> Repository: `ProfHepta/hepta-private-ci`  
> Canonical default branch: `integration/vnext-main-20260811`  
> Canonical default head observed: `fe0889ecd46a5fc89de7b1ff3f28158c133a3502`  
> Development PR: `#73`  
> Development branch: `codex/hepta-inference-gap-closure-20260829`  
> Development parent: `6f3286f99f5576b4cc545003889437bf388bb28a`  
> Development head observed before this plan: `211948a45874890d69cb60b1bdafd946da2fc77f`  
> Development tree observed: `ad1879dc529f0da44ebb48d84b22e06749a8ef5a`  
> Supersedes for current execution: `HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V2.md`  
> Status: `SOURCE_BOUND_ACTIVE_PLAN / QUALIFICATION_ONLY`

## 0. 定案与权威边界

本计划把本地推理工程从“阶段名称驱动”改为“可执行 package + 不可伪造 evidence 驱动”。源码存在、
控制面测试通过、真实模型执行、硬件性能通过、产品接入、operator acceptance 和 promotion 是七个互不
等价的状态，任何文档、提交信息或局部绿色工作流都不能自动提升状态。

所有工作保持以下负权限：

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

本 PR 必须保持 Draft；不得自合并，不得改变默认产品路由，不得把控制面 fixture、模拟 token 或源码测试
解释为真实模型、硬件或生产资格。

---

## 1. 单一真值模型

### 1.1 真值优先级

从高到低：

1. exact-head Git commit/tree 与 tracked source；
2. exact-head、非空 steps、已分配 runner 的 GitHub Actions 执行记录；
3. digest-bound artifact/receipt；
4. 本目录当前状态、阶段矩阵和 blocker ledger；
5. 计划文档；
6. PR 描述、提交信息和人工说明。

低优先级材料不得覆盖高优先级事实。

### 1.2 状态词汇

```text
NOT_STARTED
PLANNED
SOURCE_PRESENT_NOT_RUN
BLOCKED_RUNNER_NOT_ASSIGNED
BLOCKED_EXTERNAL_DEPENDENCY
EXECUTED_FAILED
EXECUTED_PASSED_QUALIFICATION_ONLY
OPERATOR_ACCEPTED
PROMOTED
RELEASED
```

`steps=[]`、`runner_id=0`、checkout 未发生或 required job 被 skipped 均不构成 PASS。

### 1.3 自引用禁止

tracked JSON 不写入其自身未知的最终 commit/tree。tracked 状态只绑定“生成该状态之前观测到的 parent
head”；exact-head commit/tree 由 CI 在运行时写入 artifact receipt。任何 append-only receipt 必须在独立
提交中绑定其 parent source candidate。

---

## 2. 已核验当前事实

### 2.1 已存在并已进入 workspace

- `codex-rs/hepta-infer-core`：身份、authority、admission、生命周期、CBOR 协议、adapter 合同、native
  worker ABI/manifest/supervisor 合同；
- `codex-rs/hepta-infer-client`：owner-local UDS client、exact capability profile、shadow-only route；
- `codex-rs/hepta-inferd`：owner-local UDS daemon、same-user peer gate、单实例 socket、generation 文件和
  digest-only terminal receipt；
- Ollama/LM Studio compatibility hardening 与 fixed-tuple capability evidence；
- exact-head workflow `Hepta inference gap closure qualification`。

在 head `211948a...` 上，run `33258442369` 的 source contract、Rust 1.95 fmt、locked check、all-target tests、
strict Clippy 与 aggregate 均实际执行并通过。因此 Rust 1.95 不是当前三个推理 crate 的已证实 blocker。

### 2.2 已证实的 CI 漂移

workflow `Hepta inference v2 exact-head runtime closure qualification` run `33258442434` 在 compile 步骤失败：

```text
Can't list 'tools/hepta-inference-v2-qualification'
No such file or directory:
  scripts/hepta-inference-inf0c-cancel-capability-v4.py
```

同一 workflow 还引用八个未落盘 crate。该工作流必须先收敛到 tracked source，未来 crate 只有在同一提交
实际加入 workspace 后才能加入 required matrix。

### 2.3 尚未实现或尚未证明

- public client、worker event producer、operator 三种角色的独立授权通道；
- daemon-minted per-request/worker-session capability；
- global/per-tenant inflight 与 running 上限、connection semaphore、I/O timeout；
- token stream rolling digest、实际 token 计数与 output limit 强制一致；
- receipt 启动恢复、内存卸载、磁盘总量/数量/TTL 上限；
- daemon 后真实 Ollama/LM Studio adapter 调用；
- 独立 native worker process、真实 llama.cpp load/forward/decode；
- worker cancel ACK、超时 kill 与 generation rollover；
- scheduler、lease、warm pool、tenant-scoped KV/prefix cache；
- product shadow wiring；
- exact-device hardware/performance receipt；
- independent operator acceptance、promotion 和 release。

---

## 3. 威胁模型与必须闭合的不变量

### 3.1 同 UID 不等于同 authority

桌面 Agent 可能同时运行插件、MCP server、工具、IDE helper 和第三方本地进程。same-user peer check 只证明
操作系统用户相同，不能授权任意进程发布 `Start/Token/Complete` 或执行 `RestartBackend`。

必须拆分：

```text
Public client channel:
  Ping / Admit / CancelOwnRequest / GetOwnReceipt / bounded Snapshot

Worker channel:
  WorkerHello / StartAck / Token / Complete / Failure / CancelAck / Health

Operator channel:
  Restart / Drain / Install / Register / Unregister / InventoryRefresh
```

public socket 必须拒绝 worker/operator-only 消息。worker channel 必须绑定 daemon-minted session nonce、
backend generation 和每请求 capability。operator channel 必须使用独立 owner-only endpoint 或继承句柄，
不得因 UID 相同而隐式授权。

### 3.2 请求 fence

每次执行至少绑定：

```text
TenantId / WorkspaceId / AgentId / agent_generation
TaskId / RequestId / request_generation / cancel_generation
backend_generation / worker_session_nonce / request_capability
PolicyDigest / ResourceBudgetId / deadline_unix_ms
ModelTupleDigest / PromptDigest / PromptByteLength / OutputTokenLimit
```

任一字段缺失、陈旧或不匹配都必须在 backend dispatch 前 fail closed。

### 3.3 输出完整性

接受第 n 个 token 时，daemon 维护：

```text
chain_0 = SHA256(domain || request identity || generations || tuple digest)
chain_n = SHA256(chain_(n-1) || sequence_n || token_digest_n || token_byte_length_n)
```

完成条件：

```text
reported_output_tokens == accepted_token_count
reported_result_digest == chain_n
accepted_token_count <= request.output_token_limit
sequence strictly monotonic
```

worker 不能自行声明一个未由已接受 token stream 推导出的 final digest。

### 3.4 有界资源

必须同时限制：

- accepted connections；
- frame read/write/idle duration；
- queue、global inflight、per-tenant inflight；
- global running、per-tenant running、per-model running；
- worker count 和 active streams；
- prompt bytes、output tokens、token bytes；
- terminal records in memory；
- receipt files、receipt bytes、TTL；
- model residency、RSS、VRAM、KV cache 和 shared-memory lease。

### 3.5 取消与故障

queued request 可由 controller 原子取消。running request 只有在 worker `CancelAck` 后才能声明正常 Cancelled；
若 ACK 超时，则必须 kill worker、递增 backend generation、fail-close 所有受影响请求，并在 receipt 中记录
`forced_worker_termination=true`。transport disconnect 不等于 backend cancellation acknowledgement。

---

## 4. 目标架构

```text
CLI / TUI / app-server / agentd
              │
              │ unprivileged owner-local protocol
              ▼
hepta-inferd
  identity/admission/policy/quota/deadline
  capability minting/router/scheduler/model registry
  receipt journal/resource controller
              │
              │ authenticated inherited pipe or private worker UDS
              ▼
isolated worker host
  watchdog/cancel ACK/generation/session nonce
              │
              │ pinned C ABI
              ▼
qualified runtime
  llama.cpp first; provider adapters remain compatibility lanes
```

Rust 管理信任、策略、状态、隔离、资源和证据；成熟 native runtime 管理 tokenizer、model load、tensor kernel
和 decode。native 崩溃不能带崩 agentd，也不能获得 Memory/KG/effect/route 权限。

---

## 5. Blocker packages 与依赖图

```text
INF-T0 docs/truth rebase
  └─ INF-T1 exact-head CI repair
      └─ INF-H1 public role boundary
          ├─ INF-H2 controller resource bounds
          ├─ INF-H3 token integrity
          └─ INF-H4 connection + receipt durability
               ├─ INF-2A real provider adapter behind daemon
               └─ INF-2B isolated native worker
                    └─ INF-2C cancellation ACK/forced termination
                         └─ INF-3 scheduler/cache/resource leases
                              └─ INF-4 device qualification
                                   └─ INF-5 product shadow integration
                                        └─ INF-7 operator acceptance/promotion
```

完整 blocker 及机器状态在 `HEPTA_INFERENCE_BLOCKER_LEDGER_V1.json`。

---

## 6. 分期实施与验收

### INF-T0 — 文档与机器真值重绑定

Deliverables:

- 本 V3 计划；
- current status V3；
- implementation status V2；
- stage matrix V4；
- blocker ledger V1；
- closure evidence contract V1。

Exit:

- 当前 repo owner、PR/base/head 和已执行 run 准确；
- 历史 receipts 不修改；
- 真实执行与源码合同明确分离；
- authority 全 false。

### INF-T1 — exact-head CI 收敛

Deliverables:

- required workflow 只引用 tracked files/crates；
- source inventory gate 检查 workflow 中的 owned path；
- Rust 1.95 对现有三 crate 的 fmt/check/test/clippy；
- exact head/tree、Cargo.lock digest、runner OS/arch/toolchain receipt；
- aggregate 对 skipped/cancelled/failure 全部红灯。

Exit:

- 不存在路径为零；
- source、Rust 和 aggregate 非空执行并通过；
- artifact 明确 `real_model_executed=false`、`hardware_qualified=false`。

### INF-H1 — public/worker/operator 权限分离

Deliverables:

- public daemon dispatch 拒绝 `Start/Token/Complete/RestartBackend`；
- public client 不暴露 worker/operator convenience API；
- 内部 controller/worker contract 保留，后续通过专用 authenticated channel 使用；
- negative tests：同 UID public peer 无法伪造 token、complete 或 restart。

Exit:

- public socket 只执行 unprivileged operations；
- role violation 返回稳定 typed error；
- 无 production activation。

### INF-H2 — controller 资源边界

Deliverables:

- `max_inflight_global/per_tenant`；
- `max_running_global/per_tenant`；
- queue 与 running accounting 全状态转换一致；
- terminal record 有界释放；
- deadline expiration 由 daemon timer 主动驱动，而不是只在 admission 检查。

Exit:

- admit→start 循环不能绕过配额；
- terminal/cancel/restart 后 reservation 确定释放；
- property/fault tests 覆盖每条转换。

### INF-H3 — token stream 完整性

Deliverables:

- domain-separated rolling digest；
- accepted token count/bytes；
- output-token limit 强制；
- completion count/digest 一致性；
- receipt 绑定 chain digest、count、bytes 和 termination reason。

Exit:

- forged final digest、少报/多报 token、超限 token、重复/跳号 sequence 全部拒绝；
- deterministic replay 产生 byte-identical receipt。

### INF-H4 — connection、journal 与恢复

Deliverables:

- connection semaphore；
- read/write/idle timeout；
- atomic receipt temp-write/sync/rename/parent-sync；
- startup receipt scan + bounded index；
- max files/bytes/TTL；
- terminal receipt durable 后从 controller memory 释放；
- restart 后可按 exact fence 查询旧 receipt。

Exit:

- partial frame/slowloris 不能无限占 task/FD；
- corrupt/duplicate/oversized receipt fail closed；
- crash/restart fixture 保持 terminal truth。

### INF-2A — daemon 后真实 provider adapter

第一条真实软件纵切固定一个预安装的小模型 tuple。普通推理请求不得触发下载。adapter 必须验证 provider
version、model identity、content type、bounded response、semantic output、inventory pre/post equality、timeout 和
controlled restart。Ollama/LM Studio 是 adapter，不是 control plane。

Exit 需要真实 provider process、真实输出、真实 cancel disposition 和 digest-bound artifact。HTTP mock 只算
L1/L2 fixture，不算 real provider qualification。

### INF-2B — 独立 llama.cpp native worker

Deliverables:

- 单独 worker-host crate/process；
- pinned upstream commit、build flags、SBOM/license、GGUF/tokenizer/chat-template digest；
- inherited pipe 或 private worker UDS；
- stable C ABI、allocator ownership、shared-memory bounds；
- load/warm/submit/poll/cancel/drain/unload/health/stats 真实调用。

Exit 需要至少一个固定 GGUF 在一个真实设备输出至少一个 UTF-8 token，并覆盖 crash/OOM/protocol violation。
仅有 Rust supervisor state machine 不算 real worker execution。

### INF-2C — 真实取消闭环

Exit 需要：cancel requested→worker cancel sent→CancelAck；超时路径必须实际 kill worker、递增 generation、
拒绝旧 token，并证明 no remote fallback。

### INF-3 — scheduler/cache/resource lease

先实现无 cache 的 EDF/WFQ 与 reservation ledger，再实现 tenant/workspace scoped prefix/KV cache。continuous
batching 只能在正确性、取消和隔离已闭合后进入。cache key 必须包含 tenant、workspace、tuple、policy、
privacy class 和 backend generation。

### INF-4 — exact-device qualification

每个 `model × tokenizer × artifact × runtime × compiler × driver × device × quantization × context × batch/cache`
组合独立 receipt。交叉编译不替代目标设备执行。未知设备或 tuple 必须 `KNOWN_GAP_NOT_ROUTED`。

### INF-5 — 产品 shadow integration

只允许 `SHADOW_COMPARE_ONLY` 起步；primary path 不变。产品只发送 unprivileged request，不能发布 worker
事件。输出必须是 typed proposal/signal，不能直接写 Memory/KG/effect/route。进入 canary 前需独立 rollout
contract、kill switch、privacy review 和 performance receipt。

### INF-7 — operator acceptance 与 promotion

只能由独立 operator 在 exact-head source、real-model、fault、hardware、privacy 和 rollback evidence 全部通过后
执行。当前 PR 不具备该 authority。

---

## 7. 测试金字塔

### L0 — source/truth

- JSON parse/schema；
- current status/stage/ledger 一致；
- workflow 引用路径存在；
- all authority closed；
- no raw prompt/media in docs/receipts；
- exact parent/source binding。

### L1 — pure contracts

- IDs/digest/generation/fence；
- role authorization；
- admission/resource accounting；
- lifecycle invalid transition；
- token-chain vectors；
- terminal immutability；
- receipt canonical round-trip。

### L2 — daemon hermetic E2E

- same-user UDS；
- unauthorized worker/operator messages；
- connection exhaustion/timeout；
- queue/inflight/running limits；
- durable receipt/restart recovery；
- worker fixture crash/cancel/stale event。

### L3 — real software runtime

- fixed Ollama/LM Studio and native worker；
- true model inventory and model digest；
- minimal semantic generation；
- streaming/tool-call capability；
- cancellation ACK or unsupported-before-dispatch；
- no implicit install/fallback。

### L4 — hardware/performance

- cold load、warm TTFT、prefill/decode tok/s；
- p50/p95/p99；
- peak RSS/VRAM/KV cache；
- cancellation latency；
- unload memory recovery；
- thermal/power；
- direct-native parity。

---

## 8. CI 设计

唯一 required aggregate 只聚合真实存在的 job：

```text
source-truth
rust-linux-x64
rust-linux-arm64 (runner available 时)
rust-macos-intel (runner available 时)
real-provider (仅当固定 tuple/runner 明确配置)
native-worker-device (仅当固定 artifact/runner 明确配置)
aggregate
```

未配置的 real-model/hardware job 必须在 tracked status 中显示 `BLOCKED_EXTERNAL_DEPENDENCY`，不能用一个
成功的 source aggregate 把它隐式标为通过。source-only aggregate 的名称必须包含 `qualification-only`。

最小 Rust gate：

```bash
cargo fmt -p codex-hepta-infer-core -p codex-hepta-infer-client -p codex-hepta-inferd -- --check
cargo check --locked --all-targets -p codex-hepta-infer-core -p codex-hepta-infer-client -p codex-hepta-inferd
cargo test --locked --all-targets -p codex-hepta-infer-core -p codex-hepta-infer-client -p codex-hepta-inferd
cargo clippy --locked --all-targets --no-deps \
  -p codex-hepta-infer-core -p codex-hepta-infer-client -p codex-hepta-inferd -- -D warnings
```

future crate 必须在同一 source candidate 中先创建、进入 workspace、更新 lockfile并通过 inventory gate，之后才
可加入 matrix。

---

## 9. 性能与模型分级

没有硬件探测时不得统一默认 20B。建议 profile：

| Profile | 目标 | 默认策略 |
|---|---|---|
| pocket/low-memory | 1–3B quantized | 首选，短 context，有界 KV |
| standard-local | 7–8B quantized | 硬件预算满足后 opt-in |
| high-capacity | 20B+ | 显式 opt-in，不自动驻留 |
| external-local | Ollama/LM Studio | adapter，固定 tuple，无隐式下载 |
| remote | 未来 lane | 当前冻结，禁止静默 fallback |

在真实基线冻结前只允许 provisional gate：warm sidecar overhead ≤ `max(10ms, 10%)`，throughput ≥ direct-native
90%，RSS/VRAM ≤ direct-native 1.1×。任何失败 tuple 必须从 router 移除并标记 `KNOWN_GAP_NOT_ROUTED`。

---

## 10. 文件级实施图

当前三 crate 内先闭合控制面：

```text
codex-rs/hepta-infer-core/
  src/controller.rs   resource accounting + token integrity + terminal release
  src/protocol.rs     role-aware public contract / stable typed errors
  src/worker.rs       worker capability/session/cancel contracts
  src/tests.rs        negative/fault vectors

codex-rs/hepta-infer-client/
  src/lib.rs          unprivileged API only
  src/tests.rs        role-denial and bounded transport

codex-rs/hepta-inferd/
  src/lib.rs          semaphore/timeouts/journal/recovery/public dispatch
  src/shadow.rs       qualification-only internal fixture; no product authority
  src/tests.rs        slowloris/restart/recovery/resource tests
```

真实 backend 在 contract 稳定后再创建最少数量 crate；禁止先铺八个空 crate。

---

## 11. Rollback

- docs/truth：删除新增 V3/V4 current files，不修改历史 receipt；
- CI：恢复上一工作流，但不得恢复不存在路径；
- public role boundary：feature flag 只能回到更严格的拒绝模式，不能开放未授权消息；
- controller/journal：schema versioned，旧 receipt 保持只读；
- adapter/native worker：从 allowlist 移除 tuple、drain/kill worker、递增 generation、保留历史 evidence；
- product shadow：kill switch 关闭 shadow，primary path 不变。

---

## 12. Definition of Done

### Source blocker closed

必须同时满足：

- exact-head source inventory 通过；
- fmt/check/test/strict Clippy 非空执行；
- role negative tests、resource fault tests、token integrity vectors、journal recovery tests通过；
- all authority false；
- source artifact 精确声明哪些 real/hardware gates 未执行。

### Runtime blocker closed

还必须满足：

- 真实 provider 或 native worker 加载固定模型并生成真实 token；
- artifact/model/tokenizer/template/runtime/device 全摘要绑定；
- cancellation ACK/kill、crash、OOM、restart、stale token 均有执行证据；
- 无隐式下载、模型切换、远端 fallback。

### Production gap closed

还必须满足：

- product shadow/canary 资格；
- exact-device performance/privacy/security/rollback receipts；
- independent operator acceptance；
- 明确 promotion/release commit。

在最后一组条件满足前，合法最高状态仍是：

```text
EXECUTED_PASSED_QUALIFICATION_ONLY
production_qualified=false
operator_accepted=false
promoted=false
released=false
```

---

## 13. 当前执行顺序

1. 落盘 INF-T0 文档和机器真值；
2. 修复 INF-T1 不存在路径与 required aggregate；
3. 在现有三 crate 闭合 INF-H1/H2/H3/H4；
4. exact-head CI 迭代到 source-only aggregate 绿色；
5. 只有固定本地模型与 runner 可用时才执行 INF-2A/2B/2C；
6. INF-3/4/5/7 按依赖推进，禁止越级。

该顺序优先消除安全与证据假闭环，再扩展实际推理能力。