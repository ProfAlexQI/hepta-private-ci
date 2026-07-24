#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

REPO_ROOT="$PWD"
source "$REPO_ROOT/scripts/lib/hepta-release-provenance.sh"
source "$REPO_ROOT/scripts/lib/hepta-v2-test-inventory.sh"

MANIFEST="${HEPTA_MANIFEST:-${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}}"
NATIVE_MANIFEST="${HEPTA_NATIVE_MANIFEST:-apps/hepta-native/Cargo.toml}"
NATIVE_TARGET_DIR="${HEPTA_NATIVE_TARGET_DIR:-apps/hepta-native/target}"
RUN_NATIVE="${HEPTA_PREFLIGHT_NATIVE:-${HEPTA_CODEX_PREFLIGHT_NATIVE:-1}}"
RUN_RELEASE="${HEPTA_PREFLIGHT_RELEASE:-${HEPTA_CODEX_PREFLIGHT_RELEASE:-0}}"
# Candidate preflight runs the read-only terminal gates before a source-bound
# release artifact/receipt can exist. Those gates therefore verify active
# health only. The release artifact receives source/toolchain/dependency
# provenance after the release build below; deployment consistency remains a
# separate, fail-closed post-install watchdog mode.
export HEPTA_WATCHDOG_GATE_MODE="${HEPTA_PREFLIGHT_WATCHDOG_GATE_MODE:-active-health}"
HEPTA_FOCUSED_TEST_MAX_SECONDS="${HEPTA_FOCUSED_TEST_MAX_SECONDS:-600}"
HEPTA_FULL_TEST_MAX_SECONDS="${HEPTA_FULL_TEST_MAX_SECONDS:-1800}"
HEPTA_TEST_MAX_DIRTY_DELTA="${HEPTA_TEST_MAX_DIRTY_DELTA:-0}"
PREFLIGHT_SOURCE_COMMIT="$(git rev-parse HEAD)"

export CARGO_INCREMENTAL="${CARGO_INCREMENTAL:-0}"

budgeted_cargo_test() {
  local name="$1"
  local max_seconds="$2"
  shift 2
  scripts/hepta-budgeted-test run \
    --name "$name" \
    --max-seconds "$max_seconds" \
    --workspace "$PWD" \
    --max-dirty-delta "$HEPTA_TEST_MAX_DIRTY_DELTA" \
    -- cargo test "$@"
}

assert_test_inventory() {
  hepta_v2_assert_test_inventory "$@"
}

HEPTA_PREFLIGHT_CREATED_JSON_REPORT_CAPTURE_CACHE_DIR=0
if [[ "${HEPTA_JSON_REPORT_CAPTURE_CACHE:-1}" != "0" \
  && -z "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}" ]]; then
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR="$(mktemp -d /tmp/hepta-json-report-capture.XXXXXX)"
  export HEPTA_JSON_REPORT_CAPTURE_CACHE_SALT="hepta-preflight:$$:${HEPTA_RELEASE_BIN:-}:${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-}:${RUN_NATIVE}:${RUN_RELEASE}:${HEPTA_WATCHDOG_GATE_MODE}"
  HEPTA_PREFLIGHT_CREATED_JSON_REPORT_CAPTURE_CACHE_DIR=1
  trap 'if [[ "${HEPTA_PREFLIGHT_CREATED_JSON_REPORT_CAPTURE_CACHE_DIR:-0}" == "1" ]]; then rm -rf "${HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR:-}"; fi' EXIT
fi

echo "[hepta-preflight] metadata"
PREFLIGHT_RELEASE_TARGET_DIR="$(
  cargo metadata --offline --manifest-path "$MANIFEST" --no-deps --format-version 1 \
    | tee /tmp/hepta-preflight-metadata.json \
    | jq -r '.target_directory'
)"
# hepta-preflight-resume: prelude-end

echo "[hepta-preflight] fmt"
just fmt-check

echo "[hepta-preflight] cargo check"
cargo check --offline --manifest-path "$MANIFEST" -q \
  -p hepta-contracts \
  -p hepta-core \
  -p hepta-intelligence \
  -p hepta-kernel \
  -p hepta-memory \
  -p hepta-plugins \
  -p hepta-runtime \
  -p hepta-gateway \
  -p hepta-cli --bin hepta

echo "[hepta-preflight] Architecture V2 contract boundary tests"
assert_test_inventory "Architecture V2 stable contracts" 11 '.*' \
  codex-rs/hepta-contracts/tests/stable_contracts.rs
budgeted_cargo_test architecture-v2-contracts "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-contracts
cargo clippy --offline --manifest-path "$MANIFEST" -q \
  -p hepta-contracts --tests -- -D warnings
echo "[hepta-preflight] Architecture V2 crate lint gates"
cargo clippy --offline --manifest-path "$MANIFEST" -q \
  -p hepta-intelligence \
  -p hepta-memory \
  -p hepta-runtime \
  --tests --no-deps -- -D warnings

echo "[hepta-preflight] Architecture V2 cognition/orchestration regression gates"
assert_test_inventory "Architecture V2 intelligence neuron activation" 4 '.*' \
  codex-rs/hepta-intelligence/src/neuron_activation/tests.rs
budgeted_cargo_test architecture-v2-neuron-activation "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-intelligence neuron_activation
assert_test_inventory "Architecture V2 tool candidate" 4 '.*' \
  codex-rs/hepta-intelligence/src/tool_candidate.rs
budgeted_cargo_test architecture-v2-tool-candidate "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-intelligence tool_candidate
assert_test_inventory "Architecture V2 intuition feedback learner" 4 \
  '.*' \
  codex-rs/hepta-intelligence/src/intuition_feedback_learning/tests.rs
budgeted_cargo_test architecture-v2-intuition-feedback-learning \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-intelligence \
  intuition_feedback_learning
assert_test_inventory "Architecture V2 intuition planner" 8 \
  '.*' \
  codex-rs/hepta-intelligence/src/intuition_planner/tests.rs
