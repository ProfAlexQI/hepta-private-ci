# Hepta Architecture Merge Route - 2026-05-19

## Correction

The target is an architecture merge from old `Hepta` into `hepta-codex`.
This is not a minimal absorption plan and not another adapter pass.

The original version of this route was written before the final production
replacement work. As of the 2026-05-21 retirement pass, the active Hepta product
line is `hepta-codex`: launchd, runtime config, Telegram, native POST state, and
the native desktop/mobile app all point at the `hepta-codex` workspace. The old
standalone Native app and duplicate `.hepta` runtime state have been removed
from the old repository.

The old Hepta architecture has now been absorbed as first-class crates under
`hepta-codex/codex-rs`:

- `crates/hepta-core`
- `crates/hepta-memory`
- `crates/hepta-intelligence`
- `crates/hepta-runtime`
- `crates/hepta-gateway`
- `crates/hepta-plugins`
- `crates/hepta-cli`

Some compatibility and HTTP routing logic still lives inside
`codex-rs/cli/src/native_gateway.rs` and `codex-rs/cli/src/native_telegram.rs`,
but the runtime policy, gateway, memory, intelligence, plugin, and core layers
are now workspace-native crates inside the new product line.

## Target Architecture

`hepta-codex` becomes the single product and process boundary.

Inside that product, the old Hepta layers must become workspace-native crates,
not compatibility shims:

```text
hepta-codex/
  codex-rs/
    hepta-core/          # contracts, memory/intelligence data types, policies
    hepta-memory/        # store snapshots, recall, inspection, restore preview
    hepta-intelligence/  # memory intelligence, topic/neuron, eval, phase gates
    hepta-plugins/       # plugin contracts required by runtime
    hepta-runtime/       # RuntimeKernel, task board, model routing, reports
    hepta-gateway/       # transport, delivery ledger, dispatch, HTTP route core
    cli/                 # binary entrypoint and thin HTTP/CLI shell only
    core/                # Codex session/model/tool engine remains authoritative
    state/               # Codex durable thread/memory/runtime state
    thread-store/        # Codex thread history source
```

Ownership rules:

- Codex remains authoritative for model invocation, session lifecycle, tool
  execution policy, sandboxing, thread store, and local state DB.
- Hepta owns memory intelligence, runtime readiness, operator surfaces,
  gateway routing policy, delivery ledgers, task board, and control-plane
  reports.
- Telegram ownership must not be flipped again unless explicitly requested.
  Current owner remains old OpenClaw Telegram.
- `codex-rs/cli/src/native_gateway.rs` must be shrunk over time into a shell
  that calls `hepta-gateway` and `hepta-runtime`, not a growing monolith.

## Current Audit Snapshot

`hepta-codex`:

- Pre-merge audit baseline was clean on `main@00320d3`.
- Current worktree now contains the architecture transplant work described
  below.
- Active binary is `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex`.
- Native gateway runs on `127.0.0.1:7373`.
- Control UI route parity is `50/50`.
- Hepta Telegram gates are off.
- Old OpenClaw Telegram is enabled and remains owner.
- Native POST real handlers are gated off by default.

Old `Hepta`:

- Repo clean on `main@25f14b5`.
- `cargo check -q -p hepta-core -p hepta-memory -p hepta-intelligence -p hepta-runtime` passes.
- Full architecture still exists in modular crates.
- Intelligence is not yet present in `hepta-codex` as a real crate/module.

Size/risk indicators from audit:

- Old Hepta selected Rust surface is about 254k lines across core, memory,
  intelligence, runtime, gateway, and CLI.
- Current `hepta-codex` Hepta-specific native gateway proving code is concentrated
  in `codex-rs/cli/src/native_gateway.rs` and `codex-rs/cli/src/native_telegram.rs`.
- Therefore the next phase must reduce monolith growth and restore Hepta's
  crate boundaries inside the Codex fork.

## Current Execution Status

Lane A is executed in the current worktree:

- `hepta-core`, `hepta-memory`, `hepta-intelligence`, and `hepta-plugins`
  are now first-class `codex-rs` workspace crates.
- Required UI/release assets referenced by `hepta-core` were copied into the
  product root instead of replacing them with placeholders.
- The four crates compile and test inside `hepta-codex` without reading the old
  `Hepta` repo.

Lane B is executed for the runtime crate transplant:

- `hepta-runtime` is now a first-class `codex-rs` workspace crate.
- The old runtime `workspace_root` assumption was corrected for the new product
  layout: Hepta runtime now treats `hepta-codex/` as the product root, while
  `codex-rs/` remains the Rust implementation workspace.
- Runtime worker evidence paths were updated to recognize both old Hepta layout
  and the new `hepta-codex/codex-rs/hepta-runtime` layout.
- Root ADR fixture expected by old runtime tests was copied into
  `docs/decisions/`.

Lane C has started with a real gateway crate boundary:

- `hepta-gateway` is now a first-class `codex-rs` workspace crate.
- `codex-cli` now depends on `hepta-gateway` directly.
- `/api/native-gateway` includes `gateway_route_core_status`, produced from
  `hepta-gateway::GatewaySurface`, so the CLI gateway surface has a first
  library-backed route-core connection instead of only local status prose.
- Native POST route contracts and the real-handler plan-kind registry moved to
  `hepta-gateway::native_post`. `codex-cli` now consumes the gateway crate as
  the fact source for the 11 POST routes and 3 scoped real-handler candidates.
- Native POST body schema, body admission, confirmation/rollback contract,
  idempotency evidence, and audit-event contract planning also moved to
  `hepta-gateway::native_post`. `codex-cli` now delegates those pure planning
  contracts to the gateway crate.
- Native POST environment gate names, execution admission, and real-handler
  scope matching/selection also moved to `hepta-gateway::native_post`.
- Native POST execution-store/harness evidence moved to
  `hepta-gateway::native_post`: redacted JSONL store records, store writes,
  duplicate idempotency suppression, rate-limit checks, capacity preflight, and
  scoped dry-run harness reporting are now gateway-crate owned.
- Native POST execution-readiness route report assembly moved to
  `hepta-gateway::native_post`; `codex-cli` now passes only env-derived gate
  and handler-scope state for that report.
- Native POST execution-store status, activation-plan, rollout-evidence, and
  gray-release-evidence report assembly also moved to
  `hepta-gateway::native_post`. `codex-cli` now provides only the store root,
  limits, real-handler gate state, approval state, and handler scope.
- Native POST plan response assembly also moved to `hepta-gateway::native_post`;
  `codex-cli` now only passes request parameter/body, env gate state, handler
  scope, and evidence-store limits/root.
- Native POST route dispatch for plan routes also moved to
  `hepta-gateway::native_post_dispatch_plan_report(...)`; `codex-cli` now wraps
  the gateway-produced response as HTTP JSON.
- Telegram delivery ledger read-only status scanning moved to
  `hepta-gateway::telegram_delivery`.
- Telegram delivery ledger write lifecycle also moved to
  `hepta-gateway::telegram_delivery`; the gateway crate now owns redacted
  lifecycle record construction, JSONL append, retry backoff, and permanent
  error classification. `codex-cli` still owns the actual Telegram Bot API
  send attempt and cursor commit timing.
- Telegram drain gate policy and execution-plan assembly moved to
  `hepta-gateway::telegram_policy`; the gateway crate now owns drain stage
  ordering, first-missing-gate selection, and the status-probe pipeline gate
  rule. `codex-cli` still reads env vars and executes the network/model/cursor
  side-effect boundary.
- Telegram duplicate-suppression / next-update-offset policy also moved to
  `hepta-gateway::telegram_policy`; the gateway crate now owns
  `NativeTelegramDuplicateDecision`, drained-update detection, next-offset
  derivation, and cursor-write eligibility for duplicate/model candidates.
