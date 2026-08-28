# AuthBus 深化开发计划与 P0.1 实施合同

**计划 ID**：`AUTHBUS-P0-PLAN-2026-08-28`  
**状态**：`IMPLEMENTATION_IN_PROGRESS / QUALIFICATION_ONLY`  
**仓库**：`ProfAlexQI/hepta-private-ci`  
**默认分支**：`integration/vnext-main-20260811`  
**基线提交**：`fe0889ecd46a5fc89de7b1ff3f28158c133a3502`  
**代码父提交**：`a85612afb43af722c61b54efe73570b25e9e4031`  
**开发分支**：`integration/vnext-main-full-ci-authbus-p0-1-20260828`  
**规范输入**：`AUTHBUS.11 v1.3`、canonical contract registry、stage matrix execution closure

> 本计划推进 qualification 实现，不授予 runtime、provider effect、production writer、
> operator acceptance、promotion、G5 或 execute authority。所有负权限标志持续为 `false`。

## 1. 总体目标

AuthBus 已具备较完整的 SecretRef、fence、quota、WAL 与 reconcile 合同，但当前 B3、B4、
B5 仍是彼此分离的 qualification reference model。最紧急风险来自 provider 调用边界：
一旦 provider 方法已经被进入，timeout、transport unavailable、connection reset 或无法验证
的响应 schema 都不能再作为普通 transient failure 直接重试，否则可能对已经执行的
refresh/rotation 产生第二次物理调用。

实施顺序固定为：

1. P0.1：关闭 provider-call uncertainty、retry/reconcile、terminal replay 语义缺口。
2. P0.2：把 B3、B4、B5 组合到真实 SQLite WAL qualification coordinator。
3. P0.3：关闭 B4 幂等与 old-fence reservation 回收缺口。
4. P1：身份签名、status anti-replay、registry-generated contracts。
5. P2：容量、TTL、GC、backpressure 和性能优化。
6. Release prep：exact-head receipts、operator acceptance、生产 caller ratchet。

## 2. 不可破坏的安全不变量

### 2.1 Secret 边界

- raw access token、refresh token、client secret、authorization header、provider body
  不得进入 contract、log、receipt、WAL、Debug 或 panic。
- Secret material 只能存在于 process-bound zeroizing wrapper。
- provider 返回值只能包含 opaque SecretRef、digest、revision 和有界状态。
- 任意错误路径不得投影 secret bytes。

### 2.2 Effect 边界

- provider call 前必须建立 deterministic operation key。
- provider call 后未知结果只能进入 `Indeterminate`。
- `Indeterminate` 禁止 blind retry，只允许 status-by-operation-key。
- 相同 operation key + 相同 request digest replay 当前 canonical result。
- 相同 operation key + 不同 request digest 必须 `Conflict`。
- 同一 provider/profile/token-family 同时最多一个活动 claim。
- terminal state 不得由旧 callback、旧 fence 或低 status revision 重新打开。

### 2.3 Authority 边界

以下标志在全部 P0/P1/P2 qualification 阶段保持 `false`：

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

## 3. 审计问题与路线映射

| 优先级 | 问题 | 风险 | 计划交付 |
|---|---|---|---|
| P0.1 | provider timeout/unavailable/schema-invalid 被当 transient | 重复 effect | 阶段感知失败分类 |
| P0.1 | retry/backoff/manual-required 无完整 API | claim 卡死 | 显式 retry API |
| P0.1 | reconcile 成功仍 replay 旧 Indeterminate | 状态/API 冲突 | terminal replay 替换 |
| P0.1 | status lookup 可从错误状态调用 | lookup 后非法 transition | 状态前置检查 |
| P0.1 | ordinary lookup 隐式解除 manual hold | 绕过人工证据 | manual 独立 ceremony |
| P0.2 | B3/B4/B5 无真实 durable coordinator | 崩溃后 exactly-once 未证明 | SQLite WAL |
| P0.3 | B4 同 key/同 payload 不返回原结果 | 幂等恢复失败 | canonical replay |
| P0.3 | rebind 后旧 permit 无 reconcile API | quota 永久 held | old-fence reconcile |
| P1 | status revision/observed-at 无单调约束 | replay/rollback | anti-replay ledger |
| P1 | B1 无签名/nonce/TTL verifier | 身份伪造 | identity verifier |
| P1 | quota 维度存在多套模型 | 守恒证明不闭合 | registry generator |
| P2 | 无容量/TTL/GC；B4 clone transaction | 资源放大 | bounded durable store |

## 4. P0.1：调用边界语义闭合

### 4.1 调用前失败

