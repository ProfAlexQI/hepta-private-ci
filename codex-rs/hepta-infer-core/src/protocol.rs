use crate::AcceptedEvent;
use crate::AgentId;
use crate::AuthoritySnapshot;
use crate::ControllerSnapshot;
use crate::Digest;
use crate::EventFence;
use crate::InferError;
use crate::InferenceRequest;
use crate::LifecycleState;
use crate::RequestId;
use crate::RequestIdentity;
use crate::ResourceBudgetId;
use crate::Result;
use crate::StateEvent;
use crate::TaskId;
use crate::TenantId;
use crate::TerminalReceipt;
use crate::WorkspaceId;

pub const PROTOCOL_VERSION: u64 = 1;
pub const MAX_FRAME_BYTES: usize = 64 * 1024;
const MAX_TEXT_BYTES: usize = 1024;

const REQUEST_PING: u64 = 0;
const REQUEST_ADMIT: u64 = 1;
const REQUEST_START: u64 = 2;
const REQUEST_TOKEN: u64 = 3;
const REQUEST_COMPLETE: u64 = 4;
const REQUEST_CANCEL: u64 = 5;
const REQUEST_RESTART_BACKEND: u64 = 6;
const REQUEST_SNAPSHOT: u64 = 7;

const RESPONSE_PONG: u64 = 100;
const RESPONSE_ACCEPTED: u64 = 101;
const RESPONSE_STATE: u64 = 102;
const RESPONSE_RECEIPT: u64 = 103;
const RESPONSE_RESTARTED: u64 = 104;
const RESPONSE_SNAPSHOT: u64 = 105;
const RESPONSE_ERROR: u64 = 199;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClientMessage {
    Ping {
        nonce: u64,
    },
    Admit(InferenceRequest),
    Start {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
    },
    Token {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
        sequence: u64,
        token_digest: Digest,
        token_byte_length: u64,
    },
    Complete {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
        sequence: u64,
        result_digest: Digest,
        output_tokens: u32,
    },
    Cancel {
        request_id: RequestId,
        request_generation: u64,
        cancel_generation: u64,
        backend_generation: u64,
    },
    RestartBackend {
        expected_generation: u64,
    },
    Snapshot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ServerMessage {
    Pong {
        nonce: u64,
    },
    Accepted(AcceptedEvent),
    State(StateEvent),
    Receipt(TerminalReceipt),
    Restarted {
        backend_generation: u64,
        receipts: Vec<TerminalReceipt>,
    },
    Snapshot(ControllerSnapshot),
    Error {
        code: String,
    },
}

impl ClientMessage {
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        let mut encoder = Encoder::default();
        match self {
            Self::Ping { nonce } => {
                encoder.array(3)?;
                encoder.uint(PROTOCOL_VERSION);
                encoder.uint(REQUEST_PING);
                encoder.uint(*nonce);
            }
            Self::Admit(request) => encode_admit(&mut encoder, request)?,
            Self::Start {
                request_id,
                request_generation,
                backend_generation,
            } => {
                encoder.array(5)?;
                encoder.uint(PROTOCOL_VERSION);
                encoder.uint(REQUEST_START);
                encoder.text(request_id.as_str())?;
                encoder.uint(*request_generation);
                encoder.uint(*backend_generation);
            }
            Self::Token {
                request_id,
                request_generation,
                backend_generation,
                sequence,
                token_digest,
                token_byte_length,
            } => {
                encoder.array(8)?;
                encoder.uint(PROTOCOL_VERSION);
                encoder.uint(REQUEST_TOKEN);
                encoder.text(request_id.as_str())?;
                encoder.uint(*request_generation);
                encoder.uint(*backend_generation);
                encoder.uint(*sequence);
                encoder.text(token_digest.as_str())?;
                encoder.uint(*token_byte_length);
            }
            Self::Complete {
                request_id,
                request_generation,
                backend_generation,
                sequence,
                result_digest,
                output_tokens,
            } => {
                encoder.array(8)?;
                encoder.uint(PROTOCOL_VERSION);
                encoder.uint(REQUEST_COMPLETE);
                encoder.text(request_id.as_str())?;
                encoder.uint(*request_generation);
                encoder.uint(*backend_generation);
                encoder.uint(*sequence);
                encoder.text(result_digest.as_str())?;
                encoder.uint(u64::from(*output_tokens));
            }
            Self::Cancel {
                request_id,
                request_generation,
                cancel_generation,
                backend_generation,
            } => {
                encoder.array(6)?;
                encoder.uint(PROTOCOL_VERSION);
                encoder.uint(REQUEST_CANCEL);
                encoder.text(request_id.as_str())?;
                encoder.uint(*request_generation);
                encoder.uint(*cancel_generation);
                encoder.uint(*backend_generation);
            }
            Self::RestartBackend {
                expected_generation,
            } => {
                encoder.array(3)?;
                encoder.uint(PROTOCOL_VERSION);
                encoder.uint(REQUEST_RESTART_BACKEND);
                encoder.uint(*expected_generation);
            }
            Self::Snapshot => {
                encoder.array(2)?;
                encoder.uint(PROTOCOL_VERSION);
                encoder.uint(REQUEST_SNAPSHOT);
            }
        }
        encoder.finish()
    }

    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let mut decoder = Decoder::new(bytes);
        let length = decoder.array()?;
        if !(2..=18).contains(&length) {
            return Err(InferError::ProtocolShape);
        }
        decoder.require_version()?;
        let kind = decoder.uint()?;
        let message = match kind {
            REQUEST_PING if length == 3 => Self::Ping {
                nonce: decoder.uint()?,
            },
            REQUEST_ADMIT if length == 18 => Self::Admit(decode_admit(&mut decoder)?),
            REQUEST_START if length == 5 => Self::Start {
                request_id: RequestId::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
                request_generation: decoder.uint()?,
                backend_generation: decoder.uint()?,
            },
            REQUEST_TOKEN if length == 8 => Self::Token {
                request_id: RequestId::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
                request_generation: decoder.uint()?,
                backend_generation: decoder.uint()?,
                sequence: decoder.uint()?,
                token_digest: Digest::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
                token_byte_length: decoder.uint()?,
            },
            REQUEST_COMPLETE if length == 8 => Self::Complete {
                request_id: RequestId::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
                request_generation: decoder.uint()?,
                backend_generation: decoder.uint()?,
                sequence: decoder.uint()?,
                result_digest: Digest::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
                output_tokens: decoder.u32()?,
            },
            REQUEST_CANCEL if length == 6 => Self::Cancel {
                request_id: RequestId::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
                request_generation: decoder.uint()?,
                cancel_generation: decoder.uint()?,
                backend_generation: decoder.uint()?,
            },
            REQUEST_RESTART_BACKEND if length == 3 => Self::RestartBackend {
                expected_generation: decoder.uint()?,
            },
            REQUEST_SNAPSHOT if length == 2 => Self::Snapshot,
            _ => return Err(InferError::ProtocolShape),
        };
        decoder.finish()?;
        Ok(message)
    }
}

