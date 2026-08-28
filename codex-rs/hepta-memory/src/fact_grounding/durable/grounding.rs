use super::*;

mod ledger;
mod prepare;

pub(super) use ledger::insert_tx;
pub(super) use ledger::verify_receipts;
pub(super) use prepare::bind_exact_citation;
pub(super) use prepare::prepare;
pub(super) use prepare::require_groundable_revision;
pub(super) use prepare::validate_canonical_identity_binding;
pub(super) use prepare::validate_source_binding;

fn fact_identity_digest<'a, I>(identities: I) -> Sha256Digest
where
    I: ExactSizeIterator<Item = &'a FactIdentity>,
{
    let count = identities.len();
    let mut hasher = Sha256::new();
    super::super::frame_part(&mut hasher, b"hepta:cognitive:fact-identities:v1");
    super::super::frame_part(
        &mut hasher,
        &u64::try_from(count)
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    for identity in identities {
        super::super::frame_part(&mut hasher, identity.kind.as_str().as_bytes());
        super::super::frame_part(&mut hasher, identity.key.as_bytes());
    }
    Sha256Digest::from_sha256_output(hasher.finalize())
}

fn validate_span_range(
    source_text: &str,
    start: usize,
    end: usize,
) -> Result<(), CognitiveStoreError> {
    if start >= end || end > source_text.len() {
        return Err(CognitiveStoreError::Invalid(format!(
            "evidence range {start}..{end} is outside the source"
        )));
    }
    if !source_text.is_char_boundary(start) || !source_text.is_char_boundary(end) {
        return Err(CognitiveStoreError::Invalid(
            "evidence range splits a UTF-8 character".to_string(),
        ));
    }
    Ok(())
}

fn support_is_sufficient(text: &str, support: &FactSupport) -> bool {
    match support {
        FactSupport::Entity { label } => text.contains(&semantic_normalize(label)),
        FactSupport::Relation {
            from_label,
            to_label,
            relation,
        } => {
            text.contains(&semantic_normalize(from_label))
                && text.contains(&semantic_normalize(to_label))
                && text.contains(&semantic_normalize(relation))
        }
    }
}

fn semantic_normalize(value: &str) -> String {
    let mut normalized = String::with_capacity(value.len());
    let mut pending_space = false;
    for character in value.chars().flat_map(char::to_lowercase) {
        if character.is_alphanumeric() {
            if pending_space && !normalized.is_empty() {
                normalized.push(' ');
            }
            normalized.push(character);
            pending_space = false;
        } else {
            pending_space = true;
        }
    }
    normalized
}

fn canonical_token(
    value: &str,
    max_bytes: usize,
    label: &str,
) -> Result<String, CognitiveStoreError> {
    let value = canonical_text(value, max_bytes, label)?.to_ascii_lowercase();
    if value.len() > max_bytes {
        return Err(CognitiveStoreError::Invalid(format!(
            "{label} exceeds {max_bytes} bytes after canonicalization"
        )));
    }
    Ok(value)
}

fn canonical_text(
    value: &str,
    max_bytes: usize,
    label: &str,
) -> Result<String, CognitiveStoreError> {
    if value.as_bytes().contains(&0) {
        return Err(CognitiveStoreError::Invalid(format!(
            "{label} contains a NUL byte"
        )));
    }
    let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
    if value.is_empty() || value.len() > max_bytes {
        return Err(CognitiveStoreError::Invalid(format!(
            "{label} must contain 1..={max_bytes} bytes"
        )));
    }
    Ok(value)
}

fn require_semantic_text(
    value: &str,
    label: &str,
) -> Result<(), CognitiveStoreError> {
    if semantic_normalize(value).is_empty() {
        return Err(CognitiveStoreError::Invalid(format!(
            "{label} contains no semantic characters"
        )));
    }
    Ok(())
}

pub(super) fn to_i64(value: u64, label: &str) -> Result<i64, CognitiveStoreError> {
    i64::try_from(value)
        .map_err(|_| CognitiveStoreError::Invalid(format!("{label} exceeds i64")))
}
