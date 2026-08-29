use std::fmt;

use codex_hepta_infer_core::Digest;
use codex_hepta_infer_core::RequestId;
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

pub const PRIVATE_WORKER_PROTOCOL_VERSION: u16 = 1;
pub const MAX_PRIVATE_WORKER_FRAME_BYTES: usize = 64 * 1024;
const MAX_TEXT_BYTES: usize = 256;

const HELLO: u8 = 1;
const READY: u8 = 2;
const SUBMIT: u8 = 3;
const TOKEN: u8 = 4;
const COMPLETE: u8 = 5;
const CANCEL: u8 = 6;
const CANCEL_ACK: u8 = 7;
const FAILURE: u8 = 8;
const SHUTDOWN: u8 = 9;

pub type ProtocolResult<T> = std::result::Result<T, ProtocolError>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolError {
    Bound,
    InvalidKind,
    InvalidShape,
    InvalidText,
    InvalidVersion,
    Io,
    TrailingData,
}

impl ProtocolError {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Bound => "INF_WORKER_PROTOCOL_BOUND",
            Self::InvalidKind => "INF_WORKER_PROTOCOL_KIND",
            Self::InvalidShape => "INF_WORKER_PROTOCOL_SHAPE",
            Self::InvalidText => "INF_WORKER_PROTOCOL_TEXT",
            Self::InvalidVersion => "INF_WORKER_PROTOCOL_VERSION",
            Self::Io => "INF_WORKER_PROTOCOL_IO",
            Self::TrailingData => "INF_WORKER_PROTOCOL_TRAILING_DATA",
        }
    }
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for ProtocolError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkerFrame {
    Hello {
        backend_generation: u64,
        session_digest: Digest,
    },
    Ready {
        backend_generation: u64,
        session_digest: Digest,
    },
    Submit {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
        sequence: u64,
        grant_digest: Digest,
        prompt_digest: Digest,
        output_token_limit: u32,
    },
    Token {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
        sequence: u64,
        token_digest: Digest,
        token_bytes: u64,
    },
    Complete {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
        sequence: u64,
        result_digest: Digest,
        output_tokens: u32,
        fixture: bool,
    },
    Cancel {
        request_id: RequestId,
        request_generation: u64,
        cancel_generation: u64,
        backend_generation: u64,
    },
    CancelAck {
        request_id: RequestId,
        request_generation: u64,
        cancel_generation: u64,
        backend_generation: u64,
    },
    Failure {
        request_id: RequestId,
        request_generation: u64,
        backend_generation: u64,
        code: String,
        forced_worker_termination: bool,
    },
    Shutdown,
}

