#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_MANIFEST:-${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}}"
NATIVE_MANIFEST="${HEPTA_NATIVE_MANIFEST:-apps/hepta-native/Cargo.toml}"
NATIVE_TARGET_DIR="${HEPTA_NATIVE_TARGET_DIR:-apps/hepta-native/target}"
RUN_NATIVE="${HEPTA_PREFLIGHT_NATIVE:-${HEPTA_CODEX_PREFLIGHT_NATIVE:-1}}"
RUN_RELEASE="${HEPTA_PREFLIGHT_RELEASE:-${HEPTA_CODEX_PREFLIGHT_RELEASE:-0}}"

export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

echo "[hepta-preflight] metadata"
cargo metadata --offline --manifest-path "$MANIFEST" --no-deps --format-version 1 >/tmp/hepta-preflight-metadata.json

echo "[hepta-preflight] fmt"
cargo fmt --all --manifest-path "$MANIFEST" -- --check

echo "[hepta-preflight] cargo check"
cargo check --offline --manifest-path "$MANIFEST" -q \
  -p hepta-core \
  -p hepta-intelligence \
  -p hepta-memory \
  -p hepta-plugins \
  -p hepta-runtime \
  -p hepta-gateway \
  -p codex-cli --bin hepta \
  -p hepta-cli --bin hepta

echo "[hepta-preflight] adapter behavior-equivalence gate"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-runtime \
  codex_engine_adapter_behavior_equivalence_gate -- --nocapture
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta \
  hepta_codex_engine_adapter_boundary -- --nocapture

echo "[hepta-preflight] adapter shadow-replay gate"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-runtime \
  all_adapter_shadow_replay -- --nocapture

echo "[hepta-preflight] name/repository closure gate"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-runtime \
  name_repository_closure -- --nocapture
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta \
  hepta_name_repository_closure -- --nocapture

echo "[hepta-preflight] active service dependency isolation gate"
HEPTA_ACTIVE_SERVICE_DEPENDENCY_ISOLATION_LIVE=0 \
  scripts/hepta-active-service-dependency-isolation.sh

echo "[hepta-preflight] legacy preflight entrypoint migration gate"
scripts/hepta-preflight-entrypoint-migration.sh

echo "[hepta-preflight] legacy watchdog entrypoint migration gate"
scripts/hepta-watchdog-entrypoint-migration.sh

echo "[hepta-preflight] legacy live gates entrypoint migration gate"
scripts/hepta-live-gates-entrypoint-migration.sh

echo "[hepta-preflight] legacy release/readiness entrypoint migration gate"
scripts/hepta-release-readiness-entrypoint-migration.sh

echo "[hepta-preflight] legacy inventory entrypoint migration gate"
scripts/hepta-inventory-entrypoint-migration.sh

echo "[hepta-preflight] memory-rem status closure gate"
scripts/hepta-memory-rem-status-closure.sh

echo "[hepta-preflight] memory-tools catalog closure gate"
scripts/hepta-memory-tools-catalog-closure.sh

echo "[hepta-preflight] native residual runtime status closure gate"
scripts/hepta-native-residual-runtime-status-closure.sh

echo "[hepta-preflight] plugin migration plan closure gate"
scripts/hepta-plugin-migration-plan-closure.sh

echo "[hepta-preflight] skill workshop plan closure gate"
scripts/hepta-skill-workshop-plan-closure.sh

echo "[hepta-preflight] memory/intelligence closure gate"
scripts/hepta-memory-intelligence-closure.sh

echo "[hepta-preflight] live mutation governance gate"
scripts/hepta-live-mutation-governance-gate.sh

echo "[hepta-preflight] live mutation rollback drill gate"
scripts/hepta-live-mutation-rollback-drill-gate.sh

echo "[hepta-preflight] live mutation approval evidence receipt gate"
scripts/hepta-live-mutation-approval-evidence-receipt-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence denial gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-denial-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence approval packet gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-approval-packet-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence operator scope binding gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-operator-scope-binding-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence no-secret payload review gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-no-secret-payload-review-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction proof gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-proof-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance matrix gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-matrix-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt command contract gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-command-contract-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt invocation dry-run gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-invocation-dry-run-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt no-write sink contract gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-no-write-sink-contract-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt write-enable fixture gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-write-enable-fixture-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt materialization dry-run gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-materialization-dry-run-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence approval packet gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-approval-packet-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem output path allowlist gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-output-path-allowlist-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem output path evidence binding gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-output-path-evidence-binding-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem sink write preview gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-sink-write-preview-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence execution denial matrix gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-execution-denial-matrix-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence dry-run ledger gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-dry-run-ledger-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger shape approval gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-shape-approval-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal denial gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-denial-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt contract gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-contract-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard review gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard review acceptance gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard review acceptance readiness gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard review acceptance readiness denial review gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard review acceptance readiness denial review acceptance gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-acceptance-gate.sh

