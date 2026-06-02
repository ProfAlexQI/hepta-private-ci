#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT"

PREFLIGHT_PATH="${HEPTA_PREFLIGHT_TERMINAL_COVERAGE_PREFLIGHT_PATH:-scripts/hepta-preflight.sh}"
PREFLIGHT_TEXT="${HEPTA_PREFLIGHT_TERMINAL_COVERAGE_PREFLIGHT_TEXT:-}"
MIN_MARKER_COUNT="${HEPTA_PREFLIGHT_TERMINAL_COVERAGE_MIN_MARKER_COUNT:-160}"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

json_lines() {
  jq -R -s 'split("\n") | map(select(length > 0))'
}

required_markers=(
  "metadata"
  "fmt"
  "cargo check"
  "adapter behavior-equivalence gate"
  "adapter shadow-replay gate"
  "name/repository closure gate"
  "active service dependency isolation gate"
  "legacy preflight entrypoint migration gate"
  "legacy watchdog entrypoint migration gate"
  "legacy live gates entrypoint migration gate"
  "legacy release/readiness entrypoint migration gate"
  "legacy inventory entrypoint migration gate"
  "memory-rem status closure gate"
  "memory-tools catalog closure gate"
  "native residual runtime status closure gate"
  "plugin migration plan closure gate"
  "skill workshop plan closure gate"
  "memory/intelligence closure gate"
  "KG prompt-preview preflight gate"
  "KG prompt-preview terminal next-action activation denial summary gate"
  "KG prompt-preview memory/intelligence full enablement activation readiness gate"
  "memory/intelligence full enablement memory live mutation staging fixture gate"
  "memory/intelligence full enablement KG external adapter staging receipt gate"
  "memory/intelligence full enablement bounded prompt-preview context handoff activation packet gate"
  "memory/intelligence full enablement runtime provider-router context attachment staging gate"
  "memory/intelligence full enablement runtime provider-router context attachment negative fixture matrix gate"
  "memory/intelligence full enablement runtime provider-router readback receipt skeleton gate"
  "memory/intelligence full enablement runtime provider-router receipt observability denial gate"
  "memory/intelligence full enablement runtime provider-router operator-facing summary non-persistence gate"
  "memory/intelligence full enablement runtime provider-router operator acknowledgement non-acceptance gate"
  "memory/intelligence full enablement runtime provider-router activation request denial matrix gate"
  "memory/intelligence full enablement runtime provider-router activation command no-op handoff gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt no-persistence gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt replay idempotency denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt ordering monotonicity denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt cancellation supersession denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt audit trail immutable evidence denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt retention expiry garbage collection denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt export query observability denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt operator-facing summary briefing non-persistence denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt final operator acknowledgement non-acceptance denial gate"
  "memory/intelligence full enablement runtime provider-router activation command result receipt terminal operator decision public-claim non-promotion denial gate"
  "readiness denial review acceptance closure summary gate"
  "upstream Codex promotion closure gate"
  "terminal release-governance final audit index gate"
  "operator-security attention-budget diagnostic gate"
  "terminal watchdog/soak regression gate"
  "core activation evidence receipt terminal closure decision gate"
  "core activation terminal closure gap evidence index gate"
  "core activation terminal closure operator packet template gate"
  "core activation terminal closure operator packet dry-run validator gate"
  "core activation terminal closure operator packet authority replay matrix gate"
  "core activation terminal closure operator packet trusted-record acceptance skeleton gate"
  "core activation terminal closure operator packet trusted-record acceptance negative-fixture matrix gate"
  "core activation terminal closure operator packet trusted-record acceptance precondition scoreboard gate"
  "core activation terminal closure operator packet trusted-record positive packet dry-run scaffold gate"
  "core activation terminal closure operator packet trusted-record positive packet authority replay denial matrix gate"
  "core activation terminal closure operator packet trusted-record positive packet authority replay denial summary gate"
  "core activation terminal closure operator packet trusted-record positive packet authority replay denial summary index gate"
  "core activation terminal closure operator packet trusted-record positive packet authority replay denial summary index manifest gate"
  "core activation terminal closure operator packet trusted-record positive packet authority replay denial summary index manifest JSON-capture boundary gate"
  "core activation terminal closure operator packet trusted-record positive packet operator approval gap ledger gate"
  "JSON report capture diagnostic contract gate"
  "JSON report capture migration inventory gate"
  "preflight terminal coverage inventory gate"
  "preflight terminal coverage diagnostic contract gate"
  "upstream Codex latest active-safety regression gate"
  "upstream Codex latest release-governance non-activation gate"
  "upstream Codex latest operator briefing non-persistence gate"
  "hepta-gateway tests"
  "codex-cli native tests"
  "control-ui smoke"
  "native app metadata/check/tests"
  "release build compatibility codex-cli"
  "release build active hepta-cli"
  "whitespace/status"
)

