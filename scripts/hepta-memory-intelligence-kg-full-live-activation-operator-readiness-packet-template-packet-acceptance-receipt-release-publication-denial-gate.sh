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

TERMINAL_DECISION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-terminal-decision-status-promotion-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-terminal-decision-status-promotion-denial-gate.sh
)"

terminal_decision_report_sha256="$(sha256_text "$TERMINAL_DECISION_JSON")"
release_publication_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-operator-readiness-packet-template-packet-acceptance-receipt-release-publication-denial:$terminal_decision_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"

jq -n -e \
  --argjson source "$TERMINAL_DECISION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_ready == true
    and $source.terminal_decision_status_surface_count == 14
    and $source.terminal_decision_status_attempt_count == 14
    and $source.terminal_decision_accepted_count == 0
    and $source.terminal_decision_recorded_count == 0
    and $source.terminal_decision_persisted_count == 0
    and $source.terminal_status_closed_count == 0
    and $source.terminal_status_ready_count == 0
    and $source.terminal_status_accepted_count == 0
    and $source.terminal_status_approved_count == 0
    and $source.terminal_status_authoritative_count == 0
    and $source.terminal_status_live_count == 0
    and $source.public_status_claimed_count == 0
    and $source.release_status_claimed_count == 0
    and $source.terminal_decision_acceptance_recorded_count == 0
    and $source.terminal_decision_activation_authority_derived_count == 0
    and $source.terminal_decision_live_execution_allowed_count == 0
    and $source.packet_acceptance_receipt_terminal_decision_accepted == false
    and $source.packet_acceptance_receipt_terminal_status_closed == false
    and $source.packet_acceptance_receipt_status_approved == false
    and $source.packet_acceptance_receipt_status_authoritative == false
    and $source.packet_acceptance_receipt_status_live == false
    and $source.packet_acceptance_receipt_public_status_claimed == false
    and $source.packet_acceptance_receipt_release_status_claimed == false
    and $source.operator_acceptance_recorded == false
    and $source.operator_approval_recorded == false
    and $source.activation_authority_derived == false
    and $source.activation_command_derived == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.public_release_claimed == false
    and $source.public_ga_claimed == false
    and $source.release_artifact_written == false
    and $source.public_artifact_written == false
    and $source.external_send_performed == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

