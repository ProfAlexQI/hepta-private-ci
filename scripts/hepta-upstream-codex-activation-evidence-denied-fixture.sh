#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_DENIED_FIXTURE.md"

echo "[hepta-upstream-codex-activation-evidence-denied-fixture] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_record_denied_fixture -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence denied fixture missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence denied fixture: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-record-denied-fixture"
require_doc_text "Source binding manifest gate: \`scripts/hepta-upstream-codex-activation-evidence-binding-record.sh\`"
require_doc_text "Denied fixture gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh\`"
require_doc_text "Required evidence count: \`8\`"
require_doc_text "Fixture record count: \`8\`"
require_doc_text "Schema-complete fixture record count: \`8\`"
require_doc_text "Trusted fixture record count: \`0\`"
require_doc_text "Operator-approved fixture record count: \`0\`"
require_doc_text "Request-binding verified record count: \`0\`"
require_doc_text "Live gate hash verified record count: \`0\`"
require_doc_text "Artifact hash verified record count: \`0\`"
require_doc_text "Fresh fixture record count: \`0\`"
require_doc_text "Blocked fixture record count: \`8\`"
require_doc_text "Allowed fixture record count: \`0\`"
require_doc_text "Denied fixture ready: \`true\`"
require_doc_text "Activation blocked by denied fixture: \`true\`"
require_doc_text "Activation allowed by denied fixture: \`false\`"
require_doc_text "\`placeholder-active-binary-sha256\`"
require_doc_text "\`placeholder-route-or-status-hash\`"
require_doc_text "\`placeholder-artifact-hash-or-redacted-path\`"
require_doc_text "\`placeholder-activation-request-id\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "Public release claim allowed: \`false\`"
require_doc_text "Release artifact write allowed: \`false\`"
require_doc_text "No upstream fetch"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg fixture_id "upstream-codex-activation-evidence-record-denied-fixture" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-evidence-binding-record.sh" \
    --arg denied_gate "scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      fixture_id:$fixture_id,
      manifest:$manifest,
      fixture_doc_path:$doc,
      source_binding_manifest_gate:$source_gate,
      denied_fixture_gate:$denied_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      fixture_status:{
        binding_manifest_ready:true,
        required_evidence_count:8,
        fixture_record_count:8,
        schema_complete_fixture_record_count:8,
        trusted_fixture_record_count:0,
        operator_approved_fixture_record_count:0,
        request_binding_verified_record_count:0,
        live_gate_hash_verified_record_count:0,
        artifact_hash_verified_record_count:0,
        fresh_fixture_record_count:0,
        blocked_fixture_record_count:8,
        allowed_fixture_record_count:0,
        denied_fixture_ready:true,
        activation_blocked_by_denied_fixture:true,
        activation_allowed_by_denied_fixture:false,
        fixture_denial_reason:"fixture evidence records are placeholders without operator approval or verified freshness",
        active_wiring_allowed:false
      },
      placeholder_values:{
        recorded_at_unix_ms:"0",
        active_binary_sha256:"placeholder-active-binary-sha256",
        route_or_status_hash:"placeholder-route-or-status-hash",
        artifact_sha256_or_redacted_path:"placeholder-artifact-hash-or-redacted-path",
        activation_request_id_binding:"placeholder-activation-request-id"
      },
      fixture_records:[
        "activation_request_id",
        "operator_approval_id",
        "operator_identity_hash",
        "live_dependency_isolation_evidence_id",
        "watchdog_evidence_id",
        "browser_smoke_evidence_id",
        "long_soak_evidence_id",
        "rollback_plan_id"
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

echo "Hepta upstream Codex activation evidence denied fixture gate passed"
