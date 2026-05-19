#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

export HEPTA_AUTOLOAD=0
export HEPTA_AUTOSAVE=0

cargo test -q -p hepta-core control_ui_report_is_complete_and_asset_backed
cargo run -q -p hepta --bin hepta -- /control-ui --json >/tmp/hepta-control-ui.json
cargo run -q -p hepta --bin hepta -- /ui-contract-audit --json >/tmp/hepta-ui-contract-audit.json
cargo run -q -p hepta --bin hepta -- /operator-snapshot --json >/tmp/hepta-operator-snapshot.json
cargo run -q -p hepta --bin hepta -- /operator-security --json >/tmp/hepta-operator-security.json
cargo run -q -p hepta --bin hepta -- /ui-action-plan gateway-dispatch --dry-run --json >/tmp/hepta-ui-action-plan.json

echo "Hepta Control UI hardening smoke passed (Rust-native retired Node suite)"
echo "Hepta Control UI Rust/no-JS contract smoke passed"
