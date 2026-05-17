# Hepta Release Preflight - 2026-05-17

This note records the first install-before-install preflight after the visible
Hepta rebrand sweep. It intentionally validates a source-built binary without
replacing the currently installed/running Hepta service.

## Scope

- Repo: `/Users/qianqi/.openclaw/workspace/hepta-codex`
- Branch: `main`
- Baseline commit before this note: `bc6272a docs: record hepta residual scan status`
- Remote: none configured
- Target dir: `/tmp/hepta-codex-release-target`
- Installed service replacement: not attempted

## Build Evidence

The true release build completed successfully:

```sh
CARGO_TARGET_DIR=/tmp/hepta-codex-release-target \
  cargo build --release --offline -q \
  -p codex-cli --bin hepta \
  --manifest-path codex-rs/Cargo.toml
```

Produced binary:

```text
/tmp/hepta-codex-release-target/release/hepta
size: 122M
version: hepta 0.0.0
```

The preflight also caught an operator slip: an earlier build command omitted
`--release` and only produced `/tmp/hepta-codex-release-target/debug/hepta`.
That debug artifact was not accepted as release evidence.

## Smoke Evidence

These release-binary smoke checks exited successfully:

- `hepta --version` prints `hepta 0.0.0`.
- `hepta --help` presents `Hepta CLI`, `Usage: hepta ...`, and Hepta command
  descriptions.
- `hepta exec --help` presents `Run Hepta non-interactively` and Hepta
  examples.
- `hepta app-server --help` presents Hepta config paths such as
  `~/.hepta/config.toml` and the expected app-server options.
- `hepta sandbox linux --help` presents the Hepta sandbox command surface.
- `hepta doctor --help` presents `Diagnose local Hepta installation, config,
  auth, and runtime health`.

## Doctor Result

The release binary was also run with temporary homes:

```sh
HEPTA_HOME="$(mktemp -d /tmp/hepta-smoke-home.XXXXXX)" \
HEPTA_SQLITE_HOME="$(mktemp -d /tmp/hepta-smoke-sql.XXXXXX)" \
  /tmp/hepta-codex-release-target/release/hepta doctor --json
```

Expected result: exit code `1` because the temporary home intentionally had no
credentials and the command ran in a non-interactive `TERM=dumb` environment.

Important observed fields:

- `auth.credentials`: `fail`, `no Hepta credentials were found`.
- `terminal.env`: `fail`, `TERM=dumb - colors and cursor control are disabled`.
- `config.load`: `ok`, with `HEPTA_HOME`, `log dir`, and `sqlite home`
  rooted in the temporary Hepta smoke directories.
- `app_server.status`: `ok`, `background server is not running`, with
  control/daemon paths under the temporary `HEPTA_HOME`.
- `updates.status`: `warning`, latest-version probing disabled for the direct
  Hepta source fork.
- `runtime.provenance`: `ok`, current executable points at the release smoke
  binary.

Note: plain `doctor --json` still performs its built-in reachability checks. In
this run it contacted provider endpoints without credentials and reported
reachable HTTP plus an expected unauthorized WebSocket handshake.

Follow-up hardening landed after this preflight: `hepta doctor --no-network`
now skips provider HTTP, Responses WebSocket, and MCP HTTP reachability probes
while preserving local config/auth/path checks.

Post-hardening release smoke:

- Rebuilt `/tmp/hepta-codex-release-target/release/hepta` after the
  `doctor --no-network` change.
- `hepta doctor --no-network --json` with temporary Hepta homes still exits
  `1` for expected local auth/terminal failures, but provider and WebSocket
  checks now report skipped summaries.
- The no-network JSON output contains no provider endpoint field, no
  `reachable over HTTP` result, and no WebSocket handshake transport error.

## Staged Installed-Path Dry Smoke

A follow-up dry smoke copied the rebuilt release binary out of Cargo's target
tree into a temporary staged install path:

```text
/tmp/hepta-staged-install.<suffix>/bin/hepta
```

The staged smoke deliberately unset package-manager provenance env vars so the
binary was validated as a direct local source-fork executable rather than as the
currently installed npm shim.

Observed result:

- `hepta --version` printed `hepta 0.0.0`.
- `hepta doctor --no-network --json` still returned expected exit code `1`
  for temporary-home auth and non-interactive terminal failures.
- `installation`: `ok`, `installation looks consistent`, install context
  `other`, current executable pointing at the staged binary.
- `runtime.provenance`: `ok`, `running local build on macos-aarch64`, install
  method `other`.
- `updates.status`: `ok`, update channel is `local source fork`, update
  action is `manual source fork update`.
- Provider HTTP and WebSocket reachability remained skipped; output contained no
  provider endpoint field, no `reachable over HTTP` result, and no WebSocket
  handshake transport error.

## Current Readiness Judgment

Release build and CLI help surfaces are clean enough for an install candidate.
This is not yet an install/restart decision. Replacing the active local Hepta
service should be a separate step because it mutates the user's current runtime.

## Recommended Next Step

Before replacing the active service:

1. Perform `hepta install` / service restart only as an explicit runtime
   replacement step.
2. After replacement, verify the running
   service binary path.
