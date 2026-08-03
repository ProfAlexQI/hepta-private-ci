#!/bin/bash -p
set +x
set -euo pipefail

unset BASH_ENV ENV CDPATH GLOBIGNORE
unset RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
PATH="/usr/bin:/bin:/usr/sbin:/sbin"
export PATH
umask 077

ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd -P)"
VERIFIER="$ROOT_DIR/scripts/hepta-ui-release-execution-approval-verifier-v1"
BUILD_SCRIPT="$ROOT_DIR/apps/hepta-native/packaging/build-macos-dmg.sh"
TMP_ROOT="$(cd "${TMPDIR:-/private/tmp}" && pwd -P)"
TEST_DIR="$(mktemp -d "$TMP_ROOT/hepta-release-execution-approval-self-test.XXXXXX")"
TEST_DIR="$(cd "$TEST_DIR" && pwd -P)"
trap '/bin/rm -rf "$TEST_DIR"' EXIT

fail() {
  echo "release execution approval verifier self-test failed: $1" >&2
  exit 1
}
sha256() { /usr/bin/shasum -a 256 "$1" | /usr/bin/awk '{print $1}'; }
text_sha256() { /usr/bin/printf '%s' "$1" | /usr/bin/shasum -a 256 | /usr/bin/awk '{print $1}'; }

PRIVATE_KEY="$TEST_DIR/test-only-private-key.pem"
PUBLIC_KEY="$TEST_DIR/test-only-public-key.pem"
SIGNATURE="$TEST_DIR/approval.sig"
APPROVAL="$TEST_DIR/approval.json"
TRUST_POLICY="$TEST_DIR/test-only-trust-policy.json"
/usr/bin/openssl genrsa -out "$PRIVATE_KEY" 3072 >/dev/null 2>&1
/usr/bin/openssl rsa -in "$PRIVATE_KEY" -pubout -out "$PUBLIC_KEY" >/dev/null 2>&1

HEAD="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
TREE="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
FINGERPRINT="cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc"
APP_PATH="$TEST_DIR/Hepta.app"
APP_RECEIPT="$TEST_DIR/unsigned-package.json"
OUTPUT_PATH="$TEST_DIR/Hepta.dmg"
RELEASE_RECEIPT="$TEST_DIR/Hepta.dmg.receipt.json"
EVIDENCE_DIR="$TEST_DIR/Hepta.dmg.evidence"
APP_RECEIPT_SHA="dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
APP_BUNDLE_SHA="eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"
APP_BINARY_SHA="ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
PACKAGING_SCRIPT_SHA="1111111111111111111111111111111111111111111111111111111111111111"
APPROVAL_VERIFIER_SHA="3333333333333333333333333333333333333333333333333333333333333333"
NOTARY_PROFILE_SHA="2222222222222222222222222222222222222222222222222222222222222222"
SIGNER="hepta-release-operator/test-only-self-test"
SIGNING_IDENTITY="Developer ID Application: Hepta Test (TEAMID1234)"
SIGNING_CERTIFICATE_SHA1="4444444444444444444444444444444444444444"
TEAM_ID="TEAMID1234"
ENTITLEMENTS_SHA="5555555555555555555555555555555555555555555555555555555555555555"
PUBLIC_KEY_SHA="$(sha256 "$PUBLIC_KEY")"
/usr/bin/jq -n \
  --arg signer "$SIGNER" --arg key_sha "$PUBLIC_KEY_SHA" \
  '{schema_version:1,kind:"hepta-ui-release-execution-approval-trust-v1",status:"ready",signer_id:$signer,public_key_sha256:$key_sha,signature_algorithm:"rsa-pkcs1-sha256",minimum_rsa_bits:3072}' \
  >"$TRUST_POLICY"
TRUST_POLICY_SHA="$(sha256 "$TRUST_POLICY")"
read -r ISSUED_AT EXPIRES_AT < <(/usr/bin/ruby --disable-gems -rtime -e 'now=Time.now.utc; puts "#{now.strftime("%Y-%m-%dT%H:%M:%SZ")} #{(now+600).strftime("%Y-%m-%dT%H:%M:%SZ")}"')