- Telegram send-request planning and send-execution report initial status moved
  to `hepta-gateway::telegram_policy`; the gateway crate now owns
  `NativeTelegramSendRequestPlan`, `NativeTelegramSendExecutionReport`, send
  gate/send-allowed calculation, waiting-state classification, and cursor-commit
  eligibility after a model output. The actual send execution lifecycle is now
  owned by `hepta-gateway::telegram_transport`.
- Telegram model candidate extraction, candidate materialization, model-turn
  plan assembly, model invocation request planning, and model execution report
  initial status moved to `hepta-gateway::telegram_policy`. The gateway crate
  now owns redacted candidate classification for messages, edited messages,
  callback queries, and reactions; prompt material is kept only in memory and is
  not serialized into status JSON.
- Telegram ingress inspection counters also moved to
  `hepta-gateway::telegram_policy`. The gateway crate now owns read-only update
  counting, latest observed/allowed update id tracking, latest allowed
  next-update-offset derivation, and message/callback/reaction counters without
  serializing prompt text, callback payloads, chat ids, sender ids, or raw
  update payloads.
- Telegram model execution orchestration moved to
  `hepta-gateway::telegram_runtime`. The gateway-runtime boundary now owns
  `NativeTelegramModelExecutionInput`, `NativeTelegramModelExecutionOutcome`,
  `NativeTelegramDrainPipelineInput`, `NativeTelegramDrainPipelineOutcome`,
  gated runner eligibility, duplicate suppression before runner invocation,
  prompt/output trimming, execution report transitions, model-error redaction,
  and drain pipeline sequencing from candidate selection through send
  execution. `codex-cli` still owns concrete runner selection and process/API
  execution for MLX, in-process runtime, or child process mode.
- Telegram cursor status/write lifecycle moved to
  `hepta-gateway::telegram_cursor`. The gateway crate now owns cursor schema
  parsing, legacy cursor compatibility, durable cursor evidence classification,
  redacted next-update-offset writes, and the read-only
  `NativeTelegramCursorPlan` status wrapper. `codex-cli` still owns the timing
  of cursor commits after delivery or duplicate suppression.
- Telegram Bot API transport request planning moved to
  `hepta-gateway::telegram_transport`. The gateway crate now owns getUpdates
  query shaping, sendMessage/sendChatAction request bodies, token-shape checks,
  token-like redaction, transient/conflict classification, getUpdates retry
  loop semantics, send retry decisions, actual Bot API `reqwest` call wrappers,
  typing keepalive thread/stop policy, per-chat send rate-limit state, and
  post-model send execution orchestration. Gateway-owned send execution now
  covers enqueued/acked/failed delivery ledger lifecycle writes, Bot API ack
  interpretation, retry loop/sleep through supplied attempts/backoff, and
  cursor commit after a successful ack. The gateway crate also owns
  `NativeTelegramTransportPlan` and `NativeTelegramSendPlan`. `codex-cli` still
  owns token/env reads, supplying retry attempts/backoff, and the model runner
  invocation boundary.
- Concrete Telegram model runner selection moved to
  `hepta-runtime::telegram_model_runner` and is re-exported through
  `hepta-gateway`. Runtime now owns the pure selection plan for
  `mlx-local`, in-process, and child-process runners, including MLX model-ref
  parsing, local MLX base URL sanitization, max-token clamping, and the
  process-spawn vs local-network distinction.
- Telegram MLX/child-runner execution helper policy first moved to
  `hepta-runtime::telegram_model_runner`, and the pure kernel-owned portion has
  now moved further into `hepta-kernel`: OpenAI-compatible MLX request body
  shape, response text extraction, child-runner argument planning, model
  timeout clamping, final-message extraction, and child exit-status formatting
  are kernel-owned policy. `hepta-runtime` keeps compatibility facades and the
  concrete child wait/kill helper; `codex-cli` still reads env vars and
  performs the actual MLX HTTP request / in-process runner / child-process
  spawn.
- `hepta-kernel` no longer depends on the full `codex-core` crate just to carry
  static Codex engine constants. It keeps the tool/plugin sigils and
  `AGENTS.md` filename as local kernel contract constants, which avoids pulling
  Codex websocket/provider dependencies into the native app package graph.
- Concrete Telegram model runner invocation facade and model-error
  classification/redaction also moved to
  `hepta-runtime::telegram_model_runner`. Runtime now owns
  `NativeTelegramModelRunnerInvocationOutcome`,
  `invoke_native_telegram_model_runner_with_plan(...)`, selected-runner
  dispatch across MLX / in-process / child-process closures, empty
  prompt/output handling, common model-runner error classification, and
  Telegram-token-like error redaction. `hepta-gateway::telegram_runtime` now
  applies the runtime-owned model-error redactor for model failures.
  `codex-cli` still performs only the concrete MLX HTTP request, in-process
  Codex execution, and child-process spawn.
- Telegram session bridge status planning first moved to
  `hepta-gateway::telegram_runtime`, and the pure policy portion has now moved
  further into `hepta-kernel`. The kernel owns
  `HeptaKernelTelegramSessionBridgePlan`, including prompt-material policy,
  session-key strategy, duplicate policy, cursor-commit policy, response
  delivery policy, approval policy, failure policy, and redaction flags.
  `hepta-runtime` and `hepta-gateway` keep compatibility aliases while gateway
  status building delegates to `plan_hepta_kernel_telegram_session_bridge(...)`.
- Telegram private config status DTO moved to
  `hepta-gateway::telegram_config`. Gateway now owns the stable
  `NativeTelegramConfigStatus` report shape and readiness helper, while
  `codex-cli` still owns private file/env discovery, secret-file inspection,
  and token-source resolution.
- Telegram status report DTOs moved to `hepta-gateway::telegram_status`.
  Gateway now owns the serializable report surface for plugin, receive-once,
  model-turn-plan, model-bridge, send-plan, drain-once, poll-loop, live-soak,
  production-guard, production-readiness, and live-soak observation reports.
  `codex-cli` still performs live env/file/network/process reads and fills
  those gateway-owned DTOs.
- Telegram production-readiness calculation moved to
  `hepta-gateway::telegram_status` through
  `NativeTelegramProductionReadinessInput` and
  `build_telegram_production_readiness_status(...)`. Gateway now owns the
  readiness blocker/warning classification, durable cursor/delivery evidence
  checks, freshness windows, attention budget, and redaction guard synthesis.
  `codex-cli` still supplies the current timestamp plus env-derived soak
  thresholds and live observation snapshots.
- Remaining monolith reduction: `native_telegram.rs` still owns the live status
  endpoint shell, live env/config snapshot assembly, poll/live-soak report
  builder shell, and concrete process/network call implementations; the next
  reductions should keep moving pure report assembly into `hepta-gateway` /
  `hepta-runtime` without changing Telegram ownership.

Current gates passed:

```text
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-core -p hepta-memory -p hepta-intelligence -p hepta-plugins -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-core -p hepta-memory -p hepta-intelligence -p hepta-plugins -p hepta-runtime
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime telegram_model_runner -- --nocapture
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway -p codex-cli --bin hepta
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway telegram_transport -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway telegram_cursor -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway telegram_runtime -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway telegram_policy -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
git diff --check
```

### 19:3x Progress - Telegram Session Bridge Policy Moved Into Hepta Kernel

Continued shrinking the runtime/gateway facades after child-runner policy moved
into `hepta-kernel`:

1. Added kernel-owned Telegram session bridge planning:
   `HeptaKernelTelegramSessionBridgePlan` and
   `plan_hepta_kernel_telegram_session_bridge(...)`.
