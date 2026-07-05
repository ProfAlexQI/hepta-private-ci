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
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-denial-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-denial-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_ready == true
    and $source.scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_accepted == true
    and $source.accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count == 1
    and $source.blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count == 9
    and $source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_count >= 100
    and $source.dry_run_execution_result_receipt_release_artifact_publication_recorded == false
    and $source.dry_run_execution_result_receipt_release_artifact_written == false
    and $source.dry_run_execution_result_receipt_public_artifact_written == false
    and $source.dry_run_execution_result_receipt_publication_queue_enqueued == false
    and $source.dry_run_execution_result_receipt_publication_manifest_written == false
    and $source.dry_run_execution_result_receipt_public_distribution_performed == false
    and $source.dry_run_execution_result_receipt_public_release_published == false
    and $source.dry_run_execution_result_receipt_authority_promoted_from_release_artifact_publication == false
    and $source.activation_performed == false
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
    and $source.install_executed == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and $source.allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary"
    and $source.allowed_next_actions[1].persists_publication_result_receipt == false
  ' >/dev/null

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
source_publication_boundary_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_publication_policy_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_policy_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_publication_result_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_release_artifact_publication_result_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_publication_handoff_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_release_artifact_publication_handoff_hash_sha256 // ""' <<<"$SOURCE_JSON")"

