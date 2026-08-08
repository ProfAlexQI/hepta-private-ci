# Hepta vNext Migration Contract

Hepta vNext uses upstream Codex as its only execution spine. The old Hepta
implementation is an oracle and evidence source, not a merge target.

## Frozen baselines

- Candidate spine: execution-time `upstream/main`, currently
  `6f647caa9bd62b16824cf5efc8e4575090feddf4`.
- Read-only Hepta oracle: `2f704dc7c1172cefca908852456beccf4d02a5d1`.
- The old raw-U evidence run, its receipts, and its verification semantics stay
  isolated from vNext work.

## Non-negotiable invariants

1. Codex owns `ThreadManager`, `CodexThread`, session/turn execution,
   `ToolRegistry`/`ToolRouter`, state, rollout, and thread storage.
2. Hepta adds typed extensions; it does not add a second session, router,
   scheduler, or generic state store.
3. Hepta product behavior is enabled explicitly and remains disabled for the
   `codex` binary by default.
4. Governance decisions and terminal lifecycle observations cross crate
   boundaries as typed contracts, not route-specific JSON builders. External
   effect acknowledgements remain separate typed material and must never be
   inferred from handler completion.
5. Old route/report/script families migrate only through parameter tables,
   state machines, manifests, or generated projections.
6. Every migrated family must pass an old-vs-vNext canonical oracle before the
   old implementation can be retired.
7. Every new public Hepta surface must have a named product caller in
   `CALLERS.toml`. Caller-zero work remains outside the promotion stack.

## Migration waves

1. Product shell: one CLI parser, `codex` and `hepta` product entries, isolated
   product configuration, and upstream-compatible release targets.
2. Governance: typed admission, authorization, effect acknowledgement,
   terminal receipt, and reconciliation hooks around the Codex lifecycle.
3. Memory: Memory, Intelligence, and KG become context/state contributors;
   mutations remain shadow-only until oracle coverage is complete.
4. Channels: Telegram, Matrix, and native clients become app-server/thread
   adapters and never enter the governance kernel.
5. Evidence: persist typed receipts and project stable historical selectors
   without recreating direct compatibility routes.
6. Control UI: generate one protocol/schema projection from Rust types and
   route all mutations back through the Codex lifecycle.
7. Retirement: prove caller zero, complete shadow soak and operator acceptance,
   then remove old report, controlled, route, facade, and script families.

## Transplant whitelist

- Stable identities, capability descriptors, frozen context, decisions, and
  terminal receipt shapes.
- Authority, egress, durable-store, and evidence boundary semantics.
- Memory, Intelligence, and KG domain algorithms after dependency audit.
- Channel protocol adapters after kernel-specific code is removed.
- Historical evidence IDs, selectors, and canonical fixtures needed as oracles.

## Never transplant wholesale

- The report surface and per-state JSON builder families.
- `controlled_*` and status-canary module forests.
- Retired direct route dispatch.
- Recursive `include!` closures and hand-written re-export facades.
- Gate scripts that duplicate timeout, SHA, canonical JSON, stderr, receipt,
  lock, or `jq` behavior.
- Telegram-specific kernel policy.

## Acceptance gates per family

1. Typed contract and owner are explicit.
2. vNext output matches the canonical old-Hepta oracle byte-for-byte where the
   external contract is retained.
3. Shadow evaluation preserves upstream handler behavior and records durable
   observations when the evidence backend is available; it does not itself
   authorize new Hepta mutations. A shadow backend failure is warn-and-allow
   and must not be reported as durable evidence.
4. Static callers and persistent telemetry show the replacement is complete.
   A caller-zero writer is reported as `writer_not_composed`; it must never be
   collapsed into a zero-event observation. S2 persistent/OTel hit, failure,
   and latency telemetry remains ineligible until a product writer is named in
   `CALLERS.toml`. S5 similarly cannot report authoritative zeroes while its
   reader is not composed.
5. An operator receipt authorizes retirement.
6. Exact-SHA local, Nix, and hosted receipts are refreshed only after the
   candidate SHA is clean and frozen.

## Current vertical slice

- The `hepta` and `codex` binaries compile the same CLI parser.
- `hepta` forces the private `hepta_governance` feature; `codex` leaves it off.
- App-server and MCP thread registries install a feature-gated governance
  extension.
- Codex Extension API exposes one generic two-phase `ToolPolicyContributor`:
  admission sees the original payload, authorization sees the effective payload
  after trusted hook rewrites, and both execute on the only ToolRegistry path.
