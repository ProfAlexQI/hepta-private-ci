#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_RELEASE_ARTIFACT_BOUNDARY_REPORT_PATH:-$READINESS_DIR/ui-release-artifact-boundary-gate.json}"
BOUNDARY_DIR="${HEPTA_UI_RELEASE_ARTIFACT_BOUNDARY_DIR:-$READINESS_DIR/release-artifact-boundary}"
BOUNDARY_MARKDOWN_PATH="$BOUNDARY_DIR/release-artifact-boundary.md"

PACKAGING_REPORT_PATH="$READINESS_DIR/native-packaging-gate.json"
DISTRIBUTION_PREFLIGHT_REPORT_PATH="$READINESS_DIR/native-distribution-preflight-gate.json"
RELEASE_OPERATOR_DRY_RUN_REPORT_PATH="$READINESS_DIR/ui-release-operator-dry-run-gate.json"
RELEASE_APPROVAL_INTAKE_REPORT_PATH="$READINESS_DIR/ui-release-approval-intake-gate.json"
TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH="$READINESS_DIR/ui-top-design-referee-refresh-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI release artifact boundary gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required release artifact boundary input: %s\n' "$path" >&2
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

require_report "$PACKAGING_REPORT_PATH"
require_report "$DISTRIBUTION_PREFLIGHT_REPORT_PATH"
require_report "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH"
require_report "$RELEASE_APPROVAL_INTAKE_REPORT_PATH"
require_report "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH"
require_report "$EVIDENCE_ARCHIVE_REPORT_PATH"

rm -rf "$BOUNDARY_DIR"
mkdir -p "$BOUNDARY_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-release-artifact-boundary.XXXXXX")"
REPORT_DRAFT="$TMP_DIR/release-artifact-boundary-draft.json"
REPORT_TMP="$TMP_DIR/release-artifact-boundary-report.json"
MARKDOWN_TMP="$TMP_DIR/release-artifact-boundary.md"
trap 'rm -rf "$TMP_DIR"' EXIT