impl ServerMessage {
    pub fn encode_canonical(&self) -> Result<Vec<u8>> {
        let mut encoder = Encoder::default();
        match self {
            Self::Pong { nonce } => {
                encoder.array(3)?;
                encoder.uint(PROTOCOL_VERSION);
                encoder.uint(RESPONSE_PONG);
                encoder.uint(*nonce);
            }
            Self::Accepted(event) => {
                encoder.array(6)?;
                encoder.uint(PROTOCOL_VERSION);
                encoder.uint(RESPONSE_ACCEPTED);
                encoder.text(event.request_id.as_str())?;
                encoder.uint(event.request_generation);
                encoder.uint(event.backend_generation);
                encoder.uint(event.sequence);
            }
            Self::State(event) => {
                encoder.array(7)?;
                encoder.uint(PROTOCOL_VERSION);
                encoder.uint(RESPONSE_STATE);
                encoder.text(event.request_id.as_str())?;
                encoder.uint(event.request_generation);
                encoder.uint(event.backend_generation);
                encoder.uint(event.sequence);
                encoder.uint(u64::from(event.state as u8));
            }
            Self::Receipt(receipt) => encode_receipt_response(&mut encoder, receipt)?,
            Self::Restarted {
                backend_generation,
                receipts,
            } => {
                encoder.array(4)?;
                encoder.uint(PROTOCOL_VERSION);
                encoder.uint(RESPONSE_RESTARTED);
                encoder.uint(*backend_generation);
                encoder.array(receipts.len())?;
                for receipt in receipts {
                    encode_receipt(&mut encoder, receipt)?;
                }
            }
            Self::Snapshot(snapshot) => {
                encoder.array(10)?;
                encoder.uint(PROTOCOL_VERSION);
                encoder.uint(RESPONSE_SNAPSHOT);
                encoder.uint(snapshot.backend_generation);
                encoder.uint(usize_to_u64(snapshot.queued_requests)?);
                encoder.uint(usize_to_u64(snapshot.running_requests)?);
                encoder.uint(usize_to_u64(snapshot.terminal_receipts)?);
                encoder.uint(usize_to_u64(snapshot.registered_tuples)?);
                encoder.uint(usize_to_u64(snapshot.max_queue)?);
                encoder.uint(usize_to_u64(snapshot.max_per_tenant)?);
                encoder.boolean(snapshot.authority.qualification_only);
            }
            Self::Error { code } => {
                encoder.array(3)?;
                encoder.uint(PROTOCOL_VERSION);
                encoder.uint(RESPONSE_ERROR);
                encoder.text(code)?;
            }
        }
        encoder.finish()
    }

