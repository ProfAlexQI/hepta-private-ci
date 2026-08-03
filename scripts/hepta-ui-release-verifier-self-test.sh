#!/bin/bash -p
set +x
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
PATH="/usr/bin:/bin:/usr/sbin:/sbin"; export PATH; umask 077

case "${0##*/}" in
  hepta-ui-device-lab-verifier-self-test.sh) PROFILE="device" ;;
  hepta-ui-accessibility-verifier-self-test.sh) PROFILE="accessibility" ;;
  hepta-ui-matrix-live-verifier-self-test.sh) PROFILE="matrix" ;;
  hepta-ui-bridge-live-verifier-self-test.sh) PROFILE="bridge" ;;
  *) PROFILE="" ;;
esac
if [[ -n "$PROFILE" ]]; then
  [[ $# == 0 ]] || { echo "profile-specific self-test entrypoints take no arguments" >&2; exit 64; }
else
  PROFILE="release"
  if [[ "${1:-}" == "--profile" ]]; then
    [[ $# == 2 && -n "${2:-}" ]] || { echo "--profile requires exactly one value" >&2; exit 64; }
    PROFILE="$2"
    shift 2
  else
    [[ $# == 0 ]] || { echo "invalid self-test arguments" >&2; exit 64; }
  fi
fi
case "$PROFILE" in
  release) STEM=release ;;
  device) STEM=device-lab ;;
  accessibility) STEM=accessibility ;;
  matrix) STEM=matrix-live ;;
  bridge) STEM=bridge-live ;;
  *) echo "invalid self-test profile" >&2; exit 64 ;;
esac

ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd -P)"
VERIFIER="$ROOT_DIR/scripts/hepta-ui-$STEM-verifier-v1"
REHASHER="$ROOT_DIR/scripts/lib/hepta-ui-promotion-verifier-v1.sh"
TMP_ROOT="$(cd "${TMPDIR:-/tmp}" && pwd -P)"
TEST_DIR="$(mktemp -d "$TMP_ROOT/hepta-ui-$STEM-verifier-self-test.XXXXXX")"
TEST_DIR="$(cd "$TEST_DIR" && pwd -P)"
trap '/bin/rm -rf "$TEST_DIR"' EXIT

fail(){ echo "hepta-ui-$STEM-verifier self-test failed: $1" >&2; exit 1; }
trap 'rc=$?; echo "hepta-ui-$STEM-verifier self-test aborted at line $LINENO (status $rc)" >&2; exit "$rc"' ERR
sha(){ /usr/bin/shasum -a 256 "$1" | /usr/bin/awk '{print $1}'; }
bytes(){ /usr/bin/wc -c <"$1" | /usr/bin/tr -d ' '; }
sign(){ /usr/bin/openssl dgst -sha256 -sign "$2" -out "$3" "$1" >/dev/null 2>&1; }

HEAD="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
TREE="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
FP="cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
RUN="dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
POLICY_SHA="abababababababababababababababababababababababababababababababab"
NOW_MS=$(( $(/bin/date +%s) * 1000 ))
LOGIN_MS=$(( NOW_MS - 5000 ))
WORKFLOW_MS=$(( NOW_MS - 4000 ))
BRIDGE_MS=$(( NOW_MS - 3000 ))
LOGOUT_MS=$(( NOW_MS - 2000 ))
ATTESTED_MS=$(( NOW_MS - 1000 ))
EXPIRES_MS=$(( NOW_MS + 300000 ))

PRODUCER="hepta-$STEM-lab/attestor-v1"
PRIVATE_KEY="$TEST_DIR/attestor-private.pem"
PUBLIC_KEY="$TEST_DIR/attestor-public.pem"
/usr/bin/openssl genrsa -out "$PRIVATE_KEY" 2048 >/dev/null 2>&1
/usr/bin/openssl rsa -in "$PRIVATE_KEY" -pubout -out "$PUBLIC_KEY" >/dev/null 2>&1
PUBLIC_KEY_SHA="$(sha "$PUBLIC_KEY")"

ARTIFACT="$TEST_DIR/evidence-manifest.json"
RECEIPT="$TEST_DIR/attestation.json"
SIGNATURE="$TEST_DIR/attestation.sig"

