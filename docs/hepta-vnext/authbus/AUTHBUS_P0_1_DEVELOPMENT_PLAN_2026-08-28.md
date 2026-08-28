# AuthBus 深化开发计划与 P0.1 实施合同

**计划 ID**：`AUTHBUS-P0-PLAN-2026-08-28`  
**状态**：`IMPLEMENTATION_IN_PROGRESS / QUALIFICATION_ONLY`  
**基线仓库**：`ProfAlexQI/hepta-private-ci`  
**基线分支**：`integration/vnext-main-20260811`  
**基线提交**：`fe0889ecd46a5fc89de7b1ff3f28158c133a3502`  
**代码父提交**：`a85612afb43af722c61b54efe73570b25e9e4031`  
**开发分支**：`integration/vnext-main-full-ci-authbus-p0-1-20260828`  
**规范输入**：`AUTHBUS.11 v1.3`、`AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml`、`AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map`

> 本计划只推进 qualification 实现，不授予 runtime、provider effect、production writer、
> operator acceptance、promotion 或 execute authority。所有负权限标志必须持续为 `false`。

## 1. 目标与总体判断

AuthBus 已形成较完整的 SecretRef、fence、quota、WAL 和 reconcile 合同，但目前 B3、B4、
B5 仍是互相分离的 qualification reference model。最紧急的问题不是功能缺失，而是调用边界
语义：provider 方法一旦被进入，timeout、连接丢失或响应 schema 无法验证，都不能再当作普通
transient failure 自动重试，否则可能对已经执行的 refresh/rotation 产生第二次物理调用。

本计划采用以下优先顺序：

1. **先关闭重复 provider effect 的语义风险。**
2. **再闭合 B3 retry/reconcile/replay 状态机。**
3. **再将 B3、B4、B5 接到真实 SQLite WAL qualification coordinator。**
4. **最后推进身份签名、状态证据、防重放、性能和生产接入。**

## 2. 不可破坏的安全不变量

### 2.1 Secret 不变量

- raw access token、refresh token、client secret、authorization header 和 provider body
  不得进入 wire contract、日志、receipt、WAL、Debug 或 panic。
- Secret material 只存在于 process-bound、zeroizing wrapper 中。
- provider 调用返回值只能包含 opaque SecretRef、digest、revision 和有界状态。
- 任意失败路径不得把 secret bytes 投影到错误字符串。

### 2.2 Effect 不变量

- provider call 前必须先建立 deterministic operation key。
- provider call 后的未知结果只能进入 `Indeterminate`。
- `Indeterminate` 不允许 blind retry，只允许 status-by-operation-key lookup。
- 相同 operation key + 相同 request digest 返回当前 canonical result。
- 相同 operation key + 不同 request digest 必须 `Conflict`。
- 同一 provider/profile/token-family 同时最多存在一个活动 claim。
- terminal state 不得被旧 callback、旧 fence 或低 revision status 重新打开。

### 2.3 Authority 不变量

以下标志在 P0/P1/P2 qualification 开发中全部保持 `false`：

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

## 3. 问题分级与交付映射

| 优先级 | 问题 | 风险 | 交付 |
|---|---|---|---|
| P0.1 | provider timeout/unavailable/schema-invalid 被当作 transient | 重复 refresh/rotation | 阶段感知失败分类 |
| P0.1 | transient、backoff、retry、manual-required 无完整 API | claim 卡死或永久 replay 旧失败 | 显式 retry API |
| P0.1 | reconcile 成功后仍 replay 初始 Indeterminate | durable state 与 API 结果冲突 | terminal replay 替换 |
| P0.1 | status lookup 可从错误状态调用 | lookup 后 invalid transition | 状态前置检查 |
| P0.1 | manual-required 被普通 lookup 隐式解除 | 人工证据语义被绕过 | manual hold 独立 |
| P0.2 | B3/B4/B5 未组合到真实 durable writer | 崩溃后 exactly-once 未证明 | SQLite WAL coordinator |
| P0.3 | B4 同 key/同 payload 不返回原 reservation | 幂等恢复失败 | canonical replay |
| P0.3 | rebind 后旧 permit 无 owner reconcile API | held quota 永久占用 | old-fence reconcile |
| P1 | status revision/observed-at 无单调约束 | replay/rollback | status anti-replay |
| P1 | B1 只有结构校验，无签名/nonce/TTL verifier | 身份伪造 | identity verifier |
| P1 | quota 维度多套模型 | 守恒证明不闭合 | registry-generated vector |
| P2 | 内存 map 无容量/TTL，B4 clone transaction | 资源放大 | bounded store + DB tx |

## 4. P0.1：调用边界语义与状态闭环

### 4.1 失败分类

#### 调用前失败

`SecretRefBackend::resolve` 发生在 provider 调用前。以下错误可投影为
`TransientFailure`，由显式 retry 处理：

- NotFound
- Unauthorized
- Timeout
- Unavailable
- Sealed
- InvalidReference

该分类只说明“provider 尚未被调用”，不说明错误最终可恢复。

