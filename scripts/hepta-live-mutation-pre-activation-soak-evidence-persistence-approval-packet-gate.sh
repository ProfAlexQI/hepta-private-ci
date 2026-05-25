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

PERSISTENCE_DENIAL_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-denial-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-denial-gate.sh
)"

persistence_denial_report_sha256="$(printf '%s' "$PERSISTENCE_DENIAL_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson denial "$PERSISTENCE_DENIAL_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $denial.runtime == "hepta"
    and $denial.status == "ready"
    and $denial.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_denial_gate"
    and $denial.persistence_denial_gate_ready == true
    and $denial.pre_activation_soak_evidence_persistence_allowed == false
    and $denial.fresh_soak_evidence_recorded == false
    and $denial.long_soak_evidence_persisted == false
    and $denial.receipt_persistence_enabled == false
    and $denial.receipt_persisted == false
    and $denial.operator_approval_recorded == false
    and $denial.activation_allowed == false
    and $denial.live_mutation_execution_ready == false
    and $denial.required_persistence_field_count == 10
    and $denial.recorded_persistence_field_count == 0
    and ($denial.denied_persistence_fixtures | length) == 4
    and ($denial.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_approval_packet_gate" \
  --arg persistence_denial_report_sha256 "$persistence_denial_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson denial "$PERSISTENCE_DENIAL_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_persistence_denial_gate:$denial.gate,
    source_persistence_denial_gate_ready:$denial.persistence_denial_gate_ready,
    source_receipt_payload_sha256:$denial.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$denial.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$persistence_denial_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    approval_packet_shape_ready:true,
    approval_packet_recorded:false,
    approval_packet_persisted:false,
    approval_packet_accepted:false,
    operator_approval_recorded:false,
    single_surface_activation_scope_recorded:false,
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
    required_approval_packet_field_count:14,
    recorded_approval_packet_field_count:0,
    redacted_or_hashed_field_count:11,
    required_approval_packet_fields:[
      "approval_packet_id",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "source_receipt_payload_sha256",
      "source_pre_activation_soak_report_sha256",
      "fresh_soak_evidence_record_id",
      "fresh_soak_evidence_report_sha256",
      "fresh_soak_evidence_sample_count",
      "fresh_soak_evidence_captured_at_unix",
      "installed_binary_sha256_after_approval",
      "rollback_plan_id",
      "no_secret_payload_review_id",
      "public_claim_and_artifact_decision"
    ],
    denied_approval_packet_fixtures:[
      {
        id:"empty-approval-packet",
        recorded_approval_packet_field_count:0,
        operator_approval_recorded:false,
        fresh_soak_evidence_recorded:false,
        rollback_plan_recorded:false,
        packet_accepted:false,
        persistence_allowed:false,
        reason:"approval_packet_fields_missing"
      },
      {
        id:"operator-approved-without-fresh-soak-record",
        recorded_approval_packet_field_count:8,
        operator_approval_recorded:true,
        fresh_soak_evidence_recorded:false,
        rollback_plan_recorded:true,
        packet_accepted:false,
        persistence_allowed:false,
        reason:"fresh_24_sample_pre_activation_soak_record_missing"
      },
      {
        id:"fresh-soak-without-rollback-plan",
        recorded_approval_packet_field_count:10,
        operator_approval_recorded:true,
        fresh_soak_evidence_recorded:true,
        rollback_plan_recorded:false,
        packet_accepted:false,
        persistence_allowed:false,
        reason:"rollback_plan_id_missing"
      },
      {
        id:"public-claim-or-release-artifact-attempt",
        recorded_approval_packet_field_count:14,
        operator_approval_recorded:true,
        fresh_soak_evidence_recorded:true,
        rollback_plan_recorded:true,
        public_claim_requested:true,
        release_artifact_write_requested:true,
        packet_accepted:false,
        persistence_allowed:false,
        public_claim_allowed:false,
        release_artifact_write_allowed:false,
        reason:"public_claim_and_release_artifact_write_denied"
      }
    ],
    denied_by_approval_packet_gate:[
      "approval_packet_not_recorded",
      "single_surface_activation_scope_not_recorded",
      "source_receipt_hash_not_bound",
      "fresh_soak_evidence_not_bound",
      "installed_binary_sha_after_approval_not_recorded",
      "rollback_plan_not_recorded",
      "no_secret_payload_review_not_recorded"
    ],
    required_before_persistence:[
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "source_receipt_payload_hash_binding",
      "source_pre_activation_soak_report_hash_binding",
      "fresh_24_sample_pre_activation_soak_evidence_record",
      "installed_binary_sha_after_approval",
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
      external_send_performed:false,
      credential_read:false
    }
  }')"

jq -e '
  .status == "ready"
  and .approval_packet_shape_ready == true
  and .source_persistence_denial_gate_ready == true
  and .source_receipt_payload_sha256 != ""
  and .source_pre_activation_soak_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .approval_packet_recorded == false
  and .approval_packet_persisted == false
  and .approval_packet_accepted == false
  and .pre_activation_soak_evidence_persistence_allowed == false
  and .receipt_persistence_enabled == false
  and .receipt_persisted == false
  and .operator_approval_recorded == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .required_approval_packet_field_count == 14
  and .recorded_approval_packet_field_count == 0
  and (.denied_approval_packet_fixtures | length) == 4
  and (.denied_approval_packet_fixtures | all(.packet_accepted == false and .persistence_allowed == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence approval packet gate passed"