case "$PROFILE" in
  release)
    INPUT_KIND=hepta-ui-release-attestation-v1; EVIDENCE_KIND=macos_dmg; CAP=public_distribution_ready
    ENTRIES='[{"role":"macos_dmg","sha256":"1010101010101010101010101010101010101010101010101010101010101010","size_bytes":4096,"media_type":"application/x-apple-diskimage"}]'
    CHECKS='{"developer_id_signed":true,"signature_valid":true,"hardened_runtime":true,"sealed_resources_valid":true,"notarized":true,"stapled":true,"stapler_valid":true,"gatekeeper_accepted":true,"public_distribution_authorized":true,"public_upload_performed":false,"notary_status":"Accepted","signing_identity":"Developer ID Application: Hepta Test (TEAMID1234)","team_identifier":"TEAMID1234","notarization_submission_id":"11111111-2222-4333-8444-555555555555"}'
    ;;
  device)
    INPUT_KIND=hepta-ui-device-lab-attestation-v1; EVIDENCE_KIND=device_lab_evidence_bundle; CAP=real_device_lab_ready
    ENTRIES='[{"role":"ios_device_audit","sha256":"1111111111111111111111111111111111111111111111111111111111111111","size_bytes":8192,"media_type":"application/json"},{"role":"android_device_audit","sha256":"2222222222222222222222222222222222222222222222222222222222222222","size_bytes":8192,"media_type":"application/json"}]'
    CHECKS='{"ios_real_device":true,"android_real_device":true,"app_install_verified":true,"cold_launch_verified":true,"foreground_verified":true,"authenticated_workflow_verified":true,"background_resume_verified":true,"rotation_verified":true,"software_keyboard_verified":true,"safe_area_or_insets_verified":true,"rtl_verified":true,"text_scale_verified":true,"performance_budget_verified":true,"secure_credential_storage_verified":true,"crash_free":true,"simulators_or_emulators":false,"ios_device_identifier_sha256":"3131313131313131313131313131313131313131313131313131313131313131","android_device_identifier_sha256":"3232323232323232323232323232323232323232323232323232323232323232","ios_model":"iPhone 16 Pro","ios_os_version":"18.5","android_model":"Pixel 9 Pro","android_os_version":"15"}'
    ;;
  accessibility)
    INPUT_KIND=hepta-ui-accessibility-attestation-v1; EVIDENCE_KIND=accessibility_evidence_bundle; CAP=accessibility_ready
    ENTRIES='[{"role":"voiceover_audit","sha256":"3333333333333333333333333333333333333333333333333333333333333333","size_bytes":8192,"media_type":"application/json"},{"role":"talkback_audit","sha256":"4444444444444444444444444444444444444444444444444444444444444444","size_bytes":8192,"media_type":"application/json"}]'
    CHECKS='{"voiceover_real_device":true,"talkback_real_device":true,"services_enabled_during_audit":true,"settings_baseline_captured":true,"settings_restored":true,"focus_order_verified":true,"all_actionable_controls_reachable":true,"roles_states_values_verified":true,"dynamic_updates_announced":true,"modal_focus_contained":true,"no_focus_trap":true,"text_scaling_verified":true,"rtl_verified":true,"contrast_verified":true,"reduced_motion_verified":true,"voiceover_service":"VoiceOver","talkback_service":"TalkBack","voiceover_device_identifier_sha256":"5353535353535353535353535353535353535353535353535353535353535353","talkback_device_identifier_sha256":"5454545454545454545454545454545454545454545454545454545454545454","semantic_node_count":40,"actionable_control_count":10,"labeled_actionable_control_count":10,"unlabeled_actionable_control_count":0,"duplicate_actionable_label_count":0,"blocking_issue_count":0}'
    ;;
  matrix)
    INPUT_KIND=hepta-ui-matrix-live-attestation-v1; EVIDENCE_KIND=matrix_live_evidence_bundle; CAP=matrix_live_ready
    ENTRIES='[{"role":"matrix_workflow_audit","sha256":"5555555555555555555555555555555555555555555555555555555555555555","size_bytes":8192,"media_type":"application/json"}]'
    CHECKS="{\"real_homeserver\":true,\"authenticated_session\":true,\"login_success\":true,\"room_list_loaded\":true,\"timeline_loaded\":true,\"encrypted_room_verified\":true,\"message_send_roundtrip_verified\":true,\"logout_verified\":true,\"credentials_redacted\":true,\"fixture_or_mock_absent\":true,\"credentials_embedded\":false,\"synthetic_server\":false,\"mutation_outside_test_room\":false,\"protocol\":\"matrix-client-server-api\",\"homeserver_origin_sha256\":\"6565656565656565656565656565656565656565656565656565656565656565\",\"session_identifier_sha256\":\"6666666666666666666666666666666666666666666666666666666666666666\",\"test_room_identifier_sha256\":\"6767676767676767676767676767676767676767676767676767676767676767\",\"timeline_event_count\":3,\"login_observed_unix_ms\":$LOGIN_MS,\"authenticated_workflow_observed_unix_ms\":$WORKFLOW_MS,\"logout_observed_unix_ms\":$LOGOUT_MS}"
    ;;
  bridge)
    INPUT_KIND=hepta-ui-bridge-live-attestation-v1; EVIDENCE_KIND=hepta_bridge_live_evidence_bundle; CAP=hepta_live_bridge_ready
    ENTRIES='[{"role":"bridge_get_audit","sha256":"7777777777777777777777777777777777777777777777777777777777777777","size_bytes":4096,"media_type":"application/json"}]'
    ;;
esac

make_evidence_entries(){
  local prefix="$1" role media path
  shift
  while [[ $# -gt 0 ]]; do
    role="$1"; media="$2"; shift 2
    if [[ "$media" == application/json ]]; then
      path="$TEST_DIR/$prefix-$role.json"
      /usr/bin/jq -n --arg role "$role" '{schema_version:1,kind:"hepta-ui-verifier-self-test-evidence",role:$role,credentials_redacted:true,secrets_included:false}' >"$path"
    else
      path="$TEST_DIR/$prefix-$role.dmg"
      printf 'synthetic signed DMG evidence for verifier self-test\n' >"$path"
    fi
    /usr/bin/jq -n --arg role "$role" --arg path "$path" --arg sha "$(sha "$path")" \
      --argjson size "$(bytes "$path")" --arg media "$media" \
      '{role:$role,path:$path,sha256:$sha,size_bytes:$size,media_type:$media}'
  done | /usr/bin/jq -s '.'
}

case "$PROFILE" in
  release) ENTRIES="$(make_evidence_entries primary macos_dmg application/x-apple-diskimage)" ;;
  device) ENTRIES="$(make_evidence_entries primary ios_device_audit application/json android_device_audit application/json)" ;;
  accessibility) ENTRIES="$(make_evidence_entries primary voiceover_audit application/json talkback_audit application/json)" ;;
  matrix) ENTRIES="$(make_evidence_entries primary matrix_workflow_audit application/json)" ;;
  bridge) ENTRIES="$(make_evidence_entries primary bridge_get_audit application/json)" ;;
esac

make_manifest(){
  local output="$1" profile_stem="$2" evidence_kind="$3" producer="$4" run="$5" entries="$6"
  /usr/bin/jq -n \
    --arg profile "$profile_stem" --arg evidence "$evidence_kind" --arg producer "$producer" --arg run "$run" \
    --arg head "$HEAD" --arg tree "$TREE" --arg fp "$FP" --argjson entries "$entries" \
    '{schema_version:1,kind:"hepta-ui-evidence-manifest-v1",profile:$profile,evidence_kind:$evidence,producer:$producer,source_binding:{schema_version:1,kind:"hepta-ui-source-binding",head:$head,head_tree:$tree,source_fingerprint:$fp,worktree_clean:true,repository_worktree_clean:true},run_identifier_sha256:$run,entries:$entries,redaction:{credentials_redacted:true,secrets_included:false,tokens_included:false,passwords_included:false,raw_payloads_included:false}}' >"$output"
}

