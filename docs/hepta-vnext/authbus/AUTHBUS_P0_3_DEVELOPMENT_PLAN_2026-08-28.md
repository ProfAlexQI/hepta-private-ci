# AuthBus P0.3：B4 canonical replay、old-fence reconcile 与 replay-evidence binding 实施计划

**计划 ID：** `AUTHBUS-P0-PLAN-2026-08-28`  
**阶段：** P0.3  
**Stack base branch：** `integration/vnext-main-full-ci-authbus-p0-2-20260828`  
**P0.2 final candidate：** `200234cb4d3eaea377058b2460cbd32098bcb380`  
**开发分支：** `integration/vnext-main-full-ci-authbus-p0-3-20260828`  
**P0.2/P0.3 stack merge：** `47f42f3bc95c9edea7387b2ccc8f9d532eb6a9b2`  
**P0.3 deterministic output commit：** `63f4957eb0a7d32e8b3b5261a8bce958f8628ea7`  
**状态：** `FINAL_EXACT_HEAD_QUALIFICATION_QUEUED / NO_AUTHORITY`

## 1. 阶段目标

P0.3 关闭 B4 的四个高风险缺口：

1. 相同 idempotency key 的 exact replay 不得被当成普通 duplicate，也不得产生第二个 permit 或第二个 quota hold；
2. resource rebind 后，旧 fence permit 必须通过 owner-evidence-bound reconcile 安全完成、释放或继续持有；
3. `DispatchStarted` 与 `OutcomeUnknown` 的 replay 必须绑定原始 marker evidence、observed-at 与 fence，不能因“无 mutation”而接受 changed replay；
4. quota invariant 必须同时验证 `used + held`，而不是只分别验证局部值。

本阶段同时引入 canonical 六维 quota 投影，为 B2、B4、WAL、receipt 和 metrics 的统一生成做准备：

```text
request_count
rpm
tpm
concurrency
day_budget
context
```

## 2. 不可破坏的不变量

### 2.1 Canonical idempotency

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

`expected_revision` 是新写入的 CAS 条件，不进入 reservation canonical digest；客户端在其他合法状态变化后重放同一原始 operation，仍可取得原 reservation，而不必伪造旧 revision。

### 2.2 Marker replay evidence

Dispatch marker digest 必须绑定：

```text
permit_id
current_fence
observed_at_ms
```

Unknown marker digest 必须绑定：

```text
permit_id
current_fence
evidence_sha256
observed_at_ms
```

因此：

```text
same state + exact same marker digest
  -> AlreadyPresent(original snapshot)

same state + changed fence/evidence/observed time
  -> ObservationConflict or StaleFence
  -> no mutation
```

即使 replay 不改变账本，也不能绕过 fence 和 evidence 校验。

### 2.3 Quota

- 任一未知维度默认拒绝 admission；
- hold 必须一次性覆盖六维 `estimate + safety_margin`；
- terminal consumed 的 actual 必须逐维不超过 reservation；
- concurrency 是同时占用，不累计进入 terminal used；
- verified no-effect 与 pre-dispatch expiry 只能 release，不能增加 used；
- post-dispatch unknown 永远不能因 TTL 自动 release；
- invariant verifier 必须验证 `used + held <= limits` 的每一维；
- 所有失败路径必须保持 held、used、active records 和 canonical snapshots 一致。

### 2.4 Fence 与 rebind

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
  - expected scheduler revision；
  - observed-at；
  - settlement outcome。

### 2.5 Reconcile history

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

历史 receipt 语义：

- 相同 reconcile request digest 永远返回原 receipt；
- 一个 Unknown receipt 在同一 permit 后续 terminal settlement 后仍可 exact replay；
- terminal settlement 后的 changed evidence 返回 `TerminalImmutable`；
- history lookup 必须先于 terminal-permit changed-evidence 拒绝判断，避免错误吞掉合法 historical replay。

### 2.6 Authority

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

独立 nested workspace：

```text
codex-rs/hepta-authbus-p0-3-qualification/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── src/
│   ├── lib.rs
│   └── scheduler.rs
└── tests/
    ├── p0_3.rs
    ├── reconcile_binding.rs
    ├── reconcile_history.rs
    └── replay_fence.rs
```

隔离规则：

- 不加入父级 product workspace；
- default feature set 只编译负权限常量；
- 只有显式 `p0-3-qualification` 才编译 scheduler model；
- 不存在 listener、provider call、OpenBao client、secret input 或 production writer；
- Rust 版本固定为 1.95，resolver 固定为 3；
- dependency graph 由 committed `Cargo.lock` 固定；
- qualification 只使用 `--locked`，不得在门禁内重新生成 lockfile。

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

Idempotency ledger 保留 canonical snapshot；active map 只保存前三种状态。

### 4.3 Active marker evidence

```text
ActivePermit {
  permit,
  state,
  dispatch_marker_sha256?,
  unknown_marker_sha256?,
}
```

合法 shape：

```text
ActiveReserved  -> no dispatch marker, no unknown marker
DispatchStarted -> dispatch marker present, unknown marker absent
OutcomeUnknown  -> dispatch marker present, unknown marker present when created by marker path
```

任何不一致均由 invariant verifier 返回 `CorruptState`。

### 4.4 Reconcile receipt

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

### Provider-boundary markers

```text
mark_dispatch_started(...)
mark_outcome_unknown(...)
```

两者只允许 exact evidence replay；changed timestamp、fence 或 unknown evidence 不得返回 `AlreadyPresent`。

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

只释放 `ActiveReserved`；`DispatchStarted` 与 `OutcomeUnknown` 返回到 `held_for_reconcile`。

## 6. 回归矩阵