#### 调用后未知

`SecretRefProvider::refresh/rotate` 已进入调用边界后，以下错误必须投影为：

```text
outcome = Indeterminate
provider_status = Unknown
next action = StatusByOperationKey
blind retry = forbidden
```

覆盖：

- Timeout
- Unavailable / connection reset
- SchemaInvalid / malformed response
- Unknown

#### 已验证的无成功结果

只有 adapter 能确认 provider 返回了明确拒绝语义时，才可进入 ordinary transient 或
quarantine：

- InvalidGrant → Quarantined
- Unauthorized → TransientFailure
- Conflict → TransientFailure
- Sealed → TransientFailure
- StaleFence → TransientFailure

生产 adapter 后续应把该集合进一步拆成 `VerifiedNoEffect` 与 `VerifiedTerminalFailure`，
qualification P0.1 先保持保守闭集。

### 4.2 retry 与 replay 语义

普通 `refresh()` / `rotate()`：

- 首次请求建立 claim 和 attempt。
- 相同请求重复调用只 replay 当前 response。
- 不得因重复调用自动进入下一 attempt。

新增显式方法：

```text
retry_refresh(request)
retry_rotate(request)
```

允许状态：

```text
TransientFailure -> RetryScheduled -> Backoff -> ClaimAgain -> Dispatch
Backoff          -> ClaimAgain -> Dispatch
```

拒绝状态：

```text
Indeterminate / Reconciling -> ReconcileRequired
ManualRequired              -> RetryBudgetExhausted
Succeeded / Quarantined     -> AlreadyTerminal
Idle / Claimed / InFlight   -> RetryNotAllowed
```

retry budget 表示初次 attempt 之外允许的 retry 次数。预算耗尽后：

```text
TransientFailure -> RetryBudgetExhausted -> ManualRequired
```

### 4.3 lookup-only reconcile

`status_by_operation_key()` 只允许：

```text
Indeterminate
Reconciling
```

其他状态：

- `TransientFailure/Backoff` → `RetryRequired`
- `ManualRequired` → `ManualEvidenceRequired`
- terminal → `AlreadyTerminal`
- pre-dispatch/in-flight → fail closed

普通 provider status lookup 不得自动触发 `ManualEvidenceSubmitted`。人工恢复必须在后续
独立合同中携带 evidence digest、operator identity、reason、issued-at、expiry 和 signature。

### 4.4 terminal replay

当 lookup 验证 terminal success/quarantine 时，adapter 必须：

1. 验证 status response 与原始 request 的 operation、payload、policy、audience、epoch 和 fence。
2. 验证 status revision 单调前进。
3. 在状态转换前构造并校验新的 canonical refresh/rotate response。
4. 进入 terminal state。
5. 用 terminal response 替换初始 Indeterminate response。
6. 释放 token-family claim。
7. 后续相同请求 replay terminal result，且 provider call count 不增加。

### 4.5 P0.1 数据模型

每个 operation entry 至少保存：

```text
request_digest
operation_kind
original_typed_request
claim_key
SecretRefOperationRecord
current_response
last_status_revision
```

`current_response` 是当前状态的 replay authority；不得永久保存“首次方法响应”作为唯一
结果。

## 5. P0.1 验收矩阵

### 5.1 正向测试

- backend timeout → transient，provider call count = 0。
- provider timeout → Indeterminate + Unknown。
- provider timeout 后 retry API → ReconcileRequired。
- provider timeout → lookup success → terminal replay success。
- verified transient 普通调用 → 本地 replay，不二次 provider call。
- verified transient 显式 retry → attempt 增加一次。
- lookup unknown → Backoff → 显式 retry。
- retry budget = 0 → ManualRequired。
- retry budget = 1，第二次 verified failure → ManualRequired。
- rotation 与 refresh 使用同一边界规则。

### 5.2 负向测试

- changed digest + same operation ID → Conflict。
- changed fence/status request → provider lookup count = 0。
- status lookup from transient/backoff → RetryRequired。
- status lookup from manual hold → ManualEvidenceRequired。
- malformed success response → Indeterminate，无第二次调用。
- provider schema error → Indeterminate，不允许直接 retry。
- terminal replay 后再次 status lookup → AlreadyTerminal。
- terminal replay 后再次 refresh/rotate → 本地 terminal result。

### 5.3 CI 门

P0.1 必须执行：

```bash
cargo fmt --all -- --check
cargo test --locked --no-default-features \
  -p codex-hepta-contracts \
  --features authbus-local-qualification \
  --tests -- --nocapture
cargo check --locked --no-default-features \
  -p codex-hepta-contracts \
  --features authbus-local-qualification \
  --all-targets
cargo clippy --locked --no-default-features \
  -p codex-hepta-contracts \
  --features authbus-local-qualification \
  --all-targets -- -D warnings
```

证据必须绑定：

- commit SHA
- tree SHA
- Cargo.lock SHA-256
- feature set
- test/check/clippy/fmt 结果
- authority posture
- changed-file manifest

