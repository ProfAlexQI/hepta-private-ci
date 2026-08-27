# Hepta vNext Protocol Contract（Qualification v0.1）

**日期**：2026-08-23（Asia/Shanghai）  
**用途**：把 Architecture v1.0 / Learning spec v0.3 中的 P0 blocker 收敛为可测试的协议合同。  
**权限**：qualification/design artifact；不新增 production caller，不改变 G4/G5 authority，不授予 promotion。

> **Active-schema pointer:** this file's v0.1/v1.4 field examples before the
> `AUTHBUS.11-v1.3 supersession and crosswalk` appendix are historical
> qualification notation. New encoders, generators and CI must resolve
> `AUTHBUS.11-v1.3` from
> `AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map` and the
> canonical Failure/Resource attachments; legacy `DispatchStatus` and short
> outbox examples are decode-only and must not be emitted.

## 1. 权威边界

Hepta 只拥有 Agent-local workflow state 与 typed evidence adapter；Codex 仍是唯一的 session/turn/model/tool execution spine。

| 对象 | 唯一写权威 | 其他层权限 |
|---|---|---|
| thread / turn / session | Codex | 读取事实、提交 typed command |
| flow definition | versioned Workflow Registry + Governance | 读取已批准 digest、离线提案 |
| flow run / step projection | owning Agent 的 state DB | OpenClaw/Lobster 只能投影 |
| event / intent / receipt | Agent DB 的 append-only ledger | outbox 投递到 evidence；不得反向写状态 |
| memory / KG head | CognitiveStore + deterministic validator | neuron 只能提出 candidate |
| model/policy artifact | signed registry + NDU promotion | runtime 只加载已批准 digest |
| fleet lifecycle | fleet supervisor | TaskFlow 不得成为 fleet bus |

禁止两个 runtime 同时写同一个 flow；禁止建立跨 Agent 的万能数据库。

## 2. 事务模型

首版每个 Agent 使用一个 authoritative SQLite DB（WAL、foreign keys、busy timeout），至少包含：

- `flow_runs`：`run_id, revision, state, owner_epoch, generation, state_digest, last_event_seq`；
- `flow_steps`：`run_id, step_id, attempt_id, status, retry_count, next_wakeup_at`；
- `event_ledger`：`event_id, run_id, event_seq, causal_parent_seq, attempt_id, owner_epoch, generation, prev_event_digest, payload_digest`；
- `commands`：`command_id, run_id, expected_revision, outcome, applied_event_seq`（唯一键去重）；
- `effect_intents` / `effect_receipts`：intent 先于 dispatch，receipt 绑定 intent；
- `outbox`：`delivery_seq, event_id, destination, delivered_at`，用于跨库 evidence 投递。

一次内部 transition 在同一 SQLite transaction 中完成：

1. 校验 `command_id`、`expected_revision`、`owner_epoch`、`generation` 和 fencing token；
2. 若 command 已存在，返回 `AlreadyApplied`，不得重复写 event；
3. 写 event（递增 `event_seq`，写 `prev_event_digest`）和 projection（CAS revision）；
4. 写 outbox row；
5. commit 后才允许 wakeup/dispatch。

跨库 evidence 只通过 transactional outbox；digest 不能替代原子提交。

## 3. Command/CAS/lease 合同

每个 command 必须带 `command_id`、`run_id`、`expected_revision`、`owner_epoch`、`generation`。实现只返回以下四类结果：

- `Accepted { new_revision, event_seq }`；
- `AlreadyApplied { revision, event_seq }`；
- `Conflict { current_revision }`；
- `StaleGeneration { current_generation, current_owner_epoch }`。

lease 是执行 fencing，不是业务授权。所有 renew/ack/transition/approval callback 都校验：

```text
owner_epoch == current.owner_epoch
generation  == current.generation
fencing_token == current.fencing_token
now < lease_expires_at
```

旧 owner、旧 timer、旧 callback 任一条件不满足都必须 fail-closed；不能以“刚好超时”推断旧 owner 已死。

## 4. Effect 语义

外部 side effect 只承诺 at-least-once：

```text
EffectIntent
  -> DispatchRequested
  -> DispatchAccepted (QueuedReceipt; 不是成功)
  -> ActivityStarted
  -> EffectReceipt(Succeeded | Failed | Indeterminate)
```

