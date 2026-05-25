#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

capture_json_report() {
  local command_name="$1"
  shift

  local output
  output="$("$@")"
  local report
  report="$(printf '%s\n' "$output" | sed '$d')"

  if ! jq -e . >/dev/null <<<"$report"; then
    echo "$command_name did not emit a parseable JSON report" >&2
    exit 1
  fi

  printf '%s\n' "$report"
}

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

OPERATOR_SCOPE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-operator-scope-binding-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-operator-scope-binding-gate.sh
)"

operator_scope_report_sha256="$(printf '%s' "$OPERATOR_SCOPE_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson scope "$OPERATOR_SCOPE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $scope.runtime == "hepta"
    and $scope.status == "ready"
    and $scope.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_operator_scope_binding_gate"
    and $scope.operator_scope_binding_ready == true
    and $scope.operator_identity_binding_recorded == false
    and $scope.single_surface_activation_scope_recorded == false
    and $scope.approved_surface_count == 0
    and $scope.maximum_allowed_surface_count == 1
    and $scope.approval_packet_recorded == false
    and $scope.approval_packet_persisted == false
    and $scope.no_secret_payload_review_recorded == false
    and $scope.pre_activation_soak_evidence_persistence_allowed == false
    and $scope.receipt_persistence_enabled == false
    and $scope.receipt_persisted == false
    and $scope.activation_allowed == false
    and $scope.live_mutation_execution_ready == false
    and $scope.required_operator_scope_binding_field_count == 12
    and $scope.recorded_operator_scope_binding_field_count == 0
    and ($scope.allowed_single_surface_activation_scopes | length) == 10
    and ($scope.denied_operator_scope_fixtures | length) == 5
    and ($scope.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_no_secret_payload_review_gate" \
  --arg operator_scope_report_sha256 "$operator_scope_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson scope "$OPERATOR_SCOPE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_operator_scope_gate:$scope.gate,
    source_operator_scope_binding_ready:$scope.operator_scope_binding_ready,
    source_receipt_payload_sha256:$scope.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$scope.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$scope.source_persistence_denial_report_sha256,
    source_approval_packet_report_sha256:$scope.source_approval_packet_report_sha256,
    source_operator_scope_report_sha256:$operator_scope_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    no_secret_payload_review_ready:true,
    no_secret_payload_review_recorded:false,
    no_secret_payload_review_id_recorded:false,
    payload_manifest_recorded:false,
    payload_hash_recorded:false,
    payload_redacted_summary_hash_recorded:false,
    payload_plaintext_recorded:false,
    secret_scanner_policy_recorded:false,
    external_send_policy_recorded:false,
    path_redaction_policy_recorded:false,
    credential_read_denial_policy_recorded:false,
    live_payload_review_performed:false,
    approved_payload_count:0,
    reviewed_payload_count:0,
    blocked_payload_fixture_count:6,
    raw_secret_marker_allowed:false,
    unredacted_path_allowed:false,
    channel_recipient_payload_allowed:false,
    provider_prompt_payload_allowed:false,
    public_artifact_payload_allowed:false,
    multi_surface_payload_allowed:false,
    memory_store_payload_allowed:false,
    registry_mutation_payload_allowed:false,
    gateway_event_payload_allowed:false,
    approval_packet_recorded:false,
    approval_packet_persisted:false,
    operator_scope_binding_recorded:false,
    operator_scope_binding_persisted:false,
    pre_activation_soak_evidence_persistence_allowed:false,
    receipt_persistence_enabled:false,
    receipt_persisted:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    required_no_secret_payload_review_field_count:14,
    recorded_no_secret_payload_review_field_count:0,
    redacted_or_hashed_field_count:12,
    reviewable_single_surface_payload_kinds:[
      "memory_store_mutation_request",
      "capability_registry_update",
      "plugin_registry_update",
      "coding_agent_spawn_request",
      "search_provider_live_query",
      "skill_workshop_patch",
      "provider_model_prompt",
      "channel_delivery_payload",
      "runtime_store_patch",
      "gateway_event_payload"
    ],
    required_no_secret_payload_review_fields:[
      "no_secret_payload_review_id",
      "reviewer_identity_hash",
      "reviewed_payload_kind",
      "single_surface_activation_scope",
      "reviewed_payload_sha256",
      "reviewed_payload_redacted_summary_sha256",
      "secret_scanner_policy_id",
      "external_send_policy_id",
      "path_redaction_policy_id",
      "credential_read_denial_policy_id",
      "source_operator_scope_report_sha256",
      "source_approval_packet_report_sha256",
      "rollback_plan_id",
      "review_captured_at_unix"
    ],
    denied_payload_review_fixtures:[
      {
        id:"raw-credential-marker",
        payload_kind:"provider_model_prompt",
        contains_raw_secret_marker:true,
        review_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"raw_secret_marker_denied"
      },
      {
        id:"unredacted-home-or-workspace-path",
        payload_kind:"runtime_store_patch",
        contains_unredacted_path:true,
        review_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"unredacted_filesystem_path_denied"
      },
      {
        id:"channel-delivery-recipient-payload",
        payload_kind:"channel_delivery_payload",
        contains_channel_recipient:true,
        external_send_requested:true,
        review_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"channel_delivery_payload_requires_separate_delivery_policy"
      },
      {
        id:"public-artifact-output-path",
        payload_kind:"runtime_store_patch",
        public_artifact_path_requested:true,
        review_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        public_claim_allowed:false,
        release_artifact_write_allowed:false,
        reason:"public_artifact_path_denied"
      },
      {
        id:"multi-surface-payload",
        payload_kind:"mixed_payload",
        requested_surface_count:2,
        review_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"payload_must_bind_to_exactly_one_surface"
      },
      {
        id:"provider-prompt-with-hidden-context",
        payload_kind:"provider_model_prompt",
        provider_invocation_requested:true,
        hidden_context_export_requested:true,
        review_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"provider_payload_requires_no_secret_review_and_separate_invocation_approval"
      }
    ],
    denied_by_no_secret_payload_review_gate:[
      "no_secret_payload_review_not_recorded",
      "payload_manifest_not_recorded",
      "reviewed_payload_hash_not_recorded",
      "secret_scanner_policy_not_recorded",
      "external_send_policy_not_recorded",
      "path_redaction_policy_not_recorded",
      "credential_read_denial_policy_not_recorded",
      "raw_secret_marker_denied",
      "unredacted_path_denied",
      "channel_delivery_payload_denied",
      "public_artifact_payload_denied",
      "multi_surface_payload_denied"
    ],
    required_before_payload_review_acceptance:[
      "reviewer_identity_hash",
      "no_secret_payload_review_id",
      "exactly_one_reviewed_payload_kind",
      "single_surface_scope_binding",
      "reviewed_payload_sha256",
      "redacted_payload_summary_hash",
      "secret_scanner_policy_id",
      "external_send_policy_id",
      "path_redaction_policy_id",
      "credential_read_denial_policy_id",
      "source_operator_scope_report_hash_binding",
      "source_approval_packet_report_hash_binding",
      "rollback_plan_id",
      "review_timestamp"
    ],
    side_effects:{
      memory_store_mutated:false,
      capability_registry_mutated:false,
      plugin_registry_mutated:false,
      coding_agent_spawned:false,
      skill_workshop_written:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      runtime_store_mutated:false,
      gateway_event_enqueued:false,
      filesystem_written:false,
      release_artifact_written:false,
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      receipt_persisted:false,
      pre_activation_soak_evidence_persisted:false,
      approval_packet_persisted:false,
      operator_scope_binding_persisted:false,
      payload_review_persisted:false,
      payload_plaintext_persisted:false,
      external_send_performed:false,
      credential_read:false,
      secret_file_read:false
    }
  }')"

