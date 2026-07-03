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

READBACK_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-readback-validation-dry-run-gate" \
    scripts/hepta-memory-live-mutation-operator-write-execution-scoped-memory-real-write-canary-readback-validation-dry-run-gate.sh
)"

source_report_sha256="$(printf '%s' "$READBACK_JSON" | shasum -a 256 | awk '{print $1}')"

jq -n -e \
  --argjson source "$READBACK_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_readback_validation_dry_run_gate"
    and $source.scoped_memory_real_write_canary_readback_validation_dry_run_ready == true
    and $source.source_scoped_memory_real_write_canary_operator_approval_packet_nonce_command_ready == true
    and $source.scoped_memory_real_write_canary_readback_fixture_count == 10
    and $source.accepted_scoped_memory_real_write_canary_readback_fixture_count == 0
    and $source.readback_performed_count == 0
    and $source.readback_result_accepted_count == 0
    and $source.durable_memory_store_read_performed_count == 0
    and $source.memory_store_write_performed_count == 0
    and $source.rollback_tombstone_handoff_accepted_count == 0
    and $source.post_write_readback_performed == false
    and $source.readback_result_accepted == false
    and $source.rollback_tombstone_handoff_accepted == false
    and $source.durable_memory_store_read_performed == false
    and $source.durable_memory_store_write_performed == false
    and $source.durable_memory_store_rollback_performed == false
    and $source.memory_store_mutated == false
    and $source.live_kg_write_performed == false
    and $source.provider_invoked == false
    and $source.model_invoked == false
    and $source.credential_read == false
    and $source.channel_send_performed == false
    and $source.external_send_performed == false
    and $source.release_artifact_written == false
    and $source.install_executed == false
    and $source.active_binary_mutated == false
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_live_mutation_operator_write_execution_scoped_memory_real_write_canary_rollback_tombstone_dry_run_gate" \
    --arg source_report_sha256 "$source_report_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$READBACK_JSON" \
    '
    def blocked_fixture($id; $status; $reason; $extra):
      {
        id:$id,
        scoped_memory_real_write_canary_rollback_tombstone_status:$status,
        source_readback_validation_dry_run_present:true,
        source_readback_validation_dry_run_ready:true,
        readback_result_acceptance_requested:false,
        rollback_plan_requested:false,
        tombstone_plan_requested:false,
        rollback_target_binding_requested:false,
        rollback_receipt_linkage_requested:false,
        rollback_idempotency_guard_requested:false,
        rollback_ordering_guard_requested:false,
        rollback_audit_evidence_requested:false,
        operator_review_handoff_requested:false,
        minimal_real_write_handoff_requested:false,
        durable_memory_read_requested:false,
        durable_memory_write_requested:false,
        durable_memory_rollback_requested:false,
        memory_store_mutation_requested:false,
        rollback_execution_requested:false,
        tombstone_write_requested:false,
        compensating_memory_write_requested:false,
        kg_live_write_requested:false,
        provider_model_invocation_requested:false,
        credential_read_requested:false,
        channel_external_send_requested:false,
        public_claim_requested:false,
        release_artifact_write_requested:false,
        install_restart_requested:false,
        active_binary_mutation_requested:false,
        readback_result_accepted:false,
        rollback_tombstone_handoff_accepted:false,
        rollback_plan_accepted:false,
        rollback_tombstone_plan_accepted:false,
        rollback_target_bound:false,
        rollback_receipt_linked:false,
        rollback_idempotency_guard_accepted:false,
        rollback_ordering_guard_accepted:false,
        rollback_audit_evidence_recorded:false,
        rollback_audit_evidence_persisted:false,
        operator_review_handoff_accepted:false,
        minimal_real_write_handoff_accepted:false,
        rollback_executed:false,
        rollback_result_recorded:false,
        rollback_result_persisted:false,
        rollback_result_accepted:false,
        tombstone_written:false,
        compensating_memory_write_performed:false,
        activation_allowed:false,
        memory_write_execution_performed:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        durable_memory_store_write_performed:false,
        durable_memory_store_read_performed:false,
        durable_memory_store_rollback_performed:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        channel_send_performed:false,
        external_send_performed:false,
        release_artifact_written:false,
        install_executed:false,
        active_binary_mutated:false,
        scoped_canary_rollback_tombstone_dry_run_noop_confirmed:true,
        reason:$reason
      } + $extra;
    [
      blocked_fixture("scoped-memory-real-write-canary-rollback-missing-readback-source"; "blocked_source_noop"; "readback_validation_dry_run_source_required"; {source_readback_validation_dry_run_present:false, source_readback_validation_dry_run_ready:false, rollback_plan_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-rollback-readback-acceptance-required"; "blocked_readback_acceptance_noop"; "readback_result_acceptance_required_before_rollback_plan"; {readback_result_acceptance_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-rollback-plan-required"; "blocked_rollback_plan_noop"; "rollback_plan_required_before_real_write_canary"; {rollback_plan_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-tombstone-plan-required"; "blocked_tombstone_plan_noop"; "tombstone_plan_required_before_real_write_canary"; {tombstone_plan_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-rollback-target-binding-required"; "blocked_target_binding_noop"; "rollback_target_binding_required_before_real_write_canary"; {rollback_target_binding_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-rollback-receipt-linkage-required"; "blocked_receipt_linkage_noop"; "rollback_receipt_linkage_required_before_real_write_canary"; {rollback_receipt_linkage_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-rollback-idempotency-ordering-guards-required"; "blocked_guard_noop"; "rollback_idempotency_and_ordering_guards_required"; {rollback_idempotency_guard_requested:true, rollback_ordering_guard_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-rollback-audit-evidence-required"; "blocked_audit_noop"; "rollback_audit_evidence_required_before_real_write_canary"; {rollback_audit_evidence_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-rollback-operator-review-and-minimal-handoff-required"; "blocked_handoff_noop"; "operator_review_and_minimal_real_write_handoff_required"; {operator_review_handoff_requested:true, minimal_real_write_handoff_requested:true}),
      blocked_fixture("scoped-memory-real-write-canary-rollback-direct-side-effect-attempt"; "blocked_execution_noop"; "direct_rollback_tombstone_memory_kg_provider_channel_release_install_active_binary_side_effects_denied"; {durable_memory_read_requested:true, durable_memory_write_requested:true, durable_memory_rollback_requested:true, memory_store_mutation_requested:true, rollback_execution_requested:true, tombstone_write_requested:true, compensating_memory_write_requested:true, kg_live_write_requested:true, provider_model_invocation_requested:true, credential_read_requested:true, channel_external_send_requested:true, public_claim_requested:true, release_artifact_write_requested:true, install_restart_requested:true, active_binary_mutation_requested:true})
    ] as $fixtures
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      scoped_memory_real_write_canary_mode:"scoped_memory_real_write_canary_rollback_tombstone_dry_run_no_rollback_no_write",
      source_scoped_memory_real_write_canary_readback_validation_gate:$source.gate,
      source_scoped_memory_real_write_canary_readback_validation_dry_run_ready:$source.scoped_memory_real_write_canary_readback_validation_dry_run_ready,
      source_scoped_memory_real_write_canary_readback_validation_report_sha256:$source_report_sha256,
      source_scoped_memory_real_write_canary_readback_fixture_count:$source.scoped_memory_real_write_canary_readback_fixture_count,
      source_accepted_scoped_memory_real_write_canary_readback_fixture_count:$source.accepted_scoped_memory_real_write_canary_readback_fixture_count,
      source_readback_performed_count:$source.readback_performed_count,
      source_readback_result_accepted_count:$source.readback_result_accepted_count,
      source_durable_memory_store_read_performed_count:$source.durable_memory_store_read_performed_count,
      source_memory_store_write_performed_count:$source.memory_store_write_performed_count,
      source_rollback_tombstone_handoff_accepted_count:$source.rollback_tombstone_handoff_accepted_count,
      minimum_required_samples:$min_long_soak_samples,
      scoped_memory_real_write_canary_rollback_tombstone_dry_run_ready:true,
      required_scoped_memory_real_write_canary_rollback_tombstone_surface_count:12,
      ready_scoped_memory_real_write_canary_rollback_tombstone_surface_count:12,
      side_effect_free_scoped_memory_real_write_canary_rollback_tombstone_surface_count:12,
      required_scoped_memory_real_write_canary_rollback_tombstone_fixture_count:10,
      scoped_memory_real_write_canary_rollback_tombstone_fixture_count:($fixtures | length),
      blocked_scoped_memory_real_write_canary_rollback_tombstone_fixture_count:($fixtures | length),
      noop_scoped_memory_real_write_canary_rollback_tombstone_fixture_count:($fixtures | length),
      allowed_scoped_memory_real_write_canary_rollback_tombstone_fixture_count:0,
      accepted_scoped_memory_real_write_canary_rollback_tombstone_fixture_count:0,
      rollback_plan_accepted_count:0,
      rollback_tombstone_plan_accepted_count:0,
      rollback_target_bound_count:0,
      rollback_receipt_linked_count:0,
      rollback_ordering_guard_accepted_count:0,
      rollback_idempotency_guard_accepted_count:0,
      rollback_audit_evidence_recorded_count:0,
      operator_review_handoff_accepted_count:0,
      minimal_real_write_handoff_accepted_count:0,
      rollback_performed_count:0,
      tombstone_written_count:0,
      compensating_memory_write_performed_count:0,
      durable_memory_store_read_performed_count:0,
      durable_memory_store_rollback_performed_count:0,
      durable_memory_store_write_performed_count:0,
      memory_store_write_performed_count:0,
      required_before_scoped_memory_real_write_canary_rollback_tombstone_acceptance_count:15,
      required_scoped_memory_real_write_canary_rollback_tombstone_fields:[
        "source_scoped_canary_readback_validation_report_sha256",
        "wal_receipt_id",
        "readback_result_id",
        "rollback_plan_id",
        "tombstone_plan_id",
        "rollback_target_digest",
        "canary_namespace",
        "canary_store",
        "canary_scope",
        "rollback_receipt_id",
        "rollback_idempotency_guard_id",
        "rollback_ordering_guard_id",
        "rollback_audit_evidence_id",
        "operator_review_handoff_id",
        "minimal_real_write_handoff_id"
      ],
      rollback_plan_required:true,
      tombstone_plan_required:true,
      rollback_target_binding_required:true,
      rollback_receipt_linkage_required:true,
      rollback_idempotency_guard_required:true,
      rollback_ordering_guard_required:true,
      rollback_audit_evidence_required:true,
      operator_review_handoff_required:true,
      minimal_real_write_handoff_required:true,
      rollback_execution_forbidden:true,
      tombstone_write_forbidden:true,
      durable_memory_read_forbidden:true,
      durable_memory_write_forbidden:true,
      durable_memory_rollback_forbidden:true,
      memory_store_mutation_forbidden:true,
      kg_live_write_forbidden:true,
      provider_model_invocation_forbidden:true,
      credential_read_forbidden:true,
      channel_external_send_forbidden:true,
      public_claim_release_artifact_forbidden:true,
      install_restart_active_binary_mutation_forbidden:true,
      scoped_memory_real_write_canary_rollback_tombstone_fixtures:$fixtures,
      denied_by_scoped_memory_real_write_canary_rollback_tombstone_dry_run:[
        "source_readback_validation_dry_run_boundary_required",
        "readback_result_not_accepted",
        "rollback_tombstone_handoff_not_accepted",
        "rollback_plan_not_accepted",
        "tombstone_plan_not_accepted",
        "rollback_target_not_bound",
        "rollback_receipt_not_linked",
        "rollback_idempotency_guard_not_accepted",
        "rollback_ordering_guard_not_accepted",
        "rollback_audit_evidence_not_recorded",
        "operator_review_handoff_not_accepted",
        "minimal_real_write_handoff_not_accepted",
        "durable_memory_store_read_denied",
        "durable_memory_store_write_denied",
        "durable_memory_store_rollback_denied",
        "memory_store_mutation_denied",
        "rollback_execution_denied",
        "tombstone_write_denied",
        "compensating_memory_write_denied",
        "kg_live_write_denied",
        "provider_model_invocation_denied",
        "credential_read_denied",
        "channel_external_send_denied",
        "public_claim_release_artifact_denied",
        "install_restart_active_binary_mutation_denied",
        "filesystem_write_denied",
        "activation_authority_denied",
        "minimal_real_write_canary_acceptance_denied"
      ],
      denied_by_scoped_memory_real_write_canary_rollback_tombstone_dry_run_count:28,
      fresh_operator_approval_packet_accepted:false,
      single_use_nonce_consumed:false,
      explicit_command_dispatched:false,
      receipt_persisted:false,
      post_write_readback_performed:false,
      readback_result_accepted:false,
      rollback_tombstone_handoff_accepted:false,
      rollback_plan_accepted:false,
      rollback_tombstone_plan_accepted:false,
      rollback_target_bound:false,
      rollback_receipt_linked:false,
      rollback_ordering_guard_accepted:false,
      rollback_idempotency_guard_accepted:false,
      rollback_audit_evidence_recorded:false,
      rollback_audit_evidence_persisted:false,
      operator_review_handoff_accepted:false,
      minimal_real_write_handoff_accepted:false,
      rollback_executed:false,
      rollback_result_recorded:false,
      rollback_result_persisted:false,
      rollback_result_accepted:false,
      tombstone_written:false,
      compensating_memory_write_performed:false,
      activation_allowed:false,
      memory_write_execution_performed:false,
      memory_store_write_performed:false,
      memory_store_mutated:false,
      durable_memory_store_write_performed:false,
      durable_memory_store_read_performed:false,
      durable_memory_store_rollback_performed:false,
      live_kg_write_performed:false,
      provider_invoked:false,
      model_invoked:false,
      credential_read:false,
      channel_send_performed:false,
      external_send_performed:false,
      release_artifact_written:false,
      install_executed:false,
      active_binary_mutated:false,
      allowed_next_actions:[
        {
          action:"run_scoped_memory_real_write_canary_rollback_tombstone_dry_run_boundary_require_live_gate",
          status:"allowed_verification_only",
          reads_memory:false,
          writes_memory:false,
          executes_rollback:false,
          writes_tombstone:false
        },
        {
          action:"prepare_minimal_scoped_memory_real_write_canary_accepted_gate",
          status:"allowed_report_only_next_slice",
          requires_rollback_tombstone_dry_run_boundary:true,
          writes_memory:false
        }
      ],
      side_effects:{
        durable_memory_store_read_performed:false,
        durable_memory_store_write_performed:false,
        durable_memory_store_rollback_performed:false,
        memory_store_mutated:false,
        rollback_executed:false,
        tombstone_written:false,
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
    }
  '
)"

jq -e '
  .status == "ready"
  and .scoped_memory_real_write_canary_rollback_tombstone_dry_run_ready == true
  and .source_scoped_memory_real_write_canary_readback_validation_dry_run_ready == true
  and .scoped_memory_real_write_canary_rollback_tombstone_fixture_count == 10
  and .accepted_scoped_memory_real_write_canary_rollback_tombstone_fixture_count == 0
  and .denied_by_scoped_memory_real_write_canary_rollback_tombstone_dry_run_count == 28
  and .rollback_performed_count == 0
  and .tombstone_written_count == 0
  and .durable_memory_store_read_performed_count == 0
  and .durable_memory_store_rollback_performed_count == 0
  and .durable_memory_store_write_performed_count == 0
  and .memory_store_write_performed_count == 0
  and .rollback_plan_required == true
  and .tombstone_plan_required == true
  and .rollback_execution_forbidden == true
  and .tombstone_write_forbidden == true
  and .durable_memory_read_forbidden == true
  and .durable_memory_write_forbidden == true
  and .durable_memory_rollback_forbidden == true
  and .kg_live_write_forbidden == true
  and .provider_model_invocation_forbidden == true
  and .credential_read_forbidden == true
  and .channel_external_send_forbidden == true
  and .install_restart_active_binary_mutation_forbidden == true
  and (.scoped_memory_real_write_canary_rollback_tombstone_fixtures | all((.scoped_memory_real_write_canary_rollback_tombstone_status | startswith("blocked")) and .rollback_executed == false and .tombstone_written == false and .durable_memory_store_read_performed == false and .durable_memory_store_write_performed == false and .durable_memory_store_rollback_performed == false and .memory_store_mutated == false and .live_kg_write_performed == false and .provider_invoked == false and .model_invoked == false and .credential_read == false and .channel_send_performed == false and .external_send_performed == false and .release_artifact_written == false and .install_executed == false and .active_binary_mutated == false and .scoped_canary_rollback_tombstone_dry_run_noop_confirmed == true))
  and ([.scoped_memory_real_write_canary_rollback_tombstone_fixtures[] | select(.rollback_execution_requested == true and .tombstone_write_requested == true)] | length) == 1
  and .allowed_next_actions[1].action == "prepare_minimal_scoped_memory_real_write_canary_accepted_gate"
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
