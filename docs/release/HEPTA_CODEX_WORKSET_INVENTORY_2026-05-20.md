# Hepta Codex Workset Inventory

Date: 2026-05-20
Scope: dirty/untracked workset packaging plan for `/Users/qianqi/.openclaw/workspace/hepta-codex`
Status: inventory complete; no install, deploy, live Telegram/POST/Matrix activation, or owner change performed

## Current Worktree

Tracked modified files:

- `codex-rs/Cargo.lock`
- `codex-rs/Cargo.toml`
- `codex-rs/cli/Cargo.toml`
- `codex-rs/cli/src/native_gateway.rs`
- `codex-rs/cli/src/native_telegram.rs`

Tracked diff shortstat:

- 5 files changed
- 829 insertions
- 8238 deletions

Untracked file count: 367 files.

Untracked groups:

- `apps/hepta-native`: 187 files
- `codex-rs/hepta-runtime`: 53 files
- `codex-rs/hepta-core`: 43 files
- `codex-rs/hepta-gateway`: 22 files
- `codex-rs/hepta-intelligence`: 16 files
- `apps/hepta-control-ui`: 14 files
- `codex-rs/hepta-plugins`: 13 files
- `docs/release`: 6 files
- `docs/architecture`: 4 files
- `codex-rs/hepta-memory`: 2 files
- `scripts/hepta-control-ui-smoke.sh`: 1 file
- `docs/decisions`: 1 file
- `codex-rs/docs`: 1 file
- `HEPTA_ARCHITECTURE_MERGE_ROUTE_2026-05-19.md`: 1 file

## Size And Excludes

Source payload sizes:

- `apps/hepta-native`: about 5.8 MB
- `codex-rs/hepta-runtime`: about 2.3 MB
- `codex-rs/hepta-core`: about 1.4 MB
- `codex-rs/hepta-gateway`: about 644 KB
- `codex-rs/hepta-intelligence`: about 428 KB
- `apps/hepta-control-ui`: about 232 KB
- `codex-rs/hepta-memory`: about 200 KB
- `codex-rs/hepta-plugins`: about 152 KB
- `docs`: about 140 KB
- `scripts`: about 4 KB

Ignored build outputs observed:

- `codex-rs/target`: about 200 GB, ignored by git
- `target`: about 32 KB, ignored by git
- `codex-rs/hepta-runtime/target`: about 4 KB, ignored by git
- `apps/hepta-native/target`: absent

Do not add any `target/`, runtime state, local Telegram ledger, secret store, or generated audit artifact to review commits.

## Sensitive Material Check

A broad keyword scan finds only expected code/documentation references such as `secret`, `token`, `password`, GitHub workflow `${{ secrets.* }}` placeholders, and synthetic redaction fixtures.

A stricter literal-secret pattern scan for common live key shapes returned no hits.

## Reviewable Commit Slices

Recommended local packaging order:

1. `feat: add hepta codex runtime crates`
   - `codex-rs/hepta-core`
   - `codex-rs/hepta-runtime`
   - `codex-rs/hepta-intelligence`
   - `codex-rs/hepta-memory`
   - `codex-rs/hepta-plugins`
   - `codex-rs/hepta-gateway`
   - `codex-rs/docs/decisions/ADR-0001-architecture-foundation.md`
   - `docs/decisions/ADR-0001-architecture-foundation.md`
   - `codex-rs/Cargo.toml`
   - `codex-rs/Cargo.lock`

2. `refactor: route native gateway surfaces through hepta-gateway`
   - `codex-rs/cli/Cargo.toml`
   - `codex-rs/cli/src/native_gateway.rs`
   - `codex-rs/cli/src/native_telegram.rs`
   - `HEPTA_ARCHITECTURE_MERGE_ROUTE_2026-05-19.md`

3. `feat: add Hepta control UI app assets`
   - `apps/hepta-control-ui`
   - Control UI release docs under `docs/release/HEPTA_CONTROL_UI_*`
   - `scripts/hepta-control-ui-smoke.sh`

4. `feat: import Hepta Native desktop and mobile client`
   - `apps/hepta-native`
   - Robrix attribution docs under `docs/architecture`
   - `docs/release/HEPTA_NATIVE_TRANSPLANT_INVENTORY_2026-05-20.md`

5. `docs: inventory Hepta codex merge workset`
   - this file

## Post-Packaging Correction

The old `apps/hepta` wrapper was initially observed in the untracked payload, but it is not a valid `hepta-codex` entrypoint because its manifest and source depend on the old Hepta workspace's `hepta-cli` API. It has been removed from the reviewable package. The supported Control UI entrypoint is now:

```bash
cargo run --manifest-path codex-rs/Cargo.toml -p codex-cli --bin hepta -- --serve-ui 127.0.0.1:7373
```

The Control UI smoke script was aligned to the same `codex-rs/Cargo.toml` manifest and `codex-cli --bin hepta` binary.

Because `codex-cli --bin hepta` does not expose old Hepta slash commands as direct CLI subcommands, the smoke now validates Control UI and native gateway behavior through Rust tests rather than `cargo run ... /control-ui --json`.

## Verification Already Seen

Recent green gates before this inventory:

- `CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta`
- `CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway`
- `CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture`
- `CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture`
- `CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture`
- `CARGO_TARGET_DIR=apps/hepta-native/target cargo check --manifest-path apps/hepta-native/Cargo.toml`
- `CARGO_TARGET_DIR=apps/hepta-native/target cargo test --manifest-path apps/hepta-native/Cargo.toml hepta_ -- --nocapture`
- targeted `rustfmt --check` on new/changed native bridge/status files
- `git diff --check`

## Safety Boundary

Packaging this workset must remain local. It must not install, deploy, restart launchd services, activate native POST, activate live Telegram polling/sending, send Matrix events, invoke external providers, or change Telegram ownership.
