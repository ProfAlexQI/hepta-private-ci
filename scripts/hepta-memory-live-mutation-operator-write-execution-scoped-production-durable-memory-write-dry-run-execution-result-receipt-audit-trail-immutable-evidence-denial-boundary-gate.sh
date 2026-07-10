#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
EXPECTED_ROUTE_COUNT="${HEPTA_EXPECTED_ROUTE_COUNT:-$(bash "$REPO_ROOT/scripts/lib/hepta-native-route-count.sh")}"

source "$REPO_ROOT/scripts/lib/hepta-json-report-capture.sh"
cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

SOURCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-cancellation-supersession-denial-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-cancellation-supersession-denial-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_ready == true
    and $source.scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_accepted == true
    and $source.accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count == 1
    and $source.blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count == 9
    and $source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_count == 65
    and $source.dry_run_execution_result_receipt_cancellation_request_accepted == false
    and $source.dry_run_execution_result_receipt_supersession_request_accepted == false
    and $source.dry_run_execution_result_receipt_replacement_receipt_persisted == false
    and $source.dry_run_execution_result_receipt_tombstone_delete_marker_written == false
    and $source.dry_run_execution_executed == false
    and $source.dry_run_execution_result_receipt_persisted == false
    and $source.production_durable_memory_write_executed == false
    and $source.production_durable_memory_store_write_performed == false
    and $source.actual_production_durable_memory_write_performed == false
    and $source.durable_memory_store_write_performed == false
    and $source.durable_memory_store_read_performed == false
    and $source.durable_memory_store_rollback_performed == false
    and $source.memory_store_write_performed == false
    and $source.wal_write_performed == false
    and $source.receipt_persisted == false
    and $source.post_write_readback_performed == false
    and $source.rollback_executed == false
    and $source.tombstone_cleanup_executed == false
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
    and $source.allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary"
    and $source.allowed_next_actions[1].requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary == true
    and $source.allowed_next_actions[1].executes_dry_run == false
    and $source.allowed_next_actions[1].writes_production_durable_memory == false
  ' >/dev/null

approved_production_namespace="$(jq -r '.approved_production_namespace // ""' <<<"$SOURCE_JSON")"
approved_production_store="$(jq -r '.approved_production_store // ""' <<<"$SOURCE_JSON")"
approved_production_scope="$(jq -r '.approved_production_scope // ""' <<<"$SOURCE_JSON")"
production_durable_memory_target_id="$(jq -r '.production_durable_memory_target_id // ""' <<<"$SOURCE_JSON")"
production_durable_memory_payload_class="$(jq -r '.production_durable_memory_payload_class // ""' <<<"$SOURCE_JSON")"
operator_packet_scope="$(jq -r '.operator_packet_scope // ""' <<<"$SOURCE_JSON")"
source_report_sha256="$(sha256_text "$SOURCE_JSON")"
source_cancellation_boundary_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_cancellation_policy_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_policy_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_cancellation_matrix_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_cancellation_result_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_cancellation_supersession_result_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_cancellation_handoff_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_cancellation_supersession_handoff_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_replacement_receipt_denial_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_replacement_receipt_denial_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_tombstone_delete_marker_denial_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_tombstone_delete_marker_denial_hash_sha256 // ""' <<<"$SOURCE_JSON")"

