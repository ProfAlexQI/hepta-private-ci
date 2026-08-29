# HEPTA 本地大模型推理运行时开发计划 V4

> Plan ID: `HEPTA-INFERENCE-RUNTIME-V4`  
> Version: `4.0.0`  
> Date: `2026-08-30`  
> Repository: `ProfHepta/hepta-private-ci`  
> Canonical default branch: `integration/vnext-main-20260811`  
> Canonical default head observed: `fe0889ecd46a5fc89de7b1ff3f28158c133a3502`  
> Development PR: `#73`  
> Development branch: `codex/hepta-inference-gap-closure-20260829`  
> Development base: `6f3286f99f5576b4cc545003889437bf388bb28a`  
> Observed parent before this plan: `2d6fa9c01af0c98a81cd804b1f35ba6a171f0c10`  
> Observed parent tree: `4bbc59481ca90ccd62e702baa3b865a221d4b2e5`  
> Supersedes for current execution: `HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V3.md`  
> Status: `SOURCE_BOUND_ACTIVE_PLAN / QUALIFICATION_ONLY`

## 0. 执行摘要

V4 将本地推理模块从“控制面原型、历史证据栈与未来实现混杂”收敛为一条可以逐层证明的闭环：

```text
unprivileged product/shadow client
        │ owner-local public UDS
        ▼
hepta-inferd
  identity / admission / quota / deadline / journal
  capability minting / scheduler / runtime supervision
        │
        ├── authenticated private provider lane
        │       └── fixed local Ollama or LM Studio tuple
        │
        └── authenticated inherited worker lane
                └── isolated hepta-infer-worker-host
                        └── pinned native runtime / fixed GGUF tuple
```

V4 作出以下强制修正：

1. 将仓库内源码缺失、exact-head 执行失败、真实软件证据、真实设备证据和独立授权拆成不同 gap class；
2. 将 H1–H4 已落盘的控制面硬化标为 `SOURCE_PRESENT_NOT_QUALIFIED`，不再重复规划已有源码；
3. 将 request capability、private worker/operator channel、真实取消、retention、scheduler、provider host、worker host 和 product shadow bridge列为可由仓库源码闭合的工作包；
4. 历史 INF-0/INF-0C receipt 只读，历史 gate 不得再以当前状态字段决定后继提交资格；
5. 只允许一个 current-plan pointer、一个当前状态、一个 stage matrix、一个 implementation status 和一个 blocker ledger；
6. 源码、真实软件、真实设备、产品 shadow、operator acceptance、promotion/release 分层验收，低层绿色不得提升高层状态。

当前最高合法结论：

```text
source_candidate_present=true
source_candidate_qualified=false
real_provider_executed=false
real_native_model_executed=false
hardware_qualified=false
product_wired=false
operator_accepted=false
promoted=false
released=false
```

## 1. 权威边界

本 PR 必须保持 Draft，只拥有 qualification authority：

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

禁止：

- 自行合并 PR、激活生产 listener、修改默认产品路由；
- 将 mock HTTP、fixture token、in-process supervisor 或交叉编译称为真实模型/设备执行；
- 普通请求下载模型、隐式切换模型或静默远端 fallback；
- worker/provider 获得 Memory、KG、effect、route 或 fleet 写权限；
- tracked JSON 自行声明未知的最终 commit/tree；
- 修改历史 receipt 来“修绿”当前 head；
- 为未实现模块铺设空 crate 或先把不存在路径加入 required workflow。

“最高权限”不改变证据真实性、独立 operator 和 promotion 权限边界。

## 2. 单一真值

### 2.1 优先级

1. exact commit/tree 与 tracked source；
2. exact-head、assigned runner、非空 steps 的 CI；
3. digest-bound append-only artifact/receipt；
4. `HEPTA_INFERENCE_CURRENT_PLAN_V1.json`；
5. current status、stage matrix、implementation status、blocker ledger；
6. 本计划；
7. PR 描述和人工说明。

