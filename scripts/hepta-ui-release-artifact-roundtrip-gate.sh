#!/bin/bash -p
set +x
PS4='+ '
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"
PATH="$SYSTEM_PATH"
export PATH

cd "$(/usr/bin/dirname "$0")/.."
REPO_ROOT="$(pwd -P)"
. "$REPO_ROOT/scripts/lib/hepta-safe-managed-output-v1.sh"

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH:-$READINESS_DIR/ui-release-artifact-roundtrip-gate.json}"
ROUNDTRIP_DIR="${HEPTA_UI_RELEASE_ARTIFACT_ROUNDTRIP_DIR:-$READINESS_DIR/release-artifact-roundtrip}"
LEGACY_ARTIFACT_PATH="$ROUNDTRIP_DIR/legacy-v1-simulated-artifact.json"
LEGACY_REJECTION_INTAKE_DIR="$ROUNDTRIP_DIR/legacy-v1-rejection-intake"
LEGACY_REJECTION_REPORT_PATH="$ROUNDTRIP_DIR/ui-release-artifact-intake-legacy-v1-rejected.json"
WAITING_INTAKE_DIR="$ROUNDTRIP_DIR/waiting-intake"
WAITING_INTAKE_REPORT_PATH="$ROUNDTRIP_DIR/ui-release-artifact-intake-waiting-gate.json"
V3_SELFTEST_LOG_PATH="$ROUNDTRIP_DIR/release-artifact-intake-v3-self-test.log"

RELEASE_ARTIFACT_INTAKE_REPORT_PATH="$READINESS_DIR/ui-release-artifact-intake-gate.json"
RELEASE_ARTIFACT_TEMPLATE_PATH="$READINESS_DIR/release-artifact-intake/release-artifact-template.json"
RELEASE_ARTIFACT_MARKDOWN_PATH="$READINESS_DIR/release-artifact-intake/release-artifact-intake.md"

require_command() {
  command -v "$1" >/dev/null 2>&1 || {
    printf '%s is required for the Hepta UI release artifact roundtrip gate\n' "$1" >&2
    exit 2
  }
}

require_report() {
  local path="$1"
  [[ -s "$path" ]] || {
    printf 'Missing required release artifact roundtrip input: %s\n' "$path" >&2
    exit 1
  }
  jq empty "$path" >/dev/null
}

require_file() {
  local path="$1"
  [[ -s "$path" ]] || {
    printf 'Missing required release artifact roundtrip file: %s\n' "$path" >&2
    exit 1
  }
}

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

file_bytes() {
  wc -c <"$1" | tr -d ' '
}

require_command jq
require_command shasum

READINESS_DIR="$(hepta_safe_normalize_path readiness "$READINESS_DIR")"
REPORT_PATH="$(hepta_safe_normalize_path report "$REPORT_PATH")"
ROUNDTRIP_DIR="$(hepta_safe_normalize_path roundtrip "$ROUNDTRIP_DIR")"
LEGACY_ARTIFACT_PATH="$ROUNDTRIP_DIR/legacy-v1-simulated-artifact.json"
LEGACY_REJECTION_INTAKE_DIR="$ROUNDTRIP_DIR/legacy-v1-rejection-intake"
LEGACY_REJECTION_REPORT_PATH="$ROUNDTRIP_DIR/ui-release-artifact-intake-legacy-v1-rejected.json"
WAITING_INTAKE_DIR="$ROUNDTRIP_DIR/waiting-intake"
WAITING_INTAKE_REPORT_PATH="$ROUNDTRIP_DIR/ui-release-artifact-intake-waiting-gate.json"
V3_SELFTEST_LOG_PATH="$ROUNDTRIP_DIR/release-artifact-intake-v3-self-test.log"
LEGACY_STDOUT_PATH="$ROUNDTRIP_DIR/legacy-v1-rejection.stdout"
LEGACY_STDERR_PATH="$ROUNDTRIP_DIR/legacy-v1-rejection.stderr"
RELEASE_ARTIFACT_INTAKE_REPORT_PATH="$READINESS_DIR/ui-release-artifact-intake-gate.json"
RELEASE_ARTIFACT_TEMPLATE_PATH="$READINESS_DIR/release-artifact-intake/release-artifact-template.json"
RELEASE_ARTIFACT_MARKDOWN_PATH="$READINESS_DIR/release-artifact-intake/release-artifact-intake.md"
REPORT_PARENT="$(hepta_safe_normalize_path report_parent "$(/usr/bin/dirname "$REPORT_PATH")")"