phase_family_ids=(
  "early-core-spine"
  "legacy-migration-closure"
  "kg-prompt-preview-readiness"
  "live-mutation-denial"
  "readiness-denial-closure"
  "upstream-codex-absorption-activation"
  "terminal-governance-release"
  "core-activation-tail"
  "json-terminal-coverage"
  "latest-regression-and-tests"
)

phase_family_min_counts=(
  7
  11
  10
  54
  1
  44
  11
  23
  4
  12
)

phase_family_anchor_specs=(
  "early-core-spine|metadata"
  "early-core-spine|active service dependency isolation gate"
  "legacy-migration-closure|legacy preflight entrypoint migration gate"
  "legacy-migration-closure|memory/intelligence closure gate"
  "kg-prompt-preview-readiness|KG prompt-preview preflight gate"
  "kg-prompt-preview-readiness|KG prompt-preview operator approval checklist schema gate"
  "kg-prompt-preview-readiness|KG prompt-preview terminal next-action activation denial summary gate"
  "live-mutation-denial|live mutation governance gate"
  "live-mutation-denial|memory live mutation operator write execution activation command result receipt final operator acknowledgement non-acceptance denial gate"
  "live-mutation-denial|live mutation pre-activation soak evidence persistence payload redaction acceptance receipt filesystem persistence ledger persistence rehearsal receipt review acceptance scoreboard review acceptance readiness denial review acceptance closure gate"
  "readiness-denial-closure|readiness denial review acceptance closure summary gate"
  "upstream-codex-absorption-activation|upstream Codex snapshot gate"
  "upstream-codex-absorption-activation|upstream Codex activation evidence receipt filesystem persistence execution denial matrix gate"
  "upstream-codex-absorption-activation|upstream Codex sync lane gate"
  "terminal-governance-release|terminal denial index gate"
  "terminal-governance-release|terminal public distribution non-publication lock gate"
  "terminal-governance-release|terminal release-governance final audit index gate"
  "core-activation-tail|core activation long-soak observation non-acceptance gate"
  "core-activation-tail|core activation fresh long-soak evidence ledger receipt gate"
  "core-activation-tail|core activation evidence receipt terminal closure decision gate"
  "core-activation-tail|core activation terminal closure gap evidence index gate"
  "core-activation-tail|core activation terminal closure operator packet template gate"
  "core-activation-tail|core activation terminal closure operator packet dry-run validator gate"
  "core-activation-tail|core activation terminal closure operator packet authority replay matrix gate"
  "core-activation-tail|core activation terminal closure operator packet trusted-record acceptance skeleton gate"
  "core-activation-tail|core activation terminal closure operator packet trusted-record acceptance negative-fixture matrix gate"
  "core-activation-tail|core activation terminal closure operator packet trusted-record acceptance precondition scoreboard gate"
  "core-activation-tail|core activation terminal closure operator packet trusted-record positive packet dry-run scaffold gate"
  "core-activation-tail|core activation terminal closure operator packet trusted-record positive packet authority replay denial matrix gate"
  "core-activation-tail|core activation terminal closure operator packet trusted-record positive packet authority replay denial summary gate"
  "core-activation-tail|core activation terminal closure operator packet trusted-record positive packet authority replay denial summary index gate"
  "core-activation-tail|core activation terminal closure operator packet trusted-record positive packet authority replay denial summary index manifest gate"
  "core-activation-tail|core activation terminal closure operator packet trusted-record positive packet authority replay denial summary index manifest JSON-capture boundary gate"
  "core-activation-tail|core activation terminal closure operator packet trusted-record positive packet operator approval gap ledger gate"
  "json-terminal-coverage|JSON report capture diagnostic contract gate"
  "json-terminal-coverage|JSON report capture migration inventory gate"
  "json-terminal-coverage|preflight terminal coverage inventory gate"
  "json-terminal-coverage|preflight terminal coverage diagnostic contract gate"
  "latest-regression-and-tests|upstream Codex latest active-safety regression gate"
  "latest-regression-and-tests|upstream Codex latest operator briefing non-persistence gate"
  "latest-regression-and-tests|hepta-gateway tests"
  "latest-regression-and-tests|codex-cli native tests"
  "latest-regression-and-tests|whitespace/status"
)

