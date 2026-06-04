#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

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

READINESS_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-activation-readiness-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-activation-readiness-gate.sh
)"

BOUNDED_PACKET_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-bounded-prompt-preview-context-handoff-activation-packet-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-bounded-prompt-preview-context-handoff-activation-packet-gate.sh
)"

RUNTIME_ATTACHMENT_JSON="$(
  capture_json_report \
    "hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-context-attachment-staging-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-intelligence-kg-full-enablement-runtime-provider-router-context-attachment-staging-gate.sh
)"

KG_OPERATOR_CHECKLIST_JSON="$(
  capture_json_report \
    "hepta-kg-prompt-preview-operator-approval-checklist-schema-gate" \
    scripts/hepta-kg-prompt-preview-operator-approval-checklist-schema-gate.sh
)"

MEMORY_WRITE_PACKET_JSON="$(
  capture_json_report \
    "hepta-memory-live-mutation-operator-write-approval-packet-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-memory-live-mutation-operator-write-approval-packet-gate.sh
)"

readiness_report_sha256="$(sha256_text "$READINESS_JSON")"
bounded_packet_report_sha256="$(sha256_text "$BOUNDED_PACKET_JSON")"
runtime_attachment_report_sha256="$(sha256_text "$RUNTIME_ATTACHMENT_JSON")"
kg_operator_checklist_report_sha256="$(sha256_text "$KG_OPERATOR_CHECKLIST_JSON")"
memory_write_packet_report_sha256="$(sha256_text "$MEMORY_WRITE_PACKET_JSON")"

source_bundle_sha256="$(
  sha256_text "memory-intelligence-kg-positive-activation-packet-dry-run:sources:$readiness_report_sha256:$bounded_packet_report_sha256:$runtime_attachment_report_sha256:$kg_operator_checklist_report_sha256:$memory_write_packet_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
scaffold_hash_sha256="$(
  sha256_text "memory-intelligence-kg-positive-activation-packet-dry-run:scaffold:v1:$source_bundle_sha256"
)"
policy_hash_sha256="$(
  sha256_text "memory-intelligence-kg-positive-activation-packet-dry-run:policy:report-only:no-approval:no-persistence:no-context-injection:no-provider:no-memory-write:no-kg-read:no-kg-write:no-secret"
)"
side_effect_hash_sha256="$(
  sha256_text "operator_approval_accepted=false;activation_packet_accepted=false;memory_write=false;context_injection=false;provider_invoked=false;model_invoked=false;external_kg_adapter_read=false;live_kg_write=false;credential_read=false;secret_read=false"
)"

