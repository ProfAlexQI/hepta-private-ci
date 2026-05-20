# Hepta Merge Completion API Install

Date: 2026-05-20
Scope: install and live verification for `/api/hepta-merge-completion` and follow-on safe inventory routes
Status: installed; live route parity updated to `53/53`; external mutation gates remain closed

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

Backups created before replacement:

- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-merge-completion-api-20260520-093102`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-merge-completion-api-20260520-093113`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-control-ui-merge-completion-20260520-095847`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-gateway-index-merge-completion-20260520-101433`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-browser-visual-smoke-20260520-110537`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-cli-command-inventory-20260520-115611`

Launchd plist backup created before the browser visual smoke continuation
replacement:

- `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-browser-visual-smoke-20260520-110537.plist`

Launchd plist backup created before the CLI command inventory continuation
replacement:

- `/Users/qianqi/.openclaw/workspace/backups/ai.hepta.gateway.pre-cli-command-inventory-20260520-115611.plist`

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
- `native_gateway_source_command_count=53` after CLI inventory continuation
- `current_hepta_codex_script_total=6` after CLI inventory continuation
- `merge_completion_control_ui_surfaced=true`
- `merge_completion_gateway_index_surfaced=true`
- `browser_visual_smoke_ready=true`
- `browser_visual_smoke_command=scripts/hepta-codex-browser-visual-smoke.sh`
- `route_count=53` after CLI inventory continuation
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
- `current_hepta_codex_script_total=6`
- `native_gateway_source_command_count=53`
- `ops_family_count=5`
- `ops_file_family_covered_count=65`
- `old_cli_command_breadth_fully_migrated=false`
- `safe_read_only_inventory_ready=true`
- provider invocation, credential reads, Telegram reads/sends, native POST real
  mutation, external network reads, and filesystem writes all `false`

## Route Parity

`/api/control-ui-route-parity` after install:

- `status=ready`
- `route_count=53` after CLI inventory continuation
- `implemented_route_count=53` after CLI inventory continuation
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
- `cargo test --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta control_ui_route_parity_report_covers_old_hepta_routes -- --nocapture`
- `cargo test --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture`: `57 passed`
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
- gateway index smoke: found `Merge completion`, `82 / 91 / 88 / 68`, and `/api/hepta-merge-completion`
- browser visual smoke after final install:
  `scripts/hepta-codex-browser-visual-smoke.sh` passed and wrote screenshot
  evidence under `/Users/qianqi/.openclaw/tmp/hepta-codex-browser-visual-smoke.Sq2URq`
- live `/api/hepta-merge-completion` reports six current scripts, browser
  visual smoke ready, route parity `53/53`, and no
  `browser_visual_e2e_not_run_in_this_audit` blocker

Known non-blocking warnings:

- stable `rustfmt` still warns that `imports_granularity = Item` is nightly-only
- Makepad metadata still reports a duplicate `bitflags` package and chooses the
  non-vulkan path
