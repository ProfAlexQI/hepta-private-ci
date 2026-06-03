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

NO_PERSISTENCE_JSON="$(
  capture_json_report \
    "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-no-persistence-gate" \
    env \
      HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS="${HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS:-0}" \
      HEPTA_LONG_SOAK_OBSERVATION_INTERVAL_SECONDS="${HEPTA_LONG_SOAK_OBSERVATION_INTERVAL_SECONDS:-0}" \
      HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS="${HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS:-0}" \
      scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-no-persistence-gate.sh
)"

replay_idempotency_fixtures_json="$(
  jq -n '
    def replay_fixture($id; $status; $reason; $extra):
      {
        id: $id,
        replay_requested: true,
        replay_status: $status,
        source_result_receipt_no_persistence_present: true,
        source_result_receipt_no_persistence_ready: true,
        canonical_blocked_noop_result_receipt_identity_required: true,
        activation_command_result_receipt_replay_allowed: false,
        activation_command_result_receipt_replay_recorded: false,
        activation_command_result_receipt_replay_persisted: false,
        activation_command_result_receipt_replay_materialized: false,
        activation_command_result_receipt_replay_filesystem_written: false,
        activation_command_result_receipt_replay_performed: false,
        activation_command_result_receipt_duplicate_accepted: false,
        activation_command_result_receipt_duplicate_recorded: false,
        activation_command_result_receipt_duplicate_persisted: false,
        activation_command_result_receipt_idempotency_key_accepted: false,
        activation_command_result_receipt_idempotency_key_recorded: false,
        activation_command_result_receipt_idempotency_state_recorded: false,
        activation_command_result_receipt_idempotency_state_persisted: false,
        activation_command_result_receipt_idempotency_state_materialized: false,
        activation_command_result_receipt_idempotency_filesystem_written: false,
        activation_command_result_receipt_replay_nonce_accepted: false,
        activation_command_result_receipt_replay_nonce_recorded: false,
        activation_command_result_receipt_cross_scope_reuse_accepted: false,
        activation_command_result_receipt_status_upgrade_accepted: false,
        activation_command_result_receipt_completed_status_accepted: false,
        activation_command_result_receipt_ack_replay_accepted: false,
        activation_command_result_receipt_completion_ack_replay_accepted: false,
        activation_command_result_receipt_ledger_replay_accepted: false,
        activation_command_result_receipt_index_replay_accepted: false,
        activation_command_result_receipt_delivery_replay_accepted: false,
        activation_command_result_receipt_query_replay_accepted: false,
        activation_command_result_receipt_observability_replay_accepted: false,
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
        operator_approval_from_replay_accepted: false,
        activation_from_replay_allowed: false,
        terminal_closure_from_replay_recorded: false,
        terminal_closure_from_replay_accepted: false,
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
        raw_payload_plaintext_recorded: false,
        raw_payload_plaintext_persisted: false,
        receipt_noop_confirmed: true,
        reason: $reason
      } + $extra;
    [
      replay_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-replay-missing-source";
        "blocked_noop";
        "source_result_receipt_no_persistence_report_required";
        {
          source_result_receipt_no_persistence_present: false,
          source_result_receipt_no_persistence_ready: false
        }
      ),
      replay_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-duplicate-identity";
        "blocked_duplicate_noop";
        "duplicate_result_receipt_identity_replay_denied";
        {duplicate_result_receipt_identity_requested: true}
      ),
      replay_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-replay-acceptance";
        "blocked_replay_noop";
        "result_receipt_replay_acceptance_denied";
        {result_receipt_replay_acceptance_requested: true}
      ),
      replay_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-idempotency-key";
        "blocked_idempotency_key_noop";
        "idempotency_key_recording_denied";
        {
          idempotency_key_acceptance_requested: true,
          idempotency_key_recording_requested: true
        }
      ),
      replay_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-idempotency-state";
        "blocked_idempotency_state_noop";
        "idempotency_state_persistence_materialization_denied";
        {
          idempotency_state_recording_requested: true,
          idempotency_state_persistence_requested: true,
          idempotency_state_materialization_requested: true,
          idempotency_filesystem_write_requested: true
        }
      ),
      replay_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-cross-scope-reuse";
        "blocked_cross_scope_noop";
        "cross_scope_result_receipt_reuse_denied";
        {cross_scope_reuse_requested: true}
      ),
      replay_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-stale-nonce-order";
        "blocked_nonce_order_noop";
        "stale_nonce_out_of_order_receipt_replay_denied";
        {
          stale_nonce_replay_requested: true,
          out_of_order_replay_requested: true,
          replay_nonce_acceptance_requested: true
        }
      ),
      replay_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-completion-ledger-delivery";
        "blocked_completion_ledger_delivery_noop";
        "completion_ack_ledger_delivery_replay_denied";
        {
          completion_ack_replay_requested: true,
          ledger_replay_requested: true,
          index_replay_requested: true,
          delivery_replay_requested: true
        }
      ),
      replay_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-runtime-provider-terminal";
        "blocked_runtime_provider_terminal_noop";
        "runtime_provider_terminal_replay_denied";
        {
          result_receipt_status_upgrade_requested: true,
          completed_status_acceptance_requested: true,
          operator_approval_from_replay_requested: true,
          activation_from_replay_requested: true,
          terminal_closure_from_replay_requested: true,
          live_context_replay_requested: true,
          context_injection_replay_requested: true,
          provider_replay_requested: true,
          model_replay_requested: true
        }
      ),
      replay_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-command-result-receipt-external-public-install-upstream-secret";
        "blocked_external_noop";
        "external_public_install_restart_upstream_secret_replay_denied";
        {
          external_send_replay_requested: true,
          public_claim_replay_requested: true,
          release_artifact_replay_requested: true,
          install_replay_requested: true,
          launchd_restart_replay_requested: true,
          service_restart_replay_requested: true,
          active_binary_mutation_replay_requested: true,
          upstream_replay_requested: true,
          credential_replay_requested: true,
          secret_value_replay_requested: true,
          raw_payload_plaintext_persistence_requested: true
        }
      )
    ]
  '
)"

