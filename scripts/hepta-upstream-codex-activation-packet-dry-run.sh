#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_ACTIVATION_PACKET_DRY_RUN.md"

echo "[hepta-upstream-codex-activation-packet-dry-run] report contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  upstream_codex_activation_packet_dry_run -- --nocapture

require_doc_text() {
  local pattern="$1"
  if ! rg -q "$pattern" "$DOC"; then
    echo "activation packet dry-run validator missing required text pattern: $pattern" >&2
    exit 1
  fi
}

if [[ ! -f "$DOC" ]]; then
  echo "missing activation packet dry-run validator: $DOC" >&2
  exit 1
fi

require_doc_text "upstream-codex-activation-packet-dry-run-validator"
require_doc_text "Source packet gate: \`scripts/hepta-upstream-codex-activation-request-packet.sh\`"
require_doc_text "Dry-run validator gate: \`scripts/hepta-upstream-codex-activation-packet-dry-run.sh\`"
require_doc_text "Activation packet schema ready: \`true\`"
require_doc_text "Activation packet recorded: \`false\`"
require_doc_text "Required schema field count: \`14\`"
require_doc_text "Fixture count: \`3\`"
require_doc_text "Blocked fixture count: \`3\`"
require_doc_text "Allowed fixture count: \`0\`"
require_doc_text "Dry-run validator ready: \`true\`"
require_doc_text "Active wiring allowed: \`false\`"
require_doc_text "\`empty-placeholder\`"
require_doc_text "\`operator-only-placeholder\`"
require_doc_text "\`public-claim-attempt-without-evidence\`"
require_doc_text "Public release claim allowed: \`false\`"
require_doc_text "Release artifact write allowed: \`false\`"
require_doc_text "No upstream fetch"
require_doc_text "No public release publication"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg validator "upstream-codex-activation-packet-dry-run-validator" \
    --arg manifest "$MANIFEST" \
    --arg doc "$DOC" \
    --arg source_gate "scripts/hepta-upstream-codex-activation-request-packet.sh" \
    --arg validator_gate "scripts/hepta-upstream-codex-activation-packet-dry-run.sh" \
    --arg active_dependency_gate "scripts/hepta-active-service-dependency-isolation.sh" \
    '{
      product:$product,
      status:"ready",
      validator_id:$validator,
      manifest:$manifest,
      validator_doc_path:$doc,
      source_packet_gate:$source_gate,
      dry_run_validator_gate:$validator_gate,
      active_dependency_isolation_gate:$active_dependency_gate,
      schema_status:{
        activation_packet_schema_ready:true,
        activation_packet_recorded:false,
        required_schema_field_count:14,
        fixture_count:3,
        blocked_fixture_count:3,
        allowed_fixture_count:0,
        dry_run_validator_ready:true,
        active_wiring_allowed:false
      },
      fixtures:[
        {
          fixture_id:"empty-placeholder",
          recorded_required_field_count:0,
          missing_required_field_count:14,
          validation_status:"blocked",
          active_wiring_allowed:false
        },
        {
          fixture_id:"operator-only-placeholder",
          recorded_required_field_count:2,
          missing_required_field_count:12,
          validation_status:"blocked",
          active_wiring_allowed:false
        },
        {
          fixture_id:"public-claim-attempt-without-evidence",
          recorded_required_field_count:6,
          missing_required_field_count:8,
          public_release_claim_requested:true,
          release_artifact_write_requested:true,
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

echo "Hepta upstream Codex activation packet dry-run validator gate passed"
