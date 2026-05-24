#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_EVIDENCE_RECEIPT_NO_WRITE_SINK_ADAPTER_CONTRACT.md"

echo "[hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_evidence_receipt_no_write_sink_adapter_contract -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation evidence receipt no-write sink adapter contract missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation evidence receipt no-write sink adapter contract: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract"
require_doc_text "Source invocation dry-run gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh\`"
require_doc_text "No-write sink adapter contract gate:"
require_doc_text "\`scripts/hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract.sh\`"
require_doc_text "Required sink surface count: \`6\`"
require_doc_text "Ready sink surface count: \`6\`"
require_doc_text "Side-effect-free surface count: \`6\`"
require_doc_text "Accepted invocation fixture count: \`3\`"
require_doc_text "Rejected write fixture count: \`3\`"
require_doc_text "Rejected public claim fixture count: \`1\`"
require_doc_text "Persisted receipt count: \`0\`"
require_doc_text "Workspace write performed count: \`0\`"
require_doc_text "Sink write path enabled by default: \`false\`"
require_doc_text "Sink accepts redacted payload hash: \`true\`"
require_doc_text "Sink accepts redacted output path: \`true\`"
require_doc_text "Sink requires operator approval: \`true\`"
require_doc_text "Sink requires fresh trusted records: \`true\`"
require_doc_text "Sink rejects public claim artifact write: \`true\`"
require_doc_text "No-write sink adapter ready: \`true\`"
require_doc_text "Activation blocked by no-write sink adapter: \`true\`"
require_doc_text "Activation allowed by no-write sink adapter: \`false\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "\`redacted_invocation_acceptance\`"
require_doc_text "\`payload_hash_binding\`"
require_doc_text "\`redacted_output_path_binding\`"
require_doc_text "\`operator_approval_requirement\`"
require_doc_text "\`fresh_trusted_record_requirement\`"
require_doc_text "\`public_claim_artifact_rejection\`"
require_doc_text "No command invocation performed"
require_doc_text "No receipt persistence execution"
require_doc_text "No evidence receipt persistence"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg no_write_sink_adapter_id "upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-persistence-invocation-dry-run.sh" \
    --arg sink_gate "scripts/hepta-upstream-codex-activation-evidence-receipt-no-write-sink-adapter-contract.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      no_write_sink_adapter_id:$no_write_sink_adapter_id,
      manifest:$manifest,
      no_write_sink_adapter_doc_path:$doc,
      source_invocation_dry_run_gate:$source_gate,
      no_write_sink_adapter_contract_gate:$sink_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      sink_status:{
        source_invocation_dry_run_ready:true,
        required_sink_surface_count:6,
        ready_sink_surface_count:6,
        side_effect_free_surface_count:6,
        accepted_invocation_fixture_count:3,
        rejected_write_fixture_count:3,
        rejected_public_claim_fixture_count:1,
        persisted_receipt_count:0,
        workspace_write_performed_count:0,
        sink_write_path_enabled_by_default:false,
        sink_accepts_redacted_payload_hash:true,
        sink_accepts_redacted_output_path:true,
        sink_requires_operator_approval:true,
        sink_requires_fresh_trusted_records:true,
        sink_rejects_public_claim_artifact_write:true,
        no_write_sink_adapter_ready:true,
        activation_blocked_by_no_write_sink_adapter:true,
        activation_allowed_by_no_write_sink_adapter:false,
        active_wiring_allowed:false
      },
      sink_surfaces:[
        "redacted_invocation_acceptance",
        "payload_hash_binding",
        "redacted_output_path_binding",
        "operator_approval_requirement",
        "fresh_trusted_record_requirement",
        "public_claim_artifact_rejection"
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
        command_invocation_performed:false,
        receipt_persistence_execution:false,
        workspace_write:false,
        evidence_receipt_persistence:false,
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

echo "Hepta upstream Codex activation evidence receipt no-write sink adapter contract gate passed"