budgeted_cargo_test architecture-v2-intuition-planner \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-intelligence \
  intuition_planner
assert_test_inventory "Architecture V2 explicit-preference reducer" 8 \
  '.*' \
  codex-rs/hepta-intelligence/src/preference_feedback/tests.rs
budgeted_cargo_test architecture-v2-preference-feedback "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-intelligence explicit_preference_
assert_test_inventory "Architecture V2 trusted preference feedback" 5 \
  '.*' \
  codex-rs/hepta-intelligence/src/trusted_preference_feedback/tests.rs
budgeted_cargo_test architecture-v2-trusted-preference-feedback \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-intelligence \
  trusted_preference_feedback
assert_test_inventory "Architecture V2 kernel safety gate" 12 \
  '.*' \
  codex-rs/hepta-kernel/src/safety_gate/tests.rs \
  codex-rs/hepta-kernel/src/safety_gate/admission_tests.rs
budgeted_cargo_test architecture-v2-kernel-safety-gate "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-kernel safety_gate
assert_test_inventory "Architecture V2 preference authority" 6 \
  '.*' \
  codex-rs/hepta-memory/src/preference_authority/tests.rs
budgeted_cargo_test architecture-v2-preference-authority \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-memory \
  preference_authority
assert_test_inventory "Architecture V2 preference-CAS" 35 \
  '.*' \
  codex-rs/hepta-memory/src/tests/preference_cas.rs \
  codex-rs/hepta-memory/src/tests/preference_cas/document.rs \
  codex-rs/hepta-memory/src/tests/preference_cas/durable.rs \
  codex-rs/hepta-memory/src/tests/preference_cas/durable_concurrency.rs \
  codex-rs/hepta-memory/src/tests/preference_cas/durable_opening.rs \
  codex-rs/hepta-memory/src/tests/preference_cas/durable_opening_security.rs \
  codex-rs/hepta-memory/src/tests/preference_cas/fixtures.rs \
  codex-rs/hepta-memory/src/tests/preference_cas/legacy.rs
assert_test_inventory "Architecture V2 durable preference-CAS" 19 \
  '.*' \
  codex-rs/hepta-memory/src/tests/preference_cas/durable.rs \
  codex-rs/hepta-memory/src/tests/preference_cas/durable_concurrency.rs \
  codex-rs/hepta-memory/src/tests/preference_cas/durable_opening.rs \
  codex-rs/hepta-memory/src/tests/preference_cas/durable_opening_security.rs
assert_test_inventory "Architecture V2 durable opening security" 7 \
  '.*' \
  codex-rs/hepta-memory/src/tests/preference_cas/durable_opening_security.rs
assert_test_inventory "Architecture V2 durable sidecar lifecycle" 2 \
  'unlinked_open_sidecar_.*' \
  codex-rs/hepta-memory/src/durable/opening/filesystem.rs
budgeted_cargo_test architecture-v2-durable-sidecar-lifecycle \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-memory \
  unlinked_open_sidecar_
budgeted_cargo_test architecture-v2-preference-cas "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-memory preference_cas
assert_test_inventory "Architecture V2 outcome-store" 50 \
  '.*' \
  codex-rs/hepta-memory/src/tests/outcome_store.rs \
  codex-rs/hepta-memory/src/tests/outcome_store/durable.rs \
  codex-rs/hepta-memory/src/tests/outcome_store/effect_ack.rs \
  codex-rs/hepta-memory/src/tests/outcome_store/execution_intent.rs \
  codex-rs/hepta-memory/src/tests/outcome_store/pending_intent.rs \
  codex-rs/hepta-memory/src/tests/outcome_store/sync_writer.rs
assert_test_inventory "Architecture V2 durable outcome-store" 29 \
  '.*' \
  codex-rs/hepta-memory/src/tests/outcome_store/durable.rs \
  codex-rs/hepta-memory/src/tests/outcome_store/effect_ack.rs \
  codex-rs/hepta-memory/src/tests/outcome_store/execution_intent.rs \
  codex-rs/hepta-memory/src/tests/outcome_store/pending_intent.rs
assert_test_inventory "Architecture V2 durable effect ACK" 4 \
  '.*' \
  codex-rs/hepta-memory/src/tests/outcome_store/effect_ack.rs
assert_test_inventory "Architecture V2 durable execution intent" 9 \
  '.*' \
  codex-rs/hepta-memory/src/tests/outcome_store/execution_intent.rs
assert_test_inventory "Architecture V2 sync durable outcome writer" 13 \
  '.*' \
  codex-rs/hepta-memory/src/tests/outcome_store/sync_writer.rs
budgeted_cargo_test architecture-v2-outcome-store "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-memory outcome_store
assert_test_inventory "Architecture V2 runtime neuron hydration" 8 \
  "$HEPTA_V2_RUNTIME_NEURON_TEST_PATTERN" \
  codex-rs/hepta-runtime/src/query/tests.rs
budgeted_cargo_test architecture-v2-runtime-neuron-activation \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib neuron_activation
assert_test_inventory "Architecture V2 outcome-receipt" 4 \
  'architecture_v2_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_terminal_outcome.rs
budgeted_cargo_test architecture-v2-runtime-outcome-receipt \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_outcome_receipt_tests
assert_test_inventory "Architecture V2 outcome-flow" 8 \
  'architecture_v2_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_outcome_flow.rs
budgeted_cargo_test architecture-v2-runtime-outcome-flow \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_outcome_flow_tests
assert_test_inventory "Architecture V2 runtime outcome sink" 19 \
  '.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/outcome_sink/tests.rs
budgeted_cargo_test architecture-v2-runtime-outcome-sink \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  'outcome_sink::tests'
assert_test_inventory "Architecture V2 provider idempotency" 2 \
  'architecture_v2_provider_idempotency_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_provider_idempotency.rs