make_receipt(){
  local output="$1" kind="$2" producer="$3" manifest="$4" evidence_kind="$5" checks="$6" run="$7" attested="$8" expires="$9"
  /usr/bin/jq -n \
    --arg kind "$kind" --arg producer "$producer" --arg head "$HEAD" --arg tree "$TREE" --arg fp "$FP" \
    --arg path "$manifest" --arg hash "$(sha "$manifest")" --argjson size "$(bytes "$manifest")" \
    --arg evidence "$evidence_kind" --arg run "$run" --argjson attested "$attested" --argjson expires "$expires" --argjson checks "$checks" \
    '{schema_version:1,kind:$kind,producer:$producer,status:"ready",source_binding:{schema_version:1,kind:"hepta-ui-source-binding",head:$head,head_tree:$tree,source_fingerprint:$fp,worktree_clean:true,repository_worktree_clean:true},source_stable_during_run:true,attested_at_unix_ms:$attested,expires_at_unix_ms:$expires,run_identifier_sha256:$run,artifact:{kind:"hepta-ui-evidence-manifest-v1",evidence_kind:$evidence,path:$path,sha256:$hash,size_bytes:$size},checks:$checks}' >"$output"
}

MATRIX_RECEIPT=""; MATRIX_ARTIFACT=""; MATRIX_SIGNATURE=""; MATRIX_PUBLIC_KEY=""; MATRIX_PUBLIC_KEY_SHA=""; MATRIX_PRODUCER=""
if [[ "$PROFILE" == bridge ]]; then
  MATRIX_PRODUCER="hepta-matrix-live-lab/attestor-v1"
  MATRIX_PRIVATE_KEY="$TEST_DIR/matrix-attestor-private.pem"
  MATRIX_PUBLIC_KEY="$TEST_DIR/matrix-attestor-public.pem"
  /usr/bin/openssl genrsa -out "$MATRIX_PRIVATE_KEY" 2048 >/dev/null 2>&1
  /usr/bin/openssl rsa -in "$MATRIX_PRIVATE_KEY" -pubout -out "$MATRIX_PUBLIC_KEY" >/dev/null 2>&1
  MATRIX_PUBLIC_KEY_SHA="$(sha "$MATRIX_PUBLIC_KEY")"
  MATRIX_ARTIFACT="$TEST_DIR/matrix-evidence-manifest.json"
  MATRIX_RECEIPT="$TEST_DIR/matrix-attestation.json"
  MATRIX_SIGNATURE="$TEST_DIR/matrix-attestation.sig"
  MATRIX_ENTRIES="$(make_evidence_entries parent matrix_workflow_audit application/json)"
  MATRIX_CHECKS="{\"real_homeserver\":true,\"authenticated_session\":true,\"login_success\":true,\"room_list_loaded\":true,\"timeline_loaded\":true,\"encrypted_room_verified\":true,\"message_send_roundtrip_verified\":true,\"logout_verified\":true,\"credentials_redacted\":true,\"fixture_or_mock_absent\":true,\"credentials_embedded\":false,\"synthetic_server\":false,\"mutation_outside_test_room\":false,\"protocol\":\"matrix-client-server-api\",\"homeserver_origin_sha256\":\"8686868686868686868686868686868686868686868686868686868686868686\",\"session_identifier_sha256\":\"6666666666666666666666666666666666666666666666666666666666666666\",\"test_room_identifier_sha256\":\"8787878787878787878787878787878787878787878787878787878787878787\",\"timeline_event_count\":3,\"login_observed_unix_ms\":$LOGIN_MS,\"authenticated_workflow_observed_unix_ms\":$WORKFLOW_MS,\"logout_observed_unix_ms\":$LOGOUT_MS}"
  make_manifest "$MATRIX_ARTIFACT" matrix-live matrix_live_evidence_bundle "$MATRIX_PRODUCER" "$RUN" "$MATRIX_ENTRIES"
  make_receipt "$MATRIX_RECEIPT" hepta-ui-matrix-live-attestation-v1 "$MATRIX_PRODUCER" "$MATRIX_ARTIFACT" matrix_live_evidence_bundle "$MATRIX_CHECKS" "$RUN" "$ATTESTED_MS" "$EXPIRES_MS"
  sign "$MATRIX_RECEIPT" "$MATRIX_PRIVATE_KEY" "$MATRIX_SIGNATURE"
  CHECKS="{\"canonical_loopback_endpoint\":true,\"exact_get_request\":true,\"http_status_200\":true,\"response_deserialized\":true,\"matrix_session_authenticated\":true,\"explicit_user_opt_in\":true,\"fixture_or_mock_absent\":true,\"run_match\":true,\"session_match\":true,\"correlation_match\":true,\"sequence_match\":true,\"authoritative_origin_valid\":true,\"redaction_valid\":true,\"provenance_valid\":true,\"raw_source_payload_rejected\":true,\"logout_transport_dropped\":true,\"login_failure_transport_dropped\":true,\"subscribe\":false,\"prepare\":false,\"confirm\":false,\"reject\":false,\"cancel\":false,\"provider_invocation\":false,\"channel_delivery\":false,\"cursor_write\":false,\"gateway_mutation\":false,\"external_mutation\":false,\"platform\":\"macos\",\"surface\":\"authenticated_post_login\",\"endpoint\":\"/api/hepta-native-bridge/v1/snapshot\",\"method\":\"GET\",\"content_type\":\"application/json\",\"request_descriptor_sha256\":\"8888888888888888888888888888888888888888888888888888888888888888\",\"response_sha256\":\"8989898989898989898989898989898989898989898989898989898989898989\",\"transport_run_identifier_sha256\":\"$RUN\",\"session_identifier_sha256\":\"6666666666666666666666666666666666666666666666666666666666666666\",\"correlation_identifier_sha256\":\"9090909090909090909090909090909090909090909090909090909090909090\",\"matrix_attestation_sha256\":\"$(sha "$MATRIX_RECEIPT")\",\"request_expected_sequence\":3,\"response_sequence\":3,\"response_byte_count\":64,\"bridge_get_observed_unix_ms\":$BRIDGE_MS,\"mutation_capability_count\":0}"
