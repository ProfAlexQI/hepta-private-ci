# Hepta Inference INF-0C Source Status — 2026-08-28

## Binding

- Plan: `HEPTA-INFERENCE-RUNTIME-V2@2.0.0`
- Canonical base: `integration/vnext-main-20260811@fe0889ecd46a5fc89de7b1ff3f28158c133a3502`
- Development branch: `codex/hepta-inference-runtime-v2-20260828`
- State: `SOURCE_PRESENT_NOT_RUN`
- Qualified: `false`
- INF-1 activation: `false`

## Completed source work

### Ollama

- non-2xx model/version/pull responses fail closed with stable error codes;
- malformed model and version payloads fail closed;
- pull transport, UTF-8, JSON, remote-error and unexpected-EOF paths are terminal failures;
- individual pull frames are bounded to 1 MiB;
- a trailing frame without a newline is parsed;
- success is emitted once;
- normal readiness no longer performs an implicit `ollama pull`;
- `wiremock` is test-only.

### LM Studio

- readiness waits for the minimal load probe;
- normal readiness no longer invokes model download;
- explicit download requires a canonical regular `lms` executable;
- `CODEX_LMS_CLI_SHA256=sha256:<64 lowercase hex>` binds executable provenance;
- the CLI runs through `tokio::process::Command` with a hard timeout and `kill_on_drop`;
- stderr is drained but only the first bounded diagnostic bytes are retained;
- endpoint and payload failures carry stable error codes.

### Debug dump privacy

- request and response bodies are represented by SHA-256, byte length and completeness only;
- raw prompt, source code, tool arguments and model output are not persisted;
- authorization, cookie, token, secret and API-key headers are redacted;
- Unix directories/files are forced to `0700`/`0600`;
- dump retention is bounded to 24 hours and 256 JSON files;
- files are create-only and cannot silently overwrite earlier evidence.

### Real-software harness

`scripts/hepta-inference-inf0c-real-e2e.py` is an explicit, loopback-only,
no-download harness for pre-provisioned Ollama and LM Studio models. It records
only response hashes, lengths, status and timing. Its current scope does not
claim cancellation or controlled restart evidence.

## Evidence boundary

The following remain unexecuted on the exact candidate:

- hosted/source runner steps;
- Rust formatting, tests and Clippy;
- real Ollama/LM Studio model execution;
- cancellation and controlled service restart;
- hardware qualification.

A runner result with `steps=[]`, `runner_id=0`, or no checkout is
`BLOCKED_RUNNER_NOT_ASSIGNED`, not a source failure and not a PASS.

## Authority

All production, effect, Memory/KG, route, fleet, NPU, remote-inference,
operator-acceptance, promotion and release authority remains false.
