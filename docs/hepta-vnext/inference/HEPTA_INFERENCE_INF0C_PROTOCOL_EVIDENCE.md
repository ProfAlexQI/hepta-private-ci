# Hepta inference INF-0C protocol evidence

> Status: `SOURCE_PRESENT_NOT_RUN / QUALIFICATION_ONLY`  
> Parent plan: `HEPTA-INFERENCE-RUNTIME-V2`  
> Parent stack: PR #11 → PR #35  
> Protocol branch: `codex/hepta-inference-inf0c-protocol-evidence-20260828`

## 1. Purpose

This tranche completes the remaining source tooling named by the INF-0C/L2 software-evidence plan:

- exact function/tool-call verification;
- no-implicit-download evidence based on model inventory stability;
- a bounded strict SSE parser;
- malformed event, truncation, ordering, media-type and timeout fixtures.

It does not activate `hepta-inferd`, grant authority, install a model, or convert hermetic fixtures into real-provider qualification.

## 2. Exact tool call

The qualification prompt requires exactly one function call:

```text
name  = hepta_probe
nonce = HEPTA_INF0C_TOOL
value = 7
```

The request binds `tool_choice` to that function, disables parallel tool calls, uses a closed JSON Schema and limits output tokens.

The response must contain exactly one recognized Responses function-call item. The function name and arguments must match exactly, including the integer type. Missing keys, additional keys, malformed JSON, a wrong nonce/value, another function name, or multiple calls fail closed.

The receipt stores:

```text
tool_call.verified=true
tool_call.name=hepta_probe
tool_call.arguments_sha256=<digest>
tool_call.arguments_byte_length=<length>
tool_call.raw_arguments_persisted=false
```

A call identifier, when present, is stored only as SHA-256 and length.

## 3. Model inventory and no implicit download

For each provider the harness:

1. reads and validates the complete model inventory;
2. requires unique bounded canonical model IDs;
3. requires the exact configured model;
4. computes a canonical sorted inventory digest;
5. performs the tool-call request;
6. fetches the inventory again;
7. requires exact tuple equality and digest equality.

The receipt never persists raw model IDs. An inventory change is a failure, so normal qualification cannot silently install, remove, rename or replace a model.

## 4. Strict SSE parser

The parser accepts only `text/event-stream`, reads at most 4 MiB total, limits one event to 256 KiB, limits the stream to 4096 events and persists no raw event content.

Only `event` and `data` SSE fields are allowed. Comments are ignored. The event name, when present, must equal the JSON payload `type`. JSON must be an object whose type is in the explicit allowlist. When `sequence_number` is supplied, it must equal the zero-based monotonic event sequence.

The stream must contain exactly one terminal `response.completed`; any event after completion is rejected. The following fail closed:

- invalid UTF-8 or JSON;
- an unknown SSE field or event type;
- event/payload type mismatch;
- non-monotonic sequence;
- `[DONE]` legacy sentinel;
- a missing terminal event;
- duplicate completion;
- data after completion;
- an unterminated event;
- oversized event/stream/event count;
- wrong media type;
- HTTP failure;
- request/read timeout.

A valid stream receipt contains only status, media type, byte count, stream SHA-256, event counts, completion state and timing.

## 5. Hermetic evidence

Hosted self-tests include:

- valid exact tool call;
- wrong nonce;
- duplicate function calls;
- canonical inventory ordering;
- duplicate inventory rejection;
- a valid two-event SSE stream;
- malformed JSON;
- unknown event;
- event/payload mismatch;
- truncation;
- unterminated event;
- duplicate completion;
- event after completion;
- bad sequence;
- legacy `[DONE]`;
- unknown SSE field;
- oversized event;
- wrong media type;
- stream timeout.

These tests prove parser and policy behavior only. They do not prove a real Ollama or LM Studio model supports tool calls.

## 6. Real-service execution

The manual self-hosted job requires exact pre-installed Ollama and LM Studio model IDs and runs the tool-call harness against loopback IP-literal-pinned endpoints. It produces a create-only owner-private digest-only receipt.

Real-service success requires:

```text
exact model present before
exact one tool call
exact function name and arguments
model inventory identical after
implicit_download=false
raw output/arguments/model IDs not persisted
qualified=false
```

The strict SSE malformed-event matrix remains hermetic in this tranche. A real provider streaming receipt remains a separate required artifact.

## 7. Authority and stage boundary

All machine-readable authority remains closed:

```text
production_listener=false
production_writer=false
provider_effect=false
external_effect=false
shared_kg_write=false
memory_write=false
route_write=false
fleet_write=false
model_npu=false
remote_inference=false
automatic_model_install=false
operator_acceptance=false
promotion=false
release=false
```

The authoritative stage remains:

```text
SOURCE_PRESENT_NOT_RUN
real_tool_call_executed=false
real_model_inventory_stability_verified=false
real_strict_sse_executed=false
backend_cancellation_acknowledged=false
qualified=false
INF-1=NOT_STARTED
```

## 8. Remaining barrier

INF-1 remains inactive until the same frozen stack has:

- parent source, Rust formatting, tests and strict Clippy PASS;
- evidence-v2 semantic/disconnect/restart source PASS;
- protocol parser and malformed-event source PASS;
- real fixed-tuple semantic and tool-call receipts;
- model inventory stability receipts;
- transport-disconnect and controlled-restart receipts;
- separate provider/backend cancellation acknowledgement evidence;
- all negative-authority fields preserved.