fi

make_manifest "$ARTIFACT" "$STEM" "$EVIDENCE_KIND" "$PRODUCER" "$RUN" "$ENTRIES"
make_receipt "$RECEIPT" "$INPUT_KIND" "$PRODUCER" "$ARTIFACT" "$EVIDENCE_KIND" "$CHECKS" "$RUN" "$ATTESTED_MS" "$EXPIRES_MS"
sign "$RECEIPT" "$PRIVATE_KEY" "$SIGNATURE"

BASE_ARGS=(--receipt "$RECEIPT" --artifact "$ARTIFACT" --signature "$SIGNATURE" --trusted-public-key "$PUBLIC_KEY" --expected-public-key-sha256 "$PUBLIC_KEY_SHA" --expected-producer "$PRODUCER" --trust-policy-sha256 "$POLICY_SHA" --source-head "$HEAD" --source-tree "$TREE" --source-fingerprint "$FP")
[[ "$PROFILE" == release ]] && BASE_ARGS+=(--expected-signing-identity 'Developer ID Application: Hepta Test (TEAMID1234)' --expected-team-id TEAMID1234)
if [[ "$PROFILE" == bridge ]]; then
  BASE_ARGS+=(--matrix-receipt "$MATRIX_RECEIPT" --matrix-artifact "$MATRIX_ARTIFACT" --matrix-signature "$MATRIX_SIGNATURE" --matrix-trusted-public-key "$MATRIX_PUBLIC_KEY" --expected-matrix-public-key-sha256 "$MATRIX_PUBLIC_KEY_SHA" --expected-matrix-producer "$MATRIX_PRODUCER")
fi

R0="$(sha "$RECEIPT")"; A0="$(sha "$ARTIFACT")"; S0="$(sha "$SIGNATURE")"; K0="$(sha "$PUBLIC_KEY")"
"$VERIFIER" "${BASE_ARGS[@]}" >"$TEST_DIR/ready.json"
/usr/bin/jq -e \
  --arg cap "$CAP" --arg r "$RECEIPT" --arg a "$ARTIFACT" --arg s "$SIGNATURE" --arg k "$PUBLIC_KEY" \
  --arg rh "$R0" --arg ah "$A0" --arg sh "$S0" --arg kh "$K0" --arg policy "$POLICY_SHA" \
  '.schema_version==1 and .status=="ready" and .producer==("scripts/hepta-ui-'"$STEM"'-verifier-v1") and
   .trust_policy.sha256==$policy and .trust_policy.exact_head_blob_required==true and
   .independent_promotion_verifier_ready==true and .input_receipt.path==$r and .input_receipt.sha256==$rh and
   .attestation_signature.algorithm=="RSA-SHA256" and .attestation_signature.path==$s and
   .attestation_signature.sha256==$sh and .attestation_signature.trusted_public_key_path==$k and
   .attestation_signature.trusted_public_key_sha256==$kh and .attestation_signature.signature_verified==true and
   .artifact.path==$a and .artifact.sha256==$ah and .artifact.kind=="hepta-ui-evidence-manifest-v1" and
   .artifact.manifest_valid==true and
   (.artifact.entry_digests|all(.content_verified==true and .redaction_scan_passed==true)) and
   .temporal_binding.freshness_verified==true and .[$cap]==true and
   (.verifier_actions|all(.==false))' "$TEST_DIR/ready.json" >/dev/null || fail "positive output invalid"
if [[ "$PROFILE" == matrix || "$PROFILE" == bridge ]]; then
  /usr/bin/jq -e '.live_chain_binding.sequence_verified==true' "$TEST_DIR/ready.json" >/dev/null || fail "live-chain binding missing"
fi
if [[ "$PROFILE" == bridge ]]; then
  /usr/bin/jq -e --arg parent "$(sha "$MATRIX_RECEIPT")" '.verified_checks.platform=="macos" and .verified_checks.surface=="authenticated_post_login" and .live_chain_binding.matrix_attestation_sha256==$parent and .live_chain_binding.transport_run_identifier_sha256==.live_chain_binding.run_identifier_sha256 and .live_chain_binding.request_expected_sequence==.live_chain_binding.response_sequence and .live_chain_binding.parent_signature_verified==true and .live_chain_binding.session_match_verified==true and .live_chain_binding.run_match_verified==true' "$TEST_DIR/ready.json" >/dev/null || fail "authenticated macOS bridge binding or parent Matrix provenance missing"
fi
[[ "$(sha "$RECEIPT")" == "$R0" && "$(sha "$ARTIFACT")" == "$A0" && "$(sha "$SIGNATURE")" == "$S0" && "$(sha "$PUBLIC_KEY")" == "$K0" ]] || fail "positive inputs mutated"

READY_RECEIPT_SHA="$(sha "$TEST_DIR/ready.json")"
READY_ENTRY_COUNT="$(/usr/bin/jq '.artifact.entry_digests | length' "$TEST_DIR/ready.json")"
"$REHASHER" rehash "$PROFILE" --receipt "$TEST_DIR/ready.json" >"$TEST_DIR/rehash-ready.json"
/usr/bin/jq -e --arg profile "$PROFILE" --arg receipt_sha "$READY_RECEIPT_SHA" --arg manifest_sha "$A0" --argjson entry_count "$READY_ENTRY_COUNT" '
  .schema_version == 1
  and .kind == "hepta-ui-promotion-evidence-rehash-v1"
  and .status == "ready"
  and .profile == $profile
  and .receipt_sha256 == $receipt_sha
  and .manifest_sha256 == $manifest_sha
  and .entry_count == $entry_count
  and (.entry_set_sha256 | test("^[0-9a-f]{64}$"))
  and .nofollow_exact_bytes_verified == true
