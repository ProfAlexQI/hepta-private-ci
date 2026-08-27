# HEPTA vNext — memory-review TaskFlow vertical slice

**状态**：qualification / shadow-only implementation contract v0.1  
**对应计划**：`hepta-vnext-development-plan-final-2026-08-23.md` H0–H4、H3  
**目的**：把 H0/H1/H2/H4 的协议和 fixture 收敛成第一条可恢复、可回放、只读的 Hepta TaskFlow；本文件不授予生产 caller、写入权、工具执行权或 promotion 权。

> **Active-schema pointer:** the v0.1 PR-0 types and short outbox examples
> below are historical qualification notation. New code generators and
> encoders must resolve AUTHBUS.11 v1.3 through
> `AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map`; the legacy
> `DispatchStatus` enum and abbreviated fields are decode-only and must not be
> emitted.

## 1. 非目标与不变量

第一条垂直切片只实现 `memory-review`：

```text
recall → evidence.validate → inspect → postcondition.validate → report
```

允许等待、有限重试、取消、崩溃恢复、fan-out/join 的通用内核语义，但本流程中的节点都必须是 `None` 或 `ReadOnly` side effect。不得调用 model、tool、网络、OpenClaw session、第二 scheduler 或跨 Agent store。

以下不变量在编译、运行和 replay 三层都必须成立：

1. workflow definition、policy digest、capability set 和 authority 都是 immutable run 输入；
2. Agent-local authoritative SQLite 是 run/step/event/effect-intent/projection 的唯一事务域；跨域 evidence 只能经 transactional outbox；
3. command 使用 `command_id` 去重并带 `expected_revision`；旧 revision、旧 owner epoch、旧 fencing token 一律 fail-closed；
4. internal transition 以 CAS exactly-once；activity dispatch 至多产生 `DispatchAccepted`，不能当作 effect terminal；
5. timeout/crash 后结果不确定时进入 `Indeterminate`，由 reconciliation worker 处理，不猜测成功；
6. replay 只读取历史 clock/random/activity receipt，不重新调用外部世界；
7. report 是 evidence projection，不是 memory admission；任何摘要、候选或 neuron proposal 都保持 provisional；
8. 任何 qualification 通过都不修改 `CALLERS.toml`、`g5_allowed`、`promotion` 或 `operator_acceptance`。

## 2. 建议的 crate / PR 分解

### PR-0：contracts（依赖 H0/H1/H2/H4）

新增/稳定以下 typed contracts；不要把它们绑定到具体 LLM provider：

```rust
pub struct RunId(Uuid);
pub struct StepId { pub run: RunId, pub node: NodeId };
pub struct CommandId(Uuid);
pub struct EventId(Uuid);
pub struct OwnerEpoch(u64);
pub struct Revision(u64);
pub struct DefinitionDigest([u8; 32]);

pub enum RunStatus {
    Created, Queued, Running, Waiting, Cancelling,
    Succeeded, Failed, Cancelled, Indeterminate, ReconcileRequired,
}

// HISTORICAL_DECODE_ONLY: active implementations use AUTHBUS.11 v1.3 statuses.
pub enum DispatchStatus { Requested, Accepted, Started, Completed, Failed, Unknown }
pub struct EffectIntent { idempotency_key: String, capability: CapabilityId,
                          input_digest: Digest, policy_digest: Digest }
pub struct ActivityReceipt { intent_id: Id, status: DispatchStatus,
                             output_digest: Option<Digest>, observed_at: LogicalTime }
```

每个公共 command 返回 `Accepted | AlreadyApplied | Conflict | StaleGeneration | Rejected`，不能以布尔值表示成功。

### PR-1：Agent-local store / event ledger

首版使用 Agent-owned SQLite/sqlx。最小表：

- `workflow_definitions`：digest、schema version、authority、capability digest；
- `runs`：run status、definition digest、revision、owner epoch、state digest；
- `steps`：node、attempt、status、retry budget、checkpoint digest；
- `events`：`event_seq`、`event_id`、`causal_parent_seq`、`attempt_id`、`owner_epoch`、`prev_event_digest`、payload digest；
- `effect_intents` / `activity_receipts`：意图和收据分离；
- `outbox`：`delivery_seq`、destination、payload digest、delivered_at；
- `projections`：last applied event seq、state digest、schema version。

