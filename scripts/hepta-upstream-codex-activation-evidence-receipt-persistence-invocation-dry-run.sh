#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_PERSISTENCE_INVOCATION_DRY_RUN.md"

echo "[hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_receipt_persistence_invocation_dry_run -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence receipt persistence invocation dry-run missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence receipt persistence invocation dry-run: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run"
require_doc_text "Source command contract gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh\`"
require_doc_text "Receipt persistence invocation dry-run gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh\`"
require_doc_text "Required invocation fixture count: \`3\`"
require_doc_text "Command invocation attempt count: \`3\`"
require_doc_text "Command invocation performed count: \`0\`"
require_doc_text "Receipt persistence execution performed count: \`0\`"
require_doc_text "Workspace write performed count: \`0\`"
require_doc_text "Evidence receipt persisted count: \`0\`"
require_doc_text "Redacted output path fixture count: \`3\`"
require_doc_text "Payload hash bound fixture count: \`3\`"
require_doc_text "Operator approved fixture count: \`3\`"
require_doc_text "Activation request bound fixture count: \`3\`"
require_doc_text "Max recorded command field count: \`10\`"
require_doc_text "Max accepted trusted record count: \`8\`"
require_doc_text "Max fresh trusted record count: \`8\`"
require_doc_text "Public claim attempt count: \`1\`"
require_doc_text "Release artifact write attempt count: \`1\`"
require_doc_text "Receipt persistence command enabled by default: \`false\`"
require_doc_text "Invocation dry-run no-op ready: \`true\`"
require_doc_text "Activation blocked by invocation dry-run: \`true\`"
require_doc_text "Activation allowed by invocation dry-run: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "\`redacted-command-shape\`"
require_doc_text "\`stale-evidence-invocation-attempt\`"
require_doc_text "\`public-claim-artifact-invocation-attempt\`"
require_doc_text "No command invocation performed"
require_doc_text "No receipt persistence execution"
require_doc_text "No evidence receipt persistence"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg invocation_dry_run_id "upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-command-contract.sh" \
    --arg invocation_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      invocation_dry_run_id:$invocation_dry_run_id,
      manifest:$manifest,
      invocation_dry_run_doc_path:$doc,
      source_command_contract_gate:$source_gate,
      receipt_persistence_invocation_dry_run_gate:$invocation_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      invocation_status:{
        source_command_contract_ready:true,
        required_invocation_fixture_count:3,
        command_invocation_attempt_count:3,
        command_invocation_performed_count:0,
        receipt_persistence_execution_performed_count:0,
        workspace_write_performed_count:0,
        evidence_receipt_persisted_count:0,
        redacted_output_path_fixture_count:3,
        payload_hash_bound_fixture_count:3,
        operator_approved_fixture_count:3,
        activation_request_bound_fixture_count:3,
        max_recorded_command_field_count:10,
        max_accepted_trusted_record_count:8,
        max_fresh_trusted_record_count:8,
        public_claim_attempt_count:1,
        release_artifact_write_attempt_count:1,
        receipt_persistence_command_enabled_by_default:false,
        invocation_dry_run_noop_ready:true,
        activation_blocked_by_invocation_dry_run:true,
        activation_allowed_by_invocation_dry_run:false,
        active_wiring_allowed:false
      },
      fixtures:[
        {
          fixture_id:"redacted-command-shape",
          fixture_kind:"redacted_command_shape",
          dry_run_status:"blocked_noop",
          command_invocation_requested:true,
          command_invocation_performed:false,
          workspace_write_performed:false,
          evidence_receipt_persisted:false
        },
        {
          fixture_id:"stale-evidence-invocation-attempt",
          fixture_kind:"stale_evidence_invocation_attempt",
          dry_run_status:"blocked_noop",
          command_invocation_requested:true,
          command_invocation_performed:false,
          workspace_write_performed:false,
          evidence_receipt_persisted:false
        },
        {
          fixture_id:"public-claim-artifact-invocation-attempt",
          fixture_kind:"public_claim_artifact_invocation_attempt",
          dry_run_status:"blocked_noop",
          command_invocation_requested:true,
          command_invocation_performed:false,
          workspace_write_performed:false,
          evidence_receipt_persisted:false,
          public_claim_requested:true,
          release_artifact_write_requested:true
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

echo "Hepta upstream Codex activation evidence receipt persistence invocation dry-run gate passed"
