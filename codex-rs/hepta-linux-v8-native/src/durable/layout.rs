use crate::NativeErrorV8;
use crate::invalid;

pub const ATTEMPTS_DIRECTORY_V8: &str = "attempts";
pub const JOURNAL_DIRECTORY_V8: &str = "journal";
pub const NONCE_CLAIMS_DIRECTORY_V8: &str = "nonce-claims";
pub const QUARANTINE_DIRECTORY_V8: &str = "quarantine";
pub const JOURNAL_RECORD_DIGITS_V8: usize = 20;
pub(crate) const MAX_STATE_ROOT_LEAVES_V8: usize = 65_536;
pub(crate) const MAX_NONCE_CLAIM_LEAVES_V8: usize = 65_536;
pub(crate) const MAX_DURABLE_JOURNAL_LEAVES_V8: usize = 65_536;
pub(crate) const MAX_DURABLE_JOURNAL_RECORDS_V8: usize = 65_536;

/// Linux exposes `/proc/sys/kernel/random/boot_id` as one canonical,
/// lowercase UUID. Accept only that exact spelling so evidence cannot mix
/// ad-hoc hex tokens with a real kernel boot identity.
pub(crate) fn validate_boot_id_v8(value: &str) -> Result<(), NativeErrorV8> {
    if value.len() != 36
        || value.as_bytes().get(8) != Some(&b'-')
        || value.as_bytes().get(13) != Some(&b'-')
        || value.as_bytes().get(18) != Some(&b'-')
        || value.as_bytes().get(23) != Some(&b'-')
        || value.bytes().all(|byte| matches!(byte, b'0' | b'-'))
        || !value.bytes().enumerate().all(|(index, byte)| {
            if matches!(index, 8 | 13 | 18 | 23) {
                byte == b'-'
            } else {
                byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')
            }
        })
    {
        return Err(invalid(
            "boot id must be one non-zero canonical lowercase UUID",
        ));
    }
    Ok(())
}

pub fn nonce_claim_relative_path_v8(nonce: &str) -> Result<String, NativeErrorV8> {
    validate_digest("nonce", nonce)?;
    Ok(format!("{NONCE_CLAIMS_DIRECTORY_V8}/{nonce}.claim"))
}

pub fn attempt_relative_path_v8(attempt_sha256: &str) -> Result<String, NativeErrorV8> {
    validate_digest("attempt", attempt_sha256)?;
    Ok(format!("{ATTEMPTS_DIRECTORY_V8}/{attempt_sha256}"))
}

pub fn journal_record_name_v8(global_sequence: u64) -> Result<String, NativeErrorV8> {
    if global_sequence == 0 {
        return Err(invalid("journal global sequence must be non-zero"));
    }
    Ok(format!(
        "{global_sequence:0JOURNAL_RECORD_DIGITS_V8$}.record"
    ))
}

pub fn incoming_name_v8(final_name: &str, nonce: &str) -> Result<String, NativeErrorV8> {
    validate_leaf_name(final_name)?;
    validate_digest("incoming nonce", nonce)?;
    Ok(format!(".{final_name}.{nonce}.incoming"))
}

pub(crate) fn validate_leaf_name(name: &str) -> Result<(), NativeErrorV8> {
    if name.is_empty()
        || name.len() > 255
        || name == "."
        || name == ".."
        || name.contains('/')
        || name.contains('\0')
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(invalid("durable leaf name is not a safe single component"));
    }
    Ok(())
}

pub(crate) fn validate_digest(label: &str, value: &str) -> Result<(), NativeErrorV8> {
    if value.len() != 64
        || value.bytes().all(|byte| byte == b'0')
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(invalid(format!(
            "{label} must be one non-zero lowercase SHA-256 digest"
        )));
    }
    Ok(())
}

pub(crate) fn require_inventory_capacity_v8(
    label: &str,
    current_entries: usize,
    maximum_entries: usize,
) -> Result<(), NativeErrorV8> {
    if current_entries >= maximum_entries {
        return Err(invalid(format!(
            "{label} reached its frozen {maximum_entries}-leaf inventory limit"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_global_exact_and_traversal_free() {
        let nonce = "1".repeat(64);
        assert_eq!(
            nonce_claim_relative_path_v8(&nonce).unwrap(),
            format!("nonce-claims/{nonce}.claim")
        );
        assert_eq!(
            journal_record_name_v8(7).unwrap(),
            "00000000000000000007.record"
        );
        assert!(journal_record_name_v8(0).is_err());
        assert!(incoming_name_v8("../record", &nonce).is_err());
        assert!(nonce_claim_relative_path_v8(&"0".repeat(64)).is_err());
    }

    #[test]
    fn boot_id_requires_kernel_canonical_uuid_spelling() {
        assert!(validate_boot_id_v8("01234567-89ab-cdef-0123-456789abcdef").is_ok());
        assert!(validate_boot_id_v8("0123456789abcdef0123456789abcdef").is_err());
        assert!(validate_boot_id_v8("01234567-89AB-cdef-0123-456789abcdef").is_err());
        assert!(validate_boot_id_v8("00000000-0000-0000-0000-000000000000").is_err());
    }

    #[test]
    fn inventory_capacity_rejects_the_first_excess_publication() {
        assert!(require_inventory_capacity_v8("test", 0, 1).is_ok());
        assert!(require_inventory_capacity_v8("test", 1, 1).is_err());
    }
}