每个 intent 绑定 `effect_id, run_id, step_id, attempt_id, definition_digest, policy_digest, authority_epoch, idempotency_key`。queue receipt 必须绑定 intent digest 和 client id；不能把 `thread/queue/add` 的返回值解释成 terminal effect。

若 timeout/crash 后结果不明，状态只能是 `Indeterminate`，进入 reconciliation；不得自动记成成功或失败。非幂等写必须先获得 approval 或显式 Saga compensation；cancel 是 sticky。

## 5. Replay/determinism

GuardExpr 只允许 typed state、常量、布尔/比较/集合运算；禁止任意 Rust/JS/LLM 调用。时间、随机数、UUID、外部结果由 runtime 注入并事件化；replay 只读历史 ActivityResult，不重新发起 effect。图定义 digest、policy digest、model receipt 和输入 state digest 必须进入 Decision/Effect receipt。

workflow 必须声明：loop 上限、fan-out 上限、join quorum/timeout、retry budget、cancel propagation、recovery path。超过预算一律 `Failed` 或 `Waiting/ManualRequired`，不能无限重试。

## 6. Legacy migration handshake

OpenClaw/Lobster → Hepta kernel 迁移只能走：

```text
prepare → quiesce legacy → CAS transfer ownership
        → projection verify → commit | abort
```

迁移记录至少包含 `owner_kind, authority_epoch, legacy_flow_id, migration_state, cutover_event_seq, source_revision, target_revision`。任何不支持的 node/effect 都 fail-closed；迁移期间禁止 legacy 与 Hepta 双写。abort 必须保留 source authority，且能重试 prepare。

## 7. Memory/compact boundary

摘要、neuron proposal、queue receipt 都是 provisional input，不是事实。Memory admission 必须通过 deterministic provenance/scope/conflict/validity 检查；forget/correct 必须传播到 CognitiveStore、KG/index、compact checkpoint、trajectory/replay manifest、training cache 和已发布 artifact manifest。

Compact checkpoint 至少绑定：`parent_context_digest, event_range, protected_refs, summary_receipt, loss_report, state_digest, policy_digest, rehydration_status`。pre/post hook 必须幂等且 CAS 保护；崩溃恢复时以 checkpoint 与 event suffix 重建，不覆盖 authority facts。

## 8. 验收门

qualification 必须至少证明：

1. duplicate command 不产生第二 event/effect；
2. stale generation/expired lease/old callback 全部被拒绝；
3. queue receipt 与 terminal receipt 类型不可混淆；
4. event sequence、causal parent、digest chain 可重建；
5. crash/restart/reconciliation 不把 Indeterminate 伪装成成功；
6. invalid workflow graph、越权 capability、无限 loop/retry 全部 fail-closed；
7. migration 不发生双 authority；
8. G4/G5 的 `g5_allowed`, `promotion`, `operator_acceptance` 和 CALLERS 不因 qualification 自动改变。

通过后仍需独立 exact-head review、CALLERS qualification entry、operator acceptance 和 promotion receipt，才能进入 production。

## v1.4 amendment — canonical envelope crosswalk (2026-08-24)

The v0.1 field lists above are historical qualification notation. For the
E.19/v1.4 implementation proposal, the canonical schemas are the strict
versioned envelopes in the v1.4 contract-closure lane. Implementations must
not infer authority from a field merely mentioned in prose.

Envelope classes are intentionally separate:

1. `ObservationEnvelope` and immutable signal/feedback events require an
   event id, source event id, snapshot digest, causal parent and provenance;
2. `DecisionEnvelope` requires the approved graph/policy/model/calibration
   digests and a consumer snapshot revision;
3. mutating `CommandEnvelope`/`EffectIntent` requires command id,
   expected revision, authority epoch, owner epoch, generation and fencing
   token;
4. `ActivityReceipt` and `EffectReceipt` have distinct terminal matrices;
   `Queued/DispatchAccepted` is never terminal and `Indeterminate` requires
   reconciliation.