低优先级材料不得覆盖高优先级事实。

### 2.2 状态机

```text
NOT_STARTED
PLANNED
SOURCE_PRESENT_NOT_QUALIFIED
EXECUTED_FAILED
BLOCKED_UPSTREAM
BLOCKED_EXTERNAL_EVIDENCE
EXECUTED_PASSED_QUALIFICATION_ONLY
SHADOW_QUALIFIED
OPERATOR_ACCEPTED
PROMOTED
RELEASED
```

`steps=[]`、`runner_id=0`、checkout 未发生、skipped、cancelled、timed_out、neutral、stale、startup_failure 和 action_required 均不是 PASS。

### 2.3 Gap class

| 类型 | 定义 | 当前 PR 能否闭合 |
|---|---|---|
| `REPOSITORY_SOURCE` | 源码、测试、workflow、文档缺失或错误 | 能 |
| `EXACT_HEAD_EXECUTION` | 源码存在但 exact-head CI 未通过 | 依赖 runner 实际执行 |
| `REAL_SOFTWARE_EVIDENCE` | 需要真实 provider/model process | 依赖预安装软件和固定模型 |
| `EXACT_DEVICE_EVIDENCE` | 需要目标硬件、driver、性能/资源证据 | 依赖目标设备 runner |
| `INDEPENDENT_AUTHORITY` | privacy/operator acceptance/promotion/release | 不能由当前作者自证 |

## 3. 已核验基线

### 3.1 已存在源码

`codex-hepta-infer-core` 已包含 bounded identity/digest/request、canonical CBOR、message role、queue/inflight/running accounting、deadline sweep、rolling token digest、token count/limit、adapter tuple、native ABI/manifest/shared-region 和 in-process supervisor contract。

`codex-hepta-infer-client` 已包含 owner-local UDS、unprivileged API、tuple/capability fail-closed 和 shadow-only route type。

`codex-hepta-inferd` 已包含 same-user public UDS、connection semaphore、frame/write timeout、atomic receipt write、parent directory sync、startup recovery、receipt file/byte budget、terminal record durable 后内存释放和 durable request-id replay guard。

### 3.2 当前 exact-head 结果

Observed head `2d6fa9c01af0c98a81cd804b1f35ba6a171f0c10`：

- V3 source/authority gate实际执行并通过；
- Rust 1.95 Linux x64、Linux ARM64、macOS Intel均在 `cargo fmt --check` 失败；
- check/test/clippy 因 formatting 失败被跳过；
- source candidate 为 `EXECUTED_FAILED`；
- historical INF-0C workflow 因旧状态字段重新执行失败，属于历史与当前资格链未隔离。

### 3.3 仓库内仍缺失

- daemon-minted request capability；
- authenticated private worker channel；
- authenticated private operator channel；
- worker session nonce 与 one-time grant；
- running cancel request/ACK/timeout kill/generation rollover；
- receipt TTL、tombstone 或 operator-approved compaction；
- daemon 后真实 provider execution host；
- isolated worker-host process 与 ABI invocation；
- deterministic no-cache EDF/WFQ scheduler 与 reservation ledger；
- typed product shadow bridge、kill switch 和 rollback fixture。

### 3.4 外部证据缺失

- 固定 Ollama/LM Studio + 预安装模型真实输出；
- 固定 GGUF + pinned runtime 真实 token；
- exact-device 性能、内存、功耗、取消和卸载；
- privacy review；
- independent operator acceptance、promotion、release。

## 4. 安全不变量

### 4.1 同 UID 不等于同 authority

```text
public client endpoint
  Ping / Admit / CancelOwn / GetOwnReceipt / bounded Snapshot

private worker endpoint or inherited pipe
  WorkerHello / Lease / StartAck / Token / Complete / Failure / CancelAck / Health

private operator endpoint
  Restart / Drain / RegisterTuple / RemoveTuple / InventoryRefresh / CompactReceipts
```

