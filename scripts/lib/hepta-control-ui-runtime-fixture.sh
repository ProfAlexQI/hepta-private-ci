#!/usr/bin/env bash

# Deterministic local databases and keys for browser-only Control UI gates.
# Source scripts/lib/hepta-ui-rust-toolchain.sh before calling start_server.

hepta_control_ui_runtime_fixture_init() {
  HEPTA_CONTROL_UI_RUNTIME_FIXTURE_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-control-ui-runtime.XXXXXX")"
  HEPTA_CONTROL_UI_RUNTIME_DATABASE="$HEPTA_CONTROL_UI_RUNTIME_FIXTURE_DIR/outcomes.sqlite3"
  HEPTA_CONTROL_UI_RUNTIME_STATE_DATABASE="$HEPTA_CONTROL_UI_RUNTIME_FIXTURE_DIR/runtime-state.json"
  HEPTA_CONTROL_UI_RUNTIME_KEY_FILE="$HEPTA_CONTROL_UI_RUNTIME_FIXTURE_DIR/integrity.key"
  HEPTA_CONTROL_UI_PREFERENCE_DATABASE="$HEPTA_CONTROL_UI_RUNTIME_FIXTURE_DIR/preferences.sqlite3"
  HEPTA_CONTROL_UI_PREFERENCE_INTEGRITY_KEY_FILE="$HEPTA_CONTROL_UI_RUNTIME_FIXTURE_DIR/preference-integrity.key"
  HEPTA_CONTROL_UI_PREFERENCE_AUTH_KEY_FILE="$HEPTA_CONTROL_UI_RUNTIME_FIXTURE_DIR/preference-auth.key"

  chmod 700 "$HEPTA_CONTROL_UI_RUNTIME_FIXTURE_DIR"
  (umask 077; printf '%s' '000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f' >"$HEPTA_CONTROL_UI_RUNTIME_KEY_FILE")
  (umask 077; printf '%s' '202122232425262728292a2b2c2d2e2f303132333435363738393a3b3c3d3e3f' >"$HEPTA_CONTROL_UI_PREFERENCE_INTEGRITY_KEY_FILE")
  (umask 077; printf '%s' '404142434445464748494a4b4c4d4e4f505152535455565758595a5b5c5d5e5f' >"$HEPTA_CONTROL_UI_PREFERENCE_AUTH_KEY_FILE")
  chmod 600 \
    "$HEPTA_CONTROL_UI_RUNTIME_KEY_FILE" \
    "$HEPTA_CONTROL_UI_PREFERENCE_INTEGRITY_KEY_FILE" \
    "$HEPTA_CONTROL_UI_PREFERENCE_AUTH_KEY_FILE"
}

hepta_control_ui_runtime_fixture_start_server() {
  local manifest="$1"
  local bind_addr="$2"
  local server_log="$3"
  local outcome_mode="bootstrap-new"
  local preference_mode="bootstrap-new"

  if [[ -z "${HEPTA_CONTROL_UI_RUNTIME_FIXTURE_DIR:-}" ]]; then
    hepta_control_ui_runtime_fixture_init
  fi

  if [[ -e "$HEPTA_CONTROL_UI_RUNTIME_DATABASE" ]]; then
    outcome_mode="open-existing"
  fi
  if [[ -e "$HEPTA_CONTROL_UI_PREFERENCE_DATABASE" ]]; then
    preference_mode="open-existing"
  fi

  : >"$server_log"
  HEPTA_AUTOLOAD=0 \
    HEPTA_AUTOSAVE=0 \
    CARGO_INCREMENTAL=0 \
    HEPTA_RUNTIME_OUTCOME_DATABASE="$HEPTA_CONTROL_UI_RUNTIME_DATABASE" \
    HEPTA_RUNTIME_STATE_DATABASE="$HEPTA_CONTROL_UI_RUNTIME_STATE_DATABASE" \
    HEPTA_RUNTIME_INTEGRITY_KEY_FILE="$HEPTA_CONTROL_UI_RUNTIME_KEY_FILE" \
    HEPTA_RUNTIME_OUTCOME_MODE="$outcome_mode" \
    HEPTA_PREFERENCE_DATABASE="$HEPTA_CONTROL_UI_PREFERENCE_DATABASE" \
    HEPTA_PREFERENCE_INTEGRITY_KEY_FILE="$HEPTA_CONTROL_UI_PREFERENCE_INTEGRITY_KEY_FILE" \
    HEPTA_PREFERENCE_INGRESS_AUTH_KEY_FILE="$HEPTA_CONTROL_UI_PREFERENCE_AUTH_KEY_FILE" \
    HEPTA_PREFERENCE_STORE_MODE="$preference_mode" \
    hepta_ui_cargo run --manifest-path "$manifest" -q -p hepta-cli --bin hepta -- --serve-ui "$bind_addr" \
    >"$server_log" 2>&1 &
  server_pid="$!"
}

hepta_control_ui_runtime_fixture_cleanup() {
  local fixture_dir="${HEPTA_CONTROL_UI_RUNTIME_FIXTURE_DIR:-}"
  local fixture_prefix="${TMPDIR:-/tmp}/hepta-control-ui-runtime."
  [[ -n "$fixture_dir" ]] || return 0
  case "$fixture_dir" in
    "$fixture_prefix"*)
      rm -rf "$fixture_dir"
      HEPTA_CONTROL_UI_RUNTIME_FIXTURE_DIR=""
      ;;
    *)
      echo "refusing to remove unexpected Control UI fixture directory: $fixture_dir" >&2
      return 1
      ;;
  esac
}
