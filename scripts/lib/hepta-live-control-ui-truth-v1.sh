#!/usr/bin/env bash

# Fail-closed schema and truth contract for the two live reports that carry
# Control UI product evidence. A candidate preflight may interrogate an older
# active binary before installing the candidate. In active-health mode only,
# an entirely legacy pair (both reports omit every new UI field) is useful as
# active-health and side-effect-denial evidence, but never as UI product
# completion evidence. Partial migrations are rejected in every mode.

hepta_live_control_ui_truth_contract_json() {
  local ga_json="$1"
  local merge_json="$2"
  local watchdog_gate_mode="$3"

  jq -cn \
    --arg watchdog_gate_mode "$watchdog_gate_mode" \
    --argjson ga "$ga_json" \
    --argjson merge "$merge_json" \
    '
      def evidence_layer_shape($layer):
        ($layer | type) == "object"
        and ($layer | has("status"))
        and ($layer | has("coverage_percent"))
        and ($layer | has("verified"))
        and ($layer | has("evidence_ref"));
      def evidence_coverage_shape($evidence):
        ($evidence | type) == "object"
        and ($evidence | has("schema_version"))
        and ($evidence | has("static_contract"))
        and ($evidence | has("unit_state"))
        and ($evidence | has("browser_behavior"))
        and ($evidence | has("backend_mutation_readback"))
        and ($evidence | has("live_adapter"))
        and ($evidence | has("overall_evidence_percent"))
        and ($evidence | has("all_required_layers_verified"))
        and ($evidence | has("boundary"))
        and evidence_layer_shape($evidence.static_contract)
        and evidence_layer_shape($evidence.unit_state)
        and evidence_layer_shape($evidence.browser_behavior)
        and evidence_layer_shape($evidence.backend_mutation_readback)
        and evidence_layer_shape($evidence.live_adapter);
      [
        ($ga | has("control_ui_product_status")),
        ($ga | has("control_ui_product_complete")),
        ($ga | has("control_ui_live_operator_surface_percent")),
        ($ga | has("control_ui_overall_evidence_percent")),
        ($merge | has("control_ui_product_status")),
        ($merge | has("control_ui_product_complete")),
        ($merge | has("control_ui_live_operator_surface_percent")),
        ($merge | has("control_ui_evidence"))
      ] as $field_presence
      | (
          $watchdog_gate_mode == "deployment-consistency"
          or $watchdog_gate_mode == "active-health"
        ) as $mode_known
      | ($field_presence | all(. == true)) as $current_schema_present
      | ($field_presence | all(. == false)) as $legacy_schema_present
      | (
          $ga.status == "blocked"
          and $ga.public_ga_ready == false
          and $ga.public_ga_claimed == false
          and $ga.operator_approval_required == true
          and ($ga.blockers | type) == "array"
          and ($ga.blockers | length) > 0
          and ($ga.blocker_count | type) == "number"
          and $ga.blocker_count == ($ga.blockers | length)
          and $merge.status == "attention"
          and $merge.public_ga_claimed == false
          and ($merge.blockers | type) == "array"
          and ($merge.blockers | length) > 0
        ) as $common_denial_truth
      | (
          $current_schema_present
          and $common_denial_truth
          and $ga.control_ui_product_status == "static_contract_complete"
          and $ga.control_ui_product_complete == false
          and $ga.control_ui_live_operator_surface_percent == 0
          and $ga.control_ui_overall_evidence_percent == 20
          and ($ga.production_replacement_percent | type) == "number"
          and ($merge.production_replacement_percent | type) == "number"
          and $ga.production_replacement_percent >= 0
          and $ga.production_replacement_percent < 100
          and $ga.production_replacement_percent == ($ga.production_replacement_percent | floor)
          and $merge.production_replacement_percent >= 0
          and $merge.production_replacement_percent < 100
          and $merge.production_replacement_percent == ($merge.production_replacement_percent | floor)
          and $ga.production_replacement_percent == $merge.production_replacement_percent
          and ($ga.blockers | type) == "array"
          and ($ga.blockers | index("control_ui_product_behavior_evidence_not_bound")) != null
          and $merge.control_ui_product_status == "static_contract_complete"
          and $merge.control_ui_product_complete == false
          and $merge.control_ui_live_operator_surface_percent == 0
          and evidence_coverage_shape($merge.control_ui_evidence)
          and $merge.control_ui_evidence.schema_version == 1
          and $merge.control_ui_evidence.static_contract.status == "verified"
          and $merge.control_ui_evidence.static_contract.coverage_percent == 100
          and $merge.control_ui_evidence.static_contract.verified == true
          and ($merge.control_ui_evidence.static_contract.evidence_ref | type) == "string"
          and ($merge.control_ui_evidence.static_contract.evidence_ref | length) > 0
          and $merge.control_ui_evidence.unit_state.status == "not_bound_to_report"
          and $merge.control_ui_evidence.unit_state.coverage_percent == 0
          and $merge.control_ui_evidence.unit_state.verified == false
          and $merge.control_ui_evidence.unit_state.evidence_ref == null
          and $merge.control_ui_evidence.browser_behavior.status == "not_bound_to_report"
          and $merge.control_ui_evidence.browser_behavior.coverage_percent == 0
          and $merge.control_ui_evidence.browser_behavior.verified == false
          and $merge.control_ui_evidence.browser_behavior.evidence_ref == null
          and $merge.control_ui_evidence.backend_mutation_readback.status == "not_bound_to_report"
          and $merge.control_ui_evidence.backend_mutation_readback.coverage_percent == 0
          and $merge.control_ui_evidence.backend_mutation_readback.verified == false
          and $merge.control_ui_evidence.backend_mutation_readback.evidence_ref == null
          and $merge.control_ui_evidence.live_adapter.status == "not_bound_to_report"
          and $merge.control_ui_evidence.live_adapter.coverage_percent == 0
          and $merge.control_ui_evidence.live_adapter.verified == false
          and $merge.control_ui_evidence.live_adapter.evidence_ref == null
          and $merge.control_ui_evidence.overall_evidence_percent == 20
          and $merge.control_ui_evidence.all_required_layers_verified == false
          and $merge.control_ui_evidence.boundary == "Source markers and declared smoke commands prove only the static contract. Unit/state runs, real browser behavior, backend mutation/readback, and live-adapter evidence must be bound explicitly before product completion or a 100% live operator surface may be claimed."
          and ($merge.blockers | type) == "array"
          and ($merge.blockers | index("control_ui_product_behavior_evidence_not_bound")) != null
        ) as $current_truth_ready
      | (
          $mode_known
          and
          $watchdog_gate_mode == "active-health"
          and $legacy_schema_present
          and $common_denial_truth
          and ($ga.production_replacement_percent | type) == "number"
          and ($merge.production_replacement_percent | type) == "number"
          and $ga.production_replacement_percent == 100
          and $merge.production_replacement_percent == 100
          and $ga.production_replacement_percent == $merge.production_replacement_percent
          and $merge.readiness_class == "active_production_replacement_ready"
        ) as $legacy_active_only_ready
      | {
          schema_version:"hepta_live_control_ui_truth_v1",
          watchdog_gate_mode:$watchdog_gate_mode,
          mode_known:$mode_known,
          schema_mode:(
            if ($mode_known | not) then "unknown_mode"
            elif $current_schema_present then "current_truth_v1"
            elif $legacy_active_only_ready then "legacy_active_only"
            elif $legacy_schema_present then "legacy_rejected_in_strict_mode"
            else "partial_or_unknown"
            end
          ),
          schema_checked:($mode_known and ($current_schema_present or $legacy_schema_present)),
          control_ui_truth_checked:($mode_known and $current_truth_ready),
          current_schema_present:$current_schema_present,
          legacy_schema_present:$legacy_schema_present,
          partial_schema_present:(($current_schema_present or $legacy_schema_present) | not),
          current_truth_ready:$current_truth_ready,
          legacy_accepted:$legacy_active_only_ready,
          legacy_active_only:$legacy_active_only_ready,
          production_semantics_checked:($mode_known and $current_truth_ready),
          product_completion_claim_allowed:false,
          product_completion_claimed:false,
          reports_sync_scope:(
            if ($mode_known and $current_truth_ready) then "full_including_control_ui_truth"
            elif $legacy_active_only_ready then "legacy_base_reports_only"
            else "none"
            end
          ),
          ready:($mode_known and ($current_truth_ready or $legacy_active_only_ready))
        }
    '
}