packaging_sha="$(file_sha256 "$PACKAGING_REPORT_PATH")"
distribution_sha="$(file_sha256 "$DISTRIBUTION_PREFLIGHT_REPORT_PATH")"
release_dry_run_sha="$(file_sha256 "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH")"
release_approval_sha="$(file_sha256 "$RELEASE_APPROVAL_INTAKE_REPORT_PATH")"
top_design_sha="$(file_sha256 "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH")"
evidence_archive_report_sha="$(file_sha256 "$EVIDENCE_ARCHIVE_REPORT_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_release_artifact_boundary_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg boundary_dir "$BOUNDARY_DIR" \
  --arg boundary_markdown_path "$BOUNDARY_MARKDOWN_PATH" \
  --arg packaging_path "$PACKAGING_REPORT_PATH" \
  --arg distribution_path "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --arg release_dry_run_path "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH" \
  --arg release_approval_path "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" \
  --arg top_design_path "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH" \
  --arg evidence_archive_path "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --arg packaging_sha "$packaging_sha" \
  --arg distribution_sha "$distribution_sha" \
  --arg release_dry_run_sha "$release_dry_run_sha" \
  --arg release_approval_sha "$release_approval_sha" \
  --arg top_design_sha "$top_design_sha" \
  --arg evidence_archive_report_sha "$evidence_archive_report_sha" \
  --slurpfile packaging_file "$PACKAGING_REPORT_PATH" \
  --slurpfile distribution_file "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --slurpfile release_dry_run_file "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH" \
  --slurpfile release_approval_file "$RELEASE_APPROVAL_INTAKE_REPORT_PATH" \
  --slurpfile top_design_file "$TOP_DESIGN_REFEREE_REFRESH_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  '
  ($packaging_file[0]) as $packaging
  | ($distribution_file[0]) as $distribution
  | ($release_dry_run_file[0]) as $dry
  | ($release_approval_file[0]) as $approval
  | ($top_design_file[0]) as $top_design
  | ($evidence_archive_file[0]) as $archive
  | def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def source_chain_ready:
      $packaging.local_packaging_gate_ready == true
      and $packaging.local_unsigned_app_bundle_probe_ready == true
      and $packaging.local_unsigned_app_bundle.ready == true
      and $packaging.local_unsigned_app_bundle.codesign_status == "unsigned_expected"
      and $packaging.local_unsigned_app_bundle.distribution_signed == false
      and $packaging.local_unsigned_app_bundle.distribution_notarized == false
      and $packaging.local_unsigned_app_bundle.distribution_stapled == false
      and $packaging.local_unsigned_app_bundle.public_distribution_artifact_written == false
      and $distribution.distribution_preflight_gate_ready == true
      and $distribution.distribution_static_contract_ready == true
      and $distribution.unsigned_app_bundle_probe.ready == true
      and $distribution.unsigned_app_bundle_probe.codesign_status == "unsigned_expected"
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
      and $dry.operator_packet.signing_notarization_performed == false
      and $dry.operator_packet.public_distribution_artifact_written == false
      and $dry.claim_boundary.release_execution_ready == false
      and $dry.claim_boundary.public_distribution_claim_ready == false
      and $dry.claim_boundary.release_claim_ready == false
      and $approval.release_approval_intake_gate_ready == true
      and $approval.release_approval_state.approval_only_can_make_release_claim == false
      and $approval.release_approval_state.signed_notarized_stapled_artifact_present == false
      and $approval.release_approval_state.public_distribution_artifact_written == false
      and $approval.claim_boundary.release_execution_ready == false
      and $approval.claim_boundary.public_distribution_claim_ready == false
      and $approval.claim_boundary.release_claim_ready == false
      and $top_design.top_design_referee_refresh_gate_ready == true
      and $top_design.claim_boundary.desktop_mobile_design_claim_ready == true
      and $top_design.claim_boundary.live_product_claim_ready == false
      and $top_design.claim_boundary.public_distribution_claim_ready == false
      and $top_design.claim_boundary.release_claim_ready == false
      and $archive.evidence_archive_gate_ready == true
      and $archive.claim_boundary.local_evidence_archive_ready == true
      and $archive.claim_boundary.public_distribution_claim_ready == false
      and $archive.claim_boundary.release_claim_ready == false
      and sha_ready($packaging_sha)
      and sha_ready($distribution_sha)
      and sha_ready($release_dry_run_sha)
      and sha_ready($release_approval_sha)
      and sha_ready($top_design_sha)
      and sha_ready($evidence_archive_report_sha);
    source_chain_ready as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      release_artifact_boundary_gate_ready:$ready,
      boundary_kind:"local_release_artifact_boundary_lock",
      boundary_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      boundary_dir:$boundary_dir,
      boundary_markdown_path:$boundary_markdown_path,
      source_reports:{
        native_packaging:$packaging_path,
        native_distribution_preflight:$distribution_path,
        release_operator_dry_run:$release_dry_run_path,
        release_approval_intake:$release_approval_path,
        top_design_referee_refresh:$top_design_path,
        evidence_archive:$evidence_archive_path
      },
      source_report_sha256:{
        native_packaging:$packaging_sha,
        native_distribution_preflight:$distribution_sha,
        release_operator_dry_run:$release_dry_run_sha,
        release_approval_intake:$release_approval_sha,
        top_design_referee_refresh:$top_design_sha,
        evidence_archive:$evidence_archive_report_sha
      },
      release_artifact_boundary:{
        unsigned_app_bundle_probe_ready:$packaging.local_unsigned_app_bundle_probe_ready,
        unsigned_app_bundle_path:$packaging.local_unsigned_app_bundle.app_bundle_path,
        unsigned_app_bundle_sha256:$packaging.local_unsigned_app_bundle.binary_sha256,
        unsigned_app_bundle_codesign_status:$packaging.local_unsigned_app_bundle.codesign_status,
        release_approval_waiting_for_approval:$approval.release_approval_state.waiting_for_release_approval,
        release_approval_present:$approval.release_approval_state.release_approval_present,
        release_approval_valid:$approval.release_approval_state.release_approval_valid,
        approval_only_can_make_release_claim:false,
        signed_app_artifact_present:false,
        notarized_app_artifact_present:false,
        stapled_app_artifact_present:false,
        signed_notarized_stapled_artifact_present:false,
        public_distribution_artifact_present:false,
        public_distribution_artifact_written:false,
        next_required_artifact_gate:"signed_notarized_stapled_artifact_gate",
        root_report_replay_required_count_after_boundary:36
      },
      source_alignment:{
        native_packaging_ready:$packaging.local_packaging_gate_ready,
        native_distribution_preflight_ready:$distribution.distribution_preflight_gate_ready,
        release_operator_dry_run_ready:$dry.release_operator_dry_run_gate_ready,
        release_approval_intake_ready:$approval.release_approval_intake_gate_ready,
        top_design_referee_refresh_ready:$top_design.top_design_referee_refresh_gate_ready,
        evidence_archive_ready:$archive.evidence_archive_gate_ready,
        desktop_mobile_design_claim_ready:$top_design.claim_boundary.desktop_mobile_design_claim_ready,
        real_backend_receipt_claim_ready:$top_design.claim_boundary.real_backend_receipt_claim_ready,
        backend_receipt_claim_ready:$top_design.claim_boundary.backend_receipt_claim_ready,
        approval_only_can_make_release_claim:$approval.release_approval_state.approval_only_can_make_release_claim,
        signed_notarized_stapled_artifact_present:$approval.release_approval_state.signed_notarized_stapled_artifact_present,
        public_distribution_artifact_written:$approval.release_approval_state.public_distribution_artifact_written,
        root_report_replay_required_count_after_top_design_refresh:$top_design.current_state.root_report_replay_required_count_after_top_design_refresh,
        root_report_replay_required_count_after_boundary:36
      },
      release_blockers:[
        (if $approval.release_approval_state.release_approval_valid then empty else "operator_release_approval_required" end),
        "signed_notarized_stapled_artifact_missing",
        "public_distribution_artifact_not_written",
        (if $top_design.claim_boundary.real_backend_receipt_claim_ready then empty else "real_backend_receipt_missing" end),
        (if $top_design.claim_boundary.backend_receipt_claim_ready then empty else "backend_contract_first_five_not_executed" end)
      ],
      claim_boundary:{
        local_release_artifact_boundary_ready:$ready,
        desktop_mobile_design_claim_ready:$top_design.claim_boundary.desktop_mobile_design_claim_ready,
        real_backend_receipt_claim_ready:$top_design.claim_boundary.real_backend_receipt_claim_ready,
        backend_receipt_claim_ready:$top_design.claim_boundary.backend_receipt_claim_ready,
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
    }' >"$REPORT_DRAFT"

