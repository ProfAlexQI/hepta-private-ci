#!/bin/bash -p
set +x
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
PATH="/usr/bin:/bin:/usr/sbin:/sbin"; export PATH; umask 077
PROFILE="release"; if [[ "${1:-}" == "--profile" ]]; then PROFILE="${2:-}"; shift 2; fi
case "$PROFILE" in release) STEM=release;; device) STEM=device-lab;; accessibility) STEM=accessibility;; *) echo "invalid self-test profile" >&2; exit 64;; esac
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd -P)"; VERIFIER="$ROOT_DIR/scripts/hepta-ui-$STEM-verifier-v1"; TMP_ROOT="$(cd "${TMPDIR:-/tmp}" && pwd -P)"; TEST_DIR="$(mktemp -d "$TMP_ROOT/hepta-ui-$STEM-verifier-self-test.XXXXXX")"; TEST_DIR="$(cd "$TEST_DIR" && pwd -P)"; trap '/bin/rm -rf "$TEST_DIR"' EXIT
fail(){ echo "hepta-ui-$STEM-verifier self-test failed: $1" >&2; exit 1; }; sha(){ /usr/bin/shasum -a 256 "$1"|/usr/bin/awk '{print $1}'; }; bytes(){ /usr/bin/wc -c <"$1"|/usr/bin/tr -d ' '; }
HEAD="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"; TREE="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"; FP="cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"; PRODUCER="hepta-$STEM-lab/attestor-v1"; ARTIFACT="$TEST_DIR/evidence.bin"; RECEIPT="$TEST_DIR/attestation.json"; printf 'synthetic read-only verifier fixture\n' >"$ARTIFACT"
case "$PROFILE" in
  release) INPUT_KIND=hepta-ui-release-attestation-v1; ARTIFACT_KIND=macos_dmg; CAP=public_distribution_ready; CHECKS='{"developer_id_signed":true,"signature_valid":true,"hardened_runtime":true,"sealed_resources_valid":true,"notarized":true,"stapled":true,"stapler_valid":true,"gatekeeper_accepted":true,"public_distribution_authorized":true,"public_upload_performed":false,"notary_status":"Accepted","signing_identity":"Developer ID Application: Hepta Test (TEAMID1234)","team_identifier":"TEAMID1234","notarization_submission_id":"11111111-2222-4333-8444-555555555555"}' ;;
  device) INPUT_KIND=hepta-ui-device-lab-attestation-v1; ARTIFACT_KIND=device_lab_evidence_bundle; CAP=real_device_lab_ready; CHECKS='{"ios_real_device":true,"android_real_device":true,"app_install_verified":true,"cold_launch_verified":true,"foreground_verified":true,"authenticated_workflow_verified":true,"background_resume_verified":true,"rotation_verified":true,"software_keyboard_verified":true,"safe_area_or_insets_verified":true,"rtl_verified":true,"text_scale_verified":true,"performance_budget_verified":true,"secure_credential_storage_verified":true,"crash_free":true,"simulators_or_emulators":false,"ios_device_identifier_sha256":"1111111111111111111111111111111111111111111111111111111111111111","android_device_identifier_sha256":"2222222222222222222222222222222222222222222222222222222222222222","ios_model":"iPhone 16 Pro","ios_os_version":"18.5","android_model":"Pixel 9 Pro","android_os_version":"15"}' ;;
  accessibility) INPUT_KIND=hepta-ui-accessibility-attestation-v1; ARTIFACT_KIND=accessibility_evidence_bundle; CAP=accessibility_ready; CHECKS='{"voiceover_real_device":true,"talkback_real_device":true,"services_enabled_during_audit":true,"settings_baseline_captured":true,"settings_restored":true,"focus_order_verified":true,"all_actionable_controls_reachable":true,"roles_states_values_verified":true,"dynamic_updates_announced":true,"modal_focus_contained":true,"no_focus_trap":true,"text_scaling_verified":true,"rtl_verified":true,"contrast_verified":true,"reduced_motion_verified":true,"voiceover_service":"VoiceOver","talkback_service":"TalkBack","voiceover_device_identifier_sha256":"3333333333333333333333333333333333333333333333333333333333333333","talkback_device_identifier_sha256":"4444444444444444444444444444444444444444444444444444444444444444","semantic_node_count":40,"actionable_control_count":10,"labeled_actionable_control_count":10,"unlabeled_actionable_control_count":0,"duplicate_actionable_label_count":0,"blocking_issue_count":0}' ;;