jq -e '
  .status == "ready"
  and .no_secret_payload_review_ready == true
  and .source_operator_scope_binding_ready == true
  and .source_receipt_payload_sha256 != ""
  and .source_pre_activation_soak_report_sha256 != ""
  and .source_persistence_denial_report_sha256 != ""
  and .source_approval_packet_report_sha256 != ""
  and .source_operator_scope_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .no_secret_payload_review_recorded == false
  and .no_secret_payload_review_id_recorded == false
  and .payload_manifest_recorded == false
  and .payload_hash_recorded == false
  and .payload_plaintext_recorded == false
  and .live_payload_review_performed == false
  and .approved_payload_count == 0
  and .reviewed_payload_count == 0
  and .blocked_payload_fixture_count == 6
  and .raw_secret_marker_allowed == false
  and .unredacted_path_allowed == false
  and .channel_recipient_payload_allowed == false
  and .provider_prompt_payload_allowed == false
  and .public_artifact_payload_allowed == false
  and .multi_surface_payload_allowed == false
  and .approval_packet_recorded == false
  and .approval_packet_persisted == false
  and .operator_scope_binding_recorded == false
  and .operator_scope_binding_persisted == false
  and .pre_activation_soak_evidence_persistence_allowed == false
  and .receipt_persistence_enabled == false
  and .receipt_persisted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .required_no_secret_payload_review_field_count == 14
  and .recorded_no_secret_payload_review_field_count == 0
  and (.reviewable_single_surface_payload_kinds | length) == 10
  and (.denied_payload_review_fixtures | length) == 6
  and (.denied_payload_review_fixtures | all(.review_accepted == false and .persistence_allowed == false and .activation_allowed == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence no-secret payload review gate passed"