marker_matches_phase_family() {
  local family_id="$1"
  local marker="$2"

  case "$family_id" in
    early-core-spine)
      case "$marker" in
        "metadata"|"fmt"|"cargo check"|"adapter behavior-equivalence gate"|"adapter shadow-replay gate"|"name/repository closure gate"|"active service dependency isolation gate")
          return 0
          ;;
      esac
      ;;
    legacy-migration-closure)
      if [[ "$marker" == legacy*" entrypoint migration gate" \
        || "$marker" == "memory-rem status closure gate" \
        || "$marker" == "memory-tools catalog closure gate" \
        || "$marker" == "native residual runtime status closure gate" \
        || "$marker" == "plugin migration plan closure gate" \
        || "$marker" == "skill workshop plan closure gate" \
        || "$marker" == "memory/intelligence closure gate" ]]; then
        return 0
      fi
      ;;
    kg-prompt-preview-readiness)
      if [[ "$marker" == KG\ prompt-preview* ]]; then
        return 0
      fi
      ;;
    live-mutation-denial)
      if [[ "$marker" == *"live mutation"* ]]; then
        return 0
      fi
      ;;
    readiness-denial-closure)
      if [[ "$marker" == "readiness denial review acceptance closure summary gate" ]]; then
        return 0
      fi
      ;;
    upstream-codex-absorption-activation)
      if [[ "$marker" == upstream\ Codex* \
        && "$marker" != upstream\ Codex\ latest\ active-safety* \
        && "$marker" != upstream\ Codex\ latest\ release-governance* \
        && "$marker" != upstream\ Codex\ latest\ operator\ briefing* ]]; then
        return 0
      fi
      ;;
    terminal-governance-release)
      if [[ "$marker" == terminal\ * \
        || "$marker" == "operator-security attention-budget diagnostic gate" ]]; then
        return 0
      fi
      ;;
    core-activation-tail)
      if [[ "$marker" == core\ activation* ]]; then
        return 0
      fi
      ;;
    json-terminal-coverage)
      if [[ "$marker" == JSON\ report\ capture* \
        || "$marker" == preflight\ terminal\ coverage* ]]; then
        return 0
      fi
      ;;
    latest-regression-and-tests)
      if [[ "$marker" == upstream\ Codex\ latest\ active-safety* \
        || "$marker" == upstream\ Codex\ latest\ release-governance* \
        || "$marker" == upstream\ Codex\ latest\ operator\ briefing* \
        || "$marker" == "hepta-gateway tests" \
        || "$marker" == "codex-cli native tests" \
        || "$marker" == "control-ui smoke" \
        || "$marker" == "native app metadata/check/tests" \
        || "$marker" == native\ app\ gates\ skipped* \
        || "$marker" == "release build compatibility codex-cli" \
        || "$marker" == "release build active hepta-cli" \
        || "$marker" == release\ build\ skipped* \
        || "$marker" == "whitespace/status" ]]; then
        return 0
      fi
      ;;
  esac

  return 1
}

marker_present_exact() {
  local target="$1"
  local marker

  for marker in "${preflight_markers[@]}"; do
    if [[ "$marker" == "$target" ]]; then
      return 0
    fi
  done

  return 1
}

inline_fixture_mode=false
if [[ -n "$PREFLIGHT_TEXT" ]]; then
  inline_fixture_mode=true
fi

preflight_exists=false
if [[ "$inline_fixture_mode" == true || -x "$PREFLIGHT_PATH" ]]; then
  preflight_exists=true
fi

syntax_ok=false
if [[ "$inline_fixture_mode" == true ]]; then
  if printf '%s\n' "$PREFLIGHT_TEXT" | bash -n; then
    syntax_ok=true
  fi
else
  if bash -n "$PREFLIGHT_PATH"; then
    syntax_ok=true
  fi
fi

preflight_markers=()
if [[ "$inline_fixture_mode" == true ]]; then
  while IFS= read -r marker; do
    preflight_markers+=("$marker")
  done < <(
    printf '%s\n' "$PREFLIGHT_TEXT" \
      | grep -E '^[[:space:]]*echo "\[hepta-preflight\] ' \
      | sed -E 's/^[[:space:]]*echo "\[hepta-preflight\] (.*)"$/\1/' \
      || true
  )
else
  while IFS= read -r marker; do
    preflight_markers+=("$marker")
  done < <(
    grep -E '^[[:space:]]*echo "\[hepta-preflight\] ' "$PREFLIGHT_PATH" \
      | sed -E 's/^[[:space:]]*echo "\[hepta-preflight\] (.*)"$/\1/' \
      || true
  )
fi

marker_count="${#preflight_markers[@]}"
marker_count_budget_ok=false
if [[ "$marker_count" -ge "$MIN_MARKER_COUNT" ]]; then
  marker_count_budget_ok=true
fi

phase_family_records=()
phase_family_failures=()
phase_family_ready_count=0