`SecretRefBackend::resolve` 发生在 provider call 前。以下错误可安全地分类为
`TransientFailure`，但只能通过显式 retry：NotFound、Unauthorized、Timeout、Unavailable、
Sealed、InvalidReference。这只证明 provider 尚未调用，不代表错误最终可恢复。

### 4.2 调用后未知

`SecretRefProvider::refresh/rotate` 已进入 provider boundary 后，Timeout、Unavailable、
connection reset、SchemaInvalid、malformed response、Unknown 必须投影为：

```text
outcome = Indeterminate
provider_status = Unknown
next_action = StatusByOperationKey
blind_retry = forbidden
```

### 4.3 已验证拒绝

只有明确、可验证的 provider 拒绝才可进入普通 transient 或 quarantine：

- InvalidGrant → Quarantined
- Unauthorized → TransientFailure
- Conflict → TransientFailure
- Sealed → TransientFailure
- StaleFence → TransientFailure

生产 adapter 后续必须进一步拆分 `VerifiedNoEffect` 和 `VerifiedTerminalFailure`；P0.1
先建立保守闭集。

### 4.4 普通调用与 replay

普通 `refresh()` / `rotate()`：首次调用建立 claim 和 attempt；相同请求重复调用只 replay
current response；不得因普通重复调用自动增加 attempt；response 尚未建立时返回
`ReconcileRequired`；terminal replay 不调用 provider。

### 4.5 显式 retry

新增：

```text
retry_refresh(request)
retry_rotate(request)
```

允许：

```text
TransientFailure -> RetryScheduled -> Backoff -> ClaimAgain -> Dispatch
Backoff          -> ClaimAgain -> Dispatch
```

拒绝：

```text
Indeterminate / Reconciling -> ReconcileRequired
ManualRequired              -> RetryBudgetExhausted
Succeeded / Quarantined     -> AlreadyTerminal
Idle / Claimed / InFlight   -> RetryNotAllowed
```

`retry_budget` 表示初次 attempt 之外的 retry 次数。预算耗尽：

```text
TransientFailure -> RetryBudgetExhausted -> ManualRequired
```

### 4.6 lookup-only reconcile

`status_by_operation_key()` 只允许 `Indeterminate` 与 `Reconciling`。其他状态：

- TransientFailure / Backoff → RetryRequired
- ManualRequired → ManualEvidenceRequired
- terminal → AlreadyTerminal
- Idle / Claimed / InFlight → fail closed

普通 provider lookup 不得自动触发 `ManualEvidenceSubmitted`。后续人工恢复合同必须携带
`evidence_digest`、operator identity、reason、时间窗、signature、current fence 和
expected revision。

### 4.7 terminal replay 更新

lookup 验证 terminal success/quarantine 时必须：

1. 校验 operation、payload、policy、audience、epoch、generation、fence。
2. 校验 status revision 单调前进。
3. 在 mutation 前构造并验证新的 canonical refresh/rotate response。
4. 执行 terminal transition。
5. 替换初始 Indeterminate response。
6. 释放 token-family claim。
7. 后续相同请求 replay terminal result。
8. provider call count 不增加。

### 4.8 OperationEntry 最小模型

```text
request_digest
operation_kind
original_typed_request
claim_key
SecretRefOperationRecord
current_response
last_status_revision
```

`current_response` 是 replay source；不得永久把首次方法响应当成唯一结果。

## 5. P0.1 验收矩阵

### 5.1 正向

- backend timeout → transient，provider call count = 0。
- provider timeout → Indeterminate + Unknown。
- provider timeout 后 retry → ReconcileRequired。
- provider timeout → lookup success → terminal replay success。
- verified transient 普通调用 → local replay。
- verified transient 显式 retry → attempt +1。
- lookup unknown → Backoff → explicit retry。
- retry budget 0 → ManualRequired。
- retry budget 1，第二次 verified failure → ManualRequired。
- rotation 与 refresh 使用同一边界规则。

### 5.2 负向

- same operation ID + changed digest → Conflict。
- stale status fence → provider lookup count = 0。
- status from transient/backoff → RetryRequired。
- status from manual → ManualEvidenceRequired。
- malformed success response → Indeterminate。
- provider schema error → Indeterminate，不得 direct retry。
- terminal 后 status → AlreadyTerminal。
- terminal 后普通 refresh/rotate → local terminal replay。
- status revision 回退或重复 → StatusRevisionConflict。
- raw credential bytes 不得出现在 serialized response。

### 5.3 CI 门

```bash
cargo fmt --all -- --check
cargo test --locked --no-default-features -p codex-hepta-contracts \
  --features authbus-local-qualification --tests -- --nocapture
cargo check --locked --no-default-features -p codex-hepta-contracts \
  --features authbus-local-qualification --all-targets
cargo clippy --locked --no-default-features -p codex-hepta-contracts \
  --features authbus-local-qualification --all-targets -- -D warnings
```