hepta_safe_require_directory_target readiness "$READINESS_DIR"
hepta_safe_require_directory_target roundtrip "$ROUNDTRIP_DIR"
hepta_safe_require_directory_target legacy_rejection_intake "$LEGACY_REJECTION_INTAKE_DIR"
hepta_safe_require_directory_target waiting_intake "$WAITING_INTAKE_DIR"
hepta_safe_require_directory_target report_parent "$REPORT_PARENT"
for safe_file in \
  "$REPORT_PATH" "$LEGACY_ARTIFACT_PATH" "$LEGACY_REJECTION_REPORT_PATH" \
  "$WAITING_INTAKE_REPORT_PATH" "$V3_SELFTEST_LOG_PATH" \
  "$LEGACY_STDOUT_PATH" "$LEGACY_STDERR_PATH"; do
  hepta_safe_require_regular_target roundtrip_output "$safe_file"
done
if hepta_safe_paths_overlap "$READINESS_DIR" "$REPO_ROOT"; then
  printf 'release artifact roundtrip readiness must not overlap the repository\n' >&2
  exit 64
fi
if ! hepta_safe_is_strict_descendant "$ROUNDTRIP_DIR" "$READINESS_DIR"; then
  printf 'release artifact roundtrip directory must be a strict readiness child\n' >&2
  exit 64
fi
if [[ "$REPORT_PARENT" != "$READINESS_DIR" ]] \
  && ! hepta_safe_is_strict_descendant "$REPORT_PARENT" "$READINESS_DIR"; then
  printf 'release artifact roundtrip report parent must remain inside readiness\n' >&2
  exit 64
fi
if hepta_safe_paths_overlap "$REPORT_PATH" "$ROUNDTRIP_DIR"; then
  printf 'release artifact roundtrip report and managed directory must be disjoint\n' >&2
  exit 64
fi
for protected_input in \
  "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH" \
  "$RELEASE_ARTIFACT_TEMPLATE_PATH" \
  "$RELEASE_ARTIFACT_MARKDOWN_PATH"; do
  if hepta_safe_paths_overlap "$protected_input" "$ROUNDTRIP_DIR" \
    || hepta_safe_paths_overlap "$protected_input" "$REPORT_PATH"; then
    printf 'release artifact roundtrip output overlaps protected input: %s\n' "$protected_input" >&2
    exit 64
  fi
done
require_report "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH"
require_report "$RELEASE_ARTIFACT_TEMPLATE_PATH"
require_file "$RELEASE_ARTIFACT_MARKDOWN_PATH"

/bin/mkdir -p "$REPORT_PARENT" "$ROUNDTRIP_DIR" "$LEGACY_REJECTION_INTAKE_DIR" "$WAITING_INTAKE_DIR"
hepta_safe_revalidate_directory report_parent "$REPORT_PARENT"
hepta_safe_revalidate_directory roundtrip "$ROUNDTRIP_DIR"
hepta_safe_revalidate_directory legacy_rejection_intake "$LEGACY_REJECTION_INTAKE_DIR"
hepta_safe_revalidate_directory waiting_intake "$WAITING_INTAKE_DIR"

