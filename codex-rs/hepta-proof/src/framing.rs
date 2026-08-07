use codex_hepta_contracts::Sha256Digest;

pub(crate) fn length_delimited_sha256<'a>(
    parts: impl IntoIterator<Item = &'a str>,
) -> Sha256Digest {
    let mut canonical = Vec::new();
    for part in parts {
        canonical.extend_from_slice(&(part.len() as u64).to_be_bytes());
        canonical.extend_from_slice(part.as_bytes());
    }
    Sha256Digest::for_bytes(&canonical)
}
