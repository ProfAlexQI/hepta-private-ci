#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_TRUSTED_RECORD_SHAPE_VALIDATOR.md"

echo "[hepta-upstream-codex-activation-trusted-record-shape-validator] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_trusted_record_shape_validator -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "trusted record shape validator missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing trusted record shape validator: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-trusted-record-shape-validator"
require_doc_text "Source trusted acceptance matrix gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh\`"
require_doc_text "Trusted record shape validator gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh\`"
require_doc_text "Required evidence count: \`8\`"
require_doc_text "Fixture count: \`2\`"
require_doc_text "Partial trusted fixture count: \`1\`"
require_doc_text "Public claim attempt fixture count: \`1\`"
require_doc_text "Blocked fixture count: \`2\`"
require_doc_text "Allowed fixture count: \`0\`"
require_doc_text "Required verification count per record: \`7\`"
require_doc_text "Total required verification count per fixture: \`56\`"
require_doc_text "Max satisfied verification count: \`48\`"
require_doc_text "Trusted record shape validator ready: \`true\`"
require_doc_text "Activation blocked by shape validator: \`true\`"
require_doc_text "Activation allowed by shape validator: \`false\`"
require_doc_text "\`partial-trusted-records\`"
require_doc_text "Total satisfied verification count: \`32\`"
require_doc_text "Artifact hash verified record count: \`0\`"
require_doc_text "Freshness window satisfied record count: \`0\`"
require_doc_text "Trusted source verified record count: \`0\`"
require_doc_text "\`public-claim-attempt-with-trusted-shape\`"
require_doc_text "Public release claim requested: \`true\`"
require_doc_text "Release artifact write requested: \`true\`"
require_doc_text "Public release claim allowed: \`false\`"
require_doc_text "Release artifact write allowed: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "No upstream fetch"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg validator_id "upstream-codex-activation-trusted-record-shape-validator" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-trusted-evidence-acceptance-matrix.sh" \
    --arg validator_gate "scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      validator_id:$validator_id,
      manifest:$manifest,
      validator_doc_path:$doc,
      source_trusted_acceptance_matrix_gate:$source_gate,
      trusted_record_shape_validator_gate:$validator_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      validator_status:{
        source_trusted_acceptance_matrix_ready:true,
        required_evidence_count:8,
        fixture_count:2,
        partial_trusted_fixture_count:1,
        public_claim_attempt_fixture_count:1,
        blocked_fixture_count:2,
        allowed_fixture_count:0,
        required_verification_count_per_record:7,
        total_required_verification_count_per_fixture:56,
        max_satisfied_verification_count:48,
        trusted_record_shape_validator_ready:true,
        activation_blocked_by_shape_validator:true,
        activation_allowed_by_shape_validator:false,
        shape_denial_reason:"partial or public-claim trusted-record shapes stay blocked until every record is fresh, bound, trusted, and operator-approved",
        active_wiring_allowed:false
      },
      fixtures:[
        {
          fixture_id:"partial-trusted-records",
          fixture_kind:"partial_trusted_records",
          total_satisfied_verification_count:32,
          artifact_hash_verified_record_count:0,
          freshness_window_satisfied_record_count:0,
          trusted_source_verified_record_count:0,
          accepted_record_count:0,
          blocked_record_count:8,
          validation_status:"blocked",
          active_wiring_allowed:false,
          public_release_claim_allowed:false,
          release_artifact_write_allowed:false
        },
        {
          fixture_id:"public-claim-attempt-with-trusted-shape",
          fixture_kind:"public_claim_attempt",
          public_release_claim_requested:true,
          release_artifact_write_requested:true,
          total_satisfied_verification_count:48,
          artifact_hash_verified_record_count:8,
          freshness_window_satisfied_record_count:0,
          trusted_source_verified_record_count:8,
          accepted_record_count:0,
          blocked_record_count:8,
          validation_status:"blocked",
          active_wiring_allowed:false,
          public_release_claim_allowed:false,
          release_artifact_write_allowed:false
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

echo "Hepta upstream Codex activation trusted record shape validator gate passed"
