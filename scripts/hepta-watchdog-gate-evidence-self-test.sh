#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
source "$ROOT/scripts/lib/hepta-watchdog-gate-evidence-v1.sh"
source "$ROOT/scripts/lib/hepta-watchdog-product-boundary-v1.sh"

base_report='{
  "status":"ok",
  "operator_security_status":"ready",
  "active_health":{"required":true,"status":"ready"},
  "health":"ready"
}'

active_report="$(
  jq -c '. + {
    watchdog_mode:"active-health",
    candidate_artifact:{required:false,evidence:{status:"not_checked",ready:null,failure_reasons:[]}},
    deployed_receipt:{required:false,evidence:{status:"not_checked",ready:null,failure_reasons:[]}},
    deployment_consistency_required:false,
    binary_sha_match:false,
    release_sha256:"",
    installed_sha256:""
  }' <<<"$base_report"
)"
active_contract="$(hepta_watchdog_gate_evidence_contract_json "$active_report" active-health)"
jq -e '
  .ready == true
  and .active_health_only == true
  and .deployment_consistency_checked == false
  and .binary_sha_match_checked == false
  and .binary_sha_match == false
' >/dev/null <<<"$active_contract"

sha='aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'
deployment_report="$(
  jq -c --arg sha "$sha" '. + {
    watchdog_mode:"deployment-consistency",
    candidate_artifact:{required:true,evidence:{status:"ready",ready:true,failure_reasons:[]}},
    deployed_receipt:{required:true,evidence:{status:"ready",ready:true,failure_reasons:[]}},
    deployment_consistency_required:true,
    binary_sha_match:true,
    release_sha256:$sha,
    installed_sha256:$sha
  }' <<<"$base_report"
)"
deployment_contract="$(
  hepta_watchdog_gate_evidence_contract_json "$deployment_report" deployment-consistency
)"
jq -e '
  .ready == true
  and .active_health_only == false
  and .deployment_consistency_checked == true
  and .binary_sha_match_checked == true
  and .binary_sha_match == true
' >/dev/null <<<"$deployment_contract"

mismatch_report="$(jq -c '.installed_sha256 = ("b" * 64)' <<<"$deployment_report")"
jq -e '.ready == false and .artifact_evidence_ready == false' >/dev/null \
  <<<"$(hepta_watchdog_gate_evidence_contract_json "$mismatch_report" deployment-consistency)"

assert_not_ready() {
  local label="$1"
  local mode="$2"
  local fixture="$3"
  local contract
  contract="$(hepta_watchdog_gate_evidence_contract_json "$fixture" "$mode")"
  jq -e '.ready == false' >/dev/null <<<"$contract" || {
    echo "watchdog evidence negative fixture unexpectedly ready: $label" >&2
    exit 1
  }
}

# Every member of the active-health not-checked/ready=null triple is binding.
while IFS=$'\t' read -r label filter; do
  [[ -n "$label" ]] || continue
  assert_not_ready \
    "$label" active-health "$(jq -c "$filter" <<<"$active_report")"
done <<'EOF'
active-candidate-required	.candidate_artifact.required = true
active-candidate-status	.candidate_artifact.evidence.status = "failed"
active-candidate-ready	.candidate_artifact.evidence.ready = true
active-candidate-failure	.candidate_artifact.evidence.failure_reasons = ["contradiction"]
active-candidate-failure-missing	del(.candidate_artifact.evidence.failure_reasons)
active-candidate-failure-type	.candidate_artifact.evidence.failure_reasons = "none"
active-deployed-required	.deployed_receipt.required = true
active-deployed-status	.deployed_receipt.evidence.status = "failed"
active-deployed-ready	.deployed_receipt.evidence.ready = false
active-deployed-failure	.deployed_receipt.evidence.failure_reasons = ["contradiction"]
active-deployed-failure-missing	del(.deployed_receipt.evidence.failure_reasons)
active-deployed-failure-type	.deployed_receipt.evidence.failure_reasons = "none"
active-deployment-required	.deployment_consistency_required = true
active-binary-match	.binary_sha_match = true
active-release-sha	.release_sha256 = ("a" * 64)
active-installed-sha	.installed_sha256 = ("a" * 64)
EOF

# Every member of the deployment ready/status/failure-free triple is binding.
while IFS=$'\t' read -r label filter; do
  [[ -n "$label" ]] || continue
  assert_not_ready \
    "$label" deployment-consistency "$(jq -c "$filter" <<<"$deployment_report")"
