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

SUMMARY_INDEX_JSON="$(
  capture_json_report \
    "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-gate.sh
)"

summary_index_report_sha256="$(sha256_text "$SUMMARY_INDEX_JSON")"
summary_index_manifest_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest:manifest:$summary_index_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
summary_index_manifest_policy_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest:policy:$summary_index_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
summary_index_manifest_denial_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest:denial:$summary_index_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
summary_index_manifest_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest:side-effects:$summary_index_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson index "$SUMMARY_INDEX_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $index.runtime == "hepta"
    and $index.status == "ready"
    and $index.gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_gate"
    and $index.terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_ready == true
    and $index.trusted_record_positive_packet_authority_replay_denial_summary_index_status == "blocked"
    and $index.source_positive_packet_authority_replay_denial_summary_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_gate"
    and $index.source_positive_packet_authority_replay_denial_summary_status == "blocked"
    and $index.source_positive_packet_authority_replay_denial_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_gate"
    and $index.source_positive_packet_authority_replay_denial_matrix_status == "blocked"
    and $index.source_terminal_closure_verdict == "blocked"
    and $index.required_positive_packet_authority_replay_fixture_count == 12
    and $index.positive_packet_authority_replay_fixture_count == 12
    and $index.blocked_positive_packet_authority_replay_fixture_count == 12
    and $index.allowed_positive_packet_authority_replay_fixture_count == 0
    and $index.required_indexed_replay_entry_point_summary_count == 8
    and $index.indexed_replay_entry_point_summary_count == 8
    and $index.blocked_indexed_replay_entry_point_summary_count == 8
    and ($index.indexed_replay_entry_point_summary | length) == 8
    and ($index.indexed_replay_entry_point_summary | all(.index_status == "blocked" and .summary_index_authority_allowed == false and .replay_authority_allowed == false))
    and $index.required_indexed_replay_surface_summary_count == 12
    and $index.indexed_replay_surface_summary_count == 12
    and $index.blocked_indexed_replay_surface_summary_count == 12
    and ($index.indexed_replay_surface_summary | length) == 12
    and ($index.indexed_replay_surface_summary | all(.index_status == "blocked" and .summary_index_authority_allowed == false and .replay_authority_allowed == false))
    and $index.required_summary_index_family_count == 8
    and $index.summary_index_family_count == 8
    and $index.ready_summary_index_family_count == 8
    and $index.activation_blocking_summary_index_family_count == 8
    and ($index.indexed_source_summary_families | length) == 8
    and ($index.indexed_source_summary_families | all(.index_status == "blocked" and .summary_index_authority_allowed == false))
    and ($index.summary_index_families | length) == 8
    and ($index.summary_index_families | all(.ready == true and .blocked == true))
    and $index.inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count == 37
    and ($index.inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary | length) == 37
    and $index.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_count == 45
    and ($index.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index | length) == 45
    and $index.summary_index_authority_allowed == false
    and $index.summary_index_recorded == false
    and $index.summary_index_persisted == false
    and $index.summary_index_materialized == false
    and $index.summary_index_delivered == false
    and $index.summary_index_promoted_to_authority == false
    and $index.trusted_record_acceptance_allowed == false
    and $index.trusted_record_accepted == false
    and $index.terminal_closure_allowed == false
    and $index.terminal_closure_recorded == false
    and $index.activation_allowed == false
    and $index.activation_performed == false
    and $index.receipt_accepted == false
    and $index.ledger_recorded == false
    and $index.index_delivered == false
    and $index.completion_ack_accepted == false
    and $index.public_release_claim_allowed == false
    and $index.release_artifact_write_allowed == false
    and ($index.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_gate" \
  --arg summary_index_report_sha256 "$summary_index_report_sha256" \
  --arg summary_index_manifest_hash_sha256 "$summary_index_manifest_hash_sha256" \
  --arg summary_index_manifest_policy_hash_sha256 "$summary_index_manifest_policy_hash_sha256" \
  --arg summary_index_manifest_denial_hash_sha256 "$summary_index_manifest_denial_hash_sha256" \
  --arg summary_index_manifest_side_effect_hash_sha256 "$summary_index_manifest_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson index "$SUMMARY_INDEX_JSON" \
  '
    ($index.indexed_replay_entry_point_summary
      | map(. + {
          manifest_status: "blocked",
          manifest_recorded: false,
          manifest_persisted: false,
          manifest_delivered: false,
          manifest_authority_allowed: false,
          terminal_closure_allowed: false,
          activation_allowed: false
        })) as $entry_point_manifest
    | ($index.indexed_replay_surface_summary
      | map(. + {
          manifest_status: "blocked",
          manifest_recorded: false,
          manifest_persisted: false,
          manifest_delivered: false,
          manifest_authority_allowed: false,
          terminal_closure_allowed: false,
          activation_allowed: false
        })) as $surface_manifest
    | ($index.indexed_source_summary_families
      | map(. + {
          manifest_status: "blocked",
          manifest_recorded: false,
          manifest_persisted: false,
          manifest_delivered: false,
          manifest_authority_allowed: false,
          terminal_closure_allowed: false,
          activation_allowed: false
        })) as $source_family_manifest
    | ($index.summary_index_families
      | map(. + {
          manifest_status: "blocked",
          manifest_recorded: false,
          manifest_persisted: false,
          manifest_delivered: false,
          manifest_authority_allowed: false,
          terminal_closure_allowed: false,
          activation_allowed: false
        })) as $index_family_manifest
    | ([
        {
          id: "source-summary-index-report-manifest",
          ready: true,
          blocked: true,
          witness_sha256: $summary_index_report_sha256,
          source_gate: $index.gate,
          source_status: $index.trusted_record_positive_packet_authority_replay_denial_summary_index_status,
          manifest_authority_allowed: false,
          reason: "the source summary index is attached as a witness hash only"
        },
        {
          id: "replay-fixture-denial-manifest",
          ready: true,
          blocked: true,
          fixture_count: $index.positive_packet_authority_replay_fixture_count,
          blocked_fixture_count: $index.blocked_positive_packet_authority_replay_fixture_count,
          allowed_fixture_count: $index.allowed_positive_packet_authority_replay_fixture_count,
          manifest_authority_allowed: false,
          reason: "all shape-complete positive packet authority replays remain blocked"
        },
        {
          id: "entry-point-summary-manifest",
          ready: true,
          blocked: true,
          manifested_replay_entry_point_summary_count: ($entry_point_manifest | length),
          manifest_authority_allowed: false,
          reason: "entry point summaries stay blocked and non-authoritative in the manifest"
        },
        {
          id: "replay-surface-summary-manifest",
          ready: true,
          blocked: true,
          manifested_replay_surface_summary_count: ($surface_manifest | length),
          manifest_authority_allowed: false,
          reason: "surface summaries stay blocked and non-authoritative in the manifest"
        },
        {
          id: "source-summary-family-manifest",
          ready: true,
          blocked: true,
          manifested_source_summary_family_count: ($source_family_manifest | length),
          manifest_authority_allowed: false,
          reason: "source summary families remain activation-blocking"
        },
        {
          id: "summary-index-family-manifest",
          ready: true,
          blocked: true,
          manifested_summary_index_family_count: ($index_family_manifest | length),
          manifest_authority_allowed: false,
          reason: "summary-index families remain blocked and cannot promote the index"
        },
        {
          id: "denial-reason-manifest",
          ready: true,
          blocked: true,
          inherited_summary_denial_reason_count: $index.inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count,
          source_index_denial_reason_count: $index.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_count,
          manifest_authority_allowed: false,
          reason: "all inherited and index-level denial reasons stay attached to the manifest"
        },
        {
          id: "side-effect-boundary-manifest",
          ready: true,
          blocked: true,
          source_side_effects_all_false: ($index.side_effects | to_entries | all(.value == false)),
          manifest_recorded: false,
          manifest_persisted: false,
          manifest_materialized: false,
          manifest_delivered: false,
          manifest_authority_allowed: false,
          reason: "the manifest is stdout-only and records no side effects"
        }
      ]) as $manifest_families
    | ([
        "positive_packet_authority_replay_denial_summary_index_manifest_recording_denied",
        "positive_packet_authority_replay_denial_summary_index_manifest_persistence_denied",
        "positive_packet_authority_replay_denial_summary_index_manifest_materialization_denied",
        "positive_packet_authority_replay_denial_summary_index_manifest_delivery_denied",
        "positive_packet_authority_replay_denial_summary_index_manifest_authority_promotion_denied",
        "positive_packet_authority_replay_denial_summary_index_manifest_terminal_closure_denied",
        "positive_packet_authority_replay_denial_summary_index_manifest_activation_denied",
        "positive_packet_authority_replay_denial_summary_index_manifest_public_release_claim_denied"
      ] + $index.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index) as $manifest_denied
    | {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_schema_version: "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_v1",
        terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_ready: true,
        trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_mode: "stdout_only_report_only_positive_packet_authority_replay_denial_summary_index_manifest_no_approval_no_persistence_no_acceptance_no_delivery_no_activation",
        trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_status: "blocked",
        trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_decision: "positive_packet_authority_replay_denial_summary_index_manifested_without_recording_persisting_materializing_delivering_promoting_closing_activating_or_releasing",
        source_positive_packet_authority_replay_denial_summary_index_gate: $index.gate,
        source_positive_packet_authority_replay_denial_summary_index_status: $index.trusted_record_positive_packet_authority_replay_denial_summary_index_status,
        source_positive_packet_authority_replay_denial_summary_index_report_sha256: $summary_index_report_sha256,
        source_positive_packet_authority_replay_denial_summary_gate: $index.source_positive_packet_authority_replay_denial_summary_gate,
        source_positive_packet_authority_replay_denial_summary_status: $index.source_positive_packet_authority_replay_denial_summary_status,
        source_positive_packet_authority_replay_denial_summary_report_sha256: $index.source_positive_packet_authority_replay_denial_summary_report_sha256,
        source_positive_packet_authority_replay_denial_matrix_gate: $index.source_positive_packet_authority_replay_denial_matrix_gate,
        source_positive_packet_authority_replay_denial_matrix_status: $index.source_positive_packet_authority_replay_denial_matrix_status,
        source_positive_packet_authority_replay_denial_matrix_report_sha256: $index.source_positive_packet_authority_replay_denial_matrix_report_sha256,
        source_terminal_closure_gate: $index.source_terminal_closure_gate,
        source_terminal_closure_verdict: $index.source_terminal_closure_verdict,
        positive_packet_authority_replay_denial_summary_index_manifest_hash_sha256: $summary_index_manifest_hash_sha256,
        positive_packet_authority_replay_denial_summary_index_manifest_policy_hash_sha256: $summary_index_manifest_policy_hash_sha256,
        positive_packet_authority_replay_denial_summary_index_manifest_denial_hash_sha256: $summary_index_manifest_denial_hash_sha256,
        positive_packet_authority_replay_denial_summary_index_manifest_side_effect_hash_sha256: $summary_index_manifest_side_effect_hash_sha256,
        source_positive_packet_authority_replay_denial_summary_index_hash_sha256: $index.positive_packet_authority_replay_denial_summary_index_hash_sha256,
        source_positive_packet_authority_replay_denial_summary_index_policy_hash_sha256: $index.positive_packet_authority_replay_denial_summary_index_policy_hash_sha256,
        source_positive_packet_authority_replay_denial_summary_index_denial_hash_sha256: $index.positive_packet_authority_replay_denial_summary_index_denial_hash_sha256,
        source_positive_packet_authority_replay_denial_summary_index_side_effect_hash_sha256: $index.positive_packet_authority_replay_denial_summary_index_side_effect_hash_sha256,
        minimum_required_long_soak_samples: $min_long_soak_samples,
        source_positive_packet_fixture_count: $index.source_positive_packet_fixture_count,
        source_blocked_positive_packet_fixture_count: $index.source_blocked_positive_packet_fixture_count,
        source_accepted_positive_packet_fixture_count: $index.source_accepted_positive_packet_fixture_count,
        source_positive_packet_scoreboard_item_count: $index.source_positive_packet_scoreboard_item_count,
        source_positive_packet_satisfied_scoreboard_item_count: $index.source_positive_packet_satisfied_scoreboard_item_count,
        source_positive_packet_trusted_record_count: $index.source_positive_packet_trusted_record_count,
        source_positive_packet_future_positive_family_count: $index.source_positive_packet_future_positive_family_count,
        required_positive_packet_authority_replay_fixture_count: $index.required_positive_packet_authority_replay_fixture_count,
        positive_packet_authority_replay_fixture_count: $index.positive_packet_authority_replay_fixture_count,
        blocked_positive_packet_authority_replay_fixture_count: $index.blocked_positive_packet_authority_replay_fixture_count,
        allowed_positive_packet_authority_replay_fixture_count: $index.allowed_positive_packet_authority_replay_fixture_count,
        required_manifested_replay_entry_point_summary_count: 8,
        manifested_replay_entry_point_summary_count: ($entry_point_manifest | length),
        blocked_manifested_replay_entry_point_summary_count: ($entry_point_manifest | map(select(.manifest_status == "blocked")) | length),
        manifested_replay_entry_point_summary: $entry_point_manifest,
        required_manifested_replay_surface_summary_count: 12,
        manifested_replay_surface_summary_count: ($surface_manifest | length),
        blocked_manifested_replay_surface_summary_count: ($surface_manifest | map(select(.manifest_status == "blocked")) | length),
        manifested_replay_surface_summary: $surface_manifest,
        required_manifested_source_summary_family_count: 8,
        manifested_source_summary_family_count: ($source_family_manifest | length),
        blocked_manifested_source_summary_family_count: ($source_family_manifest | map(select(.manifest_status == "blocked")) | length),
        manifested_source_summary_families: $source_family_manifest,
        required_manifested_summary_index_family_count: 8,
        manifested_summary_index_family_count: ($index_family_manifest | length),
        blocked_manifested_summary_index_family_count: ($index_family_manifest | map(select(.manifest_status == "blocked")) | length),
        manifested_summary_index_families: $index_family_manifest,
        required_summary_index_manifest_family_count: 8,
        summary_index_manifest_family_count: ($manifest_families | length),
        ready_summary_index_manifest_family_count: ($manifest_families | map(select(.ready == true)) | length),
        activation_blocking_summary_index_manifest_family_count: ($manifest_families | map(select(.blocked == true)) | length),
        summary_index_manifest_families: $manifest_families,
        inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count: $index.inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count,
        source_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_count: $index.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_count,
        denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest: $manifest_denied,
        denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_count: ($manifest_denied | length),
        trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_executed: true,
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
  and .gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_gate"
  and .terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_schema_version == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_v1"
  and .terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_ready == true
  and .trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_mode == "stdout_only_report_only_positive_packet_authority_replay_denial_summary_index_manifest_no_approval_no_persistence_no_acceptance_no_delivery_no_activation"
  and .trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_status == "blocked"
  and .source_positive_packet_authority_replay_denial_summary_index_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_gate"
  and .source_positive_packet_authority_replay_denial_summary_index_status == "blocked"
  and .source_positive_packet_authority_replay_denial_summary_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_gate"
  and .source_positive_packet_authority_replay_denial_summary_status == "blocked"
  and .source_positive_packet_authority_replay_denial_matrix_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_matrix_gate"
  and .source_positive_packet_authority_replay_denial_matrix_status == "blocked"
  and .source_terminal_closure_verdict == "blocked"
  and .required_positive_packet_authority_replay_fixture_count == 12
  and .positive_packet_authority_replay_fixture_count == 12
  and .blocked_positive_packet_authority_replay_fixture_count == 12
  and .allowed_positive_packet_authority_replay_fixture_count == 0
  and .required_manifested_replay_entry_point_summary_count == 8
  and .manifested_replay_entry_point_summary_count == 8
  and .blocked_manifested_replay_entry_point_summary_count == 8
  and (.manifested_replay_entry_point_summary | length) == 8
  and (.manifested_replay_entry_point_summary | all(
    .index_status == "blocked"
    and .manifest_status == "blocked"
    and .summary_index_authority_allowed == false
    and .manifest_authority_allowed == false
    and .manifest_recorded == false
    and .manifest_persisted == false
    and .manifest_delivered == false
    and .terminal_closure_allowed == false
    and .activation_allowed == false
  ))
  and .required_manifested_replay_surface_summary_count == 12
  and .manifested_replay_surface_summary_count == 12
  and .blocked_manifested_replay_surface_summary_count == 12
  and (.manifested_replay_surface_summary | length) == 12
  and (.manifested_replay_surface_summary | all(
    .index_status == "blocked"
    and .manifest_status == "blocked"
    and .summary_index_authority_allowed == false
    and .manifest_authority_allowed == false
    and .manifest_recorded == false
    and .manifest_persisted == false
    and .manifest_delivered == false
    and .terminal_closure_allowed == false
    and .activation_allowed == false
  ))
  and .required_manifested_source_summary_family_count == 8
  and .manifested_source_summary_family_count == 8
  and .blocked_manifested_source_summary_family_count == 8
  and (.manifested_source_summary_families | length) == 8
  and (.manifested_source_summary_families | all(.manifest_status == "blocked" and .manifest_authority_allowed == false))
  and .required_manifested_summary_index_family_count == 8
  and .manifested_summary_index_family_count == 8
  and .blocked_manifested_summary_index_family_count == 8
  and (.manifested_summary_index_families | length) == 8
  and (.manifested_summary_index_families | all(.manifest_status == "blocked" and .manifest_authority_allowed == false))
  and .required_summary_index_manifest_family_count == 8
  and .summary_index_manifest_family_count == 8
  and .ready_summary_index_manifest_family_count == 8
  and .activation_blocking_summary_index_manifest_family_count == 8
  and (.summary_index_manifest_families | length) == 8
  and (.summary_index_manifest_families | all(.ready == true and .blocked == true and .manifest_authority_allowed == false))
  and (.summary_index_manifest_families | any(.id == "source-summary-index-report-manifest" and .source_status == "blocked"))
  and (.summary_index_manifest_families | any(.id == "replay-fixture-denial-manifest" and .fixture_count == 12 and .blocked_fixture_count == 12 and .allowed_fixture_count == 0))
  and (.summary_index_manifest_families | any(.id == "entry-point-summary-manifest" and .manifested_replay_entry_point_summary_count == 8))
  and (.summary_index_manifest_families | any(.id == "replay-surface-summary-manifest" and .manifested_replay_surface_summary_count == 12))
  and (.summary_index_manifest_families | any(.id == "source-summary-family-manifest" and .manifested_source_summary_family_count == 8))
  and (.summary_index_manifest_families | any(.id == "summary-index-family-manifest" and .manifested_summary_index_family_count == 8))
  and (.summary_index_manifest_families | any(.id == "denial-reason-manifest" and .inherited_summary_denial_reason_count == 37 and .source_index_denial_reason_count == 45))
  and (.summary_index_manifest_families | any(.id == "side-effect-boundary-manifest" and .source_side_effects_all_false == true and .manifest_recorded == false and .manifest_persisted == false and .manifest_materialized == false and .manifest_delivered == false))
  and .inherited_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_count == 37
  and .source_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_count == 45
  and .denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_count == 53
  and (.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest | length) == 53
  and .trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_executed == true
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
  and .summary_recorded == false
  and .summary_persisted == false
  and .summary_materialized == false
  and .summary_delivered == false
  and .operator_packet_recorded == false
  and .operator_packet_persisted == false
  and .operator_packet_accepted == false
  and .operator_packet_delivered == false
  and .operator_packet_authorizes_activation == false
  and .operator_packet_authorizes_terminal_closure == false
  and .trusted_record_acceptance_allowed == false
  and .trusted_record_accepted == false
  and .terminal_closure_allowed == false
  and .terminal_closure_recorded == false
  and .terminal_closure_accepted == false
  and .activation_allowed == false
  and .activation_performed == false
  and .receipt_persistence_allowed == false
  and .receipt_acceptance_allowed == false
  and .receipt_accepted == false
  and .ledger_recording_allowed == false
  and .ledger_recorded == false
  and .index_delivery_allowed == false
  and .index_delivered == false
  and .completion_ack_acceptance_allowed == false
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