write_approval() {
  local destination="$1" issued_at="${2:-$ISSUED_AT}" expires_at="${3:-$EXPIRES_AT}"
  /usr/bin/jq -n \
    --arg issued "$issued_at" --arg expires "$expires_at" \
    --arg signer "$SIGNER" --arg key_sha "$PUBLIC_KEY_SHA" \
    --arg head "$HEAD" --arg tree "$TREE" --arg fingerprint "$FINGERPRINT" \
    --arg app_path "$APP_PATH" --arg app_receipt "$APP_RECEIPT" \
    --arg app_receipt_sha "$APP_RECEIPT_SHA" --arg app_bundle_sha "$APP_BUNDLE_SHA" \
    --arg app_binary_sha "$APP_BINARY_SHA" --arg packaging_sha "$PACKAGING_SCRIPT_SHA" \
    --arg approval_verifier_sha "$APPROVAL_VERIFIER_SHA" \
    --arg trust_policy_sha "$TRUST_POLICY_SHA" \
    --arg output "$OUTPUT_PATH" --arg release_receipt "$RELEASE_RECEIPT" \
    --arg evidence "$EVIDENCE_DIR" --arg identity "$SIGNING_IDENTITY" \
    --arg signing_certificate_sha1 "$SIGNING_CERTIFICATE_SHA1" \
    --arg team "$TEAM_ID" --arg entitlements_sha "$ENTITLEMENTS_SHA" \
    --arg notary_profile_sha "$NOTARY_PROFILE_SHA" \
    '{
      schema_version:1,
      kind:"hepta-ui-release-execution-approval-v1",
      approval_id:"11111111-2222-4333-8444-555555555555",
      signer:{id:$signer,public_key_sha256:$key_sha,signature_algorithm:"rsa-pkcs1-sha256"},
      validity:{issued_at_utc:$issued,expires_at_utc:$expires},
      source_binding:{schema_version:1,kind:"hepta-ui-source-binding",head:$head,head_tree:$tree,source_fingerprint:$fingerprint,worktree_clean:true,repository_worktree_clean:true},
      input:{unsigned_app_path:$app_path,unsigned_app_receipt_path:$app_receipt,unsigned_app_receipt_sha256:$app_receipt_sha,unsigned_app_bundle_fingerprint_sha256:$app_bundle_sha,unsigned_app_binary_sha256:$app_binary_sha},
      action:{kind:"sign_notarize_staple_local_dmg",product:"Hepta Native",bundle_identifier:"ai.hepta.nativeapp",product_version:"1.0.0",packager_arch:"aarch64",packaging_script_sha256:$packaging_sha,approval_verifier_sha256:$approval_verifier_sha,trust_policy_sha256:$trust_policy_sha,output_path:$output,release_receipt_path:$release_receipt,evidence_dir:$evidence,signing_identity:$identity,signing_certificate_sha1:$signing_certificate_sha1,team_identifier:$team,entitlements_sha256:$entitlements_sha,notary_profile_sha256:$notary_profile_sha,developer_id_signing:true,notarization_submission:true,stapling:true,local_distribution_artifact_write:true,public_distribution_authorized:false,public_upload_authorized:false,public_upload_performed:false}
    }' >"$destination"
}

sign_approval() {
  local approval="$1" signature="$2" key="${3:-$PRIVATE_KEY}"
  /usr/bin/openssl dgst -sha256 -sign "$key" -out "$signature" "$approval"
}

