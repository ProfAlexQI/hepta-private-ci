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
- `hepta --hepta-vnext-live-shell-contract-v1` prints one exact machine
  contract. Release materialization rejects any executable (including the
  legacy broad-capability binary) that does not emit that exact route,
  immutable-open, and all-authority-false contract.

The executable handshake is a capability/shape guard, not a cryptographic
build-provenance attestation. V1 assumes a cooperative same-euid operator has
selected the reviewed local build artifact. Promotion-grade provenance is a
separate future gate; it is not inferred from this receipt.

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
  --source-state-root /absolute/vnext/live-snapshot \
  --target-manifest /absolute/release/manifest.json \
  --snapshot-receipt /absolute/evidence/state-snapshot.json \
  --output-receipt /absolute/evidence/canary.json \
  --output-soak-receipt /absolute/evidence/soak.json
```

The E2E harness binds only `127.0.0.1:17373`, uses unique canary launchd
labels, exercises installer plans without `--install`, and verifies that every
state-fixture byte remains unchanged. It also inventories the production 7373
listener and LaunchAgents before and after, rejects old protected routes and
all non-GET requests, binds the source commit/binary digest, and emits the
three-sample soak as separate private evidence. Bridge-ready evidence requires
a clean source worktree plus the exact target manifest and state-snapshot
receipt; the canary binds those digests while still exercising a private copy.

## Release and launchd defaults

`hepta-immutable-release-tree materialize` binds the state root, install root,
gateway/watchdog labels, and loopback port into one immutable manifest. The
materializer requires an exact lowercase 40-hex `--source-commit`; it never
infers provenance from the current directory. It also invokes the candidate
executable's exact live-shell contract before copying it and again when the
manifest is verified.
The
labels and port remain compatible with the existing internal production
service, but state and executable roots are intentionally separate:

- state root: `~/.local/share/hepta-vnext/live-snapshot`
- install root: `~/.local/opt/hepta-vnext`
- gateway label: `ai.hepta.gateway`
- watchdog label: `ai.hepta.installed-live-watchdog`
- listen address: `127.0.0.1:7373`

The installer commands are dry-run unless `--install` is explicitly present.
Direct `--install` is limited to non-production canary labels and ports;
production-compatible labels or port 7373 can transition only through the
receipt-bound bridge.
The generation-pointer command changes only the single `active` symlink and
never restarts a service. Mutating pointer operations use an exclusive,
fail-closed lock and reserve both a durable pending receipt and pending chain
cursor before changing `active`. Public verification accepts only a ready
cursor whose exact head receipt digest is present. An interruption at any
publish phase therefore fails closed and can be aborted back to the reviewed
source generation with:

```sh
scripts/hepta-generation-pointer recover-pending \
  --receipt /absolute/evidence/interrupted-generation.json
