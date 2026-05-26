#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh

GOVERNANCE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    capture_json_report "hepta-live-mutation-governance-gate" \
    scripts/hepta-live-mutation-governance-gate.sh
)"
ROLLBACK_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    capture_json_report "hepta-live-mutation-rollback-drill-gate" \
    scripts/hepta-live-mutation-rollback-drill-gate.sh
)"
MEMORY_JSON="$(curl -fsS "$BASE_URL/api/hepta-memory-capability-absorption-inventory")"
CORE_JSON="$(curl -fsS "$BASE_URL/api/hepta-core-fusion-readiness")"
DEPENDENCY_JSON="$(curl -fsS "$BASE_URL/api/hepta-engine-dependency-closure")"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson governance "$GOVERNANCE_JSON" \
  --argjson rollback "$ROLLBACK_JSON" \
  --argjson memory "$MEMORY_JSON" \
  --argjson core "$CORE_JSON" \
  --argjson dependency "$DEPENDENCY_JSON" \
  '
    $governance.runtime == "hepta"
    and $governance.status == "ready"
    and $governance.gate == "hepta_live_mutation_governance_gate"
    and $governance.approval_packet_ready == true
    and $governance.safe_default_mode == "plan_only_no_live_mutation"
    and $governance.live_mutation_execution_ready == false
    and $governance.release_installed_sha_match == true
    and $governance.rollback_anchor_present == true
    and $governance.minimum_long_soak_required_samples >= 24
    and $governance.long_soak_executed_by_this_gate == false
    and ($governance.side_effects | to_entries | all(.value == false))
    and $rollback.runtime == "hepta"
    and $rollback.status == "ready"
    and $rollback.gate == "hepta_live_mutation_rollback_drill_gate"
    and $rollback.rollback_plan_ready == true
    and $rollback.rollback_execution_enabled == false
    and $rollback.operator_approval_required_before_execution == true
    and $rollback.release_installed_sha_match == true
    and $rollback.rollback_would_change_installed_binary == true
    and ($rollback.rollback_dry_run_commands | length) >= 5
    and ($rollback.side_effects | to_entries | all(.value == false))
    and $memory.runtime == "hepta"
    and ($memory.status == "attention" or $memory.status == "ready")
    and $memory.surface_count == 14
    and $memory.absorbed_or_represented_count == 14
    and $memory.live_mutation_enabled_count == 0
    and ($memory.side_effects | to_entries | all(.value == false))
    and $core.runtime == "hepta"
    and $core.status == "ready"
    and $core.full_fusion_complete == true
    and $core.active_binary_package == "hepta-cli"
    and $core.phase_5_engine_dependency_closure_remaining_dependency_count == 0
    and ($core.phase_5_engine_dependency_closure_blockers | length) == 0
    and $dependency.runtime == "hepta"
    and $dependency.status == "ready"
    and $dependency.full_fusion_complete == true
    and $dependency.remaining_direct_dependency_count == 0
    and ($dependency.blockers | length) == 0
    and $min_long_soak_samples >= 24
    and $governance.installed_sha == $rollback.installed_sha
  ' >/dev/null

