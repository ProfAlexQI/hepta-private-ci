#!/bin/bash -p
set +x
PS4='+ '
set -euo pipefail
unset BASH_ENV ENV CDPATH GLOBIGNORE RUBYOPT RUBYLIB GEM_HOME GEM_PATH BUNDLE_GEMFILE BUNDLE_PATH
SYSTEM_PATH="/usr/bin:/bin:/usr/sbin:/sbin"
PATH="$SYSTEM_PATH"
export PATH

ROOT="$(cd "$(/usr/bin/dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
FIXTURE_ARCHIVE="$ROOT/scripts/fixtures/hepta-ui-release-consumer-ready-v1.tar.gz"
FIXTURE_ARCHIVE_SHA256="9634d0b0a297e3d9f67122b1702d43a8e3a19f78a9484d78ebae470f9e198f5c"
CURRENT_PLAN_SCRIPT="$ROOT/scripts/hepta-ui-current-plan-refresh-gate.sh"
BLOCKER_SCRIPT="$ROOT/scripts/hepta-ui-blocker-closure-gate.sh"
ROOT_REPORT_SCRIPT="$ROOT/scripts/hepta-ui-root-report-replay-gate.sh"
TEST_ROOT="$(/usr/bin/mktemp -d /private/tmp/hepta-ui-release-consumer-forged-tuple.XXXXXX)"
UNPACK_ROOT="$TEST_ROOT/unpacked"
READY="$UNPACK_ROOT/ready-v1"

cleanup() {
  case "$TEST_ROOT" in
    /private/tmp/hepta-ui-release-consumer-forged-tuple.*)
      /bin/rm -rf -- "$TEST_ROOT"
      ;;
  esac
}
trap cleanup EXIT INT TERM

fail() {
  printf 'hepta-ui-release-consumer-forged-tuple-self-test: %s\n' "$1" >&2
  exit 1
}

sha256() {
  /usr/bin/shasum -a 256 "$1" | /usr/bin/awk '{print $1}'
}

validate_relative_path() {
  local path="$1"
  case "$path" in
    ""|/*|.|..|../*|*/../*|*/..|*//*|*/|*[!A-Za-z0-9._/-]*)
      fail "unsafe fixture path: $path"
      ;;
  esac
}

run_consumer() {
  local script="$1" stdout_path="$2" stderr_path="$3"
  /usr/bin/env -i \
    PATH="$SYSTEM_PATH" \
    HOME=/var/empty \
    TMPDIR=/private/tmp \
    HEPTA_UI_PRODUCT_READINESS_DIR="$READY" \
    HEPTA_UI_PRODUCT_READINESS_STRICT_CURRENT_SOURCE=0 \
    "$script" >"$stdout_path" 2>"$stderr_path"
}

run_control_consumer() {
  local label="$1" script="$2"
  local stdout_path="$TEST_ROOT/control-$label.stdout"
  local stderr_path="$TEST_ROOT/control-$label.stderr"
  if ! run_consumer "$script" "$stdout_path" "$stderr_path"; then
    /bin/cat "$stderr_path" >&2 || true
    fail "control $label consumer was rejected"
  fi
}

run_forged_consumer() {
  local label="$1" script="$2"
  local stdout_path="$TEST_ROOT/forged-$label.stdout"
  local stderr_path="$TEST_ROOT/forged-$label.stderr"
  local status
  set +e
  run_consumer "$script" "$stdout_path" "$stderr_path"
  status=$?
  set -e
  if [[ "$status" -ne 1 ]]; then
    /bin/cat "$stderr_path" >&2 || true
    fail "forged $label consumer exited $status instead of rejecting with status 1"
  fi
}

[[ -f "$FIXTURE_ARCHIVE" && ! -L "$FIXTURE_ARCHIVE" ]] \
  || fail "versioned ready fixture archive is missing or a symlink"
[[ "$(sha256 "$FIXTURE_ARCHIVE")" == "$FIXTURE_ARCHIVE_SHA256" ]] \
  || fail "versioned ready fixture archive SHA-256 mismatch"

