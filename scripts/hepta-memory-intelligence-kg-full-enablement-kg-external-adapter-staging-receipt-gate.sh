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

MEMORY_STAGING_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-memory-live-mutation-staging-fixture-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-memory-live-mutation-staging-fixture-gate.sh
)"

ROLLBACK_KILL_SWITCH_JSON="$(
  capture_json_report \
    "hepta-kg-prompt-preview-rollback-kill-switch-evidence-checklist-gate" \
    scripts/hepta-kg-prompt-preview-rollback-kill-switch-evidence-checklist-gate.sh
)"

adapter_manifest="$(
  printf '%s\n' \
    "graphiti|HEPTA_KG_GRAPHITI_STAGING|HEPTA_KG_GRAPHITI_ENDPOINT|HEPTA_KG_GRAPHITI_CREDENTIAL_REF|HEPTA_KG_GRAPHITI_ROLLBACK_PLAN_READY|HEPTA_KG_GRAPHITI_POST_WRITE_VALIDATION_READY|episode-entity-relation-temporal-memory" \
    "neo4j|HEPTA_KG_NEO4J_STAGING|HEPTA_KG_NEO4J_ENDPOINT|HEPTA_KG_NEO4J_CREDENTIAL_REF|HEPTA_KG_NEO4J_ROLLBACK_PLAN_READY|HEPTA_KG_NEO4J_POST_WRITE_VALIDATION_READY|node-relationship-property-graph" \
    "cocoindex|HEPTA_KG_COCOINDEX_STAGING|HEPTA_KG_COCOINDEX_ENDPOINT|HEPTA_KG_COCOINDEX_CREDENTIAL_REF|HEPTA_KG_COCOINDEX_ROLLBACK_PLAN_READY|HEPTA_KG_COCOINDEX_POST_WRITE_VALIDATION_READY|document-chunk-entity-index"
)"

adapter_staging_receipts_json="$(
  printf '%s\n' "$adapter_manifest" |
    jq -R -s '
      split("\n")
      | map(select(length > 0))
      | map(split("|"))
      | map({
          adapter_id: .[0],
          dry_run_contract: "hepta-kg-external-adapter-dry-run-v0",
          staging_gate_contract: "hepta-kg-external-adapter-staging-gate-v0",
          config_env_contract: "hepta-kg-external-adapter-config-env-v0",
          client_contract: "hepta-kg-external-adapter-client-v0",
          feature_gate_key: .[1],
          endpoint_key: .[2],
          credential_ref_key: .[3],
          rollback_plan_ready_key: .[4],
          post_write_validation_ready_key: .[5],
          projection_family: .[6],
          credential_reference_slot_declared: true,
          credential_reference_recorded: false,
          credential_reference_persisted: false,
          credential_reference_value_captured: false,
          credential_value_captured: false,
          credential_read: false,
          secret_file_read: false,
          endpoint_value_captured: false,
          feature_gate_enabled: false,
          network_allowlisted: false,
          external_write_allowlisted: false,
          operator_review_accepted: false,
          dry_run_sample_receipt_accepted: false,
          rollback_plan_receipt_declared: true,
          rollback_plan_receipt_accepted: false,
          rollback_dry_run_receipt_accepted: false,
          kill_switch_receipt_accepted: false,
          kill_switch_dry_run_receipt_accepted: false,
          post_write_validation_receipt_declared: true,
          post_write_validation_receipt_accepted: false,
          live_write_requested: false,
          staging_ready: false,
          network_call_allowed: false,
          network_call_attempted: false,
          external_write_allowed: false,
          external_write_attempted: false,
          live_write_allowed: false,
          live_write_attempted: false,
          external_adapter_client_constructed: false,
          external_adapter_read_performed: false,
          external_db_write_performed: false,
          live_kg_write_performed: false,
          persisted_records: 0,
          missing_receipt_items: [
            "credential_ref_not_recorded",
            "operator_review_not_accepted",
            "dry_run_sample_not_accepted",
            "rollback_plan_receipt_not_accepted",
            "rollback_dry_run_receipt_not_accepted",
            "kill_switch_receipt_not_accepted",
            "kill_switch_dry_run_receipt_not_accepted",
            "post_write_validation_receipt_not_accepted",
            "live_write_forbidden"
          ]
        })
    '
)"

