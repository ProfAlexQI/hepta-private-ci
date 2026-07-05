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
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-minimal-scoped-memory-real-write-canary-durable-store-write-rollback-tombstone-zero-residue-acceptance-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_accepted == true
    and $source.accepted_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixture_count == 1
    and $source.blocked_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixture_count == 9
    and $source.zero_residue_acceptance_result_accepted_count == 1
    and $source.source_single_shot_memory_store_write_performed_count == 1
    and $source.source_single_shot_wal_write_performed_count == 1
    and $source.source_single_shot_receipt_persisted_count == 1
    and $source.source_single_shot_post_write_readback_performed_count == 1
    and $source.source_single_shot_rollback_executed_count == 1
    and $source.source_single_shot_tombstone_cleanup_executed_count == 1
    and $source.source_single_shot_canary_post_rollback_memory_count == 0
    and $source.source_single_shot_canary_artifact_post_cleanup_count == 0
    and $source.source_single_shot_canary_artifact_zero_residue_confirmed == true
    and $source.rollback_tombstone_cleanup_absence_verified == true
    and $source.artifact_zero_residue_verified == true
    and $source.memory_store_write_performed == false
    and $source.wal_write_performed == false
    and $source.receipt_persisted == false
    and $source.rollback_executed == false
    and $source.tombstone_cleanup_executed == false
    and $source.production_durable_memory_store_write_performed == false
    and $source.actual_production_durable_memory_write_performed == false
    and $source.durable_memory_store_write_performed == false
    and $source.durable_memory_store_read_performed == false
    and $source.durable_memory_store_rollback_performed == false
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
    and $source.side_effects.durable_store_write_rollback_tombstone_zero_residue_acceptance_performed == true
    and $source.side_effects.durable_store_write_rollback_tombstone_zero_residue_acceptance_result_accepted == true
    and $source.side_effects.memory_store_write_performed == false
    and $source.side_effects.wal_write_performed == false
    and $source.side_effects.receipt_persisted == false
    and $source.side_effects.rollback_executed == false
    and $source.side_effects.tombstone_cleanup_executed == false
    and $source.side_effects.external_send_performed == false
    and $source.allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_preflight_boundary"
    and $source.allowed_next_actions[1].requires_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary == true
    and $source.allowed_next_actions[1].writes_production_durable_memory == false
  ' >/dev/null

approved_production_namespace="hepta.memory.production.scoped"
approved_production_store="hepta-memory-durable-store-production-preflight-only"
approved_production_scope="operator-approved-session"
production_durable_memory_target_id="hepta-scoped-production-durable-memory-write-target-v1"
production_durable_memory_payload_class="redacted-minimal-operator-approved-memory-fact"
operator_packet_scope="hepta.memory.production.scoped:session:single-write-preflight"
source_report_sha256="$(sha256_text "$SOURCE_JSON")"
source_zero_residue_acceptance_boundary_hash_sha256="$(
  jq -r '.minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_hash_sha256 // ""' <<<"$SOURCE_JSON"
)"
source_zero_residue_acceptance_policy_hash_sha256="$(
  jq -r '.minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_policy_hash_sha256 // ""' <<<"$SOURCE_JSON"
)"
source_zero_residue_acceptance_hash_sha256="$(
  jq -r '.zero_residue_acceptance_hash_sha256 // ""' <<<"$SOURCE_JSON"
)"