一个 transition 在同一 SQLite transaction 内写 event、projection 和 outbox；提交后再唤醒 dispatcher。禁止“先发消息、后写状态”。

索引必须覆盖 `(run_id, event_seq)`、`(command_id)`、`(run_id, revision)`、`(idempotency_key)`；重建 projection 时只接受连续 event sequence 和 digest chain。

### PR-2：definition registry / compiler

复用 H2 的受限 definition schema，但生产 registry 仍关闭。编译器必须验证：

- 节点/边 ID、schema 和 capability scope；
- 无未界定 cycle、终态覆盖、recovery path；
- loop、fan-out、join、retry 的预算上限；
- GuardExpr 仅允许 typed field comparison、`and/or/not`、存在性和常量，不执行 Rust/JS/LLM；
- `External` node 必须同时声明 idempotency template、receipt 和 compensation；
- `memory-review` 的所有节点 authority ≤ `QualificationOnly`，side effect ≤ `ReadOnly`。

编译结果是 immutable `CompiledDefinition { digest, nodes, edges, limits }`；相同 canonical JSON 必须得到相同 digest。

### PR-3：kernel state machine

Kernel 只负责 orchestration，不拥有业务判断。核心 API：

```rust
trait TaskFlowStore {
    fn load_run(&self, id: RunId) -> Result<RunSnapshot>;
    fn append_command(&self, cmd: Command, expected: Revision) -> Result<CommandResult>;
    fn commit_transition(&self, tx: Transition) -> Result<Revision>;
}

trait ActivityDispatcher {
    fn dispatch(&self, intent: EffectIntent) -> DispatchAccepted;
}

trait WakeupOwner {
    fn claim(&self, run: RunId, epoch: OwnerEpoch) -> Result<Lease>;
    fn wake_due(&self, now: LogicalTime) -> Result<Vec<RunId>>;
}
```

唯一 wakeup owner 负责 timer、retry 和 outbox poll；`hepta-automation` 只负责把外部请求送入 owning Agent 的 queue，不得成为第二 scheduler。启动恢复顺序固定为：claim generation → scan non-terminal runs → rebuild projection → reconcile outbox/indeterminate → schedule due timers。

### PR-4：activity seam / compatibility projection

`thread/queue/add` 适配器只返回 `QueuedReceipt`，并将其映射为 `DispatchAccepted`。旧 OpenClaw TaskFlow/Lobster 只能读取 projection 或发送 command；迁移期间禁止双写。未来切换采用：

```text
prepare → quiesce legacy → CAS transfer(owner_kind, authority_epoch)
→ verify projection/event_seq → commit
```

不支持的节点、旧 revision、缺失 receipt 均 abort；legacy flow id 和 cutover event seq 写入 evidence。

## 3. memory-review golden definition

使用 H2 fixture `h2_memory_review_v1.json`，固定 definition/policy digest。输入最小结构：

```json
{
  "request": {"turn_id": "t-...", "scope": "agent-local", "query": "..."},
  "limits": {"max_candidates": 32, "max_steps": 32}
}
```

每个节点产生 typed output 和 event：

| 节点 | 输入 | 输出 | 允许副作用 | 失败路由 |
|---|---|---|---|---|
| recall | request | candidate refs + source digests | read-only | bounded retry → failed-recall |
| validate | candidate refs | validated refs/conflicts | read-only | bounded retry → failed-validate |
| inspect | validated refs | findings/staleness/open-loops | read-only | bounded retry → failed-inspect |
| report | findings | report digest | none | terminal failure |

`report` 不调用 `remember`、KG writer 或 compact writer。若发现候选事实冲突，只输出 conflict finding 和 abstain reason。

## 4. 运行与恢复语义

### 4.1 正常路径

1. `StartRun(command_id, definition_digest)` 在 CAS transaction 中创建 run；
2. kernel 根据 entry node 写 `StepReady`；
3. read-only activity 写 `EffectIntent`（若没有外部 effect 可直接写 deterministic result）；
4. dispatcher/adapter 返回 receipt；
5. kernel 校验 receipt 的 run/node/attempt/epoch/digest 后提交 `StepCompleted`；
6. 后继 guard 只读 typed state，推进下一 step；
7. report 完成后写 terminal event 和 immutable report digest。

### 4.2 等待、取消和重试