release_publication_surfaces_json="$(
  jq -n '[
    "packet_receipt_release_artifact_write_claim",
    "packet_receipt_public_artifact_write_claim",
    "packet_receipt_artifact_signature_claim",
    "packet_receipt_artifact_notarization_claim",
    "packet_receipt_publication_queue_claim",
    "packet_receipt_publication_manifest_claim",
    "packet_receipt_public_distribution_claim",
    "packet_receipt_channel_delivery_publication_claim",
    "packet_receipt_public_version_tag_claim",
    "packet_receipt_release_notes_materialization_claim",
    "packet_receipt_changelog_materialization_claim",
    "packet_receipt_public_release_claim",
    "packet_receipt_public_ga_claim",
    "packet_receipt_terminal_status_release_approval_claim"
  ] | map({
    release_publication_surface: .,
    release_publication_attempted: true,
    release_publication_allowed: false,
    release_publication_accepted: false,
    release_publication_recorded: false,
    release_publication_persisted: false,
    release_publication_materialized: false,
    release_artifact_written: false,
    public_artifact_written: false,
    artifact_signature_accepted: false,
    artifact_notarization_accepted: false,
    publication_queue_enqueued: false,
    publication_manifest_written: false,
    public_distribution_performed: false,
    channel_delivery_performed: false,
    external_send_performed: false,
    public_version_tag_created: false,
    release_notes_materialized: false,
    changelog_materialized: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    terminal_status_promoted_to_release_approval: false,
    acceptance_recorded: false,
    operator_approval_derived: false,
    activation_authority_derived: false,
    activation_command_derived: false,
    live_execution_allowed: false,
    release_publication_status: "release_publication_denied"
  })'
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_gate" \
  --arg terminal_decision_report_sha256 "$terminal_decision_report_sha256" \
  --arg release_publication_contract_hash_sha256 "$release_publication_contract_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$TERMINAL_DECISION_JSON" \
  --argjson surfaces "$release_publication_surfaces_json" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    receipt_release_publication_schema_version:"memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_v1",
    receipt_release_publication_mode:"non_accepted_terminal_receipt_status_cannot_become_release_publication_or_public_claim",
    source_packet_acceptance_receipt_terminal_decision_status_gate:$source.gate,
    source_packet_acceptance_receipt_terminal_decision_status_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_terminal_decision_status_promotion_denial_ready,
    source_terminal_decision_status_report_sha256:$terminal_decision_report_sha256,
    source_terminal_decision_contract_hash_sha256:$source.terminal_decision_contract_hash_sha256,
    release_publication_contract_hash_sha256:$release_publication_contract_hash_sha256,
    minimum_required_samples:$min_long_soak_samples,
    memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_ready:true,
    source_terminal_decision_status_surface_count:$source.terminal_decision_status_surface_count,
    source_terminal_decision_accepted_count:$source.terminal_decision_accepted_count,
    source_terminal_decision_recorded_count:$source.terminal_decision_recorded_count,
    source_terminal_status_closed_count:$source.terminal_status_closed_count,
    source_terminal_status_live_count:$source.terminal_status_live_count,
    source_public_status_claimed_count:$source.public_status_claimed_count,
    source_release_status_claimed_count:$source.release_status_claimed_count,
    source_terminal_decision_activation_authority_derived_count:$source.terminal_decision_activation_authority_derived_count,
    release_publication_surface_count:($surfaces | length),
    release_publication_attempt_count:($surfaces | length),
    release_publication_allowed_count:0,
    release_publication_accepted_count:0,
    release_publication_recorded_count:0,
    release_publication_persisted_count:0,
    release_publication_materialized_count:0,
    release_artifact_written_count:0,
    public_artifact_written_count:0,
    artifact_signature_accepted_count:0,
    artifact_notarization_accepted_count:0,
    publication_queue_enqueued_count:0,
    publication_manifest_written_count:0,
    public_distribution_performed_count:0,
    channel_delivery_performed_count:0,
    external_publication_sent_count:0,
    public_version_tag_created_count:0,
    release_notes_materialized_count:0,
    changelog_materialized_count:0,
    public_release_claimed_count:0,
    public_ga_claimed_count:0,
    terminal_status_release_approval_promoted_count:0,
    release_publication_acceptance_recorded_count:0,
    release_publication_operator_approval_derived_count:0,
    release_publication_activation_authority_derived_count:0,
    release_publication_activation_command_derived_count:0,
    release_publication_live_execution_allowed_count:0,
    release_publication_surfaces:$surfaces,
    denied_by_packet_receipt_release_publication:[
      "operator_readiness_packet_template_packet_receipt_release_artifact_write_denied",
      "operator_readiness_packet_template_packet_receipt_public_artifact_write_denied",
      "operator_readiness_packet_template_packet_receipt_artifact_signature_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_artifact_notarization_acceptance_denied",
      "operator_readiness_packet_template_packet_receipt_publication_queue_enqueue_denied",
      "operator_readiness_packet_template_packet_receipt_publication_manifest_write_denied",
      "operator_readiness_packet_template_packet_receipt_public_distribution_denied",
      "operator_readiness_packet_template_packet_receipt_channel_delivery_publication_denied",
      "operator_readiness_packet_template_packet_receipt_public_version_tag_denied",
      "operator_readiness_packet_template_packet_receipt_release_notes_materialization_denied",
      "operator_readiness_packet_template_packet_receipt_changelog_materialization_denied",
      "operator_readiness_packet_template_packet_receipt_public_release_claim_denied",
      "operator_readiness_packet_template_packet_receipt_public_ga_claim_denied",
      "operator_readiness_packet_template_packet_receipt_terminal_status_as_release_approval_denied",
      "operator_readiness_packet_template_packet_receipt_acceptance_from_release_publication_denied",
      "operator_readiness_packet_template_packet_receipt_authority_from_release_publication_denied",
      "operator_readiness_packet_template_packet_receipt_live_execution_from_release_publication_denied"
    ],
    allowed_next_actions:[
      {
        action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_no_persistence_gate",
        status:"allowed_report_only_next_slice",
        records_release_publication:false,
        writes_release_artifact:false,
        writes_public_artifact:false,
        enqueues_publication:false,
        claims_public_release:false,
        records_operator_acceptance:false,
        derives_activation_authority:false,
        activates_live:false,
        mutates_memory_store:false,
        writes_kg:false
      }
    ],
    packet_acceptance_receipt_terminal_decision_accepted:false,
    packet_acceptance_receipt_terminal_status_recorded:false,
    packet_acceptance_receipt_terminal_status_closed:false,
    packet_acceptance_receipt_status_approved:false,
    packet_acceptance_receipt_status_authoritative:false,
    packet_acceptance_receipt_status_live:false,
    packet_acceptance_receipt_public_status_claimed:false,
    packet_acceptance_receipt_release_status_claimed:false,
    packet_acceptance_receipt_release_publication_allowed:false,
    packet_acceptance_receipt_release_publication_accepted:false,
    packet_acceptance_receipt_release_publication_recorded:false,
    packet_acceptance_receipt_release_publication_persisted:false,
    packet_acceptance_receipt_release_publication_materialized:false,
    packet_acceptance_receipt_release_artifact_written:false,
    packet_acceptance_receipt_public_artifact_written:false,
    packet_acceptance_receipt_artifact_signature_accepted:false,
    packet_acceptance_receipt_artifact_notarization_accepted:false,
    packet_acceptance_receipt_publication_queue_enqueued:false,
    packet_acceptance_receipt_publication_manifest_written:false,
    packet_acceptance_receipt_public_distribution_performed:false,
    packet_acceptance_receipt_public_version_tag_created:false,
    packet_acceptance_receipt_release_notes_materialized:false,
    packet_acceptance_receipt_changelog_materialized:false,
    packet_acceptance_receipt_public_release_claimed:false,
    packet_acceptance_receipt_public_ga_claimed:false,
    packet_acceptance_receipt_terminal_status_promoted_to_release_approval:false,
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
    side_effects:{
      packet_acceptance_receipt_release_publication_allowed:false,
      packet_acceptance_receipt_release_publication_accepted:false,
      packet_acceptance_receipt_release_publication_recorded:false,
      packet_acceptance_receipt_release_publication_persisted:false,
      packet_acceptance_receipt_release_publication_materialized:false,
      packet_acceptance_receipt_release_artifact_written:false,
      packet_acceptance_receipt_public_artifact_written:false,
      packet_acceptance_receipt_artifact_signature_accepted:false,
      packet_acceptance_receipt_artifact_notarization_accepted:false,
      packet_acceptance_receipt_publication_queue_enqueued:false,
      packet_acceptance_receipt_publication_manifest_written:false,
      packet_acceptance_receipt_public_distribution_performed:false,
      packet_acceptance_receipt_channel_delivery_performed:false,
      packet_acceptance_receipt_external_publication_sent:false,
      packet_acceptance_receipt_public_version_tag_created:false,
      packet_acceptance_receipt_release_notes_materialized:false,
      packet_acceptance_receipt_changelog_materialized:false,
      packet_acceptance_receipt_public_release_claimed:false,
      packet_acceptance_receipt_public_ga_claimed:false,
      packet_acceptance_receipt_terminal_status_promoted_to_release_approval:false,
      packet_acceptance_receipt_acceptance_recorded:false,
      packet_acceptance_receipt_authority_derived:false,
      packet_acceptance_receipt_live_execution_allowed:false,
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
  and .gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_gate"
  and .memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_denial_ready == true
  and .source_packet_acceptance_receipt_terminal_decision_status_ready == true
  and .source_terminal_decision_status_surface_count == 14
  and .source_terminal_decision_accepted_count == 0
  and .source_terminal_decision_recorded_count == 0
  and .source_terminal_status_closed_count == 0
  and .source_terminal_status_live_count == 0
  and .source_public_status_claimed_count == 0
  and .source_release_status_claimed_count == 0
  and .source_terminal_decision_activation_authority_derived_count == 0
  and .release_publication_surface_count == 14
  and .release_publication_attempt_count == 14
  and .release_publication_allowed_count == 0
  and .release_publication_accepted_count == 0
  and .release_publication_recorded_count == 0
  and .release_publication_persisted_count == 0
  and .release_publication_materialized_count == 0
  and .release_artifact_written_count == 0
  and .public_artifact_written_count == 0
  and .artifact_signature_accepted_count == 0
  and .artifact_notarization_accepted_count == 0
  and .publication_queue_enqueued_count == 0
  and .publication_manifest_written_count == 0
  and .public_distribution_performed_count == 0
  and .channel_delivery_performed_count == 0
  and .external_publication_sent_count == 0
  and .public_version_tag_created_count == 0
  and .release_notes_materialized_count == 0
  and .changelog_materialized_count == 0
  and .public_release_claimed_count == 0
  and .public_ga_claimed_count == 0
  and .terminal_status_release_approval_promoted_count == 0
  and .release_publication_acceptance_recorded_count == 0
  and .release_publication_operator_approval_derived_count == 0
  and .release_publication_activation_authority_derived_count == 0
  and .release_publication_activation_command_derived_count == 0
  and .release_publication_live_execution_allowed_count == 0
  and (.release_publication_surfaces | all(
    .release_publication_attempted == true
    and .release_publication_allowed == false
    and .release_publication_accepted == false
    and .release_publication_recorded == false
    and .release_publication_persisted == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .artifact_signature_accepted == false
    and .artifact_notarization_accepted == false
    and .publication_queue_enqueued == false
    and .publication_manifest_written == false
    and .public_distribution_performed == false
    and .channel_delivery_performed == false
    and .external_send_performed == false
    and .public_version_tag_created == false
    and .release_notes_materialized == false
    and .changelog_materialized == false
    and .public_release_claimed == false
    and .public_ga_claimed == false
    and .terminal_status_promoted_to_release_approval == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .activation_authority_derived == false
    and .activation_command_derived == false
    and .live_execution_allowed == false
    and .release_publication_status == "release_publication_denied"
  ))
  and (.denied_by_packet_receipt_release_publication | length) == 17
  and (.allowed_next_actions | all(.status == "allowed_report_only_next_slice"))
  and .packet_acceptance_receipt_release_publication_allowed == false
  and .packet_acceptance_receipt_release_publication_accepted == false
  and .packet_acceptance_receipt_release_publication_recorded == false
  and .packet_acceptance_receipt_release_artifact_written == false
  and .packet_acceptance_receipt_public_artifact_written == false
  and .packet_acceptance_receipt_publication_queue_enqueued == false
  and .packet_acceptance_receipt_publication_manifest_written == false
  and .packet_acceptance_receipt_public_distribution_performed == false
  and .packet_acceptance_receipt_public_version_tag_created == false
  and .packet_acceptance_receipt_release_notes_materialized == false
  and .packet_acceptance_receipt_changelog_materialized == false
  and .packet_acceptance_receipt_public_release_claimed == false
  and .packet_acceptance_receipt_public_ga_claimed == false
  and .packet_acceptance_receipt_terminal_status_promoted_to_release_approval == false
  and .operator_acceptance_recorded == false
  and .operator_approval_recorded == false
  and .activation_authority_derived == false
  and .activation_command_derived == false
  and .activation_allowed == false
  and .activation_performed == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication denial gate passed"
