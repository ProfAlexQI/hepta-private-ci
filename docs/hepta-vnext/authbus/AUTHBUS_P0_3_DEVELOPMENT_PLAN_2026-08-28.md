# AuthBus P0.3：B4 canonical replay 与 old-fence reconcile 实施计划

**计划 ID：** `AUTHBUS-P0-PLAN-2026-08-28`  
**阶段：** P0.3  
**Stack base branch：** `integration/vnext-main-full-ci-authbus-p0-2-20260828`  
**P0.2 corrected base candidate：** `e72fd4cf99989f92e831ac00ee314e749012b921`  
**开发分支：** `integration/vnext-main-full-ci-authbus-p0-3-20260828`  
**状态：** `IMPLEMENTED_SOURCE / QUALIFICATION_PENDING / NO_AUTHORITY`

## 1. 阶段目标

P0.3 关闭 B4 的两个高风险缺口：

1. 相同 idempotency key 的 exact replay 不得被当成普通 duplicate，也不得再产生第二个 quota hold；
2. resource rebind 后，旧 fence permit 必须能通过 owner-authorized evidence 安全完成、释放或继续持有，不能永久占用 quota，也不能由旧 callback 直接改变账本。

本阶段同时引入 canonical 六维 quota 投影，为后续 B2、B4、WAL、receipt 和 metrics 的统一生成做准备：

```text
request_count
rpm
tpm
concurrency
day_budget
context
```

## 2. 不可破坏的不变量

### 2.1 Idempotency

```text
same idempotency key + exact same binding
  -> return original canonical reservation snapshot
  -> no new permit
  -> no new hold
  -> no revision increase

same idempotency key + changed payload digest
  -> IdempotencyConflict
  -> no mutation

same idempotency key + same payload + changed command/resource/policy/fence/quota/time binding
  -> BindingConflict
  -> no mutation
```

`expected_revision` 是新写入的 CAS 条件，不进入 canonical request digest；因此客户端在其他合法状态变化后重放同一原始操作，仍可取得原 reservation，而不会被迫伪造旧 revision。

### 2.2 Quota

- 任一未知维度默认拒绝 admission；
- hold 必须一次性覆盖六维 estimate + safety margin；
- terminal consumed 的 actual 必须逐维不超过 reservation；
- concurrency 是同时占用，不累计进入 terminal used；
- verified no-effect、pre-dispatch expiry 只能 release，不能增加 used；
- post-dispatch unknown 永远不能因 TTL 自动 release；
- 所有失败路径必须保持 `held + used` 与 active/terminal records 一致。

### 2.3 Fence 与 rebind

- 新 fence 的 owner epoch 和 generation 必须严格增加；
- fencing token 必须变化；
- authority epoch 不得回退；
- rebind 不自动释放旧 permit；
- 旧 permit 不能使用普通 current-fence callback；
- old-fence reconcile 必须同时绑定：
  - permit ID；
  - old fence；
  - current fence；
  - provider-status receipt digest；
  - owner evidence digest；
  - observed-at；
  - expected scheduler revision；
  - settlement outcome。

### 2.4 Reconcile

```text
VerifiedConsumed(actual)
  -> held -= reservation
  -> used += terminal(actual)
  -> active permit removed
  -> state = Completed

VerifiedNoEffect
  -> held -= reservation
  -> used unchanged
  -> active permit removed
  -> state = Released

Unknown
  -> held unchanged
  -> active permit retained
  -> state = OutcomeUnknown
```

相同 reconcile request digest 返回原 receipt；同一 permit 已 terminal 后的 changed evidence 返回 `TerminalImmutable`。

### 2.5 Authority

以下值在源码、receipt、测试与 workflow 中持续为 false：

```text
authority
effect_authority
production_caller
production_writer
operator_acceptance
promotion
g5_allowed
execute_allowed
```

## 3. 实施结构

新增独立 nested workspace：

```text
codex-rs/hepta-authbus-p0-3-qualification/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   └── scheduler.rs
└── tests/
    └── p0_3.rs
```

设计理由：

