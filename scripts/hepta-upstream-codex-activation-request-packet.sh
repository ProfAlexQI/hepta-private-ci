#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_REQUEST_PACKET.md"

echo "[hepta-upstream-codex-activation-request-packet] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_request_packet -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation request packet schema missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation request packet schema: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-request-packet-schema"
require_doc_text "Active wiring precondition ready: \`true\`"
require_doc_text "Active wiring allowed by precondition: \`false\`"
require_doc_text "Operator approval required: \`true\`"
require_doc_text "Operator approval recorded: \`false\`"
require_doc_text "Activation request id required: \`true\`"
require_doc_text "Activation request id recorded: \`false\`"
require_doc_text "Required schema field count: \`14\`"
require_doc_text "Recorded required schema field count: \`0\`"
require_doc_text "Schema field count: \`14\`"
require_doc_text "Activation packet schema ready: \`true\`"
require_doc_text "Activation packet recorded: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "\`activation_request_id\`"
require_doc_text "\`operator_approval_id\`"
require_doc_text "\`operator_identity_hash\`"
require_doc_text "\`live_dependency_isolation_evidence_id\`"
require_doc_text "\`release_artifact_write_decision\`"
require_doc_text "Active runtime code wiring allowed: \`false\`"
require_doc_text "Public release claim allowed: \`false\`"
require_doc_text "Release artifact write allowed: \`false\`"
require_doc_text "No gateway RPC"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg packet "upstream-codex-activation-request-packet-schema" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg precondition_gate "scripts/hepta-upstream-codex-active-wiring-precondition.sh" \
    --arg packet_gate "scripts/hepta-upstream-codex-activation-request-packet.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      packet_id:$packet,
      manifest:$manifest,
      packet_schema_path:$doc,
      source_precondition_gate:$precondition_gate,
      activation_request_packet_gate:$packet_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      source_preconditions:{
        active_wiring_precondition_ready:true,
        active_wiring_allowed_by_precondition:false,
        operator_approval_required:true,
        operator_approval_recorded:false,
        activation_request_id_required:true,
        activation_request_id_recorded:false
      },
      schema_status:{
        required_schema_field_count:14,
        recorded_required_schema_field_count:0,
        schema_field_count:14,
        activation_packet_schema_ready:true,
        activation_packet_recorded:false,
        active_wiring_allowed:false
      },
      required_fields:[
        "activation_request_id",
        "operator_approval_id",
        "operator_identity_hash",
        "approved_bucket_ids",
        "approved_surface_ids",
        "requested_runtime_wiring_scope",
        "requested_dependency_change_set",
        "live_dependency_isolation_evidence_id",
        "watchdog_evidence_id",
        "browser_smoke_evidence_id",
        "long_soak_evidence_id",
        "rollback_plan_id",
        "public_release_claim_decision",
        "release_artifact_write_decision"
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

echo "Hepta upstream Codex activation request packet schema gate passed"
