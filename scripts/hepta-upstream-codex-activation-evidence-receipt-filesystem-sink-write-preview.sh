#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_SINK_WRITE_PREVIEW.md"

echo "[hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_receipt_filesystem_sink_write_preview -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence receipt filesystem sink write preview missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence receipt filesystem sink write preview: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview"
require_doc_text "Source filesystem output path evidence binding gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh\`"
require_doc_text "Filesystem sink write preview gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh\`"
require_doc_text "Required preview fixture count: \`3\`"
require_doc_text "Preview fixture count: \`3\`"
require_doc_text "Allowed output path entry count: \`3\`"
require_doc_text "Previewed output path count: \`3\`"
require_doc_text "Deterministic payload hash count: \`3\`"
require_doc_text "Redacted output path preview count: \`3\`"
require_doc_text "Fresh live evidence bound fixture count: \`3\`"
require_doc_text "Active binary SHA bound fixture count: \`3\`"
require_doc_text "Trusted source bound fixture count: \`3\`"
require_doc_text "Operator approval bound fixture count: \`3\`"
require_doc_text "Blocked preview fixture count: \`3\`"
require_doc_text "Allowed preview fixture count: \`0\`"
require_doc_text "Public claim attempt fixture count: \`1\`"
require_doc_text "Release artifact write attempt fixture count: \`1\`"
require_doc_text "Filesystem persistence allowed count: \`0\`"
require_doc_text "Workspace write performed count: \`0\`"
require_doc_text "Evidence receipt persisted count: \`0\`"
require_doc_text "Sink write preview ready: \`true\`"
require_doc_text "Activation blocked by sink write preview: \`true\`"
require_doc_text "Activation allowed by sink write preview: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "\`receipt-root-sink-write-preview\`"
require_doc_text "\`dry-run-root-sink-write-preview\`"
require_doc_text "\`public-artifact-sink-write-preview-attempt\`"
require_doc_text "No command invocation performed"
require_doc_text "No receipt persistence execution"
require_doc_text "No materialization execution"
require_doc_text "No filesystem persistence execution"
require_doc_text "No workspace write"
require_doc_text "No evidence receipt persistence"
require_doc_text "No public release publication"
require_doc_text "Deterministic payload hashes and redacted output paths are preview evidence, not"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg preview_id "upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh" \
    --arg preview_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      filesystem_sink_write_preview_id:$preview_id,
      manifest:$manifest,
      filesystem_sink_write_preview_doc_path:$doc,
      source_filesystem_output_path_evidence_binding_gate:$source_gate,
      filesystem_sink_write_preview_gate:$preview_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      preview_status:{
        source_filesystem_output_path_evidence_binding_ready:true,
        required_preview_fixture_count:3,
        preview_fixture_count:3,
        allowed_output_path_entry_count:3,
        previewed_output_path_count:3,
        deterministic_payload_hash_count:3,
        redacted_output_path_preview_count:3,
        fresh_live_evidence_bound_fixture_count:3,
        active_binary_sha_bound_fixture_count:3,
        trusted_source_bound_fixture_count:3,
        operator_approval_bound_fixture_count:3,
        blocked_preview_fixture_count:3,
        allowed_preview_fixture_count:0,
        public_claim_attempt_fixture_count:1,
        release_artifact_write_attempt_fixture_count:1,
        filesystem_persistence_allowed_count:0,
        workspace_write_performed_count:0,
        evidence_receipt_persisted_count:0,
        sink_write_preview_ready:true,
        activation_blocked_by_sink_write_preview:true,
        activation_allowed_by_sink_write_preview:false,
        active_wiring_allowed:false
      },
      preview_fixtures:[
        "receipt-root-sink-write-preview",
        "dry-run-root-sink-write-preview",
        "public-artifact-sink-write-preview-attempt"
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
echo "Hepta upstream Codex activation evidence receipt filesystem sink write preview gate passed"
