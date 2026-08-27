//! Qualification-only artifact/source/build binding gate for a future Servo worker.
//!
//! This crate does not link or execute Servo. It proves a bounded parent-child
//! launch layer that binds the child executable bytes and embedded build/source
//! manifests before handing control to the browser protocol. All authority bits
//! remain negative.

#![forbid(unsafe_code)]

use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub const PROTOCOL_VERSION: u16 = 1;
pub const MAX_FRAME_BYTES: usize = 512;
pub const QUALIFICATION_ONLY_BIT: u16 = 1;
pub const KNOWN_AUTHORITY_BITS: u16 = QUALIFICATION_ONLY_BIT;

const MAGIC: &[u8; 8] = b"HPTALN01";
const HELLO: u8 = 1;
const HOST_ACK: u8 = 2;
const WORKER_CONFIRM: u8 = 3;
const PING: u8 = 10;
const PONG: u8 = 11;
const SHUTDOWN: u8 = 12;
const SHUTDOWN_ACK: u8 = 13;

#[derive(Debug)]
pub enum GateError {
    Invalid(&'static str),
    Io(std::io::Error),
    FrameTooLarge { length: usize, maximum: usize },
    AuthenticationFailed,
    ArtifactMismatch,
    DeadlineExceeded,
    ChildExit(String),
}

impl fmt::Display for GateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Invalid(message) => formatter.write_str(message),
            Self::Io(error) => write!(formatter, "artifact launch gate I/O failed: {error}"),
            Self::FrameTooLarge { length, maximum } => write!(
                formatter,
                "artifact launch gate frame length {length} exceeds maximum {maximum}"
            ),
            Self::AuthenticationFailed => {
                formatter.write_str("artifact launch gate challenge authentication failed")
            }
            Self::ArtifactMismatch => {
                formatter.write_str("worker artifact/build/source binding does not match")
            }
            Self::DeadlineExceeded => formatter.write_str("artifact launch gate deadline exceeded"),
            Self::ChildExit(status) => write!(formatter, "artifact-bound child exited: {status}"),
        }
    }
}

impl std::error::Error for GateError {}

impl From<std::io::Error> for GateError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Digest32([u8; 32]);

impl Digest32 {
    pub fn new(bytes: [u8; 32]) -> Result<Self, GateError> {
        if bytes.iter().all(|byte| *byte == 0) {
            return Err(GateError::Invalid("digest must not be all zero"));
        }
        Ok(Self(bytes))
    }

    pub fn from_hex(value: &str) -> Result<Self, GateError> {
        if value.len() != 64 {
            return Err(GateError::Invalid("digest hex must contain exactly 64 bytes"));
        }
        let mut output = [0_u8; 32];
        for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
            output[index] = (hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?;
        }
        Self::new(output)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn to_hex(self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut output = String::with_capacity(64);
        for byte in self.0 {
            output.push(char::from(HEX[usize::from(byte >> 4)]));
            output.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
        output
    }

    pub fn matches(self, other: Self) -> bool {
        constant_time_eq(&self.0, &other.0)
    }
}

impl fmt::Debug for Digest32 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_hex())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArtifactBinding {
    pub executable_sha256: Digest32,
    pub build_manifest_sha256: Digest32,
    pub source_receipt_sha256: Digest32,
}

impl ArtifactBinding {
    pub fn new(
        executable_sha256: Digest32,
        build_manifest_sha256: Digest32,
        source_receipt_sha256: Digest32,
    ) -> Self {
        Self {
            executable_sha256,
            build_manifest_sha256,
            source_receipt_sha256,
        }
    }

