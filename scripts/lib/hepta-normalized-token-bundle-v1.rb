# frozen_string_literal: true

require "digest"
require "stringio"

module HeptaNormalizedTokenBundle
  Token = Struct.new(:kind, :text)
  Entry = Struct.new(:name, :source)
  Segment = Struct.new(:kind, :text)
  Family = Struct.new(:segments, :slot_count)
  EncodedEntry = Struct.new(
    :name,
    :family_index,
    :source_sha256,
    :source_size,
    :replacements
  )
  Bundle = Struct.new(:families, :entries, :metrics)

  RUST_KEYWORDS = %w[
    as async await break const continue crate dyn else enum extern false fn for if impl in let loop
    match mod move mut pub ref return self Self static struct super trait true type union unsafe use
    where while
  ].freeze
  SHELL_KEYWORDS = %w[
    break case continue coproc declare do done echo elif else esac eval exec exit export false fi
    for function getopts if in local mapfile printf pwd read readonly return select set shift source
    test then time trap true typeset unset until wait while
  ].freeze

  module_function

  def rust_tokens(source)
    tokens = []
    index = 0
    while index < source.bytesize
      byte = source.getbyte(index)
      if whitespace_byte?(byte)
        finish = index + 1
        finish += 1 while whitespace_byte?(source.getbyte(finish))
        tokens << Token.new(:space, source.byteslice(index...finish))
        index = finish
      elsif source.byteslice(index, 2) == "//"
        finish = source.index("\n", index) || source.bytesize
        finish += 1 if finish < source.bytesize
        tokens << Token.new(:comment, source.byteslice(index...finish))
        index = finish
      elsif source.byteslice(index, 2) == "/*"
        finish = source.index("*/", index + 2)
        raise "unterminated Rust block comment" unless finish
        finish += 2
        tokens << Token.new(:comment, source.byteslice(index...finish))
        index = finish
      elsif (raw = rust_raw_string_at(source, index))
        tokens << Token.new(:literal, raw)
        index += raw.bytesize
      elsif source.byteslice(index, 2) == 'b"' || byte == 34
        finish = quoted_end(source, index + (byte == 98 ? 1 : 0), 34)
        tokens << Token.new(:literal, source.byteslice(index...finish))
        index = finish
      elsif source.byteslice(index, 2) == "b'" || byte == 39
        literal_index = byte == 98 ? index + 1 : index
        finish = rust_lifetime_or_char_end(source, literal_index)
        kind = source.getbyte(finish - 1) == 39 ? :literal : :identifier
        tokens << Token.new(kind, source.byteslice(index...finish))
        index = finish
      elsif source.byteslice(index, 2) == "r#" && identifier_start?(source.getbyte(index + 2))
        finish = index + 3
        finish += 1 while identifier_continue?(source.getbyte(finish))
        tokens << Token.new(:identifier, source.byteslice(index...finish))
        index = finish
      elsif identifier_start?(byte)
        finish = index + 1
        finish += 1 while identifier_continue?(source.getbyte(finish))
        text = source.byteslice(index...finish)
        tokens << Token.new(RUST_KEYWORDS.include?(text) ? :fixed : :identifier, text)
        index = finish
      elsif byte&.between?(48, 57)
        finish = index + 1
        finish += 1 while source.getbyte(finish)&.chr&.match?(/[A-Za-z0-9_.]/)
        tokens << Token.new(:number, source.byteslice(index...finish))
        index = finish
      else
        tokens << Token.new(:fixed, source.byteslice(index, 1))
        index += 1
      end
    end
    tokens
  end

  def shell_tokens(source)
    tokens = []
    index = 0
    while index < source.bytesize
      byte = source.getbyte(index)
      if whitespace_byte?(byte)
        finish = index + 1
        finish += 1 while whitespace_byte?(source.getbyte(finish))
        tokens << Token.new(:space, source.byteslice(index...finish))
        index = finish
      elsif [34, 39, 96].include?(byte)
        finish = source.index(byte.chr, index + 1)
        if finish
          tokens << Token.new(:literal, source.byteslice(index..finish))
          index = finish + 1
        else
          tokens << Token.new(:fixed, source.byteslice(index, 1))
          index += 1
        end
      elsif identifier_start?(byte)
        finish = index + 1
        finish += 1 while identifier_continue?(source.getbyte(finish))
        text = source.byteslice(index...finish)
        tokens << Token.new(SHELL_KEYWORDS.include?(text) ? :fixed : :identifier, text)
        index = finish
      elsif byte&.between?(48, 57)
        finish = index + 1
        while source.getbyte(finish)&.then { |candidate| candidate.between?(48, 57) || [46, 95].include?(candidate) }
          finish += 1
        end
        tokens << Token.new(:number, source.byteslice(index...finish))
        index = finish
      else
        tokens << Token.new(:fixed, source.byteslice(index, 1))
        index += 1
      end
    end
    tokens
  end

  def build(entries, tokenizer:)
    raise "normalized token bundle must not be empty" if entries.empty?
    raise "duplicate normalized token bundle entry" unless entries.map(&:name).uniq.length == entries.length
    token_sets = entries.to_h { |entry| [entry.name, tokenizer.call(entry.source)] }
    grouped = entries.group_by { |entry| signature(token_sets.fetch(entry.name)) }
      .values
      .sort_by { |group| group.first.name }
    families = []
    encoded_entries = []
    fixed_line_breaks = 0
    replacement_line_breaks = 0
    grouped.each_with_index do |group, family_index|
      group_token_sets = group.map { |entry| token_sets.fetch(entry.name) }
      slot_positions = []
      segments = []
      group_token_sets.first.each_index do |token_index|
        values = group_token_sets.map { |tokens| tokens.fetch(token_index).text }
        token = group_token_sets.first.fetch(token_index)
        if token.kind == :fixed || values.uniq.one?
          append_fixed_segment(segments, values.first)
          fixed_line_breaks += values.first.count("\n")
        else
          segments << Segment.new(:slot, nil)
          slot_positions << token_index
        end
      end
      family = Family.new(segments, slot_positions.length)
      families << family
      group.each_with_index do |entry, entry_index|
        replacements = slot_positions.map do |token_index|
          value = group_token_sets.fetch(entry_index).fetch(token_index).text
          replacement_line_breaks += value.count("\n")
          value
        end
        encoded = EncodedEntry.new(
          entry.name,
          family_index,
          Digest::SHA256.hexdigest(entry.source),
          entry.source.bytesize,
          replacements
        )
        raise "normalized token reconstruction drifted: #{entry.name}" unless expand(family, encoded) == entry.source
        encoded_entries << encoded
      end
    end
    encoded_entries.sort_by!(&:name)
    normalized_effective_lines =
      fixed_line_breaks + replacement_line_breaks + families.length + encoded_entries.length
    Bundle.new(
      families,
      encoded_entries,
      {
        normalized_template_count: families.length,
        normalized_parameter_row_count: encoded_entries.length,
        normalized_effective_lines: normalized_effective_lines,
        normalized_fixed_line_breaks: fixed_line_breaks,
        normalized_replacement_line_breaks: replacement_line_breaks
      }
    )
  end

  def encode(bundle, schema:)
    payload = "#{schema}\0".b
    payload << u32(bundle.families.length)
    bundle.families.each do |family|
      payload << u32(family.segments.length)
      payload << u32(family.slot_count)
      family.segments.each do |segment|
        if segment.kind == :fixed
          payload << [0].pack("C") << u32(segment.text.bytesize) << segment.text.b
        else
          payload << [1].pack("C")
        end
      end
    end
    payload << u32(bundle.entries.length)
    bundle.entries.each do |entry|
      payload << u32(entry.name.bytesize)
      payload << u32(entry.family_index)
      payload << u32(entry.replacements.length)
      payload << u32(entry.source_size)
      payload << entry.name.b
      payload << [entry.source_sha256].pack("H*")
      entry.replacements.each do |replacement|
        payload << u32(replacement.bytesize) << replacement.b
      end
    end
    payload
  end

  def decode(payload, schema:, name_validator:)
    cursor = StringIO.new(payload.b)
    expected_magic = "#{schema}\0".b
    raise "invalid normalized token bundle magic" unless read_exact(cursor, expected_magic.bytesize) == expected_magic
    family_count = read_u32(cursor)
    families = Array.new(family_count) do
      segment_count = read_u32(cursor)
      slot_count = read_u32(cursor)
      actual_slots = 0
      segments = Array.new(segment_count) do
        case read_exact(cursor, 1).unpack1("C")
        when 0
          Segment.new(:fixed, read_exact(cursor, read_u32(cursor)))
        when 1
          actual_slots += 1
          Segment.new(:slot, nil)
        else
          raise "invalid normalized token segment kind"
        end
      end
      raise "normalized token slot count drifted" unless slot_count == actual_slots
      Family.new(segments, slot_count)
    end
    entry_count = read_u32(cursor)
    names = {}
    entries = Array.new(entry_count) do
      name_length = read_u32(cursor)
      family_index = read_u32(cursor)
      replacement_count = read_u32(cursor)
      source_size = read_u32(cursor)
      name = read_exact(cursor, name_length).force_encoding(Encoding::UTF_8)
      raise "invalid normalized token entry name: #{name}" unless name_validator.call(name)
      raise "duplicate normalized token entry name: #{name}" if names[name]
      names[name] = true
      source_sha256 = read_exact(cursor, 32).unpack1("H*")
      family = families.fetch(family_index)
      raise "normalized token replacement count drifted: #{name}" unless replacement_count == family.slot_count
      replacements = Array.new(replacement_count) do
        read_exact(cursor, read_u32(cursor))
      end
      encoded = EncodedEntry.new(
        name,
        family_index,
        source_sha256,
        source_size,
        replacements
      )
      source = expand(family, encoded)
      raise "normalized token source size drifted: #{name}" unless source.bytesize == source_size
      raise "normalized token source SHA drifted: #{name}" unless Digest::SHA256.hexdigest(source) == source_sha256
      Entry.new(name, source)
    end
    raise "trailing normalized token bundle bytes" unless cursor.eof?
    entries.sort_by(&:name)
  end

  def aggregate_sha(entries)
    digest = Digest::SHA256.new
    entries.sort_by(&:name).each do |entry|
      digest << entry.name << "\0" << Digest::SHA256.hexdigest(entry.source) << "\n"
    end
    digest.hexdigest
  end

  def expand(family, entry)
    replacement_index = 0
    source = family.segments.each_with_object(+"".b) do |segment, output|
      if segment.kind == :fixed
        output << segment.text
      else
        output << entry.replacements.fetch(replacement_index)
        replacement_index += 1
      end
    end
    raise "normalized token replacement vector was not fully consumed" unless replacement_index == entry.replacements.length
    source
  end

  def signature(tokens)
    tokens.map do |token|
      token.kind == :fixed ? "fixed:#{token.text}" : token.kind.to_s
    end.join("\0")
  end

  def append_fixed_segment(segments, text)
    if segments.last&.kind == :fixed
      segments.last.text << text
    else
      segments << Segment.new(:fixed, text.dup)
    end
  end

  def rust_raw_string_at(source, index)
    prefix = if source.byteslice(index, 2) == "br"
               "br"
             elsif source.getbyte(index) == 114
               "r"
             end
    return nil unless prefix
    cursor = index + prefix.bytesize
    hashes = 0
    while source.getbyte(cursor) == 35
      hashes += 1
      cursor += 1
    end
    return nil unless source.getbyte(cursor) == 34
    terminator = '"' + ("#" * hashes)
    finish = source.index(terminator, cursor + 1)
    raise "unterminated Rust raw string" unless finish
    source.byteslice(index...(finish + terminator.bytesize))
  end

  def quoted_end(source, quote_index, quote)
    cursor = quote_index + 1
    escaped = false
    while cursor < source.bytesize
      byte = source.getbyte(cursor)
      if escaped
        escaped = false
      elsif byte == 92
        escaped = true
      elsif byte == quote
        return cursor + 1
      end
      cursor += 1
    end
    raise "unterminated quoted literal"
  end

  def rust_lifetime_or_char_end(source, index)
    cursor = index + 1
    if identifier_start?(source.getbyte(cursor))
      cursor += 1 while identifier_continue?(source.getbyte(cursor))
      return cursor unless source.getbyte(cursor) == 39
    end
    quoted_end(source, index, 39)
  end

  def identifier_start?(byte)
    byte == 95 || byte&.between?(65, 90) || byte&.between?(97, 122)
  end

  def identifier_continue?(byte)
    identifier_start?(byte) || byte&.between?(48, 57)
  end

  def whitespace_byte?(byte)
    [9, 10, 13, 32].include?(byte)
  end

  def u32(value)
    raise "normalized token bundle integer overflow" unless value.between?(0, 0xffff_ffff)
    [value].pack("N")
  end

  def read_u32(cursor)
    read_exact(cursor, 4).unpack1("N")
  end

  def read_exact(cursor, length)
    value = cursor.read(length)
    raise "truncated normalized token bundle" unless value&.bytesize == length
    value
  end
end
