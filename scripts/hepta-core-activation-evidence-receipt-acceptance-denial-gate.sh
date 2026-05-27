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

FILESYSTEM_PERSISTENCE_DENIAL_JSON="$(
  capture_json_report \
    "hepta-core-activation-evidence-receipt-filesystem-persistence-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-evidence-receipt-filesystem-persistence-denial-gate.sh
)"

FILESYSTEM_PERSISTENCE_APPROVAL_PACKET_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet" \
    scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh
)"

PERSISTENCE_COMMAND_CONTRACT_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract" \
    scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh
)"

PERSISTENCE_INVOCATION_DRY_RUN_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run" \
    scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh
)"

filesystem_persistence_denial_report_sha256="$(sha256_text "$FILESYSTEM_PERSISTENCE_DENIAL_JSON")"
filesystem_persistence_approval_packet_report_sha256="$(sha256_text "$FILESYSTEM_PERSISTENCE_APPROVAL_PACKET_JSON")"
persistence_command_contract_report_sha256="$(sha256_text "$PERSISTENCE_COMMAND_CONTRACT_JSON")"
persistence_invocation_dry_run_report_sha256="$(sha256_text "$PERSISTENCE_INVOCATION_DRY_RUN_JSON")"
receipt_acceptance_denial_index_hash_sha256="$(sha256_text "hepta-core-activation-evidence-receipt-acceptance-denial:index:$filesystem_persistence_denial_report_sha256:$filesystem_persistence_approval_packet_report_sha256:$persistence_command_contract_report_sha256:$persistence_invocation_dry_run_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
receipt_acceptance_denial_no_acceptance_hash_sha256="$(sha256_text "hepta-core-activation-evidence-receipt-acceptance-denial:no-acceptance:$filesystem_persistence_denial_report_sha256:$filesystem_persistence_approval_packet_report_sha256:$persistence_command_contract_report_sha256:$persistence_invocation_dry_run_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
receipt_acceptance_denial_redaction_hash_sha256="$(sha256_text "hepta-core-activation-evidence-receipt-acceptance-denial:redaction:$filesystem_persistence_denial_report_sha256:$filesystem_persistence_approval_packet_report_sha256:$persistence_command_contract_report_sha256:$persistence_invocation_dry_run_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson persistence_denial "$FILESYSTEM_PERSISTENCE_DENIAL_JSON" \
  --argjson approval_packet "$FILESYSTEM_PERSISTENCE_APPROVAL_PACKET_JSON" \
  --argjson command_contract "$PERSISTENCE_COMMAND_CONTRACT_JSON" \
  --argjson invocation_dry_run "$PERSISTENCE_INVOCATION_DRY_RUN_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $persistence_denial.runtime == "hepta"
    and $persistence_denial.status == "ready"
    and $persistence_denial.gate == "hepta_core_activation_evidence_receipt_filesystem_persistence_denial_gate"
    and $persistence_denial.filesystem_persistence_denial_gate_ready == true
    and $persistence_denial.required_source_count == 3
    and $persistence_denial.ready_source_count == 3
    and $persistence_denial.activation_blocking_source_count == 3
    and $persistence_denial.required_persistence_execution_field_count == 20
    and $persistence_denial.recorded_persistence_execution_field_count == 0
    and $persistence_denial.blocked_execution_fixture_count == 4
    and $persistence_denial.allowed_execution_fixture_count == 0
    and $persistence_denial.filesystem_persistence_approval_recorded == false
    and $persistence_denial.filesystem_persistence_allowed == false
    and $persistence_denial.filesystem_persistence_execution_performed == false
    and $persistence_denial.workspace_write_performed == false
    and $persistence_denial.evidence_receipt_persisted == false
    and $persistence_denial.activation_allowed == false
    and $persistence_denial.live_mutation_execution_ready == false
    and $persistence_denial.active_wiring_allowed == false
    and ($persistence_denial.side_effects | to_entries | all(.value == false))
    and $approval_packet.product == "Hepta"
    and $approval_packet.status == "ready"
    and $approval_packet.filesystem_persistence_approval_packet_id == "upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet"
    and $approval_packet.packet_status.source_materialization_dry_run_ready == true
    and $approval_packet.packet_status.required_approval_field_count == 12
    and $approval_packet.packet_status.approval_field_count == 12
    and $approval_packet.packet_status.recorded_approval_field_count == 0
    and $approval_packet.packet_status.required_for_filesystem_persistence_field_count == 12
    and $approval_packet.packet_status.operator_approval_required == true
    and $approval_packet.packet_status.operator_approval_recorded == false
    and $approval_packet.packet_status.activation_request_required == true
    and $approval_packet.packet_status.activation_request_recorded == false
    and $approval_packet.packet_status.filesystem_persistence_approval_packet_ready == true
    and $approval_packet.packet_status.filesystem_persistence_allowed == false
    and $approval_packet.packet_status.filesystem_persistence_execution_performed == false
    and $approval_packet.packet_status.workspace_write_performed == false
    and $approval_packet.packet_status.evidence_receipt_persisted == false
    and $approval_packet.packet_status.activation_blocked_by_filesystem_persistence_approval == true
    and $approval_packet.packet_status.activation_allowed_by_filesystem_persistence_approval == false
    and $approval_packet.packet_status.active_wiring_allowed == false
    and ($approval_packet.side_effects | to_entries | all(.value == false))
    and $command_contract.product == "Hepta"
    and $command_contract.status == "ready"
    and $command_contract.command_contract_id == "upstream-codex-activation-evidence-receipt-persistence-command-contract"
    and $command_contract.command_status.source_denial_matrix_ready == true
    and $command_contract.command_status.required_command_field_count == 10
    and $command_contract.command_status.recorded_command_field_count == 0
    and $command_contract.command_status.operator_approval_required == true
    and $command_contract.command_status.operator_approval_recorded == false
    and $command_contract.command_status.activation_request_required == true
    and $command_contract.command_status.activation_request_recorded == false
    and $command_contract.command_status.receipt_persistence_command_enabled_by_default == false
    and $command_contract.command_status.receipt_persistence_command_invoked == false
    and $command_contract.command_status.receipt_persistence_execution_performed == false
    and $command_contract.command_status.workspace_write_performed == false
    and $command_contract.command_status.evidence_receipt_persisted == false
    and $command_contract.command_status.activation_blocked_by_persistence_contract == true
    and $command_contract.command_status.activation_allowed_by_persistence_contract == false
    and $command_contract.command_status.active_wiring_allowed == false
    and ($command_contract.side_effects | to_entries | all(.value == false))
    and $invocation_dry_run.product == "Hepta"
    and $invocation_dry_run.status == "ready"
    and $invocation_dry_run.invocation_dry_run_id == "upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run"
    and $invocation_dry_run.invocation_status.source_command_contract_ready == true
    and $invocation_dry_run.invocation_status.required_invocation_fixture_count == 3
    and $invocation_dry_run.invocation_status.command_invocation_attempt_count == 3
    and $invocation_dry_run.invocation_status.command_invocation_performed_count == 0
    and $invocation_dry_run.invocation_status.receipt_persistence_execution_performed_count == 0
    and $invocation_dry_run.invocation_status.workspace_write_performed_count == 0
    and $invocation_dry_run.invocation_status.evidence_receipt_persisted_count == 0
    and $invocation_dry_run.invocation_status.operator_approved_fixture_count == 3
    and $invocation_dry_run.invocation_status.activation_request_bound_fixture_count == 3
    and $invocation_dry_run.invocation_status.public_claim_attempt_count == 1
    and $invocation_dry_run.invocation_status.release_artifact_write_attempt_count == 1
    and $invocation_dry_run.invocation_status.receipt_persistence_command_enabled_by_default == false
    and $invocation_dry_run.invocation_status.invocation_dry_run_noop_ready == true
    and $invocation_dry_run.invocation_status.activation_blocked_by_invocation_dry_run == true
    and $invocation_dry_run.invocation_status.activation_allowed_by_invocation_dry_run == false
    and $invocation_dry_run.invocation_status.active_wiring_allowed == false
    and ($invocation_dry_run.fixtures | length) == 3
    and ($invocation_dry_run.fixtures | all(.command_invocation_performed == false and .workspace_write_performed == false and .evidence_receipt_persisted == false))
    and ($invocation_dry_run.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_evidence_receipt_acceptance_denial_gate" \
  --arg filesystem_persistence_denial_report_sha256 "$filesystem_persistence_denial_report_sha256" \
  --arg filesystem_persistence_approval_packet_report_sha256 "$filesystem_persistence_approval_packet_report_sha256" \
  --arg persistence_command_contract_report_sha256 "$persistence_command_contract_report_sha256" \
  --arg persistence_invocation_dry_run_report_sha256 "$persistence_invocation_dry_run_report_sha256" \
  --arg receipt_acceptance_denial_index_hash_sha256 "$receipt_acceptance_denial_index_hash_sha256" \
  --arg receipt_acceptance_denial_no_acceptance_hash_sha256 "$receipt_acceptance_denial_no_acceptance_hash_sha256" \
  --arg receipt_acceptance_denial_redaction_hash_sha256 "$receipt_acceptance_denial_redaction_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson persistence_denial "$FILESYSTEM_PERSISTENCE_DENIAL_JSON" \
  --argjson approval_packet "$FILESYSTEM_PERSISTENCE_APPROVAL_PACKET_JSON" \
  --argjson command_contract "$PERSISTENCE_COMMAND_CONTRACT_JSON" \
  --argjson invocation_dry_run "$PERSISTENCE_INVOCATION_DRY_RUN_JSON" \
  '
    ([
      "receipt_acceptance_request_id",
      "receipt_id",
      "receipt_payload_hash",
      "receipt_persistence_command_id",
      "receipt_persistence_approval_id",
      "filesystem_persistence_approval_id",
      "operator_approval_id",
      "operator_identity_hash",
      "fresh_long_soak_evidence_id",
      "trusted_evidence_record_id",
      "active_binary_sha256",
      "source_filesystem_persistence_denial_report_sha256",
      "source_filesystem_persistence_approval_packet_report_sha256",
      "source_persistence_command_contract_report_sha256",
      "source_persistence_invocation_dry_run_report_sha256",
      "no_secret_payload_review_id",
      "ledger_record_id",
      "index_record_id",
      "delivery_record_id",
      "completion_ack_id"
    ]) as $acceptance_fields
    | ([
      "filesystem-persistence-denial-boundary",
      "filesystem-persistence-approval-packet-boundary",
      "receipt-persistence-command-contract-boundary",
      "receipt-persistence-invocation-dry-run-boundary",
      "operator-authority-non-acceptance-boundary",
      "ledger-index-delivery-non-recording-boundary",
      "completion-ack-non-acceptance-boundary",
      "activation-side-effect-boundary"
    ]) as $acceptance_families
    | ([
      "receipt_persistence_denied",
      "receipt_acceptance_request_not_recorded",
      "receipt_acceptance_not_performed",
      "receipt_acceptance_not_recorded",
      "receipt_acceptance_not_persisted",
      "receipt_acceptance_not_materialized",
      "receipt_acceptance_filesystem_write_denied",
      "operator_approval_not_recorded",
      "operator_authority_not_accepted",
      "activation_request_not_recorded",
      "filesystem_persistence_approval_not_recorded",
      "filesystem_persistence_execution_denied",
      "receipt_persistence_command_disabled_by_default",
      "receipt_persistence_command_invocation_denied",
      "receipt_persistence_execution_denied",
      "workspace_write_denied",
      "evidence_receipt_persistence_denied",
      "ledger_record_not_recorded",
      "ledger_record_not_persisted",
      "index_record_not_recorded",
      "delivery_record_not_recorded",
      "completion_ack_not_recorded",
      "completion_ack_not_accepted",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "install_restart_active_binary_mutation_denied",
      "provider_model_invocation_denied",
      "channel_delivery_denied",
      "upstream_fetch_merge_denied"
    ]) as $denied
    | ([
      {
        fixture_id:"receipt-persistence-denied-attempt",
        fixture_kind:"receipt_persistence_denied_attempt",
        acceptance_requested:true,
        receipt_persisted:false,
        receipt_accepted:false,
        acceptance_recorded:false,
        activation_allowed:false
      },
      {
        fixture_id:"operator-authority-unrecorded-attempt",
        fixture_kind:"operator_authority_unrecorded_attempt",
        acceptance_requested:true,
        operator_approval_recorded:false,
        operator_authority_accepted:false,
        receipt_accepted:false,
        activation_allowed:false
      },
      {
        fixture_id:"ledger-index-delivery-ack-attempt",
        fixture_kind:"ledger_index_delivery_ack_attempt",
        acceptance_requested:true,
        ledger_recorded:false,
        index_recorded:false,
        delivery_recorded:false,
        completion_ack_recorded:false,
        completion_ack_accepted:false,
        receipt_accepted:false,
        activation_allowed:false
      },
      {
        fixture_id:"public-claim-artifact-acceptance-attempt",
        fixture_kind:"public_claim_artifact_acceptance_attempt",
        acceptance_requested:true,
        public_release_claim_requested:true,
        release_artifact_write_requested:true,
        receipt_accepted:false,
        public_release_claim_allowed:false,
        release_artifact_write_allowed:false,
        activation_allowed:false
      }
    ]) as $fixtures
    | ([
      {id:"filesystem-persistence-denial", ready:true, blocked:true, source_gate:$persistence_denial.gate, source_report_sha256:$filesystem_persistence_denial_report_sha256},
      {id:"filesystem-persistence-approval-packet", ready:true, blocked:true, source_gate:$approval_packet.filesystem_persistence_approval_packet_gate, source_report_sha256:$filesystem_persistence_approval_packet_report_sha256},
      {id:"persistence-command-contract", ready:true, blocked:true, source_gate:$command_contract.receipt_persistence_command_contract_gate, source_report_sha256:$persistence_command_contract_report_sha256},
      {id:"persistence-invocation-dry-run", ready:true, blocked:true, source_gate:$invocation_dry_run.receipt_persistence_invocation_dry_run_gate, source_report_sha256:$persistence_invocation_dry_run_report_sha256},
      {id:"activation-side-effect-boundary", ready:true, blocked:true, denied_action_count:($denied | length)}
    ]) as $source_families
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      receipt_acceptance_denial_schema_version:"hepta_core_activation_evidence_receipt_acceptance_denial_v1",
      receipt_acceptance_denial_gate_ready:true,
      acceptance_mode:"schema_and_authority_denial_only_no_acceptance_no_persistence",
      acceptance_decision:"blocked_until_operator_approval_filesystem_persistence_approval_receipt_persistence_and_fresh_long_soak_evidence_acceptance_exist",
      source_filesystem_persistence_denial_gate:$persistence_denial.gate,
      source_filesystem_persistence_approval_packet_gate:$approval_packet.filesystem_persistence_approval_packet_gate,
      source_persistence_command_contract_gate:$command_contract.receipt_persistence_command_contract_gate,
      source_persistence_invocation_dry_run_gate:$invocation_dry_run.receipt_persistence_invocation_dry_run_gate,
      source_filesystem_persistence_denial_report_sha256:$filesystem_persistence_denial_report_sha256,
      source_filesystem_persistence_approval_packet_report_sha256:$filesystem_persistence_approval_packet_report_sha256,
      source_persistence_command_contract_report_sha256:$persistence_command_contract_report_sha256,
      source_persistence_invocation_dry_run_report_sha256:$persistence_invocation_dry_run_report_sha256,
      source_report_hashes:[
        $filesystem_persistence_denial_report_sha256,
        $filesystem_persistence_approval_packet_report_sha256,
        $persistence_command_contract_report_sha256,
        $persistence_invocation_dry_run_report_sha256
      ],
      receipt_acceptance_denial_index_hash_sha256:$receipt_acceptance_denial_index_hash_sha256,
      receipt_acceptance_denial_no_acceptance_hash_sha256:$receipt_acceptance_denial_no_acceptance_hash_sha256,
      receipt_acceptance_denial_redaction_hash_sha256:$receipt_acceptance_denial_redaction_hash_sha256,
      required_source_count:4,
      ready_source_count:4,
      activation_blocking_source_count:4,
      minimum_required_long_soak_samples:$min_long_soak_samples,
      source_required_persistence_execution_field_count:$persistence_denial.required_persistence_execution_field_count,
      source_recorded_persistence_execution_field_count:$persistence_denial.recorded_persistence_execution_field_count,
      source_required_approval_field_count:$approval_packet.packet_status.required_approval_field_count,
      source_recorded_approval_field_count:$approval_packet.packet_status.recorded_approval_field_count,
      source_required_command_field_count:$command_contract.command_status.required_command_field_count,
      source_recorded_command_field_count:$command_contract.command_status.recorded_command_field_count,
      source_command_invocation_attempt_count:$invocation_dry_run.invocation_status.command_invocation_attempt_count,
      source_command_invocation_performed_count:$invocation_dry_run.invocation_status.command_invocation_performed_count,
      required_receipt_acceptance_field_count:($acceptance_fields | length),
      recorded_receipt_acceptance_field_count:0,
      required_receipt_acceptance_fixture_count:($fixtures | length),
      receipt_acceptance_fixture_count:($fixtures | length),
      receipt_acceptance_request_count:($fixtures | length),
      blocked_receipt_acceptance_fixture_count:($fixtures | length),
      allowed_receipt_acceptance_fixture_count:0,
      receipt_accepted_count:0,
      receipt_acceptance_allowed_count:0,
      receipt_acceptance_performed_count:0,
      receipt_acceptance_recorded_count:0,
      receipt_acceptance_persisted_count:0,
      receipt_acceptance_materialized_count:0,
      receipt_acceptance_filesystem_written_count:0,
      operator_approval_required:true,
      operator_approval_recorded:false,
      operator_authority_accepted:false,
      activation_request_required:true,
      activation_request_recorded:false,
      filesystem_persistence_approval_required:true,
      filesystem_persistence_approval_recorded:false,
      receipt_persistence_command_enabled_by_default:false,
      receipt_persistence_command_invoked:false,
      command_invocation_performed_count:0,
      receipt_persistence_execution_performed_count:0,
      workspace_write_performed_count:0,
      evidence_receipt_persisted_count:0,
      filesystem_persistence_allowed:false,
      filesystem_persistence_execution_performed:false,
      workspace_write_performed:false,
      evidence_receipt_persisted:false,
      receipt_persisted:false,
      receipt_accepted:false,
      receipt_acceptance_recorded:false,
      receipt_acceptance_persisted:false,
      receipt_acceptance_materialized:false,
      completion_ack_recorded:false,
      completion_ack_accepted:false,
      ledger_recorded:false,
      ledger_persisted:false,
      ledger_materialized:false,
      ledger_filesystem_written:false,
      index_recorded:false,
      index_persisted:false,
      delivery_recorded:false,
      delivery_persisted:false,
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
      required_receipt_acceptance_fields:$acceptance_fields,
      receipt_acceptance_denial_families:$acceptance_families,
      receipt_acceptance_fixtures:$fixtures,
      source_readiness_families:$source_families,
      denied_by_receipt_acceptance_denial_gate:$denied,
      denied_by_receipt_acceptance_denial_gate_count:($denied | length),
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
        ledger_index_mutated:false,
        delivery_record_persisted:false,
        completion_ack_recorded:false,
        operator_authority_accepted:false,
        receipt_acceptance_recorded:false,
        receipt_acceptance_materialized:false,
        receipt_acceptance_persisted:false,
        receipt_acceptance_filesystem_written:false,
        receipt_persistence_command_invoked:false,
        receipt_persistence_execution:false,
        trusted_record_persisted:false,
        activation_authority_granted:false
      }
    }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_evidence_receipt_acceptance_denial_gate"
  and .receipt_acceptance_denial_gate_ready == true
  and .acceptance_mode == "schema_and_authority_denial_only_no_acceptance_no_persistence"
  and .required_source_count == 4
  and .ready_source_count == 4
  and .activation_blocking_source_count == 4
  and .minimum_required_long_soak_samples >= 24
  and (.source_report_hashes | length) == 4
  and (.source_report_hashes | all(. != ""))
  and .source_required_persistence_execution_field_count == 20
  and .source_recorded_persistence_execution_field_count == 0
  and .source_required_approval_field_count == 12
  and .source_recorded_approval_field_count == 0
  and .source_required_command_field_count == 10
  and .source_recorded_command_field_count == 0
  and .source_command_invocation_attempt_count == 3
  and .source_command_invocation_performed_count == 0
  and .required_receipt_acceptance_field_count == 20
  and .recorded_receipt_acceptance_field_count == 0
  and .required_receipt_acceptance_fixture_count == 4
  and .receipt_acceptance_fixture_count == 4
  and .receipt_acceptance_request_count == 4
  and .blocked_receipt_acceptance_fixture_count == 4
  and .allowed_receipt_acceptance_fixture_count == 0
  and .receipt_accepted_count == 0
  and .receipt_acceptance_allowed_count == 0
  and .receipt_acceptance_performed_count == 0
  and .receipt_acceptance_recorded_count == 0
  and .receipt_acceptance_persisted_count == 0
  and .receipt_acceptance_materialized_count == 0
  and .receipt_acceptance_filesystem_written_count == 0
  and .operator_approval_required == true
  and .operator_approval_recorded == false
  and .operator_authority_accepted == false
  and .activation_request_required == true
  and .activation_request_recorded == false
  and .filesystem_persistence_approval_required == true
  and .filesystem_persistence_approval_recorded == false
  and .receipt_persistence_command_enabled_by_default == false
  and .receipt_persistence_command_invoked == false
  and .command_invocation_performed_count == 0
  and .receipt_persistence_execution_performed_count == 0
  and .workspace_write_performed_count == 0
  and .evidence_receipt_persisted_count == 0
  and .filesystem_persistence_allowed == false
  and .filesystem_persistence_execution_performed == false
  and .workspace_write_performed == false
  and .evidence_receipt_persisted == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .receipt_acceptance_recorded == false
  and .receipt_acceptance_persisted == false
  and .receipt_acceptance_materialized == false
  and .completion_ack_recorded == false
  and .completion_ack_accepted == false
  and .ledger_recorded == false
  and .ledger_persisted == false
  and .ledger_materialized == false
  and .ledger_filesystem_written == false
  and .index_recorded == false
  and .index_persisted == false
  and .delivery_recorded == false
  and .delivery_persisted == false
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
  and (.required_receipt_acceptance_fields | length) == 20
  and (.receipt_acceptance_denial_families | length) == 8
  and (.receipt_acceptance_fixtures | length) == 4
  and (.receipt_acceptance_fixtures | all(.acceptance_requested == true and .receipt_accepted == false and .activation_allowed == false))
  and (.source_readiness_families | length) == 5
  and (.source_readiness_families | all(.ready == true and .blocked == true))
  and .denied_by_receipt_acceptance_denial_gate_count == 29
  and (.denied_by_receipt_acceptance_denial_gate | length) == 29
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