budgeted_cargo_test architecture-v2-runtime-provider-idempotency \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_provider_idempotency_
assert_test_inventory "Architecture V2 exact-safety" 10 \
  'architecture_v2_exact_(safety|admission)_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_exact_safety.rs
budgeted_cargo_test architecture-v2-runtime-exact-safety \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_exact_
assert_test_inventory "Architecture V2 execution-lease" 5 \
  'architecture_v2_execution_lease_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_execution_lease.rs
budgeted_cargo_test architecture-v2-runtime-execution-lease \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_execution_lease_
assert_test_inventory "Architecture V2 resource-reservation" 4 \
  'architecture_v2_resource_reservation_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_resource_reservation.rs
budgeted_cargo_test architecture-v2-runtime-resource-reservation \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_resource_reservation_
assert_test_inventory "Architecture V2 capability-descriptor" 4 \
  'architecture_v2_capability_descriptor_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_capability_descriptor.rs
budgeted_cargo_test architecture-v2-runtime-capability-descriptor \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_capability_descriptor_
assert_test_inventory "Architecture V2 symlink-reservation" 4 \
  'architecture_v2_symlink_reservation_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_symlink_reservation.rs
budgeted_cargo_test architecture-v2-runtime-symlink-reservation \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_symlink_reservation_
assert_test_inventory "Architecture V2 process-reservation" 8 \
  'architecture_v2_process_reservation_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_process_reservation.rs
budgeted_cargo_test architecture-v2-runtime-process-reservation \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_process_reservation_
assert_test_inventory "Architecture V2 cross-process write lock" 4 \
  '.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/cross_process_write_lock.rs
budgeted_cargo_test architecture-v2-runtime-cross-process-write-lock \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  'cross_process_write_lock::tests'
assert_test_inventory "Architecture V2 dispatch-selector" 3 \
  'architecture_v2_dispatch_selector_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_dispatch_selector.rs
budgeted_cargo_test architecture-v2-runtime-dispatch-selector \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_dispatch_selector_
assert_test_inventory "Architecture V2 native-mutation" 8 \
  'architecture_v2_native_mutation_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_native_mutation.rs
budgeted_cargo_test architecture-v2-runtime-native-mutation \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_native_mutation_
assert_test_inventory "Architecture V2 provider-effect ACK" 2 \
  'architecture_v2_provider_effect_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_provider_effect.rs
budgeted_cargo_test architecture-v2-runtime-provider-effect \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_provider_effect_
assert_test_inventory "Architecture V2 sealed-read" 9 \
  'architecture_v2_sealed_read_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_sealed_read.rs
budgeted_cargo_test architecture-v2-runtime-sealed-read \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_sealed_read_
assert_test_inventory "Architecture V2 process-control" 2 \
  'architecture_v2_process_control_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_process_control.rs
budgeted_cargo_test architecture-v2-runtime-process-control \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_process_control_
assert_test_inventory "Architecture V2 maintenance-mutation" 7 \
  'architecture_v2_maintenance_.*' \
  codex-rs/hepta-runtime/src/runtime_kernel/tests/architecture_v2_maintenance_mutation.rs
budgeted_cargo_test architecture-v2-runtime-maintenance-mutation \
  "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib \
  architecture_v2_maintenance_
for turn_coordinator_test in \
  generic_read_only_tool_call_runs_through_tool_loop \
  medium_risk_tool_requires_approval_until_granted \
  quarantined_exec_intent_never_enters_the_production_tool_loop \
  returns_and_validates_structured_tool_output \
  exposes_sessions_memory_and_history_snapshots
do
  budgeted_cargo_test "architecture-v2-${turn_coordinator_test}" \
    "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
    --offline --manifest-path "$MANIFEST" -q -p hepta-runtime --lib "$turn_coordinator_test"
done

echo "[hepta-preflight] adapter behavior-equivalence gate"
budgeted_cargo_test adapter-runtime-behavior-equivalence "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime \
  codex_engine_adapter_behavior_equivalence_gate -- --nocapture
budgeted_cargo_test adapter-native-boundary "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
  hepta_codex_engine_adapter_boundary -- --nocapture

echo "[hepta-preflight] adapter shadow-replay gate"
budgeted_cargo_test adapter-shadow-replay "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime \
  all_adapter_shadow_replay -- --nocapture

echo "[hepta-preflight] name/repository closure gate"
budgeted_cargo_test name-repository-runtime "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-runtime \
  name_repository_closure -- --nocapture