impl WorkerFrame {
    pub fn encode(&self) -> ProtocolResult<Vec<u8>> {
        let mut encoder = Encoder::new();
        encoder.u16(PRIVATE_WORKER_PROTOCOL_VERSION);
        match self {
            Self::Hello {
                backend_generation,
                session_digest,
            } => {
                encoder.u8(HELLO);
                encoder.generation(*backend_generation)?;
                encoder.digest(session_digest)?;
            }
            Self::Ready {
                backend_generation,
                session_digest,
            } => {
                encoder.u8(READY);
                encoder.generation(*backend_generation)?;
                encoder.digest(session_digest)?;
            }
            Self::Submit {
                request_id,
                request_generation,
                backend_generation,
                sequence,
                grant_digest,
                prompt_digest,
                output_token_limit,
            } => {
                encoder.u8(SUBMIT);
                encoder.request_id(request_id)?;
                encoder.generation(*request_generation)?;
                encoder.generation(*backend_generation)?;
                encoder.sequence(*sequence)?;
                encoder.digest(grant_digest)?;
                encoder.digest(prompt_digest)?;
                if *output_token_limit == 0 {
                    return Err(ProtocolError::InvalidShape);
                }
                encoder.u32(*output_token_limit);
            }
            Self::Token {
                request_id,
                request_generation,
                backend_generation,
                sequence,
                token_digest,
                token_bytes,
            } => {
                encoder.u8(TOKEN);
                encoder.request_id(request_id)?;
                encoder.generation(*request_generation)?;
                encoder.generation(*backend_generation)?;
                encoder.sequence(*sequence)?;
                encoder.digest(token_digest)?;
                if *token_bytes == 0 {
                    return Err(ProtocolError::InvalidShape);
                }
                encoder.u64(*token_bytes);
            }
            Self::Complete {
                request_id,
                request_generation,
                backend_generation,
                sequence,
                result_digest,
                output_tokens,
                fixture,
            } => {
                encoder.u8(COMPLETE);
                encoder.request_id(request_id)?;
                encoder.generation(*request_generation)?;
                encoder.generation(*backend_generation)?;
                encoder.sequence(*sequence)?;
                encoder.digest(result_digest)?;
                if *output_tokens == 0 {
                    return Err(ProtocolError::InvalidShape);
                }
                encoder.u32(*output_tokens);
                encoder.boolean(*fixture);
            }
            Self::Cancel {
                request_id,
                request_generation,
                cancel_generation,
                backend_generation,
            } => {
                encoder.u8(CANCEL);
                encoder.request_id(request_id)?;
                encoder.generation(*request_generation)?;
                encoder.generation(*cancel_generation)?;
                encoder.generation(*backend_generation)?;
            }
            Self::CancelAck {
                request_id,
                request_generation,
                cancel_generation,
                backend_generation,
            } => {
                encoder.u8(CANCEL_ACK);
                encoder.request_id(request_id)?;
                encoder.generation(*request_generation)?;
                encoder.generation(*cancel_generation)?;
                encoder.generation(*backend_generation)?;
            }
            Self::Failure {
                request_id,
                request_generation,
                backend_generation,
                code,
                forced_worker_termination,
            } => {
                encoder.u8(FAILURE);
                encoder.request_id(request_id)?;
                encoder.generation(*request_generation)?;
                encoder.generation(*backend_generation)?;
                if !valid_error_code(code) {
                    return Err(ProtocolError::InvalidText);
                }
                encoder.text(code)?;
                encoder.boolean(*forced_worker_termination);
            }
            Self::Shutdown => encoder.u8(SHUTDOWN),
        }
        let bytes = encoder.finish();
        if bytes.is_empty() || bytes.len() > MAX_PRIVATE_WORKER_FRAME_BYTES {
            return Err(ProtocolError::Bound);
        }
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> ProtocolResult<Self> {
        if bytes.is_empty() || bytes.len() > MAX_PRIVATE_WORKER_FRAME_BYTES {
            return Err(ProtocolError::Bound);
        }
        let mut decoder = Decoder::new(bytes);
        if decoder.u16()? != PRIVATE_WORKER_PROTOCOL_VERSION {
            return Err(ProtocolError::InvalidVersion);
        }
        let frame = match decoder.u8()? {
            HELLO => Self::Hello {
                backend_generation: decoder.generation()?,
                session_digest: decoder.digest()?,
            },
            READY => Self::Ready {
                backend_generation: decoder.generation()?,
                session_digest: decoder.digest()?,
            },
            SUBMIT => {
                let request_id = decoder.request_id()?;
                let request_generation = decoder.generation()?;
                let backend_generation = decoder.generation()?;
                let sequence = decoder.sequence()?;
                let grant_digest = decoder.digest()?;
                let prompt_digest = decoder.digest()?;
                let output_token_limit = decoder.u32()?;
                if output_token_limit == 0 {
                    return Err(ProtocolError::InvalidShape);
                }
                Self::Submit {
                    request_id,
                    request_generation,
                    backend_generation,
                    sequence,
                    grant_digest,
                    prompt_digest,
                    output_token_limit,
                }
            }
            TOKEN => {
                let request_id = decoder.request_id()?;
                let request_generation = decoder.generation()?;
                let backend_generation = decoder.generation()?;
                let sequence = decoder.sequence()?;
                let token_digest = decoder.digest()?;
                let token_bytes = decoder.u64()?;
                if token_bytes == 0 {
                    return Err(ProtocolError::InvalidShape);
                }
                Self::Token {
                    request_id,
                    request_generation,
                    backend_generation,
                    sequence,
                    token_digest,
                    token_bytes,
                }
            }
            COMPLETE => {
                let request_id = decoder.request_id()?;
                let request_generation = decoder.generation()?;
                let backend_generation = decoder.generation()?;
                let sequence = decoder.sequence()?;
                let result_digest = decoder.digest()?;
                let output_tokens = decoder.u32()?;
                let fixture = decoder.boolean()?;
                if output_tokens == 0 {
                    return Err(ProtocolError::InvalidShape);
                }
                Self::Complete {
                    request_id,
                    request_generation,
                    backend_generation,
                    sequence,
                    result_digest,
                    output_tokens,
                    fixture,
                }
            }
            CANCEL => Self::Cancel {
                request_id: decoder.request_id()?,
                request_generation: decoder.generation()?,
                cancel_generation: decoder.generation()?,
                backend_generation: decoder.generation()?,
            },
            CANCEL_ACK => Self::CancelAck {
                request_id: decoder.request_id()?,
                request_generation: decoder.generation()?,
                cancel_generation: decoder.generation()?,
                backend_generation: decoder.generation()?,
            },
            FAILURE => {
                let request_id = decoder.request_id()?;
                let request_generation = decoder.generation()?;
                let backend_generation = decoder.generation()?;
                let code = decoder.text()?;
                let forced_worker_termination = decoder.boolean()?;
                if !valid_error_code(&code) {
                    return Err(ProtocolError::InvalidText);
                }
                Self::Failure {
                    request_id,
                    request_generation,
                    backend_generation,
                    code,
                    forced_worker_termination,
                }
            }
            SHUTDOWN => Self::Shutdown,
            _ => return Err(ProtocolError::InvalidKind),
        };
        decoder.finish()?;
        Ok(frame)
    }
}

pub async fn read_frame<R>(reader: &mut R, max_frame_bytes: usize) -> ProtocolResult<WorkerFrame>
where
    R: AsyncRead + Unpin,
{
    if max_frame_bytes == 0 || max_frame_bytes > MAX_PRIVATE_WORKER_FRAME_BYTES {
        return Err(ProtocolError::Bound);
    }
    let mut length_bytes = [0u8; 4];
    reader
        .read_exact(&mut length_bytes)
        .await
        .map_err(|_| ProtocolError::Io)?;
    let length = usize::try_from(u32::from_be_bytes(length_bytes))
        .map_err(|_| ProtocolError::Bound)?;
    if length == 0 || length > max_frame_bytes {
        return Err(ProtocolError::Bound);
    }
    let mut bytes = vec![0u8; length];
    reader
        .read_exact(&mut bytes)
        .await
        .map_err(|_| ProtocolError::Io)?;
    WorkerFrame::decode(&bytes)
}

pub async fn write_frame<W>(
    writer: &mut W,
    frame: &WorkerFrame,
    max_frame_bytes: usize,
) -> ProtocolResult<()>
where
    W: AsyncWrite + Unpin,
{
    if max_frame_bytes == 0 || max_frame_bytes > MAX_PRIVATE_WORKER_FRAME_BYTES {
        return Err(ProtocolError::Bound);
    }
    let bytes = frame.encode()?;
    if bytes.len() > max_frame_bytes {
        return Err(ProtocolError::Bound);
    }
    let length = u32::try_from(bytes.len()).map_err(|_| ProtocolError::Bound)?;
    writer
        .write_all(&length.to_be_bytes())
        .await
        .map_err(|_| ProtocolError::Io)?;
    writer
        .write_all(&bytes)
        .await
        .map_err(|_| ProtocolError::Io)?;
    writer.flush().await.map_err(|_| ProtocolError::Io)
}

struct Encoder {
    bytes: Vec<u8>,
}

impl Encoder {
    fn new() -> Self {
        Self {
            bytes: Vec::with_capacity(512),
        }
    }

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