2. Kept existing public gateway/runtime compatibility names:
   `NativeTelegramSessionBridgePlan` now aliases the kernel-owned contract
   through `hepta-runtime` and `hepta-gateway`.
3. Updated Telegram model bridge status construction to use the kernel planner
   while preserving status JSON redaction flags and side-effect boundaries.
4. Concrete Telegram reads/sends, cursor writes, process spawning, local MLX
   requests, launchd state, and model execution remain outside the kernel.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -- --nocapture
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime --lib telegram_model_runner -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
```

### 22:5x Progress - Telegram Poll/Soak Timing Policies Moved Into Hepta Kernel

Continued after first-candidate selection with another side-effect-free kernel
ownership slice:

1. Moved Telegram poll-loop spawn gating, poll interval clamp, receive-limit
   clamp, live-soak threshold clamps/defaults, and bounded `SystemTime` to
   unix-ms conversion into `hepta-kernel`.
2. Added runtime compatibility wrappers and root re-exports so gateway/CLI
   callers keep the `native_telegram_*` / `telegram_*` surfaces while policy
   ownership moves into the Hepta kernel contract.
3. Left actual polling, sleeps, Telegram Bot API reads/sends, cursor commits,
   delivery ledger writes, model execution, token checks, retries, and launchd
   mutation outside `hepta-kernel`.
4. Added kernel/runtime tests for the moved policies and kept gateway tests
   validating the compatibility wrappers.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime --lib telegram_model_runner -- --nocapture
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture
git diff --check
```

### 23:5x Progress - Telegram Transport Timing Policies Moved Into Hepta Kernel

Continued after the poll/soak slice with the adjacent transport timing and
retry policy:

1. Moved Telegram typing keepalive interval clamps, read retry attempt/backoff
   defaults, send minimum interval, send retry attempt/backoff defaults, and
   their upper bounds into `hepta-kernel`.
2. Added `hepta-runtime` compatibility wrappers and root re-exports so
   gateway/CLI callers keep the existing `telegram_*` function names while the
   pure policy is now kernel-owned.
3. Kept concrete Bot API HTTP calls, retry sleeps, send rate-limit sleeps,
   delivery ledger writes, cursor commits, token reads, and launchd/service
   mutation outside `hepta-kernel`.
4. Added kernel/runtime tests for the moved policies and kept
   `hepta-gateway::telegram_transport` tests validating the compatibility
   facade.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime --lib telegram_model_runner -- --nocapture
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_transport -- --nocapture
```

### 21:4x Progress - Telegram Ingress/Model-Turn Planning Moved Into Hepta Kernel

Continued after the model-invocation policy slice:

1. Moved the remaining side-effect-free Telegram ingress/model-turn planning
   DTOs and drain gate policy into `hepta-kernel`:
   `HeptaKernelTelegramGatewayGateSummary`,
   `HeptaKernelTelegramGatewayGateSummaryInput`,
   `HeptaKernelTelegramExecutionPlan`,
   `HeptaKernelTelegramIngressInspection`, and
   `HeptaKernelTelegramModelTurnPlan`.
2. Added kernel-owned drain planning helpers:
   `build_hepta_kernel_telegram_gateway_gate_summary`,
   `hepta_kernel_telegram_drain_first_missing_gate`,
   `hepta_kernel_telegram_drain_status_probe_executes_pipeline`, and
   `hepta_kernel_telegram_drain_execution_plan`.
3. Kept runtime/gateway compatibility names via `NativeTelegram*` aliases and
   `telegram_policy` re-exports, so existing status, runtime, and transport
   callers keep their public surface while ownership moves into the kernel.
4. Preserved the boundary: Telegram JSON parsing, candidate extraction from
   raw update payloads, model execution, Bot API calls, cursor commits,
   delivery ledger writes, retries, token checks, network I/O, and launchd
   mutation remain outside `hepta-kernel`.

Focused gates for this slice:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime --lib telegram_model_runner -- --nocapture
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
git diff --check
HEPTA_CODEX_PREFLIGHT_RELEASE=0 CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 scripts/hepta-codex-preflight.sh
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo build --offline --manifest-path codex-rs/Cargo.toml --release -p codex-cli --bin hepta
```

Live install and post-install gates passed:

```text
installed_sha256=2aec5b4639c5c30772f5cb3d06d5bf9d1566ac422381c619f28b8b1c6cfc9c6d
backup_dir=/Users/qianqi/.openclaw/workspace/backups/hepta-kernel-ingress-model-turn-policy-20260521-220549
scripts/hepta-codex-watchdog.sh
HEPTA_CODEX_SOAK_SAMPLES=3 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=2 scripts/hepta-codex-live-soak.sh
scripts/hepta-codex-public-ga-readiness.sh
scripts/hepta-codex-native-packaging-gate.sh
scripts/hepta-codex-browser-visual-smoke.sh
browser_smoke=/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.WptK29
```

### 22:3x Progress - Telegram Model-Turn Aggregation Moved Into Hepta Kernel

Continued after the ingress/model-turn DTO slice with a smaller ownership
cut:

1. Added `hepta_kernel_telegram_model_turn_plan_from_candidates(...)` in
   `hepta-kernel`, so candidate counting, kind classification, reply-target
   aggregation, and raw-identifier exposure flags are now kernel-owned policy.
2. Kept `hepta-runtime::telegram_model_runner` as the compatibility facade via
   `native_telegram_model_turn_plan_from_candidates(...)`.
3. Simplified `hepta-gateway::telegram_policy::plan_model_turn_for_updates` so
   the gateway only parses raw Telegram JSON into bounded candidate material
   and then delegates model-turn aggregation to the kernel-owned policy.
4. Preserved the boundary: raw Telegram update parsing, prompt material,
   actual model execution, Bot API calls, cursor commits, delivery ledger
   writes, retries, token checks, network I/O, and launchd mutation remain
   outside `hepta-kernel`.

Focused gates for this slice:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime --lib telegram_model_runner -- --nocapture
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
git diff --check
HEPTA_CODEX_PREFLIGHT_RELEASE=0 CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 scripts/hepta-codex-preflight.sh
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo build --offline --manifest-path codex-rs/Cargo.toml --release -p codex-cli --bin hepta
```

Live install and post-install gates passed:

```text
installed_sha256=f59b51d5fea429af6ab0a32aef0c22f609cf98c5dfb7be3f1f44505d264fae6c
backup_dir=/Users/qianqi/.openclaw/workspace/backups/hepta-kernel-model-turn-aggregation-20260521-222927
launchd=ai.hepta.gateway running pid=86594 runs=11
curl -fsS http://127.0.0.1:7373/health
curl -fsS http://127.0.0.1:7373/api/telegram-model-bridge
curl -fsS http://127.0.0.1:7373/api/gateway-replacement-readiness
scripts/hepta-codex-watchdog.sh
HEPTA_CODEX_SOAK_SAMPLES=3 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=2 scripts/hepta-codex-live-soak.sh
scripts/hepta-codex-public-ga-readiness.sh
scripts/hepta-codex-native-packaging-gate.sh
scripts/hepta-codex-browser-visual-smoke.sh
browser_smoke=/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.ZTzGsk
```

### 22:5x Progress - Telegram First-Candidate Selection Moved Into Hepta Kernel

Continued after the model-turn aggregation slice:

1. Added
   `hepta_kernel_telegram_first_model_candidate_with_duplicate_decision(...)`
   in `hepta-kernel`, so the first model-eligible candidate selection,
   missing-update-id handling, duplicate decision binding, and invocation
   request construction are kernel-owned policy.
2. Kept `hepta-runtime::telegram_model_runner` as the compatibility facade via
   `native_telegram_first_model_candidate_with_duplicate_decision(...)`.
3. Simplified `hepta-gateway::telegram_policy::first_model_candidate_with_duplicate_decision`
   so the gateway only parses the first 20 raw Telegram updates into bounded
   candidate material and then delegates candidate selection / duplicate policy
   to the kernel.
4. Preserved the boundary: raw Telegram JSON parsing, actual model execution,
   Bot API calls, cursor commits, delivery ledger writes, retries, token
   checks, network I/O, and launchd mutation remain outside `hepta-kernel`.

Focused gates for this slice:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime --lib telegram_model_runner -- --nocapture
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
git diff --check
HEPTA_CODEX_PREFLIGHT_RELEASE=0 CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 scripts/hepta-codex-preflight.sh
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo build --offline --manifest-path codex-rs/Cargo.toml --release -p codex-cli --bin hepta
```

