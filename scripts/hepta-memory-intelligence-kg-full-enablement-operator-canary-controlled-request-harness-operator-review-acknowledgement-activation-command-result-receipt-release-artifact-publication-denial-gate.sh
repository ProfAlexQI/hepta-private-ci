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

TERMINAL_DECISION_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-enablement-operator-canary-controlled-request-harness-operator-review-acknowledgement-activation-command-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-gate.sh
)"

terminal_decision_report_sha256="$(printf '%s' "$TERMINAL_DECISION_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$TERMINAL_DECISION_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_gate"
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready == true
    and $source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready == true
    and $source.source_activation_command_result_receipt_final_operator_acknowledgement_report_sha256 != ""
    and $source.minimum_required_samples >= 24
    and $source.required_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count == 12
    and $source.ready_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count == 12
    and $source.side_effect_free_activation_command_result_receipt_terminal_operator_decision_public_claim_surface_count == 12
    and $source.required_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 10
    and $source.activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 10
    and $source.blocked_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 10
    and $source.allowed_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 0
    and $source.accepted_activation_command_result_receipt_terminal_operator_decision_public_claim_fixture_count == 0
    and $source.activation_command_result_receipt_terminal_operator_decision_recorded == false
    and $source.activation_command_result_receipt_terminal_operator_decision_persisted == false
    and $source.activation_command_result_receipt_terminal_operator_decision_materialized == false
    and $source.activation_command_result_receipt_terminal_operator_decision_filesystem_written == false
    and $source.activation_command_result_receipt_terminal_operator_decision_delivered == false
    and $source.activation_command_result_receipt_public_claim_recorded == false
    and $source.activation_command_result_receipt_public_claim_persisted == false
    and $source.activation_command_result_receipt_public_claim_materialized == false
    and $source.activation_command_result_receipt_public_claim_promoted == false
    and $source.activation_command_result_receipt_public_ga_claimed == false
    and $source.activation_command_result_receipt_public_release_published == false
    and $source.activation_command_result_receipt_public_distribution_performed == false
    and $source.activation_command_result_receipt_public_artifact_written == false
    and $source.telegram_send_performed == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.activation_command_result_receipt_recorded == false
    and $source.activation_command_result_receipt_persisted == false
    and $source.activation_command_result_receipt_accepted == false
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
    and $source.public_release_published == false
    and $source.public_ga_claimed == false
    and $source.release_artifact_written == false
    and $source.public_artifact_written == false
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
  --arg gate "hepta_memory_intelligence_kg_full_enablement_operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_gate" \
  --arg terminal_decision_report_sha256 "$terminal_decision_report_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson source "$TERMINAL_DECISION_JSON" \
  '
  def blocked_fixture($id; $status; $reason; $extra):
    {
      id:$id,
      release_artifact_publication_status:$status,
      source_terminal_operator_decision_present:true,
      source_terminal_operator_decision_ready:true,
      release_artifact_publication_requested:false,
      release_artifact_publication_allowed:false,
      release_artifact_publication_accepted:false,
      release_artifact_publication_recorded:false,
      release_artifact_publication_persisted:false,
      release_artifact_publication_materialized:false,
      release_artifact_filesystem_written:false,
      release_artifact_written:false,
      public_artifact_written:false,
      artifact_signature_accepted:false,
      artifact_notarization_accepted:false,
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
      receipt_recorded:false,
      receipt_persisted:false,
      receipt_accepted:false,
      receipt_materialized:false,
      completion_ack_recorded:false,
      activation_allowed:false,
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
      release_artifact_publication_noop_confirmed:true,
      reason:$reason
    } + $extra;
  [
    blocked_fixture("operator-canary-controlled-request-harness-activation-result-receipt-release-artifact-publication-missing-terminal-decision"; "blocked_noop"; "source_terminal_operator_decision_report_required"; {source_terminal_operator_decision_present:false, source_terminal_operator_decision_ready:false, release_artifact_publication_requested:true}),
    blocked_fixture("operator-canary-controlled-request-harness-activation-result-receipt-release-artifact-write-request"; "blocked_artifact_noop"; "release_artifact_write_denied"; {release_artifact_write_requested:true, release_artifact_publication_requested:true}),
    blocked_fixture("operator-canary-controlled-request-harness-activation-result-receipt-public-artifact-write-request"; "blocked_artifact_noop"; "public_artifact_write_denied"; {public_artifact_write_requested:true, release_artifact_publication_requested:true}),
    blocked_fixture("operator-canary-controlled-request-harness-activation-result-receipt-artifact-signature-notarization-request"; "blocked_artifact_noop"; "artifact_signature_notarization_acceptance_denied"; {artifact_signature_requested:true, artifact_notarization_requested:true, release_artifact_publication_requested:true}),
    blocked_fixture("operator-canary-controlled-request-harness-activation-result-receipt-publication-queue-request"; "blocked_publication_noop"; "publication_queue_enqueue_denied"; {publication_queue_enqueue_requested:true, publication_manifest_write_requested:true}),
    blocked_fixture("operator-canary-controlled-request-harness-activation-result-receipt-distribution-channel-request"; "blocked_distribution_noop"; "public_distribution_channel_delivery_denied"; {public_distribution_requested:true, telegram_delivery_requested:true, channel_delivery_requested:true, external_delivery_requested:true}),
    blocked_fixture("operator-canary-controlled-request-harness-activation-result-receipt-public-version-tag-request"; "blocked_release_noop"; "public_version_tag_release_promotion_denied"; {public_version_tag_requested:true, public_release_publish_requested:true, public_ga_claim_requested:true}),
    blocked_fixture("operator-canary-controlled-request-harness-activation-result-receipt-release-notes-changelog-request"; "blocked_artifact_noop"; "release_notes_changelog_materialization_denied"; {release_notes_materialization_requested:true, changelog_materialization_requested:true}),
    blocked_fixture("operator-canary-controlled-request-harness-activation-result-receipt-terminal-decision-as-release-approval"; "blocked_promotion_noop"; "terminal_operator_decision_is_not_release_approval"; {terminal_operator_decision_release_approval_requested:true, release_artifact_publication_requested:true}),
    blocked_fixture("operator-canary-controlled-request-harness-activation-result-receipt-release-publication-activation-memory-provider-install"; "blocked_promotion_noop"; "activation_memory_provider_install_restart_active_binary_publication_denied"; {activation_from_release_publication_requested:true, memory_write_publication_requested:true, provider_prompt_publication_requested:true, install_publication_requested:true, service_restart_publication_requested:true, active_binary_publication_requested:true})
  ] as $fixtures
  | {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    gate:$gate,
    activation_command_result_receipt_release_artifact_publication_mode:"operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial",
    source_activation_command_result_receipt_terminal_operator_decision_public_claim_gate:$source.gate,
    source_activation_command_result_receipt_terminal_operator_decision_public_claim_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready,
    source_activation_command_result_receipt_terminal_operator_decision_public_claim_report_sha256:$terminal_decision_report_sha256,
    source_activation_command_result_receipt_final_operator_acknowledgement_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready,
    source_activation_command_result_receipt_operator_facing_summary_briefing_ready:$source.operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready,
    minimum_required_samples:$min_long_soak_samples,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready:true,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready:true,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_final_operator_acknowledgement_non_acceptance_denial_ready:true,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_operator_facing_summary_briefing_non_persistence_denial_ready:true,
    operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_no_persistence_ready:true,
    required_activation_command_result_receipt_release_artifact_publication_surface_count:12,
    ready_activation_command_result_receipt_release_artifact_publication_surface_count:12,
    side_effect_free_activation_command_result_receipt_release_artifact_publication_surface_count:12,
    required_activation_command_result_receipt_release_artifact_publication_fixture_count:10,
    activation_command_result_receipt_release_artifact_publication_fixture_count:($fixtures | length),
    blocked_activation_command_result_receipt_release_artifact_publication_fixture_count:($fixtures | length),
    noop_activation_command_result_receipt_release_artifact_publication_fixture_count:($fixtures | length),
    allowed_activation_command_result_receipt_release_artifact_publication_fixture_count:0,
    accepted_activation_command_result_receipt_release_artifact_publication_fixture_count:0,
    activation_command_result_receipt_release_artifact_publication_performed_count:0,
    release_artifact_publication_allowed:false,
    release_artifact_publication_requested:false,
    release_artifact_publication_accepted:false,
    release_artifact_publication_recorded:false,
    release_artifact_publication_persisted:false,
    release_artifact_publication_materialized:false,
    release_artifact_filesystem_written:false,
    release_artifact_written:false,
    public_artifact_written:false,
    artifact_signature_accepted:false,
    artifact_notarization_accepted:false,
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
    activation_command_result_receipt_recorded:false,
    activation_command_result_receipt_persisted:false,
    activation_command_result_receipt_accepted:false,
    activation_command_result_receipt_materialized:false,
    activation_command_completion_ack_recorded:false,
    activation_allowed_by_release_artifact_publication:false,
    activation_allowed_by_terminal_operator_decision:false,
    activation_allowed_by_result_receipt:false,
    activation_allowed:false,
    activation_performed:false,
    live_mutation_execution_ready:false,
    live_mutation_execution_allowed:false,
    live_mutation_execution_performed:false,
    memory_write_execution_allowed:false,
    memory_write_execution_ready:false,
    memory_write_execution_performed:false,
    memory_store_write_path_enabled:false,
    memory_store_write_allowed:false,
    memory_store_write_performed:false,
    memory_store_write_performed_count:0,
    memory_store_mutation_allowed:false,
    memory_store_mutated:false,
    rollback_execution_allowed:false,
    rollback_executed:false,
    raw_payload_plaintext_recorded:false,
    raw_payload_plaintext_persisted:false,
    secret_material_read:false,
    provider_prompt_replay_enabled:false,
    provider_invoked:false,
    model_invoked:false,
    install_executed:false,
    launchd_mutated:false,
    service_restarted:false,
    active_binary_mutated:false,
    activation_command_result_receipt_release_artifact_publication_surfaces:[
      "source_terminal_operator_decision_report_required",
      "release_artifact_write_denied",
      "public_artifact_write_denied",
      "artifact_signature_notarization_acceptance_denied",
      "publication_queue_enqueue_denied",
      "publication_manifest_write_denied",
      "public_distribution_channel_delivery_denied",
      "public_version_tag_release_promotion_denied",
      "release_notes_changelog_materialization_denied",
      "terminal_operator_decision_is_not_release_approval",
      "activation_from_release_artifact_publication_denied",
      "external_public_install_restart_active_binary_publication_denied"
    ],
    activation_command_result_receipt_release_artifact_publication_fixtures:$fixtures,
    denied_by_activation_command_result_receipt_release_artifact_publication:[
      "source_terminal_operator_decision_report_required",
      "release_artifact_write_denied",
      "public_artifact_write_denied",
      "artifact_signature_notarization_acceptance_denied",
      "publication_queue_enqueue_denied",
      "publication_manifest_write_denied",
      "public_distribution_channel_delivery_denied",
      "public_version_tag_release_promotion_denied",
      "release_notes_changelog_materialization_denied",
      "terminal_operator_decision_is_not_release_approval",
      "activation_from_release_artifact_publication_denied",
      "memory_write_publication_denied",
      "provider_prompt_publication_denied",
      "install_restart_active_binary_publication_denied"
    ],
    side_effects:{
      release_artifact_publication_recorded:false,
      release_artifact_publication_persisted:false,
      release_artifact_publication_materialized:false,
      release_artifact_filesystem_written:false,
      release_artifact_written:false,
      public_artifact_written:false,
      artifact_signature_accepted:false,
      artifact_notarization_accepted:false,
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
      activation_command_result_receipt_terminal_operator_decision_recorded:false,
      activation_command_result_receipt_terminal_operator_decision_persisted:false,
      activation_command_result_receipt_public_claim_recorded:false,
      activation_command_result_receipt_public_claim_promoted:false,
      activation_command_result_receipt_recorded:false,
      activation_command_result_receipt_persisted:false,
      activation_command_result_receipt_accepted:false,
      activation_command_result_receipt_materialized:false,
      activation_command_completion_ack_recorded:false,
      activation_performed:false,
      live_mutation_execution_performed:false,
      memory_write_execution_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      rollback_executed:false,
      raw_payload_inspected:false,
      payload_plaintext_persisted:false,
      secret_file_read:false,
      credential_read:false,
      provider_invoked:false,
      model_invoked:false,
      provider_prompt_replayed:false,
      runtime_store_mutated:false,
      gateway_event_enqueued:false,
      capability_registry_mutated:false,
      plugin_registry_mutated:false,
      skill_workshop_written:false,
      filesystem_written:false,
      install_executed:false,
      active_binary_mutated:false,
      launchd_mutated:false,
      service_restarted:false
    }
  }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_release_artifact_publication_denial_ready == true
  and .operator_canary_controlled_request_harness_operator_review_acknowledgement_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready == true
  and .required_activation_command_result_receipt_release_artifact_publication_surface_count == 12
  and .ready_activation_command_result_receipt_release_artifact_publication_surface_count == 12
  and .side_effect_free_activation_command_result_receipt_release_artifact_publication_surface_count == 12
  and .required_activation_command_result_receipt_release_artifact_publication_fixture_count == 10
  and .activation_command_result_receipt_release_artifact_publication_fixture_count == 10
  and .blocked_activation_command_result_receipt_release_artifact_publication_fixture_count == 10
  and .noop_activation_command_result_receipt_release_artifact_publication_fixture_count == 10
  and .allowed_activation_command_result_receipt_release_artifact_publication_fixture_count == 0
  and .accepted_activation_command_result_receipt_release_artifact_publication_fixture_count == 0
  and .release_artifact_publication_recorded == false
  and .release_artifact_publication_persisted == false
  and .release_artifact_publication_materialized == false
  and .release_artifact_filesystem_written == false
  and .release_artifact_written == false
  and .public_artifact_written == false
  and .artifact_signature_accepted == false
  and .artifact_notarization_accepted == false
  and .publication_queue_enqueued == false
  and .publication_manifest_written == false
  and .public_distribution_performed == false
  and .public_release_published == false
  and .public_ga_claimed == false
  and .public_claim_promoted == false
  and .public_version_tag_created == false
  and .release_notes_materialized == false
  and .changelog_materialized == false
  and .terminal_operator_decision_promoted_to_release_approval == false
  and .telegram_send_performed == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .activation_command_result_receipt_recorded == false
  and .activation_command_result_receipt_persisted == false
  and .activation_command_result_receipt_accepted == false
  and .activation_command_completion_ack_recorded == false
  and .activation_allowed_by_release_artifact_publication == false
  and .activation_allowed == false
  and .activation_performed == false
  and .live_mutation_execution_performed == false
  and .memory_write_execution_performed == false
  and .memory_store_write_performed == false
  and .memory_store_write_performed_count == 0
  and .memory_store_mutated == false
  and .rollback_executed == false
  and .secret_material_read == false
  and .provider_invoked == false
  and .model_invoked == false
  and .install_executed == false
  and .launchd_mutated == false
  and .service_restarted == false
  and .active_binary_mutated == false
  and (.activation_command_result_receipt_release_artifact_publication_fixtures | length) == 10
  and (.activation_command_result_receipt_release_artifact_publication_fixtures | all((.release_artifact_publication_status == "blocked_noop" or .release_artifact_publication_status == "blocked_artifact_noop" or .release_artifact_publication_status == "blocked_publication_noop" or .release_artifact_publication_status == "blocked_distribution_noop" or .release_artifact_publication_status == "blocked_release_noop" or .release_artifact_publication_status == "blocked_promotion_noop") and .release_artifact_publication_allowed == false and .release_artifact_publication_accepted == false and .release_artifact_publication_recorded == false and .release_artifact_publication_persisted == false and .release_artifact_filesystem_written == false and .release_artifact_written == false and .public_artifact_written == false and .publication_queue_enqueued == false and .public_distribution_performed == false and .public_release_published == false and .public_ga_claimed == false and .public_claim_promoted == false and .telegram_send_performed == false and .external_send_performed == false and .receipt_recorded == false and .receipt_persisted == false and .receipt_accepted == false and .activation_allowed == false and .live_mutation_execution_performed == false and .memory_store_write_performed == false and .memory_store_mutated == false and .rollback_executed == false and .release_artifact_publication_noop_confirmed == true))
  and (.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full enablement operator canary controlled-request harness operator review acknowledgement activation command result receipt release artifact publication denial gate passed"
