#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_RELEASE_ARTIFACT_INTAKE_REPORT_PATH:-$READINESS_DIR/ui-release-artifact-intake-gate.json}"
INTAKE_DIR="${HEPTA_UI_RELEASE_ARTIFACT_INTAKE_DIR:-$READINESS_DIR/release-artifact-intake}"
ARTIFACT_INPUT_PATH="${HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH:-}"
TEMPLATE_PATH="$INTAKE_DIR/release-artifact-template.json"
MARKDOWN_PATH="$INTAKE_DIR/release-artifact-intake.md"
ACCEPTED_ARTIFACT_INPUT_PATH="$INTAKE_DIR/release-artifact-input.accepted.json"

DISTRIBUTION_PREFLIGHT_REPORT_PATH="$READINESS_DIR/native-distribution-preflight-gate.json"
RELEASE_APPROVAL_INTAKE_REPORT_PATH="$READINESS_DIR/ui-release-approval-intake-gate.json"
RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-release-artifact-boundary-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI release artifact intake gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required release artifact intake input: %s\n' "$path" >&2
    exit 1
  fi
  jq empty "$path" >/dev/null
}

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

file_bytes() {
  wc -c <"$1" | tr -d ' '
}

require_command jq
require_command shasum

require_report "$DISTRIBUTION_PREFLIGHT_REPORT_PATH"
require_report "$RELEASE_APPROVAL_INTAKE_REPORT_PATH"
require_report "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH"
require_report "$EVIDENCE_ARCHIVE_REPORT_PATH"

rm -rf "$INTAKE_DIR"
mkdir -p "$INTAKE_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-release-artifact-intake.XXXXXX")"
REPORT_TMP="$TMP_DIR/release-artifact-intake-report.json"
ARTIFACT_CAPTURE_PATH="$TMP_DIR/release-artifact-input.json"
trap 'rm -rf "$TMP_DIR"' EXIT

artifact_present=false
artifact_input_path_json=null
artifact_captured_input_path_json=null
artifact_sha_json=null
artifact_bytes=0

if [[ -n "$ARTIFACT_INPUT_PATH" ]]; then
  require_report "$ARTIFACT_INPUT_PATH"
  cp "$ARTIFACT_INPUT_PATH" "$ARTIFACT_CAPTURE_PATH"
  cp "$ARTIFACT_INPUT_PATH" "$ACCEPTED_ARTIFACT_INPUT_PATH"
  artifact_present=true
  artifact_input_path_json="$(jq -n --arg path "$ARTIFACT_INPUT_PATH" '$path')"
  artifact_captured_input_path_json="$(jq -n --arg path "$ACCEPTED_ARTIFACT_INPUT_PATH" '$path')"
  artifact_sha_json="$(jq -n --arg sha "$(file_sha256 "$ARTIFACT_INPUT_PATH")" '$sha')"
  artifact_bytes="$(file_bytes "$ARTIFACT_INPUT_PATH")"
else
  jq -n '{present:false}' >"$ARTIFACT_CAPTURE_PATH"
fi

distribution_sha="$(file_sha256 "$DISTRIBUTION_PREFLIGHT_REPORT_PATH")"
approval_sha="$(file_sha256 "$RELEASE_APPROVAL_INTAKE_REPORT_PATH")"
boundary_sha="$(file_sha256 "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH")"
evidence_archive_report_sha="$(file_sha256 "$EVIDENCE_ARCHIVE_REPORT_PATH")"

