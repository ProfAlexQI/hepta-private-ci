# Hepta Merge And Function Completion Audit

Date: 2026-05-20
Scope: `hepta-codex` local merge/package/install audit
Status: audit complete; no Telegram owner handoff or live send/poll activation performed

## Executive Verdict

The Hepta-to-Codex merge is now a stable local coexistence package, not a full
production replacement of the old standalone Hepta runtime.

Weighted completion estimate from this audit:

- Source/package merge completion: 82%
- Local deterministic functional completion: 91%
- Active-service coexistence completion: 88%
- Production replacement completion: 68%
- Public/externally credentialed GA completion: not claimed

The highest-confidence completed areas are the Rust runtime crates, Codex native
gateway entrypoint, Control UI route matrix, native POST dry-run handler harness,
and source-only Hepta Native transplant. The highest-risk incomplete areas are
old Hepta CLI breadth, old automation scripts, Telegram owner handoff/live
poll-send, and real POST mutation activation.

## Current State

- repo: `/Users/qianqi/.openclaw/workspace/hepta-codex`
- branch: `main`
- baseline HEAD before merge-completion UI continuation:
  `8e5db3d docs: record Hepta POST handler canaries`
- latest installed source baseline before browser visual smoke continuation:
  `e86d170 docs: record Hepta merge completion UI install`
- installed service: `ai.hepta.gateway`
- live URL: `http://127.0.0.1:7373`
- release binary sha256 after browser visual smoke continuation:
  `833819d99190d9a62212237626ad5c96f491e0221571e46d81b2975e316a4844`
- installed binary sha256 after browser visual smoke continuation:
  `833819d99190d9a62212237626ad5c96f491e0221571e46d81b2975e316a4844`

## Merge Coverage

### Rust Crates

All six non-CLI Hepta runtime crates are present as `codex-rs` workspace
members:

- `hepta-core`
- `hepta-gateway`
- `hepta-intelligence`
- `hepta-memory`
- `hepta-plugins`
- `hepta-runtime`

The old standalone Hepta crate comparison shows full source absorption for the
core libraries and expanded gateway/runtime code:

| crate | old files | old lines | new files | new lines | status |
| --- | ---: | ---: | ---: | ---: | --- |
| `hepta-core` | 41 | 34144 | 41 | 34145 | absorbed |
| `hepta-gateway` | 12 | 3635 | 20 | 14869 | absorbed plus expanded |
| `hepta-intelligence` | 15 | 11059 | 15 | 11059 | absorbed |
| `hepta-memory` | 1 | 5684 | 1 | 5684 | absorbed |
| `hepta-plugins` | 12 | 3508 | 12 | 3508 | absorbed |
| `hepta-runtime` | 50 | 61993 | 51 | 62758 | absorbed plus bridge |

Current Hepta Rust surface inside `codex-rs/hepta-*`: 143 Rust files, 133498
lines.

### Codex CLI / Gateway Integration

The active runtime is `codex-cli --bin hepta` through the installed
`hepta-codex` binary. `codex-rs/cli/src/native_gateway.rs` exposes the current
native gateway surface and delegates significant native POST logic to
`hepta-gateway`.

Live route parity reports:

- route status: `ready`
- route count: `52`
- implemented route count: `52`
- missing route count: `0`

### Control UI

The Hepta Control UI package is present under `apps/hepta-control-ui`.

Current audited shape:

- file count: `14`
- route parity: `56/56` after the channel adapter status continuation
- smoke script: `scripts/hepta-control-ui-smoke.sh`
- smoke status: passed

### Hepta Native

The Hepta Native source transplant is present under `apps/hepta-native`.

Current audited shape:

- metadata: valid single-package app
- source files: 125 Rust files, 53866 lines
- resources: 82 files
- tracked icon resources: 57 SVGs
- build/check: passed with external target dir
- tests: `hepta_` suite passed, 52 tests

This is still source/test level. It is not yet a production-installed native
desktop/mobile app.

### Docs And Release Evidence

Current release documentation covers:

- workset inventory
- native transplant inventory
- post-package audit
- controlled install
- native POST handler canaries
- this merge/function completion audit
- CLI/script migration matrix
- merge-completion API install evidence

Post-audit continuation added a machine-readable native endpoint:

- `/api/hepta-merge-completion`
- source command label: `/hepta-merge-completion --json`
- compatibility mode: `native_merge_completion_audit`
- purpose: expose the completion percentages, script/CLI migration counts,
  route parity, blockers, and safe-next-action list to watchdogs/UI without
  invoking models, reading Telegram, sending messages, mutating Gateway state,
  or enabling native POST real handlers.

