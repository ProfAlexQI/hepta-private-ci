use sha2::Digest as _;

use crate::NativeErrorV8;
use crate::invalid;

use super::validate_boot_id_v8;
use super::validate_digest;

pub const DURABLE_JOURNAL_RECORD_SCHEMA_V8: &[u8] = b"hepta-linux-v8-durable-journal-record-v1\0";
const ZERO_SHA256: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const MAX_PAYLOAD_BYTES_V8: usize = 64 * 1024 * 1024;
const MAX_BOOT_ID_BYTES_V8: usize = 128;

/// Canonical bytes stored as one no-replace durable journal file.
///
/// Constructing this value proves only structural validity. Durable origin is
/// established separately by the publisher/scanner token; semantic effects
/// remain the responsibility of the qualification contract verifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurableJournalRecordV8 {
    attempt_identity_sha256: String,
    boot_epoch: u64,
    boot_id: String,
    global_sequence: u64,
    payload: Vec<u8>,
    previous_record_sha256: String,
}

impl DurableJournalRecordV8 {
    pub fn new(
        attempt_identity_sha256: String,
        boot_epoch: u64,
        boot_id: String,
        global_sequence: u64,
        previous_record_sha256: String,
        payload: Vec<u8>,
    ) -> Result<Self, NativeErrorV8> {
        let record = Self {
            attempt_identity_sha256,
            boot_epoch,
            boot_id,
            global_sequence,
            payload,
            previous_record_sha256,
        };
        record.validate()?;
        Ok(record)
    }

    pub fn attempt_identity_sha256(&self) -> &str {
        &self.attempt_identity_sha256
    }

    pub fn boot_epoch(&self) -> u64 {
        self.boot_epoch
    }

    pub fn boot_id(&self) -> &str {
        &self.boot_id
    }

