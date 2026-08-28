# Hepta inference INF-0C explicit cancellation capability v3

> Status: `SOURCE_PRESENT_NOT_RUN / QUALIFICATION_ONLY`  
> Parent receipt: `9d5a592d50e333fd5db3bf73f5ab3fc9fe4d8988`  
> Stacked branch: `codex/hepta-inference-inf0c-cancel-capability-v3-20260828`  
> All production, effect, Memory/KG, route, fleet, model/NPU, remote-inference and promotion authority remains disabled.

## 1. Purpose

The previous evidence lane deliberately separated a client transport close from provider/backend cancellation acknowledgement. This tranche adds the missing explicit capability probe without changing that boundary.

The probe uses the Responses background-cancellation shape:

```text
POST <responses endpoint>                 background=true, store=true
POST <responses endpoint>/<id>/cancel
GET  <responses endpoint>/<id>
```

An acknowledgement exists only when the cancel response and a subsequent retrieve both bind the same canonical response ID and report a cancelled terminal status.

## 2. Evidence classifications

### `explicit_background_cancel_acknowledged`

All of the following must hold:

- background create returns a bounded `application/json` response;
- the initial status is `queued` or `in_progress`;
- the response ID is canonical, bounded and safe for one path segment;
- the explicit cancel endpoint returns the same response ID;
- cancel status is `cancelled` or `canceled`;
- terminal retrieve returns the same response ID and cancelled status;
- no transport-disconnect inference is used.

### `explicit_cancel_unsupported`

A provider returns a bounded protocol response showing that background creation, explicit cancel or terminal retrieval is unsupported. This is a valid capability classification, but it is not cancellation acknowledgement and it cannot qualify the provider.

### `failed_closed`

Malformed JSON, wrong media type, response-ID mismatch, path-unsafe ID, non-cancelled terminal state, timeout or transport error fails closed.

## 3. Response-ID and privacy boundary

The raw response ID is used only in memory after validation against a bounded ASCII path-segment grammar. Receipts store only its SHA-256 and byte length.

Receipts never contain:

- raw prompt;
- raw response ID;
- model output;
- provider error body;
- Authorization, Cookie or secret headers.

All HTTP bodies are bounded to 4 MiB, endpoints are pinned loopback literals, environment proxies are disabled and redirects are rejected through the parent harness.

## 4. Semantic distinction

The receipt fields remain independent:

```text
provider_cancel_capability_classified
provider_cancel_acknowledged
backend_cancellation_acknowledged
transport_disconnect_used
```

`transport_disconnect_used=false` is mandatory for explicit acknowledgement. An unsupported provider keeps both acknowledgement fields false.

## 5. Hosted and real-service evidence

Hosted CI runs Python compilation, a hermetic supported fixture, unsupported create/cancel fixtures, media-type rejection, response-ID mismatch rejection and non-cancelled terminal rejection. It then runs the v3 source gate plus both parent source gates.

The optional self-hosted job targets fixed pre-installed Ollama and LM Studio models. It always uploads a create-only digest-only receipt, including when a provider is classified unsupported and the job fails.

## 6. Remaining barrier

Source presence does not prove either provider implements explicit cancellation. Until a real fixed-tuple receipt is reviewed:

```text
provider_cancel_capability_probe_executed=false
provider_cancel_capability_classified=false
provider_explicit_cancel_acknowledged=false
backend_cancellation_acknowledged=false
qualified=false
INF-1=NOT_STARTED
```

This tranche does not activate `hepta-inferd`; it only prevents a transport close from being promoted into a false backend acknowledgement.
