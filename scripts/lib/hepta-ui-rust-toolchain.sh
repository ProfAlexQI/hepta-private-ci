#!/usr/bin/env bash

# Shared, deterministic Rust entry points for UI gates. The repository-wide
# default toolchain may lag the SQLx MSRV, so UI scripts must not inherit it.
HEPTA_UI_RUST_TOOLCHAIN_VERSION="${HEPTA_UI_RUST_TOOLCHAIN_VERSION:-1.95.0}"

hepta_ui_activate_rust_toolchain() {
  hepta_ui_require_rust_toolchain || return 1
  local rustc_path
  local rustdoc_path
  rustc_path="$(rustup which --toolchain "$HEPTA_UI_RUST_TOOLCHAIN_VERSION" rustc)"
  rustdoc_path="$(rustup which --toolchain "$HEPTA_UI_RUST_TOOLCHAIN_VERSION" rustdoc)"
  export PATH="$(dirname "$rustc_path"):$PATH"
  export RUSTC="$rustc_path"
  export RUSTDOC="$rustdoc_path"
  export RUSTUP_TOOLCHAIN="$HEPTA_UI_RUST_TOOLCHAIN_VERSION"
}

hepta_ui_require_rust_toolchain() {
  if ! command -v rustup >/dev/null 2>&1; then
    echo "error: rustup is required for Hepta UI gates" >&2
    return 1
  fi

  if ! rustup run "$HEPTA_UI_RUST_TOOLCHAIN_VERSION" rustc --version >/dev/null 2>&1; then
    echo "error: Rust $HEPTA_UI_RUST_TOOLCHAIN_VERSION is required; install it with: rustup toolchain install $HEPTA_UI_RUST_TOOLCHAIN_VERSION" >&2
    return 1
  fi
}

hepta_ui_cargo() {
  hepta_ui_activate_rust_toolchain || return 1
  cargo "$@"
}

hepta_ui_rustc() {
  hepta_ui_require_rust_toolchain || return 1
  rustup run "$HEPTA_UI_RUST_TOOLCHAIN_VERSION" rustc "$@"
}
