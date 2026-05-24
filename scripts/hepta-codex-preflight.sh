#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
NATIVE_MANIFEST="${HEPTA_NATIVE_MANIFEST:-apps/hepta-native/Cargo.toml}"
NATIVE_TARGET_DIR="${HEPTA_NATIVE_TARGET_DIR:-apps/hepta-native/target}"
RUN_NATIVE="${HEPTA_CODEX_PREFLIGHT_NATIVE:-1}"
RUN_RELEASE="${HEPTA_CODEX_PREFLIGHT_RELEASE:-0}"

export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

echo "[hepta-codex-preflight] metadata"
cargo metadata --offline --manifest-path "$MANIFEST" --no-deps --format-version 1 >/tmp/hepta-codex-preflight-metadata.json

echo "[hepta-codex-preflight] fmt"
cargo fmt --all --manifest-path "$MANIFEST" -- --check

echo "[hepta-codex-preflight] cargo check"
cargo check --offline --manifest-path "$MANIFEST" -q \
  -p hepta-core \
  -p hepta-intelligence \
  -p hepta-memory \
  -p hepta-plugins \
  -p hepta-runtime \
  -p hepta-gateway \
  -p codex-cli --bin hepta \
  -p hepta-cli --bin hepta

echo "[hepta-codex-preflight] adapter behavior-equivalence gate"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-runtime \
  codex_engine_adapter_behavior_equivalence_gate -- --nocapture
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta \
  hepta_codex_engine_adapter_boundary -- --nocapture

echo "[hepta-codex-preflight] adapter shadow-replay gate"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-runtime \
  all_adapter_shadow_replay -- --nocapture

echo "[hepta-codex-preflight] name/repository closure gate"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-runtime \
  name_repository_closure -- --nocapture
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta \
  hepta_name_repository_closure -- --nocapture

echo "[hepta-codex-preflight] active service dependency isolation gate"
HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_LIVE=0 \
  scripts/hepta-active-service-dependency-isolation.sh

echo "[hepta-codex-preflight] upstream Codex snapshot gate"
HEPTA_UPSTREAM_CODEX_SNAPSHOT_OBSERVE_REMOTE=0 \
  scripts/hepta-upstream-codex-snapshot.sh

echo "[hepta-codex-preflight] upstream Codex diff ledger gate"
scripts/hepta-upstream-codex-diff-ledger.sh

echo "[hepta-codex-preflight] upstream Codex product-governance absorption gate"
scripts/hepta-upstream-codex-product-governance-absorption.sh

echo "[hepta-codex-preflight] upstream Codex product-governance translation gate"
scripts/hepta-upstream-codex-product-governance-translation.sh

echo "[hepta-codex-preflight] upstream Codex release-governance promotion gate"
scripts/hepta-upstream-codex-release-governance-promotion.sh

echo "[hepta-codex-preflight] upstream Codex legacy compatibility absorption gate"
scripts/hepta-upstream-codex-legacy-compatibility-absorption.sh

echo "[hepta-codex-preflight] upstream Codex legacy compatibility replay gate"
scripts/hepta-upstream-codex-legacy-compatibility-replay.sh

echo "[hepta-codex-preflight] upstream Codex legacy compatibility promotion gate"
scripts/hepta-upstream-codex-legacy-compatibility-promotion.sh

echo "[hepta-codex-preflight] upstream Codex provider/security absorption gate"
scripts/hepta-upstream-codex-provider-security-absorption.sh

echo "[hepta-codex-preflight] upstream Codex provider/security replay gate"
scripts/hepta-upstream-codex-provider-security-replay.sh

echo "[hepta-codex-preflight] upstream Codex provider/security promotion gate"
scripts/hepta-upstream-codex-provider-security-promotion.sh

echo "[hepta-codex-preflight] upstream Codex runtime/app-server absorption gate"
scripts/hepta-upstream-codex-runtime-appserver-absorption.sh

echo "[hepta-codex-preflight] upstream Codex runtime/app-server replay gate"
scripts/hepta-upstream-codex-runtime-appserver-replay.sh

echo "[hepta-codex-preflight] upstream Codex runtime/app-server promotion gate"
scripts/hepta-upstream-codex-runtime-appserver-promotion.sh

