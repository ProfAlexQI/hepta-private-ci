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

ORDERING_MONOTONICITY_JSON="$(
  capture_json_report \
    "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-gate" \
    env \
      HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS="${HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS:-0}" \
      HEPTA_LONG_SOAK_OBSERVATION_INTERVAL_SECONDS="${HEPTA_LONG_SOAK_OBSERVATION_INTERVAL_SECONDS:-0}" \
      HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS="${HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS:-0}" \
      scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-ordering-monotonicity-denial-gate.sh
)"

cancellation_supersession_fixtures_json="$(
  jq -n '
    def cancellation_supersession_fixture($id; $status; $reason; $extra):
      {
        id: $id,
        cancellation_requested: true,
        supersession_requested: false,
        cancellation_supersession_status: $status,
        source_ordering_monotonicity_present: true,
        source_ordering_monotonicity_ready: true,
        activation_command_result_receipt_cancellation_allowed: false,
        activation_command_result_receipt_cancellation_recorded: false,
        activation_command_result_receipt_cancellation_persisted: false,
        activation_command_result_receipt_cancellation_materialized: false,
        activation_command_result_receipt_cancellation_filesystem_written: false,
        activation_command_result_receipt_cancellation_request_accepted: false,
        activation_command_result_receipt_supersession_allowed: false,
        activation_command_result_receipt_supersession_recorded: false,
        activation_command_result_receipt_supersession_persisted: false,
        activation_command_result_receipt_supersession_materialized: false,
        activation_command_result_receipt_supersession_filesystem_written: false,
        activation_command_result_receipt_supersession_request_accepted: false,
        activation_command_result_receipt_replacement_receipt_accepted: false,
        activation_command_result_receipt_replacement_receipt_recorded: false,
        activation_command_result_receipt_replacement_receipt_persisted: false,
        activation_command_result_receipt_replacement_hash_accepted: false,
        activation_command_result_receipt_tombstone_recorded: false,
        activation_command_result_receipt_tombstone_persisted: false,
        activation_command_result_receipt_delete_marker_recorded: false,
        activation_command_result_receipt_ack_cancellation_accepted: false,
        activation_command_result_receipt_ledger_cancellation_accepted: false,
        activation_command_result_receipt_index_cancellation_accepted: false,
        activation_command_result_receipt_delivery_cancellation_accepted: false,
        activation_command_result_receipt_export_cancellation_accepted: false,
        activation_command_result_receipt_query_cancellation_accepted: false,
        activation_command_result_receipt_observability_cancellation_accepted: false,
        activation_command_result_receipt_ordering_allowed: false,
        activation_command_result_receipt_ordering_recorded: false,
        activation_command_result_receipt_ordering_persisted: false,
        activation_command_result_receipt_sequence_cursor_accepted: false,
        activation_command_result_receipt_sequence_cursor_recorded: false,
        activation_command_result_receipt_sequence_cursor_persisted: false,
        activation_command_result_receipt_monotonicity_state_recorded: false,
        activation_command_result_receipt_monotonicity_state_persisted: false,
        activation_command_result_receipt_latest_wins_overwrite_accepted: false,
        activation_command_result_receipt_same_sequence_hash_override_accepted: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_command_result_receipt_materialized: false,
        activation_command_result_receipt_filesystem_written: false,
        activation_command_result_receipt_ledger_written: false,
        activation_command_result_receipt_indexed: false,
        activation_command_result_receipt_enqueued: false,
        activation_command_result_receipt_delivered: false,
        activation_command_result_receipt_exported: false,
        activation_command_result_receipt_query_registered: false,
        activation_command_result_receipt_observability_recorded: false,
        activation_command_completion_ack_recorded: false,
        activation_command_completion_ack_persisted: false,
        activation_command_completion_ack_accepted: false,
        activation_command_completion_ack_delivered: false,
        operator_approval_from_cancellation_accepted: false,
        operator_approval_from_supersession_accepted: false,
        activation_from_cancellation_allowed: false,
        activation_from_supersession_allowed: false,
        activation_from_ordering_allowed: false,
        activation_from_replay_allowed: false,
        activation_from_receipt_allowed: false,
        terminal_closure_from_cancellation_recorded: false,
        terminal_closure_from_supersession_recorded: false,
        terminal_closure_from_cancellation_accepted: false,
        terminal_closure_from_supersession_accepted: false,
        activation_command_shape_registered: false,
        activation_command_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_command_dispatch_performed: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_executed: false,
        activation_allowed: false,
        activation_performed: false,
        terminal_closure_recorded: false,
        terminal_closure_accepted: false,
        runtime_attachment_performed: false,
        live_context_attached: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        usage_recorded: false,
        memory_store_write_performed: false,
        memory_store_mutated: false,
        live_kg_write_performed: false,
        readback_evidence_recorded: false,
        readback_evidence_persisted: false,
        router_handoff_recorded: false,
        router_handoff_persisted: false,
        rollback_executed: false,
        telegram_send_performed: false,
        channel_send_performed: false,
        external_send_performed: false,
        public_release_claimed: false,
        public_ga_claimed: false,
        release_artifact_written: false,
        install_executed: false,
        launchd_mutated: false,
        service_restarted: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        credential_read: false,
        secret_value_read: false,
        raw_payload_plaintext_recorded: false,
        raw_payload_plaintext_persisted: false,
        receipt_noop_confirmed: true,
        reason: $reason
      } + $extra;
    [
      cancellation_supersession_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-cancellation-missing-source-ordering-report";
        "blocked_noop";
        "source_ordering_monotonicity_report_required";
        {
          source_ordering_monotonicity_present: false,
          source_ordering_monotonicity_ready: false
        }
      ),
      cancellation_supersession_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-cancel-blocked-noop";
        "blocked_cancellation_noop";
        "cancellation_of_blocked_noop_receipt_denied";
        {
          cancellation_request_shape: "cancel_blocked_noop_receipt"
        }
      ),
      cancellation_supersession_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-supersede-with-completed";
        "blocked_supersession_noop";
        "supersession_of_blocked_noop_with_completed_denied";
        {
          supersession_requested: true,
          cancellation_requested: false,
          requested_replacement_status: "completed"
        }
      ),
      cancellation_supersession_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-replacement-hash";
        "blocked_supersession_noop";
        "replacement_hash_identity_attempt_denied";
        {
          supersession_requested: true,
          cancellation_requested: false,
          replacement_hash_requested: true,
          requested_hash_relation: "different_hash_for_same_receipt_identity"
        }
      ),
      cancellation_supersession_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-tombstone-delete-marker";
        "blocked_cancellation_noop";
        "tombstone_or_delete_marker_denied";
        {
          tombstone_requested: true,
          delete_marker_requested: true
        }
      ),
      cancellation_supersession_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-completion-ack-cancel";
        "blocked_cancellation_noop";
        "completion_ack_cancellation_denied";
        {
          completion_ack_cancellation_requested: true,
          ack_cancellation_requested: true
        }
      ),
      cancellation_supersession_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-ledger-index-delivery-export-cancel";
        "blocked_cancellation_noop";
        "ledger_index_delivery_export_cancellation_denied";
        {
          ledger_cancellation_requested: true,
          index_cancellation_requested: true,
          delivery_cancellation_requested: true,
          export_cancellation_requested: true,
          query_cancellation_requested: true,
          observability_cancellation_requested: true
        }
      ),
      cancellation_supersession_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-runtime-provider-model-supersede";
        "blocked_supersession_noop";
        "runtime_provider_model_supersession_denied";
        {
          supersession_requested: true,
          cancellation_requested: false,
          runtime_supersession_requested: true,
          live_context_supersession_requested: true,
          provider_supersession_requested: true,
          model_supersession_requested: true,
          usage_supersession_requested: true
        }
      ),
      cancellation_supersession_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-memory-kg-rollback-secret-supersede";
        "blocked_supersession_noop";
        "memory_kg_rollback_secret_supersession_denied";
        {
          supersession_requested: true,
          cancellation_requested: false,
          memory_store_supersession_requested: true,
          live_kg_supersession_requested: true,
          rollback_supersession_requested: true,
          upstream_supersession_requested: true,
          credential_secret_supersession_requested: true
        }
      ),
      cancellation_supersession_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-external-public-install-supersede";
        "blocked_supersession_noop";
        "external_public_install_restart_active_binary_supersession_denied";
        {
          supersession_requested: true,
          cancellation_requested: false,
          external_send_supersession_requested: true,
          public_claim_supersession_requested: true,
          release_artifact_supersession_requested: true,
          install_supersession_requested: true,
          service_restart_supersession_requested: true,
          active_binary_mutation_supersession_requested: true
        }
      )
    ]
  '
)"