jq -n \
  --slurpfile distribution_file "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --slurpfile approval_file "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" \
  --slurpfile boundary_file "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  '
  ($distribution_file[0]) as $distribution
  | ($approval_file[0]) as $approval
  | ($boundary_file[0]) as $boundary
  | ($evidence_archive_file[0]) as $archive
  | {
      artifact_kind:"signed_notarized_stapled_artifact",
      artifact_version:1,
      owner_lane:"release_operator",
      product:"Hepta Native",
      bundle_identifier:$distribution.package_metadata.bundle_identifier,
      bundle_name:$distribution.package_metadata.bundle_name,
      bundle_executable:$distribution.package_metadata.bundle_executable,
      required_state:{
        release_approval_valid_required:true,
        signed_app_artifact_required:true,
        notarized_app_artifact_required:true,
        stapled_app_artifact_required:true,
        local_distribution_artifact_required:true,
        public_distribution_policy_required:true,
        public_upload_must_be_false:true,
        no_live_product_claim_from_artifact_alone:true
      },
      expected_source_evidence:{
        release_approval_template_sha256:$approval.template_sha256,
        release_artifact_boundary_markdown_sha256:$boundary.boundary_markdown_sha256,
        evidence_archive_sha256:$archive.archive_sha256,
        unsigned_app_bundle_sha256:$boundary.release_artifact_boundary.unsigned_app_bundle_sha256
      },
      artifact_evidence:{
        signed:false,
        notarized:false,
        stapled:false,
        local_distribution_artifact_written:false,
        public_distribution_artifact_written:false,
        public_upload_performed:false,
        signed_artifact_sha256:"",
        notarization_ticket_sha256:"",
        stapler_validate_sha256:"",
        spctl_assessment_sha256:""
      },
      claim_boundary:{
        release_artifact_claim_ready:false,
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
        local_distribution_artifact_written:false,
        public_distribution_artifact_written:false,
        public_upload_performed:false
      }
    }' >"$TEMPLATE_PATH"

jq -r '
  "# Hepta UI Release Artifact Intake\n\n"
  + "- Kind: local signed/notarized/stapled artifact intake contract\n"
  + "- Target: \(.product) / \(.bundle_identifier)\n"
  + "- Artifact input env: `HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH`\n"
  + "- Artifact evidence alone does not make live-product, public distribution, or release claims ready.\n\n"
  + "## Required Artifact Evidence\n\n"
  + "- valid release approval\n"
  + "- signed app artifact\n"
  + "- notarized app artifact\n"
  + "- stapled app artifact\n"
  + "- local signed/notarized/stapled DMG artifact-write evidence\n"
  + "- no public upload/public claim from the artifact receipt alone\n"
  + "- post-artifact UI readiness refresh\n"
' "$TEMPLATE_PATH" >"$MARKDOWN_PATH"

