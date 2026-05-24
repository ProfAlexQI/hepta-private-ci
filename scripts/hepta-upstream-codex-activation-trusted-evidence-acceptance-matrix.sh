#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_TRUSTED_EVIDENCE_ACCEPTANCE_MATRIX.md"

echo "[hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_trusted_evidence_acceptance_matrix -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "trusted evidence acceptance matrix missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing trusted evidence acceptance matrix: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-trusted-evidence-acceptance-matrix"
require_doc_text "Source denied fixture gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh\`"
require_doc_text "Trusted acceptance matrix gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh\`"
require_doc_text "Required evidence count: \`8\`"
require_doc_text "Verification entry count: \`8\`"
require_doc_text "Schema-complete verification entry count: \`8\`"
require_doc_text "Required verification count per record: \`7\`"
require_doc_text "Total required verification count: \`56\`"
require_doc_text "Total satisfied verification count: \`0\`"
require_doc_text "Operator approval verified record count: \`0\`"
require_doc_text "Request-binding verified record count: \`0\`"
require_doc_text "Active binary SHA verified record count: \`0\`"
require_doc_text "Route/status hash verified record count: \`0\`"
require_doc_text "Artifact hash verified record count: \`0\`"
require_doc_text "Freshness window satisfied record count: \`0\`"
require_doc_text "Trusted source verified record count: \`0\`"
require_doc_text "Accepted record count: \`0\`"
require_doc_text "Blocked record count: \`8\`"
require_doc_text "Trusted evidence acceptance matrix ready: \`true\`"
require_doc_text "Activation blocked by trusted evidence acceptance matrix: \`true\`"
require_doc_text "Activation allowed by trusted evidence acceptance matrix: \`false\`"
require_doc_text "Operator approval verified"
require_doc_text "Activation request binding verified"
require_doc_text "Active binary SHA verified"
require_doc_text "Route/status hash verified"
require_doc_text "Artifact hash or redacted path verified"
require_doc_text "Freshness window satisfied"
require_doc_text "Trusted source verified"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "Public release claim allowed: \`false\`"
require_doc_text "Release artifact write allowed: \`false\`"
require_doc_text "No upstream fetch"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg matrix_id "upstream-codex-activation-trusted-evidence-acceptance-matrix" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-evidence-denied-fixture.sh" \
    --arg matrix_gate "scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      matrix_id:$matrix_id,
      manifest:$manifest,
      matrix_doc_path:$doc,
      source_denied_fixture_gate:$source_gate,
      trusted_acceptance_matrix_gate:$matrix_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      matrix_status:{
        source_denied_fixture_ready:true,
        required_evidence_count:8,
        verification_entry_count:8,
        schema_complete_verification_entry_count:8,
        required_verification_count_per_record:7,
        total_required_verification_count:56,
        total_satisfied_verification_count:0,
        operator_approval_verified_record_count:0,
        request_binding_verified_record_count:0,
        active_binary_sha_verified_record_count:0,
        route_or_status_hash_verified_record_count:0,
        artifact_hash_verified_record_count:0,
        freshness_window_satisfied_record_count:0,
        trusted_source_verified_record_count:0,
        accepted_record_count:0,
        blocked_record_count:8,
        trusted_evidence_acceptance_matrix_ready:true,
        activation_blocked_by_trusted_acceptance_matrix:true,
        activation_allowed_by_trusted_acceptance_matrix:false,
        acceptance_denial_reason:"trusted evidence acceptance requires operator approval, request binding, hashes, freshness, and trusted source verification",
        active_wiring_allowed:false
      },
      required_verification_checks:[
        "operator_approval_verified",
        "activation_request_binding_verified",
        "active_binary_sha_verified",
        "route_or_status_hash_verified",
        "artifact_hash_or_redacted_path_verified",
        "freshness_window_satisfied",
        "trusted_source_verified"
      ],
      evidence_records:[
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

echo "Hepta upstream Codex activation trusted evidence acceptance matrix gate passed"
