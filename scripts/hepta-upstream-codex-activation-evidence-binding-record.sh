#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_BINDING_RECORD.md"

echo "[hepta-upstream-codex-activation-evidence-binding-record] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_binding_record_manifest -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence binding record manifest missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence binding record manifest: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-binding-record-manifest"
require_doc_text "Source freshness policy gate: \`scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh\`"
require_doc_text "Binding manifest gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-binding-record.sh\`"
require_doc_text "Required evidence count: \`8\`"
require_doc_text "Binding record count: \`8\`"
require_doc_text "Missing binding record count: \`8\`"
require_doc_text "Recorded binding record count: \`0\`"
require_doc_text "Required record schema field count: \`7\`"
require_doc_text "Recorded record schema field count: \`0\`"
require_doc_text "Timestamped record count: \`0\`"
require_doc_text "Binary SHA bound record count: \`0\`"
require_doc_text "Route or status hash bound record count: \`0\`"
require_doc_text "Artifact hash or redacted path bound record count: \`0\`"
require_doc_text "Activation request id bound record count: \`0\`"
require_doc_text "Binding manifest ready: \`true\`"
require_doc_text "Activation blocked by binding manifest: \`true\`"
require_doc_text "Activation allowed by binding manifest: \`false\`"
require_doc_text "\`evidence_record_id\`"
require_doc_text "\`source_gate\`"
require_doc_text "\`recorded_at_unix_ms\`"
require_doc_text "\`active_binary_sha256\`"
require_doc_text "\`route_or_status_hash\`"
require_doc_text "\`artifact_sha256_or_redacted_path\`"
require_doc_text "\`activation_request_id_binding\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "Public release claim allowed: \`false\`"
require_doc_text "Release artifact write allowed: \`false\`"
require_doc_text "No upstream fetch"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg manifest_id "upstream-codex-activation-evidence-binding-record-manifest" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh" \
    --arg binding_gate "scripts/hepta-upstream-codex-activation-evidence-binding-record.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      manifest_id:$manifest_id,
      manifest:$manifest,
      manifest_doc_path:$doc,
      source_freshness_policy_gate:$source_gate,
      binding_manifest_gate:$binding_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      manifest_status:{
        freshness_policy_ready:true,
        required_evidence_count:8,
        binding_record_count:8,
        missing_binding_record_count:8,
        recorded_binding_record_count:0,
        required_record_schema_field_count:7,
        recorded_record_schema_field_count:0,
        timestamped_record_count:0,
        binary_sha_bound_record_count:0,
        route_or_status_hash_bound_record_count:0,
        artifact_hash_or_redacted_path_bound_record_count:0,
        activation_request_id_bound_record_count:0,
        binding_manifest_ready:true,
        activation_blocked_by_binding_manifest:true,
        activation_allowed_by_binding_manifest:false,
        binding_denial_reason:"all evidence binding records are schema-only and unrecorded",
        active_wiring_allowed:false
      },
      required_record_fields:[
        "evidence_record_id",
        "source_gate",
        "recorded_at_unix_ms",
        "active_binary_sha256",
        "route_or_status_hash",
        "artifact_sha256_or_redacted_path",
        "activation_request_id_binding"
      ],
      binding_records:[
        {evidence_id:"activation_request_id", source_gate:"scripts/hepta-upstream-codex-activation-request-packet.sh", evidence_recorded:false},
        {evidence_id:"operator_approval_id", source_gate:"scripts/hepta-codex-public-ga-operator-approval-packet.sh", evidence_recorded:false},
        {evidence_id:"operator_identity_hash", source_gate:"scripts/hepta-codex-public-ga-operator-approval-packet.sh", evidence_recorded:false},
        {evidence_id:"live_dependency_isolation_evidence_id", source_gate:"scripts/hepta-active-service-dependency-isolation.sh", evidence_recorded:false},
        {evidence_id:"watchdog_evidence_id", source_gate:"scripts/hepta-watchdog.sh", evidence_recorded:false},
        {evidence_id:"browser_smoke_evidence_id", source_gate:"scripts/hepta-browser-visual-smoke.sh", evidence_recorded:false},
        {evidence_id:"long_soak_evidence_id", source_gate:"scripts/hepta-live-soak.sh", evidence_recorded:false},
        {evidence_id:"rollback_plan_id", source_gate:"docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_DENIED_SAMPLE.md", evidence_recorded:false}
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

echo "Hepta upstream Codex activation evidence binding record manifest gate passed"