production_durable_memory_write_preflight_target_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-target:v1:namespace=${approved_production_namespace}:store=${approved_production_store}:scope=${approved_production_scope}:target=${production_durable_memory_target_id}:source=${source_zero_residue_acceptance_hash_sha256}"
)"
production_durable_memory_write_preflight_operator_packet_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-operator-packet-preflight:v1:scope=${operator_packet_scope}:target=${production_durable_memory_write_preflight_target_hash_sha256}:requires-fresh-approval=true"
)"
production_durable_memory_write_preflight_nonce_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-nonce-preflight:v1:packet=${production_durable_memory_write_preflight_operator_packet_hash_sha256}:single-use=true"
)"
production_durable_memory_write_preflight_command_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-explicit-command-preflight:v1:nonce=${production_durable_memory_write_preflight_nonce_hash_sha256}:budget=single-write"
)"
production_durable_memory_write_preflight_payload_redaction_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-payload-redaction-preflight:v1:class=${production_durable_memory_payload_class}:raw-plaintext-recording=false"
)"
production_durable_memory_write_preflight_wal_receipt_plan_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-wal-receipt-plan:v1:command=${production_durable_memory_write_preflight_command_hash_sha256}:payload=${production_durable_memory_write_preflight_payload_redaction_hash_sha256}:persist-now=false"
)"
production_durable_memory_write_preflight_readback_plan_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-readback-plan:v1:wal-receipt=${production_durable_memory_write_preflight_wal_receipt_plan_hash_sha256}:execute-now=false"
)"
production_durable_memory_write_preflight_rollback_tombstone_zero_residue_plan_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-rollback-tombstone-zero-residue-plan:v1:readback=${production_durable_memory_write_preflight_readback_plan_hash_sha256}:source-zero-residue=${source_zero_residue_acceptance_hash_sha256}:execute-now=false"
)"
scoped_production_durable_memory_write_preflight_result_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-preflight-result:v1:target=${production_durable_memory_write_preflight_target_hash_sha256}:operator=${production_durable_memory_write_preflight_operator_packet_hash_sha256}:command=${production_durable_memory_write_preflight_command_hash_sha256}:rollback=${production_durable_memory_write_preflight_rollback_tombstone_zero_residue_plan_hash_sha256}:accepted=true"
)"
scoped_production_durable_memory_write_preflight_boundary_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-preflight-boundary:v1:source=${source_report_sha256}:result=${scoped_production_durable_memory_write_preflight_result_hash_sha256}:fixtures=10:accepted=1:denials=36:production-write=false"
)"
scoped_production_durable_memory_write_preflight_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-preflight-policy:v1:bind-target-operator-nonce-command-wal-receipt-readback-rollback-tombstone-zero-residue:no-production-write:no-kg:no-provider:no-channel:no-release:no-install"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_production_durable_memory_write_preflight_boundary_gate" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-preflight-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-preflight-boundary --json" \
  --arg approved_production_namespace "$approved_production_namespace" \
  --arg approved_production_store "$approved_production_store" \
  --arg approved_production_scope "$approved_production_scope" \
  --arg production_durable_memory_target_id "$production_durable_memory_target_id" \
  --arg production_durable_memory_payload_class "$production_durable_memory_payload_class" \
  --arg operator_packet_scope "$operator_packet_scope" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg source_zero_residue_acceptance_boundary_hash_sha256 "$source_zero_residue_acceptance_boundary_hash_sha256" \
  --arg source_zero_residue_acceptance_policy_hash_sha256 "$source_zero_residue_acceptance_policy_hash_sha256" \
  --arg source_zero_residue_acceptance_hash_sha256 "$source_zero_residue_acceptance_hash_sha256" \
  --arg production_durable_memory_write_preflight_target_hash_sha256 "$production_durable_memory_write_preflight_target_hash_sha256" \
  --arg production_durable_memory_write_preflight_operator_packet_hash_sha256 "$production_durable_memory_write_preflight_operator_packet_hash_sha256" \
  --arg production_durable_memory_write_preflight_nonce_hash_sha256 "$production_durable_memory_write_preflight_nonce_hash_sha256" \
  --arg production_durable_memory_write_preflight_command_hash_sha256 "$production_durable_memory_write_preflight_command_hash_sha256" \
  --arg production_durable_memory_write_preflight_payload_redaction_hash_sha256 "$production_durable_memory_write_preflight_payload_redaction_hash_sha256" \
  --arg production_durable_memory_write_preflight_wal_receipt_plan_hash_sha256 "$production_durable_memory_write_preflight_wal_receipt_plan_hash_sha256" \
  --arg production_durable_memory_write_preflight_readback_plan_hash_sha256 "$production_durable_memory_write_preflight_readback_plan_hash_sha256" \
  --arg production_durable_memory_write_preflight_rollback_tombstone_zero_residue_plan_hash_sha256 "$production_durable_memory_write_preflight_rollback_tombstone_zero_residue_plan_hash_sha256" \
  --arg scoped_production_durable_memory_write_preflight_result_hash_sha256 "$scoped_production_durable_memory_write_preflight_result_hash_sha256" \
  --arg scoped_production_durable_memory_write_preflight_boundary_hash_sha256 "$scoped_production_durable_memory_write_preflight_boundary_hash_sha256" \
  --arg scoped_production_durable_memory_write_preflight_policy_hash_sha256 "$scoped_production_durable_memory_write_preflight_policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  ([
    "source_zero_residue_acceptance_boundary_required",
    "production_durable_memory_target_required",
    "operator_approval_packet_required",
    "single_use_nonce_required",
    "explicit_command_required",
    "payload_redaction_required",
    "wal_receipt_plan_required",
    "post_write_readback_plan_required",
    "rollback_tombstone_zero_residue_plan_required",
    "replay_idempotency_guard_required",
    "operator_preflight_handoff_required",
    "production_write_execution_forbidden_on_preflight_route"
  ]) as $surfaces
  | ([
    "source_zero_residue_acceptance_boundary_required",
    "source_zero_residue_acceptance_hash_required",
    "approved_production_namespace_required",
    "approved_production_store_required",
    "approved_production_scope_required",
    "production_durable_memory_target_required",
    "operator_approval_packet_required",
    "operator_identity_session_required",
    "operator_scope_binding_required",
    "single_use_nonce_required",
    "explicit_command_required",
    "command_budget_required",
    "payload_redaction_required",
    "raw_plaintext_payload_denied",
    "wal_plan_required",
    "receipt_plan_required",
    "receipt_hash_chain_required",
    "post_write_readback_plan_required",
    "rollback_plan_required",
    "tombstone_cleanup_plan_required",
    "zero_residue_plan_required",
    "replay_idempotency_guard_required",
    "preflight_result_record_required",
    "preflight_result_readback_required",
    "production_write_execution_report_route_denied",
    "production_durable_memory_backend_write_denied",
    "durable_memory_backend_read_or_rollback_denied",
    "memory_store_mutation_denied",
    "wal_write_report_route_denied",
    "receipt_persist_report_route_denied",
    "rollback_execution_report_route_denied",
    "tombstone_write_report_route_denied",
    "kg_live_write_denied",
    "provider_model_invocation_denied",
    "credential_channel_release_install_denied",
    "unrestricted_full_live_activation_denied"
  ]) as $denials
  | ([
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
    "scoped_production_durable_memory_write_preflight_performed",
    "scoped_production_durable_memory_write_preflight_result_recorded",
    "scoped_production_durable_memory_write_preflight_result_accepted",
    "source_zero_residue_acceptance_boundary_accepted",
    "production_durable_memory_target_bound",
    "operator_approval_packet_preflight_bound",
    "operator_identity_session_preflight_bound",
    "single_use_nonce_preflight_bound",
    "explicit_command_preflight_bound",
    "payload_redaction_preflight_bound",
    "wal_receipt_preflight_bound",
    "post_write_readback_preflight_bound",
    "rollback_tombstone_zero_residue_preflight_bound",
    "replay_idempotency_preflight_bound",
    "production_write_execution_forbidden_on_preflight_route",
    "kg_provider_channel_release_install_active_binary_forbidden"
  ]) as $true_fields
  | ($false_fields | reduce .[] as $key ({}; .[$key] = false | .[$key + "_count"] = 0)) as $false_map
  | ($true_fields | reduce .[] as $key ({}; .[$key] = true | .[$key + "_count"] = 1)) as $true_map
  | ([
    "source_zero_residue_acceptance_boundary_bound",
    "approved_production_namespace_bound",
    "approved_production_store_bound",
    "approved_production_scope_bound",
    "production_durable_memory_target_bound",
    "operator_approval_packet_preflight_bound",
    "operator_identity_session_preflight_bound",
    "single_use_nonce_preflight_bound",
    "explicit_command_preflight_bound",
    "payload_redaction_preflight_bound",
    "wal_receipt_preflight_bound",
    "post_write_readback_preflight_bound",
    "rollback_tombstone_zero_residue_preflight_bound",
    "replay_idempotency_preflight_bound",
    "production_write_execution_forbidden_on_preflight_route",
    "production_durable_memory_write_forbidden",
    "memory_store_mutation_forbidden",
    "wal_write_forbidden_on_preflight_route",
    "receipt_persist_forbidden_on_preflight_route",
    "rollback_execution_forbidden_on_preflight_route",
    "tombstone_write_forbidden_on_preflight_route",
    "kg_live_write_forbidden",
    "provider_model_invocation_forbidden",
    "credential_channel_public_release_forbidden",
    "install_restart_active_binary_mutation_forbidden"
  ] | reduce .[] as $key ({}; .[$key] = true)) as $binding_map
  | ([
    {
      id:"scoped-production-durable-memory-write-preflight",
      fixture_id:"scoped-production-durable-memory-write-preflight",
      scoped_production_durable_memory_write_preflight_accepted:true,
      reason:"production_durable_memory_write_preflight_guards_bound_without_execution",
      source_zero_residue_acceptance_boundary_bound:true,
      production_durable_memory_target_bound:true,
      operator_approval_packet_preflight_bound:true,
      single_use_nonce_preflight_bound:true,
      explicit_command_preflight_bound:true,
      wal_receipt_preflight_bound:true,
      post_write_readback_preflight_bound:true,
      rollback_tombstone_zero_residue_preflight_bound:true,
      production_durable_memory_store_write_performed:false,
      external_send_performed:false
    }
  ] + ([
    "missing-zero-residue-source",
    "wrong-production-namespace",
    "missing-operator-approval-packet",
    "missing-single-use-nonce",
    "missing-explicit-command",
    "missing-wal-receipt-plan",
    "missing-post-write-readback-plan",
    "missing-rollback-tombstone-zero-residue-plan",
    "production-write-or-external-side-effect-attempt"
  ] | map({
    id:.,
    fixture_id:.,
    scoped_production_durable_memory_write_preflight_accepted:false,
    reason:"blocked_noop",
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
      memory_write_execution_scoped_production_durable_memory_write_preflight_boundary_ready:true,
      scoped_production_durable_memory_write_preflight_ready:true,
      scoped_production_durable_memory_write_preflight_performed:true,
      scoped_production_durable_memory_write_preflight_accepted:true,
      scoped_production_durable_memory_write_preflight_mode:"preflight_only_no_production_durable_memory_mutation",
      source_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_ready:true,
      source_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_boundary_report_sha256:$source_report_sha256,
      source_zero_residue_acceptance_boundary_hash_sha256:$source_zero_residue_acceptance_boundary_hash_sha256,
      source_zero_residue_acceptance_policy_hash_sha256:$source_zero_residue_acceptance_policy_hash_sha256,
      source_zero_residue_acceptance_hash_sha256:$source_zero_residue_acceptance_hash_sha256,
      source_accepted_zero_residue_acceptance_fixture_count:$source.accepted_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixture_count,
      source_blocked_zero_residue_acceptance_fixture_count:$source.blocked_minimal_scoped_memory_real_write_canary_durable_store_write_rollback_tombstone_zero_residue_acceptance_fixture_count,
      source_zero_residue_acceptance_result_accepted_count:$source.zero_residue_acceptance_result_accepted_count,
      source_single_shot_memory_store_write_performed_count:$source.source_single_shot_memory_store_write_performed_count,
      source_single_shot_wal_write_performed_count:$source.source_single_shot_wal_write_performed_count,
      source_single_shot_receipt_persisted_count:$source.source_single_shot_receipt_persisted_count,
      source_single_shot_post_write_readback_performed_count:$source.source_single_shot_post_write_readback_performed_count,
      source_single_shot_rollback_executed_count:$source.source_single_shot_rollback_executed_count,
      source_single_shot_tombstone_cleanup_executed_count:$source.source_single_shot_tombstone_cleanup_executed_count,
      source_single_shot_canary_post_rollback_memory_count:$source.source_single_shot_canary_post_rollback_memory_count,
      source_single_shot_canary_artifact_post_cleanup_count:$source.source_single_shot_canary_artifact_post_cleanup_count,
      source_single_shot_canary_artifact_zero_residue_confirmed:$source.source_single_shot_canary_artifact_zero_residue_confirmed,
      source_current_memory_store_write_performed_count:0,
      source_current_wal_write_performed_count:0,
      source_current_receipt_persisted_count:0,
      source_current_rollback_executed_count:0,
      source_current_tombstone_cleanup_executed_count:0,
      source_current_durable_memory_store_write_performed_count:0,
      source_current_external_send_performed_count:0,
      approved_production_namespace:$approved_production_namespace,
      approved_production_store:$approved_production_store,
      approved_production_scope:$approved_production_scope,
      production_durable_memory_target_id:$production_durable_memory_target_id,
      production_durable_memory_payload_class:$production_durable_memory_payload_class,
      operator_packet_scope:$operator_packet_scope,
      production_durable_memory_write_preflight_target_hash_sha256:$production_durable_memory_write_preflight_target_hash_sha256,
      production_durable_memory_write_preflight_operator_packet_hash_sha256:$production_durable_memory_write_preflight_operator_packet_hash_sha256,
      production_durable_memory_write_preflight_nonce_hash_sha256:$production_durable_memory_write_preflight_nonce_hash_sha256,
      production_durable_memory_write_preflight_command_hash_sha256:$production_durable_memory_write_preflight_command_hash_sha256,
      production_durable_memory_write_preflight_payload_redaction_hash_sha256:$production_durable_memory_write_preflight_payload_redaction_hash_sha256,
      production_durable_memory_write_preflight_wal_receipt_plan_hash_sha256:$production_durable_memory_write_preflight_wal_receipt_plan_hash_sha256,
      production_durable_memory_write_preflight_readback_plan_hash_sha256:$production_durable_memory_write_preflight_readback_plan_hash_sha256,
      production_durable_memory_write_preflight_rollback_tombstone_zero_residue_plan_hash_sha256:$production_durable_memory_write_preflight_rollback_tombstone_zero_residue_plan_hash_sha256,
      scoped_production_durable_memory_write_preflight_result_hash_sha256:$scoped_production_durable_memory_write_preflight_result_hash_sha256,
      scoped_production_durable_memory_write_preflight_boundary_hash_sha256:$scoped_production_durable_memory_write_preflight_boundary_hash_sha256,
      scoped_production_durable_memory_write_preflight_policy_hash_sha256:$scoped_production_durable_memory_write_preflight_policy_hash_sha256,
      required_scoped_production_durable_memory_write_preflight_surface_count:($surfaces | length),
      ready_scoped_production_durable_memory_write_preflight_surface_count:($surfaces | length),
      scoped_production_durable_memory_write_preflight_surfaces:$surfaces,
      scoped_production_durable_memory_write_preflight_fixture_count:($fixtures | length),
      accepted_scoped_production_durable_memory_write_preflight_fixture_count:1,
      blocked_scoped_production_durable_memory_write_preflight_fixture_count:9,
      scoped_production_durable_memory_write_preflight_fixtures:$fixtures,
      denied_by_scoped_production_durable_memory_write_preflight_boundary:$denials,
      denied_by_scoped_production_durable_memory_write_preflight_boundary_count:($denials | length),
      allowed_next_actions:[
        {
          action:"run_scoped_production_durable_memory_write_preflight_boundary_require_live_gate",
          status:"allowed_verification_only",
          accepts_preflight_evidence:true,
          writes_production_durable_memory:false,
          writes_memory_store:false,
          writes_wal:false,
          persists_receipt:false,
          executes_rollback:false,
          writes_tombstone:false
        },
        {
          action:"prepare_scoped_production_durable_memory_write_operator_packet_acceptance_boundary",
          status:"requires_separate_operator_packet_acceptance_gate",
          requires_scoped_production_durable_memory_write_preflight_boundary:true,
          writes_production_durable_memory:false
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
