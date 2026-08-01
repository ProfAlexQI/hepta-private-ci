# frozen_string_literal: true

def heterogeneous_family_source(family_module, chunk)
  sanitized, renamed_private_symbols = sanitize_chunk(chunk)
  output = +""
  sanitized.each do |variant|
    baseline_digest = Digest::SHA256.hexdigest(variant.fetch(:baseline_source))
    variant_module = family_variant_module_name(variant.fetch(:name))
    output << "/*workgraph-family-variant:#{variant.fetch(:name)}:sha256=#{baseline_digest}*/"
    output << "/*workgraph-family-alias:#{variant.fetch(:name)}:#{variant_module}*/"
    output << "#[allow(dead_code,unused_imports)]pub(crate) mod #{variant_module}{\n"
    output << variant.fetch(:source)
    output << "\n}\n"
  end
  output << "const _:crate::work_graph_family_state_machine_core::WorkGraphFamilyStateMachine=" \
    "crate::work_graph_family_state_machine_runner::run_work_graph_family_state_machine(" \
    "crate::work_graph_family_state_machine_core::WorkGraphFamilyIdentity::new(" \
    "\"#{family_module}\",#{chunk.length})," \
    "crate::work_graph_family_state_machine_caller_closure::WorkGraphFamilyCallerClosure::Preserved," \
    "crate::work_graph_family_state_machine_recovery_closure::WorkGraphFamilyRecoveryClosure::GitBound);\n"
  stats = {
    canonicalized_space_token_count: 0,
    omitted_comment_token_count: 0,
    exact_value_token_count: 0,
    renamed_private_symbol_count: renamed_private_symbols
  }
  [output.b, stats, sanitized]
end
