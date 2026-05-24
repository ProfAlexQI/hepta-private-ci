#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_PERSISTENCE_APPROVAL_PACKET.md"

echo "[hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_receipt_filesystem_persistence_approval_packet -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence receipt filesystem persistence approval packet missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence receipt filesystem persistence approval packet: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet"
require_doc_text "Source materialization dry-run gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-materialization-dry-run.sh\`"
require_doc_text "Filesystem persistence approval packet gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh\`"
require_doc_text "Required approval field count: \`12\`"
require_doc_text "Approval field count: \`12\`"
require_doc_text "Recorded approval field count: \`0\`"
require_doc_text "Redacted or hashed field count: \`10\`"
require_doc_text "Required for filesystem persistence field count: \`12\`"
require_doc_text "Operator approval required: \`true\`"
require_doc_text "Operator approval recorded: \`false\`"
require_doc_text "Activation request required: \`true\`"
require_doc_text "Activation request recorded: \`false\`"
require_doc_text "Materialization plan required: \`true\`"
require_doc_text "Materialization plan recorded: \`false\`"
require_doc_text "Fresh trusted records required: \`true\`"
require_doc_text "Fresh trusted records recorded: \`false\`"
require_doc_text "Active binary SHA required: \`true\`"
require_doc_text "Active binary SHA recorded: \`false\`"
require_doc_text "Public artifact policy required: \`true\`"
require_doc_text "Public artifact policy recorded: \`false\`"
require_doc_text "Filesystem persistence approval packet ready: \`true\`"
require_doc_text "Filesystem persistence allowed: \`false\`"
require_doc_text "Filesystem persistence execution performed: \`false\`"
require_doc_text "Workspace write performed: \`false\`"
require_doc_text "Evidence receipt persisted: \`false\`"
require_doc_text "Activation blocked by filesystem persistence approval: \`true\`"
require_doc_text "Activation allowed by filesystem persistence approval: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "\`filesystem_persistence_approval_id\`"
require_doc_text "\`materialization_plan_id\`"
require_doc_text "\`receipt_payload_hash\`"
require_doc_text "\`redacted_output_path\`"
require_doc_text "No command invocation performed"
require_doc_text "No receipt persistence execution"
require_doc_text "No materialization execution"
require_doc_text "No filesystem persistence execution"
require_doc_text "No workspace write"
require_doc_text "No evidence receipt persistence"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg packet_id "upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-materialization-dry-run.sh" \
    --arg approval_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      filesystem_persistence_approval_packet_id:$packet_id,
      manifest:$manifest,
      filesystem_persistence_approval_packet_doc_path:$doc,
      source_materialization_dry_run_gate:$source_gate,
      filesystem_persistence_approval_packet_gate:$approval_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      packet_status:{
        source_materialization_dry_run_ready:true,
        required_approval_field_count:12,
        approval_field_count:12,
        recorded_approval_field_count:0,
        redacted_or_hashed_field_count:10,
        required_for_filesystem_persistence_field_count:12,
        operator_approval_required:true,
        operator_approval_recorded:false,
        activation_request_required:true,
        activation_request_recorded:false,
        materialization_plan_required:true,
        materialization_plan_recorded:false,
        fresh_trusted_records_required:true,
        fresh_trusted_records_recorded:false,
        active_binary_sha_required:true,
        active_binary_sha_recorded:false,
        public_artifact_policy_required:true,
        public_artifact_policy_recorded:false,
        filesystem_persistence_approval_packet_ready:true,
        filesystem_persistence_allowed:false,
        filesystem_persistence_execution_performed:false,
        workspace_write_performed:false,
        evidence_receipt_persisted:false,
        activation_blocked_by_filesystem_persistence_approval:true,
        activation_allowed_by_filesystem_persistence_approval:false,
        active_wiring_allowed:false
      },
      required_approval_fields:[
        "filesystem_persistence_approval_id",
        "activation_request_id",
        "operator_approval_id",
        "operator_identity_hash",
        "materialization_plan_id",
        "receipt_payload_hash",
        "redacted_output_path",
        "accepted_trusted_record_ids",
        "fresh_trusted_record_ids",
        "active_binary_sha256",
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
        command_invocation_performed:false,
        receipt_persistence_execution:false,
        materialization_execution:false,
        filesystem_persistence_execution:false,
        workspace_write:false,
        evidence_receipt_persistence:false,
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

echo "$report"
echo "Hepta upstream Codex activation evidence receipt filesystem persistence approval packet gate passed"
