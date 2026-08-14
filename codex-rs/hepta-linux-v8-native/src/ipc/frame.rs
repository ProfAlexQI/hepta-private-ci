use super::NativeIpcErrorV8;
use super::NativeIpcResultV8;
use super::SeqpacketConnectionV8;
use super::VerifiedPeerV8;
use std::fs::File;

#[cfg(target_os = "linux")]
use super::ipc_io;
#[cfg(not(target_os = "linux"))]
use super::ipc_unsupported;
#[cfg(target_os = "linux")]
use std::os::fd::AsRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::FromRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::OwnedFd;

pub const MAX_SEQPACKET_PAYLOAD_BYTES_V8: usize = 64 * 1024;
pub const MAX_SEQPACKET_FILE_DESCRIPTORS_V8: usize = 8;
#[cfg(target_os = "linux")]
const FRAME_MAGIC_V8: &[u8] = b"hepta-linux-v8-seqpacket-frame-v1\0";
#[cfg(target_os = "linux")]
const FIRST_AND_ONLY_SEQUENCE_V8: u64 = 1;
#[cfg(target_os = "linux")]
const FRAME_HEADER_BYTES_V8: usize = FRAME_MAGIC_V8.len() + 8 + 8;
#[cfg(target_os = "linux")]
const MAX_WIRE_FRAME_BYTES_V8: usize = FRAME_HEADER_BYTES_V8 + MAX_SEQPACKET_PAYLOAD_BYTES_V8;

/// Compile-time-bounded exact ancillary FD expectation for one request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExactFileDescriptorCountV8 {
    count: usize,
}

impl ExactFileDescriptorCountV8 {
    pub fn new(count: usize) -> NativeIpcResultV8<Self> {
        if count > MAX_SEQPACKET_FILE_DESCRIPTORS_V8 {
            return Err(NativeIpcErrorV8::InvalidInput(format!(
                "expected FD count {count} exceeds maximum {MAX_SEQPACKET_FILE_DESCRIPTORS_V8}"
            )));
        }
        Ok(Self { count })
    }

    pub fn count(self) -> usize {
        self.count
    }
}

/// A close-on-exec descriptor received in a fully verified packet.
#[derive(Debug)]
pub struct ReceivedFileDescriptorV8 {
    #[cfg(target_os = "linux")]
    descriptor: OwnedFd,
    _private: (),
}

impl ReceivedFileDescriptorV8 {
    pub fn into_file(self) -> NativeIpcResultV8<File> {
        into_file_impl(self)
    }
}

/// One complete packet with exact framing, peer credentials, and ancillary FD
/// count. This token cannot be deserialized or directly constructed.
#[derive(Debug)]
pub struct VerifiedPacketV8 {
    payload: Vec<u8>,
    peer: VerifiedPeerV8,
    file_descriptors: Vec<ReceivedFileDescriptorV8>,
    _private: (),
}

impl VerifiedPacketV8 {
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn peer(&self) -> &VerifiedPeerV8 {
        &self.peer
    }

    pub fn file_descriptor_count(&self) -> usize {
        self.file_descriptors.len()
    }

    pub fn into_parts(self) -> (Vec<u8>, VerifiedPeerV8, Vec<ReceivedFileDescriptorV8>) {
        (self.payload, self.peer, self.file_descriptors)
    }
}

impl SeqpacketConnectionV8 {
    /// Sends the first and only request on this connection. The consuming API
    /// makes a second packet impossible through the verified transport.
    pub fn send_one_request(
        self,
        payload: &[u8],
        file_descriptors: &[&File],
    ) -> NativeIpcResultV8<()> {
        send_one_request_impl(self, payload, file_descriptors)
    }

    /// Receives the first and only request, requiring the exact ancillary FD
    /// count supplied by the frozen request profile.
    pub fn receive_one_request(
        self,
        expected_file_descriptors: ExactFileDescriptorCountV8,
    ) -> NativeIpcResultV8<VerifiedPacketV8> {
        receive_one_request_impl(self, expected_file_descriptors)
    }
}

