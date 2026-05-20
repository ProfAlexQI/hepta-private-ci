# Hepta Merge Completion API Install

Date: 2026-05-20
Scope: install and live verification for `/api/hepta-merge-completion`
Status: installed; live route parity updated to `52/52`; external mutation gates remain closed

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

Backups created before replacement:

- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-merge-completion-api-20260520-093102`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-merge-completion-api-20260520-093113`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-control-ui-merge-completion-20260520-095847`
- `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex.pre-gateway-index-merge-completion-20260520-101433`

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
- `native_gateway_source_command_count=52`
- `route_count=52`
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
- `browser_visual_e2e_not_run_in_this_audit`

## Route Parity

`/api/control-ui-route-parity` after install:

- `status=ready`
- `route_count=52`
- `implemented_route_count=52`
- `missing_route_count=0`

## Safety Boundary

The install did not perform Telegram owner handoff and did not enable live send
or native POST real handlers.

Live checks after install:

- `/api/telegram-owner-handoff`: `status=legacy_owner`, `active_owner=legacy_openclaw`, `hepta_poll_loop_armed=false`, `double_poller_risk=false`
- `/api/native-post-activation-plan`: `status=ready`, `activation_currently_enabled=false`, `activation_blocked_reason=real_handler_gate_disabled`, `real_mutation_performed=false`, `external_side_effects=false`
- `/api/operator-security`: `status=attention`, `security_mode=legacy_owner_coexistence_ready`, `legacy_owner_coexistence_ready=true`, `attention_reason=telegram_replacement_not_requested`

## Verification

Pre-install gates:

- `cargo fmt --manifest-path codex-rs/Cargo.toml --all --check`
- `cargo check --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta`
- `cargo test --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta hepta_merge_completion_endpoint_returns_machine_readable_audit -- --nocapture`
- `cargo test --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta control_ui_route_parity_report_covers_old_hepta_routes -- --nocapture`
- `cargo test --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture`: `56 passed`
- `HEPTA_CODEX_PREFLIGHT_RELEASE=0 scripts/hepta-codex-preflight.sh`
- `CARGO_INCREMENTAL=0 cargo build --release --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta`
- `scripts/hepta-control-ui-smoke.sh`: passed after surfacing merge completion in the Rust/no-JS Control UI model
- `cargo test --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway_readiness_exposes_pending_telegram_migration -- --nocapture`: passed after adding the gateway index completion card

Post-install gates:

- `scripts/hepta-codex-watchdog.sh`: passed
- `HEPTA_CODEX_SOAK_SAMPLES=3 HEPTA_CODEX_SOAK_INTERVAL_SECONDS=2 scripts/hepta-codex-live-soak.sh`: `3/3` samples passed
- gateway index smoke: found `Merge completion`, `82 / 91 / 88 / 68`, and `/api/hepta-merge-completion`

Known non-blocking warnings:

- stable `rustfmt` still warns that `imports_granularity = Item` is nightly-only
- Makepad metadata still reports a duplicate `bitflags` package and chooses the
  non-vulkan path
