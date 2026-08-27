# H8/H9 governance gate (qualification contract)

> **Current development pointer (E.45 document-sync successor / 2026-08-27)**
>
> ```yaml
> current_profile: DEVELOPMENT
> implementation_status: IMPLEMENTATION_BACKLOG_ONLY
> development_blockers: implementation_backlog_only
> plan_pointer: hepta-vnext-development-plan-final-2026-08-23.md#development-docs-sync-e45
> historical_e44_pointer: hepta-vnext-development-plan-final-2026-08-23.md#authbus11-artifact-closure-v13
> current_plan_pointer: hepta-vnext-development-plan-final-2026-08-23.md#development-docs-sync-e45
> current_binding_manifest: HEPTA_DEVELOPMENT_DOCS_CURRENT_BINDING_V1.json
> current_sync_receipt: HEPTA-DEVELOPMENT-DOCS-SYNC-RECEIPT-2026-08-27.json
> qualification_pointer: HEPTA_VNEXT_QUALIFICATION_INDEX.md#authbus11-execution-closure-v13
> authbus_registry: AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry
> authbus_stage_selector: AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map
> behavioral_implementation_evidence: NOT_RUN
> runtime_authority: false
> production_effect_authority: false
> ```
>
> This is a scoped developer-document pointer; it does not make this gate an AuthBus
> canonical source or grant runtime, model/NPU, provider, effect, or promotion authority.
> The domain contract below remains in force, and the HNL federated lane stays
> `NOT_READY_FAIL_CLOSED` until Gate-0.

本文件定义 H8 单 Agent canary 与 H9 fleet 扩展的进入条件。它是**阻断合同**，不是批准书；当前所有生产权、CALLERS、G5 解冻和 operator acceptance 均为 false，因此结果必须是 `BLOCKED_GOVERNANCE_PREREQUISITES`。

## Gate input

Gate 只接受版本化 receipt digest，不接受“服务看起来正常”或 queue receipt：

- G4 `qualified_exact` paired receipt，exact head/tree 与 runner/evidence binding；
- G5 bounded aggregate、独立 operator review、CALLERS qualification entry；
- H4 memory/compact、H5 neuron、H6 intuition、H7 NDU shadow receipts；
- policy/model/workflow artifact registry 中的 digest、签名、评测窗口和 rollback target；
- 当前 authority snapshot（production caller、writer、effect、operator、promotion、`g5_allowed`）。

缺任一项或 digest 不匹配，门直接拒绝，不降级成“暂时允许”。

## H8 state machine

```text
Draft
  → Prepared (all receipts bound)
  → QuiescedLegacy (legacy projection stopped accepting writes)
  → OwnershipCAS (owner_kind + authority_epoch transferred once)
  → CanaryShadow (read-only/simulated effect)
  → OperatorAccepted
  → ProductionCanary
  → RolledBack | Promoted
```

每个转移都必须带 `expected_revision`、`authority_epoch`、`policy_digest` 和 `cutover_event_seq`。旧 owner、重复 callback、过期 approval 或第二 authority 一律 `Rejected/StaleGeneration`。

### H8 hard checks

1. H4–H7 receipts 的 parent exact G4、artifact digest、schema version 和 negative authority flags 一致；
2. legacy OpenClaw/Lobster 处于 quiesced projection-only，不可双写；
3. 单 Agent、单 workflow、低流量预算和资源上限已经固定；
4. effect adapter 仍是 simulated/approval-gated；`QueuedReceipt` 不等于 terminal effect；
5. rollback target 可重放，indeterminate effect 有 reconciliation/manual-required 路径；
6. operator 能暂停、对账、回滚，但不能绕过 capability/invariant；
7. fault injection（kill、timeout、duplicate callback、stale owner、disk full）不产生第二外部 effect。

## H9 extension checks

H9 只能在 H8 稳定窗口和 R2-G5 正式 promotion 后评估。额外要求：

- 每个 Agent 保持自己的 run/event/evidence authority，不建立跨 Agent workflow state bus；
- fleet lifecycle 只传播已批准 artifact digest，不传播 mutable run state；
- cross-Agent federation 通过 typed receipt、租约和 generation fence；
- provider/auth fallback、写入 workflow 和自动 adapter 更新必须有独立风险评测、预算和回滚；
- 任一 Agent liveness、memory isolation 或 effect dedupe 回归，立即回到 H8/legacy。

## 当前 gate result

```json
{
  "status": "BLOCKED_GOVERNANCE_PREREQUISITES",
  "g5_allowed": false,
  "operator_acceptance": false,
  "promotion": false,
  "production_caller": false,
  "production_writer": false,
  "effect_authority": false,
  "legacy_dual_write": false,
  "execute_allowed": false,
  "next_action": "obtain independent operator acceptance and governed CALLERS/promotion receipt"
}
```

此状态是预期的 fail-closed 结果，不是测试失败。任何 agent 不得自行把它改成 `ProductionCanary` 或 `Promoted`。

## Local-development override (2026-08-24)

上面的阻断合同继续适用于 **production** H8/H9。为了不把本地实现、shadow、回放和 sandbox
开发绑定到外部 provider/trust owner，另有 `local_development` profile：

- 不等待 provider-owned exactly-once 契约或独立 SSHSIG；
- 只允许 `CanaryShadow`、sandbox effect 和 bounded local fleet；
- 使用 `EffectIntent → DispatchAccepted → EffectReceipt` 或
  `Indeterminate → reconcile`，未知结果禁止盲重试；
- `production_activation=false`、`promotion=false`、`g5_allowed=false`、
  `fleet_and_automation_unfrozen=false` 始终保持不变。
