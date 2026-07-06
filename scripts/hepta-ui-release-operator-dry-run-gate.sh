#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_RELEASE_OPERATOR_DRY_RUN_REPORT_PATH:-$READINESS_DIR/ui-release-operator-dry-run-gate.json}"
DRY_RUN_DIR="${HEPTA_UI_RELEASE_OPERATOR_DRY_RUN_DIR:-$READINESS_DIR/release-operator-dry-run}"
DRY_RUN_MANIFEST_PATH="${HEPTA_UI_RELEASE_OPERATOR_DRY_RUN_MANIFEST_PATH:-$DRY_RUN_DIR/release-operator-dry-run-manifest.json}"

PACKAGING_REPORT_PATH="$READINESS_DIR/native-packaging-gate.json"
DISTRIBUTION_PREFLIGHT_REPORT_PATH="$READINESS_DIR/native-distribution-preflight-gate.json"
PLAN_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-plan-boundary-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI release-operator dry-run gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required release-operator dry-run input: %s\n' "$path" >&2
    exit 1
  fi
  jq empty "$path" >/dev/null
}

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_command jq
require_command shasum

require_report "$PACKAGING_REPORT_PATH"
require_report "$DISTRIBUTION_PREFLIGHT_REPORT_PATH"
require_report "$PLAN_BOUNDARY_REPORT_PATH"
require_report "$EVIDENCE_ARCHIVE_REPORT_PATH"

mkdir -p "$DRY_RUN_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-release-operator-dry-run.XXXXXX")"
MANIFEST_TMP="$TMP_DIR/release-operator-dry-run-manifest.json"
REPORT_TMP="$TMP_DIR/release-operator-dry-run-report.json"
trap 'rm -rf "$TMP_DIR"' EXIT

