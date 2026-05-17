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

## Cleaned in entitlement/error-code pass

- Missing-entitlement detection now uses a Hepta-named helper and keeps
  user-facing access-denied copy branded as Hepta.
- The upstream OAuth `access_denied` code and `missing_codex_entitlement`
  marker are centralized as compatibility constants instead of being repeated
  as raw strings in callback rendering and tests.

## Cleaned in login home naming pass

- Login-server integration tests and local credential persistence internals now
  use Hepta home terminology for local variables and test names.
- The public/cross-crate `codex_home` field and auth-manager compatibility API
  names remain unchanged for now; they describe the same resolved Hepta home
  path under the current fork.

## Cleaned in auth storage home naming pass

- Auth storage backends now use `hepta_home` for their private resolved-home
  fields, constructor parameters, auth-file helpers, keyring key derivation, and
  ephemeral store keys.
- Auth storage unit tests now use Hepta home terminology for local variables
  and helper parameters.

## Cleaned in auth manager home naming pass

- Auth-manager private storage/load/logout helper parameters now use
  Hepta home terminology while preserving the existing auth storage behavior.
- AuthManager now stores its resolved runtime home in a private hepta_home
  field and reports that name in debug output.
- AuthManager::new, AuthManager::shared, and test-only home constructors now
  use Hepta home terminology for their local parameter names.
- The public AuthConfig::codex_home field and AuthManagerConfig::codex_home
  method remain compatibility API names for now.

## Cleaned in CLI login home naming pass

- The direct browser-login helper now accepts a Hepta-named resolved-home
  parameter before constructing the compatibility ServerOptions value.
- CLI login integration-test helpers now use hepta_command and hepta_home local
  names while still exercising the HEPTA_HOME runtime environment.

## Cleaned in CLI integration-test home naming pass

- Remaining CLI integration-test helpers now use Hepta command/home local names
  across app-server, debug, execpolicy, feature, marketplace, MCP, plugin, and
  update tests.
- The tests still exercise the same HEPTA_HOME runtime behavior and unchanged
  config/auth file semantics.

## Cleaned in login test home naming pass

- Login suite temp directories and helper parameters now use Hepta home
  terminology in device-code, refresh, logout, and auth unit tests.
- Public ServerOptions/AuthConfig fields named codex_home remain compatibility
  API keys while their local test inputs now use hepta_home names.

## Cleaned in CLI MCP home naming pass

- MCP add/remove command handlers now use Hepta home terminology for resolved
  local config paths before writing global MCP server config.
- Compatibility uses of config.codex_home and MCP protocol/event aliases stay
  unchanged.

## Cleaned in message history home naming pass

- Message-history docs now describe the default persistence path as
  `~/.hepta/history.jsonl`.
- The message-history config now stores its resolved runtime directory as
  `hepta_home`, and its local tests use Hepta home terminology.
- The crate/package name remains `codex-message-history` as an internal
  compatibility identifier.

## Cleaned in developer tooling pass

- Root `justfile` now makes `hepta` the primary local run recipe and keeps
  `codex` only as a compatibility convenience that runs the Hepta binary.
- The app-server test-client README now documents `hepta app-server`,
  `--hepta-bin`, and the Hepta temp runtime directory.
- Bubblewrap build-time diagnostics now prefer `HEPTA_BWRAP_SOURCE_DIR` and
  `HEPTA_SKIP_BWRAP_BUILD`, while keeping `CODEX_*` fallbacks.
- Bazel test launcher templates now prefer `HEPTA_BAZEL_TEST_SKIP_FILTERS`
  and retain the older `CODEX_BAZEL_TEST_SKIP_FILTERS` fallback.
- RMCP test servers and resource fixtures now use `memo://hepta/*` and Hepta
  memo titles for local test resources; protocol metadata keys such as
  `codex/imageDetail` remain compatibility names.
- TUI source-level comments now describe the Hepta TUI where they are
  human-readable documentation rather than crate names.

## Cleaned in exec surface pass

- Non-interactive exec help and usage now describe `hepta exec`, Hepta config,
  and `HEPTA_HOME` in visible CLI text.
- The exec originator, root tracing span, telemetry process name, and
  in-process client name now use `hepta_exec`.
- Exec JSONL event comments, stdin-behavior comments, and focused tests now use
  Hepta wording while keeping the `codex-exec` crate/binary compatibility
  name.
- Exec-server README now documents `hepta exec-server` as the command surface
  while preserving legacy wire/environment names such as
  `CODEX_EXEC_SERVER_REMOTE_BEARER_TOKEN`.
- MCP config comments and protocol README now refer to Hepta command surfaces
  where they describe user-facing behavior.

## Cleaned in user-facing error/prose pass

- Local image attachment placeholders now say Hepta when files cannot be read
  or are unsupported.
- Skill-description truncation warnings now say Hepta can still see every
  skill.
- Unified exec network denial fallback now names the Hepta sandbox network
  proxy.
- TUI API-key status guidance now suggests `hepta login`.
- Workspace plugin setting warnings/errors now say Hepta plugins.
- Feedback doctor-report comments and attachment filename now use
  `hepta-doctor-report.json`.
- Memory read/search tool descriptions now refer to Hepta memory files.
- Bubblewrap sandbox warnings, session-storage initialization errors, memory
  MCP server descriptions, IDE context retry hints, and plugin-disabled tests
  now use Hepta wording.

## Cleaned in MCP/OAuth and exec-server surface pass

- MCP startup auth-required errors now tell users to run `hepta mcp login`.
- ChatGPT connector-auth elicitations now say the connector will be used in
  Hepta.
- MCP OAuth dynamic-client registration now presents Hepta as the client name.
- Exec-server missing-runtime-path errors, environment comments, and sample
  configured commands now use Hepta wording while preserving legacy
  compatibility environment names.