budgeted_cargo_test name-repository-native "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib \
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

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution readiness route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-readiness-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled readback receipt no-persistence gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-no-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled readback receipt authority-denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-authority-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled readback receipt trusted operator packet separation gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-separation-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled readback receipt trusted operator packet intake precondition gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-intake-precondition-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled readback receipt trusted operator packet partial precondition denial matrix gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-partial-precondition-denial-matrix-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled readback receipt trusted operator packet complete precondition authority-denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-authority-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router operator-approved shadow context activation execution controlled readback receipt trusted operator packet complete precondition operator approval lane separation gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-approved-shadow-context-activation-execution-controlled-readback-receipt-trusted-operator-packet-complete-precondition-operator-approval-lane-separation-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator-approved memory live mutation durable lane gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-memory-live-mutation-durable-lane-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator-approved Hepta Intelligence context attachment lane gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-hepta-intelligence-context-attachment-lane-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator-approved KG prompt-preview read-only adapter lane gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-preview-read-only-adapter-lane-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator-approved KG prompt payload materialization lane gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-materialization-lane-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator-approved KG prompt payload acceptance receipt lane gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-acceptance-receipt-lane-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator-approved KG prompt payload readback audit receipt lane gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-kg-prompt-payload-readback-audit-receipt-lane-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator-approved context handoff acceptance lane gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-acceptance-lane-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator-approved context handoff receipt audit lane gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-context-handoff-receipt-audit-lane-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator-approved bounded provider-router injection precondition lane gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-precondition-lane-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator-approved bounded provider-router injection dry-run envelope lane gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-lane-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator-approved bounded provider-router injection dry-run envelope readback audit receipt lane gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-readback-audit-receipt-lane-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator-approved bounded provider-router injection dry-run envelope readback audit receipt acknowledgement no-op handoff lane gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-approved-bounded-provider-router-injection-dry-run-envelope-readback-audit-receipt-acknowledgement-no-op-handoff-lane-gate.sh

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

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness single-budget dispatch dry-run no-op receipt route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-single-budget-dispatch-dry-run-noop-receipt-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review/readback index no-persistence gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review/readback index no-persistence route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-readback-index-no-persistence-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement non-acceptance gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement non-acceptance route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation request denial matrix gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation request denial matrix route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-request-denial-matrix-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command no-op handoff gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-noop-handoff-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command no-op handoff route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-noop-handoff-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt no-persistence gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-no-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt no-persistence route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-no-persistence-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt replay idempotency denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt replay idempotency denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-replay-idempotency-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt ordering monotonicity denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt ordering monotonicity denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt cancellation supersession denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt cancellation supersession denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt audit trail immutable evidence denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt audit trail immutable evidence denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt retention expiry garbage collection denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt retention expiry garbage collection denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-retention-expiry-garbage-collection-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt export query observability denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt export query observability denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-export-query-observability-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt operator-facing summary briefing non-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt operator-facing summary briefing non-persistence denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt final operator acknowledgement non-acceptance denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt final operator acknowledgement non-acceptance denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt terminal operator decision public-claim non-promotion denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt terminal operator decision public-claim non-promotion denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt release artifact publication denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt release artifact publication denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt release artifact publication result receipt no-persistence gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement operator canary controlled request harness operator review acknowledgement activation command result receipt release artifact publication result receipt no-persistence route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-result-receipt-no-persistence-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation readiness index gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-readiness-index-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation readiness index replay/idempotency denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation readiness index replay/idempotency denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-readiness-index-replay-idempotency-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG activation truth index route gate"
scripts/hepta-memory-intelligence-kg-activation-truth-index-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template non-acceptance authority replay denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-non-acceptance-authority-replay-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template non-acceptance authority replay denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-non-acceptance-authority-replay-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template field validation denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-field-validation-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template field validation denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-field-validation-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template section completion non-acceptance gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template section completion non-acceptance route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-section-completion-non-acceptance-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet assembly non-acceptance gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-assembly-non-acceptance-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet assembly non-acceptance route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-assembly-non-acceptance-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt non-persistence gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt non-persistence route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-non-persistence-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt replay/idempotency denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt replay/idempotency denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-replay-idempotency-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt ordering/monotonicity denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt ordering/monotonicity denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-ordering-monotonicity-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt cancellation/supersession denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt cancellation/supersession denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-cancellation-supersession-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt audit-trail/immutable-evidence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt audit-trail/immutable-evidence denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-audit-trail-immutable-evidence-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt retention/expiry/garbage-collection denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-retention-expiry-garbage-collection-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt retention/expiry/garbage-collection denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-retention-expiry-garbage-collection-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt export/query/observability denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt export/query/observability denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-export-query-observability-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt redaction/privacy/payload-exposure denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt redaction/privacy/payload-exposure denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-redaction-privacy-payload-exposure-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt operator briefing non-persistence gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt operator briefing non-persistence route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-operator-briefing-non-persistence-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt final acknowledgement non-acceptance gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt final acknowledgement non-acceptance route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-final-acknowledgement-non-acceptance-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt terminal decision/status promotion denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-terminal-decision-status-promotion-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt terminal decision/status promotion denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-terminal-decision-status-promotion-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt no-persistence gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt no-persistence route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-no-persistence-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt replay/idempotency denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt replay/idempotency denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-replay-idempotency-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt ordering/monotonicity denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt ordering/monotonicity denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-ordering-monotonicity-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt cancellation/supersession denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt cancellation/supersession denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-cancellation-supersession-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt audit-trail/immutable-evidence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-audit-trail-immutable-evidence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt audit-trail/immutable-evidence denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-audit-trail-immutable-evidence-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt retention/expiry/garbage-collection denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-retention-expiry-garbage-collection-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt retention/expiry/garbage-collection denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-retention-expiry-garbage-collection-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt export/query/observability denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt export/query/observability denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-export-query-observability-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt operator-facing summary/briefing non-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt operator-facing summary/briefing non-persistence denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-operator-facing-summary-briefing-non-persistence-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt final operator acknowledgement non-acceptance denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt final operator acknowledgement non-acceptance denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-final-operator-acknowledgement-non-acceptance-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal decision/status promotion denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-decision-status-promotion-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal decision/status promotion denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-decision-status-promotion-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal public claim/status exposure denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-public-claim-status-exposure-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal public claim/status exposure denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-public-claim-status-exposure-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution queue/artifact availability status denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-queue-artifact-availability-status-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution queue/artifact availability status denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-queue-artifact-availability-status-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt/external delivery non-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt/external delivery non-persistence denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-external-delivery-non-persistence-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt query/export/observability denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-query-export-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt query/export/observability denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-query-export-observability-denial-route-gate.sh

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

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt package/release channel status exposure denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-package-release-channel-status-exposure-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt distribution artifact/manifest status denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt distribution artifact/manifest status denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-distribution-artifact-manifest-status-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact distribution signing/notarization surface denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-distribution-signing-notarization-surface-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact distribution signing/notarization surface denial route gate"
scripts/hepta-artifact-distribution-signing-notarization-surface-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-distribution-delivery-receipt-artifact-download-install-affordance-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt no-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-result-receipt-terminal-delivery-receipt-artifact-download-install-affordance-result-receipt-no-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt no-persistence denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-no-persistence-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt replay/idempotency denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt replay/idempotency denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-replay-idempotency-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt ordering/monotonicity denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt ordering/monotonicity denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-ordering-monotonicity-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt cancellation/supersession denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt cancellation/supersession denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-cancellation-supersession-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt audit-trail/immutable-evidence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-audit-trail-immutable-evidence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt audit-trail/immutable-evidence denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-audit-trail-immutable-evidence-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt retention/expiry/garbage-collection denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt retention/expiry/garbage-collection denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-retention-expiry-garbage-collection-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt export/query/observability denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt export/query/observability denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-export-query-observability-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator-facing summary/briefing non-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator-facing summary/briefing non-persistence denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-facing-summary-briefing-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt final operator acknowledgement non-acceptance denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt final operator acknowledgement non-acceptance denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-final-operator-acknowledgement-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt terminal decision/status promotion denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-terminal-decision-status-promotion-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt terminal decision/status promotion denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-terminal-decision-status-promotion-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator intent/consent reconfirmation denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-reconfirmation-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator intent/consent reconfirmation denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-reconfirmation-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session binding denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-binding-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session binding denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-binding-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session replay/cross-binding denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-replay-cross-binding-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session replay/cross-binding denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-replay-cross-binding-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement ordering/monotonicity denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement ordering/monotonicity denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-ordering-monotonicity-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement cancellation/supersession denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement cancellation/supersession denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-cancellation-supersession-route-gate.sh

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

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement terminal decision/status promotion denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-decision-status-promotion-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement terminal public-claim/status exposure denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-public-claim-status-exposure-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement terminal public-claim/status exposure denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-terminal-public-claim-status-exposure-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent reconfirmation denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-reconfirmation-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent reconfirmation denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-reconfirmation-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence persistence denial route gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-persistence-route-gate.sh

