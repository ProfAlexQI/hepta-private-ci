#!/usr/bin/env bash

hepta_watchdog_native_post_contract_json() {
  local product_boundary_json="$1"
  local post_json="$2"

  jq -cn \
    --argjson boundary "$product_boundary_json" \
    --argjson post "$post_json" \
    '
      (
        $boundary.schema == "hepta_product_boundary_v1"
        and $boundary.status == "accepted"
        and ($boundary.decided_at | type == "string" and length > 0)
        and $boundary.product_role == "openclaw_governed_backend"
        and $boundary.channel_owner == "legacy_openclaw"
        and $boundary.defaults == {
          telegram_external_read:false,
          telegram_external_send:false,
          telegram_poll_loop_owner:false,
          native_real_mutation:false,
          formal_service_links_full_codex_provider_tool_runner:false
        }
        and $boundary.upstream_intake == "semantic_selective"
        and $boundary.standalone_reconsideration_requires_controlled_live_approval == true
      ) as $boundary_ready
      | ($post.required_gates | INDEX(.env)) as $required_gates
      | (
          ($post.required_gates | type == "array" and length == 3)
          and (($post.required_gates | map(.env) | sort) == [
            "HEPTA_NATIVE_POST_REAL_HANDLERS",
            "HEPTA_NATIVE_POST_REAL_HANDLER_APPROVED",
            "HEPTA_NATIVE_POST_REAL_HANDLER_SCOPE"
          ])
          and ($post.required_gates | all(.required_for_activation == true))
        ) as $required_gates_valid
      | (
          $required_gates_valid
          and $required_gates["HEPTA_NATIVE_POST_REAL_HANDLERS"].enabled == false
          and $required_gates["HEPTA_NATIVE_POST_REAL_HANDLER_APPROVED"].enabled == false
        ) as $mutation_gates_disabled
      | (
          $required_gates_valid
          and $required_gates["HEPTA_NATIVE_POST_REAL_HANDLER_SCOPE"].enabled == true
        ) as $scope_gate_enabled
      | (
          $post.product == "Hepta"
          and $post.runtime == "hepta"
          and $post.endpoint == "/api/native-post-activation-plan"
          and $post.source_command == "/native-post-activation-plan --json"
          and $post.native_route == true
          and $post.compatibility_mode == "native_post_activation_plan"
          and $post.side_effect_free == true
          and $post.activation_currently_enabled == false
          and $post.handler_candidate_count == 3
          and $post.handler_scope_env == "HEPTA_NATIVE_POST_REAL_HANDLER_SCOPE"
          and $post.handler_scope == "task_publish"
          and $post.handler_scope_configured == true
          and $post.execution_evidence_ready == true
          and $post.store_contracts_ready == true
          and $post.store_jsonl_valid == true
          and $post.store_capacity_ok == true
          and $post.rollback_anchor_required == true
          and $post.rollback_store_kind == "rollback"
          and $post.rollback_store_file == "rollback.jsonl"
          and $post.rollback_schema_id == "hepta.post.rollback_anchor.v1"
          and $post.dry_run_only == true
          and $post.real_mutation_performed == false
          and $post.store_write_attempted == false
          and $post.approval_applied == false
          and $post.task_published == false
          and $post.chat_mutated == false
          and $post.external_side_effects == false
          and $post.gateway_mutation_performed == false
          and $post.telegram_read_performed == false
          and $post.model_invoked == false
          and $post.message_sent == false
          and $post.cursor_written == false
          and $post.raw_request_body_exposed == false
          and $post.raw_idempotency_key_exposed == false
          and $post.raw_audit_payload_exposed == false
          and $mutation_gates_disabled
        ) as $common_ready
      | (
          $common_ready
          and $post.status == "ready"
          and $post.activation_preflight_ready == true
          and $post.activation_blocked_reason == "real_handler_gate_disabled"
          and $post.handler_implemented_count == 3
          and $post.all_handlers_implemented == true
          and $post.single_handler_scope_ready == true
          and $post.selected_handler_count == 1
          and $post.selected_handler_kinds == ["task_publish"]
          and $post.rollback_ready == true
          and $scope_gate_enabled
        ) as $legacy_plan_ready
      | (
          $common_ready
          and $post.status == "attention"
          and $post.activation_preflight_ready == false
          and $post.activation_blocked_reason == "real_handler_not_implemented"
          and $post.handler_implemented_count == 0
          and $post.all_handlers_implemented == false
          and $post.single_handler_scope_ready == false
          and $post.selected_handler_count == 0
          and $post.selected_handler_kinds == []
          and $post.rollback_ready == false
          and ($scope_gate_enabled | not)
        ) as $governed_backend_disabled
      | {
          schema:"hepta_watchdog_native_post_contract_v1",
          status:(if $boundary_ready and ($legacy_plan_ready or $governed_backend_disabled)
            then "ready" else "failed" end),
          ready:($boundary_ready and ($legacy_plan_ready or $governed_backend_disabled)),
          product_role:$boundary.product_role,
          product_boundary_ready:$boundary_ready,
          mode:(
            if $boundary_ready and $governed_backend_disabled
            then "governed_backend_disabled"
            elif $boundary_ready and $legacy_plan_ready
            then "legacy_plan_ready"
            else "invalid"
            end
          ),
          legacy_plan_ready:$legacy_plan_ready,
          governed_backend_disabled:$governed_backend_disabled,
          required_gates_valid:$required_gates_valid,
          mutation_gates_disabled:$mutation_gates_disabled,
          scope_gate_enabled:$scope_gate_enabled
        }
    '
}