- Approval/Input/Timer 进入 `Waiting`，resume token 必须绑定 run、step、revision、policy digest；
- retry 由 definition 的 bounded budget 控制，backoff 使用 replay clock；
- cancel 是 sticky，尚未发生的节点不再 dispatch；已发出的 intent 进入 reconciliation；
- join 必须明确 quorum、timeout 和 branch cap；缺 quorum 进入 failure/indeterminate，而不是隐式成功。

### 4.3 崩溃与不确定结果

故障注入点至少包括：commit 前 kill、commit 后 kill、dispatcher 超时、重复 callback、SQLite 重启、旧 owner callback、outbox 重复投递。每个场景都必须能由 event ledger 重建同一 `state_digest`；任何“已 dispatch、无 receipt”实例都只能是 `Indeterminate`。

## 5. 测试矩阵与硬门

### 单元/属性测试

- canonical definition digest 稳定；invalid graph corpus 全部拒绝；
- CAS/revision/command dedupe；旧 epoch、旧 timer、旧 approval callback 拒绝；
- event digest chain、projection rebuild、outbox exactly-once enqueue；
- guard DSL 不可执行任意代码；loop/fan-out/join/retry cap 不可绕过；
- idempotency key 对同一 intent 重放不产生第二 receipt。

### 集成/故障测试

- 1000 次随机 kill/restart/replay，terminal runs 的 state digest 与 oracle 一致；
- 三个 peer Agent 并发运行时，单 Agent store 故障不改变其他 peer liveness；
- legacy projection 读写迁移中断后可 abort，不能出现双 authority；
- resource matrix：Mac M5 默认只读 shadow，j3160 施加 MemoryHigh/TasksMax，RTX4060 只做离线 batch。

### H3 qualification receipt 必须包含

`schema_version`、kernel/definition/policy digests、worktree HEAD/tree、test output SHA、fault matrix、authority flags、CALLERS diff、G4/G5 binding、operator/promotion flags。硬门结果只能是 `PASS_H3_SHADOW` 或明确的 `BLOCKED_*`；不得把 `QueuedReceipt` 写成 effect success。

## 6. 与 H4/H5/H6/H7 的边界

- H4 只能通过 `RecallResult`、`MemoryCandidate` 和 `CompactCheckpoint` typed messages 交互；kernel 不直接写 CognitiveStore；
- H5 neuron group 是 activity/decision producer，必须经 scheduler budget 和 model receipt；kernel 不加载模型；
- H6 intuition 选择 definition/version/合法 branch，不能修改 compiled graph、capability 或 approval policy；
- H7 NDU 读取 trajectory，离线产出已签名 artifact；kernel 只接受治理层批准的 digest；
- H8 之前所有 effect adapter 都是 shadow/simulated，真实执行权保持在既有 Codex/App Server。

## 7. 实施顺序与停止条件

1. 先落 PR-0/PR-1 的 isolated qualification crate 和 deterministic in-memory store；
2. 接 H2 compiler fixture，完成 `memory-review` dry-run；
3. 加 SQLite persistence、rebuild、wakeup owner 和 reconciliation；
4. 接 OpenClaw projection-only adapter，做 paired replay；
5. 生成 H3 receipt，独立验证后才允许进入下一阶段；
6. 任一 authority flag、CALLERS、exact G4/G5 tree、生产服务或外部 effect 发生变化，立即停止并重建 receipt。

**Definition of Done（H3 shadow）**：同一输入和历史 receipt 可重放到同一 terminal digest；kill/restart/duplicate callback 不产生双 effect；跨 Agent 隔离和 stale fencing 通过；projection 可读但无双写；receipt 可独立验证；所有 promotion/production flags 为 false。

## E.21/v1.4 status and crosswalk (append-only, 2026-08-24)

This document remains the historical L0 `memory-review` qualification slice.
It is not the E.19 `salience_neuron_closed_loop` L2 sandbox and cannot satisfy
H5a/H5b, H6a/H6b or H7a/H7b by itself.

The slice uses `ActivityIntent`/`ActivityReceipt` for internal recall,
inspection and report work. It must not manufacture an `EffectReceipt` for a
read-only activity. Any future sandbox mutation is a separately named
`EffectIntent` (or a CognitiveStore admission command owned by CognitiveStore)
with its own idempotency key, postcondition and Indeterminate reconciliation.
`DecisionReceipt` is advisory input; TaskFlow must independently revalidate
snapshot, capability, approval, CAS and fence before creating an activity or
effect intent.