TMP_DIR="$(/usr/bin/mktemp -d /private/tmp/hepta-ui-release-artifact-roundtrip.XXXXXX)"
REPORT_TMP="$TMP_DIR/release-artifact-roundtrip-report.json"
LEGACY_ARTIFACT_TMP="$TMP_DIR/legacy-v1-simulated-artifact.json"
LEGACY_STDOUT_TMP="$TMP_DIR/legacy-v1-rejection.stdout"
LEGACY_STDERR_TMP="$TMP_DIR/legacy-v1-rejection.stderr"
V3_SELFTEST_LOG_TMP="$TMP_DIR/release-artifact-intake-v3-self-test.log"
cleanup() {
  /bin/rm -rf "$TMP_DIR"
}
trap cleanup EXIT INT TERM

template_sha="$(file_sha256 "$RELEASE_ARTIFACT_TEMPLATE_PATH")"
template_bytes="$(file_bytes "$RELEASE_ARTIFACT_TEMPLATE_PATH")"
markdown_sha="$(file_sha256 "$RELEASE_ARTIFACT_MARKDOWN_PATH")"
markdown_bytes="$(file_bytes "$RELEASE_ARTIFACT_MARKDOWN_PATH")"
main_intake_waiting="$(jq -r '.release_artifact_state.waiting_for_release_artifact' "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH")"
main_intake_waiting_contract_ready=false
if jq -e '
  .status == "ready"
  and .release_artifact_intake_gate_ready == true
  and .release_artifact_state.waiting_for_release_artifact == true
  and .release_artifact_state.release_artifact_present == false
  and .release_artifact_state.release_artifact_valid == false
  and .release_artifact_state.signed_notarized_stapled_artifact_present == false
  and .release_artifact_state.local_distribution_artifact_written == false
  and .release_artifact_state.public_distribution_artifact_written == false
  and .release_artifact_state.public_upload_performed == false
  and .source_alignment.present_artifact_branch_supported == false
  and .source_alignment.independent_approval_verifier_contract_ready == false
  and (.release_artifact_blockers | index("release_artifact_present_branch_unsupported_without_independent_approval_verifier")) != null
' "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH" >/dev/null; then
  main_intake_waiting_contract_ready=true
fi

# This receipt is intentionally legacy and simulated. The production intake
# must reject it; it exists only to prove the fail-closed boundary.
jq -n \
  --slurpfile template_file "$RELEASE_ARTIFACT_TEMPLATE_PATH" \
  --slurpfile intake_file "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH" \
  --arg template_sha "$template_sha" \
  --arg markdown_sha "$markdown_sha" \
  --argjson template_bytes "$template_bytes" \
  '
  ($template_file[0]) as $template
  | ($intake_file[0]) as $intake
  | {
      artifact_kind:"signed_notarized_stapled_artifact",
      artifact_version:1,
      artifact_mode:"local_simulated_artifact_roundtrip_only",
      owner_lane:"release_operator",
      product:$template.product,
      bundle_identifier:$template.bundle_identifier,
      release_approval_valid:true,
      simulated_provenance:{
        source:"hepta-ui-release-artifact-roundtrip-gate",
        release_operator_execution_performed:false,
        network_call_performed:false,
        signing_performed:false,
        notarization_performed:false,
        stapling_performed:false,
        public_upload_performed:false,
        external_mutation:false
      },
      artifact_evidence:{
        signed:true,
        notarized:true,
        stapled:true,
        local_distribution_artifact_written:true,
        public_distribution_artifact_written:true,
        public_distribution_artifact_semantics:"local_simulated_signed_notarized_stapled_dmg_written_not_public_upload",
        public_upload_performed:false,
        signed_artifact_path:"/tmp/hepta-simulated-release-artifact-roundtrip.dmg",
        signed_artifact_sha256:$template_sha,
        signed_artifact_bytes:$template_bytes,
        notarization_ticket_sha256:$intake.source_report_sha256.release_artifact_boundary,
        codesign_verify_app_sha256:$intake.source_report_sha256.native_distribution_preflight,
        codesign_verify_dmg_sha256:$intake.source_report_sha256.release_approval_intake,
        stapler_staple_sha256:$intake.source_report_sha256.release_artifact_boundary,
        stapler_validate_sha256:$intake.source_report_sha256.evidence_archive,
        spctl_assessment_sha256:$markdown_sha,
        notarytool_submit_log_path:"/tmp/hepta-simulated-notarytool-submit.log",
        codesign_verify_app_log_path:"/tmp/hepta-simulated-codesign-verify-app.log",
        codesign_verify_dmg_log_path:"/tmp/hepta-simulated-codesign-verify-dmg.log",
        stapler_staple_log_path:"/tmp/hepta-simulated-stapler-staple.log",
        stapler_validate_log_path:"/tmp/hepta-simulated-stapler-validate.log",
        spctl_assessment_log_path:"/tmp/hepta-simulated-spctl-assess.log",
        signing_identity:"simulated-developer-id-application",
        notary_auth_mode:"keychain_profile"
      },
      claim_boundary:{release_artifact_claim_ready:false,release_execution_ready:false,live_product_claim_ready:false,public_distribution_claim_ready:false,release_claim_ready:false},
      side_effects:{network_call_performed:false,notary_submission_performed:false,app_signed:false,app_notarized:false,app_stapled:false,local_distribution_artifact_written:false,public_distribution_artifact_written:false,public_upload_performed:false,external_mutation:false}
    }
  ' >"$LEGACY_ARTIFACT_TMP"
