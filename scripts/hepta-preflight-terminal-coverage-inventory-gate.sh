#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT"

PREFLIGHT_PATH="scripts/hepta-preflight.sh"
MIN_MARKER_COUNT="${HEPTA_PREFLIGHT_TERMINAL_COVERAGE_MIN_MARKER_COUNT:-160}"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

json_lines() {
  jq -R -s 'split("\n") | map(select(length > 0))'
}

required_markers=(
  "metadata"
  "cargo check"
  "KG prompt-preview preflight gate"
  "KG prompt-preview terminal next-action activation denial summary gate"
  "readiness denial review acceptance closure summary gate"
  "upstream Codex promotion closure gate"
  "terminal release-governance final audit index gate"
  "operator-security attention-budget diagnostic gate"
  "terminal watchdog/soak regression gate"
  "core activation evidence receipt terminal closure decision gate"
  "JSON report capture diagnostic contract gate"
  "JSON report capture migration inventory gate"
  "preflight terminal coverage inventory gate"
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

preflight_exists=false
if [[ -x "$PREFLIGHT_PATH" ]]; then
  preflight_exists=true
fi

syntax_ok=false
if bash -n "$PREFLIGHT_PATH"; then
  syntax_ok=true
fi

mapfile -t preflight_markers < <(
  grep -E '^[[:space:]]*echo "\[hepta-preflight\] ' "$PREFLIGHT_PATH" \
    | sed -E 's/^[[:space:]]*echo "\[hepta-preflight\] (.*)"$/\1/'
)

marker_count="${#preflight_markers[@]}"
marker_count_budget_ok=false
if [[ "$marker_count" -ge "$MIN_MARKER_COUNT" ]]; then
  marker_count_budget_ok=true
fi

terminal_pass_marker_present=false
if grep -q '^echo "Hepta preflight passed"$' "$PREFLIGHT_PATH"; then
  terminal_pass_marker_present=true
fi

native_release_skip_branches_present=false
if grep -q 'native app gates skipped' "$PREFLIGHT_PATH" \
  && grep -q 'release build skipped' "$PREFLIGHT_PATH" \
  && grep -q 'HEPTA_PREFLIGHT_NATIVE' "$PREFLIGHT_PATH" \
  && grep -q 'HEPTA_PREFLIGHT_RELEASE' "$PREFLIGHT_PATH"; then
  native_release_skip_branches_present=true
fi

missing_markers=()
duplicate_markers=()
out_of_order_markers=()
required_marker_lines=()
previous_line=0
ordered_markers=true
present_required_marker_count=0

for marker in "${required_markers[@]}"; do
  mapfile -t lines < <(
    grep -nF "echo \"[hepta-preflight] $marker\"" "$PREFLIGHT_PATH" \
      | cut -d: -f1
  )
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
  && "$terminal_pass_marker_present" == true \
  && "$native_release_skip_branches_present" == true \
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

inventory_hash_sha256="$(
  sha256_text "$marker_count:$present_required_marker_count:$MIN_MARKER_COUNT:$terminal_pass_marker_present:$native_release_skip_branches_present:${required_marker_lines[*]}"
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
  --argjson preflight_exists "$preflight_exists" \
  --argjson syntax_ok "$syntax_ok" \
  --argjson marker_count "$marker_count" \
  --argjson min_marker_count "$MIN_MARKER_COUNT" \
  --argjson marker_count_budget_ok "$marker_count_budget_ok" \
  --argjson required_marker_count "${#required_markers[@]}" \
  --argjson present_required_marker_count "$present_required_marker_count" \
  --argjson ordered_markers "$ordered_markers" \
  --argjson terminal_pass_marker_present "$terminal_pass_marker_present" \
  --argjson native_release_skip_branches_present "$native_release_skip_branches_present" \
  --argjson required_markers "$required_markers_json" \
  --argjson preflight_markers "$preflight_markers_json" \
  --argjson missing_markers "$missing_markers_json" \
  --argjson duplicate_markers "$duplicate_markers_json" \
  --argjson out_of_order_markers "$out_of_order_markers_json" \
  --argjson required_marker_lines "$required_marker_lines_json" \
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
    preflight_exists: $preflight_exists,
    preflight_syntax_ok: $syntax_ok,
    preflight_marker_count: $marker_count,
    minimum_required_preflight_marker_count: $min_marker_count,
    marker_count_budget_ok: $marker_count_budget_ok,
    required_marker_count: $required_marker_count,
    present_required_marker_count: $present_required_marker_count,
    missing_required_marker_count: ($missing_markers | length),
    duplicate_required_marker_count: ($duplicate_markers | length),
    out_of_order_required_marker_count: ($out_of_order_markers | length),
    required_markers_ordered: $ordered_markers,
    terminal_pass_marker_present: $terminal_pass_marker_present,
    native_release_skip_branches_present: $native_release_skip_branches_present,
    required_markers: $required_markers,
    required_marker_lines: $required_marker_lines,
    missing_required_markers: $missing_markers,
    duplicate_required_markers: $duplicate_markers,
    out_of_order_required_markers: $out_of_order_markers,
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
