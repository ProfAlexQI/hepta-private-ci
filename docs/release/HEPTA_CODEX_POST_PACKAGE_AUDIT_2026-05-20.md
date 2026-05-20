# Hepta Codex Post-Package Audit

Date: 2026-05-20
Scope: local release-review audit for `00320d3..6fc04e5` plus the audit fix commit
Status: audit complete; controlled install later performed without Telegram owner handoff

## Audit Scope

Reviewed the six local packaging commits after `00320d3`:

- `3e33f7a feat: add Hepta codex runtime crates`
- `7b8b2fd refactor: route native gateway surfaces through hepta-gateway`
- `61a7158 feat: add Hepta control UI app wrapper`
- `ad2de43 feat: import Hepta Native client`
- `217a27c docs: inventory Hepta codex workset`
- `6fc04e5 fix: align Hepta control UI smoke with codex workspace`

The audit focused on deploy-impacting risks: package completeness, runtime entrypoints, Telegram owner safety, native POST side effects, secret exposure, ignored artifacts, Control UI serving boundary, and source-only native transplant hygiene.

## Findings

### P1 - Fixed: Hepta Native icon resources were ignored

The repo-root `.gitignore` rule `Icon?` ignored `apps/hepta-native/resources/icons/`, leaving 57 required SVG resources present locally but untracked. Native checks could pass on this machine because the files existed in the working tree, while a clean clone would miss resources referenced by `crate_resource("self://resources/icons/...")`.

Fix applied:

- added explicit `.gitignore` exceptions for `apps/hepta-native/resources/icons/`;
- included the 57 SVG resources in the review package;
- updated the Robrix copy manifest and native transplant inventory to reflect the excluded local `AGENTS.md` and the icon-resource correction.

### P2 - Closed: installed binary was older than audited release build

The audited source built a release binary at `codex-rs/target/release/hepta`, but it had not been installed at initial audit time. The active service binary at `/Users/qianqi/.local/opt/hepta-codex/bin/hepta-codex` still represented the previously deployed binary.

This was expected because the audit intentionally avoided install/deploy/restart. It was closed later by the controlled install documented in `HEPTA_CODEX_CONTROLLED_INSTALL_2026-05-20.md`, while preserving old OpenClaw as Telegram owner. The current installed binary after the follow-up coexistence-status patch is `ed61f2b` with sha256 `8aa6dd230a83054eb8eba528635cc8346e2e1d337fd91c8b941bb04dea8af333`.

## No P0 Blockers Found

No audit finding requires blocking the local package itself after the icon-resource fix. The remaining hard boundary is operational: installation and Telegram ownership must stay controlled and explicit.

## Safety Boundary Verified

- Old OpenClaw remains Telegram owner.
- Hepta Telegram poll loop is not armed.
- No double-poller risk was observed.
- Native POST activation is preflight-ready but disabled by default because required real-handler gates are not enabled.
- Control UI bind defaults to `127.0.0.1:7373`, with non-loopback exposure requiring `HEPTA_ALLOW_NON_LOOPBACK_UI=1`.
- Control UI responses include local security headers including `Content-Security-Policy`, `X-Content-Type-Options`, `Referrer-Policy`, and `X-Frame-Options`.
- Secret-pattern scan found no new live key material in the audited package; hits outside the new package were pre-existing synthetic tests.
- `apps/hepta-native/target`, local runtime state, `.hepta`, generated artifacts, and native build outputs are not included.

## Verification

Post-package gates already passed before the audit fix:

- `cargo fmt --all --manifest-path codex-rs/Cargo.toml -- --check`
- `CARGO_INCREMENTAL=0 cargo check --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-runtime -p hepta-gateway -p codex-cli --bin hepta`
- `CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p hepta-gateway`
- `CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_telegram -- --nocapture`
- `CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_gateway -- --nocapture`
- `CARGO_INCREMENTAL=0 cargo test --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta native_post -- --nocapture`
- `CARGO_NET_OFFLINE=true scripts/hepta-control-ui-smoke.sh`
- `CARGO_INCREMENTAL=0 cargo build --release --offline --manifest-path codex-rs/Cargo.toml -q -p codex-cli --bin hepta`

Release binary from the non-deploy build:

- path: `codex-rs/target/release/hepta`
- sha256: `31914dec00ca16793e396013951d350f63d5aa5cf00554c4f40690cc700e312e`

Additional audit-fix verification:

- `cargo metadata --offline --manifest-path codex-rs/Cargo.toml --no-deps --format-version 1`
- `cargo metadata --offline --manifest-path apps/hepta-native/Cargo.toml --no-deps --format-version 1`
- `git diff --check`

## Release Decision

The local package was audit-clean for a controlled install rehearsal after the icon-resource fix was committed. It has since been installed through the controlled install path, not as a blind continuation. Any future Telegram owner handoff remains a separate operation requiring explicit instruction.

Subsequent native POST single-handler dry-run canaries for `task_publish`, `approval_apply`, and `chat_send` recorded gray-release evidence without publishing a task, applying an approval, sending chat, or enabling active-service native POST activation. Those canaries are recorded in `HEPTA_CODEX_CONTROLLED_INSTALL_2026-05-20.md`.

Controlled install requirements were:

- binary/plist backups;
- active-owner snapshot;
- old OpenClaw Telegram kept as owner unless explicitly changed;
- post-install health, route parity, Telegram owner handoff, Telegram poll-loop gated status, and native POST activation-plan smoke;
- rollback command list before any service mutation.
