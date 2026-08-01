# frozen_string_literal: true

def workgraph_top_level_items(source)
  tokens = HeptaNormalizedTokenBundle.rust_tokens(source)
  items = []
  start_index = 0
  brace_depth = 0
  parenthesis_depth = 0
  bracket_depth = 0
  block_waits_for_semicolon = false
  tokens.each_with_index do |token, token_index|
    next unless token.kind == :fixed

    case token.text
    when "("
      parenthesis_depth += 1
    when ")"
      parenthesis_depth -= 1
    when "["
      bracket_depth += 1
    when "]"
      bracket_depth -= 1
    when "{"
      if brace_depth.zero? && parenthesis_depth.zero? && bracket_depth.zero?
        header_texts = tokens[start_index..token_index].reject do |candidate|
          [:space, :comment].include?(candidate.kind)
        end.map(&:text)
        block_waits_for_semicolon = header_texts.any? do |text|
          %w[use static type].include?(text)
        end || (header_texts.include?("const") && !header_texts.include?("fn"))
      end
      brace_depth += 1
    when "}"
      brace_depth -= 1
      if brace_depth.zero? && parenthesis_depth.zero? && bracket_depth.zero?
        following = tokens[(token_index + 1)..]&.find do |candidate|
          ![:space, :comment].include?(candidate.kind)
        end
        unless block_waits_for_semicolon || following&.text == ";"
          items << tokens[start_index..token_index].map(&:text).join
          start_index = token_index + 1
          block_waits_for_semicolon = false
        end
      end
    when ";"
      if brace_depth.zero? && parenthesis_depth.zero? && bracket_depth.zero?
        items << tokens[start_index..token_index].map(&:text).join
        start_index = token_index + 1
        block_waits_for_semicolon = false
      end
    end
    if brace_depth.negative? || parenthesis_depth.negative? || bracket_depth.negative?
      raise "WorkGraph top-level item delimiter underflow"
    end
  end
  tail = tokens[start_index..]&.map(&:text)&.join
  if tail && !tail.empty?
    if tail.strip.empty? && !items.empty?
      items[-1] << tail
    else
      items << tail
    end
  end
  unless [brace_depth, parenthesis_depth, bracket_depth] == [0, 0, 0]
    raise "WorkGraph top-level item delimiters are unbalanced"
  end
  raise "WorkGraph top-level item reconstruction drifted" unless items.join == source

  items
end

# Function bodies are statement/expression grammars rather than item grammars.
# A closing brace is not a safe boundary there: it can be followed by `else`,
# `.await`, `?`, or another operator. Split only after top-level semicolons and
# keep the remaining tail expression intact. Adjacent semicolon-free block
# expressions deliberately remain one unit.
def workgraph_function_body_units(source)
  tokens = HeptaNormalizedTokenBundle.rust_tokens(source)
  units = []
  start_index = 0
  brace_depth = 0
  parenthesis_depth = 0
  bracket_depth = 0
  tokens.each_with_index do |token, token_index|
    next unless token.kind == :fixed

    case token.text
    when "("
      parenthesis_depth += 1
    when ")"
      parenthesis_depth -= 1
    when "["
      bracket_depth += 1
    when "]"
      bracket_depth -= 1
    when "{"
      brace_depth += 1
    when "}"
      brace_depth -= 1
    when ";"
      if brace_depth.zero? && parenthesis_depth.zero? && bracket_depth.zero?
        units << tokens[start_index..token_index].map(&:text).join
        start_index = token_index + 1
      end
    end
    if brace_depth.negative? || parenthesis_depth.negative? || bracket_depth.negative?
      raise "WorkGraph function-body delimiter underflow"
    end
  end
  tail = tokens[start_index..]&.map(&:text)&.join
  if tail && !tail.empty?
    if tail.strip.empty? && !units.empty?
      units[-1] << tail
    else
      units << tail
    end
  end
  unless [brace_depth, parenthesis_depth, bracket_depth] == [0, 0, 0]
    raise "WorkGraph function-body delimiters are unbalanced"
  end
  raise "WorkGraph function-body reconstruction drifted" unless units.join == source

  units
