use crate::Digest;
use crate::InferError;
use crate::OperatorAuthenticationTag;
use crate::RequestGrant;
use crate::RequestId;
use crate::Result;
use crate::WorkerAuthenticationTag;

pub const PRIVATE_PROTOCOL_VERSION: u16 = 1;
pub const MAX_PRIVATE_FRAME_BYTES: usize = 64 * 1024;
pub const MAX_PRIVATE_TOKEN_BYTES: u64 = 16 * 1024;
const MAX_PRIVATE_TEXT_BYTES: usize = 1024;
const MAGIC: [u8; 4] = *b"HIFP";

const ROLE_WORKER: u8 = 1;
const ROLE_OPERATOR: u8 = 2;

const WORKER_HELLO: u8 = 1;
const WORKER_AUTHENTICATE: u8 = 2;
const WORKER_LEASE: u8 = 3;
const WORKER_START_ACK: u8 = 4;
const WORKER_TOKEN: u8 = 5;
const WORKER_COMPLETE: u8 = 6;
const WORKER_FAILURE: u8 = 7;
const WORKER_CANCEL: u8 = 8;
const WORKER_CANCEL_ACK: u8 = 9;
const WORKER_HEALTH: u8 = 10;

const OPERATOR_HELLO: u8 = 32;
const OPERATOR_AUTHENTICATE: u8 = 33;
const OPERATOR_RESTART: u8 = 34;
const OPERATOR_DRAIN: u8 = 35;
const OPERATOR_REGISTER_TUPLE: u8 = 36;
const OPERATOR_REMOVE_TUPLE: u8 = 37;
const OPERATOR_INVENTORY_REFRESH: u8 = 38;
const OPERATOR_COMPACT_RECEIPTS: u8 = 39;

pub enum WorkerPrivateMessage {
    Hello {
        worker_pid: u32,
        backend_generation: u64,
        worker_nonce_digest: Digest,
        daemon_challenge_digest: Digest,
    },
    Authenticate {
        worker_pid: u32,
        backend_generation: u64,
        worker_nonce_digest: Digest,
        daemon_challenge_digest: Digest,
        authentication: WorkerAuthenticationTag,
    },
    Lease {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
        worker_session_digest: Digest,
        request_grant: RequestGrant,
        tuple_digest: Digest,
        prompt_region_digest: Digest,
        deadline_unix_ms: u64,
        output_token_limit: u32,
        output_byte_limit: u64,
    },
    StartAck {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
        worker_session_digest: Digest,
        sequence: u64,
    },
    Token {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
        worker_session_digest: Digest,
        sequence: u64,
        token_digest: Digest,
        token_byte_length: u64,
    },
    Complete {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
        worker_session_digest: Digest,
        sequence: u64,
        result_digest: Digest,
        output_tokens: u32,
        output_bytes: u64,
    },
    Failure {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
        worker_session_digest: Digest,
        sequence: u64,
        reason_code: String,
    },
    Cancel {
        request_id: RequestId,
        request_generation: u64,
        cancel_generation: u64,
        backend_generation: u64,
        worker_session_digest: Digest,
        acknowledgement_deadline_unix_ms: u64,
    },
    CancelAck {
        request_id: RequestId,
        request_generation: u64,
        cancel_generation: u64,
        backend_generation: u64,
        worker_session_digest: Digest,
        sequence: u64,
    },
    Health {
        worker_pid: u32,
        backend_generation: u64,
        worker_session_digest: Digest,
        active_requests: u32,
        rss_bytes: u64,
        resident_model_bytes: u64,
    },
}