证据绑定 commit SHA、tree SHA、Cargo.lock SHA-256、feature set、四项 gate、authority posture
和 changed-file manifest。

## 6. P0.2：SQLite WAL qualification coordinator

P0.1 通过后新增独立 qualification crate `codex-hepta-authbus-qualification`，表至少包含：

```text
operations
token_family_claims
quota_reservations
dispatch_attempts
status_observations
outbox
fsync_receipts
writer_epochs
```

事务顺序：

```text
admission + quota hold
  -> operation intent durable
  -> dispatch-attempt durable
  -> commit + fsync witness
  -> process-bound provider call
  -> accepted/terminal/unknown marker
  -> outbox receipt
  -> complete/release/reconcile
```

强制 crash windows：intent 前；intent commit 后；attempt fsync 后；call 后 response marker 前；
unknown marker 后 receipt 前；ACK 后 cursor 前；disk full；WAL reopen；corrupt row；stale writer；
outbox ACK 丢失；status lookup crash/replay。

P0.2 DoD：物理调用最多一次或 lookup-only；reopen 不从 uncertain 自动 dispatch；旧 writer
不能提交 callback；WAL corruption fail closed；fsync receipt 可验证；仍无生产 authority。

## 7. P0.3：B4 幂等与 old-fence reconcile

幂等语义：同 idempotency key + 同 payload digest 返回原 reservation/permit；同 key + changed
payload 为 Conflict。rebind 后旧 permit 继续 held，只能通过 owner-authorized reconcile 关闭：
verified consumed → complete；verified no-effect → release；unknown → continue hold；伪造 evidence
拒绝且不修改 accounting。post-dispatch unknown 不得因 TTL 自动 release。

scheduler weight 必须由 policy class 推导；deadline 有 policy 上下界；unknown quota 默认 deny；
quota vector 由 registry 生成；fairness、starvation、oversell、conservation 写入 receipt。

## 8. P1：身份和证据

B1 verifier 增加 signature verification、key epoch/revocation、nonce replay cache、maximum TTL、
wall + monotonic clock、descriptor binding、service identity attestation、pidfd/start-time recheck。

status ledger 保存 last revision、observed_at、binding digest、provider query receipt。revision 必须
严格增加或 exact duplicate；same revision + different digest 为 Conflict；observed_at 回退拒绝；
terminal immutable；signed tuple 绑定 operation、payload、policy、mode、audience、fence。

统一 quota dimensions：request_count、rpm、tpm、concurrency、day_budget、context。B2、B4、WAL、
receipt、metrics 由 canonical registry 生成或验证。

## 9. P2：容量与性能

- operations、claims、status cache 增加容量、TTL、terminal GC。
- B4 clone transaction 替换为 SQLite transaction。
- external strings 全部 bounded。
- WAL/outbox 增加 backpressure、lease renewal、dead-letter。
- backend 尽量 in-place read，减少敏感数据复制。
- 增加 credential scan、heap dump negative test、panic/redaction test。
- 建立 fault injection、latency、fairness、storage growth budgets。

## 10. CI、PR、回滚与发布策略

- P0 分支使用独立 AuthBus workflow。
- PR 合并前必须有 actual runner、actual steps、exact-head evidence。
- `steps=[]`、`runner_id=0`、skipped 或仅 CLA 不构成 qualification。
- 默认分支不直接写入；不修改 production caller、CALLERS ratchet 或 authority flags。
- CI 未执行、exact-head 不一致、任一 gate 失败、authority 变 true、timeout 可 direct retry、status
  path 可 dispatch、terminal replay 仍旧、changed payload 产生第二 operation、旧 callback 修改 terminal
  或 credential scan 命中时全部 fail closed。
- 回滚仅回退 qualification branch，不触碰默认分支和生产 caller。

## 11. 本轮 P0.1 Definition of Done

- [x] exact base 和开发分支锁定。
- [x] 深化 P0-P2 路线与安全不变量。
- [x] provider-call uncertainty 分类实现。
- [x] explicit retry API 实现。
- [x] retry budget 与 ManualRequired 实现。
- [x] lookup 状态前置检查实现。
- [x] ordinary lookup 不解除 manual hold。
- [x] terminal replay replacement 实现。
- [x] P0.1 回归测试实现。
- [x] feature-on exact-head workflow 实现。
- [ ] cargo fmt 通过。
- [ ] feature-on tests 通过。
- [ ] feature-on check 通过。
- [ ] strict clippy 通过。
- [ ] exact-head evidence artifact 生成。
- [ ] PR 建立并完成审阅。
