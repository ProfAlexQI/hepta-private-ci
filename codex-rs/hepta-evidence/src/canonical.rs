use codex_hepta_contracts::Sha256Digest;
use serde::Serialize;
use serde_json::Value;

use crate::EvidenceError;

pub(crate) fn canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, EvidenceError> {
    let mut value = serde_json::to_value(value)
        .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
    sort_value(&mut value);
    serde_json::to_vec(&value).map_err(|error| EvidenceError::Serialization(error.to_string()))
}

pub(crate) fn canonical_storage_payload<T: Serialize>(
    value: &T,
) -> Result<(String, Sha256Digest), EvidenceError> {
    let payload = canonical_json(value)?;
    let payload_json = String::from_utf8(payload)
        .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
    let payload_sha256 = Sha256Digest::for_bytes(payload_json.as_bytes());
    Ok((payload_json, payload_sha256))
}

pub(crate) fn verify_storage_payload_digest(
    payload_json: &str,
    expected: &str,
    record_kind: &str,
) -> Result<(), EvidenceError> {
    let actual = Sha256Digest::for_bytes(payload_json.as_bytes());
    if actual.as_str() == expected {
        Ok(())
    } else {
        Err(EvidenceError::Corrupt(format!(
            "stored {record_kind} payload digest mismatch"
        )))
    }
}

pub(crate) fn validate_digest(label: &str, digest: &Sha256Digest) -> Result<(), EvidenceError> {
    Sha256Digest::parse(digest.as_str())
        .map(|_| ())
        .map_err(|_| {
            EvidenceError::InvalidRecord(format!(
                "{label} digest is not canonical lowercase SHA-256"
            ))
        })
}

pub(crate) fn verify_canonical_storage_payload<T: Serialize>(
    value: &T,
    stored: &str,
    record_kind: &str,
) -> Result<(), EvidenceError> {
    let canonical = canonical_json(value)?;
    if canonical == stored.as_bytes() {
        Ok(())
    } else {
        Err(EvidenceError::Corrupt(format!(
            "stored {record_kind} JSON is not canonical"
        )))
    }
}

pub(crate) fn invalid_record_as_corrupt(error: EvidenceError) -> EvidenceError {
    match error {
        EvidenceError::InvalidRecord(detail) => EvidenceError::Corrupt(detail),
        other => other,
    }
}

fn sort_value(value: &mut Value) {
    match value {
        Value::Array(items) => {
            for item in items {
                sort_value(item);
            }
        }
        Value::Object(map) => {
            let mut entries = std::mem::take(map).into_iter().collect::<Vec<_>>();
            entries.sort_by(|(left, _), (right, _)| left.cmp(right));
            for (_, item) in &mut entries {
                sort_value(item);
            }
            map.extend(entries);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::canonical_storage_payload;

    #[test]
    fn storage_payload_has_fixed_sorted_json_and_sha256_oracle() {
        let value = serde_json::json!({
            "z": 1,
            "list": [{"b": 1, "a": 2}],
            "a": {"z": 2, "a": 3},
        });
        let (payload_json, payload_sha256) =
            canonical_storage_payload(&value).expect("canonical storage payload");

        assert_eq!(
            payload_json,
            r#"{"a":{"a":3,"z":2},"list":[{"a":2,"b":1}],"z":1}"#
        );
        assert_eq!(
            payload_sha256.as_str(),
            "2d0c8efa120f8fed7856c164ea8b5ae5f828b2ec798b48ddbf2942692115c47d"
        );
    }
}