Live install and post-install gates passed:

```text
installed_sha256=a468c7df0f682c6de60eddeb12d111f8286a48e1b28fc56d47a64987c1c6d427
backup_dir=/Users/qianqi/.openclaw/workspace/backups/hepta-kernel-first-candidate-selection-20260521-225511
launchd=ai.hepta.gateway running pid=90782 runs=12
curl -fsS http://127.0.0.1:7373/health
curl -fsS http://127.0.0.1:7373/api/telegram-model-bridge
curl -fsS http://127.0.0.1:7373/api/gateway-replacement-readiness
scripts/hepta-codex-watchdog.sh
HEPTA_CODEX_SOAK_SAMPLES=3 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=2 scripts/hepta-codex-live-soak.sh
scripts/hepta-codex-public-ga-readiness.sh
scripts/hepta-codex-native-packaging-gate.sh
scripts/hepta-codex-browser-visual-smoke.sh
browser_smoke=/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.2NKFLC
```

### 20:0x Progress - Telegram Model Failure Fallback Policy Moved Into Hepta Kernel

Continued the same kernel shrink path with another small side-effect-free slice:

1. Added kernel-owned default failure fallback message:
   `HEPTA_KERNEL_TELEGRAM_MODEL_FAILURE_FALLBACK_MESSAGE`.
2. Added kernel-owned fallback safety rule:
   `hepta_kernel_telegram_model_failure_fallback_allowed(...)`.
3. Kept runtime/gateway compatibility names:
   `native_telegram_model_failure_fallback_message()` and
   `NATIVE_TELEGRAM_MODEL_FAILURE_FALLBACK_MESSAGE` still exist at the old
   public boundary, but now delegate to the kernel contract.
4. Gateway drain runtime now asks the kernel policy whether fallback delivery is
   safe, while the actual send/cursor side effects remain in gateway transport
   code and only execute under their existing gates.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime --lib telegram_model_runner -- --nocapture
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
git diff --check
```

### 20:2x Progress - Telegram Drain Final Status Policy Moved Into Hepta Kernel

Continued with another pure policy extraction:

1. Added kernel-owned drain final status planning:
   `HeptaKernelTelegramDrainFinalStatusPlan` and
   `hepta_kernel_telegram_drain_final_status(...)`.
2. Kept runtime compatibility via `NativeTelegramDrainFinalStatusPlan` and
   `native_telegram_drain_final_status(...)`.
3. Gateway `finalize_telegram_drain_pipeline_status(...)` now delegates the
   delivered/attention/previous status choice and local-process marker to the
   kernel, while retaining the concrete pipeline outcome assembly in gateway.
4. Send execution, cursor writes, model execution, Telegram API calls, and
   process waits remain outside `hepta-kernel`.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime --lib telegram_model_runner -- --nocapture
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
git diff --check
```

### 20:4x Progress - Telegram Send Delivery Request Policy Moved Into Hepta Kernel

Continued with the next pure Telegram delivery-planning extraction:

1. Added kernel-owned send delivery request/report planning:
   `HeptaKernelTelegramSendRequestPlan` and
   `HeptaKernelTelegramSendExecutionReport`.
2. Kept runtime/gateway compatibility via
   `NativeTelegramSendRequestPlan` and
   `NativeTelegramSendExecutionReport`.
3. `hepta-gateway::telegram_policy` now re-exports the runtime aliases instead
   of owning the send request/report structs and constructors.
4. Actual Bot API calls, delivery ledger writes, cursor commits, token
   validation, retry/backoff loops, and network side effects remain in gateway
   transport code behind the existing gates.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime --lib telegram_model_runner -- --nocapture
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
git diff --check
```

Observed test totals:

- `hepta-core`: 200 passed.
- `hepta-memory`: 57 passed.
- `hepta-intelligence`: 84 passed.
- `hepta-plugins`: 42 passed.
- `hepta-runtime`: 352 passed plus its integration/doc-test surfaces.
- `hepta-runtime` Telegram runner targeted regression: 14 passed.
- `hepta-gateway`: 95 lib tests and 18 integration tests passed.
- `hepta-gateway` Telegram transport targeted regression: 17 passed.
- `hepta-gateway` Telegram cursor targeted regression: 7 passed.
- `hepta-gateway` Telegram runtime targeted regression: 4 passed.
- `hepta-gateway` Telegram policy targeted regression: 11 passed.
- `codex-cli` native Telegram targeted regression: 50 passed.
- `codex-cli` native POST targeted regression: 17 passed.
- `codex-cli` native gateway targeted regression: 55 passed.

## Merge Lanes

### Lane A - Workspace Crate Transplant

Goal: make old Hepta architecture compile inside `hepta-codex` as first-class
workspace crates.

Actions:

1. Copy `Hepta/crates/hepta-core` to `hepta-codex/codex-rs/hepta-core`.
2. Copy `Hepta/crates/hepta-memory` to `hepta-codex/codex-rs/hepta-memory`.
3. Copy `Hepta/crates/hepta-intelligence` to `hepta-codex/codex-rs/hepta-intelligence`.
4. Copy `Hepta/crates/hepta-plugins` to `hepta-codex/codex-rs/hepta-plugins`.
5. Add all four to `codex-rs/Cargo.toml` workspace members and workspace dependencies.
6. Normalize package metadata to the Codex workspace: version, edition, license.
7. Keep APIs intact; do not rewrite semantics during transplant.

Validation:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-core -p hepta-memory -p hepta-intelligence -p hepta-plugins
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-core -p hepta-memory -p hepta-intelligence -p hepta-plugins
```

Exit criterion:

- The old Hepta data, memory, and intelligence crates compile in `hepta-codex`
  without depending on the old `Hepta` repo.

### Lane B - Runtime Kernel Transplant

Goal: bring `RuntimeKernel` into `hepta-codex` as the Hepta runtime kernel,
not as a shadow standalone application.

Actions:

1. Copy `Hepta/crates/hepta-runtime` to `hepta-codex/codex-rs/hepta-runtime`.
2. Add `hepta-runtime` to the workspace.
3. Keep `RuntimeKernel` internal state and reports intact.
4. Replace old standalone assumptions with explicit host inputs from Codex:
   thread id, session id, model id, workspace root, and state paths.
5. Keep live model invocation, live memory attachment, and external channel
   sends gated off until later cutover lanes.

Validation:

```text
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime intelligence_eval neuron_lifecycle memory_context model_provider_router
```

Exit criterion:

- `hepta-runtime` compiles in the fork and can produce local reports from sample
  or fixture state without invoking providers or mutating production state.

### Lane C - Gateway Crate Transplant And Monolith Reduction

Goal: move gateway architecture back into a library layer and stop growing the
CLI monolith.

Actions:

