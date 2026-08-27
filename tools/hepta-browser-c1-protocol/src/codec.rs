use std::io::Read;
use std::io::Write;

use crate::MAX_DENIAL_CODE_BYTES;
use crate::MAX_FIXTURE_ID_BYTES;
use crate::MAX_FRAME_BYTES;
use crate::MAX_REFERENCE_BYTES;
use crate::MAX_TEXT_BYTES;
use crate::PROTOCOL_VERSION;
use crate::ProtocolError;
use crate::invalid;
use crate::protocol::AuthorityPosture;
use crate::protocol::BrowserSessionId;
use crate::protocol::CommandFrame;
use crate::protocol::CommandKind;
use crate::protocol::HostAck;
use crate::protocol::Message;
use crate::protocol::OutcomeFrame;
use crate::protocol::OutcomeStatus;
use crate::protocol::SourcePin;
use crate::protocol::StartupCapability;
use crate::protocol::WorkerConfirm;
use crate::protocol::WorkerHello;
use crate::protocol::WorkerIdentity;

const MAGIC: &[u8; 8] = b"HEPTABR1";

const WORKER_HELLO: u8 = 1;
const HOST_ACK: u8 = 2;
const WORKER_CONFIRM: u8 = 3;
const COMMAND: u8 = 10;
const OUTCOME: u8 = 11;

const COMMAND_PING: u8 = 1;
const COMMAND_NAVIGATE_LOCAL: u8 = 2;
const COMMAND_OBSERVE: u8 = 3;
const COMMAND_CLICK: u8 = 4;
const COMMAND_TYPE_TEXT: u8 = 5;
const COMMAND_HUMAN_TAKEOVER: u8 = 6;
const COMMAND_HUMAN_RELEASE: u8 = 7;
const COMMAND_SHUTDOWN: u8 = 8;

const OUTCOME_COMPLETED: u8 = 1;
const OUTCOME_DENIED: u8 = 2;
const OUTCOME_STALE: u8 = 3;
const OUTCOME_INVALID: u8 = 4;
const OUTCOME_INDETERMINATE: u8 = 5;

pub fn encode_message(message: &Message) -> Result<Vec<u8>, ProtocolError> {
    let mut encoder = Encoder::default();
    encoder.bytes(MAGIC);
    encoder.u16(PROTOCOL_VERSION);
    match message {
        Message::WorkerHello(hello) => {
            encoder.u8(WORKER_HELLO);
            encode_identity(&mut encoder, hello.identity);
            encoder.bytes(&hello.source_pin.servo_commit);
            encoder.bytes(&hello.source_pin.servo_tree);
            encoder.u16(hello.authority.wire_bits()?);
            encoder.bytes(&hello.startup_capability.0);
        }
        Message::HostAck(ack) => {
            encoder.u8(HOST_ACK);
            encode_identity(&mut encoder, ack.identity);
            encoder.u8(u8::from(ack.accepted));
            encoder.bytes(&ack.host_nonce);
        }
        Message::WorkerConfirm(confirm) => {
            encoder.u8(WORKER_CONFIRM);
            encode_identity(&mut encoder, confirm.identity);
            encoder.bytes(&confirm.host_nonce);
        }
        Message::Command(command) => {
            encoder.u8(COMMAND);
            encoder.u64(command.request_id);
            encode_identity(&mut encoder, command.identity);
            encoder.u64(command.page_revision);
            encode_command(&mut encoder, &command.command)?;
        }
        Message::Outcome(outcome) => {
            encoder.u8(OUTCOME);
            encoder.u64(outcome.request_id);
            encode_identity(&mut encoder, outcome.identity);
            encoder.u64(outcome.page_revision);
            encoder.u8(match outcome.status {
                OutcomeStatus::Completed => OUTCOME_COMPLETED,
                OutcomeStatus::Denied => OUTCOME_DENIED,
                OutcomeStatus::Stale => OUTCOME_STALE,
                OutcomeStatus::Invalid => OUTCOME_INVALID,
                OutcomeStatus::Indeterminate => OUTCOME_INDETERMINATE,
            });
            encoder.string(&outcome.code, MAX_DENIAL_CODE_BYTES)?;
        }
    }
    let bytes = encoder.finish();
    if bytes.len() > MAX_FRAME_BYTES {
        return Err(ProtocolError::FrameTooLarge {
            length: bytes.len(),
            maximum: MAX_FRAME_BYTES,
        });
    }
    Ok(bytes)
}

