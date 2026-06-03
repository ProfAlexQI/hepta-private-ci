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

ACKNOWLEDGEMENT_JSON="$(
  capture_json_report \
    "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-non-acceptance-gate" \
    env \
      HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS="${HEPTA_TERMINAL_SOAK_INTERVAL_SECONDS:-0}" \
      HEPTA_LONG_SOAK_OBSERVATION_INTERVAL_SECONDS="${HEPTA_LONG_SOAK_OBSERVATION_INTERVAL_SECONDS:-0}" \
      HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS="${HEPTA_LONG_SOAK_FRESHNESS_OBSERVATION_INTERVAL_SECONDS:-0}" \
      scripts/hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-non-acceptance-gate.sh
)"

activation_request_fixtures_json="$(
  jq -n '
    def blocked_fixture($id; $status; $reason; $extra):
      {
        id: $id,
        activation_request_status: $status,
        source_summary_briefing_acknowledgement_non_acceptance_present: true,
        source_summary_briefing_acknowledgement_non_acceptance_ready: true,
        activation_request_requested: false,
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
        activation_nonce_accepted: false,
        activation_generation_accepted: false,
        acknowledgement_accepted: false,
        acknowledgement_recorded: false,
        acknowledgement_persisted: false,
        operator_approval_recorded: false,
        operator_approval_accepted: false,
        operator_identity_accepted: false,
        operator_signature_accepted: false,
        operator_timestamp_accepted: false,
        trusted_record_accepted: false,
        fresh_evidence_accepted: false,
        receipt_recorded: false,
        receipt_persisted: false,
        receipt_accepted: false,
        receipt_materialized: false,
        receipt_filesystem_written: false,
        ledger_recorded: false,
        index_delivered: false,
        completion_ack_recorded: false,
        completion_ack_persisted: false,
        completion_ack_accepted: false,
        completion_ack_delivered: false,
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
        activation_request_noop_confirmed: true,
        reason: $reason
      } + $extra;
    [
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-request-missing-source";
        "blocked_noop";
        "source_summary_briefing_acknowledgement_non_acceptance_report_required";
        {
          source_summary_briefing_acknowledgement_non_acceptance_present: false,
          source_summary_briefing_acknowledgement_non_acceptance_ready: false,
          activation_request_requested: true
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-activation-request";
        "blocked_activation_noop";
        "activation_request_shape_denied";
        {activation_request_requested: true}
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-operator-identity-activation-request";
        "blocked_identity_noop";
        "operator_identity_signature_timestamp_activation_denied";
        {
          activation_request_requested: true,
          operator_identity_acceptance_requested: true,
          operator_signature_acceptance_requested: true,
          operator_timestamp_acceptance_requested: true
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-nonce-generation-activation-request";
        "blocked_nonce_noop";
        "activation_nonce_generation_denied";
        {
          activation_request_requested: true,
          activation_nonce_requested: true,
          activation_generation_requested: true
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-trusted-record-evidence-activation-request";
        "blocked_evidence_noop";
        "trusted_record_fresh_evidence_activation_denied";
        {
          activation_request_requested: true,
          trusted_record_acceptance_requested: true,
          fresh_evidence_acceptance_requested: true
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-receipt-ledger-index-activation-request";
        "blocked_receipt_ledger_noop";
        "receipt_ledger_index_activation_denied";
        {
          activation_request_requested: true,
          receipt_record_requested: true,
          receipt_persist_requested: true,
          receipt_accept_requested: true,
          ledger_record_requested: true,
          index_delivery_requested: true
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-completion-ack-activation-request";
        "blocked_completion_ack_noop";
        "completion_ack_activation_denied";
        {
          activation_request_requested: true,
          completion_ack_record_requested: true,
          completion_ack_acceptance_requested: true,
          completion_ack_delivery_requested: true
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-terminal-closure-activation-request";
        "blocked_terminal_closure_noop";
        "terminal_closure_final_state_activation_denied";
        {
          activation_request_requested: true,
          terminal_closure_record_requested: true,
          terminal_closure_acceptance_requested: true,
          final_state_promotion_requested: true,
          completion_promotion_requested: true
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-runtime-provider-activation-request";
        "blocked_runtime_provider_noop";
        "runtime_context_provider_model_activation_denied";
        {
          activation_request_requested: true,
          runtime_attachment_requested: true,
          live_context_attachment_requested: true,
          context_injection_requested: true,
          provider_invocation_requested: true,
          model_invocation_requested: true
        }
      ),
      blocked_fixture(
        "operator-approval-gap-ledger-summary-briefing-ack-public-install-upstream-secret-activation-request";
        "blocked_external_noop";
        "external_public_install_restart_upstream_secret_activation_denied";
        {
          activation_request_requested: true,
          external_send_requested: true,
          public_claim_requested: true,
          release_artifact_requested: true,
          install_requested: true,
          service_restart_requested: true,
          upstream_merge_requested: true,
          secret_read_requested: true
        }
      )
    ]
  '
)"

acknowledgement_report_sha256="$(sha256_text "$ACKNOWLEDGEMENT_JSON")"
activation_request_fixtures_sha256="$(sha256_text "$activation_request_fixtures_json")"
activation_request_contract_hash_sha256="$(
  sha256_text "hepta-core-activation-operator-approval-gap-ledger-summary-briefing-acknowledgement-activation-request-denial:$acknowledgement_report_sha256:$activation_request_fixtures_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
activation_request_policy_hash_sha256="$(
  sha256_text "operator-gap-ledger-summary-briefing-acknowledgement-activation-request-denial:report-only:no-accept:no-record:no-persist:no-terminal-closure:no-activation:no-release"
)"
side_effect_hash_sha256="$(
  sha256_text "activation_request=false;operator_approval=false;trusted_record=false;receipt=false;ledger=false;completion_ack=false;terminal_closure=false;activation=false;release=false;install=false;secret=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$ACKNOWLEDGEMENT_JSON" \
  --argjson fixtures "$activation_request_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance_gate"
    and $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_ready == true
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_status == "blocked"
    and $source.source_operator_approval_gap_ledger_summary_briefing_gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_non_persistence_gate"
    and $source.source_operator_approval_gap_ledger_summary_briefing_ready == true
    and $source.source_operator_approval_gap_ledger_summary_briefing_status == "blocked"
    and $source.source_operator_approval_gap_ledger_item_count == 16
    and $source.source_missing_operator_approval_gap_ledger_item_count == 16
    and $source.source_denied_by_operator_approval_gap_ledger_summary_briefing_count == 81
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count == 10
    and $source.blocked_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count == 10
    and $source.noop_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count == 10
    and $source.allowed_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count == 0
    and $source.accepted_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count == 0
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_performed_count == 0
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_allowed == false
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_accepted == false
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_recorded == false
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_persisted == false
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_materialized == false
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_filesystem_written == false
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_delivered == false
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_identity_accepted == false
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_signature_accepted == false
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_final_state_promoted == false
    and $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_completion_promoted == false
    and $source.operator_approval_recorded == false
    and $source.operator_approval_accepted == false
    and $source.operator_identity_accepted == false
    and $source.trusted_record_accepted == false
    and $source.fresh_evidence_accepted == false
    and $source.activation_request_recorded == false
    and $source.receipt_recorded == false
    and $source.receipt_persisted == false
    and $source.receipt_accepted == false
    and $source.ledger_recorded == false
    and $source.index_delivered == false
    and $source.completion_ack_accepted == false
    and $source.terminal_closure_recorded == false
    and $source.terminal_closure_accepted == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.public_release_claim_allowed == false
    and $source.release_artifact_write_allowed == false
    and $source.provider_model_invocation_allowed == false
    and $source.channel_delivery_allowed == false
    and $source.install_restart_allowed == false
    and $source.active_binary_mutation_allowed == false
    and $source.upstream_fetch_merge_allowed == false
    and $source.credential_read_allowed == false
    and $source.secret_value_read_allowed == false
    and $source.telegram_send_performed == false
    and $source.external_send_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.release_artifact_written == false
    and $source.public_release_claimed == false
    and $source.install_executed == false
    and $source.launchd_mutated == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and $source.upstream_fetch_performed == false
    and $source.upstream_merge_performed == false
    and $source.credential_read == false
    and $source.secret_value_read == false
    and ($source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance | length) == 99
    and ($source.side_effects | to_entries | all(.value == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.activation_request_status == "blocked_noop" or .activation_request_status == "blocked_activation_noop" or .activation_request_status == "blocked_identity_noop" or .activation_request_status == "blocked_nonce_noop" or .activation_request_status == "blocked_evidence_noop" or .activation_request_status == "blocked_receipt_ledger_noop" or .activation_request_status == "blocked_completion_ack_noop" or .activation_request_status == "blocked_terminal_closure_noop" or .activation_request_status == "blocked_runtime_provider_noop" or .activation_request_status == "blocked_external_noop")
      and .activation_request_allowed == false
      and .activation_request_accepted == false
      and .activation_request_recorded == false
      and .activation_request_persisted == false
      and .activation_request_executed == false
      and .activation_allowed == false
      and .activation_performed == false
      and .operator_approval_recorded == false
      and .operator_approval_accepted == false
      and .trusted_record_accepted == false
      and .fresh_evidence_accepted == false
      and .receipt_recorded == false
      and .receipt_persisted == false
      and .receipt_accepted == false
      and .ledger_recorded == false
      and .completion_ack_accepted == false
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
      and .activation_request_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_gate" \
    --arg acknowledgement_report_sha256 "$acknowledgement_report_sha256" \
    --arg activation_request_fixtures_sha256 "$activation_request_fixtures_sha256" \
    --arg activation_request_contract_hash_sha256 "$activation_request_contract_hash_sha256" \
    --arg activation_request_policy_hash_sha256 "$activation_request_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ACKNOWLEDGEMENT_JSON" \
    --argjson fixtures "$activation_request_fixtures_json" \
    '{
      product: $product,
      runtime: $runtime,
      status: "ready",
      base_url: $base_url,
      gate: $gate,
      schema_version: "core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_v1",
      mode: "summary_briefing_acknowledgement_activation_request_denial_no_accept_no_record_no_execute",
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_gate: $source.gate,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_ready: $source.core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_ready,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_status: $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_status,
      source_operator_approval_gap_ledger_summary_briefing_acknowledgement_report_sha256: $acknowledgement_report_sha256,
      source_operator_approval_gap_ledger_summary_briefing_gate: $source.source_operator_approval_gap_ledger_summary_briefing_gate,
      source_operator_approval_gap_ledger_summary_briefing_status: $source.source_operator_approval_gap_ledger_summary_briefing_status,
      source_operator_approval_gap_ledger_item_count: $source.source_operator_approval_gap_ledger_item_count,
      source_missing_operator_approval_gap_ledger_item_count: $source.source_missing_operator_approval_gap_ledger_item_count,
      source_denied_by_operator_approval_gap_ledger_summary_briefing_count: $source.source_denied_by_operator_approval_gap_ledger_summary_briefing_count,
      activation_request_fixtures_sha256: $activation_request_fixtures_sha256,
      activation_request_contract_hash_sha256: $activation_request_contract_hash_sha256,
      activation_request_policy_hash_sha256: $activation_request_policy_hash_sha256,
      side_effect_hash_sha256: $side_effect_hash_sha256,
      minimum_required_samples: $min_long_soak_samples,
      core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_ready: true,
      operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_status: "blocked",
      acknowledgement_fixture_count: $source.operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count,
      blocked_acknowledgement_fixture_count: $source.blocked_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count,
      noop_acknowledgement_fixture_count: $source.noop_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count,
      allowed_acknowledgement_fixture_count: $source.allowed_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count,
      accepted_acknowledgement_fixture_count: $source.accepted_operator_approval_gap_ledger_summary_briefing_acknowledgement_fixture_count,
      source_acknowledgement_denied_count: $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance_count,
      activation_request_surface_count: 12,
      activation_request_surface_ready_count: 12,
      activation_request_side_effect_free_surface_count: 12,
      activation_request_fixture_count: ($fixtures | length),
      blocked_activation_request_fixture_count: ($fixtures | length),
      noop_activation_request_fixture_count: ($fixtures | length),
      allowed_activation_request_fixture_count: 0,
      accepted_activation_request_fixture_count: 0,
      activation_request_denied_count: 10,
      activation_request_performed_count: 0,
      activation_execution_performed_count: 0,
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
      activation_nonce_accepted: false,
      activation_generation_accepted: false,
      acknowledgement_accepted: false,
      acknowledgement_recorded: false,
      acknowledgement_persisted: false,
      operator_approval_recorded: false,
      operator_approval_accepted: false,
      operator_identity_accepted: false,
      operator_signature_accepted: false,
      operator_timestamp_accepted: false,
      trusted_record_accepted: false,
      fresh_evidence_accepted: false,
      receipt_recorded: false,
      receipt_persisted: false,
      receipt_accepted: false,
      receipt_materialized: false,
      receipt_filesystem_written: false,
      ledger_recorded: false,
      index_delivered: false,
      completion_ack_recorded: false,
      completion_ack_persisted: false,
      completion_ack_accepted: false,
      completion_ack_delivered: false,
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
      activation_request_surfaces: [
        "source_summary_briefing_acknowledgement_non_acceptance_report_required",
        "activation_request_acceptance_denied",
        "activation_request_recording_denied",
        "activation_request_persistence_denied",
        "operator_identity_signature_timestamp_activation_denied",
        "activation_nonce_generation_denied",
        "trusted_record_fresh_evidence_activation_denied",
        "receipt_ledger_index_activation_denied",
        "completion_ack_activation_denied",
        "terminal_closure_final_state_activation_denied",
        "runtime_context_provider_model_activation_denied",
        "external_public_install_restart_upstream_secret_activation_denied"
      ],
      activation_request_fixtures: $fixtures,
      denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix: (
        $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance + [
          "source_summary_briefing_acknowledgement_non_acceptance_report_required",
          "activation_request_acceptance_denied",
          "activation_request_recording_denied",
          "activation_request_persistence_denied",
          "activation_request_materialization_denied",
          "activation_request_filesystem_write_denied",
          "activation_request_delivery_denied",
          "activation_request_execution_denied",
          "activation_request_activation_denied",
          "operator_identity_signature_timestamp_activation_denied",
          "activation_nonce_generation_denied",
          "trusted_record_fresh_evidence_activation_denied",
          "receipt_ledger_index_activation_denied",
          "completion_ack_activation_denied",
          "terminal_closure_final_state_activation_denied",
          "runtime_context_provider_model_activation_denied",
          "external_public_install_restart_upstream_secret_activation_denied",
          "acknowledgement_cannot_promote_activation_request_authority"
        ]
      ),
      denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_count: (
        ($source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance | length) + 18
      ),
      inherited_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_count: (
        $source.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_non_acceptance | length
      ),
      allowed_next_actions: [
        {
          action: "review_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix",
          status: "allowed_report_only",
          accepts_activation_request: false,
          records_activation_request: false,
          executes_activation: false,
          records_terminal_closure: false,
          mutates_runtime: false
        },
        {
          action: "stage_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff",
          status: "allowed_report_only_next_slice",
          accepts_activation_request: false,
          records_activation_request: false,
          executes_activation: false,
          records_terminal_closure: false,
          writes_release_artifact: false
        },
        {
          action: "run_full_light_preflight",
          status: "allowed_verification_only",
          mutates_runtime: false,
          attaches_live_context: false,
          invokes_model: false,
          writes_kg: false
        }
      ],
      source_summary_briefing_acknowledgement_non_acceptance_report_required: true,
      activation_request_acceptance_forbidden: true,
      activation_request_recording_forbidden: true,
      activation_request_persistence_forbidden: true,
      activation_request_execution_forbidden: true,
      terminal_closure_promotion_forbidden: true,
      activation_forbidden: true,
      public_release_claim_forbidden: true,
      release_artifact_write_forbidden: true,
      install_restart_forbidden: true,
      upstream_fetch_merge_forbidden: true,
      credential_secret_read_forbidden: true,
      side_effects: {
        workspace_written: false,
        filesystem_written: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_materialized: false,
        activation_request_filesystem_written: false,
        activation_request_delivered: false,
        activation_request_executed: false,
        activation_performed: false,
        activation_nonce_accepted: false,
        activation_generation_accepted: false,
        acknowledgement_recorded: false,
        acknowledgement_persisted: false,
        acknowledgement_accepted: false,
        operator_approval_recorded: false,
        operator_approval_accepted: false,
        operator_identity_accepted: false,
        operator_signature_accepted: false,
        trusted_record_accepted: false,
        fresh_evidence_accepted: false,
        receipt_recorded: false,
        receipt_persisted: false,
        receipt_accepted: false,
        ledger_recorded: false,
        index_delivered: false,
        completion_ack_recorded: false,
        completion_ack_accepted: false,
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
        secret_value_read: false
      }
    }'
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_gate"
  and .core_activation_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_ready == true
  and .operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_status == "blocked"
  and .source_operator_approval_gap_ledger_summary_briefing_acknowledgement_ready == true
  and .source_operator_approval_gap_ledger_summary_briefing_acknowledgement_status == "blocked"
  and .source_missing_operator_approval_gap_ledger_item_count == 16
  and .source_denied_by_operator_approval_gap_ledger_summary_briefing_count == 81
  and .acknowledgement_fixture_count == 10
  and .blocked_acknowledgement_fixture_count == 10
  and .noop_acknowledgement_fixture_count == 10
  and .allowed_acknowledgement_fixture_count == 0
  and .accepted_acknowledgement_fixture_count == 0
  and .source_acknowledgement_denied_count == 99
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
  and .activation_request_allowed == false
  and .activation_request_accepted == false
  and .activation_request_recorded == false
  and .activation_request_persisted == false
  and .activation_request_executed == false
  and .activation_allowed == false
  and .activation_performed == false
  and .operator_approval_recorded == false
  and .operator_approval_accepted == false
  and .trusted_record_accepted == false
  and .fresh_evidence_accepted == false
  and .receipt_recorded == false
  and .receipt_persisted == false
  and .receipt_accepted == false
  and .ledger_recorded == false
  and .completion_ack_accepted == false
  and .terminal_closure_recorded == false
  and .terminal_closure_accepted == false
  and .terminal_closure_final_state_promoted == false
  and .terminal_closure_completion_promoted == false
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
  and (.activation_request_surfaces | length) == 12
  and (.activation_request_fixtures | length) == 10
  and (.activation_request_fixtures | all(
    (.activation_request_status == "blocked_noop" or .activation_request_status == "blocked_activation_noop" or .activation_request_status == "blocked_identity_noop" or .activation_request_status == "blocked_nonce_noop" or .activation_request_status == "blocked_evidence_noop" or .activation_request_status == "blocked_receipt_ledger_noop" or .activation_request_status == "blocked_completion_ack_noop" or .activation_request_status == "blocked_terminal_closure_noop" or .activation_request_status == "blocked_runtime_provider_noop" or .activation_request_status == "blocked_external_noop")
    and .activation_request_recorded == false
    and .activation_request_persisted == false
    and .activation_request_executed == false
    and .activation_performed == false
    and .operator_approval_recorded == false
    and .trusted_record_accepted == false
    and .fresh_evidence_accepted == false
    and .receipt_recorded == false
    and .receipt_persisted == false
    and .ledger_recorded == false
    and .completion_ack_accepted == false
    and .terminal_closure_recorded == false
    and .terminal_closure_accepted == false
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
    and .activation_request_noop_confirmed == true
  ))
  and .denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix_count == 117
  and (.denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_request_denial_matrix | length) == 117
  and .inherited_denied_by_operator_approval_gap_ledger_summary_briefing_acknowledgement_count == 99
  and (.allowed_next_actions | any(.action == "stage_operator_approval_gap_ledger_summary_briefing_acknowledgement_activation_command_noop_handoff" and .status == "allowed_report_only_next_slice" and .accepts_activation_request == false and .records_activation_request == false and .executes_activation == false and .records_terminal_closure == false and .writes_release_artifact == false))
  and .source_summary_briefing_acknowledgement_non_acceptance_report_required == true
  and .activation_request_acceptance_forbidden == true
  and .activation_request_recording_forbidden == true
  and .activation_request_persistence_forbidden == true
  and .activation_request_execution_forbidden == true
  and .terminal_closure_promotion_forbidden == true
  and .activation_forbidden == true
  and .public_release_claim_forbidden == true
  and .release_artifact_write_forbidden == true
  and .install_restart_forbidden == true
  and .upstream_fetch_merge_forbidden == true
  and .credential_secret_read_forbidden == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report" | jq .
echo "Hepta core activation operator approval gap ledger summary briefing acknowledgement activation request denial matrix gate passed"