' "$TEST_DIR/rehash-ready.json" >/dev/null || fail "positive post-verifier evidence rehash invalid"

NEG=0
reject_args(){
  local name="$1"; shift
  if "$VERIFIER" "$@" >"$TEST_DIR/$name.out" 2>"$TEST_DIR/$name.err"; then fail "$name accepted"; fi
  [[ ! -s "$TEST_DIR/$name.out" ]] || fail "$name emitted stdout"
  NEG=$((NEG+1))
}

reject_rehash(){
  local name="$1"
  if "$REHASHER" rehash "$PROFILE" --receipt "$TEST_DIR/ready.json" >"$TEST_DIR/$name.out" 2>"$TEST_DIR/$name.err"; then
    fail "$name accepted"
  fi
  [[ ! -s "$TEST_DIR/$name.out" ]] || fail "$name emitted stdout"
  NEG=$((NEG+1))
}

reject_receipt(){
  local name="$1" receipt="$2" artifact="${3:-$ARTIFACT}" signature="${4:-$SIGNATURE}" key="${5:-$PUBLIC_KEY}" key_sha="${6:-$PUBLIC_KEY_SHA}" producer="${7:-$PRODUCER}"
  local args=(--receipt "$receipt" --artifact "$artifact" --signature "$signature" --trusted-public-key "$key" --expected-public-key-sha256 "$key_sha" --expected-producer "$producer" --trust-policy-sha256 "$POLICY_SHA" --source-head "$HEAD" --source-tree "$TREE" --source-fingerprint "$FP")
  [[ "$PROFILE" == release ]] && args+=(--expected-signing-identity 'Developer ID Application: Hepta Test (TEAMID1234)' --expected-team-id TEAMID1234)
  if [[ "$PROFILE" == bridge ]]; then
    args+=(--matrix-receipt "$MATRIX_RECEIPT" --matrix-artifact "$MATRIX_ARTIFACT" --matrix-signature "$MATRIX_SIGNATURE" --matrix-trusted-public-key "$MATRIX_PUBLIC_KEY" --expected-matrix-public-key-sha256 "$MATRIX_PUBLIC_KEY_SHA" --expected-matrix-producer "$MATRIX_PRODUCER")
  fi
  reject_args "$name" "${args[@]}"
}

mutate_receipt(){
  local name="$1" filter="$2" file="$TEST_DIR/$name.json" sig="$TEST_DIR/$name.sig"
  /usr/bin/jq "$filter" "$RECEIPT" >"$file"
  sign "$file" "$PRIVATE_KEY" "$sig"
  reject_receipt "$name" "$file" "$ARTIFACT" "$sig"
}

reject_manifest(){
  local name="$1" filter="$2" manifest="$TEST_DIR/$name-manifest.json" receipt="$TEST_DIR/$name-receipt.json" sig="$TEST_DIR/$name.sig"
  /usr/bin/jq "$filter" "$ARTIFACT" >"$manifest"
  /usr/bin/jq --arg path "$manifest" --arg hash "$(sha "$manifest")" --argjson size "$(bytes "$manifest")" '.artifact.path=$path | .artifact.sha256=$hash | .artifact.size_bytes=$size' "$RECEIPT" >"$receipt"
  sign "$receipt" "$PRIVATE_KEY" "$sig"
  reject_receipt "$name" "$receipt" "$manifest" "$sig"
}

while IFS='|' read -r name filter; do mutate_receipt "$name" "$filter"; done <<'EOF'
schema|.schema_version=2
kind|.kind="wrong"
producer|.producer="wrong"
status|.status="not_ready"
top_extra|.extra=true
source_extra|.source_binding.extra=true
source_schema|.source_binding.schema_version=2
source_kind|.source_binding.kind="wrong"
source_head|.source_binding.head=("e"*40)
source_tree|.source_binding.head_tree=("e"*40)
source_fingerprint|.source_binding.source_fingerprint=("e"*64)
source_dirty|.source_binding.worktree_clean=false
repository_dirty|.source_binding.repository_worktree_clean=false
source_unstable|.source_stable_during_run=false
run_invalid|.run_identifier_sha256="short"
attested_future|.attested_at_unix_ms+=999999999
expired|.expires_at_unix_ms=1
expiry_before_attestation|.expires_at_unix_ms=.attested_at_unix_ms
artifact_extra|.artifact.extra=true
artifact_kind|.artifact.kind="wrong"
artifact_evidence_kind|.artifact.evidence_kind="wrong"
artifact_path|.artifact.path="/tmp/wrong"
artifact_hash|.artifact.sha256=("e"*64)
artifact_size|.artifact.size_bytes+=1
checks_extra|.checks.extra=true
EOF

while IFS=$'\t' read -r key type value; do
  case "$type" in
    boolean) replacement=$([[ "$value" == true ]] && echo false || echo true) ;;
    string) replacement='""' ;;
    number) replacement=$([[ "$value" == 0 ]] && echo 1 || echo 0) ;;
    *) fail "unsupported check value type" ;;
  esac
  mutate_receipt "check_$key" ".checks[\"$key\"]=$replacement"
done < <(/usr/bin/jq -r '.checks|to_entries[]|[.key,(.value|type),(.value|tostring)]|@tsv' "$RECEIPT")

