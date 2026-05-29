#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

ACCEPTANCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-gate" \
    scripts/hepta-live-mutation-pre-activation-soak-evidence-persistence-payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-gate.sh
)"

acceptance_report_sha256="$(
  printf '%s' "$ACCEPTANCE_JSON" | shasum -a 256 | awk '{print $1}'
)"

scoreboard_family_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard:families:$acceptance_report_sha256"
)"
scoreboard_denial_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard:denials:$acceptance_report_sha256"
)"
scoreboard_side_effect_hash_sha256="$(
  sha256_text "payload-redaction-acceptance-receipt-filesystem-persistence-ledger-persistence-rehearsal-receipt-review-acceptance-scoreboard:side-effects:$acceptance_report_sha256"
)"

jq -n -e \
  --argjson acceptance "$ACCEPTANCE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $acceptance.runtime == "hepta"
    and $acceptance.status == "ready"
    and $acceptance.gate == "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_gate"
    and $acceptance.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_ready == true
    and $acceptance.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_ready == true
    and $acceptance.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_ready == true
    and $acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_report_sha256 != ""
    and $acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_report_sha256 != ""
    and $acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_report_sha256 != ""
    and $acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256 != ""
    and $acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256 != ""
    and $acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256 != ""
    and $acceptance.source_pre_activation_soak_report_sha256 != ""
    and $acceptance.source_persistence_denial_report_sha256 != ""
    and $acceptance.minimum_required_samples >= 24
    and $acceptance.required_rehearsal_receipt_review_acceptance_field_count == 20
    and $acceptance.rehearsal_receipt_review_acceptance_field_count == 20
    and $acceptance.recorded_rehearsal_receipt_review_acceptance_field_count == 0
    and $acceptance.redacted_or_hashed_rehearsal_receipt_review_acceptance_field_count == 18
    and $acceptance.required_rehearsal_receipt_review_acceptance_fixture_count == 4
    and $acceptance.rehearsal_receipt_review_acceptance_fixture_count == 4
    and $acceptance.blocked_rehearsal_receipt_review_acceptance_fixture_count == 4
    and $acceptance.allowed_rehearsal_receipt_review_acceptance_fixture_count == 0
    and $acceptance.rehearsal_receipt_review_acceptance_hash_count == 4
    and $acceptance.rehearsal_receipt_review_acceptance_requested_count == 4
    and $acceptance.rehearsal_receipt_review_acceptance_performed_count == 0
    and $acceptance.rehearsal_receipt_review_acceptance_recorded_count == 0
    and $acceptance.rehearsal_receipt_review_acceptance_persisted_count == 0
    and $acceptance.rehearsal_receipt_review_acceptance_materialized_count == 0
    and $acceptance.rehearsal_receipt_review_acceptance_filesystem_written_count == 0
    and $acceptance.review_acceptance_allowed_count == 0
    and $acceptance.accepted_rehearsal_receipt_review_count == 0
    and $acceptance.review_acceptance_policy_satisfied_count == 0
    and $acceptance.rehearsal_receipt_review_performed_count == 0
    and $acceptance.rehearsal_receipt_review_recorded_count == 0
    and $acceptance.rehearsal_receipt_review_persisted_count == 0
    and $acceptance.rehearsal_receipt_review_materialized_count == 0
    and $acceptance.rehearsal_receipt_review_filesystem_written_count == 0
    and $acceptance.ledger_persistence_allowed == false
    and $acceptance.ledger_persistence_execution_performed == false
    and $acceptance.ledger_recorded == false
    and $acceptance.ledger_persisted == false
    and $acceptance.ledger_materialized == false
    and $acceptance.ledger_filesystem_written == false
    and $acceptance.receipt_persistence_allowed_count == 0
    and $acceptance.receipt_persistence_execution_performed_count == 0
    and $acceptance.receipt_materialized_count == 0
    and $acceptance.receipt_persisted_count == 0
    and $acceptance.filesystem_persistence_allowed == false
    and $acceptance.filesystem_persistence_execution_performed == false
    and $acceptance.filesystem_write_performed == false
    and $acceptance.workspace_write_performed == false
    and $acceptance.command_invocation_requested_count == 0
    and $acceptance.command_invocation_performed_count == 0
    and $acceptance.command_execution_requested_count == 0
    and $acceptance.command_execution_performed_count == 0
    and $acceptance.materialization_execution_requested_count == 0
    and $acceptance.materialization_execution_performed_count == 0
    and $acceptance.selected_output_path_count == 0
    and $acceptance.recorded_output_path_count == 0
    and $acceptance.recorded_path_binding_count == 0
    and $acceptance.active_binary_sha_bound_count == 0
    and $acceptance.trusted_source_bound_count == 0
    and $acceptance.operator_scope_bound_count == 0
    and $acceptance.accepted_redaction_proof_bound_count == 0
    and $acceptance.rollback_rehearsal_evidence_recorded_count == 0
    and $acceptance.public_claim_allowed == false
    and $acceptance.release_artifact_write_allowed == false
    and $acceptance.raw_payload_plaintext_recorded == false
    and $acceptance.raw_payload_plaintext_persisted == false
    and $acceptance.raw_payload_inspected == false
    and $acceptance.live_secret_scan_performed == false
    and $acceptance.receipt_persistence_enabled == false
    and $acceptance.operator_approval_recorded == false
    and $acceptance.fresh_pre_activation_soak_evidence_recorded == false
    and $acceptance.accepted_redaction_proof_recorded == false
    and $acceptance.rollback_rehearsal_evidence_recorded == false
    and $acceptance.public_artifact_decision_recorded == false
    and $acceptance.activation_blocked_by_rehearsal_receipt_review_acceptance == true
    and $acceptance.activation_allowed_by_rehearsal_receipt_review_acceptance == false
    and $acceptance.activation_allowed == false
    and $acceptance.live_mutation_execution_ready == false
    and ($acceptance.required_rehearsal_receipt_review_acceptance_fields | length) == 20
    and ($acceptance.rehearsal_receipt_review_acceptance_fixtures | length) == 4
    and ($acceptance.rehearsal_receipt_review_acceptance_fixtures | all(.acceptance_status == "blocked" and .rehearsal_receipt_review_acceptance_requested == true and .rehearsal_receipt_review_acceptance_performed == false and .rehearsal_receipt_review_acceptance_recorded == false and .rehearsal_receipt_review_accepted == false and .rehearsal_receipt_review_acceptance_persisted == false and .rehearsal_receipt_review_acceptance_materialized == false and .rehearsal_receipt_review_acceptance_filesystem_written == false and .activation_allowed == false and .deterministic_acceptance_sha256 != ""))
    and ($acceptance.denied_by_rehearsal_receipt_review_acceptance | length) == 20
    and ($acceptance.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_pre_activation_soak_evidence_persistence_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_gate" \
  --arg acceptance_report_sha256 "$acceptance_report_sha256" \
  --arg scoreboard_family_hash_sha256 "$scoreboard_family_hash_sha256" \
  --arg scoreboard_denial_hash_sha256 "$scoreboard_denial_hash_sha256" \
  --arg scoreboard_side_effect_hash_sha256 "$scoreboard_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson acceptance "$ACCEPTANCE_JSON" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    rehearsal_receipt_review_acceptance_scoreboard_schema_version:"payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_v1",
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_gate:$acceptance.gate,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_ready:$acceptance.payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_ready,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_report_sha256:$acceptance_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_contract_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_denial_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_shape_approval_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_dry_run_ledger_report_sha256,
    source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256:$acceptance.source_payload_redaction_acceptance_receipt_filesystem_persistence_execution_denial_matrix_report_sha256,
    source_pre_activation_soak_report_sha256:$acceptance.source_pre_activation_soak_report_sha256,
    source_persistence_denial_report_sha256:$acceptance.source_persistence_denial_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_ready:true,
    payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_ready:true,
    required_scoreboard_family_count:10,
    ready_scoreboard_family_count:10,
    activation_blocking_scoreboard_family_count:10,
    required_rehearsal_receipt_review_acceptance_field_count:$acceptance.required_rehearsal_receipt_review_acceptance_field_count,
    rehearsal_receipt_review_acceptance_field_count:$acceptance.rehearsal_receipt_review_acceptance_field_count,
    recorded_rehearsal_receipt_review_acceptance_field_count:$acceptance.recorded_rehearsal_receipt_review_acceptance_field_count,
    redacted_or_hashed_rehearsal_receipt_review_acceptance_field_count:$acceptance.redacted_or_hashed_rehearsal_receipt_review_acceptance_field_count,
    required_rehearsal_receipt_review_acceptance_fixture_count:$acceptance.required_rehearsal_receipt_review_acceptance_fixture_count,
    rehearsal_receipt_review_acceptance_fixture_count:$acceptance.rehearsal_receipt_review_acceptance_fixture_count,
    blocked_rehearsal_receipt_review_acceptance_fixture_count:$acceptance.blocked_rehearsal_receipt_review_acceptance_fixture_count,
    allowed_rehearsal_receipt_review_acceptance_fixture_count:$acceptance.allowed_rehearsal_receipt_review_acceptance_fixture_count,
    accepted_rehearsal_receipt_review_count:$acceptance.accepted_rehearsal_receipt_review_count,
    review_acceptance_policy_satisfied_count:$acceptance.review_acceptance_policy_satisfied_count,
    review_acceptance_allowed_count:$acceptance.review_acceptance_allowed_count,
    rehearsal_receipt_review_acceptance_requested_count:$acceptance.rehearsal_receipt_review_acceptance_requested_count,
    rehearsal_receipt_review_acceptance_performed_count:$acceptance.rehearsal_receipt_review_acceptance_performed_count,
    rehearsal_receipt_review_acceptance_recorded_count:$acceptance.rehearsal_receipt_review_acceptance_recorded_count,
    rehearsal_receipt_review_acceptance_persisted_count:$acceptance.rehearsal_receipt_review_acceptance_persisted_count,
    rehearsal_receipt_review_acceptance_materialized_count:$acceptance.rehearsal_receipt_review_acceptance_materialized_count,
    rehearsal_receipt_review_acceptance_filesystem_written_count:$acceptance.rehearsal_receipt_review_acceptance_filesystem_written_count,
    rehearsal_receipt_review_performed_count:$acceptance.rehearsal_receipt_review_performed_count,
    rehearsal_receipt_review_recorded_count:$acceptance.rehearsal_receipt_review_recorded_count,
    rehearsal_receipt_review_persisted_count:$acceptance.rehearsal_receipt_review_persisted_count,
    rehearsal_receipt_review_materialized_count:$acceptance.rehearsal_receipt_review_materialized_count,
    rehearsal_receipt_review_filesystem_written_count:$acceptance.rehearsal_receipt_review_filesystem_written_count,
    ledger_persistence_allowed:false,
    ledger_persistence_execution_performed:false,
    ledger_recorded:false,
    ledger_persisted:false,
    ledger_materialized:false,
    ledger_filesystem_written:false,
    receipt_persistence_allowed_count:0,
    receipt_persistence_execution_performed_count:0,
    receipt_materialized_count:0,
    receipt_persisted_count:0,
    filesystem_persistence_allowed:false,
    filesystem_persistence_execution_performed:false,
    filesystem_write_performed:false,
    workspace_write_performed:false,
    command_invocation_requested_count:0,
    command_invocation_performed_count:0,
    command_execution_requested_count:0,
    command_execution_performed_count:0,
    materialization_execution_requested_count:0,
    materialization_execution_performed_count:0,
    selected_output_path_count:0,
    recorded_output_path_count:0,
    recorded_path_binding_count:0,
    active_binary_sha_bound_count:0,
    trusted_source_bound_count:0,
    operator_scope_bound_count:0,
    accepted_redaction_proof_bound_count:0,
    operator_approval_recorded:false,
    fresh_pre_activation_soak_evidence_recorded:false,
    accepted_redaction_proof_recorded:false,
    rollback_rehearsal_evidence_recorded:false,
    public_artifact_decision_recorded:false,
    public_claim_allowed:false,
    release_artifact_write_allowed:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    raw_payload_inspected:false,
    live_secret_scan_performed:false,
    receipt_persistence_enabled:false,
    scoreboard_family_hash_sha256:$scoreboard_family_hash_sha256,
    scoreboard_denial_hash_sha256:$scoreboard_denial_hash_sha256,
    scoreboard_side_effect_hash_sha256:$scoreboard_side_effect_hash_sha256,
    activation_blocked_by_rehearsal_receipt_review_acceptance_scoreboard:true,
    activation_allowed_by_rehearsal_receipt_review_acceptance_scoreboard:false,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    scoreboard_denial_reason:"review acceptance families are ready as report-only gates, but no real acceptance, operator approval, fresh live evidence, redaction proof, rollback evidence, output path, ledger, receipt, review, filesystem persistence, public artifact decision, or activation exists",
    scoreboard_families:[
      "source-rehearsal-receipt-review-gate",
      "source-rehearsal-receipt-contract-gate",
      "source-rehearsal-denial-gate",
      "source-ledger-shape-approval-gate",
      "source-dry-run-ledger-gate",
      "source-execution-denial-matrix-gate",
      "source-pre-activation-soak-gate",
      "source-persistence-denial-gate",
      "review-acceptance-fixture-family",
      "review-acceptance-side-effect-boundary"
    ],
    scoreboard_entries:[
      {
        id:"acceptance-schema-and-fixtures",
        ready:true,
        blocked:true,
        required_count:4,
        accepted_count:0,
        reason:"all acceptance fixtures are deterministic and blocked"
      },
      {
        id:"operator-and-live-evidence",
        ready:true,
        blocked:true,
        operator_approval_recorded:false,
        fresh_pre_activation_soak_evidence_recorded:false,
        reason:"operator approval and fresh live evidence are absent"
      },
      {
        id:"redaction-and-rollback-binding",
        ready:true,
        blocked:true,
        accepted_redaction_proof_recorded:false,
        rollback_rehearsal_evidence_recorded:false,
        reason:"accepted redaction proof and rollback rehearsal evidence are absent"
      },
      {
        id:"persistence-and-output-path",
        ready:true,
        blocked:true,
        ledger_persistence_allowed:false,
        filesystem_persistence_allowed:false,
        selected_output_path_count:0,
        reason:"ledger, receipt, review, acceptance, and filesystem persistence remain disabled"
      },
      {
        id:"public-artifact-and-live-mutation",
        ready:true,
        blocked:true,
        public_claim_allowed:false,
        release_artifact_write_allowed:false,
        activation_allowed:false,
        live_mutation_execution_ready:false,
        reason:"public claims, release artifact writes, activation, and live mutation remain denied"
      }
    ],
    denied_by_rehearsal_receipt_review_acceptance_scoreboard:[
      "review_acceptance_recording_denied",
      "review_acceptance_materialization_denied",
      "review_acceptance_persistence_denied",
      "review_acceptance_execution_denied",
      "operator_approval_missing",
      "fresh_pre_activation_soak_evidence_missing_or_stale",
      "accepted_redaction_proof_missing",
      "rollback_rehearsal_evidence_missing",
      "output_path_selection_missing",
      "ledger_persistence_execution_denied",
      "receipt_persistence_execution_denied",
      "review_persistence_execution_denied",
      "acceptance_persistence_execution_denied",
      "filesystem_persistence_execution_denied",
      "filesystem_write_denied",
      "workspace_write_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "public_artifact_decision_missing",
      "live_mutation_execution_denied"
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
      command_invocation_performed:false,
      command_execution_performed:false,
      receipt_persistence_execution_performed:false,
      materialization_execution_performed:false,
      filesystem_persistence_execution_performed:false,
      ledger_persistence_execution_performed:false,
      rehearsal_receipt_review_acceptance_performed:false,
      rehearsal_receipt_review_acceptance_recorded:false,
      rehearsal_receipt_review_acceptance_persisted:false,
      rehearsal_receipt_review_acceptance_materialized:false,
      rehearsal_receipt_review_acceptance_filesystem_written:false,
      rehearsal_receipt_review_accepted:false,
      rehearsal_receipt_review_performed:false,
      rehearsal_receipt_review_recorded:false,
      rehearsal_receipt_review_persisted:false,
      rehearsal_receipt_review_materialized:false,
      rehearsal_receipt_review_filesystem_written:false,
      rehearsal_receipt_materialized:false,
      rehearsal_receipt_persisted:false,
      ledger_recorded:false,
      ledger_persisted:false,
      ledger_materialized:false,
      ledger_filesystem_written:false,
      filesystem_written:false,
      workspace_write_performed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      rollback_rehearsal_evidence_recorded:false,
      receipt_materialized:false,
      receipt_persisted:false,
      output_path_selected:false,
      output_path_binding_selected:false,
      pre_activation_soak_evidence_persisted:false,
      approval_packet_persisted:false,
      operator_scope_binding_persisted:false,
      payload_review_persisted:false,
      payload_redaction_proof_persisted:false,
      payload_redaction_acceptance_matrix_persisted:false,
      payload_redaction_acceptance_receipt_persisted:false,
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
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_scoreboard_ready == true
  and .payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_ready == true
  and .source_payload_redaction_acceptance_receipt_filesystem_persistence_ledger_persistence_rehearsal_receipt_review_acceptance_report_sha256 != ""
  and .minimum_required_samples >= 24
  and .required_scoreboard_family_count == 10
  and .ready_scoreboard_family_count == 10
  and .activation_blocking_scoreboard_family_count == 10
  and .required_rehearsal_receipt_review_acceptance_field_count == 20
  and .recorded_rehearsal_receipt_review_acceptance_field_count == 0
  and .rehearsal_receipt_review_acceptance_fixture_count == 4
  and .blocked_rehearsal_receipt_review_acceptance_fixture_count == 4
  and .allowed_rehearsal_receipt_review_acceptance_fixture_count == 0
  and .accepted_rehearsal_receipt_review_count == 0
  and .review_acceptance_policy_satisfied_count == 0
  and .review_acceptance_allowed_count == 0
  and .rehearsal_receipt_review_acceptance_performed_count == 0
  and .rehearsal_receipt_review_acceptance_recorded_count == 0
  and .rehearsal_receipt_review_acceptance_persisted_count == 0
  and .rehearsal_receipt_review_acceptance_materialized_count == 0
  and .rehearsal_receipt_review_acceptance_filesystem_written_count == 0
  and .ledger_persistence_allowed == false
  and .ledger_persistence_execution_performed == false
  and .ledger_recorded == false
  and .ledger_persisted == false
  and .ledger_materialized == false
  and .ledger_filesystem_written == false
  and .receipt_persistence_execution_performed_count == 0
  and .receipt_materialized_count == 0
  and .receipt_persisted_count == 0
  and .filesystem_persistence_allowed == false
  and .filesystem_persistence_execution_performed == false
  and .filesystem_write_performed == false
  and .workspace_write_performed == false
  and .command_invocation_requested_count == 0
  and .command_invocation_performed_count == 0
  and .command_execution_requested_count == 0
  and .command_execution_performed_count == 0
  and .materialization_execution_requested_count == 0
  and .materialization_execution_performed_count == 0
  and .selected_output_path_count == 0
  and .recorded_output_path_count == 0
  and .recorded_path_binding_count == 0
  and .operator_approval_recorded == false
  and .fresh_pre_activation_soak_evidence_recorded == false
  and .accepted_redaction_proof_recorded == false
  and .rollback_rehearsal_evidence_recorded == false
  and .public_artifact_decision_recorded == false
  and .public_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and .raw_payload_inspected == false
  and .live_secret_scan_performed == false
  and .receipt_persistence_enabled == false
  and .scoreboard_family_hash_sha256 != ""
  and .scoreboard_denial_hash_sha256 != ""
  and .scoreboard_side_effect_hash_sha256 != ""
  and .activation_blocked_by_rehearsal_receipt_review_acceptance_scoreboard == true
  and .activation_allowed_by_rehearsal_receipt_review_acceptance_scoreboard == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and (.scoreboard_families | length) == 10
  and (.scoreboard_entries | length) == 5
  and (.scoreboard_entries | all(.ready == true and .blocked == true))
  and (.denied_by_rehearsal_receipt_review_acceptance_scoreboard | length) == 20
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard gate passed"