#[cfg(target_os = "linux")]
fn send_one_request_impl(
    connection: SeqpacketConnectionV8,
    payload: &[u8],
    file_descriptors: &[&File],
) -> NativeIpcResultV8<()> {
    if file_descriptors.len() > MAX_SEQPACKET_FILE_DESCRIPTORS_V8 {
        return Err(NativeIpcErrorV8::InvalidInput(format!(
            "sent FD count {} exceeds maximum {MAX_SEQPACKET_FILE_DESCRIPTORS_V8}",
            file_descriptors.len()
        )));
    }
    let wire = encode_frame(payload)?;
    let (descriptor, peer) = connection.into_verified_parts();
    peer.verify_live()?;
    let mut io_vector = libc::iovec {
        iov_base: wire.as_ptr().cast_mut().cast(),
        iov_len: wire.len(),
    };
    // SAFETY: zero is a valid empty msghdr representation; all pointer-bearing
    // fields are populated below before sendmsg.
    let mut message: libc::msghdr = unsafe { std::mem::zeroed() };
    message.msg_iov = &mut io_vector;
    message.msg_iovlen = 1;

    let mut control = if file_descriptors.is_empty() {
        Vec::new()
    } else {
        aligned_control_buffer(cmsg_space(
            file_descriptors.len() * std::mem::size_of::<libc::c_int>(),
        )?)
    };
    if !control.is_empty() {
        message.msg_control = control.as_mut_ptr().cast();
        message.msg_controllen = control.len() * std::mem::size_of::<usize>();
        // SAFETY: msg_control is aligned and sized for one SCM_RIGHTS cmsg.
        let header = unsafe { libc::CMSG_FIRSTHDR(&message) };
        if header.is_null() {
            return Err(NativeIpcErrorV8::ProtocolViolation(
                "failed to allocate SCM_RIGHTS header".to_string(),
            ));
        }
        let data_bytes = file_descriptors.len() * std::mem::size_of::<libc::c_int>();
        // SAFETY: header points inside the live aligned control buffer.
        unsafe {
            (*header).cmsg_level = libc::SOL_SOCKET;
            (*header).cmsg_type = libc::SCM_RIGHTS;
            (*header).cmsg_len = cmsg_len(data_bytes)?;
            let data = libc::CMSG_DATA(header).cast::<libc::c_int>();
            for (index, descriptor) in file_descriptors.iter().enumerate() {
                std::ptr::write_unaligned(data.add(index), descriptor.as_raw_fd());
            }
        }
    }

    // Keep the kernel-identified peer token alive and revalidate it immediately
    // before the only transport side effect.
    peer.verify_live()?;
    // SAFETY: all msghdr buffers remain live and sendmsg retains no pointer.
    let sent = unsafe { libc::sendmsg(descriptor.as_raw_fd(), &message, libc::MSG_NOSIGNAL) };
    if sent < 0 {
        return Err(ipc_io(
            "sendmsg verified seqpacket request",
            std::io::Error::last_os_error(),
        ));
    }
    if usize::try_from(sent).ok() != Some(wire.len()) {
        return Err(NativeIpcErrorV8::ProtocolViolation(
            "seqpacket send was not an exact whole frame".to_string(),
        ));
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn send_one_request_impl(
    _connection: SeqpacketConnectionV8,
    _payload: &[u8],
    _file_descriptors: &[&File],
) -> NativeIpcResultV8<()> {
    Err(ipc_unsupported("sendmsg AF_UNIX SOCK_SEQPACKET"))
}

#[cfg(target_os = "linux")]
fn receive_one_request_impl(
    connection: SeqpacketConnectionV8,
    expected_file_descriptors: ExactFileDescriptorCountV8,
) -> NativeIpcResultV8<VerifiedPacketV8> {
    let (descriptor, peer) = connection.into_verified_parts();
    peer.verify_live()?;
    let mut wire = vec![0_u8; MAX_WIRE_FRAME_BYTES_V8];
    let mut io_vector = libc::iovec {
        iov_base: wire.as_mut_ptr().cast(),
        iov_len: wire.len(),
    };
    let credential_space = cmsg_space(std::mem::size_of::<libc::ucred>())?;
    // Always reserve the full frozen descriptor ceiling, even when the
    // profile expects zero. This lets us take ownership of every installed
    // unexpected descriptor before rejecting the packet.
    let rights_space =
        cmsg_space(MAX_SEQPACKET_FILE_DESCRIPTORS_V8 * std::mem::size_of::<libc::c_int>())?;
    let mut control = aligned_control_buffer(
        credential_space
            .checked_add(rights_space)
            .ok_or_else(|| NativeIpcErrorV8::InvalidInput("control size overflow".to_string()))?,
    );
    // SAFETY: zero is a valid empty msghdr representation; buffers are filled
    // before recvmsg.
    let mut message: libc::msghdr = unsafe { std::mem::zeroed() };
    message.msg_iov = &mut io_vector;
    message.msg_iovlen = 1;
    message.msg_control = control.as_mut_ptr().cast();
    message.msg_controllen = control.len() * std::mem::size_of::<usize>();

    // SAFETY: all receive buffers are writable and live. MSG_CMSG_CLOEXEC
    // atomically marks every received descriptor close-on-exec.
    let received =
        unsafe { libc::recvmsg(descriptor.as_raw_fd(), &mut message, libc::MSG_CMSG_CLOEXEC) };
    if received < 0 {
        return Err(ipc_io(
            "recvmsg verified seqpacket request",
            std::io::Error::last_os_error(),
        ));
    }
    if received == 0 {
        return Err(NativeIpcErrorV8::ProtocolViolation(
            "seqpacket peer closed without one request".to_string(),
        ));
    }

    // Parse ancillary descriptors before inspecting truncation flags so every
    // descriptor delivered into our process is owned and closed on rejection.
    let received_files = parse_control_messages(&message, &peer)?;
    if message.msg_flags & libc::MSG_TRUNC != 0 {
        return Err(NativeIpcErrorV8::ProtocolViolation(
            "seqpacket payload was truncated".to_string(),
        ));
    }
    if message.msg_flags & libc::MSG_CTRUNC != 0 {
        return Err(NativeIpcErrorV8::ProtocolViolation(
            "seqpacket ancillary data was truncated".to_string(),
        ));
    }
    if received_files.len() != expected_file_descriptors.count() {
        return Err(NativeIpcErrorV8::ProtocolViolation(format!(
            "received {} file descriptors, expected {}",
            received_files.len(),
            expected_file_descriptors.count()
        )));
    }
    let received = usize::try_from(received).map_err(|_| {
        NativeIpcErrorV8::ProtocolViolation("recvmsg returned an invalid size".to_string())
    })?;
    wire.truncate(received);
    let payload = decode_frame(&wire)?;
    // The peer pidfd must still represent a live process after the complete
    // packet and ancillary set have been received and verified.
    peer.verify_live()?;
    reject_queued_nonempty_packet(descriptor.as_raw_fd())?;
    peer.verify_live()?;
    Ok(VerifiedPacketV8 {
        payload,
        peer,
        file_descriptors: received_files,
        _private: (),
    })
}

#[cfg(not(target_os = "linux"))]
fn receive_one_request_impl(
    _connection: SeqpacketConnectionV8,
    _expected_file_descriptors: ExactFileDescriptorCountV8,
) -> NativeIpcResultV8<VerifiedPacketV8> {
    Err(ipc_unsupported("recvmsg AF_UNIX SOCK_SEQPACKET"))
}

#[cfg(target_os = "linux")]
fn encode_frame(payload: &[u8]) -> NativeIpcResultV8<Vec<u8>> {
    if payload.is_empty() || payload.len() > MAX_SEQPACKET_PAYLOAD_BYTES_V8 {
        return Err(NativeIpcErrorV8::InvalidInput(format!(
            "seqpacket payload size {} is outside 1..={MAX_SEQPACKET_PAYLOAD_BYTES_V8}",
            payload.len()
        )));
    }
    let payload_length = u64::try_from(payload.len()).map_err(|_| {
        NativeIpcErrorV8::InvalidInput("seqpacket payload length exceeds u64".to_string())
    })?;
    let mut wire = Vec::with_capacity(FRAME_HEADER_BYTES_V8 + payload.len());
    wire.extend_from_slice(FRAME_MAGIC_V8);
    wire.extend_from_slice(&FIRST_AND_ONLY_SEQUENCE_V8.to_be_bytes());
    wire.extend_from_slice(&payload_length.to_be_bytes());
    wire.extend_from_slice(payload);
    Ok(wire)
}

#[cfg(target_os = "linux")]
fn decode_frame(wire: &[u8]) -> NativeIpcResultV8<Vec<u8>> {
    if wire.len() < FRAME_HEADER_BYTES_V8 || &wire[..FRAME_MAGIC_V8.len()] != FRAME_MAGIC_V8 {
        return Err(NativeIpcErrorV8::ProtocolViolation(
            "seqpacket frame magic or header is invalid".to_string(),
        ));
    }
    let sequence_offset = FRAME_MAGIC_V8.len();
    let sequence = u64::from_be_bytes(
        wire[sequence_offset..sequence_offset + 8]
            .try_into()
            .map_err(|_| NativeIpcErrorV8::ProtocolViolation("missing sequence".to_string()))?,
    );
    if sequence != FIRST_AND_ONLY_SEQUENCE_V8 {
        return Err(NativeIpcErrorV8::ProtocolViolation(format!(
            "seqpacket sequence is {sequence}, expected {FIRST_AND_ONLY_SEQUENCE_V8}"
        )));
    }
    let length_offset = sequence_offset + 8;
    let payload_length = u64::from_be_bytes(
        wire[length_offset..length_offset + 8]
            .try_into()
            .map_err(|_| {
                NativeIpcErrorV8::ProtocolViolation("missing payload length".to_string())
            })?,
    );
    let payload_length = usize::try_from(payload_length).map_err(|_| {
        NativeIpcErrorV8::ProtocolViolation("payload length exceeds usize".to_string())
    })?;
    if payload_length == 0 || payload_length > MAX_SEQPACKET_PAYLOAD_BYTES_V8 {
        return Err(NativeIpcErrorV8::ProtocolViolation(
            "seqpacket payload length is outside the fixed bounds".to_string(),
        ));
    }
    if FRAME_HEADER_BYTES_V8.checked_add(payload_length) != Some(wire.len()) {
        return Err(NativeIpcErrorV8::ProtocolViolation(
            "seqpacket payload length is not exact".to_string(),
        ));
    }
    Ok(wire[FRAME_HEADER_BYTES_V8..].to_vec())
}

#[cfg(target_os = "linux")]
fn parse_control_messages(
    message: &libc::msghdr,
    peer: &VerifiedPeerV8,
) -> NativeIpcResultV8<Vec<ReceivedFileDescriptorV8>> {
    let mut credentials_seen = 0_usize;
    let mut rights_messages_seen = 0_usize;
    let mut received = Vec::new();
    let mut violation = None;
    // SAFETY: message points to the live control buffer populated by recvmsg.
    let mut header = unsafe { libc::CMSG_FIRSTHDR(message) };
    while !header.is_null() {
        // SAFETY: CMSG_FIRSTHDR/NXTHDR return headers within msg_control.
        let level = unsafe { (*header).cmsg_level };
        // SAFETY: same bounded header as above.
        let kind = unsafe { (*header).cmsg_type };
        if level == libc::SOL_SOCKET && kind == libc::SCM_CREDENTIALS {
            credentials_seen += 1;
            match read_credentials(message, header) {
                Ok(credentials)
                    if credentials.pid == peer.pid() as libc::pid_t
                        && credentials.uid == peer.uid()
                        && credentials.gid == peer.gid() => {}
                Ok(_) => {
                    violation.get_or_insert_with(|| {
                        "packet SCM_CREDENTIALS differs from initial SO_PEERCRED".to_string()
                    });
                }
                Err(error) => {
                    violation.get_or_insert(error);
                }
            }
        } else if level == libc::SOL_SOCKET && kind == libc::SCM_RIGHTS {
            rights_messages_seen += 1;
            let parsed = read_rights(message, header);
            received.extend(parsed.descriptors);
            if let Some(error) = parsed.violation {
                violation.get_or_insert(error);
            }
        } else {
            violation.get_or_insert_with(|| {
                format!("unexpected ancillary message level={level} type={kind}")
            });
        }
        // SAFETY: message and current header are from the same live control
        // buffer; libc validates bounds before returning the next header.
        header = unsafe { libc::CMSG_NXTHDR(message, header) };
    }
    if credentials_seen != 1 {
        violation.get_or_insert_with(|| {
            format!("packet carried {credentials_seen} SCM_CREDENTIALS records, expected one")
        });
    }
    if rights_messages_seen > 1 {
        violation.get_or_insert_with(|| {
            format!(
                "packet carried {rights_messages_seen} SCM_RIGHTS records, expected at most one"
            )
        });
    }
    if let Some(message) = violation {
        return Err(NativeIpcErrorV8::ProtocolViolation(message));
    }
    Ok(received)
}

#[cfg(target_os = "linux")]
fn read_credentials(
    message: &libc::msghdr,
    header: *mut libc::cmsghdr,
) -> Result<libc::ucred, String> {
    let expected =
        cmsg_len(std::mem::size_of::<libc::ucred>()).map_err(|error| error.to_string())?;
    let control_start = message.msg_control as usize;
    let control_end = control_start
        .checked_add(message.msg_controllen)
        .ok_or_else(|| "ancillary control range overflowed".to_string())?;
    let header_start = header as usize;
    let header_end = header_start
        .checked_add(std::mem::size_of::<libc::cmsghdr>())
        .ok_or_else(|| "SCM_CREDENTIALS header range overflowed".to_string())?;
    if header_start < control_start || header_end > control_end {
        return Err("SCM_CREDENTIALS header lies outside ancillary storage".to_string());
    }
    // SAFETY: the complete cmsghdr lies within the live control buffer.
    let declared_length = unsafe { (*header).cmsg_len };
    if declared_length != expected {
        return Err("SCM_CREDENTIALS has a non-exact length".to_string());
    }
    let declared_end = header_start
        .checked_add(declared_length)
        .ok_or_else(|| "SCM_CREDENTIALS record range overflowed".to_string())?;
    // SAFETY: the cmsghdr is fully bounded above.
    let data = unsafe { libc::CMSG_DATA(header).cast::<libc::ucred>() };
    let data_start = data as usize;
    let data_end = data_start
        .checked_add(std::mem::size_of::<libc::ucred>())
        .ok_or_else(|| "SCM_CREDENTIALS data range overflowed".to_string())?;
    if data_start < header_end || data_end != declared_end || data_end > control_end {
        return Err("SCM_CREDENTIALS data lies outside its exact record".to_string());
    }
    // SAFETY: the complete ucred lies within both the exact record and the
    // live control buffer. Use an unaligned read for ancillary storage.
    Ok(unsafe { std::ptr::read_unaligned(data) })
}

#[cfg(target_os = "linux")]
fn read_rights(message: &libc::msghdr, header: *mut libc::cmsghdr) -> ParsedRightsV8 {
    let mut parsed = ParsedRightsV8 {
        descriptors: Vec::new(),
        violation: None,
    };
    let header_length = match cmsg_len(0) {
        Ok(length) => length,
        Err(error) => {
            parsed.violation = Some(error.to_string());
            return parsed;
        }
    };
    let control_start = message.msg_control as usize;
    let Some(control_end) = control_start.checked_add(message.msg_controllen) else {
        parsed.violation = Some("ancillary control range overflowed".to_string());
        return parsed;
    };
    let header_start = header as usize;
    if header_start < control_start || header_start >= control_end {
        parsed.violation = Some("SCM_RIGHTS header lies outside ancillary storage".to_string());
        return parsed;
    }
    // SAFETY: CMSG_FIRSTHDR/NXTHDR returned this header within msg_control.
    let declared_length = unsafe { (*header).cmsg_len };
    if declared_length < header_length {
        parsed.violation = Some("SCM_RIGHTS length is smaller than its header".to_string());
        return parsed;
    }
    let declared_end = header_start.checked_add(declared_length);
    let bounded_end = declared_end.unwrap_or(usize::MAX).min(control_end);
    if declared_end.is_none() || declared_end.is_some_and(|end| end > control_end) {
        parsed.violation = Some("SCM_RIGHTS length exceeds ancillary storage".to_string());
    }
    // SAFETY: header is a bounded cmsghdr returned by libc's CMSG traversal.
    let data = unsafe { libc::CMSG_DATA(header).cast::<libc::c_int>() };
    let data_start = data as usize;
    if data_start > bounded_end {
        parsed
            .violation
            .get_or_insert_with(|| "SCM_RIGHTS data starts outside its record".to_string());
        return parsed;
    }
    let data_length = bounded_end - data_start;
    let descriptor_bytes = std::mem::size_of::<libc::c_int>();
    if data_length == 0 || !data_length.is_multiple_of(descriptor_bytes) {
        parsed
            .violation
            .get_or_insert_with(|| "SCM_RIGHTS has an invalid descriptor byte length".to_string());
    }
    let count = data_length / descriptor_bytes;
    parsed.descriptors.reserve(count);
    // Adopt every complete non-negative descriptor first. All validation
    // errors are delayed until ownership has been established, so the vector
    // closes every installed fd on rejection.
    for index in 0..count {
        // SAFETY: index is bounded by the control-buffer-clamped byte count.
        let fd = unsafe { std::ptr::read_unaligned(data.add(index)) };
        if fd < 0 {
            parsed
                .violation
                .get_or_insert_with(|| "SCM_RIGHTS contained a negative descriptor".to_string());
            continue;
        }
        // SAFETY: each non-negative SCM_RIGHTS fd was installed uniquely into
        // this receiver by recvmsg.
        let descriptor = unsafe { OwnedFd::from_raw_fd(fd) };
        parsed.descriptors.push(ReceivedFileDescriptorV8 {
            descriptor,
            _private: (),
        });
    }
    if count > MAX_SEQPACKET_FILE_DESCRIPTORS_V8 {
        parsed.violation.get_or_insert_with(|| {
            format!("SCM_RIGHTS count {count} exceeds maximum {MAX_SEQPACKET_FILE_DESCRIPTORS_V8}")
        });
    }
    for descriptor in &parsed.descriptors {
        // SAFETY: F_GETFD reads flags from an owned received descriptor.
        let flags = unsafe { libc::fcntl(descriptor.descriptor.as_raw_fd(), libc::F_GETFD) };
        if flags < 0 || flags & libc::FD_CLOEXEC == 0 {
            parsed.violation.get_or_insert_with(|| {
                "received descriptor is not atomically close-on-exec".to_string()
            });
        }
    }
    parsed
}

#[cfg(target_os = "linux")]
struct ParsedRightsV8 {
    descriptors: Vec<ReceivedFileDescriptorV8>,
    violation: Option<String>,
}

#[cfg(target_os = "linux")]
fn reject_queued_nonempty_packet(fd: libc::c_int) -> NativeIpcResultV8<()> {
    let mut byte = 0_u8;
    loop {
        // SAFETY: byte is writable for one byte. MSG_PEEK does not consume or
        // install ancillary rights from a queued second packet.
        let rc = unsafe {
            libc::recv(
                fd,
                (&mut byte as *mut u8).cast(),
                1,
                libc::MSG_PEEK | libc::MSG_DONTWAIT,
            )
        };
        if rc > 0 {
            return Err(NativeIpcErrorV8::ProtocolViolation(
                "more than one non-empty seqpacket request was queued".to_string(),
            ));
        }
        if rc == 0 {
            // Either orderly closure or a zero-length packet. Zero-length
            // frames can never validate and the consuming connection is now
            // dropped, so neither case can create a second verified request.
            return Ok(());
        }
        let error = std::io::Error::last_os_error();
        match error.raw_os_error() {
            Some(libc::EAGAIN) => return Ok(()),
            Some(libc::EINTR) => continue,
            _ => return Err(ipc_io("peek for queued seqpacket request", error)),
        }
    }
}

#[cfg(target_os = "linux")]
fn cmsg_space(data_bytes: usize) -> NativeIpcResultV8<usize> {
    let data_bytes = libc::c_uint::try_from(data_bytes).map_err(|_| {
        NativeIpcErrorV8::InvalidInput("ancillary data size exceeds c_uint".to_string())
    })?;
    // SAFETY: CMSG_SPACE is a pure size calculation for this bounded value.
    Ok(unsafe { libc::CMSG_SPACE(data_bytes) as usize })
}

#[cfg(target_os = "linux")]
fn cmsg_len(data_bytes: usize) -> NativeIpcResultV8<usize> {
    let data_bytes = libc::c_uint::try_from(data_bytes).map_err(|_| {
        NativeIpcErrorV8::InvalidInput("ancillary data size exceeds c_uint".to_string())
    })?;
    // SAFETY: CMSG_LEN is a pure size calculation for this bounded value.
    Ok(unsafe { libc::CMSG_LEN(data_bytes) as usize })
}

#[cfg(target_os = "linux")]
fn aligned_control_buffer(required_bytes: usize) -> Vec<usize> {
    let word = std::mem::size_of::<usize>();
    let words = required_bytes.div_ceil(word);
    vec![0_usize; words]
}

#[cfg(target_os = "linux")]
fn into_file_impl(received: ReceivedFileDescriptorV8) -> NativeIpcResultV8<File> {
    Ok(File::from(received.descriptor))
}

#[cfg(not(target_os = "linux"))]
fn into_file_impl(_received: ReceivedFileDescriptorV8) -> NativeIpcResultV8<File> {
    Err(ipc_unsupported("consume received SCM_RIGHTS descriptor"))
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::SeqpacketListenerV8;
    use crate::connect_seqpacket_v8;
    use std::fs;
    use std::fs::File;
    use std::io::Read;
    use std::os::fd::AsRawFd;
    use std::path::PathBuf;
    use std::thread;

    struct TestSocketDirectory {
        root: PathBuf,
        socket: PathBuf,
    }

    impl TestSocketDirectory {
        fn create(label: &str) -> Self {
            let root = std::env::temp_dir().join(format!(
                "hepta-linux-v8-frame-{label}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("system clock")
                    .as_nanos()
            ));
            fs::create_dir(&root).expect("create socket directory");
            let socket = root.join("ipc.sock");
            Self { root, socket }
        }
    }

    impl Drop for TestSocketDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    #[test]
    fn receives_one_exact_credential_bound_packet() {
        let temporary = TestSocketDirectory::create("payload");
        let listener = SeqpacketListenerV8::bind(&temporary.socket).expect("bind listener");
        let socket = temporary.socket.clone();
        let client = thread::spawn(move || {
            connect_seqpacket_v8(&socket)
                .expect("connect")
                .send_one_request(b"one request", &[])
                .expect("send request");
        });
        let packet = listener
            .accept()
            .expect("accept")
            .receive_one_request(ExactFileDescriptorCountV8::new(0).expect("zero FDs"))
            .expect("receive request");
        assert_eq!(packet.payload(), b"one request");
        assert_eq!(packet.peer().pid(), std::process::id());
        assert_eq!(packet.file_descriptor_count(), 0);
        client.join().expect("join client");
    }

    #[test]
    fn receives_exactly_one_cloexec_file_descriptor() {
        let temporary = TestSocketDirectory::create("fd");
        let payload_path = temporary.root.join("passed-file");
        fs::write(&payload_path, b"passed by descriptor").expect("write passed file");
        let listener = SeqpacketListenerV8::bind(&temporary.socket).expect("bind listener");
        let socket = temporary.socket.clone();
        let client = thread::spawn(move || {
            let file = File::open(payload_path).expect("open passed file");
            connect_seqpacket_v8(&socket)
                .expect("connect")
                .send_one_request(b"with fd", &[&file])
                .expect("send request with FD");
        });
        let packet = listener
            .accept()
            .expect("accept")
            .receive_one_request(ExactFileDescriptorCountV8::new(1).expect("one FD"))
            .expect("receive request with FD");
        let (_, _, mut descriptors) = packet.into_parts();
        let mut file = descriptors
            .remove(0)
            .into_file()
            .expect("consume received FD");
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).expect("read received FD");
        assert_eq!(bytes, b"passed by descriptor");
        client.join().expect("join client");
    }

    #[test]
    fn rejects_unexpected_ancillary_descriptor() {
        let temporary = TestSocketDirectory::create("unexpected-fd");
        let payload_path = temporary.root.join("passed-file");
        fs::write(&payload_path, b"unexpected").expect("write passed file");
        let listener = SeqpacketListenerV8::bind(&temporary.socket).expect("bind listener");
        let socket = temporary.socket.clone();
        let client = thread::spawn(move || {
            let file = File::open(payload_path).expect("open passed file");
            connect_seqpacket_v8(&socket)
                .expect("connect")
                .send_one_request(b"wrong fd count", &[&file])
        });
        let error = listener
            .accept()
            .expect("accept")
            .receive_one_request(ExactFileDescriptorCountV8::new(0).expect("zero FDs"))
            .expect_err("unexpected FD must fail closed");
        assert!(
            error.to_string().contains("ancillary") || error.to_string().contains("descriptor")
        );
        client.join().expect("join client").expect("send request");
    }

    #[test]
    fn rejects_oversized_payload_and_nonfirst_sequence() {
        let payload = vec![0_u8; MAX_SEQPACKET_PAYLOAD_BYTES_V8 + 1];
        assert!(encode_frame(&payload).is_err());

        let mut wire = encode_frame(b"valid").expect("encode fixture");
        let sequence_offset = FRAME_MAGIC_V8.len();
        wire[sequence_offset..sequence_offset + 8].copy_from_slice(&2_u64.to_be_bytes());
        let error = decode_frame(&wire).expect_err("sequence two must fail closed");
        assert!(error.to_string().contains("sequence"));
    }

    #[test]
    fn rejects_a_second_queued_nonempty_packet() {
        let temporary = TestSocketDirectory::create("two-packets");
        let listener = SeqpacketListenerV8::bind(&temporary.socket).expect("bind listener");
        let socket = temporary.socket.clone();
        let client = thread::spawn(move || {
            let connection = connect_seqpacket_v8(&socket).expect("connect");
            let (descriptor, peer) = connection.into_verified_parts();
            peer.verify_live().expect("live server peer");
            send_raw_frame(descriptor.as_raw_fd(), b"first", &[]);
            send_raw_frame(descriptor.as_raw_fd(), b"second", &[]);
        });
        client.join().expect("join two-packet sender");
        let error = listener
            .accept()
            .expect("accept")
            .receive_one_request(ExactFileDescriptorCountV8::new(0).expect("zero FDs"))
            .expect_err("a queued second packet must fail closed");
        assert!(error.to_string().contains("more than one"));
    }

    #[test]
    fn adopts_and_closes_all_complete_rights_before_rejecting_malformed_oversize() {
        let temporary = TestSocketDirectory::create("rights-overflow");
        let payload_path = temporary.root.join("passed-file");
        fs::write(&payload_path, b"overflow fixture").expect("write passed file");
        let source = File::open(&payload_path).expect("open passed file");
        let count = MAX_SEQPACKET_FILE_DESCRIPTORS_V8 + 1;
        let descriptor_bytes = count * std::mem::size_of::<libc::c_int>();
        let mut installed = Vec::with_capacity(count);
        for _ in 0..count {
            // SAFETY: F_DUPFD_CLOEXEC duplicates the live source descriptor.
            let fd = unsafe { libc::fcntl(source.as_raw_fd(), libc::F_DUPFD_CLOEXEC, 0) };
            assert!(fd >= 0, "duplicate descriptor fixture");
            installed.push(fd);
        }

        // Add one malformed trailing byte while retaining nine complete
        // installed descriptors. The parser must adopt all nine before it can
        // report either the malformed length or frozen-count violation.
        let malformed_data_bytes = descriptor_bytes + 1;
        let mut control = aligned_control_buffer(
            cmsg_space(malformed_data_bytes).expect("malformed fixture cmsg space"),
        );
        // SAFETY: zero is a valid empty msghdr; control is assigned below.
        let mut message: libc::msghdr = unsafe { std::mem::zeroed() };
        message.msg_control = control.as_mut_ptr().cast();
        message.msg_controllen = control.len() * std::mem::size_of::<usize>();
        // SAFETY: control is aligned and sized for this cmsghdr and payload.
        let header = unsafe { libc::CMSG_FIRSTHDR(&message) };
        assert!(!header.is_null());
        // SAFETY: header/data remain inside the live synthetic control buffer.
        unsafe {
            (*header).cmsg_level = libc::SOL_SOCKET;
            (*header).cmsg_type = libc::SCM_RIGHTS;
            (*header).cmsg_len = cmsg_len(malformed_data_bytes).expect("malformed cmsg len");
            let data = libc::CMSG_DATA(header).cast::<libc::c_int>();
            for (index, fd) in installed.iter().copied().enumerate() {
                std::ptr::write_unaligned(data.add(index), fd);
            }
        }
        let parsed = read_rights(&message, header);
        assert_eq!(parsed.descriptors.len(), count);
        assert!(parsed.violation.is_some());
        drop(parsed);
        for fd in installed {
            // SAFETY: F_GETFD only probes whether the adopted descriptor was
            // closed when the rejected parse token dropped.
            assert_eq!(unsafe { libc::fcntl(fd, libc::F_GETFD) }, -1);
            assert_eq!(
                std::io::Error::last_os_error().raw_os_error(),
                Some(libc::EBADF)
            );
        }
    }

    #[test]
    fn credential_parser_rejects_a_control_range_shorter_than_the_exact_record() {
        let required = cmsg_space(std::mem::size_of::<libc::ucred>()).expect("credential space");
        let mut control = aligned_control_buffer(required);
        // SAFETY: zero is a valid empty msghdr; the live control storage is
        // installed before CMSG_FIRSTHDR is called.
        let mut message: libc::msghdr = unsafe { std::mem::zeroed() };
        message.msg_control = control.as_mut_ptr().cast();
        message.msg_controllen = control.len() * std::mem::size_of::<usize>();
        // SAFETY: the aligned buffer contains a complete cmsghdr.
        let header = unsafe { libc::CMSG_FIRSTHDR(&message) };
        assert!(!header.is_null());
        let exact = cmsg_len(std::mem::size_of::<libc::ucred>()).expect("credential length");
        // SAFETY: header and its data both lie in the full live buffer.
        unsafe {
            (*header).cmsg_level = libc::SOL_SOCKET;
            (*header).cmsg_type = libc::SCM_CREDENTIALS;
            (*header).cmsg_len = exact;
            std::ptr::write_unaligned(
                libc::CMSG_DATA(header).cast::<libc::ucred>(),
                libc::ucred {
                    pid: 1,
                    uid: 2,
                    gid: 3,
                },
            );
        }
        assert!(read_credentials(&message, header).is_ok());
        message.msg_controllen = exact - 1;
        assert!(read_credentials(&message, header).is_err());
    }

    fn send_raw_frame(fd: libc::c_int, payload: &[u8], rights: &[libc::c_int]) {
        let wire = encode_frame(payload).expect("encode raw test frame");
        let mut io_vector = libc::iovec {
            iov_base: wire.as_ptr().cast_mut().cast(),
            iov_len: wire.len(),
        };
        // SAFETY: zero is a valid empty msghdr; all live buffers are assigned
        // before sendmsg.
        let mut message: libc::msghdr = unsafe { std::mem::zeroed() };
        message.msg_iov = &mut io_vector;
        message.msg_iovlen = 1;
        let mut control = if rights.is_empty() {
            Vec::new()
        } else {
            aligned_control_buffer(
                cmsg_space(std::mem::size_of_val(rights)).expect("raw cmsg space"),
            )
        };
        if !control.is_empty() {
            message.msg_control = control.as_mut_ptr().cast();
            message.msg_controllen = control.len() * std::mem::size_of::<usize>();
            // SAFETY: control is aligned and sized for one SCM_RIGHTS record.
            let header = unsafe { libc::CMSG_FIRSTHDR(&message) };
            assert!(!header.is_null());
            // SAFETY: header/data remain within the live control buffer.
            unsafe {
                (*header).cmsg_level = libc::SOL_SOCKET;
                (*header).cmsg_type = libc::SCM_RIGHTS;
                (*header).cmsg_len = cmsg_len(std::mem::size_of_val(rights)).expect("raw cmsg len");
                let data = libc::CMSG_DATA(header).cast::<libc::c_int>();
                for (index, raw_fd) in rights.iter().copied().enumerate() {
                    std::ptr::write_unaligned(data.add(index), raw_fd);
                }
            }
        }
        // SAFETY: message references live wire/control buffers only.
        let sent = unsafe { libc::sendmsg(fd, &message, libc::MSG_NOSIGNAL) };
        assert_eq!(sent, wire.len() as isize, "raw sendmsg must be exact");
    }
}