public peer 即使同 UID，也不能发送 worker/operator event。

### 4.2 Capability fence

daemon 为每次 admission 生成：

```text
request_capability = HMAC-SHA256(
  daemon_epoch_secret,
  tenant || workspace || agent || task || request_id ||
  agent_generation || request_generation || cancel_generation ||
  backend_generation || worker_session_nonce ||
  tuple_digest || policy_digest || budget_id || deadline
)
```

要求：

- daemon epoch/session secret 使用 OS CSPRNG，至少 256 bit；
- capability 至少 128-bit 不可预测，日志/receipt 只允许记录 digest；
- request grant 一次性绑定 worker session、backend generation 和 request generation；
- restart 后旧 grant 全部失效；
- public client永远不能获得 worker session secret；
- secret 类型禁止 `Debug`、序列化、复制和持久化，并在 drop 时清零。

### 4.3 输出完整性

```text
chain_0 = SHA256(domain || complete request fence)
chain_n = SHA256(chain_(n-1) || sequence_n || token_digest_n || token_byte_length_n)
```

完成必须满足：

```text
reported_output_tokens == accepted_token_count
reported_result_digest == chain_n
accepted_token_count <= output_token_limit
accepted_token_bytes <= output_byte_limit
sequence strictly monotonic
```

receipt 必须绑定 tuple、runtime、worker session digest、termination reason、token count/bytes 和 chain digest。

### 4.4 取消

```text
CancelRequested -> CancelDispatched -> CancelAck -> Cancelled receipt
```

超时必须执行：

```text
CancelAckTimeout
  -> kill worker process
  -> wait/reap child
  -> increment backend_generation
  -> fail-close every affected request
  -> reject every stale event
  -> forced_worker_termination=true
```

transport disconnect、future drop 或从 map 删除均不等于 backend acknowledgement。

### 4.5 有界资源

必须独立配置并测试：

```text
max_connections
max_frame_bytes
read_timeout / write_timeout / idle_timeout
max_queue_global / per_tenant
max_inflight_global / per_tenant
max_running_global / per_tenant / per_tuple
max_prompt_bytes / output_tokens / output_bytes / token_bytes
max_worker_processes / worker_active_requests
max_receipt_files / bytes / age
max_terminal_index_entries
max_model_resident_bytes / RSS / VRAM / KV / shared memory
```

所有 reservation 只能由一个 owner 修改，并在 every terminal/fault/restart path exactly once 释放。

## 5. 组件与 crate 策略

保留：

```text
hepta-infer-core
  protocol / identity / controller / capability / scheduler contracts

hepta-infer-client
  public unprivileged transport and typed shadow client

hepta-inferd
  public listener / private channels / journal / runtime supervision
```

只新增一个必需 runtime crate：

```text
hepta-infer-worker-host
```

只有在 `Cargo.toml`、process entry point、private protocol、pinned ABI loader、hermetic fake backend、crash/cancel/stale-event E2E、Cargo.lock/Bazel lock 和 workflow inventory 同一提交出现时才加入 workspace。

scheduler 先作为 core 私有模块；provider host 先作为 inferd 私有模块；product shadow 接入现有明确 owner，不预建多个空 crate。

## 6. Package 依赖图

```text
INF-D0 V4 docs/truth
  └─ INF-Q0 exact-head CI + historical gate isolation
      └─ INF-S1 capability/private role channels
          ├─ INF-S2 real cancellation lifecycle
          ├─ INF-S3 receipt retention/compaction
          └─ INF-S4 scheduler/reservation no-cache
               ├─ INF-R1 provider runtime host
               └─ INF-R2 isolated worker-host + ABI harness
                    ├─ INF-E1 real provider/model evidence
                    └─ INF-E2 real native model/device evidence
                         └─ INF-R3 product shadow bridge
                              └─ INF-E3 performance/privacy/rollback
                                   └─ INF-A1 independent acceptance/promotion
```

