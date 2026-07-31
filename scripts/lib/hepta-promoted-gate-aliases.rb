# frozen_string_literal: true

require "digest"
require "fileutils"
require "json"

module HeptaPromotedGateAliases
  CONTEXT_SPEC = "scripts/hepta-context-gate-specs-v1.json"
  WORKGRAPH_SPEC = "scripts/hepta-workgraph-source-report-specs-v1.json"
  ALIAS_PREFIX = "scripts/lib/hepta-gate-pair-compat-v1/"

  module_function

  def input_sha256s(root)
    [__FILE__, CONTEXT_SPEC, WORKGRAPH_SPEC].map do |path|
      absolute = path == __FILE__ ? path : File.join(root, path)
      Digest::SHA256.file(absolute).hexdigest
    end
  end

  def entries(root)
    @entries_by_root ||= {}
    @entries_by_root[root] ||= begin
      context = JSON.parse(File.binread(File.join(root, CONTEXT_SPEC)))
      workgraph = JSON.parse(File.binread(File.join(root, WORKGRAPH_SPEC)))
      raise "invalid context gate promotion spec" unless context["schema_version"] ==
        "hepta_context_gate_specs_v1" && context["entry_count"] == context.fetch("entries").length
      raise "invalid WorkGraph gate promotion spec" unless workgraph["schema_version"] ==
        "hepta_workgraph_source_report_specs_v1" &&
        workgraph["entry_count"] == workgraph.fetch("entries").length

      aliases = context.fetch("entries").flat_map do |entry|
        [entry.fetch("gate"), entry.fetch("report")].map do |payload|
          payload_entry(payload.fetch("path"), payload.fetch("compatibility_alias"),
                        payload.fetch("sha256"), payload.fetch("lines"))
        end
      end
      aliases.concat(workgraph.fetch("entries").map do |entry|
        payload_entry(entry.fetch("report_path"), entry.fetch("compatibility_alias"),
                      entry.fetch("report_sha256"), entry.fetch("report_lines"))
      end)
      raise "duplicate promoted gate alias" unless aliases.map { |entry| entry.fetch(:alias) }.uniq.length ==
        aliases.length
      aliases.each { |entry| validate_entry(root, entry) }
      aliases.freeze
    end
  end

  def materialized_payload_store?(root)
    store = File.join(root, ALIAS_PREFIX)
    Dir.exist?(store) && Dir.children(store).any? do |name|
      !File.symlink?(File.join(store, name))
    end
  end

  def verify_source_aliases(root)
    expected = entries(root)
      .select { |entry| entry.fetch(:alias).bytesize <= 240 }
      .to_h { |entry| [entry.fetch(:alias), entry.fetch(:source)] }
    store = File.join(root, ALIAS_PREFIX)
    actual = Dir.exist?(store) ? Dir.children(store).map { |name| "#{ALIAS_PREFIX}#{name}" }.sort : []
    raise "promoted gate alias path-set drifted" unless actual == expected.keys.sort
    expected.each do |alias_path, source_path|
      absolute_alias = File.join(root, alias_path)
      raise "promoted gate alias is not a symlink: #{alias_path}" unless File.symlink?(absolute_alias)
      raise "promoted gate alias target drifted: #{alias_path}" unless File.realpath(absolute_alias) ==
        File.realpath(File.join(root, source_path))
    end
    public_entries(root).each do |entry|
      alias_path = File.join(root, entry.fetch(:alias))
      if entry.fetch(:alias).bytesize > 240
        raise "long promoted gate alias was materialized: #{entry.fetch(:alias)}" if File.exist?(alias_path) ||
          File.symlink?(alias_path)
        next
      end
      raise "promoted public gate alias is not a symlink: #{entry.fetch(:alias)}" unless File.symlink?(alias_path)
      raise "promoted public gate alias target drifted: #{entry.fetch(:alias)}" unless File.realpath(alias_path) ==
        File.realpath(File.join(root, entry.fetch(:source)))
    end
  end

  def materialize(root, staging)
    (entries(root) + public_entries(root)).each do |entry|
      target = File.join(root, entry.fetch(:source))
      alias_path = File.join(staging, entry.fetch(:alias))
      FileUtils.mkdir_p(File.dirname(alias_path))
      if File.exist?(alias_path) || File.symlink?(alias_path)
        raise "promoted gate overlay alias collision: #{entry.fetch(:alias)}" unless File.realpath(alias_path) ==
          File.realpath(target)
        next
      end
      File.symlink(target, alias_path)
    end
  end

  def public_entries(root)
    @public_entries_by_root ||= {}
    @public_entries_by_root[root] ||= begin
      context = JSON.parse(File.binread(File.join(root, CONTEXT_SPEC))).fetch("entries").flat_map do |entry|
        %w[gate report].map do |kind|
          {
            source: "scripts/hepta-context-gate-launch",
            alias: "scripts/#{entry.fetch("id")}-#{kind}.sh"
          }
        end
      end
      workgraph = JSON.parse(File.binread(File.join(root, WORKGRAPH_SPEC))).fetch("entries").flat_map do |entry|
        %w[gate report].map do |kind|
          {
            source: "scripts/hepta-workgraph-source-report-smoke-launch",
            alias: "scripts/#{entry.fetch("id")}-#{kind}.sh"
          }
        end
      end
      (context + workgraph).freeze
    end
  end

  def payload_entry(source, alias_path, sha256, lines)
    raise "invalid promoted gate source path: #{source}" if source.start_with?("/") ||
      source.split("/").include?("..")
    raise "invalid promoted gate compatibility alias: #{alias_path}" unless alias_path.start_with?(
      ALIAS_PREFIX
    ) && alias_path.match?(%r{\A#{Regexp.escape(ALIAS_PREFIX)}[a-z0-9-]+\.(?:gate|report)\z})
    {source: source, alias: alias_path, sha256: sha256, lines: lines}
  end
  private_class_method :payload_entry

  def validate_entry(root, entry)
    source = File.join(root, entry.fetch(:source))
    raise "missing promoted gate source: #{entry.fetch(:source)}" unless File.file?(source)
    raise "promoted gate source SHA drifted: #{entry.fetch(:source)}" unless Digest::SHA256.file(
      source
    ).hexdigest == entry.fetch(:sha256)
    raise "promoted gate source line count drifted: #{entry.fetch(:source)}" unless File.binread(
      source
    ).lines.length == entry.fetch(:lines)
  end
  private_class_method :validate_entry
end