The E.19 S0–S6 order is the effective proposal order; this historical slice
remains a read-only prerequisite/reference contract.

## AUTHBUS.11-v1.3 supersession and crosswalk (append-only, 2026-08-26)

This appendix is the active implementation pointer when this historical
vertical slice is used with the Basil-derived Hepta AuthBus. It does not turn
the qualification slice into a production authority, does not change H3/E.41,
and does not rewrite the earlier v0.1 examples. Active generators and new
Rust code must use AUTHBUS.11 v1.3 and codex-hepta-contracts.v1.4_e21.

### 1. Historical DispatchStatus is decode-only

The earlier PR-0 type

    DispatchStatus { Requested, Accepted, Started, Completed, Failed, Unknown }

is retained only for decoding old fixtures. It must not be emitted by a new
TaskFlow or AuthBus implementation. The active public effect status set is:

    Queued | DispatchAccepted | Running | Succeeded | Failed |
    Cancelled | Indeterminate | Reconciled

The crosswalk is explicit:

| Old value | Active projection | Guard |
|---|---|---|
| Requested | Queued or internal command reference | non-terminal |
| Accepted | DispatchAccepted | queue acknowledgement only |
| Started | Running or DispatchAttemptStarted reference | carries attempt and fence |
| Completed | Succeeded only with verified terminal ACK | otherwise Indeterminate |
| Failed | Failed only with verified terminal failure | otherwise Indeterminate |
| Unknown | Indeterminate plus lookup-only reconcile | never release/refund/settle |

ActivityReceipt and EffectReceipt remain different objects. The
memory-review read-only activity cannot manufacture an EffectReceipt, and a
QueuedReceipt from thread/queue/add cannot be interpreted as a successful
effect.

### 2. Writer and outbox boundary

TaskFlow remains the sole durable writer for FlowRun, Step, EffectIntent,
ActivityReceipt and the canonical E21 EffectReceipt. Per-host authbusd owns
only AuthResource, quota, lease, permit and adapter-local dispatch references.
There is no shared database and no cross-database two-phase commit.

The local outbox guarantee called “exactly-once enqueue” means only this:
the owning SQLite transaction commits its state mutation and one outbox row,
deduplicated by destination, command id, idempotency key and payload digest.
It is not a network exactly-once guarantee. The bridge between TaskFlow,
authbusd, market-adapter, walletd or HNL is explicitly at-least-once:

    local state + outbox (one transaction)
      -> zero or more deliveries
      -> receiver inbox dedupe/apply/ack (one transaction)

The v1.3 outbox row is the canonical `hepta.auth.outbox-delivery.v1` shape:

    schema_version, outbox_id, source_owner, destination, event_type, event_id,
    aggregate_id, command_id, causal_parent_event_id, delivery_seq, delivery_attempt,
    provider_attempt, resolver_attempt, payload_ref, payload_encoding, payload_digest,
    source_record_digest, idempotency_key, lease_owner, lease_expires_at, next_retry_at,
    ack_digest, created_at, updated_at, status, authority_epoch, owner_epoch, generation,
    fencing_token, writer_record_fsync_confirmed, writer_fsync_witness_digest,
    dead_letter_reason

`attempt` is a decode-only alias for `delivery_attempt`; provider and resolver
attempts are independent counters and use their dedicated fields.

`lease_owner`, `lease_expires_at`, `next_retry_at`, `ack_digest` and
`dead_letter_reason` are nullable only under the canonical status conditions.
Payload is a canonical reference/digest, never raw token, Authorization
header, prompt, or private key. A receiver returns AlreadyApplied for the
same key and digest, Conflict for a changed digest, and StaleGeneration for
an old epoch/fence. A lost ACK may cause duplicate delivery, but never a
second state mutation. Outbox delivery attempt, provider effect attempt and
reconcile resolver attempt are separate counters.