for index in "${!phase_family_ids[@]}"; do
  family_id="${phase_family_ids[$index]}"
  min_count="${phase_family_min_counts[$index]}"
  current_count=0

  for marker in "${preflight_markers[@]}"; do
    if marker_matches_phase_family "$family_id" "$marker"; then
      current_count=$((current_count + 1))
    fi
  done

  family_ready=false
  if [[ "$current_count" -ge "$min_count" ]]; then
    family_ready=true
    phase_family_ready_count=$((phase_family_ready_count + 1))
  else
    phase_family_failures+=("$family_id|$current_count|$min_count")
  fi

  phase_family_records+=("$family_id|$current_count|$min_count|$family_ready")
done

phase_family_count="${#phase_family_ids[@]}"
phase_family_budget_failure_count="${#phase_family_failures[@]}"
phase_family_budget_ready=false
if [[ "$phase_family_budget_failure_count" -eq 0 ]]; then
  phase_family_budget_ready=true
fi

phase_family_anchor_records=()
phase_family_anchor_failures=()
phase_family_anchor_ready_count=0
phase_family_anchor_count="${#phase_family_anchor_specs[@]}"

for anchor_spec in "${phase_family_anchor_specs[@]}"; do
  family_id="${anchor_spec%%|*}"
  anchor_marker="${anchor_spec#*|}"
  anchor_ready=false

  if marker_present_exact "$anchor_marker"; then
    anchor_ready=true
    phase_family_anchor_ready_count=$((phase_family_anchor_ready_count + 1))
  else
    phase_family_anchor_failures+=("$family_id|$anchor_marker")
  fi

  phase_family_anchor_records+=("$family_id|$anchor_marker|$anchor_ready")
done

phase_family_anchor_failure_count="${#phase_family_anchor_failures[@]}"
phase_family_anchor_ready=false
if [[ "$phase_family_anchor_failure_count" -eq 0 ]]; then
  phase_family_anchor_ready=true
fi

terminal_pass_marker_present=false
if [[ "$inline_fixture_mode" == true ]]; then
  if grep -q '^echo "Hepta preflight passed"$' <<<"$PREFLIGHT_TEXT"; then
    terminal_pass_marker_present=true
  fi
elif grep -q '^echo "Hepta preflight passed"$' "$PREFLIGHT_PATH"; then
  terminal_pass_marker_present=true
fi

native_release_skip_branches_present=false
if [[ "$inline_fixture_mode" == true ]]; then
  if grep -q 'native app gates skipped' <<<"$PREFLIGHT_TEXT" \
    && grep -q 'release build skipped' <<<"$PREFLIGHT_TEXT" \
    && grep -q 'HEPTA_PREFLIGHT_NATIVE' <<<"$PREFLIGHT_TEXT" \
    && grep -q 'HEPTA_PREFLIGHT_RELEASE' <<<"$PREFLIGHT_TEXT"; then
    native_release_skip_branches_present=true
  fi
else
  if grep -q 'native app gates skipped' "$PREFLIGHT_PATH" \
    && grep -q 'release build skipped' "$PREFLIGHT_PATH" \
    && grep -q 'HEPTA_PREFLIGHT_NATIVE' "$PREFLIGHT_PATH" \
    && grep -q 'HEPTA_PREFLIGHT_RELEASE' "$PREFLIGHT_PATH"; then
    native_release_skip_branches_present=true
  fi
fi

final_workspace_diff_check_present=false
final_cached_diff_check_present=false
final_git_status_present=false
line_for_literal() {
  local target="$1"
  local line
  if [[ "$inline_fixture_mode" == true ]]; then
    line="$(
      printf '%s\n' "$PREFLIGHT_TEXT" \
        | awk -v target="$target" '$0 == target { print NR; exit }'
    )"
  else
    line="$(
      awk -v target="$target" '$0 == target { print NR; exit }' "$PREFLIGHT_PATH"
    )"
  fi
  printf '%s' "${line:-0}"
}

whitespace_status_marker_line="$(line_for_literal 'echo "[hepta-preflight] whitespace/status"')"
terminal_pass_marker_line="$(line_for_literal 'echo "Hepta preflight passed"')"
workspace_diff_check_line="$(line_for_literal 'git diff --check')"
cached_diff_check_line="$(line_for_literal 'git diff --cached --check')"
git_status_line="$(line_for_literal 'git status -sb')"

if [[ "$inline_fixture_mode" == true ]]; then
  if grep -Eq '^[[:space:]]*git diff --check[[:space:]]*$' <<<"$PREFLIGHT_TEXT"; then
    final_workspace_diff_check_present=true
  fi
  if grep -Eq '^[[:space:]]*git diff --cached --check[[:space:]]*$' <<<"$PREFLIGHT_TEXT"; then
    final_cached_diff_check_present=true
  fi
  if grep -Eq '^[[:space:]]*git status -sb[[:space:]]*$' <<<"$PREFLIGHT_TEXT"; then
    final_git_status_present=true
  fi
