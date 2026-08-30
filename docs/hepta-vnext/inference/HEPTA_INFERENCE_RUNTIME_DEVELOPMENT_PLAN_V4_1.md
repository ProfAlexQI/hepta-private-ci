# HEPTA 本地推理运行时开发计划 V4.1

> Plan ID: `HEPTA-INFERENCE-RUNTIME-V4`  
> Version: `4.1.0`  
> Date: `2026-08-31`  
> Repository: `ProfHepta/hepta-private-ci`  
> Canonical default branch: `integration/vnext-main-20260811`  
> Canonical default head observed: `b621768b70a09d56626bb8a2c331e3dc424e6a4d`  
> Development PR: `#87`  
> Development branch: `codex/hepta-inference-v4-closure-20260830`  
> Development base: `6f3286f99f5576b4cc545003889437bf388bb28a`  
> Observed parent before this plan: `5693819c1fe8873fba8da76edd85d9e31bb61afb`  
> Observed parent tree: `a6ea97e2f357096f6e812a47cfceb6b79ebdccb4`  
> Supersedes for current execution: `HEPTA_INFERENCE_RUNTIME_DEVELOPMENT_PLAN_V4.md`  
> Status: `SOURCE_BOUND_ACTIVE_PLAN / QUALIFICATION_ONLY`

## 0. 执行结论

V4.1 纠正 V4 当前执行中的两个根本问题：

1. 当前真值仍硬编码已关闭的 PR #73，无法证明正在开发的 PR #87；
2. 安全关键源码曾以压缩补丁和可写 CI 回推分支，导致被审查 head 与最终 head 不同。

从 V4.1 开始：

- 所有安全关键源码必须以普通、可审查的 tracked files 提交；
- CI 只能验证，禁止生成、提交或回推推理源码；
- current truth 绑定已观察 parent，exact candidate head/tree 只由 CI 产生；
- source、real software、exact device、product shadow 与 independent authority 分层；
- 外部证据不可由源码或作者自行声明闭合。

当前最高合法结论：

```text
source_candidate_present=true
source_candidate_qualified=false
real_provider_executed=false
real_native_model_executed=false
hardware_qualified=false
product_wired=false
runtime_activated=false
operator_accepted=false
promoted=false
released=false
```

## 1. 不可突破的权限边界

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

本计划允许闭合仓库源码和资格链，不允许自行：

- 合并 PR；
- 修改产品默认路由；
- 激活生产 listener/writer；
- 下载或安装模型；
- 将 fixture/mock/交叉编译称为真实模型或真实设备证据；
- 代表独立 operator 接受、promotion 或 release。

## 2. 单一真值与 source binding

权威顺序：

1. exact Git commit/tree；
2. exact-head、assigned-runner、非空 steps 的 CI；
3. digest-bound append-only receipt；
4. `HEPTA_INFERENCE_CURRENT_PLAN_V1.json`；
5. current status、implementation status、stage matrix、blocker ledger、evidence contract；
6. 本计划；
7. PR 描述与人工说明。

Tracked truth 只能记录已观察 parent。CI receipt 必须记录：

```text
pull_request_number
head
tree
parent
runner_id
runner_name
runner_os
runner_arch
rustc
cargo
source_inventory_digest
fmt/check/tests/clippy
```

`steps=[]`、`runner_id=0`、skipped、cancelled、timed_out、neutral、startup_failure 与 action_required 都不是 PASS。

## 3. 目标架构

```text
typed product shadow client
        │ public owner-local UDS + product session principal
        ▼
hepta-inferd
  identity / ownership / admission / deadline / receipt journal
  capability minting / one-time grants / scheduler / reservations
        │
        ├── authenticated private provider lane
        │       └── fixed loopback Ollama or LM Studio tuple
        │
        └── authenticated inherited worker lane
                └── isolated hepta-infer-worker-host
                        └── pinned ABI runtime / fixed GGUF tuple
```

公开 endpoint 只允许：

```text
Ping / Admit / CancelOwn / GetOwnReceipt / bounded Snapshot
```

private worker lane 只允许：

```text
WorkerHello / Authenticate / Lease / StartAck / Token / Complete
Failure / CancelAck / Health
```

private operator endpoint 默认不创建，只允许显式资格配置后提供：

```text
Drain / Restart / RegisterTuple / RemoveTuple / CompactReceipts
```

## 4. 核心安全不变量

### 4.1 Principal 与 ownership

同 UID 不是 request ownership。公开会话必须绑定：

```text
OS peer identity
+ product-issued session capability digest
+ tenant/workspace/agent generation
```

每个请求由 daemon 生成 owner capability。`CancelOwn` 与 `GetOwnReceipt` 必须服务器端验证 owner fence。

### 4.2 Capability 与 one-time grant

- daemon epoch key 与 session nonce 使用 OS CSPRNG，至少 256 bit；
- secret 类型不得 `Debug`、`Clone`、序列化或持久化，drop 时清零；
- worker handshake 绑定 PID、process start identity、backend generation、nonce 与 challenge；
- request grant 绑定完整 request fence、worker session 与 backend generation；
- grant 必须 `issued -> consumed|expired|revoked`，consume 与 dispatch 原子；
- restart 后旧 session/grant 全部失效；
- receipt/log 只能记录 capability digest。

### 4.3 调度和资源

首个可资格化 scheduler 为 deterministic、no-cache、starvation-bounded EDF/WFQ。必须独立限制：

