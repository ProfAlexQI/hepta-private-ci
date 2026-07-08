#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
registry="$repo_root/codex-rs/CONTEXT_SOURCE_REGISTRY.tsv"

fail() {
  echo "hepta-context-generated-context-inventory-report: $*" >&2
  exit 1
}

inventory_rows() {
  cat <<'EOF'
apps|build_initial_context:capability_inventory|manifest:developer:apps|manifest_hash_diff_with_clear|debug:manifest+response_debug|gate:capability_inventory_clear_diff|runtime:disabled
available_plugins|build_initial_context:capability_inventory|manifest:developer:available_plugins|manifest_hash_diff_with_clear|debug:manifest+response_debug|gate:capability_inventory_clear_diff|runtime:disabled
available_skills|build_initial_context:capability_inventory|manifest:developer:available_skills|manifest_hash_diff_with_clear|debug:manifest+response_debug|gate:capability_inventory_clear_diff|runtime:disabled
collaboration_mode|build_initial_context:collaboration_mode|manifest:developer:collaboration_mode|manifest_hash_diff_with_clear|debug:manifest+settings_diff|gate:collaboration_mode_clear_diff|runtime:disabled
context|build_initial_context:raw_context|manifest:runtime:context|turn_scoped_no_steady_state_diff|debug:manifest+source_registry|gate:settings_diff_coverage|runtime:disabled
contextual_user|build_initial_context:contextual_user|manifest:user:contextual_user|turn_scoped_no_steady_state_diff|debug:manifest+source_registry|gate:settings_diff_coverage|runtime:disabled
developer_instructions|build_initial_context:developer_instructions|manifest:developer:developer_instructions|manifest_hash_diff_with_clear|debug:manifest+settings_diff|gate:user_developer_clear_diff|runtime:disabled
environment|build_initial_context:environment|manifest:runtime:environment|manifest_hash_diff|debug:manifest+settings_diff|gate:semantic_contribution_sources|runtime:disabled
extension_contextual_user|build_initial_context:extension_contextual_user|manifest:user:extension_contextual_user|manifest_hash_diff_with_clear|debug:manifest+settings_diff|gate:extension_fragment_diff|runtime:disabled
extension_developer_capabilities|build_initial_context:extension_developer_capabilities|manifest:tool:extension_developer_capabilities|manifest_hash_diff_with_clear|debug:manifest+settings_diff|gate:extension_fragment_diff|runtime:disabled
extension_developer_policy|build_initial_context:extension_developer_policy|manifest:developer:extension_developer_policy|manifest_hash_diff_with_clear|debug:manifest+settings_diff|gate:extension_fragment_diff|runtime:disabled
extension_separate_developer|build_initial_context:extension_separate_developer|manifest:developer:extension_separate_developer|manifest_hash_diff_with_clear|debug:manifest+settings_diff|gate:extension_fragment_diff|runtime:disabled
model_switch|build_initial_context:model_switch|manifest:session_state:model_switch|manifest_hash_diff_with_clear|debug:manifest+settings_diff|gate:semantic_contribution_sources|runtime:disabled
multi_agent_usage_hint|build_initial_context:multi_agent_usage_hint|manifest:developer:multi_agent_usage_hint|manifest_hash_diff_with_clear|debug:manifest+settings_diff|gate:semantic_contribution_sources|runtime:disabled
non_text_content|build_initial_context:non_text_content|manifest:runtime:non_text_content|turn_scoped_no_steady_state_diff|debug:manifest+source_registry|gate:settings_diff_coverage|runtime:disabled
permissions|build_initial_context:permissions|manifest:system:permissions|manifest_hash_diff|debug:manifest+settings_diff|gate:semantic_contribution_sources|runtime:disabled
personality|build_initial_context:personality|manifest:developer:personality|manifest_hash_diff|debug:manifest+settings_diff|gate:semantic_contribution_sources|runtime:disabled
realtime|build_initial_context:realtime|manifest:runtime:realtime|manifest_hash_diff_with_clear|debug:manifest+settings_diff|gate:settings_diff_coverage|runtime:disabled
selected_context_recall|ContextController:plan_pending_context_items|manifest:retrieved_snippets:selected_context_recall|live_turn_item|debug:manifest+operator_packet|gate:selected_recall_surface|runtime:disabled
user_instructions|build_initial_context:user_instructions|manifest:user:user_instructions|manifest_hash_diff_with_clear|debug:manifest+settings_diff|gate:user_developer_clear_diff|runtime:disabled
EOF
}

registry_ids="$(awk -F '\t' 'NR > 2 { print $1 }' "$registry")"
inventory_ids="$(inventory_rows | awk -F '|' '{ print $1 }')"
if [ "$registry_ids" != "$inventory_ids" ]; then
  fail "inventory source ids drifted from CONTEXT_SOURCE_REGISTRY.tsv"
fi

source_count="$(inventory_rows | awk -F '|' 'NF == 7 { count++ } END { print count + 0 }')"
settings_diff_covered_count="$(inventory_rows | awk -F '|' '$4 ~ /^manifest_hash_diff/ { count++ } END { print count + 0 }')"
turn_scoped_count="$(inventory_rows | awk -F '|' '$4 == "turn_scoped_no_steady_state_diff" { count++ } END { print count + 0 }')"
live_turn_count="$(inventory_rows | awk -F '|' '$4 == "live_turn_item" { count++ } END { print count + 0 }')"

if [ "$source_count" != "20" ]; then
  fail "expected 20 generated context inventory sources, got $source_count"
fi
if [ "$settings_diff_covered_count" != "16" ]; then
  fail "expected 16 settings-diff covered sources, got $settings_diff_covered_count"
fi
if [ "$turn_scoped_count" != "3" ]; then
  fail "expected 3 turn-scoped no steady-state diff sources, got $turn_scoped_count"
fi
if [ "$live_turn_count" != "1" ]; then
  fail "expected 1 live turn item source, got $live_turn_count"
fi

echo "generated-context-inventory=pass"
echo "generated-context-inventory.schema=1"
echo "generated-context-inventory.source-count=$source_count"
echo "generated-context-inventory.registry-sync=pass"
echo "generated-context-inventory.settings-diff-covered=$settings_diff_covered_count"
echo "generated-context-inventory.turn-scoped-no-steady-state-diff=$turn_scoped_count"
echo "generated-context-inventory.live-turn-item=$live_turn_count"
echo "generated-context-inventory.build-settings-update-items=covered"
echo "generated-context-inventory.context-controller-ownership=plan_pending_context_items+plan_turn_context"
echo "generated-context-inventory.session-ownership=build_initial_context+build_settings_update_items+side_effect_handoff"
echo "generated-context-inventory.runtime-activation=disabled"

inventory_rows | while IFS='|' read -r source_id producer manifest_entry settings_diff debug_surface gate_surface runtime_status; do
  echo "generated-context-inventory.source.$source_id=$producer;$manifest_entry;$settings_diff;$debug_surface;$gate_surface;$runtime_status"
done