## 7. 详细工作包

### INF-D0 — V4 文档与机器真值

Deliverables：当前计划指针、V4 plan/current status/implementation status/stage matrix/blocker ledger/evidence contract。

Exit：全部文件共享 plan/repository，historical files 保持不变，authority 全关闭，当前 pointer 唯一。

### INF-Q0 — exact-head 资格链

Deliverables：

- `scripts/hepta-inference-v4-source-truth.py`；
- 修复 owned Rust files 的 rustfmt 漂移；
- V4 source truth + Linux x64/ARM64/macOS Intel fmt/check/test/clippy；
- aggregate 必须拒绝 skipped/cancelled/empty-runner；
- historical INF-0C gate 只验证 immutable ancestor/receipt；
- real provider job 继续保持严格独立。

Exit：三平台非空执行并通过，artifact 绑定 exact head/tree，source receipt仍诚实声明真实模型和设备未执行。

### INF-S1 — capability 与 private role channel

Deliverables：

- OS CSPRNG daemon epoch secret、worker session nonce、one-time grant；
- constant-time capability verification；
- private worker protocol 与 inherited pipe/owner-local private UDS；
- worker handshake challenge、protocol version、PID/peer/session fence；
- private operator endpoint，默认不创建；
- secret zeroization/no-debug/no-serialization；
- public raw-peer negative tests。

Exit：wrong capability/session/generation/PID 被拒；restart 使旧 grant失效；receipt/log不含 secret；public socket 无法执行 worker/operator operation。

### INF-S2 — running cancel 与 worker fault

Deliverables：

- `CancelRequested/CancelDispatched/CancelAck/CancelTimedOut` lifecycle；
- bounded cancel deadline；
- child-process handle owner；
- timeout kill + wait + generation rollover；
- affected requests batch fail-close；
- stale event rejection；
- termination receipt 记录真实 disposition。

Exit：queued cancel 原子完成；running cancel无 ACK时不能产生正常 Cancelled；ACK、timeout kill、worker crash均有 hermetic child-process E2E；无远端 fallback。

### INF-S3 — retention 与 compaction

Policy：默认 append-only minimum retention；TTL只影响 terminal receipt；compaction仅由 operator-only command 或启动前固定 policy触发；删除前写 tombstone/compaction receipt；file/rename/parent fsync；immutable digest index阻止 request-id replay。

Exit：expired receipt deterministic compact；active/young receipt不删除；tombstone/delete/reindex任意阶段崩溃可恢复；budget exhaustion仍 fail closed。

### INF-S4 — scheduler 与 reservation ledger

第一阶段禁止 cache 和 continuous batching。实现 deterministic EDF primary order、tenant weighted-fair tie-break、global/per-tenant/per-tuple reservation、admission/dispatch 两阶段、cancel/deadline/crash/restart exactly-once release、starvation bound 和 injected clock。

Exit：property tests deterministic；无法 overcommit；配置边界内无 tenant starvation；journal/recovery一致。

只有 S4 通过后才能引入 tenant/workspace scoped prefix/KV cache；cache key必须绑定 tenant/workspace/privacy class/policy/tuple/runtime/backend generation/template/context prefix。

### INF-R1 — 本地 provider runtime host

Source：fixed `ProviderTupleManifest`；loopback-only、no proxy、no redirect；bounded request/response/content-type；provider version/capability handshake；pre/post inventory digest；model identity/response model verification；semantic parser；timeout/controlled disposition；禁止 install/model switch/remote fallback。

Source exit：hermetic fake provider覆盖 success/malformed/oversize/timeout/model mismatch/inventory drift；daemon通过 private runtime lane执行；fixture明确 `real_provider_executed=false`。

Evidence exit：真实预安装 provider process、固定模型真实语义输出、inventory before==after、digest-bound artifact；不支持 cancel acknowledgement时在 dispatch 前 fail closed。

### INF-R2 — isolated native worker-host