    pub fn decode_canonical(bytes: &[u8]) -> Result<Self> {
        let mut decoder = Decoder::new(bytes);
        let length = decoder.array()?;
        decoder.require_version()?;
        let kind = decoder.uint()?;
        let message = match kind {
            RESPONSE_PONG if length == 3 => Self::Pong {
                nonce: decoder.uint()?,
            },
            RESPONSE_ACCEPTED if length == 6 => Self::Accepted(AcceptedEvent {
                request_id: RequestId::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
                request_generation: decoder.uint()?,
                backend_generation: decoder.uint()?,
                sequence: decoder.uint()?,
            }),
            RESPONSE_STATE if length == 7 => Self::State(StateEvent {
                request_id: RequestId::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
                request_generation: decoder.uint()?,
                backend_generation: decoder.uint()?,
                sequence: decoder.uint()?,
                state: decode_state(decoder.uint()?)?,
            }),
            RESPONSE_RECEIPT if length == 3 => Self::Receipt(decode_receipt(&mut decoder)?),
            RESPONSE_RESTARTED if length == 4 => {
                let backend_generation = decoder.uint()?;
                let receipt_count = decoder.array()?;
                if receipt_count > 1024 {
                    return Err(InferError::ProtocolBound);
                }
                let mut receipts = Vec::with_capacity(receipt_count);
                for _ in 0..receipt_count {
                    receipts.push(decode_receipt(&mut decoder)?);
                }
                Self::Restarted {
                    backend_generation,
                    receipts,
                }
            }
            RESPONSE_SNAPSHOT if length == 10 => Self::Snapshot(ControllerSnapshot {
                backend_generation: decoder.uint()?,
                queued_requests: decoder.usize()?,
                running_requests: decoder.usize()?,
                terminal_receipts: decoder.usize()?,
                registered_tuples: decoder.usize()?,
                max_queue: decoder.usize()?,
                max_per_tenant: decoder.usize()?,
                authority: decode_snapshot_authority(decoder.boolean()?)?,
            }),
            RESPONSE_ERROR if length == 3 => Self::Error {
                code: decoder.text(MAX_TEXT_BYTES)?,
            },
            _ => return Err(InferError::ProtocolShape),
        };
        decoder.finish()?;
        Ok(message)
    }
}

