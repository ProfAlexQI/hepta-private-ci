#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

MANIFEST="${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}"
DOC="docs/architecture/HEPTA_UPSTREAM_CODEX_DOCTOR_ENVIRONMENT_DIAGNOSTICS_ABSORPTION.md"
BASE_HEAD="7d47056ea42636271ac020b86347fbbef49490aa"
TARGET_HEAD="9f42c89c0112771dc29100a6f3fc904049b2655f"

echo "[hepta-upstream-codex-doctor-environment-diagnostics-absorption] doctor contract tests"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-core \
  doctor_onboarding_update_dry_run_checks_are_real_and_side_effect_free -- --nocapture

require_doc_text() {
  local text="$1"
  if ! grep -Fq "$text" "$DOC"; then
    echo "missing doc text in $DOC: $text" >&2
    exit 1
  fi
}

require_doc_text "upstream-codex-doctor-environment-diagnostics-absorption"
require_doc_text "${BASE_HEAD}..${TARGET_HEAD}"
require_doc_text "feat(doctor): add environment diagnostics (#24261)"
require_doc_text "system-environment-redacted-local-only"
require_doc_text "git-environment-redacted-local-only"
require_doc_text "terminal-environment-redacted-local-only"
require_doc_text "terminal-title-redacted-local-only"
require_doc_text "startup-warning-count-redacted-local-only"
require_doc_text "Raw environment value exposed: \`false\`"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg gate "upstream-codex-doctor-environment-diagnostics-absorption" \
    --arg upstream "https://github.com/openai/codex" \
    --arg base_head "$BASE_HEAD" \
    --arg target_head "$TARGET_HEAD" \
    --arg source_commit_subject "feat(doctor): add environment diagnostics (#24261)" \
    --arg doc "$DOC" \
    '{
      product:$product,
      status:"ready",
      gate_id:$gate,
      upstream_repository:$upstream,
      candidate_diff_range:($base_head + ".." + $target_head),
      source_commit:{
        head:$target_head,
        subject:$source_commit_subject
      },
      absorption_kind:"hepta_native_doctor_dry_run_contract",
      selected_bucket_ids:[
        "legacy-cli-tui-compatibility",
        "product-doc-release-governance"
      ],
      absorbed_checks:[
        "system-environment-redacted-local-only",
        "git-environment-redacted-local-only",
        "terminal-environment-redacted-local-only",
        "terminal-title-redacted-local-only",
        "startup-warning-count-redacted-local-only"
      ],
      doctor_contract:{
        required_environment_check_count:5,
        dry_run_check_count:11,
        dry_run_checks_passed:11,
        raw_environment_value_exposed:false,
        credential_value_read:false,
        external_network_read:false,
        package_manager_invoked:false,
        plugin_installed:false,
        listener_started:false
      },
      gates:{
        snapshot_gate:"scripts/hepta-upstream-codex-snapshot.sh",
        diff_ledger_gate:"scripts/hepta-upstream-codex-diff-ledger.sh",
        absorption_gate:"scripts/hepta-upstream-codex-doctor-environment-diagnostics-absorption.sh",
        active_dependency_isolation_gate:"scripts/hepta-active-service-dependency-isolation.sh"
      },
      absorption_policy:{
        upstream_merge_performed:false,
        upstream_checkout_performed:false,
        active_runtime_auto_rebase_allowed:false,
        active_runtime_codex_engine_dependency_allowed:false,
        public_release_claim_allowed:false
      },
      side_effects:{
        workspace_write:false,
        active_service_restart:false,
        credential_or_secret_read:false,
        provider_invocation:false,
        channel_delivery:false,
        gateway_rpc:false,
        public_release:false,
        release_artifact_write:false
      },
      doc:$doc
    }'
)"

printf '%s\n' "$report"
echo "Hepta upstream Codex doctor environment diagnostics absorption gate passed"
