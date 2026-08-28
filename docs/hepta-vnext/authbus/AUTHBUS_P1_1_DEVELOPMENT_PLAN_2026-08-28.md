# AuthBus P1.1：签名身份与 signed evidence qualification 开发计划

**计划 ID：** `AUTHBUS-P1-PLAN-2026-08-28`  
**阶段：** P1.1  
**Stack base：** `integration/vnext-main-full-ci-authbus-p0-3-20260828`  
**开发分支：** `integration/vnext-main-full-ci-authbus-p1-1-20260828`  
**状态：** `IMPLEMENTED_SOURCE / EXECUTABLE_QUALIFICATION_PENDING / NO_AUTHORITY`

## 1. 阶段目标

P1.1 把现有 B1 结构性 `IdentityBinding` 推进到可执行的密码学验证边界，并为
provider status 与人工证据建立独立签名、防重放和状态单调性模型。

本阶段必须证明：

1. 身份声明由当前 epoch 的预注册 Ed25519 公钥严格验证；
2. audience、service identity、policy、process peer、nonce、TTL、epoch 与 fence
   均进入签名和验证边界；
3. launch nonce 重放在有效窗口内必然拒绝，缓存满时 fail closed；
4. provider status 必须由独立用途的 key 签名，revision 与 observed-at 单调；
5. `ManualRequired` 只能由 operator-purpose key 的独立证据处理；
6. manual decision 最多恢复 lookup-only、继续人工处理或 quarantine，绝不授予
   provider effect / execute authority。

## 2. 不变量

### 2.1 Key registry

```text
same issuer/key/epoch + exact same registration -> exact replay
same issuer/key/epoch + changed material       -> KeyConflict
lower epoch after a newer epoch exists         -> StaleKeyEpoch
same epoch + different key id                  -> KeyConflict
revoked key at/after revocation time           -> KeyRevoked
wrong key purpose                              -> KeyPurposeMismatch
```

私钥不进入任何 P1.1 数据模型。测试使用的 deterministic signing key 只存在于
integration test 进程。

### 2.2 Identity

签名 preimage 绑定 canonical `IdentityBinding`、issuer、key ID 与 key epoch。
验证顺序为：

```text
B1 contract validation
-> audience/service/policy/peer binding
-> iat/nbf/exp/max-TTL
-> current key epoch / validity / revocation
-> Ed25519 verify_strict
-> bounded nonce insertion
```

只有全部验证通过后才写入 nonce cache。缓存不得驱逐仍有效 nonce 来容纳新请求。

### 2.3 Provider status

每个 operation 预注册：

```text
operation_id
provider_id
profile_id
token_family_id
status_binding_sha256
authority_epoch / owner_epoch / generation / fence digest
```

status evidence 必须满足：

```text
first revision = 1
next revision strictly increases
observed_at never rolls back
same revision + same digest -> exact replay
same revision + changed digest -> EvidenceConflict
terminal + changed later evidence -> TerminalImmutable
```

### 2.4 Manual evidence

`ManualRequired` 后普通 provider status 被阻断。独立 operator evidence 只允许：

```text
ResumeLookupOnly
KeepManualRequired
Quarantine
```

它不允许直接标记成功、不允许重新 dispatch，也不产生 effect authority。

### 2.5 Authority

以下值在源码、测试、receipt 与 workflow 中持续为 false：

```text
authority
effect_authority
production_caller
production_writer
operator_acceptance
promotion
g5_allowed
execute_allowed
listener_enabled
provider_call_enabled
openbao_enabled
private_key_storage
```

## 3. 实施结构

```text
codex-rs/hepta-authbus-p1-qualification/
├── Cargo.toml
├── Cargo.lock                 # bootstrap qualification 后固化
├── README.md
├── src/
│   ├── lib.rs
│   ├── model.rs
│   └── verifier.rs
└── tests/
    └── p1_1.rs
```

采用 nested workspace、Rust 1.95、resolver 3、`default = []`。只有显式
`p1-qualification` 才编译签名验证逻辑。

## 4. 回归矩阵

必须覆盖：

1. 全部 authority 常量为 false；
2. key registration exact replay 与 changed registration conflict；
3. 合法 Ed25519 identity；
4. invalid signature；
5. wrong audience / peer / service / policy；
6. not-before、expiry、future issued-at、max TTL；
7. nonce exact replay；
8. nonce capacity fail closed；
9. key epoch rotation；
10. revocation；
11. provider status exact replay；
12. same-revision changed evidence conflict；
13. stale revision / observed-at rollback；
14. wrong operation/provider/profile/token-family/fence；
15. provider evidence age/future skew；
16. terminal immutability；
17. wrong key purpose；
18. `ManualRequired` 阻断普通 status；
19. operator evidence 恢复 lookup-only；
20. manual exact replay / changed evidence conflict；
21. terminal manual quarantine；
22. bounded operation ledger。

## 5. Qualification gates

```bash
python3 scripts/verify-authbus-p1-1.py
cargo fmt --manifest-path codex-rs/hepta-authbus-p1-qualification/Cargo.toml \
  --package codex-hepta-authbus-p1-qualification -- --check
cargo test --locked --manifest-path codex-rs/hepta-authbus-p1-qualification/Cargo.toml \
  --no-default-features --lib
cargo test --locked --manifest-path codex-rs/hepta-authbus-p1-qualification/Cargo.toml \
  --features p1-qualification --tests
cargo check --locked --manifest-path codex-rs/hepta-authbus-p1-qualification/Cargo.toml \
  --features p1-qualification --all-targets
cargo clippy --locked --manifest-path codex-rs/hepta-authbus-p1-qualification/Cargo.toml \
  --features p1-qualification --all-targets -- -D warnings
```

Hosted receipt 必须来自 exact head、真实 runner、非空 steps。source-only PASS、queued
job 或 bootstrap artifact 均不能解释为 executable qualification。

## 6. 非目标

本阶段不：

- 接入父 product workspace；
- 打开 listener；
- 读取或保存私钥；
- 调用 provider/OpenBao；
- 替换生产 scheduler；
- 授予 production/effect/execute authority；
- 合并 P0.1/P0.2/P0.3/P1.1。

## 7. 下一阶段

P1.1 executable-qualified 后进入 P1.2：

1. 将 key registration/revocation 持久化到 SQLite WAL；
2. 将 nonce replay cache 持久化并验证 reopen 后继续防重放；
3. 持久化 status/manual revision ledger 与 terminal tombstone；
4. 加入 bounded retention/GC，证明不会删除仍存活的 replay protection；
5. stale writer、disk-full、crash-before-commit 全部 fail closed。
