use serde::Serialize;
use serde_json::Value;

use crate::EvidenceError;

pub(crate) fn canonical_json<T: Serialize>(value: &T) -> Result<Vec<u8>, EvidenceError> {
    let mut value = serde_json::to_value(value)
        .map_err(|error| EvidenceError::Serialization(error.to_string()))?;
    sort_value(&mut value);
    serde_json::to_vec(&value).map_err(|error| EvidenceError::Serialization(error.to_string()))
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
    use codex_hepta_contracts::Sha256Digest;

    use super::canonical_json;

    #[test]
    fn storage_payload_has_fixed_sorted_json_and_sha256_oracle() {
        let value = serde_json::json!({
            "z": 1,
            "list": [{"b": 1, "a": 2}],
            "a": {"z": 2, "a": 3},
        });
        let payload = canonical_json(&value).expect("canonical storage payload");

        assert_eq!(
            String::from_utf8(payload.clone()).expect("UTF-8 JSON"),
            r#"{"a":{"a":3,"z":2},"list":[{"a":2,"b":1}],"z":1}"#
        );
        assert_eq!(
            Sha256Digest::for_bytes(&payload).as_str(),
            "2d0c8efa120f8fed7856c164ea8b5ae5f828b2ec798b48ddbf2942692115c47d"
        );
    }
}
