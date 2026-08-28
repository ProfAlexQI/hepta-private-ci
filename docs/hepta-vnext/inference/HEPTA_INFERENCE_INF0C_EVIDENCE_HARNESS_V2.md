# Hepta inference INF-0C evidence harness v2

> Status: `SOURCE_PRESENT_NOT_RUN / QUALIFICATION_ONLY`  
> Parent plan: `HEPTA-INFERENCE-RUNTIME-V2`  
> Stacked branch: `codex/hepta-inference-inf0c-evidence-v2-20260828`  
> Production, effect, Memory/KG, route, fleet, model/NPU, remote inference and promotion authority remain disabled.

## 1. Purpose

This tranche closes the source/tooling gap between the minimal real-software smoke and the evidence required before INF-1 may be considered. It adds:

1. a bounded streaming transport-disconnect probe;
2. a digest-pinned, no-shell service-control helper contract;
3. controlled stop/start and exact-model recovery verification;
4. digest-only receipts and hosted self-tests.

It does not activate `hepta-inferd`, does not download a model and does not claim backend cancellation acknowledgement.

## 2. Transport-disconnect evidence

For each provider, the harness:

1. sends a streaming Responses request to the validated loopback endpoint;
2. reads at most a configured prefix, capped at 4096 bytes;
3. records only status, byte length, SHA-256 and timing;
4. closes the response and connection;
5. reruns exact model discovery and bounded inference.

The receipt records:

```text
transport_disconnect_executed=true
backend_cancellation_acknowledged=false
raw_prefix_persisted=false
```

A client-side connection close is intentionally not promoted to proof that the provider acknowledged cancellation or stopped all compute. That stronger claim remains a separate gate.

## 3. Trusted service-control helper

Controlled restart is optional and can execute only when the pre-provisioned runner supplies:

```text
HEPTA_INF0C_SERVICE_CONTROL_HELPER=/absolute/path/to/helper
HEPTA_INF0C_SERVICE_CONTROL_HELPER_SHA256=sha256:<64 lowercase hex>
```

The harness requires:

- a canonical regular file;
- no symlink at the supplied path;
- an exact SHA-256 match;
- fixed argv, never a shell command;
- no stdin;
- stdout and stderr discarded rather than persisted;
- a 60-second process timeout;
- a cleared environment rebuilt from a narrow OS/runtime allowlist.

The helper protocol is exactly:

```text
<helper> stop  ollama
<helper> start ollama
<helper> stop  lmstudio
<helper> start lmstudio
```

Any other action or service identifier is rejected before process creation.

## 4. Controlled restart procedure

For each service, the harness:

1. establishes baseline exact-model readiness;
2. invokes the pinned helper with `stop`;
3. requires the loopback health endpoint to become unreachable;
4. invokes the helper with `start`;
5. waits for exact model discovery and bounded inference to pass again;
6. records helper digest, exit codes, stop/start timing, observed outage and post-restart response digests.

If an error occurs after a successful stop and before start completes, the harness makes a best-effort fixed-argv start call. This recovery attempt does not convert a failed qualification into a pass.

## 5. Network and privacy boundary

Both the minimal and v2 harnesses retain:

- loopback HTTP only;
- explicit non-zero ports;
- no userinfo, query or fragment;
- loopback-only DNS results;
- no environment proxies;
- no redirects;
- no implicit model installation;
- bounded response bodies;
- create-only owner-private receipts;
- no raw prompt, model output, streaming prefix or helper output in receipts.

## 6. Hosted source self-test

The normal hosted source job runs:

```bash
python3 -m py_compile \
  scripts/hepta-inference-inf0-source-gate.py \
  scripts/hepta-inference-inf0c-real-e2e.py \
  scripts/hepta-inference-inf0c-evidence-v2.py

python3 scripts/hepta-inference-inf0c-evidence-v2.py --self-test
```

The self-test validates the import boundary, digest parser, helper canonicalization, helper digest binding and fixed stop/start invocation without contacting a real model service.

## 7. Manual real-service execution

The self-hosted job requires exact pre-installed model IDs. It first produces the minimal real-software receipt, then produces the disconnect/restart evidence receipt. Controlled restart remains opt-in through the workflow boolean and runner-pinned helper environment.

Neither receipt may set `qualified=true`. INF-0C qualification additionally requires exact-head source/Rust gates and review of the actual real-service receipts.

## 8. Remaining barrier

Even after transport disconnect and controlled restart execute successfully, the following remain distinct:

- provider/backend cancellation acknowledgement;
- INF-1 daemon request-generation cancellation fencing;
- real worker crash/restart fencing;
- hardware performance receipts;
- operator acceptance and promotion.

Until those gates are satisfied, the authoritative state remains:

```text
SOURCE_PRESENT_NOT_RUN
qualified=false
INF-1=NOT_STARTED
```
