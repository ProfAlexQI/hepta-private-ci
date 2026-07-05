#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_OPERATOR_BRIEFING_REPORT_PATH:-$READINESS_DIR/ui-operator-briefing-gate.json}"

PLAN_BOUNDARY_REPORT_PATH="$READINESS_DIR/ui-plan-boundary-gate.json"
DEMO_EVIDENCE_REPORT_PATH="$READINESS_DIR/ui-demo-evidence-gate.json"
EVIDENCE_ARCHIVE_REPORT_PATH="$READINESS_DIR/ui-evidence-archive-gate.json"
RELEASE_OPERATOR_DRY_RUN_REPORT_PATH="$READINESS_DIR/ui-release-operator-dry-run-gate.json"
BASE_GAP_BACKEND_HANDOFF_PATH="$READINESS_DIR/native-base-gap-backend-handoff.json"
DISTRIBUTION_PREFLIGHT_REPORT_PATH="$READINESS_DIR/native-distribution-preflight-gate.json"
PRODUCTIZATION_ROLLUP_REPORT_PATH="$READINESS_DIR/native-productization-blocker-rollup.json"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI operator briefing gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required operator-briefing input: %s\n' "$path" >&2
    exit 1
  fi
  jq empty "$path" >/dev/null
}

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

require_command jq
require_command shasum

require_report "$PLAN_BOUNDARY_REPORT_PATH"
require_report "$DEMO_EVIDENCE_REPORT_PATH"
require_report "$EVIDENCE_ARCHIVE_REPORT_PATH"
require_report "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH"
require_report "$BASE_GAP_BACKEND_HANDOFF_PATH"
require_report "$DISTRIBUTION_PREFLIGHT_REPORT_PATH"
require_report "$PRODUCTIZATION_ROLLUP_REPORT_PATH"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-operator-briefing.XXXXXX")"
REPORT_TMP="$TMP_DIR/operator-briefing-report.json"
trap 'rm -rf "$TMP_DIR"' EXIT

plan_boundary_sha="$(file_sha256 "$PLAN_BOUNDARY_REPORT_PATH")"
demo_evidence_sha="$(file_sha256 "$DEMO_EVIDENCE_REPORT_PATH")"
evidence_archive_sha="$(file_sha256 "$EVIDENCE_ARCHIVE_REPORT_PATH")"
release_operator_sha="$(file_sha256 "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH")"
handoff_sha="$(file_sha256 "$BASE_GAP_BACKEND_HANDOFF_PATH")"
distribution_sha="$(file_sha256 "$DISTRIBUTION_PREFLIGHT_REPORT_PATH")"
rollup_sha="$(file_sha256 "$PRODUCTIZATION_ROLLUP_REPORT_PATH")"

