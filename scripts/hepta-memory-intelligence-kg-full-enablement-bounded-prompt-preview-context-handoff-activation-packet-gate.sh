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

KG_EXTERNAL_ADAPTER_STAGING_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-kg-external-adapter-staging-receipt-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-kg-external-adapter-staging-receipt-gate.sh
)"

KG_CONTEXT_HANDOFF_JSON="$(
  capture_json_report \
    "hepta-kg-prompt-preview-context-handoff-checklist-gate" \
    scripts/hepta-kg-prompt-preview-context-handoff-checklist-gate.sh
)"

activation_packet_items_json="$(
  jq -n '
    [
      {
        item:"operator_identity_and_scope_binding",
        evidence_class:"operator_authority",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_prompt_preview:true,
        blocks_context_injection:true
      },
      {
        item:"bounded_prompt_preview_scope",
        evidence_class:"prompt_preview_scope",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_prompt_preview:true,
        blocks_context_injection:true
      },
      {
        item:"context_handoff_acceptance",
        evidence_class:"context_handoff",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_prompt_preview:true,
        blocks_context_injection:true
      },
      {
        item:"redacted_diff_review_receipt",
        evidence_class:"redaction_review",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_prompt_preview:true,
        blocks_context_injection:true
      },
      {
        item:"rollback_kill_switch_receipt",
        evidence_class:"rollback_kill_switch",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_prompt_preview:true,
        blocks_context_injection:true
      },
      {
        item:"kg_external_adapter_staging_receipt",
        evidence_class:"kg_external_adapter_staging",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_prompt_preview:true,
        blocks_context_injection:true
      },
      {
        item:"post_handoff_monitoring_plan",
        evidence_class:"monitoring",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_prompt_preview:true,
        blocks_context_injection:true
      },
      {
        item:"provider_model_invocation_noop_guard",
        evidence_class:"provider_boundary",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_prompt_preview:true,
        blocks_context_injection:true
      },
      {
        item:"kg_write_noop_guard",
        evidence_class:"kg_write_boundary",
        required:true,
        shape_declared:true,
        accepted:false,
        persisted:false,
        blocks_prompt_preview:true,
        blocks_context_injection:true
      }
    ]
  '
)"