while IFS='|' read -r name filter; do reject_manifest "$name" "$filter"; done <<'EOF'
manifest_schema|.schema_version=2
manifest_kind|.kind="wrong"
manifest_profile|.profile="wrong"
manifest_evidence_kind|.evidence_kind="wrong"
manifest_producer|.producer="wrong"
manifest_run|.run_identifier_sha256=("e"*64)
manifest_source|.source_binding.head=("e"*40)
manifest_extra|.extra=true
manifest_missing_role|.entries=[]
manifest_duplicate_role|.entries += [.entries[0]]
manifest_bad_hash|.entries[0].sha256="short"
manifest_zero_size|.entries[0].size_bytes=0
manifest_bad_media|.entries[0].media_type="text/plain"
manifest_secret_flag|.redaction.secrets_included=true
manifest_token_flag|.redaction.tokens_included=true
manifest_password_flag|.redaction.passwords_included=true
manifest_raw_payload_flag|.redaction.raw_payloads_included=true
manifest_not_redacted|.redaction.credentials_redacted=false
manifest_secret_field|.redaction.access_token="Bearer synthetic-not-a-secret"
manifest_raw_payload_field|.raw_response={"body":"synthetic"}
EOF

reject_manifest manifest_missing_evidence '.entries[0].path="/definitely/missing/hepta-evidence"'
reject_manifest manifest_evidence_outside_bundle '.entries[0].path="/etc/hosts"'

primary_evidence="$(/usr/bin/jq -r '.entries[0].path' "$ARTIFACT")"
/bin/cp "$primary_evidence" "$TEST_DIR/tampered-evidence"
printf x >>"$TEST_DIR/tampered-evidence"
/usr/bin/jq --arg path "$TEST_DIR/tampered-evidence" '.entries[0].path=$path' "$ARTIFACT" >"$TEST_DIR/tampered-evidence-manifest.json"
/usr/bin/jq --arg path "$TEST_DIR/tampered-evidence-manifest.json" --arg hash "$(sha "$TEST_DIR/tampered-evidence-manifest.json")" --argjson size "$(bytes "$TEST_DIR/tampered-evidence-manifest.json")" '.artifact.path=$path | .artifact.sha256=$hash | .artifact.size_bytes=$size' "$RECEIPT" >"$TEST_DIR/tampered-evidence-receipt.json"
sign "$TEST_DIR/tampered-evidence-receipt.json" "$PRIVATE_KEY" "$TEST_DIR/tampered-evidence.sig"
reject_receipt manifest_actual_bytes_mismatch "$TEST_DIR/tampered-evidence-receipt.json" "$TEST_DIR/tampered-evidence-manifest.json" "$TEST_DIR/tampered-evidence.sig"

/bin/ln -s "$primary_evidence" "$TEST_DIR/evidence-link"
/usr/bin/jq --arg path "$TEST_DIR/evidence-link" '.entries[0].path=$path' "$ARTIFACT" >"$TEST_DIR/evidence-link-manifest.json"
/usr/bin/jq --arg path "$TEST_DIR/evidence-link-manifest.json" --arg hash "$(sha "$TEST_DIR/evidence-link-manifest.json")" --argjson size "$(bytes "$TEST_DIR/evidence-link-manifest.json")" '.artifact.path=$path | .artifact.sha256=$hash | .artifact.size_bytes=$size' "$RECEIPT" >"$TEST_DIR/evidence-link-receipt.json"
sign "$TEST_DIR/evidence-link-receipt.json" "$PRIVATE_KEY" "$TEST_DIR/evidence-link.sig"
reject_receipt manifest_evidence_symlink "$TEST_DIR/evidence-link-receipt.json" "$TEST_DIR/evidence-link-manifest.json" "$TEST_DIR/evidence-link.sig"

if [[ "$(/usr/bin/jq -r '.entries[0].media_type' "$ARTIFACT")" == application/json ]]; then
  /usr/bin/jq -n '{schema_version:1,access_token:"Bearer synthetic-token-material"}' >"$TEST_DIR/unredacted-evidence.json"
  /usr/bin/jq --arg path "$TEST_DIR/unredacted-evidence.json" --arg sha "$(sha "$TEST_DIR/unredacted-evidence.json")" --argjson size "$(bytes "$TEST_DIR/unredacted-evidence.json")" '.entries[0].path=$path | .entries[0].sha256=$sha | .entries[0].size_bytes=$size' "$ARTIFACT" >"$TEST_DIR/unredacted-manifest.json"
  /usr/bin/jq --arg path "$TEST_DIR/unredacted-manifest.json" --arg hash "$(sha "$TEST_DIR/unredacted-manifest.json")" --argjson size "$(bytes "$TEST_DIR/unredacted-manifest.json")" '.artifact.path=$path | .artifact.sha256=$hash | .artifact.size_bytes=$size' "$RECEIPT" >"$TEST_DIR/unredacted-receipt.json"
  sign "$TEST_DIR/unredacted-receipt.json" "$PRIVATE_KEY" "$TEST_DIR/unredacted.sig"
  reject_receipt unredacted_actual_evidence "$TEST_DIR/unredacted-receipt.json" "$TEST_DIR/unredacted-manifest.json" "$TEST_DIR/unredacted.sig"
fi

reject_args missing_args
reject_args relative_receipt --receipt relative --artifact "$ARTIFACT" --signature "$SIGNATURE" --trusted-public-key "$PUBLIC_KEY" --expected-public-key-sha256 "$PUBLIC_KEY_SHA" --expected-producer "$PRODUCER" --trust-policy-sha256 "$POLICY_SHA" --source-head "$HEAD" --source-tree "$TREE" --source-fingerprint "$FP"

/bin/ln -s "$RECEIPT" "$TEST_DIR/receipt-link"
reject_receipt receipt_symlink "$TEST_DIR/receipt-link"
/bin/ln -s "$ARTIFACT" "$TEST_DIR/artifact-link"
reject_receipt artifact_symlink "$RECEIPT" "$TEST_DIR/artifact-link"
/bin/ln -s "$SIGNATURE" "$TEST_DIR/signature-link"
reject_receipt signature_symlink "$RECEIPT" "$ARTIFACT" "$TEST_DIR/signature-link"
/bin/ln -s "$PUBLIC_KEY" "$TEST_DIR/public-key-link"
reject_receipt public_key_symlink "$RECEIPT" "$ARTIFACT" "$SIGNATURE" "$TEST_DIR/public-key-link"
/bin/ln -s "$TEST_DIR" "$TEST_DIR/parent-link"
reject_receipt parent_symlink "$TEST_DIR/parent-link/attestation.json"

