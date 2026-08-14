use super::NativeIpcResultV8;
use std::path::Path;

#[cfg(target_os = "linux")]
use super::NativeIpcErrorV8;
#[cfg(target_os = "linux")]
use super::ipc_io;
#[cfg(not(target_os = "linux"))]
use super::ipc_unsupported;
#[cfg(target_os = "linux")]
use crate::PidHandleV8;
#[cfg(target_os = "linux")]
use crate::bind_kernel_peer_pidfd_verified;
#[cfg(target_os = "linux")]
use std::os::fd::AsRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::FromRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::OwnedFd;
#[cfg(target_os = "linux")]
use std::os::unix::ffi::OsStrExt;

#[cfg(target_os = "linux")]
const SO_PEERPIDFD_V8: libc::c_int = 77;
#[cfg(target_os = "linux")]
const SEQPACKET_IO_TIMEOUT_SECONDS_V8: libc::time_t = 30;

/// Kernel-authenticated peer identity tied to a live pidfd and stable process
/// start-time. Its fields cannot be populated from request bytes.
#[derive(Debug)]
pub struct VerifiedPeerV8 {
    pid: u32,
    uid: u32,
    gid: u32,
    start_ticks: u64,
    #[cfg(target_os = "linux")]
    process: PidHandleV8,
    _private: (),
}

impl VerifiedPeerV8 {
    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn uid(&self) -> u32 {
        self.uid
    }

    pub fn gid(&self) -> u32 {
        self.gid
    }

    pub fn start_ticks(&self) -> u64 {
        self.start_ticks
    }

    pub fn process_exited(&self) -> NativeIpcResultV8<bool> {
        process_exited_impl(self)
    }

    #[cfg(target_os = "linux")]
    pub(super) fn verify_live(&self) -> NativeIpcResultV8<()> {
        if self.process.is_exited()? {
            return Err(NativeIpcErrorV8::ProtocolViolation(
                "verified seqpacket peer has exited".to_string(),
            ));
        }
        Ok(())
    }
}

/// A bound Linux seqpacket listener. Existing pathnames are never replaced.
/// Until a trusted anchored runtime-directory abstraction exists, a rare
/// post-bind listen failure deliberately leaves the socket leaf for recovery
/// instead of risking a pathname-race unlink of an attacker replacement.
#[derive(Debug)]
pub struct SeqpacketListenerV8 {
    #[cfg(target_os = "linux")]
    descriptor: OwnedFd,
    _private: (),
}

impl SeqpacketListenerV8 {
    pub fn bind(path: &Path) -> NativeIpcResultV8<Self> {
        bind_impl(path)
    }

    pub fn accept(&self) -> NativeIpcResultV8<SeqpacketConnectionV8> {
        accept_impl(self)
    }
}

/// One kernel-identified connected socket. Frame APIs consume this value,
/// making the transport one-request-per-connection by construction. This is
/// an observation token, not peer authorization.
#[derive(Debug)]
pub struct SeqpacketConnectionV8 {
    #[cfg(target_os = "linux")]
    pub(super) descriptor: OwnedFd,
    pub(super) peer: VerifiedPeerV8,
    _private: (),
}

impl SeqpacketConnectionV8 {
    pub fn peer(&self) -> &VerifiedPeerV8 {
        &self.peer
    }

    #[cfg(target_os = "linux")]
    pub(super) fn from_verified_parts(descriptor: OwnedFd, peer: VerifiedPeerV8) -> Self {
        Self {
            descriptor,
            peer,
            _private: (),
        }
    }

    #[cfg(target_os = "linux")]
    pub(super) fn into_verified_parts(self) -> (OwnedFd, VerifiedPeerV8) {
        (self.descriptor, self.peer)
    }
}

/// Connects to one filesystem-backed Unix seqpacket listener and identifies
/// the server with `SO_PEERCRED` plus pidfd/start-time binding. The caller must
/// separately compare that observation with a frozen expected-peer profile.
pub fn connect_seqpacket_v8(path: &Path) -> NativeIpcResultV8<SeqpacketConnectionV8> {
    connect_impl(path)
}

