# HEPTA 本地大模型推理运行时开发计划 V5

> Plan ID: `HEPTA-INFERENCE-RUNTIME-V5`  
> Version: `5.0.0`  
> Date: `2026-08-30`  
> Repository: `ProfHepta/hepta-private-ci`  
> Canonical default branch: `integration/vnext-main-20260811`  
> Development PR: `#73`  
> Development branch: `codex/hepta-inference-gap-closure-20260829`  
> Audited PR head: `dbf68bfed0a2a034fb519c66a89725e2b74edce3`  
> V5 integration staging parent: `705c0916098c39ddd0ef9e6b069e1efe072f410b`  
> Supersedes for current execution: `HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V4.md`  
> Status: `SOURCE_BOUND_INTEGRATION_PLAN / QUALIFICATION_ONLY`

## 0. 执行摘要

V5 修正 V4 的核心误判：**独立组件存在、单元测试存在、fixture 子进程存在，并不等于运行时链路已接通。**

V4 候选 payload 已提供 capability、private protocol、running cancel、receipt retention、scheduler、provider host、worker-host 和 shadow bridge 的若干可测试组件；审计同时确认以下端到端断点仍然存在：

1. public admission 只携带 prompt digest/length，没有把真实 prompt 通过一次性、受限、可验证的输入租约交给 provider 或 worker；
2. private control、scheduler、provider host、worker-host 与 controller 尚未由一个 daemon-owned coordinator 串成同一状态机；
3. scheduler journal 目前只是调用者提供的内存事件切片，没有 crash-safe durable owner，也未接入 daemon admission/dispatch/terminal 路径；
4. provider host 是导出的 loopback helper，尚未绑定 request grant、统一 deadline、controller token chain、cancel disposition 或 terminal receipt；
5. worker-host 的可执行入口仍是 fixture mode，ABI loader 只调用 fixture symbol，尚未实现真实 load/warm/submit/poll/cancel/drain/unload/health/stats；
6. product shadow bridge 只是一段泛型比较代码，尚未接入明确产品 owner，kill switch 也未由独立 operator capability 控制；
7. 取消异常响应路径可能在未 kill 的活子进程上无限 `wait()`，且 worker-exit 与 forced-kill receipt 语义可能混淆；
8. receipt compaction 对未知普通文件和临时文件命名的处理仍需 fail-closed；
9. provider inventory 允许同名多 digest 条目，model digest 未强制 canonical digest；native library digest 检查与 `dlopen(path)` 之间存在路径替换窗口；
10. 真实 provider、真实 GGUF、exact-device、privacy、operator acceptance、promotion/release 仍没有可接受证据。

V5 的目标不是再增加孤立 crate，而是形成一条可证明的 daemon-owned 数据与控制链：

```text
agentd / explicit qualification product owner
        │
        │ public digest-only request + one-time prompt input lease
        ▼
hepta-inferd RuntimeCoordinator
  identity / policy / deadline / quota
  durable scheduler + reservation journal
  capability minting + private session ownership
  receipt journal + retention + replay fence
        │
        ├── fixed provider lane
        │     verified prompt lease -> bounded loopback provider
        │     -> controller-owned token/result digest -> receipt
        │
        └── native worker lane
              inherited authenticated handles + one-time grant
              -> isolated worker-host
              -> pinned ABI + fixed model handle
              -> token/cancel/fault events
```

当前最高合法结论保持：

```text
source_components_present=true
source_runtime_integrated=false
source_candidate_qualified=false
real_provider_executed=false
real_native_model_executed=false
hardware_qualified=false
product_shadow_wired=false
operator_accepted=false
promoted=false
released=false
```

## 1. 权威边界

本计划与当前 PR 仅拥有 qualification authority：

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

- 自合并 PR、修改默认产品路由、激活生产 listener 或把 qualification profile 当生产配置；
- 把 mock HTTP、fixture token、动态库回声函数、交叉编译或源码存在称为真实模型执行；
- 普通请求下载、安装、切换模型，读取代理配置，跟随 redirect 或静默远端 fallback；
- 将 raw prompt/token/output/capability/session secret 写入日志、receipt、metrics、Memory 或 KG；
- 由当前作者自行声明 privacy acceptance、operator acceptance、promotion 或 release；
- 为了“全绿”修改历史 receipt、降低 gate、跳过 runner、接受 `steps=[]` 或 `runner_id=0`。