impl WorkerPrivateMessage {
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut writer = Writer::new(ROLE_WORKER, self.kind());
        match self {
            Self::Hello {
                worker_pid,
                backend_generation,
                worker_nonce_digest,
                daemon_challenge_digest,
            } => {
                writer.u32(*worker_pid);
                writer.u64(*backend_generation);
                writer.digest(worker_nonce_digest)?;
                writer.digest(daemon_challenge_digest)?;
            }
            Self::Authenticate {
                worker_pid,
                backend_generation,
                worker_nonce_digest,
                daemon_challenge_digest,
                authentication,
            } => {
                writer.u32(*worker_pid);
                writer.u64(*backend_generation);
                writer.digest(worker_nonce_digest)?;
                writer.digest(daemon_challenge_digest)?;
                writer.bytes32(authentication.private_bytes());
            }
            Self::Lease {
                request_id,
                request_generation,
                backend_generation,
                worker_session_digest,
                request_grant,
                tuple_digest,
                prompt_region_digest,
                deadline_unix_ms,
                output_token_limit,
                output_byte_limit,
            } => {
                writer.request_id(request_id)?;
                writer.u64(*request_generation);
                writer.u64(*backend_generation);
                writer.digest(worker_session_digest)?;
                writer.bytes32(request_grant.private_bytes());
                writer.digest(tuple_digest)?;
                writer.digest(prompt_region_digest)?;
                writer.u64(*deadline_unix_ms);
                writer.u32(*output_token_limit);
                writer.u64(*output_byte_limit);
            }
            Self::StartAck {
                request_id,
                request_generation,
                backend_generation,
                worker_session_digest,
                sequence,
            } => {
                write_event_fence(
                    &mut writer,
                    request_id,
                    *request_generation,
                    *backend_generation,
                    worker_session_digest,
                    *sequence,
                )?;
            }
            Self::Token {
                request_id,
                request_generation,
                backend_generation,
                worker_session_digest,
                sequence,
                token_digest,
                token_byte_length,
            } => {
                write_event_fence(
                    &mut writer,
                    request_id,
                    *request_generation,
                    *backend_generation,
                    worker_session_digest,
                    *sequence,
                )?;
                writer.digest(token_digest)?;
                writer.u64(*token_byte_length);
            }
            Self::Complete {
                request_id,
                request_generation,
                backend_generation,
                worker_session_digest,
                sequence,
                result_digest,
                output_tokens,
                output_bytes,
            } => {
                write_event_fence(
                    &mut writer,
                    request_id,
                    *request_generation,
                    *backend_generation,
                    worker_session_digest,
                    *sequence,
                )?;
                writer.digest(result_digest)?;
                writer.u32(*output_tokens);
                writer.u64(*output_bytes);
            }
            Self::Failure {
                request_id,
                request_generation,
                backend_generation,
                worker_session_digest,
                sequence,
                reason_code,
            } => {
                write_event_fence(
                    &mut writer,
                    request_id,
                    *request_generation,
                    *backend_generation,
                    worker_session_digest,
                    *sequence,
                )?;
                writer.text(reason_code)?;
            }
            Self::Cancel {
                request_id,
                request_generation,
                cancel_generation,
                backend_generation,
                worker_session_digest,
                acknowledgement_deadline_unix_ms,
            } => {
                writer.request_id(request_id)?;
                writer.u64(*request_generation);
                writer.u64(*cancel_generation);
                writer.u64(*backend_generation);
                writer.digest(worker_session_digest)?;
                writer.u64(*acknowledgement_deadline_unix_ms);
            }
            Self::CancelAck {
                request_id,
                request_generation,
                cancel_generation,
                backend_generation,
                worker_session_digest,
                sequence,
            } => {
                writer.request_id(request_id)?;
                writer.u64(*request_generation);
                writer.u64(*cancel_generation);
                writer.u64(*backend_generation);
                writer.digest(worker_session_digest)?;
                writer.u64(*sequence);
            }
            Self::Health {
                worker_pid,
                backend_generation,
                worker_session_digest,
                active_requests,
                rss_bytes,
                resident_model_bytes,
            } => {
                writer.u32(*worker_pid);
                writer.u64(*backend_generation);
                writer.digest(worker_session_digest)?;
                writer.u32(*active_requests);
                writer.u64(*rss_bytes);
                writer.u64(*resident_model_bytes);
            }
        }
        writer.finish()
    }

    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let (mut reader, kind) = Reader::new(bytes, ROLE_WORKER)?;
        let message = match kind {
            WORKER_HELLO => Self::Hello {
                worker_pid: reader.u32()?,
                backend_generation: reader.u64()?,
                worker_nonce_digest: reader.digest()?,
                daemon_challenge_digest: reader.digest()?,
            },
            WORKER_AUTHENTICATE => Self::Authenticate {
                worker_pid: reader.u32()?,
                backend_generation: reader.u64()?,
                worker_nonce_digest: reader.digest()?,
                daemon_challenge_digest: reader.digest()?,
                authentication: WorkerAuthenticationTag::from_private_bytes(reader.bytes32()?)?,
            },
            WORKER_LEASE => Self::Lease {
                request_id: reader.request_id()?,
                request_generation: reader.u64()?,
                backend_generation: reader.u64()?,
                worker_session_digest: reader.digest()?,
                request_grant: RequestGrant::from_private_bytes(reader.bytes32()?)?,
                tuple_digest: reader.digest()?,
                prompt_region_digest: reader.digest()?,
                deadline_unix_ms: reader.u64()?,
                output_token_limit: reader.u32()?,
                output_byte_limit: reader.u64()?,
            },
            WORKER_START_ACK => {
                let (request_id, request_generation, backend_generation, session, sequence) =
                    read_event_fence(&mut reader)?;
                Self::StartAck {
                    request_id,
                    request_generation,
                    backend_generation,
                    worker_session_digest: session,
                    sequence,
                }
            }
            WORKER_TOKEN => {
                let (request_id, request_generation, backend_generation, session, sequence) =
                    read_event_fence(&mut reader)?;
                Self::Token {
                    request_id,
                    request_generation,
                    backend_generation,
                    worker_session_digest: session,
                    sequence,
                    token_digest: reader.digest()?,
                    token_byte_length: reader.u64()?,
                }
            }
            WORKER_COMPLETE => {
                let (request_id, request_generation, backend_generation, session, sequence) =
                    read_event_fence(&mut reader)?;
                Self::Complete {
                    request_id,
                    request_generation,
                    backend_generation,
                    worker_session_digest: session,
                    sequence,
                    result_digest: reader.digest()?,
                    output_tokens: reader.u32()?,
                    output_bytes: reader.u64()?,
                }
            }
            WORKER_FAILURE => {
                let (request_id, request_generation, backend_generation, session, sequence) =
                    read_event_fence(&mut reader)?;
                Self::Failure {
                    request_id,
                    request_generation,
                    backend_generation,
                    worker_session_digest: session,
                    sequence,
                    reason_code: reader.text()?,
                }
            }
            WORKER_CANCEL => Self::Cancel {
                request_id: reader.request_id()?,
                request_generation: reader.u64()?,
                cancel_generation: reader.u64()?,
                backend_generation: reader.u64()?,
                worker_session_digest: reader.digest()?,
                acknowledgement_deadline_unix_ms: reader.u64()?,
            },
            WORKER_CANCEL_ACK => Self::CancelAck {
                request_id: reader.request_id()?,
                request_generation: reader.u64()?,
                cancel_generation: reader.u64()?,
                backend_generation: reader.u64()?,
                worker_session_digest: reader.digest()?,
                sequence: reader.u64()?,
            },
            WORKER_HEALTH => Self::Health {
                worker_pid: reader.u32()?,
                backend_generation: reader.u64()?,
                worker_session_digest: reader.digest()?,
                active_requests: reader.u32()?,
                rss_bytes: reader.u64()?,
                resident_model_bytes: reader.u64()?,
            },
            _ => return Err(InferError::ProtocolShape),
        };
        message.validate()?;
        reader.finish()?;
        Ok(message)
    }

    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Hello {
                worker_pid,
                backend_generation,
                ..
            }
            | Self::Authenticate {
                worker_pid,
                backend_generation,
                ..
            }
            | Self::Health {
                worker_pid,
                backend_generation,
                ..
            } => {
                require_nonzero_u32(*worker_pid)?;
                require_generation(*backend_generation)
            }
            Self::Lease {
                request_generation,
                backend_generation,
                deadline_unix_ms,
                output_token_limit,
                output_byte_limit,
                ..
            } => {
                require_generation(*request_generation)?;
                require_generation(*backend_generation)?;
                if *deadline_unix_ms == 0 || *output_token_limit == 0 || *output_byte_limit == 0 {
                    return Err(InferError::ProtocolBound);
                }
                Ok(())
            }
            Self::StartAck {
                request_generation,
                backend_generation,
                sequence,
                ..
            }
            | Self::Token {
                request_generation,
                backend_generation,
                sequence,
                ..
            }
            | Self::Complete {
                request_generation,
                backend_generation,
                sequence,
                ..
            }
            | Self::Failure {
                request_generation,
                backend_generation,
                sequence,
                ..
            } => {
                require_generation(*request_generation)?;
                require_generation(*backend_generation)?;
                require_sequence(*sequence)?;
                if let Self::Token {
                    token_byte_length,
                    ..
                } = self
                {
                    if *token_byte_length == 0 || *token_byte_length > MAX_PRIVATE_TOKEN_BYTES {
                        return Err(InferError::ProtocolBound);
                    }
                }
                if let Self::Complete {
                    output_tokens,
                    output_bytes,
                    ..
                } = self
                {
                    if *output_tokens == 0 || *output_bytes == 0 {
                        return Err(InferError::ProtocolBound);
                    }
                }
                if let Self::Failure { reason_code, .. } = self {
                    validate_reason_code(reason_code)?;
                }
                Ok(())
            }
            Self::Cancel {
                request_generation,
                cancel_generation,
                backend_generation,
                acknowledgement_deadline_unix_ms,
                ..
            } => {
                require_generation(*request_generation)?;
                require_generation(*cancel_generation)?;
                require_generation(*backend_generation)?;
                if *acknowledgement_deadline_unix_ms == 0 {
                    return Err(InferError::ProtocolBound);
                }
                Ok(())
            }
            Self::CancelAck {
                request_generation,
                cancel_generation,
                backend_generation,
                sequence,
                ..
            } => {
                require_generation(*request_generation)?;
                require_generation(*cancel_generation)?;
                require_generation(*backend_generation)?;
                require_sequence(*sequence)
            }
        }
    }

    const fn kind(&self) -> u8 {
        match self {
            Self::Hello { .. } => WORKER_HELLO,
            Self::Authenticate { .. } => WORKER_AUTHENTICATE,
            Self::Lease { .. } => WORKER_LEASE,
            Self::StartAck { .. } => WORKER_START_ACK,
            Self::Token { .. } => WORKER_TOKEN,
            Self::Complete { .. } => WORKER_COMPLETE,
            Self::Failure { .. } => WORKER_FAILURE,
            Self::Cancel { .. } => WORKER_CANCEL,
            Self::CancelAck { .. } => WORKER_CANCEL_ACK,
            Self::Health { .. } => WORKER_HEALTH,
        }
    }
}

