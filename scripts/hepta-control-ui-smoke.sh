#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

export HEPTA_AUTOLOAD=0
export HEPTA_AUTOSAVE=0
export CARGO_INCREMENTAL=0
export RUST_MIN_STACK="${RUST_MIN_STACK:-33554432}"

MANIFEST="codex-rs/Cargo.toml"

cargo test --manifest-path "${MANIFEST}" -q -p hepta-core control_ui_report_is_complete_and_asset_backed
cargo test --manifest-path "${MANIFEST}" -q -p hepta-core operator_security_report_reaches_local_100_without_external_claims
cargo test --manifest-path "${MANIFEST}" -q -p hepta-gateway native_post_execution_readiness_report_is_gateway_owned
cargo test --manifest-path "${MANIFEST}" -q -p codex-cli --bin hepta native_gateway

if [[ "${HEPTA_CONTROL_UI_SKIP_BROWSER_SMOKE:-0}" != "1" ]]; then
  ./scripts/hepta-control-ui-browser-smoke.sh
fi

echo "Hepta Control UI hardening smoke passed (Rust-native retired Node suite)"
echo "Hepta Control UI Rust/no-JS contract smoke passed"
