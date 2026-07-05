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
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-final-operator-acknowledgement-non-acceptance-denial-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready == true
    and $source.scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_accepted == true
    and $source.accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count == 1
    and $source.blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count == 9
    and $source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_count >= 60
    and $source.dry_run_execution_result_receipt_final_operator_acknowledgement_recorded == false
    and $source.dry_run_execution_result_receipt_final_operator_acknowledgement_accepted == false
    and $source.dry_run_execution_result_receipt_terminal_operator_decision_recorded == false
    and $source.dry_run_execution_result_receipt_terminal_operator_status_recorded == false
    and $source.dry_run_execution_result_receipt_authority_promoted_from_final_acknowledgement == false
    and $source.dry_run_execution_executed == false
    and $source.production_durable_memory_store_write_performed == false
    and $source.memory_store_write_performed == false
    and $source.wal_write_performed == false
    and $source.receipt_persisted == false
    and $source.live_kg_write_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.credential_read == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and $source.allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary"
    and $source.allowed_next_actions[1].accepts_terminal_decision == false
  ' >/dev/null

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
source_final_ack_boundary_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_final_ack_policy_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_policy_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_final_ack_result_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_result_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_final_ack_handoff_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_handoff_hash_sha256 // ""' <<<"$SOURCE_JSON")"

