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
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-operator-packet-acceptance-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-operator-packet-acceptance-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_ready == true
    and $source.scoped_production_durable_memory_write_operator_packet_acceptance_ready == true
    and $source.scoped_production_durable_memory_write_operator_packet_acceptance_accepted == true
    and $source.accepted_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count == 1
    and $source.blocked_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count == 9
    and $source.scoped_production_durable_memory_write_operator_packet_acceptance_result_accepted_count == 1
    and $source.operator_packet_persisted == false
    and $source.operator_packet_acceptance_receipt_persisted == false
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
    and $source.raw_payload_plaintext_recorded == false
    and $source.raw_payload_plaintext_persisted == false
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
    and $source.side_effects.scoped_production_durable_memory_write_operator_packet_acceptance_performed == true
    and $source.side_effects.scoped_production_durable_memory_write_operator_packet_acceptance_result_accepted == true
    and $source.side_effects.operator_packet_persisted == false
    and $source.side_effects.production_durable_memory_store_write_performed == false
    and $source.side_effects.memory_store_write_performed == false
    and $source.side_effects.wal_write_performed == false
    and $source.side_effects.receipt_persisted == false
    and $source.side_effects.external_send_performed == false
    and $source.allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary"
    and $source.allowed_next_actions[1].requires_scoped_production_durable_memory_write_operator_packet_acceptance_boundary == true
    and $source.allowed_next_actions[1].writes_production_durable_memory == false
    and $source.allowed_next_actions[1].persists_operator_packet == false
  ' >/dev/null

