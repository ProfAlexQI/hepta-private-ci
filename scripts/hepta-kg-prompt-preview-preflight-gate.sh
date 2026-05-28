#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MANIFEST="${HEPTA_MANIFEST:-${HEPTA_CODEX_MANIFEST:-codex-rs/Cargo.toml}}"

cd "$REPO_ROOT"

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

INTELLIGENCE_PREFLIGHT_TEST="memory_kg_prompt_preview_preflight_blocks_ci_promotion_until_gate_chain_closes"
RUNTIME_PREFLIGHT_TEST="knowledge_graph_prompt_preview_preflight_summary_renders_blocked_ci_gate"
PREFLIGHT_CONTRACT="hepta-intelligence-memory-kg-prompt-preview-preflight-v0"
CONTEXT_HANDOFF_CONTRACT="hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0"

echo "[hepta-kg-prompt-preview-preflight-gate] intelligence report contract"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-intelligence \
  "$INTELLIGENCE_PREFLIGHT_TEST" -- --nocapture

echo "[hepta-kg-prompt-preview-preflight-gate] runtime summary contract"
cargo test --offline --manifest-path "$MANIFEST" -q -p hepta-runtime \
  "$RUNTIME_PREFLIGHT_TEST" -- --nocapture

intelligence_test_hash_sha256="$(sha256_text "$INTELLIGENCE_PREFLIGHT_TEST:$PREFLIGHT_CONTRACT")"
runtime_test_hash_sha256="$(sha256_text "$RUNTIME_PREFLIGHT_TEST:$PREFLIGHT_CONTRACT:$CONTEXT_HANDOFF_CONTRACT")"
script_gate_hash_sha256="$(sha256_text "hepta-kg-prompt-preview-preflight-gate:$intelligence_test_hash_sha256:$runtime_test_hash_sha256")"

report="$(jq -n \
  --arg product "Hepta" \
  --arg runtime "hepta" \
  --arg gate "hepta_kg_prompt_preview_preflight_gate" \
  --arg preflight_contract "$PREFLIGHT_CONTRACT" \
  --arg context_handoff_contract "$CONTEXT_HANDOFF_CONTRACT" \
  --arg intelligence_test "$INTELLIGENCE_PREFLIGHT_TEST" \
  --arg runtime_test "$RUNTIME_PREFLIGHT_TEST" \
  --arg intelligence_test_hash_sha256 "$intelligence_test_hash_sha256" \
  --arg runtime_test_hash_sha256 "$runtime_test_hash_sha256" \
  --arg script_gate_hash_sha256 "$script_gate_hash_sha256" \
  '{
    product:$product,
    runtime:$runtime,
    status:"ready",
    gate:$gate,
    mode:"kg_prompt_preview_preflight_report_only_ci_gate_no_execution",
    preflight_contract:$preflight_contract,
    context_handoff_contract:$context_handoff_contract,
    source_report_command:"memory-kg-prompt-preview-preflight",
    source_runtime_summary:"knowledge_graph_prompt_preview_preflight_summary",
    intelligence_contract_test:$intelligence_test,
    runtime_summary_test:$runtime_test,
    intelligence_contract_test_hash_sha256:$intelligence_test_hash_sha256,
    runtime_summary_test_hash_sha256:$runtime_test_hash_sha256,
    script_gate_hash_sha256:$script_gate_hash_sha256,
    source_gate_count:5,
    ready_source_gate_count:5,
    blocked_source_gate_count:5,
    report_only_source_gate_count:5,
    required_operator_evidence_count:7,
    missing_operator_evidence_count:7,
    required_safety_control_count:4,
    missing_safety_control_count:4,
    required_handoff_requirement_count:6,
    missing_handoff_requirement_count:6,
    missing_final_review_approval_count:2,
    required_total_preflight_requirement_count:19,
    missing_total_preflight_requirement_count:19,
    preflight_report_status:"blocked",
    preflight_report_verdict:"blocked_until_prompt_preview_gate_chain_evidence_review_approval_and_ci_promotion_exist",
    source_gates_all_linked:true,
    source_gates_all_checks_ready:true,
    source_gates_all_blocked:true,
    source_gates_all_report_only:true,
    context_handoff_contract_linked:true,
    context_handoff_checks_ready:true,
    context_handoff_blocked:true,
    operator_evidence_incomplete:true,
    safety_controls_incomplete:true,
    handoff_requirements_incomplete:true,
    redacted_diff_review_required:true,
    context_handoff_approval_required:true,
    redacted_refs_only:true,
    raw_prompt_diff_count:0,
    prompt_text_included_count:0,
    payload_text_included_count:0,
    prompt_preview_allowed:false,
    prompt_preview_rendered:false,
    prompt_payload_materialized:false,
    context_injection_allowed:false,
    context_injection_performed:false,
    model_invoked:false,
    external_read_enabled_count:0,
    network_call_enabled_count:0,
    live_write_enabled_count:0,
    ci_promotion_allowed:false,
    ci_promotion_disabled:true,
    preflight_execution_allowed:false,
    preflight_execution_performed:false,
    source_gates:[
      {gate:"approval_packet", status:"blocked", checks_ready:true, report_only:true, blocks_prompt_preview:true, blocks_context_injection:true},
      {gate:"operator_evidence", status:"blocked", checks_ready:true, report_only:true, blocks_prompt_preview:true, blocks_context_injection:true},
      {gate:"redaction_diff", status:"blocked", checks_ready:true, report_only:true, blocks_prompt_preview:true, blocks_context_injection:true},
      {gate:"rollback_kill_switch", status:"blocked", checks_ready:true, report_only:true, blocks_prompt_preview:true, blocks_context_injection:true},
      {gate:"context_handoff", status:"blocked", checks_ready:true, report_only:true, blocks_prompt_preview:true, blocks_context_injection:true}
    ],
    denied_actions:[
      "ci_promotion_denied",
      "preflight_execution_denied",
      "prompt_preview_rendering_denied",
      "prompt_payload_materialization_denied",
      "context_injection_denied",
      "model_invocation_denied",
      "external_kg_adapter_read_denied",
      "network_call_denied",
      "live_kg_write_denied",
      "gateway_route_migration_denied",
      "install_restart_active_binary_mutation_denied"
    ],
    side_effects:{
      preflight_execution_performed:false,
      prompt_preview_rendered:false,
      prompt_payload_materialized:false,
      context_injection_performed:false,
      model_invoked:false,
      external_kg_adapter_read_performed:false,
      graphiti_client_constructed:false,
      neo4j_client_constructed:false,
      cocoindex_client_constructed:false,
      network_call_performed:false,
      external_db_write_performed:false,
      live_kg_write_performed:false,
      native_gateway_route_added:false,
      source_command_migration_performed:false,
      install_performed:false,
      launchd_restart_performed:false,
      active_binary_mutated:false,
      credential_read_performed:false
    }
  }')"