receipt_payload="$(
  jq -c -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg receipt_id "hepta-live-mutation-approval-evidence-receipt" \
    --arg base_url "$BASE_URL" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson governance "$GOVERNANCE_JSON" \
    --argjson rollback "$ROLLBACK_JSON" \
    --argjson memory "$MEMORY_JSON" \
    --argjson core "$CORE_JSON" \
    --argjson dependency "$DEPENDENCY_JSON" \
    '{
      product:$product,
      runtime:$runtime,
      receipt_id:$receipt_id,
      base_url:$base_url,
      receipt_mode:"candidate_no_write_no_activation",
      approval_evidence_receipt_ready:true,
      activation_allowed_by_receipt:false,
      operator_approval_recorded:false,
      operator_approval_id_present:false,
      trusted_evidence_recorded:false,
      receipt_persisted:false,
      receipt_persistence_enabled:false,
      live_mutation_execution_ready:false,
      single_surface_activation_scope_present:false,
      minimum_long_soak_required_samples:$min_long_soak_samples,
      long_soak_executed_by_this_gate:false,
      post_activation_soak_executed:false,
      governance_gate:{
        status:$governance.status,
        gate:$governance.gate,
        safe_default_mode:$governance.safe_default_mode,
        approval_packet_ready:$governance.approval_packet_ready,
        mutation_approval_required_count:$governance.mutation_approval_required_count,
        installed_sha:$governance.installed_sha,
        rollback_anchor_present:$governance.rollback_anchor_present,
        rollback_backup_count:$governance.rollback_backup_count
      },
      rollback_gate:{
        status:$rollback.status,
        gate:$rollback.gate,
        rollback_plan_ready:$rollback.rollback_plan_ready,
        rollback_execution_enabled:$rollback.rollback_execution_enabled,
        installed_sha:$rollback.installed_sha,
        rollback_sha:$rollback.rollback_sha,
        latest_rollback_backup:$rollback.latest_rollback_backup,
        rollback_dry_run_commands:$rollback.rollback_dry_run_commands
      },
      memory_capability_evidence:{
        surface_count:$memory.surface_count,
        absorbed_or_represented_count:$memory.absorbed_or_represented_count,
        live_mutation_enabled_count:$memory.live_mutation_enabled_count,
        gap_only_surface_count:($memory.gap_only_surface_count // 0),
        memory_capability_inventory_ready:$memory.memory_capability_inventory_ready
      },
      core_evidence:{
        full_fusion_complete:$core.full_fusion_complete,
        active_binary_package:$core.active_binary_package,
        remaining_direct_dependency_count:$dependency.remaining_direct_dependency_count,
        dependency_blocker_count:($dependency.blockers | length)
      },
      required_before_activation:[
        "explicit_operator_approval_id",
        "single_surface_activation_scope",
        "fresh_trusted_evidence_record",
        "current_installed_binary_backup_after_approval",
        "reviewed_rollback_plan",
        "minimum_24_sample_pre_activation_soak",
        "post_activation_watchdog",
        "post_activation_minimum_24_sample_soak",
        "side_effect_receipt_with_no_secret_values"
      ],
      denied_by_receipt:[
        "missing_operator_approval_id",
        "missing_single_surface_activation_scope",
        "trusted_evidence_not_recorded",
        "receipt_not_persisted",
        "post_activation_soak_not_executed"
      ],
      side_effects:{
        memory_store_mutated:false,
        capability_registry_mutated:false,
        plugin_registry_mutated:false,
        coding_agent_spawned:false,
        skill_workshop_written:false,
        provider_invoked:false,
        model_invoked:false,
        channel_send_performed:false,
        runtime_store_mutated:false,
        gateway_event_enqueued:false,
        filesystem_written:false,
        release_artifact_written:false,
        launchd_mutated:false,
        service_restarted:false,
        rollback_executed:false,
        receipt_persisted:false,
        external_send_performed:false,
        credential_read:false
      }
    }'
)"
receipt_payload_sha256="$(printf '%s' "$receipt_payload" | shasum -a 256 | awk '{print $1}')"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_live_mutation_approval_evidence_receipt_gate" \
  --arg receipt_payload_sha256 "$receipt_payload_sha256" \
  --argjson receipt_payload "$receipt_payload" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    approval_evidence_receipt_ready:true,
    receipt_payload_sha256:$receipt_payload_sha256,
    receipt_payload:$receipt_payload,
    activation_allowed:false,
    live_mutation_execution_ready:false,
    receipt_persistence_enabled:false,
    receipt_persisted:false,
    operator_approval_recorded:false,
    side_effects:{
      memory_store_mutated:false,
      capability_registry_mutated:false,
      plugin_registry_mutated:false,
      coding_agent_spawned:false,
      skill_workshop_written:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      runtime_store_mutated:false,
      gateway_event_enqueued:false,
      filesystem_written:false,
      release_artifact_written:false,
      launchd_mutated:false,
      service_restarted:false,
      rollback_executed:false,
      receipt_persisted:false,
      external_send_performed:false,
      credential_read:false
    }
  }')"

jq -e '
  .status == "ready"
  and .approval_evidence_receipt_ready == true
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .receipt_persistence_enabled == false
  and .receipt_persisted == false
  and .operator_approval_recorded == false
  and .receipt_payload.activation_allowed_by_receipt == false
  and .receipt_payload.governance_gate.installed_sha == .receipt_payload.rollback_gate.installed_sha
  and .receipt_payload.memory_capability_evidence.absorbed_or_represented_count == 14
  and .receipt_payload.memory_capability_evidence.live_mutation_enabled_count == 0
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta live mutation approval evidence receipt gate passed"
