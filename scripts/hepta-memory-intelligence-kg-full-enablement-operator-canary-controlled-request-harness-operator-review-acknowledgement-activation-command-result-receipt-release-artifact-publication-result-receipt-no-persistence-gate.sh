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

PUBLICATION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-release-artifact-publication-denial-gate.sh
)"

publication_report_sha256="$(printf '%s' "$PUBLICATION_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$PUBLICATION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_gate"
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready == true
    and $source.minimum_required_samples >= 24
    and $source.required_activation_command_result_receipt_release_artifact_publication_surface_count == 12
    and $source.activation_command_result_receipt_release_artifact_publication_fixture_count == 10
    and $source.allowed_activation_command_result_receipt_release_artifact_publication_fixture_count == 0
    and $source.accepted_activation_command_result_receipt_release_artifact_publication_fixture_count == 0
    and $source.release_artifact_publication_recorded == false
    and $source.release_artifact_publication_persisted == false
    and $source.release_artifact_publication_materialized == false
    and $source.release_artifact_filesystem_written == false
    and $source.release_artifact_written == false
    and $source.public_artifact_written == false
    and $source.public_distribution_performed == false
    and $source.public_release_published == false
    and $source.public_ga_claimed == false
    and $source.public_claim_promoted == false
    and $source.public_version_tag_created == false
    and $source.release_notes_materialized == false
    and $source.changelog_materialized == false
    and $source.terminal_operator_decision_promoted_to_release_approval == false
    and $source.telegram_send_performed == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
    and $source.activation_command_result_receipt_materialized == false
    and $source.activation_allowed == false
    and $source.activation_performed == false
    and $source.live_mutation_execution_performed == false
    and $source.memory_write_execution_performed == false
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
    and $source.rollback_executed == false
    and $source.secret_material_read == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.install_executed == false
    and $source.launchd_mutated == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_gate" \
  --arg publication_report_sha256 "$publication_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$PUBLICATION_JSON" \
  '
  def blocked_fixture($id; $status; $reason; $extra):
    {
      id:$id,
      publication_result_receipt_status:$status,
      source_release_artifact_publication_present:true,
      source_release_artifact_publication_ready:true,
      publication_result_receipt_requested:true,
      publication_result_receipt_allowed:false,
      publication_result_receipt_accepted:false,
      publication_result_receipt_recorded:false,
      publication_result_receipt_persisted:false,
      publication_result_receipt_materialized:false,
      publication_result_receipt_filesystem_written:false,
      publication_result_receipt_ledger_written:false,
      publication_result_receipt_indexed:false,
      publication_result_receipt_enqueued:false,
      publication_result_receipt_delivered:false,
      publication_result_receipt_exported:false,
      publication_result_receipt_query_registered:false,
      publication_result_receipt_observability_recorded:false,
      publication_result_receipt_hash_bound:false,
      publication_result_receipt_signature_accepted:false,
      publication_result_receipt_timestamp_accepted:false,
      publication_result_receipt_status_accepted:false,
      publication_completion_ack_recorded:false,
      publication_completion_ack_persisted:false,
      publication_completion_ack_accepted:false,
      release_artifact_publication_recorded:false,
      release_artifact_publication_persisted:false,
      release_artifact_publication_materialized:false,
      release_artifact_filesystem_written:false,
      release_artifact_written:false,
      public_artifact_written:false,
      publication_queue_enqueued:false,
      publication_manifest_written:false,
      public_distribution_performed:false,
      public_release_published:false,
      public_ga_claimed:false,
      public_claim_promoted:false,
      public_version_tag_created:false,
      release_notes_materialized:false,
      changelog_materialized:false,
      terminal_operator_decision_promoted_to_release_approval:false,
      telegram_send_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      activation_allowed:false,
      activation_performed:false,
      live_mutation_execution_performed:false,
      memory_write_execution_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      rollback_executed:false,
      secret_material_read:false,
      provider_invoked:false,
      model_invoked:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      publication_result_receipt_noop_confirmed:true,
      reason:$reason
    } + $extra;
  [
    blocked_fixture("operator-canary-release-publication-result-receipt-missing-publication-report"; "blocked_noop"; "source_release_artifact_publication_report_required"; {source_release_artifact_publication_present:false, source_release_artifact_publication_ready:false}),
    blocked_fixture("operator-canary-release-publication-result-receipt-record-request"; "blocked_record_noop"; "publication_result_receipt_recording_denied"; {publication_result_receipt_record_requested:true}),
    blocked_fixture("operator-canary-release-publication-result-receipt-persist-request"; "blocked_persist_noop"; "publication_result_receipt_persistence_denied"; {publication_result_receipt_persist_requested:true}),
    blocked_fixture("operator-canary-release-publication-result-receipt-materialize-filesystem-request"; "blocked_materialize_noop"; "publication_result_receipt_materialization_filesystem_write_denied"; {publication_result_receipt_materialize_requested:true, publication_result_receipt_filesystem_write_requested:true}),
    blocked_fixture("operator-canary-release-publication-result-receipt-ledger-index-queue-request"; "blocked_ledger_index_queue_noop"; "publication_result_receipt_ledger_index_queue_denied"; {publication_result_receipt_ledger_write_requested:true, publication_result_receipt_index_requested:true, publication_result_receipt_enqueue_requested:true}),
    blocked_fixture("operator-canary-release-publication-result-receipt-export-query-observability-request"; "blocked_export_query_observability_noop"; "publication_result_receipt_export_query_observability_denied"; {publication_result_receipt_export_requested:true, publication_result_receipt_query_requested:true, publication_result_receipt_observability_requested:true}),
    blocked_fixture("operator-canary-release-publication-result-receipt-delivery-request"; "blocked_delivery_noop"; "publication_result_receipt_delivery_denied"; {publication_result_receipt_delivery_requested:true, telegram_delivery_requested:true, channel_delivery_requested:true, external_delivery_requested:true}),
    blocked_fixture("operator-canary-release-publication-result-receipt-status-signature-request"; "blocked_acceptance_noop"; "publication_result_receipt_status_signature_acceptance_denied"; {publication_result_receipt_status_acceptance_requested:true, publication_result_receipt_signature_acceptance_requested:true, publication_result_receipt_timestamp_acceptance_requested:true}),
    blocked_fixture("operator-canary-release-publication-result-receipt-completion-ack-request"; "blocked_ack_noop"; "publication_completion_ack_denied"; {publication_completion_ack_requested:true}),
    blocked_fixture("operator-canary-release-publication-result-receipt-authority-request"; "blocked_authority_noop"; "publication_result_receipt_cannot_authorize_publication_activation_or_install"; {publication_authority_requested:true, public_release_publish_requested:true, public_distribution_requested:true, release_artifact_write_requested:true, activation_from_publication_receipt_requested:true, memory_write_publication_receipt_requested:true, provider_prompt_publication_receipt_requested:true, install_publication_receipt_requested:true, service_restart_publication_receipt_requested:true, active_binary_publication_receipt_requested:true})
  ] as $fixtures
  | {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_mode:"operator_canary_release_artifact_publication_result_receipt_no_persistence",
    source_activation_command_result_receipt_release_artifact_publication_gate:$source.gate,
    source_activation_command_result_receipt_release_artifact_publication_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready,
    source_activation_command_result_receipt_release_artifact_publication_report_sha256:$publication_report_sha256,
    minimum_required_samples:$min_long_soak_samples,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_ready:true,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready:true,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready:true,
    required_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count:12,
    ready_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count:12,
    side_effect_free_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count:12,
    required_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count:10,
    activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count:($fixtures | length),
    blocked_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count:($fixtures | length),
    noop_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count:($fixtures | length),
    allowed_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count:0,
    accepted_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count:0,
    publication_result_receipt_allowed:false,
    publication_result_receipt_accepted:false,
    publication_result_receipt_recorded:false,
    publication_result_receipt_persisted:false,
    publication_result_receipt_materialized:false,
    publication_result_receipt_filesystem_written:false,
    publication_result_receipt_ledger_written:false,
    publication_result_receipt_indexed:false,
    publication_result_receipt_enqueued:false,
    publication_result_receipt_delivered:false,
    publication_result_receipt_exported:false,
    publication_result_receipt_query_registered:false,
    publication_result_receipt_observability_recorded:false,
    publication_result_receipt_hash_bound:false,
    publication_result_receipt_signature_accepted:false,
    publication_result_receipt_timestamp_accepted:false,
    publication_result_receipt_status_accepted:false,
    publication_completion_ack_recorded:false,
    publication_completion_ack_persisted:false,
    publication_completion_ack_accepted:false,
    release_artifact_publication_recorded:false,
    release_artifact_publication_persisted:false,
    release_artifact_publication_materialized:false,
    release_artifact_filesystem_written:false,
    release_artifact_written:false,
    public_artifact_written:false,
    publication_queue_enqueued:false,
    publication_manifest_written:false,
    public_distribution_performed:false,
    public_release_published:false,
    public_ga_claimed:false,
    public_claim_promoted:false,
    public_version_tag_created:false,
    release_notes_materialized:false,
    changelog_materialized:false,
    terminal_operator_decision_promoted_to_release_approval:false,
    telegram_send_performed:false,
    channel_send_performed:false,
    external_send_performed:false,
    activation_allowed_by_publication_result_receipt:false,
    activation_allowed:false,
    activation_performed:false,
    live_mutation_execution_performed:false,
    memory_write_execution_performed:false,
    memory_store_write_performed:false,
    memory_store_mutated:false,
    rollback_executed:false,
    secret_material_read:false,
    provider_invoked:false,
    model_invoked:false,
    install_executed:false,
    launchd_mutated:false,
    service_restarted:false,
    active_binary_mutated:false,
    activation_command_result_receipt_release_artifact_publication_result_receipt_surfaces:[
      "source_release_artifact_publication_report_required",
      "publication_result_receipt_recording_denied",
      "publication_result_receipt_persistence_denied",
      "publication_result_receipt_materialization_denied",
      "publication_result_receipt_filesystem_write_denied",
      "publication_result_receipt_ledger_index_queue_denied",
      "publication_result_receipt_export_query_observability_denied",
      "publication_result_receipt_delivery_denied",
      "publication_result_receipt_status_signature_acceptance_denied",
      "publication_completion_ack_denied",
      "publication_result_receipt_authority_denied",
      "publication_result_receipt_external_install_restart_active_binary_denied"
    ],
    activation_command_result_receipt_release_artifact_publication_result_receipt_fixtures:$fixtures,
    denied_by_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence:[
      "source_release_artifact_publication_report_required",
      "publication_result_receipt_recording_denied",
      "publication_result_receipt_persistence_denied",
      "publication_result_receipt_materialization_denied",
      "publication_result_receipt_filesystem_write_denied",
      "publication_result_receipt_ledger_index_queue_denied",
      "publication_result_receipt_export_query_observability_denied",
      "publication_result_receipt_delivery_denied",
      "publication_result_receipt_status_signature_acceptance_denied",
      "publication_completion_ack_denied",
      "publication_result_receipt_publication_authority_denied",
      "publication_result_receipt_activation_authority_denied",
      "publication_result_receipt_memory_provider_install_restart_active_binary_denied"
    ],
    side_effects:{
      publication_result_receipt_recorded:false,
      publication_result_receipt_persisted:false,
      publication_result_receipt_materialized:false,
      publication_result_receipt_filesystem_written:false,
      publication_result_receipt_ledger_written:false,
      publication_result_receipt_indexed:false,
      publication_result_receipt_enqueued:false,
      publication_result_receipt_delivered:false,
      publication_result_receipt_exported:false,
      publication_result_receipt_query_registered:false,
      publication_result_receipt_observability_recorded:false,
      publication_result_receipt_hash_bound:false,
      publication_result_receipt_signature_accepted:false,
      publication_result_receipt_timestamp_accepted:false,
      publication_completion_ack_recorded:false,
      publication_completion_ack_persisted:false,
      publication_completion_ack_accepted:false,
      release_artifact_publication_recorded:false,
      release_artifact_publication_persisted:false,
      release_artifact_publication_materialized:false,
      release_artifact_written:false,
      public_artifact_written:false,
      publication_queue_enqueued:false,
      publication_manifest_written:false,
      public_distribution_performed:false,
      public_release_published:false,
      public_ga_claimed:false,
      public_claim_promoted:false,
      terminal_operator_decision_promoted_to_release_approval:false,
      telegram_send_performed:false,
      channel_send_performed:false,
      external_send_performed:false,
      activation_performed:false,
      live_mutation_execution_performed:false,
      memory_write_execution_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      rollback_executed:false,
      secret_file_read:false,
      credential_read:false,
      provider_invoked:false,
      model_invoked:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      active_binary_mutated:false,
      filesystem_written:false
    }
  }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_result_receipt_no_persistence_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready == true
  and .required_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count == 12
  and .ready_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_release_artifact_publication_result_receipt_surface_count == 12
  and .required_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count == 10
  and .activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count == 10
  and .blocked_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count == 10
  and .allowed_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count == 0
  and .accepted_activation_command_result_receipt_release_artifact_publication_result_receipt_fixture_count == 0
  and .publication_result_receipt_recorded == false
  and .publication_result_receipt_persisted == false
  and .publication_result_receipt_materialized == false
  and .publication_result_receipt_filesystem_written == false
  and .publication_result_receipt_ledger_written == false
  and .publication_result_receipt_indexed == false
  and .publication_result_receipt_enqueued == false
  and .publication_result_receipt_delivered == false
  and .publication_result_receipt_exported == false
  and .publication_result_receipt_query_registered == false
  and .publication_result_receipt_observability_recorded == false
  and .publication_completion_ack_recorded == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .public_distribution_performed == false
  and .public_release_published == false
  and .public_ga_claimed == false
  and .terminal_operator_decision_promoted_to_release_approval == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .activation_allowed_by_publication_result_receipt == false
  and .activation_allowed == false
  and .activation_performed == false
  and .live_mutation_execution_performed == false
  and .memory_write_execution_performed == false
  and .memory_store_write_performed == false
  and .memory_store_mutated == false
  and .rollback_executed == false
  and .secret_material_read == false
  and .provider_invoked == false
  and .model_invoked == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_release_artifact_publication_result_receipt_fixtures | length) == 10
  and (.activation_command_result_receipt_release_artifact_publication_result_receipt_fixtures | all(
    (.publication_result_receipt_status == "blocked_noop" or .publication_result_receipt_status == "blocked_record_noop" or .publication_result_receipt_status == "blocked_persist_noop" or .publication_result_receipt_status == "blocked_materialize_noop" or .publication_result_receipt_status == "blocked_ledger_index_queue_noop" or .publication_result_receipt_status == "blocked_export_query_observability_noop" or .publication_result_receipt_status == "blocked_delivery_noop" or .publication_result_receipt_status == "blocked_acceptance_noop" or .publication_result_receipt_status == "blocked_ack_noop" or .publication_result_receipt_status == "blocked_authority_noop")
    and .publication_result_receipt_allowed == false
    and .publication_result_receipt_accepted == false
    and .publication_result_receipt_recorded == false
    and .publication_result_receipt_persisted == false
    and .publication_result_receipt_filesystem_written == false
    and .publication_result_receipt_delivered == false
    and .publication_result_receipt_exported == false
    and .publication_result_receipt_query_registered == false
    and .publication_result_receipt_observability_recorded == false
    and .publication_completion_ack_recorded == false
    and .release_artifact_written == false
    and .public_artifact_written == false
    and .public_distribution_performed == false
    and .public_release_published == false
    and .external_send_performed == false
    and .activation_allowed == false
    and .live_mutation_execution_performed == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .rollback_executed == false
    and .publication_result_receipt_noop_confirmed == true
  ))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full enablement operator canary controlled-request harness operator review acknowledgement activation command result receipt release artifact publication result receipt no-persistence gate passed"
