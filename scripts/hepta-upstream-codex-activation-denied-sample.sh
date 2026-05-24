#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_DENIED_SAMPLE.md"

echo "[hepta-upstream-codex-activation-denied-sample] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_denied_sample -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation denied sample missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation denied sample: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-denied-sample-packet"
require_doc_text "Source readiness closure gate: \`scripts/hepta-upstream-codex-activation-readiness-closure.sh\`"
require_doc_text "Denied sample gate: \`scripts/hepta-upstream-codex-activation-denied-sample.sh\`"
require_doc_text "Activation readiness closure ready: \`true\`"
require_doc_text "Sample packet shape complete: \`true\`"
require_doc_text "Sample required schema field count: \`14\`"
require_doc_text "Sample recorded schema field count: \`14\`"
require_doc_text "Sample required evidence count: \`8\`"
require_doc_text "Sample fresh evidence count: \`0\`"
require_doc_text "Sample operator approval field present: \`true\`"
require_doc_text "Sample operator approval recorded: \`false\`"
require_doc_text "Sample public release claim requested: \`true\`"
require_doc_text "Sample release artifact write requested: \`true\`"
require_doc_text "Sample validation status: \`blocked\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "Public release claim allowed: \`false\`"
require_doc_text "Release artifact write allowed: \`false\`"
require_doc_text "No upstream fetch"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg sample "upstream-codex-activation-denied-sample-packet" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-readiness-closure.sh" \
    --arg denied_gate "scripts/hepta-upstream-codex-activation-denied-sample.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      sample_id:$sample,
      manifest:$manifest,
      sample_doc_path:$doc,
      source_readiness_closure_gate:$source_gate,
      denied_sample_gate:$denied_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      sample_status:{
        activation_readiness_closure_ready:true,
        sample_packet_shape_complete:true,
        sample_required_schema_field_count:14,
        sample_recorded_schema_field_count:14,
        sample_required_evidence_count:8,
        sample_fresh_evidence_count:0,
        sample_operator_approval_field_present:true,
        sample_operator_approval_recorded:false,
        sample_public_release_claim_requested:true,
        sample_release_artifact_write_requested:true,
        sample_validation_status:"blocked",
        sample_blocked_reason:"operator approval is not recorded and activation evidence is not fresh",
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

echo "Hepta upstream Codex activation denied sample gate passed"
