# Hepta inference INF-0C evidence harness v2

> Status: `SOURCE_PRESENT_NOT_RUN / QUALIFICATION_ONLY`  
> Parent plan: `HEPTA-INFERENCE-RUNTIME-V2`  
> Stacked branch: `codex/hepta-inference-inf0c-evidence-v2-20260828`  
> Production, effect, Memory/KG, route, fleet, model/NPU, remote inference and promotion authority remain disabled.

## 1. Purpose

This tranche closes the source/tooling gap between the compatibility-hardening source and the evidence required before INF-1 may be considered. It adds:

1. exact semantic-output verification for the minimal real-model smoke;
2. loopback DNS-to-literal pinning and response media-type fences;
3. a bounded streaming transport-disconnect probe;
4. a digest-pinned, no-shell service-control helper contract;
5. per-invocation helper identity/digest revalidation;
6. controlled stop/start and exact-model recovery verification;
7. digest-only receipts and hosted hermetic self-tests.

It does not activate `hepta-inferd`, download a model, or claim provider/backend cancellation acknowledgement.

## 2. Minimal real-model semantic evidence

The fixed prompt requests exactly:

```text
HEPTA_INF0C_OK
```

For both Ollama and LM Studio, qualification requires:

- the exact configured model ID is already installed;
- the endpoint returns `application/json`;
- the Responses payload contains output text in a recognized Responses field;
- the normalized output equals `HEPTA_INF0C_OK` exactly.

The receipt stores only:

```text
semantic_output.verified=true
semantic_output.sha256=<digest>
semantic_output.byte_length=<length>
semantic_output.raw_persisted=false
```

A parseable JSON response without the exact semantic marker is a failure.

## 3. Loopback pinning and protocol media types

All service URLs remain HTTP-only and must use `localhost`, `127.0.0.1`, or `::1` with an explicit non-zero port. Resolution must return loopback addresses only. The harness then rewrites the in-memory request base to a resolved loopback IP literal before connecting, preventing a second hostname lookup from changing the destination.

Bounded JSON requests require:

```text
application/json
```

Streaming disconnect requests allow only:

```text
text/event-stream
application/json
```

Missing or different media types fail closed.

## 4. Transport-disconnect evidence

For each provider, the harness:

1. sends a streaming Responses request to the pinned loopback literal;
2. reads at most a configured prefix, capped at 4096 bytes;
3. records only status, normalized media type, byte length, SHA-256 and timing;
4. closes the response and connection;
5. reruns exact model discovery, media-type validation and semantic inference.

The receipt records:

```text
transport_disconnect_executed=true
backend_cancellation_acknowledged=false
raw_prefix_persisted=false
```

A client-side close is intentionally not promoted to proof that the provider acknowledged cancellation or stopped compute.

## 5. Trusted service-control helper

Controlled restart is optional and can execute only when the pre-provisioned runner supplies:

```text
HEPTA_INF0C_SERVICE_CONTROL_HELPER=/absolute/canonical/path/to/helper
HEPTA_INF0C_SERVICE_CONTROL_HELPER_SHA256=sha256:<64 lowercase hex>
```

The harness requires:

- an absolute path already in canonical form;
- a regular executable file, not a symlink;
- exact SHA-256;
- on Unix, neither the file nor its immediate parent may be group/other writable;
- on Unix, file and parent ownership must be root or the current runner user;
- file and parent device, inode, mode, size and modification time are captured;
- identity and SHA-256 are revalidated immediately before and after every invocation;
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

## 6. Controlled restart procedure

For each service, the harness:

1. establishes baseline exact-model readiness and semantic output;
2. revalidates the helper and invokes `stop`;
3. requires the loopback health endpoint to become unreachable;
4. revalidates the helper and invokes `start`;
5. waits for exact model discovery and exact semantic output to pass again;
6. records helper digest, invocation revalidation, exit codes, timing, observed outage and post-restart response digests.

If an error occurs after a successful stop and before start completes, the harness makes a best-effort fixed-argv start call. The recovery attempt does not convert a failed qualification into a pass.

## 7. Privacy boundary

The minimal and v2 harnesses retain:

- no environment proxies;
- no redirects;
- no implicit model installation;
- bounded response bodies and stream prefixes;
- create-only owner-private receipts;
- no raw prompt, model output, streaming prefix or helper output in receipts;
- all authority fields false.

## 8. Hosted hermetic evidence

The hosted source job runs:

```bash
python3 -m py_compile \
  scripts/hepta-inference-inf0-source-gate.py \
  scripts/hepta-inference-inf0c-real-e2e.py \
  scripts/hepta-inference-inf0c-evidence-v2.py \
  scripts/hepta-inference-inf0c-evidence-v2-loopback-selftest.py \
  scripts/hepta-inference-inf0c-evidence-v2-source-gate.py

python3 scripts/hepta-inference-inf0c-evidence-v2.py --self-test
python3 scripts/hepta-inference-inf0c-evidence-v2-loopback-selftest.py
python3 scripts/hepta-inference-inf0c-evidence-v2-source-gate.py
python3 scripts/hepta-inference-inf0-source-gate.py
```

The self-tests cover:

- exact semantic-output success and mismatch rejection;
- loopback hostname-to-literal pinning;
- supported and unsupported stream media types;
- all four fixed helper argv pairs;
- invalid action and service rejection;
- sensitive environment non-inheritance;
- helper replacement detection before a later invocation.

## 9. Manual real-service execution

The self-hosted job requires exact pre-installed model IDs. It first produces the minimal semantic receipt, then the disconnect/restart receipt. Controlled restart remains opt-in through the workflow boolean and the runner-pinned helper environment.

Neither receipt may set `qualified=true`. INF-0C qualification additionally requires exact-head source/Rust gates and review of the actual real-service receipts.

## 10. Remaining barrier

Even after semantic inference, transport disconnect and controlled restart execute successfully, the following remain distinct:

- provider/backend cancellation acknowledgement;
- INF-1 daemon request-generation cancellation fencing;
- real worker crash/restart fencing;
- hardware performance receipts;
- operator acceptance and promotion.

Until those gates are satisfied, the authoritative state remains:

```text
SOURCE_PRESENT_NOT_RUN
qualified=false
backend_cancellation_acknowledged=false
INF-1=NOT_STARTED
```