jq -n \
  --arg product "Hepta UI" \
  --arg runtime "hepta" \
  --arg gate "hepta_ui_operator_briefing_gate" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg report_path "$REPORT_PATH" \
  --arg plan_boundary_path "$PLAN_BOUNDARY_REPORT_PATH" \
  --arg demo_evidence_path "$DEMO_EVIDENCE_REPORT_PATH" \
  --arg evidence_archive_path "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --arg release_operator_path "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH" \
  --arg handoff_path "$BASE_GAP_BACKEND_HANDOFF_PATH" \
  --arg distribution_path "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --arg rollup_path "$PRODUCTIZATION_ROLLUP_REPORT_PATH" \
  --arg plan_boundary_sha "$plan_boundary_sha" \
  --arg demo_evidence_sha "$demo_evidence_sha" \
  --arg evidence_archive_sha "$evidence_archive_sha" \
  --arg release_operator_sha "$release_operator_sha" \
  --arg handoff_sha "$handoff_sha" \
  --arg distribution_sha "$distribution_sha" \
  --arg rollup_sha "$rollup_sha" \
  --slurpfile plan_boundary_file "$PLAN_BOUNDARY_REPORT_PATH" \
  --slurpfile demo_evidence_file "$DEMO_EVIDENCE_REPORT_PATH" \
  --slurpfile evidence_archive_file "$EVIDENCE_ARCHIVE_REPORT_PATH" \
  --slurpfile release_operator_file "$RELEASE_OPERATOR_DRY_RUN_REPORT_PATH" \
  --slurpfile handoff_file "$BASE_GAP_BACKEND_HANDOFF_PATH" \
  --slurpfile distribution_file "$DISTRIBUTION_PREFLIGHT_REPORT_PATH" \
  --slurpfile rollup_file "$PRODUCTIZATION_ROLLUP_REPORT_PATH" \
  '
  ($plan_boundary_file[0]) as $plan
  | ($demo_evidence_file[0]) as $demo
  | ($evidence_archive_file[0]) as $archive
  | ($release_operator_file[0]) as $release_dry_run
  | ($handoff_file[0]) as $handoff
  | ($distribution_file[0]) as $distribution
  | ($rollup_file[0]) as $rollup
  | def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def backend_priority_ids: ($handoff.items | sort_by(.priority) | map(.id));
    def release_blockers: ($plan.release_claim.blocked_by // []);
    def all_sources_ready:
      $plan.plan_boundary_gate_ready == true
      and $plan.claim_boundary.local_fixture_demo_ready == true
      and $plan.claim_boundary.live_product_claim_ready == false
      and $plan.claim_boundary.public_distribution_claim_ready == false
      and $plan.release_claim.ready == false
      and $demo.demo_evidence_gate_ready == true
      and $demo.claim_boundary.local_fixture_demo_evidence_ready == true
      and $archive.evidence_archive_gate_ready == true
      and $archive.claim_boundary.local_evidence_archive_ready == true
      and $archive.all_extracted_items_sha256_match == true
      and ($archive.archive_sha256 | test("^[0-9a-f]{64}$"))
      and $archive.archive_bytes > 0
      and $release_dry_run.release_operator_dry_run_gate_ready == true
      and $release_dry_run.claim_boundary.local_release_operator_dry_run_ready == true
      and $release_dry_run.denial_case_count == 4
      and $release_dry_run.allowed_dry_run_case_count == 1
      and $release_dry_run.claim_boundary.release_execution_ready == false
      and $release_dry_run.claim_boundary.release_claim_ready == false
      and $release_dry_run.claim_boundary.public_distribution_claim_ready == false
      and $release_dry_run.side_effects.external_mutation == false
      and $handoff.native_base_gap_backend_handoff_ready == true
      and $handoff.handoff_count == 12
      and ($handoff.items | all(.status == "partial_live_backend_contract_remaining"))
      and ($handoff.items | all(.next_owner_lane == "backend_contract"))
      and $distribution.distribution_preflight_gate_ready == true
      and $distribution.public_distribution_ready == false
      and $distribution.release_approval_required == true
      and $distribution.credential_values_read == false
      and $distribution.notary_submission_performed == false
      and $distribution.public_distribution_artifact_written == false
      and $rollup.productization_blocker_rollup_ready == true
      and $rollup.base_gap_backend_handoff.handoff_count == 12
      and sha_ready($plan_boundary_sha)
      and sha_ready($demo_evidence_sha)
      and sha_ready($evidence_archive_sha)
      and sha_ready($release_operator_sha)
      and sha_ready($handoff_sha)
      and sha_ready($distribution_sha)
      and sha_ready($rollup_sha);
    all_sources_ready as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      operator_briefing_gate_ready:$ready,
      briefing_kind:"local_ui_operator_readiness_briefing",
      briefing_version:1,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      source_reports:{
        plan_boundary:$plan_boundary_path,
        demo_evidence:$demo_evidence_path,
        evidence_archive:$evidence_archive_path,
        release_operator_dry_run:$release_operator_path,
        backend_handoff:$handoff_path,
        distribution_preflight:$distribution_path,
        productization_rollup:$rollup_path
      },
      source_report_sha256:{
        plan_boundary:$plan_boundary_sha,
        demo_evidence:$demo_evidence_sha,
        evidence_archive:$evidence_archive_sha,
        release_operator_dry_run:$release_operator_sha,
        backend_handoff:$handoff_sha,
        distribution_preflight:$distribution_sha,
        productization_rollup:$rollup_sha
      },
      current_position:{
        local_fixture_demo_ready:$plan.claim_boundary.local_fixture_demo_ready,
        r33_minimum_hard_demo_ready:$plan.claim_boundary.r33_minimum_hard_demo_ready,
        local_fixture_demo_evidence_ready:$demo.claim_boundary.local_fixture_demo_evidence_ready,
        local_evidence_archive_ready:$archive.claim_boundary.local_evidence_archive_ready,
        local_release_operator_dry_run_ready:$release_dry_run.claim_boundary.local_release_operator_dry_run_ready,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false
      },
      evidence_pointer:{
        archive_path:$archive.archive_path,
        archive_sha256:$archive.archive_sha256,
        archive_bytes:$archive.archive_bytes,
        archive_file_count:$archive.extracted_archive_file_count,
        evidence_bundle_file_count:$archive.extracted_bundle_file_count,
        dry_run_manifest_path:$release_dry_run.dry_run_manifest_path,
        dry_run_manifest_sha256:$release_dry_run.dry_run_manifest_sha256,
        dry_run_manifest_bytes:$release_dry_run.dry_run_manifest_bytes,
        required_report_count:$demo.report_evidence.required_report_count,
        required_screenshot_count:$demo.screenshot_evidence.required_screenshot_count,
        key_screenshot_count:$demo.screenshot_evidence.key_screenshot_count
      },
      critical_risks:[
        {
          id:"live_backend_contracts_remaining",
          severity:"blocker",
          owner_lane:"backend_contract",
          remaining_count:$plan.live_product_claim.remaining_backend_contract_count,
          first_five:(backend_priority_ids | .[0:5])
        },
        {
          id:"release_public_distribution_not_approved",
          severity:"blocker",
          owner_lane:"release_operator",
          blockers:release_blockers,
          dry_run_denial_case_count:$release_dry_run.denial_case_count
        },
        {
          id:"hard_true_window_required_for_public_demo",
          severity:"guardrail",
          owner_lane:"hepta-ui",
          r33_minimum_hard_demo_ready:$plan.claim_boundary.r33_minimum_hard_demo_ready,
          hard_true_window_required:$demo.claim_boundary.hard_true_window_required
        }
      ],
      critical_risk_count:3,
      backend_priority_ids:backend_priority_ids,
      backend_remaining_contract_count:$plan.live_product_claim.remaining_backend_contract_count,
      next_plan:$plan.next_plan,
      next_plan_count:($plan.next_plan | length),
      answer_guardrail:{
        allowed_summary:"local UI fixture, packaging, evidence archive, and release dry-run readiness only",
        forbidden_claims:["live_product_ready","public_distribution_ready","release_ready"],
        required_closeout_fields:["artifact_path","archive_sha256","backend_remaining_contract_count","release_claim_ready_false"]
      },
      claim_boundary:{
        local_operator_briefing_ready:$ready,
        local_fixture_demo_ready:$plan.claim_boundary.local_fixture_demo_ready,
        local_evidence_archive_ready:$archive.claim_boundary.local_evidence_archive_ready,
        local_release_operator_dry_run_ready:$release_dry_run.claim_boundary.local_release_operator_dry_run_ready,
        live_product_claim_ready:false,
        public_distribution_claim_ready:false,
        release_claim_ready:false,
        external_actions_allowed:false,
        public_upload_performed:false,
        signing_notarization_performed:false
      },
      side_effects:{
        filesystem_read:true,
        local_report_written:true,
        matrix_login:false,
        gateway_call:false,
        provider_invoked:false,
        channel_delivery:false,
        external_mutation:false
      }
    }' >"$REPORT_TMP"