audit_trail_denial_matrix_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-audit-trail-denial-matrix:v1:source=${source_cancellation_result_hash_sha256}:audit=deny:record=false:persist=false:materialize=false:filesystem=false"
)"
immutable_evidence_denial_matrix_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-immutable-evidence-denial-matrix:v1:source=${source_cancellation_result_hash_sha256}:immutable=deny:hash-chain=false:merkle=false:attestation=false:witness=false:notary=false:persist=false"
)"
audit_hash_chain_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-audit-hash-chain-denial:v1:audit=${audit_trail_denial_matrix_hash_sha256}:immutable=${immutable_evidence_denial_matrix_hash_sha256}:record=false"
)"
audit_attestation_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-audit-attestation-denial:v1:hash-chain=${audit_hash_chain_denial_hash_sha256}:attestation=false:witness=false:notary=false"
)"
audit_ledger_evidence_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-audit-ledger-evidence-denial:v1:attestation=${audit_attestation_denial_hash_sha256}:ledger=false:index=false:delivery=false:export=false:query=false:readback=false"
)"
audit_evidence_handoff_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-audit-evidence-handoff:v1:ledger=${audit_ledger_evidence_denial_hash_sha256}:next=retention-expiry-garbage-collection-denial-boundary"
)"
audit_evidence_result_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-audit-evidence-result:v1:audit=${audit_trail_denial_matrix_hash_sha256}:immutable=${immutable_evidence_denial_matrix_hash_sha256}:handoff=${audit_evidence_handoff_hash_sha256}:accepted=true:record=false:persist=false:authority=false:execution=false:production-write=false"
)"
audit_evidence_boundary_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-audit-trail-immutable-evidence-denial-boundary:v1:source=${source_report_sha256}:result=${audit_evidence_result_hash_sha256}:fixtures=10:accepted=1:denials=72:audit=false:immutable=false:authority=false:dry-run-executed=false:production-write=false"
)"
audit_evidence_boundary_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-audit-trail-immutable-evidence-denial-policy:v1:bind-source-cancellation-supersession-no-audit-log-no-immutable-evidence-no-hash-chain-no-merkle-root-no-attestation-no-ledger-no-authority-no-execution-no-production-write-no-kg-no-provider-no-channel-no-release-no-install"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_gate" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-audit-trail-immutable-evidence-denial-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-audit-trail-immutable-evidence-denial-boundary --json" \
  --arg approved_production_namespace "$approved_production_namespace" \
  --arg approved_production_store "$approved_production_store" \
  --arg approved_production_scope "$approved_production_scope" \
  --arg production_durable_memory_target_id "$production_durable_memory_target_id" \
  --arg production_durable_memory_payload_class "$production_durable_memory_payload_class" \
  --arg operator_packet_scope "$operator_packet_scope" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg source_cancellation_boundary_hash_sha256 "$source_cancellation_boundary_hash_sha256" \
  --arg source_cancellation_policy_hash_sha256 "$source_cancellation_policy_hash_sha256" \
  --arg source_cancellation_matrix_hash_sha256 "$source_cancellation_matrix_hash_sha256" \
  --arg source_cancellation_result_hash_sha256 "$source_cancellation_result_hash_sha256" \
  --arg source_cancellation_handoff_hash_sha256 "$source_cancellation_handoff_hash_sha256" \
  --arg source_replacement_receipt_denial_hash_sha256 "$source_replacement_receipt_denial_hash_sha256" \
  --arg source_tombstone_delete_marker_denial_hash_sha256 "$source_tombstone_delete_marker_denial_hash_sha256" \
  --arg audit_trail_denial_matrix_hash_sha256 "$audit_trail_denial_matrix_hash_sha256" \
  --arg immutable_evidence_denial_matrix_hash_sha256 "$immutable_evidence_denial_matrix_hash_sha256" \
  --arg audit_hash_chain_denial_hash_sha256 "$audit_hash_chain_denial_hash_sha256" \
  --arg audit_attestation_denial_hash_sha256 "$audit_attestation_denial_hash_sha256" \
  --arg audit_ledger_evidence_denial_hash_sha256 "$audit_ledger_evidence_denial_hash_sha256" \
  --arg audit_evidence_handoff_hash_sha256 "$audit_evidence_handoff_hash_sha256" \
  --arg audit_evidence_result_hash_sha256 "$audit_evidence_result_hash_sha256" \
  --arg audit_evidence_boundary_hash_sha256 "$audit_evidence_boundary_hash_sha256" \
  --arg audit_evidence_boundary_policy_hash_sha256 "$audit_evidence_boundary_policy_hash_sha256" \
  --argjson expected_route_count "$EXPECTED_ROUTE_COUNT" \
  --argjson source "$SOURCE_JSON" \
  '
  def zero_fields($keys): reduce $keys[] as $k ({}; .[$k]=false | .[$k + "_count"]=0);
  def true_count_fields($keys): reduce $keys[] as $k ({}; .[$k]=true | .[$k + "_count"]=1);
  ([
    "source_cancellation_supersession_denial_boundary_required",
    "source_cancellation_supersession_result_required",
    "dry_run_execution_result_receipt_audit_trail_request_denied",
    "dry_run_execution_result_receipt_immutable_evidence_request_denied",
    "dry_run_execution_result_receipt_hash_chain_recording_denied",
    "dry_run_execution_result_receipt_merkle_root_recording_denied",
    "dry_run_execution_result_receipt_attestation_witness_notary_denied",
    "dry_run_execution_result_receipt_audit_materialization_denied",
    "dry_run_execution_result_receipt_immutable_evidence_persistence_denied",
    "dry_run_execution_result_receipt_ledger_index_delivery_evidence_denied",
    "dry_run_execution_result_receipt_authority_promotion_from_audit_evidence_denied",
    "dry_run_execution_result_receipt_memory_kg_provider_channel_evidence_denied",
    "dry_run_execution_result_receipt_release_install_active_binary_evidence_denied",
    "dry_run_execution_execution_forbidden_on_audit_evidence_route",
    "production_write_execution_forbidden_on_audit_evidence_route",
    "kg_provider_channel_release_install_active_binary_forbidden_on_audit_evidence_route"
  ]) as $surfaces
  | ([
    "source_cancellation_supersession_denial_boundary_required",
    "source_cancellation_supersession_result_hash_required",
    "source_cancellation_supersession_policy_hash_required",
    "source_cancellation_supersession_matrix_required",
    "source_replacement_receipt_denial_required",
    "source_tombstone_delete_marker_denial_required",
    "approved_production_namespace_required",
    "approved_production_store_required",
    "approved_production_scope_required",
    "production_durable_memory_target_required",
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
    "audit_evidence_persistence_denied",
    "hash_chain_recording_denied",
    "hash_chain_persistence_denied",
    "merkle_root_recording_denied",
    "merkle_root_persistence_denied",
    "attestation_recording_denied",
    "attestation_persistence_denied",
    "witness_recording_denied",
    "witness_persistence_denied",
    "notary_recording_denied",
    "notary_persistence_denied",
    "ledger_evidence_recording_denied",
    "ledger_evidence_persistence_denied",
    "index_evidence_recording_denied",
    "delivery_evidence_recording_denied",
    "export_evidence_recording_denied",
    "query_evidence_recording_denied",
    "readback_evidence_recording_denied",
    "audit_evidence_authority_derivation_denied",
    "immutable_evidence_authority_derivation_denied",
    "result_receipt_authority_promotion_from_audit_denied",
    "result_receipt_authority_promotion_from_immutable_evidence_denied",
    "cancellation_supersession_authority_from_audit_denied",
    "replacement_receipt_authority_from_evidence_denied",
    "dry_run_execution_execution_denied",
    "dry_run_execution_envelope_persistence_denied",
    "dry_run_execution_result_persistence_denied",
    "dry_run_execution_result_receipt_persistence_denied",
    "dry_run_execution_result_receipt_filesystem_write_denied",
    "dry_run_execution_result_receipt_ledger_recording_denied",
    "dry_run_execution_result_receipt_delivery_denied",
    "dry_run_execution_result_receipt_materialization_denied",
    "acceptance_receipt_persistence_denied",
    "operator_packet_persistence_denied",
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
    "channel_external_send_denied",
    "release_public_artifact_write_denied",
    "install_restart_authority_denied",
    "active_binary_mutation_denied",
    "unrestricted_full_live_activation_denied"
  ]) as $denials
  | ([
    "dry_run_execution_result_receipt_audit_trail_accepted",
    "dry_run_execution_result_receipt_audit_trail_recorded",
    "dry_run_execution_result_receipt_audit_trail_persisted",
    "dry_run_execution_result_receipt_audit_trail_materialized",
    "dry_run_execution_result_receipt_audit_trail_filesystem_written",
    "dry_run_execution_result_receipt_immutable_evidence_accepted",
    "dry_run_execution_result_receipt_immutable_evidence_recorded",
    "dry_run_execution_result_receipt_immutable_evidence_persisted",
    "dry_run_execution_result_receipt_immutable_evidence_materialized",
    "dry_run_execution_result_receipt_immutable_evidence_filesystem_written",
    "dry_run_execution_result_receipt_hash_chain_recorded",
    "dry_run_execution_result_receipt_hash_chain_persisted",
    "dry_run_execution_result_receipt_merkle_root_recorded",
    "dry_run_execution_result_receipt_merkle_root_persisted",
    "dry_run_execution_result_receipt_attestation_recorded",
    "dry_run_execution_result_receipt_attestation_persisted",
    "dry_run_execution_result_receipt_witness_recorded",
    "dry_run_execution_result_receipt_witness_persisted",
    "dry_run_execution_result_receipt_notary_recorded",
    "dry_run_execution_result_receipt_notary_persisted",
    "dry_run_execution_result_receipt_ledger_evidence_recorded",
    "dry_run_execution_result_receipt_ledger_evidence_persisted",
    "dry_run_execution_result_receipt_index_evidence_recorded",
    "dry_run_execution_result_receipt_delivery_evidence_recorded",
    "dry_run_execution_result_receipt_export_evidence_recorded",
    "dry_run_execution_result_receipt_query_evidence_recorded",
    "dry_run_execution_result_receipt_readback_evidence_recorded",
    "dry_run_execution_result_receipt_authority_promoted_from_audit_trail",
    "dry_run_execution_result_receipt_authority_promoted_from_immutable_evidence",
    "dry_run_execution_result_receipt_persisted",
    "dry_run_execution_result_receipt_filesystem_written",
    "dry_run_execution_result_receipt_ledger_recorded",
    "dry_run_execution_result_receipt_delivered",
    "dry_run_execution_result_receipt_materialized",
    "dry_run_execution_envelope_persisted",
    "dry_run_execution_executed",
    "dry_run_execution_result_persisted",
    "acceptance_receipt_persisted",
    "operator_packet_persisted",
    "operator_packet_acceptance_receipt_persisted",
    "production_durable_memory_write_executed",
    "production_durable_memory_backend_present",
    "production_durable_memory_store_write_performed",
    "actual_production_durable_memory_write_performed",
    "durable_memory_store_write_performed",
    "durable_memory_store_read_performed",
    "durable_memory_store_rollback_performed",
    "memory_write_execution_performed",
    "memory_store_write_path_enabled",
    "memory_store_write_allowed",
    "memory_store_write_performed",
    "memory_store_mutation_allowed",
    "memory_store_mutated",
    "wal_write_performed",
    "wal_recorded",
    "wal_persisted",
    "receipt_recorded",
    "receipt_persisted",
    "receipt_materialized",
    "receipt_delivered",
    "post_write_readback_performed",
    "readback_result_recorded",
    "readback_result_persisted",
    "readback_result_accepted",
    "rollback_executed",
    "rollback_performed",
    "rollback_result_recorded",
    "rollback_result_persisted",
    "rollback_result_accepted",
    "tombstone_write_performed",
    "tombstone_cleanup_executed",
    "tombstone_cleanup_result_recorded",
    "tombstone_cleanup_result_accepted",
    "raw_payload_plaintext_recorded",
    "raw_payload_plaintext_persisted",
    "secret_material_read",
    "credential_read",
    "secret_file_read",
    "kg_adapter_read_performed",
    "live_kg_write_performed",
    "provider_invoked",
    "model_invoked",
    "telegram_send_performed",
    "channel_send_performed",
    "external_send_performed",
    "public_claim_promoted",
    "public_release_published",
    "public_ga_claimed",
    "release_artifact_written",
    "public_artifact_written",
    "install_executed",
    "launchd_mutated",
    "service_restarted",
    "service_restart_performed",
    "active_binary_mutated"
  ]) as $false_keys
  | ([
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_performed",
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_result_recorded",
    "scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_result_accepted",
    "source_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_accepted",
    "dry_run_execution_result_receipt_audit_trail_denial_matrix_bound",
    "dry_run_execution_result_receipt_immutable_evidence_denial_matrix_bound",
    "dry_run_execution_result_receipt_audit_trail_request_denied",
    "dry_run_execution_result_receipt_immutable_evidence_request_denied",
    "dry_run_execution_result_receipt_hash_chain_denied",
    "dry_run_execution_result_receipt_merkle_root_denied",
    "dry_run_execution_result_receipt_attestation_denied",
    "dry_run_execution_result_receipt_witness_denied",
    "dry_run_execution_result_receipt_notary_denied",
    "dry_run_execution_result_receipt_ledger_index_delivery_evidence_denied",
    "dry_run_execution_result_receipt_audit_evidence_authority_denied",
    "dry_run_execution_result_receipt_audit_evidence_handoff_bound",
    "dry_run_execution_result_receipt_audit_evidence_persistence_forbidden",
    "dry_run_execution_execution_forbidden_on_audit_evidence_route",
    "production_write_execution_forbidden_on_audit_evidence_route",
    "kg_provider_channel_release_install_active_binary_forbidden_on_audit_evidence_route"
  ]) as $true_keys
  | ({
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      native_gateway_source_command_count:$expected_route_count,
      route_count:$expected_route_count,
      implemented_route_count:$expected_route_count,
      missing_route_count:0,
      route_count_source_command_accepted:true,
      memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_ready:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_ready:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_performed:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_accepted:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_mode:"dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_no_audit_no_immutable_evidence_no_hash_chain_no_attestation_no_authority_no_execution_no_production_durable_memory_mutation",
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_ready:true,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_report_sha256:$source_report_sha256,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_accepted_count:1,
      source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count:1,
      source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_fixture_count:9,
      source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_count:65,
      approved_production_namespace:$approved_production_namespace,
      approved_production_store:$approved_production_store,
      approved_production_scope:$approved_production_scope,
      production_durable_memory_target_id:$production_durable_memory_target_id,
      production_durable_memory_payload_class:$production_durable_memory_payload_class,
      operator_packet_scope:$operator_packet_scope,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_hash_sha256:$source_cancellation_boundary_hash_sha256,
      source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_cancellation_supersession_denial_policy_hash_sha256:$source_cancellation_policy_hash_sha256,
      source_dry_run_execution_result_receipt_cancellation_supersession_denial_matrix_hash_sha256:$source_cancellation_matrix_hash_sha256,
      source_dry_run_execution_result_receipt_cancellation_supersession_result_hash_sha256:$source_cancellation_result_hash_sha256,
      source_dry_run_execution_result_receipt_cancellation_supersession_handoff_hash_sha256:$source_cancellation_handoff_hash_sha256,
      source_dry_run_execution_result_receipt_replacement_receipt_denial_hash_sha256:$source_replacement_receipt_denial_hash_sha256,
      source_dry_run_execution_result_receipt_tombstone_delete_marker_denial_hash_sha256:$source_tombstone_delete_marker_denial_hash_sha256,
      dry_run_execution_result_receipt_audit_trail_denial_matrix_hash_sha256:$audit_trail_denial_matrix_hash_sha256,
      dry_run_execution_result_receipt_immutable_evidence_denial_matrix_hash_sha256:$immutable_evidence_denial_matrix_hash_sha256,
      dry_run_execution_result_receipt_audit_hash_chain_denial_hash_sha256:$audit_hash_chain_denial_hash_sha256,
      dry_run_execution_result_receipt_audit_attestation_denial_hash_sha256:$audit_attestation_denial_hash_sha256,
      dry_run_execution_result_receipt_audit_ledger_evidence_denial_hash_sha256:$audit_ledger_evidence_denial_hash_sha256,
      dry_run_execution_result_receipt_audit_evidence_handoff_hash_sha256:$audit_evidence_handoff_hash_sha256,
      dry_run_execution_result_receipt_audit_evidence_result_hash_sha256:$audit_evidence_result_hash_sha256,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_hash_sha256:$audit_evidence_boundary_hash_sha256,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_policy_hash_sha256:$audit_evidence_boundary_policy_hash_sha256,
      required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_surface_count:($surfaces | length),
      ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_surface_count:($surfaces | length),
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_surfaces:$surfaces,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count:10,
      accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count:1,
      blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixture_count:9,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_fixtures:([
        {
          id:"scoped-production-durable-memory-write-dry-run-execution-result-receipt-audit-trail-immutable-evidence-denial",
          fixture_id:"scoped-production-durable-memory-write-dry-run-execution-result-receipt-audit-trail-immutable-evidence-denial",
          scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_accepted:true,
          reason:"dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_bound_without_audit_evidence_persistence_authority_execution_or_production_write",
          source_cancellation_supersession_denial_boundary_bound:true,
          dry_run_execution_result_receipt_audit_trail_denied:true,
          dry_run_execution_result_receipt_immutable_evidence_denied:true,
          dry_run_execution_result_receipt_hash_chain_denied:true,
          dry_run_execution_result_receipt_merkle_root_denied:true,
          dry_run_execution_result_receipt_attestation_denied:true,
          dry_run_execution_result_receipt_ledger_index_delivery_evidence_denied:true,
          dry_run_execution_result_receipt_audit_trail_recorded:false,
          dry_run_execution_result_receipt_immutable_evidence_persisted:false,
          dry_run_execution_result_receipt_hash_chain_recorded:false,
          dry_run_execution_result_receipt_authority_promoted_from_audit_trail:false,
          dry_run_execution_executed:false,
          production_durable_memory_store_write_performed:false,
          external_send_performed:false
        }
      ] + ([
        "missing-cancellation-supersession-source",
        "missing-cancellation-result-hash",
        "audit-trail-append-request-attempt",
        "immutable-evidence-seal-request-attempt",
        "hash-chain-merkle-root-attempt",
        "attestation-witness-notary-attempt",
        "ledger-index-delivery-evidence-attempt",
        "memory-kg-provider-channel-evidence-attempt",
        "release-install-active-binary-evidence-attempt"
      ] | map({
        id:.,
        fixture_id:.,
        scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_accepted:false,
        reason:"blocked_noop",
        dry_run_execution_result_receipt_audit_trail_recorded:false,
        dry_run_execution_result_receipt_immutable_evidence_persisted:false,
        dry_run_execution_result_receipt_hash_chain_recorded:false,
        dry_run_execution_result_receipt_authority_promoted_from_audit_trail:false,
        dry_run_execution_executed:false,
        production_durable_memory_store_write_performed:false,
        external_send_performed:false
      }))),
      denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary:$denials,
      denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_count:($denials | length),
      source_dry_run_execution_result_receipt_cancellation_supersession_denial_boundary_bound:true,
      approved_production_namespace_bound:true,
      approved_production_store_bound:true,
      approved_production_scope_bound:true,
      production_durable_memory_target_bound:true,
      dry_run_execution_result_receipt_audit_trail_denial_matrix_bound:true,
      dry_run_execution_result_receipt_immutable_evidence_denial_matrix_bound:true,
      dry_run_execution_result_receipt_audit_trail_request_denied:true,
      dry_run_execution_result_receipt_immutable_evidence_request_denied:true,
      dry_run_execution_result_receipt_hash_chain_denied:true,
      dry_run_execution_result_receipt_merkle_root_denied:true,
      dry_run_execution_result_receipt_attestation_denied:true,
      dry_run_execution_result_receipt_witness_denied:true,
      dry_run_execution_result_receipt_notary_denied:true,
      dry_run_execution_result_receipt_ledger_index_delivery_evidence_denied:true,
      dry_run_execution_result_receipt_audit_evidence_authority_denied:true,
      dry_run_execution_result_receipt_audit_evidence_handoff_bound:true,
      dry_run_execution_result_receipt_audit_evidence_persistence_forbidden:true,
      dry_run_execution_execution_forbidden_on_audit_evidence_route:true,
      dry_run_execution_result_receipt_persistence_forbidden_on_audit_evidence_route:true,
      production_write_execution_forbidden_on_audit_evidence_route:true,
      production_durable_memory_write_forbidden:true,
      memory_store_mutation_forbidden:true,
      wal_write_forbidden_on_audit_evidence_route:true,
      receipt_persist_forbidden_on_audit_evidence_route:true,
      rollback_execution_forbidden_on_audit_evidence_route:true,
      tombstone_write_forbidden_on_audit_evidence_route:true,
      kg_live_write_forbidden:true,
      provider_model_invocation_forbidden:true,
      credential_channel_public_release_forbidden:true,
      install_restart_active_binary_mutation_forbidden:true,
      allowed_next_actions:[
        {
          action:"run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary_require_live_gate",
          status:"allowed_verification_only",
          accepts_audit_trail_immutable_evidence_denial_matrix:true,
          records_audit_trail:false,
          records_immutable_evidence:false,
          persists_evidence:false,
          records_hash_chain:false,
          records_attestation:false,
          promotes_authority:false,
          executes_dry_run:false,
          persists_dry_run_result_receipt:false,
          writes_production_durable_memory:false,
          writes_memory_store:false,
          writes_wal:false,
          persists_receipt:false
        },
        {
          action:"prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_retention_expiry_garbage_collection_denial_boundary",
          status:"requires_separate_result_receipt_retention_expiry_garbage_collection_denial_gate",
          requires_scoped_production_durable_memory_write_dry_run_execution_result_receipt_audit_trail_immutable_evidence_denial_boundary:true,
          records_audit_trail:false,
          records_immutable_evidence:false,
          persists_evidence:false,
          executes_dry_run:false,
          writes_production_durable_memory:false,
          persists_dry_run_result_receipt:false
        }
      ]
    }
    + zero_fields($false_keys)
    + true_count_fields($true_keys)
    + {
      side_effects:(zero_fields($false_keys) + true_count_fields($true_keys))
    })
  '