ordering_monotonicity_report_sha256="$(sha256_text "$ORDERING_MONOTONICITY_JSON")"
cancellation_supersession_fixtures_sha256="$(sha256_text "$cancellation_supersession_fixtures_json")"
cancellation_supersession_contract_hash_sha256="$(
  sha256_text "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial:$ordering_monotonicity_report_sha256:$cancellation_supersession_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
cancellation_supersession_policy_hash_sha256="$(
  sha256_text "operator-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial:no-cancel:no-supersede:no-replacement:no-tombstone:no-terminal:no-provider:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "cancellation=false;supersession=false;replacement=false;tombstone=false;record=false;persist=false;activation=false;terminal-closure=false;provider=false;model=false;memory=false;kg=false;release=false;install=false;restart=false;active_binary=false;upstream=false;secret=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$ORDERING_MONOTONICITY_JSON" \
  --argjson fixtures "$cancellation_supersession_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_gate"
    and $source.schema_version == "core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_v1"
    and $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status == "blocked"
    and $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready == true
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status == "blocked"
    and $source.source_activation_command_result_receipt_replay_idempotency_denied_count == 212
    and $source.activation_command_result_receipt_ordering_monotonicity_surface_count == 14
    and $source.activation_command_result_receipt_ordering_monotonicity_surface_ready_count == 14
    and $source.activation_command_result_receipt_ordering_monotonicity_side_effect_free_surface_count == 14
    and $source.activation_command_result_receipt_ordering_monotonicity_fixture_count == 10
    and $source.blocked_activation_command_result_receipt_ordering_monotonicity_fixture_count == 10
    and $source.noop_activation_command_result_receipt_ordering_monotonicity_fixture_count == 10
    and $source.allowed_activation_command_result_receipt_ordering_monotonicity_fixture_count == 0
    and $source.accepted_activation_command_result_receipt_ordering_monotonicity_fixture_count == 0
    and $source.activation_command_result_receipt_ordering_allowed == false
    and $source.activation_command_result_receipt_ordering_recorded == false
    and $source.activation_command_result_receipt_ordering_persisted == false
    and $source.activation_command_result_receipt_sequence_cursor_accepted == false
    and $source.activation_command_result_receipt_sequence_cursor_recorded == false
    and $source.activation_command_result_receipt_sequence_cursor_persisted == false
    and $source.activation_command_result_receipt_monotonicity_state_recorded == false
    and $source.activation_command_result_receipt_monotonicity_state_persisted == false
    and $source.activation_command_result_receipt_latest_wins_overwrite_accepted == false
    and $source.activation_command_result_receipt_same_sequence_hash_override_accepted == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_completion_ack_accepted == false
    and $source.operator_approval_from_ordering_accepted == false
    and $source.activation_from_ordering_allowed == false
    and $source.activation_from_receipt_allowed == false
    and $source.terminal_closure_from_ordering_recorded == false
    and $source.terminal_closure_from_ordering_accepted == false
    and $source.activation_command_enabled == false
    and $source.activation_command_invoked == false
    and $source.activation_command_dispatched == false
    and $source.activation_request_accepted == false
    and $source.activation_request_recorded == false
    and $source.activation_request_persisted == false
    and $source.activation_request_executed == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.terminal_closure_recorded == false
    and $source.terminal_closure_accepted == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
    and $source.live_kg_write_performed == false
    and $source.external_send_performed == false
    and $source.public_release_claimed == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and $source.upstream_fetch_performed == false
    and $source.upstream_merge_performed == false
    and $source.credential_read == false
    and $source.secret_value_read == false
    and $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_count == 242
    and ($source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial | length) == 242
    and ($source.allowed_next_actions | any(.action == "stage_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial" and .status == "allowed_report_only_next_slice" and .accepts_cancellation == false and .accepts_supersession == false and .persists_replacement_receipt == false and .mutates_runtime == false and .invokes_model == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.cancellation_supersession_status | startswith("blocked_"))
      and .activation_command_result_receipt_cancellation_allowed == false
      and .activation_command_result_receipt_cancellation_recorded == false
      and .activation_command_result_receipt_cancellation_persisted == false
      and .activation_command_result_receipt_cancellation_request_accepted == false
      and .activation_command_result_receipt_supersession_allowed == false
      and .activation_command_result_receipt_supersession_recorded == false
      and .activation_command_result_receipt_supersession_persisted == false
      and .activation_command_result_receipt_supersession_request_accepted == false
      and .activation_command_result_receipt_replacement_receipt_accepted == false
      and .activation_command_result_receipt_replacement_receipt_recorded == false
      and .activation_command_result_receipt_replacement_receipt_persisted == false
      and .activation_command_result_receipt_replacement_hash_accepted == false
      and .activation_command_result_receipt_tombstone_recorded == false
      and .activation_command_result_receipt_delete_marker_recorded == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_command_result_receipt_accepted == false
      and .activation_command_completion_ack_accepted == false
      and .operator_approval_from_cancellation_accepted == false
      and .operator_approval_from_supersession_accepted == false
      and .activation_from_cancellation_allowed == false
      and .activation_from_supersession_allowed == false
      and .terminal_closure_from_cancellation_recorded == false
      and .terminal_closure_from_supersession_recorded == false
      and .terminal_closure_from_cancellation_accepted == false
      and .terminal_closure_from_supersession_accepted == false
      and .activation_command_enabled == false
      and .activation_command_invoked == false
      and .activation_command_dispatched == false
      and .activation_request_accepted == false
      and .activation_request_recorded == false
      and .activation_request_persisted == false
      and .activation_request_executed == false
      and .activation_performed == false
      and .terminal_closure_recorded == false
      and .terminal_closure_accepted == false
      and .provider_invoked == false
      and .model_invoked == false
      and .memory_store_write_performed == false
      and .memory_store_mutated == false
      and .live_kg_write_performed == false
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
      and .receipt_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_gate" \
    --arg ordering_monotonicity_report_sha256 "$ordering_monotonicity_report_sha256" \
    --arg cancellation_supersession_fixtures_sha256 "$cancellation_supersession_fixtures_sha256" \
    --arg cancellation_supersession_contract_hash_sha256 "$cancellation_supersession_contract_hash_sha256" \
    --arg cancellation_supersession_policy_hash_sha256 "$cancellation_supersession_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ORDERING_MONOTONICITY_JSON" \
    --argjson fixtures "$cancellation_supersession_fixtures_json" \
    '{
      product: $product,
      runtime: $runtime,
      status: "ready",
      base_url: $base_url,
      gate: $gate,
      schema_version: "core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_v1",
      mode: "summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_no_cancel_no_supersede_no_replacement_persist",
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_gate: $source.gate,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_ready: $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_status: $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_report_sha256: $ordering_monotonicity_report_sha256,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_ready: $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_status: $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denied_count: $source.source_activation_command_result_receipt_replay_idempotency_denied_count,
      cancellation_supersession_fixtures_sha256: $cancellation_supersession_fixtures_sha256,
      cancellation_supersession_contract_hash_sha256: $cancellation_supersession_contract_hash_sha256,
      cancellation_supersession_policy_hash_sha256: $cancellation_supersession_policy_hash_sha256,
      side_effect_hash_sha256: $side_effect_hash_sha256,
      minimum_required_samples: $min_long_soak_samples,
      core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready: true,
      operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status: "blocked",
      core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready: $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready,
      operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status: $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_status,
      core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready: $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready,
      source_activation_command_result_receipt_ordering_monotonicity_denied_count: $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_count,
      activation_command_result_receipt_ordering_monotonicity_surface_count: $source.activation_command_result_receipt_ordering_monotonicity_surface_count,
      activation_command_result_receipt_ordering_monotonicity_surface_ready_count: $source.activation_command_result_receipt_ordering_monotonicity_surface_ready_count,
      activation_command_result_receipt_cancellation_supersession_surface_count: 14,
      activation_command_result_receipt_cancellation_supersession_surface_ready_count: 14,
      activation_command_result_receipt_cancellation_supersession_side_effect_free_surface_count: 14,
      activation_command_result_receipt_cancellation_supersession_fixture_count: ($fixtures | length),
      blocked_activation_command_result_receipt_cancellation_supersession_fixture_count: ($fixtures | length),
      noop_activation_command_result_receipt_cancellation_supersession_fixture_count: ($fixtures | length),
      allowed_activation_command_result_receipt_cancellation_supersession_fixture_count: 0,
      accepted_activation_command_result_receipt_cancellation_supersession_fixture_count: 0,
      activation_command_result_receipt_cancellation_fixture_count: ($fixtures | map(select(.cancellation_requested == true)) | length),
      activation_command_result_receipt_supersession_fixture_count: ($fixtures | map(select(.supersession_requested == true)) | length),
      activation_command_result_receipt_cancellation_denied_count: ($fixtures | map(select(.cancellation_requested == true)) | length),
      activation_command_result_receipt_supersession_denied_count: ($fixtures | map(select(.supersession_requested == true)) | length),
      activation_command_result_receipt_cancellation_performed_count: 0,
      activation_command_result_receipt_supersession_performed_count: 0,
      activation_command_result_receipt_replacement_receipt_accepted_count: 0,
      activation_command_result_receipt_replacement_receipt_recorded_count: 0,
      activation_command_result_receipt_replacement_receipt_persisted_count: 0,
      activation_command_result_receipt_tombstone_recorded_count: 0,
      activation_command_result_receipt_delete_marker_recorded_count: 0,
      activation_command_result_receipt_cancellation_allowed: false,
      activation_command_result_receipt_cancellation_recorded: false,
      activation_command_result_receipt_cancellation_persisted: false,
      activation_command_result_receipt_cancellation_materialized: false,
      activation_command_result_receipt_cancellation_filesystem_written: false,
      activation_command_result_receipt_cancellation_request_accepted: false,
      activation_command_result_receipt_supersession_allowed: false,
      activation_command_result_receipt_supersession_recorded: false,
      activation_command_result_receipt_supersession_persisted: false,
      activation_command_result_receipt_supersession_materialized: false,
      activation_command_result_receipt_supersession_filesystem_written: false,
      activation_command_result_receipt_supersession_request_accepted: false,
      activation_command_result_receipt_replacement_receipt_accepted: false,
      activation_command_result_receipt_replacement_receipt_recorded: false,
      activation_command_result_receipt_replacement_receipt_persisted: false,
      activation_command_result_receipt_replacement_hash_accepted: false,
      activation_command_result_receipt_tombstone_recorded: false,
      activation_command_result_receipt_tombstone_persisted: false,
      activation_command_result_receipt_delete_marker_recorded: false,
      activation_command_result_receipt_ack_cancellation_accepted: false,
      activation_command_result_receipt_ledger_cancellation_accepted: false,
      activation_command_result_receipt_index_cancellation_accepted: false,
      activation_command_result_receipt_delivery_cancellation_accepted: false,
      activation_command_result_receipt_export_cancellation_accepted: false,
      activation_command_result_receipt_query_cancellation_accepted: false,
      activation_command_result_receipt_observability_cancellation_accepted: false,
      activation_command_result_receipt_ordering_allowed: false,
      activation_command_result_receipt_ordering_recorded: false,
      activation_command_result_receipt_ordering_persisted: false,
      activation_command_result_receipt_sequence_cursor_accepted: false,
      activation_command_result_receipt_sequence_cursor_recorded: false,
      activation_command_result_receipt_sequence_cursor_persisted: false,
      activation_command_result_receipt_monotonicity_state_recorded: false,
      activation_command_result_receipt_monotonicity_state_persisted: false,
      activation_command_result_receipt_latest_wins_overwrite_accepted: false,
      activation_command_result_receipt_same_sequence_hash_override_accepted: false,
      activation_command_result_receipt_recorded: false,
      activation_command_result_receipt_persisted: false,
      activation_command_result_receipt_accepted: false,
      activation_command_result_receipt_materialized: false,
      activation_command_result_receipt_filesystem_written: false,
      activation_command_result_receipt_ledger_written: false,
      activation_command_result_receipt_indexed: false,
      activation_command_result_receipt_enqueued: false,
      activation_command_result_receipt_delivered: false,
      activation_command_result_receipt_exported: false,
      activation_command_result_receipt_query_registered: false,
      activation_command_result_receipt_observability_recorded: false,
      activation_command_completion_ack_recorded: false,
      activation_command_completion_ack_persisted: false,
      activation_command_completion_ack_accepted: false,
      activation_command_completion_ack_delivered: false,
      operator_approval_from_cancellation_accepted: false,
      operator_approval_from_supersession_accepted: false,
      activation_from_cancellation_allowed: false,
      activation_from_supersession_allowed: false,
      activation_from_ordering_allowed: false,
      activation_from_replay_allowed: false,
      activation_from_receipt_allowed: false,
      terminal_closure_from_cancellation_recorded: false,
      terminal_closure_from_supersession_recorded: false,
      terminal_closure_from_cancellation_accepted: false,
      terminal_closure_from_supersession_accepted: false,
      activation_command_shape_registered: false,
      activation_command_accepted: false,
      activation_command_enabled: false,
      activation_command_invoked: false,
      activation_command_dispatched: false,
      activation_command_dispatch_performed: false,
      activation_request_accepted: false,
      activation_request_recorded: false,
      activation_request_persisted: false,
      activation_request_executed: false,
      activation_allowed: false,
      activation_performed: false,
      terminal_closure_recorded: false,
      terminal_closure_accepted: false,
      terminal_closure_final_state_promoted: false,
      terminal_closure_completion_promoted: false,
      runtime_attachment_performed: false,
      live_context_attached: false,
      context_injection_performed: false,
      provider_invoked: false,
      model_invoked: false,
      usage_recorded: false,
      memory_store_write_performed: false,
      memory_store_mutated: false,
      live_kg_write_performed: false,
      readback_evidence_recorded: false,
      readback_evidence_persisted: false,
      router_handoff_recorded: false,
      router_handoff_persisted: false,
      rollback_executed: false,
      telegram_send_performed: false,
      channel_send_performed: false,
      external_send_performed: false,
      public_release_claimed: false,
      public_ga_claimed: false,
      release_artifact_written: false,
      install_executed: false,
      launchd_mutated: false,
      service_restarted: false,
      active_binary_mutated: false,
      upstream_fetch_performed: false,
      upstream_merge_performed: false,
      credential_read: false,
      secret_value_read: false,
      raw_payload_plaintext_recorded: false,
      raw_payload_plaintext_persisted: false,
      activation_command_result_receipt_cancellation_supersession_surfaces: [
        "source_ordering_monotonicity_report_required",
        "cancellation_request_shape_denied",
        "supersession_request_shape_denied",
        "replacement_receipt_hash_denied",
        "tombstone_or_delete_marker_denied",
        "cancel_after_blocked_noop_denied",
        "supersede_blocked_noop_with_completed_denied",
        "acknowledgement_cancellation_denied",
        "ledger_index_delivery_export_cancellation_denied",
        "receipt_export_query_observability_cancellation_denied",
        "runtime_provider_model_supersession_denied",
        "memory_kg_rollback_upstream_supersession_denied",
        "external_public_install_restart_active_binary_supersession_denied",
        "credential_secret_supersession_denied"
      ],
      activation_command_result_receipt_cancellation_supersession_fixtures: $fixtures,
      new_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial: [
        "source_ordering_monotonicity_report_required",
        "cancellation_request_acceptance_denied",
        "cancellation_recording_denied",
        "cancellation_persistence_denied",
        "cancellation_request_materialization_denied",
        "supersession_request_acceptance_denied",
        "supersession_recording_denied",
        "supersession_persistence_denied",
        "supersession_request_materialization_denied",
        "replacement_receipt_acceptance_denied",
        "replacement_receipt_recording_denied",
        "replacement_receipt_persistence_denied",
        "replacement_hash_acceptance_denied",
        "tombstone_recording_denied",
        "tombstone_persistence_denied",
        "delete_marker_recording_denied",
        "cancel_after_blocked_noop_denied",
        "supersede_blocked_noop_with_completed_denied",
        "completion_ack_cancellation_denied",
        "ledger_cancellation_denied",
        "index_cancellation_denied",
        "delivery_cancellation_denied",
        "export_query_observability_cancellation_denied",
        "terminal_closure_cancellation_supersession_denied",
        "activation_command_cancellation_supersession_denied",
        "activation_request_cancellation_supersession_denied",
        "runtime_provider_model_supersession_denied",
        "memory_kg_rollback_supersession_denied",
        "external_public_release_install_restart_active_binary_supersession_denied",
        "upstream_credential_secret_supersession_denied"
      ],
      inherited_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial: $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial,
      inherited_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_count: $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_count,
      denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial:
        ($source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial + [
          "source_ordering_monotonicity_report_required",
          "cancellation_request_acceptance_denied",
          "cancellation_recording_denied",
          "cancellation_persistence_denied",
          "cancellation_request_materialization_denied",
          "supersession_request_acceptance_denied",
          "supersession_recording_denied",
          "supersession_persistence_denied",
          "supersession_request_materialization_denied",
          "replacement_receipt_acceptance_denied",
          "replacement_receipt_recording_denied",
          "replacement_receipt_persistence_denied",
          "replacement_hash_acceptance_denied",
          "tombstone_recording_denied",
          "tombstone_persistence_denied",
          "delete_marker_recording_denied",
          "cancel_after_blocked_noop_denied",
          "supersede_blocked_noop_with_completed_denied",
          "completion_ack_cancellation_denied",
          "ledger_cancellation_denied",
          "index_cancellation_denied",
          "delivery_cancellation_denied",
          "export_query_observability_cancellation_denied",
          "terminal_closure_cancellation_supersession_denied",
          "activation_command_cancellation_supersession_denied",
          "activation_request_cancellation_supersession_denied",
          "runtime_provider_model_supersession_denied",
          "memory_kg_rollback_supersession_denied",
          "external_public_release_install_restart_active_binary_supersession_denied",
          "upstream_credential_secret_supersession_denied"
        ]),
      denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_count: 272,
      allowed_next_actions: [
        {
          action: "review_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial",
          status: "allowed_report_only",
          accepts_cancellation: false,
          accepts_supersession: false,
          persists_replacement_receipt: false,
          records_terminal_closure: false,
          mutates_runtime: false,
          invokes_model: false
        },
        {
          action: "stage_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
          status: "allowed_report_only_next_slice",
          accepts_cancellation: false,
          accepts_supersession: false,
          writes_audit_trail: false,
          persists_evidence: false,
          records_terminal_closure: false,
          mutates_runtime: false,
          invokes_model: false
        },
        {
          action: "run_full_preflight",
          status: "allowed_verification_only",
          accepts_cancellation: false,
          accepts_supersession: false,
          persists_replacement_receipt: false,
          records_terminal_closure: false,
          mutates_runtime: false,
          invokes_model: false
        }
      ],
      source_ordering_monotonicity_report_required: true,
      cancellation_acceptance_forbidden: true,
      cancellation_recording_forbidden: true,
      cancellation_persistence_forbidden: true,
      supersession_acceptance_forbidden: true,
      supersession_recording_forbidden: true,
      supersession_persistence_forbidden: true,
      replacement_receipt_persistence_forbidden: true,
      tombstone_or_delete_marker_forbidden: true,
      terminal_closure_from_cancellation_supersession_forbidden: true,
      runtime_provider_memory_kg_supersession_forbidden: true,
      upstream_credential_secret_supersession_forbidden: true,
      side_effects: {
        activation_command_result_receipt_cancellation_recorded: false,
        activation_command_result_receipt_cancellation_persisted: false,
        activation_command_result_receipt_cancellation_materialized: false,
        activation_command_result_receipt_cancellation_filesystem_written: false,
        activation_command_result_receipt_supersession_recorded: false,
        activation_command_result_receipt_supersession_persisted: false,
        activation_command_result_receipt_supersession_materialized: false,
        activation_command_result_receipt_supersession_filesystem_written: false,
        activation_command_result_receipt_replacement_receipt_recorded: false,
        activation_command_result_receipt_replacement_receipt_persisted: false,
        activation_command_result_receipt_replacement_hash_accepted: false,
        activation_command_result_receipt_tombstone_recorded: false,
        activation_command_result_receipt_tombstone_persisted: false,
        activation_command_result_receipt_delete_marker_recorded: false,
        activation_command_result_receipt_ack_cancellation_accepted: false,
        activation_command_result_receipt_ledger_cancellation_accepted: false,
        activation_command_result_receipt_index_cancellation_accepted: false,
        activation_command_result_receipt_delivery_cancellation_accepted: false,
        activation_command_result_receipt_export_cancellation_accepted: false,
        activation_command_result_receipt_query_cancellation_accepted: false,
        activation_command_result_receipt_observability_cancellation_accepted: false,
        activation_command_result_receipt_ordering_recorded: false,
        activation_command_result_receipt_ordering_persisted: false,
        activation_command_result_receipt_sequence_cursor_accepted: false,
        activation_command_result_receipt_sequence_cursor_recorded: false,
        activation_command_result_receipt_sequence_cursor_persisted: false,
        activation_command_result_receipt_monotonicity_state_recorded: false,
        activation_command_result_receipt_monotonicity_state_persisted: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_command_result_receipt_materialized: false,
        activation_command_result_receipt_filesystem_written: false,
        activation_command_result_receipt_ledger_written: false,
        activation_command_result_receipt_indexed: false,
        activation_command_result_receipt_enqueued: false,
        activation_command_result_receipt_delivered: false,
        activation_command_completion_ack_recorded: false,
        activation_command_completion_ack_persisted: false,
        activation_command_completion_ack_accepted: false,
        activation_command_completion_ack_delivered: false,
        terminal_closure_from_cancellation_recorded: false,
        terminal_closure_from_supersession_recorded: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_executed: false,
        activation_performed: false,
        terminal_closure_recorded: false,
        terminal_closure_accepted: false,
        runtime_attachment_performed: false,
        live_context_attached: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        usage_recorded: false,
        memory_store_write_performed: false,
        memory_store_mutated: false,
        live_kg_write_performed: false,
        readback_evidence_recorded: false,
        readback_evidence_persisted: false,
        router_handoff_recorded: false,
        router_handoff_persisted: false,
        rollback_executed: false,
        telegram_send_performed: false,
        channel_send_performed: false,
        external_send_performed: false,
        filesystem_written: false,
        public_release_claimed: false,
        public_ga_claimed: false,
        release_artifact_written: false,
        install_executed: false,
        launchd_mutated: false,
        service_restarted: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        credential_read: false,
        secret_value_read: false,
        raw_payload_plaintext_recorded: false,
        raw_payload_plaintext_persisted: false
      }
    }'
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_gate"
  and .schema_version == "core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_v1"
  and .core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready == true
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status == "blocked"
  and .source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_ready == true
  and .source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_status == "blocked"
  and .core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_ready == true
  and .source_activation_command_result_receipt_ordering_monotonicity_denied_count == 242
  and .activation_command_result_receipt_cancellation_supersession_surface_count == 14
  and .activation_command_result_receipt_cancellation_supersession_surface_ready_count == 14
  and .activation_command_result_receipt_cancellation_supersession_side_effect_free_surface_count == 14
  and .activation_command_result_receipt_cancellation_supersession_fixture_count == 10
  and .blocked_activation_command_result_receipt_cancellation_supersession_fixture_count == 10
  and .noop_activation_command_result_receipt_cancellation_supersession_fixture_count == 10
  and .allowed_activation_command_result_receipt_cancellation_supersession_fixture_count == 0
  and .accepted_activation_command_result_receipt_cancellation_supersession_fixture_count == 0
  and .activation_command_result_receipt_cancellation_fixture_count == 5
  and .activation_command_result_receipt_supersession_fixture_count == 5
  and .activation_command_result_receipt_cancellation_denied_count == 5
  and .activation_command_result_receipt_supersession_denied_count == 5
  and .activation_command_result_receipt_cancellation_performed_count == 0
  and .activation_command_result_receipt_supersession_performed_count == 0
  and .activation_command_result_receipt_replacement_receipt_accepted_count == 0
  and .activation_command_result_receipt_replacement_receipt_recorded_count == 0
  and .activation_command_result_receipt_replacement_receipt_persisted_count == 0
  and .activation_command_result_receipt_tombstone_recorded_count == 0
  and .activation_command_result_receipt_delete_marker_recorded_count == 0
  and .activation_command_result_receipt_cancellation_allowed == false
  and .activation_command_result_receipt_cancellation_recorded == false
  and .activation_command_result_receipt_cancellation_persisted == false
  and .activation_command_result_receipt_cancellation_request_accepted == false
  and .activation_command_result_receipt_supersession_allowed == false
  and .activation_command_result_receipt_supersession_recorded == false
  and .activation_command_result_receipt_supersession_persisted == false
  and .activation_command_result_receipt_supersession_request_accepted == false
  and .activation_command_result_receipt_replacement_receipt_accepted == false
  and .activation_command_result_receipt_replacement_receipt_recorded == false
  and .activation_command_result_receipt_replacement_receipt_persisted == false
  and .activation_command_result_receipt_replacement_hash_accepted == false
  and .activation_command_result_receipt_tombstone_recorded == false
  and .activation_command_result_receipt_tombstone_persisted == false
  and .activation_command_result_receipt_delete_marker_recorded == false
  and .activation_command_result_receipt_ack_cancellation_accepted == false
  and .activation_command_result_receipt_ledger_cancellation_accepted == false
  and .activation_command_result_receipt_index_cancellation_accepted == false
  and .activation_command_result_receipt_delivery_cancellation_accepted == false
  and .activation_command_result_receipt_export_cancellation_accepted == false
  and .activation_command_result_receipt_query_cancellation_accepted == false
  and .activation_command_result_receipt_observability_cancellation_accepted == false
  and .activation_command_result_receipt_ordering_allowed == false
  and .activation_command_result_receipt_ordering_recorded == false
  and .activation_command_result_receipt_ordering_persisted == false
  and .activation_command_result_receipt_sequence_cursor_accepted == false
  and .activation_command_result_receipt_sequence_cursor_recorded == false
  and .activation_command_result_receipt_sequence_cursor_persisted == false
  and .activation_command_result_receipt_monotonicity_state_recorded == false
  and .activation_command_result_receipt_monotonicity_state_persisted == false
  and .activation_command_result_receipt_latest_wins_overwrite_accepted == false
  and .activation_command_result_receipt_same_sequence_hash_override_accepted == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_result_receipt_materialized == false
  and .activation_command_result_receipt_filesystem_written == false
  and .activation_command_result_receipt_ledger_written == false
  and .activation_command_result_receipt_indexed == false
  and .activation_command_result_receipt_enqueued == false
  and .activation_command_result_receipt_delivered == false
  and .activation_command_result_receipt_exported == false
  and .activation_command_result_receipt_query_registered == false
  and .activation_command_result_receipt_observability_recorded == false
  and .activation_command_completion_ack_recorded == false
  and .activation_command_completion_ack_persisted == false
  and .activation_command_completion_ack_accepted == false
  and .activation_command_completion_ack_delivered == false
  and .operator_approval_from_cancellation_accepted == false
  and .operator_approval_from_supersession_accepted == false
  and .activation_from_cancellation_allowed == false
  and .activation_from_supersession_allowed == false
  and .activation_from_ordering_allowed == false
  and .activation_from_replay_allowed == false
  and .activation_from_receipt_allowed == false
  and .terminal_closure_from_cancellation_recorded == false
  and .terminal_closure_from_supersession_recorded == false
  and .terminal_closure_from_cancellation_accepted == false
  and .terminal_closure_from_supersession_accepted == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_command_dispatch_performed == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_persisted == false
  and .activation_request_executed == false
  and .activation_allowed == false
  and .activation_performed == false
  and .terminal_closure_recorded == false
  and .terminal_closure_accepted == false
  and .provider_invoked == false
  and .model_invoked == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .external_send_performed == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .upstream_fetch_performed == false
  and .upstream_merge_performed == false
  and .credential_read == false
  and .secret_value_read == false
  and (.activation_command_result_receipt_cancellation_supersession_surfaces | length) == 14
  and (.activation_command_result_receipt_cancellation_supersession_fixtures | length) == 10
  and (.activation_command_result_receipt_cancellation_supersession_fixtures | all(
    (.cancellation_supersession_status | startswith("blocked_"))
    and .activation_command_result_receipt_cancellation_allowed == false
    and .activation_command_result_receipt_cancellation_recorded == false
    and .activation_command_result_receipt_cancellation_persisted == false
    and .activation_command_result_receipt_supersession_allowed == false
    and .activation_command_result_receipt_supersession_recorded == false
    and .activation_command_result_receipt_supersession_persisted == false
    and .activation_command_result_receipt_replacement_receipt_accepted == false
    and .activation_command_result_receipt_replacement_receipt_recorded == false
    and .activation_command_result_receipt_replacement_receipt_persisted == false
    and .activation_command_result_receipt_replacement_hash_accepted == false
    and .activation_command_result_receipt_tombstone_recorded == false
    and .activation_command_result_receipt_delete_marker_recorded == false
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_command_result_receipt_accepted == false
    and .activation_command_completion_ack_accepted == false
    and .activation_from_cancellation_allowed == false
    and .activation_from_supersession_allowed == false
    and .terminal_closure_from_cancellation_recorded == false
    and .terminal_closure_from_supersession_recorded == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .external_send_performed == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .upstream_fetch_performed == false
    and .upstream_merge_performed == false
    and .credential_read == false
    and .secret_value_read == false
    and .receipt_noop_confirmed == true
  ))
  and (.new_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial | length) == 30
  and .inherited_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial_count == 242
  and .denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_count == 272
  and (.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial | length) == 272
  and (.allowed_next_actions | any(.action == "stage_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial" and .status == "allowed_report_only_next_slice" and .writes_audit_trail == false and .persists_evidence == false and .records_terminal_closure == false and .mutates_runtime == false and .invokes_model == false))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
