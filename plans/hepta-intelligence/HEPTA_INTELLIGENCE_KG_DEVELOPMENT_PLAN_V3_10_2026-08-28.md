# Hepta Intelligence KG Development Plan v3.10

**日期**：2026-08-28  
**仓库**：`ProfAlexQI/hepta-private-ci`  
**执行状态**：ACTIVE / FAIL-CLOSED

## 1. 精确基线

```text
P0.3.2 Draft PR #40
branch = codex/hepta-intelligence-shared-projection-planner-v5-20260828
head   = a03863c8c6124a2024d959063da7ea412386de97
qualified = false（正式 Linux v7 仍无 runner）

P0.3.3 Draft PR #30
branch = codex/hepta-intelligence-evidence-resolver-v4-20260828
head   = cdfd1abe36f8de45300fd66c3dedb663e8d51b48
base   = codex/hepta-intelligence-grounding-gate-v3-20260828@256a47d882413ac1f80892a80177419bb5d12c70
qualified = false
```

## 2. 本轮发现的资格链冲突

旧 P0.3.3 source gate 只接受 `dependency.qualified=false`，而 governed restack 会把它写成 `true`。因此成功重栈后 source gate 必然失败。旧 workflow 还重新执行 extension/core 全 crate `-D warnings`，容易把历史 lint 债务误判为本 tranche 失败；旧 restack 在 push 之前也没有 source/rustfmt/all-target compile preflight。

## 3. 唯一执行顺序

```text
P0.3.2 exact-head v7 executable all-green
→ hard-bound P0.3.2 run/job/runner/steps/artifact/tree receipt
→ P0.3.3 governed restack preflight
→ force-with-lease update PR #30 and retain Draft
→ P0.3.3 exact-head v5 executable all-green
→ P0.3.4 legacy inventory / backfill / quarantine
```

## 4. P0.3.3 v5 source contract

Source verifier 必须接受且区分两个合法状态：

```text
pre-restack:
  dependency.qualified=false
  activation_blocking=true

post-restack:
  dependency.qualified=true
  implemented_in_repository=true
  activation_blocking=false
  ledger_verified_in_snapshot=true
  repository_branch/head 精确绑定 P0.3.2
```

Source receipt 中的 `p0_3_2_dependency_qualified` 由状态文件计算，禁止硬编码。

## 5. P0.3.3 v5 executable qualification

```text
python compile
source contract
P0.3.2 dependency exact binding
candidate rustfmt
extension cargo check --all-targets
core cargo check --all-targets
resolver focused tests
core grounding compatibility tests
extension full tests
core full tests
P0.3.3 governed-file scoped Clippy
source tree clean
```

`qualified=true` 仅当全部 exit code 为 0，且 dependency 已资格化。全 crate 编译错误仍阻断；与 P0.3.3 无关的既有 warning 不得伪装成本 tranche 新增缺陷。

## 6. P0.3.2 governance binding

P0.3.2 formal v7 workflow 的 path filters 同时覆盖 restack v2 与 v3.10 documents；P0.3.2 source gate 直接验证 restack 的 run/job/runner/steps/artifact/tree/preflight/force-with-lease markers。任何治理链修改都会产生新的 exact head 并重新执行 v7，不能复用旧 receipt。

## 7. Governed restack v2

Restack 在写入远端前必须验证：

- workflow path、run ID、head、tree、status、conclusion；
- jobs 非空、`runner_id>0`、steps 非空、全部 success；
- exact artifact 存在且未过期；
- P0.3.2 receipt 全部 checks 为 `passed=true / exit_code=0`；
- same-snapshot、current replan 与所有 authority false；
- PR #30 仍 open、Draft、head 精确匹配；
- P0.3.3 source、JSON、rustfmt、extension/core all-target check 在 push 前通过；
- changed paths 不超出 governed allowlist；
- 发布使用 force-with-lease，并在 retarget 后复核远端 head/base/Draft。

## 8. 权限边界

```text
wired=false
tool_v3_registered=false
tool_v4_registered=false
default_projection_pointer_changed=false
default_recall_query_changed=false
production_projection_gate=false
production_authority=false
external_effects=false
operator_accepted=false
promoted=false
callers_ratchet=false
P0.3.4_started=false
```

排队、`runner_id=0`、`steps=[]`、source-only PASS、静态检查或本补丁包都不得抬高 `qualified`。