else
  if grep -Eq '^[[:space:]]*git diff --check[[:space:]]*$' "$PREFLIGHT_PATH"; then
    final_workspace_diff_check_present=true
  fi
  if grep -Eq '^[[:space:]]*git diff --cached --check[[:space:]]*$' "$PREFLIGHT_PATH"; then
    final_cached_diff_check_present=true
  fi
  if grep -Eq '^[[:space:]]*git status -sb[[:space:]]*$' "$PREFLIGHT_PATH"; then
    final_git_status_present=true
  fi
fi

final_status_checks_present=false
if [[ "$final_workspace_diff_check_present" == true \
  && "$final_cached_diff_check_present" == true \
  && "$final_git_status_present" == true ]]; then
  final_status_checks_present=true
fi

final_status_checks_ordered=false
if [[ "$final_status_checks_present" == true \
  && "$whitespace_status_marker_line" -gt 0 \
  && "$terminal_pass_marker_line" -gt 0 \
  && "$workspace_diff_check_line" -gt "$whitespace_status_marker_line" \
  && "$cached_diff_check_line" -gt "$workspace_diff_check_line" \
  && "$git_status_line" -gt "$cached_diff_check_line" \
  && "$git_status_line" -lt "$terminal_pass_marker_line" ]]; then
  final_status_checks_ordered=true
fi

final_status_checks_ready=false
if [[ "$final_status_checks_present" == true \
  && "$final_status_checks_ordered" == true ]]; then
  final_status_checks_ready=true
fi

missing_markers=()
duplicate_markers=()
out_of_order_markers=()
required_marker_lines=()
previous_line=0
ordered_markers=true
present_required_marker_count=0

for marker in "${required_markers[@]}"; do
  lines=()
  if [[ "$inline_fixture_mode" == true ]]; then
    while IFS= read -r line; do
      lines+=("$line")
    done < <(
      grep -nF "echo \"[hepta-preflight] $marker\"" <<<"$PREFLIGHT_TEXT" \
        | cut -d: -f1 \
        || true
    )
  else
    while IFS= read -r line; do
      lines+=("$line")
    done < <(
      grep -nF "echo \"[hepta-preflight] $marker\"" "$PREFLIGHT_PATH" \
        | cut -d: -f1 \
        || true
    )
  fi
  line_count="${#lines[@]}"
  if [[ "$line_count" -eq 0 ]]; then
    missing_markers+=("$marker")
    required_marker_lines+=("$marker|0")
    continue
  fi
  if [[ "$line_count" -gt 1 ]]; then
    duplicate_markers+=("$marker")
  fi
  present_required_marker_count=$((present_required_marker_count + 1))
  line="${lines[0]}"
  required_marker_lines+=("$marker|$line")
  if [[ "$line" -le "$previous_line" ]]; then
    ordered_markers=false
    out_of_order_markers+=("$marker")
  fi
  previous_line="$line"
done

coverage_ready=false
if [[ "$preflight_exists" == true \
  && "$syntax_ok" == true \
  && "$marker_count_budget_ok" == true \
  && "$phase_family_budget_ready" == true \
  && "$phase_family_anchor_ready" == true \
  && "$terminal_pass_marker_present" == true \
  && "$native_release_skip_branches_present" == true \
  && "$final_status_checks_ready" == true \
  && "${#missing_markers[@]}" -eq 0 \
  && "${#duplicate_markers[@]}" -eq 0 \
  && "$ordered_markers" == true ]]; then
  coverage_ready=true
fi

