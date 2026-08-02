#!/usr/bin/env bash

hepta_rust_toolchain_init() {
  local repo_root="$1"
  local manifest_toolchain
  local toolchain_manifest

  toolchain_manifest="${HEPTA_RUST_TOOLCHAIN_MANIFEST:-$repo_root/codex-rs/rust-toolchain.toml}"
  [[ -f "$toolchain_manifest" ]] || {
    echo "missing pinned Rust toolchain manifest: $toolchain_manifest" >&2
    return 2
  }
  manifest_toolchain="$(
    sed -n 's/^channel = "\([^"]*\)"$/\1/p' "$toolchain_manifest" | head -n 1
  )"
  [[ -n "$manifest_toolchain" ]] || {
    echo "unable to read pinned Rust channel from: $toolchain_manifest" >&2
    return 2
  }

  HEPTA_PINNED_RUST_TOOLCHAIN="${HEPTA_RUST_TOOLCHAIN:-$manifest_toolchain}"
  command -v rustup >/dev/null 2>&1 || {
    echo "rustup is required for pinned Rust $HEPTA_PINNED_RUST_TOOLCHAIN" >&2
    return 2
  }

  HEPTA_PINNED_RUSTC="$(rustup which --toolchain "$HEPTA_PINNED_RUST_TOOLCHAIN" rustc)"
  HEPTA_PINNED_RUSTDOC="$(rustup which --toolchain "$HEPTA_PINNED_RUST_TOOLCHAIN" rustdoc)"
  HEPTA_PINNED_RUST_BIN="$(dirname "$HEPTA_PINNED_RUSTC")"
  export HEPTA_PINNED_RUST_TOOLCHAIN HEPTA_PINNED_RUSTC HEPTA_PINNED_RUSTDOC
  export RUSTUP_TOOLCHAIN="$HEPTA_PINNED_RUST_TOOLCHAIN"
  export RUSTC="$HEPTA_PINNED_RUSTC"
  export RUSTDOC="$HEPTA_PINNED_RUSTDOC"
  export PATH="$HEPTA_PINNED_RUST_BIN:$PATH"
}

hepta_rust_toolchain_cargo() {
  exec rustup run "$HEPTA_PINNED_RUST_TOOLCHAIN" cargo "$@"
}