packaging_sha="$(file_sha256 "$PACKAGING_REPORT_PATH")"
distribution_sha="$(file_sha256 "$DISTRIBUTION_PREFLIGHT_REPORT_PATH")"
plan_boundary_sha="$(file_sha256 "$PLAN_BOUNDARY_REPORT_PATH")"
evidence_archive_report_sha="$(file_sha256 "$EVIDENCE_ARCHIVE_REPORT_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_release_operator_dry_run_manifest" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg manifest_path "$DRY_RUN_MANIFEST_PATH" \
  --arg packaging_report_path "$PACKAGING_REPORT_PATH" \
  --arg distribution_preflight_report_path "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --arg plan_boundary_report_path "$PLAN_BOUNDARY_REPORT_PATH" \
  --arg evidence_archive_report_path "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --arg packaging_sha "$packaging_sha" \
  --arg distribution_sha "$distribution_sha" \
  --arg plan_boundary_sha "$plan_boundary_sha" \
  --arg evidence_archive_report_sha "$evidence_archive_report_sha" \
  --slurpfile packaging_file "$PACKAGING_REPORT_PATH" \
  --slurpfile distribution_file "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --slurpfile plan_boundary_file "$PLAN_BOUNDARY_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  '
  ($packaging_file[0]) as $packaging
  | ($distribution_file[0]) as $distribution
  | ($plan_boundary_file[0]) as $plan
  | ($evidence_archive_file[0]) as $archive
  | def source_ready:
      $packaging.local_packaging_gate_ready == true
      and $packaging.local_unsigned_app_bundle_probe_ready == true
      and $packaging.local_unsigned_app_bundle.ready == true
      and $packaging.local_unsigned_app_bundle.codesign_status == "unsigned_expected"
      and $packaging.local_unsigned_app_bundle.distribution_signed == false
      and $packaging.local_unsigned_app_bundle.public_distribution_artifact_written == false
      and $distribution.distribution_preflight_gate_ready == true
      and $distribution.distribution_static_contract_ready == true
      and $distribution.unsigned_app_bundle_probe.ready == true
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
      and $plan.plan_boundary_gate_ready == true
      and $plan.claim_boundary.local_fixture_demo_ready == true
      and $plan.claim_boundary.live_product_claim_ready == false
      and $plan.claim_boundary.public_distribution_claim_ready == false
      and $plan.release_claim.ready == false
      and ($plan.release_claim.blocked_by | index("operator_release_approval_required") != null)
      and ($plan.release_claim.blocked_by | index("apple_credentials_not_read") != null)
      and ($plan.release_claim.blocked_by | index("notary_submission_not_performed") != null)
      and ($plan.release_claim.blocked_by | index("public_distribution_artifact_not_written") != null)
      and $archive.evidence_archive_gate_ready == true
      and $archive.claim_boundary.local_evidence_archive_ready == true
      and ($archive.archive_sha256 | test("^[0-9a-f]{64}$"))
      and $archive.archive_bytes > 0
      and $archive.claim_boundary.release_claim_ready == false;
  def denial_matrix: [
      {
        id:"missing_operator_release_approval",
        request:"execute_signed_notarized_public_release",
        allowed:false,
        denied:true,
        reason:"operator_release_approval_required",
        operator_approval_recorded:false,
        release_artifact_written:false
      },
      {
        id:"backend_contracts_not_promoted",
        request:"claim_live_product_or_ga_release",
        allowed:false,
        denied:true,
        reason:"partial_live_backend_contract_remaining",
        remaining_backend_contract_count:$plan.live_product_claim.remaining_backend_contract_count,
        release_artifact_written:false
      },
      {
        id:"credentials_and_notary_not_performed",
        request:"sign_notarize_or_staple",
        allowed:false,
        denied:true,
        reason:"apple_credentials_not_read_and_notary_submission_not_performed",
        credential_values_read:$distribution.credential_values_read,
        notary_submission_performed:$distribution.notary_submission_performed,
        release_artifact_written:false
      },
      {
        id:"public_distribution_artifact_write_attempt",
        request:"write_public_distribution_artifact",
        allowed:false,
        denied:true,
        reason:"public_distribution_artifact_not_written",
        public_distribution_artifact_written:false
      },
      {
        id:"local_dry_run_manifest_only",
        request:"write_local_release_operator_dry_run_manifest",
        allowed:true,
        denied:false,
        reason:"local_non_mutating_ui_lane_evidence",
        release_artifact_written:false
      }
    ];
  (source_ready and (denial_matrix | length) == 5 and (denial_matrix | map(select(.allowed == false and .denied == true)) | length) == 4) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      manifest_kind:"release_operator_dry_run",
      status:(if $ready then "ready" else "failed" end),
      release_operator_dry_run_manifest_ready:$ready,
      release_operator_dry_run_mode:"local_manifest_denial_matrix_only",
      readiness_dir:$readiness_dir,
      manifest_path:$manifest_path,
      source_reports:{
        packaging:$packaging_report_path,
        distribution_preflight:$distribution_preflight_report_path,
        plan_boundary:$plan_boundary_report_path,
        evidence_archive:$evidence_archive_report_path
      },
      source_report_sha256:{
        packaging:$packaging_sha,
        distribution_preflight:$distribution_sha,
        plan_boundary:$plan_boundary_sha,
        evidence_archive:$evidence_archive_report_sha
      },
      release_candidate:{
        unsigned_app_bundle_path:$packaging.local_unsigned_app_bundle.app_bundle_path,
        unsigned_app_bundle_sha256:$packaging.local_unsigned_app_bundle.binary_sha256,
        unsigned_app_bundle_bytes:$packaging.local_unsigned_app_bundle.bundle_bytes,
        unsigned_app_bundle_codesign_status:$packaging.local_unsigned_app_bundle.codesign_status,
        bundle_identifier:$packaging.local_unsigned_app_bundle.bundle_identifier,
        bundle_executable:$packaging.local_unsigned_app_bundle.bundle_executable,
        bundle_name:$packaging.local_unsigned_app_bundle.bundle_name,
        evidence_archive_path:$archive.archive_path,
        evidence_archive_sha256:$archive.archive_sha256,
        evidence_archive_bytes:$archive.archive_bytes,
        public_distribution_artifacts:[]
      },
      operator_packet:{
        requested_release_action:"signed_notarized_stapled_public_distribution",
        dry_run_only:true,
        operator_approval_recorded:false,
        operator_identity_hash_recorded:false,
        approval_scope:"none",
        approval_required_for_release:true,
        credential_values_read:false,
        keychain_identity_lookup_performed:false,
        network_call_performed:false,
        notary_submission_performed:false,
        signing_notarization_performed:false,
        public_distribution_artifact_written:false,
        allowed_actions:["read_local_source_reports","write_local_dry_run_manifest","write_local_dry_run_report"],
        denied_actions:["read_apple_credentials","query_keychain_identity","execute_codesign","submit_notarytool","staple_artifact","write_public_distribution_artifact","make_release_or_ga_claim"]
      },
      denial_matrix:denial_matrix,
      release_claim_boundary:{
        local_release_operator_dry_run_ready:$ready,
        release_execution_ready:false,
        public_distribution_ready:false,
        live_product_claim_ready:false,
        release_claim_ready:false,
        public_distribution_claim_ready:false,
        external_actions_allowed:false,
        backend_live_wiring_claim_allowed:false,
        blocked_by:$plan.release_claim.blocked_by,
        next_owner_lane:"release_operator"
      },
      next_actions:[
        {
          priority:1,
          id:"backend_contract_promotion_first",
          owner_lane:"backend_contract",
          action:"promote remaining backend contracts before live-product or GA release claims",
          remaining_backend_contract_count:$plan.live_product_claim.remaining_backend_contract_count
        },
        {
          priority:2,
          id:"operator_release_approval_packet",
          owner_lane:"release_operator",
          action:"provide explicit operator approval, selected output path, Apple credential handling, signed/notarized/stapled artifact evidence, and public artifact policy before release execution"
        }
      ],
      side_effects:{
        filesystem_read:true,
        local_dry_run_manifest_written:true,
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
    }' >"$MANIFEST_TMP"