end

def workgraph_comma_sequence_units(source)
  tokens = HeptaNormalizedTokenBundle.rust_tokens(source)
  units = []
  separators = []
  start_index = 0
  brace_depth = 0
  parenthesis_depth = 0
  bracket_depth = 0
  tokens.each_with_index do |token, token_index|
    next unless token.kind == :fixed

    case token.text
    when "("
      parenthesis_depth += 1
    when ")"
      parenthesis_depth -= 1
    when "["
      bracket_depth += 1
    when "]"
      bracket_depth -= 1
    when "{"
      brace_depth += 1
    when "}"
      brace_depth -= 1
    when ","
      if brace_depth.zero? && parenthesis_depth.zero? && bracket_depth.zero?
        units << tokens[start_index...token_index].map(&:text).join
        separators << token.text
        start_index = token_index + 1
      end
    end
    if brace_depth.negative? || parenthesis_depth.negative? || bracket_depth.negative?
      raise "WorkGraph comma-sequence delimiter underflow"
    end
  end
  tail = tokens[start_index..]&.map(&:text)&.join
  if tail && !tail.empty?
    if tail.strip.empty? && !units.empty?
      separators[-1] << tail
    else
      units << tail
    end
  end
  unless [brace_depth, parenthesis_depth, bracket_depth] == [0, 0, 0]
    raise "WorkGraph comma-sequence delimiters are unbalanced"
  end
  reconstructed = units.each_with_index.map do |unit, unit_index|
    unit + separators.fetch(unit_index, "")
  end.join
  raise "WorkGraph comma-sequence reconstruction drifted" unless reconstructed == source

  { units: units, separators: separators }
end

def workgraph_vec_sequence_parts(source)
  tokens = HeptaNormalizedTokenBundle.rust_tokens(source)
  significant = tokens.each_index.reject do |token_index|
    [:space, :comment].include?(tokens.fetch(token_index).kind)
  end
  open_index = nil
  significant.each_cons(3) do |left, middle, right|
    next unless tokens.fetch(left).kind == :identifier && tokens.fetch(left).text == "vec"
    next unless tokens.fetch(middle).kind == :fixed && tokens.fetch(middle).text == "!"
    next unless tokens.fetch(right).kind == :fixed && tokens.fetch(right).text == "["

    open_index = right
    break
  end
  return nil unless open_index

  depth = 0
  close_index = nil
  tokens.each_with_index do |token, token_index|
    next if token_index < open_index || token.kind != :fixed
    depth += 1 if token.text == "["
    depth -= 1 if token.text == "]"
    if depth.zero?
      close_index = token_index
      break
    end
  end
  raise "WorkGraph vec sequence is unbalanced" unless close_index

  header = tokens[0..open_index].map(&:text).join
  body = tokens[(open_index + 1)...close_index].map(&:text).join
  footer = tokens[close_index..].map(&:text).join
  sequence = workgraph_comma_sequence_units(body)
  children = sequence.fetch(:units)
  return nil if children.length < 2
  raise "WorkGraph vec sequence reconstruction drifted" unless header + body + footer == source

  {
    context: "vec",
    header: header,
    children: children,
    separators: sequence.fetch(:separators),
    footer: footer
  }
end

def workgraph_item_signature(source)
  HeptaNormalizedTokenBundle.rust_tokens(source).map do |token|
    token.kind == :fixed ? "fixed:#{token.text}" : token.kind.to_s
  end.join("\0")
end

def canonical_workgraph_item_source(source)
  output = +""
  pending_separator = false
  HeptaNormalizedTokenBundle.rust_tokens(source).each do |token|
    case token.kind
    when :space, :comment
      pending_separator = true
    else
      output << " " if pending_separator && !output.empty? && !output.end_with?(" ")
      output << token.text
      pending_separator = false
    end
  end
  output