required_markers_json="$(printf '%s\n' "${required_markers[@]}" | json_lines)"
preflight_markers_json="$(printf '%s\n' "${preflight_markers[@]}" | json_lines)"
missing_markers_json="$(printf '%s\n' "${missing_markers[@]}" | json_lines)"
duplicate_markers_json="$(printf '%s\n' "${duplicate_markers[@]}" | json_lines)"
out_of_order_markers_json="$(printf '%s\n' "${out_of_order_markers[@]}" | json_lines)"
required_marker_lines_json="$(
  printf '%s\n' "${required_marker_lines[@]}" \
    | jq -R -s '
        split("\n")
        | map(select(length > 0))
        | map(split("|") | {marker: .[0], line: (.[1] | tonumber)})
      '
)"
phase_family_records_json="$(
  printf '%s\n' "${phase_family_records[@]}" \
    | jq -R -s '
        split("\n")
        | map(select(length > 0))
        | map(split("|") | {
            id: .[0],
            current_count: (.[1] | tonumber),
            minimum_count: (.[2] | tonumber),
            ready: (.[3] == "true"),
            blocked: true
          })
      '
)"
phase_family_failures_json="$(
  printf '%s\n' "${phase_family_failures[@]}" \
    | jq -R -s '
        split("\n")
        | map(select(length > 0))
        | map(split("|") | {
            id: .[0],
            current_count: (.[1] | tonumber),
            minimum_count: (.[2] | tonumber)
          })
      '
)"
phase_family_anchor_records_json="$(
  printf '%s\n' "${phase_family_anchor_records[@]}" \
    | jq -R -s '
        split("\n")
        | map(select(length > 0))
        | map(split("|") | {
            family_id: .[0],
            marker: .[1],
            ready: (.[2] == "true"),
            blocked: true
          })
      '
)"
phase_family_anchor_failures_json="$(
  printf '%s\n' "${phase_family_anchor_failures[@]}" \
    | jq -R -s '
        split("\n")
        | map(select(length > 0))
        | map(split("|") | {
            family_id: .[0],
            marker: .[1]
          })
      '
)"
phase_family_ids_json="$(printf '%s\n' "${phase_family_ids[@]}" | json_lines)"
phase_family_anchor_family_coverage_json="$(
  jq -n \
    --argjson family_ids "$phase_family_ids_json" \
    --argjson anchor_records "$phase_family_anchor_records_json" \
    '
      $family_ids
      | map(. as $family_id
        | ($anchor_records | map(select(.family_id == $family_id))) as $family_anchors
        | {
            id: $family_id,
            required_anchor_count: ($family_anchors | length),
            ready_anchor_count: ($family_anchors | map(select(.ready == true)) | length),
            missing_anchor_count: ($family_anchors | map(select(.ready != true)) | length),
            ready: (
              ($family_anchors | length) > 0
              and (($family_anchors | map(select(.ready == true)) | length) == ($family_anchors | length))
            ),
            blocked: true,
            anchors: $family_anchors,
            missing_anchors: ($family_anchors | map(select(.ready != true) | .marker))
          }
      )
    '
)"
phase_family_anchor_family_count="${#phase_family_ids[@]}"
phase_family_anchor_family_ready_count="$(
  jq '[.[] | select(.ready == true)] | length' <<<"$phase_family_anchor_family_coverage_json"
)"
phase_family_anchor_family_failure_count="$(
  jq '[.[] | select(.ready != true)] | length' <<<"$phase_family_anchor_family_coverage_json"
)"
phase_family_anchor_family_ready=false
if [[ "$phase_family_anchor_family_failure_count" -eq 0 ]]; then
  phase_family_anchor_family_ready=true
fi

