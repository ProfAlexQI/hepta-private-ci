set working-directory := "codex-rs"
set positional-arguments

rust_min_stack := "8388608" # 8 MiB
rusty_v8_cargo := justfile_directory() + "/scripts/rusty-v8-cargo"

# Display help
help:
    just -l

# `hepta`
alias h := hepta
alias c := codex
hepta *args:
    {{ rusty_v8_cargo }} run -p hepta-cli --bin hepta -- "$@"

# `codex` compatibility convenience for old local muscle memory.
codex *args:
    {{ rusty_v8_cargo }} run -p codex-cli --bin hepta-codex-compat -- "$@"

# Legacy full-feature `exec` compatibility.
exec *args:
    {{ rusty_v8_cargo }} run -p codex-cli --bin hepta-codex-compat -- exec "$@"

# Start the Hepta TUI through the exec-server harness.
[no-cd]
tui-with-exec-server *args:
    {{ justfile_directory() }}/scripts/run_tui_with_exec_server.sh "$@"

# Run the CLI version of the file-search crate.
file-search *args:
    {{ rusty_v8_cargo }} run --bin codex-file-search -- "$@"

# Build the Hepta CLI and run the app-server test client.
app-server-test-client *args:
    {{ rusty_v8_cargo }} build -p codex-cli --bin hepta-codex-compat
    {{ rusty_v8_cargo }} run -p codex-app-server-test-client -- --hepta-bin ./target/debug/hepta-codex-compat "$@"

# Run Cargo with the exact sandboxed V8 archive and binding for the selected target.
cargo *args:
    {{ rusty_v8_cargo }} "$@"

check *args:
    {{ rusty_v8_cargo }} check "$@"

# Format the repository-native Just and Rust sources.
fmt:
    just --unstable --fmt
    {{ rusty_v8_cargo }} fmt -- --config imports_granularity=Item

# Check repository-native Just and Rust formatting without modifying files.
fmt-check:
    just --unstable --fmt --check
    {{ rusty_v8_cargo }} fmt -- --config imports_granularity=Item --check

fix *args:
    {{ rusty_v8_cargo }} clippy --fix --tests --allow-dirty "$@"

clippy *args:
    {{ rusty_v8_cargo }} clippy --tests "$@"

install:
    {{ rusty_v8_cargo }} --version
    {{ rusty_v8_cargo }} fetch

# Run `cargo nextest` since it's faster than `cargo test`, though including
# --no-fail-fast is important to ensure all tests are run.
#
# Run `cargo install --locked cargo-nextest` if you don't have it installed.
# Prefer this for routine local runs. Workspace crate features are banned, so
# there should be no need to add `--all-features`.
test:
    RUST_MIN_STACK={{ rust_min_stack }} {{ rusty_v8_cargo }} nextest run --no-fail-fast

# Build and run the legacy Bazel CLI target from source.
# Note we have to use the combination of `[no-cd]` and `--run_under="cd $PWD &&"`
# to ensure that Bazel runs the command in the current working directory.
[no-cd]
bazel-codex *args:
    bazel run //codex-rs/cli:codex --run_under="cd $PWD &&" -- "$@"

[no-cd]
bazel-lock-update:
    bazel mod deps --lockfile_mode=update

[no-cd]
bazel-lock-check:
    {{ justfile_directory() }}/scripts/check-module-bazel-lock.sh

bazel-test:
    bazel test --test_tag_filters=-argument-comment-lint //... --keep_going

[no-cd]
bazel-clippy:
    bazel_targets="$({{ justfile_directory() }}/scripts/list-bazel-clippy-targets.sh)" && bazel build --config=clippy -- ${bazel_targets}

[no-cd]
bazel-argument-comment-lint:
    bazel build --config=argument-comment-lint -- $({{ justfile_directory() }}/tools/argument-comment-lint/list-bazel-targets.sh)

bazel-remote-test:
    bazel test --test_tag_filters=-argument-comment-lint //... --config=remote --platforms=//:rbe --keep_going

build-for-release:
    bazel build //codex-rs/cli:release_runtime_binaries --config=remote

# Run the MCP server
mcp-server-run *args:
    {{ rusty_v8_cargo }} run -p codex-mcp-server -- "$@"

# Regenerate the json schema for config.toml from the current config types.
write-config-schema:
    {{ rusty_v8_cargo }} run -p codex-core --bin codex-write-config-schema

# Regenerate vendored app-server protocol schema artifacts.
write-app-server-schema *args:
    {{ rusty_v8_cargo }} run -p codex-app-server-protocol --bin write_schema_fixtures -- "$@"

[no-cd]
write-hooks-schema:
    {{ rusty_v8_cargo }} run --manifest-path {{ justfile_directory() }}/codex-rs/Cargo.toml -p codex-hooks --bin write_hooks_schema_fixtures

# Run the argument-comment Dylint checks across codex-rs.
[no-cd]
argument-comment-lint *args:
    if [ "$#" -eq 0 ]; then \
      bazel build --config=argument-comment-lint -- $({{ justfile_directory() }}/tools/argument-comment-lint/list-bazel-targets.sh); \
    else \
      {{ justfile_directory() }}/tools/argument-comment-lint/run-prebuilt-linter.py "$@"; \
    fi

[no-cd]
argument-comment-lint-from-source *args:
    {{ justfile_directory() }}/tools/argument-comment-lint/run.py "$@"

# Tail logs from the state SQLite database
log *args:
    if [ "${1:-}" = "--" ]; then shift; fi; {{ rusty_v8_cargo }} run -p codex-state --bin logs_client -- "$@"