jq -n -e \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson readiness "$READINESS_JSON" \
  --argjson bounded "$BOUNDED_PACKET_JSON" \
  --argjson runtime_attachment "$RUNTIME_ATTACHMENT_JSON" \
  --argjson kg_checklist "$KG_OPERATOR_CHECKLIST_JSON" \
  --argjson memory_write "$MEMORY_WRITE_PACKET_JSON" \
  '
    $readiness.runtime == "hepta"
    and $readiness.status == "ready"
    and $readiness.gate == "hepta_memory_intelligence_kg_full_enablement_activation_readiness_gate"
    and $readiness.full_enablement_activation_readiness_ready == true
    and $readiness.live_activation_status == "not_performed_by_this_gate"
    and $readiness.memory_surface_count == 14
    and $readiness.absorbed_or_represented_count == 14
    and $readiness.live_mutation_enabled_count == 0
    and $readiness.enablement_lane_count == 6
    and $readiness.ready_enablement_lane_count == 6
    and $readiness.current_live_enabled_lane_count == 0
    and $readiness.kg_source_gate_count == 5
    and $readiness.kg_ready_source_gate_count == 5
    and $readiness.kg_blocked_source_gate_count == 5
    and $readiness.hepta_intelligence_context_attached == false
    and $readiness.context_injection_performed == false
    and $readiness.provider_invoked == false
    and $readiness.model_invoked == false
    and $readiness.external_kg_adapter_read_performed == false
    and $readiness.live_kg_write_performed == false
    and $readiness.credential_read == false
    and ($readiness.side_effects | to_entries | all(.value == false))

    and $bounded.runtime == "hepta"
    and $bounded.status == "ready"
    and $bounded.gate == "hepta_memory_intelligence_kg_full_enablement_bounded_prompt_preview_context_handoff_activation_packet_gate"
    and $bounded.bounded_prompt_preview_context_handoff_activation_packet_ready == true
    and $bounded.bounded_prompt_preview_context_handoff_activation_packet_status == "blocked"
    and $bounded.activation_packet_shape_ready == true
    and $bounded.activation_packet_recorded == false
    and $bounded.activation_packet_persisted == false
    and $bounded.activation_packet_accepted == false
    and $bounded.activation_packet_delivered == false
    and $bounded.activation_packet_item_count == 9
    and $bounded.accepted_activation_packet_item_count == 0
    and $bounded.prompt_preview_rendered == false
    and $bounded.prompt_payload_materialized == false
    and $bounded.context_injection_performed == false
    and $bounded.provider_invoked == false
    and $bounded.model_invoked == false
    and $bounded.external_kg_adapter_read_performed == false
    and $bounded.live_kg_write_performed == false
    and $bounded.credential_read == false
    and $bounded.network_call_performed == false
    and $bounded.raw_prompt_diff_count == 0
    and $bounded.prompt_text_included_count == 0
    and $bounded.payload_text_included_count == 0
    and ($bounded.side_effects | to_entries | all(.value == false))

    and $runtime_attachment.runtime == "hepta"
    and $runtime_attachment.status == "ready"
    and $runtime_attachment.gate == "hepta_memory_intelligence_kg_full_enablement_runtime_provider_router_context_attachment_staging_gate"
    and $runtime_attachment.runtime_provider_router_context_attachment_staging_ready == true
    and $runtime_attachment.runtime_provider_router_context_attachment_staging_status == "blocked"
    and $runtime_attachment.runtime_attachment_packet_shape_ready == true
    and $runtime_attachment.runtime_attachment_packet_recorded == false
    and $runtime_attachment.runtime_attachment_packet_persisted == false
    and $runtime_attachment.runtime_attachment_packet_accepted == false
    and $runtime_attachment.runtime_attachment_packet_delivered == false
    and $runtime_attachment.runtime_attachment_packet_item_count == 12
    and $runtime_attachment.accepted_runtime_attachment_packet_item_count == 0
    and $runtime_attachment.router_handoff_recorded == false
    and $runtime_attachment.runtime_router_mutated == false
    and $runtime_attachment.hepta_intelligence_context_attached == false
    and $runtime_attachment.live_context_attached_to_prompt == false
    and $runtime_attachment.context_injection_performed == false
    and $runtime_attachment.provider_invoked == false
    and $runtime_attachment.model_invoked == false
    and $runtime_attachment.auth_secret_read == false
    and $runtime_attachment.credential_read == false
    and $runtime_attachment.usage_recorded == false
    and ($runtime_attachment.side_effects | to_entries | all(.value == false))

    and $kg_checklist.runtime == "hepta"
    and $kg_checklist.status == "ready"
    and $kg_checklist.gate == "hepta_kg_prompt_preview_operator_approval_checklist_schema_gate"
    and $kg_checklist.operator_approval_checklist_ready == true
    and $kg_checklist.operator_approval_checklist_status == "blocked"
    and $kg_checklist.checklist_item_count == 7
    and $kg_checklist.required_checklist_item_count == 7
    and $kg_checklist.missing_checklist_item_count == 7
    and $kg_checklist.operator_approval_recorded == false
    and $kg_checklist.operator_approval_accepted == false
    and $kg_checklist.operator_approval_checklist_persisted == false
    and $kg_checklist.operator_approval_checklist_delivered == false
    and $kg_checklist.prompt_preview_rendered == false
    and $kg_checklist.context_injection_performed == false
    and $kg_checklist.model_invoked == false
    and $kg_checklist.external_kg_adapter_read_performed == false
    and $kg_checklist.network_call_performed == false
    and $kg_checklist.live_kg_write_performed == false
    and $kg_checklist.telegram_send_performed == false
    and $kg_checklist.channel_send_performed == false
    and ($kg_checklist.side_effects | to_entries | all(.value == false))

    and $memory_write.runtime == "hepta"
    and $memory_write.status == "ready"
    and $memory_write.gate == "hepta_memory_live_mutation_operator_write_approval_packet_gate"
    and $memory_write.memory_write_approval_packet_shape_ready == true
    and $memory_write.memory_write_approval_packet_recorded == false
    and $memory_write.memory_write_approval_packet_persisted == false
    and $memory_write.memory_write_approval_packet_accepted == false
    and $memory_write.required_memory_write_approval_packet_field_count == 21
    and $memory_write.recorded_memory_write_approval_packet_field_count == 0
    and $memory_write.memory_store_mutated == false
    and $memory_write.memory_store_mutation_allowed == false
    and $memory_write.memory_write_operation_allowed == false
    and $memory_write.memory_write_execution_ready == false
    and $memory_write.live_mutation_execution_ready == false
    and $memory_write.external_send_enabled == false
    and $memory_write.provider_prompt_replay_enabled == false
    and $memory_write.public_claim_or_release_artifact_write_enabled == false
    and ($memory_write.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

denied_reasons_json="$(
  jq -n '
    [
      "positive_activation_packet_dry_run_scaffold_not_authority",
      "explicit_operator_approval_record_missing",
      "operator_identity_and_scope_acceptance_missing",
      "signed_activation_digest_missing",
      "rollback_kill_switch_acceptance_missing",
      "bounded_context_preview_scope_acceptance_missing",
      "runtime_provider_router_attachment_acceptance_missing",
      "memory_write_approval_packet_acceptance_missing",
      "external_kg_adapter_probe_approval_missing",
      "live_kg_write_approval_missing",
      "audit_trail_and_retention_acceptance_missing",
      "receipt_persistence_and_readback_acceptance_missing",
      "credential_secret_read_denied",
      "provider_model_invocation_denied",
      "memory_store_write_denied",
      "external_kg_adapter_read_denied",
      "live_kg_write_denied",
      "install_restart_active_binary_mutation_denied",
      "public_release_claim_denied"
    ]
  '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_positive_activation_packet_dry_run_scaffold_gate" \
  --arg readiness_report_sha256 "$readiness_report_sha256" \
  --arg bounded_packet_report_sha256 "$bounded_packet_report_sha256" \
  --arg runtime_attachment_report_sha256 "$runtime_attachment_report_sha256" \
  --arg kg_operator_checklist_report_sha256 "$kg_operator_checklist_report_sha256" \
  --arg memory_write_packet_report_sha256 "$memory_write_packet_report_sha256" \
  --arg source_bundle_sha256 "$source_bundle_sha256" \
  --arg scaffold_hash_sha256 "$scaffold_hash_sha256" \
  --arg policy_hash_sha256 "$policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson readiness "$READINESS_JSON" \
  --argjson bounded "$BOUNDED_PACKET_JSON" \
  --argjson runtime_attachment "$RUNTIME_ATTACHMENT_JSON" \
  --argjson kg_checklist "$KG_OPERATOR_CHECKLIST_JSON" \
  --argjson memory_write "$MEMORY_WRITE_PACKET_JSON" \
  --argjson denied_reasons "$denied_reasons_json" \
  '
    [
      {
        phase_order: 1,
        phase_id: "phase_a_read_only_kg_adapter_probe_shape",
        phase_label: "Phase A read-only KG adapter probe shape",
        source_gate: $bounded.source_kg_external_adapter_staging_receipt_gate,
        source_report_sha256: $bounded.source_kg_external_adapter_staging_report_sha256,
        packet_shape_declared: true,
        packet_shape_source_ready: true,
        packet_shape_source_status: "blocked",
        required_operator_approval_record: "missing",
        operator_approval_recorded: false,
        operator_approval_accepted: false,
        activation_packet_recorded: false,
        activation_packet_persisted: false,
        activation_packet_accepted: false,
        activation_packet_delivered: false,
        live_enabled: false,
        external_kg_adapter_read_performed: false,
        kg_adapter_client_constructed_count: $bounded.kg_adapter_client_constructed_count,
        kg_adapter_network_call_attempted_count: $bounded.kg_adapter_network_call_attempted_count,
        kg_adapter_credential_read_count: $bounded.kg_adapter_credential_read_count,
        kg_adapter_credential_value_captured_count: $bounded.kg_adapter_credential_value_captured_count,
        credential_read: false,
        network_call_performed: false,
        live_kg_write_performed: false,
        status: "blocked"
      },
      {
        phase_order: 2,
        phase_id: "phase_b_bounded_context_preview_shape",
        phase_label: "Phase B bounded context preview shape",
        source_gate: $bounded.gate,
        source_report_sha256: $bounded_packet_report_sha256,
        packet_shape_declared: $bounded.activation_packet_shape_ready,
        packet_shape_source_ready: $bounded.bounded_prompt_preview_context_handoff_activation_packet_ready,
        packet_shape_source_status: $bounded.bounded_prompt_preview_context_handoff_activation_packet_status,
        required_packet_item_count: $bounded.required_activation_packet_item_count,
        accepted_packet_item_count: $bounded.accepted_activation_packet_item_count,
        required_operator_approval_record: "missing",
        operator_approval_recorded: false,
        operator_approval_accepted: false,
        activation_packet_recorded: $bounded.activation_packet_recorded,
        activation_packet_persisted: $bounded.activation_packet_persisted,
        activation_packet_accepted: $bounded.activation_packet_accepted,
        activation_packet_delivered: $bounded.activation_packet_delivered,
        prompt_preview_rendered: false,
        prompt_payload_materialized: false,
        raw_prompt_diff_count: $bounded.raw_prompt_diff_count,
        prompt_text_included_count: $bounded.prompt_text_included_count,
        payload_text_included_count: $bounded.payload_text_included_count,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        live_enabled: false,
        status: "blocked"
      },
      {
        phase_order: 3,
        phase_id: "phase_c_provider_model_context_injection_dry_run_shape",
        phase_label: "Phase C provider/model context injection dry-run shape",
        source_gate: $runtime_attachment.gate,
        source_report_sha256: $runtime_attachment_report_sha256,
        packet_shape_declared: $runtime_attachment.runtime_attachment_packet_shape_ready,
        packet_shape_source_ready: $runtime_attachment.runtime_provider_router_context_attachment_staging_ready,
        packet_shape_source_status: $runtime_attachment.runtime_provider_router_context_attachment_staging_status,
        required_packet_item_count: $runtime_attachment.required_runtime_attachment_packet_item_count,
        accepted_packet_item_count: $runtime_attachment.accepted_runtime_attachment_packet_item_count,
        required_operator_approval_record: "missing",
        operator_approval_recorded: false,
        operator_approval_accepted: false,
        activation_packet_recorded: $runtime_attachment.runtime_attachment_packet_recorded,
        activation_packet_persisted: $runtime_attachment.runtime_attachment_packet_persisted,
        activation_packet_accepted: $runtime_attachment.runtime_attachment_packet_accepted,
        activation_packet_delivered: $runtime_attachment.runtime_attachment_packet_delivered,
        router_handoff_recorded: $runtime_attachment.router_handoff_recorded,
        runtime_router_mutated: $runtime_attachment.runtime_router_mutated,
        hepta_intelligence_context_attached: false,
        live_context_attached_to_prompt: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        usage_recorded: false,
        credential_read: false,
        auth_secret_read: false,
        live_enabled: false,
        status: "blocked"
      },
      {
        phase_order: 4,
        phase_id: "phase_d_memory_write_enablement_packet_shape",
        phase_label: "Phase D Memory write enablement packet shape",
        source_gate: $memory_write.gate,
        source_report_sha256: $memory_write_packet_report_sha256,
        packet_shape_declared: $memory_write.memory_write_approval_packet_shape_ready,
        packet_shape_source_ready: $memory_write.memory_write_approval_packet_shape_ready,
        packet_shape_source_status: "blocked",
        required_packet_field_count: $memory_write.required_memory_write_approval_packet_field_count,
        recorded_packet_field_count: $memory_write.recorded_memory_write_approval_packet_field_count,
        required_operator_approval_record: "missing",
        operator_approval_recorded: false,
        operator_approval_accepted: false,
        activation_packet_recorded: $memory_write.memory_write_approval_packet_recorded,
        activation_packet_persisted: $memory_write.memory_write_approval_packet_persisted,
        activation_packet_accepted: $memory_write.memory_write_approval_packet_accepted,
        activation_packet_delivered: false,
        memory_write_operation_allowed: false,
        memory_write_execution_ready: false,
        live_mutation_execution_ready: false,
        memory_store_write_performed: false,
        memory_store_mutated: false,
        provider_prompt_replay_enabled: false,
        external_send_enabled: false,
        public_claim_or_release_artifact_write_enabled: false,
        live_enabled: false,
        status: "blocked"
      },
      {
        phase_order: 5,
        phase_id: "phase_e_live_kg_write_packet_shape",
        phase_label: "Phase E live KG write packet shape",
        source_gate: $bounded.source_kg_external_adapter_staging_receipt_gate,
        source_report_sha256: $bounded.source_kg_external_adapter_staging_report_sha256,
        packet_shape_declared: true,
        packet_shape_source_ready: true,
        packet_shape_source_status: "blocked",
        required_operator_approval_record: "missing",
        operator_approval_recorded: false,
        operator_approval_accepted: false,
        activation_packet_recorded: false,
        activation_packet_persisted: false,
        activation_packet_accepted: false,
        activation_packet_delivered: false,
        external_kg_adapter_read_performed: false,
        external_db_write_performed: false,
        live_kg_write_performed: false,
        live_kg_write_approval_recorded: false,
        audit_trail_recorded: false,
        retention_gc_policy_accepted: false,
        export_query_observability_accepted: false,
        rollback_executed: false,
        live_enabled: false,
        status: "blocked"
      }
    ] as $phases
    | {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        positive_activation_packet_dry_run_scaffold_schema_version: "memory_intelligence_kg_full_enablement_positive_activation_packet_dry_run_scaffold_v1",
        positive_activation_packet_dry_run_scaffold_ready: true,
        positive_activation_packet_dry_run_scaffold_mode: "stdout_only_report_only_positive_activation_packet_shape_no_approval_no_persistence_no_live_enablement",
        positive_activation_packet_dry_run_scaffold_status: "blocked",
        positive_activation_packet_dry_run_scaffold_decision: "phase_a_to_e_packet_shapes_declared_but_all_live_enablement_remains_blocked_until_explicit_operator_approval_evidence_receipts_readback_and_rollback_controls_exist",
        minimum_required_samples: $min_long_soak_samples,
        source_bundle_sha256: $source_bundle_sha256,
        scaffold_hash_sha256: $scaffold_hash_sha256,
        policy_hash_sha256: $policy_hash_sha256,
        side_effect_hash_sha256: $side_effect_hash_sha256,
        source_report_count: 5,
        source_readiness_gate: $readiness.gate,
        source_readiness_report_sha256: $readiness_report_sha256,
        source_bounded_prompt_preview_context_handoff_activation_packet_gate: $bounded.gate,
        source_bounded_prompt_preview_context_handoff_activation_packet_report_sha256: $bounded_packet_report_sha256,
        source_runtime_provider_router_context_attachment_staging_gate: $runtime_attachment.gate,
        source_runtime_provider_router_context_attachment_report_sha256: $runtime_attachment_report_sha256,
        source_kg_operator_approval_checklist_schema_gate: $kg_checklist.gate,
        source_kg_operator_approval_checklist_report_sha256: $kg_operator_checklist_report_sha256,
        source_memory_write_operator_approval_packet_gate: $memory_write.gate,
        source_memory_write_packet_report_sha256: $memory_write_packet_report_sha256,
        source_memory_surface_count: $readiness.memory_surface_count,
        source_absorbed_or_represented_memory_surface_count: $readiness.absorbed_or_represented_count,
        source_enablement_lane_count: $readiness.enablement_lane_count,
        source_ready_enablement_lane_count: $readiness.ready_enablement_lane_count,
        source_current_live_enabled_lane_count: $readiness.current_live_enabled_lane_count,
        source_kg_source_gate_count: $readiness.kg_source_gate_count,
        source_kg_ready_source_gate_count: $readiness.kg_ready_source_gate_count,
        source_kg_blocked_source_gate_count: $readiness.kg_blocked_source_gate_count,
        operator_approval_checklist_item_count: $kg_checklist.checklist_item_count,
        missing_operator_approval_checklist_item_count: $kg_checklist.missing_checklist_item_count,
        bounded_activation_packet_item_count: $bounded.activation_packet_item_count,
        bounded_activation_packet_accepted_item_count: $bounded.accepted_activation_packet_item_count,
        runtime_attachment_packet_item_count: $runtime_attachment.runtime_attachment_packet_item_count,
        runtime_attachment_packet_accepted_item_count: $runtime_attachment.accepted_runtime_attachment_packet_item_count,
        memory_write_approval_packet_required_field_count: $memory_write.required_memory_write_approval_packet_field_count,
        memory_write_approval_packet_recorded_field_count: $memory_write.recorded_memory_write_approval_packet_field_count,
        phase_count: ($phases | length),
        ready_phase_count: ($phases | map(select(.packet_shape_declared == true)) | length),
        blocked_phase_count: ($phases | map(select(.status == "blocked")) | length),
        accepted_phase_count: ($phases | map(select(.activation_packet_accepted == true or .operator_approval_accepted == true)) | length),
        live_enabled_phase_count: ($phases | map(select(.live_enabled == true)) | length),
        activation_phases: $phases,
        denied_reason_count: ($denied_reasons | length),
        denied_reasons: $denied_reasons,
        operator_approval_recorded: false,
        operator_approval_accepted: false,
        activation_packet_recorded: false,
        activation_packet_persisted: false,
        activation_packet_accepted: false,
        activation_packet_delivered: false,
        readiness_index_persisted: false,
        readiness_index_delivered: false,
        runtime_router_mutated: false,
        router_handoff_recorded: false,
        memory_context_activation_handoff_persisted: false,
        hepta_intelligence_context_attached: false,
        live_context_attached_to_prompt: false,
        prompt_preview_rendered: false,
        prompt_payload_materialized: false,
        context_injection_performed: false,
        provider_invoked: false,
        model_invoked: false,
        usage_recorded: false,
        memory_write_operation_allowed: false,
        memory_store_write_performed: false,
        memory_store_mutated: false,
        external_kg_adapter_read_performed: false,
        external_adapter_client_constructed: false,
        network_call_performed: false,
        external_db_write_performed: false,
        live_kg_write_performed: false,
        credential_read: false,
        auth_secret_read: false,
        secret_file_read: false,
        rollback_executed: false,
        channel_send_performed: false,
        telegram_send_performed: false,
        external_send_performed: false,
        filesystem_written: false,
        release_artifact_written: false,
        public_release_claimed: false,
        public_ga_claimed: false,
        install_performed: false,
        service_restarted: false,
        active_binary_mutated: false,
        upstream_fetch_performed: false,
        upstream_merge_performed: false,
        side_effects: {
          operator_approval_recorded: false,
          operator_approval_accepted: false,
          activation_packet_recorded: false,
          activation_packet_persisted: false,
          activation_packet_accepted: false,
          activation_packet_delivered: false,
          readiness_index_persisted: false,
          readiness_index_delivered: false,
          runtime_router_mutated: false,
          router_handoff_recorded: false,
          memory_context_activation_handoff_persisted: false,
          hepta_intelligence_context_attached: false,
          live_context_attached_to_prompt: false,
          prompt_preview_rendered: false,
          prompt_payload_materialized: false,
          context_injection_performed: false,
          provider_invoked: false,
          model_invoked: false,
          usage_recorded: false,
          memory_store_write_performed: false,
          memory_store_mutated: false,
          external_kg_adapter_read_performed: false,
          external_adapter_client_constructed: false,
          network_call_performed: false,
          external_db_write_performed: false,
          live_kg_write_performed: false,
          credential_read: false,
          auth_secret_read: false,
          secret_file_read: false,
          rollback_executed: false,
          channel_send_performed: false,
          telegram_send_performed: false,
          external_send_performed: false,
          filesystem_written: false,
          release_artifact_written: false,
          public_release_claimed: false,
          public_ga_claimed: false,
          install_performed: false,
          service_restarted: false,
          active_binary_mutated: false,
          upstream_fetch_performed: false,
          upstream_merge_performed: false
        }
      }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_enablement_positive_activation_packet_dry_run_scaffold_gate"
  and .positive_activation_packet_dry_run_scaffold_ready == true
  and .positive_activation_packet_dry_run_scaffold_status == "blocked"
  and .positive_activation_packet_dry_run_scaffold_mode == "stdout_only_report_only_positive_activation_packet_shape_no_approval_no_persistence_no_live_enablement"
  and .source_report_count == 5
  and .source_memory_surface_count == 14
  and .source_absorbed_or_represented_memory_surface_count == 14
  and .source_current_live_enabled_lane_count == 0
  and .source_kg_source_gate_count == 5
  and .source_kg_ready_source_gate_count == 5
  and .source_kg_blocked_source_gate_count == 5
  and .operator_approval_checklist_item_count == 7
  and .missing_operator_approval_checklist_item_count == 7
  and .bounded_activation_packet_item_count == 9
  and .bounded_activation_packet_accepted_item_count == 0
  and .runtime_attachment_packet_item_count == 12
  and .runtime_attachment_packet_accepted_item_count == 0
  and .memory_write_approval_packet_required_field_count == 21
  and .memory_write_approval_packet_recorded_field_count == 0
  and .phase_count == 5
  and .ready_phase_count == 5
  and .blocked_phase_count == 5
  and .accepted_phase_count == 0
  and .live_enabled_phase_count == 0
  and (.activation_phases | length) == 5
  and (.activation_phases | all(
    .packet_shape_declared == true
    and .status == "blocked"
    and .operator_approval_recorded == false
    and .operator_approval_accepted == false
    and .activation_packet_recorded == false
    and .activation_packet_persisted == false
    and .activation_packet_accepted == false
    and .activation_packet_delivered == false
    and .live_enabled == false
  ))
  and .operator_approval_recorded == false
  and .operator_approval_accepted == false
  and .activation_packet_recorded == false
  and .activation_packet_persisted == false
  and .activation_packet_accepted == false
  and .activation_packet_delivered == false
  and .runtime_router_mutated == false
  and .router_handoff_recorded == false
  and .hepta_intelligence_context_attached == false
  and .live_context_attached_to_prompt == false
  and .prompt_preview_rendered == false
  and .prompt_payload_materialized == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .usage_recorded == false
  and .memory_write_operation_allowed == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .external_kg_adapter_read_performed == false
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .live_kg_write_performed == false
  and .credential_read == false
  and .auth_secret_read == false
  and .secret_file_read == false
  and .channel_send_performed == false
  and .telegram_send_performed == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta Memory/Intelligence/KG positive activation packet dry-run scaffold gate passed"
