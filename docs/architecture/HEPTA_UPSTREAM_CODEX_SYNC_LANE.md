# Hepta Upstream Codex Sync Lane

This lane defines how Hepta consumes upstream `openai/codex` changes after
active service fusion.

The retained `codex-rs` compatibility surface is an intake and regression
oracle, not the active production engine. Upstream changes must move through a
bounded path:

1. Record the observed upstream head and diff range.
2. Classify deltas into provider/credential/security, runtime/session/tool,
   MCP/app-server, sandbox/exec, TUI/legacy command, and product/documentation
   groups.
3. Materialize Hepta absorption contracts for the selected deltas.
4. Keep `scripts/hepta-active-service-dependency-isolation.sh` green so the
   active `hepta-cli --bin hepta` service does not regain direct Codex engine
   dependencies.
5. Run preflight, watchdog, soak, and release-governance gates before any public
   release claim.

The head/diff intake gate is:

```bash
scripts/hepta-upstream-codex-snapshot.sh
```

By default this gate is offline/local-only. It records the Hepta repo head and
the `codex-rs` compatibility tree hash, and it emits the required risk buckets.
Set `HEPTA_UPSTREAM_CODEX_SNAPSHOT_OBSERVE_REMOTE=1` to perform a read-only
`git ls-remote` observation of `https://github.com/openai/codex` HEAD. Set
`HEPTA_UPSTREAM_CODEX_BASE_HEAD` and either `HEPTA_UPSTREAM_CODEX_TARGET_HEAD`
or remote observation to materialize a candidate diff range.

The concrete local diff-range ledger gate is:

```bash
scripts/hepta-upstream-codex-diff-ledger.sh
```

This gate is also offline by default. It uses the local upstream import baseline
`108234b5ebe6941764a6b8edbb37b2aa04369f07` and the local
`refs/remotes/openai-codex/main` target, currently
`7d47056ea42636271ac020b86347fbbef49490aa`, to classify the
`codex-rs` diff range into provider/security, runtime/session/tool,
legacy CLI/TUI compatibility, and product/release-governance buckets. Set
`HEPTA_UPSTREAM_CODEX_DIFF_BASE_HEAD`, `HEPTA_UPSTREAM_CODEX_DIFF_TARGET_HEAD`,
or `HEPTA_UPSTREAM_CODEX_DIFF_TARGET_REF` to audit a different already-present
local range. The gate does not fetch or merge upstream.

The first selected absorption-contract gate is:

```bash
scripts/hepta-upstream-codex-product-governance-absorption.sh
```

This gate selects the `product-doc-release-governance` bucket as the first
bounded absorption family. It verifies the 22 selected paths from the current
diff ledger and requires Hepta-specific translation before any product,
packaging, or release-governance wording is promoted. It explicitly does not
copy upstream docs verbatim, wire active runtime code, fetch or merge upstream,
or make a public release claim.

The translated Hepta release-governance packet is:

```bash
scripts/hepta-upstream-codex-product-governance-translation.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_PRODUCT_GOVERNANCE_TRANSLATION.md`.
It converts the selected upstream package, README, protocol, plugin, sandbox,
exec, network, and release-governance deltas into Hepta-owned policy language.
It keeps public release claims, active runtime wiring, and live plugin mutation
behind Hepta gates and operator approval.

The release-governance promotion packet gate is:

```bash
scripts/hepta-upstream-codex-release-governance-promotion.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_RELEASE_GOVERNANCE_PROMOTION.md`. It
marks the release-governance claim promotion packet ready with `7 / 7`
promotion conditions while still keeping public GA claims, public release
claims, release artifact writes, channel delivery, gateway RPC, and release
publication false.

The P1 compatibility absorption-contract gate is:

```bash
scripts/hepta-upstream-codex-legacy-compatibility-absorption.sh
```