terminal_decision_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-denial:v1:source=${source_final_ack_result_hash_sha256}:decision=false:record=false:persist=false:deliver=false"
)"
public_claim_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-public-claim-denial:v1:decision=${terminal_decision_denial_hash_sha256}:claim=false:ga=false:release=false:artifact=false"
)"
terminal_public_claim_matrix_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-matrix:v1:decision=${terminal_decision_denial_hash_sha256}:public=${public_claim_denial_hash_sha256}:fixtures=10"
)"
terminal_public_claim_handoff_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-handoff:v1:matrix=${terminal_public_claim_matrix_hash_sha256}:next=release-artifact-publication-denial-boundary"
)"
terminal_public_claim_result_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-result:v1:decision=${terminal_decision_denial_hash_sha256}:public=${public_claim_denial_hash_sha256}:handoff=${terminal_public_claim_handoff_hash_sha256}:accepted=true:terminal=false:public=false:execution=false:production-write=false"
)"
terminal_public_claim_boundary_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary:v1:source=${source_report_sha256}:result=${terminal_public_claim_result_hash_sha256}:accepted=1:blocked=9:decision=false:public=false:authority=false:execution=false:production-write=false"
)"
terminal_public_claim_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-policy:v1:no-terminal-decision-recording-no-terminal-status-no-public-claim-no-publication-no-release-artifact-no-authority-no-execution-no-production-write-no-kg-no-provider-no-release-no-install"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary --json" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg source_final_ack_boundary_hash_sha256 "$source_final_ack_boundary_hash_sha256" \
  --arg source_final_ack_policy_hash_sha256 "$source_final_ack_policy_hash_sha256" \
  --arg source_final_ack_result_hash_sha256 "$source_final_ack_result_hash_sha256" \
  --arg source_final_ack_handoff_hash_sha256 "$source_final_ack_handoff_hash_sha256" \
  --arg terminal_decision_denial_hash_sha256 "$terminal_decision_denial_hash_sha256" \
  --arg public_claim_denial_hash_sha256 "$public_claim_denial_hash_sha256" \
  --arg terminal_public_claim_matrix_hash_sha256 "$terminal_public_claim_matrix_hash_sha256" \
  --arg terminal_public_claim_handoff_hash_sha256 "$terminal_public_claim_handoff_hash_sha256" \
  --arg terminal_public_claim_result_hash_sha256 "$terminal_public_claim_result_hash_sha256" \
  --arg terminal_public_claim_boundary_hash_sha256 "$terminal_public_claim_boundary_hash_sha256" \
  --arg terminal_public_claim_policy_hash_sha256 "$terminal_public_claim_policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  def fixture($id; $status; $accepted; $reason; $extra):
    {
      id:$id,
      fixture_id:$id,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_status:$status,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_accepted:$accepted,
      source_final_acknowledgement_present:true,
      source_final_acknowledgement_ready:true,
      terminal_operator_decision_public_claim_noop_confirmed:true,
      reason:$reason,
      terminal_operator_decision_requested:false,
      terminal_operator_status_requested:false,
      terminal_operator_decision_recording_requested:false,
      terminal_operator_decision_persistence_requested:false,
      terminal_operator_decision_materialization_requested:false,
      terminal_operator_decision_filesystem_write_requested:false,
      terminal_operator_decision_delivery_requested:false,
      terminal_operator_decision_acceptance_requested:false,
      terminal_operator_status_recording_requested:false,
      terminal_operator_status_acceptance_requested:false,
      terminal_operator_decision_promotion_requested:false,
      terminal_operator_status_promotion_requested:false,
      public_claim_requested:false,
      public_claim_promotion_requested:false,
      public_ga_claim_requested:false,
      public_release_claim_requested:false,
      public_distribution_requested:false,
      public_artifact_write_requested:false,
      release_artifact_write_requested:false,
      telegram_send_requested:false,
      channel_delivery_requested:false,
      external_send_requested:false,
      dry_run_execution_requested:false,
      production_write_requested:false,
      memory_write_decision_requested:false,
      rollback_decision_requested:false,
      kg_write_decision_requested:false,
      provider_prompt_decision_requested:false,
      credential_decision_requested:false,
      install_decision_requested:false,
      service_restart_decision_requested:false,
      active_binary_decision_requested:false,
      terminal_operator_decision_allowed:false,
      terminal_operator_decision_request_accepted:false,
      terminal_operator_decision_accepted:false,
      terminal_operator_decision_recorded:false,
      terminal_operator_decision_persisted:false,
      terminal_operator_status_recorded:false,
      terminal_operator_status_accepted:false,
      terminal_operator_decision_promoted:false,
      terminal_operator_status_promoted:false,
      public_claim_accepted:false,
      public_claim_recorded:false,
      public_claim_promoted:false,
      public_ga_claimed:false,
      public_release_published:false,
      public_distribution_performed:false,
      public_artifact_written:false,
      release_artifact_written:false,
      telegram_send_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      receipt_persisted:false,
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
      install_executed:false,
      service_restarted:false,
      active_binary_mutated:false
    } + $extra;
  ([
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-terminal-decision-public-claim-report-only-binding"; "accepted_report_only"; true; "terminal_operator_decision_public_claim_non_promotion_denial_matrix_bound_without_terminal_decision_or_public_claim"; {}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-terminal-decision-public-claim-missing-source-final-acknowledgement"; "blocked_noop"; false; "source_final_operator_acknowledgement_non_acceptance_report_required"; {source_final_acknowledgement_present:false, source_final_acknowledgement_ready:false, terminal_operator_decision_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-decision-request"; "blocked_decision_noop"; false; "terminal_operator_decision_request_denied"; {terminal_operator_decision_requested:true, terminal_operator_decision_recording_requested:true, terminal_operator_decision_acceptance_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-terminal-operator-status-request"; "blocked_status_noop"; false; "terminal_operator_status_recording_denied"; {terminal_operator_status_requested:true, terminal_operator_status_recording_requested:true, terminal_operator_status_acceptance_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-terminal-decision-persistence-request"; "blocked_decision_noop"; false; "terminal_operator_decision_persistence_materialization_filesystem_write_denied"; {terminal_operator_decision_requested:true, terminal_operator_decision_persistence_requested:true, terminal_operator_decision_materialization_requested:true, terminal_operator_decision_filesystem_write_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-terminal-decision-delivery-request"; "blocked_delivery_noop"; false; "terminal_operator_decision_delivery_denied"; {terminal_operator_decision_requested:true, terminal_operator_decision_delivery_requested:true, telegram_send_requested:true, channel_delivery_requested:true, external_send_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-terminal-decision-public-claim-promotion-request"; "blocked_public_claim_noop"; false; "public_claim_public_release_public_ga_promotion_denied"; {terminal_operator_decision_requested:true, terminal_operator_decision_promotion_requested:true, terminal_operator_status_promotion_requested:true, public_claim_requested:true, public_claim_promotion_requested:true, public_ga_claim_requested:true, public_release_claim_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-request"; "blocked_publication_noop"; false; "release_artifact_public_artifact_public_distribution_denied"; {terminal_operator_decision_requested:true, public_distribution_requested:true, public_artifact_write_requested:true, release_artifact_write_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-terminal-decision-production-memory-provider-request"; "blocked_authority_noop"; false; "production_memory_rollback_kg_provider_credential_authority_denied"; {terminal_operator_decision_requested:true, dry_run_execution_requested:true, production_write_requested:true, memory_write_decision_requested:true, rollback_decision_requested:true, kg_write_decision_requested:true, provider_prompt_decision_requested:true, credential_decision_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-terminal-decision-install-restart-active-binary-request"; "blocked_install_noop"; false; "install_restart_active_binary_terminal_decision_denied"; {terminal_operator_decision_requested:true, install_decision_requested:true, service_restart_decision_requested:true, active_binary_decision_requested:true})
  ]) as $fixtures
  | ($source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary + [
    "terminal_operator_decision_request_denied",
    "terminal_operator_decision_recording_denied",
    "terminal_operator_decision_persistence_denied",
    "terminal_operator_decision_materialization_denied",
    "terminal_operator_decision_filesystem_write_denied",
    "terminal_operator_decision_delivery_denied",
    "terminal_operator_status_recording_denied",
    "terminal_operator_status_persistence_denied",
    "terminal_operator_decision_promotion_denied",
    "terminal_operator_status_promotion_denied",
    "public_claim_recording_denied",
    "public_claim_promotion_denied",
    "public_ga_claim_denied",
    "public_release_publication_denied",
    "public_distribution_denied",
    "public_artifact_write_denied",
    "release_artifact_write_denied",
    "dry_run_execution_and_production_memory_write_denied_by_terminal_decision_boundary",
    "kg_provider_channel_release_install_active_binary_terminal_decision_denied"
  ]) as $denials
  | {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    endpoint:$endpoint,
    source_command:$source_command,
    native_route:true,
    side_effect_free:true,
    native_gateway_source_command_count:280,
    route_count:280,
    implemented_route_count:280,
    missing_route_count:0,
    route_count_source_command_accepted:true,
    memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_ready:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_accepted:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_mode:"dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_no_terminal_decision_no_public_claim_no_authority_no_execution_no_production_durable_memory_mutation",
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_ready:true,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_report_sha256:$source_report_sha256,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_hash_sha256:$source_final_ack_boundary_hash_sha256,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_policy_hash_sha256:$source_final_ack_policy_hash_sha256,
    source_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_result_hash_sha256:$source_final_ack_result_hash_sha256,
    source_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_handoff_hash_sha256:$source_final_ack_handoff_hash_sha256,
    source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count:1,
    source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_fixture_count:9,
    source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_count:($source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_count),
    dry_run_execution_result_receipt_terminal_operator_decision_denial_hash_sha256:$terminal_decision_denial_hash_sha256,
    dry_run_execution_result_receipt_public_claim_denial_hash_sha256:$public_claim_denial_hash_sha256,
    dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_matrix_hash_sha256:$terminal_public_claim_matrix_hash_sha256,
    dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_handoff_hash_sha256:$terminal_public_claim_handoff_hash_sha256,
    dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_result_hash_sha256:$terminal_public_claim_result_hash_sha256,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_hash_sha256:$terminal_public_claim_boundary_hash_sha256,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_policy_hash_sha256:$terminal_public_claim_policy_hash_sha256,
    required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_surface_count:14,
    ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_surface_count:14,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count:($fixtures | length),
    accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count:1,
    blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count:9,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixtures:$fixtures,
    denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary:$denials,
    denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_count:($denials | length),
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_performed_count:1,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_result_accepted_count:1,
    source_dry_run_execution_result_receipt_final_operator_acknowledgement_non_acceptance_denial_boundary_bound_count:1,
    dry_run_execution_result_receipt_terminal_operator_decision_request_denied_count:1,
    dry_run_execution_result_receipt_terminal_operator_decision_status_denied_count:1,
    dry_run_execution_result_receipt_public_claim_denied_count:1,
    dry_run_execution_result_receipt_public_release_publication_denied_count:1,
    dry_run_execution_result_receipt_terminal_operator_decision_public_claim_authority_denied_count:1,
    dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_handoff_bound_count:1,
    dry_run_execution_result_receipt_terminal_operator_decision_recorded_count:0,
    dry_run_execution_result_receipt_terminal_operator_decision_accepted_count:0,
    dry_run_execution_result_receipt_terminal_operator_decision_persisted_count:0,
    dry_run_execution_result_receipt_terminal_operator_status_recorded_count:0,
    dry_run_execution_result_receipt_public_claim_recorded_count:0,
    dry_run_execution_result_receipt_public_claim_promoted_count:0,
    dry_run_execution_result_receipt_public_ga_claimed_count:0,
    dry_run_execution_result_receipt_public_release_published_count:0,
    dry_run_execution_result_receipt_release_artifact_written_count:0,
    dry_run_execution_result_receipt_authority_promoted_from_terminal_operator_decision_count:0,
    dry_run_execution_executed_count:0,
    production_durable_memory_write_executed_count:0,
    production_durable_memory_store_write_performed_count:0,
    memory_store_write_performed_count:0,
    wal_write_performed_count:0,
    receipt_persisted_count:0,
    live_kg_write_performed_count:0,
    provider_invoked_count:0,
    model_invoked_count:0,
    credential_read_count:0,
    channel_send_performed_count:0,
    external_send_performed_count:0,
    release_artifact_written_count:0,
    install_executed_count:0,
    service_restarted_count:0,
    active_binary_mutated_count:0,
    dry_run_execution_result_receipt_terminal_operator_decision_requested:false,
    dry_run_execution_result_receipt_terminal_operator_decision_recorded:false,
    dry_run_execution_result_receipt_terminal_operator_decision_accepted:false,
    dry_run_execution_result_receipt_terminal_operator_decision_persisted:false,
    dry_run_execution_result_receipt_terminal_operator_status_recorded:false,
    dry_run_execution_result_receipt_terminal_operator_status_persisted:false,
    dry_run_execution_result_receipt_terminal_operator_decision_promoted:false,
    dry_run_execution_result_receipt_terminal_operator_status_promoted:false,
    dry_run_execution_result_receipt_public_claim_recorded:false,
    dry_run_execution_result_receipt_public_claim_promoted:false,
    dry_run_execution_result_receipt_public_ga_claimed:false,
    dry_run_execution_result_receipt_public_release_published:false,
    dry_run_execution_result_receipt_release_artifact_written:false,
    dry_run_execution_result_receipt_authority_promoted_from_terminal_operator_decision:false,
    dry_run_execution_executed:false,
    production_durable_memory_write_executed:false,
    production_durable_memory_store_write_performed:false,
    actual_production_durable_memory_write_performed:false,
    durable_memory_store_write_performed:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    wal_write_performed:false,
    receipt_persisted:false,
    rollback_executed:false,
    tombstone_cleanup_executed:false,
    live_kg_write_performed:false,
    provider_invoked:false,
    model_invoked:false,
    credential_read:false,
    channel_send_performed:false,
    external_send_performed:false,
    release_artifact_written:false,
    install_executed:false,
    service_restarted:false,
    active_binary_mutated:false,
    allowed_next_actions:[
      {
        action:"run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_require_live_gate",
        status:"allowed_verification_only",
        accepts_terminal_decision:false,
        records_terminal_decision:false,
        promotes_public_claim:false,
        claims_public_release:false,
        writes_release_artifact:false,
        executes_dry_run:false,
        writes_production_durable_memory:false,
        writes_memory_or_kg:false,
        invokes_provider:false,
        sends_externally:false,
        installs_or_restarts:false,
        mutates_active_binary:false
      },
      {
        action:"prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary",
        status:"allowed_report_only_next_slice",
        publishes_release_artifact:false,
        writes_release_artifact:false,
        claims_public_release:false,
        mutates_runtime:false,
        invokes_model:false,
        writes_memory_or_kg:false
      }
    ],
    side_effects:{
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_performed:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_result_accepted:true,
      dry_run_execution_result_receipt_terminal_operator_decision_requested:false,
      dry_run_execution_result_receipt_terminal_operator_decision_recorded:false,
      dry_run_execution_result_receipt_terminal_operator_decision_accepted:false,
      dry_run_execution_result_receipt_terminal_operator_status_recorded:false,
      dry_run_execution_result_receipt_public_claim_recorded:false,
      dry_run_execution_result_receipt_public_claim_promoted:false,
      dry_run_execution_result_receipt_public_release_published:false,
      dry_run_execution_result_receipt_authority_promoted_from_terminal_operator_decision:false,
      dry_run_execution_executed:false,
      production_durable_memory_store_write_performed:false,
      memory_store_write_performed:false,
      wal_write_performed:false,
      receipt_persisted:false,
      external_send_performed:false,
      release_artifact_written:false,
      install_executed:false,
      service_restarted:false,
      active_binary_mutated:false
    }
  }'
