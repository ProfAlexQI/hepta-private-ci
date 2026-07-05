#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-operator-facing-summary-briefing-non-persistence-denial-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-operator-facing-summary-briefing-non-persistence-denial-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_ready == true
    and $source.scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_accepted == true
    and $source.accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count == 1
    and $source.blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count == 9
    and $source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_count >= 60
    and $source.dry_run_execution_result_receipt_final_operator_acknowledgement_recorded == false
    and $source.dry_run_execution_result_receipt_terminal_operator_decision_recorded == false
    and $source.dry_run_execution_result_receipt_terminal_operator_status_recorded == false
    and $source.dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement == false
    and $source.dry_run_execution_executed == false
    and $source.production_durable_memory_store_write_performed == false
    and $source.memory_store_write_performed == false
    and $source.wal_write_performed == false
    and $source.receipt_persisted == false
    and $source.external_send_performed == false
    and $source.allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary"
    and $source.allowed_next_actions[1].accepts_operator_acknowledgement == false
    and $source.allowed_next_actions[1].records_terminal_decision == false
    and $source.allowed_next_actions[1].executes_dry_run == false
    and $source.allowed_next_actions[1].writes_production_durable_memory == false
  ' >/dev/null

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
source_operator_summary_briefing_boundary_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_operator_summary_briefing_policy_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_policy_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_operator_summary_briefing_result_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_operator_summary_briefing_result_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_operator_summary_briefing_handoff_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_operator_summary_briefing_handoff_hash_sha256 // ""' <<<"$SOURCE_JSON")"

