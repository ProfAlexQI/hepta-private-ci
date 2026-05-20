# Hepta CLI And Script Migration Matrix

Date: 2026-05-20
Scope: old standalone Hepta CLI/script surface versus current `hepta-codex`
Status: public-GA local readiness slices and CLI breadth inventories landed

## Why This Exists

The merge audit found that the non-CLI Hepta crates are largely absorbed into
`hepta-codex`, but the old standalone `hepta-cli` command breadth and script
operations are not yet fully represented.

This document turns that finding into a migration/retirement matrix rather than
leaving it as a vague "not done" bucket.

## Current Counts

Old standalone Hepta:

- `crates/hepta-cli/src/*_ops.rs`: 65 files
- rough slash/command references: 574
- scripts: 20

Current `hepta-codex`:

- native gateway source commands: 62 after the Hepta Native packaging gate
  (`61` after the public GA readiness gate; `60` after the provider/channel
  dry-run plan; `59` after the release/hardening status gate; `58` after the
  memory/capability absorption inventory endpoint; `57`
  after the local tooling/content inventory endpoint; `56` after the channel
  adapter status inventory endpoint; `55` after the runtime/session dry-run
  inventory endpoint; `54` after the provider metadata inventory endpoint;
  `53` after the CLI command inventory endpoint)
- scripts before this slice: 1
- scripts after this slice: 15

New executable scripts added in this slice:

- `scripts/hepta-codex-preflight.sh`
- `scripts/hepta-codex-live-soak.sh`
- `scripts/hepta-codex-watchdog.sh`
- `scripts/hepta-codex-browser-visual-smoke.sh`
- `scripts/hepta-codex-cli-command-inventory.sh`
- `scripts/hepta-codex-provider-metadata-inventory.sh`
- `scripts/hepta-codex-runtime-session-dry-run-inventory.sh`
- `scripts/hepta-codex-channel-adapter-status-inventory.sh`
- `scripts/hepta-codex-local-tooling-content-inventory.sh`
- `scripts/hepta-codex-memory-capability-inventory.sh`
- `scripts/hepta-codex-release-hardening-status-gate.sh`
- `scripts/hepta-codex-provider-channel-dry-run-plan.sh`
- `scripts/hepta-codex-public-ga-readiness.sh`
- `scripts/hepta-codex-native-packaging-gate.sh`

Existing carried script:

- `scripts/hepta-control-ui-smoke.sh`

## Script Migration Matrix

| Old standalone script family | Current status in `hepta-codex` | Decision |
| --- | --- | --- |
| `hepta-control-ui-smoke.sh` | carried and adapted | keep |
| `hepta-v0.1-preflight.sh` | replaced by `hepta-codex-preflight.sh` | migrate as Codex package preflight |
| `hepta-v0.1-soak.sh` | replaced by `hepta-codex-live-soak.sh` for active-service coexistence | migrate scoped soak |
| Control UI browser/visual smoke | replaced by `hepta-codex-browser-visual-smoke.sh` for Chrome headless gateway-index screenshots | migrate scoped visual smoke |
| `hepta-installed-live-watchdog*.sh` | replaced by `hepta-codex-watchdog.sh` for one-shot live status | migrate recurring service later |
| `hepta-gateway-service*.sh` | covered by controlled install docs and launchd evidence | do not auto-install; keep manual/explicit |
| `hepta-watchdog-service*.sh` | not ported as LaunchAgent manager | defer until recurring watchdog is requested |
| `hepta-release-artifact-pack.sh` | not ported | defer until release packaging track |
| `hepta-external-production-gates.sh` | not ported | defer; contains external-evidence policy not suitable for blind activation |
| `hepta-external-production-o60-o69-gate.sh` | not ported | defer; old standalone release train specific |
| `hepta-production-parity-gate.sh` | not ported as script | partially represented by live `/api/operator-security`, `/api/gateway-runtime`, `/api/control-ui-route-parity` |
| `hepta-project-hardening-gate.sh` | not ported | defer |
| `hepta-release-architecture-gate.sh` | not ported | defer |
| `hepta-ops-status-gate.sh` | not ported | partially represented by `hepta-codex-watchdog.sh` |
| `hepta-local-import.sh` | not ported | partially represented by `/api/config`, `/api/optional-configs`, and installed env |
| `hepta-onboard-daemon-wizard-parity-gate.sh` | not ported | defer |
| `hepta-autonomous-coding-subagent-gate.sh` | not ported | defer until old worker command surface is intentionally re-exposed |
| Runtime/session dry-run inventory | replaced by `hepta-codex-runtime-session-dry-run-inventory.sh` for 12 old runtime/admin ops files | migrate as side-effect-free dry-run inventory |
| Channel adapter status inventory | replaced by `hepta-codex-channel-adapter-status-inventory.sh` for 13 old channel/runtime adapter ops files | migrate as disabled/live-gated status inventory |
| Local tooling/content inventory | replaced by `hepta-codex-local-tooling-content-inventory.sh` for 11 old canvas/diff/filesystem/process/search/tool/wiki ops files | migrate as planning-only inventory |
| Memory/capability absorption inventory | replaced by `hepta-codex-memory-capability-inventory.sh` for 14 old memory/capability/plugin/coding-agent/skill ops files | migrate as read-only gap inventory |
| Release/hardening status gate | replaced by `hepta-codex-release-hardening-status-gate.sh` for 12 remaining old release/external/ops/hardening script families | migrate as local-only status gate |