## 2. 单一真值与状态机

### 2.1 优先级

1. exact commit/tree 与 tracked source；
2. exact-head、assigned runner、非空 steps 的 CI；
3. digest-bound append-only artifact/receipt；
4. current-plan pointer；
5. current status / implementation status / stage matrix / blocker ledger；
6. 本计划；
7. PR 描述和人工说明。

### 2.2 状态词汇

```text
NOT_STARTED
PLANNED
SOURCE_COMPONENT_PRESENT
SOURCE_INTEGRATED_NOT_QUALIFIED
EXECUTED_FAILED
BLOCKED_UPSTREAM
BLOCKED_EXTERNAL_EVIDENCE
EXECUTED_PASSED_QUALIFICATION_ONLY
SHADOW_QUALIFIED
OPERATOR_ACCEPTED
PROMOTED
RELEASED
```

### 2.3 Gap class

| Gap class | 含义 | 当前作者能否闭合 |
|---|---|---|
| `REPOSITORY_COMPONENT` | 独立源码、协议、测试、fixture 缺失 | 能 |
| `REPOSITORY_INTEGRATION` | 组件未接入同一 daemon/product owner | 能 |
| `EXACT_HEAD_EXECUTION` | exact-head CI 未实际通过 | 依赖 runner |
| `REAL_SOFTWARE_EVIDENCE` | 真实 provider/runtime/model process | 依赖固定软件与模型 |
| `EXACT_DEVICE_EVIDENCE` | 真实目标设备、driver、性能/资源 | 依赖设备 runner |
| `INDEPENDENT_AUTHORITY` | privacy/operator/promotion/release | 当前作者不能自证 |

## 3. 强制不变量

### 3.1 Public/private 分离

Public endpoint 只允许：

```text
Ping
AdmitDigestOnly
CancelOwnRequest
GetOwnReceipt
BoundedSnapshot
```

Worker/operator/provider event 永远不能通过 public endpoint；同 UID 不等于同 authority。

### 3.2 一次性输入租约

真实 prompt 必须通过 `PromptInputLease` 传递，至少绑定：

```text
tenant/workspace/agent/task/request
agent_generation/request_generation/cancel_generation
backend_generation/worker_session_digest/request_grant_digest
policy_digest/budget_id/deadline/model_tuple_digest
prompt_digest/prompt_byte_length
```

要求：

- public CBOR 不携带 raw prompt；
- 输入由 product owner 写入匿名 pipe、sealed memory object 或等价继承句柄；
- daemon 在 dispatch 前逐字节校验长度与 SHA-256；
- lease 一次性消费，重复、截断、超长、digest mismatch、EOF、stale generation 全部 fail closed；
- raw bytes 不持久化，不进入 Debug/serde/log/receipt/metric；
- lease owner 在 terminal/cancel/fault/restart 时 exactly-once release；
- provider/worker 只能读取其获授权 request 的输入，不能读取其他 tenant/workspace lease。

### 3.3 完整 request fence

每个调度、执行、事件和 receipt 至少绑定：

```text
TenantId / WorkspaceId / AgentId / TaskId / RequestId
agent_generation / request_generation / cancel_generation
backend_generation / worker_session_digest / request_grant_digest
PolicyDigest / ResourceBudgetId / deadline_unix_ms
ModelTupleDigest / PromptDigest / PromptByteLength
OutputTokenLimit / OutputByteLimit
```

### 3.4 输出完整性

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

provider 的整段文本响应必须由 daemon 切成受限事件或作为一个受限 token event，由 controller 计算最终 chain；provider/worker 不能自行声明未由已接收事件推导的 final digest。

### 3.5 统一总截止时间

connect、write、read、inventory-before、generate、inventory-after、cancel、kill 和 reap 必须共享一个 request total deadline。不得把每一步各自的 timeout 串联成无界总时长。

### 3.6 取消与进程终止

