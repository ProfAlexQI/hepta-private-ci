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

NO_SECRET_REVIEW_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-no-secret-payload-review-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-no-secret-payload-review-gate.sh
)"

no_secret_payload_review_report_sha256="$(printf '%s' "$NO_SECRET_REVIEW_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson review "$NO_SECRET_REVIEW_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $review.runtime == "hepta"
    and $review.status == "ready"
    and $review.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_no_secret_payload_review_gate"
    and $review.no_secret_payload_review_ready == true
    and $review.no_secret_payload_review_recorded == false
    and $review.payload_manifest_recorded == false
    and $review.payload_hash_recorded == false
    and $review.payload_plaintext_recorded == false
    and $review.live_payload_review_performed == false
    and $review.approved_payload_count == 0
    and $review.reviewed_payload_count == 0
    and $review.blocked_payload_fixture_count == 6
    and $review.receipt_persistence_enabled == false
    and $review.receipt_persisted == false
    and $review.activation_allowed == false
    and $review.live_mutation_execution_ready == false
    and $review.required_no_secret_payload_review_field_count == 14
    and $review.recorded_no_secret_payload_review_field_count == 0
    and ($review.reviewable_single_surface_payload_kinds | length) == 10
    and ($review.denied_payload_review_fixtures | length) == 6
    and ($review.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_proof_gate" \
  --arg no_secret_payload_review_report_sha256 "$no_secret_payload_review_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson review "$NO_SECRET_REVIEW_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_no_secret_payload_review_gate:$review.gate,
    source_no_secret_payload_review_ready:$review.no_secret_payload_review_ready,
    source_receipt_payload_sha256:$review.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$review.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$review.source_persistence_denial_report_sha256,
    source_approval_packet_report_sha256:$review.source_approval_packet_report_sha256,
    source_operator_scope_report_sha256:$review.source_operator_scope_report_sha256,
    source_no_secret_payload_review_report_sha256:$no_secret_payload_review_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_proof_ready:true,
    payload_redaction_proof_recorded:false,
    payload_redaction_proof_id_recorded:false,
    redaction_policy_recorded:false,
    redacted_summary_hash_recorded:false,
    raw_payload_hash_recorded:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    redacted_summary_plaintext_recorded:false,
    reviewed_payload_kind_recorded:false,
    single_surface_scope_binding_recorded:false,
    reviewer_identity_hash_recorded:false,
    proof_timestamp_recorded:false,
    approved_redaction_proof_count:0,
    reviewed_redaction_proof_count:0,
    blocked_redaction_fixture_count:6,
    raw_secret_marker_after_redaction_allowed:false,
    unredacted_path_after_redaction_allowed:false,
    channel_recipient_after_redaction_allowed:false,
    provider_hidden_context_after_redaction_allowed:false,
    public_artifact_path_after_redaction_allowed:false,
    redacted_summary_equals_raw_payload_allowed:false,
    no_secret_payload_review_recorded:false,
    approval_packet_recorded:false,
    approval_packet_persisted:false,
    operator_scope_binding_recorded:false,
    operator_scope_binding_persisted:false,
    payload_review_persisted:false,
    payload_redaction_proof_persisted:false,
    pre_activation_soak_evidence_persistence_allowed:false,
    receipt_persistence_enabled:false,
    receipt_persisted:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    required_payload_redaction_proof_field_count:14,
    recorded_payload_redaction_proof_field_count:0,
    redacted_or_hashed_field_count:13,
    required_payload_redaction_proof_fields:[
      "payload_redaction_proof_id",
      "no_secret_payload_review_id",
      "reviewed_payload_kind",
      "single_surface_activation_scope",
      "raw_payload_sha256",
      "redacted_payload_summary_sha256",
      "redaction_policy_id",
      "secret_scanner_policy_id",
      "path_redaction_policy_id",
      "external_recipient_redaction_policy_id",
      "source_no_secret_payload_review_report_sha256",
      "source_operator_scope_report_sha256",
      "reviewer_identity_hash",
      "proof_captured_at_unix"
    ],
    denied_redaction_proof_fixtures:[
      {
        id:"missing-redacted-summary-hash",
        raw_payload_hash_recorded:true,
        redacted_summary_hash_recorded:false,
        proof_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"redacted_summary_hash_required"
      },
      {
        id:"redacted-summary-equals-raw-payload",
        raw_payload_hash_recorded:true,
        redacted_summary_hash_recorded:true,
        redacted_summary_equals_raw_payload:true,
        proof_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"redacted_summary_must_not_equal_raw_payload"
      },
      {
        id:"raw-secret-marker-after-redaction",
        redacted_summary_hash_recorded:true,
        secret_marker_after_redaction:true,
        proof_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"redacted_summary_still_contains_secret_marker"
      },
      {
        id:"unredacted-path-after-redaction",
        redacted_summary_hash_recorded:true,
        unredacted_path_after_redaction:true,
        proof_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"redacted_summary_still_contains_unredacted_path"
      },
      {
        id:"channel-recipient-after-redaction",
        redacted_summary_hash_recorded:true,
        channel_recipient_after_redaction:true,
        proof_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"redacted_summary_still_contains_delivery_recipient"
      },
      {
        id:"public-artifact-path-after-redaction",
        redacted_summary_hash_recorded:true,
        public_artifact_path_after_redaction:true,
        proof_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        public_claim_allowed:false,
        release_artifact_write_allowed:false,
        reason:"redacted_summary_still_contains_public_artifact_path"
      }
    ],
    denied_by_payload_redaction_proof_gate:[
      "payload_redaction_proof_not_recorded",
      "redaction_policy_not_recorded",
      "raw_payload_hash_not_recorded",
      "redacted_summary_hash_not_recorded",
      "raw_payload_plaintext_denied",
      "redacted_summary_equals_raw_payload_denied",
      "secret_marker_after_redaction_denied",
      "unredacted_path_after_redaction_denied",
      "channel_recipient_after_redaction_denied",
      "public_artifact_path_after_redaction_denied"
    ],
    required_before_redaction_proof_acceptance:[
      "payload_redaction_proof_id",
      "source_no_secret_payload_review_hash_binding",
      "source_operator_scope_hash_binding",
      "single_surface_scope_binding",
      "reviewed_payload_kind",
      "raw_payload_sha256",
      "redacted_payload_summary_sha256",
      "redaction_policy_id",
      "secret_scanner_policy_id",
      "path_redaction_policy_id",
      "external_recipient_redaction_policy_id",
      "reviewer_identity_hash",
      "proof_timestamp",
      "plaintext_payload_absent_from_record"
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
      payload_redaction_proof_persisted:false,
      payload_plaintext_persisted:false,
      raw_payload_inspected:false,
      live_secret_scan_performed:false,
      external_send_performed:false,
      credential_read:false,
      secret_file_read:false
    }
  }')"