## CLI Ops Migration Matrix

### Absorbed As Runtime Libraries

These areas are represented by imported non-CLI crates and current tests, but
not necessarily by the old standalone slash-command names:

- memory/intelligence/kernel reports
- runtime worker/snapshot/evidence machinery
- core capability and control UI reports
- plugin metadata/core registry contracts
- external benchmark/control UI/operator reports

### Surfaced Through Current Native Gateway

These are currently exposed as live API/source-command equivalents:

- control UI
- UI contract audit
- operator snapshot/security/console
- sessions/activity/transcript/task evidence routes
- gateway runtime/dispatch/ledger/dead-letter
- Telegram owner handoff / production readiness / delivery ledger
- native POST readiness/stores/activation/gray-release/rollout
- machine-readable merge/function completion audit
- machine-readable CLI command breadth inventory
- machine-readable provider/search metadata inventory
- machine-readable runtime/task/session dry-run inventory
- machine-readable channel adapter disabled status inventory
- machine-readable local tooling/content planning inventory
- machine-readable memory/capability absorption gap inventory
- machine-readable release/hardening status gate
- machine-readable provider/channel/runtime dry-run approval plan
- machine-readable public GA readiness gate
- machine-readable Hepta Native packaging readiness gate
- task/chat/approval dry-run and confirm-required plans
- external agent benchmark

### Not Yet First-Class In `hepta-codex`

The old standalone specialized ops modules remain the major gap:

- provider bridges: Anthropic, OpenAI, OpenAI Codex, OpenRouter, DeepInfra,
  Mistral, Google/Vertex/Gemini, xAI, Ollama
- channel/runtime adapters: Discord, Feishu, iMessage, Google Chat, Telegram
  old command surface, voice/TTS, file/media transfer
- plugin migration/audit mega-surface
- runtime event plane command surface
- diagnostics/OTel/Prometheus old command surface
- process/filesystem/search/tool invocation old command surface
- autonomous coding/subagent old command surface

These are not safe to bulk-enable blindly because many are credentialed,
external, or old-workspace-specific. They should be migrated as explicit slices:

1. read-only metadata/status commands;
2. dry-run planners;
3. isolated temp-workspace smokes;
4. credentialed/live smokes only after explicit approval.

## New Script Contracts

### `hepta-codex-preflight.sh`

Purpose: repeat the local deterministic package gate for current Codex fork.

Default gates:

- codex workspace metadata
- rustfmt check
- cargo check for all six Hepta crates plus `codex-cli --bin hepta`
- `hepta-gateway` tests
- CLI `native_gateway`, `native_telegram`, and `native_post` tests
- Control UI smoke
- Hepta Native metadata/check/tests
- `git diff --check`

Optional:

- set `HEPTA_CODEX_PREFLIGHT_RELEASE=1` to include release build.

### `hepta-codex-live-soak.sh`

Purpose: repeat active-service coexistence stability checks.

Checks:

- `/health=ready`
- route parity ready and missing routes `0`
- `active_owner=legacy_openclaw`
- no double poller risk
- Telegram poll loop gated
- no live read/send by status
- native POST activation disabled
- native POST stores valid/capacity OK
- operator security in expected coexistence `attention` mode

### `hepta-codex-browser-visual-smoke.sh`

Purpose: repeat the browser-visible merge-completion check without invoking
Telegram, native POST real handlers, providers, or model calls.

Checks:

- gateway index contains `Merge completion`
- gateway index contains `82 / 91 / 88 / 68`
- gateway index links `/api/hepta-merge-completion`
- `/api/hepta-merge-completion` remains route-ready and side-effect disabled
- Chrome headless captures desktop and mobile PNG screenshots
- screenshot dimensions and minimum byte size are validated

The screenshots are written under a temporary directory by default, outside the
repo working tree.

### `hepta-codex-watchdog.sh`

