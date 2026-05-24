#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECORDING_DENIAL_MATRIX.md"

echo "[hepta-upstream-codex-activation-evidence-recording-denial-matrix] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_recording_denial_matrix -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence recording denial matrix missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence recording denial matrix: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-recording-denial-matrix"
require_doc_text "Source evidence recording dry-run receipt gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-recording-dry-run-receipt.sh\`"
require_doc_text "Evidence recording denial matrix gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh\`"
require_doc_text "Required denied attempt count: \`3\`"
require_doc_text "Denied receipt attempt count: \`3\`"
require_doc_text "Allowed receipt attempt count: \`0\`"
require_doc_text "Max recorded receipt field count: \`12\`"
require_doc_text "Max accepted trusted record count: \`8\`"
require_doc_text "Max fresh trusted record count: \`8\`"
require_doc_text "Public claim attempt count: \`1\`"
require_doc_text "Release artifact write attempt count: \`1\`"
require_doc_text "Receipt sink write performed: \`false\`"
require_doc_text "Evidence receipt persisted: \`false\`"
require_doc_text "Trusted record materialized: \`false\`"
require_doc_text "No-write sink ready: \`true\`"
require_doc_text "Activation blocked by no-write sink: \`true\`"
require_doc_text "Activation allowed by no-write sink: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "\`partial-receipt-fields\`"
require_doc_text "\`operator-approved-but-stale-evidence\`"
require_doc_text "\`public-claim-release-artifact-attempt\`"
require_doc_text "No receipt persistence"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg matrix_id "upstream-codex-activation-evidence-recording-denial-matrix" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-evidence-recording-dry-run-receipt.sh" \
    --arg matrix_gate "scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      matrix_id:$matrix_id,
      manifest:$manifest,
      matrix_doc_path:$doc,
      source_evidence_recording_dry_run_receipt_gate:$source_gate,
      evidence_recording_denial_matrix_gate:$matrix_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      matrix_status:{
        source_receipt_gate_ready:true,
        required_denied_attempt_count:3,
        denied_receipt_attempt_count:3,
        allowed_receipt_attempt_count:0,
        max_recorded_receipt_field_count:12,
        max_accepted_trusted_record_count:8,
        max_fresh_trusted_record_count:8,
        public_claim_attempt_count:1,
        release_artifact_write_attempt_count:1,
        receipt_sink_write_performed:false,
        evidence_receipt_persisted:false,
        trusted_record_materialized:false,
        no_write_sink_ready:true,
        activation_blocked_by_no_write_sink:true,
        activation_allowed_by_no_write_sink:false,
        active_wiring_allowed:false
      },
      denied_receipt_attempts:[
        {
          attempt_id:"partial-receipt-fields",
          attempt_kind:"partial_receipt_fields",
          recorded_receipt_field_count:5,
          accepted_trusted_record_count:3,
          fresh_trusted_record_count:0,
          denial_status:"blocked",
          receipt_materialized:false,
          workspace_write_allowed:false,
          active_wiring_allowed:false,
          public_release_claim_allowed:false,
          release_artifact_write_allowed:false
        },
        {
          attempt_id:"operator-approved-but-stale-evidence",
          attempt_kind:"operator_approved_stale_evidence",
          recorded_receipt_field_count:12,
          accepted_trusted_record_count:8,
          fresh_trusted_record_count:0,
          denial_status:"blocked",
          receipt_materialized:false,
          workspace_write_allowed:false,
          active_wiring_allowed:false,
          public_release_claim_allowed:false,
          release_artifact_write_allowed:false
        },
        {
          attempt_id:"public-claim-release-artifact-attempt",
          attempt_kind:"public_claim_release_artifact_attempt",
          recorded_receipt_field_count:12,
          accepted_trusted_record_count:8,
          fresh_trusted_record_count:8,
          public_claim_requested:true,
          release_artifact_write_requested:true,
          denial_status:"blocked",
          receipt_materialized:false,
          workspace_write_allowed:false,
          active_wiring_allowed:false,
          public_release_claim_allowed:false,
          release_artifact_write_allowed:false
        }
      ],
      denied_active_decisions:{
        active_runtime_code_wiring_allowed:false,
        active_runtime_dependency_allowed:false,
        active_runtime_auto_rebase_allowed:false,
        active_codex_engine_dependency_allowed:false,
        public_release_claim_allowed:false,
        public_ga_claim_allowed:false,
        release_artifact_write_allowed:false
      },
      side_effects:{
        upstream_fetch_performed:false,
        upstream_merge_performed:false,
        upstream_checkout_performed:false,
        workspace_write:false,
        receipt_persistence:false,
        active_service_restart:false,
        credential_value_read:false,
        secret_file_read:false,
        provider_invocation:false,
        channel_delivery:false,
        gateway_rpc:false,
        public_release:false
      }
    }'
)"

printf '%s\n' "$report"

echo "Hepta upstream Codex activation evidence recording denial matrix gate passed"