publication_result_receipt_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-no-persistence:v1:source=${source_publication_result_hash_sha256}:record=false:persist=false:materialize=false:deliver=false"
)"
publication_result_receipt_matrix_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-no-persistence-matrix:v1:receipt=${publication_result_receipt_hash_sha256}:fixtures=10"
)"
publication_result_receipt_handoff_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-no-persistence-handoff:v1:matrix=${publication_result_receipt_matrix_hash_sha256}:next=release-artifact-publication-result-receipt-replay-idempotency-denial-boundary"
)"
publication_result_receipt_result_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-no-persistence-result:v1:receipt=${publication_result_receipt_hash_sha256}:handoff=${publication_result_receipt_handoff_hash_sha256}:accepted=true:record=false:persist=false:publication=false:authority=false:execution=false:production-write=false"
)"
publication_result_receipt_boundary_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary:v1:source=${source_report_sha256}:result=${publication_result_receipt_result_hash_sha256}:accepted=1:blocked=9:receipt-persist=false:publication=false:authority=false:execution=false:production-write=false"
)"
publication_result_receipt_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result-receipt-no-persistence-policy:v1:no-receipt-recording-no-receipt-persistence-no-ledger-no-index-no-queue-no-delivery-no-export-no-query-no-observability-no-completion-ack-no-release-artifact-no-publication-no-authority-no-execution-no-production-write"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-result-receipt-no-persistence-boundary --json" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg source_publication_boundary_hash_sha256 "$source_publication_boundary_hash_sha256" \
  --arg source_publication_policy_hash_sha256 "$source_publication_policy_hash_sha256" \
  --arg source_publication_result_hash_sha256 "$source_publication_result_hash_sha256" \
  --arg source_publication_handoff_hash_sha256 "$source_publication_handoff_hash_sha256" \
  --arg publication_result_receipt_hash_sha256 "$publication_result_receipt_hash_sha256" \
  --arg publication_result_receipt_matrix_hash_sha256 "$publication_result_receipt_matrix_hash_sha256" \
  --arg publication_result_receipt_handoff_hash_sha256 "$publication_result_receipt_handoff_hash_sha256" \
  --arg publication_result_receipt_result_hash_sha256 "$publication_result_receipt_result_hash_sha256" \
  --arg publication_result_receipt_boundary_hash_sha256 "$publication_result_receipt_boundary_hash_sha256" \
  --arg publication_result_receipt_policy_hash_sha256 "$publication_result_receipt_policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  ($source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary + [
    "source_release_artifact_publication_denial_report_required",
    "publication_result_receipt_recording_denied",
    "publication_result_receipt_acceptance_denied",
    "publication_result_receipt_persistence_denied",
    "publication_result_receipt_materialization_denied",
    "publication_result_receipt_filesystem_write_denied",
    "publication_result_receipt_ledger_write_denied",
    "publication_result_receipt_index_denied",
    "publication_result_receipt_queue_denied",
    "publication_result_receipt_delivery_denied",
    "publication_result_receipt_export_denied",
    "publication_result_receipt_query_registration_denied",
    "publication_result_receipt_observability_denied",
    "publication_result_receipt_signature_timestamp_status_denied",
    "publication_completion_ack_recording_denied",
    "publication_completion_ack_persistence_denied",
    "publication_result_receipt_authority_promotion_denied",
    "release_artifact_publication_result_receipt_no_persistence_only",
    "release_artifact_public_artifact_publication_remain_denied",
    "execution_memory_kg_provider_channel_install_active_binary_remain_denied"
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
    native_gateway_source_command_count:282,
    route_count:282,
    implemented_route_count:282,
    missing_route_count:0,
    route_count_source_command_accepted:true,
    memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_ready:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_ready:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_accepted:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_mode:"dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_no_receipt_persistence_no_publication_no_authority_no_execution_no_production_durable_memory_mutation",
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_ready:true,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_report_sha256:$source_report_sha256,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_hash_sha256:$source_publication_boundary_hash_sha256,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_policy_hash_sha256:$source_publication_policy_hash_sha256,
    source_dry_run_execution_result_receipt_release_artifact_publication_result_hash_sha256:$source_publication_result_hash_sha256,
    source_dry_run_execution_result_receipt_release_artifact_publication_handoff_hash_sha256:$source_publication_handoff_hash_sha256,
    source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count:1,
    source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count:9,
    source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_count:($source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_count),
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_hash_sha256:$publication_result_receipt_hash_sha256,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_matrix_hash_sha256:$publication_result_receipt_matrix_hash_sha256,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_handoff_hash_sha256:$publication_result_receipt_handoff_hash_sha256,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_result_hash_sha256:$publication_result_receipt_result_hash_sha256,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_hash_sha256:$publication_result_receipt_boundary_hash_sha256,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_policy_hash_sha256:$publication_result_receipt_policy_hash_sha256,
    required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_surface_count:14,
    ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_surface_count:14,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixture_count:10,
    accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixture_count:1,
    blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_fixture_count:9,
    denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary:$denials,
    denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_count:($denials | length),
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_performed_count:1,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_result_accepted_count:1,
    source_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_bound_count:1,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_rendered_count:1,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recording_denied_count:1,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persistence_denied_count:1,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_index_denied_count:1,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_queue_delivery_denied_count:1,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_export_query_observability_denied_count:1,
    dry_run_execution_result_receipt_release_artifact_publication_completion_ack_denied_count:1,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_denied_count:1,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recorded_count:0,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persisted_count:0,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_written_count:0,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_indexed_count:0,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_delivered_count:0,
    dry_run_execution_result_receipt_release_artifact_publication_completion_ack_recorded_count:0,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_promoted_count:0,
    dry_run_execution_result_receipt_release_artifact_publication_recorded_count:0,
    dry_run_execution_result_receipt_release_artifact_written_count:0,
    dry_run_execution_result_receipt_public_artifact_written_count:0,
    activation_performed_count:0,
    dry_run_execution_executed_count:0,
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
    public_artifact_written_count:0,
    install_executed_count:0,
    service_restarted_count:0,
    active_binary_mutated_count:0,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recorded:false,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_accepted:false,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persisted:false,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_materialized:false,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_filesystem_written:false,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_written:false,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_indexed:false,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_queued:false,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_delivered:false,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_exported:false,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_query_registered:false,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_observability_recorded:false,
    dry_run_execution_result_receipt_release_artifact_publication_completion_ack_recorded:false,
    dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_promoted:false,
    dry_run_execution_result_receipt_release_artifact_written:false,
    dry_run_execution_result_receipt_public_artifact_written:false,
    activation_performed:false,
    dry_run_execution_executed:false,
    production_durable_memory_store_write_performed:false,
    memory_store_write_performed:false,
    wal_write_performed:false,
    receipt_persisted:false,
    live_kg_write_performed:false,
    provider_invoked:false,
    model_invoked:false,
    credential_read:false,
    channel_send_performed:false,
    external_send_performed:false,
    release_artifact_written:false,
    public_artifact_written:false,
    install_executed:false,
    service_restarted:false,
    active_binary_mutated:false,
    allowed_next_actions:[
      {
        action:"run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_require_live_gate",
        status:"allowed_verification_only",
        persists_publication_result_receipt:false,
        publishes_release_artifact:false,
        promotes_activation_authority:false,
        executes_dry_run:false,
        writes_production_durable_memory:false,
        sends_externally:false,
        installs_or_restarts:false,
        mutates_active_binary:false
      },
      {
        action:"prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_replay_idempotency_denial_boundary",
        status:"allowed_report_only_next_slice",
        accepts_replay:false,
        persists_publication_result_receipt:false,
        publishes_release_artifact:false,
        promotes_activation_authority:false,
        executes_dry_run:false,
        writes_production_durable_memory:false,
        sends_externally:false,
        installs_or_restarts:false,
        mutates_active_binary:false
      }
    ],
    side_effects:{
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary_performed:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_result_accepted:true,
      dry_run_execution_result_receipt_release_artifact_publication_result_receipt_recorded:false,
      dry_run_execution_result_receipt_release_artifact_publication_result_receipt_persisted:false,
      dry_run_execution_result_receipt_release_artifact_publication_result_receipt_ledger_written:false,
      dry_run_execution_result_receipt_release_artifact_publication_result_receipt_indexed:false,
      dry_run_execution_result_receipt_release_artifact_publication_result_receipt_delivered:false,
      dry_run_execution_result_receipt_release_artifact_publication_completion_ack_recorded:false,
      dry_run_execution_result_receipt_release_artifact_publication_result_receipt_authority_promoted:false,
      dry_run_execution_result_receipt_release_artifact_written:false,
      dry_run_execution_result_receipt_public_artifact_written:false,
      activation_performed:false,
      dry_run_execution_executed:false,
      production_durable_memory_store_write_performed:false,
      memory_store_write_performed:false,
      wal_write_performed:false,
      receipt_persisted:false,
      external_send_performed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      active_binary_mutated:false
    }
  }'