Source：child process + inherited private handles；stable versioned frame protocol；pinned ABI loader；runtime/model/tokenizer/template/SBOM/license/build flags manifest；bounded shared memory；load/warm/submit/poll/cancel/drain/unload/health/stats；watchdog/crash/OOM/protocol violation；hermetic fake backend。

Source exit：真实 child-process E2E，而非 in-process map；session/grant/cancel ACK/forced kill通过；fake token只能声明 fixture。

Evidence exit：pinned runtime真实调用、fixed GGUF真实加载、至少一个 UTF-8 token、crash/OOM/cancel/stale-event receipt、无远端 fallback。

### INF-E1 / INF-E2 — 真实软件与设备证据

每个 receipt 必须绑定：

```text
head/tree/workflow/job/runner
runtime binary/build/SBOM/license
model/tokenizer/template/artifact/quantization
driver/device/compiler
request fixture/output/token chain
cancel/kill disposition
inventory before/after
authority/fallback flags
```

模型 artifact 不进入普通 git history；只能使用受控 runner cache/artifact store并校验预期 SHA-256。

### INF-R3 — typed product shadow bridge

起点必须是：

```text
route_mode=SHADOW_COMPARE_ONLY
primary_route_changed=false
shadow_output_authoritative=false
```

桥接要求：explicit config + kill switch；bounded prompt projection；receipt/log无 raw prompt；typed proposal/signal output；无 Memory/KG/effect/route write；deterministic sampling/comparison budget；timeout/cancel与 primary隔离；rollback恢复未接入状态。

### INF-E3 / INF-A1 — 性能、隐私与 promotion

Exact-device measurements：cold load、warm TTFT p50/p95/p99、prefill/decode tok/s、peak RSS/VRAM/KV/shared memory、cancel/forced-kill latency、unload memory recovery、thermal/power（可得时）、sidecar/direct-native parity。

临时阈值：

```text
warm sidecar overhead <= max(10 ms, 10%)
throughput >= 90% direct-native
RSS/VRAM <= 1.10x direct-native
```

最终 threshold 必须由独立 operator 在 evidence review 后冻结。当前 PR不能自行 acceptance、promotion 或 release。

## 8. 协议与兼容

- public CBOR v1保持 decode兼容；
- private worker/operator protocol独立从 v1开始；
- capability不追加到 public privileged message，public endpoint根本不接受该类消息；
- receipt schema versioned，旧 receipt read-only；
- unknown field/kind/version fail closed；
- frame canonical、definite-length、bounded；
- upgrade使用 daemon/worker supported-version intersection；
- downgrade必须显式配置且不能降低 authority fence。

## 9. 可观测性与隐私

允许：request-id/tuple/policy digest、state transition、latency bucket、resource counters、worker generation/session digest、error code、termination reason。

禁止：raw prompt、raw token、model output、capability/session secret、user filesystem path、Memory/KG payload、provider authorization material。

metrics必须 bounded-cardinality；tenant/request标识不得成为无限 label。

## 10. 测试矩阵

### L0 truth/source

current pointer唯一；JSON parse/schema；current docs共享 plan/repository；workflow paths和Cargo packages存在；historical receipts unchanged；authority closed；tracked evidence无 raw prompt/output。

### L1 pure contracts

identity/digest/generation；role/capability/session；token chain vectors；scheduler/reservation；retention decision；canonical protocol round-trip；invalid transition/overflow。

### L2 daemon hermetic E2E

public UDS、privileged denial、private handshake/grant denial、slowloris/timeouts、deadline sweep、receipt crash recovery/compaction、fake provider malformed/timeout/inventory drift、child worker start/token/cancel ACK/kill/crash/stale event。

### L3 real software

real Ollama/LM Studio fixed tuple；real worker/runtime/GGUF；semantic output；cancellation disposition；no install/switch/fallback；inventory stability。

### L4 exact device/product

