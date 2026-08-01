# frozen_string_literal: true

require "digest"
require "json"
require "stringio"
require "zlib"

# Shared, line-oriented canonical representation for the normalized WorkGraph
# and gate-compat token sources. The gzip bundle remains a build/runtime
# artifact; templates.jsonl and parameters.jsonl are the reviewable source.
module HeptaReadableTokenSource
  SCHEMA = "hepta_readable_normalized_token_source_v1"
  SCHEMA_FILE = "source.schema.json"
  TEMPLATE_FILE = "templates.jsonl"
  PARAMETER_FILE = "parameters.jsonl"
  FIXED_GZIP_MTIME = 1
  FIXED_GZIP_OS = 255

  module_function


  def write(
    root:,
    bundle:,
    artifact_schema:,
    domain:,
    artifact_file:,
    atomic_write:,
    template_fixed_chunk_bytes: nil,
    source_chunk_lines: nil,
    source_chunk_tokenizer: "shell_v1"
  )
    if source_chunk_lines
      return write_chunked(
        root: root,
        bundle: bundle,
        artifact_schema: artifact_schema,
        domain: domain,
        artifact_file: artifact_file,
        atomic_write: atomic_write,
        source_chunk_lines: source_chunk_lines,
        source_chunk_tokenizer: source_chunk_tokenizer,
        template_fixed_chunk_bytes: template_fixed_chunk_bytes
      )
    end
    template_records = bundle.families.each_with_index.flat_map do |family, template_id|
      template_records(
        family,
        template_id,
        fixed_chunk_bytes: template_fixed_chunk_bytes
      )
    end
    templates = template_records.map { |record| JSON.generate(record) }.join("\n") + "\n"
    parameters = bundle.entries.sort_by(&:name).map do |entry|
      family = bundle.families.fetch(entry.family_index)
      source = HeptaNormalizedTokenBundle.expand(family, entry)
      JSON.generate(
        name: entry.name,
        template_id: entry.family_index,
        source_bytes: source.bytesize,
        source_sha256: Digest::SHA256.hexdigest(source),
        replacements: entry.replacements.map { |replacement| utf8(replacement) }
      )
    end.join("\n") + "\n"
    atomic_write.call(File.join(root, TEMPLATE_FILE), templates, 0o644)
    atomic_write.call(File.join(root, PARAMETER_FILE), parameters, 0o644)
    metrics = metrics_for(bundle)
    entries = entries_for(bundle)
    template_max_record_bytes = templates.lines.map(&:bytesize).max || 0
    parameter_max_record_bytes = parameters.lines.map(&:bytesize).max || 0
    metadata = {
      schema: SCHEMA,
      domain: domain,
      artifact_schema: artifact_schema,
      canonical_files: [TEMPLATE_FILE, PARAMETER_FILE],
      generated_artifact: artifact_file,
      template_count: bundle.families.length,
      template_record_count: template_records.length,
      template_fixed_chunk_bytes: template_fixed_chunk_bytes,
      parameter_row_count: bundle.entries.length,
      source_bytes: entries.sum { |entry| entry.source.bytesize },
      source_lines: entries.sum { |entry| entry.source.lines.count },
      aggregate_source_sha256: HeptaNormalizedTokenBundle.aggregate_sha(entries),
      templates_sha256: Digest::SHA256.hexdigest(templates),
      parameters_sha256: Digest::SHA256.hexdigest(parameters),
      jsonl_record_byte_semantics: "record_including_lf",
      templates_max_record_bytes: template_max_record_bytes,
      parameters_max_record_bytes: parameter_max_record_bytes,
      canonical_max_record_bytes: [
        template_max_record_bytes,
        parameter_max_record_bytes
      ].max,
      normalized_effective_lines: metrics.fetch(:normalized_effective_lines),
      gzip_profile: {
        compression: "zlib_best_compression",
        mtime: FIXED_GZIP_MTIME,
        os: FIXED_GZIP_OS
      },
      source_is_canonical: true,
      artifact_is_generated: true,
      exact_reassembly: true
    }
    atomic_write.call(
      File.join(root, SCHEMA_FILE),
      JSON.pretty_generate(metadata) + "\n",
      0o644
    )
    metadata
  end


  def read(root:, artifact_schema:, domain:, name_validator:, strict_metadata:)
    metadata = JSON.parse(File.binread(File.join(root, SCHEMA_FILE)))
    raise "invalid readable token source schema" unless metadata["schema"] == SCHEMA
    raise "readable token source domain drifted" unless metadata["domain"] == domain
    raise "readable token artifact schema drifted" unless metadata["artifact_schema"] ==
      artifact_schema
    templates_body = File.binread(File.join(root, TEMPLATE_FILE))
    parameters_body = File.binread(File.join(root, PARAMETER_FILE))
    if metadata["canonical_layout"] == "chunked_source_sequence_v1"
      return read_chunked(
        metadata: metadata,
        templates_body: templates_body,
        parameters_body: parameters_body,
        name_validator: name_validator,
        strict_metadata: strict_metadata
      )
    end
    families = parse_templates(templates_body)
    encoded_entries = parse_parameters(
      parameters_body,
      families,
      name_validator,
      strict_metadata
    )
    bundle = HeptaNormalizedTokenBundle::Bundle.new(
      families,
      encoded_entries,
      metrics_for_parts(families, encoded_entries)
    )
    entries = entries_for(bundle)
    if strict_metadata
      expected = {
        "canonical_files" => [TEMPLATE_FILE, PARAMETER_FILE],
        "template_count" => families.length,
        "parameter_row_count" => encoded_entries.length,
        "source_bytes" => entries.sum { |entry| entry.source.bytesize },
        "source_lines" => entries.sum { |entry| entry.source.lines.count },
        "aggregate_source_sha256" => HeptaNormalizedTokenBundle.aggregate_sha(entries),
        "templates_sha256" => Digest::SHA256.hexdigest(templates_body),
        "parameters_sha256" => Digest::SHA256.hexdigest(parameters_body),
        "jsonl_record_byte_semantics" => "record_including_lf",
        "templates_max_record_bytes" =>
          (templates_body.lines.map(&:bytesize).max || 0),
        "parameters_max_record_bytes" =>
          (parameters_body.lines.map(&:bytesize).max || 0),
        "canonical_max_record_bytes" => [
          templates_body.lines.map(&:bytesize).max || 0,
          parameters_body.lines.map(&:bytesize).max || 0
        ].max,
        "normalized_effective_lines" => bundle.metrics.fetch(:normalized_effective_lines),
        "source_is_canonical" => true,
        "artifact_is_generated" => true,
        "exact_reassembly" => true
      }
      if metadata.key?("template_record_count")
        expected["template_record_count"] = templates_body.lines.length
      end
      if metadata.key?("template_fixed_chunk_bytes")
        expected["template_fixed_chunk_bytes"] = metadata["template_fixed_chunk_bytes"]
      end
      expected.each do |key, value|
        raise "readable token source metadata drifted: #{key}" unless metadata[key] == value
      end
      gzip_profile = metadata.fetch("gzip_profile")
      raise "readable token gzip profile drifted" unless gzip_profile == {
        "compression" => "zlib_best_compression",
        "mtime" => FIXED_GZIP_MTIME,
        "os" => FIXED_GZIP_OS
      }
    end
    [entries, bundle, metadata]
  end


  def present?(root)
    [SCHEMA_FILE, TEMPLATE_FILE, PARAMETER_FILE].all? do |name|
      File.file?(File.join(root, name))
    end
  end

  def entries_for(bundle)
    bundle.entries.sort_by(&:name).map do |entry|
      HeptaNormalizedTokenBundle::Entry.new(
        entry.name,
        HeptaNormalizedTokenBundle.expand(bundle.families.fetch(entry.family_index), entry)
      )
    end
  end

  def metrics_for(bundle)
    metrics_for_parts(bundle.families, bundle.entries)
  end

  def metrics_for_parts(families, entries)
    fixed_breaks = families.sum do |family|
      family.segments.sum { |segment| segment.kind == :fixed ? segment.text.count("\n") : 0 }
    end
    replacement_breaks = entries.sum do |entry|
      entry.replacements.sum { |replacement| replacement.count("\n") }
    end
    {
      normalized_template_count: families.length,
      normalized_parameter_row_count: entries.length,
      normalized_effective_lines:
        fixed_breaks + replacement_breaks + families.length + entries.length
    }
  end

  def parse_parameters(body, families, name_validator, strict_metadata)
    names = {}
    entries = json_lines(body, PARAMETER_FILE).map do |record|
      name = record.fetch("name")
      raise "invalid readable token parameter name: #{name}" unless name_validator.call(name)
      raise "duplicate readable token parameter name: #{name}" if names[name]
      names[name] = true
      family_index = record.fetch("template_id")
      family = families.fetch(family_index)
      replacements = record.fetch("replacements").map(&:b)
      raise "readable token replacement count drifted: #{name}" unless replacements.length ==
        family.slot_count
      provisional = HeptaNormalizedTokenBundle::EncodedEntry.new(
        name,
        family_index,
        "",
        0,
        replacements
      )
      source = HeptaNormalizedTokenBundle.expand(family, provisional)
      source_sha = Digest::SHA256.hexdigest(source)
      source_size = source.bytesize
      if strict_metadata
        raise "readable token parameter source SHA drifted: #{name}" unless record.fetch(
          "source_sha256"
        ) == source_sha
        raise "readable token parameter source size drifted: #{name}" unless record.fetch(
          "source_bytes"
        ) == source_size
      end
      HeptaNormalizedTokenBundle::EncodedEntry.new(
        name,
        family_index,
        source_sha,
        source_size,
        replacements
      )
    end
    raise "readable token parameter order drifted" unless entries.map(&:name) ==
      entries.map(&:name).sort
    entries
  end


  def json_lines(body, name)
    lines = body.lines(chomp: true)
    raise "empty readable token source: #{name}" if lines.empty?
    lines.map { |line| JSON.parse(line) }
  end

  def utf8(bytes)
    value = bytes.dup.force_encoding(Encoding::UTF_8)
    raise "readable token source contains non-UTF-8 bytes" unless value.valid_encoding?
    value
  end
end

require_relative "lib/hepta-readable-token-template-v1"
require_relative "lib/hepta-chunked-readable-token-source-v1"
