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

MATERIALIZATION_GATE_JSON="$(
  capture_json_report \
    "hepta-core-activation-evidence-receipt-materialization-dry-run-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-evidence-receipt-materialization-dry-run-gate.sh
)"

SINK_WRITE_PREVIEW_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview" \
    scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh
)"

EXECUTION_DENIAL_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix" \
    scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix.sh
)"

materialization_gate_report_sha256="$(sha256_text "$MATERIALIZATION_GATE_JSON")"
sink_write_preview_report_sha256="$(sha256_text "$SINK_WRITE_PREVIEW_JSON")"
execution_denial_report_sha256="$(sha256_text "$EXECUTION_DENIAL_JSON")"
persistence_denial_index_hash_sha256="$(sha256_text "hepta-core-activation-evidence-receipt-filesystem-persistence-denial:index:$materialization_gate_report_sha256:$sink_write_preview_report_sha256:$execution_denial_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
persistence_denial_no_write_hash_sha256="$(sha256_text "hepta-core-activation-evidence-receipt-filesystem-persistence-denial:no-write:$materialization_gate_report_sha256:$sink_write_preview_report_sha256:$execution_denial_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
persistence_denial_redaction_hash_sha256="$(sha256_text "hepta-core-activation-evidence-receipt-filesystem-persistence-denial:redaction:$materialization_gate_report_sha256:$sink_write_preview_report_sha256:$execution_denial_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson materialization "$MATERIALIZATION_GATE_JSON" \
  --argjson preview "$SINK_WRITE_PREVIEW_JSON" \
  --argjson denial "$EXECUTION_DENIAL_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $materialization.runtime == "hepta"
    and $materialization.status == "ready"
    and $materialization.gate == "hepta_core_activation_evidence_receipt_materialization_dry_run_gate"
    and $materialization.receipt_materialization_dry_run_ready == true
    and $materialization.required_source_count == 5
    and $materialization.ready_source_count == 5
    and $materialization.activation_blocking_source_count == 5
    and $materialization.required_materialization_field_count == 20
    and $materialization.recorded_materialization_field_count == 0
    and $materialization.planned_materialization_field_count == 0
    and $materialization.receipt_materialized == false
    and $materialization.receipt_persisted == false
    and $materialization.filesystem_persistence_allowed == false
    and $materialization.filesystem_persistence_execution_performed == false
    and $materialization.output_path_selected == false
    and $materialization.workspace_write_performed == false
    and $materialization.activation_allowed == false
    and ($materialization.side_effects | to_entries | all(.value == false))
    and $preview.product == "Hepta"
    and $preview.status == "ready"
    and $preview.filesystem_sink_write_preview_id == "upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview"
    and $preview.preview_status.source_filesystem_output_path_evidence_binding_ready == true
    and $preview.preview_status.required_preview_fixture_count == 3
    and $preview.preview_status.preview_fixture_count == 3
    and $preview.preview_status.allowed_output_path_entry_count == 3
    and $preview.preview_status.previewed_output_path_count == 3
    and $preview.preview_status.deterministic_payload_hash_count == 3
    and $preview.preview_status.redacted_output_path_preview_count == 3
    and $preview.preview_status.fresh_live_evidence_bound_fixture_count == 3
    and $preview.preview_status.active_binary_sha_bound_fixture_count == 3
    and $preview.preview_status.trusted_source_bound_fixture_count == 3
    and $preview.preview_status.operator_approval_bound_fixture_count == 3
    and $preview.preview_status.blocked_preview_fixture_count == 3
    and $preview.preview_status.allowed_preview_fixture_count == 0
    and $preview.preview_status.filesystem_persistence_allowed_count == 0
    and $preview.preview_status.workspace_write_performed_count == 0
    and $preview.preview_status.evidence_receipt_persisted_count == 0
    and $preview.preview_status.sink_write_preview_ready == true
    and $preview.preview_status.activation_blocked_by_sink_write_preview == true
    and $preview.preview_status.activation_allowed_by_sink_write_preview == false
    and $preview.preview_status.active_wiring_allowed == false
    and ($preview.preview_fixtures | length) == 3
    and ($preview.side_effects | to_entries | all(.value == false))
    and $denial.product == "Hepta"
    and $denial.status == "ready"
    and $denial.filesystem_persistence_execution_denial_matrix_id == "upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix"
    and $denial.denial_status.source_filesystem_sink_write_preview_ready == true
    and $denial.denial_status.required_denial_fixture_count == 4
    and $denial.denial_status.denial_fixture_count == 4
    and $denial.denial_status.source_preview_fixture_count == 3
    and $denial.denial_status.execution_requested_fixture_count == 4
    and $denial.denial_status.future_persistence_approval_slot_count == 4
    and $denial.denial_status.explicit_persistence_approval_id_present_count == 3
    and $denial.denial_status.explicit_persistence_approval_id_missing_count == 1
    and $denial.denial_status.stale_or_missing_fresh_evidence_fixture_count == 1
    and $denial.denial_status.active_binary_sha_bound_fixture_count == 4
    and $denial.denial_status.trusted_source_bound_fixture_count == 4
    and $denial.denial_status.operator_approval_bound_fixture_count == 3
    and $denial.denial_status.workspace_path_attempt_fixture_count == 1
    and $denial.denial_status.public_claim_attempt_fixture_count == 1
    and $denial.denial_status.release_artifact_write_attempt_fixture_count == 1
    and $denial.denial_status.blocked_execution_fixture_count == 4
    and $denial.denial_status.allowed_execution_fixture_count == 0
    and $denial.denial_status.filesystem_persistence_allowed_count == 0
    and $denial.denial_status.filesystem_persistence_execution_performed_count == 0
    and $denial.denial_status.workspace_write_performed_count == 0
    and $denial.denial_status.evidence_receipt_persisted_count == 0
    and $denial.denial_status.execution_denial_matrix_ready == true
    and $denial.denial_status.activation_blocked_by_execution_denial_matrix == true
    and $denial.denial_status.activation_allowed_by_execution_denial_matrix == false
    and $denial.denial_status.active_wiring_allowed == false
    and ($denial.denial_fixtures | length) == 4
    and ($denial.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_evidence_receipt_filesystem_persistence_denial_gate" \
  --arg materialization_gate_report_sha256 "$materialization_gate_report_sha256" \
  --arg sink_write_preview_report_sha256 "$sink_write_preview_report_sha256" \
  --arg execution_denial_report_sha256 "$execution_denial_report_sha256" \
  --arg persistence_denial_index_hash_sha256 "$persistence_denial_index_hash_sha256" \
  --arg persistence_denial_no_write_hash_sha256 "$persistence_denial_no_write_hash_sha256" \
  --arg persistence_denial_redaction_hash_sha256 "$persistence_denial_redaction_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson materialization "$MATERIALIZATION_GATE_JSON" \
  --argjson preview "$SINK_WRITE_PREVIEW_JSON" \
  --argjson denial "$EXECUTION_DENIAL_JSON" \
  '
    ([
      "filesystem_persistence_execution_request_id",
      "receipt_id",
      "materialization_plan_id",
      "future_persistence_approval_id",
      "redacted_payload_hash",
      "redacted_output_path",
      "output_path_root_id",
      "output_path_evidence_binding_id",
      "fresh_live_evidence_id",
      "active_binary_sha256",
      "operator_approval_id",
      "trusted_source_binding_id",
      "source_materialization_gate_report_sha256",
      "source_sink_write_preview_report_sha256",
      "source_execution_denial_report_sha256",
      "no_secret_payload_review_id",
      "workspace_path_denial_id",
      "public_artifact_denial_id",
      "rollback_plan_id",
      "public_claim_and_artifact_decision"
    ]) as $persistence_fields
    | ([
      "materialization-dry-run-boundary",
      "sink-write-preview-boundary",
      "execution-denial-matrix-boundary",
      "future-persistence-approval-slot-boundary",
      "workspace-path-denial-boundary",
      "public-artifact-denial-boundary",
      "activation-side-effect-boundary"
    ]) as $readiness_families
    | ([
      "filesystem_persistence_approval_not_recorded",
      "explicit_persistence_approval_id_missing_for_one_fixture",
      "stale_or_missing_fresh_evidence_fixture_present",
      "workspace_path_execution_attempt_denied",
      "public_artifact_execution_attempt_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "filesystem_persistence_execution_denied",
      "workspace_write_denied",
      "evidence_receipt_persistence_denied",
      "receipt_materialization_execution_denied",
      "long_soak_execution_denied",
      "operator_approval_authority_not_accepted",
      "activation_request_not_recorded",
      "install_restart_active_binary_mutation_denied",
      "provider_model_invocation_denied",
      "channel_delivery_denied",
      "upstream_fetch_merge_denied"
    ]) as $denied
    | ([
      {id:"core-materialization-dry-run", ready:true, blocked:true, source_gate:$materialization.gate, source_report_sha256:$materialization_gate_report_sha256},
      {id:"sink-write-preview", ready:true, blocked:true, source_gate:$preview.filesystem_sink_write_preview_gate, source_report_sha256:$sink_write_preview_report_sha256},
      {id:"persistence-execution-denial-matrix", ready:true, blocked:true, source_gate:$denial.filesystem_persistence_execution_denial_matrix_gate, source_report_sha256:$execution_denial_report_sha256},
      {id:"activation-side-effect-boundary", ready:true, blocked:true, denied_action_count:($denied | length)}
    ]) as $source_families
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      filesystem_persistence_denial_schema_version:"hepta_core_activation_evidence_receipt_filesystem_persistence_denial_v1",
      filesystem_persistence_denial_gate_ready:true,
      persistence_mode:"preview_and_execution_denial_only_no_write",
      persistence_decision:"blocked_until_explicit_filesystem_persistence_approval_fresh_evidence_and_operator_approval_exist",
      source_materialization_gate:$materialization.gate,
      source_sink_write_preview_gate:$preview.filesystem_sink_write_preview_gate,
      source_execution_denial_matrix_gate:$denial.filesystem_persistence_execution_denial_matrix_gate,
      source_materialization_gate_report_sha256:$materialization_gate_report_sha256,
      source_sink_write_preview_report_sha256:$sink_write_preview_report_sha256,
      source_execution_denial_report_sha256:$execution_denial_report_sha256,
      source_report_hashes:[
        $materialization_gate_report_sha256,
        $sink_write_preview_report_sha256,
        $execution_denial_report_sha256
      ],
      persistence_denial_index_hash_sha256:$persistence_denial_index_hash_sha256,
      persistence_denial_no_write_hash_sha256:$persistence_denial_no_write_hash_sha256,
      persistence_denial_redaction_hash_sha256:$persistence_denial_redaction_hash_sha256,
      required_source_count:3,
      ready_source_count:3,
      activation_blocking_source_count:3,
      minimum_required_long_soak_samples:$min_long_soak_samples,
      source_required_materialization_field_count:$materialization.required_materialization_field_count,
      source_recorded_materialization_field_count:$materialization.recorded_materialization_field_count,
      required_persistence_execution_field_count:($persistence_fields | length),
      recorded_persistence_execution_field_count:0,
      required_preview_fixture_count:$preview.preview_status.required_preview_fixture_count,
      preview_fixture_count:$preview.preview_status.preview_fixture_count,
      previewed_output_path_count:$preview.preview_status.previewed_output_path_count,
      deterministic_payload_hash_count:$preview.preview_status.deterministic_payload_hash_count,
      redacted_output_path_preview_count:$preview.preview_status.redacted_output_path_preview_count,
      blocked_preview_fixture_count:$preview.preview_status.blocked_preview_fixture_count,
      allowed_preview_fixture_count:$preview.preview_status.allowed_preview_fixture_count,
      required_denial_fixture_count:$denial.denial_status.required_denial_fixture_count,
      denial_fixture_count:$denial.denial_status.denial_fixture_count,
      execution_requested_fixture_count:$denial.denial_status.execution_requested_fixture_count,
      future_persistence_approval_slot_count:$denial.denial_status.future_persistence_approval_slot_count,
      explicit_persistence_approval_id_present_count:$denial.denial_status.explicit_persistence_approval_id_present_count,
      explicit_persistence_approval_id_missing_count:$denial.denial_status.explicit_persistence_approval_id_missing_count,
      stale_or_missing_fresh_evidence_fixture_count:$denial.denial_status.stale_or_missing_fresh_evidence_fixture_count,
      workspace_path_attempt_fixture_count:$denial.denial_status.workspace_path_attempt_fixture_count,
      public_claim_attempt_fixture_count:$denial.denial_status.public_claim_attempt_fixture_count,
      release_artifact_write_attempt_fixture_count:$denial.denial_status.release_artifact_write_attempt_fixture_count,
      blocked_execution_fixture_count:$denial.denial_status.blocked_execution_fixture_count,
      allowed_execution_fixture_count:$denial.denial_status.allowed_execution_fixture_count,
      filesystem_persistence_approval_recorded:false,
      filesystem_persistence_allowed:false,
      filesystem_persistence_execution_performed:false,
      workspace_write_performed:false,
      evidence_receipt_persisted:false,
      receipt_materialization_execution_performed:false,
      output_path_selected:false,
      output_path_previewed:true,
      output_path_write_authority_granted:false,
      preview_payload_hashes_are_write_authority:false,
      activation_allowed:false,
      live_mutation_execution_ready:false,
      active_wiring_allowed:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      release_artifact_write_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      install_restart_allowed:false,
      upstream_fetch_merge_allowed:false,
      required_persistence_execution_fields:$persistence_fields,
      filesystem_persistence_readiness_families:$readiness_families,
      source_readiness_families:$source_families,
      denied_by_filesystem_persistence_denial_gate:$denied,
      denied_by_filesystem_persistence_denial_gate_count:($denied | length),
      side_effects:{
        workspace_written:false,
        filesystem_written:false,
        memory_store_mutated:false,
        capability_registry_mutated:false,
        plugin_registry_mutated:false,
        provider_invoked:false,
        model_invoked:false,
        channel_send_performed:false,
        telegram_send_performed:false,
        gateway_event_enqueued:false,
        gateway_rpc_performed:false,
        external_network_read:false,
        external_send_performed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        public_release_published:false,
        public_ga_claimed:false,
        install_executed:false,
        launchd_mutated:false,
        service_restarted:false,
        active_binary_mutated:false,
        rollback_executed:false,
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        credential_read:false,
        secret_value_read:false,
        long_soak_executed:false,
        approval_packet_persisted:false,
        ledger_record_persisted:false,
        receipt_materialization_plan_recorded:false,
        receipt_materialized:false,
        receipt_persisted:false,
        output_path_selected:false,
        output_path_bound:false,
        output_path_previewed:false,
        filesystem_persistence_executed:false,
        trusted_record_persisted:false
      }
    }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_evidence_receipt_filesystem_persistence_denial_gate"
  and .filesystem_persistence_denial_gate_ready == true
  and .persistence_mode == "preview_and_execution_denial_only_no_write"
  and .required_source_count == 3
  and .ready_source_count == 3
  and .activation_blocking_source_count == 3
  and .minimum_required_long_soak_samples >= 24
  and .source_required_materialization_field_count == 20
  and .source_recorded_materialization_field_count == 0
  and .required_persistence_execution_field_count == 20
  and .recorded_persistence_execution_field_count == 0
  and .required_preview_fixture_count == 3
  and .preview_fixture_count == 3
  and .previewed_output_path_count == 3
  and .deterministic_payload_hash_count == 3
  and .redacted_output_path_preview_count == 3
  and .blocked_preview_fixture_count == 3
  and .allowed_preview_fixture_count == 0
  and .required_denial_fixture_count == 4
  and .denial_fixture_count == 4
  and .execution_requested_fixture_count == 4
  and .future_persistence_approval_slot_count == 4
  and .explicit_persistence_approval_id_present_count == 3
  and .explicit_persistence_approval_id_missing_count == 1
  and .stale_or_missing_fresh_evidence_fixture_count == 1
  and .workspace_path_attempt_fixture_count == 1
  and .public_claim_attempt_fixture_count == 1
  and .release_artifact_write_attempt_fixture_count == 1
  and .blocked_execution_fixture_count == 4
  and .allowed_execution_fixture_count == 0
  and .filesystem_persistence_approval_recorded == false
  and .filesystem_persistence_allowed == false
  and .filesystem_persistence_execution_performed == false
  and .workspace_write_performed == false
  and .evidence_receipt_persisted == false
  and .receipt_materialization_execution_performed == false
  and .output_path_selected == false
  and .output_path_previewed == true
  and .output_path_write_authority_granted == false
  and .preview_payload_hashes_are_write_authority == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .active_wiring_allowed == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .provider_model_invocation_allowed == false
  and .channel_delivery_allowed == false
  and .install_restart_allowed == false
  and .upstream_fetch_merge_allowed == false
  and (.required_persistence_execution_fields | length) == 20
  and (.filesystem_persistence_readiness_families | length) == 7
  and (.source_readiness_families | length) == 4
  and (.source_readiness_families | all(.ready == true and .blocked == true))
  and .denied_by_filesystem_persistence_denial_gate_count == 18
  and (.denied_by_filesystem_persistence_denial_gate | length) == 18
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