ARCHIVE_MEMBERS="$TEST_ROOT/archive-members.txt"
ARCHIVE_MEMBERS_SORTED="$TEST_ROOT/archive-members.sorted.txt"
/usr/bin/tar -tzf "$FIXTURE_ARCHIVE" >"$ARCHIVE_MEMBERS"
[[ "$(/usr/bin/wc -l <"$ARCHIVE_MEMBERS" | /usr/bin/tr -d ' ')" == 48 ]] \
  || fail "versioned ready fixture archive member count mismatch"
while IFS= read -r member; do
  case "$member" in
    ready-v1/|ready-v1/native-fixture/)
      ;;
    ready-v1/*)
      relative_member="${member#ready-v1/}"
      validate_relative_path "$relative_member"
      ;;
    *)
      fail "versioned ready fixture archive has an escaping member: $member"
      ;;
  esac
done <"$ARCHIVE_MEMBERS"
/usr/bin/sort "$ARCHIVE_MEMBERS" >"$ARCHIVE_MEMBERS_SORTED"
[[ "$(/usr/bin/uniq "$ARCHIVE_MEMBERS_SORTED" | /usr/bin/wc -l | /usr/bin/tr -d ' ')" == 48 ]] \
  || fail "versioned ready fixture archive has duplicate members"
if /usr/bin/tar -tvzf "$FIXTURE_ARCHIVE" \
  | /usr/bin/awk 'substr($1, 1, 1) != "-" && substr($1, 1, 1) != "d" { bad=1 } END { exit bad ? 0 : 1 }'; then
  fail "versioned ready fixture archive contains a link or special file"
fi

/bin/mkdir -p "$UNPACK_ROOT"
/usr/bin/tar -xzf "$FIXTURE_ARCHIVE" -C "$UNPACK_ROOT"
[[ -d "$READY" && ! -L "$READY" ]] || fail "ready fixture root is not a real directory"
if /usr/bin/find "$READY" -mindepth 1 ! -type f ! -type d -print | /usr/bin/grep -q .; then
  fail "ready fixture contains a link or special file after extraction"
fi

FIXTURE_MANIFEST="$READY/fixture-manifest.json"
/usr/bin/jq -e '
  .schema == "hepta_ui_release_consumer_ready_fixture_v1"
  and .fixture_version == 1
  and .strict_current_source == false
  and .captured_path_metadata_inert == true
  and .report_count == 45
  and (.report_paths | type == "array" and length == 45 and length == (unique | length))
  and (.report_paths | all(type == "string" and length > 0))
  and (.report_paths | index("ui-current-plan-refresh-gate.json") == null)
  and (.report_paths | index("ui-blocker-closure-gate.json") == null)
  and (.report_paths | index("ui-root-report-replay-gate.json") == null)
' "$FIXTURE_MANIFEST" >/dev/null || fail "invalid ready fixture manifest"

EXPECTED_FILES="$TEST_ROOT/expected-files.txt"
ACTUAL_FILES="$TEST_ROOT/actual-files.txt"
EXPECTED_DIRECTORIES="$TEST_ROOT/expected-directories.txt"
ACTUAL_DIRECTORIES="$TEST_ROOT/actual-directories.txt"
/usr/bin/jq -r '.report_paths[], "fixture-manifest.json"' "$FIXTURE_MANIFEST" \
  | /usr/bin/sort >"$EXPECTED_FILES"
while IFS= read -r report_path; do
  validate_relative_path "$report_path"
  [[ -f "$READY/$report_path" && ! -L "$READY/$report_path" ]] \
    || fail "fixture report is missing or not a regular file: $report_path"
  /usr/bin/jq empty "$READY/$report_path" >/dev/null \
    || fail "fixture report is not valid JSON: $report_path"
done < <(/usr/bin/jq -r '.report_paths[]' "$FIXTURE_MANIFEST")
/usr/bin/find "$READY" -type f -print \
  | /usr/bin/sed "s#^$READY/##" \
  | /usr/bin/sort >"$ACTUAL_FILES"
/usr/bin/cmp "$EXPECTED_FILES" "$ACTUAL_FILES" >/dev/null \
  || fail "ready fixture file set differs from its exact manifest"
printf 'native-fixture\n' >"$EXPECTED_DIRECTORIES"
/usr/bin/find "$READY" -mindepth 1 -type d -print \
  | /usr/bin/sed "s#^$READY/##" \
  | /usr/bin/sort >"$ACTUAL_DIRECTORIES"
/usr/bin/cmp "$EXPECTED_DIRECTORIES" "$ACTUAL_DIRECTORIES" >/dev/null \
  || fail "ready fixture directory set is not minimal"

# Prove that the captured ready graph is accepted by the actual consumers in
# dependency order before any adversarial fields are introduced.
run_control_consumer current-plan "$CURRENT_PLAN_SCRIPT"
run_control_consumer blocker "$BLOCKER_SCRIPT"
run_control_consumer root "$ROOT_REPORT_SCRIPT"

CURRENT_REPORT="$READY/ui-current-plan-refresh-gate.json"
BLOCKER_REPORT="$READY/ui-blocker-closure-gate.json"
ROOT_REPORT="$READY/ui-root-report-replay-gate.json"
/usr/bin/jq -e --arg readiness "$READY" '
  .status == "ready"
  and .gate == "hepta_ui_current_plan_refresh_gate"
  and .readiness_dir == $readiness
  and .report_path == ($readiness + "/ui-current-plan-refresh-gate.json")
  and .source_alignment.release_artifact_intake_artifact_present == false
  and .source_alignment.release_artifact_intake_artifact_valid == false
  and .source_alignment.release_artifact_intake_present_artifact_branch_supported == false
  and .source_alignment.release_artifact_intake_independent_approval_verifier_contract_ready == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_present == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_valid == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_branch_supported == false
  and .source_alignment.release_artifact_roundtrip_independent_approval_verifier_contract_ready == false
  and .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
' "$CURRENT_REPORT" >/dev/null || fail "control current-plan report did not preserve fail-closed claims"
/usr/bin/jq -e --arg readiness "$READY" '
  .status == "ready"
  and .gate == "hepta_ui_blocker_closure_gate"
  and .readiness_dir == $readiness
  and .report_path == ($readiness + "/ui-blocker-closure-gate.json")
  and .source_alignment.release_artifact_present == false
  and .source_alignment.release_artifact_valid == false
  and .source_alignment.release_artifact_present_artifact_branch_supported == false
  and .source_alignment.release_artifact_independent_approval_verifier_contract_ready == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_present == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_valid == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_branch_supported == false
  and .source_alignment.release_artifact_roundtrip_independent_approval_verifier_contract_ready == false
  and .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
' "$BLOCKER_REPORT" >/dev/null || fail "control blocker report did not preserve fail-closed claims"
/usr/bin/jq -e --arg readiness "$READY" '
  .status == "ready"
  and .gate == "hepta_ui_root_report_replay_gate"
  and .product == "Hepta UI"
  and .runtime == "hepta"
  and .readiness_dir == $readiness
  and .report_path == ($readiness + "/ui-root-report-replay-gate.json")
  and .source_alignment.release_artifact_present == false
  and .source_alignment.release_artifact_valid == false
  and .source_alignment.release_artifact_intake_present_artifact_branch_supported == false
  and .source_alignment.release_artifact_intake_independent_approval_verifier_contract_ready == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_valid == false
  and .source_alignment.release_artifact_roundtrip_present_artifact_branch_supported == false
  and .source_alignment.release_artifact_roundtrip_independent_approval_verifier_contract_ready == false
  and .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
' "$ROOT_REPORT" >/dev/null || fail "control root report did not preserve fail-closed claims"

/bin/cp "$ROOT_REPORT" "$TEST_ROOT/control-root-report.json"
CURRENT_REPORT_SHA="$(sha256 "$CURRENT_REPORT")"
BLOCKER_REPORT_SHA="$(sha256 "$BLOCKER_REPORT")"
/usr/bin/printf 'operator-owned current sentinel\n' >"$READY/current-plan-refresh/operator-owned.txt"
/usr/bin/printf 'operator-owned blocker sentinel\n' >"$READY/blocker-closure/operator-owned.txt"

INTAKE_REPORT="$READY/ui-release-artifact-intake-gate.json"
ROUNDTRIP_REPORT="$READY/ui-release-artifact-roundtrip-gate.json"
/usr/bin/jq '
  .release_artifact_state.waiting_for_release_artifact=false
  | .release_artifact_state.release_artifact_present=true
  | .release_artifact_state.release_artifact_valid=true
  | .release_artifact_state.release_approval_valid=true
  | .release_artifact_state.receipt_contract_version=3
  | .release_artifact_state.evidence_readback_valid=true
  | .release_artifact_state.referenced_paths_absolute_and_unique=true
  | .release_artifact_state.signed_app_artifact_present=true
  | .release_artifact_state.notarized_app_artifact_present=true
  | .release_artifact_state.stapled_app_artifact_present=true
  | .release_artifact_state.stapled_dmg_artifact_present=true
  | .release_artifact_state.signed_notarized_stapled_artifact_present=true
  | .release_artifact_state.local_distribution_artifact_written=true
  | .release_artifact_state.public_distribution_artifact_written=true
  | .release_artifact_state.public_upload_performed=true
  | .source_alignment.release_approval_valid=true
  | .source_alignment.independent_approval_verifier_ready=true
  | .source_alignment.present_artifact_branch_supported=true
  | .source_alignment.independent_approval_verifier_contract_ready=true
  | .release_artifact_blockers=[]
  | .claim_boundary.release_approval_claim_ready=true
  | .claim_boundary.release_artifact_claim_ready=true
  | .claim_boundary.release_execution_ready=true
  | .claim_boundary.live_product_claim_ready=true
  | .claim_boundary.public_distribution_claim_ready=true
  | .claim_boundary.release_claim_ready=true
  | .claim_boundary.external_actions_allowed=true
  | .claim_boundary.public_upload_performed=true
  | .claim_boundary.signing_notarization_performed=true
' "$INTAKE_REPORT" >"$READY/.forged-intake.json"
/bin/mv "$READY/.forged-intake.json" "$INTAKE_REPORT"
/usr/bin/jq '
  .roundtrip_ready_count=3
  | .roundtrip_artifact_source_mode="copied_forged_present_public_tuple"
  | .source_alignment.waiting_branch_ready=true
  | .source_alignment.present_branch_ready=true
  | .source_alignment.present_artifact_branch_supported=true
  | .source_alignment.independent_approval_verifier_contract_ready=true
  | .source_alignment.present_branch_release_artifact_present=true
  | .source_alignment.present_branch_release_artifact_valid=true
  | .source_alignment.present_branch_signed_notarized_stapled_artifact_present=true
  | .source_alignment.present_branch_local_distribution_artifact_written=true
  | .source_alignment.present_branch_public_distribution_artifact_written=true
  | .source_alignment.present_branch_public_upload_performed=true
  | .source_alignment.present_branch_source_public_upload_performed=true
  | .source_alignment.present_branch_release_approval_valid=true
  | .source_alignment.present_branch_operator_release_approval_required=false
  | .source_alignment.present_branch_release_artifact_claim_ready=true
  | .source_alignment.present_branch_release_claim_ready=true
  | .release_artifact_blockers=[]
  | .claim_boundary.local_release_artifact_intake_present_branch_ready=true
  | .claim_boundary.release_artifact_claim_ready=true
  | .claim_boundary.release_execution_ready=true
  | .claim_boundary.live_product_claim_ready=true
  | .claim_boundary.public_distribution_claim_ready=true
  | .claim_boundary.release_claim_ready=true
  | .claim_boundary.external_actions_allowed=true
  | .claim_boundary.public_upload_performed=true
  | .claim_boundary.signing_notarization_performed=true
' "$ROUNDTRIP_REPORT" >"$READY/.forged-roundtrip.json"
/bin/mv "$READY/.forged-roundtrip.json" "$ROUNDTRIP_REPORT"

/usr/bin/jq -e '
  .release_artifact_state.waiting_for_release_artifact == false
  and .release_artifact_state.release_artifact_present == true
  and .release_artifact_state.release_artifact_valid == true
  and .release_artifact_state.signed_notarized_stapled_artifact_present == true
  and .release_artifact_state.public_distribution_artifact_written == true
  and .release_artifact_state.public_upload_performed == true
  and .source_alignment.present_artifact_branch_supported == true
  and .source_alignment.independent_approval_verifier_contract_ready == true
  and .claim_boundary.release_artifact_claim_ready == true
  and .claim_boundary.public_distribution_claim_ready == true
  and .claim_boundary.release_claim_ready == true
' "$INTAKE_REPORT" >/dev/null || fail "forged intake tuple was not constructed"
/usr/bin/jq -e '
  .source_alignment.present_branch_ready == true
  and .source_alignment.present_branch_release_artifact_present == true
  and .source_alignment.present_branch_release_artifact_valid == true
  and .source_alignment.present_branch_signed_notarized_stapled_artifact_present == true
  and .source_alignment.present_branch_public_distribution_artifact_written == true
  and .source_alignment.present_branch_public_upload_performed == true
  and .source_alignment.present_artifact_branch_supported == true
  and .source_alignment.independent_approval_verifier_contract_ready == true
  and .claim_boundary.release_artifact_claim_ready == true
  and .claim_boundary.public_distribution_claim_ready == true
  and .claim_boundary.release_claim_ready == true
' "$ROUNDTRIP_REPORT" >/dev/null || fail "forged roundtrip tuple was not constructed"
INTAKE_REPORT_SHA="$(sha256 "$INTAKE_REPORT")"
ROUNDTRIP_REPORT_SHA="$(sha256 "$ROUNDTRIP_REPORT")"

# Execute each real consumer against the same copied forged tuple. Current and
# blocker must reject without replacing their last false-claim reports. Root
# must clear only its verified owned stale report and emit no replacement.
run_forged_consumer current-plan "$CURRENT_PLAN_SCRIPT"
[[ "$(sha256 "$CURRENT_REPORT")" == "$CURRENT_REPORT_SHA" ]] \
  || fail "forged current-plan run replaced the prior control report"
/usr/bin/jq -e '
  .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
' "$CURRENT_REPORT" >/dev/null || fail "forged current-plan run left a positive claim"
[[ "$(/bin/cat "$READY/current-plan-refresh/operator-owned.txt")" == "operator-owned current sentinel" ]] \
  || fail "forged current-plan run changed caller-owned directory content"

run_forged_consumer blocker "$BLOCKER_SCRIPT"
[[ "$(sha256 "$BLOCKER_REPORT")" == "$BLOCKER_REPORT_SHA" ]] \
  || fail "forged blocker run replaced the prior control report"
/usr/bin/jq -e '
  .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
' "$BLOCKER_REPORT" >/dev/null || fail "forged blocker run left a positive claim"
[[ "$(/bin/cat "$READY/blocker-closure/operator-owned.txt")" == "operator-owned blocker sentinel" ]] \
  || fail "forged blocker run changed caller-owned directory content"

run_forged_consumer root "$ROOT_REPORT_SCRIPT"
[[ ! -e "$ROOT_REPORT" ]] || fail "forged root run left a claim-bearing report"
/usr/bin/jq -e '
  .claim_boundary.release_artifact_claim_ready == false
  and .claim_boundary.public_distribution_claim_ready == false
  and .claim_boundary.release_claim_ready == false
' "$TEST_ROOT/control-root-report.json" >/dev/null \
  || fail "control root evidence was not fail closed before forged replay"
[[ "$(sha256 "$INTAKE_REPORT")" == "$INTAKE_REPORT_SHA" ]] \
  || fail "consumer replay changed the forged intake source"
[[ "$(sha256 "$ROUNDTRIP_REPORT")" == "$ROUNDTRIP_REPORT_SHA" ]] \
  || fail "consumer replay changed the forged roundtrip source"

printf 'hepta-ui release consumer forged tuple self-test: PASS\n'
