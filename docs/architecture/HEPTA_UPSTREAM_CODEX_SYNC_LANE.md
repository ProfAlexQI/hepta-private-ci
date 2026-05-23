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

The P1 compatibility absorption-contract gate is:

```bash
scripts/hepta-upstream-codex-legacy-compatibility-absorption.sh
```

This gate selects the `legacy-cli-tui-compatibility` bucket. It keeps upstream
CLI, TUI, code-mode, terminal-detection, and `utils/cli` deltas as compatibility
snapshot inputs until explicit Hepta command contracts, behavior-equivalence,
shadow-replay, active dependency isolation, and Hepta-native parity gates are
green. It does not promote Codex CLI/TUI behavior into the active Hepta service.

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
