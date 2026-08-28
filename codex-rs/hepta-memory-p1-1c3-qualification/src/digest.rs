use crate::ContractError;
use sha2::{Digest as _, Sha256};
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Digest32([u8; 32]);

impl Digest32 {
    #[must_use]
    pub fn for_bytes(bytes: &[u8]) -> Self {
        let output = Sha256::digest(bytes);
        let mut digest = [0_u8; 32];
        digest.copy_from_slice(&output);
        Self(digest)
    }

    pub fn from_hex(value: &str) -> Result<Self, ContractError> {
        if value.len() != 64 {
            return Err(ContractError::Invalid(
                "SHA-256 hex value must contain exactly 64 characters".to_string(),
            ));
        }
        let mut bytes = [0_u8; 32];
        for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
            bytes[index] = (decode_nibble(pair[0])? << 4) | decode_nibble(pair[1])?;
        }
        Ok(Self(bytes))
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    #[must_use]
    pub fn to_hex(self) -> String {
        let mut output = String::with_capacity(64);
        for byte in self.0 {
            use std::fmt::Write as _;
            write!(&mut output, "{byte:02x}").expect("writing to String cannot fail");
        }
        output
    }
}

impl Display for Digest32 {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.to_hex())
    }
}

fn decode_nibble(byte: u8) -> Result<u8, ContractError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(ContractError::Invalid(
            "SHA-256 digest must use lowercase hexadecimal".to_string(),
        )),
    }
}

pub(crate) fn framed_digest(domain: &[u8], parts: &[&[u8]]) -> Digest32 {
    let mut bytes = Vec::new();
    append_frame(&mut bytes, domain);
    for part in parts {
        append_frame(&mut bytes, part);
    }
    Digest32::for_bytes(&bytes)
}

pub(crate) fn append_frame(output: &mut Vec<u8>, value: &[u8]) {
    output.extend_from_slice(&u64::try_from(value.len()).unwrap_or(u64::MAX).to_be_bytes());
    output.extend_from_slice(value);
}

#[cfg(test)]
mod tests {
    use super::Digest32;

    #[test]
    fn sha256_known_answer_matches() {
        assert_eq!(
            Digest32::for_bytes(b"abc").to_hex(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
