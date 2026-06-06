#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"
RELEASE_BIN="${HEPTA_RELEASE_BIN:-${HEPTA_CODEX_RELEASE_BIN:-$HOME/.local/opt/hepta/bin/hepta}}"

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

CANCELLATION_SUPERSESSION_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_RELEASE_BIN="$RELEASE_BIN" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-cancellation-supersession-denial-gate.sh
)"

audit_trail_immutable_evidence_fixtures_json="$(
  jq -n '
    def audit_fixture($id; $status; $reason; $extra):
      {
        fixture_id: $id,
        audit_evidence_status: $status,
        source_cancellation_supersession_present: true,
        source_cancellation_supersession_ready: true,
        audit_trail_requested: true,
        immutable_evidence_requested: false,
        audit_trail_allowed: false,
        audit_trail_recorded: false,
        audit_trail_persisted: false,
        audit_trail_materialized: false,
        audit_trail_filesystem_written: false,
        immutable_evidence_allowed: false,
        immutable_evidence_recorded: false,
        immutable_evidence_persisted: false,
        immutable_evidence_materialized: false,
        immutable_evidence_filesystem_written: false,
        hash_chain_recorded: false,
        hash_chain_persisted: false,
        merkle_root_recorded: false,
        merkle_root_persisted: false,
        attestation_recorded: false,
        attestation_persisted: false,
        witness_recorded: false,
        witness_persisted: false,
        notary_recorded: false,
        notary_persisted: false,
        ledger_evidence_recorded: false,
        ledger_evidence_persisted: false,
        index_evidence_recorded: false,
        index_evidence_persisted: false,
        delivery_evidence_recorded: false,
        delivery_evidence_persisted: false,
        export_evidence_recorded: false,
        query_evidence_registered: false,
        observability_evidence_recorded: false,
        activation_command_result_receipt_cancellation_allowed: false,
        activation_command_result_receipt_cancellation_recorded: false,
        activation_command_result_receipt_cancellation_persisted: false,
        activation_command_result_receipt_supersession_allowed: false,
        activation_command_result_receipt_supersession_recorded: false,
        activation_command_result_receipt_supersession_persisted: false,
        activation_command_result_receipt_replacement_receipt_accepted: false,
        activation_command_result_receipt_replacement_receipt_recorded: false,
        activation_command_result_receipt_replacement_receipt_persisted: false,
        activation_command_result_receipt_tombstone_recorded: false,
        activation_command_result_receipt_delete_marker_recorded: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_command_result_receipt_materialized: false,
        activation_command_completion_ack_recorded: false,
        activation_command_completion_ack_persisted: false,
        activation_command_completion_ack_accepted: false,
        operator_approval_from_audit_trail_accepted: false,
        operator_approval_from_immutable_evidence_accepted: false,
        activation_from_audit_trail_allowed: false,
        activation_from_immutable_evidence_allowed: false,
        activation_from_cancellation_allowed: false,
        activation_from_supersession_allowed: false,
        activation_from_receipt_allowed: false,
        activation_command_allowed: false,
        activation_command_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_executed: false,
        operator_approval_recorded: false,
        dispatch_performed: false,
        execution_performed: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        provider_prompt_replayed: false,
        memory_store_write_performed: false,
        memory_store_mutated: false,
        external_kg_adapter_read_performed: false,
        live_kg_write_performed: false,
        readback_evidence_recorded: false,
        readback_evidence_persisted: false,
        credential_read: false,
        secret_file_read: false,
        auth_secret_read: false,
        secret_value_read: false,
        raw_payload_plaintext_recorded: false,
        raw_payload_plaintext_persisted: false,
        channel_send_performed: false,
        telegram_send_performed: false,
        external_send_performed: false,
        public_claim_performed: false,
        public_release_claimed: false,
        public_ga_claimed: false,
        release_artifact_written: false,
        install_performed: false,
        install_executed: false,
        launchd_mutated: false,
        service_restarted: false,
        service_restart_performed: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        rollback_executed: false,
        receipt_noop_confirmed: true,
        denial_reason: $reason
      } + $extra;
    [
      audit_fixture("missing-source-cancellation-supersession-report"; "blocked_noop"; "source_result_receipt_cancellation_supersession_report_required"; {source_cancellation_supersession_present: false, source_cancellation_supersession_ready: false}),
      audit_fixture("append-audit-trail-to-blocked-noop-result-receipt"; "blocked_audit_noop"; "audit_trail_append_request_denied"; {audit_trail_request_shape: "append_blocked_noop_result_receipt"}),
      audit_fixture("seal-blocked-noop-as-immutable-evidence"; "blocked_evidence_noop"; "immutable_evidence_packet_request_denied"; {immutable_evidence_requested: true, audit_trail_requested: false, immutable_evidence_request_shape: "seal_blocked_noop_result_receipt"}),
      audit_fixture("hash-chain-merkle-root-evidence-attempt"; "blocked_evidence_noop"; "hash_chain_merkle_root_recording_denied"; {immutable_evidence_requested: true, audit_trail_requested: false, hash_chain_requested: true, merkle_root_requested: true}),
      audit_fixture("attestation-witness-notary-evidence-attempt"; "blocked_evidence_noop"; "attestation_witness_notary_recording_denied"; {immutable_evidence_requested: true, audit_trail_requested: false, attestation_requested: true, witness_requested: true, notary_requested: true}),
      audit_fixture("audit-trail-materialization-filesystem-attempt"; "blocked_audit_noop"; "audit_trail_materialization_filesystem_denied"; {audit_trail_materialization_requested: true, audit_trail_filesystem_write_requested: true}),
      audit_fixture("ledger-index-delivery-export-query-observability-evidence-attempt"; "blocked_evidence_noop"; "ledger_index_delivery_export_query_observability_evidence_denied"; {ledger_evidence_requested: true, index_evidence_requested: true, delivery_evidence_requested: true, export_evidence_requested: true, query_evidence_requested: true, observability_evidence_requested: true}),
      audit_fixture("activation-from-audit-evidence-attempt"; "blocked_evidence_noop"; "activation_from_audit_evidence_denied"; {immutable_evidence_requested: true, audit_trail_requested: false, activation_from_audit_evidence_requested: true}),
      audit_fixture("context-provider-model-memory-kg-readback-evidence-attempt"; "blocked_evidence_noop"; "context_provider_model_memory_kg_readback_evidence_denied"; {immutable_evidence_requested: true, audit_trail_requested: false, context_evidence_requested: true, provider_prompt_evidence_requested: true, model_output_evidence_requested: true, memory_store_evidence_requested: true, external_kg_evidence_requested: true, live_kg_evidence_requested: true, readback_evidence_requested: true}),
      audit_fixture("rollback-secret-external-public-install-evidence-attempt"; "blocked_evidence_noop"; "rollback_secret_external_public_install_evidence_denied"; {immutable_evidence_requested: true, audit_trail_requested: false, rollback_evidence_requested: true, credential_secret_evidence_requested: true, external_send_evidence_requested: true, public_claim_evidence_requested: true, release_artifact_evidence_requested: true, install_evidence_requested: true, service_restart_evidence_requested: true, active_binary_mutation_evidence_requested: true, upstream_evidence_requested: true})
    ]
  '
)"