echo "[hepta-preflight] minimal Memory canary scoped operator packet write/readback/rollback/idempotency receipt route gate"
scripts/hepta-minimal-memory-canary-scoped-operator-packet-write-readback-rollback-idempotency-receipt-route-gate.sh

echo "[hepta-preflight] Intelligence bounded context attachment preview/readback route gate"
scripts/hepta-intelligence-bounded-context-attachment-preview-readback-route-gate.sh

echo "[hepta-preflight] KG read-only adapter shadow-rank canary route gate"
scripts/hepta-kg-read-only-adapter-shadow-rank-canary-route-gate.sh

echo "[hepta-preflight] provider-router dry-run envelope readback audit route gate"
scripts/hepta-provider-router-dry-run-envelope-readback-audit-route-gate.sh

echo "[hepta-preflight] first model invocation separate approval slice preflight route gate"
scripts/hepta-first-model-invocation-separate-approval-slice-preflight-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval packet review acceptance-denial preflight route gate"
scripts/hepta-first-model-invocation-operator-approval-packet-review-acceptance-denial-preflight-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval acceptance artifact precondition route gate"
scripts/hepta-first-model-invocation-operator-approval-acceptance-artifact-precondition-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval nonce/session/command binding preflight route gate"
scripts/hepta-first-model-invocation-operator-approval-nonce-session-command-binding-preflight-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval final authorization dry-run envelope preflight route gate"
scripts/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-envelope-preflight-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval final authorization dry-run result receipt no-persistence route gate"
scripts/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-no-persistence-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval final authorization dry-run result receipt replay/idempotency denial route gate"
scripts/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-replay-idempotency-denial-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval final authorization dry-run result receipt ordering/monotonicity denial route gate"
scripts/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-ordering-monotonicity-denial-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval final authorization dry-run result receipt cancellation/supersession denial route gate"
scripts/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-cancellation-supersession-denial-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval final authorization dry-run result receipt audit/immutable-evidence denial route gate"
scripts/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-audit-immutable-evidence-denial-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval final authorization dry-run result receipt retention/expiry/garbage-collection denial route gate"
scripts/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-retention-expiry-garbage-collection-denial-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval final authorization dry-run result receipt export/query/observability denial route gate"
scripts/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-export-query-observability-denial-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval final authorization dry-run result receipt operator-facing summary/briefing non-persistence denial route gate"
scripts/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-operator-facing-summary-briefing-non-persistence-denial-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval final authorization dry-run result receipt final operator acknowledgement non-acceptance denial route gate"
scripts/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-denial-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval final authorization dry-run result receipt terminal operator decision/public-claim non-promotion denial route gate"
scripts/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval final authorization dry-run result receipt terminal public-claim/status exposure denial route gate"
scripts/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-public-claim-status-exposure-denial-route-gate.sh

echo "[hepta-preflight] first model invocation operator approval final authorization dry-run result receipt terminal public-claim delivery/readback denial route gate"
scripts/hepta-first-model-invocation-operator-approval-final-authorization-dry-run-result-receipt-terminal-public-claim-delivery-readback-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence export/query/observability denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence summary/briefing non-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-summary-briefing-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence final operator acknowledgement non-acceptance denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-final-ack-non-acceptance-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence terminal decision/status promotion denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-terminal-decision-status-promotion-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence terminal public claim/status exposure denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-terminal-public-claim-status-exposure-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence package/release channel status exposure denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-package-release-channel-status-exposure-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence distribution artifact/manifest status denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-distribution-artifact-manifest-status-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization surface denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-notarization-surface-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization result receipt no-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact signing receipt no-persistence denial route gate"
scripts/hepta-artifact-signing-receipt-no-persistence-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt replay/idempotency denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact signing receipt replay/idempotency denial route gate"
scripts/hepta-artifact-signing-receipt-replay-idempotency-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt ordering/monotonicity denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact signing receipt ordering/monotonicity denial route gate"
scripts/hepta-artifact-signing-receipt-ordering-monotonicity-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt cancellation/supersession denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-cancel-supersession-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact signing receipt cancellation/supersession denial route gate"
scripts/hepta-artifact-signing-receipt-cancellation-supersession-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt audit/evidence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-audit-evidence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact signing receipt audit/evidence denial route gate"
scripts/hepta-artifact-signing-receipt-audit-evidence-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt retention/expiry/garbage-collection denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-retention-gc-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact signing receipt retention/expiry/garbage-collection denial route gate"
scripts/hepta-artifact-signing-receipt-retention-expiry-garbage-collection-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt export/query/observability denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact signing receipt export/query/observability denial route gate"
scripts/hepta-artifact-signing-receipt-export-query-observability-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt operator-facing summary/briefing non-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-summary-briefing-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact signing receipt operator-facing summary/briefing non-persistence denial route gate"
scripts/hepta-artifact-signing-receipt-operator-facing-summary-briefing-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt final operator acknowledgement non-acceptance denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-final-ack-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact signing receipt final operator acknowledgement non-acceptance denial route gate"
scripts/hepta-artifact-signing-receipt-final-operator-acknowledgement-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt terminal decision/status promotion denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-status-promotion-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt terminal public claim/status exposure denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-status-exposure-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact signing receipt terminal public claim/status exposure denial route gate"
scripts/hepta-artifact-signing-receipt-terminal-public-claim-status-exposure-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt terminal public claim delivery/readback denial gate"
scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-readback-denial-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact signing receipt terminal public claim delivery/readback denial route gate"
scripts/hepta-artifact-signing-receipt-terminal-public-claim-delivery-readback-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence/KG full live activation artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact signing receipt release/public artifact publication denial route gate"
scripts/hepta-artifact-signing-receipt-release-public-artifact-publication-denial-route-gate.sh