- Tool identities use a versioned, length-delimited digest. Decisions bind a
  versioned policy stamp and payload digest while preserving direct versus Code
  Mode call identity; raw tool arguments are never written to governance rows.
  Both bootstrap phase records are explicitly `NotEvaluated`. The
  Authorization phase name identifies the lifecycle boundary only; it is not
  capability-policy authorization, fresh approval, or an execution credential.
- `codex-hepta-evidence` owns an append-only `hepta_evidence_2.sqlite` lineage with WAL,
  `synchronous=FULL`, immutable rows, foreign-key-bound decisions/receipts,
  exact-replay idempotency, and hard conflict detection. It is opened lazily
  only for feature-enabled threads and is not a rebuildable Codex state index.
  Open validates SQLite `quick_check`, SQLx migration checksums, a required
  table/index/immutable-trigger schema-object manifest, and
  `foreign_key_check`. This is a required-object and SQL-fragment ratchet, not
  a cryptographic proof against complete database replacement.
- The App Server and MCP installations currently use Shadow mode until
  canonical oracle soak and an operator acceptance receipt. In Enforce mode, a
  pre-handler decision-write failure blocks the handler. A terminal-write
  failure occurs after the handler, returns a fatal error, and preserves an
  authorized pending action so replay of that same action is blocked; it does
  not prove exactly-once delivery or lock unrelated later effects.
- An opaque per-dispatch attempt ID scopes the in-process claim. Only the
  attempt that wins the first durable admission insert may finalize the
  original receipt. Enforce blocks an existing action before the handler.
  Shadow deliberately allows the replay to run, but the replay attempt cannot
  borrow the original claim or mint or complete its receipt. The attempt ID is
  not itself a durable execution credential.
- An observed abort terminal after durable authorization is recorded as
  `Indeterminate`. If the outer tool runtime observes a handler-task panic or
  `JoinError` before another terminal commits, it records one stable
  `Indeterminate` terminal for the same attempt. Dropping the outer future can
  still leave an authorized action pending; no asynchronous `Drop` path invents
  evidence. Enforce blocks a later replay. Shadow performs no automatic
  reconciliation, but an explicit retry is still allowed and cannot finalize
  the original receipt.
- Active policy dispatches use a monotonic, attempt-scoped terminal state for
  pre-handler and handler outcomes. Once a handler result starts its terminal
  write, cancellation cannot relabel it as `Aborted`. A bounded terminal-write
  timeout becomes unconfirmed and fatal. Once the policy terminal commits,
  cancellation may stop the PostToolUse/lifecycle tail and returns a
  result-withheld, do-not-retry response without exposing the raw tool result.
  Feature-disabled dispatch keeps the upstream legacy cancellation ordering.
- Provider invocation types, a generic Hepta-neutral Extension API pre-send
  contributor/opaque lease, and separate immutable provider intent/terminal
  tables now exist. Hepta registers a provider-policy implementation that can
  atomically claim one retry-stable host request binding, persist its exact
  terminal, and block pending, completed, or indeterminate retries in Enforce
  mode. Raw host attempt/request identities, endpoint contents, and provider
  material cross the evidence boundary only as SHA-256 digests. Unary remote
  compaction success has its own items-only terminal; missing response IDs,
  token usage, or end-turn fields are never synthesized. Core HTTP, websocket,
  compact, prewarm, detached-memory, and physical-send paths now use the shared
  provider attempt/terminal lifecycle. This does not prove exactly-once external
  delivery.
- The promoted stack governs ToolRegistry dispatch and the listed provider send
  paths. Memory recall, memory mutation writers, external channel transport,
  outbound delivery, effect ACK, retirement, and automatic reconciliation stay
  outside this candidate. `HandlerCompleted` is handler-reported status, not
  proof of an external effect or exactly-once execution.
- The Hepta product resolves its process home as
  `HEPTA_HOME > CODEX_HOME > ~/.hepta` before the shared Codex loader starts.
  Ordinary `codex` keeps its upstream home behavior. Explicit `HEPTA_HOME` and
  `CODEX_HOME` values must already name a directory, and non-UTF-8 values fail
  rather than silently falling back. The resolved home is bound once within a
  process and cannot be rebound. Across launches, changing `HEPTA_HOME` selects
  a different evidence path; current code neither detects nor migrates the
  prior store. No implicit merge or fallback occurs, so store-instance binding
  and a checked migration gate remain future work.
- `hepta --version` embeds an unbound identity for ordinary local builds.
  Release automation may declare an exact identity only with a 40- or 64-hex
  `CODEX_RELEASE_SOURCE_SHA` and `CODEX_BUILD_SOURCE_DIRTY=false`. The resolver
  does not inspect Git or cryptographically bind the artifact to that checkout,
  so this is declared build metadata, not verified exact-SHA provenance.
  Readback, Nix, hosted, signing, and promotion receipts remain acceptance work.
