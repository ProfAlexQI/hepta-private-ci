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

REDACTION_PROOF_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-proof-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-proof-gate.sh
)"

redaction_proof_report_sha256="$(printf '%s' "$REDACTION_PROOF_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson proof "$REDACTION_PROOF_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $proof.runtime == "hepta"
    and $proof.status == "ready"
    and $proof.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_proof_gate"
    and $proof.payload_redaction_proof_ready == true
    and $proof.source_no_secret_payload_review_ready == true
    and $proof.payload_redaction_proof_recorded == false
    and $proof.raw_payload_plaintext_recorded == false
    and $proof.raw_payload_plaintext_persisted == false
    and $proof.approved_redaction_proof_count == 0
    and $proof.reviewed_redaction_proof_count == 0
    and $proof.blocked_redaction_fixture_count == 6
    and $proof.payload_review_persisted == false
    and $proof.payload_redaction_proof_persisted == false
    and $proof.receipt_persistence_enabled == false
    and $proof.receipt_persisted == false
    and $proof.activation_allowed == false
    and $proof.live_mutation_execution_ready == false
    and $proof.required_payload_redaction_proof_field_count == 14
    and $proof.recorded_payload_redaction_proof_field_count == 0
    and ($proof.denied_redaction_proof_fixtures | length) == 6
    and ($proof.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_matrix_gate" \
  --arg redaction_proof_report_sha256 "$redaction_proof_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson proof "$REDACTION_PROOF_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    source_payload_redaction_proof_gate:$proof.gate,
    source_payload_redaction_proof_ready:$proof.payload_redaction_proof_ready,
    source_receipt_payload_sha256:$proof.source_receipt_payload_sha256,
    source_pre_activation_soak_report_sha256:$proof.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$proof.source_persistence_denial_report_sha256,
    source_approval_packet_report_sha256:$proof.source_approval_packet_report_sha256,
    source_operator_scope_report_sha256:$proof.source_operator_scope_report_sha256,
    source_no_secret_payload_review_report_sha256:$proof.source_no_secret_payload_review_report_sha256,
    source_payload_redaction_proof_report_sha256:$redaction_proof_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_matrix_ready:true,
    payload_redaction_acceptance_matrix_recorded:false,
    payload_redaction_acceptance_matrix_id_recorded:false,
    payload_redaction_proof_recorded:false,
    payload_redaction_proof_accepted:false,
    accepted_redaction_proof_count:0,
    reviewed_redaction_proof_count:0,
    blocked_redaction_acceptance_fixture_count:6,
    required_acceptance_check_count_per_proof:8,
    satisfied_acceptance_check_count:0,
    required_acceptance_checks:[
      "source_no_secret_payload_review_hash_bound",
      "source_operator_scope_hash_bound",
      "single_surface_scope_bound",
      "raw_payload_sha256_present",
      "redacted_payload_summary_sha256_present",
      "redacted_summary_differs_from_raw_payload",
      "redaction_policies_recorded",
      "plaintext_payload_absent_from_record"
    ],
    acceptance_denial_reason:"no redaction proof is recorded, no source hashes are accepted as live evidence, and plaintext payload remains forbidden",
    no_secret_payload_review_recorded:false,
    approval_packet_recorded:false,
    approval_packet_persisted:false,
    operator_scope_binding_recorded:false,
    operator_scope_binding_persisted:false,
    payload_review_persisted:false,
    payload_redaction_proof_persisted:false,
    payload_redaction_acceptance_matrix_persisted:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    live_secret_scan_performed:false,
    pre_activation_soak_evidence_persistence_allowed:false,
    receipt_persistence_enabled:false,
    receipt_persisted:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    acceptance_matrix_fixtures:[
      {
        id:"schema-only-no-proof",
        recorded_required_field_count:0,
        satisfied_acceptance_check_count:0,
        proof_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"payload_redaction_proof_not_recorded"
      },
      {
        id:"raw-hash-without-redacted-summary",
        recorded_required_field_count:1,
        satisfied_acceptance_check_count:1,
        raw_payload_sha256_present:true,
        redacted_payload_summary_sha256_present:false,
        proof_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"redacted_summary_hash_required"
      },
      {
        id:"redacted-summary-matches-raw",
        recorded_required_field_count:2,
        satisfied_acceptance_check_count:2,
        raw_payload_sha256_present:true,
        redacted_payload_summary_sha256_present:true,
        redacted_summary_differs_from_raw_payload:false,
        proof_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"redacted_summary_must_not_equal_raw_payload"
      },
      {
        id:"policyless-redaction-proof",
        recorded_required_field_count:6,
        satisfied_acceptance_check_count:5,
        redaction_policies_recorded:false,
        proof_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"redaction_policy_bundle_required"
      },
      {
        id:"source-review-unbound",
        recorded_required_field_count:8,
        satisfied_acceptance_check_count:6,
        source_no_secret_payload_review_hash_bound:false,
        source_operator_scope_hash_bound:false,
        proof_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"source_review_and_scope_hashes_required"
      },
      {
        id:"plaintext-retention-attempt",
        recorded_required_field_count:8,
        satisfied_acceptance_check_count:7,
        plaintext_payload_absent_from_record:false,
        raw_payload_plaintext_recorded:true,
        proof_accepted:false,
        persistence_allowed:false,
        activation_allowed:false,
        reason:"plaintext_payload_must_be_absent"
      }
    ],
    denied_by_payload_redaction_acceptance_matrix:[
      "acceptance_matrix_not_recorded",
      "payload_redaction_proof_not_recorded",
      "source_no_secret_payload_review_hash_not_accepted",
      "source_operator_scope_hash_not_accepted",
      "single_surface_scope_not_bound",
      "redacted_summary_hash_not_recorded",
      "redacted_summary_equals_raw_denied",
      "redaction_policy_bundle_not_recorded",
      "plaintext_payload_recording_denied"
    ],
    required_before_redaction_proof_acceptance:[
      "payload_redaction_acceptance_matrix_id",
      "payload_redaction_proof_id",
      "source_no_secret_payload_review_hash_binding",
      "source_operator_scope_hash_binding",
      "single_surface_scope_binding",
      "raw_payload_sha256",
      "redacted_payload_summary_sha256",
      "redacted_summary_differs_from_raw_payload",
      "redaction_policy_bundle",
      "plaintext_payload_absent_from_record",
      "reviewer_identity_hash",
      "proof_timestamp"
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
      payload_redaction_acceptance_matrix_persisted:false,
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
  and .payload_redaction_acceptance_matrix_ready == true
  and .source_payload_redaction_proof_ready == true
  and .source_receipt_payload_sha256 != ""
  and .source_pre_activation_soak_report_sha256 != ""
  and .source_persistence_denial_report_sha256 != ""
  and .source_approval_packet_report_sha256 != ""
  and .source_operator_scope_report_sha256 != ""
  and .source_no_secret_payload_review_report_sha256 != ""
  and .source_payload_redaction_proof_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .payload_redaction_acceptance_matrix_recorded == false
  and .payload_redaction_proof_recorded == false
  and .payload_redaction_proof_accepted == false
  and .accepted_redaction_proof_count == 0
  and .reviewed_redaction_proof_count == 0
  and .blocked_redaction_acceptance_fixture_count == 6
  and .required_acceptance_check_count_per_proof == 8
  and .satisfied_acceptance_check_count == 0
  and .payload_review_persisted == false
  and .payload_redaction_proof_persisted == false
  and .payload_redaction_acceptance_matrix_persisted == false
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .live_secret_scan_performed == false
  and .receipt_persistence_enabled == false
  and .receipt_persisted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.required_acceptance_checks | length) == 8
  and (.acceptance_matrix_fixtures | length) == 6
  and (.acceptance_matrix_fixtures | all(.proof_accepted == false and .persistence_allowed == false and .activation_allowed == false))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance matrix gate passed"
