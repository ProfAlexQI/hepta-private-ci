fn write_u32(output: &mut Vec<u8>, value: u32) {
    output.extend_from_slice(&value.to_be_bytes());
}

fn write_u64(output: &mut Vec<u8>, value: u64) {
    output.extend_from_slice(&value.to_be_bytes());
}

fn write_digest(output: &mut Vec<u8>, digest: Digest32) {
    output.extend_from_slice(digest.as_bytes());
}

fn write_string(output: &mut Vec<u8>, value: &str) -> Result<(), ContractError> {
    write_u32(output, usize_to_u32(value.len(), "ANN string bytes")?);
    output.extend_from_slice(value.as_bytes());
    Ok(())
}

fn append_length_prefixed(output: &mut Vec<u8>, value: &[u8]) {
    output.extend_from_slice(
        &u64::try_from(value.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    output.extend_from_slice(value);
}

struct ByteCursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> ByteCursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], ContractError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(ContractError::Overflow)?;
        let slice = self.bytes.get(self.position..end).ok_or_else(|| {
            ContractError::Corrupt("unexpected end of ANN index file".to_string())
        })?;
        self.position = end;
        Ok(slice)
    }

    fn read_u32(&mut self) -> Result<u32, ContractError> {
        let bytes: [u8; 4] = self
            .take(4)?
            .try_into()
            .map_err(|_| ContractError::Corrupt("invalid u32 bytes".to_string()))?;
        Ok(u32::from_be_bytes(bytes))
    }

    fn read_u64(&mut self) -> Result<u64, ContractError> {
        let bytes: [u8; 8] = self
            .take(8)?
            .try_into()
            .map_err(|_| ContractError::Corrupt("invalid u64 bytes".to_string()))?;
        Ok(u64::from_be_bytes(bytes))
    }

    fn read_i16(&mut self) -> Result<i16, ContractError> {
        let bytes: [u8; 2] = self
            .take(2)?
            .try_into()
            .map_err(|_| ContractError::Corrupt("invalid i16 bytes".to_string()))?;
        Ok(i16::from_be_bytes(bytes))
    }

    fn read_digest(&mut self) -> Result<Digest32, ContractError> {
        let bytes: [u8; 32] = self
            .take(32)?
            .try_into()
            .map_err(|_| ContractError::Corrupt("invalid digest bytes".to_string()))?;
        Ok(Digest32::from_bytes(bytes))
    }

    fn read_string(&mut self) -> Result<String, ContractError> {
        let length = usize::try_from(self.read_u32()?).map_err(|_| ContractError::Overflow)?;
        if length > crate::MAX_ID_BYTES.saturating_mul(4) {
            return Err(ContractError::Corrupt(
                "ANN string exceeds bounded file limit".to_string(),
            ));
        }
        String::from_utf8(self.take(length)?.to_vec())
            .map_err(|_| ContractError::Corrupt("ANN string is not UTF-8".to_string()))
    }

    fn is_finished(&self) -> bool {
        self.position == self.bytes.len()
    }
}
