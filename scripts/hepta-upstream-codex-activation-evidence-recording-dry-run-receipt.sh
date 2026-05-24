#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECORDING_DRY_RUN_RECEIPT.md"

echo "[hepta-upstream-codex-activation-evidence-recording-dry-run-receipt] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_recording_dry_run_receipt -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence recording dry-run receipt missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence recording dry-run receipt: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-recording-dry-run-receipt"
require_doc_text "Source evidence completeness scoreboard gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh\`"
require_doc_text "Evidence recording dry-run receipt gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-recording-dry-run-receipt.sh\`"
require_doc_text "Required receipt field count: \`12\`"
require_doc_text "Recorded receipt field count: \`0\`"
require_doc_text "Redacted or hashed field count: \`10\`"
require_doc_text "Required evidence count: \`8\`"
require_doc_text "Required trusted record count: \`8\`"
require_doc_text "Accepted trusted record count: \`0\`"
require_doc_text "Fresh trusted record count: \`0\`"
require_doc_text "Operator approval recorded: \`false\`"
require_doc_text "Activation request recorded: \`false\`"
require_doc_text "Receipt schema ready: \`true\`"
require_doc_text "Receipt recorded: \`false\`"
require_doc_text "Real evidence recorded: \`false\`"
require_doc_text "Trusted record materialized: \`false\`"
require_doc_text "Evidence recording dry-run ready: \`true\`"
require_doc_text "Activation blocked by receipt: \`true\`"
require_doc_text "Activation allowed by receipt: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "\`operator_identity_hash\`"
require_doc_text "\`artifact_sha256_or_redacted_path_bundle\`"
require_doc_text "No upstream fetch"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg receipt_id "upstream-codex-activation-evidence-recording-dry-run-receipt" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh" \
    --arg receipt_gate "scripts/hepta-upstream-codex-activation-evidence-recording-dry-run-receipt.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      receipt_id:$receipt_id,
      manifest:$manifest,
      receipt_doc_path:$doc,
      source_evidence_completeness_scoreboard_gate:$source_gate,
      evidence_recording_dry_run_receipt_gate:$receipt_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      receipt_status:{
        source_scoreboard_ready:true,
        required_receipt_field_count:12,
        recorded_receipt_field_count:0,
        redacted_or_hashed_field_count:10,
        required_evidence_count:8,
        required_trusted_record_count:8,
        accepted_trusted_record_count:0,
        fresh_trusted_record_count:0,
        operator_approval_recorded:false,
        activation_request_recorded:false,
        receipt_schema_ready:true,
        receipt_recorded:false,
        real_evidence_recorded:false,
        trusted_record_materialized:false,
        public_claim_attempt_blocked:true,
        release_artifact_write_attempt_blocked:true,
        evidence_recording_dry_run_ready:true,
        activation_blocked_by_receipt:true,
        activation_allowed_by_receipt:false,
        receipt_denial_reason:"recording receipt is schema-only; no real activation request, operator approval, fresh trusted records, or workspace write is present",
        active_wiring_allowed:false
      },
      required_receipt_fields:[
        "evidence_recording_receipt_id",
        "activation_request_id",
        "operator_approval_id",
        "operator_identity_hash",
        "accepted_trusted_record_ids",
        "fresh_trusted_record_ids",
        "active_binary_sha256",
        "route_or_status_hash_bundle",
        "artifact_sha256_or_redacted_path_bundle",
        "freshness_window_summary",
        "rollback_plan_id",
        "public_claim_and_artifact_decision"
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

echo "Hepta upstream Codex activation evidence recording dry-run receipt gate passed"