```text
CancelRequested
  -> CancelDispatched
  -> CancelAck
  -> Cancelled receipt(forced_worker_termination=false)
```

任何 timeout、畸形 ACK、stale ACK 或协议错误：

```text
try_wait
  -> if alive: kill
  -> bounded wait/reap
  -> generation rollover
  -> fail-close affected requests
  -> forced_worker_termination reflects actual kill, not merely Draining state
```

transport EOF、future drop、map removal 或 malformed response 不等于 ACK。

### 3.7 持久化与恢复

- scheduler reservation journal、terminal receipt 和 tombstone 都必须 temp-write/sync/rename/parent-sync；
- startup 先恢复 journal/receipt，再开放 public listener；
- unknown/non-regular/unexpected temporary store entry fail closed；
- durable replay fence 阻止 request-id 复用；
- prepared-but-not-committed reservation 在恢复时 deterministic rollback；
- committed-but-nonterminal reservation在 worker/session 不可恢复时 fail-close并释放；
- journal、released-id、terminal index 和 tombstone index均有独立上限。

## 4. 组件策略

保留：

```text
hepta-infer-core
  identity / protocol / controller / capability / scheduler contracts

hepta-infer-client
  public unprivileged transport / prompt-lease submission / typed shadow client

hepta-inferd
  public listener / RuntimeCoordinator / private channels
  scheduler journal / receipt store / provider lane / process supervision

hepta-infer-worker-host
  inherited private protocol / pinned ABI / isolated model process
```

不再新增空 crate。`PromptInputLease`、scheduler journal、provider coordinator 先分别作为现有 owner 的私有模块；只有出现不可接受依赖环时才拆 crate。

明确产品 owner 选择 `codex-hepta-agentd` 的 qualification-only shadow seam；默认构建与默认配置必须保持 disabled/kill-switched，primary App Server 输出始终权威。

## 5. Package 依赖图

```text
INF-D0  V5 docs/truth
  └─ INF-Q0 exact-head source and CI
      ├─ INF-I1 prompt input lease
      └─ INF-S1 authenticated private control
           ├─ INF-S2 bounded cancel/kill/reap
           ├─ INF-S3 receipt retention/recovery
           └─ INF-S4 durable scheduler/reservation integration
                ├─ INF-R1 provider lane integration
                └─ INF-R2 worker-host/ABI integration
                     ├─ INF-E1 real provider evidence
                     └─ INF-E2 real native model/device evidence
                          └─ INF-R3 agentd shadow integration
                               └─ INF-E3 privacy/performance/rollback
                                    └─ INF-A1 independent acceptance/promotion
```

## 6. 详细工作包

### INF-D0 — V5 文档与机器真值

Deliverables：

- V5 plan；
- current-plan pointer V2；
- current status V5；
- implementation status V4；
- stage matrix V6；
- blocker ledger V3；
- evidence contract V2；
- source-truth gate V2。

Exit：

- 当前文件共享同一 plan/repository/ref；
- V4 与历史 receipts 保持只读；
- component、integrated、qualified、real software、device、product、operator状态分别表达；
- authority 全关闭。

### INF-Q0 — exact-head 资格链

Deliverables：

- Linux x64、Linux ARM64、macOS Intel 对 owned crate 的 Rust 1.95 fmt/check/test/clippy；
- Cargo/Bazel lock exact-head materialization；
- source inventory、no-empty-runner、no-skipped aggregate；
- artifact 绑定 head/tree/parent/job/runner/toolchain；
- historical gate 只验证 immutable ancestor/receipt。

Exit：required jobs 全部非空执行并通过；真实模型/设备/product/operator字段仍保持 false。

### INF-I1 — PromptInputLease

Deliverables：

- one-time lease descriptor；
- inherited/anonymous bounded transport；
- exact length + digest verification；
- tenant/workspace/request/generation/capability fence；
- deadline/cancel/fault cleanup；
- zero-persistence/no-debug/no-serde；
- truncation/oversize/replay/cross-request/stale-generation fault tests。

Exit：provider/worker 能获得真实 prompt bytes，但 public wire、receipt、日志和状态文件均无 raw prompt。