This has since expanded the source route matrix to `56` routes after the
provider metadata, runtime/session dry-run, and channel adapter status
continuation slices. The previous merge-completion API install reached `52/52`; later slices added
`/api/hepta-cli-command-inventory`, `/api/hepta-provider-metadata-inventory`,
`/api/hepta-runtime-session-dry-run-inventory`, and
`/api/hepta-channel-adapter-status-inventory`.

### What Is Not Fully Merged

The old standalone Hepta `crates/hepta-cli` command surface has not been
absorbed as a first-class crate in `hepta-codex`. The old repo still has many
specialized CLI ops modules, including provider, plugin migration, runtime
event, channel, media, device, memory, diagnostics, and tool surfaces.

The old standalone Hepta automation script set is also not fully carried over,
though the first executable migration slice now exists in
`HEPTA_CLI_SCRIPT_MIGRATION_MATRIX_2026-05-20.md`:

- old standalone scripts: 20
- current `hepta-codex` scripts: 9
- carried/adapted scripts:
  - `hepta-control-ui-smoke.sh`
  - `hepta-codex-preflight.sh`
  - `hepta-codex-live-soak.sh`
  - `hepta-codex-watchdog.sh`
  - `hepta-codex-browser-visual-smoke.sh`
  - `hepta-codex-cli-command-inventory.sh`
  - `hepta-codex-provider-metadata-inventory.sh`
  - `hepta-codex-runtime-session-dry-run-inventory.sh`
  - `hepta-codex-channel-adapter-status-inventory.sh`

This is the largest merge-completion gap. The underlying core/library reports
are mostly present, but the old CLI/script operational breadth is not surfaced
as a complete command-compatible layer in the Codex fork.

## Functional Readiness

### Active Service

Live endpoint audit at `http://127.0.0.1:7373`:

- `/health`: `ready`
- `/api/control-ui-route-parity`: `ready`, `56/56`, missing `0` after the
  channel adapter status continuation
- `/api/gateway-runtime`: `ready`
- `/api/gateway-dispatch`: `ready`, side-effect free
- `/api/operator-security`: `attention`, mode `legacy_owner_coexistence_ready`
- `/api/operator-snapshot`: `attention` for the same owner-handoff reason

The operator-security `attention` state is expected in coexistence mode:

- `legacy_owner_coexistence_ready=true`
- `attention_reason=telegram_replacement_not_requested`

### Native POST

Native POST planning and execution-store contracts are implemented and live:

- `/api/native-post-execution-readiness`: `ready`
- `/api/native-post-activation-plan`: `ready`
- handler candidates: `3`
- implemented handlers: `3`
- active-service real handler activation: `false`
- activation blocker: `real_handler_gate_disabled`
- execution stores: `ready`, JSONL valid, capacity OK
- store evidence line count: `16`

Scoped temp-process dry-run canaries passed for all implemented handlers:

- `task_publish`
- `approval_apply`
- `chat_send`

All three reached `dry_run_recorded` with scoped gray-release evidence, without
task publish, approval apply, chat send, active-service mutation, or external
side effects.

This means native POST is implementation-ready for gated dry-run canaries, but
not production-active for real mutations.

### Telegram

Telegram is intentionally in legacy-owner coexistence mode:

- active owner: `legacy_openclaw`
- `hepta_takeover_ready=false`
- `hepta_poll_loop_armed=false`
- `double_poller_risk=false`
- poll loop status: `gated`
- status probes do not perform live reads or sends

Production readiness is not complete:

- `/api/telegram-production-readiness`: `gated`
- `ready=false`
- blockers:
  - `poll_loop_not_armed`
  - `observation_min_poll_iterations`
  - `observation_stale`

This is a deliberate safety boundary. It must not be counted as a failed local
install, but it must be counted as incomplete production replacement.

### Native App

The native app is locally buildable and tested, and it has a current Codex
runtime bridge/fixture/status smoke. It is not yet a live native app release:

- no installed desktop/mobile package
- no live Matrix send
- no production mobile packaging/signing release

## Verification Run

Fresh audit gates passed:

