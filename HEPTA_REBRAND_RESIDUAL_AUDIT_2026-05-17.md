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

## Cleaned in prompt identity pass

- Bundled base prompts, realtime prompts, model instruction templates, and the
  models-manager catalog now identify the local agent/runtime as Hepta instead
  of Codex.
- IDE-context prompt serialization now emits `## My request for Hepta:`.
  Protocol, rollout, state, thread-store, app-server summary, and TUI transcript
  extraction paths still accept the legacy `## My request for Codex:` marker
  for older saved transcripts.
- TUI session headers, status cards, exec banner, feedback subject lines,
  guardian review prompts, dynamic-tool descriptions, and related snapshots now
  use Hepta product wording.

## Cleaned in analytics opt-in pass

- Analytics event export is now explicit opt-in for the Hepta source fork:
  unset analytics config no longer creates a sender for upstream Codex analytics
  endpoints.
- Tests that exercise analytics delivery now set `[analytics].enabled = true`
  in their temporary config, keeping the compatibility event schema covered
  without making local Hepta installs send analytics by default.

## Cleaned in login/default-client identity pass

- The default outbound `originator` and `User-Agent` identity now uses
  `hepta_cli_rs` instead of `codex_cli_rs`.
- The default residency header name now uses
  `x-openai-internal-hepta-residency`.
- `HEPTA_INTERNAL_ORIGINATOR_OVERRIDE` is now the preferred override env var;
  `CODEX_INTERNAL_ORIGINATOR_OVERRIDE` remains as a legacy fallback.
- Hepta-owned app-server daemon probing now initializes as
  `hepta_app_server_daemon`, while the old daemon name remains non-originating
  compatibility input.
- Backend/cloud-task HTTP fallback and suffix user agents now use Hepta names
  such as `hepta-cli` and `hepta_cloud_tasks_tui`.
- Downstream default-client callers now use `get_hepta_user_agent()`; the old
  `get_codex_user_agent()` helper is retained only as a compatibility shim.

## Cleaned in Cloud Tasks / ChatGPT wrapper pass

- Cloud Tasks CLI help now says Hepta Cloud for task submission, status, list,
  apply, diff, prompt, branch, and task-id descriptions.
- Cloud Tasks sign-in and environment resolution guidance now points users at
  `hepta login` / `hepta cloud`; pagination guidance now prints
  `hepta cloud list --cursor=...`.
- Cloud Tasks request identity suffixes now use `hepta_cloud_tasks_*` values.
- ChatGPT wrapper errors now say Hepta ChatGPT auth and `hepta login` instead
  of Codex backend auth / `codex login`.
- ChatGPT apply-command help and crate README now describe Hepta agent tasks.

## Cleaned in developer auxiliary-tool pass

- App-server test/debug clients now present Hepta app-server command names,
  help text, default binary names, trace guidance, and visible runtime dirs.
  Legacy `--codex-bin`, `CODEX_BIN`, and `CODEX_APP_SERVER_URL` remain
  compatibility inputs only.
- Root Rust workspace README now documents local Hepta source builds,
  `hepta mcp`, `hepta mcp-server`, `hepta exec`, Hepta sandbox commands,
  and `~/.hepta` runtime paths instead of upstream Codex install commands.
- Execpolicy README now documents `hepta execpolicy check`; the standalone
  dev crate name remains `codex-execpolicy`.
- Windows sandbox smoke script now resolves `hepta.exe` / `hepta`, uses
  `HEPTA_HOME`, and tests Hepta runtime artifact protection under `.hepta`.
- Nix package metadata now names the forked package/program as Hepta, while
  keeping the Cargo workspace crate names unchanged.

## Cleaned in auth storage/keyring pass

- Hepta auth now recognizes `HEPTA_API_KEY` and `HEPTA_ACCESS_TOKEN` as
  preferred environment variables, while preserving `CODEX_API_KEY` and
  `CODEX_ACCESS_TOKEN` as legacy fallbacks through the existing compatibility
  reader names.
- Auth environment telemetry now treats either Hepta or legacy Codex API-key
  variables as present without exposing the secret value.
- Auth storage comments and keyring errors now refer to Hepta auth,
  `$HEPTA_HOME/auth.json`, and the `Hepta Auth` keyring service.
- Keyring store-key coverage now uses the `~/.hepta` home path; the legacy
  fallback still uses the same auth.json schema if selected by config loading.

## Cleaned in device-code/login URL pass

- The local login redirect error now reports that redirecting back to Hepta
  failed instead of naming Codex.
- Device-code login now documents the retained upstream ChatGPT authorization
  paths as compatibility constants instead of leaving `/codex/device` and
  `/deviceauth/callback` as unexplained product-surface strings.
- OAuth authorize/success flow query keys such as `codex_cli_simplified_flow`
  and `codex_streamlined_login` are now centralized as upstream compatibility
  constants while preserving the wire names expected by the ChatGPT login
  service and older browser callbacks.

## Keep for compatibility

- Crate, module, and package names such as `codex-core`, `codex-tui`, `codex_protocol`, and `codex_app_server_protocol`.
- Wire/API field names such as `codexHome`, `codexErrorInfo`, `codexStreamlinedLogin`, and generated TypeScript/JSON schema type names.
- Legacy environment variables and paths that remain fallback compatibility surfaces, including `CODEX_HOME`, `CODEX_SQLITE_HOME`, `CODEX_OSS_*`, and `.codex`.
- Remote-control headers and protocol identifiers that external clients may already depend on.
- Legacy originator/client compatibility inputs such as
  `codex_app_server_daemon`, `codex_vscode`, `codex-tui`, and the
  `get_codex_user_agent()` shim.
- Cloud Tasks crate/module identifiers such as `codex_cloud_tasks_client`
  remain compatibility/internal Rust API names.
- MCP compatibility aliases and event names such as `codex`, `codex-reply`,
  and `codex/event`.
- App-server test-client internal helper types such as `CodexClient` and
  `SpawnCodex` remain deferred internal identifiers; the user-facing CLI now
  defaults to `hepta` and Hepta wording.
- Auth type/API names such as `CodexAuth`, `codex_home`, and exported
  `read_codex_*_from_env` helpers remain compatibility identifiers. The
  helpers now read Hepta environment variable names first.
- ChatGPT auth wire paths and query keys such as `/codex/device`,
  `/deviceauth/callback`, `codex_cli_simplified_flow`, and
  `codex_streamlined_login` remain upstream compatibility names.

## Deferred

- App-server README and generated schema prose still contain deliberate
  compatibility references: crate/package names, model ids such as
  `gpt-5.1-codex`, `.codex` project-hook metadata folders, upstream
  ChatGPT Codex attestation/device URLs, and OpenAI VS Code compatibility
  examples.
- Upstream service URLs such as ChatGPT Codex backend and usage pages remain until Hepta-owned endpoints are available.
- Snapshot fixture names retain `codex_tui` because they follow the crate/test target name.