echo "[hepta-preflight] first-model positive approval packet boundary route gate"
scripts/hepta-first-model-positive-approval-packet-boundary-route-gate.sh

echo "[hepta-preflight] scoped Memory canary durable receipt boundary route gate"
scripts/hepta-scoped-memory-canary-durable-receipt-boundary-route-gate.sh

echo "[hepta-preflight] bounded Intelligence context handoff prompt preview boundary route gate"
scripts/hepta-bounded-intelligence-context-handoff-prompt-preview-boundary-route-gate.sh

echo "[hepta-preflight] activation evidence no-write provider-router dry-run boundary route gate"
scripts/hepta-activation-evidence-no-write-provider-router-dry-run-boundary-route-gate.sh

echo "[hepta-preflight] first-model invocation explicit approval evidence no-invocation boundary route gate"
scripts/hepta-first-model-invocation-explicit-approval-evidence-no-invocation-boundary-route-gate.sh

echo "[hepta-preflight] full-live activation closure index route gate"
scripts/hepta-full-live-activation-closure-index-route-gate.sh

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

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router operator acknowledgement non-acceptance route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-operator-acknowledgement-non-acceptance-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation request denial matrix gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation request denial matrix route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-request-denial-matrix-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command no-op handoff gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command no-op handoff route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-noop-handoff-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt no-persistence gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt no-persistence route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-no-persistence-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt replay idempotency denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt replay idempotency denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-replay-idempotency-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt ordering monotonicity denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt ordering monotonicity denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-ordering-monotonicity-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt cancellation supersession denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt cancellation supersession denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-cancellation-supersession-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt audit trail immutable evidence denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt audit trail immutable evidence denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-audit-trail-immutable-evidence-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt retention expiry garbage collection denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt retention expiry garbage collection denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-retention-expiry-garbage-collection-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt export query observability denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt export query observability denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-export-query-observability-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt operator-facing summary briefing non-persistence denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt operator-facing summary briefing non-persistence denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt final operator acknowledgement non-acceptance denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt final operator acknowledgement non-acceptance denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt terminal operator decision public-claim non-promotion denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt terminal operator decision public-claim non-promotion denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-route-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt release artifact publication denial gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial-gate.sh

echo "[hepta-preflight] memory/intelligence full enablement runtime provider-router activation command result receipt release artifact publication denial route gate"
scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-activation-command-result-receipt-release-artifact-publication-denial-route-gate.sh

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

