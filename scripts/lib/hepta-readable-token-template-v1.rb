# frozen_string_literal: true

module HeptaReadableTokenSource
  module_function

  def parse_templates(body)
    records = json_lines(body, TEMPLATE_FILE)
    return parse_legacy_templates(records) unless records.first.key?("chunk_index")
    raise "mixed readable token template layouts" unless records.all? do |record|
      record.key?("chunk_index") && record.key?("chunk_count")
    end
    grouped = records.group_by { |record| record.fetch("template_id") }
    raise "readable token template order drifted" unless grouped.keys.sort ==
      (0...grouped.length).to_a
    grouped.keys.sort.map do |template_id|
      chunks = grouped.fetch(template_id)
      chunk_count = chunks.first.fetch("chunk_count")
      slot_count = chunks.first.fetch("slot_count")
      raise "readable token template chunk count drifted" unless chunks.length == chunk_count
      raise "readable token template chunk order drifted" unless chunks.map do |record|
        record.fetch("chunk_index")
      end == (0...chunk_count).to_a
      raise "readable token template chunk metadata drifted" unless chunks.all? do |record|
        record.fetch("chunk_count") == chunk_count && record.fetch("slot_count") == slot_count
      end
      slot_id = 0
      segments = []
      chunks.each do |record|
        record.fetch("segments").each do |segment|
          if segment.keys == ["fixed"]
            fixed = segment.fetch("fixed").b
            if segments.last&.kind == :fixed
              segments.last.text << fixed
            else
              segments << HeptaNormalizedTokenBundle::Segment.new(:fixed, fixed)
            end
          elsif segment.keys == ["slot"] && segment.fetch("slot") == slot_id
            slot_id += 1
            segments << HeptaNormalizedTokenBundle::Segment.new(:slot, nil)
          else
            raise "invalid readable token template segment"
          end
        end
      end
      raise "readable token template slot count drifted" unless slot_id == slot_count
      HeptaNormalizedTokenBundle::Family.new(segments, slot_id)
    end
  end

  def parse_legacy_templates(records)
    records.each_with_index.map do |record, template_id|
      raise "readable token template order drifted" unless record.fetch("template_id") ==
        template_id
      slot_id = 0
      segments = record.fetch("segments").map do |segment|
        if segment.keys == ["fixed"]
          HeptaNormalizedTokenBundle::Segment.new(:fixed, segment.fetch("fixed").b)
        elsif segment.keys == ["slot"] && segment.fetch("slot") == slot_id
          slot_id += 1
          HeptaNormalizedTokenBundle::Segment.new(:slot, nil)
        else
          raise "invalid readable token template segment"
        end
      end
      raise "readable token template slot count drifted" unless record.fetch("slot_count") ==
        slot_id
      HeptaNormalizedTokenBundle::Family.new(segments, slot_id)
    end
  end

  def template_records(family, template_id, fixed_chunk_bytes:)
    slot_id = 0
    segments = family.segments.flat_map do |segment|
      if segment.kind == :fixed
        fixed_chunks(utf8(segment.text), fixed_chunk_bytes).map { |fixed| { fixed: fixed } }
      else
        value = { slot: slot_id }
        slot_id += 1
        value
      end
    end
    raise "readable token template slot count drifted" unless slot_id == family.slot_count
    return [{ template_id: template_id, slot_count: family.slot_count, segments: segments }] unless fixed_chunk_bytes
    chunk_count = segments.length
    segments.each_with_index.map do |segment, chunk_index|
      {
        template_id: template_id,
        chunk_index: chunk_index,
        chunk_count: chunk_count,
        slot_count: family.slot_count,
        segments: [segment]
      }
    end
  end

  def fixed_chunks(text, chunk_bytes)
    return [text] unless chunk_bytes
    raise "invalid readable token fixed chunk size" unless chunk_bytes.positive?
    chunks = []
    offset = 0
    while offset < text.bytesize
      finish = [offset + chunk_bytes, text.bytesize].min
      finish -= 1 while finish > offset && !text.byteslice(offset...finish).force_encoding(
        Encoding::UTF_8
      ).valid_encoding?
      raise "cannot split readable token fixed segment" if finish == offset
      chunks << text.byteslice(offset...finish).force_encoding(Encoding::UTF_8)
      offset = finish
    end
    chunks.empty? ? [""] : chunks
  end
end