jq -r '
  "# Hepta UI Release Artifact Boundary\n\n"
  + "- Kind: \(.boundary_kind)\n"
  + "- Status: \(.status)\n"
  + "- Unsigned app probe: \(.release_artifact_boundary.unsigned_app_bundle_codesign_status)\n"
  + "- Next required artifact gate: \(.release_artifact_boundary.next_required_artifact_gate)\n"
  + "- Root replay after this boundary: \(.release_artifact_boundary.root_report_replay_required_count_after_boundary)/\(.release_artifact_boundary.root_report_replay_required_count_after_boundary)\n\n"
  + "## Boundary\n\n"
  + "- Approval alone cannot make a release claim.\n"
  + "- Signed, notarized, and stapled artifact evidence is absent.\n"
  + "- Public distribution artifact writing is absent.\n"
  + "- Live-product, public-distribution, and release claims remain false.\n"
' "$REPORT_DRAFT" >"$MARKDOWN_TMP"

markdown_sha="$(file_sha256 "$MARKDOWN_TMP")"
markdown_bytes="$(file_bytes "$MARKDOWN_TMP")"

jq \
  --arg markdown_sha "$markdown_sha" \
  --argjson markdown_bytes "$markdown_bytes" \
  '. + {
    boundary_markdown_sha256:$markdown_sha,
    boundary_markdown_bytes:$markdown_bytes
  }' "$REPORT_DRAFT" >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .release_artifact_boundary_gate_ready == true
  and .boundary_kind == "local_release_artifact_boundary_lock"
  and .boundary_version == 1
  and (.boundary_markdown_sha256 | test("^[0-9a-f]{64}$"))
  and .boundary_markdown_bytes > 0
  and .release_artifact_boundary.unsigned_app_bundle_probe_ready == true
  and .release_artifact_boundary.unsigned_app_bundle_codesign_status == "unsigned_expected"
  and (
    (
      .release_artifact_boundary.release_approval_waiting_for_approval == true
      and .release_artifact_boundary.release_approval_present == false
      and .release_artifact_boundary.release_approval_valid == false
      and (.release_blockers | index("operator_release_approval_required") != null)
      and .claim_boundary.release_approval_claim_ready == false
    )
    or (
      .release_artifact_boundary.release_approval_waiting_for_approval == false
      and .release_artifact_boundary.release_approval_present == true
      and .release_artifact_boundary.release_approval_valid == true
      and (.release_blockers | index("operator_release_approval_required") == null)
      and .claim_boundary.release_approval_claim_ready == true
    )
  )
  and .release_artifact_boundary.approval_only_can_make_release_claim == false
  and .release_artifact_boundary.signed_app_artifact_present == false
  and .release_artifact_boundary.notarized_app_artifact_present == false
  and .release_artifact_boundary.stapled_app_artifact_present == false
  and .release_artifact_boundary.signed_notarized_stapled_artifact_present == false
  and .release_artifact_boundary.public_distribution_artifact_present == false
  and .release_artifact_boundary.public_distribution_artifact_written == false
  and .release_artifact_boundary.next_required_artifact_gate == "signed_notarized_stapled_artifact_gate"
  and .release_artifact_boundary.root_report_replay_required_count_after_boundary == 36
  and .source_alignment.native_packaging_ready == true
  and .source_alignment.native_distribution_preflight_ready == true
  and .source_alignment.release_operator_dry_run_ready == true
  and .source_alignment.release_approval_intake_ready == true
  and .source_alignment.top_design_referee_refresh_ready == true
  and .source_alignment.evidence_archive_ready == true
  and .source_alignment.desktop_mobile_design_claim_ready == true
  and (.source_alignment.real_backend_receipt_claim_ready | type) == "boolean"
  and (.source_alignment.backend_receipt_claim_ready | type) == "boolean"
  and .source_alignment.approval_only_can_make_release_claim == false
  and .source_alignment.signed_notarized_stapled_artifact_present == false
  and .source_alignment.public_distribution_artifact_written == false
  and .source_alignment.root_report_replay_required_count_after_top_design_refresh == 35
  and .source_alignment.root_report_replay_required_count_after_boundary == 36
  and (.release_blockers | index("signed_notarized_stapled_artifact_missing") != null)
  and (.release_blockers | index("public_distribution_artifact_not_written") != null)
  and (
    (
      .source_alignment.real_backend_receipt_claim_ready == true
      and (.release_blockers | index("real_backend_receipt_missing") == null)
      and (.release_blockers | index("backend_contract_first_five_not_executed") == null)
    )
    or
    (
      .source_alignment.real_backend_receipt_claim_ready == false
      and (.release_blockers | index("real_backend_receipt_missing") != null)
    )
  )
  and .claim_boundary.local_release_artifact_boundary_ready == true
  and .claim_boundary.desktop_mobile_design_claim_ready == true
  and .claim_boundary.real_backend_receipt_claim_ready == .source_alignment.real_backend_receipt_claim_ready
  and .claim_boundary.backend_receipt_claim_ready == .source_alignment.backend_receipt_claim_ready
  and .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.release_execution_ready == false
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .claim_boundary.public_upload_performed == false
  and .claim_boundary.signing_notarization_performed == false
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

mkdir -p "$(dirname "$REPORT_PATH")"
cp "$MARKDOWN_TMP" "$BOUNDARY_MARKDOWN_PATH"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