fn encode_admit(encoder: &mut Encoder, request: &InferenceRequest) -> Result<()> {
    encoder.array(18)?;
    encoder.uint(PROTOCOL_VERSION);
    encoder.uint(REQUEST_ADMIT);
    encoder.text(request.identity.tenant_id.as_str())?;
    encoder.text(request.identity.workspace_id.as_str())?;
    encoder.text(request.identity.agent_id.as_str())?;
    encoder.text(request.identity.task_id.as_str())?;
    encoder.text(request.identity.request_id.as_str())?;
    encoder.uint(request.agent_generation);
    encoder.uint(request.request_generation);
    encoder.uint(request.cancel_generation);
    encoder.uint(request.deadline_unix_ms);
    encoder.text(request.model_tuple_digest.as_str())?;
    encoder.text(request.policy_digest.as_str())?;
    encoder.text(request.resource_budget_id.as_str())?;
    encoder.text(request.prompt_digest.as_str())?;
    encoder.uint(request.prompt_byte_length);
    encoder.uint(u64::from(request.output_token_limit));
    encoder.boolean(request.authority.qualification_only);
    Ok(())
}

fn decode_admit(decoder: &mut Decoder<'_>) -> Result<InferenceRequest> {
    let request = InferenceRequest {
        identity: RequestIdentity {
            tenant_id: TenantId::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
            workspace_id: WorkspaceId::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
            agent_id: AgentId::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
            task_id: TaskId::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
            request_id: RequestId::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
        },
        agent_generation: decoder.uint()?,
        request_generation: decoder.uint()?,
        cancel_generation: decoder.uint()?,
        deadline_unix_ms: decoder.uint()?,
        model_tuple_digest: Digest::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
        policy_digest: Digest::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
        resource_budget_id: ResourceBudgetId::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
        prompt_digest: Digest::parse(&decoder.text(MAX_TEXT_BYTES)?)?,
        prompt_byte_length: decoder.uint()?,
        output_token_limit: decoder.u32()?,
        authority: decode_snapshot_authority(decoder.boolean()?)?,
    };
    request.validate_shape()?;
    Ok(request)
}

fn encode_receipt_response(encoder: &mut Encoder, receipt: &TerminalReceipt) -> Result<()> {
    encoder.array(3)?;
    encoder.uint(PROTOCOL_VERSION);
    encoder.uint(RESPONSE_RECEIPT);
    encode_receipt(encoder, receipt)
}

fn encode_receipt(encoder: &mut Encoder, receipt: &TerminalReceipt) -> Result<()> {
    encoder.array(11)?;
    encoder.text(receipt.request_id.as_str())?;
    encoder.uint(receipt.request_generation);
    encoder.uint(receipt.cancel_generation);
    encoder.uint(receipt.backend_generation);
    encoder.uint(u64::from(receipt.terminal_state as u8));
    encoder.uint(receipt.last_sequence);
    encoder.uint(u64::from(receipt.output_tokens));
    match &receipt.result_digest {
        Some(digest) => {
            encoder.boolean(true);
            encoder.text(digest.as_str())?;
        }
        None => {
            encoder.boolean(false);
            encoder.text("")?;
        }
    }
    encoder.boolean(receipt.forced_worker_termination);
    encoder.boolean(receipt.authority.qualification_only);
    Ok(())
}

fn decode_receipt(decoder: &mut Decoder<'_>) -> Result<TerminalReceipt> {
    if decoder.array()? != 11 {
        return Err(InferError::ProtocolShape);
    }
    let request_id = RequestId::parse(&decoder.text(MAX_TEXT_BYTES)?)?;
    let request_generation = decoder.uint()?;
    let cancel_generation = decoder.uint()?;
    let backend_generation = decoder.uint()?;
    let terminal_state = decode_state(decoder.uint()?)?;
    if !terminal_state.is_terminal() {
        return Err(InferError::ProtocolShape);
    }
    let last_sequence = decoder.uint()?;
    let output_tokens = decoder.u32()?;
    let has_result_digest = decoder.boolean()?;
    let digest_text = decoder.text(MAX_TEXT_BYTES)?;
    let result_digest = if has_result_digest {
        Some(Digest::parse(&digest_text)?)
    } else if digest_text.is_empty() {
        None
    } else {
        return Err(InferError::ProtocolShape);
    };
    let forced_worker_termination = decoder.boolean()?;
    let authority = decode_snapshot_authority(decoder.boolean()?)?;
    Ok(TerminalReceipt {
        request_id,
        request_generation,
        cancel_generation,
        backend_generation,
        terminal_state,
        last_sequence,
        output_tokens,
        result_digest,
        forced_worker_termination,
        authority,
    })
}