inventory_hash_sha256="$(
  sha256_text "$marker_count:$present_required_marker_count:$MIN_MARKER_COUNT:$phase_family_budget_ready:${phase_family_records[*]}:$phase_family_anchor_ready:${phase_family_anchor_records[*]}:$phase_family_anchor_family_ready:$phase_family_anchor_family_ready_count:$terminal_pass_marker_present:$native_release_skip_branches_present:$final_status_checks_ready:${required_marker_lines[*]}"
)"
policy_hash_sha256="$(sha256_text "hepta-preflight-terminal-coverage:static-inventory:no-run:no-write:no-restart")"
side_effect_hash_sha256="$(sha256_text "filesystem_written=false;runtime_mutation=false;service_restarted=false;external_send=false;secret_read=false")"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "$(if [[ "$coverage_ready" == true ]]; then echo ready; else echo attention; fi)" \
  --arg gate "hepta_preflight_terminal_coverage_inventory_gate" \
  --arg schema_version "hepta_preflight_terminal_coverage_inventory_v1" \
  --arg mode "static_preflight_marker_inventory_no_child_gate_execution" \
  --arg decision "terminal_preflight_coverage_and_order_are_machine_readable_without_running_release_or_native_gates" \
  --arg preflight_path "$PREFLIGHT_PATH" \
  --arg inventory_hash_sha256 "$inventory_hash_sha256" \
  --arg policy_hash_sha256 "$policy_hash_sha256" \
  --arg side_effect_hash_sha256 "$side_effect_hash_sha256" \
  --argjson ready "$coverage_ready" \
  --argjson inline_fixture_mode "$inline_fixture_mode" \
  --argjson preflight_exists "$preflight_exists" \
  --argjson syntax_ok "$syntax_ok" \
  --argjson marker_count "$marker_count" \
  --argjson min_marker_count "$MIN_MARKER_COUNT" \
  --argjson marker_count_budget_ok "$marker_count_budget_ok" \
  --argjson phase_family_count "$phase_family_count" \
  --argjson phase_family_ready_count "$phase_family_ready_count" \
  --argjson phase_family_budget_failure_count "$phase_family_budget_failure_count" \
  --argjson phase_family_budget_ready "$phase_family_budget_ready" \
  --argjson phase_family_anchor_count "$phase_family_anchor_count" \
  --argjson phase_family_anchor_ready_count "$phase_family_anchor_ready_count" \
  --argjson phase_family_anchor_failure_count "$phase_family_anchor_failure_count" \
  --argjson phase_family_anchor_ready "$phase_family_anchor_ready" \
  --argjson phase_family_anchor_family_count "$phase_family_anchor_family_count" \
  --argjson phase_family_anchor_family_ready_count "$phase_family_anchor_family_ready_count" \
  --argjson phase_family_anchor_family_failure_count "$phase_family_anchor_family_failure_count" \
  --argjson phase_family_anchor_family_ready "$phase_family_anchor_family_ready" \
  --argjson required_marker_count "${#required_markers[@]}" \
  --argjson present_required_marker_count "$present_required_marker_count" \
  --argjson ordered_markers "$ordered_markers" \
  --argjson terminal_pass_marker_present "$terminal_pass_marker_present" \
  --argjson native_release_skip_branches_present "$native_release_skip_branches_present" \
  --argjson final_workspace_diff_check_present "$final_workspace_diff_check_present" \
  --argjson final_cached_diff_check_present "$final_cached_diff_check_present" \
  --argjson final_git_status_present "$final_git_status_present" \
  --argjson final_status_checks_present "$final_status_checks_present" \
  --argjson final_status_checks_ordered "$final_status_checks_ordered" \
  --argjson final_status_checks_ready "$final_status_checks_ready" \
  --argjson whitespace_status_marker_line "$whitespace_status_marker_line" \
  --argjson terminal_pass_marker_line "$terminal_pass_marker_line" \
  --argjson workspace_diff_check_line "$workspace_diff_check_line" \
  --argjson cached_diff_check_line "$cached_diff_check_line" \
  --argjson git_status_line "$git_status_line" \
  --argjson required_markers "$required_markers_json" \
  --argjson preflight_markers "$preflight_markers_json" \
  --argjson missing_markers "$missing_markers_json" \
  --argjson duplicate_markers "$duplicate_markers_json" \
  --argjson out_of_order_markers "$out_of_order_markers_json" \
  --argjson required_marker_lines "$required_marker_lines_json" \
  --argjson phase_family_records "$phase_family_records_json" \
  --argjson phase_family_failures "$phase_family_failures_json" \
  --argjson phase_family_anchor_records "$phase_family_anchor_records_json" \
  --argjson phase_family_anchor_failures "$phase_family_anchor_failures_json" \
  --argjson phase_family_anchor_family_coverage "$phase_family_anchor_family_coverage_json" \
  '{
    product: $product,
    runtime: $runtime,
    status: $status,
    gate: $gate,
    preflight_terminal_coverage_inventory_schema_version: $schema_version,
    preflight_terminal_coverage_inventory_ready: $ready,
    inventory_mode: $mode,
    inventory_decision: $decision,
    preflight_path: $preflight_path,
    inline_fixture_mode: $inline_fixture_mode,
    preflight_exists: $preflight_exists,
    preflight_syntax_ok: $syntax_ok,
    preflight_marker_count: $marker_count,
    minimum_required_preflight_marker_count: $min_marker_count,
    marker_count_budget_ok: $marker_count_budget_ok,
    phase_family_count: $phase_family_count,
    phase_family_ready_count: $phase_family_ready_count,
    phase_family_budget_failure_count: $phase_family_budget_failure_count,
    phase_family_budget_ready: $phase_family_budget_ready,
    phase_family_anchor_count: $phase_family_anchor_count,
    phase_family_anchor_ready_count: $phase_family_anchor_ready_count,
    phase_family_anchor_failure_count: $phase_family_anchor_failure_count,
    phase_family_anchor_ready: $phase_family_anchor_ready,
    phase_family_anchor_family_count: $phase_family_anchor_family_count,
    phase_family_anchor_family_ready_count: $phase_family_anchor_family_ready_count,
    phase_family_anchor_family_failure_count: $phase_family_anchor_family_failure_count,
    phase_family_anchor_family_ready: $phase_family_anchor_family_ready,
    required_marker_count: $required_marker_count,
    present_required_marker_count: $present_required_marker_count,
    missing_required_marker_count: ($missing_markers | length),
    duplicate_required_marker_count: ($duplicate_markers | length),
    out_of_order_required_marker_count: ($out_of_order_markers | length),
    required_markers_ordered: $ordered_markers,
    terminal_pass_marker_present: $terminal_pass_marker_present,
    native_release_skip_branches_present: $native_release_skip_branches_present,
    final_workspace_diff_check_present: $final_workspace_diff_check_present,
    final_cached_diff_check_present: $final_cached_diff_check_present,
    final_git_status_present: $final_git_status_present,
    final_status_checks_present: $final_status_checks_present,
    final_status_checks_ordered: $final_status_checks_ordered,
    final_status_checks_ready: $final_status_checks_ready,
    whitespace_status_marker_line: $whitespace_status_marker_line,
    terminal_pass_marker_line: $terminal_pass_marker_line,
    workspace_diff_check_line: $workspace_diff_check_line,
    cached_diff_check_line: $cached_diff_check_line,
    git_status_line: $git_status_line,
    required_markers: $required_markers,
    required_marker_lines: $required_marker_lines,
    missing_required_markers: $missing_markers,
    duplicate_required_markers: $duplicate_markers,
    out_of_order_required_markers: $out_of_order_markers,
    phase_family_coverage: $phase_family_records,
    phase_family_budget_failures: $phase_family_failures,
    phase_family_anchors: $phase_family_anchor_records,
    phase_family_anchor_failures: $phase_family_anchor_failures,
    phase_family_anchor_family_coverage: $phase_family_anchor_family_coverage,
    preflight_markers: $preflight_markers,
    inventory_hash_sha256: $inventory_hash_sha256,
    policy_hash_sha256: $policy_hash_sha256,
    side_effect_hash_sha256: $side_effect_hash_sha256,
    inventory_families: [
      {
        id: "preflight-entrypoint",
        ready: ($preflight_exists and $syntax_ok),
        blocked: true,
        reason: "canonical preflight script must exist and parse before terminal coverage can be trusted"
      },
      {
        id: "preflight-marker-count-budget",
        ready: $marker_count_budget_ok,
        blocked: true,
        current_count: $marker_count,
        minimum_count: $min_marker_count,
        reason: "large terminal coverage inventory cannot shrink silently"
      },
      {
        id: "phase-family-coverage-budget",
        ready: $phase_family_budget_ready,
        blocked: true,
        ready_count: $phase_family_ready_count,
        required_count: $phase_family_count,
        failure_count: $phase_family_budget_failure_count,
        failures: $phase_family_failures,
        reason: "large preflight phase families must not collapse to one representative marker"
      },
      {
        id: "phase-family-anchor-contract",
        ready: $phase_family_anchor_ready,
        blocked: true,
        ready_count: $phase_family_anchor_ready_count,
        required_count: $phase_family_anchor_count,
        failure_count: $phase_family_anchor_failure_count,
        family_ready_count: $phase_family_anchor_family_ready_count,
        family_required_count: $phase_family_anchor_family_count,
        family_failure_count: $phase_family_anchor_family_failure_count,
        failures: $phase_family_anchor_failures,
        family_coverage: $phase_family_anchor_family_coverage,
        reason: "each large preflight phase family must retain named anchor markers, not only count filler"
      },
      {
        id: "required-terminal-marker-order",
        ready: (($missing_markers | length) == 0 and ($duplicate_markers | length) == 0 and $ordered_markers),
        blocked: true,
        present_count: $present_required_marker_count,
        required_count: $required_marker_count,
        reason: "critical preflight phases must remain present exactly once and in order"
      },
      {
        id: "native-release-skip-branches",
        ready: $native_release_skip_branches_present,
        blocked: true,
        reason: "verification-only preflight must keep explicit native and release skip controls"
      },
      {
        id: "terminal-pass-marker",
        ready: $terminal_pass_marker_present,
        blocked: true,
        reason: "preflight completion must retain the terminal pass marker consumed by operators"
      },
      {
        id: "final-whitespace-status-checks",
        ready: $final_status_checks_ready,
        blocked: true,
        workspace_diff_check_present: $final_workspace_diff_check_present,
        cached_diff_check_present: $final_cached_diff_check_present,
        git_status_present: $final_git_status_present,
        status_checks_ordered: $final_status_checks_ordered,
        reason: "preflight must end with workspace diff, cached diff, and status checks"
      }
    ],
    denied_by_preflight_terminal_coverage_inventory: [
      "preflight_terminal_coverage_child_gate_execution_denied",
      "preflight_terminal_coverage_release_build_denied",
      "preflight_terminal_coverage_native_gate_execution_denied",
      "preflight_terminal_coverage_workspace_write_denied",
      "preflight_terminal_coverage_service_restart_denied",
      "preflight_terminal_coverage_external_send_denied",
      "preflight_terminal_coverage_secret_read_denied"
    ],
    side_effects: {
      child_gate_execution_performed: false,
      release_build_executed: false,
      native_app_gate_executed: false,
      workspace_written: false,
      filesystem_written: false,
      runtime_mutation_performed: false,
      active_binary_mutated: false,
      service_restarted: false,
      launchd_mutated: false,
      upstream_fetch_performed: false,
      upstream_merge_performed: false,
      provider_invoked: false,
      model_invoked: false,
      external_send_performed: false,
      credential_read: false,
      secret_file_read: false
    }
  }'

if [[ "$coverage_ready" != true ]]; then
  echo "Hepta preflight terminal coverage inventory gate needs attention" >&2
  exit 1
fi

echo "Hepta preflight terminal coverage inventory gate passed"
