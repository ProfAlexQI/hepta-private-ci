#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_WRITE_ENABLE_FIXTURE.md"

echo "[hepta-upstream-codex-activation-evidence-receipt-write-enable-fixture] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_receipt_write_enable_fixture -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence receipt write-enable fixture missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence receipt write-enable fixture: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-receipt-write-enable-fixture"
require_doc_text "Source no-write sink adapter gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract.sh\`"
require_doc_text "Write-enable fixture gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-write-enable-fixture.sh\`"
require_doc_text "Required write-enable fixture count: \`3\`"
require_doc_text "Write-enable fixture count: \`3\`"
require_doc_text "Blocked write-enable fixture count: \`3\`"
require_doc_text "Allowed write-enable fixture count: \`0\`"
require_doc_text "Explicit write-enable requested fixture count: \`3\`"
require_doc_text "Operator-approved fixture count: \`2\`"
require_doc_text "Activation request bound fixture count: \`3\`"
require_doc_text "Fresh trusted record fixture count: \`2\`"
require_doc_text "Active binary SHA bound fixture count: \`3\`"
require_doc_text "Public claim attempt fixture count: \`1\`"
require_doc_text "Release artifact write attempt fixture count: \`1\`"
require_doc_text "Public artifact policy satisfied fixture count: \`2\`"
require_doc_text "Filesystem persistence allowed count: \`0\`"
require_doc_text "Workspace write performed count: \`0\`"
require_doc_text "Evidence receipt persisted count: \`0\`"
require_doc_text "Write-enable fixture contract ready: \`true\`"
require_doc_text "Activation blocked by write-enable fixture: \`true\`"
require_doc_text "Activation allowed by write-enable fixture: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "\`write-enable-without-operator-approval\`"
require_doc_text "\`operator-approved-stale-evidence-write-enable\`"
require_doc_text "\`public-artifact-write-enable-attempt\`"
require_doc_text "No command invocation performed"
require_doc_text "No receipt persistence execution"
require_doc_text "No evidence receipt persistence"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg write_enable_fixture_id "upstream-codex-activation-evidence-receipt-write-enable-fixture" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract.sh" \
    --arg write_enable_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-write-enable-fixture.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      write_enable_fixture_id:$write_enable_fixture_id,
      manifest:$manifest,
      write_enable_fixture_doc_path:$doc,
      source_no_write_sink_adapter_gate:$source_gate,
      write_enable_fixture_gate:$write_enable_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      fixture_status:{
        source_no_write_sink_adapter_ready:true,
        required_write_enable_fixture_count:3,
        write_enable_fixture_count:3,
        blocked_write_enable_fixture_count:3,
        allowed_write_enable_fixture_count:0,
        explicit_write_enable_requested_fixture_count:3,
        operator_approved_fixture_count:2,
        activation_request_bound_fixture_count:3,
        fresh_trusted_record_fixture_count:2,
        active_binary_sha_bound_fixture_count:3,
        public_claim_attempt_fixture_count:1,
        release_artifact_write_attempt_fixture_count:1,
        public_artifact_policy_satisfied_fixture_count:2,
        filesystem_persistence_allowed_count:0,
        workspace_write_performed_count:0,
        evidence_receipt_persisted_count:0,
        write_enable_fixture_contract_ready:true,
        activation_blocked_by_write_enable_fixture:true,
        activation_allowed_by_write_enable_fixture:false,
        active_wiring_allowed:false
      },
      fixtures:[
        "write-enable-without-operator-approval",
        "operator-approved-stale-evidence-write-enable",
        "public-artifact-write-enable-attempt"
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

printf '%s\n' "$report"

echo "Hepta upstream Codex activation evidence receipt write-enable fixture gate passed"