echo "[hepta-preflight] live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard review acceptance readiness denial review acceptance closure gate"
scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard-review-acceptance-readiness-denial-review-acceptance-closure-gate.sh

echo "[hepta-preflight] readiness denial review acceptance closure summary gate"
scripts/hepta-readiness-denial-review-acceptance-closure-summary-gate.sh

echo "[hepta-preflight] upstream Codex snapshot gate"
HEPTA_UPSTREAM_CODEX_SNAPSHOT_OBSERVE_REMOTE=0 \
  scripts/hepta-upstream-codex-snapshot.sh

echo "[hepta-preflight] upstream Codex diff ledger gate"
scripts/hepta-upstream-codex-diff-ledger.sh

echo "[hepta-preflight] upstream Codex doctor environment diagnostics absorption gate"
scripts/hepta-upstream-codex-doctor-environment-diagnostics-absorption.sh

echo "[hepta-preflight] upstream Codex product-governance absorption gate"
scripts/hepta-upstream-codex-product-governance-absorption.sh

echo "[hepta-preflight] upstream Codex product-governance translation gate"
scripts/hepta-upstream-codex-product-governance-translation.sh

echo "[hepta-preflight] upstream Codex release-governance promotion gate"
scripts/hepta-upstream-codex-release-governance-promotion.sh

echo "[hepta-preflight] upstream Codex legacy compatibility absorption gate"
scripts/hepta-upstream-codex-legacy-compatibility-absorption.sh

echo "[hepta-preflight] upstream Codex legacy compatibility replay gate"
scripts/hepta-upstream-codex-legacy-compatibility-replay.sh

echo "[hepta-preflight] upstream Codex legacy compatibility promotion gate"
scripts/hepta-upstream-codex-legacy-compatibility-promotion.sh

echo "[hepta-preflight] upstream Codex provider/security absorption gate"
scripts/hepta-upstream-codex-provider-security-absorption.sh

echo "[hepta-preflight] upstream Codex provider/security replay gate"
scripts/hepta-upstream-codex-provider-security-replay.sh

echo "[hepta-preflight] upstream Codex provider/security promotion gate"
scripts/hepta-upstream-codex-provider-security-promotion.sh

echo "[hepta-preflight] upstream Codex runtime/app-server absorption gate"
scripts/hepta-upstream-codex-runtime-appserver-absorption.sh

echo "[hepta-preflight] upstream Codex runtime/app-server replay gate"
scripts/hepta-upstream-codex-runtime-appserver-replay.sh

echo "[hepta-preflight] upstream Codex runtime/app-server promotion gate"
scripts/hepta-upstream-codex-runtime-appserver-promotion.sh

echo "[hepta-preflight] upstream Codex absorption/replay readiness gate"
scripts/hepta-upstream-codex-absorption-replay-readiness.sh

echo "[hepta-preflight] upstream Codex promotion readiness gate"
scripts/hepta-upstream-codex-promotion-readiness.sh

echo "[hepta-preflight] upstream Codex promotion closure gate"
scripts/hepta-upstream-codex-promotion-closure.sh

echo "[hepta-preflight] upstream Codex active-wiring precondition gate"
scripts/hepta-upstream-codex-active-wiring-precondition.sh

echo "[hepta-preflight] upstream Codex activation request packet gate"
scripts/hepta-upstream-codex-activation-request-packet.sh

echo "[hepta-preflight] upstream Codex activation packet dry-run gate"
scripts/hepta-upstream-codex-activation-packet-dry-run.sh

echo "[hepta-preflight] upstream Codex activation evidence ledger gate"
scripts/hepta-upstream-codex-activation-evidence-ledger.sh

echo "[hepta-preflight] upstream Codex activation readiness closure gate"
scripts/hepta-upstream-codex-activation-readiness-closure.sh

echo "[hepta-preflight] upstream Codex activation denied sample gate"
scripts/hepta-upstream-codex-activation-denied-sample.sh

echo "[hepta-preflight] upstream Codex activation evidence freshness policy gate"
scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh

echo "[hepta-preflight] upstream Codex activation evidence binding record gate"
scripts/hepta-upstream-codex-activation-evidence-binding-record.sh

echo "[hepta-preflight] upstream Codex activation evidence denied fixture gate"
scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh

echo "[hepta-preflight] upstream Codex activation trusted evidence acceptance matrix gate"
scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh

echo "[hepta-preflight] upstream Codex activation trusted record shape validator gate"
scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh

echo "[hepta-preflight] upstream Codex activation evidence completeness scoreboard gate"
scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh

echo "[hepta-preflight] upstream Codex activation evidence recording dry-run receipt gate"
scripts/hepta-upstream-codex-activation-evidence-recording-dry-run-receipt.sh

echo "[hepta-preflight] upstream Codex activation evidence recording denial matrix gate"
scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh

echo "[hepta-preflight] upstream Codex activation evidence receipt persistence command contract gate"
scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh

echo "[hepta-preflight] upstream Codex activation evidence receipt persistence invocation dry-run gate"
scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh

echo "[hepta-preflight] upstream Codex activation evidence receipt no-write sink adapter contract gate"
scripts/hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract.sh

echo "[hepta-preflight] upstream Codex activation evidence receipt write-enable fixture gate"
scripts/hepta-upstream-codex-activation-evidence-receipt-write-enable-fixture.sh

echo "[hepta-preflight] upstream Codex activation evidence receipt materialization dry-run gate"
scripts/hepta-upstream-codex-activation-evidence-receipt-materialization-dry-run.sh

echo "[hepta-preflight] upstream Codex activation evidence receipt filesystem persistence approval packet gate"
scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh

echo "[hepta-preflight] upstream Codex activation evidence receipt filesystem output path allowlist gate"
scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh

echo "[hepta-preflight] upstream Codex activation evidence receipt filesystem output path evidence binding gate"
scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh

echo "[hepta-preflight] upstream Codex activation evidence receipt filesystem sink write preview gate"
scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh

echo "[hepta-preflight] upstream Codex activation evidence receipt filesystem persistence execution denial matrix gate"
scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix.sh

echo "[hepta-preflight] upstream Codex sync lane gate"
HEPTA_UPSTREAM_CODEX_SYNC_REQUIRE_LIVE=0 \
  scripts/hepta-upstream-codex-sync-lane.sh

echo "[hepta-preflight] terminal denial index gate"
scripts/hepta-terminal-denial-index-gate.sh

echo "[hepta-preflight] terminal non-activation release-claim index gate"
scripts/hepta-terminal-non-activation-release-claim-index-gate.sh

echo "[hepta-preflight] terminal operator-readiness non-approval index gate"
scripts/hepta-terminal-operator-readiness-non-approval-index-gate.sh

echo "[hepta-preflight] terminal governance closure summary gate"
scripts/hepta-terminal-governance-closure-summary-gate.sh

echo "[hepta-preflight] terminal governance active-state lock gate"
scripts/hepta-terminal-governance-active-state-lock-gate.sh

echo "[hepta-preflight] hepta-gateway tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-gateway

echo "[hepta-preflight] codex-cli native tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta native_gateway -- --nocapture
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta native_telegram -- --nocapture
cargo test --offline --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta native_post -- --nocapture

echo "[hepta-preflight] control-ui smoke"
CARGO_NET_OFFLINE=true scripts/hepta-control-ui-smoke.sh

if [[ "$RUN_NATIVE" == "1" ]]; then
  echo "[hepta-preflight] native app metadata/check/tests"
  cargo metadata --offline --manifest-path "$NATIVE_MANIFEST" --no-deps --format-version 1 >/tmp/hepta-native-preflight-metadata.json
  CARGO_TARGET_DIR="$NATIVE_TARGET_DIR" cargo check --manifest-path "$NATIVE_MANIFEST"
  CARGO_TARGET_DIR="$NATIVE_TARGET_DIR" cargo test --manifest-path "$NATIVE_MANIFEST" hepta_ -- --nocapture
else
  echo "[hepta-preflight] native app gates skipped (HEPTA_PREFLIGHT_NATIVE=$RUN_NATIVE)"
fi

if [[ "$RUN_RELEASE" == "1" ]]; then
  echo "[hepta-preflight] release build compatibility codex-cli"
  cargo build --release --offline --manifest-path "$MANIFEST" -q -p codex-cli --bin hepta
  echo "[hepta-preflight] release build active hepta-cli"
  cargo build --release --offline --manifest-path "$MANIFEST" -q -p hepta-cli --bin hepta
else
  echo "[hepta-preflight] release build skipped (set HEPTA_PREFLIGHT_RELEASE=1)"
fi

echo "[hepta-preflight] whitespace/status"
git diff --check
git status -sb

echo "Hepta preflight passed"