fn decode_state(value: u64) -> Result<LifecycleState> {
    match value {
        value if value == u64::from(LifecycleState::Queued as u8) => Ok(LifecycleState::Queued),
        value if value == u64::from(LifecycleState::Running as u8) => Ok(LifecycleState::Running),
        value if value == u64::from(LifecycleState::Draining as u8) => Ok(LifecycleState::Draining),
        value if value == u64::from(LifecycleState::Completed as u8) => {
            Ok(LifecycleState::Completed)
        }
        value if value == u64::from(LifecycleState::Cancelled as u8) => {
            Ok(LifecycleState::Cancelled)
        }
        value if value == u64::from(LifecycleState::FailedClosed as u8) => {
            Ok(LifecycleState::FailedClosed)
        }
        _ => Err(InferError::ProtocolShape),
    }
}

fn decode_snapshot_authority(qualification_only: bool) -> Result<AuthoritySnapshot> {
    if qualification_only {
        Ok(AuthoritySnapshot::qualification_only_closed())
    } else {
        Err(InferError::AuthorityEscalation)
    }
}

fn usize_to_u64(value: usize) -> Result<u64> {
    u64::try_from(value).map_err(|_| InferError::ProtocolBound)
}

#[derive(Default)]
struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    fn finish(self) -> Result<Vec<u8>> {
        if self.bytes.len() > MAX_FRAME_BYTES {
            Err(InferError::ProtocolBound)
        } else {
            Ok(self.bytes)
        }
    }

    fn array(&mut self, length: usize) -> Result<()> {
        self.major(4, usize_to_u64(length)?)
    }

    fn uint(&mut self, value: u64) {
        let _ = self.major(0, value);
    }

    fn boolean(&mut self, value: bool) {
        self.bytes.push(if value { 0xf5 } else { 0xf4 });
    }

    fn text(&mut self, value: &str) -> Result<()> {
        if value.len() > MAX_TEXT_BYTES {
            return Err(InferError::ProtocolBound);
        }
        self.major(3, usize_to_u64(value.len())?)?;
        self.bytes.extend_from_slice(value.as_bytes());
        Ok(())
    }

    fn major(&mut self, major: u8, value: u64) -> Result<()> {
        let prefix = major.checked_shl(5).ok_or(InferError::ProtocolShape)?;
        match value {
            0..=23 => self
                .bytes
                .push(prefix | u8::try_from(value).map_err(|_| InferError::ProtocolShape)?),
            24..=0xff => {
                self.bytes.push(prefix | 24);
                self.bytes
                    .push(u8::try_from(value).map_err(|_| InferError::ProtocolShape)?);
            }
            0x100..=0xffff => {
                self.bytes.push(prefix | 25);
                self.bytes.extend_from_slice(
                    &u16::try_from(value)
                        .map_err(|_| InferError::ProtocolShape)?
                        .to_be_bytes(),
                );
            }
            0x1_0000..=0xffff_ffff => {
                self.bytes.push(prefix | 26);
                self.bytes.extend_from_slice(
                    &u32::try_from(value)
                        .map_err(|_| InferError::ProtocolShape)?
                        .to_be_bytes(),
                );
            }
            _ => {
                self.bytes.push(prefix | 27);
                self.bytes.extend_from_slice(&value.to_be_bytes());
            }
        }
        Ok(())
    }
}