kg_external_adapter_staging_report_sha256="$(sha256_text "$KG_EXTERNAL_ADAPTER_STAGING_JSON")"
kg_context_handoff_report_sha256="$(sha256_text "$KG_CONTEXT_HANDOFF_JSON")"
activation_packet_items_sha256="$(sha256_text "$activation_packet_items_json")"
activation_packet_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-bounded-prompt-preview-context-handoff-activation-packet:$kg_external_adapter_staging_report_sha256:$kg_context_handoff_report_sha256:$activation_packet_items_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
activation_packet_policy_hash_sha256="$(
  sha256_text "bounded-prompt-preview-context-handoff:report-only:no-prompt-render:no-context-injection:no-model-invocation:no-kg-write:no-credential-read"
)"
side_effect_hash_sha256="$(
  sha256_text "prompt_preview_rendered=false;context_injection_performed=false;model_invoked=false;external_kg_adapter_read_performed=false;live_kg_write_performed=false;credential_read=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson adapter "$KG_EXTERNAL_ADAPTER_STAGING_JSON" \
  --argjson handoff "$KG_CONTEXT_HANDOFF_JSON" \
  --argjson packet_items "$activation_packet_items_json" \
  '
    $adapter.runtime == "hepta"
    and $adapter.status == "ready"
    and $adapter.gate == "hepta_memory_intelligence_kg_full_enablement_kg_external_adapter_staging_receipt_gate"
    and $adapter.kg_external_adapter_staging_lane_ready == true
    and $adapter.kg_external_adapter_staging_lane_current_live_execution_enabled == false
    and $adapter.adapter_staging_receipt_count == 3
    and $adapter.credential_reference_slot_count == 3
    and $adapter.credential_reference_recorded_count == 0
    and $adapter.credential_value_captured_count == 0
    and $adapter.credential_read_count == 0
    and $adapter.external_adapter_client_constructed_count == 0
    and $adapter.network_call_attempted_count == 0
    and $adapter.external_adapter_read_performed_count == 0
    and $adapter.live_kg_write_performed_count == 0
    and ($adapter.allowed_next_actions | any(.action == "prepare_bounded_prompt_preview_context_handoff_activation_packet" and .status == "allowed_report_only_next_slice" and .renders_prompt_preview == false and .injects_context == false and .invokes_model == false))
    and ($adapter.side_effects | to_entries | all(.value == false))
    and $handoff.runtime == "hepta"
    and $handoff.status == "ready"
    and $handoff.gate == "hepta_kg_prompt_preview_context_handoff_checklist_gate"
    and $handoff.context_handoff_checklist_ready == true
    and $handoff.context_handoff_checklist_status == "blocked"
    and $handoff.handoff_checklist_item_count == 6
    and $handoff.missing_handoff_checklist_item_count == 6
    and $handoff.redacted_refs_only == true
    and $handoff.raw_prompt_diff_count == 0
    and $handoff.prompt_text_included_count == 0
    and $handoff.payload_text_included_count == 0
    and $handoff.context_handoff_operator_approval_accepted == false
    and $handoff.context_injection_scope_record_accepted == false
    and $handoff.post_handoff_monitoring_plan_accepted == false
    and $handoff.prompt_preview_allowed == false
    and $handoff.prompt_preview_rendered == false
    and $handoff.context_injection_allowed == false
    and $handoff.context_injection_performed == false
    and $handoff.model_invocation_allowed == false
    and $handoff.model_invoked == false
    and $handoff.external_kg_adapter_read_allowed == false
    and $handoff.external_kg_adapter_read_performed == false
    and $handoff.live_kg_write_allowed == false
    and $handoff.live_kg_write_performed == false
    and ($handoff.side_effects | to_entries | all(.value == false))
    and ($packet_items | length) == 9
    and ($packet_items | all(
      .required == true
      and .shape_declared == true
      and .accepted == false
      and .persisted == false
      and .blocks_prompt_preview == true
      and .blocks_context_injection == true
    ))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_bounded_prompt_preview_context_handoff_activation_packet_gate" \
  --arg kg_external_adapter_staging_report_sha256 "$kg_external_adapter_staging_report_sha256" \
  --arg kg_context_handoff_report_sha256 "$kg_context_handoff_report_sha256" \
  --arg activation_packet_items_sha256 "$activation_packet_items_sha256" \
  --arg activation_packet_contract_hash_sha256 "$activation_packet_contract_hash_sha256" \
  --arg activation_packet_policy_hash_sha256 "$activation_packet_policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson adapter "$KG_EXTERNAL_ADAPTER_STAGING_JSON" \
  --argjson handoff "$KG_CONTEXT_HANDOFF_JSON" \
  --argjson packet_items "$activation_packet_items_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_packet_schema_version:"memory_intelligence_kg_full_enablement_bounded_prompt_preview_context_handoff_activation_packet_v1",
    activation_packet_mode:"bounded_prompt_preview_context_handoff_activation_packet_shape_no_prompt_render_no_context_injection_no_model_invocation_no_kg_write",
    source_kg_external_adapter_staging_receipt_gate:$adapter.gate,
    source_kg_context_handoff_checklist_gate:$handoff.gate,
    source_kg_external_adapter_staging_report_sha256:$kg_external_adapter_staging_report_sha256,
    source_kg_context_handoff_report_sha256:$kg_context_handoff_report_sha256,
    activation_packet_items_sha256:$activation_packet_items_sha256,
    activation_packet_contract_hash_sha256:$activation_packet_contract_hash_sha256,
    activation_packet_policy_hash_sha256:$activation_packet_policy_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    bounded_prompt_preview_context_handoff_activation_packet_ready:true,
    bounded_prompt_preview_context_handoff_activation_packet_status:"blocked",
    activation_packet_shape_ready:true,
    activation_packet_recorded:false,
    activation_packet_persisted:false,
    activation_packet_accepted:false,
    activation_packet_delivery_allowed:false,
    activation_packet_delivered:false,
    kg_external_adapter_staging_lane_ready:$adapter.kg_external_adapter_staging_lane_ready,
    kg_external_adapter_staging_lane_current_live_execution_enabled:$adapter.kg_external_adapter_staging_lane_current_live_execution_enabled,
    kg_adapter_staging_receipt_count:$adapter.adapter_staging_receipt_count,
    kg_adapter_credential_reference_slot_count:$adapter.credential_reference_slot_count,
    kg_adapter_credential_value_captured_count:$adapter.credential_value_captured_count,
    kg_adapter_credential_read_count:$adapter.credential_read_count,
    kg_adapter_client_constructed_count:$adapter.external_adapter_client_constructed_count,
    kg_adapter_network_call_attempted_count:$adapter.network_call_attempted_count,
    kg_adapter_live_write_performed_count:$adapter.live_kg_write_performed_count,
    context_handoff_checklist_ready:$handoff.context_handoff_checklist_ready,
    context_handoff_checklist_status:$handoff.context_handoff_checklist_status,
    context_handoff_checklist_item_count:$handoff.handoff_checklist_item_count,
    context_handoff_missing_checklist_item_count:$handoff.missing_handoff_checklist_item_count,
    context_handoff_redacted_refs_only:$handoff.redacted_refs_only,
    raw_prompt_diff_count:$handoff.raw_prompt_diff_count,
    prompt_text_included_count:$handoff.prompt_text_included_count,
    payload_text_included_count:$handoff.payload_text_included_count,
    activation_packet_item_count:($packet_items | length),
    required_activation_packet_item_count:($packet_items | map(select(.required == true)) | length),
    declared_activation_packet_item_count:($packet_items | map(select(.shape_declared == true)) | length),
    accepted_activation_packet_item_count:($packet_items | map(select(.accepted == true)) | length),
    persisted_activation_packet_item_count:($packet_items | map(select(.persisted == true)) | length),
    missing_activation_packet_item_count:($packet_items | map(select(.accepted == false)) | length),
    prompt_preview_blocking_activation_packet_item_count:($packet_items | map(select(.blocks_prompt_preview == true)) | length),
    context_injection_blocking_activation_packet_item_count:($packet_items | map(select(.blocks_context_injection == true)) | length),
    activation_packet_items:$packet_items,
    denied_by_activation_packet:[
      "operator_identity_and_scope_not_accepted",
      "bounded_prompt_preview_scope_not_accepted",
      "context_handoff_not_accepted",
      "redacted_diff_review_receipt_not_accepted",
      "rollback_kill_switch_receipt_not_accepted",
      "kg_external_adapter_staging_receipt_not_accepted",
      "post_handoff_monitoring_plan_not_accepted",
      "provider_model_invocation_noop_guard_not_accepted",
      "kg_write_noop_guard_not_accepted",
      "prompt_preview_rendering_denied",
      "context_injection_denied",
      "model_invocation_denied",
      "external_kg_adapter_read_denied",
      "live_kg_write_denied",
      "credential_read_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_bounded_prompt_preview_context_handoff_activation_packet_shape",
        status:"allowed_report_only",
        renders_prompt_preview:false,
        injects_context:false,
        invokes_model:false,
        writes_kg:false
      },
      {
        action:"stage_runtime_provider_router_context_attachment_packet",
        status:"allowed_report_only_next_slice",
        attaches_live_context:false,
        mutates_runtime:false,
        invokes_model:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        mutates_runtime:false,
        renders_prompt_preview:false,
        writes_kg:false
      }
    ],
    operator_approval_required_before_prompt_preview:true,
    operator_activation_receipt_required:true,
    bounded_prompt_preview_scope_required:true,
    context_handoff_acceptance_required:true,
    context_injection_scope_record_required:true,
    redacted_diff_review_receipt_required:true,
    rollback_kill_switch_required:true,
    kg_external_adapter_staging_receipt_required:true,
    post_handoff_monitoring_required:true,
    provider_model_invocation_forbidden:true,
    live_kg_write_forbidden:true,
    credential_value_capture_forbidden:true,
    credential_read_forbidden:true,
    full_live_enablement_performed:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    hepta_intelligence_context_attached:false,
    bounded_prompt_preview_scope_accepted:false,
    prompt_preview_allowed:false,
    prompt_preview_rendered:false,
    prompt_payload_materialized:false,
    context_handoff_accepted:false,
    context_injection_scope_record_accepted:false,
    context_injection_allowed:false,
    context_injection_performed:false,
    provider_invoked:false,
    model_invoked:false,
    credential_reference_recorded:false,
    credential_value_captured:false,
    credential_read:false,
    secret_file_read:false,
    external_adapter_client_constructed:false,
    external_kg_adapter_read_performed:false,
    network_call_performed:false,
    external_db_write_performed:false,
    live_kg_write_performed:false,
    rollback_executed:false,
    external_send_performed:false,
    channel_send_performed:false,
    public_release_claimed:false,
    public_ga_claimed:false,
    service_restart_performed:false,
    active_binary_mutated:false,
    side_effects:{
      full_live_enablement_performed:false,
      activation_packet_recorded:false,
      activation_packet_persisted:false,
      activation_packet_delivered:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      hepta_intelligence_context_attached:false,
      bounded_prompt_preview_scope_accepted:false,
      prompt_preview_rendered:false,
      prompt_payload_materialized:false,
      context_handoff_accepted:false,
      context_injection_scope_record_accepted:false,
      context_injection_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_reference_recorded:false,
      credential_value_captured:false,
      credential_read:false,
      secret_file_read:false,
      external_adapter_client_constructed:false,
      external_kg_adapter_read_performed:false,
      network_call_performed:false,
      external_db_write_performed:false,
      live_kg_write_performed:false,
      rollback_executed:false,
      filesystem_written:false,
      external_send_performed:false,
      channel_send_performed:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_bounded_prompt_preview_context_handoff_activation_packet_gate"
  and .activation_packet_schema_version == "memory_intelligence_kg_full_enablement_bounded_prompt_preview_context_handoff_activation_packet_v1"
  and .bounded_prompt_preview_context_handoff_activation_packet_ready == true
  and .bounded_prompt_preview_context_handoff_activation_packet_status == "blocked"
  and .activation_packet_shape_ready == true
  and .activation_packet_recorded == false
  and .activation_packet_persisted == false
  and .activation_packet_accepted == false
  and .activation_packet_delivered == false
  and .kg_external_adapter_staging_lane_ready == true
  and .kg_external_adapter_staging_lane_current_live_execution_enabled == false
  and .kg_adapter_staging_receipt_count == 3
  and .kg_adapter_credential_reference_slot_count == 3
  and .kg_adapter_credential_value_captured_count == 0
  and .kg_adapter_credential_read_count == 0
  and .kg_adapter_client_constructed_count == 0
  and .kg_adapter_network_call_attempted_count == 0
  and .kg_adapter_live_write_performed_count == 0
  and .context_handoff_checklist_ready == true
  and .context_handoff_checklist_status == "blocked"
  and .context_handoff_checklist_item_count == 6
  and .context_handoff_missing_checklist_item_count == 6
  and .context_handoff_redacted_refs_only == true
  and .raw_prompt_diff_count == 0
  and .prompt_text_included_count == 0
  and .payload_text_included_count == 0
  and .activation_packet_item_count == 9
  and .required_activation_packet_item_count == 9
  and .declared_activation_packet_item_count == 9
  and .accepted_activation_packet_item_count == 0
  and .persisted_activation_packet_item_count == 0
  and .missing_activation_packet_item_count == 9
  and .prompt_preview_blocking_activation_packet_item_count == 9
  and .context_injection_blocking_activation_packet_item_count == 9
  and (.activation_packet_items | length) == 9
  and (.activation_packet_items | all(.required == true and .shape_declared == true and .accepted == false and .persisted == false and .blocks_prompt_preview == true and .blocks_context_injection == true))
  and (.denied_by_activation_packet | length) == 15
  and (.allowed_next_actions | any(.action == "review_bounded_prompt_preview_context_handoff_activation_packet_shape" and .status == "allowed_report_only" and .renders_prompt_preview == false and .injects_context == false and .invokes_model == false and .writes_kg == false))
  and (.allowed_next_actions | any(.action == "stage_runtime_provider_router_context_attachment_packet" and .status == "allowed_report_only_next_slice" and .attaches_live_context == false and .mutates_runtime == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "run_full_light_preflight" and .status == "allowed_verification_only" and .mutates_runtime == false and .renders_prompt_preview == false and .writes_kg == false))
  and .operator_approval_required_before_prompt_preview == true
  and .operator_activation_receipt_required == true
  and .bounded_prompt_preview_scope_required == true
  and .context_handoff_acceptance_required == true
  and .provider_model_invocation_forbidden == true
  and .live_kg_write_forbidden == true
  and .credential_read_forbidden == true
  and .full_live_enablement_performed == false
  and .memory_store_mutated == false
  and .hepta_intelligence_context_attached == false
  and .bounded_prompt_preview_scope_accepted == false
  and .prompt_preview_allowed == false
  and .prompt_preview_rendered == false
  and .prompt_payload_materialized == false
  and .context_handoff_accepted == false
  and .context_injection_scope_record_accepted == false
  and .context_injection_allowed == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_value_captured == false
  and .credential_read == false
  and .secret_file_read == false
  and .external_adapter_client_constructed == false
  and .external_kg_adapter_read_performed == false
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .live_kg_write_performed == false
  and .rollback_executed == false
  and .external_send_performed == false
  and .channel_send_performed == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .service_restart_performed == false
  and .active_binary_mutated == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full enablement bounded prompt-preview context-handoff activation packet gate passed"