performance/resource/thermal；product shadow safety；kill switch；privacy review；rollback；independent operator acceptance。

## 11. CI 拓扑

```text
v4-source-truth
  ├── rust-linux-x64
  ├── rust-linux-arm64
  └── rust-macos-intel
       └── v4-source-qualification-only-aggregate
```

Optional evidence workflows必须分离：

```text
real-provider-fixed-tuple
native-worker-exact-device
product-shadow
operator-acceptance
```

规则：optional evidence未配置时为 `BLOCKED_EXTERNAL_EVIDENCE`；current aggregate不依赖历史 stage gate；historical workflow只验证 immutable receipt/ancestor；artifact name和receipt绑定 exact head/tree/job/runner；rerun追加新 artifact，不覆盖历史 artifact。

## 12. 模型与硬件分级

| Profile | Model | Default |
|---|---|---|
| pocket | 1–3B quantized | 首选、短 context、有界 KV |
| standard-local | 7–8B quantized | 预算满足后 opt-in |
| high-capacity | 20B+ | 显式 opt-in，不自动驻留 |
| external-local | fixed Ollama/LM Studio tuple | adapter lane |
| remote | future | 当前冻结 |

无硬件探测时禁止统一默认 20B。未知 tuple/device必须返回 `KNOWN_GAP_NOT_ROUTED`。

## 13. Rollback

- docs/truth：回退 current pointer，历史 receipt不改；
- CI：只回退到路径存在且不提升权限的 workflow；
- capability/private channel：失败时关闭 private endpoint，不开放 public privileged message；
- worker/provider：从 tuple allowlist移除、drain/kill、递增 generation；
- scheduler/cache：回退 no-cache单 worker fail-closed；
- product shadow：kill switch关闭，primary不变；
- retention：停止 compaction但不删除已有 receipt；
- promotion：必须独立授权提交，当前 PR不得创建生产状态。

## 14. Definition of Done

### 14.1 Repository source closed

- V4 current pointer与truth一致；
- exact-head source gate通过；
- fmt/check/test/clippy三平台非空执行；
- INF-S1/S2/S3/S4/R1-source/R2-source/R3-source全部实现并有 hermetic E2E；
- 无 authority escalation；
- source receipt对未执行的真实模型/设备保持 false。

### 14.2 Runtime evidence closed

- fixed real provider和/或native worker真实模型执行；
- model/runtime/device identity摘要绑定；
- real cancel ACK/kill/crash/OOM/stale event；
- no install/switch/remote fallback；
- exact-device resource/performance receipt。

### 14.3 Product gap closed

- shadow bridge与kill switch通过；
- privacy/security/rollback通过；
- independent operator acceptance；
- separate promotion/release commit。

14.3 完成前不得声称“所有产品 gap 已闭合”。仓库源码可以先达到 `MODULE_CLOSED_CANDIDATE_SOURCE_ONLY`，外部证据和独立 authority必须保持显式 blocker。

## 15. 当前执行队列

严格顺序：

1. `INF-D0`：V4 文档与唯一 current pointer；
2. `INF-Q0`：rustfmt、三平台 check/test/clippy、历史 INF-0C gate隔离；
3. `INF-S1`：capability/private worker/operator channel；
4. `INF-S2`：running cancel ACK/kill/generation；
5. `INF-S3`：retention/compaction；
6. `INF-S4`：no-cache scheduler/reservation；
7. `INF-R1`：provider host source + real provider evidence；
8. `INF-R2`：worker-host source + real native evidence；
9. `INF-R3`：typed product shadow source；
10. `INF-E3/A1`：exact-device、privacy、rollback、independent acceptance/promotion。

每个 package只允许：

```text
PACKAGE_QUALIFIED
BLOCKED_UPSTREAM
BLOCKED_EXTERNAL_EVIDENCE
BASE_DRIFT
STOP_CONDITION
```

不得用新计划替代执行，也不得用 source fixture替代真实模型、设备或 operator evidence。
