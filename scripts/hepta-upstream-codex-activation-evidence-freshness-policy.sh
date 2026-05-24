#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_FRESHNESS_POLICY.md"

echo "[hepta-upstream-codex-activation-evidence-freshness-policy] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_freshness_policy -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence freshness policy missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence freshness policy: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-freshness-policy"
require_doc_text "Source denied sample gate: \`scripts/hepta-upstream-codex-activation-denied-sample.sh\`"
require_doc_text "Freshness policy gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh\`"
require_doc_text "Required evidence count: \`8\`"
require_doc_text "Policy entry count: \`8\`"
require_doc_text "Missing evidence count: \`8\`"
require_doc_text "Fresh evidence count: \`0\`"
require_doc_text "Freshness policy ready: \`true\`"
require_doc_text "Activation blocked by freshness policy: \`true\`"
require_doc_text "Activation allowed by freshness policy: \`false\`"
require_doc_text "\`live_dependency_isolation_evidence_id\`"
require_doc_text "\`watchdog_evidence_id\`"
require_doc_text "\`browser_smoke_evidence_id\`"
require_doc_text "\`long_soak_evidence_id\`"
require_doc_text "Max age policy: \`30 minutes\`"
require_doc_text "Max age policy: \`120 minutes\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "Public release claim allowed: \`false\`"
require_doc_text "Release artifact write allowed: \`false\`"
require_doc_text "No upstream fetch"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg policy "upstream-codex-activation-evidence-freshness-policy" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-denied-sample.sh" \
    --arg freshness_gate "scripts/hepta-upstream-codex-activation-evidence-freshness-policy.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      policy_id:$policy,
      manifest:$manifest,
      policy_doc_path:$doc,
      source_denied_sample_gate:$source_gate,
      freshness_policy_gate:$freshness_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      policy_status:{
        denied_sample_ready:true,
        required_evidence_count:8,
        policy_entry_count:8,
        missing_evidence_count:8,
        fresh_evidence_count:0,
        expired_evidence_count:0,
        stale_evidence_count:0,
        freshness_policy_ready:true,
        activation_blocked_by_freshness_policy:true,
        activation_allowed_by_freshness_policy:false,
        freshness_denial_reason:"all required activation evidence slots are absent from the denied sample",
        active_wiring_allowed:false
      },
      evidence_freshness_entries:[
        {evidence_id:"activation_request_id", max_age_policy:"same activation request", recorded:false, fresh:false},
        {evidence_id:"operator_approval_id", max_age_policy:"same activation request", recorded:false, fresh:false},
        {evidence_id:"operator_identity_hash", max_age_policy:"same activation request", recorded:false, fresh:false},
        {evidence_id:"live_dependency_isolation_evidence_id", max_age_policy:"30 minutes", recorded:false, fresh:false},
        {evidence_id:"watchdog_evidence_id", max_age_policy:"30 minutes", recorded:false, fresh:false},
        {evidence_id:"browser_smoke_evidence_id", max_age_policy:"30 minutes", recorded:false, fresh:false},
        {evidence_id:"long_soak_evidence_id", max_age_policy:"120 minutes", recorded:false, fresh:false},
        {evidence_id:"rollback_plan_id", max_age_policy:"same activation request", recorded:false, fresh:false}
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

echo "Hepta upstream Codex activation evidence freshness policy gate passed"
