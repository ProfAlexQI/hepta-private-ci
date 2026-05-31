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

JSON_CAPTURE_BOUNDARY_JSON="$(
  capture_json_report \
    "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest-json-capture-boundary-gate" \
    env HEPTA_LIVE_URL="$BASE_URL" \
      HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
      scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest-json-capture-boundary-gate.sh
)"

json_capture_boundary_report_sha256="$(sha256_text "$JSON_CAPTURE_BOUNDARY_JSON")"
operator_approval_gap_ledger_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-operator-approval-gap-ledger:ledger:$json_capture_boundary_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
operator_approval_gap_ledger_policy_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-operator-approval-gap-ledger:policy:$json_capture_boundary_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
operator_approval_gap_ledger_denial_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-operator-approval-gap-ledger:denial:$json_capture_boundary_report_sha256:$MIN_LONG_SOAK_SAMPLES")"
operator_approval_gap_ledger_side_effect_hash_sha256="$(sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-operator-approval-gap-ledger:side-effects:$json_capture_boundary_report_sha256:$MIN_LONG_SOAK_SAMPLES")"

jq -n -e \
  --argjson boundary "$JSON_CAPTURE_BOUNDARY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    $boundary.runtime == "hepta"
    and $boundary.status == "ready"
    and $boundary.gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_gate"
    and $boundary.terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_ready == true
    and $boundary.trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_status == "blocked"
    and $boundary.source_positive_packet_authority_replay_denial_summary_index_manifest_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_gate"
    and $boundary.source_positive_packet_authority_replay_denial_summary_index_manifest_status == "blocked"
    and $boundary.source_positive_packet_authority_replay_denial_summary_index_status == "blocked"
    and $boundary.source_positive_packet_authority_replay_denial_summary_status == "blocked"
    and $boundary.source_positive_packet_authority_replay_denial_matrix_status == "blocked"
    and $boundary.source_terminal_closure_verdict == "blocked"
    and $boundary.positive_packet_authority_replay_fixture_count == 12
    and $boundary.blocked_positive_packet_authority_replay_fixture_count == 12
    and $boundary.allowed_positive_packet_authority_replay_fixture_count == 0
    and $boundary.manifested_replay_entry_point_summary_count == 8
    and $boundary.manifested_replay_surface_summary_count == 12
    and $boundary.summary_index_manifest_family_count == 8
    and $boundary.captured_summary_index_manifest_family_count == 8
    and $boundary.required_json_capture_boundary_family_count == 8
    and $boundary.json_capture_boundary_family_count == 8
    and $boundary.ready_json_capture_boundary_family_count == 8
    and $boundary.activation_blocking_json_capture_boundary_family_count == 8
    and $boundary.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_count == 61
    and ($boundary.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary | length) == 61
    and $boundary.json_report_capture_helper_used == true
    and $boundary.json_capture_boundary_authority_allowed == false
    and $boundary.json_capture_boundary_recorded == false
    and $boundary.json_capture_boundary_persisted == false
    and $boundary.json_capture_boundary_materialized == false
    and $boundary.json_capture_boundary_delivered == false
    and $boundary.json_capture_boundary_promoted_to_authority == false
    and $boundary.operator_packet_recorded == false
    and $boundary.operator_packet_persisted == false
    and $boundary.operator_packet_accepted == false
    and $boundary.operator_packet_delivered == false
    and $boundary.operator_packet_authorizes_activation == false
    and $boundary.operator_packet_authorizes_terminal_closure == false
    and $boundary.trusted_record_acceptance_allowed == false
    and $boundary.trusted_record_accepted == false
    and $boundary.terminal_closure_allowed == false
    and $boundary.terminal_closure_recorded == false
    and $boundary.terminal_closure_accepted == false
    and $boundary.activation_allowed == false
    and $boundary.activation_performed == false
    and $boundary.receipt_persistence_allowed == false
    and $boundary.receipt_acceptance_allowed == false
    and $boundary.receipt_accepted == false
    and $boundary.ledger_recording_allowed == false
    and $boundary.ledger_recorded == false
    and $boundary.index_delivery_allowed == false
    and $boundary.index_delivered == false
    and $boundary.completion_ack_acceptance_allowed == false
    and $boundary.completion_ack_accepted == false
    and $boundary.public_release_claim_allowed == false
    and $boundary.release_artifact_write_allowed == false
    and $boundary.provider_model_invocation_allowed == false
    and $boundary.channel_delivery_allowed == false
    and $boundary.install_restart_allowed == false
    and $boundary.active_binary_mutation_allowed == false
    and $boundary.upstream_fetch_merge_allowed == false
    and $boundary.credential_read_allowed == false
    and $boundary.secret_value_read_allowed == false
    and ($boundary.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

ledger_specs=(
  "explicit_operator_approval_record_missing|operator-approval|hepta_core_activation_long_soak_operator_approval_packet_gate|scripts/hepta-core-activation-long-soak-operator-approval-packet-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_LONG_SOAK_OPERATOR_APPROVAL_PACKET_GATE.md#required-missing-records|operator_packet_accepted|operator_approval_record_missing|future_operator_approval_record|explicit operator approval is absent, so the positive packet cannot become trusted-record authority"
  "operator_identity_hash_binding_missing|operator-approval|hepta_core_activation_long_soak_operator_approval_packet_gate|scripts/hepta-core-activation-long-soak-operator-approval-packet-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_LONG_SOAK_OPERATOR_APPROVAL_PACKET_GATE.md#required-missing-records|operator_packet_authorizes_activation|operator_identity_hash_binding_missing|future_operator_identity_attestation|operator identity binding is absent, so approval cannot be accountable or current"
  "activation_request_nonce_generation_acceptance_missing|activation-request|hepta_core_activation_request_monotonic_single_use_approval_nonce_denial_gate|scripts/hepta-core-activation-request-monotonic-single-use-approval-nonce-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_REQUEST_MONOTONIC_SINGLE_USE_APPROVAL_NONCE_DENIAL_GATE.md#blocked-transition-surface|activation_allowed|activation_request_nonce_generation_missing|future_current_activation_request_nonce_generation|activation request id, generation, and single-use nonce acceptance are absent"
  "fresh_long_soak_evidence_acceptance_missing|fresh-evidence|hepta_core_activation_long_soak_observation_freshness_denial_gate|scripts/hepta-core-activation-long-soak-observation-freshness-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_LONG_SOAK_OBSERVATION_FRESHNESS_DENIAL_GATE.md#freshness-boundary|trusted_record_accepted|fresh_long_soak_evidence_not_accepted|future_fresh_long_soak_evidence|fresh long-soak evidence has not been accepted as a current trusted record"
  "fresh_trusted_evidence_record_set_missing|fresh-evidence|hepta_core_activation_fresh_long_soak_evidence_ledger_receipt_gate|scripts/hepta-core-activation-fresh-long-soak-evidence-ledger-receipt-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_FRESH_LONG_SOAK_EVIDENCE_LEDGER_RECEIPT_GATE.md#required-missing-records|trusted_record_accepted|fresh_trusted_evidence_set_missing|future_hash_bound_trusted_evidence_set|trusted evidence records are not accepted, so no evidence set can satisfy activation"
  "filesystem_persistence_approval_missing|receipt-persistence|hepta_core_activation_evidence_receipt_filesystem_persistence_denial_gate|scripts/hepta-core-activation-evidence-receipt-filesystem-persistence-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_PERSISTENCE_DENIAL_GATE.md#persistence-boundary|receipt_persistence_allowed|filesystem_persistence_approval_missing|future_filesystem_persistence_approval|filesystem persistence approval is absent, so receipt materialization remains denied"
  "receipt_persistence_command_enablement_missing|receipt-persistence|hepta_core_activation_evidence_receipt_acceptance_denial_gate|scripts/hepta-core-activation-evidence-receipt-acceptance-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_EVIDENCE_RECEIPT_ACCEPTANCE_DENIAL_GATE.md#receipt-acceptance-boundary|receipt_persistence_allowed|receipt_persistence_command_enablement_missing|future_receipt_persistence_enablement|receipt persistence command enablement remains disabled and unapproved"
  "receipt_persistence_execution_missing|receipt-persistence|hepta_core_activation_evidence_receipt_acceptance_denial_gate|scripts/hepta-core-activation-evidence-receipt-acceptance-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_EVIDENCE_RECEIPT_ACCEPTANCE_DENIAL_GATE.md#receipt-acceptance-boundary|receipt_persistence_allowed|receipt_persistence_execution_missing|future_receipt_persistence_execution_record|receipt persistence execution has not run and has no accepted execution record"
  "receipt_acceptance_missing|receipt-acceptance|hepta_core_activation_evidence_receipt_acceptance_denial_gate|scripts/hepta-core-activation-evidence-receipt-acceptance-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_EVIDENCE_RECEIPT_ACCEPTANCE_DENIAL_GATE.md#receipt-acceptance-boundary|receipt_acceptance_allowed|receipt_acceptance_missing|future_receipt_acceptance_record|receipt acceptance is absent, so no persisted evidence receipt can close activation"
  "ledger_recording_missing|ledger-index-delivery|hepta_core_activation_fresh_long_soak_evidence_ledger_receipt_gate|scripts/hepta-core-activation-fresh-long-soak-evidence-ledger-receipt-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_FRESH_LONG_SOAK_EVIDENCE_LEDGER_RECEIPT_GATE.md#required-missing-records|ledger_recording_allowed|ledger_recording_missing|future_ledger_record|ledger recording is absent, so there is no durable acceptance chain"
  "index_delivery_missing|ledger-index-delivery|hepta_core_activation_evidence_receipt_acceptance_denial_gate|scripts/hepta-core-activation-evidence-receipt-acceptance-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_EVIDENCE_RECEIPT_ACCEPTANCE_DENIAL_GATE.md#receipt-acceptance-boundary|index_delivery_allowed|index_delivery_missing|future_index_delivery_record|index delivery is absent, so no operator-facing completion delivery exists"
  "completion_ack_acceptance_missing|completion-ack-terminal-closure|hepta_core_activation_evidence_receipt_acceptance_denial_gate|scripts/hepta-core-activation-evidence-receipt-acceptance-denial-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_EVIDENCE_RECEIPT_ACCEPTANCE_DENIAL_GATE.md#receipt-acceptance-boundary|completion_ack_acceptance_allowed|completion_ack_acceptance_missing|future_completion_ack_acceptance|completion acknowledgement is absent, so terminal closure cannot be accepted"
  "terminal_closure_record_acceptance_missing|completion-ack-terminal-closure|hepta_core_activation_evidence_receipt_terminal_closure_decision_gate|scripts/hepta-core-activation-evidence-receipt-terminal-closure-decision-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_EVIDENCE_RECEIPT_TERMINAL_CLOSURE_DECISION_GATE.md#terminal-closure-boundary|terminal_closure_allowed|terminal_closure_record_acceptance_missing|future_terminal_closure_record_acceptance|terminal closure remains unrecorded and unaccepted until all authority prerequisites exist"
  "activation_execution_missing|activation-release|hepta_core_activation_evidence_receipt_terminal_closure_decision_gate|scripts/hepta-core-activation-evidence-receipt-terminal-closure-decision-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_EVIDENCE_RECEIPT_TERMINAL_CLOSURE_DECISION_GATE.md#terminal-closure-boundary|activation_allowed|activation_execution_missing|future_activation_execution_authority|activation execution is still denied because terminal closure authority is absent"
  "release_artifact_public_claim_missing|activation-release|hepta_upstream_codex_latest_release_governance_non_activation_gate|scripts/hepta-upstream-codex-latest-release-governance-non-activation-gate.sh|docs/architecture/HEPTA_UPSTREAM_CODEX_LATEST_RELEASE_GOVERNANCE_NON_ACTIVATION_GATE.md#non-activation-boundary|public_release_claim_allowed|release_artifact_public_claim_missing|future_release_governance_acceptance|release artifact writes and public release claims remain denied"
  "provider_channel_install_upstream_secret_boundary_missing|external-side-effect-boundary|hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_gate|scripts/hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-authority-replay-denial-summary-index-manifest-json-capture-boundary-gate.sh|docs/architecture/HEPTA_CORE_ACTIVATION_TERMINAL_CLOSURE_OPERATOR_PACKET_TRUSTED_RECORD_POSITIVE_PACKET_AUTHORITY_REPLAY_DENIAL_SUMMARY_INDEX_MANIFEST_JSON_CAPTURE_BOUNDARY_GATE.md#non-authority-boundary|provider_model_invocation_allowed|external_side_effect_boundary_missing|future_operator_approved_external_surface_scope|provider, channel, install, upstream, credential, and secret surfaces remain denied"
)

ledger_record_lines=()
for ledger_spec in "${ledger_specs[@]}"; do
  IFS='|' read -r item_id family_id source_gate source_gate_path doc_anchor source_field denial_reason_id future_evidence_class denial_reason <<<"$ledger_spec"
  witness_hash_sha256="$(
    sha256_text "hepta-core-activation-terminal-closure-operator-packet-trusted-record-positive-packet-operator-approval-gap-ledger:$item_id:$family_id:$source_gate:$source_field:$denial_reason_id:$json_capture_boundary_report_sha256"
  )"
  ledger_record_lines+=("$ledger_spec|$witness_hash_sha256")
