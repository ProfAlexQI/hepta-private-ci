use std::collections::BTreeMap;
use std::io::Read;

use anyhow::Context;
use anyhow::Result;
use flate2::read::GzDecoder;
use sha2::Digest;
use sha2::Sha256;

const MAGIC: &[u8] = b"hepta_gate_pair_normalized_payload_bundle_v3\0";
const MAX_BUNDLE_BYTES: u64 = 64 * 1024 * 1024;
const MAX_TEMPLATE_COUNT: usize = 1_499;

enum TemplateSegment<'a> {
    Fixed(&'a [u8]),
    Slot,
}

struct TemplateFamily<'a> {
    segments: Vec<TemplateSegment<'a>>,
    slot_count: usize,
}

pub(super) fn decode(compressed: &[u8]) -> Result<(BTreeMap<String, Vec<u8>>, usize)> {
    let mut decoder = GzDecoder::new(compressed).take(MAX_BUNDLE_BYTES + 1);
    let mut bundle = Vec::new();
    decoder
        .read_to_end(&mut bundle)
        .context("failed to decode Hepta gate-pair payload bundle")?;
    if bundle.len() as u64 > MAX_BUNDLE_BYTES {
        anyhow::bail!("Hepta gate-pair payload bundle exceeds byte limit");
    }

    let mut cursor = 0;
    consume_exact(&bundle, &mut cursor, MAGIC)?;
    let family_count = read_u32(&bundle, &mut cursor)? as usize;
    if family_count == 0 || family_count > MAX_TEMPLATE_COUNT {
        anyhow::bail!("Hepta normalized gate-pair template count exceeds limit");
    }
    let mut families = Vec::with_capacity(family_count);
    for _ in 0..family_count {
        families.push(decode_family(&bundle, &mut cursor)?);
    }
    let payload_count = read_u32(&bundle, &mut cursor)? as usize;
    let mut payloads = BTreeMap::new();
    for _ in 0..payload_count {
        let path_length = read_u32(&bundle, &mut cursor)? as usize;
        let family_index = read_u32(&bundle, &mut cursor)? as usize;
        let replacement_count = read_u32(&bundle, &mut cursor)? as usize;
        let source_length = read_u32(&bundle, &mut cursor)? as usize;
        let path_bytes = read_bytes(&bundle, &mut cursor, path_length)?;
        let expected_sha256 = read_bytes(&bundle, &mut cursor, 32)?;
        let family = families
            .get(family_index)
            .context("invalid Hepta gate-pair template family index")?;
        if replacement_count != family.slot_count {
            anyhow::bail!("Hepta gate-pair replacement count drifted");
        }
        let mut replacements = Vec::with_capacity(replacement_count);
        for _ in 0..replacement_count {
            let length = read_u32(&bundle, &mut cursor)? as usize;
            replacements.push(read_bytes(&bundle, &mut cursor, length)?);
        }
        let source = expand(family, &replacements)?;
        if source.len() != source_length {
            anyhow::bail!("Hepta gate-pair payload source size drifted");
        }
        if Sha256::digest(&source).as_slice() != expected_sha256 {
            anyhow::bail!("Hepta gate-pair payload digest mismatch");
        }
        let path = std::str::from_utf8(path_bytes)
            .context("Hepta gate-pair payload path is not UTF-8")?
            .to_string();
        if !valid_payload_path(&path) || payloads.insert(path.clone(), source).is_some() {
            anyhow::bail!("invalid or duplicate Hepta gate-pair payload path: {path}");
        }
    }
    if cursor != bundle.len() {
        anyhow::bail!("Hepta gate-pair payload bundle has trailing bytes");
    }
    Ok((payloads, family_count))
}

fn decode_family<'a>(bundle: &'a [u8], cursor: &mut usize) -> Result<TemplateFamily<'a>> {
    let segment_count = read_u32(bundle, cursor)? as usize;
    let slot_count = read_u32(bundle, cursor)? as usize;
    let mut actual_slot_count = 0;
    let mut segments = Vec::with_capacity(segment_count);
    for _ in 0..segment_count {
        match read_bytes(bundle, cursor, 1)?[0] {
            0 => {
                let length = read_u32(bundle, cursor)? as usize;
                segments.push(TemplateSegment::Fixed(read_bytes(bundle, cursor, length)?));
            }
            1 => {
                actual_slot_count += 1;
                segments.push(TemplateSegment::Slot);
            }
            _ => anyhow::bail!("invalid Hepta gate-pair template segment kind"),
        }
    }
    if actual_slot_count != slot_count {
        anyhow::bail!("Hepta gate-pair template slot count drifted");
    }
    Ok(TemplateFamily {
        segments,
        slot_count,
    })
}

fn expand(family: &TemplateFamily<'_>, replacements: &[&[u8]]) -> Result<Vec<u8>> {
    let mut replacement_index = 0;
    let mut source = Vec::new();
    for segment in &family.segments {
        match segment {
            TemplateSegment::Fixed(bytes) => source.extend_from_slice(bytes),
            TemplateSegment::Slot => {
                let replacement = replacements
                    .get(replacement_index)
                    .context("missing Hepta gate-pair template replacement")?;
                source.extend_from_slice(replacement);
                replacement_index += 1;
            }
        }
    }
    if replacement_index != replacements.len() {
        anyhow::bail!("unused Hepta gate-pair template replacements");
    }
    Ok(source)
}

fn consume_exact(bundle: &[u8], cursor: &mut usize, expected: &[u8]) -> Result<()> {
    if read_bytes(bundle, cursor, expected.len())? != expected {
        anyhow::bail!("invalid Hepta gate-pair payload bundle magic");
    }
    Ok(())
}

fn read_u32(bundle: &[u8], cursor: &mut usize) -> Result<u32> {
    let bytes: [u8; 4] = read_bytes(bundle, cursor, 4)?
        .try_into()
        .expect("four-byte slice");
    Ok(u32::from_be_bytes(bytes))
}

fn read_bytes<'a>(bundle: &'a [u8], cursor: &mut usize, length: usize) -> Result<&'a [u8]> {
    let end = cursor
        .checked_add(length)
        .context("Hepta gate-pair payload bundle offset overflow")?;
    let bytes = bundle
        .get(*cursor..end)
        .context("truncated Hepta gate-pair payload bundle")?;
    *cursor = end;
    Ok(bytes)
}

fn valid_payload_path(path: &str) -> bool {
    let Some(name) = path.strip_prefix("scripts/lib/hepta-gate-pair-compat-v1/") else {
        return false;
    };
    let Some(stem) = name
        .strip_suffix(".gate")
        .or_else(|| name.strip_suffix(".report"))
    else {
        return false;
    };
    !stem.is_empty()
        && stem
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}
