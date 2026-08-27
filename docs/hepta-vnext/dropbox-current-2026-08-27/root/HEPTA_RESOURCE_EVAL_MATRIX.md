# Hepta vNext Resource Evaluation Matrix（Planning v0.1）

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
> This cross-domain pointer is informational: this resource matrix is not an AuthBus
> canonical source and grants no runtime, model/NPU, provider, effect, or promotion
> authority. The position-specific fallback rules below remain scoped and non-executable
> until bound by a future snapshot; HNL federated work stays `NOT_READY_FAIL_CLOSED`.

本表用于 neuron/TaskFlow qualification 的预算门，不是对本地模型可运行性的承诺。所有数值必须用同一版本 artifact、同一 prompt/trajectory manifest 实测并写入 receipt。

| 节点 | 可承担的 Hepta 角色 | 首选后端 | 硬预算/门槛 | 不应承担 |
|---|---|---|---|---|
| Mac M5 32GB | 小型 neuron group、Core ML sidecar、memory-review shadow | Core ML ANE；CPU fallback；MLX GPU 作为旁路 | 单 group 内存上限、p95 延迟、ANE/CPU fallback 必须有 receipt；不得阻塞 Agent | 长上下文 planner、通用 tool executor、在线 base-model training |
| 台式机 RTX 4060 8GB/125GB RAM | 较重 neuron batch、Music/embedding smoke、TaskFlow shadow | CUDA/CPU；远程 fallback | 显存/主存/并发预算固定；量化模型需单独 artifact digest | H3 生产视频服务、未经评测的多模型常驻池 |
| j3160 4GB RAM | 轻量 Agent runtime、deterministic baseline、低并发 TaskFlow | CPU/rule；远程 | gateway MemoryHigh/Max、SwapMax、TasksMax；日志库/恢复任务有上限 | Music/H3、本地 neuron model pool、无限恢复重试 |

## 预算 contract

- 每个 neuron invocation 带 `resource_budget_id`、设备、模型 digest、预计/实际 token、内存和延迟；超限进入 fallback 或 abstain。
- fallback 顺序由 registry 固定：`NPU → local CPU/GPU → remote → deterministic rule`；每次切换写 model receipt，不得静默改变 policy。
- p50/p95、peak RSS/VRAM、queue wait、CPU fallback ratio、error/abstain rate 和 thermal/throttle 状态必须进入 replay manifest。
- 资源故障不能破坏 CognitiveStore、TaskFlow authority 或 peer Agent；只允许停止当前 sidecar/run 并进入 reconciliation。

## E.21/v1.4 effective fallback profiles (append-only, 2026-08-24)

The global fallback sentence above is retained as historical planning context;
it is not an executable universal chain. Runtime selection is
position-specific and must be bound by the `RunStartSnapshot` and registry
compatibility matrix:

| profile | allowed order | remote/privacy rule | failure disposition |
|---|---|---|---|
| `guard_deterministic` | deterministic guard/rule only | remote forbidden | veto, abstain, or escalate |
| `privacy_local` | Core ML/NPU → local CPU/MLX → rule | remote forbidden; redaction is not a bypass | abstain or deterministic baseline |
| `proposal_local` | Core ML/NPU → local CPU/MLX → explicitly allowed remote → rule | remote requires policy digest, `RedactionReceipt`, endpoint allowlist and artifact compatibility | proposal/shadow only; never effect authority |

Each `RunStartSnapshot` binds an `ArtifactSetDigest` containing the ordered
backend candidates plus compiled-artifact and calibration digests. A fallback
attempt may not load an artifact absent from that snapshot or the registry
compatibility matrix. The attempt chain records device, compiled artifact,
calibration, redaction, resource outcome and reason. H5 scripted latency and
fallback receipts remain semantic fixtures, not hardware efficacy evidence.

This amendment does not install a model, enable remote inference, or grant
production/effect authority.
