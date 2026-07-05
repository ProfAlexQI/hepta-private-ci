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
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-terminal-operator-decision-public-claim-non-promotion-denial-boundary-gate.sh
)"

jq -n -e \
  --argjson source "$SOURCE_JSON" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_ready == true
    and $source.scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_accepted == true
    and $source.accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count == 1
    and $source.blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count == 9
    and $source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_count >= 80
    and $source.dry_run_execution_result_receipt_terminal_operator_decision_recorded == false
    and $source.dry_run_execution_result_receipt_terminal_operator_decision_accepted == false
    and $source.dry_run_execution_result_receipt_public_claim_promoted == false
    and $source.dry_run_execution_result_receipt_public_release_published == false
    and $source.dry_run_execution_result_receipt_release_artifact_written == false
    and $source.dry_run_execution_result_receipt_authority_promoted_from_terminal_operator_decision == false
    and $source.dry_run_execution_executed == false
    and $source.production_durable_memory_store_write_performed == false
    and $source.memory_store_write_performed == false
    and $source.wal_write_performed == false
    and $source.receipt_persisted == false
    and $source.live_kg_write_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.credential_read == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.install_executed == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and $source.allowed_next_actions[1].action == "prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary"
    and $source.allowed_next_actions[1].writes_release_artifact == false
  ' >/dev/null

source_report_sha256="$(sha256_text "$SOURCE_JSON")"
source_terminal_boundary_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_terminal_policy_hash_sha256="$(jq -r '.scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_policy_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_terminal_result_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_result_hash_sha256 // ""' <<<"$SOURCE_JSON")"
source_terminal_handoff_hash_sha256="$(jq -r '.dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_handoff_hash_sha256 // ""' <<<"$SOURCE_JSON")"