jq -e '
  .status == "ready"
  and .release_operator_dry_run_manifest_ready == true
  and .release_operator_dry_run_mode == "local_manifest_denial_matrix_only"
  and .operator_packet.dry_run_only == true
  and .operator_packet.operator_approval_recorded == false
  and .operator_packet.credential_values_read == false
  and .operator_packet.notary_submission_performed == false
  and .operator_packet.public_distribution_artifact_written == false
  and (.denial_matrix | length) == 5
  and (.denial_matrix | map(select(.allowed == false and .denied == true)) | length) == 4
  and .release_claim_boundary.release_execution_ready == false
  and .release_claim_boundary.release_claim_ready == false
  and .release_claim_boundary.public_distribution_claim_ready == false
  and (.release_claim_boundary.blocked_by | index("operator_release_approval_required") != null)
  and (.release_claim_boundary.blocked_by | index("public_distribution_artifact_not_written") != null)
  and .side_effects.external_mutation == false
' "$MANIFEST_TMP" >/dev/null

cp "$MANIFEST_TMP" "$DRY_RUN_MANIFEST_PATH"
manifest_sha="$(file_sha256 "$DRY_RUN_MANIFEST_PATH")"
manifest_bytes="$(wc -c <"$DRY_RUN_MANIFEST_PATH" | tr -d ' ')"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_release_operator_dry_run_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg dry_run_dir "$DRY_RUN_DIR" \
  --arg dry_run_manifest_path "$DRY_RUN_MANIFEST_PATH" \
  --arg dry_run_manifest_sha "$manifest_sha" \
  --argjson dry_run_manifest_bytes "$manifest_bytes" \
  --slurpfile manifest_file "$DRY_RUN_MANIFEST_PATH" \
  '
  ($manifest_file[0]) as $manifest
  | (
      $manifest.release_operator_dry_run_manifest_ready == true
      and ($dry_run_manifest_sha | test("^[0-9a-f]{64}$"))
      and $dry_run_manifest_bytes > 0
      and $manifest.operator_packet.dry_run_only == true
      and $manifest.operator_packet.operator_approval_recorded == false
      and $manifest.operator_packet.credential_values_read == false
      and $manifest.operator_packet.notary_submission_performed == false
      and $manifest.operator_packet.public_distribution_artifact_written == false
      and ($manifest.denial_matrix | length) == 5
      and ($manifest.denial_matrix | map(select(.allowed == false and .denied == true)) | length) == 4
      and $manifest.release_claim_boundary.release_execution_ready == false
      and $manifest.release_claim_boundary.release_claim_ready == false
      and $manifest.release_claim_boundary.public_distribution_claim_ready == false
      and $manifest.side_effects.external_mutation == false
    ) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      release_operator_dry_run_gate_ready:$ready,
      release_operator_dry_run_mode:$manifest.release_operator_dry_run_mode,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      dry_run_dir:$dry_run_dir,
      dry_run_manifest_path:$dry_run_manifest_path,
      dry_run_manifest_sha256:$dry_run_manifest_sha,
      dry_run_manifest_bytes:$dry_run_manifest_bytes,
      dry_run_manifest:$manifest,
      source_reports:$manifest.source_reports,
      source_report_sha256:$manifest.source_report_sha256,
      release_candidate:$manifest.release_candidate,
      operator_packet:$manifest.operator_packet,
      denial_matrix:$manifest.denial_matrix,
      denial_case_count:($manifest.denial_matrix | map(select(.allowed == false and .denied == true)) | length),
      allowed_dry_run_case_count:($manifest.denial_matrix | map(select(.allowed == true and .denied == false)) | length),
      claim_boundary:$manifest.release_claim_boundary,
      next_actions:$manifest.next_actions,
      side_effects:$manifest.side_effects
    }' >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .release_operator_dry_run_gate_ready == true
  and .release_operator_dry_run_mode == "local_manifest_denial_matrix_only"
  and (.dry_run_manifest_sha256 | test("^[0-9a-f]{64}$"))
  and .dry_run_manifest_bytes > 0
  and .dry_run_manifest.release_operator_dry_run_manifest_ready == true
  and .operator_packet.dry_run_only == true
  and .operator_packet.operator_approval_recorded == false
  and .operator_packet.credential_values_read == false
  and .operator_packet.notary_submission_performed == false
  and .operator_packet.public_distribution_artifact_written == false
  and .denial_case_count == 4
  and .allowed_dry_run_case_count == 1
  and .claim_boundary.local_release_operator_dry_run_ready == true
  and .claim_boundary.release_execution_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and (.claim_boundary.blocked_by | index("operator_release_approval_required") != null)
  and (.claim_boundary.blocked_by | index("apple_credentials_not_read") != null)
  and (.claim_boundary.blocked_by | index("notary_submission_not_performed") != null)
  and (.claim_boundary.blocked_by | index("public_distribution_artifact_not_written") != null)
  and .side_effects.local_dry_run_manifest_written == true
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
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