final_acknowledgement_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-denial:v1:source=${source_operator_summary_briefing_result_hash_sha256}:request=false:accept=false:record=false:persist=false:materialize=false:deliver=false"
)"
final_acknowledgement_readback_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-readback-denial:v1:ack=${final_acknowledgement_denial_hash_sha256}:readback=false:receipt=false:persist=false"
)"
final_acknowledgement_receipt_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-receipt-denial:v1:readback=${final_acknowledgement_readback_denial_hash_sha256}:receipt-record=false:receipt-persist=false"
)"
terminal_decision_status_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-status-denial:v1:ack=${final_acknowledgement_denial_hash_sha256}:decision=false:status=false:promotion=false"
)"
final_acknowledgement_matrix_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-matrix:v1:ack=${final_acknowledgement_denial_hash_sha256}:readback=${final_acknowledgement_readback_denial_hash_sha256}:receipt=${final_acknowledgement_receipt_denial_hash_sha256}:terminal=${terminal_decision_status_denial_hash_sha256}:fixtures=10"
)"
final_acknowledgement_handoff_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-handoff:v1:matrix=${final_acknowledgement_matrix_hash_sha256}:next=terminal-operator-decision-public-claim-non-promotion-denial-boundary"
)"
final_acknowledgement_result_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-result:v1:ack=${final_acknowledgement_denial_hash_sha256}:terminal=${terminal_decision_status_denial_hash_sha256}:handoff=${final_acknowledgement_handoff_hash_sha256}:accepted=true:persist=false:delivery=false:authority=false:execution=false:production-write=false"
)"
final_acknowledgement_boundary_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary:v1:source=${source_report_sha256}:result=${final_acknowledgement_result_hash_sha256}:accepted=1:blocked=9:ack=false:terminal=false:authority=false:execution=false:production-write=false"
)"
final_acknowledgement_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-acknowledgement-non-acceptance-denial-policy:v1:bind-source-operator-summary-briefing-no-final-ack-request-acceptance-no-recording-no-persistence-no-materialization-no-filesystem-no-channel-no-readback-receipt-no-terminal-decision-status-no-authority-no-execution-no-production-write-no-kg-no-provider-no-release-no-install"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_gate" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary --json" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg source_operator_summary_briefing_boundary_hash_sha256 "$source_operator_summary_briefing_boundary_hash_sha256" \
  --arg source_operator_summary_briefing_policy_hash_sha256 "$source_operator_summary_briefing_policy_hash_sha256" \
  --arg source_operator_summary_briefing_result_hash_sha256 "$source_operator_summary_briefing_result_hash_sha256" \
  --arg source_operator_summary_briefing_handoff_hash_sha256 "$source_operator_summary_briefing_handoff_hash_sha256" \
  --arg final_acknowledgement_denial_hash_sha256 "$final_acknowledgement_denial_hash_sha256" \
  --arg final_acknowledgement_readback_denial_hash_sha256 "$final_acknowledgement_readback_denial_hash_sha256" \
  --arg final_acknowledgement_receipt_denial_hash_sha256 "$final_acknowledgement_receipt_denial_hash_sha256" \
  --arg terminal_decision_status_denial_hash_sha256 "$terminal_decision_status_denial_hash_sha256" \
  --arg final_acknowledgement_matrix_hash_sha256 "$final_acknowledgement_matrix_hash_sha256" \
  --arg final_acknowledgement_handoff_hash_sha256 "$final_acknowledgement_handoff_hash_sha256" \
  --arg final_acknowledgement_result_hash_sha256 "$final_acknowledgement_result_hash_sha256" \
  --arg final_acknowledgement_boundary_hash_sha256 "$final_acknowledgement_boundary_hash_sha256" \
  --arg final_acknowledgement_policy_hash_sha256 "$final_acknowledgement_policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  def fixture($id; $status; $reason; $accepted; $extra):
    {
      id:$id,
      fixture_id:$id,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_status:$status,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_accepted:$accepted,
      source_operator_summary_briefing_present:true,
      source_operator_summary_briefing_ready:true,
      final_acknowledgement_noop_confirmed:true,
      final_operator_acknowledgement_requested:false,
      acknowledgement_acceptance_requested:false,
      acknowledgement_recording_requested:false,
      acknowledgement_persistence_requested:false,
      acknowledgement_materialization_requested:false,
      acknowledgement_filesystem_write_requested:false,
      acknowledgement_readback_requested:false,
      acknowledgement_receipt_requested:false,
      acknowledgement_delivery_requested:false,
      telegram_send_requested:false,
      channel_delivery_requested:false,
      terminal_operator_decision_requested:false,
      terminal_operator_status_requested:false,
      terminal_operator_decision_promotion_requested:false,
      terminal_operator_status_promotion_requested:false,
      authority_promotion_requested:false,
      dry_run_execution_requested:false,
      production_write_requested:false,
      memory_write_acknowledgement_requested:false,
      rollback_acknowledgement_requested:false,
      kg_write_acknowledgement_requested:false,
      provider_prompt_acknowledgement_requested:false,
      credential_acknowledgement_requested:false,
      external_send_acknowledgement_requested:false,
      public_claim_acknowledgement_requested:false,
      release_artifact_acknowledgement_requested:false,
      install_acknowledgement_requested:false,
      service_restart_acknowledgement_requested:false,
      active_binary_acknowledgement_requested:false,
      acknowledgement_allowed:false,
      acknowledgement_request_accepted:false,
      acknowledgement_accepted:false,
      acknowledgement_recorded:false,
      acknowledgement_persisted:false,
      acknowledgement_materialized:false,
      acknowledgement_filesystem_written:false,
      acknowledgement_delivered:false,
      acknowledgement_channel_delivery_performed:false,
      acknowledgement_readback_recorded:false,
      acknowledgement_readback_persisted:false,
      acknowledgement_receipt_recorded:false,
      acknowledgement_receipt_persisted:false,
      terminal_operator_decision_recorded:false,
      terminal_operator_decision_persisted:false,
      terminal_operator_decision_accepted:false,
      terminal_operator_decision_promoted:false,
      terminal_operator_status_recorded:false,
      terminal_operator_status_persisted:false,
      terminal_operator_status_accepted:false,
      terminal_operator_status_promoted:false,
      authority_promoted:false,
      telegram_send_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      receipt_recorded:false,
      receipt_persisted:false,
      receipt_accepted:false,
      receipt_materialized:false,
      receipt_filesystem_written:false,
      dry_run_execution_executed:false,
      production_durable_memory_store_write_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      wal_write_performed:false,
      rollback_executed:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      release_artifact_written:false,
      install_executed:false,
      service_restarted:false,
      active_binary_mutated:false,
      reason:$reason
    } + $extra;
  ([
    "source_operator_summary_briefing_denial_boundary_required",
    "source_operator_summary_briefing_result_required",
    "final_operator_acknowledgement_request_denied",
    "final_operator_acknowledgement_acceptance_denied",
    "final_operator_acknowledgement_recording_denied",
    "final_operator_acknowledgement_persistence_materialization_denied",
    "final_operator_acknowledgement_readback_receipt_denied",
    "final_operator_acknowledgement_delivery_denied",
    "terminal_operator_decision_recording_denied",
    "terminal_operator_status_recording_denied",
    "final_operator_acknowledgement_authority_promotion_denied",
    "dry_run_execution_production_write_and_receipt_persistence_forbidden_on_final_acknowledgement_route",
    "kg_provider_channel_release_install_active_binary_acknowledgement_denied",
    "final_operator_acknowledgement_non_acceptance_handoff_bound"
  ]) as $surfaces
  | ([
    "source_operator_summary_briefing_denial_boundary_required",
    "source_operator_summary_briefing_result_hash_required",
    "source_operator_summary_briefing_policy_hash_required",
    "source_operator_summary_briefing_handoff_hash_required",
    "approved_production_namespace_required",
    "approved_production_store_required",
    "approved_production_scope_required",
    "production_durable_memory_target_required",
    "final_operator_acknowledgement_request_acceptance_denied",
    "final_operator_acknowledgement_recording_denied",
    "final_operator_acknowledgement_acceptance_denied",
    "final_operator_acknowledgement_persistence_denied",
    "final_operator_acknowledgement_materialization_denied",
    "final_operator_acknowledgement_filesystem_write_denied",
    "final_operator_acknowledgement_delivery_denied",
    "final_operator_acknowledgement_channel_delivery_denied",
    "final_operator_acknowledgement_readback_recording_denied",
    "final_operator_acknowledgement_readback_persistence_denied",
    "final_operator_acknowledgement_receipt_recording_denied",
    "final_operator_acknowledgement_receipt_persistence_denied",
    "terminal_operator_decision_recording_denied",
    "terminal_operator_decision_persistence_denied",
    "terminal_operator_decision_acceptance_denied",
    "terminal_operator_decision_promotion_denied",
    "terminal_operator_status_recording_denied",
    "terminal_operator_status_persistence_denied",
    "terminal_operator_status_acceptance_denied",
    "terminal_operator_status_promotion_denied",
    "result_receipt_final_acknowledgement_authority_promotion_denied",
    "result_receipt_terminal_decision_authority_promotion_denied",
    "dry_run_execution_execution_denied",
    "dry_run_execution_result_persistence_denied",
    "dry_run_execution_result_receipt_persistence_denied",
    "operator_summary_briefing_state_mutation_denied",
    "export_query_observability_state_mutation_denied",
    "retention_expiry_garbage_collection_state_mutation_denied",
    "audit_evidence_state_mutation_denied",
    "cancellation_supersession_state_mutation_denied",
    "ordering_monotonicity_state_mutation_denied",
    "replay_idempotency_state_mutation_denied",
    "production_write_execution_denied",
    "production_durable_memory_backend_write_denied",
    "durable_memory_backend_read_or_rollback_denied",
    "memory_store_mutation_denied",
    "wal_write_denied",
    "receipt_persistence_denied",
    "post_write_readback_denied",
    "rollback_execution_denied",
    "tombstone_write_denied",
    "raw_payload_plaintext_denied",
    "kg_live_write_denied",
    "provider_model_invocation_denied",
    "credential_secret_read_denied",
    "telegram_channel_delivery_denied",
    "external_send_denied",
    "release_public_artifact_write_denied",
    "install_authority_denied",
    "restart_authority_denied",
    "active_binary_mutation_denied",
    "unrestricted_full_live_activation_denied",
    "operator_final_acknowledgement_non_acceptance_denial_only",
    "operator_terminal_decision_public_claim_non_promotion_required_next"
  ]) as $denied
  | ([
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-non-acceptance-report-only-denial"; "accepted_final_operator_acknowledgement_non_acceptance_denial"; "source_operator_summary_briefing_denial_bound_without_final_acknowledgement_acceptance_terminal_decision_authority_execution_or_production_write"; true; {}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-missing-source"; "blocked_noop"; "source_operator_summary_briefing_denial_boundary_required"; false; {source_operator_summary_briefing_present:false, source_operator_summary_briefing_ready:false, final_operator_acknowledgement_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-request"; "blocked_ack_noop"; "final_operator_acknowledgement_request_shape_denied"; false; {final_operator_acknowledgement_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-acceptance-request"; "blocked_acceptance_noop"; "final_operator_acknowledgement_acceptance_denied"; false; {final_operator_acknowledgement_requested:true, acknowledgement_acceptance_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-recording-request"; "blocked_recording_noop"; "final_operator_acknowledgement_recording_denied"; false; {final_operator_acknowledgement_requested:true, acknowledgement_recording_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-persistence-materialization-request"; "blocked_persistence_noop"; "final_operator_acknowledgement_persistence_materialization_denied"; false; {final_operator_acknowledgement_requested:true, acknowledgement_persistence_requested:true, acknowledgement_materialization_requested:true, acknowledgement_filesystem_write_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-readback-receipt-request"; "blocked_readback_receipt_noop"; "final_operator_acknowledgement_readback_receipt_denied"; false; {final_operator_acknowledgement_requested:true, acknowledgement_readback_requested:true, acknowledgement_receipt_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-delivery-request"; "blocked_delivery_noop"; "final_operator_acknowledgement_delivery_denied"; false; {final_operator_acknowledgement_requested:true, acknowledgement_delivery_requested:true, telegram_send_requested:true, channel_delivery_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-status-request"; "blocked_terminal_decision_status_noop"; "terminal_operator_decision_status_denied"; false; {final_operator_acknowledgement_requested:true, terminal_operator_decision_requested:true, terminal_operator_status_requested:true, terminal_operator_decision_promotion_requested:true, terminal_operator_status_promotion_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-final-operator-ack-authority-memory-provider-external-request"; "blocked_authority_noop"; "final_operator_acknowledgement_authority_memory_provider_external_denied"; false; {final_operator_acknowledgement_requested:true, authority_promotion_requested:true, dry_run_execution_requested:true, production_write_requested:true, memory_write_acknowledgement_requested:true, rollback_acknowledgement_requested:true, kg_write_acknowledgement_requested:true, provider_prompt_acknowledgement_requested:true, credential_acknowledgement_requested:true, external_send_acknowledgement_requested:true, public_claim_acknowledgement_requested:true, release_artifact_acknowledgement_requested:true, install_acknowledgement_requested:true, service_restart_acknowledgement_requested:true, active_binary_acknowledgement_requested:true})
  ]) as $fixtures
  | ($source + {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    endpoint:$endpoint,
    source_command:$source_command,
    native_route:true,
    side_effect_free:true,
    audit_date:"2026-07-05",
    memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_accepted:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_mode:"dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_no_ack_acceptance_no_terminal_decision_no_authority_no_execution_no_production_durable_memory_mutation",
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_ready:true,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_accepted_count:1,
    source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count:$source.accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count,
    source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count:$source.blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_fixture_count,
    source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_count:$source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_count,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_report_sha256:$source_report_sha256,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_hash_sha256:$source_operator_summary_briefing_boundary_hash_sha256,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_policy_hash_sha256:$source_operator_summary_briefing_policy_hash_sha256,
    source_dry_run_execution_result_receipt_operator_summary_briefing_result_hash_sha256:$source_operator_summary_briefing_result_hash_sha256,
    source_dry_run_execution_result_receipt_operator_summary_briefing_handoff_hash_sha256:$source_operator_summary_briefing_handoff_hash_sha256,
    dry_run_execution_result_receipt_final_operator_acknowledgement_denial_hash_sha256:$final_acknowledgement_denial_hash_sha256,
    dry_run_execution_result_receipt_final_operator_acknowledgement_readback_denial_hash_sha256:$final_acknowledgement_readback_denial_hash_sha256,
    dry_run_execution_result_receipt_final_operator_acknowledgement_receipt_denial_hash_sha256:$final_acknowledgement_receipt_denial_hash_sha256,
    dry_run_execution_result_receipt_terminal_operator_decision_status_denial_hash_sha256:$terminal_decision_status_denial_hash_sha256,
    dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_matrix_hash_sha256:$final_acknowledgement_matrix_hash_sha256,
    dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_handoff_hash_sha256:$final_acknowledgement_handoff_hash_sha256,
    dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_result_hash_sha256:$final_acknowledgement_result_hash_sha256,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_hash_sha256:$final_acknowledgement_boundary_hash_sha256,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_policy_hash_sha256:$final_acknowledgement_policy_hash_sha256,
    required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_surface_count:($surfaces | length),
    ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_surface_count:($surfaces | length),
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_surfaces:$surfaces,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count:($fixtures | length),
    accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count:1,
    blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count:9,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixtures:$fixtures,
    denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary:$denied,
    denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_count:($denied | length),
    allowed_next_actions:[
      {
        action:"run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_require_live_gate",
        status:"allowed_verification_only",
        accepts_operator_acknowledgement:false,
        records_acknowledgement:false,
        persists_acknowledgement:false,
        materializes_acknowledgement:false,
        delivers_acknowledgement:false,
        records_terminal_decision:false,
        records_terminal_status:false,
        promotes_authority:false,
        executes_dry_run:false,
        writes_production_durable_memory:false,
        writes_memory_store:false,
        writes_wal:false,
        persists_receipt:false
      },
      {
        action:"prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary",
        status:"requires_separate_result_receipt_terminal_operator_decision_denial_gate",
        requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary:true,
        accepts_terminal_decision:false,
        records_terminal_status:false,
        claims_public_release:false,
        promotes_authority:false,
        executes_dry_run:false,
        writes_production_durable_memory:false,
        persists_dry_run_result_receipt:false
      }
    ]
  })
  | (.side_effects = {
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_performed:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_result_accepted:true,
      dry_run_execution_result_receipt_final_operator_acknowledgement_recorded:false,
      dry_run_execution_result_receipt_final_operator_acknowledgement_accepted:false,
      dry_run_execution_result_receipt_terminal_operator_decision_recorded:false,
      dry_run_execution_result_receipt_terminal_operator_status_recorded:false,
      dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement:false,
      dry_run_execution_executed:false,
      production_durable_memory_store_write_performed:false,
      memory_store_write_performed:false,
      wal_write_performed:false,
      receipt_persisted:false,
      external_send_performed:false
    })
  | reduce [
      "dry_run_execution_result_receipt_final_operator_acknowledgement_requested",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_recorded",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_accepted",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_persisted",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_materialized",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_filesystem_written",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_delivered",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_readback_recorded",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_receipt_persisted",
      "dry_run_execution_result_receipt_terminal_operator_decision_recorded",
      "dry_run_execution_result_receipt_terminal_operator_status_recorded",
      "dry_run_execution_result_receipt_terminal_operator_decision_promoted",
      "dry_run_execution_result_receipt_terminal_operator_status_promoted",
      "dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement",
      "dry_run_execution_executed",
      "production_durable_memory_write_executed",
      "production_durable_memory_store_write_performed",
      "actual_production_durable_memory_write_performed",
      "durable_memory_store_write_performed",
      "memory_store_write_performed",
      "memory_store_mutated",
      "wal_write_performed",
      "receipt_persisted",
      "rollback_executed",
      "tombstone_cleanup_executed",
      "live_kg_write_performed",
      "provider_invoked",
      "model_invoked",
      "credential_read",
      "channel_send_performed",
      "external_send_performed",
      "release_artifact_written",
      "install_executed",
      "service_restarted",
      "active_binary_mutated"
    ][] as $k (.; .[$k]=false | .[$k + "_count"]=0)
  | reduce [
      "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_performed",
      "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_result_recorded",
      "scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_result_accepted",
      "source_dry_run_execution_result_receipt_operator_facing_summary_briefing_non_persistence_denial_boundary_bound",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_request_denied",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_bound",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_readback_denied",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_receipt_denied",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_delivery_denied",
      "dry_run_execution_result_receipt_terminal_operator_decision_status_denied",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_authority_denied",
      "dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_handoff_bound"
    ][] as $k (.; .[$k]=true | .[$k + "_count"]=1)
  '