    pub fn global_sequence(&self) -> u64 {
        self.global_sequence
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn previous_record_sha256(&self) -> &str {
        &self.previous_record_sha256
    }

    pub fn payload_sha256(&self) -> String {
        sha256(&self.payload)
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, NativeErrorV8> {
        self.validate()?;
        let mut output = Vec::with_capacity(
            DURABLE_JOURNAL_RECORD_SCHEMA_V8.len()
                + 8
                + 64
                + 8
                + 8
                + self.boot_id.len()
                + 8
                + 64
                + 8
                + self.payload.len(),
        );
        output.extend_from_slice(DURABLE_JOURNAL_RECORD_SCHEMA_V8);
        output.extend_from_slice(&self.global_sequence.to_be_bytes());
        output.extend_from_slice(self.attempt_identity_sha256.as_bytes());
        output.extend_from_slice(&self.boot_epoch.to_be_bytes());
        append_bytes(&mut output, self.boot_id.as_bytes())?;
        output.extend_from_slice(self.previous_record_sha256.as_bytes());
        append_bytes(&mut output, &self.payload)?;
        Ok(output)
    }

    pub fn record_sha256(&self) -> Result<String, NativeErrorV8> {
        Ok(sha256(&self.canonical_bytes()?))
    }

    pub fn decode_exact(bytes: &[u8]) -> Result<Self, NativeErrorV8> {
        if bytes.len() > MAX_PAYLOAD_BYTES_V8 + 1024 {
            return Err(invalid("durable journal record exceeds the size limit"));
        }
        let mut decoder = Decoder::new(bytes);
        decoder.expect(DURABLE_JOURNAL_RECORD_SCHEMA_V8)?;
        let global_sequence = decoder.u64()?;
        let attempt_identity_sha256 = decoder.ascii(64, "attempt identity")?;
        let boot_epoch = decoder.u64()?;
        let boot_id = decoder.length_prefixed_ascii(MAX_BOOT_ID_BYTES_V8, "boot id")?;
        let previous_record_sha256 = decoder.ascii(64, "previous journal hash")?;
        let payload = decoder.length_prefixed(MAX_PAYLOAD_BYTES_V8, "journal payload")?;
        if !decoder.is_finished() {
            return Err(invalid("durable journal record has trailing bytes"));
        }
        let record = Self::new(
            attempt_identity_sha256,
            boot_epoch,
            boot_id,
            global_sequence,
            previous_record_sha256,
            payload,
        )?;
        if record.canonical_bytes()? != bytes {
            return Err(invalid("durable journal record is not canonical"));
        }
        Ok(record)
    }

    fn validate(&self) -> Result<(), NativeErrorV8> {
        validate_digest("attempt identity", &self.attempt_identity_sha256)?;
        if self.global_sequence == 0 {
            return Err(invalid("journal global sequence must be non-zero"));
        }
        if self.boot_epoch == 0 {
            return Err(invalid("journal boot epoch must be non-zero"));
        }
        validate_boot_id_v8(&self.boot_id)?;
        if self.global_sequence == 1 {
            if self.previous_record_sha256 != ZERO_SHA256 {
                return Err(invalid(
                    "first journal record must use the zero predecessor",
                ));
            }
        } else {
            validate_digest("previous journal", &self.previous_record_sha256)?;
        }
        if self.payload.is_empty() || self.payload.len() > MAX_PAYLOAD_BYTES_V8 {
            return Err(invalid("journal payload size is invalid"));
        }
        Ok(())
    }
}

/// Opaque result of verifying canonical bytes as one complete hash chain.
#[derive(Debug)]
pub struct VerifiedDurableJournalChainV8 {
    attempt_identity_sha256: String,
    record_count: u64,
    tip_sha256: String,
}

impl VerifiedDurableJournalChainV8 {
    pub fn attempt_identity_sha256(&self) -> &str {
        &self.attempt_identity_sha256
    }

    pub fn record_count(&self) -> u64 {
        self.record_count
    }

    pub fn tip_sha256(&self) -> &str {
        &self.tip_sha256
    }
}

pub fn verify_durable_journal_chain_v8(
    encoded_records: &[Vec<u8>],
    expected_attempt_identity_sha256: &str,
) -> Result<VerifiedDurableJournalChainV8, NativeErrorV8> {
    validate_digest(
        "expected attempt identity",
        expected_attempt_identity_sha256,
    )?;
    if encoded_records.is_empty() {
        return Err(invalid("durable journal chain must not be empty"));
    }

    let mut previous_hash = ZERO_SHA256.to_string();
    for (index, encoded) in encoded_records.iter().enumerate() {
        let record = DurableJournalRecordV8::decode_exact(encoded)?;
        let expected_sequence = u64::try_from(index)
            .map_err(|_| invalid("journal record index overflow"))?
            .checked_add(1)
            .ok_or_else(|| invalid("journal sequence overflow"))?;
        if record.global_sequence() != expected_sequence {
            return Err(invalid("durable journal sequence has a gap or reorder"));
        }
        if record.attempt_identity_sha256() != expected_attempt_identity_sha256 {
            return Err(invalid("durable journal attempts were spliced"));
        }
        if record.previous_record_sha256() != previous_hash {
            return Err(invalid("durable journal predecessor hash mismatches"));
        }
        previous_hash = record.record_sha256()?;
    }

    Ok(VerifiedDurableJournalChainV8 {
        attempt_identity_sha256: expected_attempt_identity_sha256.to_string(),
        record_count: u64::try_from(encoded_records.len())
            .map_err(|_| invalid("journal record count overflow"))?,
        tip_sha256: previous_hash,
    })
}

fn append_bytes(output: &mut Vec<u8>, value: &[u8]) -> Result<(), NativeErrorV8> {
    let length = u64::try_from(value.len()).map_err(|_| invalid("journal field overflow"))?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(bytes))
}

struct Decoder<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Decoder<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn expect(&mut self, expected: &[u8]) -> Result<(), NativeErrorV8> {
        if self.take(expected.len())? != expected {
            return Err(invalid("durable journal schema magic mismatches"));
        }
        Ok(())
    }