printf '{bad\n' >"$TEST_DIR/bad.json"
sign "$TEST_DIR/bad.json" "$PRIVATE_KEY" "$TEST_DIR/bad.sig"
reject_receipt malformed_json "$TEST_DIR/bad.json" "$ARTIFACT" "$TEST_DIR/bad.sig"
printf '{"schema_version":1,"schema_version":1}\n' >"$TEST_DIR/duplicate.json"
sign "$TEST_DIR/duplicate.json" "$PRIVATE_KEY" "$TEST_DIR/duplicate.sig"
reject_receipt duplicate_key "$TEST_DIR/duplicate.json" "$ARTIFACT" "$TEST_DIR/duplicate.sig"
reject_receipt same_file "$RECEIPT" "$RECEIPT"

printf 'arbitrary text is not an evidence manifest\n' >"$TEST_DIR/arbitrary.txt"
/usr/bin/jq --arg path "$TEST_DIR/arbitrary.txt" --arg hash "$(sha "$TEST_DIR/arbitrary.txt")" --argjson size "$(bytes "$TEST_DIR/arbitrary.txt")" '.artifact.path=$path | .artifact.sha256=$hash | .artifact.size_bytes=$size' "$RECEIPT" >"$TEST_DIR/arbitrary-receipt.json"
sign "$TEST_DIR/arbitrary-receipt.json" "$PRIVATE_KEY" "$TEST_DIR/arbitrary.sig"
reject_receipt arbitrary_text_artifact "$TEST_DIR/arbitrary-receipt.json" "$TEST_DIR/arbitrary.txt" "$TEST_DIR/arbitrary.sig"

/bin/dd if=/dev/zero of="$TEST_DIR/oversize-manifest.json" bs=1048576 count=2 >/dev/null 2>&1
/usr/bin/jq --arg path "$TEST_DIR/oversize-manifest.json" --arg hash "$(sha "$TEST_DIR/oversize-manifest.json")" --argjson size "$(bytes "$TEST_DIR/oversize-manifest.json")" '.artifact.path=$path | .artifact.sha256=$hash | .artifact.size_bytes=$size' "$RECEIPT" >"$TEST_DIR/oversize-receipt.json"
sign "$TEST_DIR/oversize-receipt.json" "$PRIVATE_KEY" "$TEST_DIR/oversize.sig"
reject_receipt oversize_artifact "$TEST_DIR/oversize-receipt.json" "$TEST_DIR/oversize-manifest.json" "$TEST_DIR/oversize.sig"

/usr/bin/openssl genrsa -out "$TEST_DIR/rogue-private.pem" 2048 >/dev/null 2>&1
/usr/bin/openssl rsa -in "$TEST_DIR/rogue-private.pem" -pubout -out "$TEST_DIR/rogue-public.pem" >/dev/null 2>&1
sign "$RECEIPT" "$TEST_DIR/rogue-private.pem" "$TEST_DIR/rogue.sig"
reject_receipt rogue_signature "$RECEIPT" "$ARTIFACT" "$TEST_DIR/rogue.sig"
reject_receipt rogue_key_anchor "$RECEIPT" "$ARTIFACT" "$TEST_DIR/rogue.sig" "$TEST_DIR/rogue-public.pem" "$PUBLIC_KEY_SHA"

/bin/cp "$SIGNATURE" "$TEST_DIR/tampered.sig"
printf x >>"$TEST_DIR/tampered.sig"
reject_receipt tampered_signature "$RECEIPT" "$ARTIFACT" "$TEST_DIR/tampered.sig"
/bin/cp "$PUBLIC_KEY" "$TEST_DIR/tampered-public.pem"
printf '\n' >>"$TEST_DIR/tampered-public.pem"
reject_receipt tampered_public_key "$RECEIPT" "$ARTIFACT" "$SIGNATURE" "$TEST_DIR/tampered-public.pem"

