#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615}"
REPORT_PATH="${HEPTA_UI_RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH:-$READINESS_DIR/ui-release-artifact-roundtrip-gate.json}"
ROUNDTRIP_DIR="${HEPTA_UI_RELEASE_ARTIFACT_ROUNDTRIP_DIR:-$READINESS_DIR/release-artifact-roundtrip}"
SIMULATED_ARTIFACT_PATH="$ROUNDTRIP_DIR/simulated-signed-artifact.json"
SIMULATED_WAITING_INTAKE_DIR="$ROUNDTRIP_DIR/simulated-waiting-intake"
SIMULATED_WAITING_INTAKE_REPORT_PATH="$ROUNDTRIP_DIR/ui-release-artifact-intake-waiting-gate.json"
SIMULATED_INTAKE_DIR="$ROUNDTRIP_DIR/simulated-intake"
SIMULATED_INTAKE_REPORT_PATH="$ROUNDTRIP_DIR/ui-release-artifact-intake-present-gate.json"

RELEASE_ARTIFACT_INTAKE_REPORT_PATH="$READINESS_DIR/ui-release-artifact-intake-gate.json"
RELEASE_ARTIFACT_TEMPLATE_PATH="$READINESS_DIR/release-artifact-intake/release-artifact-template.json"
RELEASE_ARTIFACT_MARKDOWN_PATH="$READINESS_DIR/release-artifact-intake/release-artifact-intake.md"

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    printf '%s is required for the Hepta UI release artifact roundtrip gate\n' "$1" >&2
    exit 2
  fi
}

require_report() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required release artifact roundtrip input: %s\n' "$path" >&2
    exit 1
  fi
  jq empty "$path" >/dev/null
}

require_file() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required release artifact roundtrip file: %s\n' "$path" >&2
    exit 1
  fi
}

file_sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

file_bytes() {
  wc -c <"$1" | tr -d ' '
}

require_command jq
require_command shasum

require_report "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH"
require_report "$RELEASE_ARTIFACT_TEMPLATE_PATH"
require_file "$RELEASE_ARTIFACT_MARKDOWN_PATH"

rm -rf "$ROUNDTRIP_DIR"
mkdir -p "$ROUNDTRIP_DIR" "$SIMULATED_WAITING_INTAKE_DIR" "$SIMULATED_INTAKE_DIR"

TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-ui-release-artifact-roundtrip.XXXXXX")"
REPORT_TMP="$TMP_DIR/release-artifact-roundtrip-report.json"
trap 'rm -rf "$TMP_DIR"' EXIT

waiting_intake_sha="$(file_sha256 "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH")"
template_sha="$(file_sha256 "$RELEASE_ARTIFACT_TEMPLATE_PATH")"
markdown_sha="$(file_sha256 "$RELEASE_ARTIFACT_MARKDOWN_PATH")"
template_bytes="$(file_bytes "$RELEASE_ARTIFACT_TEMPLATE_PATH")"
markdown_bytes="$(file_bytes "$RELEASE_ARTIFACT_MARKDOWN_PATH")"
main_intake_waiting="$(jq -r '.release_artifact_state.waiting_for_release_artifact' "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH")"
roundtrip_artifact_source_mode="simulated_present_branch"
waiting_intake_source_path="$RELEASE_ARTIFACT_INTAKE_REPORT_PATH"
present_intake_source_path="$SIMULATED_INTAKE_REPORT_PATH"

if [[ "$main_intake_waiting" == "true" ]]; then
  jq -n \
    --slurpfile template_file "$RELEASE_ARTIFACT_TEMPLATE_PATH" \
    --slurpfile intake_file "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH" \
    --arg template_sha "$template_sha" \
    --arg markdown_sha "$markdown_sha" \
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
          credential_value_read:false,
          keychain_identity_lookup_performed:false,
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
          signed_artifact_sha256:$template_sha,
          notarization_ticket_sha256:$intake.source_report_sha256.release_artifact_boundary,
          stapler_validate_sha256:$intake.source_report_sha256.evidence_archive,
          spctl_assessment_sha256:$markdown_sha
        },
        claim_boundary:{
          release_artifact_claim_ready:false,
          simulated_release_artifact_branch_ready:true,
          live_product_claim_ready:false,
          public_distribution_claim_ready:false,
          release_claim_ready:false
        },
        side_effects:{
          filesystem_write:true,
          credential_value_read:false,
          keychain_identity_lookup_performed:false,
          network_call_performed:false,
          app_signed:false,
          app_notarized:false,
          app_stapled:false,
          local_distribution_artifact_written:false,
          public_distribution_artifact_written:false,
          public_upload_performed:false,
          external_mutation:false
        }
      }' >"$SIMULATED_ARTIFACT_PATH"

  env \
    HEPTA_UI_PRODUCT_READINESS_DIR="$READINESS_DIR" \
    HEPTA_UI_RELEASE_ARTIFACT_INTAKE_REPORT_PATH="$SIMULATED_INTAKE_REPORT_PATH" \
    HEPTA_UI_RELEASE_ARTIFACT_INTAKE_DIR="$SIMULATED_INTAKE_DIR" \
    HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH="$SIMULATED_ARTIFACT_PATH" \
    ./scripts/hepta-ui-release-artifact-intake-gate.sh >/dev/null
