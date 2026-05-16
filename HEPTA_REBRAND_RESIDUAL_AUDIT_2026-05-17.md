# Hepta Rebrand Residual Audit - 2026-05-17

This audit tracks Codex-era strings after the direct Hepta fork rebase. It separates user-visible cleanup from compatibility surfaces that should not be renamed mechanically.

## Cleaned in this pass

- CLI remote-mode rejection now reports `hepta <subcommand>` instead of `codex <subcommand>`.
- `hepta app` no longer opens or downloads upstream Codex Desktop by default. It looks for Hepta Desktop and fails closed unless an explicit `--download-url` is supplied.
- TUI model migration and availability copy now says Hepta in user-visible upgrade prompts.
- Terminal title and status-line configuration descriptions now say Hepta for app name/version items.

## Cleaned in API/docs/schema pass

- Root README no longer points users at upstream Codex release artifacts or
  `codex` login/docs as the default Hepta path.
- App-server README now documents `hepta app-server`, `HEPTA_HOME`, Hepta
  generation/review/sandbox language, and Hepta skill/config examples while
  keeping compatibility field names such as `codexHome`.
- App-server protocol schema comments now describe `codexHome` as the resolved
  Hepta home compatibility field and update thread-origin examples to
  `hepta exec` / `hepta app-server`.

## Cleaned in feature/docs pass

- Experimental feature menu descriptions and announcements now use Hepta for
  transcript reflow, memories, network proxy restart, external migration, and
  idle-sleep copy.
- Deprecated feature warnings no longer point users to upstream Codex
  documentation as the default Hepta configuration reference.
- Skill installer sample guidance now tells users to restart Hepta after
  installing new skills.
- Root security, changelog, announcement-tip, and changelog-generation prose now
  describe this repository as a Hepta fork rather than an install/update path
  for upstream Codex artifacts.

## Cleaned in configuration/schema wording pass

- Config-loading diagnostics, network-proxy managed CA diagnostics, and
  app-server config-manager comments now say Hepta for user-facing
  configuration surfaces.
- App-server auth protocol descriptions now say Hepta-managed ChatGPT/API-key
  tokens while retaining compatibility field and type names.
- Config JSON schema, app-server JSON/TypeScript schema fixtures, memory
  consolidation templates, and the imagegen fallback network note now use Hepta
  product wording.

## Cleaned in MCP surface pass

- MCP initialize `serverInfo` now reports `hepta-mcp-server`, title `Hepta`,
  and a Hepta-owned `user_agent` value.
- MCP `tools/list` now advertises `hepta` and `hepta-reply` with Hepta tool
  titles, descriptions, and schema descriptions.
- MCP tool-call parse/load/start/runtime errors and elicitation prompts now say
  Hepta instead of Codex.
- Integration tests exercise the new `hepta` tool name while the server keeps
  accepting legacy `codex` / `codex-reply` calls as hidden compatibility
  aliases.

## Cleaned in TUI microcopy pass

- Composer placeholder and related test helpers now say `Ask Hepta to do
  anything`.
- Approval denial, permission-preset, memory-settings, feedback-upload, and
  hook-event descriptions now use Hepta product wording.
- TUI model chooser descriptions now say models are optimized for Hepta while
  preserving model ids such as `gpt-5.1-codex`.

## Keep for compatibility

- Crate, module, and package names such as `codex-core`, `codex-tui`, `codex_protocol`, and `codex_app_server_protocol`.
- Wire/API field names such as `codexHome`, `codexErrorInfo`, `codexStreamlinedLogin`, and generated TypeScript/JSON schema type names.
- Legacy environment variables and paths that remain fallback compatibility surfaces, including `CODEX_HOME`, `CODEX_SQLITE_HOME`, `CODEX_OSS_*`, and `.codex`.
- Remote-control headers and protocol identifiers that external clients may already depend on.
- MCP compatibility aliases and event names such as `codex`, `codex-reply`,
  and `codex/event`.

## Deferred

- App-server README and generated schema prose still contain deliberate
  compatibility references: crate/package names, model ids such as
  `gpt-5.1-codex`, `.codex` project-hook metadata folders, upstream
  ChatGPT Codex attestation/device URLs, and OpenAI VS Code compatibility
  examples.
- Model base instructions still identify the agent as Codex in bundled catalog data; changing that affects model behavior and should be reviewed separately from UI string cleanup.
- Upstream service URLs such as ChatGPT Codex backend and usage pages remain until Hepta-owned endpoints are available.
- Snapshot fixture names retain `codex_tui` because they follow the crate/test target name.