1. Copy `Hepta/crates/hepta-gateway` to `hepta-codex/codex-rs/hepta-gateway`.
2. Add `hepta-gateway` to the workspace.
3. Move route contracts, dispatch policy, delivery ledger, transport decisions,
   and native POST planning out of `codex-rs/cli/src/native_gateway.rs` into
   `hepta-gateway` and `hepta-runtime` modules.
4. Keep `codex-rs/cli` responsible only for:
   - `--serve-ui` argument parsing
   - TCP bind / HTTP request shell
   - calling `hepta-gateway` route handlers
   - process startup and shutdown
5. Keep Telegram owner policy unchanged: old OpenClaw owner unless explicitly
   told otherwise.

Validation:

```text
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway -p codex-cli --bin hepta
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway
```

Exit criterion:

- `native_gateway.rs` stops being the architecture host. It becomes a shell over
  first-class Hepta gateway/runtime crates.

### Lane D - Intelligence Real Integration

Goal: make old Hepta intelligence a real part of `hepta-codex`, not a status
surface.

Actions:

1. Expose `hepta-intelligence` reports through `hepta-runtime` first:
   - memory intelligence readiness
   - intelligence eval
   - phase-2 gate
   - neuron lifecycle
   - neuron activation
   - intuition calibration
2. Add native HTTP endpoints backed by real `hepta-runtime` calls:
   - `/api/memory-intelligence`
   - `/api/intelligence-eval`
   - `/api/intelligence-phase2`
   - `/api/neuron-lifecycle`
   - `/api/memory-neuron-compression-v2`
3. Add Codex state readers that transform Codex thread/state records into
   Hepta session, transcript, memory, topic, and neuron inputs.
4. Keep the first pass read-only/sample-run.
5. Only after read-only parity is green, attach the memory intelligence output
   to Codex model turns behind explicit live gates.

Validation:

```text
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-intelligence
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime intelligence_eval neuron_lifecycle memory_context
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway intelligence
```

Exit criterion:

- Intelligence endpoints are backed by real Hepta crates and fixture/Codex
  state, not by prose status or placeholder compatibility reports.

### Lane E - CLI Command Surface Migration

Goal: recover old Hepta slash command semantics without copying the old CLI
parser wholesale into Codex CLI.

Actions:

1. Identify command groups from old `Hepta/crates/hepta-cli/src/commands.rs`.
2. Promote command groups by architecture domain:
   - intelligence and memory
   - runtime event/control
   - gateway/operator
   - task board/workers
   - providers/plugins/tools
3. For each command group, implement the command by calling the new
   `hepta-*` crates inside `hepta-codex`.
4. Avoid importing the old `commands.rs` as one giant parser.

Validation:

```text
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta command_registry intelligence runtime_event gateway
```

Exit criterion:

- User-facing Hepta commands exist in the Codex fork and resolve to the merged
  architecture crates.

### Lane F - Runtime Cutover

Goal: switch production behavior only after architecture parity exists.

Actions:

1. Keep old OpenClaw Telegram owner unchanged.
2. Keep Hepta Telegram poll loop gated until explicitly approved.
3. Keep native POST live mutation gates off by default.
4. Run installed-binary read-only smoke for merged routes.
5. Run one handler at a time gray evidence only after route and rollback evidence
   are green.
6. Only then consider replacing legacy OpenClaw/old Hepta ownership boundaries.

Validation:

```text
/health
/api/control-ui-route-parity
/api/operator-security
/api/gateway-replacement-readiness
/api/memory-intelligence
/api/intelligence-eval
/api/native-post-gray-release-evidence
/api/telegram-owner-handoff
```

Exit criterion:

- Architecture parity comes before ownership cutover.

## Explicit Non-Goals

- Do not create another adapter-first layer.
- Do not keep adding real architecture into `codex-rs/cli/src/native_gateway.rs`.
- Do not disable old OpenClaw Telegram again without explicit instruction.
- Do not port old `hepta-cli` wholesale as a giant parser blob.
- Do not activate live memory attachment, live Telegram polling, or POST
  mutations during crate transplant.

## Immediate Next Patch

Continue Lane C monolith reduction.

### 20:xx Progress - Telegram Status Builders Moved

Completed in the current Lane C slice:

1. Moved plugin/model-turn/model-bridge/send-plan/poll-loop/live-soak status
   builders from `codex-rs/cli/src/native_telegram.rs` into
   `hepta-gateway::telegram_status`.
2. Moved drain-once final report assembly into
   `hepta-gateway::telegram_status` via `NativeTelegramDrainOnceStatusInput`
   and `build_telegram_drain_once_status`.
3. Moved receive-once preflight/status assembly into
   `hepta-gateway::telegram_status` via
   `NativeTelegramReceiveOncePreflightInput`,
   `NativeTelegramReceiveOnceStatusInput`, and
   `build_telegram_receive_once_status`.
4. Moved production-guard report assembly into
   `hepta-gateway::telegram_status` via
   `NativeTelegramProductionGuardStatusInput` and
   `build_telegram_production_guard_status`.
5. Moved live-soak observation state/update/report helper into
   `hepta-gateway::telegram_status` via
   `NativeTelegramLiveSoakObservationState`; CLI keeps only the process-local
   `OnceLock<Mutex<...>>` storage and timestamp injection.
6. Kept the CLI as the shell for env/config observations, live network calls,
   Codex child-process/in-process execution, and live delivery gates.
7. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture
```

### 21:4x Progress - Drain Preflight And API Result Planning Moved

Completed in the follow-up Lane C slice:

1. Moved drain-once preflight/default status planning into
   `hepta-gateway::telegram_status` via
   `NativeTelegramDrainOncePreflightInput`,
   `NativeTelegramDrainOncePreflightPlan`, and
   `plan_telegram_drain_once_preflight`.
2. Moved drain-once Bot API result/error planning into
   `hepta-gateway::telegram_status` via
   `NativeTelegramDrainOnceApiResultInput`,
   `NativeTelegramDrainOnceApiResultPlan`, and
   `plan_telegram_drain_once_api_result`.
3. The CLI still owns the concrete shell: token/config reads, cursor file reads,
   real `reqwest` calls, Codex in-process/child-process execution, typing
   keepalive, send rate limit, delivery ledger writes, and cursor commits.
4. `native_telegram.rs` dropped to roughly 3175 lines; gateway status planning
   grew to roughly 2167 lines with targeted tests covering disabled/gated/armed
   preflight, `ok=true`, `ok=false`, and `409 busy` getUpdates outcomes.
5. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway drain_once -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture
git diff --check
```

The next code patch should continue shrinking `native_telegram.rs` by moving
pure config/token observation classification into `hepta-gateway` while keeping
secret reads, env reads, file IO, network calls, process execution, and delivery
side effects in the CLI shell until an explicit ownership cutover is approved.

### 21:5x Progress - Telegram Config Observation Planning Moved

Completed in the follow-up Lane C slice:

1. Moved pure config observation/status derivation into
   `hepta-gateway::telegram_config` via
   `NativeTelegramConfigStatusInput` and
   `build_native_telegram_config_status`.
2. Moved Telegram binding id normalization into
   `hepta-gateway::telegram_config::normalize_telegram_binding_id`.
3. Added gateway-owned constructors for missing/error config status so the CLI
   no longer manually assembles those serializable report shapes.
4. CLI still owns all sensitive and side-effecting operations: env lookup,
   private config discovery, file reads, secret-file reads, token material
   handling, token shape validation input, network calls, and process execution.