The active `NoEffectProof` shape is generated from
`AUTHBUS_FAILURE_RECONCILE_STATE_MACHINE_v1.yaml#/execution_closure_v1_3/reconcile_evidence`:

    proof_id, proof_stage, reconciliation_id, provider_id, provider_namespace, effect_id,
    effect_intent_id, effect_key, attempt, idempotency_key, operation_ref,
    proof_kind, dispatch_attempted, dispatch_marker_state, dispatch_marker_digest,
    outbox_delivery_seq, writer_record_fsync_confirmed, writer_fsync_witness_digest,
    provider_query_receipt_digest, external_state_digest, evidence_digest,
    observed_at, verified_at, decided_at, taskflow_decision_id,
    taskflow_decision_digest, authority_epoch, owner_epoch, generation,
    fencing_token, payload_digest, current_revision, cas

The fsync witness digest must resolve to
`source_owner,wal_segment,wal_offset,event_seq,commit_digest,directory_fsync,
writer_boot_id,writer_generation,verified_at`; the boolean witness flag alone
is invalid. Pre-dispatch/verified-not-found proofs use
`operation-ref:v1:not-applicable:<provider_namespace>:<effect_id>:<effect_key>:<attempt>`;
only provider-explicit no-effect may carry a real operation reference with the
full binding. TaskFlow commits the current-fence ReconcileDecision; authbusd
cannot close an Indeterminate effect by itself.

### Opaque SecretRef refresh contract

TaskFlow may submit `RefreshWithSecretRef`/`RotateSecretRef`, but the adapter
alone may resolve the opaque `SecretRef`. The canonical request fields are
`schema_version,operation_id,refresh_operation_key,command_id,run_id,profile_id,
provider_id,token_family_id,secret_ref,expected_secret_revision,idempotency_key,
payload_digest,policy_digest,scope_digest,authority_epoch,owner_epoch,generation,
fencing_token,logical_clock,causal_parent_event_id,deadline_at,purpose_digest,audience`.
`deadline` is not an active spelling in this context. The exact required
`RefreshWithSecretRef` response is
`schema_version,response_id,operation_id,provider_id,profile_id,token_family_id,outcome,
access_secret_ref,refresh_secret_ref,secret_revision,refresh_operation_key,provider_status,
response_digest,idempotency_key,payload_digest,expected_secret_revision,authority_epoch,owner_epoch,
generation,fencing_token`; the exact required `RotateSecretRef` response is
`schema_version,response_id,operation_id,provider_id,profile_id,token_family_id,outcome,
new_refresh_secret_ref,secret_revision,refresh_operation_key,response_digest,idempotency_key,
payload_digest,expected_secret_revision,authority_epoch,owner_epoch,generation,fencing_token`.
Outcome-specific nullable/forbidden rules come from the canonical registry; the Resource
attachment is a generated projection and may add only explicitly owned adapter guards.
They preserve the request idempotency, payload, CAS and fence binding; raw token bytes,
provider headers and response bodies are forbidden. Conditional response fields are generated from
the canonical registry; the Resource attachment is a generated projection and may add only explicitly
owned adapter guards.

TaskFlow's reconcile command is the public trigger. An authbusd lookup
worklist is only an adapter-local cache/work queue; it cannot mutate the
TaskFlow effect ledger or close a receipt. Dead-letter and backpressure pause
delivery and retain evidence rather than silently failing an unknown effect.

### 3. Dispatch markers, direct terminal ACK and unknown lookup

For an external effect, the cross-owner sequence is:

    TaskFlow commits EffectIntent + outbox
      -> authbusd persists EffectIntentDurable/dispatch reference
      -> authbusd fsyncs DispatchAttemptStarted
      -> provider adapter call
      -> DispatchAcceptedRef OR DirectTerminalAckRef OR DispatchUnknownRef
      -> TaskFlow receives a typed EffectAck/Indeterminate proposal
      -> TaskFlow writes the closed E21 EffectReceipt

An adapter may return a verified terminal result in the first response. In
that case the legal path is:

    DispatchAttemptStarted -> DirectTerminalAckRef
      -> public Succeeded | Failed | Cancelled

It does not require an independent network DispatchAccepted/202. The direct
ACK must bind effect_id, effect_intent_id, idempotency_key, effect key,
provider namespace, provider operation id and digest (or a canonical
no-operation marker), payload digest, terminal state, receipt digest and the
current authority/owner/generation/fencing tuple. A synthetic acceptance
reference is internal only and must be marked synthetic.

