# frozen_string_literal: true

require "digest"
require "json"
require "set"

module HeptaWorkgraphRetirement
  SCHEMA = "hepta_workgraph_ast_caller_scc_retirement_v1"
  EXPORT_FILES = %w[
    codex-rs/hepta-runtime/src/runtime_kernel/exports.rs
    codex-rs/hepta-runtime/src/runtime_kernel/exports_workgraph.rs
    codex-rs/hepta-runtime/src/runtime_kernel/exports/workgraph_projection.rs
  ].freeze
  PUBLIC_SYMBOL_PATTERN =
    /^pub(?:\([^)]*\))?\s+(?:async\s+)?(?:fn|struct|enum|const|static|type|trait)\s+([A-Za-z_][A-Za-z0-9_]*)/

  module_function

  def inventory(receipt)
    seeds = receipt.fetch("seed_modules")
    retired = receipt.fetch("retired_modules").map { |entry| entry.fetch("name") }
    exported = receipt.fetch("public_exports_removed")
    raise "WorkGraph retirement seed inventory is empty" if seeds.empty?
    raise "WorkGraph retirement inventory is empty" if retired.empty?
    raise "WorkGraph retirement seed inventory is not unique" unless seeds.uniq.length == seeds.length
    raise "WorkGraph retirement inventory is not unique" unless retired.uniq.length == retired.length
    raise "WorkGraph retirement export inventory is not unique" unless exported.uniq.length == exported.length
    raise "WorkGraph retirement seed escaped inventory" unless (seeds - retired).empty?
    raise "WorkGraph retirement export escaped inventory" unless (exported - retired).empty?
    [seeds.sort.freeze, retired.sort.freeze, exported.sort.freeze]
  end

  def load_receipt(path)
    receipt = JSON.parse(File.binread(path))
    raise "invalid WorkGraph retirement receipt schema" unless receipt.fetch("schema") == SCHEMA
    raise "WorkGraph retirement receipt is not ready" unless receipt.fetch("status") == "ready"
    inventory(receipt)
    raise "WorkGraph retirement retained a live root" unless receipt.fetch(
      "first_party_rooted_closure"
    ).empty?
    receipt
  end

  def public_symbols(source)
    source.scan(PUBLIC_SYMBOL_PATTERN).flatten.to_set
  end

  def dependency_graph(entries)
    owners = Hash.new { |hash, key| hash[key] = Set.new }
    entries.each do |entry|
      owners[entry.name] << entry.name
      public_symbols(entry.source).each { |symbol| owners[symbol] << entry.name }
    end
    entries.to_h do |entry|
      targets = Set.new
      HeptaNormalizedTokenBundle.rust_tokens(entry.source).each do |token|
        owners[token.text].each { |owner| targets << owner } if token.kind == :identifier
      end
      targets.delete(entry.name)
      [entry.name, targets]
    end
  end

  def reverse_closure(graph, seeds)
    callers = Hash.new { |hash, key| hash[key] = Set.new }
    graph.each do |caller, targets|
      targets.each { |target| callers[target] << caller }
    end
    closure = seeds.to_set
    pending = seeds.dup
    until pending.empty?
      callers[pending.pop].each do |caller|
        pending << caller if closure.add?(caller)
      end
    end
    closure
  end

  def strongly_connected_components(graph, nodes)
    allowed = nodes.to_set
    index = 0
    stack = []
    on_stack = Set.new
    indices = {}
    low_links = {}
    components = []
    visit = lambda do |node|
      indices[node] = index
      low_links[node] = index
      index += 1
      stack << node
      on_stack << node
      graph.fetch(node).each do |target|
        next unless allowed.include?(target)
        unless indices.key?(target)
          visit.call(target)
          low_links[node] = [low_links[node], low_links[target]].min
          next
        end
        low_links[node] = [low_links[node], indices[target]].min if on_stack.include?(target)
      end
      return unless low_links[node] == indices[node]
      component = []
      loop do
        member = stack.pop
        on_stack.delete(member)
        component << member
        break if member == node
      end
      components << component.sort
    end
    allowed.sort.each { |node| visit.call(node) unless indices.key?(node) }
    components
  end

  def validate_baseline!(entries, receipt)
    seeds, retired, = inventory(receipt)
    by_name = entries.to_h { |entry| [entry.name, entry] }
    raise "WorkGraph retirement baseline inventory drifted" unless by_name.length == 284
    receipt.fetch("retired_modules").each do |record|
      entry = by_name.fetch(record.fetch("name"))
      raise "WorkGraph retirement source SHA drifted: #{entry.name}" unless Digest::SHA256.hexdigest(
        entry.source
      ) == record.fetch("source_sha256")
      raise "WorkGraph retirement source size drifted: #{entry.name}" unless entry.source.bytesize ==
        record.fetch("source_bytes")
      raise "WorkGraph retirement public symbol count drifted: #{entry.name}" unless public_symbols(
        entry.source
      ).length == record.fetch("public_symbol_count")
    end
    graph = dependency_graph(entries)
    closure = reverse_closure(graph, seeds)
    raise "WorkGraph retirement caller closure drifted" unless closure.to_a.sort == retired
    components = strongly_connected_components(graph, closure)
    proof = receipt.fetch("selection_proof")
    raise "WorkGraph retirement caller edge count drifted" unless proof.fetch("caller_edge_count") ==
      graph.values.sum(&:length)
    raise "WorkGraph retirement closure count drifted" unless proof.fetch("reverse_closure_count") ==
      closure.length
    raise "WorkGraph retirement SCC count drifted" unless proof.fetch("scc_count") == components.length
    raise "WorkGraph retirement SCC size drifted" unless proof.fetch("largest_scc_size") ==
      components.map(&:length).max
  end

  def active_entries(entries, receipt)
    validate_baseline!(entries, receipt)
    retired = inventory(receipt).fetch(1)
    entries.reject { |entry| retired.include?(entry.name) }
  end

  def prepare(entries, rows, receipt_path, root)
    receipt = load_receipt(receipt_path)
    active = active_entries(entries, receipt)
    retired = inventory(receipt).fetch(1)
    assert_no_live_references!(root, entries, receipt)
    [active, rows.reject { |row| retired.include?(row.fetch("name")) }, receipt]
  end

  def prune_export_blocks(relative_path, source, exported)
    return [source, 0] unless EXPORT_FILES.include?(relative_path)
    result = source.dup
    removed = 0
    exported.each do |module_name|
      pattern = /^pub use #{Regexp.escape(module_name)}::(?:\{\n.*?^\};|\*;)\n/m
      updated = result.sub(pattern, "")
      next if updated == result
      result = updated
      removed += 1
    end
    [result, removed]
  end

  def prune_runtime_sources(sources, receipt)
    exported = inventory(receipt).fetch(2)
    removed = 0
    pruned = sources.to_h do |relative_path, source|
      result, count = prune_export_blocks(relative_path, source, exported)
      removed += count
      [relative_path, result]
    end
    raise "WorkGraph retirement export closure drifted" unless removed == exported.length
    pruned
  end

  def verify_current!(entries, receipt_path, root, legacy_map)
    receipt = load_receipt(receipt_path)
    validate_baseline!(entries, receipt)
    retired = inventory(receipt).fetch(1)
    assert_no_live_references!(root, entries, receipt)
    raise "retired WorkGraph module remains in legacy map" unless (
      legacy_map.keys & retired
    ).empty?
    receipt
  end

  def assert_no_live_references!(root, entries, receipt)
    retired = inventory(receipt).fetch(1)
    retired_symbols = entries.select { |entry| retired.include?(entry.name) }
      .each_with_object(retired.to_set) do |entry, symbols|
        symbols.merge(public_symbols(entry.source))
      end
    hits = []
    source_paths = IO.popen(
      ["git", "-C", root, "ls-files", "-co", "--exclude-standard", "-z", "--", "codex-rs"],
      &:read
    ).split("\0").select { |path| path.end_with?(".rs") }
    source_paths.sort.each do |relative_path|
      next if EXPORT_FILES.include?(relative_path)
      source = File.binread(File.join(root, relative_path))
      identifiers = HeptaNormalizedTokenBundle.rust_tokens(source).map do |token|
        token.text if token.kind == :identifier
      end.compact.to_set
      referenced = identifiers & retired_symbols
      hits << [relative_path, referenced.to_a.sort] unless referenced.empty?
    end
    return if hits.empty?
    path, symbols = hits.first
    raise "WorkGraph retired live reference remains in #{path}: #{symbols.first(5).join(", ")}"
  end
end