The v1.4 contract also standardizes the authority enum to
`fixed | observe_only | proposal_only | runtime_read`; the historical
`learnable` label maps to `proposal_only` and is not accepted by a v1.4
runtime schema. Existing v0.1 receipts remain qualification evidence only and
must not be mixed into L2 efficacy evidence without an explicit migration
adapter and claim-level marker.

## AUTHBUS.11-v1.3 supersession and crosswalk (append-only, 2026-08-26)

This note is an explicit machine-consumer pointer for the Basil-derived Hepta
AuthBus implementation. It does not rewrite the historical v0.1 notation
above, does not change the E.41 phase policy, and does not grant runtime or
production authority.

~~~yaml
supersession:
  id: AUTHBUS.11
  revision: v1.3
  anchor: authbus11-execution-closure-v13
  canonical_status_source: codex-hepta-contracts.v1.4_e21
  authbus_extension_source: hepta-auth-contracts.v1
  active_stage_ref: AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map
  legacy_status_decode_only: true
  delivery_semantics: bridge_at_least_once
  local_enqueue_semantics: transactionally_exactly_once
  external_effect_semantics: unknown_is_lookup_only
~~~

### Historical DispatchStatus is decode-only

The DispatchStatus { Requested, Accepted, Started, Completed, Failed,
Unknown } list in the v0.1 sections is historical qualification notation.
It must not be emitted by a v1.3 implementation or used as a code-generator
enum. The canonical public effect statuses remain the E21 set:

~~~text
Queued | DispatchAccepted | Running | Succeeded | Failed |
Cancelled | Indeterminate | Reconciled
~~~

The old values are accepted only by an explicitly named compatibility decoder:

| Historical input | v1.3 projection | Required guard |
|---|---|---|
| Requested | Queued or an internal command reference | never terminal |
| Accepted | DispatchAccepted | queue acknowledgement, never success |
| Started | Running or DispatchAttemptStarted reference | must carry attempt/fence |
| Completed | Succeeded only with verified terminal ACK | otherwise Indeterminate |
| Failed | Failed only with verified terminal failure | otherwise Indeterminate |
| Unknown | Indeterminate + lookup-only reconcile | never Failed/Released/Refunded |

ActivityReceipt remains separate from EffectReceipt; a read-only
ActivityIntent must never be upgraded to an effect merely because an adapter
returned a queue receipt.

### Local enqueue versus bridge delivery

“Exactly once” applies only to the local transaction that enqueues a command:
the owning SQLite writer commits its state mutation and one outbox row in the
same transaction, protected by (destination, idempotency_key, payload_digest).
It does not mean that a packet crosses a process, machine, relay, or HNL link
exactly once.

The bridge contract is explicitly at-least-once:

~~~text
source state + outbox row (one local transaction)
  → zero or more bridge deliveries
  → receiver inbox dedupe/apply/ack (one receiver transaction)
~~~

Receivers must return AlreadyApplied for a duplicate with the same canonical
payload digest and Conflict for the same key with a different digest. A lost
ACK may cause another delivery, but must not cause a second state mutation.
Delivery attempts, provider effect attempts, and reconcile resolver attempts
are separate counters and must not be conflated.

At-least-once bridge fields are the canonical `hepta.auth.outbox-delivery.v1`
set (not an open-ended “at minimum” variant):

`schema_version, outbox_id, source_owner, destination, event_type, event_id,
aggregate_id, command_id, causal_parent_event_id, delivery_seq, delivery_attempt,
provider_attempt, resolver_attempt, payload_ref, payload_encoding, payload_digest,
source_record_digest, idempotency_key, lease_owner, lease_expires_at, next_retry_at,
ack_digest, created_at, updated_at, status, authority_epoch, owner_epoch, generation,
fencing_token, writer_record_fsync_confirmed, writer_fsync_witness_digest,
dead_letter_reason`.

`attempt` is a decode-only alias for `delivery_attempt`; provider and resolver
attempts are independent counters and must be encoded in their dedicated fields.

`lease_owner, lease_expires_at, next_retry_at, ack_digest` and
`dead_letter_reason` are nullable only under the status conditions in
`AUTHBUS_FAILURE_RECONCILE_STATE_MACHINE_v1.yaml#/execution_closure_v1_3/outbox_delivery`.
Payloads are canonical references/digests only; raw credentials and
Authorization headers are forbidden. Dead-letter/backpressure pauses delivery
but does not close an unknown effect.

