#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_MANIFEST:-${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}}"
NATIVE_MANIFEST="${HEPTA_NATIVE_MANIFEST:-apps/hepta-native/Cargo.toml}"
NATIVE_TARGET_DIR="${HEPTA_NATIVE_TARGET_DIR:-apps/hepta-native/target}"
RUN_NATIVE="${HEPTA_PREFLIGHT_NATIVE:-${HEPTA_CODEX_PREFLIGHT_NATIVE:-1}}"
RUN_RELEASE="${HEPTA_PREFLIGHT_RELEASE:-${HEPTA_CODEX_PREFLIGHT_RELEASE:-0}}"

export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

HEPTA_PREFLIGHT_CREATED_JSON_REPORT_CAPTURE_CACHE_DIR=0
if [[ "${HEPTA_JSON_REPORT_CAPTURE_CACHE:-1}" != "0" \
  && -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$(mktemp -d /tmp/hepta-json-report-capture.XXXXXX)"
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_SALT="hepta-preflight:$$:${HEPTA_RELEASE_BIN:-}:${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-}:${RUN_NATIVE}:${RUN_RELEASE}"
  HEPTA_PREFLIGHT_CREATED_JSON_REPORT_CAPTURE_CACHE_DIR=1
  trap 'if [[ "${HEPTA_PREFLIGHT_CREATED_JSON_REPORT_CAPTURE_CACHE_DIR:-0}" == "1" ]]; then rm -rf "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}"; fi' EXIT
fi

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

echo "[hepta-preflight] KG prompt-preview preflight gate"
scripts/hepta-kg-prompt-preview-preflight-gate.sh

echo "[hepta-preflight] KG prompt-preview terminal summary gate"
scripts/hepta-kg-prompt-preview-terminal-summary-gate.sh

echo "[hepta-preflight] KG prompt-preview operator briefing non-persistence gate"
scripts/hepta-kg-prompt-preview-operator-briefing-non-persistence-gate.sh

echo "[hepta-preflight] KG prompt-preview readiness next-action index gate"
scripts/hepta-kg-prompt-preview-readiness-next-action-index-gate.sh

echo "[hepta-preflight] KG prompt-preview operator approval checklist schema gate"
scripts/hepta-kg-prompt-preview-operator-approval-checklist-schema-gate.sh

echo "[hepta-preflight] KG prompt-preview rollback/kill-switch evidence checklist gate"
scripts/hepta-kg-prompt-preview-rollback-kill-switch-evidence-checklist-gate.sh

echo "[hepta-preflight] KG prompt-preview redacted diff review checklist gate"
scripts/hepta-kg-prompt-preview-redacted-diff-review-checklist-gate.sh

echo "[hepta-preflight] KG prompt-preview context handoff checklist gate"
scripts/hepta-kg-prompt-preview-context-handoff-checklist-gate.sh

echo "[hepta-preflight] KG prompt-preview terminal next-action activation denial summary gate"
scripts/hepta-kg-prompt-preview-terminal-next-action-activation-denial-summary-gate.sh

echo "[hepta-preflight] KG prompt-preview memory/intelligence full enablement activation readiness gate"
scripts/hepta-memory-intelligence-kg-full-enablement-activation-readiness-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement memory live mutation staging fixture gate"
scripts/hepta-memory-intelligence-kg-full-enablement-memory-live-mutation-staging-fixture-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement KG external adapter staging receipt gate"
scripts/hepta-memory-intelligence-kg-full-enablement-kg-external-adapter-staging-receipt-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement bounded prompt-preview context handoff activation packet gate"
scripts/hepta-memory-intelligence-kg-full-enablement-bounded-prompt-preview-context-handoff-activation-packet-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router context attachment staging gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-context-attachment-staging-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement positive activation packet dry-run scaffold gate"
scripts/hepta-memory-intelligence-kg-full-enablement-positive-activation-packet-dry-run-scaffold-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement positive activation packet validator scoreboard gate"
scripts/hepta-memory-intelligence-kg-full-enablement-positive-activation-packet-validator-scoreboard-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement canary live harness scaffold gate"
scripts/hepta-memory-intelligence-kg-full-enablement-canary-live-harness-scaffold-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement explicit operator-approved canary packet record scaffold gate"
scripts/hepta-memory-intelligence-kg-full-enablement-explicit-operator-approved-canary-packet-record-scaffold-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary packet value fixture scoreboard gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-packet-value-fixture-scoreboard-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary arm plan dry-run gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-arm-plan-dry-run-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary arm readiness scoreboard gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-arm-readiness-scoreboard-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary dispatch envelope preview gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-dispatch-envelope-preview-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request payload preview no-write sink gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-preview-no-write-sink-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request payload readback audit receipt preview gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-preview-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request payload readback audit receipt acceptance packet dry-run scaffold gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-acceptance-packet-dry-run-scaffold-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request payload readback audit receipt acceptance packet value scoreboard gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-acceptance-packet-value-scoreboard-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request payload readback audit receipt trusted operator acceptance record scaffold gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-scaffold-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request payload readback audit receipt trusted operator acceptance record intake validator gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-intake-validator-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request payload readback audit receipt trusted operator acceptance record negative fixture matrix gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-negative-fixture-matrix-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request payload readback audit receipt trusted operator acceptance record positive precondition scoreboard gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-positive-precondition-scoreboard-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request payload readback audit receipt trusted operator acceptance record template gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-template-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request payload readback audit receipt trusted operator acceptance record readiness lock gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-readiness-lock-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request payload readback audit receipt trusted operator acceptance record controlled request dispatch envelope lock validator gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-payload-readback-audit-receipt-trusted-operator-acceptance-record-controlled-request-dispatch-envelope-lock-validator-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness no-dispatch readback audit scoreboard gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-no-dispatch-readback-audit-scoreboard-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness redacted payload preview no-materialization gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-redacted-payload-preview-no-materialization-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness readback/audit receipt hash preview acceptance skeleton gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-readback-audit-receipt-hash-preview-acceptance-skeleton-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness single-budget dispatch dry-run no-op receipt gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review/readback index no-persistence gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement non-acceptance gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation request denial matrix gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command no-op handoff gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-noop-handoff-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt no-persistence gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-no-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt replay idempotency denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt ordering monotonicity denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt cancellation supersession denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt audit trail immutable evidence denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt retention expiry garbage collection denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt export query observability denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt operator-facing summary briefing non-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt final operator acknowledgement non-acceptance denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt terminal operator decision public-claim non-promotion denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt release artifact publication denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt release artifact publication result receipt no-persistence gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation readiness index gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-readiness-index-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation readiness index replay/idempotency denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template non-acceptance authority replay denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-non-acceptance-authority-replay-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template field validation denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-field-validation-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template section completion non-acceptance gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet assembly non-acceptance gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-assembly-non-acceptance-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt non-persistence gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt replay/idempotency denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt ordering/monotonicity denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt cancellation/supersession denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt audit-trail/immutable-evidence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt retention/expiry/garbage-collection denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-retention-expiry-garbage-collection-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt export/query/observability denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt redaction/privacy/payload-exposure denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt operator briefing non-persistence gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt final acknowledgement non-acceptance gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt terminal decision/status promotion denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-terminal-decision-status-promotion-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt no-persistence gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt replay/idempotency denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt ordering/monotonicity denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt cancellation/supersession denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt audit-trail/immutable-evidence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-audit-trail-immutable-evidence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt retention/expiry/garbage-collection denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-retention-expiry-garbage-collection-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt export/query/observability denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt operator-facing summary/briefing non-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt final operator acknowledgement non-acceptance denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal decision/status promotion denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-decision-status-promotion-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal public claim/status exposure denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-public-claim-status-exposure-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution queue/artifact availability status denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-queue-artifact-availability-status-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt/external delivery non-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt query/export/observability denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-query-export-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt privacy/redaction/payload-exposure denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-privacy-redaction-exposure-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt operator briefing non-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-operator-briefing-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt final operator acknowledgement non-acceptance denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt terminal decision/status promotion denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-terminal-decision-status-promotion-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt terminal public claim/status exposure denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-terminal-public-claim-status-exposure-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt package/release channel status exposure denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-package-release-channel-status-exposure-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt distribution artifact/manifest status denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact distribution signing/notarization surface denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-distribution-signing-notarization-surface-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt no-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-delivery-receipt-artifact-download-install-affordance-result-receipt-no-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt replay/idempotency denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt ordering/monotonicity denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt cancellation/supersession denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt audit-trail/immutable-evidence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-audit-trail-immutable-evidence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt retention/expiry/garbage-collection denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt export/query/observability denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator-facing summary/briefing non-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt final operator acknowledgement non-acceptance denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt terminal decision/status promotion denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-terminal-decision-status-promotion-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator intent/consent reconfirmation denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-reconfirmation-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session binding denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-binding-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session replay/cross-binding denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-replay-cross-binding-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement ordering/monotonicity denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement cancellation/supersession denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement audit/evidence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-audit-evidence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement retention/expiry/garbage-collection denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-retention-expiry-garbage-collection-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement export/query/observability denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator-facing summary/briefing non-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-facing-summary-briefing-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement final operator acknowledgement non-acceptance denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-final-operator-acknowledgement-non-acceptance-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement terminal decision/status promotion denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-decision-status-promotion-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent reconfirmation denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-reconfirmation-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence export/query/observability denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router context attachment negative fixture matrix gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-context-attachment-negative-fixture-matrix-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router readback receipt skeleton gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-readback-receipt-skeleton-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router receipt observability denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-receipt-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router operator-facing summary non-persistence gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-facing-summary-non-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router operator acknowledgement non-acceptance gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation request denial matrix gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command no-op handoff gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt no-persistence gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt replay idempotency denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt ordering monotonicity denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt cancellation supersession denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt audit trail immutable evidence denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt retention expiry garbage collection denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt export query observability denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt operator-facing summary briefing non-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt final operator acknowledgement non-acceptance denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt terminal operator decision public-claim non-promotion denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt release artifact publication denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial-gate.sh

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

echo "[hepta-preflight] memory live mutation operator write contract gate"
scripts/hepta-memory-live-mutation-operator-write-contract-gate.sh

echo "[hepta-preflight] memory live mutation operator write approval packet gate"
scripts/hepta-memory-live-mutation-operator-write-approval-packet-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution preflight gate"
scripts/hepta-memory-live-mutation-operator-write-execution-preflight-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution denial matrix gate"
scripts/hepta-memory-live-mutation-operator-write-execution-denial-matrix-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution no-write sink contract gate"
scripts/hepta-memory-live-mutation-operator-write-execution-no-write-sink-contract-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution write-enable fixture gate"
scripts/hepta-memory-live-mutation-operator-write-execution-write-enable-fixture-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution post-write validation dry-run gate"
scripts/hepta-memory-live-mutation-operator-write-execution-post-write-validation-dry-run-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution post-write operator acceptance denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-post-write-operator-acceptance-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation closure denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-closure-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command no-op handoff gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-noop-handoff-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt no-persistence gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-no-persistence-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt replay idempotency denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt ordering monotonicity denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt cancellation supersession denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt audit trail immutable evidence denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt retention expiry garbage collection denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt export query observability denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt operator-facing summary briefing non-persistence denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt final operator acknowledgement non-acceptance denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt terminal operator decision public-claim non-promotion denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt release artifact publication denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-release-artifact-publication-denial-gate.sh

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

echo "[hepta-preflight] upstream Codex latest multi-surface absorption gate"
scripts/hepta-upstream-codex-latest-multisurface-absorption.sh

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

echo "[hepta-preflight] terminal release artifact non-write lock gate"
scripts/hepta-terminal-release-artifact-non-write-lock-gate.sh

echo "[hepta-preflight] terminal public distribution non-publication lock gate"
scripts/hepta-terminal-public-distribution-non-publication-lock-gate.sh

echo "[hepta-preflight] terminal publication evidence non-persistence summary gate"
scripts/hepta-terminal-publication-evidence-non-persistence-summary-gate.sh

echo "[hepta-preflight] terminal release-governance final audit index gate"
scripts/hepta-terminal-release-governance-final-audit-index-gate.sh

echo "[hepta-preflight] operator-security attention-budget diagnostic gate"
scripts/hepta-operator-security-attention-budget-diagnostic-gate.sh

echo "[hepta-preflight] terminal watchdog/soak regression gate"
scripts/hepta-terminal-watchdog-soak-regression-gate.sh

echo "[hepta-preflight] core activation long-soak observation non-acceptance gate"
scripts/hepta-core-activation-long-soak-observation-non-acceptance-gate.sh

echo "[hepta-preflight] core activation long-soak observation freshness denial gate"
scripts/hepta-core-activation-long-soak-observation-freshness-denial-gate.sh

echo "[hepta-preflight] core activation readiness summary gate"
scripts/hepta-core-activation-readiness-summary-gate.sh

echo "[hepta-preflight] core activation long-soak operator approval packet gate"
scripts/hepta-core-activation-long-soak-operator-approval-packet-gate.sh

echo "[hepta-preflight] core activation operator approval fresh evidence supersession-expiry denial gate"
scripts/hepta-core-activation-operator-approval-fresh-evidence-supersession-expiry-denial-gate.sh

echo "[hepta-preflight] core activation request monotonic single-use approval nonce denial gate"
scripts/hepta-core-activation-request-monotonic-single-use-approval-nonce-denial-gate.sh

echo "[hepta-preflight] core activation fresh long-soak evidence ledger receipt gate"
scripts/hepta-core-activation-fresh-long-soak-evidence-ledger-receipt-gate.sh

echo "[hepta-preflight] core activation evidence receipt materialization dry-run gate"
scripts/hepta-core-activation-evidence-receipt-materialization-dry-run-gate.sh

echo "[hepta-preflight] core activation evidence receipt filesystem persistence denial gate"
scripts/hepta-core-activation-evidence-receipt-filesystem-persistence-denial-gate.sh

echo "[hepta-preflight] core activation evidence receipt acceptance denial gate"
scripts/hepta-core-activation-evidence-receipt-acceptance-denial-gate.sh

echo "[hepta-preflight] core activation evidence receipt terminal closure decision gate"
scripts/hepta-core-activation-evidence-receipt-terminal-closure-decision-gate.sh

echo "[hepta-preflight] core activation terminal closure gap evidence index gate"
scripts/hepta-core-activation-terminal-closure-gap-evidence-index-gate.sh

echo "[hepta-preflight] core activation terminal closure operator packet template gate"
scripts/hepta-core-activation-terminal-closure-operator-packet-template-gate.sh

echo "[hepta-preflight] core activation terminal closure operator packet dry-run validator gate"
scripts/hepta-core-activation-terminal-closure-operator-packet-dry-run-validator-gate.sh

echo "[hepta-preflight] core activation terminal closure operator packet authority replay matrix gate"
scripts/hepta-core-activation-terminal-closure-operator-packet-authority-replay-matrix-gate.sh

echo "[hepta-preflight] core activation terminal closure operator packet trusted-record acceptance skeleton gate"
scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-skeleton-gate.sh

echo "[hepta-preflight] core activation terminal closure operator packet trusted-record acceptance negative-fixture matrix gate"
scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-negative-fixture-matrix-gate.sh

echo "[hepta-preflight] core activation terminal closure operator packet trusted-record acceptance precondition scoreboard gate"
scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-acceptance-precondition-scoreboard-gate.sh

echo "[hepta-preflight] core activation terminal closure operator packet trusted-record positive packet dry-run scaffold gate"
scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-dry-run-scaffold-gate.sh

echo "[hepta-preflight] core activation terminal closure operator packet trusted-record positive packet authority replay denial matrix gate"
scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-matrix-gate.sh

echo "[hepta-preflight] core activation terminal closure operator packet trusted-record positive packet authority replay denial summary gate"
scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-gate.sh

echo "[hepta-preflight] core activation terminal closure operator packet trusted-record positive packet authority replay denial summary index gate"
scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-gate.sh

echo "[hepta-preflight] core activation terminal closure operator packet trusted-record positive packet authority replay denial summary index manifest gate"
scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest-gate.sh

echo "[hepta-preflight] core activation terminal closure operator packet trusted-record positive packet authority replay denial summary index manifest JSON-capture boundary gate"
scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest-json-capture-boundary-gate.sh

echo "[hepta-preflight] core activation terminal closure operator packet trusted-record positive packet operator approval gap ledger gate"
scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-operator-approval-gap-ledger-gate.sh

echo "[hepta-preflight] core activation operator approval gap ledger summary briefing non-persistence gate"
scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-non-persistence-gate.sh

echo "[hepta-preflight] core activation operator approval gap ledger summary briefing acknowledgement non-acceptance gate"
scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-non-acceptance-gate.sh

echo "[hepta-preflight] core activation operator approval gap ledger summary briefing acknowledgement activation request denial matrix gate"
scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-request-denial-matrix-gate.sh

echo "[hepta-preflight] core activation operator approval gap ledger summary briefing acknowledgement activation command no-op handoff gate"
scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-noop-handoff-gate.sh

echo "[hepta-preflight] core activation operator approval gap ledger summary briefing acknowledgement activation command result receipt no-persistence gate"
scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-no-persistence-gate.sh

echo "[hepta-preflight] core activation operator approval gap ledger summary briefing acknowledgement activation command result receipt replay idempotency denial gate"
scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] core activation operator approval gap ledger summary briefing acknowledgement activation command result receipt ordering monotonicity denial gate"
scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] core activation operator approval gap ledger summary briefing acknowledgement activation command result receipt cancellation supersession denial gate"
scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] JSON report capture diagnostic contract gate"
scripts/hepta-json-report-capture-diagnostic-contract-gate.sh

echo "[hepta-preflight] JSON report capture migration inventory gate"
scripts/hepta-json-report-capture-migration-inventory-gate.sh

echo "[hepta-preflight] preflight terminal coverage inventory gate"
scripts/hepta-preflight-terminal-coverage-inventory-gate.sh

echo "[hepta-preflight] preflight terminal coverage diagnostic contract gate"
scripts/hepta-preflight-terminal-coverage-diagnostic-contract-gate.sh

echo "[hepta-preflight] upstream Codex latest active-safety regression gate"
scripts/hepta-upstream-codex-latest-active-safety-regression.sh

echo "[hepta-preflight] upstream Codex latest release-governance non-activation gate"
scripts/hepta-upstream-codex-latest-release-governance-non-activation-gate.sh

echo "[hepta-preflight] upstream Codex latest operator briefing non-persistence gate"
scripts/hepta-upstream-codex-latest-operator-briefing-non-persistence-gate.sh

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
git diff --cached --check
git status -sb

echo "Hepta preflight passed"
