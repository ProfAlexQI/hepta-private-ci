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

APPROVAL_PACKET_JSON="$(
  capture_json_report \
    "hepta-core-activation-long-soak-operator-approval-packet-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-long-soak-operator-approval-packet-gate.sh
)"

PRE_ACTIVATION_SOAK_JSON="$(
  capture_json_report \
    "hepta-live-mutation-pre-activation-soak-evidence-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      HEPTA_LIVE_MUTATION_PRE_ACTIVATION_SOAK_RUN=0 \
      scripts/hepta-live-mutation-pre-activation-soak-evidence-gate.sh
)"

ACTIVATION_LEDGER_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-ledger" \
    scripts/hepta-upstream-codex-activation-evidence-ledger.sh
)"

RECEIPT_PERSISTENCE_PACKET_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet" \
    scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh
)"

approval_packet_report_sha256="$(sha256_text "$APPROVAL_PACKET_JSON")"
pre_activation_soak_report_sha256="$(sha256_text "$PRE_ACTIVATION_SOAK_JSON")"
activation_ledger_report_sha256="$(sha256_text "$ACTIVATION_LEDGER_JSON")"
receipt_persistence_packet_report_sha256="$(sha256_text "$RECEIPT_PERSISTENCE_PACKET_JSON")"
ledger_receipt_index_hash_sha256="$(sha256_text "hepta-core-activation-fresh-long-soak-evidence-ledger-receipt:index:$approval_packet_report_sha256:$pre_activation_soak_report_sha256:$activation_ledger_report_sha256:$receipt_persistence_packet_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
ledger_receipt_redaction_hash_sha256="$(sha256_text "hepta-core-activation-fresh-long-soak-evidence-ledger-receipt:redaction:$approval_packet_report_sha256:$pre_activation_soak_report_sha256:$activation_ledger_report_sha256:$receipt_persistence_packet_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
ledger_receipt_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-fresh-long-soak-evidence-ledger-receipt:side-effects:$approval_packet_report_sha256:$pre_activation_soak_report_sha256:$activation_ledger_report_sha256:$receipt_persistence_packet_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson approval "$APPROVAL_PACKET_JSON" \
  --argjson soak "$PRE_ACTIVATION_SOAK_JSON" \
  --argjson ledger "$ACTIVATION_LEDGER_JSON" \
  --argjson receipt "$RECEIPT_PERSISTENCE_PACKET_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $approval.runtime == "hepta"
    and $approval.status == "ready"
    and $approval.gate == "hepta_core_activation_long_soak_operator_approval_packet_gate"
    and $approval.long_soak_operator_approval_packet_ready == true
    and $approval.approval_packet_decision == "blocked_until_operator_approval_and_fresh_24_sample_evidence_records_exist"
    and $approval.required_source_count == 5
    and $approval.ready_source_count == 5
    and $approval.activation_blocking_source_count == 5
    and $approval.required_evidence_count == 8
    and $approval.missing_evidence_count == 8
    and $approval.fresh_evidence_count == 0
    and $approval.required_approval_packet_field_count == 16
    and $approval.recorded_approval_packet_field_count == 0
    and $approval.operator_approved_activation_ready == false
    and $approval.activation_allowed == false
    and $approval.live_mutation_execution_ready == false
    and $approval.public_release_claim_allowed == false
    and $approval.release_artifact_write_allowed == false
    and ($approval.side_effects | to_entries | all(.value == false))
    and $soak.runtime == "hepta"
    and $soak.status == "ready"
    and $soak.gate == "hepta_live_mutation_pre_activation_soak_evidence_gate"
    and $soak.pre_activation_soak_evidence_gate_ready == true
    and $soak.minimum_required_samples >= 24
    and $soak.long_soak_executed_by_this_gate == false
    and $soak.long_soak_execution_default_enabled == false
    and $soak.long_soak_evidence_candidate_ready == true
    and $soak.long_soak_evidence_persisted == false
    and $soak.activation_allowed == false
    and $soak.live_mutation_execution_ready == false
    and $soak.receipt_persistence_enabled == false
    and $soak.receipt_persisted == false
    and $soak.operator_approval_recorded == false
    and ($soak.side_effects | to_entries | all(.value == false))
    and $ledger.product == "Hepta"
    and $ledger.status == "ready"
    and $ledger.ledger_status.dry_run_validator_ready == true
    and $ledger.ledger_status.activation_packet_recorded == false
    and $ledger.ledger_status.required_evidence_count == 8
    and $ledger.ledger_status.recorded_evidence_count == 0
    and $ledger.ledger_status.fresh_evidence_count == 0
    and $ledger.ledger_status.evidence_ledger_ready == true
    and $ledger.ledger_status.evidence_recorded == false
    and $ledger.ledger_status.active_wiring_allowed == false
    and ($ledger.required_evidence | length) == 8
    and $ledger.denied_active_decisions.public_release_claim_allowed == false
    and $ledger.denied_active_decisions.release_artifact_write_allowed == false
    and ($ledger.side_effects | to_entries | all(.value == false))
    and $receipt.product == "Hepta"
    and $receipt.status == "ready"
    and $receipt.packet_status.source_materialization_dry_run_ready == true
    and $receipt.packet_status.required_approval_field_count == 12
    and $receipt.packet_status.approval_field_count == 12
    and $receipt.packet_status.recorded_approval_field_count == 0
    and $receipt.packet_status.redacted_or_hashed_field_count == 10
    and $receipt.packet_status.operator_approval_recorded == false
    and $receipt.packet_status.activation_request_recorded == false
    and $receipt.packet_status.materialization_plan_recorded == false
    and $receipt.packet_status.fresh_trusted_records_recorded == false
    and $receipt.packet_status.active_binary_sha_recorded == false
    and $receipt.packet_status.public_artifact_policy_recorded == false
    and $receipt.packet_status.filesystem_persistence_approval_packet_ready == true
    and $receipt.packet_status.filesystem_persistence_allowed == false
    and $receipt.packet_status.filesystem_persistence_execution_performed == false
    and $receipt.packet_status.workspace_write_performed == false
    and $receipt.packet_status.evidence_receipt_persisted == false
    and $receipt.packet_status.activation_blocked_by_filesystem_persistence_approval == true
    and $receipt.packet_status.activation_allowed_by_filesystem_persistence_approval == false
    and $receipt.packet_status.active_wiring_allowed == false
    and ($receipt.required_approval_fields | length) == 12
    and $receipt.denied_active_decisions.public_release_claim_allowed == false
    and $receipt.denied_active_decisions.release_artifact_write_allowed == false
    and ($receipt.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_fresh_long_soak_evidence_ledger_receipt_gate" \
  --arg approval_packet_report_sha256 "$approval_packet_report_sha256" \
  --arg pre_activation_soak_report_sha256 "$pre_activation_soak_report_sha256" \
  --arg activation_ledger_report_sha256 "$activation_ledger_report_sha256" \
  --arg receipt_persistence_packet_report_sha256 "$receipt_persistence_packet_report_sha256" \
  --arg ledger_receipt_index_hash_sha256 "$ledger_receipt_index_hash_sha256" \
  --arg ledger_receipt_redaction_hash_sha256 "$ledger_receipt_redaction_hash_sha256" \
  --arg ledger_receipt_side_effect_hash_sha256 "$ledger_receipt_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson approval "$APPROVAL_PACKET_JSON" \
  --argjson soak "$PRE_ACTIVATION_SOAK_JSON" \
  --argjson ledger "$ACTIVATION_LEDGER_JSON" \
  --argjson receipt "$RECEIPT_PERSISTENCE_PACKET_JSON" \
  '
    ([
      "long_soak_evidence_id",
      "activation_request_id",
      "operator_approval_id",
      "operator_identity_hash",
      "single_surface_activation_scope",
      "soak_command_hash",
      "sample_count",
      "ok_count",
      "fail_count",
      "started_at_unix_ms",
      "finished_at_unix_ms",
      "active_binary_sha256",
      "watchdog_evidence_id",
      "dependency_isolation_evidence_id",
      "browser_smoke_evidence_id",
      "receipt_payload_hash",
      "redaction_policy_id",
      "no_secret_review_id",
      "rollback_plan_id",
      "public_claim_and_artifact_decision"
    ]) as $ledger_fields
    | ([
      "receipt_id",
      "ledger_record_id",
      "activation_request_id",
      "operator_approval_id",
      "operator_identity_hash",
      "long_soak_evidence_id",
      "sample_set_hash",
      "redacted_soak_summary_hash",
      "source_approval_packet_report_sha256",
      "source_pre_activation_soak_report_sha256",
      "source_activation_ledger_report_sha256",
      "source_receipt_persistence_packet_report_sha256",
      "active_binary_sha256",
      "route_status_hash",
      "no_secret_payload_review_id",
      "materialization_plan_id",
      "filesystem_persistence_approval_id",
      "rollback_plan_id",
      "post_activation_watchdog_plan_id",
      "post_activation_long_soak_plan_id"
    ]) as $receipt_fields
    | ([
      "raw_soak_sample_payloads_must_not_be_persisted",
      "credential_values_must_be_absent",
      "secret_file_paths_must_be_absent_or_redacted",
      "channel_payloads_must_be_hash_only",
      "provider_prompts_and_outputs_must_be_absent",
      "operator_identity_must_be_hash_only",
      "timestamps_must_be bounded evidence metadata only",
      "filesystem_paths_must_be redacted or allowlist-bound",
      "public_claim_and_artifact_decision_must_remain_false",
      "receipt_hash_chain_must_not_write_until_specific_approval"
    ]) as $redaction_rules
    | ([
      {id:"approval-packet", ready:true, blocked:true, source_gate:$approval.gate, source_report_sha256:$approval_packet_report_sha256},
      {id:"pre-activation-soak-schema", ready:true, blocked:true, source_gate:$soak.gate, source_report_sha256:$pre_activation_soak_report_sha256},
      {id:"activation-evidence-ledger", ready:true, blocked:true, source_gate:$ledger.evidence_ledger_gate, source_report_sha256:$activation_ledger_report_sha256},
      {id:"receipt-filesystem-persistence-approval", ready:true, blocked:true, source_gate:$receipt.filesystem_persistence_approval_packet_gate, source_report_sha256:$receipt_persistence_packet_report_sha256},
      {id:"redaction-and-no-secret-review", ready:true, blocked:true, required_rule_count:($redaction_rules | length)},
      {id:"audit-binding-and-hash-chain", ready:true, blocked:true, persisted_record_count:0},
      {id:"activation-side-effect-boundary", ready:true, blocked:true, mutation_surface_count:24}
    ]) as $families
    | ([
      "fresh_24_sample_long_soak_not_executed_by_this_gate",
      "long_soak_evidence_record_not_recorded",
      "long_soak_evidence_record_not_persisted",
      "trusted_evidence_records_not_accepted",
      "approval_packet_not_recorded",
      "operator_approval_not_recorded",
      "activation_request_not_recorded",
      "ledger_record_not_recorded",
      "receipt_not_materialized",
      "receipt_not_persisted",
      "filesystem_persistence_approval_not_recorded",
      "raw_soak_sample_payload_persistence_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "provider_model_invocation_denied",
      "channel_delivery_denied",
      "install_restart_active_binary_mutation_denied",
      "upstream_fetch_merge_denied"
    ]) as $denied
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      fresh_long_soak_evidence_ledger_receipt_schema_version:"hepta_core_activation_fresh_long_soak_evidence_ledger_receipt_v1",
      fresh_long_soak_evidence_ledger_receipt_ready:true,
      ledger_receipt_mode:"schema_only_no_long_soak_execution_no_persistence",
      ledger_receipt_decision:"blocked_until_fresh_24_sample_long_soak_evidence_receipt_and_operator_approval_records_exist",
      source_approval_packet_gate:$approval.gate,
      source_pre_activation_soak_gate:$soak.gate,
      source_activation_evidence_ledger_gate:$ledger.evidence_ledger_gate,
      source_receipt_persistence_approval_packet_gate:$receipt.filesystem_persistence_approval_packet_gate,
      source_approval_packet_report_sha256:$approval_packet_report_sha256,
      source_pre_activation_soak_report_sha256:$pre_activation_soak_report_sha256,
      source_activation_ledger_report_sha256:$activation_ledger_report_sha256,
      source_receipt_persistence_packet_report_sha256:$receipt_persistence_packet_report_sha256,
      source_report_hashes:[
        $approval_packet_report_sha256,
        $pre_activation_soak_report_sha256,
        $activation_ledger_report_sha256,
        $receipt_persistence_packet_report_sha256
      ],
      ledger_receipt_index_hash_sha256:$ledger_receipt_index_hash_sha256,
      ledger_receipt_redaction_hash_sha256:$ledger_receipt_redaction_hash_sha256,
      ledger_receipt_side_effect_hash_sha256:$ledger_receipt_side_effect_hash_sha256,
      required_source_count:4,
      ready_source_count:4,
      activation_blocking_source_count:4,
      minimum_required_long_soak_samples:$min_long_soak_samples,
      required_soak_command:$soak.required_soak_command,
      long_soak_executed_by_this_gate:false,
      long_soak_execution_default_enabled:false,
      long_soak_evidence_candidate_ready:true,
      long_soak_evidence_recorded:false,
      long_soak_evidence_persisted:false,
      long_soak_evidence_fresh:false,
      source_required_evidence_count:$ledger.ledger_status.required_evidence_count,
      source_recorded_evidence_count:$ledger.ledger_status.recorded_evidence_count,
      source_fresh_evidence_count:$ledger.ledger_status.fresh_evidence_count,
      required_ledger_record_field_count:($ledger_fields | length),
      recorded_ledger_record_field_count:0,
      required_receipt_field_count:($receipt_fields | length),
      recorded_receipt_field_count:0,
      materialized_receipt_field_count:0,
      required_redaction_rule_count:($redaction_rules | length),
      accepted_redaction_rule_count:0,
      redacted_or_hashed_field_count:14,
      required_filesystem_persistence_approval_field_count:$receipt.packet_status.required_approval_field_count,
      recorded_filesystem_persistence_approval_field_count:$receipt.packet_status.recorded_approval_field_count,
      accepted_trusted_record_count:$approval.accepted_trusted_record_count,
      fresh_trusted_record_count:$approval.fresh_trusted_record_count,
      operator_approval_recorded:false,
      activation_request_recorded:false,
      approval_packet_recorded:false,
      approval_packet_accepted:false,
      filesystem_persistence_approval_recorded:false,
      filesystem_persistence_approval_accepted:false,
      ledger_record_recorded:false,
      ledger_record_persisted:false,
      receipt_materialized:false,
      receipt_persisted:false,
      receipt_hash_chain_recorded:false,
      audit_trail_recorded:false,
      audit_trail_persisted:false,
      raw_soak_sample_payload_persisted:false,
      redacted_soak_summary_persisted:false,
      activation_allowed:false,
      live_mutation_execution_ready:false,
      memory_store_mutation_allowed:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      public_distribution_publication_allowed:false,
      release_artifact_write_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      install_restart_allowed:false,
      upstream_fetch_merge_allowed:false,
      required_ledger_record_fields:$ledger_fields,
      required_receipt_fields:$receipt_fields,
      redaction_and_audit_rules:$redaction_rules,
      ledger_receipt_readiness_families:$families,
      denied_by_fresh_long_soak_evidence_ledger_receipt:$denied,
      denied_by_fresh_long_soak_evidence_ledger_receipt_count:($denied | length),
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
        receipt_materialized:false,
        receipt_persisted:false,
        audit_trail_persisted:false,
        trusted_record_persisted:false
      }
    }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_fresh_long_soak_evidence_ledger_receipt_gate"
  and .fresh_long_soak_evidence_ledger_receipt_ready == true
  and .ledger_receipt_mode == "schema_only_no_long_soak_execution_no_persistence"
  and .ledger_receipt_decision == "blocked_until_fresh_24_sample_long_soak_evidence_receipt_and_operator_approval_records_exist"
  and .required_source_count == 4
  and .ready_source_count == 4
  and .activation_blocking_source_count == 4
  and .minimum_required_long_soak_samples >= 24
  and .long_soak_executed_by_this_gate == false
  and .long_soak_execution_default_enabled == false
  and .long_soak_evidence_candidate_ready == true
  and .long_soak_evidence_recorded == false
  and .long_soak_evidence_persisted == false
  and .long_soak_evidence_fresh == false
  and .source_required_evidence_count == 8
  and .source_recorded_evidence_count == 0
  and .source_fresh_evidence_count == 0
  and .required_ledger_record_field_count == 20
  and .recorded_ledger_record_field_count == 0
  and .required_receipt_field_count == 20
  and .recorded_receipt_field_count == 0
  and .materialized_receipt_field_count == 0
  and .required_redaction_rule_count == 10
  and .accepted_redaction_rule_count == 0
  and .redacted_or_hashed_field_count == 14
  and .required_filesystem_persistence_approval_field_count == 12
  and .recorded_filesystem_persistence_approval_field_count == 0
  and .accepted_trusted_record_count == 0
  and .fresh_trusted_record_count == 0
  and .operator_approval_recorded == false
  and .activation_request_recorded == false
  and .approval_packet_recorded == false
  and .approval_packet_accepted == false
  and .filesystem_persistence_approval_recorded == false
  and .filesystem_persistence_approval_accepted == false
  and .ledger_record_recorded == false
  and .ledger_record_persisted == false
  and .receipt_materialized == false
  and .receipt_persisted == false
  and .receipt_hash_chain_recorded == false
  and .audit_trail_recorded == false
  and .audit_trail_persisted == false
  and .raw_soak_sample_payload_persisted == false
  and .redacted_soak_summary_persisted == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .memory_store_mutation_allowed == false
  and .public_release_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .provider_model_invocation_allowed == false
  and .channel_delivery_allowed == false
  and .install_restart_allowed == false
  and .upstream_fetch_merge_allowed == false
  and (.required_ledger_record_fields | length) == 20
  and (.required_receipt_fields | length) == 20
  and (.redaction_and_audit_rules | length) == 10
  and (.ledger_receipt_readiness_families | length) == 7
  and (.ledger_receipt_readiness_families | all(.ready == true and .blocked == true))
  and .denied_by_fresh_long_soak_evidence_ledger_receipt_count == 18
  and (.denied_by_fresh_long_soak_evidence_ledger_receipt | length) == 18
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