echo "[hepta-preflight] memory live mutation operator write approval packet boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-approval-packet-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution preflight gate"
scripts/hepta-memory-live-mutation-operator-write-execution-preflight-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution preflight boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-preflight-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution denial matrix gate"
scripts/hepta-memory-live-mutation-operator-write-execution-denial-matrix-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution denial matrix boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-denial-matrix-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution no-write sink contract gate"
scripts/hepta-memory-live-mutation-operator-write-execution-no-write-sink-contract-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution no-write sink contract boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-no-write-sink-contract-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution write-enable fixture gate"
scripts/hepta-memory-live-mutation-operator-write-execution-write-enable-fixture-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution write-enable fixture boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-write-enable-fixture-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution post-write validation dry-run gate"
scripts/hepta-memory-live-mutation-operator-write-execution-post-write-validation-dry-run-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution post-write validation dry-run boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-post-write-validation-dry-run-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution post-write operator acceptance denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-post-write-operator-acceptance-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution post-write operator acceptance denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-post-write-operator-acceptance-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation closure denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-closure-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation closure denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-closure-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command no-op handoff gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-noop-handoff-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command no-op handoff boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-noop-handoff-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt no-persistence gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-no-persistence-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt no-persistence boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-no-persistence-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt replay idempotency denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt replay idempotency denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-replay-idempotency-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt ordering monotonicity denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt ordering monotonicity denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-ordering-monotonicity-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt cancellation supersession denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-cancellation-supersession-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt cancellation supersession denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-cancellation-supersession-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt audit trail immutable evidence denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-audit-trail-immutable-evidence-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt audit trail immutable evidence denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-audit-trail-immutable-evidence-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt retention expiry garbage collection denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-retention-expiry-garbage-collection-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt retention expiry garbage collection denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-retention-expiry-garbage-collection-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt export query observability denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-export-query-observability-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt export query observability denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-export-query-observability-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt operator-facing summary briefing non-persistence denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt operator-facing summary briefing non-persistence denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-operator-facing-summary-briefing-non-persistence-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt final operator acknowledgement non-acceptance denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt final operator acknowledgement non-acceptance denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt terminal operator decision public-claim non-promotion denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt terminal operator decision public-claim non-promotion denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt release artifact publication denial gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-release-artifact-publication-denial-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution activation command result receipt release artifact publication denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-release-artifact-publication-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped memory real write canary operator approval packet nonce command dry-run gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-operator-approval-packet-nonce-command-dry-run-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped memory real write canary operator approval packet nonce command dry-run boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-operator-approval-packet-nonce-command-dry-run-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped memory real write canary readback validation dry-run gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-readback-validation-dry-run-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped memory real write canary readback validation dry-run boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-readback-validation-dry-run-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped memory real write canary rollback tombstone dry-run gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-rollback-tombstone-dry-run-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped memory real write canary rollback tombstone dry-run boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-rollback-tombstone-dry-run-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary operator approval nonce command accepted gate boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-operator-approval-nonce-command-accepted-gate-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary operator approval nonce command accepted gate boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-operator-approval-nonce-command-accepted-gate-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary wal receipt binding boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-wal-receipt-binding-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary wal receipt binding boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-wal-receipt-binding-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary post-write readback binding boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-post-write-readback-binding-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary post-write readback binding boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-post-write-readback-binding-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary rollback tombstone proof boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-tombstone-proof-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary rollback tombstone proof boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-tombstone-proof-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary execution boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-execution-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary execution boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-execution-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable WAL receipt persistence boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable WAL receipt persistence boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-wal-receipt-persistence-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable readback receipt acceptance boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-readback-receipt-acceptance-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable readback receipt acceptance boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-readback-receipt-acceptance-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary rollback receipt acceptance boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary rollback receipt acceptance boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-rollback-receipt-acceptance-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary tombstone cleanup acceptance boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-tombstone-cleanup-acceptance-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary tombstone cleanup acceptance boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-tombstone-cleanup-acceptance-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable store write plan boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-plan-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable store write plan boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-plan-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable store write preflight boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-preflight-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable store write preflight boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-preflight-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable store write guarded execution readiness boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-readiness-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable store write guarded execution readiness boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-readiness-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable store write guarded execution boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable store write guarded execution boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-guarded-execution-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable store write single shot execution boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable store write single shot execution boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-single-shot-execution-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable store write receipt acceptance boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable store write receipt acceptance boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-receipt-acceptance-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable store write rollback tombstone zero residue acceptance boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution minimal scoped memory real write canary durable store write rollback tombstone zero residue acceptance boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write preflight boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-preflight-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write preflight boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-preflight-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write operator packet acceptance boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-operator-packet-acceptance-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write operator packet acceptance boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-operator-packet-acceptance-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write operator packet acceptance receipt boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-operator-packet-acceptance-receipt-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write operator packet acceptance receipt boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-operator-packet-acceptance-receipt-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution envelope boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-envelope-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution envelope boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-envelope-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt replay idempotency denial boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-idempotency-denial-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt replay idempotency denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-replay-idempotency-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt ordering monotonicity denial boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt ordering monotonicity denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-ordering-monotonicity-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt cancellation supersession denial boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-cancellation-supersession-denial-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt cancellation supersession denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-cancellation-supersession-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt audit trail immutable evidence denial boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-audit-trail-immutable-evidence-denial-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt audit trail immutable evidence denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-audit-trail-immutable-evidence-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt retention expiry garbage collection denial boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-retention-expiry-garbage-collection-denial-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt retention expiry garbage collection denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-retention-expiry-garbage-collection-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt export query observability denial boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-export-query-observability-denial-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt export query observability denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-export-query-observability-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt operator-facing summary briefing non-persistence denial boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-operator-facing-summary-briefing-non-persistence-denial-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt operator-facing summary briefing non-persistence denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-operator-facing-summary-briefing-non-persistence-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt final operator acknowledgement non-acceptance denial boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt final operator acknowledgement non-acceptance denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt terminal operator decision public claim non-promotion denial boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt terminal operator decision public claim non-promotion denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt release artifact publication denial boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-denial-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt release artifact publication denial boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-denial-boundary-route-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt release artifact publication result receipt no-persistence boundary gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary-gate.sh

echo "[hepta-preflight] memory live mutation operator write execution scoped production durable memory write dry-run execution result receipt release artifact publication result receipt no-persistence boundary route gate"
scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary-route-gate.sh

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

echo "[hepta-preflight] upstream Codex current latest-recorded intake gate (includes R3 historical integrity)"
scripts/hepta-upstream-codex-current-intake.sh

echo "[hepta-preflight] upstream Codex current latest-recorded intake negative fixture"
scripts/hepta-upstream-codex-current-intake-negative-fixture.sh

echo "[hepta-preflight] upstream Codex doctor environment diagnostics absorption gate"
scripts/hepta-upstream-codex-doctor-environment-diagnostics-absorption.sh

echo "[hepta-preflight] upstream Codex historical latest multi-surface absorption receipt gate"
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

echo "[hepta-preflight] route gate dynamic count regression gate"
scripts/hepta-route-gate-dynamic-count-regression-gate.sh

echo "[hepta-preflight] state-machine gate runner self-test"
scripts/hepta-state-machine-gate-runner-self-test

echo "[hepta-preflight] suffix-ladder freeze"
scripts/check-hepta-suffix-ladder-freeze.sh

echo "[hepta-preflight] test evidence classification matrix"
scripts/hepta-test-evidence-matrix verify
scripts/hepta-test-evidence-matrix self-test

echo "[hepta-preflight] preflight terminal coverage inventory gate"
scripts/hepta-preflight-terminal-coverage-inventory-gate.sh

