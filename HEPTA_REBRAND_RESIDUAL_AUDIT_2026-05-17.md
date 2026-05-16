# Hepta Rebrand Residual Audit - 2026-05-17

This audit tracks Codex-era strings after the direct Hepta fork rebase. It separates user-visible cleanup from compatibility surfaces that should not be renamed mechanically.

## Cleaned in this pass

- CLI remote-mode rejection now reports `hepta <subcommand>` instead of `codex <subcommand>`.
- `hepta app` no longer opens or downloads upstream Codex Desktop by default. It looks for Hepta Desktop and fails closed unless an explicit `--download-url` is supplied.
- TUI model migration and availability copy now says Hepta in user-visible upgrade prompts.
- Terminal title and status-line configuration descriptions now say Hepta for app name/version items.

## Keep for compatibility

- Crate, module, and package names such as `codex-core`, `codex-tui`, `codex_protocol`, and `codex_app_server_protocol`.
- Wire/API field names such as `codexHome`, `codexErrorInfo`, `codexStreamlinedLogin`, and generated TypeScript/JSON schema type names.
- Legacy environment variables and paths that remain fallback compatibility surfaces, including `CODEX_HOME`, `CODEX_SQLITE_HOME`, `CODEX_OSS_*`, and `.codex`.
- Remote-control headers and protocol identifiers that external clients may already depend on.

## Deferred

- App-server README and generated schema prose still contain Codex-era product wording; clean them in a dedicated API-doc/schema regeneration pass.
- Model base instructions still identify the agent as Codex in bundled catalog data; changing that affects model behavior and should be reviewed separately from UI string cleanup.
- Upstream service URLs such as ChatGPT Codex backend and usage pages remain until Hepta-owned endpoints are available.
- Snapshot fixture names retain `codex_tui` because they follow the crate/test target name.