template_sha="$(file_sha256 "$TEMPLATE_PATH")"
template_bytes="$(file_bytes "$TEMPLATE_PATH")"
markdown_sha="$(file_sha256 "$MARKDOWN_PATH")"
markdown_bytes="$(file_bytes "$MARKDOWN_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_release_artifact_intake_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg intake_dir "$INTAKE_DIR" \
  --arg template_path "$TEMPLATE_PATH" \
  --arg markdown_path "$MARKDOWN_PATH" \
  --arg distribution_path "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --arg approval_path "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" \
  --arg boundary_path "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" \
  --arg evidence_archive_path "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --arg distribution_sha "$distribution_sha" \
  --arg approval_sha "$approval_sha" \
  --arg boundary_sha "$boundary_sha" \
  --arg evidence_archive_report_sha "$evidence_archive_report_sha" \
  --arg template_sha "$template_sha" \
  --arg markdown_sha "$markdown_sha" \
  --argjson template_bytes "$template_bytes" \
  --argjson markdown_bytes "$markdown_bytes" \
  --argjson artifact_present "$artifact_present" \
  --argjson artifact_input_path "$artifact_input_path_json" \
  --argjson artifact_captured_input_path "$artifact_captured_input_path_json" \
  --argjson artifact_sha "$artifact_sha_json" \
  --argjson artifact_bytes "$artifact_bytes" \
  --slurpfile distribution_file "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --slurpfile approval_file "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" \
  --slurpfile boundary_file "$RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --slurpfile template_file "$TEMPLATE_PATH" \
  --slurpfile artifact_file "$ARTIFACT_CAPTURE_PATH" \
  '
  ($distribution_file[0]) as $distribution
  | ($approval_file[0]) as $approval
  | ($boundary_file[0]) as $boundary
  | ($evidence_archive_file[0]) as $archive
  | ($template_file[0]) as $template
  | ($artifact_file[0]) as $artifact
  | def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def source_chain_ready:
      $distribution.distribution_preflight_gate_ready == true
      and $distribution.distribution_static_contract_ready == true
      and $distribution.public_distribution_ready == false
      and $distribution.app_signed == false
      and $distribution.app_notarized == false
      and $distribution.app_stapled == false
      and $distribution.public_distribution_artifact_written == false
      and $approval.release_approval_intake_gate_ready == true
      and $approval.release_approval_state.approval_only_can_make_release_claim == false
      and $approval.release_approval_state.signed_notarized_stapled_artifact_present == false
      and $approval.release_approval_state.public_distribution_artifact_written == false
      and $approval.claim_boundary.release_execution_ready == false
      and $approval.claim_boundary.public_distribution_claim_ready == false
      and $approval.claim_boundary.release_claim_ready == false
      and $boundary.release_artifact_boundary_gate_ready == true
      and $boundary.release_artifact_boundary.next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
      and $boundary.release_artifact_boundary.signed_notarized_stapled_artifact_present == false
      and $boundary.release_artifact_boundary.public_distribution_artifact_written == false
      and $boundary.claim_boundary.release_artifact_claim_ready == false
      and $boundary.claim_boundary.public_distribution_claim_ready == false
      and $boundary.claim_boundary.release_claim_ready == false
      and $archive.evidence_archive_gate_ready == true
      and $archive.claim_boundary.local_evidence_archive_ready == true
      and $archive.all_extracted_items_sha256_match == true
      and $archive.claim_boundary.public_distribution_claim_ready == false
      and $archive.claim_boundary.release_claim_ready == false
      and sha_ready($distribution_sha)
      and sha_ready($approval_sha)
      and sha_ready($boundary_sha)
      and sha_ready($evidence_archive_report_sha);
    def template_ready:
      $template.artifact_kind == "signed_notarized_stapled_artifact"
      and $template.artifact_version == 1
      and $template.owner_lane == "release_operator"
      and $template.bundle_identifier == $distribution.package_metadata.bundle_identifier
      and $template.required_state.release_approval_valid_required == true
      and $template.required_state.signed_app_artifact_required == true
      and $template.required_state.notarized_app_artifact_required == true
      and $template.required_state.stapled_app_artifact_required == true
      and $template.required_state.local_distribution_artifact_required == true
      and $template.required_state.public_upload_must_be_false == true
      and $template.required_state.no_live_product_claim_from_artifact_alone == true
      and sha_ready($template_sha)
      and $template_bytes > 0
      and sha_ready($markdown_sha)
      and $markdown_bytes > 0;
    def artifact_distribution_semantics:
      ($artifact.artifact_evidence.public_distribution_artifact_semantics // "");
    def artifact_input_valid:
      $artifact_present == true
      and $artifact.artifact_kind == "signed_notarized_stapled_artifact"
      and $artifact.artifact_version == 1
      and $artifact.owner_lane == "release_operator"
      and $artifact.bundle_identifier == $distribution.package_metadata.bundle_identifier
      and $artifact.release_approval_valid == true
      and $artifact.artifact_evidence.signed == true
      and $artifact.artifact_evidence.notarized == true
      and $artifact.artifact_evidence.stapled == true
      and $artifact.artifact_evidence.local_distribution_artifact_written == true
      and $artifact.artifact_evidence.public_distribution_artifact_written == true
      and ($artifact.artifact_evidence.public_upload_performed // false) == false
      and (
        artifact_distribution_semantics == "local_signed_notarized_stapled_dmg_written_not_public_upload"
        or artifact_distribution_semantics == "local_simulated_signed_notarized_stapled_dmg_written_not_public_upload"
      )
      and ($artifact.claim_boundary.release_artifact_claim_ready // false) == false
      and ($artifact.claim_boundary.public_distribution_claim_ready // false) == false
      and ($artifact.claim_boundary.release_claim_ready // false) == false
      and ($artifact.artifact_evidence.signed_artifact_sha256 | test("^[0-9a-f]{64}$"))
      and ($artifact.artifact_evidence.notarization_ticket_sha256 | test("^[0-9a-f]{64}$"))
      and ($artifact.artifact_evidence.stapler_validate_sha256 | test("^[0-9a-f]{64}$"))
      and ($artifact.artifact_evidence.spctl_assessment_sha256 | test("^[0-9a-f]{64}$"));
    (
      source_chain_ready
      and template_ready
      and (($artifact_present == false) or artifact_input_valid)
    ) as $ready
  | (if $artifact_present then artifact_input_valid else false end) as $artifact_valid
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      release_artifact_intake_gate_ready:$ready,
      intake_kind:"local_signed_notarized_stapled_artifact_intake_contract",
      intake_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      intake_dir:$intake_dir,
      template_path:$template_path,
      markdown_path:$markdown_path,
      source_reports:{
        native_distribution_preflight:$distribution_path,
        release_approval_intake:$approval_path,
        release_artifact_boundary:$boundary_path,
        evidence_archive:$evidence_archive_path
      },
      source_report_sha256:{
        native_distribution_preflight:$distribution_sha,
        release_approval_intake:$approval_sha,
        release_artifact_boundary:$boundary_sha,
        evidence_archive:$evidence_archive_report_sha
      },
      template_sha256:$template_sha,
      template_bytes:$template_bytes,
      markdown_sha256:$markdown_sha,
      markdown_bytes:$markdown_bytes,
      root_report_replay_required_count_after_intake:37,
      release_artifact_state:{
        waiting_for_release_artifact:($artifact_present == false),
        release_artifact_present:$artifact_present,
        release_artifact_input_path:$artifact_input_path,
        release_artifact_captured_input_path:$artifact_captured_input_path,
        release_artifact_input_sha256:$artifact_sha,
        release_artifact_input_bytes:$artifact_bytes,
        release_artifact_valid:$artifact_valid,
        release_approval_valid:$approval.release_approval_state.release_approval_valid,
        signed_app_artifact_present:($artifact.artifact_evidence.signed // false),
        notarized_app_artifact_present:($artifact.artifact_evidence.notarized // false),
        stapled_app_artifact_present:($artifact.artifact_evidence.stapled // false),
        signed_notarized_stapled_artifact_present:$artifact_valid,
        local_distribution_artifact_written:($artifact.artifact_evidence.local_distribution_artifact_written // false),
        public_distribution_artifact_written:($artifact.artifact_evidence.public_distribution_artifact_written // false),
        public_upload_performed:($artifact.artifact_evidence.public_upload_performed // false),
        public_distribution_artifact_semantics:($artifact.artifact_evidence.public_distribution_artifact_semantics // "missing_release_artifact_distribution_semantics"),
        next_required_step:"post_artifact_ui_readiness_refresh"
      },
      release_artifact_source_side_effects:{
        credential_value_read:($artifact.side_effects.credential_value_read // false),
        keychain_identity_lookup_performed:($artifact.side_effects.keychain_identity_lookup_performed // false),
        network_call_performed:($artifact.side_effects.network_call_performed // false),
        notary_submission_performed:($artifact.side_effects.notary_submission_performed // false),
        app_signed:($artifact.side_effects.app_signed // false),
        app_notarized:($artifact.side_effects.app_notarized // false),
        app_stapled:($artifact.side_effects.app_stapled // false),
        local_distribution_artifact_written:($artifact.side_effects.local_distribution_artifact_written // false),
        public_distribution_artifact_written:($artifact.side_effects.public_distribution_artifact_written // false),
        public_upload_performed:($artifact.side_effects.public_upload_performed // $artifact.artifact_evidence.public_upload_performed // false),
        external_mutation:($artifact.side_effects.external_mutation // false)
      },
      source_alignment:{
        native_distribution_preflight_ready:$distribution.distribution_preflight_gate_ready,
        release_approval_intake_ready:$approval.release_approval_intake_gate_ready,
        release_approval_waiting_for_approval:$approval.release_approval_state.waiting_for_release_approval,
        release_approval_present:$approval.release_approval_state.release_approval_present,
        release_approval_valid:$approval.release_approval_state.release_approval_valid,
        release_artifact_boundary_ready:$boundary.release_artifact_boundary_gate_ready,
        release_artifact_boundary_root_report_required_count:$boundary.release_artifact_boundary.root_report_replay_required_count_after_boundary,
        release_artifact_boundary_next_required_artifact_gate:$boundary.release_artifact_boundary.next_required_artifact_gate,
        evidence_archive_ready:$archive.evidence_archive_gate_ready,
        real_backend_receipt_claim_ready:($boundary.claim_boundary.real_backend_receipt_claim_ready // false),
        backend_receipt_claim_ready:($boundary.claim_boundary.backend_receipt_claim_ready // false),
        unsigned_app_bundle_codesign_status:$boundary.release_artifact_boundary.unsigned_app_bundle_codesign_status,
        approval_only_can_make_release_claim:$approval.release_approval_state.approval_only_can_make_release_claim,
        boundary_signed_notarized_stapled_artifact_present:$boundary.release_artifact_boundary.signed_notarized_stapled_artifact_present,
        boundary_public_distribution_artifact_written:$boundary.release_artifact_boundary.public_distribution_artifact_written
      },
      release_artifact_blockers:[
        (if $approval.release_approval_state.release_approval_valid then empty else "operator_release_approval_required" end),
        (if $artifact_valid then empty else "signed_notarized_stapled_artifact_missing" end),
        (if (($artifact.artifact_evidence.local_distribution_artifact_written // false) == true and ($artifact.artifact_evidence.public_distribution_artifact_written // false) == true) then empty else "public_distribution_artifact_not_written" end),
        "post_artifact_ui_readiness_refresh_required",
        (if ($boundary.claim_boundary.real_backend_receipt_claim_ready // false) then empty else "real_backend_receipt_missing" end)
      ],
      claim_boundary:{
        local_release_artifact_intake_ready:$ready,
        release_approval_claim_ready:$approval.claim_boundary.release_approval_claim_ready,
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
        local_distribution_artifact_written:false,
        public_distribution_artifact_written:false,
        public_upload_performed:false,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        external_mutation:false
      }
    }' >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .release_artifact_intake_gate_ready == true
  and .intake_kind == "local_signed_notarized_stapled_artifact_intake_contract"
  and .intake_version == 1
  and (.template_sha256 | test("^[0-9a-f]{64}$"))
  and .template_bytes > 0
  and (.markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .markdown_bytes > 0
  and .root_report_replay_required_count_after_intake == 37
  and .release_artifact_state.next_required_step == "post_artifact_ui_readiness_refresh"
  and (.release_artifact_state.public_distribution_artifact_semantics | type) == "string"
  and (.release_artifact_source_side_effects.credential_value_read | type) == "boolean"
  and (.release_artifact_source_side_effects.keychain_identity_lookup_performed | type) == "boolean"
  and (.release_artifact_source_side_effects.network_call_performed | type) == "boolean"
  and (.release_artifact_source_side_effects.notary_submission_performed | type) == "boolean"
  and (.release_artifact_source_side_effects.app_signed | type) == "boolean"
  and (.release_artifact_source_side_effects.app_notarized | type) == "boolean"
  and (.release_artifact_source_side_effects.app_stapled | type) == "boolean"
  and (.release_artifact_source_side_effects.local_distribution_artifact_written | type) == "boolean"
  and (.release_artifact_source_side_effects.public_distribution_artifact_written | type) == "boolean"
  and (.release_artifact_source_side_effects.public_upload_performed | type) == "boolean"
  and (.release_artifact_source_side_effects.external_mutation | type) == "boolean"
  and (
    .release_artifact_state.public_distribution_artifact_semantics != "local_signed_notarized_stapled_dmg_written_not_public_upload"
    or (
      .release_artifact_source_side_effects.keychain_identity_lookup_performed == true
      and .release_artifact_source_side_effects.network_call_performed == true
      and .release_artifact_source_side_effects.notary_submission_performed == true
      and .release_artifact_source_side_effects.app_signed == true
      and .release_artifact_source_side_effects.app_notarized == true
      and .release_artifact_source_side_effects.app_stapled == true
      and .release_artifact_source_side_effects.local_distribution_artifact_written == true
      and .release_artifact_source_side_effects.public_distribution_artifact_written == true
      and .release_artifact_source_side_effects.public_upload_performed == false
      and .release_artifact_source_side_effects.external_mutation == true
    )
  )
  and .source_alignment.release_approval_intake_ready == true
  and (
    (
      .source_alignment.release_approval_present == false
      and .source_alignment.release_approval_valid == false
      and (.release_artifact_blockers | index("operator_release_approval_required") != null)
      and .claim_boundary.release_approval_claim_ready == false
    )
    or
    (
      .source_alignment.release_approval_present == true
      and .source_alignment.release_approval_valid == true
      and (.release_artifact_blockers | index("operator_release_approval_required") == null)
      and .claim_boundary.release_approval_claim_ready == true
    )
  )
  and .source_alignment.release_artifact_boundary_ready == true
  and .source_alignment.release_artifact_boundary_root_report_required_count == 36
  and .source_alignment.release_artifact_boundary_next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
  and (.source_alignment.real_backend_receipt_claim_ready | type) == "boolean"
  and (.source_alignment.backend_receipt_claim_ready | type) == "boolean"
  and .source_alignment.approval_only_can_make_release_claim == false
  and .source_alignment.boundary_signed_notarized_stapled_artifact_present == false
  and .source_alignment.boundary_public_distribution_artifact_written == false
  and (.release_artifact_blockers | index("post_artifact_ui_readiness_refresh_required") != null)
  and (
    (
      .source_alignment.real_backend_receipt_claim_ready == true
      and (.release_artifact_blockers | index("real_backend_receipt_missing") == null)
    )
    or
    (
      .source_alignment.real_backend_receipt_claim_ready == false
      and (.release_artifact_blockers | index("real_backend_receipt_missing") != null)
    )
  )
  and (
    (
      .release_artifact_state.waiting_for_release_artifact == true
      and .release_artifact_state.release_artifact_present == false
      and .release_artifact_state.release_artifact_valid == false
      and .release_artifact_state.release_approval_valid == .source_alignment.release_approval_valid
      and .release_artifact_state.signed_app_artifact_present == false
      and .release_artifact_state.notarized_app_artifact_present == false
      and .release_artifact_state.stapled_app_artifact_present == false
      and .release_artifact_state.signed_notarized_stapled_artifact_present == false
      and .release_artifact_state.local_distribution_artifact_written == false
      and .release_artifact_state.public_distribution_artifact_written == false
      and .release_artifact_state.public_upload_performed == false
      and .release_artifact_state.public_distribution_artifact_semantics == "missing_release_artifact_distribution_semantics"
      and (.release_artifact_blockers | index("signed_notarized_stapled_artifact_missing") != null)
      and (.release_artifact_blockers | index("public_distribution_artifact_not_written") != null)
    )
    or
    (
      .release_artifact_state.waiting_for_release_artifact == false
      and .release_artifact_state.release_artifact_present == true
      and .release_artifact_state.release_artifact_valid == true
      and .release_artifact_state.release_approval_valid == .source_alignment.release_approval_valid
      and .release_artifact_state.signed_app_artifact_present == true
      and .release_artifact_state.notarized_app_artifact_present == true
      and .release_artifact_state.stapled_app_artifact_present == true
      and .release_artifact_state.signed_notarized_stapled_artifact_present == true
      and .release_artifact_state.local_distribution_artifact_written == true
      and .release_artifact_state.public_distribution_artifact_written == true
      and .release_artifact_state.public_upload_performed == false
      and (
        .release_artifact_state.public_distribution_artifact_semantics == "local_signed_notarized_stapled_dmg_written_not_public_upload"
        or .release_artifact_state.public_distribution_artifact_semantics == "local_simulated_signed_notarized_stapled_dmg_written_not_public_upload"
      )
      and (.release_artifact_blockers | index("signed_notarized_stapled_artifact_missing") == null)
      and (.release_artifact_blockers | index("public_distribution_artifact_not_written") == null)
    )
  )
  and .claim_boundary.local_release_artifact_intake_ready == true
  and .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.release_execution_ready == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .claim_boundary.public_upload_performed == false
  and .side_effects.local_template_written == true
  and .side_effects.local_markdown_written == true
  and .side_effects.local_report_written == true
  and .side_effects.local_distribution_artifact_written == false
  and .side_effects.public_distribution_artifact_written == false
  and .side_effects.public_upload_performed == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

mkdir -p "$(dirname "$REPORT_PATH")"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