    fn generation(&mut self, value: u64) -> ProtocolResult<()> {
        if value == 0 {
            return Err(ProtocolError::InvalidShape);
        }
        self.u64(value);
        Ok(())
    }

    fn sequence(&mut self, value: u64) -> ProtocolResult<()> {
        self.generation(value)
    }

    fn boolean(&mut self, value: bool) {
        self.u8(u8::from(value));
    }

    fn text(&mut self, value: &str) -> ProtocolResult<()> {
        if value.is_empty() || value.len() > MAX_TEXT_BYTES {
            return Err(ProtocolError::InvalidText);
        }
        let length = u16::try_from(value.len()).map_err(|_| ProtocolError::Bound)?;
        self.u16(length);
        self.bytes.extend_from_slice(value.as_bytes());
        Ok(())
    }

    fn request_id(&mut self, value: &RequestId) -> ProtocolResult<()> {
        self.text(value.as_str())
    }

    fn digest(&mut self, value: &Digest) -> ProtocolResult<()> {
        self.text(value.as_str())
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

struct Decoder<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> Decoder<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    fn take<const N: usize>(&mut self) -> ProtocolResult<[u8; N]> {
        let end = self
            .cursor
            .checked_add(N)
            .ok_or(ProtocolError::Bound)?;
        let bytes = self
            .bytes
            .get(self.cursor..end)
            .ok_or(ProtocolError::InvalidShape)?;
        self.cursor = end;
        bytes.try_into().map_err(|_| ProtocolError::InvalidShape)
    }

    fn u8(&mut self) -> ProtocolResult<u8> {
        Ok(self.take::<1>()?[0])
    }

    fn u16(&mut self) -> ProtocolResult<u16> {
        Ok(u16::from_be_bytes(self.take()?))
    }

    fn u32(&mut self) -> ProtocolResult<u32> {
        Ok(u32::from_be_bytes(self.take()?))
    }

    fn u64(&mut self) -> ProtocolResult<u64> {
        Ok(u64::from_be_bytes(self.take()?))
    }

    fn generation(&mut self) -> ProtocolResult<u64> {
        let value = self.u64()?;
        if value == 0 {
            return Err(ProtocolError::InvalidShape);
        }
        Ok(value)
    }

    fn sequence(&mut self) -> ProtocolResult<u64> {
        self.generation()
    }

    fn boolean(&mut self) -> ProtocolResult<bool> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ProtocolError::InvalidShape),
        }
    }

    fn text(&mut self) -> ProtocolResult<String> {
        let length = usize::from(self.u16()?);
        if length == 0 || length > MAX_TEXT_BYTES {
            return Err(ProtocolError::InvalidText);
        }
        let end = self
            .cursor
            .checked_add(length)
            .ok_or(ProtocolError::Bound)?;
        let bytes = self
            .bytes
            .get(self.cursor..end)
            .ok_or(ProtocolError::InvalidShape)?;
        self.cursor = end;
        std::str::from_utf8(bytes)
            .map(str::to_owned)
            .map_err(|_| ProtocolError::InvalidText)
    }

    fn request_id(&mut self) -> ProtocolResult<RequestId> {
        RequestId::parse(&self.text()?).map_err(|_| ProtocolError::InvalidText)
    }

    fn digest(&mut self) -> ProtocolResult<Digest> {
        Digest::parse(&self.text()?).map_err(|_| ProtocolError::InvalidText)
    }

    fn finish(self) -> ProtocolResult<()> {
        if self.cursor == self.bytes.len() {
            Ok(())
        } else {
            Err(ProtocolError::TrailingData)
        }
    }
}

