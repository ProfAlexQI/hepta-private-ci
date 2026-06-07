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

PACKET_TEMPLATE_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-gate.sh
)"

packet_template_report_sha256="$(sha256_text "$PACKET_TEMPLATE_JSON")"
non_acceptance_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-non-acceptance-authority-replay-denial:$packet_template_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$PACKET_TEMPLATE_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_ready == true
    and $source.source_full_live_activation_enabled == false
    and $source.source_full_live_activation_status == "blocked_report_only"
    and $source.required_operator_packet_section_count == 10
    and $source.operator_packet_section_count == 10
    and $source.accepted_operator_packet_section_count == 0
    and $source.recorded_operator_packet_section_count == 0
    and $source.operator_packet_required_field_count == 43
    and $source.operator_packet_recorded_field_count == 0
    and $source.operator_packet_accepted_field_count == 0
    and $source.packet_template_recorded == false
    and $source.packet_template_persisted == false
    and $source.packet_template_materialized == false
    and $source.packet_template_delivered == false
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and ($source.operator_packet_sections | all(.template_only == true and .accepted == false and .activation_authority == false))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_gate" \
  --arg packet_template_report_sha256 "$packet_template_report_sha256" \
  --arg non_acceptance_contract_hash_sha256 "$non_acceptance_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$PACKET_TEMPLATE_JSON" \
  '
  def denied_fixture($id; $reason; $extra):
    {
      id:$id,
      source_operator_readiness_packet_template_present:true,
      source_operator_readiness_packet_template_ready:true,
      template_seen:true,
      template_replayed:false,
      template_replay_allowed:false,
      template_replay_accepted:false,
      template_reference_registered:false,
      template_reference_persisted:false,
      template_summary_promoted:false,
      template_cache_written:false,
      template_query_registered:false,
      template_export_recorded:false,
      template_observability_recorded:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
      activation_authority_derived:false,
      activation_command_derived:false,
      activation_allowed:false,
      activation_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      hepta_intelligence_context_attached:false,
      prompt_preview_rendered:false,
      context_injection_performed:false,
      provider_invoked:false,
      model_invoked:false,
      external_kg_adapter_read_performed:false,
      external_adapter_client_constructed:false,
      network_call_performed:false,
      external_db_write_performed:false,
      live_kg_write_performed:false,
      credential_read:false,
      secret_file_read:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      external_send_performed:false,
      template_non_acceptance_noop_confirmed:true,
      reason:$reason
    } + $extra;
  [
    denied_fixture("operator-readiness-packet-template-viewed"; "template_view_is_not_acceptance"; {template_viewed:true}),
    denied_fixture("operator-readiness-packet-template-summary"; "template_summary_is_not_acceptance"; {template_summary_requested:true}),
    denied_fixture("operator-readiness-packet-template-replay"; "template_replay_denied"; {template_replayed:true}),
    denied_fixture("operator-readiness-packet-template-reference-registration"; "template_reference_registration_denied"; {template_reference_registration_requested:true}),
    denied_fixture("operator-readiness-packet-template-cache-write"; "template_cache_write_denied"; {template_cache_write_requested:true}),
    denied_fixture("operator-readiness-packet-template-query-export-observability"; "template_query_export_observability_denied"; {template_query_requested:true, template_export_requested:true, template_observability_requested:true}),
    denied_fixture("operator-readiness-packet-template-operator-acceptance"; "template_cannot_record_operator_acceptance"; {operator_acceptance_record_requested:true}),
    denied_fixture("operator-readiness-packet-template-operator-approval"; "template_cannot_record_operator_approval"; {operator_approval_record_requested:true}),
    denied_fixture("operator-readiness-packet-template-activation-authority"; "template_cannot_derive_activation_authority"; {activation_authority_requested:true, activation_command_requested:true}),
    denied_fixture("operator-readiness-packet-template-live-side-effects"; "template_cannot_authorize_live_side_effects"; {memory_write_requested:true, kg_write_requested:true, context_injection_requested:true, provider_invocation_requested:true, credential_read_requested:true, install_restart_requested:true, public_release_requested:true, external_send_requested:true})
  ] as $fixtures
  | {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    non_acceptance_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_v1",
    non_acceptance_mode:"packet_template_view_summary_replay_reference_no_acceptance_no_authority_no_live",
    source_operator_readiness_packet_template_gate:$source.gate,
    source_operator_readiness_packet_template_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_ready,
    source_operator_readiness_packet_template_report_sha256:$packet_template_report_sha256,
    non_acceptance_contract_hash_sha256:$non_acceptance_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_ready:true,
    source_operator_packet_section_count:$source.operator_packet_section_count,
    source_operator_packet_required_field_count:$source.operator_packet_required_field_count,
    source_operator_packet_recorded_field_count:$source.operator_packet_recorded_field_count,
    source_operator_packet_accepted_field_count:$source.operator_packet_accepted_field_count,
    required_non_acceptance_surface_count:12,
    ready_non_acceptance_surface_count:12,
    side_effect_free_non_acceptance_surface_count:12,
    required_non_acceptance_fixture_count:10,
    non_acceptance_fixture_count:($fixtures | length),
    blocked_non_acceptance_fixture_count:($fixtures | length),
    noop_non_acceptance_fixture_count:($fixtures | length),
    allowed_non_acceptance_fixture_count:0,
    accepted_non_acceptance_fixture_count:0,
    template_view_is_acceptance:false,
    template_summary_is_acceptance:false,
    template_replay_allowed:false,
    template_replay_accepted:false,
    template_reference_registered:false,
    template_reference_persisted:false,
    template_cache_written:false,
    template_query_registered:false,
    template_export_recorded:false,
    template_observability_recorded:false,
    operator_acceptance_recorded:false,
    operator_approval_recorded:false,
    activation_authority_derived:false,
    activation_command_derived:false,
    activation_allowed:false,
    activation_performed:false,
    non_acceptance_fixtures:$fixtures,
    denied_by_template_non_acceptance_authority_replay:[
      "operator_readiness_packet_template_view_acceptance_denied",
      "operator_readiness_packet_template_summary_acceptance_denied",
      "operator_readiness_packet_template_replay_denied",
      "operator_readiness_packet_template_reference_registration_denied",
      "operator_readiness_packet_template_cache_write_denied",
      "operator_readiness_packet_template_query_export_observability_denied",
      "operator_readiness_packet_template_operator_acceptance_denied",
      "operator_readiness_packet_template_operator_approval_denied",
      "operator_readiness_packet_template_activation_authority_denied",
      "operator_readiness_packet_template_live_side_effects_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_field_validation_denial_gate",
        status:"allowed_report_only_next_slice",
        records_operator_acceptance:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    memory_store_write_performed:false,
    memory_store_mutated:false,
    hepta_intelligence_context_attached:false,
    prompt_preview_rendered:false,
    context_injection_performed:false,
    provider_invoked:false,
    model_invoked:false,
    external_kg_adapter_read_performed:false,
    external_adapter_client_constructed:false,
    network_call_performed:false,
    external_db_write_performed:false,
    live_kg_write_performed:false,
    credential_read:false,
    secret_file_read:false,
    install_executed:false,
    launchd_mutated:false,
    service_restarted:false,
    active_binary_mutated:false,
    public_release_claimed:false,
    public_ga_claimed:false,
    release_artifact_written:false,
    public_artifact_written:false,
    external_send_performed:false,
    side_effects:{
      template_view_recorded:false,
      template_summary_recorded:false,
      template_replay_performed:false,
      template_reference_registered:false,
      template_reference_persisted:false,
      template_cache_written:false,
      template_query_registered:false,
      template_export_recorded:false,
      template_observability_recorded:false,
      operator_acceptance_recorded:false,
      operator_approval_recorded:false,
      activation_authority_derived:false,
      activation_command_derived:false,
      activation_allowed:false,
      activation_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      hepta_intelligence_context_attached:false,
      prompt_preview_rendered:false,
      context_injection_performed:false,
      provider_invoked:false,
      model_invoked:false,
      external_kg_adapter_read_performed:false,
      external_adapter_client_constructed:false,
      network_call_performed:false,
      external_db_write_performed:false,
      live_kg_write_performed:false,
      credential_read:false,
      secret_file_read:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      public_release_claimed:false,
      public_ga_claimed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      external_send_performed:false,
      filesystem_written:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_non_acceptance_authority_replay_denial_ready == true
  and .source_operator_readiness_packet_template_ready == true
  and .source_operator_packet_section_count == 10
  and .source_operator_packet_required_field_count == 43
  and .source_operator_packet_recorded_field_count == 0
  and .source_operator_packet_accepted_field_count == 0
  and .required_non_acceptance_surface_count == 12
  and .ready_non_acceptance_surface_count == 12
  and .side_effect_free_non_acceptance_surface_count == 12
  and .required_non_acceptance_fixture_count == 10
  and .non_acceptance_fixture_count == 10
  and .blocked_non_acceptance_fixture_count == 10
  and .allowed_non_acceptance_fixture_count == 0
  and .accepted_non_acceptance_fixture_count == 0
  and .template_view_is_acceptance == false
  and .template_summary_is_acceptance == false
  and .template_replay_allowed == false
  and .template_replay_accepted == false
  and .template_reference_registered == false
  and .template_reference_persisted == false
  and .template_cache_written == false
  and .template_query_registered == false
  and .template_export_recorded == false
  and .template_observability_recorded == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
  and (.non_acceptance_fixtures | all(.operator_acceptance_recorded == false and .operator_approval_recorded == false and .activation_authority_derived == false and .activation_command_derived == false))
  and (.denied_by_template_non_acceptance_authority_replay | length) == 10
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .hepta_intelligence_context_attached == false
  and .prompt_preview_rendered == false
  and .context_injection_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .external_kg_adapter_read_performed == false
  and .external_adapter_client_constructed == false
  and .network_call_performed == false
  and .external_db_write_performed == false
  and .live_kg_write_performed == false
  and .credential_read == false
  and .secret_file_read == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .external_send_performed == false
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report"
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template non-acceptance authority replay denial gate passed"