### Canonical NoEffectProof and writer-fsync witness

The only active `NoEffectProof` shape is generated from
`AUTHBUS_FAILURE_RECONCILE_STATE_MACHINE_v1.yaml#/execution_closure_v1_3/reconcile_evidence`:

`proof_id, proof_stage, reconciliation_id, provider_id, provider_namespace, effect_id,
effect_intent_id, effect_key, attempt, idempotency_key, operation_ref,
proof_kind, dispatch_attempted, dispatch_marker_state, dispatch_marker_digest,
outbox_delivery_seq, writer_record_fsync_confirmed, writer_fsync_witness_digest,
provider_query_receipt_digest, external_state_digest, evidence_digest,
observed_at, verified_at, decided_at, taskflow_decision_id,
taskflow_decision_digest, authority_epoch, owner_epoch, generation,
fencing_token, payload_digest, current_revision, cas`.

`writer_fsync_witness_digest` must resolve to a durable witness containing
`source_owner, wal_segment, wal_offset, event_seq, commit_digest,
directory_fsync, writer_boot_id, writer_generation, verified_at`; a boolean
`writer_record_fsync_confirmed` without that witness is invalid. For
`pre_dispatch_cancel` and `verified_not_found`, `operation_ref` is the
deterministic sentinel
`operation-ref:v1:not-applicable:<provider_namespace>:<effect_id>:<effect_key>:<attempt>`;
a provider operation reference is allowed only for provider-explicit no-effect
and must bind namespace, effect ID/key, payload digest, idempotency key and
attempt. TaskFlow alone commits the current-fence `ReconcileDecision`; a local
no-effect observation cannot close an Indeterminate effect.

### Opaque SecretRef refresh boundary

`RefreshWithSecretRef` and `RotateSecretRef` are adapter-local process-bound
operations. Their request uses the canonical registry contract
`AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry/contracts/oauth_secretref`
and must include
`schema_version, operation_id, refresh_operation_key, command_id, run_id,
profile_id, provider_id, token_family_id, secret_ref,
expected_secret_revision, idempotency_key, payload_digest, policy_digest,
scope_digest, authority_epoch, owner_epoch, generation, fencing_token,
logical_clock, causal_parent_event_id, deadline_at, purpose_digest, audience`.
`deadline` is not an active alias; encoders must use `deadline_at` in this
context. The exact required `RefreshWithSecretRef` response is
`schema_version,response_id,operation_id,provider_id,profile_id,token_family_id,outcome,
access_secret_ref,refresh_secret_ref,secret_revision,refresh_operation_key,provider_status,
response_digest,idempotency_key,payload_digest,expected_secret_revision,authority_epoch,owner_epoch,
generation,fencing_token`; the exact required `RotateSecretRef` response is
`schema_version,response_id,operation_id,provider_id,profile_id,token_family_id,outcome,
new_refresh_secret_ref,secret_revision,refresh_operation_key,response_digest,idempotency_key,
payload_digest,expected_secret_revision,authority_epoch,owner_epoch,generation,fencing_token`.
Outcome-specific nullable/forbidden rules are generated from the canonical registry;
the Resource attachment is a generated projection and may add only explicitly owned
adapter guards. They retain the same idempotency,
payload, CAS and fence binding; token bytes, provider headers and provider
response bodies are forbidden. The exact conditional response fields are generated from
the canonical registry; the Resource attachment is a generated projection and may add only
explicitly owned adapter guards.

### Direct terminal ACK and response-lost handling

An adapter may return a verified terminal ACK in the initial call. The legal
paths are:

~~~text
DispatchAttemptStarted
  → DispatchAcceptedRef (non-terminal 202/queued response)
  → public DispatchAccepted
  → later ACK or Indeterminate

DispatchAttemptStarted
  → DirectTerminalAckRef (verified terminal response)
  → public Succeeded | Failed | Cancelled
~~~

The second path does not require a preceding network DispatchAccepted.
DirectTerminalAckRef must bind effect key, effect/intent IDs, idempotency
key, payload digest, provider namespace, operation-id digest (or an explicit
no-operation marker), terminal state, receipt digest, current epoch and
fencing tuple. A synthetic internal acceptance marker is allowed only when it
is committed atomically with the terminal ACK and is marked synthetic; it
must not be projected as a separate network acknowledgement.