5. `native_telegram.rs` dropped to roughly 3129 lines; gateway config planning
   now has focused unit coverage for binding readiness, missing binding scope,
   and prefix normalization.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway -p codex-cli --bin hepta
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway telegram_config -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta telegram_config -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
git diff --check
```

The next code patch should continue Lane C by extracting the remaining pure
environment clamp/default policy helpers for Telegram retry/timeout/soak
settings into gateway/runtime-owned planners, while keeping actual env reads in
the CLI shell.

### 22:4x Progress - Telegram Env Policy Helpers Moved

Completed in the follow-up Lane C slice:

1. Moved pure Telegram retry/typing/send clamp/default policy helpers into
   `hepta-gateway::telegram_transport`.
   - `telegram_typing_keepalive_interval_policy`
   - `telegram_read_max_attempts_policy`
   - `telegram_read_retry_backoff_policy`
   - `telegram_send_min_interval_policy`
   - `telegram_send_max_attempts_policy`
   - `telegram_send_retry_backoff_policy`
2. Moved pure Telegram live-soak clamp/default policy helpers into
   `hepta-gateway::telegram_status`.
   - `telegram_soak_min_poll_iterations_policy`
   - `telegram_soak_max_attention_count_policy`
   - `telegram_soak_max_observed_age_ms_policy`
3. `codex-cli` still owns only the shell side of this policy: `env::var`
   reads, env truthiness, time source, config/secret/file IO, network calls,
   model process execution, delivery ledger writes, and cursor commits.
4. `native_telegram.rs` dropped to `3078` lines; gateway status/transport are
   now `2222` / `1185` lines respectively.
5. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway env_policy -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway soak_env_policy -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture
git diff --check
```

The next code patch should continue Lane C by extracting the remaining small
pure Telegram runtime policies and status wrappers still sitting in
`native_telegram.rs`, while leaving all env/file/network/process side effects
in the CLI shell until explicit cutover approval.

### 22:5x Progress - Telegram Runtime Policy Wrappers Moved

Completed in the follow-up Lane C slice:

1. Moved Telegram gateway gate summary construction into
   `hepta-gateway::telegram_policy` via
   `NativeTelegramGatewayGateSummaryInput` and
   `build_telegram_gateway_gate_summary`.
2. Moved config-to-transport-plan mapping into
   `hepta-gateway::telegram_transport` via
   `telegram_transport_plan_for_config_status`.
3. Moved production-guard raw env clamp/default status planning into
   `hepta-gateway::telegram_status` via
   `NativeTelegramProductionGuardPolicyInput` and
   `build_telegram_production_guard_status_from_policy`.
4. Moved the Telegram model-failure fallback message into
   `hepta-gateway::telegram_runtime`; the CLI now passes the runtime-owned
   static message into the drain pipeline.
5. Removed the remaining local receive-once base-status wrapper from
   `native_telegram.rs`; the CLI now calls the gateway-owned
   `NativeTelegramReceiveOnceStatus::base`.
6. `native_telegram.rs` is now `3032` lines; the CLI still owns env reads,
   config/secret/file IO, real Bot API calls, model process execution, delivery
   ledger writes, and cursor commit timing.
7. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture
git diff --check
```

The next code patch should continue Lane C by extracting the remaining
receive/drain side-effect shell result shaping out of `native_telegram.rs`,
while keeping token loading, `reqwest`, process execution, cursor writes, and
delivery ledger writes owned by the CLI shell.

### 23:0x Progress - Drain Pipeline Final Status Shaping Moved

Completed in the follow-up Lane C slice:

1. Moved post-drain pipeline final status shaping into
   `hepta-gateway::telegram_runtime` via
   `NativeTelegramDrainPipelineFinalStatus` and
   `finalize_telegram_drain_pipeline_status`.
2. Gateway-runtime now owns the pure rule that:
   - marks child-process runner reports as locally spawned only when the
     selected runner plan is process-backed and the runner was invoked;
   - converts successful send delivery into overall drain status `drained`;
   - prefers send attention errors over model attention errors;
   - otherwise preserves the pre-pipeline status/error chosen by the drain
     fetch plan.
3. `native_telegram.rs` is now `3026` lines; the CLI still owns token loading,
   real Bot API calls, model execution closures, delivery ledger write timing,
   and cursor commit timing.
4. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway telegram_runtime -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture
git diff --check
```

The next code patch should continue Lane C by extracting remaining
receive/drain preflight shell helpers, or start shifting Telegram config/token
source resolution into a pure planner that still accepts file/env observations
from the CLI shell.

### 23:1x Progress - Telegram Token Observation Planning Moved

Completed in the follow-up Lane C slice:

1. Moved Telegram token source observation into
   `hepta-gateway::telegram_config` via
   `NativeTelegramTokenObservationInput`,
   `NativeTelegramTokenObservation`, and
   `resolve_native_telegram_token_observation`.
2. Gateway-config now owns the pure priority and classification rule for
   env-token vs secret-file-token vs inline-token observations, including the
   distinction between missing token material and a present-but-missing secret
   file reference.
3. `native_telegram.rs` still owns actual env reads, config reads, secret-file
   reads, token values, and token shape checks. It passes only boolean
   observations into the gateway planner; no token string crosses into the
   planner.
4. `native_telegram.rs` is now `3025` lines. `telegram_config.rs` is now `342`
   lines.
5. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway telegram_config -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture
git diff --check
find . -maxdepth 2 -type d -name artifacts -print
```

The next code patch should continue Lane C by extracting the remaining small
Telegram config/status pure mappers from `native_telegram.rs`, while keeping
actual secret/env/file/network/process side effects in the CLI shell.

### 23:2x Progress - Telegram Env And Secret Path Parsers Moved

Completed in the follow-up Lane C slice:

1. Moved pure Telegram env value parsing into
   `hepta-gateway::telegram_config` via
   `parse_telegram_env_truthy_value` and `parse_telegram_env_u64_value`.
2. Moved pure Telegram secret provider path resolution into
   `hepta-gateway::telegram_config` via
   `resolve_telegram_secret_provider_path`.
3. `native_telegram.rs` still owns actual `env::var`, config-file reads,
   secret-file reads, token values, file-mode checks, network calls, process
   execution, cursor writes, and delivery ledger writes.
4. `native_telegram.rs` is now `3007` lines. `telegram_config.rs` is now `416`
   lines.
5. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway telegram_config -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture
git diff --check
find . -maxdepth 2 -type d -name artifacts -print
```

The next code patch should continue Lane C by extracting remaining pure
Telegram status helpers that do not perform file/env/network/process side
effects, or by shrinking CLI-only test scaffolding around already migrated
gateway APIs.

### 23:3x Progress - Poll Loop Policy Helpers Moved

Completed in the follow-up Lane C slice:

1. Moved pure Telegram poll-loop spawn gating into
   `hepta-gateway::telegram_status` via `telegram_poll_loop_should_spawn`.
2. Moved poll-loop interval clamp policy into
   `hepta-gateway::telegram_status` via
   `telegram_poll_loop_interval_ms_policy`, with exported min/max constants.
3. `native_telegram.rs` still owns actual thread spawning, infinite poll loop
   execution, drain-once invocation, stderr logging, and sleep timing.
4. `native_telegram.rs` is now `3009` lines. `telegram_status.rs` is now
   `2354` lines.
5. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway poll_loop -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture
git diff --check
find . -maxdepth 2 -type d -name artifacts -print
```

The next code patch should continue Lane C by extracting remaining pure
status/time helpers where doing so does not hide side effects, or by moving
Telegram config metadata extraction that does not expose token values.

### 23:4x Progress - Telegram Time Conversion Helper Moved

Completed in the follow-up Lane C slice:

1. Moved bounded `SystemTime -> unix_ms` conversion into
   `hepta-gateway::telegram_status` via `telegram_system_time_unix_ms`.
2. `native_telegram.rs` still owns reading `SystemTime::now()`; gateway owns
   only the overflow-safe and pre-epoch-safe conversion.
3. `native_telegram.rs` is now `3002` lines. `telegram_status.rs` is now
   `2374` lines.
4. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway system_time -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture
git diff --check
find . -maxdepth 2 -type d -name artifacts -print
```

