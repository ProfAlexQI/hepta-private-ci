#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_LEDGER.md"

echo "[hepta-upstream-codex-activation-evidence-ledger] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_ledger -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence ledger missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence ledger: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-ledger-checklist"
require_doc_text "Source dry-run gate: \`scripts/hepta-upstream-codex-activation-packet-dry-run.sh\`"
require_doc_text "Evidence ledger gate: \`scripts/hepta-upstream-codex-activation-evidence-ledger.sh\`"
require_doc_text "Dry-run validator ready: \`true\`"
require_doc_text "Activation packet recorded: \`false\`"
require_doc_text "Required evidence count: \`8\`"
require_doc_text "Recorded evidence count: \`0\`"
require_doc_text "Fresh evidence count: \`0\`"
require_doc_text "Evidence ledger ready: \`true\`"
require_doc_text "Evidence recorded: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "\`activation_request_id\`"
require_doc_text "\`operator_approval_id\`"
require_doc_text "\`live_dependency_isolation_evidence_id\`"
require_doc_text "\`watchdog_evidence_id\`"
require_doc_text "\`browser_smoke_evidence_id\`"
require_doc_text "\`long_soak_evidence_id\`"
require_doc_text "\`rollback_plan_id\`"
require_doc_text "Public release claim allowed: \`false\`"
require_doc_text "Release artifact write allowed: \`false\`"
require_doc_text "No upstream fetch"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg ledger "upstream-codex-activation-evidence-ledger-checklist" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-packet-dry-run.sh" \
    --arg ledger_gate "scripts/hepta-upstream-codex-activation-evidence-ledger.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      ledger_id:$ledger,
      manifest:$manifest,
      ledger_doc_path:$doc,
      source_dry_run_gate:$source_gate,
      evidence_ledger_gate:$ledger_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      ledger_status:{
        dry_run_validator_ready:true,
        activation_packet_recorded:false,
        required_evidence_count:8,
        recorded_evidence_count:0,
        fresh_evidence_count:0,
        evidence_ledger_ready:true,
        evidence_recorded:false,
        active_wiring_allowed:false
      },
      required_evidence:[
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

echo "Hepta upstream Codex activation evidence ledger gate passed"