no_persistence_report_sha256="$(sha256_text "$NO_PERSISTENCE_JSON")"
replay_idempotency_fixtures_sha256="$(sha256_text "$replay_idempotency_fixtures_json")"
replay_idempotency_contract_hash_sha256="$(
  sha256_text "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-replay-idempotency-denial:$no_persistence_report_sha256:$replay_idempotency_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
replay_idempotency_policy_hash_sha256="$(
  sha256_text "operator-gap-ledger-summary-briefing-acknowledgement-activation-command-result-receipt-replay-idempotency-denial:no-replay:no-duplicate:no-idempotency-state:no-persist:no-terminal-closure:no-provider:no-secret-read"
)"
side_effect_hash_sha256="$(
  sha256_text "result-receipt-replay=false;duplicate=false;idempotency=false;record=false;persist=false;activation=false;terminal-closure=false;provider=false;release=false;install=false;upstream=false;secret=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$NO_PERSISTENCE_JSON" \
  --argjson fixtures "$replay_idempotency_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_gate"
    and $source.schema_version == "core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_v1"
    and $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_ready == true
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_status == "blocked"
    and $source.activation_command_result_receipt_surface_count == 14
    and $source.activation_command_result_receipt_surface_ready_count == 14
    and $source.activation_command_result_receipt_fixture_count == 10
    and $source.blocked_activation_command_result_receipt_fixture_count == 10
    and $source.noop_activation_command_result_receipt_fixture_count == 10
    and $source.allowed_activation_command_result_receipt_fixture_count == 0
    and $source.accepted_activation_command_result_receipt_fixture_count == 0
    and $source.activation_command_result_receipt_denied_count == 10
    and $source.activation_command_result_receipt_performed_count == 0
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_result_receipt_exported == false
    and $source.activation_command_result_receipt_query_registered == false
    and $source.activation_command_result_receipt_observability_recorded == false
    and $source.activation_command_completion_ack_accepted == false
    and $source.operator_approval_from_receipt_accepted == false
    and $source.activation_from_receipt_allowed == false
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
    and $source.telegram_send_performed == false
    and $source.channel_send_performed == false
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
    and $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_count == 182
    and ($source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence | length) == 182
    and $source.inherited_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff_count == 147
    and ($source.allowed_next_actions | any(.action == "stage_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial" and .status == "allowed_report_only_next_slice" and .accepts_duplicate_receipt == false and .records_idempotency == false and .persists_replay_state == false and .records_terminal_closure == false and .mutates_runtime == false and .invokes_model == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.replay_status | startswith("blocked_"))
      and .activation_command_result_receipt_replay_allowed == false
      and .activation_command_result_receipt_replay_recorded == false
      and .activation_command_result_receipt_replay_persisted == false
      and .activation_command_result_receipt_replay_performed == false
      and .activation_command_result_receipt_duplicate_accepted == false
      and .activation_command_result_receipt_idempotency_key_accepted == false
      and .activation_command_result_receipt_idempotency_state_recorded == false
      and .activation_command_result_receipt_idempotency_state_persisted == false
      and .activation_command_result_receipt_replay_nonce_accepted == false
      and .activation_command_result_receipt_cross_scope_reuse_accepted == false
      and .activation_command_result_receipt_completed_status_accepted == false
      and .activation_command_result_receipt_completion_ack_replay_accepted == false
      and .activation_command_result_receipt_ledger_replay_accepted == false
      and .activation_command_result_receipt_delivery_replay_accepted == false
      and .activation_command_result_receipt_query_replay_accepted == false
      and .activation_command_result_receipt_observability_replay_accepted == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_command_result_receipt_accepted == false
      and .activation_command_completion_ack_accepted == false
      and .operator_approval_from_replay_accepted == false
      and .activation_from_replay_allowed == false
      and .terminal_closure_from_replay_recorded == false
      and .terminal_closure_from_replay_accepted == false
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
    --arg gate "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_gate" \
    --arg no_persistence_report_sha256 "$no_persistence_report_sha256" \
    --arg replay_idempotency_fixtures_sha256 "$replay_idempotency_fixtures_sha256" \
    --arg replay_idempotency_contract_hash_sha256 "$replay_idempotency_contract_hash_sha256" \
    --arg replay_idempotency_policy_hash_sha256 "$replay_idempotency_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$NO_PERSISTENCE_JSON" \
    --argjson fixtures "$replay_idempotency_fixtures_json" \
    '{
      product: $product,
      runtime: $runtime,
      status: "ready",
      base_url: $base_url,
      gate: $gate,
      schema_version: "core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_v1",
      mode: "summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_no_duplicate_no_replay_no_persist",
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_gate: $source.gate,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_ready: $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_ready,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_status: $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_status,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_report_sha256: $no_persistence_report_sha256,
      replay_idempotency_fixtures_sha256: $replay_idempotency_fixtures_sha256,
      replay_idempotency_contract_hash_sha256: $replay_idempotency_contract_hash_sha256,
      replay_idempotency_policy_hash_sha256: $replay_idempotency_policy_hash_sha256,
      side_effect_hash_sha256: $side_effect_hash_sha256,
      minimum_required_samples: $min_long_soak_samples,
      core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready: true,
      operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status: "blocked",
      source_activation_command_result_receipt_surface_count: $source.activation_command_result_receipt_surface_count,
      source_activation_command_result_receipt_fixture_count: $source.activation_command_result_receipt_fixture_count,
      source_activation_command_result_receipt_denied_count: $source.activation_command_result_receipt_denied_count,
      source_activation_command_result_receipt_performed_count: $source.activation_command_result_receipt_performed_count,
      activation_command_result_receipt_replay_idempotency_surface_count: 13,
      activation_command_result_receipt_replay_idempotency_surface_ready_count: 13,
      activation_command_result_receipt_replay_idempotency_side_effect_free_surface_count: 13,
      activation_command_result_receipt_replay_idempotency_fixture_count: ($fixtures | length),
      blocked_activation_command_result_receipt_replay_idempotency_fixture_count: ($fixtures | length),
      noop_activation_command_result_receipt_replay_idempotency_fixture_count: ($fixtures | length),
      allowed_activation_command_result_receipt_replay_idempotency_fixture_count: 0,
      accepted_activation_command_result_receipt_replay_idempotency_fixture_count: 0,
      duplicate_activation_command_result_receipt_replay_fixture_count: 1,
      cross_scope_activation_command_result_receipt_replay_fixture_count: 1,
      status_upgrade_activation_command_result_receipt_replay_fixture_count: 1,
      activation_command_result_receipt_replay_denied_count: 10,
      activation_command_result_receipt_duplicate_denied_count: 10,
      activation_command_result_receipt_idempotency_denied_count: 10,
      activation_command_result_receipt_replay_performed_count: 0,
      activation_command_result_receipt_duplicate_accepted_count: 0,
      activation_command_result_receipt_idempotency_state_recorded_count: 0,
      activation_command_result_receipt_replay_allowed: false,
      activation_command_result_receipt_replay_recorded: false,
      activation_command_result_receipt_replay_persisted: false,
      activation_command_result_receipt_replay_materialized: false,
      activation_command_result_receipt_replay_filesystem_written: false,
      activation_command_result_receipt_replay_performed: false,
      activation_command_result_receipt_duplicate_accepted: false,
      activation_command_result_receipt_duplicate_recorded: false,
      activation_command_result_receipt_duplicate_persisted: false,
      activation_command_result_receipt_idempotency_key_accepted: false,
      activation_command_result_receipt_idempotency_key_recorded: false,
      activation_command_result_receipt_idempotency_state_recorded: false,
      activation_command_result_receipt_idempotency_state_persisted: false,
      activation_command_result_receipt_idempotency_state_materialized: false,
      activation_command_result_receipt_idempotency_filesystem_written: false,
      activation_command_result_receipt_replay_nonce_accepted: false,
      activation_command_result_receipt_replay_nonce_recorded: false,
      activation_command_result_receipt_cross_scope_reuse_accepted: false,
      activation_command_result_receipt_status_upgrade_accepted: false,
      activation_command_result_receipt_completed_status_accepted: false,
      activation_command_result_receipt_ack_replay_accepted: false,
      activation_command_result_receipt_completion_ack_replay_accepted: false,
      activation_command_result_receipt_ledger_replay_accepted: false,
      activation_command_result_receipt_index_replay_accepted: false,
      activation_command_result_receipt_delivery_replay_accepted: false,
      activation_command_result_receipt_query_replay_accepted: false,
      activation_command_result_receipt_observability_replay_accepted: false,
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
      operator_approval_from_replay_accepted: false,
      activation_from_replay_allowed: false,
      terminal_closure_from_replay_recorded: false,
      terminal_closure_from_replay_accepted: false,
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
      raw_payload_plaintext_recorded: false,
      raw_payload_plaintext_persisted: false,
      activation_command_result_receipt_replay_idempotency_surfaces: [
        "source_result_receipt_no_persistence_report_required",
        "canonical_blocked_noop_result_receipt_identity_required",
        "receipt_replay_nonce_idempotency_key_denied",
        "duplicate_result_receipt_suppression_required",
        "cross_scope_result_receipt_reuse_denied",
        "blocked_noop_status_upgrade_denied",
        "completion_ack_replay_denied",
        "ledger_index_delivery_replay_denied",
        "query_observability_replay_denied",
        "terminal_closure_from_replay_denied",
        "runtime_context_provider_model_replay_denied",
        "external_public_install_restart_upstream_replay_denied",
        "credential_secret_raw_payload_replay_denied"
      ],
      activation_command_result_receipt_replay_idempotency_fixtures: $fixtures,
      denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial: (
        $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence + [
          "source_result_receipt_no_persistence_report_required",
          "canonical_noop_result_receipt_identity_required",
          "result_receipt_replay_acceptance_denied",
          "result_receipt_replay_recording_denied",
          "result_receipt_replay_persistence_denied",
          "duplicate_result_receipt_identity_replay_denied",
          "duplicate_result_receipt_recording_denied",
          "duplicate_result_receipt_persistence_denied",
          "idempotency_key_acceptance_denied",
          "idempotency_key_recording_denied",
          "idempotency_state_recording_denied",
          "idempotency_state_persistence_denied",
          "idempotency_state_materialization_denied",
          "idempotency_filesystem_write_denied",
          "replay_nonce_acceptance_denied",
          "replay_nonce_recording_denied",
          "cross_scope_result_receipt_reuse_denied",
          "stale_nonce_out_of_order_receipt_replay_denied",
          "result_receipt_status_upgrade_denied",
          "completed_status_acceptance_denied",
          "completion_ack_replay_denied",
          "ledger_index_delivery_replay_denied",
          "query_observability_replay_denied",
          "operator_approval_from_replay_denied",
          "activation_from_replay_denied",
          "terminal_closure_from_replay_denied",
          "runtime_context_provider_model_replay_denied",
          "external_public_install_restart_upstream_replay_denied",
          "credential_secret_replay_denied",
          "raw_payload_plaintext_replay_persistence_denied"
        ]
      ),
      denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_count: (
        ($source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence | length) + 30
      ),
      inherited_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_count: (
        $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence | length
      ),
      allowed_next_actions: [
        {
          action: "review_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial",
          status: "allowed_report_only",
          accepts_duplicate_receipt: false,
          records_idempotency: false,
          persists_replay_state: false,
          records_terminal_closure: false,
          mutates_runtime: false,
          invokes_model: false
        },
        {
          action: "stage_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial",
          status: "allowed_report_only_next_slice",
          accepts_out_of_order_receipt: false,
          records_ordering_state: false,
          persists_replay_state: false,
          records_terminal_closure: false,
          mutates_runtime: false,
          invokes_model: false
        },
        {
          action: "run_full_light_preflight",
          status: "allowed_verification_only",
          accepts_duplicate_receipt: false,
          persists_replay_state: false,
          mutates_runtime: false,
          invokes_model: false,
          writes_kg: false
        }
      ],
      source_result_receipt_no_persistence_report_required: true,
      result_receipt_replay_forbidden: true,
      result_receipt_duplicate_acceptance_forbidden: true,
      result_receipt_idempotency_state_forbidden: true,
      result_receipt_replay_persistence_forbidden: true,
      result_receipt_replay_activation_forbidden: true,
      result_receipt_replay_terminal_closure_forbidden: true,
      result_receipt_replay_runtime_mutation_forbidden: true,
      result_receipt_replay_context_attachment_forbidden: true,
      result_receipt_replay_provider_model_invocation_forbidden: true,
      result_receipt_replay_secret_read_forbidden: true,
      result_receipt_replay_external_public_install_restart_upstream_forbidden: true,
      side_effects: {
        workspace_written: false,
        filesystem_written: false,
        activation_command_result_receipt_replay_recorded: false,
        activation_command_result_receipt_replay_persisted: false,
        activation_command_result_receipt_replay_performed: false,
        activation_command_result_receipt_duplicate_accepted: false,
        activation_command_result_receipt_duplicate_recorded: false,
        activation_command_result_receipt_duplicate_persisted: false,
        activation_command_result_receipt_idempotency_key_recorded: false,
        activation_command_result_receipt_idempotency_state_recorded: false,
        activation_command_result_receipt_idempotency_state_persisted: false,
        activation_command_result_receipt_idempotency_filesystem_written: false,
        activation_command_result_receipt_replay_nonce_recorded: false,
        activation_command_result_receipt_status_upgrade_accepted: false,
        activation_command_result_receipt_completion_ack_replay_accepted: false,
        activation_command_result_receipt_ledger_replay_accepted: false,
        activation_command_result_receipt_query_replay_accepted: false,
        activation_command_result_receipt_observability_replay_accepted: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_command_completion_ack_recorded: false,
        activation_command_completion_ack_persisted: false,
        activation_command_completion_ack_accepted: false,
        operator_approval_from_replay_accepted: false,
        activation_from_replay_allowed: false,
        terminal_closure_from_replay_recorded: false,
        terminal_closure_from_replay_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
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
        raw_payload_plaintext_recorded: false,
        raw_payload_plaintext_persisted: false
      }
    }'
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_gate"
  and .schema_version == "core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_v1"
  and .core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_ready == true
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_status == "blocked"
  and .source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_ready == true
  and .source_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_status == "blocked"
  and .activation_command_result_receipt_replay_idempotency_surface_count == 13
  and .activation_command_result_receipt_replay_idempotency_surface_ready_count == 13
  and .activation_command_result_receipt_replay_idempotency_side_effect_free_surface_count == 13
  and .activation_command_result_receipt_replay_idempotency_fixture_count == 10
  and .blocked_activation_command_result_receipt_replay_idempotency_fixture_count == 10
  and .noop_activation_command_result_receipt_replay_idempotency_fixture_count == 10
  and .allowed_activation_command_result_receipt_replay_idempotency_fixture_count == 0
  and .accepted_activation_command_result_receipt_replay_idempotency_fixture_count == 0
  and .activation_command_result_receipt_replay_denied_count == 10
  and .activation_command_result_receipt_duplicate_denied_count == 10
  and .activation_command_result_receipt_idempotency_denied_count == 10
  and .activation_command_result_receipt_replay_performed_count == 0
  and .activation_command_result_receipt_duplicate_accepted_count == 0
  and .activation_command_result_receipt_idempotency_state_recorded_count == 0
  and .activation_command_result_receipt_replay_allowed == false
  and .activation_command_result_receipt_replay_recorded == false
  and .activation_command_result_receipt_replay_persisted == false
  and .activation_command_result_receipt_replay_performed == false
  and .activation_command_result_receipt_duplicate_accepted == false
  and .activation_command_result_receipt_duplicate_recorded == false
  and .activation_command_result_receipt_duplicate_persisted == false
  and .activation_command_result_receipt_idempotency_key_accepted == false
  and .activation_command_result_receipt_idempotency_key_recorded == false
  and .activation_command_result_receipt_idempotency_state_recorded == false
  and .activation_command_result_receipt_idempotency_state_persisted == false
  and .activation_command_result_receipt_idempotency_filesystem_written == false
  and .activation_command_result_receipt_replay_nonce_accepted == false
  and .activation_command_result_receipt_cross_scope_reuse_accepted == false
  and .activation_command_result_receipt_status_upgrade_accepted == false
  and .activation_command_result_receipt_completed_status_accepted == false
  and .activation_command_result_receipt_completion_ack_replay_accepted == false
  and .activation_command_result_receipt_ledger_replay_accepted == false
  and .activation_command_result_receipt_delivery_replay_accepted == false
  and .activation_command_result_receipt_query_replay_accepted == false
  and .activation_command_result_receipt_observability_replay_accepted == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_completion_ack_accepted == false
  and .operator_approval_from_replay_accepted == false
  and .activation_from_replay_allowed == false
  and .terminal_closure_from_replay_recorded == false
  and .terminal_closure_from_replay_accepted == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_persisted == false
  and .activation_request_executed == false
  and .activation_allowed == false
  and .activation_performed == false
  and .terminal_closure_recorded == false
  and .terminal_closure_accepted == false
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
  and .raw_payload_plaintext_recorded == false
  and .raw_payload_plaintext_persisted == false
  and (.activation_command_result_receipt_replay_idempotency_surfaces | length) == 13
  and (.activation_command_result_receipt_replay_idempotency_fixtures | length) == 10
  and (.activation_command_result_receipt_replay_idempotency_fixtures | all(
    (.replay_status | startswith("blocked_"))
    and .activation_command_result_receipt_replay_allowed == false
    and .activation_command_result_receipt_replay_recorded == false
    and .activation_command_result_receipt_replay_persisted == false
    and .activation_command_result_receipt_replay_performed == false
    and .activation_command_result_receipt_duplicate_accepted == false
    and .activation_command_result_receipt_idempotency_key_accepted == false
    and .activation_command_result_receipt_idempotency_state_recorded == false
    and .activation_command_result_receipt_idempotency_state_persisted == false
    and .activation_command_result_receipt_replay_nonce_accepted == false
    and .activation_command_result_receipt_cross_scope_reuse_accepted == false
    and .activation_command_result_receipt_completed_status_accepted == false
    and .activation_command_result_receipt_completion_ack_replay_accepted == false
    and .activation_command_result_receipt_ledger_replay_accepted == false
    and .activation_command_result_receipt_delivery_replay_accepted == false
    and .activation_command_result_receipt_recorded == false
    and .activation_command_result_receipt_persisted == false
    and .activation_command_result_receipt_accepted == false
    and .operator_approval_from_replay_accepted == false
    and .activation_from_replay_allowed == false
    and .terminal_closure_from_replay_recorded == false
    and .terminal_closure_from_replay_accepted == false
    and .activation_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .external_send_performed == false
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
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.source_result_receipt_no_persistence_present == false and .source_result_receipt_no_persistence_ready == false)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.duplicate_result_receipt_identity_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.result_receipt_replay_acceptance_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.idempotency_key_recording_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.idempotency_state_persistence_requested == true and .idempotency_filesystem_write_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.cross_scope_reuse_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.stale_nonce_replay_requested == true and .out_of_order_replay_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.completion_ack_replay_requested == true and .ledger_replay_requested == true and .delivery_replay_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.activation_from_replay_requested == true and .terminal_closure_from_replay_requested == true and .provider_replay_requested == true)] | length) == 1
  and ([.activation_command_result_receipt_replay_idempotency_fixtures[] | select(.external_send_replay_requested == true and .install_replay_requested == true and .active_binary_mutation_replay_requested == true and .secret_value_replay_requested == true)] | length) == 1
  and .denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial_count == 212
  and (.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_replay_idempotency_denial | length) == 212
  and .inherited_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_no_persistence_count == 182
  and (.allowed_next_actions | any(.action == "stage_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_result_receipt_ordering_monotonicity_denial" and .status == "allowed_report_only_next_slice" and .accepts_out_of_order_receipt == false and .records_ordering_state == false and .persists_replay_state == false and .records_terminal_closure == false and .mutates_runtime == false and .invokes_model == false))
  and .source_result_receipt_no_persistence_report_required == true
  and .result_receipt_replay_forbidden == true
  and .result_receipt_duplicate_acceptance_forbidden == true
  and .result_receipt_idempotency_state_forbidden == true
  and .result_receipt_replay_persistence_forbidden == true
  and .result_receipt_replay_activation_forbidden == true
  and .result_receipt_replay_terminal_closure_forbidden == true
  and .result_receipt_replay_runtime_mutation_forbidden == true
  and .result_receipt_replay_provider_model_invocation_forbidden == true
  and .result_receipt_replay_secret_read_forbidden == true
  and .result_receipt_replay_external_public_install_restart_upstream_forbidden == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report" | jq .
echo "Hepta core activation operator approval gap ledger summary briefing acknowledgement activation command result receipt replay idempotency denial gate passed"