    fn u64(&mut self) -> Result<u64, NativeErrorV8> {
        let bytes: [u8; 8] = self
            .take(8)?
            .try_into()
            .map_err(|_| invalid("durable journal integer is truncated"))?;
        Ok(u64::from_be_bytes(bytes))
    }

    fn ascii(&mut self, length: usize, label: &str) -> Result<String, NativeErrorV8> {
        let bytes = self.take(length)?;
        if !bytes.is_ascii() {
            return Err(invalid(format!("durable journal {label} is not ASCII")));
        }
        String::from_utf8(bytes.to_vec())
            .map_err(|_| invalid(format!("durable journal {label} is not UTF-8")))
    }

    fn length_prefixed(&mut self, maximum: usize, label: &str) -> Result<Vec<u8>, NativeErrorV8> {
        let length = usize::try_from(self.u64()?)
            .map_err(|_| invalid(format!("durable journal {label} length overflows")))?;
        if length > maximum {
            return Err(invalid(format!("durable journal {label} is too large")));
        }
        Ok(self.take(length)?.to_vec())
    }

    fn length_prefixed_ascii(
        &mut self,
        maximum: usize,
        label: &str,
    ) -> Result<String, NativeErrorV8> {
        let bytes = self.length_prefixed(maximum, label)?;
        if !bytes.is_ascii() {
            return Err(invalid(format!("durable journal {label} is not ASCII")));
        }
        String::from_utf8(bytes)
            .map_err(|_| invalid(format!("durable journal {label} is not UTF-8")))
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], NativeErrorV8> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or_else(|| invalid("durable journal decoder offset overflow"))?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or_else(|| invalid("durable journal record is truncated"))?;
        self.offset = end;
        Ok(value)
    }

    fn is_finished(&self) -> bool {
        self.offset == self.bytes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(character: char) -> String {
        character.to_string().repeat(64)
    }

    fn record(sequence: u64, previous: String, attempt: &str) -> DurableJournalRecordV8 {
        DurableJournalRecordV8::new(
            attempt.to_string(),
            1,
            "01234567-89ab-cdef-0123-456789abcdef".to_string(),
            sequence,
            previous,
            format!("payload-{sequence}").into_bytes(),
        )
        .unwrap()
    }

    #[test]
    fn exact_encoding_round_trips_and_chain_verifies() {
        let attempt = digest('1');
        let first = record(1, ZERO_SHA256.to_string(), &attempt);
        let second = record(2, first.record_sha256().unwrap(), &attempt);
        let encoded = vec![
            first.canonical_bytes().unwrap(),
            second.canonical_bytes().unwrap(),
        ];
        let verified = verify_durable_journal_chain_v8(&encoded, &attempt).unwrap();
        assert_eq!(verified.record_count(), 2);
        assert_eq!(verified.tip_sha256(), second.record_sha256().unwrap());
        assert_eq!(
            DurableJournalRecordV8::decode_exact(&encoded[0]).unwrap(),
            first
        );
    }

    #[test]
    fn gap_splice_tamper_and_trailing_bytes_fail_closed() {
        let attempt = digest('1');
        let first = record(1, ZERO_SHA256.to_string(), &attempt);
        let mut gap = record(2, first.record_sha256().unwrap(), &attempt)
            .canonical_bytes()
            .unwrap();
        let sequence_offset = DURABLE_JOURNAL_RECORD_SCHEMA_V8.len();
        gap[sequence_offset..sequence_offset + 8].copy_from_slice(&3_u64.to_be_bytes());
        assert!(
            verify_durable_journal_chain_v8(&[first.canonical_bytes().unwrap(), gap], &attempt)
                .is_err()
        );

        let spliced = record(2, first.record_sha256().unwrap(), &digest('2'));
        assert!(
            verify_durable_journal_chain_v8(
                &[
                    first.canonical_bytes().unwrap(),
                    spliced.canonical_bytes().unwrap()
                ],
                &attempt
            )
            .is_err()
        );

        let mut trailing = first.canonical_bytes().unwrap();
        trailing.push(0);
        assert!(DurableJournalRecordV8::decode_exact(&trailing).is_err());
    }
}