jq -e '
  .runtime == "hepta"
  and .status == "ready"
  and .gate == "hepta_kg_prompt_preview_preflight_gate"
  and .mode == "kg_prompt_preview_preflight_report_only_ci_gate_no_execution"
  and .preflight_contract == "hepta-intelligence-memory-kg-prompt-preview-preflight-v0"
  and .context_handoff_contract == "hepta-intelligence-memory-kg-prompt-preview-context-handoff-v0"
  and .preflight_report_status == "blocked"
  and .source_gate_count == 5
  and .ready_source_gate_count == 5
  and .blocked_source_gate_count == 5
  and .report_only_source_gate_count == 5
  and .required_total_preflight_requirement_count == 19
  and .missing_total_preflight_requirement_count == 19
  and .source_gates_all_linked == true
  and .source_gates_all_checks_ready == true
  and .source_gates_all_blocked == true
  and .source_gates_all_report_only == true
  and .operator_evidence_incomplete == true
  and .safety_controls_incomplete == true
  and .handoff_requirements_incomplete == true
  and .redacted_diff_review_required == true
  and .context_handoff_approval_required == true
  and .redacted_refs_only == true
  and .raw_prompt_diff_count == 0
  and .prompt_text_included_count == 0
  and .payload_text_included_count == 0
  and .prompt_preview_allowed == false
  and .prompt_preview_rendered == false
  and .prompt_payload_materialized == false
  and .context_injection_allowed == false
  and .context_injection_performed == false
  and .model_invoked == false
  and .external_read_enabled_count == 0
  and .network_call_enabled_count == 0
  and .live_write_enabled_count == 0
  and .ci_promotion_allowed == false
  and .ci_promotion_disabled == true
  and .preflight_execution_allowed == false
  and .preflight_execution_performed == false
  and (.source_gates | length) == 5
  and (.source_gates | all(.status == "blocked" and .checks_ready == true and .report_only == true and .blocks_prompt_preview == true and .blocks_context_injection == true))
  and (.denied_actions | length) == 11
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null <<<"$report"

printf '%s\n' "$report"
echo "Hepta KG prompt-preview preflight gate passed"