jq -e '
  .status == "ready"
  and .operator_briefing_gate_ready == true
  and .briefing_kind == "local_ui_operator_readiness_briefing"
  and .briefing_version == 1
  and (.source_report_sha256.plan_boundary | test("^[0-9a-f]{64}$"))
  and (.source_report_sha256.evidence_archive | test("^[0-9a-f]{64}$"))
  and (.source_report_sha256.release_operator_dry_run | test("^[0-9a-f]{64}$"))
  and .current_position.local_fixture_demo_ready == true
  and .current_position.local_evidence_archive_ready == true
  and .current_position.local_release_operator_dry_run_ready == true
  and .current_position.live_product_claim_ready == false
  and .current_position.public_distribution_claim_ready == false
  and .current_position.release_claim_ready == false
  and .evidence_pointer.archive_bytes > 0
  and (.evidence_pointer.archive_sha256 | test("^[0-9a-f]{64}$"))
  and (.evidence_pointer.dry_run_manifest_sha256 | test("^[0-9a-f]{64}$"))
  and .critical_risk_count == 3
  and (.critical_risks | map(.id) | index("live_backend_contracts_remaining") != null)
  and (.critical_risks | map(.id) | index("release_public_distribution_not_approved") != null)
  and (.critical_risks | map(.id) | index("hard_true_window_required_for_public_demo") != null)
  and .backend_remaining_contract_count == 12
  and (.backend_priority_ids | length) == 12
  and .backend_priority_ids[0] == "message_search"
  and .backend_priority_ids[1] == "file_upload_send"
  and .backend_priority_ids[2] == "media_download_playback"
  and .next_plan_count == 3
  and .next_plan[0].id == "minimum_ui_demo_gate"
  and .next_plan[1].id == "backend_contract_promotion"
  and .next_plan[2].id == "release_artifact_gate"
  and (.answer_guardrail.forbidden_claims | index("live_product_ready") != null)
  and (.answer_guardrail.forbidden_claims | index("public_distribution_ready") != null)
  and (.answer_guardrail.forbidden_claims | index("release_ready") != null)
  and .claim_boundary.local_operator_briefing_ready == true
  and .claim_boundary.live_product_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .claim_boundary.public_upload_performed == false
  and .claim_boundary.signing_notarization_performed == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

mkdir -p "$(dirname "$REPORT_PATH")"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
