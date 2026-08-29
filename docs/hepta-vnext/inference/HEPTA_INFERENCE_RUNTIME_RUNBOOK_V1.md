# Hepta Local Inference Runtime Runbook V1

This runbook implements Plan V4. It is qualification-only. It does not authorize
production activation, implicit model installation, Memory/KG writes, or remote fallback.

## 1. Before every run

Record and verify:

```bash
git remote get-url origin
git rev-parse HEAD
git rev-parse 'HEAD^{tree}'
git status --porcelain
sha256sum codex-rs/Cargo.lock
cat rust-toolchain.toml
```

Abort with `BASE_DRIFT` when the expected branch/head/tree changed. Never re-label evidence
from one commit as evidence for a descendant.

## 2. Source qualification

The V4 exact-head workflow must run:

```bash
python3 -m py_compile scripts/hepta-inference-v4-source-truth.py
python3 scripts/hepta-inference-v4-source-truth.py

cd codex-rs
cargo metadata --locked --no-deps --format-version 1
cargo fmt -p codex-hepta-infer-core -p codex-hepta-infer-client \
  -p codex-hepta-inferd -- --check
cargo check --locked --all-targets <all inference packages>
cargo test --locked --all-targets <all inference packages>
cargo clippy --locked --all-targets --no-deps <all inference packages> -- -D warnings
```

Repository `AGENTS.md` remains authoritative for `just fmt`, scoped tests, fixes, Bazel
lock updates, module size, and dependency changes.

## 3. Model registration

Registration is explicit and offline:

1. place the already-acquired model in an owner-only directory;
2. hash the model, GGUF metadata, tokenizer, template, license, and SBOM;
3. bind the pinned runtime build and upstream commit;
4. bind the exact device and resource profile;
5. review the tuple;
6. invoke the operator channel with the digest-bound manifest.

Never accept a mutable tag as model authority. Never download a model in readiness,
admission, product startup, or qualification source jobs.

## 4. Start sequence

```text
create private runtime directory
-> verify owner and mode
-> open receipt journal/index
-> recover and reconcile receipts
-> create public, worker and operator sockets
-> mint daemon session secrets
-> spawn worker with allowlisted environment
-> authenticate worker hello
-> load exact tuple only on explicit request
-> enter qualification-ready state
```

Any failed step leaves the daemon fail closed and emits only digest-safe diagnostics.

## 5. Cancellation

```text
queued request:
  mark cancelled -> persist receipt -> release accounting

running request:
  CancelRequested -> send private cancel -> await bounded ACK
    ACK observed -> persist acknowledged cancellation
    timeout -> terminate process group -> observe process death
            -> roll backend generation -> fail colocated requests closed
            -> persist forced-kill receipts
```

Do not set `forced_worker_termination=true` before process death is observed.

## 6. Receipt recovery and maintenance

At startup:

- reject unsafe directory ownership/mode;
- recover generation and journal;
- verify receipt filenames, canonical encoding, digests, and replay uniqueness;
- rebuild the terminal index;
- reconcile orphaned nonterminal records as failed closed;
- apply TTL and disk budget;
- compact through atomic replace and directory sync.

Raw prompt, token text, model output, capability secret, and session nonce must not be
stored. Corruption produces a fail-closed diagnostic and does not silently delete evidence.

## 7. Provider qualification

For Ollama and LM Studio:

- use only pinned loopback IP literals;
- disable proxy and redirect;
- require exact preinstalled model ID;
- verify service version and inventory before and after;
- run semantic output, strict media/JSON/SSE, tool-call, timeout, disconnect, restart,
  cancellation, and no-download checks;
- bind the control helper path and SHA-256;
- classify cancel as acknowledged, unsupported fail closed, or disconnect observed.

## 8. Native qualification

The native job must bind the llama.cpp commit and build digest, exact GGUF tuple, and
device profile. It must execute:

```text
load -> warm -> submit -> first real token -> stream -> complete
cancel before first token
cancel during stream
hang -> forced kill
crash -> generation rollover
restart -> reject stale token
unload -> verify bounded memory reclamation
```

A fixture token is E1 evidence only and cannot satisfy E3.

## 9. Product shadow

Enable only through an explicit developer flag. The authoritative route remains
unchanged. Record digest-safe comparison metadata and the daemon receipt. On any bridge
error, disable shadow processing for that request. The bridge has no Memory/KG writer.

## 10. Incident and rollback triggers

Immediate drain/disable triggers include:

- authority bit drift;
- remote endpoint or implicit download observation;
- raw prompt/output persistence;
- receipt corruption or replay;
- capability/session leakage;
- unbounded RSS/disk/connection growth;
- cancellation timeout without observed kill;
- stale token accepted after generation rollover;
- exact model/runtime/device digest mismatch;
- product shadow influencing authoritative output.

Rollback is the kill switch plus worker drain/termination. Promotion and release require
separate operator authority and are outside this source package.