else
  roundtrip_artifact_source_mode="actual_present_artifact_input"
  actual_artifact_input_path="$(jq -r '.release_artifact_state.release_artifact_input_path // ""' "$RELEASE_ARTIFACT_INTAKE_REPORT_PATH")"
  if [[ -z "$actual_artifact_input_path" || ! -s "$actual_artifact_input_path" ]]; then
    printf 'Main release artifact intake is present but its artifact input path is unavailable: %s\n' "$actual_artifact_input_path" >&2
    exit 1
  fi
  cp "$actual_artifact_input_path" "$SIMULATED_ARTIFACT_PATH"
  env \
    HEPTA_UI_PRODUCT_READINESS_DIR="$READINESS_DIR" \
    HEPTA_UI_RELEASE_ARTIFACT_INTAKE_REPORT_PATH="$SIMULATED_WAITING_INTAKE_REPORT_PATH" \
    HEPTA_UI_RELEASE_ARTIFACT_INTAKE_DIR="$SIMULATED_WAITING_INTAKE_DIR" \
    HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH="" \
    ./scripts/hepta-ui-release-artifact-intake-gate.sh >/dev/null
  waiting_intake_source_path="$SIMULATED_WAITING_INTAKE_REPORT_PATH"
  present_intake_source_path="$RELEASE_ARTIFACT_INTAKE_REPORT_PATH"
fi

require_report "$waiting_intake_source_path"
require_report "$present_intake_source_path"

