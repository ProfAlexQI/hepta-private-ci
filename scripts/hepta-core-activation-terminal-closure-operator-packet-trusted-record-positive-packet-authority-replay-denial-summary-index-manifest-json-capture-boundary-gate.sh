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

MANIFEST_JSON="$(
  capture_json_report \
    "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest-gate.sh
)"

manifest_report_sha256="$(sha256_text "$MANIFEST_JSON")"
json_capture_boundary_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest-json-capture-boundary:boundary:$manifest_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
json_capture_boundary_policy_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest-json-capture-boundary:policy:$manifest_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
json_capture_boundary_denial_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest-json-capture-boundary:denial:$manifest_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
json_capture_boundary_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest-json-capture-boundary:side-effects:$manifest_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson manifest "$MANIFEST_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $manifest.runtime == "hepta"
    and $manifest.status == "ready"
    and $manifest.gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_gate"
    and $manifest.terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_ready == true
    and $manifest.trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_status == "blocked"
    and $manifest.source_positive_packet_authority_replay_denial_summary_index_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_gate"
    and $manifest.source_positive_packet_authority_replay_denial_summary_index_status == "blocked"
    and $manifest.source_positive_packet_authority_replay_denial_summary_status == "blocked"
    and $manifest.source_positive_packet_authority_replay_denial_matrix_status == "blocked"
    and $manifest.source_terminal_closure_verdict == "blocked"
    and $manifest.required_positive_packet_authority_replay_fixture_count == 12
    and $manifest.positive_packet_authority_replay_fixture_count == 12
    and $manifest.blocked_positive_packet_authority_replay_fixture_count == 12
    and $manifest.allowed_positive_packet_authority_replay_fixture_count == 0
    and $manifest.required_manifested_replay_entry_point_summary_count == 8
    and $manifest.manifested_replay_entry_point_summary_count == 8
    and $manifest.blocked_manifested_replay_entry_point_summary_count == 8
    and ($manifest.manifested_replay_entry_point_summary | length) == 8
    and ($manifest.manifested_replay_entry_point_summary | all(.manifest_status == "blocked" and .manifest_authority_allowed == false and .manifest_recorded == false and .manifest_persisted == false and .manifest_delivered == false))
    and $manifest.required_manifested_replay_surface_summary_count == 12
    and $manifest.manifested_replay_surface_summary_count == 12
    and $manifest.blocked_manifested_replay_surface_summary_count == 12
    and ($manifest.manifested_replay_surface_summary | length) == 12
    and ($manifest.manifested_replay_surface_summary | all(.manifest_status == "blocked" and .manifest_authority_allowed == false and .manifest_recorded == false and .manifest_persisted == false and .manifest_delivered == false))
    and $manifest.required_manifested_source_summary_family_count == 8
    and $manifest.manifested_source_summary_family_count == 8
    and $manifest.blocked_manifested_source_summary_family_count == 8
    and ($manifest.manifested_source_summary_families | length) == 8
    and ($manifest.manifested_source_summary_families | all(.manifest_status == "blocked" and .manifest_authority_allowed == false))
    and $manifest.required_manifested_summary_index_family_count == 8
    and $manifest.manifested_summary_index_family_count == 8
    and $manifest.blocked_manifested_summary_index_family_count == 8
    and ($manifest.manifested_summary_index_families | length) == 8
    and ($manifest.manifested_summary_index_families | all(.manifest_status == "blocked" and .manifest_authority_allowed == false))
    and $manifest.required_summary_index_manifest_family_count == 8
    and $manifest.summary_index_manifest_family_count == 8
    and $manifest.ready_summary_index_manifest_family_count == 8
    and $manifest.activation_blocking_summary_index_manifest_family_count == 8
    and ($manifest.summary_index_manifest_families | length) == 8
    and ($manifest.summary_index_manifest_families | all(.ready == true and .blocked == true and .manifest_authority_allowed == false))
    and $manifest.inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count == 37
    and $manifest.source_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_count == 45
    and $manifest.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_count == 53
    and ($manifest.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest | length) == 53
    and $manifest.summary_index_manifest_authority_allowed == false
    and $manifest.summary_index_manifest_recorded == false
    and $manifest.summary_index_manifest_persisted == false
    and $manifest.summary_index_manifest_materialized == false
    and $manifest.summary_index_manifest_delivered == false
    and $manifest.summary_index_manifest_promoted_to_authority == false
    and $manifest.summary_index_authority_allowed == false
    and $manifest.summary_index_recorded == false
    and $manifest.summary_index_persisted == false
    and $manifest.summary_index_materialized == false
    and $manifest.summary_index_delivered == false
    and $manifest.trusted_record_acceptance_allowed == false
    and $manifest.trusted_record_accepted == false
    and $manifest.terminal_closure_allowed == false
    and $manifest.terminal_closure_recorded == false
    and $manifest.activation_allowed == false
    and $manifest.activation_performed == false
    and $manifest.receipt_accepted == false
    and $manifest.ledger_recorded == false
    and $manifest.index_delivered == false
    and $manifest.completion_ack_accepted == false
    and $manifest.public_release_claim_allowed == false
    and $manifest.release_artifact_write_allowed == false
    and ($manifest.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_gate" \
  --arg manifest_report_sha256 "$manifest_report_sha256" \
  --arg json_capture_boundary_hash_sha256 "$json_capture_boundary_hash_sha256" \
  --arg json_capture_boundary_policy_hash_sha256 "$json_capture_boundary_policy_hash_sha256" \
  --arg json_capture_boundary_denial_hash_sha256 "$json_capture_boundary_denial_hash_sha256" \
  --arg json_capture_boundary_side_effect_hash_sha256 "$json_capture_boundary_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson manifest "$MANIFEST_JSON" \
  '
    ($manifest.summary_index_manifest_families
      | map(. + {
          json_capture_status: "blocked",
          json_capture_recorded: false,
          json_capture_persisted: false,
          json_capture_materialized: false,
          json_capture_delivered: false,
          json_capture_authority_allowed: false,
          trusted_record_acceptance_allowed: false,
          terminal_closure_allowed: false,
          activation_allowed: false
        })) as $captured_manifest_families
    | ([
        {
          id: "source-manifest-json-capture-witness",
          ready: true,
          blocked: true,
          witness_sha256: $manifest_report_sha256,
          source_gate: $manifest.gate,
          source_status: $manifest.trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_status,
          json_capture_authority_allowed: false,
          reason: "capture_json_report produces a witness hash only"
        },
        {
          id: "replay-fixture-invariant-json-capture-boundary",
          ready: true,
          blocked: true,
          fixture_count: $manifest.positive_packet_authority_replay_fixture_count,
          blocked_fixture_count: $manifest.blocked_positive_packet_authority_replay_fixture_count,
          allowed_fixture_count: $manifest.allowed_positive_packet_authority_replay_fixture_count,
          json_capture_authority_allowed: false,
          reason: "capturing the manifest preserves the 12 blocked replay fixtures"
        },
        {
          id: "entry-point-manifest-json-capture-boundary",
          ready: true,
          blocked: true,
          manifested_replay_entry_point_summary_count: $manifest.manifested_replay_entry_point_summary_count,
          json_capture_authority_allowed: false,
          reason: "captured entry point summaries stay non-authoritative"
        },
        {
          id: "replay-surface-manifest-json-capture-boundary",
          ready: true,
          blocked: true,
          manifested_replay_surface_summary_count: $manifest.manifested_replay_surface_summary_count,
          json_capture_authority_allowed: false,
          reason: "captured replay surface summaries stay non-authoritative"
        },
        {
          id: "source-and-index-family-json-capture-boundary",
          ready: true,
          blocked: true,
          manifested_source_summary_family_count: $manifest.manifested_source_summary_family_count,
          manifested_summary_index_family_count: $manifest.manifested_summary_index_family_count,
          summary_index_manifest_family_count: $manifest.summary_index_manifest_family_count,
          json_capture_authority_allowed: false,
          reason: "captured family summaries remain blocked"
        },
        {
          id: "denial-reason-json-capture-boundary",
          ready: true,
          blocked: true,
          inherited_summary_denial_reason_count: $manifest.inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count,
          source_index_denial_reason_count: $manifest.source_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_count,
          manifest_denial_reason_count: $manifest.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_count,
          json_capture_authority_allowed: false,
          reason: "captured denial reasons cannot become approval"
        },
        {
          id: "authority-nonpromotion-json-capture-boundary",
          ready: true,
          blocked: true,
          summary_index_manifest_authority_allowed: false,
          json_capture_authority_allowed: false,
          trusted_record_acceptance_allowed: false,
          terminal_closure_allowed: false,
          activation_allowed: false,
          public_release_claim_allowed: false,
          reason: "JSON capture is not authority promotion"
        },
        {
          id: "side-effect-json-capture-boundary",
          ready: true,
          blocked: true,
          source_side_effects_all_false: ($manifest.side_effects | to_entries | all(.value == false)),
          json_capture_recorded: false,
          json_capture_persisted: false,
          json_capture_materialized: false,
          json_capture_delivered: false,
          json_capture_authority_allowed: false,
          filesystem_written: false,
          reason: "the boundary emits stdout only and records no side effects"
        }
      ]) as $capture_families
    | ([
        "summary_index_manifest_json_capture_recording_denied",
        "summary_index_manifest_json_capture_persistence_denied",
        "summary_index_manifest_json_capture_materialization_denied",
        "summary_index_manifest_json_capture_delivery_denied",
        "summary_index_manifest_json_capture_authority_promotion_denied",
        "summary_index_manifest_json_capture_trusted_record_acceptance_denied",
        "summary_index_manifest_json_capture_terminal_closure_denied",
        "summary_index_manifest_json_capture_activation_denied"
      ] + $manifest.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest) as $capture_denied
    | {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_schema_version: "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_v1",
        terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_ready: true,
        trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_mode: "stdout_only_report_only_summary_index_manifest_json_capture_boundary_no_persistence_no_acceptance_no_delivery_no_activation",
        trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_status: "blocked",
        trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_decision: "summary_index_manifest_captured_as_witness_without_recording_persisting_materializing_delivering_promoting_accepting_closing_activating_or_releasing",
        source_positive_packet_authority_replay_denial_summary_index_manifest_gate: $manifest.gate,
        source_positive_packet_authority_replay_denial_summary_index_manifest_status: $manifest.trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_status,
        source_positive_packet_authority_replay_denial_summary_index_manifest_report_sha256: $manifest_report_sha256,
        source_positive_packet_authority_replay_denial_summary_index_manifest_hash_sha256: $manifest.positive_packet_authority_replay_denial_summary_index_manifest_hash_sha256,
        source_positive_packet_authority_replay_denial_summary_index_manifest_policy_hash_sha256: $manifest.positive_packet_authority_replay_denial_summary_index_manifest_policy_hash_sha256,
        source_positive_packet_authority_replay_denial_summary_index_manifest_denial_hash_sha256: $manifest.positive_packet_authority_replay_denial_summary_index_manifest_denial_hash_sha256,
        source_positive_packet_authority_replay_denial_summary_index_manifest_side_effect_hash_sha256: $manifest.positive_packet_authority_replay_denial_summary_index_manifest_side_effect_hash_sha256,
        source_positive_packet_authority_replay_denial_summary_index_gate: $manifest.source_positive_packet_authority_replay_denial_summary_index_gate,
        source_positive_packet_authority_replay_denial_summary_index_status: $manifest.source_positive_packet_authority_replay_denial_summary_index_status,
        source_positive_packet_authority_replay_denial_summary_gate: $manifest.source_positive_packet_authority_replay_denial_summary_gate,
        source_positive_packet_authority_replay_denial_summary_status: $manifest.source_positive_packet_authority_replay_denial_summary_status,
        source_positive_packet_authority_replay_denial_matrix_gate: $manifest.source_positive_packet_authority_replay_denial_matrix_gate,
        source_positive_packet_authority_replay_denial_matrix_status: $manifest.source_positive_packet_authority_replay_denial_matrix_status,
        source_terminal_closure_gate: $manifest.source_terminal_closure_gate,
        source_terminal_closure_verdict: $manifest.source_terminal_closure_verdict,
        positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_hash_sha256: $json_capture_boundary_hash_sha256,
        positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_policy_hash_sha256: $json_capture_boundary_policy_hash_sha256,
        positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_denial_hash_sha256: $json_capture_boundary_denial_hash_sha256,
        positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_side_effect_hash_sha256: $json_capture_boundary_side_effect_hash_sha256,
        minimum_required_long_soak_samples: $min_long_soak_samples,
        required_positive_packet_authority_replay_fixture_count: $manifest.required_positive_packet_authority_replay_fixture_count,
        positive_packet_authority_replay_fixture_count: $manifest.positive_packet_authority_replay_fixture_count,
        blocked_positive_packet_authority_replay_fixture_count: $manifest.blocked_positive_packet_authority_replay_fixture_count,
        allowed_positive_packet_authority_replay_fixture_count: $manifest.allowed_positive_packet_authority_replay_fixture_count,
        manifested_replay_entry_point_summary_count: $manifest.manifested_replay_entry_point_summary_count,
        manifested_replay_surface_summary_count: $manifest.manifested_replay_surface_summary_count,
        manifested_source_summary_family_count: $manifest.manifested_source_summary_family_count,
        manifested_summary_index_family_count: $manifest.manifested_summary_index_family_count,
        summary_index_manifest_family_count: $manifest.summary_index_manifest_family_count,
        captured_summary_index_manifest_family_count: ($captured_manifest_families | length),
        captured_summary_index_manifest_families: $captured_manifest_families,
        required_json_capture_boundary_family_count: 8,
        json_capture_boundary_family_count: ($capture_families | length),
        ready_json_capture_boundary_family_count: ($capture_families | map(select(.ready == true)) | length),
        activation_blocking_json_capture_boundary_family_count: ($capture_families | map(select(.blocked == true)) | length),
        json_capture_boundary_families: $capture_families,
        inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count: $manifest.inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count,
        source_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_count: $manifest.source_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_count,
        source_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_count: $manifest.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_count,
        denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary: $capture_denied,
        denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_count: ($capture_denied | length),
        trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_executed: true,
        json_report_capture_helper_used: true,
        json_capture_boundary_authority_allowed: false,
        json_capture_boundary_recorded: false,
        json_capture_boundary_persisted: false,
        json_capture_boundary_materialized: false,
        json_capture_boundary_delivered: false,
        json_capture_boundary_promoted_to_authority: false,
        summary_index_manifest_authority_allowed: false,
        summary_index_manifest_recorded: false,
        summary_index_manifest_persisted: false,
        summary_index_manifest_materialized: false,
        summary_index_manifest_delivered: false,
        summary_index_manifest_promoted_to_authority: false,
        summary_index_authority_allowed: false,
        summary_index_recorded: false,
        summary_index_persisted: false,
        summary_index_materialized: false,
        summary_index_delivered: false,
        summary_index_promoted_to_authority: false,
        summary_recorded: false,
        summary_persisted: false,
        summary_materialized: false,
        summary_delivered: false,
        operator_packet_recorded: false,
        operator_packet_persisted: false,
        operator_packet_accepted: false,
        operator_packet_delivered: false,
        operator_packet_authorizes_activation: false,
        operator_packet_authorizes_terminal_closure: false,
        trusted_record_acceptance_allowed: false,
        trusted_record_accepted: false,
        terminal_closure_allowed: false,
        terminal_closure_recorded: false,
        terminal_closure_accepted: false,
        activation_allowed: false,
        activation_performed: false,
        receipt_persistence_allowed: false,
        receipt_acceptance_allowed: false,
        receipt_accepted: false,
        ledger_recording_allowed: false,
        ledger_recorded: false,
        index_delivery_allowed: false,
        index_delivered: false,
        completion_ack_acceptance_allowed: false,
        completion_ack_accepted: false,
        public_release_claim_allowed: false,
        release_artifact_write_allowed: false,
        provider_model_invocation_allowed: false,
        channel_delivery_allowed: false,
        install_restart_allowed: false,
        active_binary_mutation_allowed: false,
        upstream_fetch_merge_allowed: false,
        credential_read_allowed: false,
        secret_value_read_allowed: false,
        side_effects: {
          workspace_written: false,
          filesystem_written: false,
          memory_store_mutated: false,
          json_report_capture_recorded: false,
          json_report_capture_persisted: false,
          json_report_capture_materialized: false,
          json_report_capture_delivered: false,
          json_report_capture_promoted_to_authority: false,
          summary_index_manifest_json_capture_recorded: false,
          summary_index_manifest_json_capture_persisted: false,
          summary_index_manifest_json_capture_materialized: false,
          summary_index_manifest_json_capture_delivered: false,
          summary_index_manifest_json_capture_promoted_to_authority: false,
          summary_index_manifest_recorded: false,
          summary_index_manifest_persisted: false,
          summary_index_manifest_materialized: false,
          summary_index_manifest_delivered: false,
          summary_index_manifest_promoted_to_authority: false,
          summary_index_recorded: false,
          summary_index_persisted: false,
          summary_index_materialized: false,
          summary_index_delivered: false,
          summary_index_promoted_to_authority: false,
          summary_recorded: false,
          summary_persisted: false,
          summary_materialized: false,
          summary_delivered: false,
          operator_packet_recorded: false,
          operator_packet_persisted: false,
          operator_packet_accepted: false,
          operator_packet_delivered: false,
          trusted_record_recorded: false,
          trusted_record_persisted: false,
          trusted_record_accepted: false,
          trusted_record_delivered: false,
          approval_recorded: false,
          activation_request_recorded: false,
          fresh_evidence_accepted: false,
          receipt_persistence_command_enabled: false,
          receipt_persistence_execution_performed: false,
          receipt_acceptance_recorded: false,
          ledger_recorded: false,
          index_recorded: false,
          delivery_recorded: false,
          completion_ack_recorded: false,
          terminal_closure_recorded: false,
          terminal_closure_persisted: false,
          terminal_closure_accepted: false,
          activation_performed: false,
          provider_invoked: false,
          model_invoked: false,
          channel_send_performed: false,
          telegram_send_performed: false,
          external_send_performed: false,
          release_artifact_written: false,
          public_release_claimed: false,
          install_executed: false,
          launchd_mutated: false,
          service_restarted: false,
          active_binary_mutated: false,
          upstream_fetch_performed: false,
          upstream_merge_performed: false,
          credential_read: false,
          secret_value_read: false
        }
      }
  ')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_gate"
  and .terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_ready == true
  and .trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_status == "blocked"
  and .source_positive_packet_authority_replay_denial_summary_index_manifest_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_gate"
  and .source_positive_packet_authority_replay_denial_summary_index_manifest_status == "blocked"
  and .source_positive_packet_authority_replay_denial_summary_index_status == "blocked"
  and .source_positive_packet_authority_replay_denial_summary_status == "blocked"
  and .source_positive_packet_authority_replay_denial_matrix_status == "blocked"
  and .source_terminal_closure_verdict == "blocked"
  and .required_positive_packet_authority_replay_fixture_count == 12
  and .positive_packet_authority_replay_fixture_count == 12
  and .blocked_positive_packet_authority_replay_fixture_count == 12
  and .allowed_positive_packet_authority_replay_fixture_count == 0
  and .manifested_replay_entry_point_summary_count == 8
  and .manifested_replay_surface_summary_count == 12
  and .manifested_source_summary_family_count == 8
  and .manifested_summary_index_family_count == 8
  and .summary_index_manifest_family_count == 8
  and .captured_summary_index_manifest_family_count == 8
  and (.captured_summary_index_manifest_families | length) == 8
  and (.captured_summary_index_manifest_families | all(.ready == true and .blocked == true and .json_capture_status == "blocked" and .json_capture_authority_allowed == false and .json_capture_recorded == false and .json_capture_persisted == false and .json_capture_materialized == false and .json_capture_delivered == false))
  and .required_json_capture_boundary_family_count == 8
  and .json_capture_boundary_family_count == 8
  and .ready_json_capture_boundary_family_count == 8
  and .activation_blocking_json_capture_boundary_family_count == 8
  and (.json_capture_boundary_families | length) == 8
  and (.json_capture_boundary_families | all(.ready == true and .blocked == true and .json_capture_authority_allowed == false))
  and (.json_capture_boundary_families | any(.id == "source-manifest-json-capture-witness" and .source_status == "blocked"))
  and (.json_capture_boundary_families | any(.id == "replay-fixture-invariant-json-capture-boundary" and .fixture_count == 12 and .blocked_fixture_count == 12 and .allowed_fixture_count == 0))
  and (.json_capture_boundary_families | any(.id == "entry-point-manifest-json-capture-boundary" and .manifested_replay_entry_point_summary_count == 8))
  and (.json_capture_boundary_families | any(.id == "replay-surface-manifest-json-capture-boundary" and .manifested_replay_surface_summary_count == 12))
  and (.json_capture_boundary_families | any(.id == "source-and-index-family-json-capture-boundary" and .manifested_source_summary_family_count == 8 and .manifested_summary_index_family_count == 8 and .summary_index_manifest_family_count == 8))
  and (.json_capture_boundary_families | any(.id == "denial-reason-json-capture-boundary" and .inherited_summary_denial_reason_count == 37 and .source_index_denial_reason_count == 45 and .manifest_denial_reason_count == 53))
  and (.json_capture_boundary_families | any(.id == "authority-nonpromotion-json-capture-boundary" and .trusted_record_acceptance_allowed == false and .terminal_closure_allowed == false and .activation_allowed == false and .public_release_claim_allowed == false))
  and (.json_capture_boundary_families | any(.id == "side-effect-json-capture-boundary" and .source_side_effects_all_false == true and .json_capture_recorded == false and .json_capture_persisted == false and .json_capture_materialized == false and .json_capture_delivered == false and .filesystem_written == false))
  and .inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count == 37
  and .source_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_count == 45
  and .source_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_count == 53
  and .denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_count == 61
  and (.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary | length) == 61
  and .trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_executed == true
  and .json_report_capture_helper_used == true
  and .json_capture_boundary_authority_allowed == false
  and .json_capture_boundary_recorded == false
  and .json_capture_boundary_persisted == false
  and .json_capture_boundary_materialized == false
  and .json_capture_boundary_delivered == false
  and .json_capture_boundary_promoted_to_authority == false
  and .summary_index_manifest_authority_allowed == false
  and .summary_index_manifest_recorded == false
  and .summary_index_manifest_persisted == false
  and .summary_index_manifest_materialized == false
  and .summary_index_manifest_delivered == false
  and .summary_index_manifest_promoted_to_authority == false
  and .summary_index_authority_allowed == false
  and .summary_index_recorded == false
  and .summary_index_persisted == false
  and .summary_index_materialized == false
  and .summary_index_delivered == false
  and .summary_index_promoted_to_authority == false
  and .trusted_record_acceptance_allowed == false
  and .trusted_record_accepted == false
  and .terminal_closure_allowed == false
  and .terminal_closure_recorded == false
  and .terminal_closure_accepted == false
  and .activation_allowed == false
  and .activation_performed == false
  and .receipt_accepted == false
  and .ledger_recorded == false
  and .index_delivered == false
  and .completion_ack_accepted == false
  and .public_release_claim_allowed == false
  and .release_artifact_write_allowed == false
  and .provider_model_invocation_allowed == false
  and .channel_delivery_allowed == false
  and .install_restart_allowed == false
  and .active_binary_mutation_allowed == false
  and .upstream_fetch_merge_allowed == false
  and .credential_read_allowed == false
  and .secret_value_read_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
