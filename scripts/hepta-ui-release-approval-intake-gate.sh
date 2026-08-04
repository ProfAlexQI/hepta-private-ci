#!/bin/bash -p
set +x
PS4='+ '
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"
PATH="$SYSTEM_PATH"
export PATH

cd "$(/usr/bin/dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_RELEASE_APPROVAL_INTAKE_REPORT_PATH:-$READINESS_DIR/ui-release-approval-intake-gate.json}"
INTAKE_DIR="${HEPTA_UI_RELEASE_APPROVAL_INTAKE_DIR:-$READINESS_DIR/release-approval-intake}"
APPROVAL_INPUT_PATH="${HEPTA_UI_RELEASE_APPROVAL_INPUT_PATH:-}"
TEMPLATE_PATH="$INTAKE_DIR/release-approval-template.json"
MARKDOWN_PATH="$INTAKE_DIR/release-approval-intake.md"
ACCEPTED_APPROVAL_INPUT_PATH="$INTAKE_DIR/release-approval-input.accepted.json"
CAPTURED_APPROVAL_INPUT_PATH="$INTAKE_DIR/release-approval-input.captured.json"

SYSTEM_ENV="/usr/bin/env"
SYSTEM_JQ="/usr/bin/jq"
SYSTEM_RUBY="/usr/bin/ruby"
SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"

for bootstrap_system_tool in \
  "$SYSTEM_ENV" \
  "$SYSTEM_JQ" \
  "$SYSTEM_RUBY" \
  /bin/cat \
  /bin/cp \
  /bin/mkdir \
  /bin/rm \
  /usr/bin/dirname \
  /usr/bin/mktemp; do
  if [[ ! -x "$bootstrap_system_tool" || -L "$bootstrap_system_tool" ]]; then
    printf 'Missing canonical macOS bootstrap tool: %s\n' "$bootstrap_system_tool" >&2
    exit 2
  fi
done

jq() { "$SYSTEM_JQ" "$@"; }
cat() { /bin/cat "$@"; }
cp() { /bin/cp "$@"; }
dirname() { /usr/bin/dirname "$@"; }
mkdir() { /bin/mkdir "$@"; }
mktemp() { /usr/bin/mktemp "$@"; }
rm() { /bin/rm "$@"; }

DISTRIBUTION_PREFLIGHT_REPORT_PATH="$READINESS_DIR/native-distribution-preflight-gate.json"
RELEASE_OPERATOR_DRY_RUN_REPORT_PATH="$READINESS_DIR/ui-release-operator-dry-run-gate.json"
OPERATOR_BRIEFING_REFRESH_REPORT_PATH="$READINESS_DIR/ui-operator-briefing-refresh-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"

HEPTA_UI_GATE_REQUIREMENT_CONTEXT="the Hepta UI release approval intake gate"
HEPTA_UI_REPORT_INPUT_LABEL="release approval intake"
source scripts/lib/hepta-ui-gate-common-v1.sh

absolute_path() {
  "$SYSTEM_ENV" -i PATH="$SYSTEM_PATH" HOME="/var/empty" TMPDIR="${TMPDIR:-/tmp}" \
    "$SYSTEM_RUBY" -e 'print File.expand_path(ARGV.fetch(0))' "$1"
}

canonical_path() {
  "$SYSTEM_ENV" -i PATH="$SYSTEM_PATH" HOME="/var/empty" TMPDIR="${TMPDIR:-/tmp}" \
    "$SYSTEM_RUBY" -e '
      cursor = File.expand_path(ARGV.fetch(0))
      suffix = []
      until File.exist?(cursor) || File.dirname(cursor) == cursor
        suffix.unshift(File.basename(cursor))
        cursor = File.dirname(cursor)
      end
      print File.join(File.realpath(cursor), *suffix)
    ' "$1"
}

