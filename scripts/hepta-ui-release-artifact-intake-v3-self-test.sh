#!/bin/bash -p
set +x
PS4='+ '
set -Eeuo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"
PATH="$SYSTEM_PATH"
export PATH

cd "$(dirname "$0")/.."

for command in clang codesign ditto hdiutil jq plutil ruby shasum swift tar; do
  command -v "$command" >/dev/null 2>&1 || {
    printf '%s is required for the release artifact intake v3 self-test\n' "$command" >&2
    exit 2
  }
done

TEST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hepta-release-intake-v3-self-test.XXXXXX")"
FAKE_BIN="$TEST_DIR/fake-bin"
mkdir -p "$FAKE_BIN"
cleanup() {
  if [[ "${HEPTA_RELEASE_SELF_TEST_KEEP:-0}" == "1" ]]; then
    printf 'intake v3 self-test evidence retained: %s\n' "$TEST_DIR" >&2
  else
    rm -rf "$TEST_DIR"
  fi
}
trap cleanup EXIT INT TERM

cat >"$FAKE_BIN/path-shim" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
parent_ppid="$(/bin/ps -p "$PPID" -o ppid= 2>/dev/null | /usr/bin/tr -d ' ' || true)"
printf '%s parent=%s grandparent=%s\n' \
  "${0##*/} $*" \
  "$(/bin/ps -p "$PPID" -o command= 2>/dev/null || true)" \
  "$(/bin/ps -p "$parent_ppid" -o command= 2>/dev/null || true)" \
  >>"${PATH_SHIM_USED_MARKER:?}"
exit 0
EOF
chmod +x "$FAKE_BIN/path-shim"
for tool in cat codesign cp dirname ditto find git hdiutil jq mkdir mktemp mount plutil rm ruby shasum spctl swift xcrun; do
  cp "$FAKE_BIN/path-shim" "$FAKE_BIN/$tool"
done

sha256() {
  shasum -a 256 "$1" | awk '{print $1}'
}

bytes() {
  wc -c <"$1" | tr -d ' '
}

write_source_reports() {
  local readiness_dir="$1" fixed_sha
  fixed_sha="$(printf 'a%.0s' {1..64})"
  mkdir -p "$readiness_dir"
  jq -n '
    {
      distribution_preflight_gate_ready:true,
      distribution_static_contract_ready:true,
      public_distribution_ready:false,
      release_approval_required:true,
      credential_values_read:false,
      keychain_identity_lookup_performed:false,
      network_call_performed:false,
      notary_submission_performed:false,
      app_signed:false,
      app_notarized:false,
      app_stapled:false,
      public_distribution_artifact_written:false,
      package_metadata:{bundle_identifier:"ai.hepta.nativeapp",bundle_name:"Hepta",bundle_executable:"hepta-native"}
    }
  ' >"$readiness_dir/native-distribution-preflight-gate.json"
  jq -n --arg sha "$fixed_sha" '
    {
      release_approval_intake_gate_ready:true,
      template_sha256:$sha,
      release_approval_state:{
        waiting_for_release_approval:true,
        release_approval_present:false,
        release_approval_valid:false,
        independent_approval_verifier_ready:false,
        self_reported_approval_can_authorize_release:false,
        approval_only_can_make_release_claim:false,
        signed_notarized_stapled_artifact_present:false,
        public_distribution_artifact_written:false
      },
      approval_blockers:[
        "operator_release_approval_required",
        "independent_release_approval_verifier_unavailable",
        "signed_notarized_stapled_artifact_missing",
        "public_distribution_artifact_not_written"
      ],
      source_alignment:{approval_valid_branch_supported:false},
      claim_boundary:{release_approval_claim_ready:false,release_execution_ready:false,public_distribution_claim_ready:false,release_claim_ready:false}
    }
  ' >"$readiness_dir/ui-release-approval-intake-gate.json"
  jq -n --arg sha "$fixed_sha" '
    {
      release_artifact_boundary_gate_ready:true,
      boundary_markdown_sha256:$sha,
      release_artifact_boundary:{
        root_report_replay_required_count_after_boundary:36,
        next_required_artifact_gate:"signed_notarized_stapled_artifact_gate",
        signed_notarized_stapled_artifact_present:false,
        public_distribution_artifact_written:false,
        unsigned_app_bundle_sha256:$sha,
        unsigned_app_bundle_codesign_status:"unsigned"
      },
      claim_boundary:{release_artifact_claim_ready:false,public_distribution_claim_ready:false,release_claim_ready:false,real_backend_receipt_claim_ready:false,backend_receipt_claim_ready:false}
    }
  ' >"$readiness_dir/ui-release-artifact-boundary-gate.json"
  jq -n --arg sha "$fixed_sha" '
    {
      evidence_archive_gate_ready:true,
      archive_sha256:$sha,
      archive_bytes:1024,
      all_extracted_items_sha256_match:true,
      claim_boundary:{local_evidence_archive_ready:true,public_distribution_claim_ready:false,release_claim_ready:false}
    }
  ' >"$readiness_dir/ui-evidence-archive-gate.json"
  jq -n --arg sha "$fixed_sha" '
    {
      release_operator_dry_run_gate_ready:true,
      dry_run_manifest_sha256:$sha,
      release_candidate:{unsigned_app_bundle_sha256:$sha},
      operator_packet:{dry_run_only:true,operator_approval_recorded:false,public_distribution_artifact_written:false},
      claim_boundary:{
        local_release_operator_dry_run_ready:true,
        release_execution_ready:false,
        release_claim_ready:false,
        public_distribution_claim_ready:false,
        blocked_by:["operator_release_approval_required","public_distribution_artifact_not_written"]
      }
    }
  ' >"$readiness_dir/ui-release-operator-dry-run-gate.json"
  jq -n --arg sha "$fixed_sha" '
    {
      operator_briefing_refresh_gate_ready:true,
      refresh_markdown_sha256:$sha,
      current_state:{root_report_replay_required_count_after_refresh:33},
      updated_critical_risk_count:1,
      refreshed_operator_briefing:{updated_critical_risks:[{id:"release_public_distribution_not_approved"}]},
      claim_boundary:{local_operator_briefing_refresh_ready:true,public_distribution_claim_ready:false,release_claim_ready:false}
    }
  ' >"$readiness_dir/ui-operator-briefing-refresh-gate.json"
}