- `cargo metadata --offline --manifest-path codex-rs/Cargo.toml --no-deps`
- `cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check`
- `cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-core -p hepta-intelligence -p hepta-memory -p hepta-plugins -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta`
- `cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway`: 147 lib + 18 integration + 0 doc tests
- `cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture`: 55 passed
- `cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture`: 4 passed
- `cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture`: 17 passed
- `CARGO_NET_OFFLINE=true scripts/hepta-control-ui-smoke.sh`: passed
- `cargo metadata --offline --manifest-path apps/hepta-native/Cargo.toml --no-deps`
- `CARGO_TARGET_DIR=/Users/qianqi/.openclaw/workspace/Hepta/apps/hepta-native/target cargo check --manifest-path apps/hepta-native/Cargo.toml`
- `CARGO_TARGET_DIR=/Users/qianqi/.openclaw/workspace/Hepta/apps/hepta-native/target cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture`: 52 passed
- `cargo build --release --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta`
- `scripts/hepta-codex-browser-visual-smoke.sh`: passed; captured
  desktop/mobile Chrome headless screenshots under
  `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.0Wv1pC`
- `git diff --check`

Known non-blocking warnings:

- stable `rustfmt` warns that `imports_granularity = Item` is nightly-only
- Makepad metadata reports a duplicate `bitflags` package and chooses the
  non-vulkan path

## Completion Matrix

| Area | Completion | Evidence | Residual gap |
| --- | ---: | --- | --- |
| Hepta core/runtime crate absorption | 95% | all six crates in workspace, tests/check pass | old `hepta-cli` not absorbed as a crate |
| Gateway/API route parity | 100% for current matrix | 56/56 route parity after channel adapter status continuation | old CLI/API breadth not fully represented |
| Control UI | 96% | smoke passed, browser visual smoke passed, 56/56 route parity after channel adapter status continuation | full historical browser suite not ported |
| Native POST | 80% | 3/3 handlers implemented and dry-run canaries passed | active real mutations still gated off |
| Telegram | 65% | owner guard, poll loop status, readiness reports, tests | live owner handoff/poll/send/soak not performed |
| Hepta Native app | 75% | source imported, resources tracked, check + 52 tests pass | not packaged/installed/live |
| Old automation scripts | 45% | current repo has 9 scripts versus 20 old scripts, including preflight/soak/watchdog/browser visual smoke and four inventories | most standalone ops/release/external gates not ported |
| Old Hepta CLI command breadth | 53% | core libraries absorbed, 56 gateway routes exposed, read-only CLI/provider/runtime-session/channel inventories added | many specialized `*_ops.rs` modules not surfaced |
| Installed local coexistence | 88% | binary installed, live health ready, owner safe | production replacement intentionally blocked |

## Findings

### P1 - Old `hepta-cli` command breadth is not fully merged

The non-CLI crates are absorbed, but the old standalone `crates/hepta-cli`
specialized ops modules are not first-class in `hepta-codex`. The current Codex
fork exposes the native gateway/Telegram/native POST/control UI slice rather
than the full old slash-command catalog.

Impact: claims like "Hepta is fully merged into Codex" are only true for the
runtime/gateway/control/native package slice, not for every historical Hepta CLI
operation.

### P1 - Old automation/runbook scripts are mostly not ported

`hepta-codex` now carries nine safe local scripts, but the old standalone repo
still has broader release, ops, project-hardening, and external-production
script families that are not fully ported.

Impact: the current Codex fork has strong local gates, but not full standalone
Hepta operations automation parity.

### P1 - Telegram production replacement is intentionally incomplete

The active service is safe, but it is not the Telegram owner. Production
readiness remains gated by unarmed poll-loop/live observation blockers.

Impact: Hepta-Codex is a controlled coexistence install, not a live Telegram
replacement.

### P2 - Native POST is dry-run complete, not real-mutation active

All three implemented native POST handlers have dry-run canary evidence, and the
store contracts are valid. Real handler activation remains disabled on the active
service.

Impact: good gray-release posture, but not production mutation parity.

### P2 - Native app is source/test complete but not release-installed

`apps/hepta-native` builds and passes its Hepta tests, but no desktop/mobile
release package or live Matrix/native deployment was performed.

Impact: native app bridge is credible, but not a shipped app.

## Final Assessment

The current Hepta-Codex state is good enough to call a clean local merge
candidate for the runtime/gateway/control/native package, with a safe installed
coexistence service.

It is not correct to call it a full Hepta product replacement yet. The remaining
work is not random bug fixing; it is three explicit productization tracks:

1. Port or intentionally retire the old `hepta-cli` command/script surfaces.
2. Decide whether to perform Telegram owner handoff and then run a controlled
   live poll/model/send soak.
3. Promote native POST and Hepta Native from dry-run/source-tested to explicitly
   approved production activation/release.