#[cfg(target_os = "linux")]
fn bind_impl(path: &Path) -> NativeIpcResultV8<SeqpacketListenerV8> {
    let (address, address_length) = socket_address(path)?;
    let descriptor = new_seqpacket_socket()?;
    enable_and_verify_passcred(descriptor.as_raw_fd())?;
    // SAFETY: address points to a fully initialized sockaddr_un and remains
    // live for the duration of bind.
    let rc = unsafe {
        libc::bind(
            descriptor.as_raw_fd(),
            (&address as *const libc::sockaddr_un).cast(),
            address_length,
        )
    };
    if rc != 0 {
        // A failed bind does not confer ownership of a colliding pathname, so
        // never unlink here. Linux does not publish the Unix leaf unless bind
        // succeeds.
        return Err(ipc_io(
            "bind AF_UNIX SOCK_SEQPACKET",
            std::io::Error::last_os_error(),
        ));
    }
    // SAFETY: the descriptor is a bound seqpacket socket and listen retains no
    // pointer or descriptor ownership.
    if unsafe { libc::listen(descriptor.as_raw_fd(), 8) } != 0 {
        return Err(ipc_io(
            "listen AF_UNIX SOCK_SEQPACKET",
            std::io::Error::last_os_error(),
        ));
    }
    Ok(SeqpacketListenerV8 {
        descriptor,
        _private: (),
    })
}

#[cfg(not(target_os = "linux"))]
fn bind_impl(_path: &Path) -> NativeIpcResultV8<SeqpacketListenerV8> {
    Err(ipc_unsupported("bind AF_UNIX SOCK_SEQPACKET"))
}