If the process or bridge loses the response after the provider call, the
writer records DispatchUnknownRef and projects public
EffectReceipt=Indeterminate. Recovery is lookup-only by the canonical effect
key/provider idempotency key or bound operation reference. It must not
blindly start a new provider attempt. 202, Queued, Running, UNAVAILABLE,
an unscoped 404, expired retention, or an unverifiable signature cannot close
the effect. Only a verified terminal state or a canonical NoEffectProof may
produce Reconciled; insufficient evidence is ManualRequired and remains
non-terminal.

### CI guard (normative for active generators)

CI and code generators must:

1. read this supersession block and the v1.3 active-stage pointer;
2. reject emission of historical DispatchStatus values and require the
   explicit crosswalk above;
3. verify E21's closed EffectReceipt required set and conditional terminal
   rules, including reconciliation IDs;
4. verify local exactly-once enqueue versus bridge at-least-once delivery,
   receiver dedupe, changed-payload conflict, and separate attempt counters;
5. verify DispatchAttemptStarted → DirectTerminalAck and
   DispatchUnknown → lookup-only fixtures, including ManualRequired
   non-terminal behavior;
6. verify the complete outbox field set, source-record digest and fsync-witness
   binding, including crash windows before and after the provider call;
7. fail on raw secret/header fields in outbox, receipt, event or projection;
8. fail if a historical paragraph is parsed as an active schema without an
   explicit legacy_status_decode_only marker.

The canonical reconciliation ID preimage is
`preimage_v1(domain=hepta.auth.reconciliation.v1; effect_id; effect_intent_id; attempt;
authority_epoch; owner_epoch; generation)`; same-attempt retries reuse it and a new attempt
or owner epoch creates a new ID. The not-applicable receipt reference is the hashed registry
sentinel `receipt-ref:v1:na:<sha256(preimage_v1(domain=hepta.auth.receipt-ref.na.v1; effect_id;
attempt; authority_epoch; owner_epoch; generation))>` and is never a dereferenceable prior receipt.

This append-only note is implementation guidance for AUTHBUS.11 v1.3.
DEVELOPMENT, INTERNAL_TEST, and RELEASE_PREP remain unblocked by external
provider, KMS/HSM, physical-power, public-settlement, or H8/H9 evidence;
those remain deferred until an explicit FINAL_RELEASE.

<a id="authbus-v13-concrete-contract-registry-crosswalk"></a>
## AUTHBUS.11 v1.3 concrete contract registry crosswalk

Protocol generators MUST resolve
`OpenClaw/AUTHBUS_CANONICAL_CONTRACT_REGISTRY_v1.yaml#/registry` before reading a domain
projection. The four generated contract inputs are:

| Contract | File |
|---|---|
| `hepta.auth.opaque-secret-operation.v1` | `OpenClaw/AUTHBUS_OAUTH_SECRETREF_CONTRACT_v1.yaml#/contract` |
| `hepta.auth.reconcile-evidence.v1` | `OpenClaw/AUTHBUS_RECONCILE_E21_CONTRACT_v1.yaml#/contract` |
| `hepta.auth.outbox-delivery.v1` | `OpenClaw/AUTHBUS_OUTBOX_DISPATCH_CONTRACT_v1.yaml#/contract` |
| `hepta.auth.remote-reservation.v1` + `hepta.gateway.credential.v1` | `OpenClaw/AUTHBUS_REMOTE_RESERVATION_GATEWAY_CONTRACT_v1.yaml#/contract` |

The projections may group fields but cannot alter the registry's status/error registry, E21
required set, canonical dimensions or fence fields. `ManualRequired`, `ReconcileBlocked` and
`DispatchUnknownRef` are non-terminal; a missing or unavailable status lookup cannot release a
reservation. The active stage selector is
`OpenClaw/AUTHBUS_STAGE_MATRIX_v1.yaml#/execution_closure_v1_3/phase_map`; all historical
DispatchStatus text is decode-only. This crosswalk is a static implementation input and does not
claim runtime/provider behavior or change E.41.