end

def workgraph_item_macro_candidate?(group)
  return false if group.length < 2
  return false if group.any? { |item| item.fetch(:source).include?("macro_rules!") }
  return false if group.any? { |item| item.fetch(:source).match?(/\$[A-Za-z_(]/) }

  true
end

def workgraph_item_expansion(tokens, slot_positions, replacements, stats)
  replacement_by_position = slot_positions.zip(replacements).to_h
  output = +""
  pending_separator = false
  tokens.each_with_index do |token, token_index|
    if replacement_by_position.key?(token_index)
      replacement = workgraph_macro_replacement(
        token.kind,
        replacement_by_position.fetch(token_index),
        stats
      )
      output << " " if pending_separator && !output.end_with?(" ")
      output << replacement
      pending_separator = false
      next
    end
    case token.kind
    when :space, :comment
      pending_separator = true
    else
      output << " " if pending_separator && !output.end_with?(" ")
      output << token.text
      pending_separator = false
    end
  end
  output
end

def workgraph_macro_replacement(kind, replacement, stats)
  return replacement if kind == :fixed

  canonical_replacement(kind, replacement, stats)
end

def workgraph_item_record(key, source, signature_prefix = "")
  canonical_source = canonical_workgraph_item_source(source)
  {
    key: key,
    context: signature_prefix,
    source: source,
    canonical_source: canonical_source,
    signature: "#{signature_prefix}\0#{workgraph_item_signature(canonical_source)}"
  }
end

def workgraph_macroize_item_records(family_module, macro_scope, item_records)
  groups = item_records.group_by { |item| item.fetch(:signature) }.values
  macro_definitions = []
  invocations = {}
  stats = {
    canonicalized_space_token_count: 0,
    omitted_comment_token_count: 0,
    exact_value_token_count: 0
  }
  groups.select { |group| workgraph_item_macro_candidate?(group) }.sort_by do |group|
    group.first.fetch(:signature)
  end.each do |group|
    normalized_inputs = group.map do |item|
      name = item.fetch(:key).join(":")
      HeptaNormalizedTokenBundle::Entry.new(name, item.fetch(:canonical_source))
    end
    bundle = HeptaNormalizedTokenBundle.build(
      normalized_inputs,
      tokenizer: HeptaNormalizedTokenBundle.method(:rust_tokens)
    )
    next unless bundle.families.length == 1

    family = bundle.families.first
    normalized = group.map do |item|
      {
        name: item.fetch(:key).join(":"),
        source: item.fetch(:canonical_source)
      }
    end
    slot_kinds, slot_positions, template_tokens = slot_layout_for(normalized, family)
    # Statement-level macros must receive identifiers from the invocation site.
    # Leaving an identical identifier in the macro definition gives it
    # definition-site hygiene, so local bindings and later uses cannot see one
    # another. Declaration-level macros keep the compact varying-value layout.
    hygienic_statement = %w[fn vec].include?(group.first.fetch(:context))
    if hygienic_statement
      invocation_site_positions = template_tokens.each_index.select do |token_index|
        token = template_tokens.fetch(token_index)
        token.kind == :identifier || (token.kind == :fixed && %w[self Self].include?(token.text))
      end
      slot_positions = (slot_positions + invocation_site_positions).uniq.sort
      slot_kinds = slot_positions.map { |token_index| template_tokens.fetch(token_index).kind }
    end
    encoded_by_name = bundle.entries.to_h { |entry| [entry.name, entry] }
    macro_id = Digest::SHA256.hexdigest(
      "#{macro_scope}\0#{group.first.fetch(:signature)}"
    )[0, 16]
    macro_name = "wg_f_#{macro_id}"
    macro_source = +"#[allow(unused_macros)]macro_rules! #{macro_name}{("
    slot_positions.length.times do |slot_id|
      slot = format("s%03d", slot_id)
      macro_source << "#{slot}=[$($#{slot}:tt)*];"
    end
    macro_source << ")=>{"
    macro_source << compact_template_body(template_tokens, slot_positions)
    macro_source << "};}\n"

    candidate_invocations = {}
    candidate_stats = {
      canonicalized_space_token_count: 0,
      omitted_comment_token_count: 0,
      exact_value_token_count: 0
    }
    group.each do |item|
      encoded_name = item.fetch(:key).join(":")
      encoded = encoded_by_name.fetch(encoded_name)
      replacements = if hygienic_statement
        item_tokens = HeptaNormalizedTokenBundle.rust_tokens(item.fetch(:canonical_source))
        slot_positions.map { |token_index| item_tokens.fetch(token_index).text }
      else
        encoded.replacements
      end
      invocation = +"#{macro_name}!{"
      replacements.each_with_index do |replacement, replacement_id|
        slot = format("s%03d", replacement_id)
        canonical = workgraph_macro_replacement(
          slot_kinds.fetch(replacement_id),
          replacement,
          candidate_stats
        )
        invocation << "#{slot}=[#{canonical}];"
      end
      invocation << "}\n"
      expanded = workgraph_item_expansion(
        template_tokens,
        slot_positions,
        replacements,
        {
          canonicalized_space_token_count: 0,
          omitted_comment_token_count: 0,
          exact_value_token_count: 0
        }
      )
      unless semantic_token_digest(expanded) == semantic_token_digest(item.fetch(:source))
        raise "WorkGraph item macro semantic drifted: #{encoded_name}"
      end
      candidate_invocations[item.fetch(:key)] = invocation
    end
    original_lines = group.sum { |item| item.fetch(:source).count("\n") }
    generated_lines = macro_source.count("\n") + candidate_invocations.values.sum do |invocation|
      invocation.count("\n")
    end
    next unless generated_lines < original_lines

    macro_definitions << macro_source
    candidate_invocations.each do |key, invocation|
      raise "duplicate WorkGraph item macro invocation" if invocations.key?(key)
      invocations[key] = invocation
    end
    candidate_stats.each_key { |key| stats[key] += candidate_stats.fetch(key) }
  end
  [macro_definitions, invocations, stats]
end

def workgraph_inline_item_parts(source, parent_context)
  if parent_context == "fn"
    vec_parts = workgraph_vec_sequence_parts(source)
    return vec_parts if vec_parts
  end
  return nil if parent_context == "vec"

  tokens = HeptaNormalizedTokenBundle.rust_tokens(source)
  open_index = tokens.index { |token| token.kind == :fixed && token.text == "{" }
  return nil unless open_index

  header_texts = tokens[0...open_index].reject do |token|
    [:space, :comment].include?(token.kind)
  end.map(&:text)
  # Recurse only through declaration containers. Function bodies use a
  # dedicated statement-safe splitter below; control-flow bodies stay intact.
  context = header_texts.find do |text|
    %w[mod impl trait fn].include?(text)
  end
  return nil unless context

  depth = 0
  close_index = nil
  tokens.each_with_index do |token, token_index|
    next if token_index < open_index || token.kind != :fixed
    depth += 1 if token.text == "{"
    depth -= 1 if token.text == "}"
    if depth.zero?
      close_index = token_index
      break
    end
  end
  raise "WorkGraph inline item body is unbalanced" unless close_index

  header = tokens[0..open_index].map(&:text).join
  body = tokens[(open_index + 1)...close_index].map(&:text).join
  footer = tokens[close_index..].map(&:text).join
  raise "WorkGraph inline item reconstruction drifted" unless header + body + footer == source

  {
    context: context,
    header: header,
    children: context == "fn" ? workgraph_function_body_units(body) : workgraph_top_level_items(body),
    footer: footer
  }
end

def workgraph_recursive_item_macro_plan(family_module, records, depth)
  scope = format("item_d%02d", depth)
  definitions, invocations, stats = workgraph_macroize_item_records(
    family_module,
    scope,
    records
  )
  inline_parts = {}
  child_records = []
  records.each do |record|
    key = record.fetch(:key)
    next if invocations.key?(key)

    parts = workgraph_inline_item_parts(record.fetch(:source), record.fetch(:context))
    next unless parts
    inline_parts[key] = parts
    parts.fetch(:children).each_with_index do |source, child_index|
      child_records << workgraph_item_record(
        key + [child_index],
        source,
        parts.fetch(:context)
      )
    end
  end
  unless child_records.empty?
    child_definitions, child_invocations, child_stats =
      workgraph_recursive_item_macro_plan(family_module, child_records, depth + 1)
    inline_parts.each do |key, parts|
      rebuilt = +parts.fetch(:header)
      parts.fetch(:children).each_with_index do |source, child_index|
        rebuilt << child_invocations.fetch(key + [child_index], source)
        rebuilt << parts.fetch(:separators, []).fetch(child_index, "")
      end
      rebuilt << parts.fetch(:footer)
      original = records.find { |record| record.fetch(:key) == key }.fetch(:source)
      invocations[key] = rebuilt if rebuilt != original
    end
    definitions.concat(child_definitions)
    stats.each_key { |key| stats[key] += child_stats.fetch(key) }
  end
  [definitions, invocations, stats]
end

def workgraph_item_macro_plan(family_module, sanitized)
  top_level_records = sanitized.flat_map do |variant|
    variant_name = variant.fetch(:name)
    workgraph_top_level_items(variant.fetch(:source)).each_with_index.map do |source, item_index|
      workgraph_item_record([variant_name, item_index], source, "module")
    end
  end
  workgraph_recursive_item_macro_plan(family_module, top_level_records, 0)
end

def heterogeneous_family_source(family_module, chunk)
  sanitized, renamed_private_symbols = sanitize_chunk(chunk)
  macro_definitions, invocations, item_stats = workgraph_item_macro_plan(
    family_module,
    sanitized
  )
  output = macro_definitions.join
  sanitized.each do |variant|
    baseline_digest = Digest::SHA256.hexdigest(variant.fetch(:baseline_source))
    variant_module = family_variant_module_name(variant.fetch(:name))
    output << "/*workgraph-family-variant:#{variant.fetch(:name)}:sha256=#{baseline_digest}*/"
    output << "/*workgraph-family-alias:#{variant.fetch(:name)}:#{variant_module}*/"
    output << "#[allow(dead_code,unused_imports)]pub(crate) mod #{variant_module}{\n"
    workgraph_top_level_items(variant.fetch(:source)).each_with_index do |item, item_index|
      output << invocations.fetch([variant.fetch(:name), item_index], item)
    end
    output << "\n}\n"
  end
  output << "const _:crate::work_graph_family_state_machine_core::WorkGraphFamilyStateMachine=" \
    "crate::work_graph_family_state_machine_runner::run_work_graph_family_state_machine(" \
    "crate::work_graph_family_state_machine_core::WorkGraphFamilyIdentity::new(" \
    "\"#{family_module}\",#{chunk.length})," \
    "crate::work_graph_family_state_machine_core::WorkGraphFamilyCallerClosure::Preserved," \
    "crate::work_graph_family_state_machine_core::WorkGraphFamilyRecoveryClosure::GitBound);\n"
  stats = {
    canonicalized_space_token_count: item_stats.fetch(:canonicalized_space_token_count),
    omitted_comment_token_count: item_stats.fetch(:omitted_comment_token_count),
    exact_value_token_count: item_stats.fetch(:exact_value_token_count),
    renamed_private_symbol_count: renamed_private_symbols
  }
  [output.b, stats, sanitized]
end