make_forged_case() {
  local name="$1" mode="${2:-forged_tar}" case_dir readiness_dir evidence_dir source_app payload_dir dmg_path source_receipt receipt_path
  local fixed_head fixed_tree fixed_source
  case_dir="$TEST_DIR/$name"
  readiness_dir="$case_dir/readiness"
  evidence_dir="$case_dir/evidence"
  source_app="$case_dir/source/Hepta.app"
  payload_dir="$case_dir/payload"
  dmg_path="$evidence_dir/Hepta.dmg"
  source_receipt="$evidence_dir/formal-unsigned-package-receipt.json"
  receipt_path="$case_dir/release-receipt.json"
  fixed_head="$(printf 'b%.0s' {1..40})"
  fixed_tree="$(printf 'c%.0s' {1..40})"
  fixed_source="$(printf 'd%.0s' {1..64})"

  mkdir -p "$source_app/Contents/MacOS" "$source_app/Contents/Resources" "$payload_dir" "$evidence_dir" "$case_dir/tmp" "$case_dir/mounts"
  if [[ "$mode" == "forged_tar" ]]; then
    printf '#!/usr/bin/env bash\nprintf "hepta intake fixture\\n"\n' >"$source_app/Contents/MacOS/hepta-native"
  else
    cat >"$case_dir/source-main.c" <<'C'
#include <stdio.h>
int main(void) { puts("hepta intake normalization source"); return 0; }
C
    if [[ "$mode" == "adhoc_fat" ]]; then
      /usr/bin/clang -arch arm64 -arch x86_64 "$case_dir/source-main.c" -o "$source_app/Contents/MacOS/hepta-native"
    else
      /usr/bin/clang "$case_dir/source-main.c" -o "$source_app/Contents/MacOS/hepta-native"
    fi
    /usr/bin/codesign --remove-signature "$source_app/Contents/MacOS/hepta-native" >/dev/null 2>&1 || true
    if /usr/bin/codesign -d "$source_app/Contents/MacOS/hepta-native" >/dev/null 2>&1; then
      printf 'failed to construct a strictly unsigned Mach-O source fixture\n' >&2
      return 1
    fi
  fi
  chmod 755 "$source_app/Contents/MacOS/hepta-native"
  printf 'fixture resource\n' >"$source_app/Contents/Resources/product.dat"
  cat >"$source_app/Contents/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleIdentifier</key><string>ai.hepta.nativeapp</string>
<key>CFBundleExecutable</key><string>hepta-native</string>
<key>CFBundleName</key><string>Hepta</string>
</dict></plist>
PLIST

  ditto "$source_app" "$payload_dir/Hepta.app"
  case "$mode" in
    forged_tar)
      printf 'finder bookmark fixture\n' >"$payload_dir/Applications"
      /usr/bin/tar -cpf "$dmg_path" -C "$payload_dir" .
      ;;
    adhoc_equal|adhoc_resource_tamper|adhoc_binary_tamper|adhoc_fat)
      if [[ "$mode" == "adhoc_resource_tamper" ]]; then
        printf 'payload-only resource tamper\n' >>"$payload_dir/Hepta.app/Contents/Resources/product.dat"
      elif [[ "$mode" == "adhoc_binary_tamper" ]]; then
        cat >"$case_dir/payload-main.c" <<'C'
