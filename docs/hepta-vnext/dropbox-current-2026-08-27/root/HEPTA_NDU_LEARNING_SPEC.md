# Hepta NDU Learning Contract（Qualification v0.1）

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
> This cross-domain pointer is informational: this NDU contract is not an AuthBus
> canonical source and does not grant runtime, model/NPU, provider, effect, or promotion
> authority. The scoped NDU rules below remain in force; HNL federated work is still
> `NOT_READY_FAIL_CLOSED` until Gate-0.

**定位**：NDU 是 Hepta 的慢速 meta-learning/control plane，不是 runtime 总大脑，也不是生产执行 authority。  
**输入**：脱敏、可重放的 trajectory；**输出**：版本化、可回滚的 bounded artifact proposal。  
**硬规则**：base model、workflow topology、permissions、invariants、effect semantics 和安全下限冻结。

## 1. NduPosition

每个可学习决策点都注册一个 position，而不是每个 crate 或每个 neuron 各自拥有一套训练循环：

```text
position_id
owner_module
phase                 # observe | propose | shadow | canary | active
input_schema
output_schema
utility_dimensions    # success, safety, latency, cost, correction, pollution
learnable_parameters
authority             # observe_only | proposal_only | runtime_read
feedback_sources
update_cadence
promotion_gate
rollback_gate
```

固定 position：governance validator、TaskFlow executor、effect adapter、approval and capability checks。  
可学习 position：memory admission/retrieval rank、neuron head、workflow selection、branch threshold、retry/abstain budget、model routing。

## 2. Trajectory 与 credit assignment

一条 trajectory 必须保持因果链：

```text
turn → recall → memory candidate → neuron signal → intuition decision
     → workflow/step → effect intent → queued/terminal receipt → feedback
```

每个 record 带 `trajectory_id, event_seq, causal_parent_seq, state_digest, policy_digest, model_receipt_digest, authority_epoch`。NDU 只读取 immutable evidence snapshot；runtime DB 与 NDU replay DB 分离。

最终 utility 不可直接粗暴归因给最后一个模型。采用分层 credit：

```text
episode outcome
  ├─ workflow/policy decision
  ├─ neuron group + position
  ├─ memory candidate/admission/retrieval
  └─ step/attempt/effect receipt
```

不确定 external effect、人工纠正、rollback、memory pollution 和安全 abstain 必须作为独立负/正信号，不能隐藏在平均成功率里。

## 3. Objective 与预算

训练/校准目标至少包含：

```text
J = success
  - λ_safety * unsafe_or_unauthorized
  - λ_correct * human_correction
  - λ_retry * excess_retry
  - λ_latency * p95_latency
  - λ_cost * resource_cost
  - λ_pollute * memory_pollution
  + λ_abstain * calibrated_safe_abstain
```

每个 position 有 max update budget、sample budget、latency budget 和 rollback threshold。超预算时只产出 proposal，不修改 active artifact。

## 4. Replay/OPE 顺序

1. 固化 trajectory manifest 与 artifact digests；
2. paired replay：current heuristic vs candidate policy；
3. offline policy evaluation（OPE）与 counterfactual branch replay；
4. 检查 safety invariants、forgetting、distribution shift、resource tail；
5. shadow 只记录 candidate outcome；
6. canary 单 Agent/单 workflow/低流量；
7. operator acceptance 后才 promote；任何异常都 rollback 到上一 digest。

NDU 不得用未验证 prose 自动制造事实；训练 cache 必须响应 forget/tombstone，并在 manifest 中记录 lineage。

## 5. Artifact 与治理

每个 artifact 包含 `artifact_id, position_id, parent_digest, training_manifest_digest, model_digest, policy_digest, metrics, safety_report, created_at, expiry, signature`。Registry 必须支持 immutable versions、superseded/retired、canary state、rollback pointer 和 revocation。

Promotion gate 至少检查：

- safety regression = 0 hard violation；
- critical workflow success 不低于 baseline；
- abstain/calibration 不劣化超过阈值；
- memory pollution/forget resurrection = 0；
- p95/resource budget 在目标机器矩阵内；
- replay 可重建且 independent evaluator 通过。

## 6. Drift、遗忘与污染防护

- 监测输入/输出分布、confidence calibration、workflow mix、provider/model change；
- 触发 drift 时自动退回 shadow，不能继续 active 学习；
- 维护 replay anchor set 和旧任务回归集，防止新 workflow 覆盖旧能力；
- forget/correct 后删除或重算 embedding、replay manifest、training cache、adapter candidate 和已发布 artifact 的派生索引；
- NDU 不可用时 Hepta 继续使用 deterministic baseline。

## 7. Definition of Done

NDU learning slice 只有在 position registry、trajectory extractor、hierarchical credit、OPE/replay、artifact registry、shadow/canary/rollback、drift/forgetting monitor 全部有 receipts 后，才可进入 H7；任何单个 neuron smoke 或离线 loss 下降都不代表 intuition 已形成，也不授予生产写入权。