The next code patch should continue Lane C by moving Telegram config metadata
extraction that does not expose token values, or by shrinking remaining CLI
status wrappers whose inputs are already side-effect-free.

### 00:5x-01:0x Progress - Telegram Config Metadata And Receive Error Builders Moved

Completed in the next Lane C continuation:

1. Moved non-secret Telegram config metadata extraction into
   `hepta-gateway::telegram_config` via
   `NativeTelegramConfigMetadata` and
   `extract_native_telegram_config_metadata`.
2. The gateway metadata extractor now owns enabled/dm/group policy normalization,
   binding-count derivation, group-count derivation, secret provider path
   resolution, secret id presence, and inline-token presence. It does not return
   token values.
3. `native_telegram.rs` still owns actual env token reads, secret-file token
   reads, inline token string reads, token-shape checks, file-mode checks,
   network calls, process execution, cursor writes, and delivery ledger writes.
4. `load_effective_telegram_token` now reuses the same gateway metadata for the
   secret provider path while keeping token values local to the CLI shell.
5. Moved receive-once token/cursor error report shaping into
   `hepta-gateway::telegram_status` via
   `NativeTelegramReceiveOnceErrorInput` and
   `build_telegram_receive_once_error_status`.
6. Removed one CLI-only send-request test wrapper; tests now call the
   gateway-owned `NativeTelegramSendRequestPlan::from_model_output` directly.
7. `native_telegram.rs` is now `2933` lines. `telegram_config.rs` is now `569`
   lines. `telegram_status.rs` is now `2437` lines.
8. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture
git diff --check
find . -maxdepth 2 -type d -name artifacts -print
```

The next code patch should continue Lane C by extracting the remaining small
side-effect-free receive/drain status shapers or by shrinking CLI-only test
scaffolding around already migrated gateway APIs.

### 01:5x Progress - Drain Shell Readiness Planner Moved

Completed in the follow-up Lane C slice:

1. Moved drain-once shell readiness classification into
   `hepta-gateway::telegram_status` via
   `NativeTelegramDrainOnceShellReadinessInput`,
   `NativeTelegramDrainOnceShellReadinessPlan`, and
   `plan_telegram_drain_once_shell_readiness`.
2. Gateway-status now owns the pure cursor/config/token error priority,
   including default cursor-error text, config-not-ready text, and token-like
   redaction for token-load failures.
3. `native_telegram.rs` still owns the side-effect shell: cursor-file reads,
   config/env/secret-file reads, token material, actual Bot API `getUpdates`,
   model execution, sends, ledger writes, and cursor commits.
4. `native_telegram.rs` is now `2945` lines. `telegram_status.rs` is now
   `2561` lines.
5. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway shell_readiness -- --nocapture
cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib shell_readiness -- --nocapture
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
git diff --check
find . -maxdepth 2 -type d -name artifacts -print
```

Note: broader `codex-cli` check attempts were intentionally stopped after the
Rust compiler made no progress in metadata compilation for `codex_cli`; no
compiler diagnostic was emitted. Continue with bounded targeted gates until the
compiler stall is isolated.

The next code patch should continue Lane C by shrinking duplicated
CLI-only Telegram tests now covered in `hepta-gateway`, or by extracting another
small receive/drain status wrapper whose inputs are already side-effect-free.

### 02:2x Progress - Duplicate CLI Telegram Tests Removed

Completed in the follow-up Lane C shrink pass:

1. Removed `22` duplicate Telegram CLI tests from
   `codex-rs/cli/src/native_telegram.rs` after confirming their behaviors are
   already covered in `hepta-gateway` modules.
2. The removed wrappers covered candidate-material privacy, cursor offset
   parsing/writes, getUpdates/send request shaping, send/model execution gates,
   duplicate suppression, and transient-error classifiers.
3. Kept the remaining CLI tests that still exercise CLI-owned glue, production
   readiness wiring, duplicate decisions, and execution shell behavior.
4. Pruned now-unused test imports from `native_telegram.rs`.
5. `native_telegram.rs` is now `2307` lines after import pruning.
   `telegram_status.rs` remains
   `2561` lines.
6. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
git diff --check
find . -maxdepth 2 -type d -name artifacts -print
```

Note: `codex-cli` native Telegram test execution was attempted and intentionally
stopped after rustc again made no progress in the `codex_cli` metadata/link
phase; no compiler diagnostic was emitted. Continue with bounded gateway gates
and small mechanical CLI cuts until that build stall is isolated.

The next code patch should either isolate the `codex-cli` build stall or
continue shrinking CLI-only Telegram scaffolding around already migrated
gateway APIs.

### 02:4x Progress - Gateway/Runtime Duplicate Test Wrappers Removed

Completed another Lane C shrink pass:

1. Removed `8` more CLI-only Telegram tests whose assertions are now covered in
   `hepta-gateway` or `hepta-runtime`.
2. Removed wrappers for cursor/duplicate policy, model-failure fallback text,
   delivered drain pipeline privacy, MLX runner selection, OpenAI-compatible
   response text extraction, and child-runner argument shape.
3. Kept tests that still exercise CLI-owned shell behavior, including config
   secret-file reads, status wrapper gating, send failure handling, receive/drain
   live gates, and production-readiness composition.
4. Pruned the now-unused duplicate-policy test imports from `native_telegram.rs`.
5. `native_telegram.rs` is now `2097` lines. `telegram_status.rs` remains
   `2561` lines.
6. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime --lib telegram_model_runner -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
git diff --check
find . -maxdepth 2 -type d -name artifacts -print
```

The next code patch should continue reducing the remaining CLI Telegram surface
from `2097` lines, preferably by extracting or deleting another bounded wrapper
whose behavior is already covered in gateway/runtime crates.

### 02:5x Progress - Production Readiness Tests Moved To Gateway

Completed another Lane C migration slice:

1. Added production-readiness regression coverage directly to
   `hepta-gateway::telegram_status`.
2. The gateway tests now cover minimum soak observations, clean guarded soak
   readiness, no-message-drained warnings, attention-budget failures, stale soak
   observations, missing durable delivery evidence after send, and stale
   delivery-ledger evidence after send.
3. Removed the corresponding `7` CLI production-readiness tests and `5` CLI test
   helper builders from `native_telegram.rs`.
4. Kept CLI tests that still exercise shell-owned behavior: config secret-file
   reads, status wrapper gating, send success/failure cursor behavior,
   drain-gate behavior, and receive-live-gate behavior.
5. `native_telegram.rs` is now `1706` lines. `telegram_status.rs` is now
   `2980` lines.
6. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib production_readiness -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
git diff --check
find . -maxdepth 2 -type d -name artifacts -print
```

The next code patch should continue from the remaining `13` CLI Telegram tests,
prioritizing any wrappers that can move cleanly to gateway/runtime without
changing live Telegram behavior.

### 03:0x Progress - Status/Policy Builder Tests Moved To Gateway

Completed another Lane C shrink pass:

1. Added gateway-side coverage for plugin status, model-bridge status, and send
   plan status builders.
2. Expanded the gateway policy send-request test to cover private response
   serialization guards, missing reply targets, missing cursor offsets, and gate
   preservation.
3. Removed `6` corresponding CLI test wrappers from `native_telegram.rs`:
   plugin supervisor report, ingress parser report, model bridge gating, send
   plan gating, and the two send request builder cases.
4. `native_telegram.rs` is now `1481` lines. `telegram_status.rs` is now
   `3109` lines. `telegram_policy.rs` is now `1454` lines.
5. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_status -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib send_request_and_execution_report -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
git diff --check
find . -maxdepth 2 -type d -name artifacts -print
```

