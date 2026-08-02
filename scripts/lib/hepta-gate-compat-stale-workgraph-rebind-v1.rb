# frozen_string_literal: true

require "digest"
require "json"
require "set"
require_relative "hepta-normalized-token-bundle-v1"
require_relative "../hepta-workgraph-readable-token-source-v1"

module HeptaGateCompatStaleWorkgraphRebind
  FAMILY_SCHEMA = "hepta_gate_compat_family_payloads_v1"
  FAMILY_DOMAIN = "hepta_gate_compat_family_payloads"
  REPORT_NAME_SHA256 = "10f47dc61b2cc2c23b4eced6d0749afe09cee6745c40acf2795a1f1453716216"
  PROBE_PLAN_BYTES = 7_309
  PROBE_PLAN_SHA256 = "bb3cb933d7625c4b9c204dac62c41f76d97f7a44775d94af2aa3236ff720d992"
  TERMINAL_PREDECESSOR_FIELDS = %w[
    prior_report_id
    prior_typed_gate
    prior_typed_schema
    prior_ready_field
    prior_compatibility_gate
  ].freeze
  TERMINAL_PREDECESSOR_TUPLE_SHA256 = {
    "hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-ack-replay-idempotency-preview.report" =>
      "6ef68a8fa861835ddcdb9097006ff2cf41ad9282e6751d120b30f9b770817097",
    "hepta-systems-work-graph-terminal-receipt-retention-readback-ack-terminal-decision-receipt-retention-readback-ack-terminal-decision-receipt-acknowledgement-replay-idempotency-preview.report" =>
      "acfb3a2d007bb242ec784fb8b64f6e6cc3cdda735d9489dcba850ee1307497f9"
  }.freeze
  REBIND_TARGET = "typed_workgraph_anchor_gate_membership_and_typed_predecessor_compatibility"
  CURRENT_REALITY_LEGACY_EXACT_HEAD_COMMIT_SHA = "a4b33153dd521746a4fbf2190d8e82912b65f186"
  CURRENT_REALITY_LEGACY_EXACT_HEAD_RECEIPT_SHA256 = "f553fccac558170c8c23a742a397e4d77f173d9dfec3d0ae5e645a81ac51e5de"

  module_function

  def verify_current_reality_receipt!(conversion)
    raise "current-reality execution receipt drifted" unless
      conversion.fetch("current_reality_parity_oracle_execution_status") ==
        "typed_report_executed_with_legacy_exact_head_receipt_bound_not_reexecuted" &&
        conversion.fetch("current_reality_typed_report_executed_this_run") == true &&
        conversion.fetch("current_reality_typed_report_digest_mode") == "sha256_exact_stdout_bytes" &&
        conversion.fetch("current_reality_typed_report_sha256").match?(/\A[0-9a-f]{64}\z/) &&
        conversion.fetch("current_reality_legacy_exact_head_receipt_bound") == true &&
        conversion.fetch("current_reality_legacy_exact_head_report_executed_this_run") == false &&
        conversion.fetch("current_reality_legacy_exact_head_commit_sha") ==
          CURRENT_REALITY_LEGACY_EXACT_HEAD_COMMIT_SHA &&
        conversion.fetch("current_reality_legacy_exact_head_receipt_sha256") ==
          CURRENT_REALITY_LEGACY_EXACT_HEAD_RECEIPT_SHA256
  end

  def terminal_predecessor_tuple_sha256(source)
    values = TERMINAL_PREDECESSOR_FIELDS.map do |field|
      escaped = Regexp.escape(field)
      assignments = source.scan(/\b#{escaped}=/)
      exact_values = source.scan(/^#{escaped}="([^"]+)"$/).flatten
      raise "stale WorkGraph terminal predecessor field #{field} is not uniquely assigned" unless
        assignments.length == 1 && exact_values.length == 1
      exact_values.fetch(0)
    end
    Digest::SHA256.hexdigest(JSON.generate(values))
  end

  def verify!(store_root:, conversion:, dead:, additional_retired_pair_id:, plugin_retired_pair_ids:)
    expected_zero_caller = ([additional_retired_pair_id] + plugin_retired_pair_ids).to_set
    observed_zero_caller = conversion.fetch("additional_zero_caller_pair_retirement_ids").to_set
    raise "additional zero-caller retirement accounting drifted" unless
      conversion.fetch("additional_zero_caller_pair_retirement_count") == 5 &&
        observed_zero_caller == expected_zero_caller
    stale = conversion.fetch("stale_workgraph_successor_rebound_retirement_pair_ids").to_set
    raise "stale WorkGraph successor-rebound retirement is unbound" unless
      conversion.fetch("stale_workgraph_successor_rebound_retirement_pair_count") == 21 &&
        stale.length == 21 && stale.subset?(dead) && (stale & observed_zero_caller).empty?
    raise "stale WorkGraph source-probe receipt drifted" unless
      conversion.fetch("stale_workgraph_family_consumer_rebind_count") == 23 &&
        conversion.fetch("stale_workgraph_family_legacy_rust_probe_typed_successor_rebind_count") == 61 &&
        conversion.fetch("stale_workgraph_family_typed_anchor_gate_membership_rebind_count") == 59 &&
        conversion.fetch("stale_workgraph_family_typed_predecessor_compatibility_rebind_count") == 2 &&
        conversion.fetch("stale_workgraph_family_generic_exports_path_probe_count").zero? &&
        conversion.fetch("stale_workgraph_family_legacy_rust_module_presence_claimed") == false &&
        conversion.fetch("stale_workgraph_family_rebind_target") == REBIND_TARGET
    entries = HeptaReadableTokenSource.read(
      root: store_root, artifact_schema: FAMILY_SCHEMA, domain: FAMILY_DOMAIN,
      name_validator: ->(name) { name.match?(/\A[a-z0-9-]+\.(?:gate|report)\z/) },
      strict_metadata: true
    ).first
    sources = entries.to_h { |entry| [entry.name, entry.source] }
    reports = entries.select do |entry|
      entry.source.include?("typed_workgraph_gate_backed") ||
        entry.source.include?("typed_prior_compatibility_backed")
    end
    names = reports.map(&:name).sort
    raise "stale WorkGraph exact rebound report inventory drifted" unless
      reports.length == 23 && Digest::SHA256.hexdigest(names.join("\n") + "\n") == REPORT_NAME_SHA256
    regular = reports.sum { |entry| entry.source.scan(/bool_for typed_workgraph_gate_backed [a-z0-9_]+/).length }
    predecessor = reports.sum { |entry| entry.source.scan("bool_for typed_prior_compatibility_backed").length }
    anchors = reports.count { |entry| entry.source.include?("typed_workgraph_family_anchor_report=") }
    raise "stale WorkGraph decoded semantic probe count drifted" unless
      regular == 59 && predecessor == 2 && anchors == 21
    terminal_predecessor_tuples = reports.each_with_object({}) do |entry, tuples|
      next unless entry.source.include?("bool_for typed_prior_compatibility_backed")
      tuples[entry.name] = terminal_predecessor_tuple_sha256(entry.source)
    end
    raise "stale WorkGraph terminal predecessor mapping drifted" unless
      terminal_predecessor_tuples == TERMINAL_PREDECESSOR_TUPLE_SHA256
    probe_plan = reports.sort_by(&:name).to_h do |entry|
      gates = entry.source.scan(/bool_for typed_workgraph_gate_backed ([a-z0-9_]+)/).flatten
      if gates.empty? && entry.source.include?("bool_for typed_prior_compatibility_backed")
        gates = ["typed_predecessor_compatibility:#{terminal_predecessor_tuples.fetch(entry.name)}"]
      end
      [entry.name, gates]
    end
    serialized_probe_plan = JSON.generate(probe_plan)
    raise "stale WorkGraph decoded semantic probe plan drifted" unless
      probe_plan.length == 23 && probe_plan.values.sum(&:length) == 61 &&
        serialized_probe_plan.bytesize == PROBE_PLAN_BYTES &&
        Digest::SHA256.hexdigest(serialized_probe_plan) == PROBE_PLAN_SHA256
    raise "stale WorkGraph decoded source retained a fake module probe" if reports.any? do |entry|
      entry.source.include?("runtime_kernel/exports_workgraph.rs") ||
        entry.source.match?(%r{path_exists codex-rs/hepta-runtime/src/(?:work_graph|wg)_[a-z0-9_]+\.rs})
    end
    fields = reports.sum { |entry| entry.source.scan(/\brust_module_present:/).length }
    assertions = names.sum do |report_name|
      sources.fetch(report_name.sub(/\.report\z/, ".gate")).scan(/\.rust_module_present == true/).length
    end
    raise "stale WorkGraph compatibility schema continuity drifted" unless
      fields == 61 && assertions == 61
    true
  end
end
