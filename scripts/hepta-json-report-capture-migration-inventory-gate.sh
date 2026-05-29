#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

HELPER_PATH="scripts/lib/hepta-json-report-capture.sh"
CONTRACT_GATE_PATH="scripts/hepta-json-report-capture-diagnostic-contract-gate.sh"

MAX_LOCAL_CAPTURE_DEFINITION_COUNT="${HEPTA_JSON_CAPTURE_MAX_LOCAL_CAPTURE_DEFINITION_COUNT:-15}"
MAX_LOCAL_EXTRACT_DEFINITION_COUNT="${HEPTA_JSON_CAPTURE_MAX_LOCAL_EXTRACT_DEFINITION_COUNT:-0}"

list_matching_files() {
  local pattern="$1"
  find scripts -type f -name '*.sh' ! -path "$HELPER_PATH" -print0 \
    | xargs -0 grep -l "$pattern" 2>/dev/null \
    | sort || true
}

count_lines() {
  sed '/^$/d' | wc -l | tr -d ' '
}

json_lines() {
  jq -R -s 'split("\n") | map(select(length > 0))'
}

local_capture_files="$(list_matching_files '^capture_json_report()')"
local_extract_files="$(list_matching_files '^extract_first_json_object()')"
helper_source_files="$(find scripts -type f -name '*.sh' -print0 \
  | xargs -0 grep -l 'hepta-json-report-capture.sh' 2>/dev/null \
  | sort || true)"

local_capture_definition_count="$(printf '%s\n' "$local_capture_files" | count_lines)"
local_extract_definition_count="$(printf '%s\n' "$local_extract_files" | count_lines)"
helper_source_count="$(printf '%s\n' "$helper_source_files" | count_lines)"

helper_exists=false
if [[ -x "$HELPER_PATH" ]]; then
  helper_exists=true
fi

contract_gate_exists=false
if [[ -x "$CONTRACT_GATE_PATH" ]]; then
  contract_gate_exists=true
fi

local_capture_budget_ok=false
if [[ "$local_capture_definition_count" -le "$MAX_LOCAL_CAPTURE_DEFINITION_COUNT" ]]; then
  local_capture_budget_ok=true
fi

local_extract_budget_ok=false
if [[ "$local_extract_definition_count" -le "$MAX_LOCAL_EXTRACT_DEFINITION_COUNT" ]]; then
  local_extract_budget_ok=true
fi

migration_inventory_ready=false
if [[ "$helper_exists" == true \
  && "$contract_gate_exists" == true \
  && "$local_capture_budget_ok" == true \
  && "$local_extract_budget_ok" == true ]]; then
  migration_inventory_ready=true
fi

capture_files_json="$(printf '%s\n' "$local_capture_files" | json_lines)"
extract_files_json="$(printf '%s\n' "$local_extract_files" | json_lines)"
helper_source_files_json="$(printf '%s\n' "$helper_source_files" | json_lines)"

inventory_hash="$(printf '%s\n%s\n%s\n%s\n%s\n%s\n' \
  "$local_capture_definition_count" \
  "$local_extract_definition_count" \
  "$MAX_LOCAL_CAPTURE_DEFINITION_COUNT" \
  "$MAX_LOCAL_EXTRACT_DEFINITION_COUNT" \
  "$local_capture_files" \
  "$local_extract_files" | shasum -a 256 | awk '{print $1}')"
policy_hash="$(printf '%s\n%s\n%s\n%s\n' \
  "$HELPER_PATH" \
  "$CONTRACT_GATE_PATH" \
  "$MAX_LOCAL_CAPTURE_DEFINITION_COUNT" \
  "$MAX_LOCAL_EXTRACT_DEFINITION_COUNT" | shasum -a 256 | awk '{print $1}')"
side_effect_hash="$(printf '%s\n' \
  "filesystem_written=false;service_restarted=false;external_send_performed=false;secret_file_read=false" \
  | shasum -a 256 | awk '{print $1}')"

jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg status "$(if [[ "$migration_inventory_ready" == true ]]; then echo ready; else echo attention; fi)" \
  --arg gate "hepta_json_report_capture_migration_inventory_gate" \
  --arg schema_version "json_report_capture_migration_inventory_v1" \
  --arg mode "static_shell_gate_inventory_no_runtime_mutation" \
  --arg decision "json_report_capture_helper_adoption_budget_recorded_without_mass_migration" \
  --arg helper_path "$HELPER_PATH" \
  --arg contract_gate_path "$CONTRACT_GATE_PATH" \
  --arg inventory_hash "$inventory_hash" \
  --arg policy_hash "$policy_hash" \
  --arg side_effect_hash "$side_effect_hash" \
  --argjson ready "$migration_inventory_ready" \
  --argjson helper_exists "$helper_exists" \
  --argjson contract_gate_exists "$contract_gate_exists" \
  --argjson local_capture_definition_count "$local_capture_definition_count" \
  --argjson local_extract_definition_count "$local_extract_definition_count" \
  --argjson max_local_capture_definition_count "$MAX_LOCAL_CAPTURE_DEFINITION_COUNT" \
  --argjson max_local_extract_definition_count "$MAX_LOCAL_EXTRACT_DEFINITION_COUNT" \
  --argjson local_capture_budget_ok "$local_capture_budget_ok" \
  --argjson local_extract_budget_ok "$local_extract_budget_ok" \
  --argjson helper_source_count "$helper_source_count" \
  --argjson local_capture_files "$capture_files_json" \
  --argjson local_extract_files "$extract_files_json" \
  --argjson helper_source_files "$helper_source_files_json" \
  '{
    product: $product,
    runtime: $runtime,
    status: $status,
    gate: $gate,
    json_report_capture_migration_inventory_schema_version: $schema_version,
    json_report_capture_migration_inventory_ready: $ready,
    inventory_mode: $mode,
    inventory_decision: $decision,
    helper_path: $helper_path,
    helper_exists: $helper_exists,
    contract_gate_path: $contract_gate_path,
    contract_gate_exists: $contract_gate_exists,
    local_capture_definition_count: $local_capture_definition_count,
    local_extract_definition_count: $local_extract_definition_count,
    max_local_capture_definition_count: $max_local_capture_definition_count,
    max_local_extract_definition_count: $max_local_extract_definition_count,
    local_capture_budget_ok: $local_capture_budget_ok,
    local_extract_budget_ok: $local_extract_budget_ok,
    helper_source_count: $helper_source_count,
    remaining_local_capture_definition_files: $local_capture_files,
    remaining_local_extract_definition_files: $local_extract_files,
    helper_source_files: $helper_source_files,
    inventory_hash_sha256: $inventory_hash,
    policy_hash_sha256: $policy_hash,
    side_effect_hash_sha256: $side_effect_hash,
    inventory_families: [
      {
        id: "shared-helper-contract-source",
        ready: ($helper_exists and $contract_gate_exists),
        blocked: true,
        helper_exists: $helper_exists,
        contract_gate_exists: $contract_gate_exists,
        reason: "shared helper and diagnostic contract must exist before further capture migrations"
      },
      {
        id: "local-capture-definition-budget",
        ready: $local_capture_budget_ok,
        blocked: true,
        current_count: $local_capture_definition_count,
        max_count: $max_local_capture_definition_count,
        reason: "remaining local capture_json_report definitions are inventoried and cannot grow silently"
      },
      {
        id: "local-extract-definition-budget",
        ready: $local_extract_budget_ok,
        blocked: true,
        current_count: $local_extract_definition_count,
        max_count: $max_local_extract_definition_count,
        reason: "remaining local extract_first_json_object definitions are inventoried and cannot grow silently"
      },
      {
        id: "static-inventory-side-effect-boundary",
        ready: true,
        blocked: true,
        reason: "inventory is static shell inspection only and performs no runtime mutation"
      }
    ],
    denied_by_json_report_capture_inventory: [
      "json_report_capture_inventory_workspace_write_denied",
      "json_report_capture_inventory_runtime_mutation_denied",
      "json_report_capture_inventory_service_restart_denied",
      "json_report_capture_inventory_external_send_denied",
      "json_report_capture_inventory_secret_read_denied"
    ],
    side_effects: {
      filesystem_written: false,
      evidence_persisted: false,
      runtime_mutation_performed: false,
      service_restarted: false,
      launchd_mutated: false,
      gateway_mutation_performed: false,
      credential_read: false,
      secret_file_read: false,
      external_send_performed: false
    }
  }'

if [[ "$migration_inventory_ready" != true ]]; then
  echo "Hepta JSON report capture migration inventory gate needs attention" >&2
  exit 1
fi

echo "Hepta JSON report capture migration inventory gate passed"
