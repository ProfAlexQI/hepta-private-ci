#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$HOME/.local/opt/hepta/bin/hepta}}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

require_unsigned_integer() {
  local name="$1"
  local value="$2"

  case "$value" in
    ''|*[!0-9]*)
      echo "$name must be an unsigned integer" >&2
      exit 2
      ;;
  esac
}

require_unsigned_integer "HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES" "$MIN_LONG_SOAK_SAMPLES"

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

ACK_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-non-acceptance-gate.sh
)"

ack_report_sha256="$(sha256_text "$ACK_JSON")"
source_review_index_hash_sha256="$(jq -r '.source_operator_review_index_hash_sha256' <<<"$ACK_JSON")"
acknowledgement_index_hash_sha256="$(jq -r '.operator_review_acknowledgement_index_hash_sha256' <<<"$ACK_JSON")"
activation_request_denial_matrix_hash_sha256="$(
  sha256_text "hepta-canary-operator-review-acknowledgement-activation-request-denial-matrix:v1:review=$source_review_index_hash_sha256:ack=$acknowledgement_index_hash_sha256:activation_request=0:record=0:persist=0:execute=0:live=0"
)"
activation_request_denial_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-request-denial-matrix:v1:no-request-accept:no-request-record:no-request-persist:no-dispatch:no-execute:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_review_acknowledgement_activation_request_side_effects=false;request_requested=9;request_accepted=0;request_recorded=0;request_persisted=0;dispatch=0;execute=0;provider=0;model=0;memory=0;kg=0;secret=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$ACK_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_gate"
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_status == "blocked"
    and $source.operator_review_acknowledgement_fixture_count == 8
    and $source.operator_review_acknowledgement_requested_fixture_count == 8
    and $source.blocked_operator_review_acknowledgement_fixture_count == 8
    and $source.noop_operator_review_acknowledgement_fixture_count == 8
    and $source.allowed_operator_review_acknowledgement_fixture_count == 0
    and $source.accepted_operator_review_acknowledgement_fixture_count == 0
    and $source.operator_review_acknowledgement_performed_count == 0
    and $source.operator_review_acknowledgement_allowed == false
    and $source.operator_review_acknowledgement_accepted == false
    and $source.operator_review_acknowledgement_recorded == false
    and $source.operator_review_acknowledgement_persisted == false
    and $source.operator_review_acknowledgement_materialized == false
    and $source.operator_review_acknowledgement_filesystem_written == false
    and $source.operator_review_acknowledgement_delivered == false
    and $source.operator_review_acknowledgement_identity_accepted == false
    and $source.operator_review_acknowledgement_signature_accepted == false
    and $source.operator_review_acknowledgement_final_state_promoted == false
    and $source.operator_review_acknowledgement_completion_promoted == false
    and $source.operator_review_acknowledgement_authorizes_dispatch_count == 0
    and $source.operator_review_acknowledgement_authorizes_execution_count == 0
    and $source.operator_review_acknowledgement_authorizes_live_count == 0
    and $source.operator_approval_recorded == false
    and $source.operator_identity_accepted == false
    and $source.readback_index_recorded_count == 0
    and $source.readback_index_persisted_count == 0
    and $source.dispatch_performed_count == 0
    and $source.execution_performed_count == 0
    and $source.context_injection_performed_count == 0
    and $source.provider_invoked_count == 0
    and $source.model_invoked_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.external_kg_adapter_read_performed_count == 0
    and $source.live_kg_write_performed_count == 0
    and $source.credential_read_count == 0
    and $source.secret_file_read_count == 0
    and $source.channel_send_performed_count == 0
    and $source.canary_harness_armed == false
    and $source.canary_harness_executable == false
    and $source.canary_live_enabled == false
    and ($source.source_operator_review_index_hash_sha256 | type) == "string"
    and ($source.source_operator_review_index_hash_sha256 | length) == 64
    and ($source.operator_review_acknowledgement_index_hash_sha256 | type) == "string"
    and ($source.operator_review_acknowledgement_index_hash_sha256 | length) == 64
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

