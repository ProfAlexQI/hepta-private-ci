# Hepta Merge Completion API Install

Date: 2026-05-20
Scope: install and live verification for `/api/hepta-merge-completion` and follow-on safe inventory routes
Status: installed; live route parity updated to `60/60`; external mutation gates remain closed

## Installed Build

- source commit: `1adebd2 feat: expose Hepta merge completion API`
- release binary: `codex-rs/target/release/hepta`
- installed binary: `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex`
- release sha256: `0fd4ed1d158dbf73e361ccb8975e0c580d4b482a2b754674d4ddcbfceb7ebb70`
- installed sha256: `0fd4ed1d158dbf73e361ccb8975e0c580d4b482a2b754674d4ddcbfceb7ebb70`
- binary sha match: `true`

Final Control UI continuation build:

- source commit: `12a3a46 feat: show merge completion on gateway index`
- release sha256: `55b08659d5d301ceb9fb6c33562330fde2482bd901eed6af4916ead73ae8c284`
- installed sha256: `55b08659d5d301ceb9fb6c33562330fde2482bd901eed6af4916ead73ae8c284`
- binary sha match: `true`

Browser visual smoke continuation build:

- source workset: browser visual smoke script plus merge-completion API blocker
  refresh after `e86d170 docs: record Hepta merge completion UI install`
- release sha256: `833819d99190d9a62212237626ad5c96f491e0221571e46d81b2975e316a4844`
- installed sha256: `833819d99190d9a62212237626ad5c96f491e0221571e46d81b2975e316a4844`
- binary sha match: `true`

CLI command inventory continuation:

- source workset: `/api/hepta-cli-command-inventory`,
  `scripts/hepta-codex-cli-command-inventory.sh`, and inventory docs
- release sha256: `ba93dc401a48728ec3d0a16d8d671639751fdaad546fe5c5889d45211576b7d0`
- installed sha256: `ba93dc401a48728ec3d0a16d8d671639751fdaad546fe5c5889d45211576b7d0`
- binary sha match: `true`
- live route parity after install: `53/53`
- script count after install: `6`

Provider metadata inventory continuation:

- source workset: `/api/hepta-provider-metadata-inventory`,
  `scripts/hepta-codex-provider-metadata-inventory.sh`, and inventory docs
- release sha256: `6a428f61d0cedfb803ffec324b3fc4a4fc09025ae4731afad9d7b346a883ec5a`
- installed sha256: `6a428f61d0cedfb803ffec324b3fc4a4fc09025ae4731afad9d7b346a883ec5a`
- binary sha match: `true`
- live route parity after install: `54/54`
- script count after install: `7`

Runtime/session dry-run inventory continuation:

- source workset: `/api/hepta-runtime-session-dry-run-inventory`,
  `scripts/hepta-codex-runtime-session-dry-run-inventory.sh`, and inventory docs
- source commit: `cfb4c5f feat: add Hepta runtime dry-run inventory`
- release sha256: `480a40c62d05d821d6aa774240ec97cfe1db4b46865ddab922f4b7be4deba9f2`
- installed sha256: `480a40c62d05d821d6aa774240ec97cfe1db4b46865ddab922f4b7be4deba9f2`
- binary sha match: `true`
- live route parity after install: `55/55`
- script count after install: `8`

Channel adapter disabled status inventory continuation:

- source workset: `/api/hepta-channel-adapter-status-inventory`,
  `scripts/hepta-codex-channel-adapter-status-inventory.sh`, and inventory docs
- source commit: `acc3c49 feat: add Hepta channel status inventory`
- release sha256: `9bf2d1163e636f37e8802cb75d4f05e5282c87576aee200d80d60ad59fdc9cec`
- installed sha256: `9bf2d1163e636f37e8802cb75d4f05e5282c87576aee200d80d60ad59fdc9cec`
- binary sha match: `true`
- live route parity after install: `56/56`
- script count after install: `9`
- native gateway source command count after install: `56`

Local tooling/content planning inventory continuation:

- source workset: `/api/hepta-local-tooling-content-inventory`,
  `scripts/hepta-codex-local-tooling-content-inventory.sh`, and inventory docs
