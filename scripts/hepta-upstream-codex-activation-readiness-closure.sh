#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_READINESS_CLOSURE.md"

echo "[hepta-upstream-codex-activation-readiness-closure] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_readiness_closure -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation readiness closure missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation readiness closure: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-readiness-closure-denial"
require_doc_text "Source packet gate: \`scripts/hepta-upstream-codex-activation-request-packet.sh\`"
require_doc_text "Source dry-run gate: \`scripts/hepta-upstream-codex-activation-packet-dry-run.sh\`"
require_doc_text "Source evidence ledger gate: \`scripts/hepta-upstream-codex-activation-evidence-ledger.sh\`"
require_doc_text "Activation readiness closure gate: \`scripts/hepta-upstream-codex-activation-readiness-closure.sh\`"
require_doc_text "Activation packet schema ready: \`true\`"
require_doc_text "Dry-run validator ready: \`true\`"
require_doc_text "Evidence ledger ready: \`true\`"
require_doc_text "Required schema field count: \`14\`"
require_doc_text "Blocked fixture count: \`3\`"
require_doc_text "Allowed fixture count: \`0\`"
require_doc_text "Required evidence count: \`8\`"
require_doc_text "Recorded evidence count: \`0\`"
require_doc_text "Fresh evidence count: \`0\`"
require_doc_text "Readiness inputs ready: \`true\`"
require_doc_text "Activation denied by default: \`true\`"
require_doc_text "Activation readiness closure ready: \`true\`"
require_doc_text "Operator-approved activation ready: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "Public release claim allowed: \`false\`"
require_doc_text "Release artifact write allowed: \`false\`"
require_doc_text "No upstream fetch"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg closure "upstream-codex-activation-readiness-closure-denial" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg packet_gate "scripts/hepta-upstream-codex-activation-request-packet.sh" \
    --arg dry_run_gate "scripts/hepta-upstream-codex-activation-packet-dry-run.sh" \
    --arg ledger_gate "scripts/hepta-upstream-codex-activation-evidence-ledger.sh" \
    --arg closure_gate "scripts/hepta-upstream-codex-activation-readiness-closure.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      closure_id:$closure,
      manifest:$manifest,
      closure_doc_path:$doc,
      source_packet_gate:$packet_gate,
      source_dry_run_gate:$dry_run_gate,
      source_evidence_ledger_gate:$ledger_gate,
      activation_readiness_closure_gate:$closure_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      closure_status:{
        activation_packet_schema_ready:true,
        dry_run_validator_ready:true,
        evidence_ledger_ready:true,
        activation_packet_recorded:false,
        evidence_recorded:false,
        required_schema_field_count:14,
        blocked_fixture_count:3,
        allowed_fixture_count:0,
        required_evidence_count:8,
        recorded_evidence_count:0,
        fresh_evidence_count:0,
        readiness_inputs_ready:true,
        activation_denied_by_default:true,
        activation_readiness_closure_ready:true,
        operator_approved_activation_ready:false,
        active_wiring_allowed:false
      },
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

echo "Hepta upstream Codex activation readiness closure gate passed"