```text
connections / frame bytes / read-write-idle timeout
queue global/per-tenant
inflight global/per-tenant
running global/per-tenant/per-tuple
prompt bytes / output tokens / output bytes / token bytes
provider requests / worker processes / worker active
receipt files / bytes / age / tombstones
shared memory / RSS / VRAM / KV / resident model bytes
```

reservation 由 daemon 单一 owner 修改，所有 terminal/fault/restart 路径 exactly-once release。

### 4.4 输出完整性

```text
chain_0 = SHA256(domain || complete request fence)
chain_n = SHA256(chain_(n-1) || sequence || token_digest || token_byte_length)
```

完成必须满足 token count、output token/byte limits、严格 sequence 和 chain digest。Receipt 必须绑定 identity、tuple、policy、prompt digest、runtime、worker session、token count/bytes、chain digest 与 termination disposition。

### 4.5 取消与真实进程事实

```text
CancelRequested -> CancelDispatched -> CancelAck -> Cancelled
```

ACK timeout：

```text
kill child -> wait/reap -> generation rollover
-> fail-close affected requests -> reject stale events
```

`forced_worker_termination` 不能由 controller 推测，必须来自 PID/start identity、kill result 和 wait/reap result。建议使用枚举：

```text
not_applicable / cancel_acknowledged / kill_requested /
kill_confirmed / process_exited / process_reaped /
identity_mismatch / unknown_fail_closed
```

## 5. 工作包与强制依赖

```text
INF-D0 truth reset
  -> INF-Q0 exact-head qualification
    -> INF-S1 principals/capabilities/private protocols
      -> INF-S2 cancellation/process lifecycle
      -> INF-S3 retention/tombstone/compaction
      -> INF-S4 EDF/WFQ + reservation ledger
        -> INF-R1 provider host
        -> INF-R2 isolated worker-host
          -> INF-R3 product shadow bridge
```

外部证据：

```text
INF-E1 real provider/model
INF-E2 real GGUF/exact device
INF-E3 performance/privacy/rollback
INF-A1 independent acceptance/promotion/release
```

这些不能由 repository-source commit 自行闭合。

## 6. 当前执行 tranche

### T0 — Truth reset

Deliverables：

- 本 V4.1 计划与唯一 current pointer；
- 所有 current truth 指向 PR #87 与当前开发分支；
- source gate 从 GitHub event/Git 推导 PR、branch、head、tree；
- 对完整 managed source inventory 哈希，不以字符串 marker 代替实现证明；
- 删除压缩 payload 与 branch-mutating workflow；
- 历史 receipts 只读。

Exit：source-truth 在 clean exact checkout 执行；mutation helper/payload 不再 tracked。

### T1 — Core private contracts

Deliverables：

- versioned bounded private protocol；
- independent worker/operator authentication domain；
- OS CSPRNG key/nonce constructors；
- bounded one-time grant ledger；
- deterministic EDF/WFQ scheduler；
- global/tenant/tuple reservation ledger；
- property/negative tests。

Exit：公开消息无法表达 private secret；重放、错 session、错 generation、错 role fail closed。

### T2 — Daemon composition

Deliverables：

- daemon 实际持有 epoch key、principal registry、grant ledger、scheduler、reservations；
- public ownership enforcement；
- inherited worker transport 与 private operator endpoint（默认关闭）；
- durable receipt schema 扩展；
- crash-safe recovery。

Exit：安全组件不再是旁路 library，admission/dispatch/receipt 全部通过组合根。

### T3 — Cancellation and retention

Deliverables：

- child handle owner；
- cancel ACK deadline、kill、wait/reap、generation rollover；
- append-only receipt + tombstone；
- operator-only compaction；
- receipt capacity admission reservation；
- crash fixtures。

### T4 — Provider and native worker hosts

Deliverables：

- fixed-manifest bounded loopback provider host；
- inventory/version/model/content-type fences；
- `hepta-infer-worker-host` 完整 crate；
- pinned ABI symbol/vtable/ownership validation；
- hermetic fake backend child-process E2E；
- no install/switch/remote fallback。

### T5 — Product shadow

Deliverables：

- typed `SHADOW_COMPARE_ONLY` bridge；
- primary route unchanged；
- shadow output non-authoritative；
- no Memory/KG/effect/route/fleet writes；
- kill switch and rollback fixture。

## 7. 资格矩阵

每个 source tranche 的最低 gate：

```text
cargo fmt --check
cargo check --locked --all-targets
cargo test --locked --all-targets
cargo clippy --locked --all-targets --no-deps -- -D warnings
Linux x64 + Linux ARM64 + macOS Intel
assigned runner + non-empty steps
artifact bound to exact head/tree
```

真实 provider、真实 GGUF、目标设备和产品 shadow 使用独立 workflow/receipt，绝不由 source aggregate 代替。

## 8. 合并与激活条件

在以下条件全部成立前保持 Draft：

1. 所有 repository-source blockers 关闭且 exact-head 三平台通过；
2. 无压缩源码 payload、无 CI 分支回推、无自动模型安装；
3. hermetic child-worker E2E 包含 crash/cancel/stale-event/recovery；
4. public ownership 与 private role negative tests 通过；
5. receipt capacity、compaction、kill/wait 事实可验证；
6. product bridge 只为 shadow，默认路由未改变；
7. 外部 evidence 与 independent authority 保持诚实状态。

即使 repository source 全部闭合，也不得从作者提交自行声明 operator acceptance、promotion 或 release。