done <<'EOF'
deployment-candidate-required	.candidate_artifact.required = false
deployment-candidate-status	.candidate_artifact.evidence.status = "failed"
deployment-candidate-ready	.candidate_artifact.evidence.ready = false
deployment-candidate-failure	.candidate_artifact.evidence.failure_reasons = ["contradiction"]
deployment-candidate-failure-missing	del(.candidate_artifact.evidence.failure_reasons)
deployment-candidate-failure-type	.candidate_artifact.evidence.failure_reasons = "none"
deployment-deployed-required	.deployed_receipt.required = false
deployment-deployed-status	.deployed_receipt.evidence.status = "failed"
deployment-deployed-ready	.deployed_receipt.evidence.ready = null
deployment-deployed-failure	.deployed_receipt.evidence.failure_reasons = ["contradiction"]
deployment-deployed-failure-missing	del(.deployed_receipt.evidence.failure_reasons)
deployment-deployed-failure-type	.deployed_receipt.evidence.failure_reasons = "none"
deployment-required	.deployment_consistency_required = false
deployment-binary-match	.binary_sha_match = false
deployment-release-sha	.release_sha256 = ""
deployment-installed-sha	.installed_sha256 = ""
EOF

unknown_attention_report="$(
  jq -c '. + {
    status:"ok",
    operator_security_status:"attention",
    operator_security_attention_state_known:false
  }' <<<"$active_report"
)"
assert_not_ready "unknown-operator-attention" active-health "$unknown_attention_report"

known_attention_report="$(
  jq -c '. + {
    status:"failed",
    operator_security_status:"attention",
    operator_security_attention_state_known:true,
    operator_security_attention_budget_known:false,
    telegram_production_readiness_state_known:false,
    telegram_production_readiness_classification:"unknown"
  }' <<<"$active_report"
)"
assert_not_ready "failed-known-operator-attention" active-health "$known_attention_report"

ok_known_attention_report="$(jq -c '.status = "ok"' <<<"$known_attention_report")"
jq -e '.ready == true and .status_known == true' >/dev/null \
  <<<"$(hepta_watchdog_gate_evidence_contract_json "$ok_known_attention_report" active-health)"

known_budget_attention_report="$(
  jq -c '. + {
    status:"failed",
    operator_security_status:"attention",
    operator_security_attention_state_known:true,
    operator_security_attention_budget_known:true,
    telegram_production_readiness_state_known:true,
    telegram_production_readiness_classification:"attention_budget_exceeded",
    telegram_production_attention_budget_ok:false
  }' <<<"$active_report"
)"
assert_not_ready "failed-known-budget-attention" active-health "$known_budget_attention_report"

ok_known_budget_attention_report="$(jq -c '.status = "ok"' <<<"$known_budget_attention_report")"
jq -e '.ready == true and .status_known == true' >/dev/null \
  <<<"$(hepta_watchdog_gate_evidence_contract_json "$ok_known_budget_attention_report" active-health)"
assert_not_ready \
  "budget-attention-unknown-readiness" active-health \
  "$(jq -c '.telegram_production_readiness_state_known = false' <<<"$ok_known_budget_attention_report")"
assert_not_ready \
  "budget-attention-inconsistent-budget" active-health \
  "$(jq -c '.telegram_production_attention_budget_ok = true' <<<"$ok_known_budget_attention_report")"
assert_not_ready \
  "budget-attention-unknown-classification" active-health \
  "$(jq -c '.telegram_production_readiness_classification = "unknown"' <<<"$ok_known_budget_attention_report")"
assert_not_ready \
  "non-budget-attention-contradictory-readiness" active-health \
  "$(jq -c '.telegram_production_readiness_state_known = true' <<<"$ok_known_attention_report")"

wrong_mode_contract="$(
  hepta_watchdog_gate_evidence_contract_json "$active_report" deployment-consistency
)"
jq -e '.ready == false and .observed_mode == "active-health"' >/dev/null \
  <<<"$wrong_mode_contract"

unknown_expected_report="$(jq -c '.watchdog_mode = "unknown-mode"' <<<"$active_report")"
unknown_expected_contract="$(
  hepta_watchdog_gate_evidence_contract_json "$unknown_expected_report" unknown-mode
)"
jq -e '
  .ready == false
  and .artifact_evidence_ready == false
  and .observed_mode == "unknown-mode"
' >/dev/null <<<"$unknown_expected_contract"