Purpose: one-shot installed-service watchdog suitable for manual run or later
LaunchAgent wrapping.

Checks:

- release and installed binary sha
- health
- route parity
- operator security coexistence mode
- Telegram owner boundary
- native POST activation boundary
- native POST store health

It intentionally does not mutate launchd, Telegram ownership, POST handlers, or
delivery state.

### `hepta-codex-cli-command-inventory.sh`

Purpose: validate the read-only old CLI breadth inventory exposed by
`/api/hepta-cli-command-inventory`.

Checks:

- old standalone `*_ops.rs` count remains `65`
- old rough command reference count remains `574`
- old standalone script count remains `20`
- current `hepta-codex` script count is `15`
- native gateway source-command count is `62`
- CLI ops families cover all `65` old ops files
- merge-completion and CLI inventory reports agree on route/script counts
- provider invocation, credential reads, Telegram reads/sends, native POST
  mutation, external network reads, and filesystem writes remain disabled

### `hepta-codex-provider-metadata-inventory.sh`

Purpose: validate the metadata-only provider/search bridge inventory exposed by
`/api/hepta-provider-metadata-inventory`.

Checks:

- old provider ops file count remains `15`
- adjacent search/readability ops file count remains `3`
- current `hepta-codex` script count is `15`
- native gateway source-command count is `62`
- provider adapter count is `15`
- adjacent search adapter count is `3`
- provider live invocation and credentialed smoke remain disabled
- provider, CLI inventory, and merge-completion reports agree on route/script
  counts
- provider invocation, credential reads, external network reads, model calls,
  Telegram reads/sends, native POST mutation, and filesystem writes remain
  disabled

### `hepta-codex-runtime-session-dry-run-inventory.sh`

Purpose: validate the side-effect-free runtime/task/session dry-run inventory
exposed by `/api/hepta-runtime-session-dry-run-inventory`.

Checks:

- old runtime/admin ops file count remains `12`
- current `hepta-codex` script count is `15`
- native gateway source-command count is `62`
- dry-run surface count is `12`
- planner-ready count is `12`
- live mutation surface count is `0`
- task registry, session store, gateway event, hook, process, provider/model,
  credential, external network, Telegram, native POST, and filesystem effects
  remain disabled
- runtime/session, CLI inventory, provider inventory, and merge-completion
  reports agree on route/script counts

### `hepta-codex-channel-adapter-status-inventory.sh`

Purpose: validate the disabled/live-gated channel adapter status inventory
exposed by `/api/hepta-channel-adapter-status-inventory`.

Checks:

- old channel/runtime adapter ops file count remains `13`
- current `hepta-codex` script count is `15`
- native gateway source-command count is `62`
- adapter count is `13`
- disabled status ready count is `13`
- live adapter enabled count is `0`
- channel live read/send and Telegram owner handoff remain disabled
- connector credentials are not read
- channel delivery, external network reads/sends, voice calls, TTS playback,
  webhook delivery, file transfer, native POST mutation, and filesystem writes
  remain disabled
- channel, runtime/session, CLI inventory, provider inventory, and
  merge-completion reports agree on route/script counts

### `hepta-codex-local-tooling-content-inventory.sh`

Purpose: validate the planning-only local tooling/content inventory exposed by
`/api/hepta-local-tooling-content-inventory`.

Checks:

- old local tooling/content ops file count remains `11`
- current `hepta-codex` script count is `15`
- native gateway source-command count is `62`
- local tooling/content surface count is `11`
- planner-ready count is `11`
- live process execution, filesystem touch, network read, and tool invocation
  counts remain `0`
- process spawn, filesystem read/write, external network read, tool invocation,
  provider/model invocation, credential reads, channel read/send, native POST
  mutation, gateway mutation, and external send remain disabled
- local tooling/content, channel adapter, runtime/session, CLI inventory,
  provider inventory, and merge-completion reports agree on route/script counts

### `hepta-codex-memory-capability-inventory.sh`

Purpose: validate the read-only memory/capability absorption gap inventory
exposed by `/api/hepta-memory-capability-absorption-inventory`.

Checks:

- old memory/capability/absorption ops file count remains `14`
- current `hepta-codex` script count is `15`
- native gateway source-command count is `62`
- surface count is `14`
- absorbed or represented surface count is `9`
- gap-report-ready count is `14`
- live mutation surface count is `0`
- memory store, capability registry, plugin registry, coding-agent spawn,
  search-provider live query, and skill-workshop writes remain disabled
- filesystem read/write, external network read, provider/model invocation,
  credential reads, channel read/send, native POST mutation, gateway mutation,
  and external send remain disabled
