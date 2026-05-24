#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_COMPLETENESS_SCOREBOARD.md"

echo "[hepta-upstream-codex-activation-evidence-completeness-scoreboard] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_completeness_scoreboard -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence completeness scoreboard missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence completeness scoreboard: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-completeness-scoreboard"
require_doc_text "Source trusted record shape validator gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh\`"
require_doc_text "Evidence completeness scoreboard gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh\`"
require_doc_text "Required gate family count: \`10\`"
require_doc_text "Ready gate family count: \`10\`"
require_doc_text "Activation-blocking gate family count: \`10\`"
require_doc_text "Required evidence count: \`8\`"
require_doc_text "Required trusted record count: \`8\`"
require_doc_text "Accepted trusted record count: \`0\`"
require_doc_text "Fresh trusted record count: \`0\`"
require_doc_text "Operator approval recorded: \`false\`"
require_doc_text "Activation request recorded: \`false\`"
require_doc_text "Public claim attempt blocked: \`true\`"
require_doc_text "Release artifact write attempt blocked: \`true\`"
require_doc_text "Operator-approved activation ready: \`false\`"
require_doc_text "Evidence completeness scoreboard ready: \`true\`"
require_doc_text "Activation blocked by scoreboard: \`true\`"
require_doc_text "Activation allowed by scoreboard: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "No upstream fetch"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg scoreboard_id "upstream-codex-activation-evidence-completeness-scoreboard" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-trusted-record-shape-validator.sh" \
    --arg scoreboard_gate "scripts/hepta-upstream-codex-activation-evidence-completeness-scoreboard.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      scoreboard_id:$scoreboard_id,
      manifest:$manifest,
      scoreboard_doc_path:$doc,
      source_trusted_record_shape_validator_gate:$source_gate,
      evidence_completeness_scoreboard_gate:$scoreboard_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      scoreboard_status:{
        source_trusted_record_shape_validator_ready:true,
        required_gate_family_count:10,
        ready_gate_family_count:10,
        activation_blocking_gate_family_count:10,
        required_evidence_count:8,
        required_trusted_record_count:8,
        accepted_trusted_record_count:0,
        fresh_trusted_record_count:0,
        operator_approval_recorded:false,
        activation_request_recorded:false,
        public_claim_attempt_blocked:true,
        release_artifact_write_attempt_blocked:true,
        operator_approved_activation_ready:false,
        evidence_completeness_scoreboard_ready:true,
        activation_blocked_by_scoreboard:true,
        activation_allowed_by_scoreboard:false,
        scoreboard_denial_reason:"activation evidence gate families are ready, but no real activation request or fresh trusted evidence records exist",
        active_wiring_allowed:false
      },
      gate_families:[
        "activation-request-packet",
        "activation-packet-dry-run",
        "activation-evidence-ledger",
        "activation-readiness-closure",
        "activation-denied-sample",
        "activation-evidence-freshness-policy",
        "activation-evidence-binding-record",
        "activation-evidence-denied-fixture",
        "activation-trusted-evidence-acceptance-matrix",
        "activation-trusted-record-shape-validator"
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

echo "Hepta upstream Codex activation evidence completeness scoreboard gate passed"