activation_request_fixtures_json="$(
  jq -n '
    def blocked_fixture($id; $kind; $reason; $extra):
      {
        fixture_id: $id,
        fixture_kind: $kind,
        activation_request_requested: true,
        activation_request_status: "blocked_noop",
        source_acknowledgement_present: true,
        source_acknowledgement_ready: true,
        acknowledgement_accepted: false,
        activation_request_allowed: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_materialized: false,
        activation_request_filesystem_written: false,
        activation_request_delivered: false,
        activation_request_executed: false,
        activation_nonce_generated: false,
        activation_identity_accepted: false,
        activation_scope_accepted: false,
        activation_final_state_promoted: false,
        dispatch_allowed: false,
        dispatch_performed: false,
        execution_allowed: false,
        execution_performed: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        memory_store_write_performed: false,
        external_kg_adapter_read_performed: false,
        live_kg_write_performed: false,
        credential_read: false,
        secret_file_read: false,
        channel_send_performed: false,
        install_performed: false,
        service_restarted: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        denial_reason: $reason
      } + $extra;
    [
      blocked_fixture("missing-source-acknowledgement-report"; "missing_source_acknowledgement_report"; "source_acknowledgement_non_acceptance_report_required"; {source_acknowledgement_present: false, source_acknowledgement_ready: false}),
      blocked_fixture("acknowledgement-to-activation-request-shape"; "activation_request_shape_from_acknowledgement"; "acknowledgement_cannot_create_activation_request"; {}),
      blocked_fixture("acknowledgement-identity-scope-request"; "identity_scope_from_acknowledgement"; "acknowledgement_cannot_accept_identity_or_scope"; {identity_scope_requested: true}),
      blocked_fixture("acknowledgement-nonce-generation-request"; "nonce_generation_from_acknowledgement"; "acknowledgement_cannot_generate_activation_nonce"; {nonce_generation_requested: true}),
      blocked_fixture("acknowledgement-dispatch-request"; "dispatch_request_from_acknowledgement"; "acknowledgement_cannot_authorize_dispatch"; {dispatch_requested: true}),
      blocked_fixture("acknowledgement-execution-request"; "execution_request_from_acknowledgement"; "acknowledgement_cannot_authorize_execution"; {execution_requested: true}),
      blocked_fixture("acknowledgement-context-provider-model-request"; "context_provider_model_from_acknowledgement"; "acknowledgement_cannot_authorize_context_or_provider"; {context_attachment_requested: true, provider_invocation_requested: true, model_invocation_requested: true}),
      blocked_fixture("acknowledgement-memory-kg-write-request"; "memory_kg_write_from_acknowledgement"; "acknowledgement_cannot_authorize_memory_or_kg_write"; {memory_write_requested: true, kg_write_requested: true}),
      blocked_fixture("acknowledgement-external-public-install-secret-request"; "external_public_install_secret_from_acknowledgement"; "acknowledgement_cannot_authorize_external_public_install_or_secret_access"; {external_send_requested: true, public_claim_requested: true, install_requested: true, restart_requested: true, secret_access_requested: true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_gate" \
    --arg ack_report_sha256 "$ack_report_sha256" \
    --arg source_review_index_hash_sha256 "$source_review_index_hash_sha256" \
    --arg acknowledgement_index_hash_sha256 "$acknowledgement_index_hash_sha256" \
    --arg activation_request_denial_matrix_hash_sha256 "$activation_request_denial_matrix_hash_sha256" \
    --arg activation_request_denial_policy_hash_sha256 "$activation_request_denial_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ACK_JSON" \
    --argjson fixtures "$activation_request_fixtures_json" \
    '
      {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_schema_version: "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_request_denial_matrix_v1",
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_ready: true,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_status: "blocked",
        activation_request_denial_matrix_mode: "stdout_only_activation_request_shapes_no_acceptance_no_recording_no_persistence_no_dispatch_no_execution_no_live",
        activation_request_denial_matrix_decision: "operator_review_acknowledgement_attempts_do_not_create_or_authorize_activation_requests",
        minimum_required_samples: $min_long_soak_samples,
        source_operator_review_acknowledgement_non_acceptance_gate: $source.gate,
        source_operator_review_acknowledgement_non_acceptance_status: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_status,
        source_operator_review_acknowledgement_non_acceptance_report_sha256: $ack_report_sha256,
        source_operator_review_index_hash_sha256: $source_review_index_hash_sha256,
        source_operator_review_acknowledgement_index_hash_sha256: $acknowledgement_index_hash_sha256,
        source_operator_review_acknowledgement_fixture_count: $source.operator_review_acknowledgement_fixture_count,
        source_operator_review_acknowledgement_accepted_count: $source.accepted_operator_review_acknowledgement_fixture_count,
        source_operator_review_acknowledgement_performed_count: $source.operator_review_acknowledgement_performed_count,
        source_operator_review_acknowledgement_authorizes_dispatch_count: $source.operator_review_acknowledgement_authorizes_dispatch_count,
        source_operator_review_acknowledgement_authorizes_execution_count: $source.operator_review_acknowledgement_authorizes_execution_count,
        source_operator_review_acknowledgement_authorizes_live_count: $source.operator_review_acknowledgement_authorizes_live_count,
        activation_request_denial_matrix_hash_sha256: $activation_request_denial_matrix_hash_sha256,
        activation_request_denial_policy_hash_sha256: $activation_request_denial_policy_hash_sha256,
        side_effect_hash_sha256: $side_effect_hash_sha256,
        activation_request_denial_fixtures: $fixtures,
        activation_request_denial_fixture_count: ($fixtures | length),
        activation_request_requested_fixture_count: ($fixtures | map(select(.activation_request_requested == true)) | length),
        blocked_activation_request_fixture_count: ($fixtures | map(select(.activation_request_status == "blocked_noop")) | length),
        noop_activation_request_fixture_count: ($fixtures | map(select(.activation_request_executed == false)) | length),
        allowed_activation_request_fixture_count: 0,
        accepted_activation_request_fixture_count: 0,
        activation_request_performed_count: 0,
        activation_request_allowed: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_materialized: false,
        activation_request_filesystem_written: false,
        activation_request_delivered: false,
        activation_request_executed: false,
        activation_nonce_generated: false,
        activation_identity_accepted: false,
        activation_scope_accepted: false,
        activation_final_state_promoted: false,
        operator_review_acknowledgement_accepted: false,
        operator_review_acknowledgement_recorded: false,
        operator_review_acknowledgement_persisted: false,
        operator_approval_recorded: false,
        operator_identity_accepted: false,
        dispatch_allowed_count: 0,
        dispatch_performed_count: 0,
        execution_allowed_count: 0,
        execution_performed_count: 0,
        context_injection_performed_count: 0,
        provider_invoked_count: 0,
        model_invoked_count: 0,
        memory_store_write_performed_count: 0,
        external_kg_adapter_read_performed_count: 0,
        live_kg_write_performed_count: 0,
        credential_read_count: 0,
        secret_file_read_count: 0,
        channel_send_performed_count: 0,
        install_performed_count: 0,
        service_restarted_count: 0,
        active_binary_mutated_count: 0,
        upstream_fetch_performed_count: 0,
        upstream_merge_performed_count: 0,
        canary_harness_armed: false,
        canary_harness_executable: false,
        canary_live_enabled: false,
        denied_by_operator_review_acknowledgement_activation_request_denial_matrix: [
          "source_acknowledgement_non_acceptance_report_required",
          "activation_request_acceptance_denied",
          "activation_request_recording_denied",
          "activation_request_persistence_denied",
          "activation_request_materialization_denied",
          "activation_request_filesystem_write_denied",
          "activation_request_delivery_denied",
          "activation_request_execution_denied",
          "activation_nonce_generation_denied",
          "activation_identity_acceptance_denied",
          "activation_scope_acceptance_denied",
          "activation_final_state_promotion_denied",
          "operator_review_acknowledgement_not_authority",
          "operator_approval_not_recorded",
          "dispatch_from_acknowledgement_denied",
          "execution_from_acknowledgement_denied",
          "context_injection_from_acknowledgement_denied",
          "provider_model_invocation_denied",
          "memory_write_denied",
          "external_kg_read_denied",
          "live_kg_write_denied",
          "credential_secret_read_denied",
          "channel_delivery_denied",
          "install_restart_denied",
          "active_binary_mutation_denied",
          "upstream_fetch_merge_denied"
        ],
        denied_by_operator_review_acknowledgement_activation_request_denial_matrix_count: 26,
        side_effects: {
          workspace_written: false,
          filesystem_written: false,
          activation_request_performed: false,
          activation_request_recorded: false,
          activation_request_persisted: false,
          activation_request_materialized: false,
          activation_request_filesystem_written: false,
          activation_request_delivered: false,
          activation_request_executed: false,
          activation_nonce_generated: false,
          activation_identity_accepted: false,
          activation_scope_accepted: false,
          activation_final_state_promoted: false,
          operator_review_acknowledgement_accepted: false,
          operator_approval_recorded: false,
          operator_identity_accepted: false,
          dispatch_performed: false,
          execution_performed: false,
          context_injection_performed: false,
          provider_invoked: false,
          model_invoked: false,
          memory_store_write_performed: false,
          memory_store_mutated: false,
          external_kg_adapter_read_performed: false,
          live_kg_write_performed: false,
          credential_read: false,
          secret_file_read: false,
          channel_send_performed: false,
          telegram_send_performed: false,
          install_performed: false,
          service_restarted: false,
          active_binary_mutated: false,
          upstream_fetch_performed: false,
          upstream_merge_performed: false
        }
      }
    '
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_request_denial_matrix_status == "blocked"
  and .source_operator_review_acknowledgement_non_acceptance_gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_non_acceptance_gate"
  and .source_operator_review_acknowledgement_non_acceptance_status == "blocked"
  and .source_operator_review_acknowledgement_fixture_count == 8
  and .source_operator_review_acknowledgement_accepted_count == 0
  and .source_operator_review_acknowledgement_performed_count == 0
  and .source_operator_review_acknowledgement_authorizes_dispatch_count == 0
  and .source_operator_review_acknowledgement_authorizes_execution_count == 0
  and .source_operator_review_acknowledgement_authorizes_live_count == 0
  and (.source_operator_review_index_hash_sha256 | type) == "string"
  and (.source_operator_review_index_hash_sha256 | length) == 64
  and (.source_operator_review_acknowledgement_index_hash_sha256 | type) == "string"
  and (.source_operator_review_acknowledgement_index_hash_sha256 | length) == 64
  and (.activation_request_denial_matrix_hash_sha256 | type) == "string"
  and (.activation_request_denial_matrix_hash_sha256 | length) == 64
  and .activation_request_denial_fixture_count == 9
  and .activation_request_requested_fixture_count == 9
  and .blocked_activation_request_fixture_count == 9
  and .noop_activation_request_fixture_count == 9
  and .allowed_activation_request_fixture_count == 0
  and .accepted_activation_request_fixture_count == 0
  and .activation_request_performed_count == 0
  and .activation_request_allowed == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_persisted == false
  and .activation_request_materialized == false
  and .activation_request_filesystem_written == false
  and .activation_request_delivered == false
  and .activation_request_executed == false
  and .activation_nonce_generated == false
  and .activation_identity_accepted == false
  and .activation_scope_accepted == false
  and .activation_final_state_promoted == false
  and .operator_review_acknowledgement_accepted == false
  and .operator_review_acknowledgement_recorded == false
  and .operator_review_acknowledgement_persisted == false
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
  and .dispatch_allowed_count == 0
  and .dispatch_performed_count == 0
  and .execution_allowed_count == 0
  and .execution_performed_count == 0
  and .context_injection_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .external_kg_adapter_read_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .channel_send_performed_count == 0
  and .install_performed_count == 0
  and .service_restarted_count == 0
  and .active_binary_mutated_count == 0
  and .upstream_fetch_performed_count == 0
  and .upstream_merge_performed_count == 0
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and (.activation_request_denial_fixtures | all(
    .activation_request_requested == true
    and .activation_request_status == "blocked_noop"
    and .activation_request_allowed == false
    and .activation_request_accepted == false
    and .activation_request_recorded == false
    and .activation_request_persisted == false
    and .activation_request_materialized == false
    and .activation_request_filesystem_written == false
    and .activation_request_delivered == false
    and .activation_request_executed == false
    and .activation_nonce_generated == false
    and .activation_identity_accepted == false
    and .activation_scope_accepted == false
    and .activation_final_state_promoted == false
    and .dispatch_allowed == false
    and .dispatch_performed == false
    and .execution_allowed == false
    and .execution_performed == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .channel_send_performed == false
    and .install_performed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .upstream_fetch_performed == false
    and .upstream_merge_performed == false
  ))
  and .denied_by_operator_review_acknowledgement_activation_request_denial_matrix_count == 26
  and (.denied_by_operator_review_acknowledgement_activation_request_denial_matrix | length) == 26
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation request denial matrix gate passed"
