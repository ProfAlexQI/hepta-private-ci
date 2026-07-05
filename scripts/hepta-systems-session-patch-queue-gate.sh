#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-session-patch-queue-report.sh"
RECONSTRUCTION_GATE="$ROOT/scripts/hepta-systems-session-reconstruction-map-gate.sh"

[[ -x "$REPORT" ]] || {
  echo "missing executable session patch queue report: $REPORT" >&2
  exit 1
}
[[ -x "$RECONSTRUCTION_GATE" ]] || {
  echo "missing executable session reconstruction map gate: $RECONSTRUCTION_GATE" >&2
  exit 1
}

reconstruction_tmp="$(mktemp)"
trap 'rm -f "$reconstruction_tmp"' EXIT
"$RECONSTRUCTION_GATE" >"$reconstruction_tmp"
grep -q "Hepta session reconstruction map gate passed" "$reconstruction_tmp" \
  || {
    echo "session reconstruction map gate did not pass" >&2
    exit 1
  }

json="$("$REPORT")"

jq -e '
  .runtime == "hepta"
  and .surface == "hepta_systems_session_patch_queue"
  and .status == "ready"
  and .side_effect_free == true
  and .report_only == true
  and .replay_applied == false
  and .patch_replay_enabled == false
  and .patch_body_emitted == false
  and .ordered_patch_call_count >= 1
  and .patch_file_entry_count >= 1
  and .touched_path_count >= 1
  and .missing_path_count >= 1
  and .phase0_anchor_patch_call_count >= 1
  and .phase0_anchor_id_count == 5
  and (.phase0_anchor_ids | index("plugin_contribution_point_abi")) != null
  and (.phase0_anchor_ids | index("tool_registry_router_lookup_shadow")) != null
  and (.phase0_anchor_ids | index("workflow_durable_store_replay_proof")) != null
  and (.phase0_anchor_ids | index("compact_capability_matrix")) != null
  and (.phase0_anchor_ids | index("scheduler_cutover_preview_chain")) != null
  and all(.phase0_anchor_summary[]; .patch_call_count >= 1)
  and .recommended_next_local_step == "extract_selected_anchor_patch_bodies_for_manual_apply_check"
' <<<"$json" >/dev/null

printf '%s\n' "$json"
echo "Hepta session patch queue gate passed"