## 6. P0.2：真实 SQLite WAL qualification coordinator

P0.1 通过后，下一阶段不再继续扩展孤立内存模型，而是新增独立 crate：

```text
codex-hepta-authbus-qualification
```

组成：

```text
SQLite WAL Store
  ├── operations
  ├── token_family_claims
  ├── quota_reservations
  ├── dispatch_attempts
  ├── status_observations
  ├── outbox
  └── fsync_receipts
```

事务顺序：

```text
admission + quota hold
  -> operation/intent durable
  -> dispatch-attempt durable
  -> commit + fsync witness
  -> process-bound provider call
  -> accepted/terminal/unknown marker
  -> outbox receipt
  -> complete/release/reconcile
```

强制 crash windows：

1. intent 前崩溃；
2. intent commit 后、attempt 前；
3. attempt fsync 后、call 前；
4. call 后、response marker 前；
5. unknown marker 后、TaskFlow receipt 前；
6. terminal ACK 后、cursor 前；
7. disk full；
8. WAL reopen；
9. torn/corrupt row；
10. stale writer boot/generation。

DoD：

- 每个 operation key 的物理调用最多一次，或明确进入 lookup-only。
- reopen 后不从 uncertain state 自动 dispatch。
- old writer/fence 无法提交 terminal callback。
- WAL corruption fail closed。
- qualification 仍无生产 authority。

## 7. P0.3：B4 reservation 与 old-fence reconcile

### 7.1 幂等语义

```text
same idempotency key + same payload digest
  -> return original reservation/permit reference

same idempotency key + changed payload digest
  -> Conflict
```

不得仅返回 generic duplicate。

### 7.2 old-fence active permit

rebind 后旧 permit 继续 held，但只能通过 owner-authorized reconcile 关闭：

```text
reconcile_old_permit(
  permit_id,
  old_fence,
  current_fence,
  provider_status_receipt,
  expected_revision
)
```

结果：

- verified consumed → complete
- verified no-effect → release
- unknown → continue hold
- stale/forged evidence → reject without accounting mutation

active permit 必须有 expiry scanner，但 post-dispatch unknown 不得因 TTL 自动 release。

## 8. P1：身份、状态证据和合同统一

### 8.1 B1 verifier

新增：

- signature verification
- current key epoch/revocation
- nonce replay cache
- max TTL
- trusted monotonic/wall-clock check
- descriptor digest binding
- service identity attestation
- pidfd/start-time revalidation

### 8.2 status anti-replay

每个 operation 保存：

```text
last_status_revision
last_observed_at
last_binding_digest
provider_query_receipt_digest
```

规则：

- revision 必须严格增加；或 exact duplicate 返回原 observation。
- 相同 revision + 不同 digest → Conflict。
- observed_at 回退 → reject。
- terminal observation immutable。
- signed tuple 必须绑定 operation key、payload、policy、mode、audience 和 fence。

### 8.3 registry 生成

统一 quota vector：

```text
request_count
rpm
tpm
concurrency
day_budget
context
```

B2、B4、WAL、receipt 和 metrics 必须由 canonical registry 生成或验证，不允许手工维护
不同维度集合。

## 9. P2：资源与性能

- operations、claims、status cache 增加容量、TTL 和 terminal GC。
- qualification in-memory clone transaction 替换为 SQLite transaction。
- 所有 external strings 增加 bounded length。
- scheduler weight 从 policy class 推导，不接受 caller 任意权重。
- deadline 加入 policy 上下界。
- WAL/outbox 加入 backpressure、lease renewal、dead-letter 和 metrics。
- secret backend 尽量使用 in-place buffer，减少复制和 page lifetime。
- 增加 secret scan、heap dump negative test、panic/redaction test。

## 10. 发布与回滚

P0.1/P0.2/P0.3 都只能进入 qualification branch/PR。出现以下任一情况立即 fail closed：

- CI 未执行或无 runner；
- exact-head 不一致；
- fmt/test/check/clippy 任一失败；
- authority flag 变为 true；
- timeout 被重新映射为 direct retry；
- status lookup 能触发 provider dispatch；
- terminal replay 仍返回旧 Indeterminate；
- changed payload 产生第二次 operation；
- secret-byte scan 命中。

回滚只回退 qualification branch，不修改生产 caller。默认分支在 PR 被审阅和 exact-head
qualification 通过前保持不变。

## 11. 本轮 P0.1 Definition of Done

- [ ] 失败分类按调用边界收紧。
- [ ] 显式 retry API 完成。
- [ ] retry budget 与 ManualRequired 完成。
- [ ] lookup 状态前置检查完成。
- [ ] ordinary lookup 不再解除 manual hold。
- [ ] reconcile terminal replay 替换完成。
- [ ] 新增 P0.1 回归测试。
- [ ] feature-on fmt/test/check/clippy workflow 完成。
- [ ] exact-head CI 有实际 runner 和 step 结果。
- [ ] PR 明确标记 qualification-only、无 authority。