## Cleaned in memory/realtime/account diagnostics pass

- Memory list tool descriptions and fallback consolidation prompts now refer
  to Hepta memories.
- Realtime startup context and rollback retry warnings now name Hepta.
- Ollama version, responses-api-proxy API-key guidance, account/rate-limit
  errors, and memory rate-limit diagnostics now use Hepta wording.

## Cleaned in app-server protocol identity pass

- App-server initialize examples and tests now use Hepta client titles while
  preserving compatibility originator names such as `codex_vscode`.
- App-server protocol documentation for auth refresh, MCP elicitation, plugin
  availability, and process spawning now uses Hepta wording.
- Hook schema comments now describe Hepta extensions before regenerating the
  vendored hook schema fixtures.

## Cleaned in Windows sandbox product surface pass

- Windows sandbox local users, desktop names, mutex names, group comments,
  firewall friendly text, WFP names/descriptions, runtime-bin cache path,
  debug config examples, and TUI fallback prompt text now use Hepta naming.
- Stable internal firewall rule identifiers remain `codex_sandbox_*` so
  existing installations can still find/update prior rules instead of
  orphaning them.

## Cleaned in general product prose pass

- TUI slash-command descriptions, session help, high-load fallback text,
  terminal reflow comments/assertions, and debug/version comments now use Hepta
  product wording.
- Sleep-inhibitor assertion reasons now say Hepta for active turns on Linux,
  macOS, and Windows.
- External-agent migration rewriting now maps Claude/Claude Code source prose to
  Hepta rather than Codex, with updated import tests.
- Skill-creator sample guidance, MCP documentation examples, ChatGPT test
  fixtures, Linux sandbox README, responses proxy README, and auto-review model
  display copy now use Hepta wording while preserving compatibility slugs and
  binary names where needed.

## Cleaned in docs/sample/commentary pass

- MCP interface documentation now presents Hepta command names and tool names,
  while documenting legacy codex / codex-reply aliases and codex/event
  notifications as compatibility surfaces.
- Bundled skill/plugin creator sample skills now describe Hepta capability,
  UI ordering, HEPTA_HOME, ~/.hepta, and hepta:// app handoff links.
- TUI approval history, pet image fallback guidance, model migration fixture
  copy, and Apps local-file upload errors now use Hepta wording.
- Core/config/client comments and test-only template fixtures now use Hepta
  where the text describes the product/runtime rather than an internal crate,
  path, environment variable, or wire field.

## Cleaned in README/crate-doc pass

- README files for the network proxy, responses proxy, API client,
  file-search, thread-manager sample, thread-store, rollout trace, memory
  pipeline, Linux sandbox, OTEL, stream parser, and template utility now use
  Hepta wording for product/runtime prose.
- Crate-level Rust docs for custom CA handling, Cloudflare-cookie handling,
  cloud requirements, rollout tracing, rollout protocol mapping, rollout
  session models, and memory read/write/MCP crates now describe Hepta instead
  of Codex.
- Sample skill assets now use Hepta command/home wording for OpenAI-docs MCP
  installation, plugin rendering, and image-generation save/helper paths.
- Compatibility names such as `CODEX_CA_CERTIFICATE`, `CODEX_ROLLOUT_TRACE_ROOT`,
  `codex-core`, `codex-*` crate names, `codex_process_hardening`, and upstream
  ChatGPT `/backend-api/codex/...` paths remain unchanged.

## Cleaned in bundled skill/reference comment pass

- Image-generation and skill-installer bundled skill references now use
  `HEPTA_HOME` / `~/.hepta` for helper scripts, generated-image paths, and
  installed-skill locations.
- Template test fixtures and utility crate docs now use Hepta in example text
  and comments where the text describes the product/runtime.
- Core skill loading, model-provider auth/capability comments, hook dispatch
  comments, thread-store comments, path utilities, and plugin utilities now use
  Hepta wording while keeping compatibility product enum and crate/type names.

## Cleaned in developer docs / trace-comment pass

- Root developer instructions, the Nix flake description, and the direct-fork
  note now refer to Hepta as the active product/runtime while still documenting
  the upstream Codex source relationship.
- Protocol v1 docs now describe the Hepta backend/core system and use Hepta in
  the sequence diagram participant names.
- Analytics, core thread/session comments, and rollout-trace model/raw-event
  comments now use Hepta wording for product/runtime behavior while retaining
  `Codex*` Rust type names and trace enum names as compatibility identifiers.

## Cleaned in MCP / skill / runtime surface pass

- Skill cache docs and bundled skill-installer helpers now prefer
  `HEPTA_HOME` / `~/.hepta`, while still accepting `CODEX_HOME` as a
  legacy fallback.
- MCP runner, elicitation, OAuth cache, RMCP client info, and network-proxy
  user-facing prose now use Hepta wording while retaining wire keys,
  compatibility capability ids, and crate names.
- Client, websocket custom-CA comments, model-manager comments, state
  migration docs, shell-command docs, TUI style notes, and memory
  consolidation templates now describe Hepta as the active runtime.

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
- ChatGPT auth error markers such as `missing_codex_entitlement` remain
  upstream compatibility names.

## Deferred

- App-server README and generated schema prose still contain deliberate
  compatibility references: crate/package names, model ids such as
  `gpt-5.1-codex`, `.codex` project-hook metadata folders, upstream
  ChatGPT Codex attestation/device URLs, and OpenAI VS Code compatibility
  examples.
- Upstream service URLs such as ChatGPT Codex backend and usage pages remain until Hepta-owned endpoints are available.
- Snapshot fixture names retain `codex_tui` because they follow the crate/test target name.