#include <stdio.h>
int main(void) { puts("hepta intake normalization payload tamper"); return 0; }
C
        /usr/bin/clang "$case_dir/payload-main.c" -o "$payload_dir/Hepta.app/Contents/MacOS/hepta-native"
      fi
      /usr/bin/codesign --force --sign - --options runtime --timestamp=none \
        "$payload_dir/Hepta.app/Contents/MacOS/hepta-native" >/dev/null
      /usr/bin/codesign --force --sign - --options runtime --timestamp=none \
        "$payload_dir/Hepta.app" >/dev/null
      /usr/bin/swift -e 'import Foundation; let target=URL(fileURLWithPath:CommandLine.arguments[1]); let destination=URL(fileURLWithPath:CommandLine.arguments[2]); let data=try target.bookmarkData(options:.suitableForBookmarkFile,includingResourceValuesForKeys:nil,relativeTo:nil); try URL.writeBookmarkData(data,to:destination)' \
        /Applications "$payload_dir/Applications"
      /usr/bin/hdiutil create -quiet -volname "HeptaIntake$$" -srcfolder "$payload_dir" -format UDZO "$dmg_path"
      /usr/bin/codesign --force --sign - "$dmg_path" >/dev/null
      ;;
    *)
      printf 'unknown intake v3 fixture mode: %s\n' "$mode" >&2
      return 64
      ;;
  esac

  local bundle_fingerprint binary_sha signed_bundle_fingerprint signed_binary_sha
  bundle_fingerprint="$(ruby apps/hepta-native/packaging/app-bundle-fingerprint-v1.rb "$source_app")"
  binary_sha="$(sha256 "$source_app/Contents/MacOS/hepta-native")"
  signed_bundle_fingerprint="$(ruby apps/hepta-native/packaging/app-bundle-fingerprint-v1.rb "$payload_dir/Hepta.app")"
  signed_binary_sha="$(sha256 "$payload_dir/Hepta.app/Contents/MacOS/hepta-native")"
  jq -n \
    --arg app "$source_app" \
    --arg binary_sha "$binary_sha" \
    --arg head "$fixed_head" \
    --arg tree "$fixed_tree" \
    --arg source "$fixed_source" \
    --argjson fingerprint "$bundle_fingerprint" \
    '{
      schema_version:1,
      kind:"hepta-native-current-package-gate",
      status:"ready",
      local_package_ready:true,
      signed:false,
      notarized:false,
      stapled:false,
      source_stable_during_run:true,
      repository_worktree_clean:true,
      artifact:{path:$app,binary_sha256:$binary_sha,bundle_fingerprint:$fingerprint,full_head_embedded:true,developer_id_signed:false},
      source_binding:{head:$head,head_tree:$tree,source_fingerprint:$source,worktree_clean:true,repository_worktree_clean:true}
    }' >"$source_receipt"

  printf '{"status":"Accepted","id":"intake-v3-self-test-submission"}\n' >"$evidence_dir/notarytool-submit.log"
  printf 'codesign app valid\n' >"$evidence_dir/codesign-verify-app.log"
  printf 'codesign dmg valid\n' >"$evidence_dir/codesign-verify-dmg.log"
  printf 'stapler staple valid\n' >"$evidence_dir/stapler-staple.log"
  printf 'stapler validate valid\n' >"$evidence_dir/stapler-validate.log"
  printf 'spctl assessment valid\n' >"$evidence_dir/spctl-assessment.log"
  printf '<plist><dict><key>mount-point</key><string>/Volumes/Hepta</string></dict></plist>\n' >"$evidence_dir/dmg-readonly-attach.plist"
  printf '/dev/disk-fixture on /Volumes/Hepta (hfs, local, read-only)\n' >"$evidence_dir/dmg-readonly-mount.log"

  local notary_log="$evidence_dir/notarytool-submit.log"
  jq -n \
    --arg source_app "$source_app" \
    --arg source_binary_sha "$binary_sha" \
    --arg signed_binary_sha "$signed_binary_sha" \
    --arg source_receipt "$source_receipt" \
    --arg source_receipt_sha "$(sha256 "$source_receipt")" \
    --arg source_head "$fixed_head" \
    --arg source_tree "$fixed_tree" \
    --arg source_fingerprint "$fixed_source" \
    --arg dmg_path "$dmg_path" \
    --arg dmg_sha "$(sha256 "$dmg_path")" \
    --argjson dmg_bytes "$(bytes "$dmg_path")" \
    --arg notary_path "$notary_log" \
    --arg notary_sha "$(sha256 "$notary_log")" \
    --argjson notary_bytes "$(bytes "$notary_log")" \
    --arg codesign_app_path "$evidence_dir/codesign-verify-app.log" \
    --arg codesign_app_sha "$(sha256 "$evidence_dir/codesign-verify-app.log")" \
    --arg codesign_dmg_path "$evidence_dir/codesign-verify-dmg.log" \
    --arg codesign_dmg_sha "$(sha256 "$evidence_dir/codesign-verify-dmg.log")" \
    --arg staple_path "$evidence_dir/stapler-staple.log" \
    --arg staple_sha "$(sha256 "$evidence_dir/stapler-staple.log")" \
    --arg validate_path "$evidence_dir/stapler-validate.log" \
    --arg validate_sha "$(sha256 "$evidence_dir/stapler-validate.log")" \
    --arg spctl_path "$evidence_dir/spctl-assessment.log" \
    --arg spctl_sha "$(sha256 "$evidence_dir/spctl-assessment.log")" \
    --arg attach_path "$evidence_dir/dmg-readonly-attach.plist" \
    --arg attach_sha "$(sha256 "$evidence_dir/dmg-readonly-attach.plist")" \
    --arg mount_path "$evidence_dir/dmg-readonly-mount.log" \
    --arg mount_sha "$(sha256 "$evidence_dir/dmg-readonly-mount.log")" \
    --argjson bundle_fingerprint "$bundle_fingerprint" \
    --argjson signed_bundle_fingerprint "$signed_bundle_fingerprint" \
    '{
      artifact_kind:"signed_notarized_stapled_artifact",
      artifact_version:3,
      receipt_contract_version:3,
      status:"ready",
      owner_lane:"release_operator",
      product:"Hepta Native",
      bundle_identifier:"ai.hepta.nativeapp",
      release_approval_valid:true,
      source_evidence:{
        source_app:$source_app,
        source_binary_sha256:$source_binary_sha,
        signed_binary_sha256:$signed_binary_sha,
        source_app_bundle_fingerprint:$bundle_fingerprint,
        signed_app_bundle_fingerprint:$signed_bundle_fingerprint,
        unsigned_package_receipt_path:$source_receipt,
        unsigned_package_receipt_sha256:$source_receipt_sha,
        source_head:$source_head,
        source_tree:$source_tree,
        source_fingerprint:$source_fingerprint,
        source_worktree_clean:true,
        source_stable_during_unsigned_package_run:true,
        private_copy_recomputed_before_signing:true,
        consumed_exact_formal_app:true,
        built_second_product_app:false
      },
      artifact_evidence:{
        signed:true,notarized:true,stapled:true,dmg_stapled:true,app_stapled:false,
        local_distribution_artifact_written:true,
        public_distribution_artifact_written:true,
        public_distribution_artifact_semantics:"local_signed_notarized_stapled_dmg_written_not_public_upload",
        public_upload_performed:false,
        signed_artifact_path:$dmg_path,
        signed_artifact_sha256:$dmg_sha,
        signed_artifact_bytes:$dmg_bytes,
        notarization_ticket_sha256:$notary_sha,
        notarytool_submit_log_sha256:$notary_sha,
        notarytool_submit_log_bytes:$notary_bytes,
        notarytool_exit_code:0,
        notary_submission_id:"intake-v3-self-test-submission",
        notary_submission_state:"accepted",
        notary_submission_confirmed:true,
        notary_submission_may_have_occurred:true,
        codesign_verify_app_sha256:$codesign_app_sha,
        codesign_verify_dmg_sha256:$codesign_dmg_sha,
        stapler_staple_sha256:$staple_sha,
        stapler_validate_sha256:$validate_sha,
        spctl_assessment_sha256:$spctl_sha,
        dmg_mounted_read_only:true,
        mounted_app_bundle_fingerprint:$signed_bundle_fingerprint,
        mounted_binary_sha256:$signed_binary_sha,
        mounted_bundle_identifier:"ai.hepta.nativeapp",
        applications_alias_verified:true,
        applications_alias_kind:"finder_bookmark_alias",
        applications_alias_resolved_target:"/Applications",
        dmg_readonly_attach_sha256:$attach_sha,
        dmg_readonly_mount_sha256:$mount_sha,
        notarytool_submit_log_path:$notary_path,
        codesign_verify_app_log_path:$codesign_app_path,
        codesign_verify_dmg_log_path:$codesign_dmg_path,
        stapler_staple_log_path:$staple_path,
        stapler_validate_log_path:$validate_path,
        spctl_assessment_log_path:$spctl_path,
        dmg_readonly_attach_path:$attach_path,
        dmg_readonly_mount_log_path:$mount_path,
        signing_identity:"Developer ID Application: Hepta Self Test (ABCDEFGHIJ)",
        signing_team_identifier:"ABCDEFGHIJ",
        codesign_app_runtime_version:"26.0.0",
        codesign_app_flags:"0x10000(runtime)",
        codesign_app_timestamp:"Aug 2, 2026 at 12:00:00",
        codesign_dmg_timestamp:"Aug 2, 2026 at 12:00:00",
        notary_auth_mode:"keychain_profile"
      },
      claim_boundary:{release_artifact_claim_ready:false,release_execution_ready:false,public_distribution_claim_ready:false,release_claim_ready:false,live_product_claim_ready:false},
      side_effects:{credential_value_captured:false,keychain_identity_lookup_performed:true,network_call_performed:true,notary_submission_performed:true,app_signed:true,app_notarized:true,app_stapled:false,dmg_stapled:true,local_distribution_artifact_written:true,public_distribution_artifact_written:true,public_upload_performed:false,external_mutation:true}
    }' >"$receipt_path"

  write_source_reports "$readiness_dir"
  printf '%s\n' "$case_dir"
}