done

operator_approval_gap_ledger_items_json="$(
  printf '%s\n' "${ledger_record_lines[@]}" \
    | jq -R -s \
      --arg json_capture_boundary_report_sha256 "$json_capture_boundary_report_sha256" \
      '
        split("\n")
        | map(select(length > 0))
        | map(split("|") | {
            item_id: .[0],
            family_id: .[1],
            status: "missing",
            ledger_status: "blocked",
            source_gate: .[2],
            source_gate_path: .[3],
            source_report_sha256: $json_capture_boundary_report_sha256,
            doc_anchor: .[4],
            source_field: .[5],
            source_field_value: false,
            current_value: false,
            denial_reason_id: .[6],
            future_evidence_class: .[7],
            denial_reason: .[8],
            witness_hash_sha256: .[9],
            operator_supplied_future_evidence_needed: true,
            terminal_closure_blocking: true,
            activation_blocking: true,
            non_actionable_report_only: true,
            report_only: true,
            records_approval: false,
            records_activation_request: false,
            accepts_trusted_record: false,
            accepts_fresh_evidence: false,
            approves_persistence: false,
            persists_receipt: false,
            accepts_receipt: false,
            records_ledger: false,
            delivers_index: false,
            accepts_completion_ack: false,
            closes_terminal: false,
            activates: false,
            writes_release_artifact: false,
            makes_public_release_claim: false,
            invokes_provider_or_channel: false,
            installs_or_restarts: false,
            mutates_upstream_or_binary: false,
            reads_credentials_or_secrets: false
          })
      '
)"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg base_url "$BASE_URL" \
  --arg gate "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_operator_approval_gap_ledger_gate" \
  --arg json_capture_boundary_report_sha256 "$json_capture_boundary_report_sha256" \
  --arg operator_approval_gap_ledger_hash_sha256 "$operator_approval_gap_ledger_hash_sha256" \
  --arg operator_approval_gap_ledger_policy_hash_sha256 "$operator_approval_gap_ledger_policy_hash_sha256" \
  --arg operator_approval_gap_ledger_denial_hash_sha256 "$operator_approval_gap_ledger_denial_hash_sha256" \
  --arg operator_approval_gap_ledger_side_effect_hash_sha256 "$operator_approval_gap_ledger_side_effect_hash_sha256" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  --argjson boundary "$JSON_CAPTURE_BOUNDARY_JSON" \
  --argjson ledger_items "$operator_approval_gap_ledger_items_json" \
  '
    ($ledger_items | map(.family_id) | unique) as $family_ids
    | ($family_ids | map(. as $family_id
        | ($ledger_items | map(select(.family_id == $family_id))) as $family_items
        | {
            family_id: $family_id,
            status: "blocked",
            ready: true,
            blocked: true,
            ledger_item_count: ($family_items | length),
            missing_item_count: ($family_items | map(select(.status == "missing")) | length),
            report_only_item_count: ($family_items | map(select(.report_only == true)) | length),
            operator_supplied_future_evidence_needed: true,
            terminal_closure_blocking: true,
            activation_blocking: true,
            non_actionable_report_only: true,
            witness_hash_sha256: ($family_items | map(.witness_hash_sha256) | join(":") | @text),
            denial_reason_ids: ($family_items | map(.denial_reason_id))
          }
      )) as $ledger_families
    | ([
        "operator_approval_gap_ledger_not_authority",
        "operator_approval_gap_ledger_items_missing_denied",
        "operator_approval_gap_ledger_future_evidence_required",
        "operator_approval_gap_ledger_trusted_record_acceptance_denied",
        "operator_approval_gap_ledger_receipt_persistence_denied",
        "operator_approval_gap_ledger_terminal_closure_denied",
        "operator_approval_gap_ledger_release_claim_denied",
        "operator_approval_gap_ledger_external_side_effect_denied"
      ] + $boundary.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary) as $ledger_denied
    | {
        product: $product,
        runtime: $runtime,
        status: "ready",
        base_url: $base_url,
        gate: $gate,
        terminal_closure_operator_packet_trusted_record_positive_packet_operator_approval_gap_ledger_schema_version: "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_operator_approval_gap_ledger_v1",
        terminal_closure_operator_packet_trusted_record_positive_packet_operator_approval_gap_ledger_ready: true,
        trusted_record_positive_packet_operator_approval_gap_ledger_mode: "stdout_only_report_only_operator_approval_gap_ledger_no_approval_no_persistence_no_acceptance_no_delivery_no_activation",
        trusted_record_positive_packet_operator_approval_gap_ledger_status: "blocked",
        trusted_record_positive_packet_operator_approval_gap_ledger_decision: "json_capture_boundary_denials_compressed_into_operator_facing_gap_ledger_without_recording_approval_acceptance_receipt_ledger_delivery_terminal_closure_activation_or_release",
        source_positive_packet_json_capture_boundary_gate: $boundary.gate,
        source_positive_packet_json_capture_boundary_status: $boundary.trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_status,
        source_positive_packet_json_capture_boundary_report_sha256: $json_capture_boundary_report_sha256,
        source_positive_packet_json_capture_boundary_hash_sha256: $boundary.positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_hash_sha256,
        source_positive_packet_json_capture_boundary_policy_hash_sha256: $boundary.positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_policy_hash_sha256,
        source_positive_packet_json_capture_boundary_denial_hash_sha256: $boundary.positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_denial_hash_sha256,
        source_positive_packet_json_capture_boundary_side_effect_hash_sha256: $boundary.positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_side_effect_hash_sha256,
        source_positive_packet_authority_replay_denial_summary_index_manifest_gate: $boundary.source_positive_packet_authority_replay_denial_summary_index_manifest_gate,
        source_positive_packet_authority_replay_denial_summary_index_manifest_status: $boundary.source_positive_packet_authority_replay_denial_summary_index_manifest_status,
        source_positive_packet_authority_replay_denial_summary_index_status: $boundary.source_positive_packet_authority_replay_denial_summary_index_status,
        source_positive_packet_authority_replay_denial_summary_status: $boundary.source_positive_packet_authority_replay_denial_summary_status,
        source_positive_packet_authority_replay_denial_matrix_status: $boundary.source_positive_packet_authority_replay_denial_matrix_status,
        source_terminal_closure_verdict: $boundary.source_terminal_closure_verdict,
        operator_approval_gap_ledger_hash_sha256: $operator_approval_gap_ledger_hash_sha256,
        operator_approval_gap_ledger_policy_hash_sha256: $operator_approval_gap_ledger_policy_hash_sha256,
        operator_approval_gap_ledger_denial_hash_sha256: $operator_approval_gap_ledger_denial_hash_sha256,
        operator_approval_gap_ledger_side_effect_hash_sha256: $operator_approval_gap_ledger_side_effect_hash_sha256,
        minimum_required_long_soak_samples: $min_long_soak_samples,
        required_positive_packet_authority_replay_fixture_count: $boundary.required_positive_packet_authority_replay_fixture_count,
        positive_packet_authority_replay_fixture_count: $boundary.positive_packet_authority_replay_fixture_count,
        blocked_positive_packet_authority_replay_fixture_count: $boundary.blocked_positive_packet_authority_replay_fixture_count,
        allowed_positive_packet_authority_replay_fixture_count: $boundary.allowed_positive_packet_authority_replay_fixture_count,
        manifested_replay_entry_point_summary_count: $boundary.manifested_replay_entry_point_summary_count,
        manifested_replay_surface_summary_count: $boundary.manifested_replay_surface_summary_count,
        manifested_source_summary_family_count: $boundary.manifested_source_summary_family_count,
        manifested_summary_index_family_count: $boundary.manifested_summary_index_family_count,
        summary_index_manifest_family_count: $boundary.summary_index_manifest_family_count,
        captured_summary_index_manifest_family_count: $boundary.captured_summary_index_manifest_family_count,
        json_capture_boundary_family_count: $boundary.json_capture_boundary_family_count,
        source_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_count: $boundary.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_count,
        required_operator_approval_gap_ledger_item_count: 16,
        operator_approval_gap_ledger_item_count: ($ledger_items | length),
        missing_operator_approval_gap_ledger_item_count: ($ledger_items | map(select(.status == "missing")) | length),
        report_only_operator_approval_gap_ledger_item_count: ($ledger_items | map(select(.report_only == true)) | length),
        activation_blocking_operator_approval_gap_ledger_item_count: ($ledger_items | map(select(.activation_blocking == true)) | length),
        terminal_closure_blocking_operator_approval_gap_ledger_item_count: ($ledger_items | map(select(.terminal_closure_blocking == true)) | length),
        required_operator_approval_gap_ledger_family_count: 9,
        operator_approval_gap_ledger_family_count: ($ledger_families | length),
        ready_operator_approval_gap_ledger_family_count: ($ledger_families | map(select(.ready == true)) | length),
        activation_blocking_operator_approval_gap_ledger_family_count: ($ledger_families | map(select(.activation_blocking == true)) | length),
        operator_approval_gap_ledger_families: $ledger_families,
        operator_approval_gap_ledger_items: $ledger_items,
        denied_by_trusted_record_positive_packet_operator_approval_gap_ledger: $ledger_denied,
        denied_by_trusted_record_positive_packet_operator_approval_gap_ledger_count: ($ledger_denied | length),
        inherited_denied_by_positive_packet_json_capture_boundary_count: $boundary.denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_count,
        trusted_record_positive_packet_operator_approval_gap_ledger_executed: true,
        json_report_capture_helper_used: true,
        operator_approval_gap_ledger_authority_allowed: false,
        operator_approval_gap_ledger_recorded: false,
        operator_approval_gap_ledger_persisted: false,
        operator_approval_gap_ledger_materialized: false,
        operator_approval_gap_ledger_delivered: false,
        operator_approval_gap_ledger_promoted_to_authority: false,
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
          operator_approval_gap_ledger_recorded: false,
          operator_approval_gap_ledger_persisted: false,
          operator_approval_gap_ledger_materialized: false,
          operator_approval_gap_ledger_delivered: false,
          operator_approval_gap_ledger_promoted_to_authority: false,
          json_report_capture_recorded: false,
          json_report_capture_persisted: false,
          json_report_capture_materialized: false,
          json_report_capture_delivered: false,
          json_report_capture_promoted_to_authority: false,
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
  and .gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_operator_approval_gap_ledger_gate"
  and .terminal_closure_operator_packet_trusted_record_positive_packet_operator_approval_gap_ledger_ready == true
  and .trusted_record_positive_packet_operator_approval_gap_ledger_status == "blocked"
  and .source_positive_packet_json_capture_boundary_gate == "hepta_core_activation_terminal_closure_operator_packet_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_gate"
  and .source_positive_packet_json_capture_boundary_status == "blocked"
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
  and .summary_index_manifest_family_count == 8
  and .captured_summary_index_manifest_family_count == 8
  and .json_capture_boundary_family_count == 8
  and .source_denied_by_trusted_record_positive_packet_authority_replay_denial_summary_index_manifest_json_capture_boundary_count == 61
  and .required_operator_approval_gap_ledger_item_count == 16
  and .operator_approval_gap_ledger_item_count == 16
  and .missing_operator_approval_gap_ledger_item_count == 16
  and .report_only_operator_approval_gap_ledger_item_count == 16
  and .activation_blocking_operator_approval_gap_ledger_item_count == 16
  and .terminal_closure_blocking_operator_approval_gap_ledger_item_count == 16
  and .required_operator_approval_gap_ledger_family_count == 9
  and .operator_approval_gap_ledger_family_count == 9
  and .ready_operator_approval_gap_ledger_family_count == 9
  and .activation_blocking_operator_approval_gap_ledger_family_count == 9
  and (.operator_approval_gap_ledger_families | length) == 9
  and (.operator_approval_gap_ledger_families | all(.ready == true and .blocked == true and .operator_supplied_future_evidence_needed == true and .non_actionable_report_only == true))
  and (.operator_approval_gap_ledger_items | length) == 16
  and (.operator_approval_gap_ledger_items | all(
    .status == "missing"
    and .ledger_status == "blocked"
    and .source_field_value == false
    and .current_value == false
    and .operator_supplied_future_evidence_needed == true
    and .terminal_closure_blocking == true
    and .activation_blocking == true
    and .non_actionable_report_only == true
    and .report_only == true
    and .records_approval == false
    and .records_activation_request == false
    and .accepts_trusted_record == false
    and .accepts_fresh_evidence == false
    and .approves_persistence == false
    and .persists_receipt == false
    and .accepts_receipt == false
    and .records_ledger == false
    and .delivers_index == false
    and .accepts_completion_ack == false
    and .closes_terminal == false
    and .activates == false
    and .writes_release_artifact == false
    and .makes_public_release_claim == false
    and .invokes_provider_or_channel == false
    and .installs_or_restarts == false
    and .mutates_upstream_or_binary == false
    and .reads_credentials_or_secrets == false
  ))
  and (.operator_approval_gap_ledger_items | any(.item_id == "explicit_operator_approval_record_missing" and .family_id == "operator-approval"))
  and (.operator_approval_gap_ledger_items | any(.item_id == "activation_request_nonce_generation_acceptance_missing" and .family_id == "activation-request"))
  and (.operator_approval_gap_ledger_items | any(.item_id == "fresh_trusted_evidence_record_set_missing" and .family_id == "fresh-evidence"))
  and (.operator_approval_gap_ledger_items | any(.item_id == "receipt_persistence_execution_missing" and .family_id == "receipt-persistence"))
  and (.operator_approval_gap_ledger_items | any(.item_id == "receipt_acceptance_missing" and .family_id == "receipt-acceptance"))
  and (.operator_approval_gap_ledger_items | any(.item_id == "ledger_recording_missing" and .family_id == "ledger-index-delivery"))
  and (.operator_approval_gap_ledger_items | any(.item_id == "completion_ack_acceptance_missing" and .family_id == "completion-ack-terminal-closure"))
  and (.operator_approval_gap_ledger_items | any(.item_id == "release_artifact_public_claim_missing" and .family_id == "activation-release"))
  and (.operator_approval_gap_ledger_items | any(.item_id == "provider_channel_install_upstream_secret_boundary_missing" and .family_id == "external-side-effect-boundary"))
  and .denied_by_trusted_record_positive_packet_operator_approval_gap_ledger_count == 69
  and (.denied_by_trusted_record_positive_packet_operator_approval_gap_ledger | length) == 69
  and .inherited_denied_by_positive_packet_json_capture_boundary_count == 61
  and .trusted_record_positive_packet_operator_approval_gap_ledger_executed == true
  and .json_report_capture_helper_used == true
  and .operator_approval_gap_ledger_authority_allowed == false
  and .operator_approval_gap_ledger_recorded == false
  and .operator_approval_gap_ledger_persisted == false
  and .operator_approval_gap_ledger_materialized == false
  and .operator_approval_gap_ledger_delivered == false
  and .operator_approval_gap_ledger_promoted_to_authority == false
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