simulated_artifact_sha="$(file_sha256 "$SIMULATED_ARTIFACT_PATH")"
simulated_intake_sha="$(file_sha256 "$present_intake_source_path")"
simulated_artifact_bytes="$(file_bytes "$SIMULATED_ARTIFACT_PATH")"
simulated_intake_bytes="$(file_bytes "$present_intake_source_path")"
waiting_intake_sha="$(file_sha256 "$waiting_intake_source_path")"

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
  --arg simulated_artifact_path "$SIMULATED_ARTIFACT_PATH" \
  --arg simulated_intake_report_path "$present_intake_source_path" \
  --arg waiting_intake_source_path "$waiting_intake_source_path" \
  --arg roundtrip_artifact_source_mode "$roundtrip_artifact_source_mode" \
  --arg waiting_intake_sha "$waiting_intake_sha" \
  --arg template_sha "$template_sha" \
  --arg markdown_sha "$markdown_sha" \
  --arg simulated_artifact_sha "$simulated_artifact_sha" \
  --arg simulated_intake_sha "$simulated_intake_sha" \
  --argjson template_bytes "$template_bytes" \
  --argjson markdown_bytes "$markdown_bytes" \
  --argjson simulated_artifact_bytes "$simulated_artifact_bytes" \
  --argjson simulated_intake_bytes "$simulated_intake_bytes" \
  --slurpfile waiting_intake_file "$waiting_intake_source_path" \
  --slurpfile template_file "$RELEASE_ARTIFACT_TEMPLATE_PATH" \
  --slurpfile simulated_artifact_file "$SIMULATED_ARTIFACT_PATH" \
  --slurpfile simulated_intake_file "$present_intake_source_path" \
  '
  ($waiting_intake_file[0]) as $waiting
  | ($template_file[0]) as $template
  | ($simulated_artifact_file[0]) as $artifact
  | ($simulated_intake_file[0]) as $present
  | def sha_ready($sha): ($sha | test("^[0-9a-f]{64}$"));
    def waiting_branch_ready:
      $waiting.release_artifact_intake_gate_ready == true
      and $waiting.status == "ready"
      and $waiting.intake_kind == "local_signed_notarized_stapled_artifact_intake_contract"
      and $waiting.release_artifact_state.waiting_for_release_artifact == true
      and $waiting.release_artifact_state.release_artifact_present == false
      and $waiting.release_artifact_state.release_artifact_valid == false
      and $waiting.release_artifact_state.signed_notarized_stapled_artifact_present == false
      and $waiting.release_artifact_state.public_distribution_artifact_written == false
      and $waiting.claim_boundary.local_release_artifact_intake_ready == true
      and $waiting.claim_boundary.release_artifact_claim_ready == false
      and $waiting.claim_boundary.public_distribution_claim_ready == false
      and $waiting.claim_boundary.release_claim_ready == false;
    def simulated_artifact_ready:
      $artifact.artifact_kind == "signed_notarized_stapled_artifact"
      and $artifact.artifact_version == 1
      and (($artifact.artifact_mode // "actual_present_artifact_input") == "local_simulated_artifact_roundtrip_only" or $roundtrip_artifact_source_mode == "actual_present_artifact_input")
      and $artifact.owner_lane == "release_operator"
      and $artifact.bundle_identifier == $template.bundle_identifier
      and $artifact.release_approval_valid == true
      and $artifact.artifact_evidence.signed == true
      and $artifact.artifact_evidence.notarized == true
      and $artifact.artifact_evidence.stapled == true
      and $artifact.artifact_evidence.local_distribution_artifact_written == true
      and $artifact.artifact_evidence.public_distribution_artifact_written == true
      and $artifact.artifact_evidence.public_distribution_artifact_semantics == "local_simulated_signed_notarized_stapled_dmg_written_not_public_upload"
      and $artifact.artifact_evidence.public_upload_performed == false
      and sha_ready($artifact.artifact_evidence.signed_artifact_sha256)
      and sha_ready($artifact.artifact_evidence.notarization_ticket_sha256)
      and sha_ready($artifact.artifact_evidence.stapler_validate_sha256)
      and sha_ready($artifact.artifact_evidence.spctl_assessment_sha256)
      and (($artifact.simulated_provenance.release_operator_execution_performed // false) == false)
      and (($artifact.simulated_provenance.credential_value_read // false) == false)
      and (($artifact.simulated_provenance.network_call_performed // false) == false)
      and (($artifact.simulated_provenance.signing_performed // false) == false)
      and (($artifact.simulated_provenance.notarization_performed // false) == false)
      and (($artifact.simulated_provenance.stapling_performed // false) == false)
      and (($artifact.simulated_provenance.public_upload_performed // false) == false)
      and (($artifact.simulated_provenance.external_mutation // false) == false)
      and $artifact.claim_boundary.release_artifact_claim_ready == false
      and (
        $artifact.claim_boundary.simulated_release_artifact_branch_ready == true
        or $roundtrip_artifact_source_mode == "actual_present_artifact_input"
      )
      and $artifact.claim_boundary.public_distribution_claim_ready == false
      and $artifact.claim_boundary.release_claim_ready == false;
    def present_branch_ready:
      $present.release_artifact_intake_gate_ready == true
      and $present.status == "ready"
      and $present.release_artifact_state.waiting_for_release_artifact == false
      and $present.release_artifact_state.release_artifact_present == true
      and $present.release_artifact_state.release_artifact_valid == true
      and $present.release_artifact_state.signed_app_artifact_present == true
      and $present.release_artifact_state.notarized_app_artifact_present == true
      and $present.release_artifact_state.stapled_app_artifact_present == true
      and $present.release_artifact_state.signed_notarized_stapled_artifact_present == true
      and $present.release_artifact_state.local_distribution_artifact_written == true
      and $present.release_artifact_state.public_distribution_artifact_written == true
      and $present.release_artifact_state.public_upload_performed == false
      and (
        $present.release_artifact_state.public_distribution_artifact_semantics == "local_signed_notarized_stapled_dmg_written_not_public_upload"
        or $present.release_artifact_state.public_distribution_artifact_semantics == "local_simulated_signed_notarized_stapled_dmg_written_not_public_upload"
      )
      and (($present.release_artifact_source_side_effects.public_upload_performed // false) == false)
      and $present.release_artifact_state.release_artifact_input_sha256 == $simulated_artifact_sha
      and $present.release_artifact_state.release_artifact_input_bytes == $simulated_artifact_bytes
      and (
        (
          $present.source_alignment.release_approval_valid == true
          and ($present.release_artifact_blockers | index("operator_release_approval_required") == null)
        )
        or
        (
          $present.source_alignment.release_approval_valid == false
          and ($present.release_artifact_blockers | index("operator_release_approval_required") != null)
        )
      )
      and ($present.release_artifact_blockers | index("post_artifact_ui_readiness_refresh_required") != null)
      and (
        (
          ($present.source_alignment.real_backend_receipt_claim_ready // false) == true
          and ($present.release_artifact_blockers | index("real_backend_receipt_missing") == null)
        )
        or
        (
          ($present.source_alignment.real_backend_receipt_claim_ready // false) == false
          and ($present.release_artifact_blockers | index("real_backend_receipt_missing") != null)
        )
      )
      and ($present.release_artifact_blockers | index("signed_notarized_stapled_artifact_missing") == null)
      and ($present.release_artifact_blockers | index("public_distribution_artifact_not_written") == null)
      and $present.claim_boundary.local_release_artifact_intake_ready == true
      and $present.claim_boundary.release_artifact_claim_ready == false
      and $present.claim_boundary.release_execution_ready == false
      and $present.claim_boundary.public_distribution_claim_ready == false
      and $present.claim_boundary.release_claim_ready == false
      and $present.side_effects.external_mutation == false;
    (
      waiting_branch_ready
      and simulated_artifact_ready
      and present_branch_ready
      and sha_ready($waiting_intake_sha)
      and sha_ready($template_sha)
      and sha_ready($markdown_sha)
      and sha_ready($simulated_artifact_sha)
      and sha_ready($simulated_intake_sha)
      and $template_bytes > 0
      and $markdown_bytes > 0
      and $simulated_artifact_bytes > 0
      and $simulated_intake_bytes > 0
    ) as $ready
  | {
      product:$product,
      runtime:$runtime,
      gate:$gate,
      status:(if $ready then "ready" else "failed" end),
      release_artifact_roundtrip_gate_ready:$ready,
      roundtrip_kind:"local_release_artifact_valid_branch_replay",
      roundtrip_version:1,
      roundtrip_artifact_source_mode:$roundtrip_artifact_source_mode,
      readiness_dir:$readiness_dir,
      report_path:$report_path,
      roundtrip_dir:$roundtrip_dir,
      source_reports:{
        waiting_intake:$waiting_intake_report_path,
        waiting_intake_source:$waiting_intake_source_path,
        release_artifact_template:$template_path,
        release_artifact_intake_markdown:$markdown_path,
        simulated_artifact:$simulated_artifact_path,
        simulated_artifact_intake:$simulated_intake_report_path
      },
      source_report_sha256:{
        waiting_intake:$waiting_intake_sha,
        release_artifact_template:$template_sha,
        release_artifact_intake_markdown:$markdown_sha,
        simulated_artifact:$simulated_artifact_sha,
        simulated_artifact_intake:$simulated_intake_sha
      },
      source_report_bytes:{
        release_artifact_template:$template_bytes,
        release_artifact_intake_markdown:$markdown_bytes,
        simulated_artifact:$simulated_artifact_bytes,
        simulated_artifact_intake:$simulated_intake_bytes
      },
      bundle_identifier:$template.bundle_identifier,
      roundtrip_ready_count:(if $ready then 2 else 0 end),
      source_alignment:{
        waiting_branch_ready:waiting_branch_ready,
        present_branch_ready:present_branch_ready,
        simulated_artifact_ready:simulated_artifact_ready,
        waiting_branch_release_artifact_present:$waiting.release_artifact_state.release_artifact_present,
        waiting_branch_release_artifact_valid:$waiting.release_artifact_state.release_artifact_valid,
        present_branch_release_artifact_present:$present.release_artifact_state.release_artifact_present,
        present_branch_release_artifact_valid:$present.release_artifact_state.release_artifact_valid,
        present_branch_signed_notarized_stapled_artifact_present:$present.release_artifact_state.signed_notarized_stapled_artifact_present,
        present_branch_local_distribution_artifact_written:$present.release_artifact_state.local_distribution_artifact_written,
        present_branch_public_distribution_artifact_written:$present.release_artifact_state.public_distribution_artifact_written,
        present_branch_public_upload_performed:$present.release_artifact_state.public_upload_performed,
        present_branch_public_distribution_artifact_semantics:$present.release_artifact_state.public_distribution_artifact_semantics,
        present_branch_source_public_upload_performed:($present.release_artifact_source_side_effects.public_upload_performed // false),
        present_branch_release_approval_valid:$present.source_alignment.release_approval_valid,
        present_branch_operator_release_approval_required:(($present.release_artifact_blockers | index("operator_release_approval_required")) != null),
        present_branch_post_artifact_refresh_required:(($present.release_artifact_blockers | index("post_artifact_ui_readiness_refresh_required")) != null),
        present_branch_real_backend_receipt_missing:(($present.release_artifact_blockers | index("real_backend_receipt_missing")) != null),
        present_branch_real_backend_receipt_claim_ready:($present.source_alignment.real_backend_receipt_claim_ready // false),
        present_branch_release_artifact_claim_ready:$present.claim_boundary.release_artifact_claim_ready,
        present_branch_release_claim_ready:$present.claim_boundary.release_claim_ready,
        root_report_replay_required_count_after_roundtrip:41
      },
      claim_boundary:{
        local_release_artifact_roundtrip_ready:$ready,
        local_release_artifact_intake_waiting_branch_ready:waiting_branch_ready,
        local_release_artifact_intake_present_branch_ready:present_branch_ready,
        simulated_release_artifact_branch_ready:simulated_artifact_ready,
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
        local_simulated_artifact_written:true,
        local_present_branch_report_written:true,
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
  and .release_artifact_roundtrip_gate_ready == true
  and .roundtrip_kind == "local_release_artifact_valid_branch_replay"
  and .roundtrip_version == 1
  and .roundtrip_ready_count == 2
  and .source_alignment.waiting_branch_ready == true
  and .source_alignment.present_branch_ready == true
  and .source_alignment.simulated_artifact_ready == true
  and .source_alignment.waiting_branch_release_artifact_present == false
  and .source_alignment.waiting_branch_release_artifact_valid == false
  and .source_alignment.present_branch_release_artifact_present == true
  and .source_alignment.present_branch_release_artifact_valid == true
  and .source_alignment.present_branch_signed_notarized_stapled_artifact_present == true
  and .source_alignment.present_branch_local_distribution_artifact_written == true
  and .source_alignment.present_branch_public_distribution_artifact_written == true
  and .source_alignment.present_branch_public_upload_performed == false
  and (
    .source_alignment.present_branch_public_distribution_artifact_semantics == "local_signed_notarized_stapled_dmg_written_not_public_upload"
    or .source_alignment.present_branch_public_distribution_artifact_semantics == "local_simulated_signed_notarized_stapled_dmg_written_not_public_upload"
  )
  and .source_alignment.present_branch_source_public_upload_performed == false
  and (
    (
      .source_alignment.present_branch_release_approval_valid == true
      and .source_alignment.present_branch_operator_release_approval_required == false
    )
    or
    (
      .source_alignment.present_branch_release_approval_valid == false
      and .source_alignment.present_branch_operator_release_approval_required == true
    )
  )
  and .source_alignment.present_branch_post_artifact_refresh_required == true
  and (
    (
      .source_alignment.present_branch_real_backend_receipt_claim_ready == true
      and .source_alignment.present_branch_real_backend_receipt_missing == false
    )
    or
    (
      .source_alignment.present_branch_real_backend_receipt_claim_ready == false
      and .source_alignment.present_branch_real_backend_receipt_missing == true
    )
  )
  and .source_alignment.present_branch_release_artifact_claim_ready == false
  and .source_alignment.present_branch_release_claim_ready == false
  and .source_alignment.root_report_replay_required_count_after_roundtrip == 41
  and .claim_boundary.local_release_artifact_roundtrip_ready == true
  and .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.release_execution_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.external_actions_allowed == false
  and .claim_boundary.public_upload_performed == false
  and .claim_boundary.signing_notarization_performed == false
  and .side_effects.local_simulated_artifact_written == true
  and .side_effects.local_present_branch_report_written == true
  and .side_effects.credential_value_read == false
  and .side_effects.network_call_performed == false
  and .side_effects.notary_submission_performed == false
  and .side_effects.app_signed == false
  and .side_effects.app_notarized == false
  and .side_effects.app_stapled == false
  and .side_effects.local_distribution_artifact_written == false
  and .side_effects.public_distribution_artifact_written == false
  and .side_effects.public_upload_performed == false
  and .side_effects.external_mutation == false
' "$REPORT_TMP" >/dev/null

mkdir -p "$(dirname "$REPORT_PATH")"
cp "$REPORT_TMP" "$REPORT_PATH"
cat "$REPORT_TMP"