### INF-S1 — private control integration

Deliverables：

- OS CSPRNG epoch/session/bootstrap secrets；
- private canonical protocol；
- inherited worker handles；
- handshake challenge、PID/session/generation fence；
- one-time request grant与per-event MAC；
- operator inherited handle或默认关闭的独立 endpoint；
- secret zeroization/no-debug/no-serialization；
- RuntimeCoordinator实际持有并调用 private control state。

Exit：wrong PID/session/grant/tag/generation/replay被拒；public raw peer无法调用 privileged operation；restart使旧 secret/grant失效。

### INF-S2 — bounded cancellation

Deliverables：

- explicit cancel phases；
- one total cancel deadline；
- ACK、malformed、EOF、crash、hang、stale event E2E；
- alive check、kill、bounded reap；
- actual-kill disposition；
- generation rollover与受影响请求批量 fail-close；
- scheduler/input lease/request grant exactly-once release。

Exit：无 ACK 永远不能产生正常 Cancelled；任何异常路径都不能无限等待子进程。

### INF-S3 — receipt retention

Deliverables：

- append-only minimum retention；
- operator-only或启动前固定 compaction；
- tombstone-before-delete；
- strict store namespace；
- bounded receipt/tombstone/index/file/byte/event budgets；
- crash injection at every fsync/rename/delete boundary。

Exit：young/active receipt不删除；expired receipt deterministic compact；unknown/nonregular/temp-name异常 fail closed；request-id replay保持禁止。

### INF-S4 — durable scheduler/reservation

Deliverables：

- deterministic EDF primary + weighted-fair arbitration；
- cost在enqueue时必须小于所有适用上限；
- global/tenant/tuple reservations；
- prepared/committed/released durable journal；
- RuntimeCoordinator admission→reserve→dispatch→terminal接线；
- crash recovery；
- bounded journal/released-id state；
- terminal/cancel/deadline/crash/restart exactly-once release。

Exit：scheduler不是孤立 helper；daemon E2E证明无法 overcommit、无配置内 starvation、恢复后 snapshot与账本一致。

### INF-R1 — provider lane integration

Deliverables：

- fixed canonical manifest；
- canonical provider/model digest；
- duplicate model identity拒绝；
- direct loopback socket、no DNS/proxy/redirect；
- single total deadline；
- prompt lease消费；
- request grant/policy/tuple/generation fence；
- controller-owned token/result chain；
- cancel capability在dispatch前证明；
- inventory before/after；
- RuntimeCoordinator provider route；
- hermetic daemon E2E。

Source exit：fixture receipt明确 `real_provider_executed=false`。Evidence exit需要真实预安装 provider+model artifact。

### INF-R2 — worker-host/ABI integration

Deliverables：

- worker executable支持非fixture manifest模式；
- inherited authenticated private protocol直接使用 S1 grant/event MAC；
- PromptInputLease；
- stable ABI：load/warm/submit/poll/cancel/drain/unload/health/stats；
- verified open handle消除 digest-check/`dlopen(path)` 替换窗口；
- GGUF/tokenizer/template/runtime/SBOM/license/device/build flags identity；
- process resource limits/watchdog；
- controller/scheduler/receipt统一接线；
- fixture与real receipt不可混淆。

Source exit：真实 child-process fake ABI E2E。Evidence exit需要 pinned runtime + fixed GGUF + target device。

### INF-R3 — product shadow integration

Deliverables：

- `codex-hepta-agentd` qualification-only owner seam；
- explicit disabled-by-default config；
- operator-controlled kill switch，普通产品调用方不能自行 clear；
- full request fence（含 generations/policy/budget/deadline）；
- PromptInputLease创建与销毁；
- bounded sampling/comparison；
- timeout触发shadow cancel/kill，不只drop future；
- primary输出byte-identical返回；
- typed non-authoritative signal；
- Memory/KG/effect/route/fleet写权限全 false；
- rollback恢复“未接入”行为。

Exit：实际 product owner E2E，而非仅测试泛型 bridge。

### INF-E1 / INF-E2 / INF-E3 / INF-A1

真实证据必须逐层独立：