This gate selects the `legacy-cli-tui-compatibility` bucket. It keeps upstream
CLI, TUI, code-mode, terminal-detection, and `utils/cli` deltas as compatibility
snapshot inputs until explicit Hepta command contracts, behavior-equivalence,
shadow-replay, active dependency isolation, and Hepta-native parity gates are
green. It does not promote Codex CLI/TUI behavior into the active Hepta service.

The P1 compatibility replay gate is:

```bash
scripts/hepta-upstream-codex-legacy-compatibility-replay.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_LEGACY_COMPATIBILITY_REPLAY.md`. It
turns the legacy compatibility intake contract into local replay evidence for
CLI command shape, TUI presentation, code-mode runtime callbacks, terminal/PTY
helpers, and the active dependency boundary. It still performs no active CLI/TUI
promotion, active runtime wiring, Codex engine dependency promotion, gateway RPC,
or public release claim.

The P1 compatibility promotion packet gate is:

```bash
scripts/hepta-upstream-codex-legacy-compatibility-promotion.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_LEGACY_COMPATIBILITY_PROMOTION.md`. It
marks the Hepta CLI/TUI parity promotion packet ready with `7 / 7` promotion
conditions while still keeping active CLI/TUI promotion, TUI presentation
promotion, code-mode promotion, runtime code wiring, channel delivery, gateway
RPC, and public release claims false.

The P0 provider/security absorption-contract gate is:

```bash
scripts/hepta-upstream-codex-provider-security-absorption.sh
```

This gate selects the `provider-credential-sandbox-security` bucket. It freezes
the current 104 upstream paths covering `codex-api`, model providers, login,
config/permissions, exec/approval, Linux and Windows sandboxing, and
network-proxy policy as a P0 review contract. It requires redacted Hepta provider
contracts, sandbox/exec replay, network-policy replay, active dependency
isolation, operator approval, and soak evidence before any security or provider
behavior is promoted into the active Hepta runtime.

The P0 provider/security replay gate is:

```bash
scripts/hepta-upstream-codex-provider-security-replay.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_PROVIDER_SECURITY_REPLAY.md`. It turns
the provider/security intake contract into local redaction and replay evidence:
redacted provider contracts, credential redaction, approval-policy replay,
sandbox/exec replay, network-proxy replay, and side-effect boundary replay. It
still performs no credential read, provider invocation, gateway RPC, active
provider promotion, active security-policy promotion, or active runtime wiring.

The P0 provider/security promotion packet gate is:

```bash
scripts/hepta-upstream-codex-provider-security-promotion.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_PROVIDER_SECURITY_PROMOTION.md`. It
marks the provider/security per-surface promotion packet ready with `7 / 7`
promotion conditions while still keeping active provider promotion,
security-policy promotion, credential reads, provider invocation, live network
allowance, gateway RPC, and public release claims false.

The P0 runtime/app-server absorption-contract gate is:

```bash
scripts/hepta-upstream-codex-runtime-appserver-absorption.sh
```

This gate selects the `runtime-session-tool-mcp-appserver` bucket. It freezes
the current 462 upstream paths covering app-server protocol/daemon/transport,
session and thread-store behavior, tools, MCP, hooks, exec-server, and runtime
event-loop surfaces as a P0 runtime review contract. It requires Hepta route and
event contracts plus session/thread, tool/MCP, app-server protocol, exec, and
hook replay before any active runtime behavior is promoted.

The P0 runtime/app-server replay gate is:

```bash
scripts/hepta-upstream-codex-runtime-appserver-replay.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_RUNTIME_APPSERVER_REPLAY.md`. It turns
the runtime/app-server intake contract into local replay evidence for
app-server protocol schemas, daemon/transport boundaries, session/thread-store
lifecycle, tool-policy invocation, MCP request envelopes, exec/hook event-loop
behavior, and side-effect boundaries. It still performs no active runtime
promotion, app-server promotion, tool/MCP promotion, runtime wiring, credential
read, provider invocation, channel delivery, gateway RPC, or public release.

The P0 runtime/app-server promotion packet gate is:

```bash
scripts/hepta-upstream-codex-runtime-appserver-promotion.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_RUNTIME_APPSERVER_PROMOTION.md`. It
marks the runtime/app-server route-event promotion packet ready with `7 / 7`
promotion conditions while still keeping active runtime promotion, app-server
promotion, tool/MCP promotion, runtime code wiring, channel delivery, gateway
RPC, and public release claims false.

The absorption/replay readiness gate is:

```bash
scripts/hepta-upstream-codex-absorption-replay-readiness.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ABSORPTION_REPLAY_READINESS.md`. It
summarizes the frozen diff ledger as 878 changed upstream paths, 716 selected
absorption paths, four selected buckets, four absorption contracts, and four
required translation/replay packets. It does not claim that every upstream file
has been ported into active Hepta code; it only closes readiness for the
selected bucket contracts while keeping active Codex engine dependencies,
runtime wiring, automatic rebase, gateway RPC, and public release claims false.

The promotion-readiness decision gate is:

```bash
scripts/hepta-upstream-codex-promotion-readiness.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_PROMOTION_READINESS.md`. It consumes the
absorption/replay readiness result and explicitly decides that `4 / 4` selected
buckets are assessed, `4 / 4` absorption/replay sources are ready, `4` surface
promotion packets are complete, and `0` buckets are promotable. The gate
therefore keeps active promotion closed until explicit active-wiring/public-claim
approval evidence exists.

The promotion closure/denial gate is:

```bash
scripts/hepta-upstream-codex-promotion-closure.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_PROMOTION_CLOSURE.md`. It consumes the
promotion-readiness decision and makes the final invariant explicit: all `4`
required promotion packets are complete, `0` buckets are promotable, all `4`
buckets remain blocked from active promotion, `active_promotion_ready=false`,
and public release/GA claims plus release artifact writes remain false.

The active-wiring precondition gate is:

```bash
scripts/hepta-upstream-codex-active-wiring-precondition.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVE_WIRING_PRECONDITION.md`. It
requires the promotion closure gate to be ready and records that active wiring
still needs an explicit operator approval record, a concrete
`activation_request_id`, live dependency isolation, watchdog, browser smoke, and
long soak evidence. It keeps active runtime wiring, Codex engine dependency
promotion, public release claims, public GA claims, and release artifact writes
false.

The activation request packet schema gate is:

```bash
scripts/hepta-upstream-codex-activation-request-packet.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_REQUEST_PACKET.md`. It
records the required `14`-field activation packet schema, including
`activation_request_id`, operator approval id, hashed operator identity, live
gate evidence ids, rollback plan id, and release-decision fields. It keeps the
activation packet unrecorded and active wiring false by default.

The activation packet dry-run validator gate is:

```bash
scripts/hepta-upstream-codex-activation-packet-dry-run.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_PACKET_DRY_RUN.md`. It runs
three representative placeholder fixtures against the `14`-field schema and
requires all three to stay blocked, with `0` allowed fixtures and public
release/artifact-write decisions false.

The activation evidence ledger gate is:

```bash
scripts/hepta-upstream-codex-activation-evidence-ledger.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_LEDGER.md`. It
defines `8` required evidence slots for activation request id, operator
approval, hashed identity, live dependency isolation, watchdog, browser smoke,
long soak, and rollback plan. It keeps recorded evidence and fresh evidence at
`0`, so active wiring remains false.

The activation readiness closure gate is:

```bash
scripts/hepta-upstream-codex-activation-readiness-closure.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_READINESS_CLOSURE.md`. It
requires the activation request schema, dry-run validator, and evidence ledger
to be ready while keeping operator-approved activation, active wiring, public
release claims, and release artifact writes denied by default.

The activation denied sample gate is:

```bash
scripts/hepta-upstream-codex-activation-denied-sample.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_DENIED_SAMPLE.md`. It keeps
a full-shaped sample packet blocked when operator approval is not recorded and
activation evidence freshness remains `0`.

The activation evidence freshness policy gate is:

