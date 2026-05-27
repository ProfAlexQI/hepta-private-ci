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

LEDGER_RECEIPT_JSON="$(
  capture_json_report \
    "hepta-core-activation-fresh-long-soak-evidence-ledger-receipt-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-fresh-long-soak-evidence-ledger-receipt-gate.sh
)"

NO_WRITE_SINK_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract" \
    scripts/hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract.sh
)"

MATERIALIZATION_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-receipt-materialization-dry-run" \
    scripts/hepta-upstream-codex-activation-evidence-receipt-materialization-dry-run.sh
)"

OUTPUT_PATH_ALLOWLIST_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist" \
    scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh
)"

OUTPUT_PATH_BINDING_JSON="$(
  capture_json_report \
    "hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding" \
    scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh
)"

ledger_receipt_report_sha256="$(sha256_text "$LEDGER_RECEIPT_JSON")"
no_write_sink_report_sha256="$(sha256_text "$NO_WRITE_SINK_JSON")"
materialization_report_sha256="$(sha256_text "$MATERIALIZATION_JSON")"
output_path_allowlist_report_sha256="$(sha256_text "$OUTPUT_PATH_ALLOWLIST_JSON")"
output_path_binding_report_sha256="$(sha256_text "$OUTPUT_PATH_BINDING_JSON")"
materialization_plan_hash_sha256="$(sha256_text "hepta-core-activation-evidence-receipt-materialization:plan:$ledger_receipt_report_sha256:$no_write_sink_report_sha256:$materialization_report_sha256:$output_path_allowlist_report_sha256:$output_path_binding_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
materialization_redaction_hash_sha256="$(sha256_text "hepta-core-activation-evidence-receipt-materialization:redaction:$ledger_receipt_report_sha256:$no_write_sink_report_sha256:$materialization_report_sha256:$output_path_allowlist_report_sha256:$output_path_binding_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
materialization_no_write_hash_sha256="$(sha256_text "hepta-core-activation-evidence-receipt-materialization:no-write:$ledger_receipt_report_sha256:$no_write_sink_report_sha256:$materialization_report_sha256:$output_path_allowlist_report_sha256:$output_path_binding_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson ledger "$LEDGER_RECEIPT_JSON" \
  --argjson sink "$NO_WRITE_SINK_JSON" \
  --argjson materialization "$MATERIALIZATION_JSON" \
  --argjson allowlist "$OUTPUT_PATH_ALLOWLIST_JSON" \
  --argjson binding "$OUTPUT_PATH_BINDING_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $ledger.runtime == "hepta"
    and $ledger.status == "ready"
    and $ledger.gate == "hepta_core_activation_fresh_long_soak_evidence_ledger_receipt_gate"
    and $ledger.fresh_long_soak_evidence_ledger_receipt_ready == true
    and $ledger.ledger_receipt_mode == "schema_only_no_long_soak_execution_no_persistence"
    and $ledger.required_source_count == 4
    and $ledger.ready_source_count == 4
    and $ledger.activation_blocking_source_count == 4
    and $ledger.minimum_required_long_soak_samples >= 24
    and $ledger.long_soak_executed_by_this_gate == false
    and $ledger.long_soak_evidence_recorded == false
    and $ledger.long_soak_evidence_persisted == false
    and $ledger.source_required_evidence_count == 8
    and $ledger.source_recorded_evidence_count == 0
    and $ledger.source_fresh_evidence_count == 0
    and $ledger.required_ledger_record_field_count == 20
    and $ledger.recorded_ledger_record_field_count == 0
    and $ledger.required_receipt_field_count == 20
    and $ledger.recorded_receipt_field_count == 0
    and $ledger.receipt_materialized == false
    and $ledger.receipt_persisted == false
    and $ledger.activation_allowed == false
    and $ledger.public_release_claim_allowed == false
    and $ledger.release_artifact_write_allowed == false
    and ($ledger.side_effects | to_entries | all(.value == false))
    and $sink.product == "Hepta"
    and $sink.status == "ready"
    and $sink.no_write_sink_adapter_id == "upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract"
    and $sink.sink_status.source_invocation_dry_run_ready == true
    and $sink.sink_status.required_sink_surface_count == 6
    and $sink.sink_status.ready_sink_surface_count == 6
    and $sink.sink_status.side_effect_free_surface_count == 6
    and $sink.sink_status.persisted_receipt_count == 0
    and $sink.sink_status.workspace_write_performed_count == 0
    and $sink.sink_status.sink_write_path_enabled_by_default == false
    and $sink.sink_status.sink_accepts_redacted_payload_hash == true
    and $sink.sink_status.sink_accepts_redacted_output_path == true
    and $sink.sink_status.sink_requires_operator_approval == true
    and $sink.sink_status.sink_requires_fresh_trusted_records == true
    and $sink.sink_status.no_write_sink_adapter_ready == true
    and $sink.sink_status.activation_blocked_by_no_write_sink_adapter == true
    and $sink.sink_status.activation_allowed_by_no_write_sink_adapter == false
    and $sink.sink_status.active_wiring_allowed == false
    and ($sink.sink_surfaces | length) == 6
    and ($sink.side_effects | to_entries | all(.value == false))
    and $materialization.product == "Hepta"
    and $materialization.status == "ready"
    and $materialization.materialization_dry_run_id == "upstream-codex-activation-evidence-receipt-materialization-dry-run"
    and $materialization.fixture_status.source_write_enable_fixture_ready == true
    and $materialization.fixture_status.required_materialization_fixture_count == 3
    and $materialization.fixture_status.materialization_fixture_count == 3
    and $materialization.fixture_status.blocked_materialization_fixture_count == 3
    and $materialization.fixture_status.allowed_materialization_fixture_count == 0
    and $materialization.fixture_status.deterministic_materialization_plan_count == 3
    and $materialization.fixture_status.filesystem_persistence_allowed_count == 0
    and $materialization.fixture_status.materialization_executed_count == 0
    and $materialization.fixture_status.workspace_write_performed_count == 0
    and $materialization.fixture_status.evidence_receipt_persisted_count == 0
    and $materialization.fixture_status.materialization_dry_run_ready == true
    and $materialization.fixture_status.activation_blocked_by_materialization_dry_run == true
    and $materialization.fixture_status.activation_allowed_by_materialization_dry_run == false
    and $materialization.fixture_status.active_wiring_allowed == false
    and ($materialization.fixtures | length) == 3
    and ($materialization.side_effects | to_entries | all(.value == false))
    and $allowlist.product == "Hepta"
    and $allowlist.status == "ready"
    and $allowlist.filesystem_output_path_allowlist_id == "upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist"
    and $allowlist.allowlist_status.source_filesystem_persistence_approval_packet_ready == true
    and $allowlist.allowlist_status.required_allowlist_entry_count == 6
    and $allowlist.allowlist_status.allowlist_entry_count == 6
    and $allowlist.allowlist_status.allowed_output_path_entry_count == 3
    and $allowlist.allowlist_status.blocked_output_path_entry_count == 3
    and $allowlist.allowlist_status.default_selected_output_path_count == 0
    and $allowlist.allowlist_status.source_tree_path_allowed == false
    and $allowlist.allowlist_status.home_directory_path_allowed == false
    and $allowlist.allowlist_status.release_artifact_path_allowed == false
    and $allowlist.allowlist_status.public_artifact_path_allowed == false
    and $allowlist.allowlist_status.receipt_output_path_allowlist_ready == true
    and $allowlist.allowlist_status.filesystem_persistence_allowed == false
    and $allowlist.allowlist_status.filesystem_persistence_execution_performed == false
    and $allowlist.allowlist_status.workspace_write_performed == false
    and $allowlist.allowlist_status.evidence_receipt_persisted == false
    and $allowlist.allowlist_status.activation_blocked_by_output_path_allowlist == true
    and $allowlist.allowlist_status.activation_allowed_by_output_path_allowlist == false
    and $allowlist.allowlist_status.active_wiring_allowed == false
    and ($allowlist.allowlist_entries | length) == 6
    and ($allowlist.side_effects | to_entries | all(.value == false))
    and $binding.product == "Hepta"
    and $binding.status == "ready"
    and $binding.filesystem_output_path_evidence_binding_id == "upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding"
    and $binding.binding_status.source_filesystem_output_path_allowlist_ready == true
    and $binding.binding_status.required_path_binding_count == 8
    and $binding.binding_status.path_binding_count == 8
    and $binding.binding_status.allowed_output_path_entry_count == 3
    and $binding.binding_status.selected_output_path_count == 0
    and $binding.binding_status.recorded_path_binding_count == 0
    and $binding.binding_status.fresh_live_evidence_bound_count == 0
    and $binding.binding_status.active_binary_sha_bound_count == 0
    and $binding.binding_status.redacted_or_hashed_binding_count == 8
    and $binding.binding_status.output_path_evidence_binding_ready == true
    and $binding.binding_status.filesystem_persistence_allowed == false
    and $binding.binding_status.filesystem_persistence_execution_performed == false
    and $binding.binding_status.workspace_write_performed == false
    and $binding.binding_status.evidence_receipt_persisted == false
    and $binding.binding_status.activation_blocked_by_output_path_evidence_binding == true
    and $binding.binding_status.activation_allowed_by_output_path_evidence_binding == false
    and $binding.binding_status.active_wiring_allowed == false
    and ($binding.evidence_bindings | length) == 8
    and ($binding.allowed_output_path_bindings | length) == 3
    and ($binding.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_evidence_receipt_materialization_dry_run_gate" \
  --arg ledger_receipt_report_sha256 "$ledger_receipt_report_sha256" \
  --arg no_write_sink_report_sha256 "$no_write_sink_report_sha256" \
  --arg materialization_report_sha256 "$materialization_report_sha256" \
  --arg output_path_allowlist_report_sha256 "$output_path_allowlist_report_sha256" \
  --arg output_path_binding_report_sha256 "$output_path_binding_report_sha256" \
  --arg materialization_plan_hash_sha256 "$materialization_plan_hash_sha256" \
  --arg materialization_redaction_hash_sha256 "$materialization_redaction_hash_sha256" \
  --arg materialization_no_write_hash_sha256 "$materialization_no_write_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson ledger "$LEDGER_RECEIPT_JSON" \
  --argjson sink "$NO_WRITE_SINK_JSON" \
  --argjson materialization "$MATERIALIZATION_JSON" \
  --argjson allowlist "$OUTPUT_PATH_ALLOWLIST_JSON" \
  --argjson binding "$OUTPUT_PATH_BINDING_JSON" \
  '
    ([
      "receipt_id",
      "ledger_record_id",
      "materialization_plan_id",
      "no_write_sink_adapter_id",
      "redacted_payload_hash",
      "redacted_output_path",
      "output_path_allowlist_id",
      "output_path_evidence_binding_id",
      "active_binary_sha256",
      "source_ledger_receipt_report_sha256",
      "source_no_write_sink_report_sha256",
      "source_materialization_report_sha256",
      "source_output_path_allowlist_report_sha256",
      "source_output_path_binding_report_sha256",
      "no_secret_payload_review_id",
      "operator_approval_id",
      "fresh_long_soak_evidence_id",
      "filesystem_persistence_approval_id",
      "rollback_plan_id",
      "public_claim_and_artifact_decision"
    ]) as $materialization_fields
    | ([
      "fresh_long_soak_ledger_receipt_schema",
      "no_write_sink_adapter_contract",
      "materialization_dry_run_fixture_matrix",
      "filesystem_output_path_allowlist",
      "filesystem_output_path_evidence_binding",
      "redaction_and_no_secret_review",
      "hash_chain_and_audit_binding",
      "activation_side_effect_denial_boundary"
    ]) as $readiness_families
    | ([
      "fresh_24_sample_long_soak_not_executed_by_this_gate",
      "fresh_long_soak_evidence_record_not_recorded",
      "operator_approval_not_recorded",
      "activation_request_not_recorded",
      "ledger_record_not_recorded",
      "receipt_materialization_plan_not_recorded",
      "receipt_not_materialized",
      "receipt_not_persisted",
      "filesystem_persistence_approval_not_recorded",
      "output_path_not_selected",
      "output_path_allowlist_not_bound_to_fresh_evidence",
      "no_write_sink_keeps_write_path_disabled_by_default",
      "raw_soak_payload_persistence_denied",
      "workspace_write_denied",
      "public_release_claim_denied",
      "release_artifact_write_denied",
      "provider_model_invocation_denied",
      "channel_delivery_denied",
      "install_restart_active_binary_mutation_denied",
      "upstream_fetch_merge_denied"
    ]) as $denied
    | ([
      {id:"fresh-long-soak-ledger-receipt", ready:true, blocked:true, source_gate:$ledger.gate, source_report_sha256:$ledger_receipt_report_sha256},
      {id:"no-write-sink-adapter", ready:true, blocked:true, source_gate:$sink.no_write_sink_adapter_contract_gate, source_report_sha256:$no_write_sink_report_sha256},
      {id:"materialization-dry-run", ready:true, blocked:true, source_gate:$materialization.materialization_dry_run_gate, source_report_sha256:$materialization_report_sha256},
      {id:"output-path-allowlist", ready:true, blocked:true, source_gate:$allowlist.filesystem_output_path_allowlist_gate, source_report_sha256:$output_path_allowlist_report_sha256},
      {id:"output-path-evidence-binding", ready:true, blocked:true, source_gate:$binding.filesystem_output_path_evidence_binding_gate, source_report_sha256:$output_path_binding_report_sha256},
      {id:"activation-side-effect-boundary", ready:true, blocked:true, denied_action_count:($denied | length)}
    ]) as $source_families
    | {
      product:$product,
      runtime:$runtime,
      status:"ready",
      base_url:$base_url,
      gate:$gate,
      activation_evidence_receipt_materialization_schema_version:"hepta_core_activation_evidence_receipt_materialization_dry_run_v1",
      receipt_materialization_dry_run_ready:true,
      materialization_mode:"schema_only_no_write_no_output_path_selection",
      materialization_decision:"blocked_until_fresh_long_soak_receipt_operator_approval_and_filesystem_persistence_approval_exist",
      source_fresh_long_soak_ledger_receipt_gate:$ledger.gate,
      source_no_write_sink_adapter_gate:$sink.no_write_sink_adapter_contract_gate,
      source_materialization_dry_run_gate:$materialization.materialization_dry_run_gate,
      source_output_path_allowlist_gate:$allowlist.filesystem_output_path_allowlist_gate,
      source_output_path_evidence_binding_gate:$binding.filesystem_output_path_evidence_binding_gate,
      source_ledger_receipt_report_sha256:$ledger_receipt_report_sha256,
      source_no_write_sink_report_sha256:$no_write_sink_report_sha256,
      source_materialization_report_sha256:$materialization_report_sha256,
      source_output_path_allowlist_report_sha256:$output_path_allowlist_report_sha256,
      source_output_path_binding_report_sha256:$output_path_binding_report_sha256,
      source_report_hashes:[
        $ledger_receipt_report_sha256,
        $no_write_sink_report_sha256,
        $materialization_report_sha256,
        $output_path_allowlist_report_sha256,
        $output_path_binding_report_sha256
      ],
      materialization_plan_hash_sha256:$materialization_plan_hash_sha256,
      materialization_redaction_hash_sha256:$materialization_redaction_hash_sha256,
      materialization_no_write_hash_sha256:$materialization_no_write_hash_sha256,
      required_source_count:5,
      ready_source_count:5,
      activation_blocking_source_count:5,
      minimum_required_long_soak_samples:$min_long_soak_samples,
      source_required_evidence_count:$ledger.source_required_evidence_count,
      source_recorded_evidence_count:$ledger.source_recorded_evidence_count,
      source_fresh_evidence_count:$ledger.source_fresh_evidence_count,
      source_required_ledger_record_field_count:$ledger.required_ledger_record_field_count,
      source_recorded_ledger_record_field_count:$ledger.recorded_ledger_record_field_count,
      source_required_receipt_field_count:$ledger.required_receipt_field_count,
      source_recorded_receipt_field_count:$ledger.recorded_receipt_field_count,
      required_materialization_field_count:($materialization_fields | length),
      recorded_materialization_field_count:0,
      planned_materialization_field_count:0,
      required_no_write_sink_surface_count:$sink.sink_status.required_sink_surface_count,
      ready_no_write_sink_surface_count:$sink.sink_status.ready_sink_surface_count,
      side_effect_free_sink_surface_count:$sink.sink_status.side_effect_free_surface_count,
      required_materialization_fixture_count:$materialization.fixture_status.required_materialization_fixture_count,
      blocked_materialization_fixture_count:$materialization.fixture_status.blocked_materialization_fixture_count,
      allowed_materialization_fixture_count:$materialization.fixture_status.allowed_materialization_fixture_count,
      deterministic_materialization_plan_count:$materialization.fixture_status.deterministic_materialization_plan_count,
      required_output_path_allowlist_entry_count:$allowlist.allowlist_status.required_allowlist_entry_count,
      allowed_output_path_entry_count:$allowlist.allowlist_status.allowed_output_path_entry_count,
      blocked_output_path_entry_count:$allowlist.allowlist_status.blocked_output_path_entry_count,
      required_output_path_binding_count:$binding.binding_status.required_path_binding_count,
      recorded_output_path_binding_count:$binding.binding_status.recorded_path_binding_count,
      redacted_or_hashed_output_path_binding_count:$binding.binding_status.redacted_or_hashed_binding_count,
      long_soak_executed_by_this_gate:false,
      long_soak_evidence_recorded:false,
      operator_approval_recorded:false,
      activation_request_recorded:false,
      ledger_record_recorded:false,
      receipt_materialization_plan_recorded:false,
      receipt_materialized:false,
      receipt_persisted:false,
      filesystem_persistence_approval_recorded:false,
      filesystem_persistence_allowed:false,
      filesystem_persistence_execution_performed:false,
      output_path_selected:false,
      output_path_allowlisted:false,
      output_path_bound_to_fresh_evidence:false,
      workspace_write_performed:false,
      activation_allowed:false,
      live_mutation_execution_ready:false,
      active_wiring_allowed:false,
      public_release_claim_allowed:false,
      public_ga_claim_allowed:false,
      release_artifact_write_allowed:false,
      provider_model_invocation_allowed:false,
      channel_delivery_allowed:false,
      install_restart_allowed:false,
      upstream_fetch_merge_allowed:false,
      required_materialization_fields:$materialization_fields,
      materialization_readiness_families:$readiness_families,
      source_readiness_families:$source_families,
      denied_by_receipt_materialization_dry_run:$denied,
      denied_by_receipt_materialization_dry_run_count:($denied | length),
      side_effects:{
        workspace_written:false,
        filesystem_written:false,
        memory_store_mutated:false,
        capability_registry_mutated:false,
        plugin_registry_mutated:false,
        provider_invoked:false,
        model_invoked:false,
        channel_send_performed:false,
        telegram_send_performed:false,
        gateway_event_enqueued:false,
        gateway_rpc_performed:false,
        external_network_read:false,
        external_send_performed:false,
        release_artifact_written:false,
        public_artifact_written:false,
        public_release_published:false,
        public_ga_claimed:false,
        install_executed:false,
        launchd_mutated:false,
        service_restarted:false,
        active_binary_mutated:false,
        rollback_executed:false,
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        credential_read:false,
        secret_value_read:false,
        long_soak_executed:false,
        approval_packet_persisted:false,
        ledger_record_persisted:false,
        receipt_materialization_plan_recorded:false,
        receipt_materialized:false,
        receipt_persisted:false,
        output_path_selected:false,
        output_path_bound:false,
        filesystem_persistence_executed:false,
        trusted_record_persisted:false
      }
    }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_evidence_receipt_materialization_dry_run_gate"
  and .receipt_materialization_dry_run_ready == true
  and .materialization_mode == "schema_only_no_write_no_output_path_selection"
  and .materialization_decision == "blocked_until_fresh_long_soak_receipt_operator_approval_and_filesystem_persistence_approval_exist"
  and .required_source_count == 5
  and .ready_source_count == 5
  and .activation_blocking_source_count == 5
  and .minimum_required_long_soak_samples >= 24
  and .source_required_evidence_count == 8
  and .source_recorded_evidence_count == 0
  and .source_fresh_evidence_count == 0
  and .source_required_ledger_record_field_count == 20
  and .source_recorded_ledger_record_field_count == 0
  and .source_required_receipt_field_count == 20
  and .source_recorded_receipt_field_count == 0
  and .required_materialization_field_count == 20
  and .recorded_materialization_field_count == 0
  and .planned_materialization_field_count == 0
  and .required_no_write_sink_surface_count == 6
  and .ready_no_write_sink_surface_count == 6
  and .side_effect_free_sink_surface_count == 6
  and .required_materialization_fixture_count == 3
  and .blocked_materialization_fixture_count == 3
  and .allowed_materialization_fixture_count == 0
  and .deterministic_materialization_plan_count == 3
  and .required_output_path_allowlist_entry_count == 6
  and .allowed_output_path_entry_count == 3
  and .blocked_output_path_entry_count == 3
  and .required_output_path_binding_count == 8
  and .recorded_output_path_binding_count == 0
  and .redacted_or_hashed_output_path_binding_count == 8
  and .long_soak_executed_by_this_gate == false
  and .long_soak_evidence_recorded == false
  and .operator_approval_recorded == false
  and .activation_request_recorded == false
  and .ledger_record_recorded == false
  and .receipt_materialization_plan_recorded == false
  and .receipt_materialized == false
  and .receipt_persisted == false
  and .filesystem_persistence_approval_recorded == false
  and .filesystem_persistence_allowed == false
  and .filesystem_persistence_execution_performed == false
  and .output_path_selected == false
  and .output_path_allowlisted == false
  and .output_path_bound_to_fresh_evidence == false
  and .workspace_write_performed == false
  and .activation_allowed == false
  and .live_mutation_execution_ready == false
  and .active_wiring_allowed == false
  and .public_release_claim_allowed == false
  and .public_ga_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .provider_model_invocation_allowed == false
  and .channel_delivery_allowed == false
  and .install_restart_allowed == false
  and .upstream_fetch_merge_allowed == false
  and (.required_materialization_fields | length) == 20
  and (.materialization_readiness_families | length) == 8
  and (.source_readiness_families | length) == 6
  and (.source_readiness_families | all(.ready == true and .blocked == true))
  and .denied_by_receipt_materialization_dry_run_count == 20
  and (.denied_by_receipt_materialization_dry_run | length) == 20
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