echo "[hepta-codex-preflight] upstream Codex absorption/replay readiness gate"
scripts/hepta-upstream-codex-absorption-replay-readiness.sh

echo "[hepta-codex-preflight] upstream Codex promotion readiness gate"
scripts/hepta-upstream-codex-promotion-readiness.sh

echo "[hepta-codex-preflight] upstream Codex promotion closure gate"
scripts/hepta-upstream-codex-promotion-closure.sh

echo "[hepta-codex-preflight] upstream Codex active-wiring precondition gate"
scripts/hepta-upstream-codex-active-wiring-precondition.sh

echo "[hepta-codex-preflight] upstream Codex activation request packet gate"
scripts/hepta-upstream-codex-activation-request-packet.sh

echo "[hepta-codex-preflight] upstream Codex activation packet dry-run gate"
scripts/hepta-upstream-codex-activation-packet-dry-run.sh

echo "[hepta-codex-preflight] upstream Codex activation evidence ledger gate"
scripts/hepta-upstream-codex-activation-evidence-ledger.sh

echo "[hepta-codex-preflight] upstream Codex activation readiness closure gate"
scripts/hepta-upstream-codex-activation-readiness-closure.sh

echo "[hepta-codex-preflight] upstream Codex activation denied sample gate"
scripts/hepta-upstream-codex-activation-denied-sample.sh

echo "[hepta-codex-preflight] upstream Codex activation evidence freshness policy gate"
scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh

echo "[hepta-codex-preflight] upstream Codex activation evidence binding record gate"
scripts/hepta-upstream-codex-activation-evidence-binding-record.sh

echo "[hepta-codex-preflight] upstream Codex activation evidence denied fixture gate"
scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh

echo "[hepta-codex-preflight] upstream Codex activation trusted evidence acceptance matrix gate"
scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh

echo "[hepta-codex-preflight] upstream Codex activation trusted record shape validator gate"
scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh

echo "[hepta-codex-preflight] upstream Codex activation evidence completeness scoreboard gate"
scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh

echo "[hepta-codex-preflight] upstream Codex sync lane gate"
HEPTA_UPSTREAM_CODEX_SYNC_REQUIRE_LIVE=0 \
  scripts/hepta-upstream-codex-sync-lane.sh

echo "[hepta-codex-preflight] hepta-gateway tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-gateway

echo "[hepta-codex-preflight] codex-cli native tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta native_gateway -- --nocapture
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta native_telegram -- --nocapture
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta native_post -- --nocapture

echo "[hepta-codex-preflight] control-ui smoke"
CARGO_NET_OFFLINE=true scripts/hepta-control-ui-smoke.sh

if [[ "$RUN_NATIVE" == "1" ]]; then
  echo "[hepta-codex-preflight] native app metadata/check/tests"
  cargo metadata --offline --manifest-path "$NATIVE_MANIFEST" --no-deps --format-version 1 >/tmp/hepta-native-preflight-metadata.json
  CARGO_TARGET_DIR="$NATIVE_TARGET_DIR" cargo check --manifest-path "$NATIVE_MANIFEST"
  CARGO_TARGET_DIR="$NATIVE_TARGET_DIR" cargo test --manifest-path "$NATIVE_MANIFEST" hepta_ -- --nocapture
else
  echo "[hepta-codex-preflight] native app gates skipped (HEPTA_CODEX_PREFLIGHT_NATIVE=$RUN_NATIVE)"
fi

if [[ "$RUN_RELEASE" == "1" ]]; then
  echo "[hepta-codex-preflight] release build compatibility codex-cli"
  cargo build --release --offline --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta
  echo "[hepta-codex-preflight] release build active hepta-cli"
  cargo build --release --offline --manifest-path "$MANIFEST" -q -p hepta-cli --bin hepta
else
  echo "[hepta-codex-preflight] release build skipped (set HEPTA_CODEX_PREFLIGHT_RELEASE=1)"
fi

echo "[hepta-codex-preflight] whitespace/status"
git diff --check
git status -sb

echo "Hepta Codex preflight passed"