The next code patch should focus on the remaining `7` CLI Telegram tests. Most
of those still exercise CLI-owned shell behavior, so the remaining reductions
need to be more deliberate than the pure duplicate-test removals above.

### 03:1x Progress - Send/Drain Execution Tests Moved To Gateway

Completed another Lane C shell-boundary pass:

1. Strengthened `hepta-gateway::telegram_transport` send execution tests so they
   cover ack delivery, token/chat/reply forwarding, report redaction, cursor
   commit-after-ack, delivery-ledger evidence, and send failure without cursor
   commit.
2. Added `hepta-gateway::telegram_runtime` coverage for drain pipeline model
   gate enforcement before model runner or send execution.
3. Removed `3` corresponding CLI wrappers from `native_telegram.rs`: send
   success cursor commit, send failure cursor non-commit, and drain pipeline
   model-gate enforcement.
4. `native_telegram.rs` is now `1292` lines. `telegram_transport.rs` is now
   `1273` lines. `telegram_runtime.rs` is now `710` lines.
5. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib send_execution -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib drain_pipeline -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
git diff --check
find . -maxdepth 2 -type d -name artifacts -print
```

Note: the reduced `codex-cli --bin hepta native_telegram` gate was retried and
still stalled in `rustc` while compiling/linking `codex_cli` metadata with no
diagnostic output. The process was stopped after several minutes; no live
Telegram behavior was exercised.

The remaining CLI Telegram tests are now down to the narrow shell-boundary set:
secret-file config read, drain-once wrapper gating, and receive-once live-read
gate behavior.

### 03:2x Progress - Receive Limit Policy Moved To Gateway

Completed a small follow-up extraction:

1. Added `telegram_receive_limit_policy` to `hepta-gateway::telegram_status`
   and exported it through `hepta-gateway`.
2. Moved the receive-once `limit` clamp (`1..=20`) out of
   `native_telegram.rs`; the CLI shell now delegates the pure policy to gateway
   before doing config/token/cursor/Bot API work.
3. Added gateway test assertions for `0 -> 1`, in-range preservation, and
   oversized values clamping to `20`.
4. `native_telegram.rs` is now `1293` lines. `telegram_status.rs` is now
   `3116` lines.
5. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib poll_loop_policies -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
git diff --check
find . -maxdepth 2 -type d -name artifacts -print
```

### 03:3x Progress - Receive Shell Readiness Planner Moved To Gateway

Completed a narrow follow-up extraction:

1. Added `NativeTelegramReceiveOnceShellReadinessInput`,
   `NativeTelegramReceiveOnceShellReadinessPlan`, and
   `plan_telegram_receive_once_shell_readiness` in
   `hepta-gateway::telegram_status`.
2. Moved receive-once token/cursor shell-readiness decision shaping into
   gateway-status while preserving CLI ownership of actual token loading,
   cursor file reads, and Bot API `getUpdates`.
3. Added gateway tests for token-error redaction, cursor-parse blocking, and
   ready-shell Bot API allowance.
4. `native_telegram.rs` reached `1309` lines before the dead helper cleanup.
   `telegram_status.rs` is now `3212` lines.
5. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib receive_once_shell -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway
git diff --check
find . -maxdepth 2 -type d -name artifacts -print
```

### 03:4x Progress - CLI Telegram Gates Recovered

Follow-up after the receive shell-readiness planner:

1. Confirmed the earlier apparent `codex_cli` stall was not permanent. A
   focused `codex-cli --lib` check completed, and the later `--bin hepta` check
   also completed once the cache warmed.
2. Re-ran the CLI Telegram gates after removing an obsolete
   `telegram_delivery_ledger_status_from_path` test helper from
   `native_telegram.rs`.
3. `native_telegram.rs` is now `1304` lines. The remaining CLI Telegram tests
   are the intended shell-boundary set.
4. Preserved ownership boundaries: no Telegram owner handoff, no live POST
   activation, no live poll/send activation, no deploy/install, no commit.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --lib
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture
git diff --check
find . -maxdepth 2 -type d -name artifacts -print
```

The next patch can safely continue small gateway/CLI cuts, but the current
slice has both gateway and CLI targeted gates green again.

### 17:5x Progress - Telegram Runner Policy Moved Into Hepta Kernel

Continued the kernel-fusion line after the Hepta-owned Telegram runner gate was
already live:

1. Added kernel-owned Telegram runner policy in `hepta-kernel`:
   `HeptaKernelTelegramRunnerPlan`,
   `HeptaKernelTelegramRunnerInvocationOutcome`,
   `select_hepta_kernel_telegram_runner`, error classification/redaction, MLX
   model-ref parsing, and bounded MLX token policy.
2. Kept `hepta-runtime::telegram_model_runner` as a compatibility facade:
   `NativeTelegramModelRunnerPlan` and
   `NativeTelegramModelRunnerInvocationOutcome` now alias the kernel-owned
   types, and the runtime selectors/invocation wrappers delegate to
   `hepta-kernel`.
3. Preserved live behavior and side-effect boundaries: MLX local network
   execution, in-process execution, child process spawning, Telegram send/poll,
   cursor writes, and launchd state remain outside the kernel policy crate.
4. This is another ownership inversion slice: runner selection and invocation
   outcome policy now live under the Hepta kernel contract while Codex remains
   an internal execution engine.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime --lib telegram_model_runner -- --nocapture
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
git diff --check
```

### 18:1x Progress - MLX Request/Response Shaping Moved Into Hepta Kernel

Continued the same ownership-inversion path:

1. Added `serde_json` to `hepta-kernel` and moved pure MLX/OpenAI-compatible
   model-runner shaping into the kernel:
   `hepta_kernel_mlx_chat_completion_body` and
   `extract_hepta_kernel_openai_chat_completion_text`.
2. Kept `hepta-runtime::telegram_model_runner` compatibility functions with
   the existing names while delegating request-body and response-text policy to
   `hepta-kernel`.
3. Preserved side-effect boundaries: the kernel shapes/parses JSON only; local
   network calls, process spawning, Telegram send/poll, and launchd/runtime
   state remain outside the kernel crate.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime --lib telegram_model_runner -- --nocapture
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
git diff --check
```

### 21:1x Progress - Telegram Model Invocation Policy Moved Into Hepta Kernel

Continued after the send-delivery policy slice:

1. Moved side-effect-free Telegram duplicate/model-invocation policy into
   `hepta-kernel`:
   `HeptaKernelTelegramDuplicateDecision`,
   `HeptaKernelTelegramCandidateMaterial`,
   `HeptaKernelTelegramReplyTargetMaterial`,
   `HeptaKernelTelegramModelInvocationRequestPlan`, and
   `HeptaKernelTelegramModelExecutionReport`.
2. Added kernel-owned duplicate/cursor helpers:
   `hepta_kernel_telegram_update_already_drained`,
   `hepta_kernel_telegram_next_update_offset`, and
   `hepta_kernel_telegram_duplicate_decision`.
3. Kept `hepta-runtime::telegram_model_runner` and
   `hepta-gateway::telegram_policy` compatibility names so existing gateway,
   status, transport, and CLI callers still use the `NativeTelegram*` surface
   while the policy is owned by the kernel.
4. Preserved the boundary: Telegram JSON parsing, actual model execution,
   Bot API calls, cursor commits, delivery ledger writes, retries, token
   checks, network I/O, and launchd mutation remain outside `hepta-kernel`.

Focused gates passed:

```text
cargo fmt --all --manifest-path codex-rs/Cargo.toml
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime --lib telegram_model_runner -- --nocapture
CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-kernel -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway --lib telegram_ -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture
CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture
```