#[cfg(target_os = "linux")]
fn accept_impl(listener: &SeqpacketListenerV8) -> NativeIpcResultV8<SeqpacketConnectionV8> {
    // SAFETY: the listener descriptor is live; null address arguments request
    // no untrusted peer pathname. accept4 returns a new descriptor on success.
    let fd = unsafe {
        libc::accept4(
            listener.descriptor.as_raw_fd(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            libc::SOCK_CLOEXEC,
        )
    };
    if fd < 0 {
        return Err(ipc_io(
            "accept4 AF_UNIX SOCK_SEQPACKET",
            std::io::Error::last_os_error(),
        ));
    }
    // SAFETY: accept4 returned a uniquely owned descriptor.
    let descriptor = unsafe { OwnedFd::from_raw_fd(fd) };
    verify_connected_socket(descriptor)
}

#[cfg(not(target_os = "linux"))]
fn accept_impl(_listener: &SeqpacketListenerV8) -> NativeIpcResultV8<SeqpacketConnectionV8> {
    Err(ipc_unsupported("accept4 AF_UNIX SOCK_SEQPACKET"))
}

#[cfg(target_os = "linux")]
fn connect_impl(path: &Path) -> NativeIpcResultV8<SeqpacketConnectionV8> {
    let (address, address_length) = socket_address(path)?;
    let descriptor = new_seqpacket_socket()?;
    enable_and_verify_passcred(descriptor.as_raw_fd())?;
    // SAFETY: address is initialized and remains live for connect.
    let rc = unsafe {
        libc::connect(
            descriptor.as_raw_fd(),
            (&address as *const libc::sockaddr_un).cast(),
            address_length,
        )
    };
    if rc != 0 {
        return Err(ipc_io(
            "connect AF_UNIX SOCK_SEQPACKET",
            std::io::Error::last_os_error(),
        ));
    }
    verify_connected_socket(descriptor)
}

#[cfg(not(target_os = "linux"))]
fn connect_impl(_path: &Path) -> NativeIpcResultV8<SeqpacketConnectionV8> {
    Err(ipc_unsupported("connect AF_UNIX SOCK_SEQPACKET"))
}

#[cfg(target_os = "linux")]
fn verify_connected_socket(descriptor: OwnedFd) -> NativeIpcResultV8<SeqpacketConnectionV8> {
    verify_cloexec(descriptor.as_raw_fd())?;
    verify_socket_scalar(descriptor.as_raw_fd(), libc::SO_TYPE, libc::SOCK_SEQPACKET)?;
    verify_socket_scalar(descriptor.as_raw_fd(), libc::SO_DOMAIN, libc::AF_UNIX)?;
    enable_and_verify_passcred(descriptor.as_raw_fd())?;
    set_and_verify_io_timeouts(descriptor.as_raw_fd())?;

    let before = peer_credentials(descriptor.as_raw_fd())?;
    if before.pid <= 0 {
        return Err(NativeIpcErrorV8::ProtocolViolation(
            "SO_PEERCRED returned a non-positive PID".to_string(),
        ));
    }
    let pid = u32::try_from(before.pid).map_err(|_| {
        NativeIpcErrorV8::ProtocolViolation("SO_PEERCRED PID does not fit u32".to_string())
    })?;
    let process = bind_kernel_peer_pidfd_verified(peer_pidfd(descriptor.as_raw_fd())?, pid)?;
    let after = peer_credentials(descriptor.as_raw_fd())?;
    if before.pid != after.pid || before.uid != after.uid || before.gid != after.gid {
        return Err(NativeIpcErrorV8::ProtocolViolation(
            "SO_PEERCRED changed while binding peer pidfd".to_string(),
        ));
    }
    if process.pid() != pid || process.is_exited()? {
        return Err(NativeIpcErrorV8::ProtocolViolation(
            "peer process exited during credential verification".to_string(),
        ));
    }
    let peer = VerifiedPeerV8 {
        pid,
        uid: before.uid,
        gid: before.gid,
        start_ticks: process.start_ticks(),
        process,
        _private: (),
    };
    Ok(SeqpacketConnectionV8::from_verified_parts(descriptor, peer))
}

#[cfg(target_os = "linux")]
fn peer_pidfd(fd: libc::c_int) -> NativeIpcResultV8<OwnedFd> {
    let mut peer_pidfd: libc::c_int = -1;
    let mut length = std::mem::size_of::<libc::c_int>() as libc::socklen_t;
    // SAFETY: peer_pidfd and length are writable exact-sized storage. Linux
    // SO_PEERPIDFD installs a new close-on-exec pidfd on success.
    let rc = unsafe {
        libc::getsockopt(
            fd,
            libc::SOL_SOCKET,
            SO_PEERPIDFD_V8,
            (&mut peer_pidfd as *mut libc::c_int).cast(),
            &mut length,
        )
    };
    if rc != 0 {
        return Err(ipc_io(
            "getsockopt SO_PEERPIDFD",
            std::io::Error::last_os_error(),
        ));
    }
    if length as usize != std::mem::size_of::<libc::c_int>() || peer_pidfd < 0 {
        return Err(NativeIpcErrorV8::ProtocolViolation(
            "SO_PEERPIDFD returned a non-exact or negative descriptor".to_string(),
        ));
    }
    // SAFETY: SO_PEERPIDFD returned a newly installed, uniquely owned fd.
    Ok(unsafe { OwnedFd::from_raw_fd(peer_pidfd) })
}

#[cfg(target_os = "linux")]
fn new_seqpacket_socket() -> NativeIpcResultV8<OwnedFd> {
    // SAFETY: socket receives scalar constants and returns a new descriptor.
    let fd = unsafe { libc::socket(libc::AF_UNIX, libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC, 0) };
    if fd < 0 {
        return Err(ipc_io(
            "socket AF_UNIX SOCK_SEQPACKET",
            std::io::Error::last_os_error(),
        ));
    }
    // SAFETY: socket returned a uniquely owned descriptor.
    let descriptor = unsafe { OwnedFd::from_raw_fd(fd) };
    verify_cloexec(descriptor.as_raw_fd())?;
    Ok(descriptor)
}

#[cfg(target_os = "linux")]
fn enable_and_verify_passcred(fd: libc::c_int) -> NativeIpcResultV8<()> {
    let enabled: libc::c_int = 1;
    // SAFETY: enabled points to an integer of the supplied exact length.
    let rc = unsafe {
        libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_PASSCRED,
            (&enabled as *const libc::c_int).cast(),
            std::mem::size_of::<libc::c_int>() as libc::socklen_t,
        )
    };
    if rc != 0 {
        return Err(ipc_io(
            "setsockopt SO_PASSCRED",
            std::io::Error::last_os_error(),
        ));
    }
    verify_socket_scalar(fd, libc::SO_PASSCRED, 1)
}