approved_production_namespace="$(jq -r '.approved_production_namespace // ""' <<<"$SOURCE_JSON")"
approved_production_store="$(jq -r '.approved_production_store // ""' <<<"$SOURCE_JSON")"
approved_production_scope="$(jq -r '.approved_production_scope // ""' <<<"$SOURCE_JSON")"
production_durable_memory_target_id="$(jq -r '.production_durable_memory_target_id // ""' <<<"$SOURCE_JSON")"
production_durable_memory_payload_class="$(jq -r '.production_durable_memory_payload_class // ""' <<<"$SOURCE_JSON")"
operator_packet_scope="$(jq -r '.operator_packet_scope // ""' <<<"$SOURCE_JSON")"
source_report_sha256="$(sha256_text "$SOURCE_JSON")"
source_acceptance_result_hash_sha256="$(jq -r '.operator_packet_acceptance_result_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_acceptance_boundary_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_operator_packet_acceptance_boundary_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_acceptance_policy_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_operator_packet_acceptance_policy_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_target_hash_sha256="$(jq -r '.source_production_durable_memory_write_preflight_target_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_operator_packet_hash_sha256="$(jq -r '.source_production_durable_memory_write_preflight_operator_packet_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_acceptance_envelope_hash_sha256="$(jq -r '.operator_packet_acceptance_envelope_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_acceptance_identity_session_hash_sha256="$(jq -r '.operator_packet_acceptance_identity_session_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_acceptance_signature_hash_sha256="$(jq -r '.operator_packet_acceptance_signature_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_acceptance_nonce_hash_sha256="$(jq -r '.operator_packet_acceptance_nonce_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_acceptance_command_hash_sha256="$(jq -r '.operator_packet_acceptance_command_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_acceptance_receipt_plan_hash_sha256="$(jq -r '.operator_packet_acceptance_receipt_plan_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_acceptance_replay_guard_hash_sha256="$(jq -r '.operator_packet_acceptance_replay_guard_hash_sha256 // ""' <<<"$SOURCE_JSON")"

acceptance_receipt_envelope_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-envelope:v1:source-result=${source_acceptance_result_hash_sha256}:receipt-plan=${source_acceptance_receipt_plan_hash_sha256}:target=${source_target_hash_sha256}:persist-now=false"
)"
acceptance_receipt_identity_session_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-identity-session:v1:envelope=${acceptance_receipt_envelope_hash_sha256}:source-identity-session=${source_acceptance_identity_session_hash_sha256}:operator-bound=true"
)"
acceptance_receipt_digest_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-digest:v1:envelope=${acceptance_receipt_envelope_hash_sha256}:command=${source_acceptance_command_hash_sha256}:signature=${source_acceptance_signature_hash_sha256}:redacted=true"
)"
acceptance_receipt_hash_chain_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-hash-chain:v1:digest=${acceptance_receipt_digest_hash_sha256}:source-boundary=${source_acceptance_boundary_hash_sha256}:append-now=false"
)"
acceptance_receipt_readback_plan_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-readback-plan:v1:hash-chain=${acceptance_receipt_hash_chain_hash_sha256}:readback-now=false"
)"
acceptance_receipt_replay_guard_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-replay-guard:v1:readback=${acceptance_receipt_readback_plan_hash_sha256}:nonce=${source_acceptance_nonce_hash_sha256}:source-replay=${source_acceptance_replay_guard_hash_sha256}:replay=false"
)"
acceptance_receipt_result_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-result:v1:envelope=${acceptance_receipt_envelope_hash_sha256}:digest=${acceptance_receipt_digest_hash_sha256}:replay=${acceptance_receipt_replay_guard_hash_sha256}:accepted=true"
)"
scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-boundary:v1:source=${source_report_sha256}:result=${acceptance_receipt_result_hash_sha256}:fixtures=10:accepted=1:denials=40:receipt-persist=false:production-write=false"
)"
scoped_production_durable_memory_write_operator_packet_acceptance_receipt_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-operator-packet-acceptance-receipt-policy:v1:bind-source-acceptance-result-envelope-digest-hash-chain-readback-replay:no-receipt-persistence:no-production-write:no-kg:no-provider:no-channel:no-release:no-install"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_gate" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-operator-packet-acceptance-receipt-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-operator-packet-acceptance-receipt-boundary --json" \
  --arg approved_production_namespace "$approved_production_namespace" \
  --arg approved_production_store "$approved_production_store" \
  --arg approved_production_scope "$approved_production_scope" \
  --arg production_durable_memory_target_id "$production_durable_memory_target_id" \
  --arg production_durable_memory_payload_class "$production_durable_memory_payload_class" \
  --arg operator_packet_scope "$operator_packet_scope" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg source_acceptance_result_hash_sha256 "$source_acceptance_result_hash_sha256" \
  --arg source_acceptance_boundary_hash_sha256 "$source_acceptance_boundary_hash_sha256" \
  --arg source_acceptance_policy_hash_sha256 "$source_acceptance_policy_hash_sha256" \
  --arg source_target_hash_sha256 "$source_target_hash_sha256" \
  --arg source_operator_packet_hash_sha256 "$source_operator_packet_hash_sha256" \
  --arg source_acceptance_envelope_hash_sha256 "$source_acceptance_envelope_hash_sha256" \
  --arg source_acceptance_identity_session_hash_sha256 "$source_acceptance_identity_session_hash_sha256" \
  --arg source_acceptance_signature_hash_sha256 "$source_acceptance_signature_hash_sha256" \
  --arg source_acceptance_nonce_hash_sha256 "$source_acceptance_nonce_hash_sha256" \
  --arg source_acceptance_command_hash_sha256 "$source_acceptance_command_hash_sha256" \
  --arg source_acceptance_receipt_plan_hash_sha256 "$source_acceptance_receipt_plan_hash_sha256" \
  --arg source_acceptance_replay_guard_hash_sha256 "$source_acceptance_replay_guard_hash_sha256" \
  --arg acceptance_receipt_envelope_hash_sha256 "$acceptance_receipt_envelope_hash_sha256" \
  --arg acceptance_receipt_identity_session_hash_sha256 "$acceptance_receipt_identity_session_hash_sha256" \
  --arg acceptance_receipt_digest_hash_sha256 "$acceptance_receipt_digest_hash_sha256" \
  --arg acceptance_receipt_hash_chain_hash_sha256 "$acceptance_receipt_hash_chain_hash_sha256" \
  --arg acceptance_receipt_readback_plan_hash_sha256 "$acceptance_receipt_readback_plan_hash_sha256" \
  --arg acceptance_receipt_replay_guard_hash_sha256 "$acceptance_receipt_replay_guard_hash_sha256" \
  --arg acceptance_receipt_result_hash_sha256 "$acceptance_receipt_result_hash_sha256" \
  --arg scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_hash_sha256 "$scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_hash_sha256" \
  --arg scoped_production_durable_memory_write_operator_packet_acceptance_receipt_policy_hash_sha256 "$scoped_production_durable_memory_write_operator_packet_acceptance_receipt_policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  ([
    "source_operator_packet_acceptance_boundary_required",
    "operator_packet_acceptance_result_required",
    "acceptance_receipt_envelope_required",
    "acceptance_receipt_identity_session_required",
    "acceptance_receipt_digest_required",
    "acceptance_receipt_hash_chain_required",
    "acceptance_receipt_readback_plan_required",
    "acceptance_receipt_replay_guard_required",
    "acceptance_receipt_handoff_required",
    "acceptance_receipt_persistence_forbidden_on_report_route",
    "production_write_execution_forbidden_on_acceptance_receipt_route",
    "kg_provider_channel_release_install_active_binary_forbidden"
  ]) as $surfaces
  | ([
    "source_operator_packet_acceptance_boundary_required",
    "source_operator_packet_acceptance_result_hash_required",
    "source_operator_packet_acceptance_policy_hash_required",
    "approved_production_namespace_required",
    "approved_production_store_required",
    "approved_production_scope_required",
    "production_durable_memory_target_required",
    "operator_packet_acceptance_envelope_required",
    "operator_identity_session_required",
    "operator_packet_signature_required",
    "single_use_acceptance_nonce_required",
    "explicit_acceptance_command_required",
    "acceptance_receipt_plan_required",
    "acceptance_receipt_envelope_required",
    "acceptance_receipt_identity_session_required",
    "acceptance_receipt_digest_required",
    "acceptance_receipt_hash_chain_required",
    "acceptance_receipt_readback_plan_required",
    "acceptance_receipt_replay_guard_required",
    "acceptance_receipt_handoff_required",
    "acceptance_receipt_persistence_report_route_denied",
    "acceptance_receipt_filesystem_write_denied",
    "acceptance_receipt_ledger_recording_denied",
    "acceptance_receipt_delivery_denied",
    "operator_packet_persistence_report_route_denied",
    "production_write_execution_report_route_denied",
    "production_durable_memory_backend_write_denied",
    "durable_memory_backend_read_or_rollback_denied",
    "memory_store_mutation_denied",
    "wal_write_report_route_denied",
    "receipt_persist_report_route_denied",
    "post_write_readback_report_route_denied",
    "rollback_execution_report_route_denied",
    "tombstone_write_report_route_denied",
    "raw_payload_plaintext_denied",
    "kg_live_write_denied",
    "provider_model_invocation_denied",
    "credential_channel_release_install_denied",
    "active_binary_mutation_denied",
    "unrestricted_full_live_activation_denied"
  ]) as $denials
  | ([
    "acceptance_receipt_persisted",
    "acceptance_receipt_filesystem_written",
    "acceptance_receipt_ledger_recorded",
    "acceptance_receipt_delivered",
    "acceptance_receipt_materialized",
    "operator_packet_persisted",
    "operator_packet_ledger_recorded",
    "operator_packet_filesystem_written",
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
  ]) as $false_fields
  | ([
    "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_performed",
    "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_result_recorded",
    "scoped_production_durable_memory_write_operator_packet_acceptance_receipt_result_accepted",
    "source_operator_packet_acceptance_boundary_accepted",
    "operator_packet_acceptance_result_bound",
    "acceptance_receipt_envelope_bound",
    "acceptance_receipt_identity_session_bound",
    "acceptance_receipt_digest_bound",
    "acceptance_receipt_hash_chain_bound",
    "acceptance_receipt_readback_plan_bound",
    "acceptance_receipt_replay_guard_bound",
    "acceptance_receipt_handoff_bound",
    "acceptance_receipt_persistence_forbidden_on_report_route",
    "production_write_execution_forbidden_on_acceptance_receipt_route",
    "kg_provider_channel_release_install_active_binary_forbidden"
  ]) as $true_fields
  | ($false_fields | reduce .[] as $key ({}; .[$key] = false | .[$key + "_count"] = 0)) as $false_map
  | ($true_fields | reduce .[] as $key ({}; .[$key] = true | .[$key + "_count"] = 1)) as $true_map
  | ([
    "source_operator_packet_acceptance_boundary_bound",
    "approved_production_namespace_bound",
    "approved_production_store_bound",
    "approved_production_scope_bound",
    "production_durable_memory_target_bound",
    "operator_packet_acceptance_result_bound",
    "operator_packet_acceptance_envelope_bound",
    "operator_identity_session_acceptance_bound",
    "operator_packet_signature_acceptance_bound",
    "single_use_acceptance_nonce_bound",
    "explicit_acceptance_command_bound",
    "acceptance_receipt_envelope_bound",
    "acceptance_receipt_identity_session_bound",
    "acceptance_receipt_digest_bound",
    "acceptance_receipt_hash_chain_bound",
    "acceptance_receipt_readback_plan_bound",
    "acceptance_receipt_replay_guard_bound",
    "acceptance_receipt_handoff_bound",
    "acceptance_receipt_persistence_forbidden_on_report_route",
    "operator_packet_persistence_forbidden_on_report_route",
    "production_write_execution_forbidden_on_acceptance_receipt_route",
    "production_durable_memory_write_forbidden",
    "memory_store_mutation_forbidden",
    "wal_write_forbidden_on_acceptance_receipt_route",
    "receipt_persist_forbidden_on_acceptance_receipt_route",
    "rollback_execution_forbidden_on_acceptance_receipt_route",
    "tombstone_write_forbidden_on_acceptance_receipt_route",
    "kg_live_write_forbidden",
    "provider_model_invocation_forbidden",
    "credential_channel_public_release_forbidden",
    "install_restart_active_binary_mutation_forbidden"
  ] | reduce .[] as $key ({}; .[$key] = true)) as $binding_map
  | ([
    {
      id:"scoped-production-durable-memory-write-operator-packet-acceptance-receipt",
      fixture_id:"scoped-production-durable-memory-write-operator-packet-acceptance-receipt",
      scoped_production_durable_memory_write_operator_packet_acceptance_receipt_accepted:true,
      reason:"operator_packet_acceptance_receipt_guards_bound_without_receipt_persistence_or_production_write",
      source_operator_packet_acceptance_boundary_bound:true,
      operator_packet_acceptance_result_bound:true,
      acceptance_receipt_envelope_bound:true,
      acceptance_receipt_digest_bound:true,
      acceptance_receipt_hash_chain_bound:true,
      acceptance_receipt_readback_plan_bound:true,
      acceptance_receipt_replay_guard_bound:true,
      acceptance_receipt_persisted:false,
      operator_packet_persisted:false,
      production_durable_memory_store_write_performed:false,
      external_send_performed:false
    }
  ] + ([
    "missing-operator-packet-acceptance-source",
    "missing-acceptance-result-hash",
    "missing-acceptance-receipt-envelope",
    "missing-acceptance-receipt-digest",
    "missing-acceptance-receipt-hash-chain",
    "missing-acceptance-receipt-readback-plan",
    "missing-acceptance-receipt-replay-guard",
    "receipt-persistence-side-effect-attempt",
    "production-write-or-external-side-effect-attempt"
  ] | map({
    id:.,
    fixture_id:.,
    scoped_production_durable_memory_write_operator_packet_acceptance_receipt_accepted:false,
    reason:"blocked_noop",
    acceptance_receipt_persisted:false,
    operator_packet_persisted:false,
    production_durable_memory_store_write_performed:false,
    external_send_performed:false
  }))) as $fixtures
  | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      endpoint:$endpoint,
      source_command:$source_command,
      native_route:true,
      side_effect_free:false,
      external_side_effect_free:true,
      audit_date:"2026-07-05",
      memory_write_execution_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_ready:true,
      scoped_production_durable_memory_write_operator_packet_acceptance_receipt_ready:true,
      scoped_production_durable_memory_write_operator_packet_acceptance_receipt_performed:true,
      scoped_production_durable_memory_write_operator_packet_acceptance_receipt_accepted:true,
      scoped_production_durable_memory_write_operator_packet_acceptance_receipt_mode:"acceptance_receipt_boundary_no_receipt_persistence_no_production_durable_memory_mutation",
      source_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_ready:true,
      source_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_report_sha256:$source_report_sha256,
      source_operator_packet_acceptance_result_hash_sha256:$source_acceptance_result_hash_sha256,
      source_scoped_production_durable_memory_write_operator_packet_acceptance_boundary_hash_sha256:$source_acceptance_boundary_hash_sha256,
      source_scoped_production_durable_memory_write_operator_packet_acceptance_policy_hash_sha256:$source_acceptance_policy_hash_sha256,
      source_scoped_production_durable_memory_write_operator_packet_acceptance_accepted_count:$source.scoped_production_durable_memory_write_operator_packet_acceptance_result_accepted_count,
      source_accepted_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count:$source.accepted_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count,
      source_blocked_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count:$source.blocked_scoped_production_durable_memory_write_operator_packet_acceptance_fixture_count,
      approved_production_namespace:$approved_production_namespace,
      approved_production_store:$approved_production_store,
      approved_production_scope:$approved_production_scope,
      production_durable_memory_target_id:$production_durable_memory_target_id,
      production_durable_memory_payload_class:$production_durable_memory_payload_class,
      operator_packet_scope:$operator_packet_scope,
      source_production_durable_memory_write_preflight_target_hash_sha256:$source_target_hash_sha256,
      source_production_durable_memory_write_preflight_operator_packet_hash_sha256:$source_operator_packet_hash_sha256,
      source_operator_packet_acceptance_envelope_hash_sha256:$source_acceptance_envelope_hash_sha256,
      source_operator_packet_acceptance_identity_session_hash_sha256:$source_acceptance_identity_session_hash_sha256,
      source_operator_packet_acceptance_signature_hash_sha256:$source_acceptance_signature_hash_sha256,
      source_operator_packet_acceptance_nonce_hash_sha256:$source_acceptance_nonce_hash_sha256,
      source_operator_packet_acceptance_command_hash_sha256:$source_acceptance_command_hash_sha256,
      source_operator_packet_acceptance_receipt_plan_hash_sha256:$source_acceptance_receipt_plan_hash_sha256,
      source_operator_packet_acceptance_replay_guard_hash_sha256:$source_acceptance_replay_guard_hash_sha256,
      acceptance_receipt_envelope_hash_sha256:$acceptance_receipt_envelope_hash_sha256,
      acceptance_receipt_identity_session_hash_sha256:$acceptance_receipt_identity_session_hash_sha256,
      acceptance_receipt_digest_hash_sha256:$acceptance_receipt_digest_hash_sha256,
      acceptance_receipt_hash_chain_hash_sha256:$acceptance_receipt_hash_chain_hash_sha256,
      acceptance_receipt_readback_plan_hash_sha256:$acceptance_receipt_readback_plan_hash_sha256,
      acceptance_receipt_replay_guard_hash_sha256:$acceptance_receipt_replay_guard_hash_sha256,
      acceptance_receipt_result_hash_sha256:$acceptance_receipt_result_hash_sha256,
      scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_hash_sha256:$scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_hash_sha256,
      scoped_production_durable_memory_write_operator_packet_acceptance_receipt_policy_hash_sha256:$scoped_production_durable_memory_write_operator_packet_acceptance_receipt_policy_hash_sha256,
      required_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_surface_count:($surfaces | length),
      ready_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_surface_count:($surfaces | length),
      scoped_production_durable_memory_write_operator_packet_acceptance_receipt_surfaces:$surfaces,
      scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count:($fixtures | length),
      accepted_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count:1,
      blocked_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixture_count:9,
      scoped_production_durable_memory_write_operator_packet_acceptance_receipt_fixtures:$fixtures,
      denied_by_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary:$denials,
      denied_by_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_count:($denials | length),
      allowed_next_actions:[
        {
          action:"run_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary_require_live_gate",
          status:"allowed_verification_only",
          accepts_acceptance_receipt_evidence:true,
          persists_acceptance_receipt:false,
          persists_operator_packet:false,
          writes_production_durable_memory:false,
          writes_memory_store:false,
          writes_wal:false,
          persists_receipt:false,
          executes_rollback:false,
          writes_tombstone:false
        },
        {
          action:"prepare_scoped_production_durable_memory_write_dry_run_execution_envelope_boundary",
          status:"requires_separate_dry_run_execution_envelope_gate",
          requires_scoped_production_durable_memory_write_operator_packet_acceptance_receipt_boundary:true,
          writes_production_durable_memory:false,
          persists_acceptance_receipt:false,
          persists_operator_packet:false
        }
      ]
    }
    + $false_map
    + $true_map
    + $binding_map
    + {
      side_effects:($false_map + $true_map)
    }
  '
