# Hepta Codex-Derived Runtime

This workspace is a Codex-derived Rust implementation being modified directly
into the new Hepta runtime.

## Building Hepta

Build the first forked binary from this workspace:

```shell
cargo build -p codex-cli --bin hepta
./target/debug/hepta --help
```

Packaged Hepta releases are not published from this fork yet; build the local
`hepta` binary from source while the rebase is in progress.

## Documentation quickstart

- First run with Hepta? Start with [`docs/getting-started.md`](../docs/getting-started.md) (links to the walkthrough for prompts, keyboard shortcuts, and session management).
- Want deeper control? See [`docs/config.md`](../docs/config.md) and [`docs/install.md`](../docs/install.md).

## What's new in the Rust CLI

The first migration step keeps upstream internal crate names in place, while
the binary and runtime home are Hepta-owned.

### Config

Hepta inherits the upstream `config.toml` format initially, but resolves runtime
state from `HEPTA_HOME` / `~/.hepta`.

### Model Context Protocol Support

#### MCP client

Hepta functions as an MCP client that allows the Hepta CLI and IDE extension to connect to MCP servers on startup. See the [`configuration documentation`](../docs/config.md#connecting-to-mcp-servers) for details.

#### MCP server (experimental)

Hepta can be launched as an MCP _server_ by running `hepta mcp-server`. This allows _other_ MCP clients to use Hepta as a tool for another agent.

Use the [`@modelcontextprotocol/inspector`](https://github.com/modelcontextprotocol/inspector) to try it out:

```shell
npx @modelcontextprotocol/inspector hepta mcp-server
```

Use `hepta mcp` to add/list/get/remove MCP server launchers defined in `config.toml`, and `hepta mcp-server` to run the MCP server directly.

### Notifications

You can enable notifications by configuring a script that is run whenever the agent finishes a turn. The [notify documentation](../docs/config.md#notify) includes a detailed example that explains how to get desktop notifications via [terminal-notifier](https://github.com/julienXX/terminal-notifier) on macOS. When Hepta detects that it is running under WSL 2 inside Windows Terminal (`WT_SESSION` is set), the TUI automatically falls back to native Windows toast notifications so approval prompts and completed turns surface even though Windows Terminal does not implement OSC 9.

### `hepta exec` to run Hepta programmatically/non-interactively

To run Hepta non-interactively, run `hepta exec PROMPT` (you can also pass the prompt via `stdin`) and Hepta will work on your task until it decides that it is done and exits. If you provide both a prompt argument and piped stdin, Hepta appends stdin as a `<stdin>` block after the prompt so patterns like `echo "my output" | hepta exec "Summarize this concisely"` work naturally. Output is printed to the terminal directly. You can set the `RUST_LOG` environment variable to see more about what's going on.
Use `hepta exec --ephemeral ...` to run without persisting session rollout files to disk.

### Experimenting with the Hepta Sandbox

To test what happens when a command is run under the sandbox provided by Hepta, use the following subcommands:

```
# macOS
hepta sandbox macos [--log-denials] [COMMAND]...

# Linux
hepta sandbox linux [COMMAND]...

# Windows
hepta sandbox windows [COMMAND]...

# Legacy aliases
hepta debug seatbelt [--log-denials] [COMMAND]...
hepta debug landlock [COMMAND]...
```

To try a writable legacy sandbox mode with these commands, pass an explicit config override such
as `-c 'sandbox_mode="workspace-write"'`.

### Selecting a sandbox policy via `--sandbox`

The Rust CLI exposes a dedicated `--sandbox` (`-s`) flag that lets you pick the sandbox policy **without** having to reach for the generic `-c/--config` option:

```shell
# Run Hepta with the default, read-only sandbox
hepta --sandbox read-only

# Allow the agent to write within the current workspace while still blocking network access
hepta --sandbox workspace-write

# Danger! Disable sandboxing entirely (only do this if you are already running in a container or other isolated env)
hepta --sandbox danger-full-access
```

The same setting can be persisted in `~/.hepta/config.toml` via the top-level `sandbox_mode = "MODE"` key, e.g. `sandbox_mode = "workspace-write"`.
In `workspace-write`, Hepta also includes `~/.hepta/memories` in its writable roots so memory maintenance does not require an extra approval.

## Code Organization

This folder is the root of a Cargo workspace. It contains quite a bit of experimental code, but here are the key crates:

- [`core/`](./core) contains the business logic for Hepta. Ultimately, we hope this becomes a library crate that is generally useful for building other Rust/native applications that use Hepta-compatible agent runtime APIs.
- [`exec/`](./exec) "headless" CLI for use in automation.
- [`tui/`](./tui) CLI that launches a fullscreen TUI built with [Ratatui](https://ratatui.rs/).
- [`cli/`](./cli) CLI multitool that provides the aforementioned CLIs via subcommands.

If you want to contribute or inspect behavior in detail, start by reading the module-level `README.md` files under each crate and run the project workspace from the top-level `codex-rs` directory so shared config, features, and build scripts stay aligned.
