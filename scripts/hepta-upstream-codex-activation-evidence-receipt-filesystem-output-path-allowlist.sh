#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_OUTPUT_PATH_ALLOWLIST.md"

echo "[hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_receipt_filesystem_output_path_allowlist -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence receipt filesystem output path allowlist missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence receipt filesystem output path allowlist: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist"
require_doc_text "Source filesystem persistence approval packet gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh\`"
require_doc_text "Filesystem output path allowlist gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh\`"
require_doc_text "Required allowlist entry count: \`6\`"
require_doc_text "Allowlist entry count: \`6\`"
require_doc_text "Allowed output path entry count: \`3\`"
require_doc_text "Blocked output path entry count: \`3\`"
require_doc_text "Redacted output path entry count: \`6\`"
require_doc_text "Default selected output path count: \`0\`"
require_doc_text "Source tree path allowed: \`false\`"
require_doc_text "Home directory path allowed: \`false\`"
require_doc_text "Release artifact path allowed: \`false\`"
require_doc_text "Public artifact path allowed: \`false\`"
require_doc_text "Receipt output path allowlist ready: \`true\`"
require_doc_text "Filesystem persistence allowed: \`false\`"
require_doc_text "Filesystem persistence execution performed: \`false\`"
require_doc_text "Workspace write performed: \`false\`"
require_doc_text "Evidence receipt persisted: \`false\`"
require_doc_text "Activation blocked by output path allowlist: \`true\`"
require_doc_text "Activation allowed by output path allowlist: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "\`activation_evidence_receipts_root\`"
require_doc_text "\`activation_evidence_dry_run_root\`"
require_doc_text "\`activation_evidence_operator_packet_root\`"
require_doc_text "\`source_tree_root\`"
require_doc_text "\`home_directory_root\`"
require_doc_text "\`release_artifact_root\`"
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
    --arg allowlist_id "upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh" \
    --arg allowlist_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      filesystem_output_path_allowlist_id:$allowlist_id,
      manifest:$manifest,
      filesystem_output_path_allowlist_doc_path:$doc,
      source_filesystem_persistence_approval_packet_gate:$source_gate,
      filesystem_output_path_allowlist_gate:$allowlist_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      allowlist_status:{
        source_filesystem_persistence_approval_packet_ready:true,
        required_allowlist_entry_count:6,
        allowlist_entry_count:6,
        allowed_output_path_entry_count:3,
        blocked_output_path_entry_count:3,
        redacted_output_path_entry_count:6,
        default_selected_output_path_count:0,
        source_tree_path_allowed:false,
        home_directory_path_allowed:false,
        release_artifact_path_allowed:false,
        public_artifact_path_allowed:false,
        receipt_output_path_allowlist_ready:true,
        filesystem_persistence_allowed:false,
        filesystem_persistence_execution_performed:false,
        workspace_write_performed:false,
        evidence_receipt_persisted:false,
        activation_blocked_by_output_path_allowlist:true,
        activation_allowed_by_output_path_allowlist:false,
        active_wiring_allowed:false
      },
      allowlist_entries:[
        "activation_evidence_receipts_root",
        "activation_evidence_dry_run_root",
        "activation_evidence_operator_packet_root",
        "source_tree_root",
        "home_directory_root",
        "release_artifact_root"
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
echo "Hepta upstream Codex activation evidence receipt filesystem output path allowlist gate passed"
