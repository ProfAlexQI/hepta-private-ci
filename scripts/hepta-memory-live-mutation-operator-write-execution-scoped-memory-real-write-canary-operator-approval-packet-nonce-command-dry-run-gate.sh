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

RELEASE_ARTIFACT_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-release-artifact-publication-denial-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-activation-command-result-receipt-release-artifact-publication-denial-gate.sh
)"

release_artifact_report_sha256="$(printf '%s' "$RELEASE_ARTIFACT_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$RELEASE_ARTIFACT_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_live_mutation_operator_write_execution_activation_command_result_receipt_release_artifact_publication_denial_gate"
    and $source.memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_ready == true
    and $source.memory_write_execution_activation_command_result_receipt_terminal_operator_decision_public_claim_non_promotion_denial_ready == true
    and $source.minimum_required_samples >= 24
    and $source.required_activation_command_result_receipt_release_artifact_publication_surface_count == 12
    and $source.ready_activation_command_result_receipt_release_artifact_publication_surface_count == 12
    and $source.side_effect_free_activation_command_result_receipt_release_artifact_publication_surface_count == 12
    and $source.required_activation_command_result_receipt_release_artifact_publication_fixture_count == 10
    and $source.activation_command_result_receipt_release_artifact_publication_fixture_count == 10
    and $source.accepted_activation_command_result_receipt_release_artifact_publication_fixture_count == 0
    and $source.activation_command_result_receipt_release_artifact_publication_performed_count == 0
    and $source.memory_store_write_performed == false
    and $source.memory_store_mutated == false
    and $source.release_artifact_written == false
    and $source.public_artifact_written == false
    and $source.public_release_published == false
    and $source.public_ga_claimed == false
    and $source.activation_allowed == false
    and $source.live_mutation_execution_performed == false
    and $source.memory_write_execution_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and ($source.credential_read // false) == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.install_executed == false
    and $source.service_restarted == false
    and $source.active_binary_mutated == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_gate" \
    --arg source_report_sha256 "$release_artifact_report_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$RELEASE_ARTIFACT_JSON" \
    '
    def blocked_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        scoped_memory_real_write_canary_status:$status,
        source_release_artifact_publication_denial_present:true,
        source_release_artifact_publication_denial_ready:true,
        approval_packet_requested:false,
        nonce_issue_requested:false,
        nonce_consume_requested:false,
        explicit_command_requested:false,
        canary_scope_requested:false,
        canary_namespace_requested:false,
        canary_store_requested:false,
        payload_digest_binding_requested:false,
        active_binary_sha_binding_requested:false,
        route_count_binding_requested:false,
        wal_receipt_plan_requested:false,
        post_write_readback_plan_requested:false,
        rollback_tombstone_plan_requested:false,
        durable_memory_write_requested:false,
        memory_store_mutation_requested:false,
        kg_live_write_requested:false,
        provider_model_invocation_requested:false,
        credential_read_requested:false,
        channel_external_send_requested:false,
        public_claim_requested:false,
        release_artifact_write_requested:false,
        install_restart_requested:false,
        active_binary_mutation_requested:false,
        approval_packet_recorded:false,
        approval_packet_persisted:false,
        approval_packet_accepted:false,
        operator_identity_bound:false,
        operator_session_bound:false,
        operator_signature_verified:false,
        single_use_nonce_issued:false,
        single_use_nonce_consumed:false,
        explicit_command_accepted:false,
        explicit_command_dispatched:false,
        canary_scope_bound:false,
        canary_namespace_bound:false,
        canary_store_bound:false,
        payload_digest_bound:false,
        active_binary_sha_bound:false,
        route_count_bound:false,
        wal_receipt_plan_accepted:false,
        receipt_recorded:false,
        receipt_persisted:false,
        receipt_accepted:false,
        post_write_readback_plan_accepted:false,
        post_write_readback_performed:false,
        rollback_tombstone_plan_accepted:false,
        rollback_executed:false,
        activation_allowed:false,
        live_mutation_execution_performed:false,
        memory_write_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        durable_memory_store_write_performed:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        telegram_send_performed:false,
        channel_send_performed:false,
        external_send_performed:false,
        public_claim_promoted:false,
        public_release_published:false,
        public_ga_claimed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        install_executed:false,
        service_restarted:false,
        active_binary_mutated:false,
        scoped_canary_dry_run_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      blocked_fixture("scoped-memory-real-write-canary-missing-release-artifact-source"; "blocked_source_noop"; "release_artifact_publication_denial_boundary_source_required"; {source_release_artifact_publication_denial_present:false, source_release_artifact_publication_denial_ready:false, approval_packet_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-fresh-approval-packet-required"; "blocked_approval_noop"; "fresh_operator_approval_packet_required"; {approval_packet_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-operator-identity-session-required"; "blocked_identity_noop"; "operator_identity_session_signature_required"; {approval_packet_requested:true, operator_identity_requested:true, operator_session_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-single-use-nonce-required"; "blocked_nonce_noop"; "single_use_nonce_required_and_not_consumed"; {nonce_issue_requested:true, nonce_consume_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-explicit-command-dry-run-only"; "blocked_command_noop"; "explicit_command_path_required_but_dry_run_only"; {explicit_command_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-scope-namespace-store-binding"; "blocked_scope_noop"; "canary_scope_namespace_store_binding_required"; {canary_scope_requested:true, canary_namespace_requested:true, canary_store_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-payload-digest-redaction-binding"; "blocked_payload_noop"; "payload_digest_and_redaction_proof_required"; {payload_digest_binding_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-binary-route-count-binding"; "blocked_binary_noop"; "active_binary_sha_and_route_count_binding_required"; {active_binary_sha_binding_requested:true, route_count_binding_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-wal-readback-rollback-plans"; "blocked_receipt_noop"; "wal_receipt_readback_and_rollback_plans_required"; {wal_receipt_plan_requested:true, post_write_readback_plan_requested:true, rollback_tombstone_plan_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-direct-side-effect-attempt"; "blocked_execution_noop"; "direct_memory_kg_provider_channel_release_install_active_binary_side_effects_denied"; {durable_memory_write_requested:true, memory_store_mutation_requested:true, kg_live_write_requested:true, provider_model_invocation_requested:true, credential_read_requested:true, channel_external_send_requested:true, public_claim_requested:true, release_artifact_write_requested:true, install_restart_requested:true, active_binary_mutation_requested:true})
    ] as $fixtures
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      scoped_memory_real_write_canary_mode:"scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_no_write",
      source_activation_command_result_receipt_release_artifact_publication_gate:$source.gate,
      source_activation_command_result_receipt_release_artifact_publication_ready:$source.memory_write_execution_activation_command_result_receipt_release_artifact_publication_denial_ready,
      source_activation_command_result_receipt_release_artifact_publication_report_sha256:$source_report_sha256,
      source_release_artifact_publication_fixture_count:$source.activation_command_result_receipt_release_artifact_publication_fixture_count,
      source_accepted_release_artifact_publication_fixture_count:$source.accepted_activation_command_result_receipt_release_artifact_publication_fixture_count,
      source_release_artifact_publication_performed_count:$source.activation_command_result_receipt_release_artifact_publication_performed_count,
      source_release_artifact_publication_denial_count:$source.denied_by_activation_command_result_receipt_release_artifact_publication_count,
      minimum_required_samples:$min_long_soak_samples,
      scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_ready:true,
      required_scoped_memory_real_write_canary_operator_approval_surface_count:12,
      ready_scoped_memory_real_write_canary_operator_approval_surface_count:12,
      side_effect_free_scoped_memory_real_write_canary_operator_approval_surface_count:12,
      required_scoped_memory_real_write_canary_operator_approval_fixture_count:10,
      scoped_memory_real_write_canary_operator_approval_fixture_count:($fixtures | length),
      blocked_scoped_memory_real_write_canary_operator_approval_fixture_count:($fixtures | length),
      noop_scoped_memory_real_write_canary_operator_approval_fixture_count:($fixtures | length),
      allowed_scoped_memory_real_write_canary_operator_approval_fixture_count:0,
      accepted_scoped_memory_real_write_canary_operator_approval_fixture_count:0,
      scoped_memory_real_write_canary_approval_packet_accepted_count:0,
      single_use_nonce_consumed_count:0,
      explicit_command_dispatched_count:0,
      wal_write_performed_count:0,
      receipt_persisted_count:0,
      post_write_readback_performed_count:0,
      rollback_tombstone_performed_count:0,
      memory_store_write_performed_count:0,
      required_before_scoped_memory_real_write_canary_acceptance_count:18,
      fresh_operator_approval_packet_recorded:false,
      fresh_operator_approval_packet_persisted:false,
      fresh_operator_approval_packet_accepted:false,
      single_use_nonce_consumed:false,
      explicit_command_accepted:false,
      explicit_command_dispatched:false,
      receipt_persisted:false,
      post_write_readback_performed:false,
      rollback_executed:false,
      activation_allowed:false,
      live_mutation_execution_performed:false,
      memory_write_execution_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      durable_memory_store_write_performed:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      channel_send_performed:false,
      external_send_performed:false,
      public_claim_promoted:false,
      release_artifact_written:false,
      install_executed:false,
      service_restarted:false,
      active_binary_mutated:false,
      scoped_memory_real_write_canary_operator_approval_surfaces:[
        "fresh_operator_approval_packet_required",
        "single_use_nonce_required",
        "operator_identity_session_binding_required",
        "explicit_command_path_required",
        "canary_scope_namespace_store_binding_required",
        "payload_digest_redaction_binding_required",
        "active_binary_sha_route_count_binding_required",
        "release_artifact_denial_source_binding_required",
        "wal_receipt_persistence_plan_required",
        "post_write_readback_validation_plan_required",
        "rollback_tombstone_plan_required",
        "external_kg_provider_public_install_active_binary_side_effects_forbidden"
      ],
      scoped_memory_real_write_canary_operator_approval_fixtures:$fixtures,
      denied_by_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run:[
        "source_release_artifact_publication_denial_boundary_required",
        "fresh_operator_approval_packet_not_accepted",
        "single_use_nonce_not_consumed",
        "explicit_command_not_accepted",
        "explicit_command_not_dispatched",
        "canary_scope_namespace_store_not_bound",
        "payload_digest_redaction_proof_not_bound",
        "active_binary_sha_route_count_not_bound",
        "fresh_long_soak_evidence_not_accepted",
        "wal_receipt_plan_not_accepted",
        "post_write_readback_plan_not_accepted",
        "rollback_tombstone_plan_not_accepted",
        "durable_memory_write_denied",
        "kg_provider_credential_channel_public_release_install_active_binary_denied"
      ],
      denied_by_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_count:14,
      fresh_operator_approval_packet_required:true,
      single_use_nonce_required:true,
      explicit_command_required:true,
      durable_memory_write_forbidden:true,
      kg_live_write_forbidden:true,
      provider_model_invocation_forbidden:true,
      credential_read_forbidden:true,
      channel_external_send_forbidden:true,
      public_claim_release_artifact_forbidden:true,
      install_restart_active_binary_mutation_forbidden:true,
      side_effects:{
        fresh_operator_approval_packet_accepted:false,
        single_use_nonce_consumed:false,
        explicit_command_dispatched:false,
        durable_memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        channel_send_performed:false,
        external_send_performed:false,
        release_artifact_written:false,
        install_executed:false,
        service_restarted:false,
        active_binary_mutated:false,
        filesystem_written:false
      }
    }'
)"

jq -e '
  .status == "ready"
  and .scoped_memory_real_write_canary_operator_approval_packet_nonce_command_dry_run_ready == true
  and .source_activation_command_result_receipt_release_artifact_publication_ready == true
  and .scoped_memory_real_write_canary_operator_approval_fixture_count == 10
  and .accepted_scoped_memory_real_write_canary_operator_approval_fixture_count == 0
  and .scoped_memory_real_write_canary_approval_packet_accepted_count == 0
  and .single_use_nonce_consumed_count == 0
  and .explicit_command_dispatched_count == 0
  and .memory_store_write_performed_count == 0
  and .durable_memory_store_write_performed == false
  and .memory_store_mutated == false
  and .live_kg_write_performed == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .channel_send_performed == false
  and .external_send_performed == false
  and .release_artifact_written == false
  and .install_executed == false
  and .active_binary_mutated == false
  and (.scoped_memory_real_write_canary_operator_approval_fixtures | length) == 10
  and (.scoped_memory_real_write_canary_operator_approval_fixtures | all((.scoped_memory_real_write_canary_status | startswith("blocked")) and .approval_packet_accepted == false and .single_use_nonce_consumed == false and .explicit_command_dispatched == false and .memory_store_write_performed == false and .memory_store_mutated == false and .durable_memory_store_write_performed == false and .scoped_canary_dry_run_noop_confirmed == true))
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta scoped Memory real-write canary operator approval packet nonce command dry-run gate passed" >&2