hepta_safe_atomic_replace "$LEGACY_ARTIFACT_TMP" "$LEGACY_ARTIFACT_PATH" legacy_artifact

if env \
  HEPTA_UI_PRODUCT_READINESS_DIR="$READINESS_DIR" \
  HEPTA_UI_RELEASE_ARTIFACT_INTAKE_REPORT_PATH="$LEGACY_REJECTION_REPORT_PATH" \
  HEPTA_UI_RELEASE_ARTIFACT_INTAKE_DIR="$LEGACY_REJECTION_INTAKE_DIR" \
  HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH="$LEGACY_ARTIFACT_PATH" \
  ./scripts/hepta-ui-release-artifact-intake-gate.sh \
  >"$LEGACY_STDOUT_TMP" \
  2>"$LEGACY_STDERR_TMP"; then
  hepta_safe_atomic_replace "$LEGACY_STDOUT_TMP" "$LEGACY_STDOUT_PATH" legacy_stdout
  hepta_safe_atomic_replace "$LEGACY_STDERR_TMP" "$LEGACY_STDERR_PATH" legacy_stderr
  printf 'Production release artifact intake accepted a legacy simulated v1 receipt\n' >&2
  exit 1
fi
hepta_safe_atomic_replace "$LEGACY_STDOUT_TMP" "$LEGACY_STDOUT_PATH" legacy_stdout
hepta_safe_atomic_replace "$LEGACY_STDERR_TMP" "$LEGACY_STDERR_PATH" legacy_stderr
require_report "$LEGACY_REJECTION_REPORT_PATH"

if HEPTA_UI_RELEASE_ARTIFACT_INTAKE_V3_SKIP_ROUNDTRIP=1 \
  ./scripts/hepta-ui-release-artifact-intake-v3-self-test.sh >"$V3_SELFTEST_LOG_TMP" 2>&1; then
  v3_selftest_ready=true
else
  cat "$V3_SELFTEST_LOG_TMP" >&2
  exit 1
fi
hepta_safe_atomic_replace "$V3_SELFTEST_LOG_TMP" "$V3_SELFTEST_LOG_PATH" v3_selftest_log

roundtrip_artifact_source_mode="waiting_for_real_v3_artifact"
waiting_intake_source_path="$RELEASE_ARTIFACT_INTAKE_REPORT_PATH"
present_intake_source_path="$LEGACY_REJECTION_REPORT_PATH"

require_report "$waiting_intake_source_path"
require_report "$present_intake_source_path"

