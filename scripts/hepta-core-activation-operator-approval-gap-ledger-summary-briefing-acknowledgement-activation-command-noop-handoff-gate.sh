#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

ACTIVATION_REQUEST_JSON="$(
  capture_json_report \
    "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-request-denial-matrix-gate" \
    env \
      HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS="${HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS:-0}" \
      HEPTA_LONG_SOAK_OBSERVATION_INTERVAL_SECONDS="${HEPTA_LONG_SOAK_OBSERVATION_INTERVAL_SECONDS:-0}" \
      HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS="${HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS:-0}" \
      scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-request-denial-matrix-gate.sh
)"

activation_command_fixtures_json="$(
  jq -n '
    def blocked_fixture($id; $status; $reason; $extra):
      {
        id: $id,
        activation_command_status: $status,
        source_activation_request_denial_matrix_present: true,
        source_activation_request_denial_matrix_ready: true,
        activation_command_requested: true,
        activation_command_shape_registered: false,
        activation_command_allowed: false,
        activation_command_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_command_dispatch_performed: false,
        activation_command_noop_decision_recorded: false,
        activation_command_noop_decision_persisted: false,
        activation_command_noop_decision_accepted: false,
        activation_command_handoff_recorded: false,
        activation_command_handoff_persisted: false,
        activation_command_handoff_accepted: false,
        activation_command_handoff_materialized: false,
        activation_command_handoff_filesystem_written: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_command_result_receipt_exported: false,
        activation_command_result_receipt_query_registered: false,
        activation_command_result_receipt_observability_recorded: false,
        activation_request_allowed: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_materialized: false,
        activation_request_filesystem_written: false,
        activation_request_delivered: false,
        activation_request_executed: false,
        activation_allowed: false,
        activation_performed: false,
        terminal_closure_recorded: false,
        terminal_closure_accepted: false,
        terminal_closure_final_state_promoted: false,
        terminal_closure_completion_promoted: false,
        operator_approval_recorded: false,
        operator_approval_accepted: false,
        trusted_record_accepted: false,
        fresh_evidence_accepted: false,
        receipt_recorded: false,
        receipt_persisted: false,
        receipt_accepted: false,
        ledger_recorded: false,
        index_delivered: false,
        completion_ack_recorded: false,
        completion_ack_persisted: false,
        completion_ack_accepted: false,
        runtime_attachment_performed: false,
        live_context_attached: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        telegram_send_performed: false,
        channel_send_performed: false,
        external_send_performed: false,
        public_release_claimed: false,
        release_artifact_written: false,
        install_executed: false,
        launchd_mutated: false,
        service_restarted: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        credential_read: false,
        secret_value_read: false,
        activation_command_noop_confirmed: true,
        reason: $reason
      } + $extra;
    [
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-missing-source";
        "blocked_noop";
        "source_activation_request_denial_matrix_report_required";
        {
          source_activation_request_denial_matrix_present: false,
          source_activation_request_denial_matrix_ready: false
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-handoff-request";
        "blocked_command_noop";
        "activation_command_handoff_shape_denied";
        {}
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-registration-enable-request";
        "blocked_register_enable_noop";
        "activation_command_registration_enablement_denied";
        {
          activation_command_registration_requested: true,
          activation_command_enable_requested: true
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-direct-invocation-request";
        "blocked_invocation_noop";
        "activation_command_invocation_denied";
        {activation_command_invocation_requested: true}
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-dispatch-request";
        "blocked_dispatch_noop";
        "activation_command_dispatch_denied";
        {
          activation_command_dispatch_requested: true,
          runtime_dispatch_requested: true
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-terminal-closure-request";
        "blocked_terminal_closure_noop";
        "terminal_closure_command_promotion_denied";
        {
          terminal_closure_record_requested: true,
          terminal_closure_acceptance_requested: true,
          final_state_promotion_requested: true,
          completion_promotion_requested: true
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-runtime-provider-request";
        "blocked_runtime_provider_noop";
        "runtime_context_provider_model_command_denied";
        {
          runtime_attachment_requested: true,
          live_context_attachment_requested: true,
          context_injection_requested: true,
          provider_invocation_requested: true,
          model_invocation_requested: true
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-receipt-ledger-request";
        "blocked_receipt_ledger_noop";
        "receipt_ledger_command_result_denied";
        {
          command_result_receipt_record_requested: true,
          command_result_receipt_persist_requested: true,
          receipt_record_requested: true,
          receipt_persist_requested: true,
          ledger_record_requested: true,
          completion_ack_record_requested: true
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-public-install-upstream-request";
        "blocked_external_noop";
        "public_install_restart_upstream_command_denied";
        {
          external_send_requested: true,
          public_claim_requested: true,
          release_artifact_requested: true,
          install_requested: true,
          service_restart_requested: true,
          active_binary_mutation_requested: true,
          upstream_merge_requested: true
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-secret-request";
        "blocked_secret_noop";
        "credential_secret_command_denied";
        {
          credential_read_requested: true,
          secret_value_read_requested: true
        }
      )
    ]
  '
)"

activation_request_report_sha256="$(sha256_text "$ACTIVATION_REQUEST_JSON")"
activation_command_fixtures_sha256="$(sha256_text "$activation_command_fixtures_json")"
activation_command_contract_hash_sha256="$(
  sha256_text "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-noop-handoff:$activation_request_report_sha256:$activation_command_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
activation_command_policy_hash_sha256="$(
  sha256_text "operator-gap-ledger-summary-briefing-acknowledgement-activation-command-noop-handoff:report-only:no-command-register:no-command-enable:no-command-invoke:no-dispatch:no-handoff-persist:no-terminal-closure:no-provider:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "activation-command=false;activation-request=false;terminal-closure=false;activation=false;provider=false;release=false;install=false;upstream=false;secret=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$ACTIVATION_REQUEST_JSON" \
  --argjson fixtures "$activation_command_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_gate"
    and $source.schema_version == "core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_v1"
    and $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_ready == true
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_status == "blocked"
    and $source.source_operator_approval_gap_ledger_summary_briefing_acknowledgement_ready == true
    and $source.source_operator_approval_gap_ledger_summary_briefing_acknowledgement_status == "blocked"
    and $source.source_missing_operator_approval_gap_ledger_item_count == 16
    and $source.source_denied_by_operator_approval_gap_ledger_summary_briefing_count == 81
    and $source.source_acknowledgement_denied_count == 99
    and $source.activation_request_surface_count == 12
    and $source.activation_request_surface_ready_count == 12
    and $source.activation_request_side_effect_free_surface_count == 12
    and $source.activation_request_fixture_count == 10
    and $source.blocked_activation_request_fixture_count == 10
    and $source.noop_activation_request_fixture_count == 10
    and $source.allowed_activation_request_fixture_count == 0
    and $source.accepted_activation_request_fixture_count == 0
    and $source.activation_request_denied_count == 10
    and $source.activation_request_performed_count == 0
    and $source.activation_execution_performed_count == 0
    and $source.activation_request_allowed == false
    and $source.activation_request_accepted == false
    and $source.activation_request_recorded == false
    and $source.activation_request_persisted == false
    and $source.activation_request_materialized == false
    and $source.activation_request_filesystem_written == false
    and $source.activation_request_delivered == false
    and $source.activation_request_executed == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.operator_approval_recorded == false
    and $source.operator_approval_accepted == false
    and $source.trusted_record_accepted == false
    and $source.fresh_evidence_accepted == false
    and $source.receipt_recorded == false
    and $source.receipt_persisted == false
    and $source.receipt_accepted == false
    and $source.ledger_recorded == false
    and $source.completion_ack_accepted == false
    and $source.terminal_closure_recorded == false
    and $source.terminal_closure_accepted == false
    and $source.terminal_closure_final_state_promoted == false
    and $source.terminal_closure_completion_promoted == false
    and $source.runtime_attachment_performed == false
    and $source.live_context_attached == false
    and $source.context_injection_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.telegram_send_performed == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.public_release_claimed == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.launchd_mutated == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and $source.upstream_fetch_performed == false
    and $source.upstream_merge_performed == false
    and $source.credential_read == false
    and $source.secret_value_read == false
    and ($source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix | length) == 117
    and $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_count == 117
    and ($source.allowed_next_actions | any(.action == "stage_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff" and .status == "allowed_report_only_next_slice" and .accepts_activation_request == false and .records_activation_request == false and .executes_activation == false and .records_terminal_closure == false and .writes_release_artifact == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.activation_command_status == "blocked_noop" or .activation_command_status == "blocked_command_noop" or .activation_command_status == "blocked_register_enable_noop" or .activation_command_status == "blocked_invocation_noop" or .activation_command_status == "blocked_dispatch_noop" or .activation_command_status == "blocked_terminal_closure_noop" or .activation_command_status == "blocked_runtime_provider_noop" or .activation_command_status == "blocked_receipt_ledger_noop" or .activation_command_status == "blocked_external_noop" or .activation_command_status == "blocked_secret_noop")
      and .activation_command_allowed == false
      and .activation_command_accepted == false
      and .activation_command_enabled == false
      and .activation_command_invoked == false
      and .activation_command_dispatched == false
      and .activation_command_dispatch_performed == false
      and .activation_command_noop_decision_recorded == false
      and .activation_command_noop_decision_persisted == false
      and .activation_command_handoff_recorded == false
      and .activation_command_handoff_persisted == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_request_accepted == false
      and .activation_request_recorded == false
      and .activation_request_persisted == false
      and .activation_request_executed == false
      and .activation_performed == false
      and .terminal_closure_recorded == false
      and .terminal_closure_accepted == false
      and .terminal_closure_final_state_promoted == false
      and .terminal_closure_completion_promoted == false
      and .operator_approval_recorded == false
      and .operator_approval_accepted == false
      and .trusted_record_accepted == false
      and .fresh_evidence_accepted == false
      and .receipt_recorded == false
      and .receipt_persisted == false
      and .receipt_accepted == false
      and .ledger_recorded == false
      and .completion_ack_accepted == false
      and .runtime_attachment_performed == false
      and .live_context_attached == false
      and .context_injection_performed == false
      and .provider_invoked == false
      and .model_invoked == false
      and .telegram_send_performed == false
      and .channel_send_performed == false
      and .external_send_performed == false
      and .public_release_claimed == false
      and .release_artifact_written == false
      and .install_executed == false
      and .service_restarted == false
      and .active_binary_mutated == false
      and .upstream_fetch_performed == false
      and .upstream_merge_performed == false
      and .credential_read == false
      and .secret_value_read == false
      and .activation_command_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff_gate" \
    --arg activation_request_report_sha256 "$activation_request_report_sha256" \
    --arg activation_command_fixtures_sha256 "$activation_command_fixtures_sha256" \
    --arg activation_command_contract_hash_sha256 "$activation_command_contract_hash_sha256" \
    --arg activation_command_policy_hash_sha256 "$activation_command_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ACTIVATION_REQUEST_JSON" \
    --argjson fixtures "$activation_command_fixtures_json" \
    '{
      product: $product,
      runtime: $runtime,
      status: "ready",
      base_url: $base_url,
      gate: $gate,
      schema_version: "core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff_v1",
      mode: "summary_briefing_acknowledgement_activation_command_noop_handoff_no_register_no_enable_no_invoke_no_dispatch",
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_gate: $source.gate,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_ready: $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_ready,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_status: $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_status,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_report_sha256: $activation_request_report_sha256,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_gate: $source.source_operator_approval_gap_ledger_summary_briefing_acknowledgement_gate,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_status: $source.source_operator_approval_gap_ledger_summary_briefing_acknowledgement_status,
      source_operator_approval_gap_ledger_summary_briefing_gate: $source.source_operator_approval_gap_ledger_summary_briefing_gate,
      source_operator_approval_gap_ledger_summary_briefing_status: $source.source_operator_approval_gap_ledger_summary_briefing_status,
      source_operator_approval_gap_ledger_item_count: $source.source_operator_approval_gap_ledger_item_count,
      source_missing_operator_approval_gap_ledger_item_count: $source.source_missing_operator_approval_gap_ledger_item_count,
      source_denied_by_operator_approval_gap_ledger_summary_briefing_count: $source.source_denied_by_operator_approval_gap_ledger_summary_briefing_count,
      source_acknowledgement_denied_count: $source.source_acknowledgement_denied_count,
      activation_command_fixtures_sha256: $activation_command_fixtures_sha256,
      activation_command_contract_hash_sha256: $activation_command_contract_hash_sha256,
      activation_command_policy_hash_sha256: $activation_command_policy_hash_sha256,
      side_effect_hash_sha256: $side_effect_hash_sha256,
      minimum_required_samples: $min_long_soak_samples,
      core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff_ready: true,
      operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff_status: "blocked",
      activation_request_denial_matrix_ready: $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_ready,
      activation_request_denial_matrix_status: $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_status,
      activation_request_surface_count: $source.activation_request_surface_count,
      activation_request_surface_ready_count: $source.activation_request_surface_ready_count,
      activation_request_side_effect_free_surface_count: $source.activation_request_side_effect_free_surface_count,
      activation_request_fixture_count: $source.activation_request_fixture_count,
      blocked_activation_request_fixture_count: $source.blocked_activation_request_fixture_count,
      noop_activation_request_fixture_count: $source.noop_activation_request_fixture_count,
      allowed_activation_request_fixture_count: $source.allowed_activation_request_fixture_count,
      accepted_activation_request_fixture_count: $source.accepted_activation_request_fixture_count,
      activation_request_denied_count: $source.activation_request_denied_count,
      activation_request_performed_count: $source.activation_request_performed_count,
      activation_execution_performed_count: $source.activation_execution_performed_count,
      activation_command_surface_count: 13,
      activation_command_surface_ready_count: 13,
      activation_command_side_effect_free_surface_count: 13,
      activation_command_fixture_count: ($fixtures | length),
      blocked_activation_command_fixture_count: ($fixtures | length),
      noop_activation_command_fixture_count: ($fixtures | length),
      allowed_activation_command_fixture_count: 0,
      accepted_activation_command_fixture_count: 0,
      activation_command_denied_count: 10,
      activation_command_performed_count: 0,
      activation_command_dispatch_performed_count: 0,
      activation_command_shape_registered: false,
      activation_command_allowed: false,
      activation_command_accepted: false,
      activation_command_enabled: false,
      activation_command_invoked: false,
      activation_command_dispatched: false,
      activation_command_noop_decision_recorded: false,
      activation_command_noop_decision_persisted: false,
      activation_command_noop_decision_accepted: false,
      activation_command_handoff_recorded: false,
      activation_command_handoff_persisted: false,
      activation_command_handoff_accepted: false,
      activation_command_handoff_materialized: false,
      activation_command_handoff_filesystem_written: false,
      activation_command_result_receipt_recorded: false,
      activation_command_result_receipt_persisted: false,
      activation_command_result_receipt_accepted: false,
      activation_command_result_receipt_exported: false,
      activation_command_result_receipt_query_registered: false,
      activation_command_result_receipt_observability_recorded: false,
      activation_request_allowed: false,
      activation_request_accepted: false,
      activation_request_recorded: false,
      activation_request_persisted: false,
      activation_request_materialized: false,
      activation_request_filesystem_written: false,
      activation_request_delivered: false,
      activation_request_executed: false,
      activation_allowed: false,
      activation_performed: false,
      terminal_closure_recorded: false,
      terminal_closure_accepted: false,
      terminal_closure_final_state_promoted: false,
      terminal_closure_completion_promoted: false,
      operator_approval_recorded: false,
      operator_approval_accepted: false,
      trusted_record_accepted: false,
      fresh_evidence_accepted: false,
      receipt_recorded: false,
      receipt_persisted: false,
      receipt_accepted: false,
      ledger_recorded: false,
      index_delivered: false,
      completion_ack_recorded: false,
      completion_ack_persisted: false,
      completion_ack_accepted: false,
      runtime_attachment_performed: false,
      live_context_attached: false,
      context_injection_performed: false,
      provider_invoked: false,
      model_invoked: false,
      telegram_send_performed: false,
      channel_send_performed: false,
      external_send_performed: false,
      public_release_claimed: false,
      release_artifact_written: false,
      install_executed: false,
      launchd_mutated: false,
      service_restarted: false,
      active_binary_mutated: false,
      upstream_fetch_performed: false,
      upstream_merge_performed: false,
      credential_read: false,
      secret_value_read: false,
      activation_command_surfaces: [
        "source_activation_request_denial_matrix_report_required",
        "activation_command_handoff_shape_denied",
        "activation_command_registration_denied",
        "activation_command_enablement_denied",
        "activation_command_invocation_denied",
        "activation_command_dispatch_denied",
        "activation_command_noop_decision_record_persist_denied",
        "activation_command_handoff_record_persist_denied",
        "activation_command_result_receipt_record_persist_denied",
        "terminal_closure_command_promotion_denied",
        "runtime_context_provider_model_command_denied",
        "receipt_ledger_completion_ack_command_denied",
        "external_public_install_restart_upstream_secret_command_denied"
      ],
      activation_command_fixtures: $fixtures,
      denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff: (
        $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix + [
          "source_activation_request_denial_matrix_report_required",
          "activation_command_shape_registration_denied",
          "activation_command_acceptance_denied",
          "activation_command_enablement_denied",
          "activation_command_invocation_denied",
          "activation_command_dispatch_denied",
          "activation_command_dispatch_execution_denied",
          "activation_command_noop_decision_recording_denied",
          "activation_command_noop_decision_persistence_denied",
          "activation_command_noop_decision_acceptance_denied",
          "activation_command_handoff_recording_denied",
          "activation_command_handoff_persistence_denied",
          "activation_command_handoff_acceptance_denied",
          "activation_command_handoff_materialization_denied",
          "activation_command_handoff_filesystem_write_denied",
          "activation_command_result_receipt_recording_denied",
          "activation_command_result_receipt_persistence_denied",
          "activation_command_result_receipt_acceptance_denied",
          "activation_command_result_receipt_export_query_observability_denied",
          "activation_request_acceptance_denied",
          "activation_request_execution_denied",
          "terminal_closure_command_promotion_denied",
          "runtime_attachment_denied",
          "live_context_attachment_denied",
          "context_injection_denied",
          "provider_model_invocation_denied",
          "receipt_ledger_completion_ack_command_denied",
          "external_public_install_restart_upstream_denied",
          "credential_secret_command_denied",
          "activation_command_cannot_promote_acknowledgement_authority"
        ]
      ),
      denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff_count: (
        ($source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix | length) + 30
      ),
      inherited_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_count: (
        $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix | length
      ),
      allowed_next_actions: [
        {
          action: "review_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff",
          status: "allowed_report_only",
          registers_command: false,
          enables_command: false,
          invokes_command: false,
          dispatches_command: false,
          persists_handoff: false,
          records_terminal_closure: false,
          invokes_model: false
        },
        {
          action: "stage_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence",
          status: "allowed_report_only_next_slice",
          records_command_result: false,
          persists_command_result: false,
          exports_receipt: false,
          registers_observability: false,
          mutates_runtime: false,
          invokes_model: false
        },
        {
          action: "run_full_light_preflight",
          status: "allowed_verification_only",
          mutates_runtime: false,
          dispatches_command: false,
          attaches_live_context: false,
          invokes_model: false,
          writes_kg: false
        }
      ],
      source_activation_request_denial_matrix_report_required: true,
      activation_command_registration_forbidden: true,
      activation_command_enablement_forbidden: true,
      activation_command_invocation_forbidden: true,
      activation_command_dispatch_forbidden: true,
      activation_command_handoff_persistence_forbidden: true,
      activation_command_result_receipt_persistence_forbidden: true,
      activation_request_acceptance_forbidden: true,
      activation_request_execution_forbidden: true,
      terminal_closure_promotion_forbidden: true,
      activation_forbidden: true,
      runtime_context_attachment_forbidden: true,
      provider_model_invocation_forbidden: true,
      public_release_claim_forbidden: true,
      release_artifact_write_forbidden: true,
      install_restart_forbidden: true,
      upstream_fetch_merge_forbidden: true,
      credential_secret_read_forbidden: true,
      side_effects: {
        workspace_written: false,
        filesystem_written: false,
        activation_command_shape_registered: false,
        activation_command_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_command_dispatch_performed: false,
        activation_command_noop_decision_recorded: false,
        activation_command_noop_decision_persisted: false,
        activation_command_noop_decision_accepted: false,
        activation_command_handoff_recorded: false,
        activation_command_handoff_persisted: false,
        activation_command_handoff_accepted: false,
        activation_command_handoff_materialized: false,
        activation_command_handoff_filesystem_written: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_command_result_receipt_exported: false,
        activation_command_result_receipt_query_registered: false,
        activation_command_result_receipt_observability_recorded: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_materialized: false,
        activation_request_filesystem_written: false,
        activation_request_delivered: false,
        activation_request_executed: false,
        activation_performed: false,
        terminal_closure_recorded: false,
        terminal_closure_accepted: false,
        terminal_closure_final_state_promoted: false,
        terminal_closure_completion_promoted: false,
        operator_approval_recorded: false,
        operator_approval_accepted: false,
        trusted_record_accepted: false,
        fresh_evidence_accepted: false,
        receipt_recorded: false,
        receipt_persisted: false,
        receipt_accepted: false,
        ledger_recorded: false,
        index_delivered: false,
        completion_ack_recorded: false,
        completion_ack_accepted: false,
        runtime_attachment_performed: false,
        live_context_attached: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        telegram_send_performed: false,
        channel_send_performed: false,
        external_send_performed: false,
        public_release_claimed: false,
        release_artifact_written: false,
        install_executed: false,
        launchd_mutated: false,
        service_restarted: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        credential_read: false,
        secret_value_read: false
      }
    }'
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff_gate"
  and .schema_version == "core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff_v1"
  and .core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff_ready == true
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff_status == "blocked"
  and .activation_request_denial_matrix_ready == true
  and .activation_request_denial_matrix_status == "blocked"
  and .activation_request_surface_count == 12
  and .activation_request_surface_ready_count == 12
  and .activation_request_side_effect_free_surface_count == 12
  and .activation_request_fixture_count == 10
  and .blocked_activation_request_fixture_count == 10
  and .noop_activation_request_fixture_count == 10
  and .allowed_activation_request_fixture_count == 0
  and .accepted_activation_request_fixture_count == 0
  and .activation_request_denied_count == 10
  and .activation_request_performed_count == 0
  and .activation_execution_performed_count == 0
  and .activation_command_surface_count == 13
  and .activation_command_surface_ready_count == 13
  and .activation_command_side_effect_free_surface_count == 13
  and .activation_command_fixture_count == 10
  and .blocked_activation_command_fixture_count == 10
  and .noop_activation_command_fixture_count == 10
  and .allowed_activation_command_fixture_count == 0
  and .accepted_activation_command_fixture_count == 0
  and .activation_command_denied_count == 10
  and .activation_command_performed_count == 0
  and .activation_command_dispatch_performed_count == 0
  and .activation_command_shape_registered == false
  and .activation_command_allowed == false
  and .activation_command_accepted == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_command_noop_decision_recorded == false
  and .activation_command_noop_decision_persisted == false
  and .activation_command_noop_decision_accepted == false
  and .activation_command_handoff_recorded == false
  and .activation_command_handoff_persisted == false
  and .activation_command_handoff_accepted == false
  and .activation_command_handoff_materialized == false
  and .activation_command_handoff_filesystem_written == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_result_receipt_exported == false
  and .activation_command_result_receipt_query_registered == false
  and .activation_command_result_receipt_observability_recorded == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_persisted == false
  and .activation_request_executed == false
  and .activation_allowed == false
  and .activation_performed == false
  and .terminal_closure_recorded == false
  and .terminal_closure_accepted == false
  and .terminal_closure_final_state_promoted == false
  and .terminal_closure_completion_promoted == false
  and .operator_approval_recorded == false
  and .operator_approval_accepted == false
  and .trusted_record_accepted == false
  and .fresh_evidence_accepted == false
  and .receipt_recorded == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .ledger_recorded == false
  and .completion_ack_accepted == false
  and .runtime_attachment_performed == false
  and .live_context_attached == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .public_release_claimed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .upstream_fetch_performed == false
  and .upstream_merge_performed == false
  and .credential_read == false
  and .secret_value_read == false
  and (.activation_command_surfaces | length) == 13
  and (.activation_command_fixtures | length) == 10
  and (.activation_command_fixtures | all(
    (.activation_command_status == "blocked_noop" or .activation_command_status == "blocked_command_noop" or .activation_command_status == "blocked_register_enable_noop" or .activation_command_status == "blocked_invocation_noop" or .activation_command_status == "blocked_dispatch_noop" or .activation_command_status == "blocked_terminal_closure_noop" or .activation_command_status == "blocked_runtime_provider_noop" or .activation_command_status == "blocked_receipt_ledger_noop" or .activation_command_status == "blocked_external_noop" or .activation_command_status == "blocked_secret_noop")
    and .activation_command_allowed == false
    and .activation_command_accepted == false
    and .activation_command_enabled == false
    and .activation_command_invoked == false
    and .activation_command_dispatched == false
    and .activation_command_dispatch_performed == false
    and .activation_command_noop_decision_recorded == false
    and .activation_command_noop_decision_persisted == false
    and .activation_command_handoff_recorded == false
    and .activation_command_handoff_persisted == false
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_request_accepted == false
    and .activation_request_recorded == false
    and .activation_request_persisted == false
    and .activation_request_executed == false
    and .activation_performed == false
    and .terminal_closure_recorded == false
    and .terminal_closure_accepted == false
    and .terminal_closure_final_state_promoted == false
    and .terminal_closure_completion_promoted == false
    and .operator_approval_recorded == false
    and .operator_approval_accepted == false
    and .trusted_record_accepted == false
    and .fresh_evidence_accepted == false
    and .receipt_recorded == false
    and .receipt_persisted == false
    and .receipt_accepted == false
    and .ledger_recorded == false
    and .completion_ack_accepted == false
    and .runtime_attachment_performed == false
    and .live_context_attached == false
    and .context_injection_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .telegram_send_performed == false
    and .external_send_performed == false
    and .public_release_claimed == false
    and .release_artifact_written == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .upstream_fetch_performed == false
    and .upstream_merge_performed == false
    and .credential_read == false
    and .secret_value_read == false
    and .activation_command_noop_confirmed == true
  ))
  and .denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff_count == 147
  and (.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff | length) == 147
  and .inherited_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_count == 117
  and (.allowed_next_actions | any(.action == "stage_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence" and .status == "allowed_report_only_next_slice" and .records_command_result == false and .persists_command_result == false and .exports_receipt == false and .registers_observability == false and .mutates_runtime == false and .invokes_model == false))
  and .source_activation_request_denial_matrix_report_required == true
  and .activation_command_registration_forbidden == true
  and .activation_command_enablement_forbidden == true
  and .activation_command_invocation_forbidden == true
  and .activation_command_dispatch_forbidden == true
  and .activation_command_handoff_persistence_forbidden == true
  and .activation_command_result_receipt_persistence_forbidden == true
  and .activation_request_acceptance_forbidden == true
  and .activation_request_execution_forbidden == true
  and .terminal_closure_promotion_forbidden == true
  and .activation_forbidden == true
  and .runtime_context_attachment_forbidden == true
  and .provider_model_invocation_forbidden == true
  and .public_release_claim_forbidden == true
  and .release_artifact_write_forbidden == true
  and .install_restart_forbidden == true
  and .upstream_fetch_merge_forbidden == true
  and .credential_secret_read_forbidden == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report" | jq .
echo "Hepta core activation operator approval gap ledger summary briefing acknowledgement activation command no-op handoff gate passed"