#[cfg(target_os = "linux")]
fn set_and_verify_io_timeouts(fd: libc::c_int) -> NativeIpcResultV8<()> {
    let timeout = libc::timeval {
        tv_sec: SEQPACKET_IO_TIMEOUT_SECONDS_V8,
        tv_usec: 0,
    };
    for (option, operation) in [
        (libc::SO_RCVTIMEO, "setsockopt SO_RCVTIMEO"),
        (libc::SO_SNDTIMEO, "setsockopt SO_SNDTIMEO"),
    ] {
        // SAFETY: timeout points to an initialized timeval of the exact size.
        let rc = unsafe {
            libc::setsockopt(
                fd,
                libc::SOL_SOCKET,
                option,
                (&timeout as *const libc::timeval).cast(),
                std::mem::size_of::<libc::timeval>() as libc::socklen_t,
            )
        };
        if rc != 0 {
            return Err(ipc_io(operation, std::io::Error::last_os_error()));
        }
        let mut observed: libc::timeval = unsafe { std::mem::zeroed() };
        let mut length = std::mem::size_of::<libc::timeval>() as libc::socklen_t;
        // SAFETY: observed and length are writable exact-sized storage.
        let rc = unsafe {
            libc::getsockopt(
                fd,
                libc::SOL_SOCKET,
                option,
                (&mut observed as *mut libc::timeval).cast(),
                &mut length,
            )
        };
        if rc != 0 {
            return Err(ipc_io(
                "getsockopt seqpacket timeout",
                std::io::Error::last_os_error(),
            ));
        }
        if length as usize != std::mem::size_of::<libc::timeval>()
            || observed.tv_sec != timeout.tv_sec
            || observed.tv_usec != 0
        {
            return Err(NativeIpcErrorV8::ProtocolViolation(format!(
                "seqpacket timeout option {option} was not retained exactly"
            )));
        }
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn verify_socket_scalar(
    fd: libc::c_int,
    option: libc::c_int,
    expected: libc::c_int,
) -> NativeIpcResultV8<()> {
    let mut observed: libc::c_int = 0;
    let mut length = std::mem::size_of::<libc::c_int>() as libc::socklen_t;
    // SAFETY: observed and length are writable and sized for one integer.
    let rc = unsafe {
        libc::getsockopt(
            fd,
            libc::SOL_SOCKET,
            option,
            (&mut observed as *mut libc::c_int).cast(),
            &mut length,
        )
    };
    if rc != 0 {
        return Err(ipc_io(
            "getsockopt seqpacket property",
            std::io::Error::last_os_error(),
        ));
    }
    if length as usize != std::mem::size_of::<libc::c_int>() || observed != expected {
        return Err(NativeIpcErrorV8::ProtocolViolation(format!(
            "seqpacket property {option} is {observed}, expected {expected}"
        )));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn peer_credentials(fd: libc::c_int) -> NativeIpcResultV8<libc::ucred> {
    // SAFETY: zero is a valid initial representation for ucred and getsockopt
    // initializes all fields on success.
    let mut credentials: libc::ucred = unsafe { std::mem::zeroed() };
    let mut length = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    // SAFETY: credentials and length point to writable exact-sized storage.
    let rc = unsafe {
        libc::getsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_PEERCRED,
            (&mut credentials as *mut libc::ucred).cast(),
            &mut length,
        )
    };
    if rc != 0 {
        return Err(ipc_io(
            "getsockopt SO_PEERCRED",
            std::io::Error::last_os_error(),
        ));
    }
    if length as usize != std::mem::size_of::<libc::ucred>() {
        return Err(NativeIpcErrorV8::ProtocolViolation(
            "SO_PEERCRED returned a non-exact credential length".to_string(),
        ));
    }
    Ok(credentials)
}

#[cfg(target_os = "linux")]
fn verify_cloexec(fd: libc::c_int) -> NativeIpcResultV8<()> {
    // SAFETY: fcntl F_GETFD reads descriptor flags and takes no pointer.
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFD) };
    if flags < 0 {
        return Err(ipc_io(
            "fcntl F_GETFD seqpacket",
            std::io::Error::last_os_error(),
        ));
    }
    if flags & libc::FD_CLOEXEC == 0 {
        return Err(NativeIpcErrorV8::ProtocolViolation(
            "seqpacket descriptor is missing FD_CLOEXEC".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn socket_address(path: &Path) -> NativeIpcResultV8<(libc::sockaddr_un, libc::socklen_t)> {
    if !path.is_absolute() {
        return Err(NativeIpcErrorV8::InvalidInput(
            "Unix seqpacket path must be absolute".to_string(),
        ));
    }
    let bytes = path.as_os_str().as_bytes();
    if bytes.is_empty() || bytes.contains(&0) {
        return Err(NativeIpcErrorV8::InvalidInput(
            "Unix seqpacket path is empty or contains NUL".to_string(),
        ));
    }
    // SAFETY: zero is a valid initial representation of sockaddr_un.
    let mut address: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    if bytes.len() >= address.sun_path.len() {
        return Err(NativeIpcErrorV8::InvalidInput(
            "Unix seqpacket path exceeds sockaddr_un capacity".to_string(),
        ));
    }
    address.sun_family = libc::AF_UNIX as libc::sa_family_t;
    for (destination, source) in address.sun_path.iter_mut().zip(bytes) {
        *destination = *source as libc::c_char;
    }
    let length = std::mem::offset_of!(libc::sockaddr_un, sun_path)
        .checked_add(bytes.len())
        .and_then(|value| value.checked_add(1))
        .ok_or_else(|| NativeIpcErrorV8::InvalidInput("socket length overflow".to_string()))?;
    let length = libc::socklen_t::try_from(length).map_err(|_| {
        NativeIpcErrorV8::InvalidInput("socket length does not fit socklen_t".to_string())
    })?;
    Ok((address, length))
}

#[cfg(target_os = "linux")]
fn process_exited_impl(peer: &VerifiedPeerV8) -> NativeIpcResultV8<bool> {
    peer.process.is_exited().map_err(Into::into)
}

#[cfg(not(target_os = "linux"))]
fn process_exited_impl(_peer: &VerifiedPeerV8) -> NativeIpcResultV8<bool> {
    Err(ipc_unsupported("poll verified seqpacket peer"))
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::fs;
    use std::os::fd::AsRawFd;
    use std::path::PathBuf;
    use std::thread;

    struct TestSocketDirectory {
        root: PathBuf,
        socket: PathBuf,
    }

    impl TestSocketDirectory {
        fn create() -> Self {
            let root = std::env::temp_dir().join(format!(
                "hepta-linux-v8-peer-{}-{}",
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
    fn authenticates_both_ends_with_peercred_pidfd_and_cloexec() {
        let temporary = TestSocketDirectory::create();
        let listener = SeqpacketListenerV8::bind(&temporary.socket).expect("bind listener");
        // SAFETY: F_GETFD reads flags from the live listener descriptor.
        let listener_flags = unsafe { libc::fcntl(listener.descriptor.as_raw_fd(), libc::F_GETFD) };
        assert_ne!(listener_flags & libc::FD_CLOEXEC, 0);
        let socket = temporary.socket.clone();
        let client = thread::spawn(move || {
            let connection = connect_seqpacket_v8(&socket).expect("connect client");
            assert_eq!(connection.peer().pid(), std::process::id());
            assert!(!connection.peer().process_exited().expect("poll server"));
        });
        let connection = listener.accept().expect("accept client");
        assert_eq!(connection.peer().pid(), std::process::id());
        // SAFETY: geteuid and getegid have no preconditions or pointer arguments.
        let expected_uid = unsafe { libc::geteuid() };
        // SAFETY: see the preceding note; this call is equally side-effect free.
        let expected_gid = unsafe { libc::getegid() };
        assert_eq!(connection.peer().uid(), expected_uid);
        assert_eq!(connection.peer().gid(), expected_gid);
        assert!(connection.peer().start_ticks() > 0);
        assert!(!connection.peer().process_exited().expect("poll client"));
        // SAFETY: F_GETFD reads flags from the kernel-issued peer pidfd.
        let pidfd_flags = unsafe { libc::fcntl(connection.peer.process.raw_fd(), libc::F_GETFD) };
        assert_ne!(pidfd_flags & libc::FD_CLOEXEC, 0);
        client.join().expect("join client");
    }

    #[test]
    fn refuses_to_replace_an_existing_socket_path() {
        let temporary = TestSocketDirectory::create();
        fs::write(&temporary.socket, b"do not replace").expect("write collision fixture");
        assert!(SeqpacketListenerV8::bind(&temporary.socket).is_err());
        assert_eq!(
            fs::read(&temporary.socket).expect("read collision fixture"),
            b"do not replace"
        );
    }
}