memory_staging_report_sha256="$(sha256_text "$MEMORY_STAGING_JSON")"
rollback_kill_switch_report_sha256="$(sha256_text "$ROLLBACK_KILL_SWITCH_JSON")"
adapter_manifest_sha256="$(sha256_text "$adapter_manifest")"
staging_receipt_contract_hash_sha256="$(
  sha256_text "hepta-full-enablement-kg-external-adapter-staging-receipt:$memory_staging_report_sha256:$rollback_kill_switch_report_sha256:$adapter_manifest_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
side_effect_hash_sha256="$(
  sha256_text "credential_read=false;external_adapter_client_constructed=false;network_call_performed=false;external_db_write_performed=false;live_kg_write_performed=false;rollback_executed=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson memory "$MEMORY_STAGING_JSON" \
  --argjson rollback "$ROLLBACK_KILL_SWITCH_JSON" \
  --argjson receipts "$adapter_staging_receipts_json" \
  '
    $memory.runtime == "hepta"
    and $memory.status == "ready"
    and $memory.gate == "hepta_memory_intelligence_kg_full_enablement_memory_live_mutation_staging_fixture_gate"
    and $memory.full_enablement_memory_live_mutation_staging_fixture_ready == true
    and $memory.enablement_lane_count == 6
    and $memory.ready_enablement_lane_count == 6
    and $memory.current_live_enabled_lane_count == 0
    and $memory.memory_store_live_mutation_lane_ready == true
    and $memory.memory_store_live_mutation_lane_current_live_execution_enabled == false
    and $memory.external_kg_adapter_read_performed == false
    and $memory.live_kg_write_performed == false
    and $memory.credential_read == false
    and ($memory.allowed_next_actions | any(.action == "stage_kg_external_adapter_credentials_and_rollback_receipts" and .status == "allowed_report_only_next_slice" and .reads_credentials == false and .invokes_external_adapter == false and .writes_kg == false))
    and ($memory.side_effects | to_entries | all(.value == false))
    and $rollback.runtime == "hepta"
    and $rollback.status == "ready"
    and $rollback.gate == "hepta_kg_prompt_preview_rollback_kill_switch_evidence_checklist_gate"
    and $rollback.rollback_kill_switch_checklist_ready == true
    and $rollback.rollback_kill_switch_checklist_status == "blocked"
    and $rollback.safety_checklist_item_count == 4
    and $rollback.missing_safety_checklist_item_count == 4
    and $rollback.rollback_plan_present == false
    and $rollback.rollback_dry_run_evidence_present == false
    and $rollback.kill_switch_present == false
    and $rollback.kill_switch_dry_run_evidence_present == false
    and $rollback.external_kg_adapter_read_allowed == false
    and $rollback.external_kg_adapter_read_performed == false
    and $rollback.network_call_allowed == false
    and $rollback.network_call_performed == false
    and $rollback.live_kg_write_allowed == false
    and $rollback.live_kg_write_performed == false
    and ($rollback.side_effects | to_entries | all(.value == false))
    and ($receipts | length) == 3
    and ($receipts | all(
      .credential_reference_slot_declared == true
      and .credential_reference_recorded == false
      and .credential_reference_persisted == false
      and .credential_reference_value_captured == false
      and .credential_value_captured == false
      and .credential_read == false
      and .secret_file_read == false
      and .endpoint_value_captured == false
      and .feature_gate_enabled == false
      and .network_allowlisted == false
      and .external_write_allowlisted == false
      and .operator_review_accepted == false
      and .dry_run_sample_receipt_accepted == false
      and .rollback_plan_receipt_declared == true
      and .rollback_plan_receipt_accepted == false
      and .rollback_dry_run_receipt_accepted == false
      and .kill_switch_receipt_accepted == false
      and .kill_switch_dry_run_receipt_accepted == false
      and .post_write_validation_receipt_declared == true
      and .post_write_validation_receipt_accepted == false
      and .live_write_requested == false
      and .staging_ready == false
      and .network_call_allowed == false
      and .network_call_attempted == false
      and .external_write_allowed == false
      and .external_write_attempted == false
      and .live_write_allowed == false
      and .live_write_attempted == false
      and .external_adapter_client_constructed == false
      and .external_adapter_read_performed == false
      and .external_db_write_performed == false
      and .live_kg_write_performed == false
      and .persisted_records == 0
      and (.missing_receipt_items | length) == 9
    ))
    and ($receipts | any(.adapter_id == "graphiti" and .feature_gate_key == "HEPTA_KG_GRAPHITI_STAGING" and .credential_ref_key == "HEPTA_KG_GRAPHITI_CREDENTIAL_REF"))
    and ($receipts | any(.adapter_id == "neo4j" and .feature_gate_key == "HEPTA_KG_NEO4J_STAGING" and .credential_ref_key == "HEPTA_KG_NEO4J_CREDENTIAL_REF"))
    and ($receipts | any(.adapter_id == "cocoindex" and .feature_gate_key == "HEPTA_KG_COCOINDEX_STAGING" and .credential_ref_key == "HEPTA_KG_COCOINDEX_CREDENTIAL_REF"))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_kg_external_adapter_staging_receipt_gate" \
  --arg memory_staging_report_sha256 "$memory_staging_report_sha256" \
  --arg rollback_kill_switch_report_sha256 "$rollback_kill_switch_report_sha256" \
  --arg adapter_manifest_sha256 "$adapter_manifest_sha256" \
  --arg staging_receipt_contract_hash_sha256 "$staging_receipt_contract_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson memory "$MEMORY_STAGING_JSON" \
  --argjson rollback "$ROLLBACK_KILL_SWITCH_JSON" \
  --argjson receipts "$adapter_staging_receipts_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    staging_receipt_schema_version:"memory_intelligence_kg_full_enablement_kg_external_adapter_staging_receipt_v1",
    staging_receipt_mode:"kg_external_adapter_credential_and_rollback_receipt_shape_no_credential_read_no_adapter_invocation_no_kg_write",
    source_memory_full_enablement_staging_gate:$memory.gate,
    source_kg_rollback_kill_switch_gate:$rollback.gate,
    source_memory_staging_report_sha256:$memory_staging_report_sha256,
    source_kg_rollback_kill_switch_report_sha256:$rollback_kill_switch_report_sha256,
    adapter_manifest_sha256:$adapter_manifest_sha256,
    staging_receipt_contract_hash_sha256:$staging_receipt_contract_hash_sha256,
    side_effect_hash_sha256:$side_effect_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    full_enablement_memory_live_mutation_staging_fixture_ready:$memory.full_enablement_memory_live_mutation_staging_fixture_ready,
    full_enablement_activation_readiness_status:$memory.full_enablement_activation_readiness_status,
    enablement_lane_count:$memory.enablement_lane_count,
    ready_enablement_lane_count:$memory.ready_enablement_lane_count,
    current_live_enabled_lane_count:$memory.current_live_enabled_lane_count,
    kg_external_adapter_staging_lane_ready:true,
    kg_external_adapter_staging_lane_current_live_execution_enabled:false,
    credential_receipt_shape_ready:true,
    rollback_receipt_shape_ready:true,
    rollback_kill_switch_checklist_ready:$rollback.rollback_kill_switch_checklist_ready,
    rollback_kill_switch_checklist_status:$rollback.rollback_kill_switch_checklist_status,
    source_safety_checklist_item_count:$rollback.safety_checklist_item_count,
    source_missing_safety_checklist_item_count:$rollback.missing_safety_checklist_item_count,
    adapter_staging_receipt_count:($receipts | length),
    required_adapter_staging_receipt_count:3,
    supported_adapter_count:3,
    supported_adapters:($receipts | map(.adapter_id)),
    credential_reference_slot_count:($receipts | map(select(.credential_reference_slot_declared == true)) | length),
    credential_reference_recorded_count:($receipts | map(select(.credential_reference_recorded == true)) | length),
    credential_reference_persisted_count:($receipts | map(select(.credential_reference_persisted == true)) | length),
    credential_reference_value_captured_count:($receipts | map(select(.credential_reference_value_captured == true)) | length),
    credential_value_captured_count:($receipts | map(select(.credential_value_captured == true)) | length),
    credential_read_count:($receipts | map(select(.credential_read == true)) | length),
    secret_file_read_count:($receipts | map(select(.secret_file_read == true)) | length),
    endpoint_value_captured_count:($receipts | map(select(.endpoint_value_captured == true)) | length),
    feature_gate_enabled_count:($receipts | map(select(.feature_gate_enabled == true)) | length),
    operator_review_accepted_count:($receipts | map(select(.operator_review_accepted == true)) | length),
    dry_run_sample_receipt_accepted_count:($receipts | map(select(.dry_run_sample_receipt_accepted == true)) | length),
    rollback_plan_receipt_declared_count:($receipts | map(select(.rollback_plan_receipt_declared == true)) | length),
    rollback_plan_receipt_accepted_count:($receipts | map(select(.rollback_plan_receipt_accepted == true)) | length),
    rollback_dry_run_receipt_accepted_count:($receipts | map(select(.rollback_dry_run_receipt_accepted == true)) | length),
    kill_switch_receipt_accepted_count:($receipts | map(select(.kill_switch_receipt_accepted == true)) | length),
    kill_switch_dry_run_receipt_accepted_count:($receipts | map(select(.kill_switch_dry_run_receipt_accepted == true)) | length),
    post_write_validation_receipt_declared_count:($receipts | map(select(.post_write_validation_receipt_declared == true)) | length),
    post_write_validation_receipt_accepted_count:($receipts | map(select(.post_write_validation_receipt_accepted == true)) | length),
    staging_ready_count:($receipts | map(select(.staging_ready == true)) | length),
    network_call_allowed_count:($receipts | map(select(.network_call_allowed == true)) | length),
    network_call_attempted_count:($receipts | map(select(.network_call_attempted == true)) | length),
    external_write_allowed_count:($receipts | map(select(.external_write_allowed == true)) | length),
    external_write_attempted_count:($receipts | map(select(.external_write_attempted == true)) | length),
    live_write_allowed_count:($receipts | map(select(.live_write_allowed == true)) | length),
    live_write_attempted_count:($receipts | map(select(.live_write_attempted == true)) | length),
    external_adapter_client_constructed_count:($receipts | map(select(.external_adapter_client_constructed == true)) | length),
    external_adapter_read_performed_count:($receipts | map(select(.external_adapter_read_performed == true)) | length),
    external_db_write_performed_count:($receipts | map(select(.external_db_write_performed == true)) | length),
    live_kg_write_performed_count:($receipts | map(select(.live_kg_write_performed == true)) | length),
    persisted_record_count:($receipts | map(.persisted_records) | add),
    missing_receipt_item_count:($receipts | map(.missing_receipt_items | length) | add),
    adapter_staging_receipts:$receipts,
    denied_by_external_adapter_staging_receipt:[
      "credential_reference_not_recorded",
      "credential_value_capture_denied",
      "operator_review_not_accepted",
      "dry_run_sample_receipt_not_accepted",
      "rollback_plan_receipt_not_accepted",
      "rollback_dry_run_receipt_not_accepted",
      "kill_switch_receipt_not_accepted",
      "kill_switch_dry_run_receipt_not_accepted",
      "post_write_validation_receipt_not_accepted",
      "external_adapter_client_construction_denied",
      "network_call_denied",
      "external_adapter_read_denied",
      "external_db_write_denied",
      "live_kg_write_denied"
    ],
    allowed_next_actions:[
      {
        action:"review_kg_external_adapter_staging_receipt_shape",
        status:"allowed_report_only",
        reads_credentials:false,
        invokes_external_adapter:false,
        writes_kg:false
      },
      {
        action:"prepare_bounded_prompt_preview_context_handoff_activation_packet",
        status:"allowed_report_only_next_slice",
        renders_prompt_preview:false,
        injects_context:false,
        invokes_model:false
      },
      {
        action:"run_full_light_preflight",
        status:"allowed_verification_only",
        mutates_runtime:false,
        reads_credentials:false,
        writes_kg:false
      }
    ],
    operator_approval_required_before_adapter_live:true,
    operator_activation_receipt_required:true,
    bounded_prompt_preview_scope_required:true,
    context_handoff_acceptance_required:true,
    credential_reference_acceptance_required:true,
    credential_value_capture_forbidden:true,
    external_adapter_client_construction_forbidden:true,
    network_allowlist_required_before_adapter_live:true,
    rollback_kill_switch_required:true,
    rollback_receipt_acceptance_required:true,
    post_write_validation_required:true,
    live_kg_write_forbidden:true,
    full_live_enablement_performed:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    hepta_intelligence_context_attached:false,
    prompt_preview_rendered:false,
    prompt_payload_materialized:false,
    context_injection_performed:false,
    provider_invoked:false,
    model_invoked:false,
    credential_reference_recorded:false,
    credential_reference_persisted:false,
    credential_value_captured:false,
    credential_read:false,
    secret_file_read:false,
    endpoint_value_captured:false,
    graphiti_client_constructed:false,
    neo4j_client_constructed:false,
    cocoindex_client_constructed:false,
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
      memory_store_write_performed:false,
      memory_store_mutated:false,
      hepta_intelligence_context_attached:false,
      prompt_preview_rendered:false,
      prompt_payload_materialized:false,
      context_injection_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_reference_recorded:false,
      credential_reference_persisted:false,
      credential_value_captured:false,
      credential_read:false,
      secret_file_read:false,
      endpoint_value_captured:false,
      graphiti_client_constructed:false,
      neo4j_client_constructed:false,
      cocoindex_client_constructed:false,
      external_adapter_client_constructed:false,
      external_kg_adapter_read_performed:false,
      network_call_performed:false,
      external_db_write_performed:false,
      live_kg_write_performed:false,
      rollback_executed:false,
      rollback_receipt_recorded:false,
      rollback_receipt_persisted:false,
      post_write_validation_recorded:false,
      post_write_validation_persisted:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_enablement_kg_external_adapter_staging_receipt_gate"
  and .staging_receipt_schema_version == "memory_intelligence_kg_full_enablement_kg_external_adapter_staging_receipt_v1"
  and .kg_external_adapter_staging_lane_ready == true
  and .kg_external_adapter_staging_lane_current_live_execution_enabled == false
  and .credential_receipt_shape_ready == true
  and .rollback_receipt_shape_ready == true
  and .rollback_kill_switch_checklist_ready == true
  and .rollback_kill_switch_checklist_status == "blocked"
  and .adapter_staging_receipt_count == 3
  and .required_adapter_staging_receipt_count == 3
  and .supported_adapter_count == 3
  and (.supported_adapters | index("graphiti") != null)
  and (.supported_adapters | index("neo4j") != null)
  and (.supported_adapters | index("cocoindex") != null)
  and .credential_reference_slot_count == 3
  and .credential_reference_recorded_count == 0
  and .credential_reference_persisted_count == 0
  and .credential_reference_value_captured_count == 0
  and .credential_value_captured_count == 0
  and .credential_read_count == 0
  and .secret_file_read_count == 0
  and .endpoint_value_captured_count == 0
  and .feature_gate_enabled_count == 0
  and .operator_review_accepted_count == 0
  and .dry_run_sample_receipt_accepted_count == 0
  and .rollback_plan_receipt_declared_count == 3
  and .rollback_plan_receipt_accepted_count == 0
  and .rollback_dry_run_receipt_accepted_count == 0
  and .kill_switch_receipt_accepted_count == 0
  and .kill_switch_dry_run_receipt_accepted_count == 0
  and .post_write_validation_receipt_declared_count == 3
  and .post_write_validation_receipt_accepted_count == 0
  and .staging_ready_count == 0
  and .network_call_allowed_count == 0
  and .network_call_attempted_count == 0
  and .external_write_allowed_count == 0
  and .external_write_attempted_count == 0
  and .live_write_allowed_count == 0
  and .live_write_attempted_count == 0
  and .external_adapter_client_constructed_count == 0
  and .external_adapter_read_performed_count == 0
  and .external_db_write_performed_count == 0
  and .live_kg_write_performed_count == 0
  and .persisted_record_count == 0
  and .missing_receipt_item_count == 27
  and (.adapter_staging_receipts | length) == 3
  and (.adapter_staging_receipts | all(.credential_reference_slot_declared == true and .credential_value_captured == false and .credential_read == false and .secret_file_read == false and .rollback_plan_receipt_declared == true and .post_write_validation_receipt_declared == true and .staging_ready == false and .network_call_attempted == false and .external_adapter_client_constructed == false and .live_kg_write_performed == false and .persisted_records == 0))
  and (.denied_by_external_adapter_staging_receipt | length) == 14
  and (.allowed_next_actions | any(.action == "review_kg_external_adapter_staging_receipt_shape" and .status == "allowed_report_only" and .reads_credentials == false and .invokes_external_adapter == false and .writes_kg == false))
  and (.allowed_next_actions | any(.action == "prepare_bounded_prompt_preview_context_handoff_activation_packet" and .status == "allowed_report_only_next_slice" and .renders_prompt_preview == false and .injects_context == false and .invokes_model == false))
  and (.allowed_next_actions | any(.action == "run_full_light_preflight" and .status == "allowed_verification_only" and .mutates_runtime == false and .reads_credentials == false and .writes_kg == false))
  and .operator_approval_required_before_adapter_live == true
  and .credential_reference_acceptance_required == true
  and .credential_value_capture_forbidden == true
  and .external_adapter_client_construction_forbidden == true
  and .rollback_kill_switch_required == true
  and .rollback_receipt_acceptance_required == true
  and .live_kg_write_forbidden == true
  and .full_live_enablement_performed == false
  and .memory_store_mutated == false
  and .hepta_intelligence_context_attached == false
  and .prompt_preview_rendered == false
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
echo "Hepta memory/intelligence/KG full enablement KG external adapter staging receipt gate passed"