cancellation_supersession_report_sha256="$(sha256_text "$CANCELLATION_SUPERSESSION_JSON")"
cancellation_supersession_contract_hash_sha256="$(jq -r '.cancellation_supersession_contract_hash_sha256' <<<"$CANCELLATION_SUPERSESSION_JSON")"
cancellation_supersession_policy_hash_sha256="$(jq -r '.cancellation_supersession_policy_hash_sha256' <<<"$CANCELLATION_SUPERSESSION_JSON")"
source_ordering_monotonicity_report_sha256="$(jq -r '.source_operator_review_acknowledgement_activation_command_result_receipt_ordering_monotonicity_report_sha256' <<<"$CANCELLATION_SUPERSESSION_JSON")"
audit_trail_immutable_evidence_fixtures_sha256="$(sha256_text "$audit_trail_immutable_evidence_fixtures_json")"
audit_trail_immutable_evidence_contract_hash_sha256="$(
  sha256_text "hepta-canary-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial:v1:source=$cancellation_supersession_report_sha256:cancellation=$cancellation_supersession_contract_hash_sha256:ordering=$source_ordering_monotonicity_report_sha256:fixtures=$audit_trail_immutable_evidence_fixtures_sha256:audit=0:evidence=0:hashchain=0:authority=0:live=0"
)"
audit_trail_immutable_evidence_policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-operator-canary-harness-operator-review-acknowledgement-activation-command-result-receipt-audit-trail-immutable-evidence-denial:v1:no-audit-write:no-evidence-persist:no-hash-chain:no-attestation:no-authority:no-live"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_side_effects=false;fixtures=10;audit=0;evidence=0;hashchain=0;attestation=0;record=0;persist=0;activation=0;provider=0;model=0;memory=0;kg=0;secret=0"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$CANCELLATION_SUPERSESSION_JSON" \
  --argjson fixtures "$audit_trail_immutable_evidence_fixtures_json" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_gate"
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status == "blocked"
    and $source.cancellation_supersession_fixture_count == 10
    and $source.blocked_cancellation_supersession_fixture_count == 10
    and $source.noop_cancellation_supersession_fixture_count == 10
    and $source.allowed_cancellation_supersession_fixture_count == 0
    and $source.accepted_cancellation_supersession_fixture_count == 0
    and $source.cancellation_performed_count == 0
    and $source.supersession_performed_count == 0
    and $source.replacement_receipt_accepted_count == 0
    and $source.replacement_receipt_recorded_count == 0
    and $source.replacement_receipt_persisted_count == 0
    and $source.tombstone_recorded_count == 0
    and $source.delete_marker_recorded_count == 0
    and $source.activation_command_result_receipt_cancellation_allowed == false
    and $source.activation_command_result_receipt_cancellation_recorded == false
    and $source.activation_command_result_receipt_cancellation_persisted == false
    and $source.activation_command_result_receipt_cancellation_request_accepted == false
    and $source.activation_command_result_receipt_supersession_allowed == false
    and $source.activation_command_result_receipt_supersession_recorded == false
    and $source.activation_command_result_receipt_supersession_persisted == false
    and $source.activation_command_result_receipt_supersession_request_accepted == false
    and $source.activation_command_result_receipt_replacement_receipt_accepted == false
    and $source.activation_command_result_receipt_replacement_receipt_recorded == false
    and $source.activation_command_result_receipt_replacement_receipt_persisted == false
    and $source.activation_command_result_receipt_tombstone_recorded == false
    and $source.activation_command_result_receipt_delete_marker_recorded == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.operator_approval_from_cancellation_accepted == false
    and $source.operator_approval_from_supersession_accepted == false
    and $source.activation_from_cancellation_allowed == false
    and $source.activation_from_supersession_allowed == false
    and $source.activation_command_enabled == false
    and $source.activation_command_invoked == false
    and $source.activation_command_dispatched == false
    and $source.activation_request_accepted == false
    and $source.activation_request_executed == false
    and $source.dispatch_performed_count == 0
    and $source.execution_performed_count == 0
    and $source.context_injection_performed_count == 0
    and $source.provider_invoked_count == 0
    and $source.model_invoked_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.external_kg_adapter_read_performed_count == 0
    and $source.live_kg_write_performed_count == 0
    and $source.credential_read_count == 0
    and $source.secret_file_read_count == 0
    and $source.channel_send_performed_count == 0
    and $source.install_performed_count == 0
    and $source.service_restarted_count == 0
    and $source.active_binary_mutated_count == 0
    and $source.upstream_fetch_performed_count == 0
    and $source.upstream_merge_performed_count == 0
    and $source.canary_harness_armed == false
    and $source.canary_harness_executable == false
    and $source.canary_live_enabled == false
    and ($source.side_effects | to_entries | all(.value == false))
    and ($source.allowed_next_actions | any(.action == "stage_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial" and .status == "allowed_report_only_next_slice" and .writes_audit_trail == false and .persists_evidence == false and .mutates_runtime == false and .invokes_model == false and .writes_memory_or_kg == false))
    and ($fixtures | length) == 10
    and ($fixtures | all(
      (.audit_evidence_status | startswith("blocked"))
      and .audit_trail_allowed == false
      and .audit_trail_recorded == false
      and .audit_trail_persisted == false
      and .audit_trail_materialized == false
      and .immutable_evidence_allowed == false
      and .immutable_evidence_recorded == false
      and .immutable_evidence_persisted == false
      and .hash_chain_recorded == false
      and .merkle_root_recorded == false
      and .attestation_recorded == false
      and .witness_recorded == false
      and .notary_recorded == false
      and .activation_command_result_receipt_recorded == false
      and .activation_command_result_receipt_persisted == false
      and .activation_command_result_receipt_accepted == false
      and .operator_approval_from_audit_trail_accepted == false
      and .operator_approval_from_immutable_evidence_accepted == false
      and .activation_from_audit_trail_allowed == false
      and .activation_from_immutable_evidence_allowed == false
      and .activation_command_enabled == false
      and .dispatch_performed == false
      and .execution_performed == false
      and .context_injection_performed == false
      and .provider_invoked == false
      and .model_invoked == false
      and .memory_store_write_performed == false
      and .external_kg_adapter_read_performed == false
      and .live_kg_write_performed == false
      and .credential_read == false
      and .secret_file_read == false
      and .channel_send_performed == false
      and .install_performed == false
      and .service_restarted == false
      and .active_binary_mutated == false
      and .upstream_fetch_performed == false
      and .upstream_merge_performed == false
      and .receipt_noop_confirmed == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate" \
    --arg cancellation_supersession_report_sha256 "$cancellation_supersession_report_sha256" \
    --arg cancellation_supersession_contract_hash_sha256 "$cancellation_supersession_contract_hash_sha256" \
    --arg cancellation_supersession_policy_hash_sha256 "$cancellation_supersession_policy_hash_sha256" \
    --arg source_ordering_monotonicity_report_sha256 "$source_ordering_monotonicity_report_sha256" \
    --arg audit_trail_immutable_evidence_fixtures_sha256 "$audit_trail_immutable_evidence_fixtures_sha256" \
    --arg audit_trail_immutable_evidence_contract_hash_sha256 "$audit_trail_immutable_evidence_contract_hash_sha256" \
    --arg audit_trail_immutable_evidence_policy_hash_sha256 "$audit_trail_immutable_evidence_policy_hash_sha256" \
    --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$CANCELLATION_SUPERSESSION_JSON" \
    --argjson fixtures "$audit_trail_immutable_evidence_fixtures_json" \
    '
      ($source.denied_by_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession + [
        "source_result_receipt_cancellation_supersession_report_required",
        "audit_trail_request_acceptance_denied",
        "audit_trail_recording_denied",
        "audit_trail_persistence_denied",
        "audit_trail_materialization_denied",
        "audit_trail_filesystem_write_denied",
        "immutable_evidence_request_acceptance_denied",
        "immutable_evidence_recording_denied",
        "immutable_evidence_persistence_denied",
        "immutable_evidence_materialization_denied",
        "immutable_evidence_filesystem_write_denied",
        "hash_chain_recording_denied",
        "merkle_root_recording_denied",
        "attestation_recording_denied",
        "witness_recording_denied",
        "notary_recording_denied",
        "ledger_index_delivery_evidence_denied",
        "export_query_observability_evidence_denied",
        "activation_from_audit_trail_denied",
        "activation_from_immutable_evidence_denied",
        "operator_approval_from_audit_trail_denied",
        "operator_approval_from_immutable_evidence_denied",
        "context_provider_model_evidence_denied",
        "memory_kg_readback_evidence_denied",
        "rollback_secret_evidence_denied",
        "external_public_install_restart_active_binary_evidence_denied"
      ]) as $denials |
      {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_schema_version: "memory_intelligence_kg_operator_canary_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_v1",
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready: true,
        operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status: "blocked",
        audit_trail_immutable_evidence_mode: "stdout_only_audit_trail_immutable_evidence_denial_no_record_no_persist_no_authority_no_live",
        audit_trail_immutable_evidence_decision: "blocked_noop_activation_command_result_receipt_cannot_be_wrapped_as_audit_trail_or_immutable_evidence_authority",
        minimum_required_samples: $min_long_soak_samples,
        source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_gate: $source.gate,
        source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_status: $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_denial_status,
        source_operator_review_acknowledgement_activation_command_result_receipt_cancellation_supersession_report_sha256: $cancellation_supersession_report_sha256,
        source_cancellation_supersession_contract_hash_sha256: $cancellation_supersession_contract_hash_sha256,
        source_cancellation_supersession_policy_hash_sha256: $cancellation_supersession_policy_hash_sha256,
        source_ordering_monotonicity_report_sha256: $source_ordering_monotonicity_report_sha256,
        audit_trail_immutable_evidence_fixtures_sha256: $audit_trail_immutable_evidence_fixtures_sha256,
        audit_trail_immutable_evidence_contract_hash_sha256: $audit_trail_immutable_evidence_contract_hash_sha256,
        audit_trail_immutable_evidence_policy_hash_sha256: $audit_trail_immutable_evidence_policy_hash_sha256,
        side_effect_hash_sha256: $side_effect_hash_sha256,
        source_cancellation_supersession_fixture_count: $source.cancellation_supersession_fixture_count,
        source_blocked_cancellation_supersession_fixture_count: $source.blocked_cancellation_supersession_fixture_count,
        source_noop_cancellation_supersession_fixture_count: $source.noop_cancellation_supersession_fixture_count,
        source_accepted_cancellation_supersession_fixture_count: $source.accepted_cancellation_supersession_fixture_count,
        source_cancellation_performed_count: $source.cancellation_performed_count,
        source_supersession_performed_count: $source.supersession_performed_count,
        source_replacement_receipt_accepted_count: $source.replacement_receipt_accepted_count,
        source_replacement_receipt_recorded_count: $source.replacement_receipt_recorded_count,
        source_replacement_receipt_persisted_count: $source.replacement_receipt_persisted_count,
        audit_trail_immutable_evidence_surface_count: 12,
        audit_trail_immutable_evidence_surface_ready_count: 12,
        audit_trail_immutable_evidence_side_effect_free_surface_count: 12,
        audit_trail_immutable_evidence_fixtures: $fixtures,
        audit_trail_immutable_evidence_fixture_count: ($fixtures | length),
        blocked_audit_trail_immutable_evidence_fixture_count: ($fixtures | length),
        noop_audit_trail_immutable_evidence_fixture_count: ($fixtures | length),
        allowed_audit_trail_immutable_evidence_fixture_count: 0,
        accepted_audit_trail_immutable_evidence_fixture_count: 0,
        audit_trail_denied_count: ($fixtures | map(select(.audit_trail_requested == true)) | length),
        immutable_evidence_denied_count: ($fixtures | map(select(.immutable_evidence_requested == true)) | length),
        audit_trail_performed_count: 0,
        immutable_evidence_performed_count: 0,
        hash_chain_recorded_count: 0,
        merkle_root_recorded_count: 0,
        attestation_recorded_count: 0,
        witness_recorded_count: 0,
        notary_recorded_count: 0,
        ledger_evidence_recorded_count: 0,
        index_evidence_recorded_count: 0,
        delivery_evidence_recorded_count: 0,
        activation_command_result_receipt_audit_trail_allowed: false,
        activation_command_result_receipt_audit_trail_recorded: false,
        activation_command_result_receipt_audit_trail_persisted: false,
        activation_command_result_receipt_audit_trail_materialized: false,
        activation_command_result_receipt_audit_trail_filesystem_written: false,
        activation_command_result_receipt_immutable_evidence_allowed: false,
        activation_command_result_receipt_immutable_evidence_recorded: false,
        activation_command_result_receipt_immutable_evidence_persisted: false,
        activation_command_result_receipt_immutable_evidence_materialized: false,
        activation_command_result_receipt_immutable_evidence_filesystem_written: false,
        activation_command_result_receipt_hash_chain_recorded: false,
        activation_command_result_receipt_hash_chain_persisted: false,
        activation_command_result_receipt_merkle_root_recorded: false,
        activation_command_result_receipt_merkle_root_persisted: false,
        activation_command_result_receipt_attestation_recorded: false,
        activation_command_result_receipt_attestation_persisted: false,
        activation_command_result_receipt_witness_recorded: false,
        activation_command_result_receipt_witness_persisted: false,
        activation_command_result_receipt_notary_recorded: false,
        activation_command_result_receipt_notary_persisted: false,
        activation_command_result_receipt_ledger_evidence_recorded: false,
        activation_command_result_receipt_ledger_evidence_persisted: false,
        activation_command_result_receipt_index_evidence_recorded: false,
        activation_command_result_receipt_index_evidence_persisted: false,
        activation_command_result_receipt_delivery_evidence_recorded: false,
        activation_command_result_receipt_delivery_evidence_persisted: false,
        activation_command_result_receipt_cancellation_recorded: false,
        activation_command_result_receipt_supersession_recorded: false,
        activation_command_result_receipt_replacement_receipt_recorded: false,
        activation_command_result_receipt_tombstone_recorded: false,
        activation_command_result_receipt_delete_marker_recorded: false,
        activation_command_result_receipt_recorded: false,
        activation_command_result_receipt_persisted: false,
        activation_command_result_receipt_accepted: false,
        activation_command_completion_ack_recorded: false,
        activation_command_completion_ack_accepted: false,
        operator_approval_from_audit_trail_accepted: false,
        operator_approval_from_immutable_evidence_accepted: false,
        operator_approval_from_cancellation_accepted: false,
        operator_approval_from_supersession_accepted: false,
        activation_from_audit_trail_allowed: false,
        activation_from_immutable_evidence_allowed: false,
        activation_from_cancellation_allowed: false,
        activation_from_supersession_allowed: false,
        activation_from_receipt_allowed: false,
        activation_command_allowed: false,
        activation_command_accepted: false,
        activation_command_enabled: false,
        activation_command_invoked: false,
        activation_command_dispatched: false,
        activation_request_accepted: false,
        activation_request_recorded: false,
        activation_request_persisted: false,
        activation_request_executed: false,
        operator_approval_recorded: false,
        dispatch_performed_count: 0,
        execution_performed_count: 0,
        context_injection_performed_count: 0,
        provider_invoked_count: 0,
        model_invoked_count: 0,
        memory_store_write_performed_count: 0,
        external_kg_adapter_read_performed_count: 0,
        live_kg_write_performed_count: 0,
        readback_evidence_recorded_count: 0,
        credential_read_count: 0,
        secret_file_read_count: 0,
        channel_send_performed_count: 0,
        install_performed_count: 0,
        service_restarted_count: 0,
        active_binary_mutated_count: 0,
        upstream_fetch_performed_count: 0,
        upstream_merge_performed_count: 0,
        canary_harness_armed: false,
        canary_harness_executable: false,
        canary_live_enabled: false,
        allowed_next_actions: [
          {
            action: "review_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial",
            status: "allowed_report_only",
            writes_audit_trail: false,
            persists_evidence: false,
            mutates_runtime: false,
            invokes_model: false,
            writes_memory_or_kg: false
          },
          {
            action: "stage_operator_review_acknowledgement_activation_command_result_receipt_retention_expiry_garbage_collection_denial",
            status: "allowed_report_only_next_slice",
            writes_audit_trail: false,
            persists_evidence: false,
            performs_retention: false,
            performs_gc: false,
            mutates_runtime: false,
            invokes_model: false,
            writes_memory_or_kg: false
          }
        ],
        denied_by_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence: $denials,
        denied_by_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_count: ($denials | length),
        side_effects: {
          workspace_written: false,
          filesystem_written: false,
          activation_command_result_receipt_audit_trail_recorded: false,
          activation_command_result_receipt_audit_trail_persisted: false,
          activation_command_result_receipt_audit_trail_materialized: false,
          activation_command_result_receipt_audit_trail_filesystem_written: false,
          activation_command_result_receipt_immutable_evidence_recorded: false,
          activation_command_result_receipt_immutable_evidence_persisted: false,
          activation_command_result_receipt_immutable_evidence_materialized: false,
          activation_command_result_receipt_immutable_evidence_filesystem_written: false,
          activation_command_result_receipt_hash_chain_recorded: false,
          activation_command_result_receipt_hash_chain_persisted: false,
          activation_command_result_receipt_merkle_root_recorded: false,
          activation_command_result_receipt_merkle_root_persisted: false,
          activation_command_result_receipt_attestation_recorded: false,
          activation_command_result_receipt_attestation_persisted: false,
          activation_command_result_receipt_witness_recorded: false,
          activation_command_result_receipt_witness_persisted: false,
          activation_command_result_receipt_notary_recorded: false,
          activation_command_result_receipt_notary_persisted: false,
          activation_command_result_receipt_ledger_evidence_recorded: false,
          activation_command_result_receipt_index_evidence_recorded: false,
          activation_command_result_receipt_delivery_evidence_recorded: false,
          activation_command_result_receipt_cancellation_recorded: false,
          activation_command_result_receipt_supersession_recorded: false,
          activation_command_result_receipt_replacement_receipt_recorded: false,
          activation_command_result_receipt_tombstone_recorded: false,
          activation_command_result_receipt_delete_marker_recorded: false,
          activation_command_result_receipt_recorded: false,
          activation_command_result_receipt_persisted: false,
          activation_command_result_receipt_accepted: false,
          activation_command_completion_ack_recorded: false,
          activation_command_completion_ack_accepted: false,
          activation_command_enabled: false,
          activation_command_invoked: false,
          activation_command_dispatched: false,
          operator_approval_recorded: false,
          dispatch_performed: false,
          execution_performed: false,
          context_injection_performed: false,
          provider_invoked: false,
          model_invoked: false,
          provider_prompt_replayed: false,
          memory_store_write_performed: false,
          memory_store_mutated: false,
          external_kg_adapter_read_performed: false,
          live_kg_write_performed: false,
          readback_evidence_recorded: false,
          credential_read: false,
          secret_file_read: false,
          auth_secret_read: false,
          channel_send_performed: false,
          telegram_send_performed: false,
          external_send_performed: false,
          public_claim_performed: false,
          install_performed: false,
          service_restarted: false,
          active_binary_mutated: false,
          upstream_fetch_performed: false,
          upstream_merge_performed: false
        }
      }
    '
)"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_gate"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_denial_status == "blocked"
  and .source_cancellation_supersession_fixture_count == 10
  and .source_accepted_cancellation_supersession_fixture_count == 0
  and .source_cancellation_performed_count == 0
  and .source_supersession_performed_count == 0
  and .source_replacement_receipt_accepted_count == 0
  and .audit_trail_immutable_evidence_fixture_count == 10
  and .blocked_audit_trail_immutable_evidence_fixture_count == 10
  and .noop_audit_trail_immutable_evidence_fixture_count == 10
  and .allowed_audit_trail_immutable_evidence_fixture_count == 0
  and .accepted_audit_trail_immutable_evidence_fixture_count == 0
  and .audit_trail_performed_count == 0
  and .immutable_evidence_performed_count == 0
  and .hash_chain_recorded_count == 0
  and .merkle_root_recorded_count == 0
  and .attestation_recorded_count == 0
  and .witness_recorded_count == 0
  and .notary_recorded_count == 0
  and .activation_command_result_receipt_audit_trail_allowed == false
  and .activation_command_result_receipt_audit_trail_recorded == false
  and .activation_command_result_receipt_audit_trail_persisted == false
  and .activation_command_result_receipt_immutable_evidence_allowed == false
  and .activation_command_result_receipt_immutable_evidence_recorded == false
  and .activation_command_result_receipt_immutable_evidence_persisted == false
  and .activation_command_result_receipt_hash_chain_recorded == false
  and .activation_command_result_receipt_attestation_recorded == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .operator_approval_from_audit_trail_accepted == false
  and .operator_approval_from_immutable_evidence_accepted == false
  and .activation_from_audit_trail_allowed == false
  and .activation_from_immutable_evidence_allowed == false
  and .activation_command_enabled == false
  and .activation_command_invoked == false
  and .activation_command_dispatched == false
  and .activation_request_accepted == false
  and .activation_request_executed == false
  and .dispatch_performed_count == 0
  and .execution_performed_count == 0
  and .context_injection_performed_count == 0
  and .provider_invoked_count == 0
  and .model_invoked_count == 0
  and .memory_store_write_performed_count == 0
  and .external_kg_adapter_read_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .channel_send_performed_count == 0
  and .install_performed_count == 0
  and .service_restarted_count == 0
  and .active_binary_mutated_count == 0
  and .upstream_fetch_performed_count == 0
  and .upstream_merge_performed_count == 0
  and .canary_harness_armed == false
  and .canary_harness_executable == false
  and .canary_live_enabled == false
  and (.audit_trail_immutable_evidence_fixtures | all(
    (.audit_evidence_status | startswith("blocked"))
    and .audit_trail_recorded == false
    and .audit_trail_persisted == false
    and .immutable_evidence_recorded == false
    and .immutable_evidence_persisted == false
    and .hash_chain_recorded == false
    and .attestation_recorded == false
    and .activation_command_result_receipt_accepted == false
    and .operator_approval_from_audit_trail_accepted == false
    and .operator_approval_from_immutable_evidence_accepted == false
    and .provider_invoked == false
    and .model_invoked == false
    and .memory_store_write_performed == false
    and .external_kg_adapter_read_performed == false
    and .live_kg_write_performed == false
    and .credential_read == false
    and .secret_file_read == false
    and .receipt_noop_confirmed == true
  ))
  and .denied_by_operator_review_acknowledgement_activation_command_result_receipt_audit_trail_immutable_evidence_count >= 190
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG operator canary controlled request harness operator review acknowledgement activation command result receipt audit-trail immutable-evidence denial gate passed"