必须覆盖：

1. 六维 quota legacy projection；
2. 任一未知维度 fail closed；
3. exact idempotency replay 返回原 permit；
4. changed payload conflict；
5. same payload + changed binding conflict；
6. duplicate/replay 不增加 revision 或 held；
7. dispatch marker exact replay；
8. dispatch marker changed observed-at conflict；
9. dispatch replay stale/changed fence fail closed；
10. unknown marker exact replay；
11. changed unknown evidence conflict；
12. unknown marker changed observed-at conflict；
13. rebind 保留旧 hold；
14. verified consumed 完成并结算 actual；
15. verified no-effect 完全释放；
16. unknown reconcile 继续 held；
17. unknown 后可由新 terminal evidence 结算；
18. historical Unknown receipt 在 terminal 后仍可 replay；
19. stale/forged old fence 无 mutation；
20. stale expected revision 无 mutation；
21. terminal receipt exact replay；
22. terminal changed evidence immutable；
23. pre-dispatch expiry release；
24. post-dispatch unknown expiry 不 release；
25. `used + held` 六维 invariant recomputation；
26. marker-shape corruption fail closed；
27. 所有 authority 常量为 false。

## 7. Deterministic closure

P0.3 在真实 hosted runner 上执行了一次性 deterministic bootstrap：

```text
run_id = 33156755305
job_id = 98801247057
result = success
```

该作业完成：

- exact branch、head 与 parent 验证；
- P0.2/P0.3 source 和 negative-authority gate；
- replay evidence hardening；
- 新增 replay/history regression tests；
- Rust 1.95 lockfile 生成；
- package-scoped rustfmt；
- qualification-crate-only changed-path allowlist；
- atomic force-with-lease push。

输出提交：

```text
63f4957eb0a7d32e8b3b5261a8bce958f8628ea7
```

一次性 bootstrap workflow 与 transform script 随后已经删除。最终候选不包含自修改 CI，也不依赖运行时源码变换。

## 8. 最终 executable qualification 门

```bash
python3 scripts/verify-authbus-p0-2.py
python3 scripts/verify-authbus-p0-3.py

cargo metadata --locked --no-deps \
  --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml \
  --format-version 1

cargo fmt \
  --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml \
  --package codex-hepta-authbus-qualification \
  -- --check

cargo test --locked \
  --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml \
  --no-default-features --lib -- --nocapture

cargo test --locked \
  --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml \
  --features sqlite-qualification --tests -- --nocapture

cargo check --locked \
  --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml \
  --features sqlite-qualification --all-targets

cargo clippy --locked \
  --manifest-path codex-rs/hepta-authbus-qualification/Cargo.toml \
  --features sqlite-qualification --all-targets -- -D warnings

cargo metadata --locked --no-deps \
  --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml \
  --format-version 1

cargo fmt \
  --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml \
  --package codex-hepta-authbus-p0-3-qualification \
  -- --check

cargo test --locked \
  --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml \
  --no-default-features --lib -- --nocapture

cargo test --locked \
  --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml \
  --features p0-3-qualification --tests -- --nocapture

cargo check --locked \
  --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml \
  --features p0-3-qualification --all-targets

cargo clippy --locked \
  --manifest-path codex-rs/hepta-authbus-p0-3-qualification/Cargo.toml \
  --features p0-3-qualification --all-targets -- -D warnings
```

Hosted evidence必须满足：

- exact commit/tree；
- real assigned runner；
- non-empty steps；
- committed manifest/lock digest；
- source、fmt、default-off、tests、check、Clippy 全部通过；
- inherited P0.2 matrix 同一 P0.3 exact head 通过。

`queued`、`runner_id=0`、空 runner name 或 `steps=[]` 均不是 PASS，也不是 Rust failure。

## 9. Definition of done

P0.3 只有在同一个最终 exact head 上满足以下全部条件，才可标记 executable-qualified：

- P0.2 与 P0.3 source/authority gates 通过；
- 两个 committed lock graphs 在 `--locked` 下不变化；
- 两个 package-scoped fmt gates 通过；
- inherited P0.2 default-off、SQLite WAL tests、check、Clippy 通过；
- P0.3 default-off、reservation/replay/reconcile tests、check、Clippy 通过；
- exact replay 与 changed-evidence conflict 通过；
- old-fence consumed/no-effect/unknown 三路径通过；
- historical receipt replay 与 terminal immutability 同时通过；
- `used + held` invariant 通过；
- post-dispatch TTL 不释放 unknown hold；
- 所有 authority 字段仍为 false。

## 10. 非目标

本阶段不：

- 替换现有产品 scheduler；
- 将 qualification crate 加入父 workspace；
- 打开 AuthBus listener；
- 调用 provider 或 OpenBao；
- 读取 secret；
- 修改 production CALLERS；
- 授予 effect/production authority；
- 合并 P0.1/P0.2/P0.3；
- 在 executable qualification 完成前启动 B1 identity verifier。

## 11. 下一阶段

只有 P0.2 与 P0.3 exact-head executable qualification 全部通过后，才进入 P1.1：

1. B1 signature verifier；
2. injected trusted clock 与 maximum TTL；
3. nonce replay cache；
4. key epoch、revocation 与 key-use binding；
5. audience、policy、subject、descriptor、service-attestation 与 fence 的签名覆盖；
6. bounded verifier capacity 与 fail-closed eviction policy。

后续顺序保持：

- P1.2 signed status evidence 与 anti-replay；
- P1.3 registry-generated 六维 quota projections；
- P1.4 manual operator evidence ceremony；
- P2 bounded retention、terminal GC 与 capacity limits。
