# Hepta vNext live shell

This lane restores only the minimum runtime chain needed to canary the unified
trunk. It is not a promotion or retirement mechanism.

## Runtime boundary

- `codex-hepta-paths` owns the typed state root and the existing `runtime-v2`
  path layout.
- `codex-hepta-runtime` opens the two existing schema-v5 SQLite stores and the
  version-1 runtime snapshot read-only. It creates and migrates nothing.
- `codex-hepta-native-gateway` serves `/`, `/healthz`, and
  `/api/hepta/runtime` on an IPv4 or IPv6 loopback address. Every non-GET
  request is rejected.
- `hepta --serve-ui` selects that shell; all other `hepta` CLI modes retain the
  unified trunk behavior.

The schema-v5 adapter is deliberately narrow but cryptographically complete
for the state it reports. It uses the old `hmac-sha256-v1` key-id and row-MAC
domains, verifies the exact key IDs, scans every bounded durable row, and
authenticates the exact raw runtime payload before reporting `ready`. SQLite is
opened with one `immutable + query_only` connection. A nonempty adjacent WAL,
or any DB/WAL/SHM identity or SHA drift during inspection, fails closed.

The future process-owned Memory integration point remains
`codex_hepta_runtime::RuntimeStateAdapter`; the parallel Memory S1/S2 port must
reuse its already-open `StateDbHandle` and must not add a second database
opener here.

Telegram, outbound delivery, model invocation, operator mutation, Enforce,
promotion, retirement, and automatic transition are false in the runtime
status and have no gateway route.

## Isolated canary

Use a copied/private state fixture and port `17373`; never point a development
canary at the legacy production state root. The harness can take an exact
read-only source copy and checks metadata plus SHA-256 before and after on both
the source and copy:

```sh
/Volumes/T5/hepta-vnext/bin/hepta-ssd-run runtime-port -- \
  cargo build --manifest-path codex-rs/Cargo.toml -p codex-cli --bin hepta
/Volumes/T5/hepta-vnext/bin/hepta-ssd-run runtime-port -- \
  scripts/hepta-runtime-tests/canary-e2e.sh \
  --binary /Volumes/T5/hepta-vnext/cache/cargo-targets/runtime-port/debug/hepta \
  --source-state-root /Users/qianqi/.local/share/hepta
```

The E2E harness binds only `127.0.0.1:17373`, uses unique canary launchd
labels, exercises installer plans without `--install`, and verifies that every
state-fixture byte remains unchanged.

## Release and launchd defaults

`hepta-immutable-release-tree materialize` binds the state root, install root,
gateway/watchdog labels, and loopback port into one immutable manifest. The
labels and port remain compatible with the existing internal production
service, but state and executable roots are intentionally separate:

- state root: `~/.local/share/hepta-vnext/live-snapshot`
- install root: `~/.local/opt/hepta-vnext`
- gateway label: `ai.hepta.gateway`
- watchdog label: `ai.hepta.installed-live-watchdog`
- listen address: `127.0.0.1:7373`

The installer commands are dry-run unless `--install` is explicitly present.
The generation-pointer command changes only the single `active` symlink and
never restarts a service. The watchdog is read-only and never self-heals by
restarting or changing a generation.

## State snapshot and legacy rollback bridge

The runtime must never attach to the mutable legacy SQLite namespace. After
the legacy writer has been stopped, create the production vNext snapshot with
`hepta-state-snapshot --materialize`. The command independently confirms that
the legacy launchd label is unloaded and no process has `runtime-v2` open,
rejects nonempty WAL files, preserves modes and sidecars, detects source
identity/hash drift, and emits a private receipt. It does not stop or start
either service.

The old broad-capability executable is not copied into a vNext release and is
never relabeled as `authority_all_closed`. Instead,
`hepta-launchd-cutover-bridge prepare` archives and hashes the exact legacy
gateway/watchdog plists alongside the exact vNext templates. Its transition
commands are dry-run unless `--apply` is present:

```sh
scripts/hepta-launchd-cutover-bridge cutover --bundle /absolute/bridge
scripts/hepta-launchd-cutover-bridge rollback --bundle /absolute/bridge \
  --previous-receipt /absolute/cutover.json
scripts/hepta-launchd-cutover-bridge recutover --bundle /absolute/bridge \
  --previous-receipt /absolute/rollback.json
```

Applied transitions require a new output receipt. Cutover replaces the same
two launchd labels only after the independent vNext generation verifies;
rollback restores the byte-exact legacy plists and service pair; recutover is
bound to the rollback receipt. The bridge self-test exercises
cutover → rollback → recutover entirely under a temporary LaunchAgents root
and never calls production launchd.