struct Decoder<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Decoder<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn finish(&self) -> Result<()> {
        if self.offset == self.bytes.len() {
            Ok(())
        } else {
            Err(InferError::ProtocolTrailingData)
        }
    }

    fn require_version(&mut self) -> Result<()> {
        if self.uint()? == PROTOCOL_VERSION {
            Ok(())
        } else {
            Err(InferError::ProtocolVersion)
        }
    }

    fn array(&mut self) -> Result<usize> {
        let value = self.major(4)?;
        usize::try_from(value).map_err(|_| InferError::ProtocolBound)
    }

    fn uint(&mut self) -> Result<u64> {
        self.major(0)
    }

    fn u32(&mut self) -> Result<u32> {
        u32::try_from(self.uint()?).map_err(|_| InferError::ProtocolBound)
    }

    fn usize(&mut self) -> Result<usize> {
        usize::try_from(self.uint()?).map_err(|_| InferError::ProtocolBound)
    }

    fn boolean(&mut self) -> Result<bool> {
        match self.byte()? {
            0xf4 => Ok(false),
            0xf5 => Ok(true),
            _ => Err(InferError::ProtocolShape),
        }
    }

    fn text(&mut self, maximum: usize) -> Result<String> {
        let length = usize::try_from(self.major(3)?).map_err(|_| InferError::ProtocolBound)?;
        if length > maximum {
            return Err(InferError::ProtocolBound);
        }
        let end = self
            .offset
            .checked_add(length)
            .ok_or(InferError::ProtocolBound)?;
        let bytes = self
            .bytes
            .get(self.offset..end)
            .ok_or(InferError::ProtocolTruncated)?;
        self.offset = end;
        let text = std::str::from_utf8(bytes).map_err(|_| InferError::ProtocolUtf8)?;
        Ok(text.to_owned())
    }

    fn major(&mut self, expected_major: u8) -> Result<u64> {
        let head = self.byte()?;
        let major = head >> 5;
        if major != expected_major {
            return Err(InferError::ProtocolShape);
        }
        let additional = head & 0x1f;
        let value = match additional {
            0..=23 => u64::from(additional),
            24 => {
                let value = u64::from(self.byte()?);
                if value < 24 {
                    return Err(InferError::ProtocolNonCanonical);
                }
                value
            }
            25 => {
                let value = u64::from(u16::from_be_bytes(self.array_bytes()?));
                if value <= 0xff {
                    return Err(InferError::ProtocolNonCanonical);
                }
                value
            }
            26 => {
                let value = u64::from(u32::from_be_bytes(self.array_bytes()?));
                if value <= 0xffff {
                    return Err(InferError::ProtocolNonCanonical);
                }
                value
            }
            27 => {
                let value = u64::from_be_bytes(self.array_bytes()?);
                if value <= 0xffff_ffff {
                    return Err(InferError::ProtocolNonCanonical);
                }
                value
            }
            _ => return Err(InferError::ProtocolIndefinite),
        };
        Ok(value)
    }

    fn byte(&mut self) -> Result<u8> {
        let value = self
            .bytes
            .get(self.offset)
            .copied()
            .ok_or(InferError::ProtocolTruncated)?;
        self.offset += 1;
        Ok(value)
    }

    fn array_bytes<const N: usize>(&mut self) -> Result<[u8; N]> {
        let end = self
            .offset
            .checked_add(N)
            .ok_or(InferError::ProtocolBound)?;
        let slice = self
            .bytes
            .get(self.offset..end)
            .ok_or(InferError::ProtocolTruncated)?;
        self.offset = end;
        slice.try_into().map_err(|_| InferError::ProtocolTruncated)
    }
}

pub fn token_fence(message: &ClientMessage) -> Option<(EventFence<'_>, &Digest, u64)> {
    match message {
        ClientMessage::Token {
            request_id,
            request_generation,
            backend_generation,
            sequence,
            token_digest,
            token_byte_length,
        } => Some((
            EventFence {
                request_id,
                request_generation: *request_generation,
                backend_generation: *backend_generation,
                sequence: *sequence,
            },
            token_digest,
            *token_byte_length,
        )),
        _ => None,
    }
}