pub enum OperatorPrivateMessage {
    Hello {
        operator_pid: u32,
        backend_generation: u64,
        operator_nonce_digest: Digest,
        daemon_challenge_digest: Digest,
    },
    Authenticate {
        operator_pid: u32,
        backend_generation: u64,
        operator_nonce_digest: Digest,
        daemon_challenge_digest: Digest,
        authentication: OperatorAuthenticationTag,
    },
    Restart {
        expected_backend_generation: u64,
    },
    Drain {
        expected_backend_generation: u64,
        deadline_unix_ms: u64,
    },
    RegisterTuple {
        expected_backend_generation: u64,
        tuple_digest: Digest,
        evidence_receipt_digest: Digest,
    },
    RemoveTuple {
        expected_backend_generation: u64,
        tuple_digest: Digest,
    },
    InventoryRefresh {
        expected_backend_generation: u64,
    },
    CompactReceipts {
        expected_backend_generation: u64,
        retain_after_unix_ms: u64,
        maximum_entries: u64,
    },
}

impl OperatorPrivateMessage {
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut writer = Writer::new(ROLE_OPERATOR, self.kind());
        match self {
            Self::Hello {
                operator_pid,
                backend_generation,
                operator_nonce_digest,
                daemon_challenge_digest,
            } => {
                writer.u32(*operator_pid);
                writer.u64(*backend_generation);
                writer.digest(operator_nonce_digest)?;
                writer.digest(daemon_challenge_digest)?;
            }
            Self::Authenticate {
                operator_pid,
                backend_generation,
                operator_nonce_digest,
                daemon_challenge_digest,
                authentication,
            } => {
                writer.u32(*operator_pid);
                writer.u64(*backend_generation);
                writer.digest(operator_nonce_digest)?;
                writer.digest(daemon_challenge_digest)?;
                writer.bytes32(authentication.private_bytes());
            }
            Self::Restart {
                expected_backend_generation,
            }
            | Self::InventoryRefresh {
                expected_backend_generation,
            } => writer.u64(*expected_backend_generation),
            Self::Drain {
                expected_backend_generation,
                deadline_unix_ms,
            } => {
                writer.u64(*expected_backend_generation);
                writer.u64(*deadline_unix_ms);
            }
            Self::RegisterTuple {
                expected_backend_generation,
                tuple_digest,
                evidence_receipt_digest,
            } => {
                writer.u64(*expected_backend_generation);
                writer.digest(tuple_digest)?;
                writer.digest(evidence_receipt_digest)?;
            }
            Self::RemoveTuple {
                expected_backend_generation,
                tuple_digest,
            } => {
                writer.u64(*expected_backend_generation);
                writer.digest(tuple_digest)?;
            }
            Self::CompactReceipts {
                expected_backend_generation,
                retain_after_unix_ms,
                maximum_entries,
            } => {
                writer.u64(*expected_backend_generation);
                writer.u64(*retain_after_unix_ms);
                writer.u64(*maximum_entries);
            }
        }
        writer.finish()
    }

    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let (mut reader, kind) = Reader::new(bytes, ROLE_OPERATOR)?;
        let message = match kind {
            OPERATOR_HELLO => Self::Hello {
                operator_pid: reader.u32()?,
                backend_generation: reader.u64()?,
                operator_nonce_digest: reader.digest()?,
                daemon_challenge_digest: reader.digest()?,
            },
            OPERATOR_AUTHENTICATE => Self::Authenticate {
                operator_pid: reader.u32()?,
                backend_generation: reader.u64()?,
                operator_nonce_digest: reader.digest()?,
                daemon_challenge_digest: reader.digest()?,
                authentication: OperatorAuthenticationTag::from_private_bytes(reader.bytes32()?)?,
            },
            OPERATOR_RESTART => Self::Restart {
                expected_backend_generation: reader.u64()?,
            },
            OPERATOR_DRAIN => Self::Drain {
                expected_backend_generation: reader.u64()?,
                deadline_unix_ms: reader.u64()?,
            },
            OPERATOR_REGISTER_TUPLE => Self::RegisterTuple {
                expected_backend_generation: reader.u64()?,
                tuple_digest: reader.digest()?,
                evidence_receipt_digest: reader.digest()?,
            },
            OPERATOR_REMOVE_TUPLE => Self::RemoveTuple {
                expected_backend_generation: reader.u64()?,
                tuple_digest: reader.digest()?,
            },
            OPERATOR_INVENTORY_REFRESH => Self::InventoryRefresh {
                expected_backend_generation: reader.u64()?,
            },
            OPERATOR_COMPACT_RECEIPTS => Self::CompactReceipts {
                expected_backend_generation: reader.u64()?,
                retain_after_unix_ms: reader.u64()?,
                maximum_entries: reader.u64()?,
            },
            _ => return Err(InferError::ProtocolShape),
        };
        message.validate()?;
        reader.finish()?;
        Ok(message)
    }

    pub fn validate(&self) -> Result<()> {
        match self {
            Self::Hello {
                operator_pid,
                backend_generation,
                ..
            }
            | Self::Authenticate {
                operator_pid,
                backend_generation,
                ..
            } => {
                require_nonzero_u32(*operator_pid)?;
                require_generation(*backend_generation)
            }
            Self::Restart {
                expected_backend_generation,
            }
            | Self::InventoryRefresh {
                expected_backend_generation,
            }
            | Self::RegisterTuple {
                expected_backend_generation,
                ..
            }
            | Self::RemoveTuple {
                expected_backend_generation,
                ..
            } => require_generation(*expected_backend_generation),
            Self::Drain {
                expected_backend_generation,
                deadline_unix_ms,
            } => {
                require_generation(*expected_backend_generation)?;
                if *deadline_unix_ms == 0 {
                    return Err(InferError::ProtocolBound);
                }
                Ok(())
            }
            Self::CompactReceipts {
                expected_backend_generation,
                retain_after_unix_ms,
                maximum_entries,
            } => {
                require_generation(*expected_backend_generation)?;
                if *retain_after_unix_ms == 0 || *maximum_entries == 0 {
                    return Err(InferError::ProtocolBound);
                }
                Ok(())
            }
        }
    }

    const fn kind(&self) -> u8 {
        match self {
            Self::Hello { .. } => OPERATOR_HELLO,
            Self::Authenticate { .. } => OPERATOR_AUTHENTICATE,
            Self::Restart { .. } => OPERATOR_RESTART,
            Self::Drain { .. } => OPERATOR_DRAIN,
            Self::RegisterTuple { .. } => OPERATOR_REGISTER_TUPLE,
            Self::RemoveTuple { .. } => OPERATOR_REMOVE_TUPLE,
            Self::InventoryRefresh { .. } => OPERATOR_INVENTORY_REFRESH,
            Self::CompactReceipts { .. } => OPERATOR_COMPACT_RECEIPTS,
        }
    }
}