wrong_policy_args=("${BASE_ARGS[@]}")
for ((i=0; i<${#wrong_policy_args[@]}; i++)); do
  [[ "${wrong_policy_args[$i]}" == --trust-policy-sha256 ]] && wrong_policy_args[$((i+1))]="short"
done
reject_args wrong_trust_policy_blob "${wrong_policy_args[@]}"

if [[ "$PROFILE" == matrix ]]; then
  mutate_receipt matrix_login_after_workflow '.checks.login_observed_unix_ms=.checks.authenticated_workflow_observed_unix_ms'
  mutate_receipt matrix_workflow_after_logout '.checks.authenticated_workflow_observed_unix_ms=.checks.logout_observed_unix_ms'
  mutate_receipt matrix_attestation_before_logout '.attested_at_unix_ms=(.checks.logout_observed_unix_ms-1)'
fi

if [[ "$PROFILE" == bridge ]]; then
  mutate_receipt bridge_parent_hash_mismatch '.checks.matrix_attestation_sha256=("f"*64)'
  mutate_receipt bridge_session_mismatch '.checks.session_identifier_sha256=("f"*64)'
  mutate_receipt bridge_at_logout '.checks.bridge_get_observed_unix_ms='"$LOGOUT_MS"

  /usr/bin/jq '.run_identifier_sha256=("f"*64)' "$ARTIFACT" >"$TEST_DIR/bridge-other-run-manifest.json"
  /usr/bin/jq --arg path "$TEST_DIR/bridge-other-run-manifest.json" --arg hash "$(sha "$TEST_DIR/bridge-other-run-manifest.json")" --argjson size "$(bytes "$TEST_DIR/bridge-other-run-manifest.json")" '.run_identifier_sha256=("f"*64) | .artifact.path=$path | .artifact.sha256=$hash | .artifact.size_bytes=$size' "$RECEIPT" >"$TEST_DIR/bridge-other-run.json"
  sign "$TEST_DIR/bridge-other-run.json" "$PRIVATE_KEY" "$TEST_DIR/bridge-other-run.sig"
  reject_receipt bridge_parent_run_mismatch "$TEST_DIR/bridge-other-run.json" "$TEST_DIR/bridge-other-run-manifest.json" "$TEST_DIR/bridge-other-run.sig"

  /bin/cp "$MATRIX_SIGNATURE" "$TEST_DIR/bad-matrix.sig"
  printf x >>"$TEST_DIR/bad-matrix.sig"
  bad_parent_args=("${BASE_ARGS[@]}")
  for ((i=0; i<${#bad_parent_args[@]}; i++)); do
    if [[ "${bad_parent_args[$i]}" == --matrix-signature ]]; then bad_parent_args[$((i+1))]="$TEST_DIR/bad-matrix.sig"; fi
  done
  reject_args bridge_bad_parent_signature "${bad_parent_args[@]}"

  /usr/bin/jq '.checks.session_identifier_sha256=("f"*64)' "$MATRIX_RECEIPT" >"$TEST_DIR/matrix-session-mismatch.json"
  sign "$TEST_DIR/matrix-session-mismatch.json" "$MATRIX_PRIVATE_KEY" "$TEST_DIR/matrix-session-mismatch.sig"
  parent_session_args=("${BASE_ARGS[@]}")
  for ((i=0; i<${#parent_session_args[@]}; i++)); do
    [[ "${parent_session_args[$i]}" == --matrix-receipt ]] && parent_session_args[$((i+1))]="$TEST_DIR/matrix-session-mismatch.json"
    [[ "${parent_session_args[$i]}" == --matrix-signature ]] && parent_session_args[$((i+1))]="$TEST_DIR/matrix-session-mismatch.sig"
  done
  reject_args bridge_parent_session_mutated "${parent_session_args[@]}"

  /usr/bin/jq '.expires_at_unix_ms=1' "$MATRIX_RECEIPT" >"$TEST_DIR/matrix-expired.json"
  sign "$TEST_DIR/matrix-expired.json" "$MATRIX_PRIVATE_KEY" "$TEST_DIR/matrix-expired.sig"
  /usr/bin/jq --arg parent "$(sha "$TEST_DIR/matrix-expired.json")" '.checks.matrix_attestation_sha256=$parent' "$RECEIPT" >"$TEST_DIR/bridge-bound-to-expired-parent.json"
  sign "$TEST_DIR/bridge-bound-to-expired-parent.json" "$PRIVATE_KEY" "$TEST_DIR/bridge-bound-to-expired-parent.sig"
  expired_parent_args=("${BASE_ARGS[@]}")
  for ((i=0; i<${#expired_parent_args[@]}; i++)); do
    [[ "${expired_parent_args[$i]}" == --receipt ]] && expired_parent_args[$((i+1))]="$TEST_DIR/bridge-bound-to-expired-parent.json"
    [[ "${expired_parent_args[$i]}" == --signature ]] && expired_parent_args[$((i+1))]="$TEST_DIR/bridge-bound-to-expired-parent.sig"
    [[ "${expired_parent_args[$i]}" == --matrix-receipt ]] && expired_parent_args[$((i+1))]="$TEST_DIR/matrix-expired.json"
    [[ "${expired_parent_args[$i]}" == --matrix-signature ]] && expired_parent_args[$((i+1))]="$TEST_DIR/matrix-expired.sig"
  done
  reject_args bridge_expired_parent "${expired_parent_args[@]}"
fi

# A verifier receipt must not stay promotable when a manifest leaf changes or
# becomes a symlink after the signed verifier has returned.
primary_evidence="$(/usr/bin/jq -r '.artifact.entry_digests[0].path' "$TEST_DIR/ready.json")"
/bin/cp "$primary_evidence" "$TEST_DIR/rehash-primary-backup"
printf x >>"$primary_evidence"
reject_rehash post_verifier_leaf_mutation
/bin/mv -f "$TEST_DIR/rehash-primary-backup" "$primary_evidence"
"$REHASHER" rehash "$PROFILE" --receipt "$TEST_DIR/ready.json" >/dev/null || fail "rehash did not recover after exact-byte restore"
/bin/mv "$primary_evidence" "$TEST_DIR/rehash-primary-real"
/bin/ln -s "$TEST_DIR/rehash-primary-real" "$primary_evidence"
reject_rehash post_verifier_leaf_symlink
/bin/rm "$primary_evidence"
/bin/mv "$TEST_DIR/rehash-primary-real" "$primary_evidence"
"$REHASHER" rehash "$PROFILE" --receipt "$TEST_DIR/ready.json" >/dev/null || fail "rehash did not recover after symlink restore"

cat >"$TEST_DIR/hook.sh" <<EOF
#!/bin/bash
/usr/bin/touch "$TEST_DIR/bash-hook-ran"
EOF
printf 'File.write(%q{%s},%q{ran})\n' "$TEST_DIR/ruby-hook-ran" >"$TEST_DIR/hook.rb"
chmod 700 "$TEST_DIR/hook.sh"
BASH_ENV="$TEST_DIR/hook.sh" RUBYOPT="-r$TEST_DIR/hook.rb" "$VERIFIER" "${BASE_ARGS[@]}" >/dev/null
[[ ! -e "$TEST_DIR/bash-hook-ran" && ! -e "$TEST_DIR/ruby-hook-ran" ]] || fail "startup hook ran"

for path in /usr/bin/codesign /usr/bin/xcrun /usr/bin/adb /usr/bin/simctl /usr/bin/defaults /usr/bin/curl /usr/bin/scp /usr/bin/openssl; do
  ! /usr/bin/grep -F "$path" "$ROOT_DIR/scripts/lib/hepta-ui-promotion-verifier-v1.sh" >/dev/null || fail "forbidden command $path"
done
[[ "$NEG" -ge 55 ]] || fail "negative matrix incomplete ($NEG)"
printf 'hepta-ui-%s-verifier self-test passed (%s negative cases)\n' "$STEM" "$NEG"