run_gate() {
  local case_dir="$1" receipt_path expected_team_id="${3:-ABCDEFGHIJ}"
  if [[ $# -ge 2 ]]; then receipt_path="$2"; else receipt_path="$1/release-receipt.json"; fi
  env \
    PATH="$FAKE_BIN:$PATH" \
    TMPDIR="$case_dir/tmp" \
    PATH_SHIM_USED_MARKER="$TEST_DIR/path-shim-used" \
    HEPTA_EXPECTED_SIGNING_IDENTITY="Developer ID Application: Hepta Self Test ($expected_team_id)" \
    HEPTA_EXPECTED_TEAM_ID="$expected_team_id" \
    HEPTA_UI_PRODUCT_READINESS_DIR="$case_dir/readiness" \
    HEPTA_UI_RELEASE_ARTIFACT_INTAKE_REPORT_PATH="$case_dir/intake-report.json" \
    HEPTA_UI_RELEASE_ARTIFACT_INTAKE_DIR="$case_dir/intake" \
    HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH="$receipt_path" \
    ./scripts/hepta-ui-release-artifact-intake-gate.sh
}

assert_invalid() {
  local case_dir="$1"
  if run_gate "$case_dir" >"$case_dir/stdout" 2>"$case_dir/stderr"; then
    printf 'release artifact intake v3 self-test unexpectedly accepted %s\n' "$(basename "$case_dir")" >&2
    exit 1
  fi
  jq -e '
    .status == "failed"
    and .release_artifact_intake_gate_ready == false
    and .release_artifact_state.release_artifact_present == true
    and .release_artifact_state.release_artifact_valid == false
    and .release_artifact_state.evidence_readback_valid == false
    and .release_artifact_state.independent_system_verification_ready == false
    and .release_artifact_state.current_source_binding_verified == false
    and .release_artifact_state.signed_notarized_stapled_artifact_present == false
    and .release_artifact_state.local_distribution_artifact_written == false
    and .release_artifact_state.public_distribution_artifact_written == false
    and (.release_artifact_blockers | index("release_artifact_v3_readback_not_verified") != null)
    and .claim_boundary.release_artifact_claim_ready == false
    and .claim_boundary.public_distribution_claim_ready == false
  ' "$case_dir/intake-report.json" >/dev/null
  jq -e '
    .all_evidence_valid == false
    and .independent_system_verification.valid == false
    and .independent_system_verification.tool_paths.codesign == "/usr/bin/codesign"
    and .independent_system_verification.tool_paths.xcrun == "/usr/bin/xcrun"
    and .independent_system_verification.tool_paths.spctl == "/usr/sbin/spctl"
    and .independent_system_verification.tool_paths.hdiutil == "/usr/bin/hdiutil"
    and .independent_system_verification.tool_paths.mount == "/sbin/mount"
    and .independent_system_verification.codesign_dmg.verified == false
    and .independent_system_verification.stapler_validate_dmg.verified == false
    and .independent_system_verification.spctl_assess_dmg.verified == false
    and .independent_system_verification.codesign_mounted_app.verified == false
    and .normalized_bundle_equivalence.source_app_strictly_unsigned == true
    and .normalized_bundle_equivalence.source_normalized == false
    and (.blockers | index("independent_system_verification_failed") != null)
    and (.blockers | index("source_app_signature_normalization_failed") != null)
    and (.blockers | index("system_codesign_verify_dmg_failed") != null)
    and (.blockers | index("system_stapler_validate_dmg_failed") != null)
    and (.blockers | index("system_spctl_assess_dmg_failed") != null)
    and (.blockers | index("system_codesign_mounted_app_not_performed") != null)
  ' "$case_dir/intake/release-artifact-readback.json" >/dev/null
  [[ ! -e "$case_dir/intake/release-artifact-input.accepted.json" ]]
  if [[ -e "$TEST_DIR/path-shim-used" ]]; then
    printf 'release artifact gate executed a PATH shim:\n' >&2
    /bin/cat "$TEST_DIR/path-shim-used" >&2
    return 1
  fi
}

assert_premount_trust_rejected() {
  local case_dir="$1" expected_source_normalized="$2"
  if run_gate "$case_dir" >"$case_dir/stdout" 2>"$case_dir/stderr"; then
    printf 'untrusted pre-mount fixture unexpectedly promoted a release: %s\n' "$(basename "$case_dir")" >&2
    exit 1
  fi
  jq -e --argjson source_normalized "$expected_source_normalized" '
    .all_evidence_valid == false
    and .normalized_bundle_equivalence.source_app_strictly_unsigned == true
    and .normalized_bundle_equivalence.source_normalized == $source_normalized
    and .normalized_bundle_equivalence.mounted_signed_app_normalized == false
    and .normalized_bundle_equivalence.exact_path_mode_content_equivalent == false
    and .normalized_bundle_equivalence.unsupported_or_fat_macho_fail_closed == true
    and (if $source_normalized
      then .normalized_bundle_equivalence.source_full_bundle_fingerprint.kind == "hepta-app-bundle-fingerprint"
      else .normalized_bundle_equivalence.source_full_bundle_fingerprint == null
        and (.blockers | index("source_app_signature_normalization_failed") != null)
      end)
    and .independent_system_verification.codesign_dmg.verified == true
    and .independent_system_verification.dmg_signature_tuple_trusted_before_mount == false
    and .independent_system_verification.apple_trust_stapler_spctl_ready_before_mount == false
    and .independent_system_verification.codesign_mounted_app.verified == false
    and .source_app.tree_nofollow_safe == true
    and .independent_dmg_readback.premount_trusted == false
    and .independent_dmg_readback.mounted_app_tree_nofollow_safe == false
    and .independent_system_verification.valid == false
    and (.blockers | index("system_dmg_premount_apple_trust_not_established") != null)
    and (.blockers | index("independent_dmg_readonly_attach_blocked_until_trusted_system_signature") != null)
    and (.blockers | index("system_codesign_mounted_app_not_performed") != null)
  ' "$case_dir/intake/release-artifact-readback.json" >/dev/null
  jq -e '
    .status == "failed"
    and .release_artifact_state.release_artifact_valid == false
    and .release_artifact_state.present_artifact_branch_supported == false
    and .release_artifact_state.independent_approval_verifier_contract_ready == false
    and .release_artifact_state.signed_notarized_stapled_artifact_present == false
    and (.release_artifact_blockers | index("release_artifact_present_branch_unsupported_without_independent_approval_verifier") != null)
    and .claim_boundary.release_artifact_claim_ready == false
  ' "$case_dir/intake-report.json" >/dev/null
  [[ ! -e "$case_dir/intake/release-artifact-input.accepted.json" ]]
  [[ -z "$(find "$case_dir/mounts" -mindepth 1 -print -quit)" ]]
  if [[ -e "$TEST_DIR/path-shim-used" ]]; then
    printf 'release artifact pre-mount trust gate executed a PATH shim\n' >&2
    return 1
  fi
}

assert_path_rejected() {
  local name="$1" report_path="$2" intake_dir="$3" input_path="$4" expected_message="$5"
  local case_dir="$TEST_DIR/path-$name"
  mkdir -p "$case_dir/tmp"
  if env \
    PATH="$FAKE_BIN:$PATH" \
    TMPDIR="$case_dir/tmp" \
    PATH_SHIM_USED_MARKER="$TEST_DIR/path-shim-used" \
    HEPTA_UI_PRODUCT_READINESS_DIR="$WAITING_DIR/readiness" \
    HEPTA_UI_RELEASE_ARTIFACT_INTAKE_REPORT_PATH="$report_path" \
    HEPTA_UI_RELEASE_ARTIFACT_INTAKE_DIR="$intake_dir" \
    HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH="$input_path" \
    ./scripts/hepta-ui-release-artifact-intake-gate.sh >"$case_dir/stdout" 2>"$case_dir/stderr"; then
    printf 'release artifact intake accepted unsafe path case: %s\n' "$name" >&2
    exit 1
  fi
  /usr/bin/grep -Fq "$expected_message" "$case_dir/stderr"
}

run_approval_gate() {
  local case_dir="$1" input_path="${2:-}"
  env \
    PATH="$FAKE_BIN:$PATH" \
    TMPDIR="$case_dir/tmp" \
    PATH_SHIM_USED_MARKER="$TEST_DIR/path-shim-used" \
    HEPTA_UI_PRODUCT_READINESS_DIR="$case_dir/readiness" \
    HEPTA_UI_RELEASE_APPROVAL_INTAKE_REPORT_PATH="$case_dir/approval-report.json" \
    HEPTA_UI_RELEASE_APPROVAL_INTAKE_DIR="$case_dir/approval-intake" \
    HEPTA_UI_RELEASE_APPROVAL_INPUT_PATH="$input_path" \
    ./scripts/hepta-ui-release-approval-intake-gate.sh
}

WAITING_DIR="$TEST_DIR/waiting"
mkdir -p "$WAITING_DIR/tmp" "$WAITING_DIR/mounts"
write_source_reports "$WAITING_DIR/readiness"
run_gate "$WAITING_DIR" "" >"$WAITING_DIR/stdout"
jq -e '
  .status == "ready"
  and .intake_version == 3
  and .release_artifact_state.waiting_for_release_artifact == true
  and .release_artifact_state.release_artifact_valid == false
' "$WAITING_DIR/intake-report.json" >/dev/null

APPROVAL_TEST_DIR="$TEST_DIR/approval-independent-verifier"
mkdir -p "$APPROVAL_TEST_DIR/tmp"
write_source_reports "$APPROVAL_TEST_DIR/readiness"
run_approval_gate "$APPROVAL_TEST_DIR" "" >"$APPROVAL_TEST_DIR/waiting.stdout"
jq -e '
  .status == "ready"
  and .release_approval_intake_gate_ready == true
  and .release_approval_state.waiting_for_release_approval == true
  and .release_approval_state.release_approval_valid == false
  and .release_approval_state.independent_approval_verifier_ready == false
  and .release_approval_state.self_reported_approval_can_authorize_release == false
  and .source_alignment.approval_valid_branch_supported == false
  and (.approval_blockers | index("independent_release_approval_verifier_unavailable") != null)
' "$APPROVAL_TEST_DIR/approval-report.json" >/dev/null

jq '
  .operator_approval_recorded = true
  | .operator_identity_hash = ("f" * 64)
  | .approved_at = "2026-08-02T19:00:00+08:00"
  | .source_evidence.authorization_message_id = "telegram:123456/7890"
' "$APPROVAL_TEST_DIR/approval-intake/release-approval-template.json" >"$APPROVAL_TEST_DIR/self-reported-complete.json"
jq -e \
  --slurpfile distribution "$APPROVAL_TEST_DIR/readiness/native-distribution-preflight-gate.json" \
  --slurpfile dry "$APPROVAL_TEST_DIR/readiness/ui-release-operator-dry-run-gate.json" \
  --slurpfile archive "$APPROVAL_TEST_DIR/readiness/ui-evidence-archive-gate.json" '
  .approval_kind == "release_operator_approval"
  and .approval_version == 1
  and .owner_lane == "release_operator"
  and .approved_release_action == "signed_notarized_stapled_public_distribution"
  and (["sign_hepta_native_app","notarize_hepta_native_app","staple_hepta_native_app","write_public_distribution_artifact"] - .approval_scope | length) == 0
  and .operator_approval_recorded == true
  and (.operator_identity_hash | test("^[0-9a-f]{64}$"))
  and .source_evidence.dry_run_manifest_sha256 == $dry[0].dry_run_manifest_sha256
  and .source_evidence.evidence_archive_sha256 == $archive[0].archive_sha256
  and .source_evidence.unsigned_app_bundle_sha256 == $dry[0].release_candidate.unsigned_app_bundle_sha256
  and (.source_evidence.authorization_message_id | test("^telegram:[0-9]+/[0-9]+$"))
  and (.approved_at | length) > 0
  and .release_target.bundle_identifier == $distribution[0].package_metadata.bundle_identifier
  and .release_target.bundle_name == $distribution[0].package_metadata.bundle_name
  and .release_target.bundle_executable == $distribution[0].package_metadata.bundle_executable
  and .post_approval_requirements.signed_notarized_stapled_artifact_gate_required == true
  and .post_approval_requirements.no_release_claim_from_approval_only == true
  and .claim_boundary.release_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .side_effects.external_mutation == false
' "$APPROVAL_TEST_DIR/self-reported-complete.json" >/dev/null
if run_approval_gate "$APPROVAL_TEST_DIR" "$APPROVAL_TEST_DIR/self-reported-complete.json" \
  >"$APPROVAL_TEST_DIR/self-reported.stdout" 2>"$APPROVAL_TEST_DIR/self-reported.stderr"; then
  printf 'release approval intake accepted a complete but self-reported authorization tuple\n' >&2
  exit 1
fi
jq -e '
  .status == "failed"
  and .release_approval_intake_gate_ready == false
  and .release_approval_state.release_approval_present == true
  and .release_approval_state.release_approval_valid == false
  and .release_approval_state.independent_approval_verifier_ready == false
  and .release_approval_state.self_reported_approval_can_authorize_release == false
  and .claim_boundary.release_approval_claim_ready == false
  and (.approval_blockers | index("independent_release_approval_verifier_unavailable") != null)
' "$APPROVAL_TEST_DIR/approval-report.json" >/dev/null
[[ -s "$APPROVAL_TEST_DIR/approval-intake/release-approval-input.captured.json" ]]
[[ ! -e "$APPROVAL_TEST_DIR/approval-intake/release-approval-input.accepted.json" ]]
if [[ -e "$TEST_DIR/path-shim-used" ]]; then
  printf 'release approval gate executed a PATH shim\n' >&2
  exit 1
fi

FORGED_V3_DIR="$(make_forged_case forged-v3-tar-and-logs)"
assert_invalid "$FORGED_V3_DIR"
jq -e '
  .release_artifact_state.present_artifact_branch_supported == false
  and .release_artifact_state.independent_approval_verifier_contract_ready == false
  and (.release_artifact_blockers | index("release_artifact_present_branch_unsupported_without_independent_approval_verifier") != null)
' "$FORGED_V3_DIR/intake-report.json" >/dev/null

HARDLINK_DIR="$(make_forged_case evidence-hardlink-single-link-rejected)"
ln "$HARDLINK_DIR/evidence/codesign-verify-app.log" "$HARDLINK_DIR/external-hardlink-alias.log"
if run_gate "$HARDLINK_DIR" >"$HARDLINK_DIR/stdout" 2>"$HARDLINK_DIR/stderr"; then
  printf 'release artifact intake accepted hardlinked evidence\n' >&2
  exit 1
fi
jq -e '
  .all_evidence_valid == false
  and .referenced_paths_absolute_and_unique == false
  and (.blockers | index("referenced_evidence_paths_or_inodes_not_absolute_unique_single_link") != null)
' "$HARDLINK_DIR/intake/release-artifact-readback.json" >/dev/null
jq -e '
  .release_artifact_state.release_artifact_valid == false
  and .release_artifact_state.referenced_paths_absolute_and_unique == false
  and .claim_boundary.release_artifact_claim_ready == false
' "$HARDLINK_DIR/intake-report.json" >/dev/null
[[ ! -e "$HARDLINK_DIR/intake/release-artifact-input.accepted.json" ]]

NOFOLLOW_DIR="$(make_forged_case source-contents-signature-symlink-nofollow-rejected)"
mkdir -p "$NOFOLLOW_DIR/external-signature"
printf 'external operator sentinel\n' >"$NOFOLLOW_DIR/external-signature/CodeResources"
NOFOLLOW_SENTINEL_SHA="$(sha256 "$NOFOLLOW_DIR/external-signature/CodeResources")"
ln -s "$NOFOLLOW_DIR/external-signature" "$NOFOLLOW_DIR/source/Hepta.app/Contents/_CodeSignature"
if run_gate "$NOFOLLOW_DIR" >"$NOFOLLOW_DIR/stdout" 2>"$NOFOLLOW_DIR/stderr"; then
  printf 'release artifact intake accepted a source Contents signature symlink\n' >&2
  exit 1
fi
jq -e '
  .all_evidence_valid == false
  and .source_app.tree_nofollow_safe == false
  and .normalized_bundle_equivalence.source_normalized == false
  and (.blockers | index("source_app_path_or_tree_not_absolute_canonical_nofollow_safe_directory") != null)
' "$NOFOLLOW_DIR/intake/release-artifact-readback.json" >/dev/null
[[ "$(sha256 "$NOFOLLOW_DIR/external-signature/CodeResources")" == "$NOFOLLOW_SENTINEL_SHA" ]]
[[ ! -e "$NOFOLLOW_DIR/intake/release-artifact-input.accepted.json" ]]

ADHOC_EQUAL_DIR="$(make_forged_case adhoc-premount-trust-rejected adhoc_equal)"
assert_premount_trust_rejected "$ADHOC_EQUAL_DIR" true

REPORT_IN_INTAKE_ROOT="$TEST_DIR/path-report-in-intake-root"
assert_path_rejected report-inside-intake \
  "$REPORT_IN_INTAKE_ROOT/intake/report.json" \
  "$REPORT_IN_INTAKE_ROOT/intake" "" \
  'release artifact report and intake paths must not overlap'

INPUT_REPORT_SHA="$(sha256 "$FORGED_V3_DIR/release-receipt.json")"
assert_path_rejected input-equals-report \
  "$FORGED_V3_DIR/release-receipt.json" \
  "$TEST_DIR/path-input-equals-report/intake" \
  "$FORGED_V3_DIR/release-receipt.json" \
  'release artifact input must not overlap report or intake paths'
[[ "$(sha256 "$FORGED_V3_DIR/release-receipt.json")" == "$INPUT_REPORT_SHA" ]]

SYMLINK_INTAKE_ROOT="$TEST_DIR/path-symlink-intake-root"
mkdir -p "$SYMLINK_INTAKE_ROOT/real-intake"
printf 'operator-owned\n' >"$SYMLINK_INTAKE_ROOT/real-intake/operator-owned.txt"
ln -s "$SYMLINK_INTAKE_ROOT/real-intake" "$SYMLINK_INTAKE_ROOT/intake-link"
assert_path_rejected symlink-intake \
  "$SYMLINK_INTAKE_ROOT/report.json" \
  "$SYMLINK_INTAKE_ROOT/intake-link" "" \
  'intake path contains a symlinked component'
[[ "$(/bin/cat "$SYMLINK_INTAKE_ROOT/real-intake/operator-owned.txt")" == 'operator-owned' ]]

INPUT_IN_INTAKE_ROOT="$TEST_DIR/path-input-in-intake-root"
mkdir -p "$INPUT_IN_INTAKE_ROOT/intake"
/bin/cp "$FORGED_V3_DIR/release-receipt.json" "$INPUT_IN_INTAKE_ROOT/intake/input.json"
INPUT_IN_INTAKE_SHA="$(sha256 "$INPUT_IN_INTAKE_ROOT/intake/input.json")"
assert_path_rejected input-inside-intake \
  "$INPUT_IN_INTAKE_ROOT/report.json" \
  "$INPUT_IN_INTAKE_ROOT/intake" \
  "$INPUT_IN_INTAKE_ROOT/intake/input.json" \
  'release artifact input must not overlap report or intake paths'
[[ "$(sha256 "$INPUT_IN_INTAKE_ROOT/intake/input.json")" == "$INPUT_IN_INTAKE_SHA" ]]

FIXED_SYMLINK_ROOT="$TEST_DIR/path-fixed-output-symlink-root"
mkdir -p "$FIXED_SYMLINK_ROOT/intake"
printf 'operator-owned\n' >"$FIXED_SYMLINK_ROOT/operator-owned.txt"
ln -s "$FIXED_SYMLINK_ROOT/operator-owned.txt" "$FIXED_SYMLINK_ROOT/intake/release-artifact-readback.json"
assert_path_rejected fixed-output-symlink \
  "$FIXED_SYMLINK_ROOT/report.json" \
  "$FIXED_SYMLINK_ROOT/intake" "" \
  'release artifact fixed output is not a safe regular-file target'
[[ "$(/bin/cat "$FIXED_SYMLINK_ROOT/operator-owned.txt")" == 'operator-owned' ]]

if [[ "${HEPTA_UI_RELEASE_ARTIFACT_INTAKE_V3_SKIP_ROUNDTRIP:-0}" != 1 ]]; then
  env \
    HEPTA_UI_PRODUCT_READINESS_DIR="$WAITING_DIR/readiness" \
    HEPTA_UI_RELEASE_ARTIFACT_INPUT_PATH="" \
    ./scripts/hepta-ui-release-artifact-intake-gate.sh >/dev/null
  env \
    HEPTA_UI_PRODUCT_READINESS_DIR="$WAITING_DIR/readiness" \
    HEPTA_UI_RELEASE_ARTIFACT_ROUNDTRIP_REPORT_PATH="$WAITING_DIR/readiness/ui-release-artifact-roundtrip-gate.json" \
    HEPTA_UI_RELEASE_ARTIFACT_ROUNDTRIP_DIR="$WAITING_DIR/readiness/release-artifact-roundtrip" \
    ./scripts/hepta-ui-release-artifact-roundtrip-gate.sh >/dev/null
  jq -e '
    .status == "ready"
    and .roundtrip_version == 3
    and .roundtrip_artifact_source_mode == "waiting_for_real_v3_artifact"
    and .source_alignment.present_branch_release_artifact_valid == false
    and .source_alignment.present_artifact_branch_supported == false
    and .source_alignment.independent_approval_verifier_contract_ready == false
    and .source_alignment.legacy_simulated_artifact_rejected == true
    and .source_alignment.v3_valid_branch_selftest_ready == true
    and .claim_boundary.simulated_release_artifact_branch_ready == false
    and (.release_artifact_blockers | index("release_artifact_present_branch_unsupported_without_independent_approval_verifier") != null)
  ' "$WAITING_DIR/readiness/ui-release-artifact-roundtrip-gate.json" >/dev/null
fi

printf 'forged_present_public_tuple_rejected_by_artifact_intake_and_roundtrip: PASS\n'
printf 'hepta-ui release artifact intake v3 self-test: PASS\n'