```bash
scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_FRESHNESS_POLICY.md`.
It defines freshness anchors and max-age policies for all `8` evidence slots.
The current policy records no evidence, so missing evidence remains an explicit
denial reason and active wiring stays false.

The activation evidence binding record gate is:

```bash
scripts/hepta-upstream-codex-activation-evidence-binding-record.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_BINDING_RECORD.md`.
It defines the concrete evidence record schema for all `8` evidence ids:
evidence record id, source gate, timestamp, active binary SHA, route/status
hash, artifact hash or redacted path, and activation request id binding. It
keeps recorded binding records at `0`, so active wiring remains false.

The activation evidence denied fixture gate is:

```bash
scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_DENIED_FIXTURE.md`.
It fills all `8` evidence record shapes with placeholder values and requires
them to stay blocked when operator approval, request binding, live gate hashes,
artifact hashes, and freshness are not verified.

The trusted evidence acceptance matrix gate is:

```bash
scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_TRUSTED_EVIDENCE_ACCEPTANCE_MATRIX.md`.
It enumerates the seven verification checks required for all `8` evidence
records before trusted evidence can be accepted: operator approval, activation
request binding, active binary SHA, route/status hash, artifact hash or
redacted path, freshness window, and trusted source verification. All accepted
record counts remain `0`, so active wiring remains false.

The trusted record shape validator gate is:

```bash
scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_TRUSTED_RECORD_SHAPE_VALIDATOR.md`.
It tests `partial-trusted-records` and
`public-claim-attempt-with-trusted-shape` fixtures. Both stay blocked, public
release and artifact writes stay false, and active wiring remains false until
all `8` evidence records satisfy every freshness, binding, trust, and operator
approval requirement.

The activation evidence completeness scoreboard gate is:

```bash
scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_COMPLETENESS_SCOREBOARD.md`.
It summarizes `10` activation evidence gate families. All are ready and all
continue blocking activation without trusted evidence; accepted trusted record
count and fresh trusted record count remain `0`, so operator-approved
activation readiness remains false.

The activation evidence recording dry-run receipt gate is:

```bash
scripts/hepta-upstream-codex-activation-evidence-recording-dry-run-receipt.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECORDING_DRY_RUN_RECEIPT.md`.
It defines the redacted receipt schema for future evidence recording. All `12`
receipt fields remain absent by default, accepted/fresh trusted evidence counts
remain `0`, and active wiring/public claim decisions stay false.

The activation evidence recording denial matrix gate is:

```bash
scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECORDING_DENIAL_MATRIX.md`.
It covers `3` denied receipt attempts, including partial receipt fields, stale
trusted evidence, and a public-claim/release-artifact attempt. All attempts stay
blocked; no receipt is persisted, no workspace write is allowed, and active
wiring/public claim decisions remain false.

The activation evidence receipt persistence command contract gate is:

```bash
scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_PERSISTENCE_COMMAND_CONTRACT.md`.
It defines the disabled/no-op command shape required before any future receipt
persistence write path. All `10` command fields remain absent by default, the
command is not invoked, no receipt is persisted, and active wiring/public claim
decisions remain false.

The activation evidence receipt persistence invocation dry-run gate is:

```bash
scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh
```

This gate verifies
`docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_PERSISTENCE_INVOCATION_DRY_RUN.md`.
It models `3` redacted command invocation attempts, including a fully shaped
redacted command, a stale-evidence attempt, and a public-claim/artifact attempt.
All attempts remain `blocked_noop`: no command is invoked, no persistence
execution runs, no workspace write occurs, no evidence receipt is persisted, and
active wiring/public claim decisions remain false.

The local sync-lane gate is:

```bash
scripts/hepta-upstream-codex-sync-lane.sh
```

Set `HEPTA_UPSTREAM_CODEX_SYNC_REQUIRE_LIVE=1` to require live route
confirmation from the running service as well as the offline cargo-tree
isolation check.

This lane does not fetch upstream, merge commits, restart the active service,
read credentials, invoke providers, deliver channel messages, or publish a
release by itself.