echo "[hepta-preflight] preflight terminal coverage diagnostic contract gate"
scripts/hepta-preflight-terminal-coverage-diagnostic-contract-gate.sh

echo "[hepta-preflight] upstream Codex latest multisurface absorption native route gate"
scripts/hepta-upstream-codex-latest-multisurface-absorption-route-gate.sh

echo "[hepta-preflight] upstream Codex latest active-safety regression gate"
scripts/hepta-upstream-codex-latest-active-safety-regression.sh

echo "[hepta-preflight] upstream Codex latest release-governance non-activation gate"
scripts/hepta-upstream-codex-latest-release-governance-non-activation-gate.sh

echo "[hepta-preflight] upstream Codex latest operator briefing non-persistence gate"
scripts/hepta-upstream-codex-latest-operator-briefing-non-persistence-gate.sh

echo "[hepta-preflight] upstream intake audit ledger"
scripts/hepta-upstream-intake-audit-ledger verify
scripts/hepta-upstream-intake-audit-ledger self-test

echo "[hepta-preflight] resume state/fuse self-test"
scripts/hepta-preflight-resume-self-test.sh

echo "[hepta-preflight] resume supervisor retry/backoff self-test"
scripts/hepta-preflight-supervisor-self-test.sh

echo "[hepta-preflight] watchdog gate evidence mode self-test"
scripts/hepta-watchdog-gate-evidence-self-test.sh

echo "[hepta-preflight] live Control UI truth mode self-test"
scripts/hepta-live-control-ui-truth-self-test.sh

echo "[hepta-preflight] immutable release tree self-test"
scripts/hepta-immutable-release-tree self-test

echo "[hepta-preflight] active binary deployment consistency self-test"
scripts/hepta-active-binary-consistency-self-test.sh

echo "[hepta-preflight] watchdog build provenance self-test"
scripts/hepta-watchdog-provenance-self-test.sh

echo "[hepta-preflight] architecture hard budget verify/self-test"
scripts/hepta-architecture-budget verify
scripts/hepta-architecture-budget self-test

echo "[hepta-preflight] safe lint and panic debt baseline verify/self-tests"
scripts/hepta-safe-lint-security-baseline verify
scripts/hepta-safe-lint-security-baseline self-test
scripts/hepta-panic-debt-baseline verify
scripts/hepta-panic-debt-baseline self-test

echo "[hepta-preflight] Architecture V2 dependency boundary verify/self-test"
scripts/hepta-v2-architecture-boundary verify
scripts/hepta-v2-architecture-boundary self-test

echo "[hepta-preflight] runtime crate-root source-set compatibility self-test"
scripts/hepta-runtime-crate-root-source-set-self-test

echo "[hepta-preflight] hepta-gateway tests"
budgeted_cargo_test hepta-gateway "$HEPTA_FULL_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-gateway

echo "[hepta-preflight] codex-cli native tests"
echo "[hepta-preflight] Hepta-owned native gateway tests"
# These report tests build deep source-report chains on explicitly enlarged
# worker stacks. Serialize the harness so concurrent convergence lanes cannot
# exhaust thread/virtual-memory resources and turn a successful report into a
# synthetic `source_report_thread_failed` result.
budgeted_cargo_test native-gateway "$HEPTA_FULL_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib native_gateway -- \
  --nocapture --test-threads=1
budgeted_cargo_test native-telegram "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib native_telegram -- --nocapture
budgeted_cargo_test native-post "$HEPTA_FOCUSED_TEST_MAX_SECONDS" \
  --offline --manifest-path "$MANIFEST" -q -p hepta-native-gateway --lib native_post -- --nocapture

echo "[hepta-preflight] control-ui smoke"
CARGO_NET_OFFLINE=true scripts/hepta-control-ui-smoke.sh

if [[ "$RUN_NATIVE" == "1" ]]; then
  echo "[hepta-preflight] native app metadata/check/tests"
  cargo metadata --offline --manifest-path "$NATIVE_MANIFEST" --no-deps --format-version 1 >/tmp/hepta-native-preflight-metadata.json
  CARGO_TARGET_DIR="$NATIVE_TARGET_DIR" cargo check --offline --locked --manifest-path "$NATIVE_MANIFEST"
  CARGO_TARGET_DIR="$NATIVE_TARGET_DIR" cargo test --offline --locked --manifest-path "$NATIVE_MANIFEST" hepta_ -- --nocapture
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

if [[ "$RUN_RELEASE" == "1" ]]; then
  [[ "$(git rev-parse HEAD)" == "$PREFLIGHT_SOURCE_COMMIT" ]] || {
    echo "release preflight source commit changed while validation was running" >&2
    exit 1
  }
  [[ -z "$(git status --porcelain=v1 --untracked-files=all)" ]] || {
    echo "release preflight provenance requires a clean worktree" >&2
    exit 1
  }

  release_artifact="${HEPTA_PREFLIGHT_RELEASE_ARTIFACT:-$PREFLIGHT_RELEASE_TARGET_DIR/release/hepta}"
  [[ -f "$release_artifact" && -x "$release_artifact" ]] || {
    echo "release preflight artifact is missing or not executable: $release_artifact" >&2
    exit 1
  }
  release_build_provenance="$(
    hepta_release_build_provenance_json \
      "$REPO_ROOT" \
      "$PREFLIGHT_SOURCE_COMMIT" \
      "$release_artifact"
  )"
  release_build_provenance="$(
    jq -c \
      --argjson native "$RUN_NATIVE" \
      --arg watchdog_gate_mode "$HEPTA_WATCHDOG_GATE_MODE" \
      '. + {
        preflight_profiles:{backend:true,native:($native == 1),release:true},
        watchdog_gate_mode:$watchdog_gate_mode,
        deployment_consistency:{
          checked_during_candidate_preflight:false,
          required_before_activation:true
        }
      }' \
      <<<"$release_build_provenance"
  )"
  printf '[hepta-preflight-provenance] %s\n' "$release_build_provenance"
fi

echo "Hepta preflight passed"