unsupported_rc=0
HEPTA_WATCHDOG_GATE_MODE=candidate-artifact hepta_watchdog_gate_mode >/dev/null 2>&1 \
  || unsupported_rc=$?
[[ "$unsupported_rc" == "2" ]]

product_boundary="$(jq -c . "$ROOT/docs/decisions/hepta-product-boundary-v1.json")"
native_post_common='{
  "product":"Hepta",
  "runtime":"hepta",
  "endpoint":"/api/native-post-activation-plan",
  "source_command":"/native-post-activation-plan --json",
  "native_route":true,
  "compatibility_mode":"native_post_activation_plan",
  "side_effect_free":true,
  "activation_currently_enabled":false,
  "handler_candidate_count":3,
  "handler_scope_env":"HEPTA_NATIVE_POST_REAL_HANDLER_SCOPE",
  "handler_scope":"task_publish",
  "handler_scope_configured":true,
  "execution_evidence_ready":true,
  "store_contracts_ready":true,
  "store_jsonl_valid":true,
  "store_capacity_ok":true,
  "required_gates":[
    {"env":"HEPTA_NATIVE_POST_REAL_HANDLERS","enabled":false,"required_for_activation":true},
    {"env":"HEPTA_NATIVE_POST_REAL_HANDLER_APPROVED","enabled":false,"required_for_activation":true},
    {"env":"HEPTA_NATIVE_POST_REAL_HANDLER_SCOPE","enabled":false,"required_for_activation":true}
  ],
  "rollback_anchor_required":true,
  "rollback_store_kind":"rollback",
  "rollback_store_file":"rollback.jsonl",
  "rollback_schema_id":"hepta.post.rollback_anchor.v1",
  "dry_run_only":true,
  "real_mutation_performed":false,
  "store_write_attempted":false,
  "approval_applied":false,
  "task_published":false,
  "chat_mutated":false,
  "external_side_effects":false,
  "gateway_mutation_performed":false,
  "telegram_read_performed":false,
  "model_invoked":false,
  "message_sent":false,
  "cursor_written":false,
  "raw_request_body_exposed":false,
  "raw_idempotency_key_exposed":false,
  "raw_audit_payload_exposed":false
}'
legacy_native_post="$(
  jq -c '. + {
    status:"ready",
    activation_preflight_ready:true,
    activation_blocked_reason:"real_handler_gate_disabled",
    handler_implemented_count:3,
    all_handlers_implemented:true,
    single_handler_scope_ready:true,
    selected_handler_count:1,
    selected_handler_kinds:["task_publish"],
    rollback_ready:true,
    required_gates:(.required_gates | map(
      if .env == "HEPTA_NATIVE_POST_REAL_HANDLER_SCOPE"
      then .enabled = true
      else .
      end
    ))
  }' <<<"$native_post_common"
)"
governed_native_post="$(
  jq -c '. + {
    status:"attention",
    activation_preflight_ready:false,
    activation_blocked_reason:"real_handler_not_implemented",
    handler_implemented_count:0,
    all_handlers_implemented:false,
    single_handler_scope_ready:false,
    selected_handler_count:0,
    selected_handler_kinds:[],
    rollback_ready:false
  }' <<<"$native_post_common"
)"
jq -e '.ready == true and .mode == "legacy_plan_ready"' >/dev/null \
  <<<"$(hepta_watchdog_native_post_contract_json "$product_boundary" "$legacy_native_post")"
jq -e '.ready == true and .mode == "governed_backend_disabled"' >/dev/null \
  <<<"$(hepta_watchdog_native_post_contract_json "$product_boundary" "$governed_native_post")"
jq -e '.ready == false and .mode == "invalid"' >/dev/null \
  <<<"$(hepta_watchdog_native_post_contract_json \
    "$(jq -c '.defaults.native_real_mutation = true' <<<"$product_boundary")" \
    "$governed_native_post")"
jq -e '.ready == false and .mode == "invalid"' >/dev/null \
  <<<"$(hepta_watchdog_native_post_contract_json \
    "$product_boundary" \
    "$(jq -c '.real_mutation_performed = true' <<<"$governed_native_post")")"
jq -e '.ready == false and .mode == "invalid"' >/dev/null \
  <<<"$(hepta_watchdog_native_post_contract_json \
    "$product_boundary" \
    "$(jq -c '.required_gates[0].enabled = true' <<<"$legacy_native_post")")"

echo "Hepta watchdog gate evidence self-test passed"