    pub fn matches(self, other: Self) -> bool {
        self.executable_sha256.matches(other.executable_sha256)
            & self
                .build_manifest_sha256
                .matches(other.build_manifest_sha256)
            & self.source_receipt_sha256.matches(other.source_receipt_sha256)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthorityPosture {
    pub qualification_only: bool,
    pub runtime_authority: bool,
    pub production_caller: bool,
    pub production_writer: bool,
    pub effect_authority: bool,
    pub external_effect: bool,
    pub external_network: bool,
    pub credential_export: bool,
    pub operator_acceptance: bool,
    pub promotion: bool,
    pub release_qualified: bool,
}

impl AuthorityPosture {
    pub const fn qualification_only() -> Self {
        Self {
            qualification_only: true,
            runtime_authority: false,
            production_caller: false,
            production_writer: false,
            effect_authority: false,
            external_effect: false,
            external_network: false,
            credential_export: false,
            operator_acceptance: false,
            promotion: false,
            release_qualified: false,
        }
    }

    pub fn wire_bits(self) -> Result<u16, GateError> {
        if self != Self::qualification_only() {
            return Err(GateError::Invalid(
                "artifact launch gate accepts only qualification-only authority",
            ));
        }
        Ok(QUALIFICATION_ONLY_BIT)
    }

    pub fn from_wire_bits(bits: u16) -> Result<Self, GateError> {
        if bits & !KNOWN_AUTHORITY_BITS != 0 || bits != QUALIFICATION_ONLY_BIT {
            return Err(GateError::Invalid(
                "artifact launch gate authority bits are unknown or open",
            ));
        }
        Ok(Self::qualification_only())
    }
}

impl Default for AuthorityPosture {
    fn default() -> Self {
        Self::qualification_only()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct LaunchHello {
    pub worker_pid: u32,
    pub binding: ArtifactBinding,
    pub authority: AuthorityPosture,
    challenge: [u8; 32],
}

impl LaunchHello {
    pub fn new(
        worker_pid: u32,
        binding: ArtifactBinding,
        challenge: [u8; 32],
    ) -> Result<Self, GateError> {
        if worker_pid == 0 || challenge.iter().all(|byte| *byte == 0) {
            return Err(GateError::Invalid("worker PID and challenge must be nonzero"));
        }
        Ok(Self {
            worker_pid,
            binding,
            authority: AuthorityPosture::qualification_only(),
            challenge,
        })
    }

    pub fn challenge_matches(&self, expected: &[u8; 32]) -> bool {
        constant_time_eq(&self.challenge, expected)
    }
}

impl fmt::Debug for LaunchHello {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LaunchHello")
            .field("worker_pid", &self.worker_pid)
            .field("binding", &self.binding)
            .field("authority", &self.authority)
            .field("challenge", &"<redacted>")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct HostAck {
    challenge: [u8; 32],
}

impl HostAck {
    pub fn new(challenge: [u8; 32]) -> Result<Self, GateError> {
        if challenge.iter().all(|byte| *byte == 0) {
            return Err(GateError::Invalid("host acknowledgement challenge must be nonzero"));
        }
        Ok(Self { challenge })
    }

    pub fn challenge_matches(&self, expected: &[u8; 32]) -> bool {
        constant_time_eq(&self.challenge, expected)
    }
}

impl fmt::Debug for HostAck {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HostAck")
            .field("challenge", &"<redacted>")
            .finish()
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct WorkerConfirm {
    challenge: [u8; 32],
}

impl WorkerConfirm {
    pub fn new(challenge: [u8; 32]) -> Result<Self, GateError> {
        if challenge.iter().all(|byte| *byte == 0) {
            return Err(GateError::Invalid("worker confirmation challenge must be nonzero"));
        }
        Ok(Self { challenge })
    }

    pub fn challenge_matches(&self, expected: &[u8; 32]) -> bool {
        constant_time_eq(&self.challenge, expected)
    }
}

impl fmt::Debug for WorkerConfirm {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorkerConfirm")
            .field("challenge", &"<redacted>")
            .finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    WorkerHello(LaunchHello),
    HostAck(HostAck),
    WorkerConfirm(WorkerConfirm),
    Ping,
    Pong,
    Shutdown,
    ShutdownAck,
}

pub fn hash_file(path: &Path) -> Result<Digest32, GateError> {
    let metadata = path.symlink_metadata()?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(GateError::Invalid(
            "worker executable must be a non-symlink regular file",
        ));
    }
    if metadata.len() == 0 {
        return Err(GateError::Invalid("worker executable must not be empty"));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if metadata.nlink() != 1 || metadata.mode() & 0o022 != 0 {
            return Err(GateError::Invalid(
                "worker executable must be single-link and not group/world writable",
            ));
        }
    }
    let mut file = File::open(path)?;
    hash_reader(&mut file)
}

pub fn binding_for_current_executable(
    build_manifest: &[u8],
    source_receipt: &[u8],
) -> Result<ArtifactBinding, GateError> {
    if build_manifest.is_empty() || source_receipt.is_empty() {
        return Err(GateError::Invalid(
            "embedded build manifest and source receipt must not be empty",
        ));
    }
    let executable = std::env::current_exe()?;
    Ok(ArtifactBinding::new(
        hash_file(&executable)?,
        Digest32::new(sha256(build_manifest))?,
        Digest32::new(sha256(source_receipt))?,
    ))
}

pub fn hash_reader(reader: &mut impl Read) -> Result<Digest32, GateError> {
    let mut state = Sha256::new();
    let mut buffer = [0_u8; 1024 * 64];
    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        state.update(&buffer[..read]);
    }
    Digest32::new(state.finalize())
}

pub fn encode_message(message: &Message) -> Result<Vec<u8>, GateError> {
    let mut output = Vec::with_capacity(160);
    output.extend_from_slice(MAGIC);
    output.extend_from_slice(&PROTOCOL_VERSION.to_be_bytes());
    match message {
        Message::WorkerHello(hello) => {
            output.push(HELLO);
            output.extend_from_slice(&hello.worker_pid.to_be_bytes());
            output.extend_from_slice(hello.binding.executable_sha256.as_bytes());
            output.extend_from_slice(hello.binding.build_manifest_sha256.as_bytes());
            output.extend_from_slice(hello.binding.source_receipt_sha256.as_bytes());
            output.extend_from_slice(&hello.authority.wire_bits()?.to_be_bytes());
            output.extend_from_slice(&hello.challenge);
        }
        Message::HostAck(ack) => {
            output.push(HOST_ACK);
            output.extend_from_slice(&ack.challenge);
        }
        Message::WorkerConfirm(confirm) => {
            output.push(WORKER_CONFIRM);
            output.extend_from_slice(&confirm.challenge);
        }
        Message::Ping => output.push(PING),
        Message::Pong => output.push(PONG),
        Message::Shutdown => output.push(SHUTDOWN),
        Message::ShutdownAck => output.push(SHUTDOWN_ACK),
    }
    if output.len() > MAX_FRAME_BYTES {
        return Err(GateError::FrameTooLarge {
            length: output.len(),
            maximum: MAX_FRAME_BYTES,
        });
    }
    Ok(output)
}

pub fn decode_message(bytes: &[u8]) -> Result<Message, GateError> {
    if bytes.is_empty() || bytes.len() > MAX_FRAME_BYTES {
        return Err(GateError::FrameTooLarge {
            length: bytes.len(),
            maximum: MAX_FRAME_BYTES,
        });
    }
    let mut decoder = Decoder::new(bytes);
    if decoder.array::<8>()? != *MAGIC {
        return Err(GateError::Invalid("artifact launch gate magic is invalid"));
    }
    if decoder.u16()? != PROTOCOL_VERSION {
        return Err(GateError::Invalid(
            "artifact launch gate protocol version is unsupported",
        ));
    }
    let message = match decoder.u8()? {
        HELLO => Message::WorkerHello(LaunchHello {
            worker_pid: decoder.u32()?,
            binding: ArtifactBinding::new(
                Digest32::new(decoder.array::<32>()?)?,
                Digest32::new(decoder.array::<32>()?)?,
                Digest32::new(decoder.array::<32>()?)?,
            ),
            authority: AuthorityPosture::from_wire_bits(decoder.u16()?)?,
            challenge: decoder.array::<32>()?,
        }),
        HOST_ACK => Message::HostAck(HostAck::new(decoder.array::<32>()?)?),
        WORKER_CONFIRM => Message::WorkerConfirm(WorkerConfirm::new(decoder.array::<32>()?)?),
        PING => Message::Ping,
        PONG => Message::Pong,
        SHUTDOWN => Message::Shutdown,
        SHUTDOWN_ACK => Message::ShutdownAck,
        _ => return Err(GateError::Invalid("artifact launch gate message kind is unknown")),
    };
    decoder.finish()?;
    if let Message::WorkerHello(hello) = &message {
        if hello.worker_pid == 0 || hello.challenge.iter().all(|byte| *byte == 0) {
            return Err(GateError::Invalid("worker hello contains zero identity material"));
        }
    }
    Ok(message)
}

pub fn write_message(writer: &mut impl Write, message: &Message) -> Result<(), GateError> {
    let bytes = encode_message(message)?;
    let length = u32::try_from(bytes.len())
        .map_err(|_| GateError::Invalid("artifact launch frame length does not fit u32"))?;
    writer.write_all(&length.to_be_bytes())?;
    writer.write_all(&bytes)?;
    writer.flush()?;
    Ok(())
}

pub fn read_message(reader: &mut impl Read) -> Result<Message, GateError> {
    let mut length_bytes = [0_u8; 4];
    reader.read_exact(&mut length_bytes)?;
    let length = u32::from_be_bytes(length_bytes) as usize;
    if length == 0 || length > MAX_FRAME_BYTES {
        return Err(GateError::FrameTooLarge {
            length,
            maximum: MAX_FRAME_BYTES,
        });
    }
    let mut bytes = vec![0_u8; length];
    reader.read_exact(&mut bytes)?;
    decode_message(&bytes)
}

pub fn validate_worker_hello(
    hello: &LaunchHello,
    expected_pid: u32,
    expected_binding: ArtifactBinding,
    challenge: &[u8; 32],
) -> Result<(), GateError> {
    if hello.worker_pid != expected_pid {
        return Err(GateError::AuthenticationFailed);
    }
    if !hello.binding.matches(expected_binding) {
        return Err(GateError::ArtifactMismatch);
    }
    if !hello.challenge_matches(challenge) {
        return Err(GateError::AuthenticationFailed);
    }
    if hello.authority != AuthorityPosture::qualification_only() {
        return Err(GateError::Invalid("worker hello authority posture is open"));
    }
    Ok(())
}

pub fn sha256(bytes: &[u8]) -> [u8; 32] {
    let mut state = Sha256::new();
    state.update(bytes);
    state.finalize()
}

struct Sha256 {
    state: [u32; 8],
    buffer: [u8; 64],
    buffer_len: usize,
    total_len: u64,
}

impl Sha256 {
    fn new() -> Self {
        Self {
            state: [
                0x6a09e667,
                0xbb67ae85,
                0x3c6ef372,
                0xa54ff53a,
                0x510e527f,
                0x9b05688c,
                0x1f83d9ab,
                0x5be0cd19,
            ],
            buffer: [0_u8; 64],
            buffer_len: 0,
            total_len: 0,
        }
    }

    fn update(&mut self, mut input: &[u8]) {
        self.total_len = self.total_len.wrapping_add(input.len() as u64);
        if self.buffer_len != 0 {
            let needed = 64 - self.buffer_len;
            let take = needed.min(input.len());
            self.buffer[self.buffer_len..self.buffer_len + take].copy_from_slice(&input[..take]);
            self.buffer_len += take;
            input = &input[take..];
            if self.buffer_len == 64 {
                let block = self.buffer;
                self.compress(&block);
                self.buffer_len = 0;
            }
        }
        while input.len() >= 64 {
            let mut block = [0_u8; 64];
            block.copy_from_slice(&input[..64]);
            self.compress(&block);
            input = &input[64..];
        }
        if !input.is_empty() {
            self.buffer[..input.len()].copy_from_slice(input);
            self.buffer_len = input.len();
        }
    }

    fn finalize(mut self) -> [u8; 32] {
        let bit_len = self.total_len.wrapping_mul(8);
        self.buffer[self.buffer_len] = 0x80;
        self.buffer_len += 1;
        if self.buffer_len > 56 {
            for byte in &mut self.buffer[self.buffer_len..] {
                *byte = 0;
            }
            let block = self.buffer;
            self.compress(&block);
            self.buffer = [0_u8; 64];
        } else {
            for byte in &mut self.buffer[self.buffer_len..56] {
                *byte = 0;
            }
        }
        self.buffer[56..64].copy_from_slice(&bit_len.to_be_bytes());
        let block = self.buffer;
        self.compress(&block);
        let mut output = [0_u8; 32];
        for (index, word) in self.state.iter().enumerate() {
            output[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        output
    }

    fn compress(&mut self, block: &[u8; 64]) {
        const K: [u32; 64] = [
            0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
            0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
            0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
            0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
            0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
            0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
            0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
            0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
            0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
            0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
            0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
        ];
        let mut schedule = [0_u32; 64];
        for (index, chunk) in block.chunks_exact(4).take(16).enumerate() {
            schedule[index] = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        for index in 16..64 {
            let s0 = schedule[index - 15].rotate_right(7)
                ^ schedule[index - 15].rotate_right(18)
                ^ (schedule[index - 15] >> 3);
            let s1 = schedule[index - 2].rotate_right(17)
                ^ schedule[index - 2].rotate_right(19)
                ^ (schedule[index - 2] >> 10);
            schedule[index] = schedule[index - 16]
                .wrapping_add(s0)
                .wrapping_add(schedule[index - 7])
                .wrapping_add(s1);
        }
        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];
        for index in 0..64 {
            let big1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choose = (e & f) ^ ((!e) & g);
            let temp1 = h
                .wrapping_add(big1)
                .wrapping_add(choose)
                .wrapping_add(K[index])
                .wrapping_add(schedule[index]);
            let big0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = big0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
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

    fn take(&mut self, length: usize) -> Result<&'a [u8], GateError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(GateError::Invalid("artifact launch gate frame offset overflowed"))?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(GateError::Invalid("artifact launch gate frame is truncated"))?;
        self.offset = end;
        Ok(value)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N], GateError> {
        let mut output = [0_u8; N];
        output.copy_from_slice(self.take(N)?);
        Ok(output)
    }

    fn u8(&mut self) -> Result<u8, GateError> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> Result<u16, GateError> {
        Ok(u16::from_be_bytes(self.array()?))
    }

    fn u32(&mut self) -> Result<u32, GateError> {
        Ok(u32::from_be_bytes(self.array()?))
    }

    fn finish(&self) -> Result<(), GateError> {
        if self.offset != self.bytes.len() {
            return Err(GateError::Invalid(
                "artifact launch gate frame contains trailing bytes",
            ));
        }
        Ok(())
    }
}

fn hex_nibble(value: u8) -> Result<u8, GateError> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => Err(GateError::Invalid("digest hex must be lowercase")),
    }
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut difference = 0_u8;
    for (left_byte, right_byte) in left.iter().zip(right) {
        difference |= left_byte ^ right_byte;
    }
    difference == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binding() -> Result<ArtifactBinding, GateError> {
        Ok(ArtifactBinding::new(
            Digest32::new([1_u8; 32])?,
            Digest32::new([2_u8; 32])?,
            Digest32::new([3_u8; 32])?,
        ))
    }

    #[test]
    fn sha256_known_vectors() -> Result<(), GateError> {
        assert_eq!(
            Digest32::new(sha256(b""))?.to_hex(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            Digest32::new(sha256(b"abc"))?.to_hex(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        Ok(())
    }

    #[test]
    fn hello_round_trip_binds_artifact_and_redacts_challenge() -> Result<(), GateError> {
        let challenge = [9_u8; 32];
        let hello = LaunchHello::new(42, binding()?, challenge)?;
        let message = Message::WorkerHello(hello.clone());
        assert_eq!(decode_message(&encode_message(&message)?)?, message);
        let debug = format!("{hello:?}");
        assert!(debug.contains("<redacted>"));
        assert!(!debug.contains(&"09".repeat(32)));
        Ok(())
    }

    #[test]
    fn unknown_authority_and_trailing_bytes_fail_closed() -> Result<(), GateError> {
        let hello = Message::WorkerHello(LaunchHello::new(42, binding()?, [7_u8; 32])?);
        let mut bytes = encode_message(&hello)?;
        let authority_offset = 8 + 2 + 1 + 4 + 32 + 32 + 32;
        bytes[authority_offset..authority_offset + 2]
            .copy_from_slice(&(QUALIFICATION_ONLY_BIT | 2).to_be_bytes());
        assert!(decode_message(&bytes).is_err());

        let mut trailing = encode_message(&Message::Ping)?;
        trailing.push(0);
        assert!(decode_message(&trailing).is_err());
        Ok(())
    }

    #[test]
    fn zero_digest_and_wrong_artifact_fail_closed() -> Result<(), GateError> {
        assert!(Digest32::new([0_u8; 32]).is_err());
        let expected = binding()?;
        let hello = LaunchHello::new(42, expected, [7_u8; 32])?;
        let other = ArtifactBinding::new(
            Digest32::new([4_u8; 32])?,
            expected.build_manifest_sha256,
            expected.source_receipt_sha256,
        );
        assert!(matches!(
            validate_worker_hello(&hello, 42, other, &[7_u8; 32]),
            Err(GateError::ArtifactMismatch)
        ));
        Ok(())
    }
}