write_approval "$APPROVAL"
sign_approval "$APPROVAL" "$SIGNATURE"
ARGS=(
  --approval "$APPROVAL" --signature "$SIGNATURE" --public-key "$PUBLIC_KEY"
  --trust-policy "$TRUST_POLICY"
  --source-head "$HEAD" --source-tree "$TREE" --source-fingerprint "$FINGERPRINT"
  --unsigned-app-path "$APP_PATH" --unsigned-app-receipt-path "$APP_RECEIPT"
  --unsigned-app-receipt-sha256 "$APP_RECEIPT_SHA"
  --unsigned-app-bundle-fingerprint-sha256 "$APP_BUNDLE_SHA"
  --unsigned-app-binary-sha256 "$APP_BINARY_SHA"
  --packaging-script-sha256 "$PACKAGING_SCRIPT_SHA" --approval-verifier-sha256 "$APPROVAL_VERIFIER_SHA"
  --product-version 1.0.0
  --packager-arch aarch64 --output-path "$OUTPUT_PATH"
  --release-receipt-path "$RELEASE_RECEIPT" --evidence-dir "$EVIDENCE_DIR"
  --signing-identity "$SIGNING_IDENTITY" --signing-certificate-sha1 "$SIGNING_CERTIFICATE_SHA1"
  --team-id "$TEAM_ID" --entitlements-sha256 "$ENTITLEMENTS_SHA"
  --notary-profile-sha256 "$NOTARY_PROFILE_SHA"
)
APPROVAL_SHA_BEFORE="$(sha256 "$APPROVAL")"
SIGNATURE_SHA_BEFORE="$(sha256 "$SIGNATURE")"
PUBLIC_KEY_SHA_BEFORE="$(sha256 "$PUBLIC_KEY")"
"$VERIFIER" "${ARGS[@]}" >"$TEST_DIR/ready.json"
/usr/bin/jq -e \
  --arg approval "$APPROVAL" --arg approval_sha "$APPROVAL_SHA_BEFORE" \
  --arg signature "$SIGNATURE" --arg signature_sha "$SIGNATURE_SHA_BEFORE" \
  --arg key "$PUBLIC_KEY" --arg key_sha "$PUBLIC_KEY_SHA_BEFORE" \
  '.schema_version==1 and .kind=="hepta-ui-release-execution-approval-verification-v1" and .status=="ready" and .approval_valid==true and .signature_verified==true and .approval.path==$approval and .approval.sha256==$approval_sha and .signature.path==$signature and .signature.sha256==$signature_sha and .trust_policy.status=="ready" and .trusted_public_key.path==$key and .trusted_public_key.sha256==$key_sha and .release_execution_approved==true and .public_distribution_authorized==false and .public_upload_authorized==false and .public_upload_performed==false and (.verifier_actions|all(.==false))' \
  "$TEST_DIR/ready.json" >/dev/null || fail "positive verification output invalid"
[[ "$(sha256 "$APPROVAL")" == "$APPROVAL_SHA_BEFORE" && "$(sha256 "$SIGNATURE")" == "$SIGNATURE_SHA_BEFORE" && "$(sha256 "$PUBLIC_KEY")" == "$PUBLIC_KEY_SHA_BEFORE" ]] || fail "verifier mutated input"

/usr/bin/printf '#!/bin/bash\n/usr/bin/touch "%s"\n' "$TEST_DIR/bash-hook-ran" >"$TEST_DIR/hostile-bash-env.sh"
/usr/bin/printf 'File.write(%%q{%s}, %%q{ran})\n' "$TEST_DIR/ruby-hook-ran" >"$TEST_DIR/hostile-rubyopt.rb"
/bin/chmod 700 "$TEST_DIR/hostile-bash-env.sh"
/usr/bin/env \
  BASH_ENV="$TEST_DIR/hostile-bash-env.sh" \
  RUBYOPT="-r$TEST_DIR/hostile-rubyopt.rb" \
  SHELLOPTS=xtrace \
  PS4='release-approval-secret-sentinel ' \
  "$VERIFIER" "${ARGS[@]}" >"$TEST_DIR/hostile-environment-ready.json" 2>"$TEST_DIR/hostile-environment.stderr"
