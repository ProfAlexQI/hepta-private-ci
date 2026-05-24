#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_OUTPUT_PATH_EVIDENCE_BINDING.md"

echo "[hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_receipt_filesystem_output_path_evidence_binding -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence receipt filesystem output path evidence binding missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence receipt filesystem output path evidence binding: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding"
require_doc_text "Source filesystem output path allowlist gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh\`"
require_doc_text "Filesystem output path evidence binding gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh\`"
require_doc_text "Required path binding count: \`8\`"
require_doc_text "Path binding count: \`8\`"
require_doc_text "Allowed output path entry count: \`3\`"
require_doc_text "Selected output path count: \`0\`"
require_doc_text "Recorded path binding count: \`0\`"
require_doc_text "Fresh live evidence bound count: \`0\`"
require_doc_text "Active binary SHA bound count: \`0\`"
require_doc_text "Redacted or hashed binding count: \`8\`"
require_doc_text "Trusted source bound count: \`0\`"
require_doc_text "Source tree path binding allowed: \`false\`"
require_doc_text "Home directory path binding allowed: \`false\`"
require_doc_text "Release artifact path binding allowed: \`false\`"
require_doc_text "Public artifact path binding allowed: \`false\`"
require_doc_text "Output path evidence binding ready: \`true\`"
require_doc_text "Filesystem persistence allowed: \`false\`"
require_doc_text "Filesystem persistence execution performed: \`false\`"
require_doc_text "Workspace write performed: \`false\`"
require_doc_text "Evidence receipt persisted: \`false\`"
require_doc_text "Activation blocked by output path evidence binding: \`true\`"
require_doc_text "Activation allowed by output path evidence binding: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "\`activation_request_id\`"
require_doc_text "\`operator_approval_id\`"
require_doc_text "\`operator_identity_hash\`"
require_doc_text "\`live_dependency_isolation_evidence_id\`"
require_doc_text "\`watchdog_evidence_id\`"
require_doc_text "\`browser_smoke_evidence_id\`"
require_doc_text "\`long_soak_evidence_id\`"
require_doc_text "\`rollback_plan_id\`"
require_doc_text "\`activation_evidence_receipts_root\`"
require_doc_text "\`activation_evidence_dry_run_root\`"
require_doc_text "\`activation_evidence_operator_packet_root\`"
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
    --arg binding_id "upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh" \
    --arg binding_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      filesystem_output_path_evidence_binding_id:$binding_id,
      manifest:$manifest,
      filesystem_output_path_evidence_binding_doc_path:$doc,
      source_filesystem_output_path_allowlist_gate:$source_gate,
      filesystem_output_path_evidence_binding_gate:$binding_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      binding_status:{
        source_filesystem_output_path_allowlist_ready:true,
        required_path_binding_count:8,
        path_binding_count:8,
        allowed_output_path_entry_count:3,
        selected_output_path_count:0,
        recorded_path_binding_count:0,
        fresh_live_evidence_bound_count:0,
        active_binary_sha_bound_count:0,
        redacted_or_hashed_binding_count:8,
        trusted_source_bound_count:0,
        source_tree_path_binding_allowed:false,
        home_directory_path_binding_allowed:false,
        release_artifact_path_binding_allowed:false,
        public_artifact_path_binding_allowed:false,
        output_path_evidence_binding_ready:true,
        filesystem_persistence_allowed:false,
        filesystem_persistence_execution_performed:false,
        workspace_write_performed:false,
        evidence_receipt_persisted:false,
        activation_blocked_by_output_path_evidence_binding:true,
        activation_allowed_by_output_path_evidence_binding:false,
        active_wiring_allowed:false
      },
      evidence_bindings:[
        "activation_request_id",
        "operator_approval_id",
        "operator_identity_hash",
        "live_dependency_isolation_evidence_id",
        "watchdog_evidence_id",
        "browser_smoke_evidence_id",
        "long_soak_evidence_id",
        "rollback_plan_id"
      ],
      allowed_output_path_bindings:[
        "activation_evidence_receipts_root",
        "activation_evidence_dry_run_root",
        "activation_evidence_operator_packet_root"
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
echo "Hepta upstream Codex activation evidence receipt filesystem output path evidence binding gate passed"