pub fn decode_message(bytes: &[u8]) -> Result<Message, ProtocolError> {
    if bytes.is_empty() || bytes.len() > MAX_FRAME_BYTES {
        return Err(ProtocolError::FrameTooLarge {
            length: bytes.len(),
            maximum: MAX_FRAME_BYTES,
        });
    }
    let mut decoder = Decoder::new(bytes);
    if decoder.take_array::<8>()? != *MAGIC {
        return Err(invalid("private worker protocol magic does not match"));
    }
    if decoder.u16()? != PROTOCOL_VERSION {
        return Err(invalid("private worker protocol version is unsupported"));
    }
    let message = match decoder.u8()? {
        WORKER_HELLO => {
            let identity = decode_identity(&mut decoder)?;
            let commit = decoder.take_array::<40>()?;
            let tree = decoder.take_array::<40>()?;
            let source_pin = SourcePin::new(
                std::str::from_utf8(&commit)?,
                std::str::from_utf8(&tree)?,
            )?;
            let authority = AuthorityPosture::from_wire_bits(decoder.u16()?)?;
            let capability = StartupCapability::new(decoder.take_array::<32>()?)?;
            let mut hello = WorkerHello::new(identity, source_pin, capability)?;
            hello.authority = authority;
            Message::WorkerHello(hello)
        }
        HOST_ACK => {
            let identity = decode_identity(&mut decoder)?;
            if decoder.u8()? != 1 {
                return Err(invalid("host acknowledgement must be an accepted binding"));
            }
            let host_nonce = decoder.take_array::<32>()?;
            Message::HostAck(HostAck::accepted(identity, host_nonce)?)
        }
        WORKER_CONFIRM => {
            let identity = decode_identity(&mut decoder)?;
            let host_nonce = decoder.take_array::<32>()?;
            Message::WorkerConfirm(WorkerConfirm::new(identity, host_nonce)?)
        }
        COMMAND => {
            let request_id = decoder.u64()?;
            let identity = decode_identity(&mut decoder)?;
            let page_revision = decoder.u64()?;
            let command = decode_command(&mut decoder)?;
            Message::Command(CommandFrame::new(
                request_id,
                identity,
                page_revision,
                command,
            )?)
        }
        OUTCOME => {
            let request_id = decoder.u64()?;
            let identity = decode_identity(&mut decoder)?;
            let page_revision = decoder.u64()?;
            let status = match decoder.u8()? {
                OUTCOME_COMPLETED => OutcomeStatus::Completed,
                OUTCOME_DENIED => OutcomeStatus::Denied,
                OUTCOME_STALE => OutcomeStatus::Stale,
                OUTCOME_INVALID => OutcomeStatus::Invalid,
                OUTCOME_INDETERMINATE => OutcomeStatus::Indeterminate,
                _ => return Err(invalid("worker outcome status is unknown")),
            };
            let code = decoder.string(MAX_DENIAL_CODE_BYTES)?;
            Message::Outcome(OutcomeFrame::new(
                request_id,
                identity,
                page_revision,
                status,
                code,
            )?)
        }
        _ => return Err(invalid("private worker message kind is unknown")),
    };
    decoder.finish()?;
    Ok(message)
}

pub fn write_message<W: Write>(writer: &mut W, message: &Message) -> Result<(), ProtocolError> {
    let bytes = encode_message(message)?;
    let length = u32::try_from(bytes.len())
        .map_err(|_| invalid("private worker frame length does not fit u32"))?;
    writer.write_all(&length.to_be_bytes())?;
    writer.write_all(&bytes)?;
    writer.flush()?;
    Ok(())
}

pub fn read_message<R: Read>(reader: &mut R) -> Result<Message, ProtocolError> {
    let mut encoded_length = [0_u8; 4];
    reader.read_exact(&mut encoded_length)?;
    let length = u32::from_be_bytes(encoded_length) as usize;
    if length == 0 || length > MAX_FRAME_BYTES {
        return Err(ProtocolError::FrameTooLarge {
            length,
            maximum: MAX_FRAME_BYTES,
        });
    }
    let mut bytes = vec![0_u8; length];
    reader.read_exact(&mut bytes)?;
    decode_message(&bytes)
}

fn encode_identity(encoder: &mut Encoder, identity: WorkerIdentity) {
    encoder.bytes(identity.session_id.as_bytes());
    encoder.u64(identity.generation);
    encoder.u64(identity.owner_epoch);
}

fn decode_identity(decoder: &mut Decoder<'_>) -> Result<WorkerIdentity, ProtocolError> {
    WorkerIdentity::new(
        BrowserSessionId::new(decoder.take_array::<32>()?)?,
        decoder.u64()?,
        decoder.u64()?,
    )
}