fn write_event_fence(
    writer: &mut Writer,
    request_id: &RequestId,
    request_generation: u64,
    backend_generation: u64,
    worker_session_digest: &Digest,
    sequence: u64,
) -> Result<()> {
    writer.request_id(request_id)?;
    writer.u64(request_generation);
    writer.u64(backend_generation);
    writer.digest(worker_session_digest)?;
    writer.u64(sequence);
    Ok(())
}

fn read_event_fence(reader: &mut Reader<'_>) -> Result<(RequestId, u64, u64, Digest, u64)> {
    Ok((
        reader.request_id()?,
        reader.u64()?,
        reader.u64()?,
        reader.digest()?,
        reader.u64()?,
    ))
}

fn require_nonzero_u32(value: u32) -> Result<()> {
    if value == 0 {
        Err(InferError::ProtocolBound)
    } else {
        Ok(())
    }
}

fn require_generation(value: u64) -> Result<()> {
    if value == 0 {
        Err(InferError::InvalidGeneration)
    } else {
        Ok(())
    }
}

fn require_sequence(value: u64) -> Result<()> {
    if value == 0 {
        Err(InferError::StaleOrNonMonotonicSequence)
    } else {
        Ok(())
    }
}

fn validate_reason_code(value: &str) -> Result<()> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || bytes.len() > 128
        || !bytes
            .iter()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || *byte == b'_')
    {
        return Err(InferError::ProtocolShape);
    }
    Ok(())
}