If the process crashes or the response is lost after the provider call,
authbusd records DispatchUnknownRef and TaskFlow projects
EffectReceipt=Indeterminate. Recovery is lookup-only by effect key,
provider idempotency key or bound operation reference. It must not start a
new provider attempt merely because an outbox delivery was retried. 202,
Queued, Running, unscoped 404, expired retention or unverifiable signatures
do not close the effect. Only a verified terminal ACK or NoEffectProof can
produce Reconciled.

### 4. ReconcileEvidenceRef and ManualRequired

The active reconcile proposal is one schema,
`hepta.auth.reconcile-evidence.v1` (generated from the Failure state-machine
attachment). Its exact canonical fields are:

    reconciliation_id, effect_id, effect_intent_id, reconciles_receipt_id,
    idempotency_key, attempt, operation_ref, outcome, evidence_kind,
    evidence_digest, external_state_digest, provider_query_receipt_digest,
    verified_at, decided_at, resolver_attempt, authority_epoch, owner_epoch,
    generation, fencing_token, payload_digest, policy_digest, current_revision, cas

`verified_at` and `provider_query_receipt_digest` are canonical in v1.3; the historical
`provider_receipt_digest` spelling and field combinations that omit either are decode-only aliases.
The E21 reconciliation IDs and receipt-handle
derivation are defined by AUTHBUS.11 v1.3; this file must not invent a second ID algorithm.
The canonical reconciliation preimage is
`preimage_v1(domain=hepta.auth.reconciliation.v1; effect_id; effect_intent_id; attempt;
authority_epoch; owner_epoch; generation)`; the initial not-applicable receipt reference is the
registry's hashed `receipt-ref:v1:na` sentinel (domain
`hepta.auth.receipt-ref.na.v1`, effect/attempt/authority/owner/generation preimage).

ManualRequired and ReconcileBlocked are non-terminal worker/decision states.
They leave the public receipt as Indeterminate with
reconciliation_required=true and keep quota, lease and escrow held or
bounded-frozen. They cannot be converted into Reconciled, Released,
Refunded, Settled or Failed without verified terminal evidence or a
canonical NoEffectProof. A local “not found” observation is not such proof.

### 5. CI guard for this slice

The vertical-slice CI must:

1. read the AUTHBUS.11 v1.3 supersession pointer and reject emission of the
   historical DispatchStatus enum;
2. verify E21's closed EffectReceipt required set and terminal conditionals;
3. test transactionally-exactly-once local enqueue versus at-least-once
   bridge delivery, receiver dedupe, changed-payload conflict and stale fence;
4. test DispatchAttemptStarted to DirectTerminalAck, asynchronous
   DispatchAccepted, and DispatchUnknown lookup-only recovery;
5. prove the complete outbox field set, source-record digest and fsync-witness
   binding across pre-call, post-call and response-lost crash windows;
6. prove ManualRequired has no terminal path and cannot release/refund/settle;
7. scan outbox, event, receipt, projection and test output for raw
   credentials/Authorization/private-key bytes;
8. fail if a historical paragraph or old fixture is consumed as an active
   schema without an explicit decode-only adapter.

The resulting H3 receipt remains shadow/qualification evidence only. All
production/effect/promotion flags stay false, and this appendix does not
alter the E.41 development/internal-test schedule.

<a id="authbus-v13-taskflow-contract-inputs"></a>
## AUTHBUS.11 v1.3 TaskFlow contract inputs

TaskFlow's canonical `EffectIntent`, `EffectReceipt` and `ReconcileDecision` encoders MUST resolve
the registry at `OpenClaw/AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry` and the concrete
reconcile projection at `OpenClaw/AUTHBUS_RECONCILE_E21_CONTRACT_v1.yaml#/contract`. The outbox
bridge uses `OpenClaw/AUTHBUS_OUTBOX_DISPATCH_CONTRACT_v1.yaml#/contract`; authbusd may provide only
lookup-worklist projections and dispatch references. E21's 24 required receipt fields, hashed
initial receipt/decision sentinels, `NoEffectProof.proof_stage` (`PROPOSED|COMMITTED`) and the
`DispatchAttemptStarted → DirectTerminalAckRef|DispatchUnknownRef` branches are generated from
that source. `ManualRequired`/`ReconcileBlocked` never close an effect or release/refund escrow.

The active stage source is `OpenClaw/AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map`;
the old vertical-slice schema and DispatchStatus labels are decode-only. This is an implementation
crosswalk, not a claim that TaskFlow behavior has been implemented or that any production authority
has been enabled.