fn encode_command(encoder: &mut Encoder, command: &CommandKind) -> Result<(), ProtocolError> {
    command.validate()?;
    match command {
        CommandKind::Ping => encoder.u8(COMMAND_PING),
        CommandKind::NavigateLocal { fixture_id } => {
            encoder.u8(COMMAND_NAVIGATE_LOCAL);
            encoder.string(fixture_id, MAX_FIXTURE_ID_BYTES)?;
        }
        CommandKind::Observe { limit } => {
            encoder.u8(COMMAND_OBSERVE);
            encoder.u16(*limit);
        }
        CommandKind::Click { semantic_ref } => {
            encoder.u8(COMMAND_CLICK);
            encoder.string(semantic_ref, MAX_REFERENCE_BYTES)?;
        }
        CommandKind::TypeText {
            semantic_ref,
            text,
        } => {
            encoder.u8(COMMAND_TYPE_TEXT);
            encoder.string(semantic_ref, MAX_REFERENCE_BYTES)?;
            encoder.string(text, MAX_TEXT_BYTES)?;
        }
        CommandKind::HumanTakeover { lease_ms } => {
            encoder.u8(COMMAND_HUMAN_TAKEOVER);
            encoder.u32(*lease_ms);
        }
        CommandKind::HumanRelease => encoder.u8(COMMAND_HUMAN_RELEASE),
        CommandKind::Shutdown => encoder.u8(COMMAND_SHUTDOWN),
    }
    Ok(())
}

fn decode_command(decoder: &mut Decoder<'_>) -> Result<CommandKind, ProtocolError> {
    let command = match decoder.u8()? {
        COMMAND_PING => CommandKind::Ping,
        COMMAND_NAVIGATE_LOCAL => CommandKind::NavigateLocal {
            fixture_id: decoder.string(MAX_FIXTURE_ID_BYTES)?,
        },
        COMMAND_OBSERVE => CommandKind::Observe {
            limit: decoder.u16()?,
        },
        COMMAND_CLICK => CommandKind::Click {
            semantic_ref: decoder.string(MAX_REFERENCE_BYTES)?,
        },
        COMMAND_TYPE_TEXT => CommandKind::TypeText {
            semantic_ref: decoder.string(MAX_REFERENCE_BYTES)?,
            text: decoder.string(MAX_TEXT_BYTES)?,
        },
        COMMAND_HUMAN_TAKEOVER => CommandKind::HumanTakeover {
            lease_ms: decoder.u32()?,
        },
        COMMAND_HUMAN_RELEASE => CommandKind::HumanRelease,
        COMMAND_SHUTDOWN => CommandKind::Shutdown,
        _ => return Err(invalid("private worker command kind is unknown")),
    };
    command.validate()?;
    Ok(command)
}

#[derive(Default)]
struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    fn u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn bytes(&mut self, value: &[u8]) {
        self.bytes.extend_from_slice(value);
    }

    fn string(&mut self, value: &str, maximum: usize) -> Result<(), ProtocolError> {
        if value.is_empty() || value.len() > maximum {
            return Err(invalid("private worker string is empty or oversized"));
        }
        let length = u32::try_from(value.len())
            .map_err(|_| invalid("private worker string length does not fit u32"))?;
        self.u32(length);
        self.bytes(value.as_bytes());
        Ok(())
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
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

    fn take(&mut self, length: usize) -> Result<&'a [u8], ProtocolError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or_else(|| invalid("private worker frame offset overflowed"))?;
        let output = self
            .bytes
            .get(self.offset..end)
            .ok_or_else(|| invalid("private worker frame is truncated"))?;
        self.offset = end;
        Ok(output)
    }

    fn take_array<const N: usize>(&mut self) -> Result<[u8; N], ProtocolError> {
        let mut output = [0_u8; N];
        output.copy_from_slice(self.take(N)?);
        Ok(output)
    }

    fn u8(&mut self) -> Result<u8, ProtocolError> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> Result<u16, ProtocolError> {
        Ok(u16::from_be_bytes(self.take_array()?))
    }

    fn u32(&mut self) -> Result<u32, ProtocolError> {
        Ok(u32::from_be_bytes(self.take_array()?))
    }

    fn u64(&mut self) -> Result<u64, ProtocolError> {
        Ok(u64::from_be_bytes(self.take_array()?))
    }

    fn string(&mut self, maximum: usize) -> Result<String, ProtocolError> {
        let length = self.u32()? as usize;
        if length == 0 || length > maximum {
            return Err(invalid("private worker string length is outside its bound"));
        }
        Ok(std::str::from_utf8(self.take(length)?)?.to_owned())
    }

    fn finish(&self) -> Result<(), ProtocolError> {
        if self.offset != self.bytes.len() {
            return Err(invalid("private worker frame contains trailing bytes"));
        }
        Ok(())
    }
}