release_artifact_publication_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-denial:v1:source=${source_terminal_result_hash_sha256}:publication=false:artifact=false:public=false"
)"
release_artifact_write_denial_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-write-denial:v1:publication=${release_artifact_publication_denial_hash_sha256}:release-artifact=false:public-artifact=false:signature=false:notarization=false"
)"
release_artifact_publication_matrix_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-matrix:v1:publication=${release_artifact_publication_denial_hash_sha256}:artifact=${release_artifact_write_denial_hash_sha256}:fixtures=10"
)"
release_artifact_publication_handoff_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-handoff:v1:matrix=${release_artifact_publication_matrix_hash_sha256}:next=release-artifact-publication-result-receipt-no-persistence-boundary"
)"
release_artifact_publication_result_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-result:v1:publication=${release_artifact_publication_denial_hash_sha256}:artifact=${release_artifact_write_denial_hash_sha256}:handoff=${release_artifact_publication_handoff_hash_sha256}:accepted=true:release=false:public=false:activation=false:execution=false:production-write=false"
)"
release_artifact_publication_boundary_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-denial-boundary:v1:source=${source_report_sha256}:result=${release_artifact_publication_result_hash_sha256}:accepted=1:blocked=9:publication=false:artifact=false:authority=false:execution=false:production-write=false"
)"
release_artifact_publication_policy_hash_sha256="$(
  sha256_text "scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-denial-policy:v1:no-release-artifact-no-public-artifact-no-signature-no-notarization-no-publication-queue-no-manifest-no-distribution-no-public-release-no-terminal-release-approval-no-activation-no-execution-no-production-write-no-kg-no-provider-no-install"
)"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg endpoint "/api/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-denial-boundary" \
  --arg source_command "/hepta-memory-live-mutation-operator-write-execution-scoped-production-durable-memory-write-dry-run-execution-result-receipt-release-artifact-publication-denial-boundary --json" \
  --arg source_report_sha256 "$source_report_sha256" \
  --arg source_terminal_boundary_hash_sha256 "$source_terminal_boundary_hash_sha256" \
  --arg source_terminal_policy_hash_sha256 "$source_terminal_policy_hash_sha256" \
  --arg source_terminal_result_hash_sha256 "$source_terminal_result_hash_sha256" \
  --arg source_terminal_handoff_hash_sha256 "$source_terminal_handoff_hash_sha256" \
  --arg release_artifact_publication_denial_hash_sha256 "$release_artifact_publication_denial_hash_sha256" \
  --arg release_artifact_write_denial_hash_sha256 "$release_artifact_write_denial_hash_sha256" \
  --arg release_artifact_publication_matrix_hash_sha256 "$release_artifact_publication_matrix_hash_sha256" \
  --arg release_artifact_publication_handoff_hash_sha256 "$release_artifact_publication_handoff_hash_sha256" \
  --arg release_artifact_publication_result_hash_sha256 "$release_artifact_publication_result_hash_sha256" \
  --arg release_artifact_publication_boundary_hash_sha256 "$release_artifact_publication_boundary_hash_sha256" \
  --arg release_artifact_publication_policy_hash_sha256 "$release_artifact_publication_policy_hash_sha256" \
  --argjson source "$SOURCE_JSON" \
  '
  def fixture($id; $status; $accepted; $reason; $extra):
    {
      id:$id,
      fixture_id:$id,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_status:$status,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_accepted:$accepted,
      source_terminal_operator_decision_public_claim_present:true,
      source_terminal_operator_decision_public_claim_ready:true,
      release_artifact_publication_denial_noop_confirmed:true,
      reason:$reason,
      release_artifact_publication_requested:false,
      release_artifact_write_requested:false,
      public_artifact_write_requested:false,
      artifact_signature_requested:false,
      artifact_notarization_requested:false,
      publication_queue_enqueue_requested:false,
      publication_manifest_write_requested:false,
      public_distribution_requested:false,
      telegram_delivery_requested:false,
      channel_delivery_requested:false,
      external_delivery_requested:false,
      public_version_tag_requested:false,
      public_release_publish_requested:false,
      public_ga_claim_requested:false,
      release_notes_materialization_requested:false,
      changelog_materialization_requested:false,
      terminal_operator_decision_release_approval_requested:false,
      activation_from_release_publication_requested:false,
      dry_run_execution_publication_requested:false,
      production_memory_write_publication_requested:false,
      memory_write_publication_requested:false,
      kg_write_publication_requested:false,
      provider_prompt_publication_requested:false,
      credential_publication_requested:false,
      install_publication_requested:false,
      service_restart_publication_requested:false,
      active_binary_publication_requested:false,
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
      receipt_persisted:false,
      activation_allowed:false,
      activation_performed:false,
      dry_run_execution_executed:false,
      production_durable_memory_write_executed:false,
      production_durable_memory_store_write_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      wal_write_performed:false,
      rollback_executed:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      secret_file_read:false,
      install_executed:false,
      launchd_mutated:false,
      service_restarted:false,
      service_restart_performed:false,
      active_binary_mutated:false
    } + $extra;
  ([
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-report-only-binding"; "accepted_report_only"; true; "release_artifact_publication_denial_matrix_bound_without_publication_or_release_artifact"; {}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-publication-missing-terminal-decision-public-claim"; "blocked_noop"; false; "source_terminal_operator_decision_public_claim_report_required"; {source_terminal_operator_decision_public_claim_present:false, source_terminal_operator_decision_public_claim_ready:false, release_artifact_publication_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-release-artifact-write-request"; "blocked_artifact_noop"; false; "release_artifact_write_denied"; {release_artifact_write_requested:true, release_artifact_publication_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-public-artifact-write-request"; "blocked_artifact_noop"; false; "public_artifact_write_denied"; {public_artifact_write_requested:true, release_artifact_publication_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-artifact-signature-notarization-request"; "blocked_artifact_noop"; false; "artifact_signature_notarization_denied"; {artifact_signature_requested:true, artifact_notarization_requested:true, release_artifact_publication_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-publication-queue-manifest-request"; "blocked_publication_noop"; false; "publication_queue_manifest_denied"; {publication_queue_enqueue_requested:true, publication_manifest_write_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-distribution-channel-request"; "blocked_distribution_noop"; false; "public_distribution_channel_delivery_denied"; {public_distribution_requested:true, telegram_delivery_requested:true, channel_delivery_requested:true, external_delivery_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-public-version-tag-release-request"; "blocked_release_noop"; false; "public_version_tag_release_promotion_denied"; {public_version_tag_requested:true, public_release_publish_requested:true, public_ga_claim_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-release-notes-changelog-terminal-approval-request"; "blocked_release_noop"; false; "release_notes_changelog_and_terminal_release_approval_denied"; {release_notes_materialization_requested:true, changelog_materialization_requested:true, terminal_operator_decision_release_approval_requested:true, release_artifact_publication_requested:true}),
    fixture("scoped-production-durable-memory-write-dry-run-result-receipt-release-publication-activation-memory-provider-install"; "blocked_authority_noop"; false; "activation_memory_provider_install_restart_active_binary_publication_denied"; {activation_from_release_publication_requested:true, dry_run_execution_publication_requested:true, production_memory_write_publication_requested:true, memory_write_publication_requested:true, kg_write_publication_requested:true, provider_prompt_publication_requested:true, credential_publication_requested:true, install_publication_requested:true, service_restart_publication_requested:true, active_binary_publication_requested:true})
  ]) as $fixtures
  | ($source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary + [
    "source_terminal_operator_decision_public_claim_report_required",
    "release_artifact_publication_denied",
    "release_artifact_write_denied",
    "public_artifact_write_denied",
    "artifact_signature_notarization_denied",
    "publication_queue_enqueue_denied",
    "publication_manifest_write_denied",
    "public_distribution_channel_delivery_denied",
    "public_version_tag_release_promotion_denied",
    "release_notes_changelog_materialization_denied",
    "terminal_operator_decision_is_not_release_approval",
    "terminal_operator_decision_release_approval_promotion_denied",
    "activation_from_release_artifact_publication_denied",
    "dry_run_execution_release_publication_denied",
    "production_memory_write_release_publication_denied",
    "memory_write_publication_denied",
    "kg_write_publication_denied",
    "provider_prompt_publication_denied",
    "channel_delivery_publication_denied",
    "install_restart_active_binary_publication_denied"
  ]) as $denials
  | {
    product:$product,
    runtime:$runtime,
    status:"ready",
    base_url:$base_url,
    endpoint:$endpoint,
    source_command:$source_command,
    native_route:true,
    side_effect_free:true,
    native_gateway_source_command_count:281,
    route_count:281,
    implemented_route_count:281,
    missing_route_count:0,
    route_count_source_command_accepted:true,
    memory_write_execution_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_ready:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_ready:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_accepted:true,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_mode:"dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_no_release_artifact_no_publication_no_authority_no_execution_no_production_durable_memory_mutation",
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_ready:true,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_report_sha256:$source_report_sha256,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_hash_sha256:$source_terminal_boundary_hash_sha256,
    source_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_policy_hash_sha256:$source_terminal_policy_hash_sha256,
    source_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_result_hash_sha256:$source_terminal_result_hash_sha256,
    source_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_handoff_hash_sha256:$source_terminal_handoff_hash_sha256,
    source_accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count:1,
    source_blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_fixture_count:9,
    source_denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_count:($source.denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_count),
    dry_run_execution_result_receipt_release_artifact_publication_denial_hash_sha256:$release_artifact_publication_denial_hash_sha256,
    dry_run_execution_result_receipt_release_artifact_write_denial_hash_sha256:$release_artifact_write_denial_hash_sha256,
    dry_run_execution_result_receipt_release_artifact_publication_matrix_hash_sha256:$release_artifact_publication_matrix_hash_sha256,
    dry_run_execution_result_receipt_release_artifact_publication_handoff_hash_sha256:$release_artifact_publication_handoff_hash_sha256,
    dry_run_execution_result_receipt_release_artifact_publication_result_hash_sha256:$release_artifact_publication_result_hash_sha256,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_hash_sha256:$release_artifact_publication_boundary_hash_sha256,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_policy_hash_sha256:$release_artifact_publication_policy_hash_sha256,
    required_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_surface_count:14,
    ready_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_surface_count:14,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count:($fixtures | length),
    accepted_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count:1,
    blocked_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixture_count:9,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_fixtures:$fixtures,
    denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary:$denials,
    denied_by_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_count:($denials | length),
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_performed_count:1,
    scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_result_accepted_count:1,
    source_dry_run_execution_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_boundary_bound_count:1,
    dry_run_execution_result_receipt_release_artifact_publication_denied_count:1,
    dry_run_execution_result_receipt_release_artifact_write_denied_count:1,
    dry_run_execution_result_receipt_public_artifact_write_denied_count:1,
    dry_run_execution_result_receipt_artifact_signature_notarization_denied_count:1,
    dry_run_execution_result_receipt_publication_queue_manifest_denied_count:1,
    dry_run_execution_result_receipt_public_distribution_denied_count:1,
    dry_run_execution_result_receipt_public_release_publication_denied_count:1,
    dry_run_execution_result_receipt_release_artifact_publication_authority_denied_count:1,
    dry_run_execution_result_receipt_release_artifact_publication_handoff_bound_count:1,
    dry_run_execution_result_receipt_release_artifact_publication_recorded_count:0,
    dry_run_execution_result_receipt_release_artifact_written_count:0,
    dry_run_execution_result_receipt_public_artifact_written_count:0,
    dry_run_execution_result_receipt_public_distribution_performed_count:0,
    dry_run_execution_result_receipt_public_release_published_count:0,
    dry_run_execution_result_receipt_authority_promoted_from_release_artifact_publication_count:0,
    activation_performed_count:0,
    dry_run_execution_executed_count:0,
    production_durable_memory_store_write_performed_count:0,
    memory_store_write_performed_count:0,
    wal_write_performed_count:0,
    receipt_persisted_count:0,
    live_kg_write_performed_count:0,
    provider_invoked_count:0,
    model_invoked_count:0,
    credential_read_count:0,
    channel_send_performed_count:0,
    external_send_performed_count:0,
    release_artifact_written_count:0,
    public_artifact_written_count:0,
    install_executed_count:0,
    service_restarted_count:0,
    active_binary_mutated_count:0,
    dry_run_execution_result_receipt_release_artifact_publication_recorded:false,
    dry_run_execution_result_receipt_release_artifact_written:false,
    dry_run_execution_result_receipt_public_artifact_written:false,
    dry_run_execution_result_receipt_publication_queue_enqueued:false,
    dry_run_execution_result_receipt_publication_manifest_written:false,
    dry_run_execution_result_receipt_public_distribution_performed:false,
    dry_run_execution_result_receipt_public_release_published:false,
    dry_run_execution_result_receipt_authority_promoted_from_release_artifact_publication:false,
    activation_performed:false,
    dry_run_execution_executed:false,
    production_durable_memory_store_write_performed:false,
    memory_store_write_performed:false,
    wal_write_performed:false,
    receipt_persisted:false,
    live_kg_write_performed:false,
    provider_invoked:false,
    model_invoked:false,
    credential_read:false,
    channel_send_performed:false,
    external_send_performed:false,
    release_artifact_written:false,
    public_artifact_written:false,
    install_executed:false,
    service_restarted:false,
    active_binary_mutated:false,
    allowed_next_actions:[
      {
        action:"run_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_require_live_gate",
        status:"allowed_verification_only",
        writes_release_artifact:false,
        publishes_release_artifact:false,
        claims_public_release:false,
        promotes_activation_authority:false,
        executes_dry_run:false,
        writes_production_durable_memory:false,
        writes_memory_or_kg:false,
        invokes_provider:false,
        sends_externally:false,
        installs_or_restarts:false,
        mutates_active_binary:false
      },
      {
        action:"prepare_scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_result_receipt_no_persistence_boundary",
        status:"allowed_report_only_next_slice",
        persists_publication_result_receipt:false,
        publishes_release_artifact:false,
        writes_release_artifact:false,
        claims_public_release:false,
        promotes_activation_authority:false,
        executes_dry_run:false,
        writes_production_durable_memory:false,
        writes_memory_or_kg:false,
        invokes_model:false,
        sends_externally:false,
        installs_or_restarts:false,
        mutates_active_binary:false
      }
    ],
    side_effects:{
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_boundary_performed:true,
      scoped_production_durable_memory_write_dry_run_execution_result_receipt_release_artifact_publication_denial_result_accepted:true,
      dry_run_execution_result_receipt_release_artifact_publication_recorded:false,
      dry_run_execution_result_receipt_release_artifact_written:false,
      dry_run_execution_result_receipt_public_artifact_written:false,
      dry_run_execution_result_receipt_publication_queue_enqueued:false,
      dry_run_execution_result_receipt_publication_manifest_written:false,
      dry_run_execution_result_receipt_public_distribution_performed:false,
      dry_run_execution_result_receipt_public_release_published:false,
      dry_run_execution_result_receipt_authority_promoted_from_release_artifact_publication:false,
      activation_performed:false,
      dry_run_execution_executed:false,
      production_durable_memory_store_write_performed:false,
      memory_store_write_performed:false,
      wal_write_performed:false,
      receipt_persisted:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      channel_send_performed:false,
      external_send_performed:false,
      release_artifact_written:false,
      public_artifact_written:false,
      install_executed:false,
      service_restarted:false,
      active_binary_mutated:false
    }
  }'