- 不改变父级 product workspace；
- default feature set 只编译负权限常量；
- 只有显式 `p0-3-qualification` 才编译调度模型；
- 与现有 B4 reference model 并存，避免在 P0.2 未 executable-qualified 前替换任何调用路径；
- 为后续把该语义移入 durable coordinator 保留清晰 review boundary。

## 4. 数据模型

### 4.1 Canonical quota

```rust
CanonicalQuotaVector {
    request_count,
    rpm,
    tpm,
    concurrency,
    day_budget,
    context,
}
```

提供：

- checked add/sub；
- fits-within；
- terminal usage；
- domain-separated digest；
- legacy B4 五维投影兼容方法。

### 4.2 Reservation lifecycle

```text
ActiveReserved
DispatchStarted
OutcomeUnknown
Completed
Released
ExpiredPreDispatch
```

Idempotency ledger 永久保留 canonical snapshot；active map 只保存前三种状态。

### 4.3 Reconcile receipt

```text
request_sha256
permit_id
resolution
actual?
before_revision
after_revision
held_after
used_after
observed_at_ms
authority=false
```

## 5. API

### Admission

```text
reserve(request)
  -> Inserted(snapshot)
  -> AlreadyPresent(original snapshot)
  -> IdempotencyConflict
  -> BindingConflict
```

### Provider-boundary phase

```text
mark_dispatch_started(...)
mark_outcome_unknown(...)
```

两者均支持 exact replay，不允许 changed unknown evidence 覆盖旧观察。

### Fence rotation

```text
rebind(current_fence, expected_revision, observed_at)
```

### Old permit settlement

```text
reconcile_old_permit(
  permit_id,
  old_fence,
  current_fence,
  provider_status_receipt_sha256,
  owner_evidence_sha256,
  expected_revision,
  observed_at_ms,
  outcome,
)
```

### Expiry

```text
expire_active_permits(now_ms, expected_revision)
```

只释放 `ActiveReserved`；`DispatchStarted` 与 `OutcomeUnknown` 被返回到 `held_for_reconcile`。

## 6. 回归矩阵

必须覆盖：

1. 六维 quota legacy projection；
2. 任一未知维度 fail closed；
3. exact idempotency replay 返回原 permit；
4. changed payload conflict；
5. same payload + changed binding conflict；
6. duplicate/replay 不增加 revision 或 held；
7. dispatch marker exact replay；
8. unknown marker exact replay；
9. changed unknown evidence conflict；
10. rebind 保留旧 hold；
11. verified consumed 完成并结算 actual；
12. verified no-effect 完全释放；
13. unknown reconcile 继续 held；
14. unknown 后可由新 terminal evidence 结算；
15. stale/forged old fence 无 mutation；
16. stale expected revision 无 mutation；
17. terminal receipt exact replay；
18. terminal changed evidence immutable；
19. pre-dispatch expiry release；
20. post-dispatch unknown expiry 不 release；
21. invariant recomputation 通过；
22. 所有 authority 常量为 false。

## 7. Qualification 门

```bash
python3 scripts/verify-authbus-p0-2.py
python3 scripts/verify-authbus-p0-3.py
cargo fmt --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml --all -- --check
cargo test --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml --no-default-features --lib -- --nocapture
cargo test --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml --features p0-3-qualification --tests -- --nocapture
cargo check --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml --features p0-3-qualification --all-targets
cargo clippy --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml --features p0-3-qualification --all-targets -- -D warnings
```

Hosted evidence必须包含非空 steps 和真实 runner。P0.3 的 source PASS 不能替代 P0.2 exact-base executable qualification。

## 8. 非目标

本阶段不：

- 替换现有产品 scheduler；
- 将 crate 加入父 workspace；
- 打开 AuthBus listener；
- 调用 provider/OpenBao；
- 读取 secret；
- 修改 CALLERS；
- 授予 effect/production authority；
- 合并 P0.1/P0.2/P0.3；
- 启动 B1 identity verifier 或 production registry generation。

## 9. 下一阶段

P0.3 executable-qualified 后进入 P1：

1. B1 signature、nonce、TTL、key epoch verifier；
2. signed status evidence 与 anti-replay；
3. canonical registry 生成六维 quota wire/store/receipt/metrics 投影；
4. manual evidence 独立 operator ceremony；
5. bounded retention、terminal GC 与 capacity limits。