esac
/usr/bin/jq -n --arg kind "$INPUT_KIND" --arg producer "$PRODUCER" --arg head "$HEAD" --arg tree "$TREE" --arg fp "$FP" --arg ak "$ARTIFACT_KIND" --arg path "$ARTIFACT" --arg hash "$(sha "$ARTIFACT")" --argjson size "$(bytes "$ARTIFACT")" --argjson checks "$CHECKS" '{schema_version:1,kind:$kind,producer:$producer,status:"ready",source_binding:{schema_version:1,kind:"hepta-ui-source-binding",head:$head,head_tree:$tree,source_fingerprint:$fp,worktree_clean:true,repository_worktree_clean:true},source_stable_during_run:true,artifact:{kind:$ak,path:$path,sha256:$hash,size_bytes:$size},checks:$checks}' >"$RECEIPT"
ARGS=(--receipt "$RECEIPT" --artifact "$ARTIFACT" --expected-producer "$PRODUCER" --source-head "$HEAD" --source-tree "$TREE" --source-fingerprint "$FP"); [[ "$PROFILE" == release ]] && ARGS+=(--expected-signing-identity 'Developer ID Application: Hepta Test (TEAMID1234)' --expected-team-id TEAMID1234)
R0="$(sha "$RECEIPT")"; A0="$(sha "$ARTIFACT")"; "$VERIFIER" "${ARGS[@]}" >"$TEST_DIR/ready.json"
/usr/bin/jq -e --arg cap "$CAP" --arg r "$RECEIPT" --arg a "$ARTIFACT" --arg rh "$R0" --arg ah "$A0" '.schema_version==1 and .status=="ready" and .producer==("scripts/hepta-ui-'"$STEM"'-verifier-v1") and .independent_promotion_verifier_ready==true and .input_receipt.path==$r and .input_receipt.sha256==$rh and .artifact.path==$a and .artifact.sha256==$ah and .[$cap]==true and (.verifier_actions|all(.==false))' "$TEST_DIR/ready.json" >/dev/null || fail "positive output invalid"
[[ "$(sha "$RECEIPT")" == "$R0" && "$(sha "$ARTIFACT")" == "$A0" ]] || fail "positive inputs mutated"
NEG=0
reject_args(){ local name="$1"; shift; if "$VERIFIER" "$@" >"$TEST_DIR/$name.out" 2>"$TEST_DIR/$name.err"; then fail "$name accepted"; fi; [[ ! -s "$TEST_DIR/$name.out" ]] || fail "$name emitted stdout"; NEG=$((NEG+1)); }
reject_receipt(){ local name="$1" file="$2" artifact="${3:-$ARTIFACT}"; local args=(--receipt "$file" --artifact "$artifact" --expected-producer "$PRODUCER" --source-head "$HEAD" --source-tree "$TREE" --source-fingerprint "$FP"); [[ "$PROFILE" == release ]] && args+=(--expected-signing-identity 'Developer ID Application: Hepta Test (TEAMID1234)' --expected-team-id TEAMID1234); reject_args "$name" "${args[@]}"; }
mutate(){ local name="$1" filter="$2" file="$TEST_DIR/$name.json"; /usr/bin/jq "$filter" "$RECEIPT" >"$file"; reject_receipt "$name" "$file"; }
while IFS='|' read -r name filter; do mutate "$name" "$filter"; done <<'EOF'
schema|.schema_version=2
kind|.kind="wrong"
producer|.producer="wrong"
status|.status="not_ready"
top_extra|.extra=true
source_extra|.source_binding.extra=true
source_schema|.source_binding.schema_version=2
source_kind|.source_binding.kind="wrong"
source_head|.source_binding.head=("d"*40)
source_tree|.source_binding.head_tree=("d"*40)
source_fingerprint|.source_binding.source_fingerprint=("d"*64)
source_dirty|.source_binding.worktree_clean=false
repository_dirty|.source_binding.repository_worktree_clean=false
source_unstable|.source_stable_during_run=false
artifact_extra|.artifact.extra=true
artifact_kind|.artifact.kind="wrong"
artifact_path|.artifact.path="/tmp/wrong"
artifact_hash|.artifact.sha256=("d"*64)
artifact_size|.artifact.size_bytes+=1
checks_extra|.checks.extra=true
EOF
while IFS=$'\t' read -r key type value; do case "$type" in boolean) replacement=$([[ "$value" == true ]] && echo false || echo true);; string) replacement='""';; number) replacement=$([[ "$value" == 0 ]] && echo 1 || echo 0);; esac; mutate "check_$key" ".checks[\"$key\"]=$replacement"; done < <(/usr/bin/jq -r '.checks|to_entries[]|[.key,(.value|type),(.value|tostring)]|@tsv' "$RECEIPT")
reject_args missing_args; reject_args relative_receipt --receipt relative --artifact "$ARTIFACT" --expected-producer "$PRODUCER" --source-head "$HEAD" --source-tree "$TREE" --source-fingerprint "$FP"
/bin/ln -s "$RECEIPT" "$TEST_DIR/receipt-link"; reject_receipt receipt_symlink "$TEST_DIR/receipt-link"; /bin/ln -s "$ARTIFACT" "$TEST_DIR/artifact-link"; reject_receipt artifact_symlink "$RECEIPT" "$TEST_DIR/artifact-link"
/bin/ln -s "$TEST_DIR" "$TEST_DIR/parent-link"; reject_receipt parent_symlink "$TEST_DIR/parent-link/attestation.json"; printf '{bad\n' >"$TEST_DIR/bad.json"; reject_receipt malformed_json "$TEST_DIR/bad.json"; printf '{"schema_version":1,"schema_version":1}\n' >"$TEST_DIR/duplicate.json"; reject_receipt duplicate_key "$TEST_DIR/duplicate.json"; reject_receipt same_file "$RECEIPT" "$RECEIPT"; : >"$TEST_DIR/empty"; reject_receipt empty_artifact "$RECEIPT" "$TEST_DIR/empty"
cat >"$TEST_DIR/hook.sh" <<EOF
#!/bin/bash
/usr/bin/touch "$TEST_DIR/bash-hook-ran"
EOF
printf 'File.write(%q{%s},%q{ran})\n' "$TEST_DIR/ruby-hook-ran" >"$TEST_DIR/hook.rb"; chmod 700 "$TEST_DIR/hook.sh"; BASH_ENV="$TEST_DIR/hook.sh" RUBYOPT="-r$TEST_DIR/hook.rb" "$VERIFIER" "${ARGS[@]}" >/dev/null; [[ ! -e "$TEST_DIR/bash-hook-ran" && ! -e "$TEST_DIR/ruby-hook-ran" ]] || fail "startup hook ran"
for path in /usr/bin/codesign /usr/bin/xcrun /usr/bin/adb /usr/bin/simctl /usr/bin/defaults /usr/bin/curl /usr/bin/scp; do ! /usr/bin/grep -F "$path" "$ROOT_DIR/scripts/lib/hepta-ui-promotion-verifier-v1.sh" >/dev/null || fail "forbidden command $path"; done
[[ "$NEG" -ge 35 ]] || fail "negative matrix incomplete"; printf 'hepta-ui-%s-verifier self-test passed (%s negative cases)\n' "$STEM" "$NEG"