```

Recovery accepts only the reviewed source or target release bytes, restores
the previous pointer and monotonic chain head, and seals a recovery receipt
that is not a transition PASS. A hard kill may leave the exclusive lock
directory orphaned; inspect the pending receipt, cursor, and pointer before
manually removing that lock and running recovery.
The watchdog is read-only, binds the exact manifest to the active release, and
never self-heals by restarting or changing a generation.

## State snapshot and legacy rollback bridge

The runtime must never attach to the mutable legacy SQLite namespace. After
the legacy writer has been stopped, create the production vNext snapshot with
`hepta-state-snapshot --materialize`. New receipts use the full-root v2
contract: every top-level entry (including archive and release-run data) is
covered, not only `runtime-v2`. The command independently confirms that the
legacy launchd label is unloaded and no process has any file under the state
root open, rejects nonempty WAL files and symlink/special/delimiter paths, and
preserves WAL/SHM/key bytes plus mode, uid, gid, mtime, BSD flags, ACLs, and
xattrs. Regular-file hardlinks are bound by a portable alias-group inventory,
preserved across materialization, and reverified; an alias whose kernel link
count extends outside the state root is rejected rather than silently split.
A source identity inventory (device/inode/ctime included) detects drift
during the copy; a separate portable payload inventory is compared across the
copy boundary. A destination-derived binding prevents replaying a receipt
against another copied root. Existing runtime-v2-only v1 receipts remain
verifiable for an old v1 release only. A release whose manifest declares
`full-state-root-v2` rejects v1 snapshot/canary evidence during both persisted
canary generation and bridge preparation.

The tool does not stop or start either service. The receipt is first reserved
as `pending`, then sealed only after the destination and binding are published;
`hepta-state-snapshot verify --receipt` recomputes the destination inventories,
while `hepta-state-snapshot verify-source --receipt` rechecks that the original
full root is still quiescent and identity-exact. Bridge preparation and every
forward cutover or recutover run that source-freshness check. The persisted
canary v2 receipt binds the snapshot id and all full-root inventory digests. If
the legacy generation is rolled back and writes state, the old snapshot becomes
stale and recutover is refused until new full-root snapshot/canary/soak evidence
is created.

The old broad-capability executable is not copied into a vNext release and is
never relabeled as `authority_all_closed`. Instead,
`hepta-launchd-cutover-bridge prepare` archives and hashes the exact legacy
gateway/watchdog plists alongside the exact vNext templates. It also requires
and copies the reverified snapshot, exact-binary canary, and bounded-soak
receipts. Its transition commands are dry-run unless `--apply` is present:

```sh
scripts/hepta-launchd-cutover-bridge prepare \
  --manifest /absolute/release/manifest.json \
  --launch-agent-root /Users/qianqi/Library/LaunchAgents \
  --bundle /absolute/bridge \
  --snapshot-receipt /absolute/evidence/state-snapshot.json \
  --canary-receipt /absolute/evidence/canary.json \
  --soak-receipt /absolute/evidence/soak.json
scripts/hepta-launchd-cutover-bridge cutover --bundle /absolute/bridge \
  --output-receipt /absolute/evidence/cutover.json
scripts/hepta-launchd-cutover-bridge rollback --bundle /absolute/bridge \
  --previous-receipt /absolute/evidence/cutover.json \
  --output-receipt /absolute/evidence/rollback.json
scripts/hepta-launchd-cutover-bridge recutover --bundle /absolute/bridge \
  --previous-receipt /absolute/evidence/rollback.json \
  --output-receipt /absolute/evidence/recutover.json
scripts/hepta-launchd-cutover-bridge recover-pending \
  --bundle /absolute/bridge \
  --pending-receipt /absolute/evidence/interrupted-transition.json
```

After a rollback has written state, do not reuse the old plan or invoke its
`recutover`. Create a new immutable release bound to the new snapshot root and
prepare an epoch-v2 plan whose sole predecessor is the current rollback head:

```sh
scripts/hepta-launchd-cutover-bridge prepare-recutover \
  --manifest /absolute/new-release/manifest.json \
  --launch-agent-root /Users/qianqi/Library/LaunchAgents \
  --bundle /absolute/bridge-epoch-2 \
  --parent-bundle /absolute/bridge-epoch-1 \
  --previous-receipt /absolute/evidence/rollback.json \
  --snapshot-receipt /absolute/evidence/fresh-full-snapshot.json \
  --canary-receipt /absolute/evidence/fresh-canary.json \
  --soak-receipt /absolute/evidence/fresh-soak.json
scripts/hepta-launchd-cutover-bridge rebase-after-rollback \
  --bundle /absolute/bridge-epoch-2 \
  --previous-receipt /absolute/evidence/rollback.json \
  --output-receipt /absolute/evidence/rebase.json --apply
scripts/hepta-launchd-cutover-bridge recutover \
  --bundle /absolute/bridge-epoch-2 \
  --previous-receipt /absolute/evidence/rollback.json \
  --output-receipt /absolute/evidence/recutover-epoch-2.json --apply