- source commit: `47de5a2 feat: add Hepta local tooling inventory`
- release sha256: `55e08841c8e5ae761037f6792c938b217e8ce9b85590da1541ffa03015577323`
- installed sha256: `55e08841c8e5ae761037f6792c938b217e8ce9b85590da1541ffa03015577323`
- binary sha match: `true`
- live route parity after install: `57/57`
- script count after install: `10`
- native gateway source command count after install: `57`

Memory/capability absorption gap inventory continuation:

- source workset: `/api/hepta-memory-capability-absorption-inventory`,
  `scripts/hepta-codex-memory-capability-inventory.sh`, and inventory docs
- source commit: `1a0bb8d feat: add Hepta memory capability inventory`
- release sha256: `219c5186d39902d6b0dd4a66ce4a7f7bb0da8e4fa8164bb669bb8732fc27602e`
- installed sha256: `219c5186d39902d6b0dd4a66ce4a7f7bb0da8e4fa8164bb669bb8732fc27602e`
- binary sha match: `true`
- live route parity after install: `58/58`
- script count after install: `11`
- native gateway source command count after install: `58`

Release/hardening status gate continuation:

- source workset: `/api/hepta-release-hardening-status-gate`,
  `scripts/hepta-codex-release-hardening-status-gate.sh`, and status gate docs
- source commit: `e3c25ac feat: add Hepta release hardening status gate`
- release sha256: `6e594511129eb3ac6b5332685d9162799ec619769b217a00098f36d11883c42f`
- installed sha256: `6e594511129eb3ac6b5332685d9162799ec619769b217a00098f36d11883c42f`
- binary sha match: `true`
- live route parity after install: `59/59`
- script count after install: `12`
- native gateway source command count after install: `59`
- release/hardening status gate count: `12`
- local status gate ready count: `12`
- live execution enabled count: `0`
- external-production gate count: `3`
- launchd mutation required count: `3`
- filesystem artifact write required count: `2`
- operator approval required count: `12`
- side-effect gates: process spawn, filesystem read/write, release artifact
  write, launchd mutation, watchdog service install, external network
  read/send, provider/model invocation, credential reads, Telegram owner
  handoff/read/send, native POST mutation, channel read/send, coding-agent
  spawn, and gateway mutation all `false`

Provider/channel/runtime dry-run plan continuation:

- source workset: `/api/hepta-provider-channel-dry-run-plan`,
  `scripts/hepta-codex-provider-channel-dry-run-plan.sh`, and dry-run plan docs
- source commit: `0334ac2 feat: add Hepta provider channel dry-run plan`
- release sha256: `a6a72e499f39cd94e80df66f6e50e9e3d2146dc7edf8be9335d9611909fcfa18`
- installed sha256: `a6a72e499f39cd94e80df66f6e50e9e3d2146dc7edf8be9335d9611909fcfa18`
- binary sha match: `true`
- live route parity after install: `60/60`
- script count after install: `13`
- native gateway source command count after install: `60`
- dry-run family count: `5`
- unique covered old ops file count: `43`
- provider/search/channel/runtime coverage: `15/3/13/12`
- dry-run plan ready count: `5`
- isolated fixture contract count: `5`
- live invocation enabled count: `0`
- credential-read-required count: `0`
- side-effect gates: provider/model invocation, credential reads, external
  network/search, channel read/send, Telegram owner handoff/read/send, process
  spawn, filesystem read/write, task/session store mutation, gateway event
  enqueue, native POST mutation, gateway mutation, and external send all `false`

Backups created before replacement:

- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-merge-completion-api-20260520-093102`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-merge-completion-api-20260520-093113`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-control-ui-merge-completion-20260520-095847`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-gateway-index-merge-completion-20260520-101433`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-browser-visual-smoke-20260520-110537`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-cli-command-inventory-20260520-115611`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-provider-metadata-inventory-20260520-124444`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-provider-channel-dry-run-plan-20260520-174908`
- `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-provider-channel-dry-run-plan-20260520-174908.plist`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-runtime-session-dry-run-inventory-20260520-133107`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-channel-adapter-status-inventory-20260520-141937`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-local-tooling-content-inventory-20260520-150740`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-memory-capability-inventory-20260520-155424`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-release-hardening-status-gate-20260520-165601`

Launchd plist backup created before the browser visual smoke continuation
replacement:

- `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-browser-visual-smoke-20260520-110537.plist`

Launchd plist backup created before the CLI command inventory continuation
replacement:

- `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-cli-command-inventory-20260520-115611.plist`

Launchd plist backup created before the provider metadata inventory continuation
replacement:

- `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-provider-metadata-inventory-20260520-124444.plist`

Launchd plist backup created before the runtime/session dry-run inventory
continuation replacement:

- `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-runtime-session-dry-run-inventory-20260520-133107.plist`

Launchd plist backup created before the channel adapter status inventory
continuation replacement:

- `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-channel-adapter-status-inventory-20260520-141937.plist`

Launchd plist backup created before the local tooling/content inventory
continuation replacement:

- `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-local-tooling-content-inventory-20260520-150740.plist`

Launchd plist backup created before the memory/capability inventory continuation
replacement:

- `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-memory-capability-inventory-20260520-155424.plist`

Launchd plist backup created before the release/hardening status gate
continuation replacement:

- `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-release-hardening-status-gate-20260520-165601.plist`

The first backup was created during a no-op install attempt that used the wrong
release path (`target/release/hepta`). The installed SHA did not change during
that attempt. The second backup was created immediately before replacing the
binary with `codex-rs/target/release/hepta`.

## Live Endpoint Smoke

`/api/hepta-merge-completion` now returns:

- `status=attention`
- `runtime=hepta-codex`
- `source_package_merge_percent=82`
- `local_deterministic_function_percent=91`
- `active_service_coexistence_percent=88`
- `production_replacement_percent=68`
- `native_gateway_source_command_count=60` after provider/channel dry-run plan continuation
- `current_hepta_codex_script_total=13` after provider/channel dry-run plan continuation
- `merge_completion_control_ui_surfaced=true`
- `merge_completion_gateway_index_surfaced=true`
- `browser_visual_smoke_ready=true`
- `browser_visual_smoke_command=scripts/hepta-codex-browser-visual-smoke.sh`
- `route_count=60` after provider/channel dry-run plan continuation
- `missing_route_count=0`
- `telegram_live_send_enabled=false`
- `native_post_real_activation_enabled=false`
- `public_ga_claimed=false`

The gateway index page now also exposes the completion signal directly:

- label: `Merge completion`
- value: `82 / 91 / 88 / 68`
- explanatory endpoint text includes `/api/hepta-merge-completion`

Expected blockers remain explicit:

- `telegram_owner_handoff_not_requested`
- `live_poll_send_not_operator_approved`
- `native_post_real_activation_not_operator_approved`
- `old_hepta_cli_command_breadth_not_fully_migrated`
- `old_hepta_release_external_scripts_not_fully_ported`

The earlier browser-visual audit blocker was closed by
`scripts/hepta-codex-browser-visual-smoke.sh`, which captures desktop and mobile
Chrome headless screenshots of the installed gateway index and checks
`/api/hepta-merge-completion` remains route-ready with Telegram send and native
POST real activation disabled.

The old CLI breadth inventory is now exposed by
`/api/hepta-cli-command-inventory` and validated by
`scripts/hepta-codex-cli-command-inventory.sh`:

- `old_hepta_ops_file_count=65`
- `old_hepta_rough_command_reference_count=574`
- `old_hepta_script_total=20`
- `current_hepta_codex_script_total=12`
- `native_gateway_source_command_count=59`
- `ops_family_count=5`
- `ops_file_family_covered_count=65`
- `old_cli_command_breadth_fully_migrated=false`
- `safe_read_only_inventory_ready=true`
- provider invocation, credential reads, Telegram reads/sends, native POST real
  mutation, external network reads, and filesystem writes all `false`

The provider/search metadata inventory continuation adds
`/api/hepta-provider-metadata-inventory` and
`scripts/hepta-codex-provider-metadata-inventory.sh`:

- `old_provider_ops_file_count=15`
- `adjacent_search_ops_file_count=3`
- `provider_adapter_count=15`
- `adjacent_search_adapter_count=3`
- `current_hepta_codex_script_total=12`
- `native_gateway_source_command_count=59`
- `provider_live_invocation_enabled=false`
- `credentialed_smoke_performed=false`
- provider invocation, credential reads, external network reads, model calls,
  Telegram reads/sends, native POST mutation, and filesystem writes all `false`

The runtime/session dry-run inventory continuation adds
`/api/hepta-runtime-session-dry-run-inventory` and
`scripts/hepta-codex-runtime-session-dry-run-inventory.sh`:

- `old_runtime_ops_file_count=12`
- `dry_run_surface_count=12`
- `planner_ready_count=12`
- `live_mutation_surface_count=0`
- `current_hepta_codex_script_total=12`
- `native_gateway_source_command_count=59`
- task registry mutation, session store mutation, gateway event enqueue, hook
  enqueue, process spawn, provider/model invocation, credential reads, external
  network/send, Telegram reads/sends, native POST mutation, and filesystem
  writes all `false`

The channel adapter disabled status inventory continuation adds
`/api/hepta-channel-adapter-status-inventory` and
`scripts/hepta-codex-channel-adapter-status-inventory.sh`:

- `old_channel_ops_file_count=13`
- `adapter_count=13`
- `disabled_status_ready_count=13`
- `live_adapter_enabled_count=0`
- `current_hepta_codex_script_total=12`
- `native_gateway_source_command_count=59`
- live channel read, live channel send, Telegram owner handoff, credential
  reads, external network/send, voice calls, TTS playback, webhook delivery,
  file transfer, native POST mutation, and filesystem writes all `false`

The local tooling/content planning inventory continuation adds
`/api/hepta-local-tooling-content-inventory` and
`scripts/hepta-codex-local-tooling-content-inventory.sh`:

- `old_local_tooling_ops_file_count=11`
- `surface_count=11`
- `planner_ready_count=11`
- `live_process_enabled_count=0`
- `filesystem_touch_enabled_count=0`
- `network_read_enabled_count=0`
- `tool_invocation_enabled_count=0`
- `current_hepta_codex_script_total=12`
- `native_gateway_source_command_count=59`
- process spawn, filesystem read/write, network read, tool invocation,
  provider/model invocation, credential reads, channel read/send, gateway
  mutation, native POST mutation, and external send all `false`

The memory/capability absorption gap inventory continuation adds
`/api/hepta-memory-capability-absorption-inventory` and
`scripts/hepta-codex-memory-capability-inventory.sh`:

- `old_memory_capability_ops_file_count=14`
- `surface_count=14`
- `absorbed_or_represented_count=11`
- `gap_report_ready_count=14`
- `live_mutation_enabled_count=0`
- `current_hepta_codex_script_total=12`
- `native_gateway_source_command_count=59`
- memory store mutation, capability registry mutation, plugin registry
  mutation, coding-agent spawn, skill-workshop write, filesystem read/write,
  external network read, provider/model invocation, credential reads, channel
  read/send, native POST mutation, gateway mutation, and external send all
  `false`

The release/hardening status gate continuation adds
`/api/hepta-release-hardening-status-gate` and
`scripts/hepta-codex-release-hardening-status-gate.sh`:

- `old_release_hardening_script_family_count=12`
- `status_gate_count=12`
- `local_status_gate_ready_count=12`
- `live_execution_enabled_count=0`
- `external_production_gate_count=3`
- `launchd_mutation_required_count=3`
- `filesystem_artifact_write_required_count=2`
- `operator_approval_required_count=12`
- `current_hepta_codex_script_total=12`
- `native_gateway_source_command_count=59`
- process spawn, filesystem read/write, release artifact write, launchd
  mutation, watchdog service install, external network read/send,
  provider/model invocation, credential reads, Telegram owner handoff/read/send,
  native POST mutation, channel read/send, coding-agent spawn, and gateway
  mutation all `false`

The provider/channel/runtime dry-run plan continuation adds
`/api/hepta-provider-channel-dry-run-plan` and
`scripts/hepta-codex-provider-channel-dry-run-plan.sh`:

- `plan_family_count=5`
- `covered_old_ops_file_count=43`
- `covered_provider_ops_file_count=15`
- `covered_search_ops_file_count=3`
- `covered_channel_ops_file_count=13`
- `covered_runtime_ops_file_count=12`
- `dry_run_plan_ready_count=5`
- `isolated_fixture_contract_count=5`
- `live_invocation_enabled_count=0`
- `credential_read_required_count=0`
- `current_hepta_codex_script_total=13`
- `native_gateway_source_command_count=60`
- provider/model invocation, credential reads, external network/search,
  channel read/send, Telegram owner handoff/read/send, process spawn,
  filesystem read/write, task/session store mutation, gateway event enqueue,
  native POST mutation, gateway mutation, and external send all `false`

## Route Parity

`/api/control-ui-route-parity` after install:

- `status=ready`
- `route_count=60` after provider/channel dry-run plan continuation
- `implemented_route_count=60` after provider/channel dry-run plan continuation
- `missing_route_count=0`

## Safety Boundary

The install did not perform Telegram owner handoff and did not enable live send
or native POST real handlers.

Live checks after install:

- `/api/telegram-owner-handoff`: `status=legacy_owner`, `active_owner=legacy_openclaw`, `hepta_poll_loop_armed=false`, `double_poller_risk=false`
- `/api/operator-security`: `status=attention`, `security_mode=legacy_owner_coexistence_ready`, `legacy_owner_coexistence_ready=true`, `attention_reason=telegram_replacement_not_requested`

## Verification

Pre-install gates:

- `cargo fmt --manifest-path codex-rs/Cargo.toml --all --check`
- `cargo check --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta`
- `cargo test --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta hepta_merge_completion_endpoint_returns_machine_readable_audit -- --nocapture`
- `cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta hepta_cli_command_inventory -- --nocapture`
- `cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta hepta_provider_metadata_inventory -- --nocapture`
- `cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta hepta_runtime_session_dry_run_inventory -- --nocapture`
- `cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta hepta_channel_adapter_status_inventory -- --nocapture`
- `cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta hepta_local_tooling_content_inventory -- --nocapture`
- `cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta hepta_memory_capability_absorption_inventory -- --nocapture`
- `cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli native_gateway::tests::hepta_release_hardening_status_gate_endpoint_is_local_only -- --nocapture`
- `cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta inventory -- --nocapture`
- `cargo test --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta control_ui_route_parity_report_covers_old_hepta_routes -- --nocapture`
- `cargo test --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture`: `63 passed`
- `HEPTA_CODEX_PREFLIGHT_RELEASE=0 scripts/hepta-codex-preflight.sh`
- `CARGO_INCREMENTAL=0 cargo build --release --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta`
- `scripts/hepta-control-ui-smoke.sh`: passed after surfacing merge completion in the Rust/no-JS Control UI model
- `cargo test --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway_readiness_exposes_pending_telegram_migration -- --nocapture`: passed after adding the gateway index completion card
- `scripts/hepta-codex-browser-visual-smoke.sh`: passed; desktop and mobile
  screenshot evidence written under a temporary directory outside the repo

Post-install gates:

- `scripts/hepta-codex-watchdog.sh`: passed
- `scripts/hepta-codex-live-soak.sh`: `12/12` samples passed
- `scripts/hepta-codex-cli-command-inventory.sh`: passed, merge-completion and
  CLI inventory counts synchronized
- `scripts/hepta-codex-provider-metadata-inventory.sh`: passed, provider, CLI,
  and merge-completion counts synchronized
- `scripts/hepta-codex-runtime-session-dry-run-inventory.sh`: passed,
  runtime/session, provider, CLI, and merge-completion counts synchronized
- `scripts/hepta-codex-channel-adapter-status-inventory.sh`: passed,
  channel adapter, runtime/session, provider, CLI, and merge-completion counts
  synchronized
- `scripts/hepta-codex-local-tooling-content-inventory.sh`: passed,
  local tooling/content, channel adapter, runtime/session, provider, CLI, and
  merge-completion counts synchronized
- `scripts/hepta-codex-memory-capability-inventory.sh`: passed,
  memory/capability, local tooling/content, channel adapter, runtime/session,
  provider, CLI, and merge-completion counts synchronized
- `scripts/hepta-codex-release-hardening-status-gate.sh`: passed,
  release/hardening, memory/capability, local tooling/content, channel adapter,
  runtime/session, provider, CLI, and merge-completion counts synchronized
- gateway index smoke: found `Merge completion`, `82 / 91 / 88 / 68`, and `/api/hepta-merge-completion`
- browser visual smoke after final install:
  `scripts/hepta-codex-browser-visual-smoke.sh` passed and wrote screenshot
  evidence under `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.MNEhKs`
- live `/api/hepta-merge-completion` reports thirteen current scripts after
  provider/channel dry-run plan continuation, browser visual smoke
  ready, route parity `60/60`, and no
  `browser_visual_e2e_not_run_in_this_audit` blocker
- provider metadata browser visual smoke after provider install:
  `scripts/hepta-codex-browser-visual-smoke.sh` passed and wrote screenshot
  evidence under `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.pXuv34`
- short live soak after provider install:
  `HEPTA_CODEX_SOAK_SAMPLES=3 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=2 scripts/hepta-codex-live-soak.sh`
  passed `3/3`
- runtime/session browser visual smoke after install:
  `scripts/hepta-codex-browser-visual-smoke.sh` passed and wrote screenshot
  evidence under `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.ufsv9m`
- short live soak after runtime/session install:
  `HEPTA_CODEX_SOAK_SAMPLES=3 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=2 scripts/hepta-codex-live-soak.sh`
  passed `3/3`
- channel adapter browser visual smoke after install:
  `scripts/hepta-codex-browser-visual-smoke.sh` passed and wrote screenshot
  evidence under `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.mJTOvv`
- short live soak after channel adapter install:
  `HEPTA_CODEX_SOAK_SAMPLES=3 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=2 scripts/hepta-codex-live-soak.sh`
  passed `3/3`
- local tooling/content browser visual smoke after install:
  `scripts/hepta-codex-browser-visual-smoke.sh` passed and wrote screenshot
  evidence under `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.sUpExr`
- short live soak after local tooling/content install:
  `HEPTA_CODEX_SOAK_SAMPLES=3 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=2 scripts/hepta-codex-live-soak.sh`
  passed `3/3`
- memory/capability browser visual smoke after install:
  `scripts/hepta-codex-browser-visual-smoke.sh` passed and wrote screenshot
  evidence under `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.B0maP1`
- short live soak after memory/capability install:
  `HEPTA_CODEX_SOAK_SAMPLES=3 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=2 scripts/hepta-codex-live-soak.sh`
  passed `3/3`
- release/hardening browser visual smoke after install:
  `scripts/hepta-codex-browser-visual-smoke.sh` passed and wrote screenshot
  evidence under `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.lk5VW6`
- short live soak after release/hardening install:
  `HEPTA_SOAK_ITERATIONS=3 HEPTA_SOAK_SLEEP_SECONDS=1 scripts/hepta-codex-live-soak.sh`
  passed

Known non-blocking warnings:

- stable `rustfmt` still warns that `imports_granularity = Item` is nightly-only
- Makepad metadata still reports a duplicate `bitflags` package and chooses the
  non-vulkan path

## Public GA Readiness Gate Continuation

The public GA readiness continuation adds
`/api/hepta-public-ga-readiness` and
`scripts/hepta-codex-public-ga-readiness.sh`.

Installed build:

- release sha256: `944b3d6006894bbd6cf0ca4e4eb51b392de655f70f8befd8d2537dd6b69a7a53`
- installed sha256: `944b3d6006894bbd6cf0ca4e4eb51b392de655f70f8befd8d2537dd6b69a7a53`
- binary SHA match: `true`
- binary backup:
  `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-public-ga-readiness-20260521-001359`
- plist backup:
  `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-public-ga-readiness-20260521-001359.plist`

Live results after install:

- `/api/control-ui-route-parity`: `61/61`, missing `0`
- `/api/hepta-public-ga-readiness`: `status=blocked`,
  `public_ga_ready=false`, `local_gate_matrix_ready=true`,
  `local_reports_synchronized=true`, blocker count `11`
- `scripts/hepta-codex-public-ga-readiness.sh`: passed
- all inventory/status scripts passed with
  `current_hepta_codex_script_total=14` and
  `native_gateway_source_command_count=61`
- `scripts/hepta-codex-watchdog.sh`: passed, SHA match true
- `HEPTA_CODEX_SOAK_SAMPLES=3 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=2 scripts/hepta-codex-live-soak.sh`: passed `3/3`
- `scripts/hepta-codex-browser-visual-smoke.sh`: passed, screenshots in
  `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.AbyfwW`

Safety state remained unchanged: active Telegram owner is still
`legacy_openclaw`, Hepta Telegram polling is gated, native POST real activation
is disabled, and no external public release was performed.

## Native Packaging Gate Continuation

The native packaging continuation adds `/api/hepta-native-packaging-gate` and
`scripts/hepta-codex-native-packaging-gate.sh`.

Installed build:

- release sha256: `c5814b0cad696a1b1e03c28ac2e1bcd311aa70b874ae2ad74c9a7016d4245a15`
- installed sha256: `c5814b0cad696a1b1e03c28ac2e1bcd311aa70b874ae2ad74c9a7016d4245a15`
- binary SHA match: `true`
- active service: `ai.hepta.gateway`
- active PID after restart: `15580`
- binary backup:
  `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-native-packaging-count-fix-20260521-012336`
- plist backup:
  `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-native-packaging-count-fix-20260521-012336.plist`

Live results after install:

- `/api/control-ui-route-parity`: `62/62`, missing `0`
- `/api/hepta-native-packaging-gate`: `status=ready`,
  `local_packaging_gate_ready=true`, Rust source files `125`,
  packaging/resource files `111`
- `/api/hepta-public-ga-readiness`: `status=blocked`,
  `public_ga_ready=false`, `local_gate_matrix_ready=true`,
  `local_reports_synchronized=true`, blocker count `10`,
  `hepta_native_release_packaging_ready=true`
- `scripts/hepta-codex-native-packaging-gate.sh`: passed
- `scripts/hepta-codex-public-ga-readiness.sh`: passed
- all inventory/status scripts passed with
  `current_hepta_codex_script_total=15` and
  `native_gateway_source_command_count=62`
- `scripts/hepta-codex-watchdog.sh`: passed, SHA match true
- `HEPTA_CODEX_SOAK_SAMPLES=3 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=2 scripts/hepta-codex-live-soak.sh`: passed `3/3`
- `scripts/hepta-codex-browser-visual-smoke.sh`: passed, screenshots in
  `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.vhMbc2`

Verification before install included:

- `cargo fmt --all --manifest-path codex-rs/Cargo.toml`
- `CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta`
- targeted native-packaging and public-GA readiness tests
- `CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture`: `66 passed`
- `HEPTA_CODEX_PREFLIGHT_RELEASE=0 CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 scripts/hepta-codex-preflight.sh`, including Hepta Native `52` tests
- `CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo build --offline --manifest-path codex-rs/Cargo.toml --release -p codex-cli --bin hepta`

Safety state remained unchanged: active Telegram owner is still
`legacy_openclaw`, Hepta Telegram polling is gated, native POST real activation
is disabled, and no provider/channel live smoke, native app signing,
notarization, artifact publishing, or external public release was performed.

## Public GA Operator Approval Packet Continuation

The operator approval packet continuation adds
`/api/hepta-public-ga-operator-approval-packet` and
`scripts/hepta-codex-public-ga-operator-approval-packet.sh`.

Installed build:

- release sha256: `6de62f43ac4d3432207441bb85bf1713118a0bade42cdb2a3da25ac85fb9d96e`
- installed sha256: `6de62f43ac4d3432207441bb85bf1713118a0bade42cdb2a3da25ac85fb9d96e`
- binary SHA match: `true`
- active service: `ai.hepta.gateway`
- active PID after restart: `25970`
- binary backup:
  `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-public-ga-operator-approval-packet-20260521-023824`
- plist backup:
  `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-public-ga-operator-approval-packet-20260521-023824.plist`

Live results after install:

- `/api/control-ui-route-parity`: `64/64`, missing `0`
- `/api/hepta-public-ga-operator-approval-packet`: `status=ready`,
  `approval_packet_ready=true`, safe default mode `plan_only_no_live_mutation`
- `/api/hepta-public-ga-readiness`: `status=blocked`,
  `public_ga_ready=false`, `local_gate_matrix_ready=true`,
  `local_reports_synchronized=true`, blocker count `8`
- `scripts/hepta-codex-public-ga-operator-approval-packet.sh`: passed
- `scripts/hepta-codex-public-ga-readiness.sh`: passed
- all inventory/status scripts passed with
  `current_hepta_codex_script_total=17` and
  `native_gateway_source_command_count=64`
- `scripts/hepta-codex-watchdog.sh`: passed, SHA match true
- `HEPTA_CODEX_SOAK_SAMPLES=12 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=5 scripts/hepta-codex-live-soak.sh`: passed `12/12`
- `scripts/hepta-codex-browser-visual-smoke.sh`: passed, screenshots in
  `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.PkGkb0`

Verification before install included:

- `cargo fmt --all --manifest-path codex-rs/Cargo.toml`
- `CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta`
- targeted operator-approval-packet and public-GA readiness tests
- `CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture`: `68 passed`
- `CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo build --offline --manifest-path codex-rs/Cargo.toml --release -p codex-cli --bin hepta`
- `HEPTA_CODEX_PREFLIGHT_RELEASE=0 scripts/hepta-codex-preflight.sh`,
  including Hepta Native `52` tests

Safety state remained unchanged: active Telegram owner is still
`legacy_openclaw`, Hepta Telegram polling is gated, native POST real activation
is disabled, and no provider/channel live smoke, native app signing,
notarization, artifact publishing, or external public release was performed.

## Legacy Compatibility Closure Continuation

The legacy compatibility closure continuation adds
`/api/hepta-legacy-compatibility-closure` and
`scripts/hepta-codex-legacy-compatibility-closure.sh`.

Installed build:

- release sha256: `f01d17324f1c4ecf885cd3d9380d888ba5c46edc6986d159168040b09b4b3bf8`
- installed sha256: `f01d17324f1c4ecf885cd3d9380d888ba5c46edc6986d159168040b09b4b3bf8`
- binary SHA match: `true`
- active service: `ai.hepta.gateway`
- active PID after restart: `19635`
- binary backup:
  `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-legacy-compatibility-closure-20260521-015538`
- plist backup:
  `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-legacy-compatibility-closure-20260521-015538.plist`

Live results after install:

- `/api/control-ui-route-parity`: `63/63`, missing `0`
- `/api/hepta-legacy-compatibility-closure`: `status=ready`,
  old ops coverage `65/65`, release/hardening status gates `12/12`
- `/api/hepta-public-ga-readiness`: `status=blocked`,
  `public_ga_ready=false`, `local_gate_matrix_ready=true`,
  `local_reports_synchronized=true`, blocker count `8`
- `scripts/hepta-codex-legacy-compatibility-closure.sh`: passed
- `scripts/hepta-codex-public-ga-readiness.sh`: passed
- all inventory/status scripts passed with
  `current_hepta_codex_script_total=16` and
  `native_gateway_source_command_count=63`
- `scripts/hepta-codex-watchdog.sh`: passed, SHA match true
- `HEPTA_CODEX_SOAK_SAMPLES=12 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=5 scripts/hepta-codex-live-soak.sh`: passed `12/12`
- `scripts/hepta-codex-browser-visual-smoke.sh`: passed, screenshots in
  `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.IUfj3T`

Verification before install included:

- `cargo fmt --all --manifest-path codex-rs/Cargo.toml`
- `CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta`
- targeted legacy-compatibility and public-GA readiness tests
- `CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture`: `67 passed`
- `CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo build --offline --manifest-path codex-rs/Cargo.toml --release -p codex-cli --bin hepta`
- `HEPTA_CODEX_PREFLIGHT_RELEASE=0 scripts/hepta-codex-preflight.sh`,
  including Hepta Native `52` tests

Safety state remained unchanged: active Telegram owner is still
`legacy_openclaw`, Hepta Telegram polling is gated, native POST real activation
is disabled, and no provider/channel live smoke, native app signing,
notarization, artifact publishing, or external public release was performed.