- local profile 还必须保持 `planning_only=true`、`provider_effects=false`、
  `kg_write_authority=false`、`production_writer=false`、`governance_bypass=false`，
  并使用 `GovernanceMode=Shadow`；它不能通过配置把产品治理旁路掉。

因此 local-development 可以直接进入实现与 shadow，不再需要本文件顶部列出的外部输入；任何
真实外部 effect、legacy cutover 或 production promotion 仍必须显式选择 production profile，
不得由 local profile 自动升级。

## E.21/v1.4 governance crosswalk (append-only, 2026-08-24)

The historical H8 state machine above remains the production-governance gate.
It must not be conflated with E.19 `H8a` local closed-loop sandbox
qualification. A sandbox review/ack is not independent `operator_acceptance`,
does not quiesce a legacy owner, and cannot satisfy this gate.

| historical gate | v1.4 meaning | authority |
|---|---|---|
| H8 | production single-Agent canary/governance | independent operator acceptance + CALLERS/promotion; currently blocked |
| H8a | local reversible sandbox evidence only | `sandbox_review_ack`; production flags remain false |
| H9 | fleet/automation expansion | only after H8 and R2-G5 promotion |
| L3/S6 | governed NeuronGraph topology proposal | separate proposal/compiler/eval gate; never substitutes for H9 fleet gate |

Every H8 gate input must carry machine-readable `claim_level`, `evidence_class`,
`runtime_authority`, `efficacy_status`, and `approval_state`. Existing H4–H7
receipts are semantic/fixture/shadow evidence; they are not L2 action efficacy
or L3 topology evidence.

### E.21 receipt binding (append-only, qualification gate)

Before any future H8a sandbox review can be recorded, the gate input bundle
must include and independently verify these lanes. Their presence is a
qualification prerequisite, not an approval or promotion signal:

```json
{
  "required_shadow_contract_lanes": [
    "v1.4-contract-closure-qualification-20260824",
    "model-eval-longitudinal-qualification-20260824",
    "e21-contract-hardening-qualification-20260824",
    "e21-model-verifier-qualification-20260824",
    "e21-named-contracts-qualification-20260824",
    "e21-delivery-hardening-qualification-20260824"
  ],
  "required_runtime_seam_lanes": [
    "e21-s3a-runtime-qualification-20260824",
    "e22-runtime-integration-qualification-20260824"
  ],
  "required_fields": [
    "claim_level", "evidence_class", "runtime_authority",
    "efficacy_status", "approval_state", "source_binding", "receipt_sha256"
  ],
  "approval_state_required_for_production": "APPROVED_RUNTIME",
  "current_allowed_claim_level": "L0_BASELINE_L1_SHADOW_ONLY",
  "production_activation": false,
  "operator_acceptance": false,
  "callers_promoted": false,
  "g5_allowed": false
}
```

The S3a lane is explicitly `execute_allowed=false`, activity/read-only or
observation-only, and cannot satisfy production H8. Missing or stale lane
receipts fail closed; a shadow receipt must never be interpreted as
`APPROVED_RUNTIME`.

The E22 runtime-integration lane adds only a file-backed SQLite/outbox
qualification and a typed H5→H6 causal adapter. It is not registered in a
product runtime, has `runtime_authority=false`, and likewise cannot satisfy
production H8 or authorize MemoryAdmission/effect execution.
## E.21 named-contract shadow input (2026-08-24)

The standalone E.21 named-contract lane is now a mandatory shadow input for
contract-closure review:

`e21-named-contracts-qualification-20260824/E21-NAMED-CONTRACTS-RECEIPT.json`

It verifies the five planning contracts only. Its receipt MUST carry
`claim_level`, `evidence_class`, `runtime_authority`, `efficacy_status` and
`approval_state` (or an equivalent explicit planning-only state). It cannot
authorize production execution, model/NPU use, CALLERS promotion, G5 or
operator acceptance.

## E.22 Phase selector and pre-release scheduling rule — 2026-08-26

The gate above is a **production gate**, not a universal development stop.
The canonical delivery plan's E.41 phase policy is the current interpretation
for scheduling work. Until an explicit `FINAL_RELEASE`/production manifest is
selected, the active profile is `development_internal_test`:

```text
active_profile=development_internal_test
phase=DEVELOPMENT
allowed_phases=DEVELOPMENT,INTERNAL_TEST,RELEASE_PREP,FINAL_RELEASE,POST_RELEASE
external_inputs_required=[]
production_gate_status=BLOCKED_GOVERNANCE_PREREQUISITES
decision=CONTINUE_PRE_RELEASE
```

In those phases, missing provider-owned status/provenance, external signer or
KMS material, H4 physical-power evidence, and H8/H9 production acceptance are
`DEFERRED_PRE_RELEASE`/`PENDING_FINAL_RELEASE`. They may be logged and tested
with fixtures or explicitly labelled sandbox inputs, but they must not stop
feature implementation, internal testing, local crash/reopen tests, or
projection-only H8/H9 work. The naked historical `status` value above is
scoped as `production_gate_status`; it must not be interpreted as
`DEVELOPMENT_BLOCKED` or `INTERNAL_TEST_BLOCKED`.

Only an explicit `FINAL_RELEASE` admission evaluates these inputs as hard
release gates and may return `BLOCKED_FINAL_RELEASE`. A release-prep/RC
candidate can be marked pending while work continues; it cannot enable
mutation, external effects, legacy cutover, CALLERS, or fleet promotion.

This selector never adds a bypass: `production_caller`, `production_writer`,
`effect_authority`, `operator_acceptance`, `promotion`, `g5_allowed`, and
`execute_allowed` remain false until the separately governed production grant
and acceptance are verified. `QueuedReceipt` remains non-terminal and unknown
outcomes still enter `Indeterminate → reconcile`.