```

Preparation is read-only. Rebase changes only the private CAS chain cursor,
never the service pair; it accepts exactly the reviewed current rollback head,
increments the plan epoch once, and records the parent plan. A stale, forked,
replayed, wrong-predecessor, or arbitrary rebase fails before receipt or service
mutation. Rebase and transition pending cursors are recoverable with the same
`recover-pending` command; recovery restores the reviewed parent/source chain
and publishes no rebase or transition PASS.

Rebase finalization is itself a receipt-bound two-stage CAS. The ready cursor
first records the exact pending receipt path and digest, the pending-cursor
digest, the prior-cursor digest, and the expected final rebase receipt digest.
Only after that final receipt is atomically published does a second CAS mark
the cursor finalized. If a crash lands between those steps,
`recover-pending --apply` accepts only that exact final receipt and only the
matching unfinalized cursor, then performs the missing CAS idempotently. Once
finalized, stale pending bytes at the original path, copied or modified pending
receipts, and sibling epoch rebases all fail closed without changing the
cursor. Recutover and controlled superseding epochs also reverify this exact
final receipt and pending provenance before advancing the chain. A superseding
epoch repeats that verification at apply time, before reserving any output or
mutating the cursor, so deleting, modifying, or downgrading the parent rebase
provenance after preparation cannot advance the epoch.

If evidence becomes stale after a successful rebase, or after recovery of an
interrupted v2 recutover, the unapplied epoch may be superseded without manual
cursor edits. `prepare-recutover` then requires the current v2 plan as its
parent, the same still-applied rollback receipt as its predecessor, and fresh
full-root snapshot/canary/soak evidence. The new plan records both the parent
chain head and the older applied rollback head, advances the epoch, and must
itself be rebased before recutover. Only this exact unapplied-parent shape is
accepted; a completed epoch or unrelated predecessor cannot be superseded.

Applied transitions reserve a durable pending receipt before changing either
plist. Both templates are staged and verified first; any publish, reload, or
health failure restores and rechecks the reviewed source pair. PASS requires
both launchd jobs, the loopback endpoint, and the listener executable digest
to match the target generation; vNext targets also pass a fresh three-sample
post-transition soak before the receipt seals. Before publishing a vNext pair,
the bridge holds both operation locks and
creates or revalidates the manifest-bound gateway and watchdog log directories
as physical, operator-owned `0700` paths; dry runs never create them.
Rollback restores the byte-exact legacy pair; receipts form a monotonic chain
supporting repeated
cutover → rollback → recutover → rollback cycles. The bridge self-test runs
that full cycle under a temporary LaunchAgents root and never calls production
launchd. If a process exits after reserving a pending receipt but before
sealing PASS, `recover-pending --apply` accepts only plist bytes from the
reviewed source or target pair, restores the reviewed source generation, and
replaces the pending record with a recovery receipt that explicitly publishes
no transition PASS. Running recovery without `--apply` is a read-only plan.
Prepare, applied transitions, and applied recovery share one lock under the
physical LaunchAgents root and the generation-pointer coordination lock under
the vNext install root, so different bundles or pointer operations cannot
interleave. A private
CAS chain cursor pins the exact current receipt path, digest, sequence, and
generation; stale predecessors and forked/replayed chains fail before any
receipt reservation or plist mutation. A crash can leave the lock directory
or chain cursor pending; both conditions fail closed until the reviewed
pending recovery is completed (an orphaned lock from an untrapped hard kill
must first be inspected and removed manually).

Transition receipt publication uses the same two-stage cursor finalization as
rebase. The pending receipt binds its own canonical path and the byte-exact
prior cursor; the ready cursor binds the pending receipt and pending cursor
digests but remains explicitly unfinalized until the final transition receipt
is atomically present. Recovery can then either restore the exact prior cursor
from a genuine pending operation or idempotently finish the final cursor CAS.
A copied, modified, or replayed pending receipt cannot roll back an already
finalized transition.