struct Writer {
    bytes: Vec<u8>,
}

impl Writer {
    fn new(role: u8, kind: u8) -> Self {
        let mut bytes = Vec::with_capacity(256);
        bytes.extend_from_slice(&MAGIC);
        bytes.extend_from_slice(&PRIVATE_PROTOCOL_VERSION.to_be_bytes());
        bytes.push(role);
        bytes.push(kind);
        Self { bytes }
    }

    fn finish(self) -> Result<Vec<u8>> {
        if self.bytes.len() > MAX_PRIVATE_FRAME_BYTES {
            Err(InferError::ProtocolBound)
        } else {
            Ok(self.bytes)
        }
    }

    fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn text(&mut self, value: &str) -> Result<()> {
        if value.len() > MAX_PRIVATE_TEXT_BYTES {
            return Err(InferError::ProtocolBound);
        }
        let length = u32::try_from(value.len()).map_err(|_| InferError::ProtocolBound)?;
        self.u32(length);
        self.bytes.extend_from_slice(value.as_bytes());
        Ok(())
    }

    fn digest(&mut self, value: &Digest) -> Result<()> {
        self.text(value.as_str())
    }

    fn request_id(&mut self, value: &RequestId) -> Result<()> {
        self.text(value.as_str())
    }

    fn bytes32(&mut self, value: &[u8; 32]) {
        self.bytes.extend_from_slice(value);
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8], expected_role: u8) -> Result<(Self, u8)> {
        if bytes.len() > MAX_PRIVATE_FRAME_BYTES || bytes.len() < 8 {
            return Err(InferError::ProtocolBound);
        }
        if bytes.get(0..4) != Some(MAGIC.as_slice()) {
            return Err(InferError::ProtocolShape);
        }
        let version = u16::from_be_bytes(
            bytes
                .get(4..6)
                .ok_or(InferError::ProtocolTruncated)?
                .try_into()
                .map_err(|_| InferError::ProtocolTruncated)?,
        );
        if version != PRIVATE_PROTOCOL_VERSION {
            return Err(InferError::ProtocolVersion);
        }
        let role = *bytes.get(6).ok_or(InferError::ProtocolTruncated)?;
        if role != expected_role {
            return Err(InferError::RoleNotAuthorized);
        }
        let kind = *bytes.get(7).ok_or(InferError::ProtocolTruncated)?;
        Ok((Self { bytes, offset: 8 }, kind))
    }

    fn finish(&self) -> Result<()> {
        if self.offset == self.bytes.len() {
            Ok(())
        } else {
            Err(InferError::ProtocolTrailingData)
        }
    }

    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes(self.array_bytes()?))
    }

    fn u64(&mut self) -> Result<u64> {
        Ok(u64::from_be_bytes(self.array_bytes()?))
    }

    fn text(&mut self) -> Result<String> {
        let length = usize::try_from(self.u32()?).map_err(|_| InferError::ProtocolBound)?;
        if length > MAX_PRIVATE_TEXT_BYTES {
            return Err(InferError::ProtocolBound);
        }
        let end = self
            .offset
            .checked_add(length)
            .ok_or(InferError::ProtocolBound)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(InferError::ProtocolTruncated)?;
        self.offset = end;
        std::str::from_utf8(value)
            .map(str::to_owned)
            .map_err(|_| InferError::ProtocolUtf8)
    }

    fn digest(&mut self) -> Result<Digest> {
        Digest::parse(&self.text()?)
    }

    fn request_id(&mut self) -> Result<RequestId> {
        RequestId::parse(&self.text()?)
    }

    fn bytes32(&mut self) -> Result<[u8; 32]> {
        self.array_bytes()
    }

    fn array_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        let end = self
            .offset
            .checked_add(N)
            .ok_or(InferError::ProtocolBound)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(InferError::ProtocolTruncated)?;
        self.offset = end;
        value.try_into().map_err(|_| InferError::ProtocolTruncated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PRIVATE_AUTH_TAG_BYTES;

    fn must<T, E: std::fmt::Display>(result: std::result::Result<T, E>) -> T {
        match result {
            Ok(value) => value,
            Err(error) => panic!("unexpected error: {error}"),
        }
    }

    fn digest(fill: char) -> Digest {
        must(Digest::parse(&format!(
            "sha256:{}",
            fill.to_string().repeat(64)
        )))
    }

    fn request_id() -> RequestId {
        must(RequestId::parse("request-private"))
    }

    #[test]
    fn worker_token_round_trip_preserves_all_fences() {
        let encoded = must(
            WorkerPrivateMessage::Token {
                request_id: request_id(),
                request_generation: 2,
                backend_generation: 3,
                worker_session_digest: digest('a'),
                sequence: 4,
                token_digest: digest('b'),
                token_byte_length: 5,
            }
            .encode_canonical(),
        );
        match must(WorkerPrivateMessage::decode_canonical(&encoded)) {
            WorkerPrivateMessage::Token {
                request_id,
                request_generation,
                backend_generation,
                sequence,
                token_byte_length,
                ..
            } => {
                assert_eq!(request_id.as_str(), "request-private");
                assert_eq!(request_generation, 2);
                assert_eq!(backend_generation, 3);
                assert_eq!(sequence, 4);
                assert_eq!(token_byte_length, 5);
            }
            _ => panic!("decoded wrong worker message"),
        }
    }

    #[test]
    fn role_domains_are_not_interchangeable() {
        let encoded = must(
            OperatorPrivateMessage::Restart {
                expected_backend_generation: 7,
            }
            .encode_canonical(),
        );
        assert_eq!(
            WorkerPrivateMessage::decode_canonical(&encoded).err(),
            Some(InferError::RoleNotAuthorized)
        );
    }

    #[test]
    fn private_frames_are_rejected_by_public_protocol() {
        let encoded = must(
            WorkerPrivateMessage::Hello {
                worker_pid: 42,
                backend_generation: 7,
                worker_nonce_digest: digest('a'),
                daemon_challenge_digest: digest('b'),
            }
            .encode_canonical(),
        );
        assert!(crate::ClientMessage::decode_canonical(&encoded).is_err());
    }

    #[test]
    fn authentication_round_trip_keeps_secret_off_debug_surface() {
        let tag = must(WorkerAuthenticationTag::from_private_bytes(
            [7u8; PRIVATE_AUTH_TAG_BYTES],
        ));
        let encoded = must(
            WorkerPrivateMessage::Authenticate {
                worker_pid: 42,
                backend_generation: 7,
                worker_nonce_digest: digest('a'),
                daemon_challenge_digest: digest('b'),
                authentication: tag,
            }
            .encode_canonical(),
        );
        match must(WorkerPrivateMessage::decode_canonical(&encoded)) {
            WorkerPrivateMessage::Authenticate { authentication, .. } => {
                assert_eq!(authentication.private_bytes(), &[7u8; PRIVATE_AUTH_TAG_BYTES]);
            }
            _ => panic!("decoded wrong worker message"),
        }
    }

    #[test]
    fn oversized_token_and_trailing_data_fail_closed() {
        let message = WorkerPrivateMessage::Token {
            request_id: request_id(),
            request_generation: 1,
            backend_generation: 1,
            worker_session_digest: digest('a'),
            sequence: 1,
            token_digest: digest('b'),
            token_byte_length: MAX_PRIVATE_TOKEN_BYTES + 1,
        };
        assert_eq!(message.validate(), Err(InferError::ProtocolBound));

        let mut encoded = must(
            OperatorPrivateMessage::Restart {
                expected_backend_generation: 7,
            }
            .encode_canonical(),
        );
        encoded.push(0);
        assert_eq!(
            OperatorPrivateMessage::decode_canonical(&encoded).err(),
            Some(InferError::ProtocolTrailingData)
        );
    }
}
