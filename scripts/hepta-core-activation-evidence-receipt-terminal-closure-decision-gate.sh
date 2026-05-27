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

RECEIPT_ACCEPTANCE_DENIAL_JSON="$(
  capture_json_report \
    "hepta-core-activation-evidence-receipt-acceptance-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-evidence-receipt-acceptance-denial-gate.sh
)"

receipt_acceptance_denial_report_sha256="$(sha256_text "$RECEIPT_ACCEPTANCE_DENIAL_JSON")"
terminal_closure_index_hash_sha256="$(sha256_text "hepta-core-activation-evidence-receipt-terminal-closure-decision:index:$receipt_acceptance_denial_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
terminal_closure_no_activation_hash_sha256="$(sha256_text "hepta-core-activation-evidence-receipt-terminal-closure-decision:no-activation:$receipt_acceptance_denial_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
terminal_closure_redaction_hash_sha256="$(sha256_text "hepta-core-activation-evidence-receipt-terminal-closure-decision:redaction:$receipt_acceptance_denial_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson acceptance "$RECEIPT_ACCEPTANCE_DENIAL_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $acceptance.runtime == "hepta"
    and $acceptance.status == "ready"
    and $acceptance.gate == "hepta_core_activation_evidence_receipt_acceptance_denial_gate"
    and $acceptance.receipt_acceptance_denial_gate_ready == true
    and $acceptance.acceptance_decision == "blocked_until_operator_approval_filesystem_persistence_approval_receipt_persistence_and_fresh_long_soak_evidence_acceptance_exist"
    and $acceptance.required_source_count == 4
    and $acceptance.ready_source_count == 4
    and $acceptance.activation_blocking_source_count == 4
    and $acceptance.minimum_required_long_soak_samples >= 24
    and $acceptance.source_required_persistence_execution_field_count == 20
    and $acceptance.source_recorded_persistence_execution_field_count == 0
    and $acceptance.source_required_approval_field_count == 12
    and $acceptance.source_recorded_approval_field_count == 0
    and $acceptance.source_required_command_field_count == 10
    and $acceptance.source_recorded_command_field_count == 0
    and $acceptance.required_receipt_acceptance_field_count == 20
    and $acceptance.recorded_receipt_acceptance_field_count == 0
    and $acceptance.blocked_receipt_acceptance_fixture_count == 4
    and $acceptance.allowed_receipt_acceptance_fixture_count == 0
    and $acceptance.receipt_accepted_count == 0
    and $acceptance.operator_approval_recorded == false
    and $acceptance.operator_authority_accepted == false
    and $acceptance.activation_request_recorded == false
    and $acceptance.filesystem_persistence_approval_recorded == false
    and $acceptance.receipt_persistence_command_enabled_by_default == false
    and $acceptance.receipt_persistence_command_invoked == false
    and $acceptance.receipt_persistence_execution_performed_count == 0
    and $acceptance.workspace_write_performed_count == 0
    and $acceptance.evidence_receipt_persisted_count == 0
    and $acceptance.receipt_persisted == false
    and $acceptance.receipt_accepted == false
    and $acceptance.receipt_acceptance_recorded == false
    and $acceptance.completion_ack_recorded == false
    and $acceptance.completion_ack_accepted == false
    and $acceptance.ledger_recorded == false
    and $acceptance.ledger_persisted == false
    and $acceptance.index_recorded == false
    and $acceptance.delivery_recorded == false
    and $acceptance.activation_allowed == false
    and $acceptance.live_mutation_execution_ready == false
    and $acceptance.public_release_claim_allowed == false
    and $acceptance.release_artifact_write_allowed == false
    and ($acceptance.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_evidence_receipt_terminal_closure_decision_gate" \
  --arg receipt_acceptance_denial_report_sha256 "$receipt_acceptance_denial_report_sha256" \
  --arg terminal_closure_index_hash_sha256 "$terminal_closure_index_hash_sha256" \
  --arg terminal_closure_no_activation_hash_sha256 "$terminal_closure_no_activation_hash_sha256" \
  --arg terminal_closure_redaction_hash_sha256 "$terminal_closure_redaction_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson acceptance "$RECEIPT_ACCEPTANCE_DENIAL_JSON" \
  '
    ([
      "terminal_closure_id",
      "terminal_decision_id",
      "activation_request_id",
      "operator_approval_id",
      "operator_identity_hash",
      "fresh_long_soak_evidence_id",
      "fresh_trusted_evidence_record_set_id",
      "long_soak_ledger_record_id",
      "evidence_receipt_id",
      "evidence_receipt_hash",
      "filesystem_persistence_approval_id",
      "receipt_persistence_command_id",
      "receipt_persistence_execution_id",
      "receipt_acceptance_id",
      "ledger_record_id",
      "index_record_id",
      "delivery_record_id",
      "completion_ack_id",
      "active_binary_sha256",
      "source_receipt_acceptance_denial_report_sha256",
      "no_secret_payload_review_id",
      "rollback_plan_id",
      "public_claim_denial_decision",
      "release_artifact_denial_decision"
    ]) as $closure_fields
    | ([
      "explicit_operator_approval_record_missing",
      "operator_identity_hash_missing",
      "activation_request_record_missing",
      "fresh_24_sample_long_soak_evidence_record_missing",
      "fresh_trusted_evidence_record_set_missing",
      "filesystem_persistence_approval_record_missing",
      "receipt_persistence_command_enablement_missing",
      "receipt_persistence_execution_record_missing",
      "receipt_acceptance_record_missing",
      "ledger_record_missing",
      "index_delivery_records_missing",
      "completion_ack_record_missing"
    ]) as $missing
    | ([
      "activation_remains_blocked",
      "live_mutation_execution_denied",
      "memory_store_mutation_denied",
      "receipt_acceptance_denied",
      "receipt_persistence_denied",
      "filesystem_persistence_denied",
      "ledger_index_delivery_ack_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "install_restart_active_binary_mutation_denied",
      "provider_model_invocation_denied",
      "channel_delivery_denied",
      "credential_secret_read_denied",
      "upstream_fetch_merge_denied"
    ]) as $blocked
    | ([
      {
        id:"core-activation-terminal-closure-missing-operator-approval",
        fixture_kind:"terminal_closure_missing_operator_approval",
        terminal_closure_requested:true,
        operator_approval_recorded:false,
        operator_authority_accepted:false,
        terminal_closure_recorded:false,
        activation_allowed:false
      },
      {
        id:"core-activation-terminal-closure-missing-fresh-long-soak-records",
        fixture_kind:"terminal_closure_missing_fresh_long_soak_records",
        terminal_closure_requested:true,
        minimum_required_samples:$min_long_soak_samples,
        long_soak_evidence_recorded:false,
        fresh_trusted_evidence_records_accepted:false,
        terminal_closure_recorded:false,
        activation_allowed:false
      },
      {
        id:"core-activation-terminal-closure-missing-filesystem-persistence-approval",
        fixture_kind:"terminal_closure_missing_filesystem_persistence_approval",
        terminal_closure_requested:true,
        filesystem_persistence_approval_recorded:false,
        filesystem_persistence_execution_performed:false,
        evidence_receipt_persisted:false,
        terminal_closure_recorded:false,
        activation_allowed:false
      },
      {
        id:"core-activation-terminal-closure-missing-receipt-persistence-acceptance",
        fixture_kind:"terminal_closure_missing_receipt_persistence_acceptance",
        terminal_closure_requested:true,
        receipt_persistence_command_invoked:false,
        receipt_persistence_execution_performed:false,
        receipt_accepted:false,
        receipt_acceptance_recorded:false,
        terminal_closure_recorded:false,
        activation_allowed:false
      },
      {
        id:"core-activation-terminal-closure-missing-ledger-index-delivery-ack",
        fixture_kind:"terminal_closure_missing_ledger_index_delivery_ack",
        terminal_closure_requested:true,
        ledger_recorded:false,
        index_recorded:false,
        delivery_recorded:false,
        completion_ack_recorded:false,
        terminal_closure_recorded:false,
        activation_allowed:false
      },
      {
        id:"core-activation-terminal-closure-public-release-or-active-mutation-attempt",
        fixture_kind:"terminal_closure_public_release_or_active_mutation_attempt",
        terminal_closure_requested:true,
        public_release_claim_requested:true,
        release_artifact_write_requested:true,
        install_restart_requested:true,
        active_binary_mutation_requested:true,
        terminal_closure_recorded:false,
        public_release_claim_allowed:false,
        release_artifact_write_allowed:false,
        active_binary_mutation_allowed:false,
        activation_allowed:false
      }
    ]) as $fixtures
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      terminal_closure_schema_version:"hepta_core_activation_evidence_receipt_terminal_closure_decision_v1",
      terminal_closure_decision_gate_ready:true,
      terminal_closure_mode:"receipt_acceptance_transitive_closure_no_activation_no_persistence_no_publication",
      terminal_closure_decision:"blocked_until_operator_approval_fresh_24_sample_evidence_filesystem_persistence_receipt_persistence_ledger_index_delivery_completion_ack_exist",
      terminal_closure_verdict:"blocked",
      source_receipt_acceptance_denial_gate:$acceptance.gate,
      source_receipt_acceptance_denial_report_sha256:$receipt_acceptance_denial_report_sha256,
      source_report_hashes:[$receipt_acceptance_denial_report_sha256],
      terminal_closure_index_hash_sha256:$terminal_closure_index_hash_sha256,
      terminal_closure_no_activation_hash_sha256:$terminal_closure_no_activation_hash_sha256,
      terminal_closure_redaction_hash_sha256:$terminal_closure_redaction_hash_sha256,
      transitive_core_activation_source_gate_count:6,
      required_source_count:1,
      ready_source_count:1,
      activation_blocking_source_count:1,
      minimum_required_long_soak_samples:$min_long_soak_samples,
      source_receipt_acceptance_required_source_count:$acceptance.required_source_count,
      source_receipt_acceptance_ready_source_count:$acceptance.ready_source_count,
      source_receipt_acceptance_activation_blocking_source_count:$acceptance.activation_blocking_source_count,
      source_required_receipt_acceptance_field_count:$acceptance.required_receipt_acceptance_field_count,
      source_recorded_receipt_acceptance_field_count:$acceptance.recorded_receipt_acceptance_field_count,
      source_blocked_receipt_acceptance_fixture_count:$acceptance.blocked_receipt_acceptance_fixture_count,
      source_allowed_receipt_acceptance_fixture_count:$acceptance.allowed_receipt_acceptance_fixture_count,
      required_terminal_closure_field_count:($closure_fields | length),
      recorded_terminal_closure_field_count:0,
      required_terminal_closure_missing_requirement_count:($missing | length),
      remaining_terminal_closure_missing_requirement_count:($missing | length),
      required_terminal_closure_fixture_count:($fixtures | length),
      terminal_closure_fixture_count:($fixtures | length),
      blocked_terminal_closure_fixture_count:($fixtures | length),
      allowed_terminal_closure_fixture_count:0,
      terminal_closure_allowed:false,
      terminal_closure_performed:false,
      terminal_closure_recorded:false,
      terminal_closure_persisted:false,
      terminal_closure_materialized:false,
      terminal_closure_filesystem_written:false,
      terminal_closure_accepted:false,
      terminal_operator_decision_recorded:false,
      terminal_operator_decision_accepted:false,
      operator_approval_required:true,
      operator_approval_recorded:false,
      operator_identity_hash_recorded:false,
      operator_authority_accepted:false,
      activation_request_required:true,
      activation_request_recorded:false,
      long_soak_evidence_recorded:false,
      long_soak_evidence_persisted:false,
      long_soak_evidence_fresh:false,
      fresh_trusted_evidence_records_accepted:false,
      filesystem_persistence_approval_required:true,
      filesystem_persistence_approval_recorded:false,
      filesystem_persistence_allowed:false,
      filesystem_persistence_execution_performed:false,
      receipt_persistence_command_enabled_by_default:false,
      receipt_persistence_command_invoked:false,
      receipt_persistence_execution_performed:false,
      evidence_receipt_persisted:false,
      receipt_materialized:false,
      receipt_persisted:false,
      receipt_accepted:false,
      receipt_acceptance_recorded:false,
      receipt_acceptance_persisted:false,
      ledger_recorded:false,
      ledger_persisted:false,
      index_recorded:false,
      index_persisted:false,
      delivery_recorded:false,
      delivery_persisted:false,
      completion_ack_recorded:false,
      completion_ack_accepted:false,
      activation_allowed:false,
      activation_performed:false,
      live_mutation_execution_ready:false,
      live_mutation_execution_allowed:false,
      live_mutation_execution_performed:false,
      memory_store_mutation_allowed:false,
      memory_store_mutated:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      public_release_published:false,
      public_ga_claimed:false,
      public_distribution_publication_allowed:false,
      release_artifact_write_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      install_restart_allowed:false,
      active_binary_mutation_allowed:false,
      upstream_fetch_merge_allowed:false,
      credential_read_allowed:false,
      secret_value_read_allowed:false,
      required_terminal_closure_fields:$closure_fields,
      terminal_closure_missing_requirements:$missing,
      terminal_closure_fixtures:$fixtures,
      denied_by_terminal_closure_decision_gate:$blocked,
      denied_by_terminal_closure_decision_gate_count:($blocked | length),
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
        long_soak_evidence_persisted:false,
        ledger_record_persisted:false,
        ledger_index_mutated:false,
        delivery_record_persisted:false,
        completion_ack_recorded:false,
        receipt_persistence_command_invoked:false,
        receipt_persistence_execution:false,
        receipt_acceptance_recorded:false,
        receipt_acceptance_persisted:false,
        receipt_acceptance_materialized:false,
        receipt_acceptance_filesystem_written:false,
        terminal_closure_recorded:false,
        terminal_closure_persisted:false,
        terminal_closure_materialized:false,
        terminal_closure_filesystem_written:false,
        operator_authority_accepted:false,
        activation_authority_granted:false
      }
    }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_evidence_receipt_terminal_closure_decision_gate"
  and .terminal_closure_decision_gate_ready == true
  and .terminal_closure_mode == "receipt_acceptance_transitive_closure_no_activation_no_persistence_no_publication"
  and .terminal_closure_verdict == "blocked"
  and .transitive_core_activation_source_gate_count == 6
  and .required_source_count == 1
  and .ready_source_count == 1
  and .activation_blocking_source_count == 1
  and .minimum_required_long_soak_samples >= 24
  and (.source_report_hashes | length) == 1
  and (.source_report_hashes | all(. != ""))
  and .source_receipt_acceptance_required_source_count == 4
  and .source_receipt_acceptance_ready_source_count == 4
  and .source_receipt_acceptance_activation_blocking_source_count == 4
  and .source_required_receipt_acceptance_field_count == 20
  and .source_recorded_receipt_acceptance_field_count == 0
  and .source_blocked_receipt_acceptance_fixture_count == 4
  and .source_allowed_receipt_acceptance_fixture_count == 0
  and .required_terminal_closure_field_count == 24
  and .recorded_terminal_closure_field_count == 0
  and .required_terminal_closure_missing_requirement_count == 12
  and .remaining_terminal_closure_missing_requirement_count == 12
  and .required_terminal_closure_fixture_count == 6
  and .terminal_closure_fixture_count == 6
  and .blocked_terminal_closure_fixture_count == 6
  and .allowed_terminal_closure_fixture_count == 0
  and .terminal_closure_allowed == false
  and .terminal_closure_performed == false
  and .terminal_closure_recorded == false
  and .terminal_closure_persisted == false
  and .terminal_closure_materialized == false
  and .terminal_closure_filesystem_written == false
  and .terminal_closure_accepted == false
  and .terminal_operator_decision_recorded == false
  and .terminal_operator_decision_accepted == false
  and .operator_approval_required == true
  and .operator_approval_recorded == false
  and .operator_identity_hash_recorded == false
  and .operator_authority_accepted == false
  and .activation_request_required == true
  and .activation_request_recorded == false
  and .long_soak_evidence_recorded == false
  and .long_soak_evidence_persisted == false
  and .long_soak_evidence_fresh == false
  and .fresh_trusted_evidence_records_accepted == false
  and .filesystem_persistence_approval_required == true
  and .filesystem_persistence_approval_recorded == false
  and .filesystem_persistence_allowed == false
  and .filesystem_persistence_execution_performed == false
  and .receipt_persistence_command_enabled_by_default == false
  and .receipt_persistence_command_invoked == false
  and .receipt_persistence_execution_performed == false
  and .evidence_receipt_persisted == false
  and .receipt_materialized == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .receipt_acceptance_recorded == false
  and .ledger_recorded == false
  and .ledger_persisted == false
  and .index_recorded == false
  and .delivery_recorded == false
  and .completion_ack_recorded == false
  and .completion_ack_accepted == false
  and .activation_allowed == false
  and .activation_performed == false
  and .live_mutation_execution_ready == false
  and .live_mutation_execution_allowed == false
  and .live_mutation_execution_performed == false
  and .memory_store_mutation_allowed == false
  and .memory_store_mutated == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .provider_model_invocation_allowed == false
  and .channel_delivery_allowed == false
  and .install_restart_allowed == false
  and .active_binary_mutation_allowed == false
  and .upstream_fetch_merge_allowed == false
  and .credential_read_allowed == false
  and .secret_value_read_allowed == false
  and (.required_terminal_closure_fields | length) == 24
  and (.terminal_closure_missing_requirements | length) == 12
  and (.terminal_closure_fixtures | length) == 6
  and (.terminal_closure_fixtures | all(.terminal_closure_requested == true and .terminal_closure_recorded == false and .activation_allowed == false))
  and .denied_by_terminal_closure_decision_gate_count == 14
  and (.denied_by_terminal_closure_decision_gate | length) == 14
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
