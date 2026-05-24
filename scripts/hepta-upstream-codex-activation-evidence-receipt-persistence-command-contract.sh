#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_PERSISTENCE_COMMAND_CONTRACT.md"

echo "[hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_receipt_persistence_command_contract -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence receipt persistence command contract missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence receipt persistence command contract: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-receipt-persistence-command-contract"
require_doc_text "Source evidence recording denial matrix gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh\`"
require_doc_text "Receipt persistence command contract gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh\`"
require_doc_text "Required command field count: \`10\`"
require_doc_text "Recorded command field count: \`0\`"
require_doc_text "Redacted or hashed field count: \`9\`"
require_doc_text "Operator approval required: \`true\`"
require_doc_text "Operator approval recorded: \`false\`"
require_doc_text "Activation request required: \`true\`"
require_doc_text "Activation request recorded: \`false\`"
require_doc_text "Receipt persistence command enabled by default: \`false\`"
require_doc_text "Receipt persistence command invoked: \`false\`"
require_doc_text "Receipt persistence execution performed: \`false\`"
require_doc_text "Receipt persistence no-op ready: \`true\`"
require_doc_text "Workspace write performed: \`false\`"
require_doc_text "Evidence receipt persisted: \`false\`"
require_doc_text "Activation blocked by persistence contract: \`true\`"
require_doc_text "Activation allowed by persistence contract: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "\`receipt_output_path_redacted\`"
require_doc_text "\`receipt_payload_hash\`"
require_doc_text "No command invocation"
require_doc_text "No receipt persistence execution"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg command_contract_id "upstream-codex-activation-evidence-receipt-persistence-command-contract" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-evidence-recording-denial-matrix.sh" \
    --arg command_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      command_contract_id:$command_contract_id,
      manifest:$manifest,
      command_contract_doc_path:$doc,
      source_evidence_recording_denial_matrix_gate:$source_gate,
      receipt_persistence_command_contract_gate:$command_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      command_status:{
        source_denial_matrix_ready:true,
        required_command_field_count:10,
        recorded_command_field_count:0,
        redacted_or_hashed_field_count:9,
        operator_approval_required:true,
        operator_approval_recorded:false,
        activation_request_required:true,
        activation_request_recorded:false,
        trusted_record_materialized:false,
        receipt_persistence_command_enabled_by_default:false,
        receipt_persistence_command_invoked:false,
        receipt_persistence_execution_performed:false,
        receipt_persistence_noop_ready:true,
        workspace_write_performed:false,
        evidence_receipt_persisted:false,
        activation_blocked_by_persistence_contract:true,
        activation_allowed_by_persistence_contract:false,
        active_wiring_allowed:false
      },
      required_command_fields:[
        "receipt_persistence_command_id",
        "activation_request_id",
        "operator_approval_id",
        "operator_identity_hash",
        "accepted_trusted_record_ids",
        "fresh_trusted_record_ids",
        "receipt_payload_hash",
        "receipt_output_path_redacted",
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
        command_invocation:false,
        receipt_persistence_execution:false,
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

echo "Hepta upstream Codex activation evidence receipt persistence command contract gate passed"