[[ ! -e "$TEST_DIR/bash-hook-ran" && ! -e "$TEST_DIR/ruby-hook-ran" ]] || fail "startup hook executed"
! /usr/bin/grep -F 'release-approval-secret-sentinel' "$TEST_DIR/hostile-environment-ready.json" "$TEST_DIR/hostile-environment.stderr" >/dev/null || fail "xtrace secret marker leaked"

NEGATIVE_COUNT=0
reject() {
  local name="$1"
  shift
  if "$VERIFIER" "$@" >"$TEST_DIR/$name.stdout" 2>"$TEST_DIR/$name.stderr"; then
    fail "$name was accepted"
  fi
  [[ ! -s "$TEST_DIR/$name.stdout" ]] || fail "$name emitted stdout"
  NEGATIVE_COUNT=$((NEGATIVE_COUNT + 1))
}

reject_with_override() {
  local name="$1" flag="$2" value="$3"
  local -a changed=("${ARGS[@]}")
  local index
  for ((index=0; index<${#changed[@]}; index+=2)); do
    if [[ "${changed[$index]}" == "$flag" ]]; then changed[$((index+1))]="$value"; break; fi
  done
  reject "$name" "${changed[@]}"
}

mutate_and_reject() {
  local name="$1" filter="$2"
  local mutated="$TEST_DIR/$name.json" signature="$TEST_DIR/$name.sig"
  /usr/bin/jq "$filter" "$APPROVAL" >"$mutated"
  sign_approval "$mutated" "$signature"
  local -a changed=("${ARGS[@]}")
  local index
  for ((index=0; index<${#changed[@]}; index+=2)); do
    [[ "${changed[$index]}" == "--approval" ]] && changed[$((index+1))]="$mutated"
    [[ "${changed[$index]}" == "--signature" ]] && changed[$((index+1))]="$signature"
  done
  reject "$name" "${changed[@]}"
}

reject missing_arguments
reject unknown_option "${ARGS[@]}" --unknown value
/usr/bin/jq '.public_key_sha256=("9"*64)' "$TRUST_POLICY" >"$TEST_DIR/wrong-key-policy.json"
reject_with_override wrong_trusted_key_hash --trust-policy "$TEST_DIR/wrong-key-policy.json"
/usr/bin/jq '.signer_id="wrong-signer"' "$TRUST_POLICY" >"$TEST_DIR/wrong-signer-policy.json"
reject_with_override wrong_signer --trust-policy "$TEST_DIR/wrong-signer-policy.json"
/usr/bin/jq '.status="not_configured" | .signer_id=null | .public_key_sha256=null' "$TRUST_POLICY" >"$TEST_DIR/unconfigured-policy.json"
reject_with_override unconfigured_trust --trust-policy "$TEST_DIR/unconfigured-policy.json"
reject_with_override production_trust_not_configured --trust-policy "$ROOT_DIR/apps/hepta-native/packaging/release-execution-approval-trust-v1.json"
reject_with_override wrong_head --source-head 9999999999999999999999999999999999999999
reject_with_override wrong_tree --source-tree 9999999999999999999999999999999999999999
reject_with_override wrong_source_fingerprint --source-fingerprint "$(text_sha256 wrong-source)"
reject_with_override wrong_app_path --unsigned-app-path "$TEST_DIR/Wrong.app"
reject_with_override wrong_app_receipt_path --unsigned-app-receipt-path "$TEST_DIR/wrong-receipt.json"
reject_with_override wrong_app_receipt_sha --unsigned-app-receipt-sha256 "$(text_sha256 wrong-receipt)"
reject_with_override wrong_bundle_sha --unsigned-app-bundle-fingerprint-sha256 "$(text_sha256 wrong-bundle)"
reject_with_override wrong_binary_sha --unsigned-app-binary-sha256 "$(text_sha256 wrong-binary)"
reject_with_override wrong_packaging_sha --packaging-script-sha256 "$(text_sha256 wrong-script)"
reject_with_override wrong_approval_verifier_sha --approval-verifier-sha256 "$(text_sha256 wrong-verifier)"
reject_with_override wrong_version --product-version 2.0.0
reject_with_override wrong_arch --packager-arch x86_64
reject_with_override wrong_output --output-path "$TEST_DIR/Wrong.dmg"
reject_with_override wrong_release_receipt --release-receipt-path "$TEST_DIR/wrong-release.json"
reject_with_override wrong_evidence --evidence-dir "$TEST_DIR/wrong-evidence"
reject_with_override wrong_signing_identity --signing-identity "Developer ID Application: Wrong (TEAMID1234)"
reject_with_override wrong_signing_certificate --signing-certificate-sha1 6666666666666666666666666666666666666666
reject_with_override wrong_team --team-id WRONGID123
reject_with_override wrong_entitlements --entitlements-sha256 "$(text_sha256 wrong-entitlements)"
reject_with_override wrong_notary_profile --notary-profile-sha256 "$(text_sha256 wrong-profile)"

while IFS='|' read -r name filter; do mutate_and_reject "$name" "$filter"; done <<'EOF'
schema_version|.schema_version=2
kind|.kind="wrong"
top_extra|.extra=true
approval_id|.approval_id="not-a-uuid"
signer_extra|.signer.extra=true
signer_id|.signer.id="wrong"
signed_key_hash|.signer.public_key_sha256=("9"*64)
signature_algorithm|.signer.signature_algorithm="none"
source_dirty|.source_binding.worktree_clean=false
repository_dirty|.source_binding.repository_worktree_clean=false
input_extra|.input.extra=true
action_extra|.action.extra=true
operation|.action.kind="upload"
product|.action.product="Wrong"
bundle_identifier|.action.bundle_identifier="wrong.bundle"
trust_policy_sha|.action.trust_policy_sha256=("9"*64)
developer_signing|.action.developer_id_signing=false
notarization|.action.notarization_submission=false
stapling|.action.stapling=false
local_write|.action.local_distribution_artifact_write=false
public_authorization|.action.public_distribution_authorized=true
upload_authorization|.action.public_upload_authorized=true
upload_performed|.action.public_upload_performed=true
EOF

/usr/bin/printf '{"schema_version":1,"schema_version":1}\n' >"$TEST_DIR/duplicate.json"
sign_approval "$TEST_DIR/duplicate.json" "$TEST_DIR/duplicate.sig"
duplicate_args=("${ARGS[@]}")
for ((i=0; i<${#duplicate_args[@]}; i+=2)); do
  [[ "${duplicate_args[$i]}" == --approval ]] && duplicate_args[$((i+1))]="$TEST_DIR/duplicate.json"
  [[ "${duplicate_args[$i]}" == --signature ]] && duplicate_args[$((i+1))]="$TEST_DIR/duplicate.sig"
done
reject duplicate_key "${duplicate_args[@]}"

/usr/bin/printf '{bad\n' >"$TEST_DIR/malformed.json"
sign_approval "$TEST_DIR/malformed.json" "$TEST_DIR/malformed.sig"
malformed_args=("${ARGS[@]}")
for ((i=0; i<${#malformed_args[@]}; i+=2)); do
  [[ "${malformed_args[$i]}" == --approval ]] && malformed_args[$((i+1))]="$TEST_DIR/malformed.json"
  [[ "${malformed_args[$i]}" == --signature ]] && malformed_args[$((i+1))]="$TEST_DIR/malformed.sig"
done
reject malformed_json "${malformed_args[@]}"

/bin/cp "$SIGNATURE" "$TEST_DIR/wrong-signature.sig"
/usr/bin/printf 'x' >>"$TEST_DIR/wrong-signature.sig"
reject_with_override invalid_signature --signature "$TEST_DIR/wrong-signature.sig"

/usr/bin/openssl genrsa -out "$TEST_DIR/other-private.pem" 3072 >/dev/null 2>&1
/usr/bin/openssl rsa -in "$TEST_DIR/other-private.pem" -pubout -out "$TEST_DIR/other-public.pem" >/dev/null 2>&1
reject_with_override untrusted_public_key --public-key "$TEST_DIR/other-public.pem"

PRIVATE_KEY_SHA="$(sha256 "$PRIVATE_KEY")"
/usr/bin/jq --arg signer "$SIGNER" --arg hash "$PRIVATE_KEY_SHA" \
  '{schema_version:1,kind:"hepta-ui-release-execution-approval-trust-v1",status:"ready",signer_id:$signer,public_key_sha256:$hash,signature_algorithm:"rsa-pkcs1-sha256",minimum_rsa_bits:3072}' \
  >"$TEST_DIR/private-key-policy.json"
PRIVATE_KEY_POLICY_SHA="$(sha256 "$TEST_DIR/private-key-policy.json")"
/usr/bin/jq --arg hash "$PRIVATE_KEY_SHA" --arg policy_sha "$PRIVATE_KEY_POLICY_SHA" \
  '.signer.public_key_sha256=$hash | .action.trust_policy_sha256=$policy_sha' \
  "$APPROVAL" >"$TEST_DIR/private-key-approval.json"
sign_approval "$TEST_DIR/private-key-approval.json" "$TEST_DIR/private-key-approval.sig"
private_key_args=("${ARGS[@]}")
for ((i=0; i<${#private_key_args[@]}; i+=2)); do
  [[ "${private_key_args[$i]}" == --approval ]] && private_key_args[$((i+1))]="$TEST_DIR/private-key-approval.json"
  [[ "${private_key_args[$i]}" == --signature ]] && private_key_args[$((i+1))]="$TEST_DIR/private-key-approval.sig"
  [[ "${private_key_args[$i]}" == --public-key ]] && private_key_args[$((i+1))]="$PRIVATE_KEY"
  [[ "${private_key_args[$i]}" == --trust-policy ]] && private_key_args[$((i+1))]="$TEST_DIR/private-key-policy.json"
done
reject private_key_input "${private_key_args[@]}"

/usr/bin/openssl genrsa -out "$TEST_DIR/weak-private.pem" 2048 >/dev/null 2>&1
/usr/bin/openssl rsa -in "$TEST_DIR/weak-private.pem" -pubout -out "$TEST_DIR/weak-public.pem" >/dev/null 2>&1
sign_approval "$APPROVAL" "$TEST_DIR/weak.sig" "$TEST_DIR/weak-private.pem"
weak_args=("${ARGS[@]}")
/usr/bin/jq --arg signer "$SIGNER" --arg hash "$(sha256 "$TEST_DIR/weak-public.pem")" \
  '{schema_version:1,kind:"hepta-ui-release-execution-approval-trust-v1",status:"ready",signer_id:$signer,public_key_sha256:$hash,signature_algorithm:"rsa-pkcs1-sha256",minimum_rsa_bits:3072}' \
  >"$TEST_DIR/weak-policy.json"
WEAK_POLICY_SHA="$(sha256 "$TEST_DIR/weak-policy.json")"
for ((i=0; i<${#weak_args[@]}; i+=2)); do
  [[ "${weak_args[$i]}" == --signature ]] && weak_args[$((i+1))]="$TEST_DIR/weak.sig"
  [[ "${weak_args[$i]}" == --public-key ]] && weak_args[$((i+1))]="$TEST_DIR/weak-public.pem"
  [[ "${weak_args[$i]}" == --trust-policy ]] && weak_args[$((i+1))]="$TEST_DIR/weak-policy.json"
done
mutated_weak="$TEST_DIR/weak-approval.json"
/usr/bin/jq --arg hash "$(sha256 "$TEST_DIR/weak-public.pem")" --arg policy_sha "$WEAK_POLICY_SHA" \
  '.signer.public_key_sha256=$hash | .action.trust_policy_sha256=$policy_sha' "$APPROVAL" >"$mutated_weak"
sign_approval "$mutated_weak" "$TEST_DIR/weak.sig" "$TEST_DIR/weak-private.pem"
for ((i=0; i<${#weak_args[@]}; i+=2)); do [[ "${weak_args[$i]}" == --approval ]] && weak_args[$((i+1))]="$mutated_weak"; done
reject weak_rsa_key "${weak_args[@]}"

/bin/ln -s "$APPROVAL" "$TEST_DIR/approval-link"
reject_with_override approval_symlink --approval "$TEST_DIR/approval-link"
/bin/ln -s "$TEST_DIR" "$TEST_DIR/parent-link"
reject_with_override approval_parent_symlink --approval "$TEST_DIR/parent-link/approval.json"
/bin/ln "$APPROVAL" "$TEST_DIR/approval-hardlink"
reject_with_override approval_hardlink --approval "$TEST_DIR/approval-hardlink"
/bin/ln -s "$TRUST_POLICY" "$TEST_DIR/trust-policy-link"
reject_with_override trust_policy_symlink --trust-policy "$TEST_DIR/trust-policy-link"
/bin/cp "$APPROVAL" "$TEST_DIR/writable-approval.json"
/bin/cp "$SIGNATURE" "$TEST_DIR/writable-approval.sig"
/bin/chmod 666 "$TEST_DIR/writable-approval.json"
writable_args=("${ARGS[@]}")
for ((i=0; i<${#writable_args[@]}; i+=2)); do
  [[ "${writable_args[$i]}" == --approval ]] && writable_args[$((i+1))]="$TEST_DIR/writable-approval.json"
  [[ "${writable_args[$i]}" == --signature ]] && writable_args[$((i+1))]="$TEST_DIR/writable-approval.sig"
done
reject writable_approval "${writable_args[@]}"

EXPIRED_ISSUED="$(/usr/bin/ruby --disable-gems -rtime -e 'print (Time.now.utc-1200).strftime("%Y-%m-%dT%H:%M:%SZ")')"
EXPIRED_AT="$(/usr/bin/ruby --disable-gems -rtime -e 'print (Time.now.utc-600).strftime("%Y-%m-%dT%H:%M:%SZ")')"
write_approval "$TEST_DIR/expired.json" "$EXPIRED_ISSUED" "$EXPIRED_AT"
sign_approval "$TEST_DIR/expired.json" "$TEST_DIR/expired.sig"
expired_args=("${ARGS[@]}")
for ((i=0; i<${#expired_args[@]}; i+=2)); do
  [[ "${expired_args[$i]}" == --approval ]] && expired_args[$((i+1))]="$TEST_DIR/expired.json"
  [[ "${expired_args[$i]}" == --signature ]] && expired_args[$((i+1))]="$TEST_DIR/expired.sig"
done
reject expired "${expired_args[@]}"

FUTURE_ISSUED="$(/usr/bin/ruby --disable-gems -rtime -e 'print (Time.now.utc+600).strftime("%Y-%m-%dT%H:%M:%SZ")')"
FUTURE_EXPIRES="$(/usr/bin/ruby --disable-gems -rtime -e 'print (Time.now.utc+1200).strftime("%Y-%m-%dT%H:%M:%SZ")')"
write_approval "$TEST_DIR/future.json" "$FUTURE_ISSUED" "$FUTURE_EXPIRES"
sign_approval "$TEST_DIR/future.json" "$TEST_DIR/future.sig"
future_args=("${ARGS[@]}")
for ((i=0; i<${#future_args[@]}; i+=2)); do
  [[ "${future_args[$i]}" == --approval ]] && future_args[$((i+1))]="$TEST_DIR/future.json"
  [[ "${future_args[$i]}" == --signature ]] && future_args[$((i+1))]="$TEST_DIR/future.sig"
done
reject future "${future_args[@]}"

# Production guard behavior is exercised without credentials or release side
# effects. Missing evidence and a complete approval without an exact prebuilt
# app both remain exit 77; a partial approval set is a usage error.
GUARD_DIR="$TEST_DIR/production-guard"
/bin/mkdir -p "$GUARD_DIR"
set +e
"$BUILD_SCRIPT" --output "$GUARD_DIR/missing.dmg" --receipt "$GUARD_DIR/missing.json" >"$GUARD_DIR/missing.stdout" 2>"$GUARD_DIR/missing.stderr"
MISSING_GUARD_STATUS=$?
"$BUILD_SCRIPT" --release-approval "$APPROVAL" --output "$GUARD_DIR/partial.dmg" --receipt "$GUARD_DIR/partial.json" >"$GUARD_DIR/partial.stdout" 2>"$GUARD_DIR/partial.stderr"
PARTIAL_GUARD_STATUS=$?
"$BUILD_SCRIPT" \
  --release-approval "$APPROVAL" \
  --release-approval-signature "$SIGNATURE" \
  --release-approval-public-key "$PUBLIC_KEY" \
  --output "$GUARD_DIR/complete.dmg" --receipt "$GUARD_DIR/complete.json" \
  >"$GUARD_DIR/complete.stdout" 2>"$GUARD_DIR/complete.stderr"
COMPLETE_GUARD_STATUS=$?
set -e
[[ "$MISSING_GUARD_STATUS" -eq 77 ]] || fail "missing production approval did not exit 77"
[[ "$PARTIAL_GUARD_STATUS" -eq 64 ]] || fail "partial production approval was not a usage error"
[[ "$COMPLETE_GUARD_STATUS" -eq 77 ]] || fail "approval without exact app did not exit 77"
[[ ! -e "$GUARD_DIR/missing.dmg" && ! -e "$GUARD_DIR/missing.json" \
  && ! -e "$GUARD_DIR/partial.dmg" && ! -e "$GUARD_DIR/partial.json" \
  && ! -e "$GUARD_DIR/complete.dmg" && ! -e "$GUARD_DIR/complete.json" ]] \
  || fail "early production guard created an output"

# The production wiring is statically ordered: exact source/app validation and
# approval verification both precede the first codesign invocation. Missing
# approval still exits 77 in the early guard.
/usr/bin/ruby --disable-gems -e '
  source = File.binread(ARGV.fetch(0))
  exact_input = source.index(%q{formal_app_receipt_not_exact_current_source}) or abort "exact input gate missing"
  approval = source.index(%q{if ! "$RELEASE_APPROVAL_VERIFIER"}) or abort "approval verifier wiring missing"
  entitlements_guard = source.index(%q{release_entitlements_changed_before_signing}) or abort "pre-sign entitlements guard missing"
  certificate_selector = source.index(%q{local arguments=(--force --sign "$SIGNING_CERTIFICATE_SHA1" --timestamp)}) or abort "exact certificate selector missing"
  first_sign = source.index(%q{codesign_with_retry "$SIGNED_APP/Contents/MacOS/hepta-native" app}) or abort "codesign action missing"
  early_exit = source.index(%q{signed_release_execution_approval_missing}) or abort "missing-approval exit missing"
  abort "identity label still selects signing certificate" if source.include?(%q{local arguments=(--force --sign "$SIGNING_IDENTITY" --timestamp)})
  abort "approval verifier not after exact input gate" unless exact_input < approval
  abort "entitlements guard not after approval verifier" unless approval < entitlements_guard
  abort "certificate selector not established before signing" unless certificate_selector < first_sign
  abort "approval verifier and final entitlements guard not before first signing" unless entitlements_guard < first_sign
  abort "missing approval guard not early" unless early_exit < exact_input
' "$BUILD_SCRIPT"

for forbidden in '/usr/bin/codesign' 'notarytool submit' 'stapler staple' '/usr/bin/curl' '/usr/bin/scp'; do
  ! /usr/bin/grep -F "$forbidden" "$VERIFIER" >/dev/null || fail "verifier contains forbidden action: $forbidden"
done

[[ "$NEGATIVE_COUNT" -ge 45 ]] || fail "negative matrix incomplete ($NEGATIVE_COUNT)"
/usr/bin/printf 'hepta release execution approval verifier self-test passed (%s negative cases)\n' "$NEGATIVE_COUNT"
