#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_FILESYSTEM_PERSISTENCE_EXECUTION_DENIAL_MATRIX.md"

echo "[hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_receipt_filesystem_persistence_execution_denial_matrix -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence receipt filesystem persistence execution denial matrix missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence receipt filesystem persistence execution denial matrix: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix"
require_doc_text "Source filesystem sink write preview gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh\`"
require_doc_text "Filesystem persistence execution denial matrix gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix.sh\`"
require_doc_text "Required denial fixture count: \`4\`"
require_doc_text "Denial fixture count: \`4\`"
require_doc_text "Source preview fixture count: \`3\`"
require_doc_text "Execution requested fixture count: \`4\`"
require_doc_text "Future persistence approval slot count: \`4\`"
require_doc_text "Explicit persistence approval id present count: \`3\`"
require_doc_text "Explicit persistence approval id missing count: \`1\`"
require_doc_text "Stale or missing fresh evidence fixture count: \`1\`"
require_doc_text "Active binary SHA bound fixture count: \`4\`"
require_doc_text "Trusted source bound fixture count: \`4\`"
require_doc_text "Operator approval bound fixture count: \`3\`"
require_doc_text "Workspace path attempt fixture count: \`1\`"
require_doc_text "Public claim attempt fixture count: \`1\`"
require_doc_text "Release artifact write attempt fixture count: \`1\`"
require_doc_text "Blocked execution fixture count: \`4\`"
require_doc_text "Allowed execution fixture count: \`0\`"
require_doc_text "Filesystem persistence allowed count: \`0\`"
require_doc_text "Filesystem persistence execution performed count: \`0\`"
require_doc_text "Workspace write performed count: \`0\`"
require_doc_text "Evidence receipt persisted count: \`0\`"
require_doc_text "Execution denial matrix ready: \`true\`"
require_doc_text "Activation blocked by execution denial matrix: \`true\`"
require_doc_text "Activation allowed by execution denial matrix: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "\`missing-persistence-approval-id-execution-attempt\`"
require_doc_text "\`stale-live-evidence-execution-attempt\`"
require_doc_text "\`workspace-path-execution-attempt\`"
require_doc_text "\`public-artifact-execution-attempt\`"
require_doc_text "Payload hash is bound to a future persistence approval id slot"
require_doc_text "No command invocation performed"
require_doc_text "No receipt persistence execution"
require_doc_text "No materialization execution"
require_doc_text "No filesystem persistence execution"
require_doc_text "No workspace write"
require_doc_text "No evidence receipt persistence"
require_doc_text "No public release publication"
require_doc_text "Preview payload hashes are bound to future persistence approval slots, not"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg matrix_id "upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh" \
    --arg matrix_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      filesystem_persistence_execution_denial_matrix_id:$matrix_id,
      manifest:$manifest,
      filesystem_persistence_execution_denial_matrix_doc_path:$doc,
      source_filesystem_sink_write_preview_gate:$source_gate,
      filesystem_persistence_execution_denial_matrix_gate:$matrix_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      denial_status:{
        source_filesystem_sink_write_preview_ready:true,
        required_denial_fixture_count:4,
        denial_fixture_count:4,
        source_preview_fixture_count:3,
        execution_requested_fixture_count:4,
        future_persistence_approval_slot_count:4,
        explicit_persistence_approval_id_present_count:3,
        explicit_persistence_approval_id_missing_count:1,
        stale_or_missing_fresh_evidence_fixture_count:1,
        active_binary_sha_bound_fixture_count:4,
        trusted_source_bound_fixture_count:4,
        operator_approval_bound_fixture_count:3,
        workspace_path_attempt_fixture_count:1,
        public_claim_attempt_fixture_count:1,
        release_artifact_write_attempt_fixture_count:1,
        blocked_execution_fixture_count:4,
        allowed_execution_fixture_count:0,
        filesystem_persistence_allowed_count:0,
        filesystem_persistence_execution_performed_count:0,
        workspace_write_performed_count:0,
        evidence_receipt_persisted_count:0,
        execution_denial_matrix_ready:true,
        activation_blocked_by_execution_denial_matrix:true,
        activation_allowed_by_execution_denial_matrix:false,
        active_wiring_allowed:false
      },
      denial_fixtures:[
        "missing-persistence-approval-id-execution-attempt",
        "stale-live-evidence-execution-attempt",
        "workspace-path-execution-attempt",
        "public-artifact-execution-attempt"
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
echo "Hepta upstream Codex activation evidence receipt filesystem persistence execution denial matrix gate passed"
