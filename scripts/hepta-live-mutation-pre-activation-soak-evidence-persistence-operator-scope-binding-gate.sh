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

APPROVAL_PACKET_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-approval-packet-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-approval-packet-gate.sh
)"

approval_packet_report_sha256="$(printf '%s' "$APPROVAL_PACKET_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson approval "$APPROVAL_PACKET_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $approval.runtime == "hepta"
    and $approval.status == "ready"
    and $approval.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_approval_packet_gate"
    and $approval.approval_packet_shape_ready == true
    and $approval.approval_packet_recorded == false
    and $approval.approval_packet_persisted == false
    and $approval.approval_packet_accepted == false
    and $approval.operator_approval_recorded == false
    and $approval.single_surface_activation_scope_recorded == false
    and $approval.pre_activation_soak_evidence_persistence_allowed == false
    and $approval.receipt_persistence_enabled == false
    and $approval.receipt_persisted == false
    and $approval.activation_allowed == false
    and $approval.live_mutation_execution_ready == false
    and $approval.required_approval_packet_field_count == 14
    and $approval.recorded_approval_packet_field_count == 0
    and ($approval.denied_approval_packet_fixtures | length) == 4
    and ($approval.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_operator_scope_binding_gate" \
  --arg approval_packet_report_sha256 "$approval_packet_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson approval "$APPROVAL_PACKET_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_approval_packet_gate:$approval.gate,
    source_approval_packet_shape_ready:$approval.approval_packet_shape_ready,
    source_receipt_payload_sha256:$approval.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$approval.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$approval.source_persistence_denial_report_sha256,
    source_approval_packet_report_sha256:$approval_packet_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    operator_scope_binding_ready:true,
    operator_identity_binding_recorded:false,
    operator_identity_hash_recorded:false,
    operator_approval_id_recorded:false,
    operator_approval_signature_recorded:false,
    operator_approval_timestamp_recorded:false,
    single_surface_activation_scope_recorded:false,
    single_surface_scope_validated:false,
    allowed_surface_selected:false,
    approved_surface_count:0,
    maximum_allowed_surface_count:1,
    approval_packet_recorded:false,
    approval_packet_persisted:false,
    approval_packet_accepted:false,
    source_receipt_hash_bound:false,
    fresh_soak_evidence_recorded:false,
    fresh_soak_evidence_bound:false,
    installed_binary_sha_after_approval_recorded:false,
    rollback_plan_recorded:false,
    no_secret_payload_review_recorded:false,
    public_claim_or_release_artifact_allowed:false,
    pre_activation_soak_evidence_persistence_allowed:false,
    receipt_persistence_enabled:false,
    receipt_persisted:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    required_operator_scope_binding_field_count:12,
    recorded_operator_scope_binding_field_count:0,
    redacted_or_hashed_field_count:10,
    allowed_single_surface_activation_scopes:[
      "memory_store_mutation",
      "capability_registry_mutation",
      "plugin_registry_mutation",
      "coding_agent_spawn",
      "search_provider_live_query",
      "skill_workshop_write",
      "provider_model_invocation",
      "channel_delivery",
      "runtime_store_mutation",
      "gateway_event_enqueue"
    ],
    required_operator_scope_binding_fields:[
      "operator_approval_id",
      "operator_identity_hash",
      "operator_approval_signature_hash",
      "operator_approval_captured_at_unix",
      "single_surface_activation_scope",
      "single_surface_scope_reason",
      "source_approval_packet_report_sha256",
      "source_receipt_payload_sha256",
      "source_pre_activation_soak_report_sha256",
      "fresh_soak_evidence_record_id",
      "rollback_plan_id",
      "no_secret_payload_review_id"
    ],
    denied_operator_scope_fixtures:[
      {
        id:"missing-operator-identity",
        operator_identity_hash_recorded:false,
        operator_approval_id_recorded:false,
        single_surface_activation_scope_recorded:true,
        approved_surface_count:1,
        binding_accepted:false,
        persistence_allowed:false,
        reason:"operator_identity_hash_and_approval_id_missing"
      },
      {
        id:"multi-surface-activation-scope",
        operator_identity_hash_recorded:true,
        operator_approval_id_recorded:true,
        single_surface_activation_scope_recorded:false,
        approved_surface_count:2,
        binding_accepted:false,
        persistence_allowed:false,
        reason:"approval_scope_must_select_exactly_one_surface"
      },
      {
        id:"unsupported-surface-scope",
        operator_identity_hash_recorded:true,
        operator_approval_id_recorded:true,
        single_surface_activation_scope_recorded:true,
        approved_surface_count:1,
        requested_surface:"filesystem_receipt_persistence",
        binding_accepted:false,
        persistence_allowed:false,
        reason:"requested_surface_not_in_allowed_live_mutation_scope_allowlist"
      },
      {
        id:"operator-scope-without-fresh-soak-or-rollback",
        operator_identity_hash_recorded:true,
        operator_approval_id_recorded:true,
        single_surface_activation_scope_recorded:true,
        approved_surface_count:1,
        fresh_soak_evidence_recorded:false,
        rollback_plan_recorded:false,
        binding_accepted:false,
        persistence_allowed:false,
        reason:"fresh_soak_evidence_and_rollback_plan_missing"
      },
      {
        id:"public-claim-release-artifact-with-operator-scope",
        operator_identity_hash_recorded:true,
        operator_approval_id_recorded:true,
        single_surface_activation_scope_recorded:true,
        approved_surface_count:1,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        binding_accepted:false,
        persistence_allowed:false,
        public_claim_allowed:false,
        release_artifact_write_allowed:false,
        reason:"public_claim_and_release_artifact_write_denied"
      }
    ],
    denied_by_operator_scope_binding_gate:[
      "operator_identity_binding_not_recorded",
      "operator_approval_id_not_recorded",
      "operator_approval_signature_not_recorded",
      "single_surface_activation_scope_not_recorded",
      "single_surface_scope_not_validated",
      "approval_packet_not_recorded",
      "fresh_soak_evidence_not_bound",
      "rollback_plan_not_recorded",
      "no_secret_payload_review_not_recorded"
    ],
    required_before_scope_binding_acceptance:[
      "redacted_operator_identity_hash",
      "operator_approval_id",
      "operator_approval_signature_hash",
      "approval_timestamp",
      "exactly_one_allowed_activation_surface",
      "source_approval_packet_hash_binding",
      "source_receipt_payload_hash_binding",
      "source_pre_activation_soak_report_hash_binding",
      "fresh_24_sample_pre_activation_soak_evidence_record",
      "reviewed_rollback_plan_id",
      "no_secret_payload_review",
      "public_claim_and_artifact_decision_denied_or_separately_approved"
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
      external_send_performed:false,
      credential_read:false
    }
  }')"

jq -e '
  .status == "ready"
  and .operator_scope_binding_ready == true
  and .source_approval_packet_shape_ready == true
  and .source_receipt_payload_sha256 != ""
  and .source_pre_activation_soak_report_sha256 != ""
  and .source_persistence_denial_report_sha256 != ""
  and .source_approval_packet_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .operator_identity_binding_recorded == false
  and .operator_identity_hash_recorded == false
  and .operator_approval_id_recorded == false
  and .single_surface_activation_scope_recorded == false
  and .single_surface_scope_validated == false
  and .approved_surface_count == 0
  and .maximum_allowed_surface_count == 1
  and .approval_packet_recorded == false
  and .approval_packet_persisted == false
  and .approval_packet_accepted == false
  and .pre_activation_soak_evidence_persistence_allowed == false
  and .receipt_persistence_enabled == false
  and .receipt_persisted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .required_operator_scope_binding_field_count == 12
  and .recorded_operator_scope_binding_field_count == 0
  and (.allowed_single_surface_activation_scopes | length) == 10
  and (.denied_operator_scope_fixtures | length) == 5
  and (.denied_operator_scope_fixtures | all(.binding_accepted == false and .persistence_allowed == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence operator scope binding gate passed"