- E1：真实固定 Ollama/LM Studio + 模型；
- E2：真实 pinned native runtime + GGUF + exact device；
- E3：性能、资源、隐私、shadow rollback；
- A1：独立 operator acceptance、单独 promotion/release commit。

当前作者不得伪造这些状态；未提供 runner/model/operator时合法终态是 `BLOCKED_EXTERNAL_EVIDENCE` 或 `BLOCKED_INDEPENDENT_AUTHORITY`，不是 PASS。

## 7. 测试矩阵

### L0 truth/source

- current pointer唯一；
- V5 machine files schema一致；
- owned path与workflow package存在；
- historical receipt immutable；
- authority closed；
- tracked evidence无 raw prompt/output/secret。

### L1 pure contracts

- identity/digest/generation；
- capability/session/MAC/replay；
- PromptInputLease length/digest/replay；
- token chain vectors；
- scheduler ordering/accounting/recovery；
- retention/tombstone decision；
- overflow与invalid transition。

### L2 daemon hermetic E2E

- public privilege denial；
- private handshake/grant/event；
- prompt lease handoff；
- admission→schedule→dispatch→token→receipt；
- provider malformed/oversize/timeout/duplicate identity/inventory drift；
- worker ACK/hang/crash/OOM/malformed/stale event；
- bounded kill/reap；
- journal/receipt crash recovery；
- no install/switch/fallback。

### L3 product hermetic E2E

- agentd default disabled；
- explicit qualification shadow；
- kill switch authority；
- primary byte identity；
- timeout cancel；
- rollback；
- no Memory/KG/effect/route/fleet write。

### L4 real software/device

- fixed provider/model；
- pinned native runtime/GGUF；
- exact device/performance/resource/privacy；
- independent operator acceptance。

## 8. CI 拓扑

```text
v5-source-truth
  -> rust-linux-x64
  -> rust-linux-arm64
  -> rust-macos-intel
  -> daemon-integration-hermetic
  -> agentd-shadow-hermetic
  -> source-only aggregate
```

外部 evidence workflow 必须独立且严格：

```text
real-provider-fixed-tuple
native-worker-exact-device
product-device-performance-privacy-rollback
operator-acceptance-promotion
```

Optional evidence未配置时保持 blocker；不得让 optional skip 变成 PASS，也不得让外部 evidence 失败污染已通过的 source-only truth。

## 9. Definition of Done

### 9.1 Repository source closed

- INF-D0/Q0/I1/S1/S2/S3/S4/R1-source/R2-source/R3-source 全部接入并通过 exact-head hermetic CI；
- scheduler/provider/worker/shadow不再只是孤立 helper；
- raw prompt只经一次性租约；
- cancellation、durability、resource、authority不变量通过 fault tests；
- source receipt诚实保留 real software/device/operator=false。

合法状态：`MODULE_CLOSED_CANDIDATE_SOURCE_ONLY`。

### 9.2 Runtime evidence closed

- 至少一条 fixed provider 或 fixed native runtime 真实模型纵切；
- 真实 cancel/crash/OOM/stale event；
- model/runtime/device identity与输出链绑定；
- exact-device performance/resource/privacy/rollback完成。

### 9.3 Product与发布闭合

- agentd shadow在真实产品路径通过；
- independent privacy/operator acceptance；
- separate authorized promotion/release commit；
- 当前作者未自合并。

在 9.1、9.2、9.3 全部满足前，禁止声称“所有 gap 已闭合”。

## 10. 执行顺序与合法终态

```text
INF-D0
INF-Q0
INF-I1 + INF-S1
INF-S2 + INF-S3 + INF-S4
INF-R1 + INF-R2
INF-R3
INF-E1 + INF-E2 + INF-E3
INF-A1
```

每个 package 只能返回：

```text
PACKAGE_QUALIFIED
BLOCKED_UPSTREAM
BLOCKED_EXTERNAL_EVIDENCE
BLOCKED_INDEPENDENT_AUTHORITY
BASE_DRIFT
STOP_CONDITION
```

不得用继续写计划替代实现，也不得用组件源码、fixture 或 queued workflow替代 exact-head执行与真实证据。