fn valid_error_code(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_TEXT_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'_')
}

#[cfg(test)]
mod tests {
    use super::*;

    fn must<T, E: fmt::Display>(result: std::result::Result<T, E>) -> T {
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

    #[test]
    fn canonical_round_trip_preserves_private_fences() {
        let frame = WorkerFrame::Submit {
            request_id: must(RequestId::parse("request-private-frame")),
            request_generation: 2,
            backend_generation: 7,
            sequence: 3,
            grant_digest: digest('a'),
            prompt_digest: digest('b'),
            output_token_limit: 8,
        };
        let bytes = must(frame.encode());
        assert_eq!(must(WorkerFrame::decode(&bytes)), frame);
    }

    #[test]
    fn unknown_version_kind_and_trailing_data_fail_closed() {
        let mut version = must(WorkerFrame::Shutdown.encode());
        version[1] = 2;
        assert_eq!(
            WorkerFrame::decode(&version),
            Err(ProtocolError::InvalidVersion)
        );
        let mut kind = must(WorkerFrame::Shutdown.encode());
        kind[2] = 255;
        assert_eq!(WorkerFrame::decode(&kind), Err(ProtocolError::InvalidKind));
        let mut trailing = must(WorkerFrame::Shutdown.encode());
        trailing.push(0);
        assert_eq!(
            WorkerFrame::decode(&trailing),
            Err(ProtocolError::TrailingData)
        );
    }
}