waiting_intake_sha="$(file_sha256 "$waiting_intake_source_path")"
present_intake_sha="$(file_sha256 "$present_intake_source_path")"
legacy_artifact_sha="$(file_sha256 "$LEGACY_ARTIFACT_PATH")"
legacy_rejection_sha="$(file_sha256 "$LEGACY_REJECTION_REPORT_PATH")"
v3_selftest_log_sha="$(file_sha256 "$V3_SELFTEST_LOG_PATH")"
legacy_artifact_bytes="$(file_bytes "$LEGACY_ARTIFACT_PATH")"
legacy_rejection_bytes="$(file_bytes "$LEGACY_REJECTION_REPORT_PATH")"
present_intake_bytes="$(file_bytes "$present_intake_source_path")"
v3_selftest_log_bytes="$(file_bytes "$V3_SELFTEST_LOG_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_release_artifact_roundtrip_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg roundtrip_dir "$ROUNDTRIP_DIR" \
  --arg waiting_intake_report_path "$waiting_intake_source_path" \
  --arg template_path "$RELEASE_ARTIFACT_TEMPLATE_PATH" \
  --arg markdown_path "$RELEASE_ARTIFACT_MARKDOWN_PATH" \
  --arg legacy_artifact_path "$LEGACY_ARTIFACT_PATH" \
  --arg legacy_rejection_report_path "$LEGACY_REJECTION_REPORT_PATH" \
  --arg present_intake_report_path "$present_intake_source_path" \
  --arg v3_selftest_log_path "$V3_SELFTEST_LOG_PATH" \
  --arg roundtrip_artifact_source_mode "$roundtrip_artifact_source_mode" \
  --arg waiting_intake_sha "$waiting_intake_sha" \
  --arg template_sha "$template_sha" \
  --arg markdown_sha "$markdown_sha" \
  --arg legacy_artifact_sha "$legacy_artifact_sha" \
  --arg legacy_rejection_sha "$legacy_rejection_sha" \
  --arg present_intake_sha "$present_intake_sha" \
  --arg v3_selftest_log_sha "$v3_selftest_log_sha" \
  --argjson template_bytes "$template_bytes" \
  --argjson markdown_bytes "$markdown_bytes" \
  --argjson legacy_artifact_bytes "$legacy_artifact_bytes" \
  --argjson legacy_rejection_bytes "$legacy_rejection_bytes" \
  --argjson present_intake_bytes "$present_intake_bytes" \
  --argjson v3_selftest_log_bytes "$v3_selftest_log_bytes" \
  --argjson v3_selftest_ready "$v3_selftest_ready" \
  --argjson main_intake_waiting_contract_ready "$main_intake_waiting_contract_ready" \
  --slurpfile waiting_intake_file "$waiting_intake_source_path" \
  --slurpfile template_file "$RELEASE_ARTIFACT_TEMPLATE_PATH" \
  --slurpfile legacy_artifact_file "$LEGACY_ARTIFACT_PATH" \
  --slurpfile legacy_rejection_file "$LEGACY_REJECTION_REPORT_PATH" \
  --slurpfile present_intake_file "$present_intake_source_path" \
  '
  ($waiting_intake_file[0]) as $waiting
  | ($template_file[0]) as $template
  | ($legacy_artifact_file[0]) as $legacy
  | ($legacy_rejection_file[0]) as $rejection
  | ($present_intake_file[0]) as $present
  | def sha_ready($value): (($value // "") | test("^[0-9a-f]{64}$"));
    def waiting_branch_ready:
      $waiting.status == "ready"
      and $waiting.release_artifact_intake_gate_ready == true
      and $waiting.intake_version == 3
      and $waiting.release_artifact_state.waiting_for_release_artifact == true
      and $waiting.release_artifact_state.release_artifact_present == false
      and $waiting.release_artifact_state.release_artifact_valid == false
      and $waiting.release_artifact_state.signed_notarized_stapled_artifact_present == false
      and $waiting.release_artifact_state.local_distribution_artifact_written == false
      and $waiting.release_artifact_state.public_distribution_artifact_written == false
      and $waiting.release_artifact_state.public_upload_performed == false
      and $waiting.source_alignment.present_artifact_branch_supported == false
      and $waiting.source_alignment.independent_approval_verifier_contract_ready == false
      and ($waiting.release_artifact_blockers | index("release_artifact_present_branch_unsupported_without_independent_approval_verifier")) != null
      and $waiting.claim_boundary.release_artifact_claim_ready == false
      and $waiting.claim_boundary.public_distribution_claim_ready == false;
    def legacy_fixture_ready:
      $legacy.artifact_kind == "signed_notarized_stapled_artifact"
      and $legacy.artifact_version == 1
      and ($legacy.receipt_contract_version // 0) != 3
      and $legacy.artifact_mode == "local_simulated_artifact_roundtrip_only"
      and $legacy.artifact_evidence.public_distribution_artifact_semantics == "local_simulated_signed_notarized_stapled_dmg_written_not_public_upload"
      and $legacy.simulated_provenance.release_operator_execution_performed == false
      and $legacy.simulated_provenance.network_call_performed == false
      and $legacy.simulated_provenance.external_mutation == false;
    def legacy_rejection_ready:
      $rejection.status == "failed"
      and $rejection.release_artifact_intake_gate_ready == false
      and $rejection.intake_version == 3
      and $rejection.release_artifact_state.release_artifact_present == true
      and $rejection.release_artifact_state.release_artifact_valid == false
      and $rejection.release_artifact_state.signed_notarized_stapled_artifact_present == false
      and $rejection.release_artifact_state.local_distribution_artifact_written == false
      and $rejection.release_artifact_state.public_distribution_artifact_written == false
      and ($rejection.release_artifact_blockers | index("release_artifact_v3_readback_not_verified") != null)
      and $rejection.claim_boundary.release_artifact_claim_ready == false
      and $rejection.claim_boundary.public_distribution_claim_ready == false;
    (
      waiting_branch_ready
      and $main_intake_waiting_contract_ready == true
      and legacy_fixture_ready
      and legacy_rejection_ready
      and $v3_selftest_ready == true
      and $roundtrip_artifact_source_mode == "waiting_for_real_v3_artifact"
      and sha_ready($waiting_intake_sha)
      and sha_ready($template_sha)
      and sha_ready($markdown_sha)
      and sha_ready($legacy_artifact_sha)
      and sha_ready($legacy_rejection_sha)
      and sha_ready($present_intake_sha)
      and sha_ready($v3_selftest_log_sha)
      and $template_bytes > 0
      and $markdown_bytes > 0
      and $legacy_artifact_bytes > 0
      and $legacy_rejection_bytes > 0
      and $present_intake_bytes > 0
      and $v3_selftest_log_bytes > 0
    ) as $ready
  | false as $actual_present_ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      release_artifact_roundtrip_gate_ready:$ready,
      roundtrip_kind:"release_artifact_v3_fail_closed_contract_replay",
      roundtrip_version:3,
      roundtrip_artifact_source_mode:$roundtrip_artifact_source_mode,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      roundtrip_dir:$roundtrip_dir,
      source_reports:{
        waiting_intake:$waiting_intake_report_path,
        release_artifact_template:$template_path,
        release_artifact_intake_markdown:$markdown_path,
        legacy_v1_simulated_artifact:$legacy_artifact_path,
        legacy_v1_rejection_intake:$legacy_rejection_report_path,
        actual_present_intake:null,
        v3_intake_selftest_log:$v3_selftest_log_path
      },
      source_report_sha256:{
        waiting_intake:$waiting_intake_sha,
        release_artifact_template:$template_sha,
        release_artifact_intake_markdown:$markdown_sha,
        legacy_v1_simulated_artifact:$legacy_artifact_sha,
        legacy_v1_rejection_intake:$legacy_rejection_sha,
        present_intake:$present_intake_sha,
        v3_intake_selftest_log:$v3_selftest_log_sha
      },
      source_report_bytes:{
        release_artifact_template:$template_bytes,
        release_artifact_intake_markdown:$markdown_bytes,
        legacy_v1_simulated_artifact:$legacy_artifact_bytes,
        legacy_v1_rejection_intake:$legacy_rejection_bytes,
        present_intake:$present_intake_bytes,
        v3_intake_selftest_log:$v3_selftest_log_bytes
      },
      bundle_identifier:$template.bundle_identifier,
      roundtrip_ready_count:(if $ready then 2 else 0 end),
      source_alignment:{
        waiting_branch_ready:waiting_branch_ready,
        present_branch_ready:$actual_present_ready,
        present_artifact_branch_supported:false,
        independent_approval_verifier_contract_ready:false,
        simulated_artifact_ready:false,
        legacy_simulated_artifact_fixture_ready:legacy_fixture_ready,
        legacy_simulated_artifact_rejected:legacy_rejection_ready,
        v3_valid_branch_selftest_ready:$v3_selftest_ready,
        waiting_branch_release_artifact_present:$waiting.release_artifact_state.release_artifact_present,
        waiting_branch_release_artifact_valid:$waiting.release_artifact_state.release_artifact_valid,
        present_branch_release_artifact_present:(if $actual_present_ready then $present.release_artifact_state.release_artifact_present else false end),
        present_branch_release_artifact_valid:(if $actual_present_ready then $present.release_artifact_state.release_artifact_valid else false end),
        present_branch_signed_notarized_stapled_artifact_present:(if $actual_present_ready then $present.release_artifact_state.signed_notarized_stapled_artifact_present else false end),
        present_branch_local_distribution_artifact_written:(if $actual_present_ready then $present.release_artifact_state.local_distribution_artifact_written else false end),
        present_branch_public_distribution_artifact_written:(if $actual_present_ready then $present.release_artifact_state.public_distribution_artifact_written else false end),
        present_branch_public_upload_performed:(if $actual_present_ready then $present.release_artifact_state.public_upload_performed else false end),
        present_branch_public_distribution_artifact_semantics:(if $actual_present_ready then $present.release_artifact_state.public_distribution_artifact_semantics else "waiting_for_real_v3_artifact" end),
        present_branch_codesign_verify_app_ready:(if $actual_present_ready then $present.release_artifact_state.codesign_verify_app_ready else false end),
        present_branch_codesign_verify_dmg_ready:(if $actual_present_ready then $present.release_artifact_state.codesign_verify_dmg_ready else false end),
        present_branch_stapler_staple_ready:(if $actual_present_ready then $present.release_artifact_state.stapler_staple_ready else false end),
        present_branch_stapler_validate_ready:(if $actual_present_ready then $present.release_artifact_state.stapler_validate_ready else false end),
        present_branch_spctl_assessment_ready:(if $actual_present_ready then $present.release_artifact_state.spctl_assessment_ready else false end),
        present_branch_source_public_upload_performed:(if $actual_present_ready then ($present.release_artifact_source_side_effects.public_upload_performed // false) else false end),
        present_branch_release_approval_valid:$waiting.source_alignment.release_approval_valid,
        present_branch_operator_release_approval_required:(if $actual_present_ready then (($present.release_artifact_blockers | index("operator_release_approval_required")) != null) else (($waiting.release_artifact_blockers | index("operator_release_approval_required")) != null) end),
        present_branch_post_artifact_refresh_required:true,
        present_branch_real_backend_receipt_missing:(if $actual_present_ready then (($present.release_artifact_blockers | index("real_backend_receipt_missing")) != null) else (($waiting.release_artifact_blockers | index("real_backend_receipt_missing")) != null) end),
        present_branch_real_backend_receipt_claim_ready:(if $actual_present_ready then ($present.source_alignment.real_backend_receipt_claim_ready // false) else ($waiting.source_alignment.real_backend_receipt_claim_ready // false) end),
        present_branch_release_artifact_claim_ready:false,
        present_branch_release_claim_ready:false,
        root_report_replay_required_count_after_roundtrip:41
      },
      release_artifact_blockers:[
        "real_signed_notarized_stapled_artifact_missing",
        "release_artifact_present_branch_unsupported_without_independent_approval_verifier",
        "post_artifact_ui_readiness_refresh_required"
      ],
      claim_boundary:{
        local_release_artifact_roundtrip_ready:$ready,
        local_release_artifact_intake_waiting_branch_ready:waiting_branch_ready,
        local_release_artifact_intake_present_branch_ready:$actual_present_ready,
        simulated_release_artifact_branch_ready:false,
        legacy_simulated_artifact_rejection_ready:legacy_rejection_ready,
        v3_contract_selftest_ready:$v3_selftest_ready,
        release_artifact_claim_ready:false,
        release_execution_ready:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false,
        public_upload_performed:false,
        signing_notarization_performed:false
      },
      side_effects:{
        filesystem_write:true,
        local_legacy_fixture_written:true,
        local_rejection_report_written:true,
        local_v3_selftest_executed:true,
        credential_value_read:false,
        keychain_identity_lookup_performed:false,
        network_call_performed:false,
        notary_submission_performed:false,
        app_signed:false,
        app_notarized:false,
        app_stapled:false,
        local_distribution_artifact_written:false,
        public_distribution_artifact_written:false,
        public_upload_performed:false,
        external_mutation:false
      }
    }
  ' >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .release_artifact_roundtrip_gate_ready == true
  and .roundtrip_kind == "release_artifact_v3_fail_closed_contract_replay"
  and .roundtrip_version == 3
  and .roundtrip_ready_count == 2
  and .source_alignment.waiting_branch_ready == true
  and .source_alignment.simulated_artifact_ready == false
  and .source_alignment.legacy_simulated_artifact_fixture_ready == true
  and .source_alignment.legacy_simulated_artifact_rejected == true
  and .source_alignment.v3_valid_branch_selftest_ready == true
  and .source_alignment.waiting_branch_release_artifact_present == false
  and .source_alignment.waiting_branch_release_artifact_valid == false
  and .roundtrip_artifact_source_mode == "waiting_for_real_v3_artifact"
  and .source_alignment.present_branch_ready == false
  and .source_alignment.present_artifact_branch_supported == false
  and .source_alignment.independent_approval_verifier_contract_ready == false
  and .source_alignment.present_branch_release_artifact_present == false
  and .source_alignment.present_branch_release_artifact_valid == false
  and .source_alignment.present_branch_signed_notarized_stapled_artifact_present == false
  and .source_alignment.present_branch_local_distribution_artifact_written == false
  and .source_alignment.present_branch_public_distribution_artifact_written == false
  and .source_alignment.present_branch_public_distribution_artifact_semantics == "waiting_for_real_v3_artifact"
  and (.release_artifact_blockers | index("real_signed_notarized_stapled_artifact_missing") != null)
  and (.release_artifact_blockers | index("release_artifact_present_branch_unsupported_without_independent_approval_verifier") != null)
  and .source_alignment.present_branch_public_upload_performed == false
  and .source_alignment.present_branch_source_public_upload_performed == false
  and .source_alignment.present_branch_post_artifact_refresh_required == true
  and .source_alignment.present_branch_release_artifact_claim_ready == false
  and .source_alignment.present_branch_release_claim_ready == false
  and .source_alignment.root_report_replay_required_count_after_roundtrip == 41
  and .claim_boundary.local_release_artifact_roundtrip_ready == true
  and .claim_boundary.simulated_release_artifact_branch_ready == false
  and .claim_boundary.legacy_simulated_artifact_rejection_ready == true
  and .claim_boundary.v3_contract_selftest_ready == true
  and .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.release_execution_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .claim_boundary.public_upload_performed == false
  and .side_effects.network_call_performed == false
  and .side_effects.notary_submission_performed == false
  and .side_effects.public_upload_performed == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

hepta_safe_atomic_replace "$REPORT_TMP" "$REPORT_PATH" roundtrip_report
cat "$REPORT_TMP"