paths_overlap() {
  "$SYSTEM_ENV" -i PATH="$SYSTEM_PATH" HOME="/var/empty" TMPDIR="${TMPDIR:-/tmp}" \
    "$SYSTEM_RUBY" -e '
      left = File.expand_path(ARGV.fetch(0))
      right = File.expand_path(ARGV.fetch(1))
      overlap = left == right || left.start_with?(right + File::SEPARATOR) || right.start_with?(left + File::SEPARATOR)
      exit(overlap ? 0 : 1)
    ' "$1" "$2"
}

normalize_trust_boundary_path() {
  local label="$1" path="$2" absolute canonical
  absolute="$(absolute_path "$path")"
  canonical="$(canonical_path "$absolute")"
  if [[ "$absolute" != "$canonical" ]]; then
    printf '%s path contains a symlinked component: %s\n' "$label" "$path" >&2
    exit 64
  fi
  printf '%s' "$canonical"
}

require_command jq
require_command shasum

READINESS_DIR="$(normalize_trust_boundary_path readiness "$READINESS_DIR")"
REPORT_PATH="$(normalize_trust_boundary_path report "$REPORT_PATH")"
INTAKE_DIR="$(normalize_trust_boundary_path intake "$INTAKE_DIR")"
if [[ -n "$APPROVAL_INPUT_PATH" ]]; then
  if [[ "$APPROVAL_INPUT_PATH" != /* ]]; then
    printf 'release approval input path must be absolute: %s\n' "$APPROVAL_INPUT_PATH" >&2
    exit 64
  fi
  APPROVAL_INPUT_PATH="$(normalize_trust_boundary_path approval_input "$APPROVAL_INPUT_PATH")"
fi
TEMPLATE_PATH="$INTAKE_DIR/release-approval-template.json"
MARKDOWN_PATH="$INTAKE_DIR/release-approval-intake.md"
ACCEPTED_APPROVAL_INPUT_PATH="$INTAKE_DIR/release-approval-input.accepted.json"
CAPTURED_APPROVAL_INPUT_PATH="$INTAKE_DIR/release-approval-input.captured.json"
DISTRIBUTION_PREFLIGHT_REPORT_PATH="$READINESS_DIR/native-distribution-preflight-gate.json"
RELEASE_OPERATOR_DRY_RUN_REPORT_PATH="$READINESS_DIR/ui-release-operator-dry-run-gate.json"
OPERATOR_BRIEFING_REFRESH_REPORT_PATH="$READINESS_DIR/ui-operator-briefing-refresh-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"

if paths_overlap "$REPORT_PATH" "$INTAKE_DIR"; then
  printf 'release approval report and intake paths must not overlap\n' >&2
  exit 64
fi
if [[ -n "$APPROVAL_INPUT_PATH" ]] \
  && { paths_overlap "$APPROVAL_INPUT_PATH" "$REPORT_PATH" || paths_overlap "$APPROVAL_INPUT_PATH" "$INTAKE_DIR"; }; then
  printf 'release approval input must not overlap report or intake paths\n' >&2
  exit 64
fi
for source_report_path in \
  "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH" \
  "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH" \
  "$EVIDENCE_ARCHIVE_REPORT_PATH"; do
  if paths_overlap "$source_report_path" "$REPORT_PATH" \
    || paths_overlap "$source_report_path" "$INTAKE_DIR" \
    || { [[ -n "$APPROVAL_INPUT_PATH" ]] && paths_overlap "$source_report_path" "$APPROVAL_INPUT_PATH"; }; then
    printf 'release approval source report collides with output or approval input: %s\n' "$source_report_path" >&2
    exit 64
  fi
done
for fixed_output in \
  "$TEMPLATE_PATH" \
  "$MARKDOWN_PATH" \
  "$ACCEPTED_APPROVAL_INPUT_PATH" \
  "$CAPTURED_APPROVAL_INPUT_PATH"; do
  if paths_overlap "$fixed_output" "$REPORT_PATH" \
    || { [[ -n "$APPROVAL_INPUT_PATH" ]] && paths_overlap "$fixed_output" "$APPROVAL_INPUT_PATH"; }; then
    printf 'release approval fixed output collides with report or input: %s\n' "$fixed_output" >&2
    exit 64
  fi
  if [[ -L "$fixed_output" || ( -e "$fixed_output" && ! -f "$fixed_output" ) ]]; then
    printf 'release approval fixed output is not a safe regular-file target: %s\n' "$fixed_output" >&2
    exit 64
  fi
done
if [[ -L "$REPORT_PATH" || ( -e "$REPORT_PATH" && ! -f "$REPORT_PATH" ) ]]; then
  printf 'release approval report is not a safe regular-file target: %s\n' "$REPORT_PATH" >&2
  exit 64
fi
if [[ -e "$INTAKE_DIR" && ! -d "$INTAKE_DIR" ]]; then
  printf 'release approval intake path is not a directory: %s\n' "$INTAKE_DIR" >&2
  exit 64
fi

require_report "$DISTRIBUTION_PREFLIGHT_REPORT_PATH"
require_report "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH"
require_report "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH"
require_report "$EVIDENCE_ARCHIVE_REPORT_PATH"

mkdir -p "$INTAKE_DIR"
if [[ "$(canonical_path "$INTAKE_DIR")" != "$INTAKE_DIR" || -L "$INTAKE_DIR" ]]; then
  printf 'release approval intake path changed during initialization\n' >&2
  exit 64
fi

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-release-approval-intake.XXXXXX")"
REPORT_TMP="$TMP_DIR/release-approval-intake-report.json"
APPROVAL_CAPTURE_PATH="$TMP_DIR/release-approval-input.json"
TEMPLATE_TMP="$TMP_DIR/release-approval-template.json"
MARKDOWN_TMP="$TMP_DIR/release-approval-intake.md"
trap 'rm -rf "$TMP_DIR"' EXIT

atomic_replace_from_file() {
  local source_path="$1" destination_path="$2" destination_dir temporary_path
  destination_dir="$(/usr/bin/dirname "$destination_path")"
  mkdir -p "$destination_dir"
  if [[ "$(canonical_path "$destination_dir")" != "$(absolute_path "$destination_dir")" || -L "$destination_dir" ]]; then
    printf 'release approval output directory contains a symlinked component: %s\n' "$destination_dir" >&2
    return 1
  fi
  if [[ -L "$destination_path" || ( -e "$destination_path" && ! -f "$destination_path" ) ]]; then
    printf 'refusing unsafe release approval output target: %s\n' "$destination_path" >&2
    return 1
  fi
  temporary_path="$(mktemp "$destination_dir/.hepta-release-approval-intake.XXXXXX")"
  if ! /bin/cp "$source_path" "$temporary_path" \
    || [[ ! -f "$temporary_path" || -L "$temporary_path" ]]; then
    /bin/rm -f "$temporary_path"
    return 1
  fi
  if ! "$SYSTEM_ENV" -i PATH="$SYSTEM_PATH" HOME="/var/empty" TMPDIR="${TMPDIR:-/tmp}" \
    "$SYSTEM_RUBY" -e '
      source, destination = ARGV
      if File.symlink?(destination) || (File.exist?(destination) && !File.file?(destination))
        abort "unsafe destination"
      end
      File.rename(source, destination)
      abort "unsafe result" unless File.file?(destination) && !File.symlink?(destination)
    ' "$temporary_path" "$destination_path"; then
    /bin/rm -f "$temporary_path"
    return 1
  fi
}

safe_remove_regular_leaf() {
  local path="$1"
  "$SYSTEM_ENV" -i PATH="$SYSTEM_PATH" HOME="/var/empty" TMPDIR="${TMPDIR:-/tmp}" \
    "$SYSTEM_RUBY" -e '
      path = ARGV.fetch(0)
      begin
        stat = File.lstat(path)
      rescue Errno::ENOENT
        exit 0
      end
      abort "unsafe leaf" unless stat.file? && !stat.symlink?
      File.unlink(path)
    ' "$path"
}

approval_present=false
# No system-verifiable Telegram authorization receipt or cryptographically
# signed approval policy is available in this repository today.  A local JSON
# document is evidence to inspect, not independent release authority.
INDEPENDENT_APPROVAL_VERIFIER_READY=false
approval_input_path_json=null
approval_captured_input_path_json=null
approval_sha_json=null
approval_bytes=0

if [[ -n "$APPROVAL_INPUT_PATH" ]]; then
  require_report "$APPROVAL_INPUT_PATH"
  /bin/cp "$APPROVAL_INPUT_PATH" "$APPROVAL_CAPTURE_PATH"
  approval_present=true
  approval_input_path_json="$(jq -n --arg path "$APPROVAL_INPUT_PATH" '$path')"
  approval_captured_input_path_json="$(jq -n --arg path "$CAPTURED_APPROVAL_INPUT_PATH" '$path')"
  approval_sha_json="$(jq -n --arg sha "$(file_sha256 "$APPROVAL_CAPTURE_PATH")" '$sha')"
  approval_bytes="$(file_bytes "$APPROVAL_CAPTURE_PATH")"
else
  jq -n '{present:false}' >"$APPROVAL_CAPTURE_PATH"
fi

distribution_sha="$(file_sha256 "$DISTRIBUTION_PREFLIGHT_REPORT_PATH")"
release_dry_run_sha="$(file_sha256 "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH")"
operator_refresh_sha="$(file_sha256 "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH")"
evidence_archive_report_sha="$(file_sha256 "$EVIDENCE_ARCHIVE_REPORT_PATH")"

jq -n \
  --slurpfile distribution_file "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --slurpfile release_dry_run_file "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH" \
  --slurpfile operator_refresh_file "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  '
  ($distribution_file[0]) as $distribution
  | ($release_dry_run_file[0]) as $dry
  | ($operator_refresh_file[0]) as $operator_refresh
  | ($evidence_archive_file[0]) as $archive
  | {
      approval_kind:"release_operator_approval",
      approval_version:1,
      owner_lane:"release_operator",
      approved_release_action:"signed_notarized_stapled_public_distribution",
      approval_scope:[
        "sign_hepta_native_app",
        "notarize_hepta_native_app",
        "staple_hepta_native_app",
        "write_public_distribution_artifact"
      ],
      operator_approval_recorded:false,
      operator_identity_hash:"",
      approved_at:"",
      release_target:{
        product:"Hepta Native",
        bundle_identifier:$distribution.package_metadata.bundle_identifier,
        bundle_name:$distribution.package_metadata.bundle_name,
        bundle_executable:$distribution.package_metadata.bundle_executable
      },
      source_evidence:{
        dry_run_manifest_sha256:$dry.dry_run_manifest_sha256,
        evidence_archive_sha256:$archive.archive_sha256,
        unsigned_app_bundle_sha256:$dry.release_candidate.unsigned_app_bundle_sha256,
        operator_briefing_refresh_markdown_sha256:$operator_refresh.refresh_markdown_sha256
      },
      post_approval_requirements:{
        independent_approval_verifier_required:true,
        signed_notarized_stapled_artifact_gate_required:true,
        public_artifact_policy_required:true,
        no_release_claim_from_approval_only:true
      },
      claim_boundary:{
        release_approval_claim_ready:false,
        release_execution_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        live_product_claim_ready:false
      },
      side_effects:{
        external_mutation:false,
        credential_value_read:false,
        keychain_identity_lookup_performed:false,
        network_call_performed:false,
        notary_submission_performed:false,
        app_signed:false,
        app_notarized:false,
        app_stapled:false,
        public_distribution_artifact_written:false
      }
    }' >"$TEMPLATE_TMP"

jq -r '
  "# Hepta UI Release Approval Intake\n\n"
  + "- Kind: local release approval intake contract\n"
  + "- Release action: \(.approved_release_action)\n"
  + "- Target: \(.release_target.product) / \(.release_target.bundle_identifier)\n"
  + "- Approval input env: `HEPTA_UI_RELEASE_APPROVAL_INPUT_PATH`\n"
  + "- Approval alone does not make release, public distribution, or live product claims ready.\n\n"
  + "## Required Approval Fields\n\n"
  + "- `approval_kind`\n"
  + "- `approval_version`\n"
  + "- `owner_lane`\n"
  + "- `approved_release_action`\n"
  + "- `approval_scope`\n"
  + "- `operator_approval_recorded`\n"
  + "- `operator_identity_hash`\n"
  + "- `source_evidence.dry_run_manifest_sha256`\n"
  + "- `source_evidence.evidence_archive_sha256`\n"
  + "- independently verified approval receipt (not currently available)\n"
  + "- `post_approval_requirements.signed_notarized_stapled_artifact_gate_required`\n\n"
  + "## Post-Approval Requirements\n\n"
  + "- signed app evidence\n"
  + "- notarized app evidence\n"
  + "- stapled app evidence\n"
  + "- explicit public artifact policy\n"
' "$TEMPLATE_TMP" >"$MARKDOWN_TMP"

template_sha="$(file_sha256 "$TEMPLATE_TMP")"
template_bytes="$(file_bytes "$TEMPLATE_TMP")"
markdown_sha="$(file_sha256 "$MARKDOWN_TMP")"
markdown_bytes="$(file_bytes "$MARKDOWN_TMP")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_release_approval_intake_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg intake_dir "$INTAKE_DIR" \
  --arg template_path "$TEMPLATE_PATH" \
  --arg markdown_path "$MARKDOWN_PATH" \
  --arg distribution_path "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --arg release_dry_run_path "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH" \
  --arg operator_refresh_path "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH" \
  --arg evidence_archive_path "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --arg distribution_sha "$distribution_sha" \
  --arg release_dry_run_sha "$release_dry_run_sha" \
  --arg operator_refresh_sha "$operator_refresh_sha" \
  --arg evidence_archive_report_sha "$evidence_archive_report_sha" \
  --arg template_sha "$template_sha" \
  --arg markdown_sha "$markdown_sha" \
  --argjson template_bytes "$template_bytes" \
  --argjson markdown_bytes "$markdown_bytes" \
  --argjson approval_present "$approval_present" \
  --argjson independent_approval_verifier_ready "$INDEPENDENT_APPROVAL_VERIFIER_READY" \
  --argjson approval_input_path "$approval_input_path_json" \
  --argjson approval_captured_input_path "$approval_captured_input_path_json" \
  --argjson approval_sha "$approval_sha_json" \
  --argjson approval_bytes "$approval_bytes" \
  --slurpfile distribution_file "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --slurpfile release_dry_run_file "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH" \
  --slurpfile operator_refresh_file "$OPERATOR_BRIEFING_REFRESH_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --slurpfile template_file "$TEMPLATE_TMP" \
  --slurpfile approval_file "$APPROVAL_CAPTURE_PATH" \
  '
  ($distribution_file[0]) as $distribution
  | ($release_dry_run_file[0]) as $dry
  | ($operator_refresh_file[0]) as $operator_refresh
  | ($evidence_archive_file[0]) as $archive
  | ($template_file[0]) as $template
  | ($approval_file[0]) as $approval
  | def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def source_chain_ready:
      $distribution.distribution_preflight_gate_ready == true
      and $distribution.distribution_static_contract_ready == true
      and $distribution.public_distribution_ready == false
      and $distribution.release_approval_required == true
      and $distribution.credential_values_read == false
      and $distribution.keychain_identity_lookup_performed == false
      and $distribution.network_call_performed == false
      and $distribution.notary_submission_performed == false
      and $distribution.public_distribution_artifact_written == false
      and $distribution.app_signed == false
      and $distribution.app_notarized == false
      and $distribution.app_stapled == false
      and $dry.release_operator_dry_run_gate_ready == true
      and $dry.operator_packet.dry_run_only == true
      and $dry.operator_packet.operator_approval_recorded == false
      and $dry.operator_packet.public_distribution_artifact_written == false
      and $dry.claim_boundary.local_release_operator_dry_run_ready == true
      and $dry.claim_boundary.release_execution_ready == false
      and $dry.claim_boundary.release_claim_ready == false
      and $dry.claim_boundary.public_distribution_claim_ready == false
      and ($dry.claim_boundary.blocked_by | index("operator_release_approval_required") != null)
      and ($dry.claim_boundary.blocked_by | index("public_distribution_artifact_not_written") != null)
      and $operator_refresh.operator_briefing_refresh_gate_ready == true
      and $operator_refresh.current_state.root_report_replay_required_count_after_refresh == 33
      and ($operator_refresh.updated_critical_risk_count >= 1 and $operator_refresh.updated_critical_risk_count <= 4)
      and ($operator_refresh.refreshed_operator_briefing.updated_critical_risks | map(.id) | index("release_public_distribution_not_approved") != null)
      and $operator_refresh.claim_boundary.local_operator_briefing_refresh_ready == true
      and $operator_refresh.claim_boundary.public_distribution_claim_ready == false
      and $operator_refresh.claim_boundary.release_claim_ready == false
      and $archive.evidence_archive_gate_ready == true
      and $archive.claim_boundary.local_evidence_archive_ready == true
      and $archive.claim_boundary.public_distribution_claim_ready == false
      and $archive.claim_boundary.release_claim_ready == false
      and $archive.all_extracted_items_sha256_match == true
      and sha_ready($distribution_sha)
      and sha_ready($release_dry_run_sha)
      and sha_ready($operator_refresh_sha)
      and sha_ready($evidence_archive_report_sha);
    def template_ready:
      $template.approval_kind == "release_operator_approval"
      and $template.approval_version == 1
      and $template.owner_lane == "release_operator"
      and $template.approved_release_action == "signed_notarized_stapled_public_distribution"
      and $template.release_target.bundle_identifier == $distribution.package_metadata.bundle_identifier
      and $template.source_evidence.dry_run_manifest_sha256 == $dry.dry_run_manifest_sha256
      and $template.source_evidence.evidence_archive_sha256 == $archive.archive_sha256
      and $template.post_approval_requirements.independent_approval_verifier_required == true
      and $template.post_approval_requirements.signed_notarized_stapled_artifact_gate_required == true
      and $template.post_approval_requirements.no_release_claim_from_approval_only == true
      and sha_ready($template_sha)
      and $template_bytes > 0
      and sha_ready($markdown_sha)
      and $markdown_bytes > 0;
    def approval_evidence_match:
      $approval.source_evidence.dry_run_manifest_sha256 == $dry.dry_run_manifest_sha256
      and $approval.source_evidence.evidence_archive_sha256 == $archive.archive_sha256
      and $approval.source_evidence.unsigned_app_bundle_sha256 == $dry.release_candidate.unsigned_app_bundle_sha256;
    def approval_authorization_context_match:
      (($approval.source_evidence.authorization_message_id // "") | test("^telegram:[0-9]+/[0-9]+$"))
      and (($approval.approved_at // "") | length) > 0
      and $approval.release_target.bundle_identifier == $distribution.package_metadata.bundle_identifier
      and $approval.release_target.bundle_name == $distribution.package_metadata.bundle_name
      and $approval.release_target.bundle_executable == $distribution.package_metadata.bundle_executable;
    def approval_valid:
      $approval_present == true
      and $approval.approval_kind == "release_operator_approval"
      and $approval.approval_version == 1
      and $approval.owner_lane == "release_operator"
      and $approval.approved_release_action == "signed_notarized_stapled_public_distribution"
      and ($approval.approval_scope | index("sign_hepta_native_app") != null)
      and ($approval.approval_scope | index("notarize_hepta_native_app") != null)
      and ($approval.approval_scope | index("staple_hepta_native_app") != null)
      and ($approval.approval_scope | index("write_public_distribution_artifact") != null)
      and $approval.operator_approval_recorded == true
      and (($approval.operator_identity_hash // "") | test("^[0-9a-f]{64}$"))
      and $approval.release_target.bundle_identifier == $distribution.package_metadata.bundle_identifier
      and approval_evidence_match
      and approval_authorization_context_match
      and $independent_approval_verifier_ready == true
      and $approval.post_approval_requirements.signed_notarized_stapled_artifact_gate_required == true
      and $approval.post_approval_requirements.no_release_claim_from_approval_only == true
      and ($approval.claim_boundary.release_claim_ready // false) == false
      and ($approval.claim_boundary.public_distribution_claim_ready // false) == false
      and ($approval.side_effects.external_mutation // false) == false
      and sha_ready($approval_sha)
      and $approval_bytes > 0;
    (source_chain_ready and template_ready and (($approval_present == false) or approval_valid)) as $ready
  | (if $approval_present then approval_valid else false end) as $approval_ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      release_approval_intake_gate_ready:$ready,
      intake_kind:"local_release_approval_intake_contract",
      intake_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      intake_dir:$intake_dir,
      template_path:$template_path,
      markdown_path:$markdown_path,
      approval_input_path:$approval_input_path,
      approval_captured_input_path:$approval_captured_input_path,
      approval_input_sha256:$approval_sha,
      approval_input_bytes:$approval_bytes,
      template_sha256:$template_sha,
      template_bytes:$template_bytes,
      markdown_sha256:$markdown_sha,
      markdown_bytes:$markdown_bytes,
      source_reports:{
        native_distribution_preflight:$distribution_path,
        release_operator_dry_run:$release_dry_run_path,
        operator_briefing_refresh:$operator_refresh_path,
        evidence_archive:$evidence_archive_path
      },
      source_report_sha256:{
        native_distribution_preflight:$distribution_sha,
        release_operator_dry_run:$release_dry_run_sha,
        operator_briefing_refresh:$operator_refresh_sha,
        evidence_archive:$evidence_archive_report_sha
      },
      release_approval_state:{
        waiting_for_release_approval:($approval_present | not),
        release_approval_present:$approval_present,
        release_approval_valid:$approval_ready,
        release_approval_claim_ready:$approval_ready,
        independent_approval_verifier_ready:$independent_approval_verifier_ready,
        self_reported_approval_can_authorize_release:false,
        approval_only_can_make_release_claim:false,
        signed_notarized_stapled_artifact_present:false,
        public_distribution_artifact_written:false,
        root_report_replay_required_count_after_intake:34
      },
      release_candidate:{
        bundle_identifier:$distribution.package_metadata.bundle_identifier,
        bundle_name:$distribution.package_metadata.bundle_name,
        bundle_executable:$distribution.package_metadata.bundle_executable,
        unsigned_app_bundle_sha256:$dry.release_candidate.unsigned_app_bundle_sha256,
        evidence_archive_sha256:$archive.archive_sha256,
        evidence_archive_bytes:$archive.archive_bytes
      },
      approval_template:$template,
      approval_blockers:(
        if $approval_ready then
          ["signed_notarized_stapled_artifact_missing","public_distribution_artifact_not_written"]
        else
          ["operator_release_approval_required","independent_release_approval_verifier_unavailable","signed_notarized_stapled_artifact_missing","public_distribution_artifact_not_written"]
        end
      ),
      source_alignment:{
        native_distribution_preflight_ready:$distribution.distribution_preflight_gate_ready,
        release_operator_dry_run_ready:$dry.release_operator_dry_run_gate_ready,
        operator_briefing_refresh_ready:$operator_refresh.operator_briefing_refresh_gate_ready,
        evidence_archive_ready:$archive.evidence_archive_gate_ready,
        dry_run_manifest_sha256:$dry.dry_run_manifest_sha256,
        evidence_archive_sha256:$archive.archive_sha256,
        release_public_distribution_not_approved_risk_present:($operator_refresh.refreshed_operator_briefing.updated_critical_risks | map(.id) | index("release_public_distribution_not_approved") != null),
        approval_valid_branch_supported:$independent_approval_verifier_ready,
        root_report_replay_required_count_after_intake:34
      },
      claim_boundary:{
        local_release_approval_intake_ready:$ready,
        release_approval_claim_ready:$approval_ready,
        release_execution_ready:false,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false,
        public_upload_performed:false,
        signing_notarization_performed:false
      },
      side_effects:{
        filesystem_read:true,
        local_template_written:true,
        local_markdown_written:true,
        local_report_written:true,
        credential_value_read:false,
        keychain_identity_lookup_performed:false,
        network_call_performed:false,
        notary_submission_performed:false,
        app_signed:false,
        app_notarized:false,
        app_stapled:false,
        public_distribution_artifact_written:false,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        external_mutation:false
      }
    }' >"$REPORT_TMP"

jq -e '
  .intake_kind == "local_release_approval_intake_contract"
  and .intake_version == 1
  and (.template_sha256 | test("^[0-9a-f]{64}$"))
  and .template_bytes > 0
  and (.markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .markdown_bytes > 0
  and (
    (
      .status == "ready"
      and .release_approval_intake_gate_ready == true
      and (
        (
          .release_approval_state.waiting_for_release_approval == true
          and .release_approval_state.release_approval_present == false
          and .release_approval_state.release_approval_valid == false
        )
        or (
          .release_approval_state.waiting_for_release_approval == false
          and .release_approval_state.release_approval_present == true
          and .release_approval_state.release_approval_valid == true
        )
      )
    )
    or (
      .status == "failed"
      and .release_approval_intake_gate_ready == false
      and .release_approval_state.release_approval_valid == false
    )
  )
  and .release_approval_state.approval_only_can_make_release_claim == false
  and .release_approval_state.independent_approval_verifier_ready == false
  and .release_approval_state.self_reported_approval_can_authorize_release == false
  and (.approval_blockers | index("independent_release_approval_verifier_unavailable") != null)
  and .release_approval_state.signed_notarized_stapled_artifact_present == false
  and .release_approval_state.public_distribution_artifact_written == false
  and .release_approval_state.root_report_replay_required_count_after_intake == 34
  and .source_alignment.native_distribution_preflight_ready == true
  and .source_alignment.release_operator_dry_run_ready == true
  and .source_alignment.operator_briefing_refresh_ready == true
  and .source_alignment.evidence_archive_ready == true
  and .source_alignment.release_public_distribution_not_approved_risk_present == true
  and .source_alignment.approval_valid_branch_supported == false
  and .claim_boundary.local_release_approval_intake_ready == .release_approval_intake_gate_ready
  and .claim_boundary.release_execution_ready == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .claim_boundary.public_upload_performed == false
  and .claim_boundary.signing_notarization_performed == false
  and .side_effects.local_template_written == true
  and .side_effects.local_markdown_written == true
  and .side_effects.credential_value_read == false
  and .side_effects.keychain_identity_lookup_performed == false
  and .side_effects.network_call_performed == false
  and .side_effects.notary_submission_performed == false
  and .side_effects.app_signed == false
  and .side_effects.app_notarized == false
  and .side_effects.app_stapled == false
  and .side_effects.public_distribution_artifact_written == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

atomic_replace_from_file "$TEMPLATE_TMP" "$TEMPLATE_PATH"
atomic_replace_from_file "$MARKDOWN_TMP" "$MARKDOWN_PATH"
atomic_replace_from_file "$APPROVAL_CAPTURE_PATH" "$CAPTURED_APPROVAL_INPUT_PATH"
if jq -e '.release_approval_state.release_approval_valid == true' "$REPORT_TMP" >/dev/null; then
  atomic_replace_from_file "$APPROVAL_CAPTURE_PATH" "$ACCEPTED_APPROVAL_INPUT_PATH"
else
  safe_remove_regular_leaf "$ACCEPTED_APPROVAL_INPUT_PATH"
fi
atomic_replace_from_file "$REPORT_TMP" "$REPORT_PATH"
/bin/cat "$REPORT_TMP"
if jq -e '.status == "ready" and .release_approval_intake_gate_ready == true' "$REPORT_TMP" >/dev/null; then
  exit 0
fi
exit 1
