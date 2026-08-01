# frozen_string_literal: true

module HeptaReadableTokenSource
  module_function
  def deterministic_gzip(payload)
    buffer = StringIO.new(+"".b)
    writer = Zlib::GzipWriter.new(buffer, Zlib::BEST_COMPRESSION)
    writer.mtime = FIXED_GZIP_MTIME
    writer.write(payload)
    writer.close
    gzip = buffer.string
    gzip[4, 4] = [FIXED_GZIP_MTIME].pack("V")
    gzip.setbyte(9, FIXED_GZIP_OS)
    gzip
  end
  def write_chunked(
    root:,
    bundle:,
    artifact_schema:,
    domain:,
    artifact_file:,
    atomic_write:,
    source_chunk_lines:,
    source_chunk_tokenizer:,
    template_fixed_chunk_bytes:
  )
    raise "invalid readable token source chunk size" unless source_chunk_lines.positive?
    entries = entries_for(bundle)
    block_names_by_source = {}
    block_entries = entries.flat_map do |entry|
      blocks = source_chunks(entry.source, source_chunk_lines).each_with_index.map do |source, index|
        block_name = "#{entry.name}##{index.to_s.rjust(6, '0')}"
        HeptaNormalizedTokenBundle::Entry.new(block_name, source)
      end
      block_names_by_source[entry.name] = blocks.map(&:name)
      blocks
    end
    block_bundle = HeptaNormalizedTokenBundle.build(
      block_entries,
      tokenizer: tokenizer_for(source_chunk_tokenizer)
    )
    template_records = block_bundle.families.each_with_index.flat_map do |family, template_id|
      template_records(
        family,
        template_id,
        fixed_chunk_bytes: template_fixed_chunk_bytes
      )
    end
    templates = template_records.map { |record| JSON.generate(record) }.join("\n") + "\n"
    encoded_blocks = block_bundle.entries.to_h { |entry| [entry.name, entry] }
    parameter_records = entries.sort_by(&:name).flat_map do |entry|
      blocks = block_names_by_source.fetch(entry.name).map do |block_name|
        block = encoded_blocks.fetch(block_name)
        family = block_bundle.families.fetch(block.family_index)
        source = HeptaNormalizedTokenBundle.expand(family, block)
        {
          record: "block",
          name: entry.name,
          index: block_name.split("#").last.to_i,
          template_id: block.family_index,
          source_bytes: source.bytesize,
          source_sha256: Digest::SHA256.hexdigest(source),
          replacements: block.replacements.map { |replacement| utf8(replacement) }
        }
      end
      [{
        record: "source",
        name: entry.name,
        source_bytes: entry.source.bytesize,
        source_sha256: Digest::SHA256.hexdigest(entry.source),
        block_count: blocks.length
      }, *blocks]
    end
    parameters = parameter_records.map { |record| JSON.generate(record) }.join("\n") + "\n"
    atomic_write.call(File.join(root, TEMPLATE_FILE), templates, 0o644)
    atomic_write.call(File.join(root, PARAMETER_FILE), parameters, 0o644)
    block_metrics = metrics_for(block_bundle)
    normalized_effective_lines = block_metrics.fetch(:normalized_effective_lines) + entries.length
    template_max_record_bytes = templates.lines.map(&:bytesize).max || 0
    parameter_max_record_bytes = parameters.lines.map(&:bytesize).max || 0
    metadata = {
      schema: SCHEMA,
      domain: domain,
      artifact_schema: artifact_schema,
      canonical_layout: "chunked_source_sequence_v1",
      canonical_files: [TEMPLATE_FILE, PARAMETER_FILE],
      generated_artifact: artifact_file,
      tokenizer: source_chunk_tokenizer,
      source_chunk_lines: source_chunk_lines,
      source_chunk_boundary: "blank_line_preferred_v1",
      template_count: block_bundle.families.length,
      template_record_count: template_records.length,
      template_fixed_chunk_bytes: template_fixed_chunk_bytes,
      parameter_row_count: block_bundle.entries.length,
      parameter_record_count: parameter_records.length,
      source_sequence_count: entries.length,
      source_bytes: entries.sum { |entry| entry.source.bytesize },
      source_lines: entries.sum { |entry| entry.source.lines.count },
      aggregate_source_sha256: HeptaNormalizedTokenBundle.aggregate_sha(entries),
      templates_sha256: Digest::SHA256.hexdigest(templates),
      parameters_sha256: Digest::SHA256.hexdigest(parameters),
      jsonl_record_byte_semantics: "record_including_lf",
      templates_max_record_bytes: template_max_record_bytes,
      parameters_max_record_bytes: parameter_max_record_bytes,
      canonical_max_record_bytes: [template_max_record_bytes, parameter_max_record_bytes].max,
      normalized_effective_lines: normalized_effective_lines,
      gzip_profile: {
        compression: "zlib_best_compression",
        mtime: FIXED_GZIP_MTIME,
        os: FIXED_GZIP_OS
      },
      source_is_canonical: true,
      artifact_is_generated: true,
      exact_reassembly: true,
      block_instances_are_counted: true,
      source_sequences_are_counted: true
    }
    atomic_write.call(
      File.join(root, SCHEMA_FILE),
      JSON.pretty_generate(metadata) + "\n",
      0o644
    )
    metadata
  end
  def read_chunked(
    metadata:,
    templates_body:,
    parameters_body:,
    name_validator:,
    strict_metadata:
  )
    tokenizer = tokenizer_for(metadata.fetch("tokenizer"))
    families = parse_templates(templates_body)
    entries, block_entries = parse_chunked_parameters(
      parameters_body,
      families,
      name_validator,
      strict_metadata
    )
    rebuilt = HeptaNormalizedTokenBundle.build(
      entries,
      tokenizer: tokenizer
    )
    block_metrics = metrics_for_parts(families, block_entries)
    normalized_effective_lines = block_metrics.fetch(:normalized_effective_lines) + entries.length
    bundle = HeptaNormalizedTokenBundle::Bundle.new(
      rebuilt.families,
      rebuilt.entries,
      rebuilt.metrics.merge(normalized_effective_lines: normalized_effective_lines)
    )
    if strict_metadata
      expected = {
        "canonical_files" => [TEMPLATE_FILE, PARAMETER_FILE],
        "canonical_layout" => "chunked_source_sequence_v1",
        "tokenizer" => metadata.fetch("tokenizer"),
        "source_chunk_boundary" => "blank_line_preferred_v1",
        "template_count" => families.length,
        "template_record_count" => templates_body.lines.length,
        "template_fixed_chunk_bytes" => metadata.fetch("template_fixed_chunk_bytes"),
        "parameter_row_count" => block_entries.length,
        "parameter_record_count" => parameters_body.lines.length,
        "source_sequence_count" => entries.length,
        "source_bytes" => entries.sum { |entry| entry.source.bytesize },
        "source_lines" => entries.sum { |entry| entry.source.lines.count },
        "aggregate_source_sha256" => HeptaNormalizedTokenBundle.aggregate_sha(entries),
        "templates_sha256" => Digest::SHA256.hexdigest(templates_body),
        "parameters_sha256" => Digest::SHA256.hexdigest(parameters_body),
        "jsonl_record_byte_semantics" => "record_including_lf",
        "templates_max_record_bytes" => templates_body.lines.map(&:bytesize).max || 0,
        "parameters_max_record_bytes" => parameters_body.lines.map(&:bytesize).max || 0,
        "canonical_max_record_bytes" => [
          templates_body.lines.map(&:bytesize).max || 0,
          parameters_body.lines.map(&:bytesize).max || 0
        ].max,
        "normalized_effective_lines" => normalized_effective_lines,
        "source_is_canonical" => true,
        "artifact_is_generated" => true,
        "exact_reassembly" => true,
        "block_instances_are_counted" => true,
        "source_sequences_are_counted" => true
      }
      expected.each do |key, value|
        raise "readable token source metadata drifted: #{key}" unless metadata[key] == value
      end
      raise "invalid readable token source chunk size" unless metadata.fetch(
        "source_chunk_lines"
      ).positive?
      gzip_profile = metadata.fetch("gzip_profile")
      raise "readable token gzip profile drifted" unless gzip_profile == {
        "compression" => "zlib_best_compression",
        "mtime" => FIXED_GZIP_MTIME,
        "os" => FIXED_GZIP_OS
      }
    end
    [entries, bundle, metadata]
  end

  def source_chunks(source, target_lines)
    paragraphs = []
    paragraph = []
    source.lines.each do |line|
      paragraph << line
      next unless line.strip.empty?
      paragraphs << paragraph
      paragraph = []
    end
    paragraphs << paragraph unless paragraph.empty?
    chunks = []
    current = []
    paragraphs.each do |candidate|
      if !current.empty? && current.length + candidate.length > target_lines
        chunks << current.join
        current = []
      end
      if candidate.length > target_lines
        chunks.concat(candidate.each_slice(target_lines).map(&:join))
      else
        current.concat(candidate)
      end
    end
    chunks << current.join unless current.empty?
    chunks
  end

  def tokenizer_for(name)
    case name
    when "shell_v1"
      HeptaNormalizedTokenBundle.method(:shell_tokens)
    when "rust_v1"
      HeptaNormalizedTokenBundle.method(:rust_tokens)
    else
      raise "unknown readable token chunk tokenizer: #{name}"
    end
  end
  def parse_chunked_parameters(body, families, name_validator, strict_metadata)
    names = {}
    block_entries = []
    records = json_lines(body, PARAMETER_FILE)
    source_records = records.select { |record| record["record"] == "source" }
    block_records = records.select { |record| record["record"] == "block" }.group_by do |record|
      record.fetch("name")
    end
    raise "invalid readable token chunk record" unless records.length ==
      source_records.length + block_records.values.sum(&:length)
    entries = source_records.map do |source_record|
      name = source_record.fetch("name")
      raise "invalid readable token parameter name: #{name}" unless name_validator.call(name)
      raise "duplicate readable token parameter name: #{name}" if names[name]
      names[name] = true
      source = +"".b
      blocks = block_records.fetch(name, []).sort_by { |block| block.fetch("index") }
      raise "readable token source block count drifted: #{name}" unless blocks.length ==
        source_record.fetch("block_count")
      blocks.each_with_index do |block, index|
        raise "readable token source block order drifted: #{name}" unless block.fetch("index") ==
          index
        family_index = block.fetch("template_id")
        family = families.fetch(family_index)
        replacements = block.fetch("replacements").map(&:b)
        raise "readable token block replacement count drifted: #{name}" unless \
          replacements.length == family.slot_count
        encoded = HeptaNormalizedTokenBundle::EncodedEntry.new(
          "#{name}##{index.to_s.rjust(6, '0')}",
          family_index,
          block.fetch("source_sha256"),
          block.fetch("source_bytes"),
          replacements
        )
        block_source = HeptaNormalizedTokenBundle.expand(family, encoded)
        if strict_metadata
          raise "readable token block source SHA drifted: #{name}" unless \
            Digest::SHA256.hexdigest(block_source) == encoded.source_sha256
          raise "readable token block source size drifted: #{name}" unless \
            block_source.bytesize == encoded.source_size
        end
        block_entries << encoded
        source << block_source
      end
      if strict_metadata
        raise "readable token parameter source SHA drifted: #{name}" unless \
          Digest::SHA256.hexdigest(source) == source_record.fetch("source_sha256")
        raise "readable token parameter source size drifted: #{name}" unless \
          source.bytesize == source_record.fetch("source_bytes")
      end
      HeptaNormalizedTokenBundle::Entry.new(name, source)
    end
    raise "readable token parameter order drifted" unless entries.map(&:name) ==
      entries.map(&:name).sort
    [entries, block_entries]
  end
end