jq -e '
  .status == "ready"
  and .payload_redaction_proof_ready == true
  and .source_no_secret_payload_review_ready == true
  and .source_receipt_payload_sha256 != ""
  and .source_pre_activation_soak_report_sha256 != ""
  and .source_persistence_denial_report_sha256 != ""
  and .source_approval_packet_report_sha256 != ""
  and .source_operator_scope_report_sha256 != ""
  and .source_no_secret_payload_review_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .payload_redaction_proof_recorded == false
  and .payload_redaction_proof_id_recorded == false
  and .redaction_policy_recorded == false
  and .redacted_summary_hash_recorded == false
  and .raw_payload_hash_recorded == false
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .approved_redaction_proof_count == 0
  and .reviewed_redaction_proof_count == 0
  and .blocked_redaction_fixture_count == 6
  and .raw_secret_marker_after_redaction_allowed == false
  and .unredacted_path_after_redaction_allowed == false
  and .channel_recipient_after_redaction_allowed == false
  and .public_artifact_path_after_redaction_allowed == false
  and .redacted_summary_equals_raw_payload_allowed == false
  and .payload_review_persisted == false
  and .payload_redaction_proof_persisted == false
  and .receipt_persistence_enabled == false
  and .receipt_persisted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .required_payload_redaction_proof_field_count == 14
  and .recorded_payload_redaction_proof_field_count == 0
  and (.denied_redaction_proof_fixtures | length) == 6
  and (.denied_redaction_proof_fixtures | all(.proof_accepted == false and .persistence_allowed == false and .activation_allowed == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction proof gate passed"