- memory/capability, local tooling/content, channel adapter, runtime/session,
  CLI inventory, provider inventory, and merge-completion reports agree on
  route/script counts

### `hepta-codex-release-hardening-status-gate.sh`

Purpose: validate the local-only release/hardening status gate exposed by
`/api/hepta-release-hardening-status-gate`.

Checks:

- remaining old release/hardening script family count is `12`
- current `hepta-codex` script count is `15`
- native gateway source-command count is `62`
- status gate count is `12`
- local status gate ready count is `12`
- live execution enabled count is `0`
- external-production gate count is `3`
- launchd mutation required count is `3`
- filesystem artifact write required count is `2`
- operator approval required count is `12`
- external production gates, artifact pack, launchd mutation, recurring
  watchdog install, local import execution, and autonomous subagent spawn remain
  disabled
- process spawn, filesystem read/write, artifact write, launchd mutation,
  watchdog service install, external network read/send, provider/model
  invocation, credential reads, Telegram owner handoff/read/send, native POST
  mutation, channel read/send, coding-agent spawn, and gateway mutation remain
  disabled
- release/hardening, memory/capability, local tooling/content, channel adapter,
  runtime/session, CLI inventory, provider inventory, and merge-completion
  reports agree on route/script counts

### `hepta-codex-provider-channel-dry-run-plan.sh`

Purpose: validate the local-only provider/channel/runtime dry-run plan exposed
by `/api/hepta-provider-channel-dry-run-plan`.

Checks:

- current `hepta-codex` script count is `15`
- native gateway source-command count is `62`
- `5/5` dry-run families are ready
- covered unique old ops file count is `43`
- provider/search/channel/runtime coverage is `15/3/13/12`
- live invocation, credential reads, provider prompt execution, search network
  query, channel delivery, runtime store mutation, and fixture materialization
  remain disabled
- provider/model invocation, credential reads, external network/search,
  channel read/send, Telegram owner handoff/read/send, process spawn,
  filesystem read/write, task/session store mutation, gateway event enqueue,
  native POST mutation, gateway mutation, and external send remain disabled
- provider/channel dry-run plan, release/hardening, memory/capability, local
  tooling/content, channel adapter, runtime/session, CLI inventory, provider
  inventory, and merge-completion reports agree on route/script counts

### `hepta-codex-public-ga-readiness.sh`

Purpose: validate the aggregate public GA readiness gate exposed by
`/api/hepta-public-ga-readiness`.

Checks:

- current `hepta-codex` script count is `15`
- native gateway source-command count is `62`
- local gate matrix and supporting reports are synchronized
- public GA is still blocked until explicit operator-approved live/external
  gates are satisfied
- release publishing, credential reads, provider/model invocation, channel
  read/send, Telegram handoff/read/send, native POST mutation, external network
  read, and external send remain disabled

### `hepta-codex-native-packaging-gate.sh`

Purpose: validate the local Hepta Native packaging readiness gate exposed by
`/api/hepta-native-packaging-gate`.

Checks:

- current `hepta-codex` script count is `15`
- native gateway source-command count is `62`
- `apps/hepta-native` source, manifest, lockfile, attribution docs, package
  metadata, macOS plist/entitlements, icon asset, and DMG helper are present
- packaging/resource count is `111`; Rust source count is `125`
- cargo manifest metadata parses; packaging helper scripts parse; macOS plist
  files lint
- optional heavy native check/test gate can be enabled with
  `HEPTA_NATIVE_PACKAGING_RUN_CARGO=1`
- signing, notarization, stapling, public artifact write, credentials, external
  network, provider/model invocation, channel delivery, Telegram handoff, native
  POST mutation, and gateway mutation remain disabled

## Updated Completion Impact

This slice improves script parity from 1/20 to 15/20 by count, and more
importantly turns twelve useful old operational intents into
`hepta-codex`-native commands:

- preflight
- live soak
- installed/live watchdog
- browser visual smoke
- CLI command breadth inventory
- provider/search metadata inventory
- runtime/task/session dry-run inventory
- channel adapter disabled status inventory
- local tooling/content planning inventory
- memory/capability absorption gap inventory
- release/hardening status gate
- provider/channel/runtime dry-run plan
- public GA readiness gate
- Hepta Native packaging readiness gate

It does not close the old CLI breadth gap, but it turns that gap and the first
provider/search/runtime-session/channel/local-tooling/memory-capability
families, plus the remaining release/hardening script family gap, into
machine-readable route and script gates. The next track is turning the highest
value remaining old CLI families into more specific planners or isolated
fixtures before any live handoff, process, filesystem, network, file-transfer,
or channel-delivery smoke.
