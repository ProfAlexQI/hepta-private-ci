use super::DirectoryAnchorV8;
use super::FileIdentityV8;
use super::NativeSysErrorV8;
use super::NativeSysResultV8;
#[cfg(not(target_os = "linux"))]
use super::unsupported;
use std::convert::Infallible;
use std::ffi::CString;
use std::path::Path;

#[cfg(target_os = "linux")]
use super::io_error;
#[cfg(target_os = "linux")]
use sha2::Digest;
#[cfg(target_os = "linux")]
use sha2::Sha256;
#[cfg(target_os = "linux")]
use std::fs::File;
#[cfg(target_os = "linux")]
use std::io::Write;
#[cfg(target_os = "linux")]
use std::os::fd::AsRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::FromRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::OwnedFd;

#[cfg(target_os = "linux")]
const MAX_ARGUMENT_COUNT: usize = 4096;
#[cfg(target_os = "linux")]
pub(super) const MAX_VERIFIED_EXECUTABLE_BYTES_V8: u64 = 256 * 1024 * 1024;

/// Frozen executable identity expected by an install or run authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutableIdentityV8 {
    file: FileIdentityV8,
    sha256: [u8; 32],
}

impl ExecutableIdentityV8 {
    pub fn new(file: FileIdentityV8, sha256: [u8; 32]) -> NativeSysResultV8<Self> {
        if sha256 == [0; 32] {
            return Err(NativeSysErrorV8::InvalidInput(
                "executable SHA-256 must not be the all-zero sentinel".to_string(),
            ));
        }
        Ok(Self { file, sha256 })
    }

    pub fn file(&self) -> FileIdentityV8 {
        self.file
    }

    pub fn sha256(&self) -> [u8; 32] {
        self.sha256
    }
}

/// A sealed anonymous copy of an executable whose source inode metadata,
/// content digest, permission policy, and ELF identity were checked. The
/// descriptor remains private and execution is available only through
/// [`execveat_verified`].
#[derive(Debug)]
pub struct VerifiedExecutableFdV8 {
    #[cfg(target_os = "linux")]
    sealed_executable: OwnedFd,
    identity: ExecutableIdentityV8,
}

impl VerifiedExecutableFdV8 {
    pub fn identity(&self) -> &ExecutableIdentityV8 {
        &self.identity
    }
}

/// Opens an executable beneath an anchored directory with the fixed openat2
/// policy, validates it, then copies the exact bytes into a write-sealed memfd
/// so later in-place writes to the source inode cannot race execution.
pub fn verify_executable_beneath(
    anchor: &DirectoryAnchorV8,
    relative: &Path,
    expected: &ExecutableIdentityV8,
) -> NativeSysResultV8<VerifiedExecutableFdV8> {
    verify_executable_beneath_impl(anchor, relative, expected)
}

/// Revalidates the verified descriptor and replaces the current process using
/// exactly `execveat(fd, "", ..., AT_EMPTY_PATH)`. No pathname-based fallback
/// exists.
pub fn execveat_verified(
    executable: &VerifiedExecutableFdV8,
    arguments: &[CString],
    environment: &[CString],
) -> NativeSysResultV8<Infallible> {
    execveat_verified_impl(executable, arguments, environment)
}

#[cfg(target_os = "linux")]
fn verify_executable_beneath_impl(
    anchor: &DirectoryAnchorV8,
    relative: &Path,
    expected: &ExecutableIdentityV8,
) -> NativeSysResultV8<VerifiedExecutableFdV8> {
    let file = anchor.open_regular_readonly_beneath(relative)?;
    let observed = file.identity();
    if observed.mode() & 0o111 == 0 || observed.mode() & 0o022 != 0 || observed.mode() & 0o6000 != 0
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "verified executable permissions are writable, privileged, or non-executable"
                .to_string(),
        ));
    }
    if observed.link_count() != 1 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "verified executable must have exactly one hard link".to_string(),
        ));
    }
    if observed != expected.file {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "executable descriptor metadata differs from frozen identity".to_string(),
        ));
    }
    if observed.size() > MAX_VERIFIED_EXECUTABLE_BYTES_V8 {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "verified executable exceeds the frozen {MAX_VERIFIED_EXECUTABLE_BYTES_V8}-byte limit"
        )));
    }
    let bytes = file.read_all(MAX_VERIFIED_EXECUTABLE_BYTES_V8)?;
    validate_x86_64_elf(&bytes)?;
    let digest: [u8; 32] = Sha256::digest(&bytes).into();
    if digest != expected.sha256 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "executable content differs from frozen SHA-256".to_string(),
        ));
    }
    file.revalidate_identity()?;
    let sealed_executable = create_sealed_executable_memfd(&bytes)?;
    if sha256_raw_fd(sealed_executable.as_raw_fd(), observed.size())? != expected.sha256 {
        return Err(NativeSysErrorV8::RaceDetected(
            "sealed executable bytes differ from the verified source".to_string(),
        ));
    }
    Ok(VerifiedExecutableFdV8 {
        sealed_executable,
        identity: expected.clone(),
    })
}

#[cfg(not(target_os = "linux"))]
fn verify_executable_beneath_impl(
    _anchor: &DirectoryAnchorV8,
    _relative: &Path,
    _expected: &ExecutableIdentityV8,
) -> NativeSysResultV8<VerifiedExecutableFdV8> {
    Err(unsupported("verify openat2 executable descriptor"))
}

#[cfg(target_os = "linux")]
fn execveat_verified_impl(
    executable: &VerifiedExecutableFdV8,
    arguments: &[CString],
    environment: &[CString],
) -> NativeSysResultV8<Infallible> {
    if arguments.is_empty() {
        return Err(NativeSysErrorV8::InvalidInput(
            "execveat argv must contain argv[0]".to_string(),
        ));
    }
    if arguments.len() > MAX_ARGUMENT_COUNT || environment.len() > MAX_ARGUMENT_COUNT {
        return Err(NativeSysErrorV8::InvalidInput(
            "execveat argv or environment exceeds fixed entry limit".to_string(),
        ));
    }
    verify_required_seals(executable.sealed_executable.as_raw_fd())?;
    if sha256_raw_fd(
        executable.sealed_executable.as_raw_fd(),
        executable.identity.file.size(),
    )? != executable.identity.sha256
    {
        return Err(NativeSysErrorV8::RaceDetected(
            "sealed executable content differs before execveat".to_string(),
        ));
    }
    verify_required_seals(executable.sealed_executable.as_raw_fd())?;

    let mut argument_pointers: Vec<*const libc::c_char> =
        arguments.iter().map(|value| value.as_ptr()).collect();
    argument_pointers.push(std::ptr::null());
    let mut environment_pointers: Vec<*const libc::c_char> =
        environment.iter().map(|value| value.as_ptr()).collect();
    environment_pointers.push(std::ptr::null());
    let empty_path = c"";
    // SAFETY: the descriptor is live and verified; all argument/environment
    // C strings and pointer arrays remain live for the syscall. AT_EMPTY_PATH
    // makes the descriptor, not a path lookup, the execution authority.
    let rc = unsafe {
        libc::syscall(
            libc::SYS_execveat,
            executable.sealed_executable.as_raw_fd(),
            empty_path.as_ptr(),
            argument_pointers.as_ptr(),
            environment_pointers.as_ptr(),
            libc::AT_EMPTY_PATH,
        )
    };
    debug_assert_eq!(rc, -1, "successful execveat never returns");
    Err(io_error(
        "execveat verified descriptor with AT_EMPTY_PATH",
        std::io::Error::last_os_error(),
    ))
}

#[cfg(not(target_os = "linux"))]
fn execveat_verified_impl(
    _executable: &VerifiedExecutableFdV8,
    _arguments: &[CString],
    _environment: &[CString],
) -> NativeSysResultV8<Infallible> {
    Err(unsupported("execveat AT_EMPTY_PATH"))
}

#[cfg(target_os = "linux")]
fn sha256_raw_fd(fd: libc::c_int, size: u64) -> NativeSysResultV8<[u8; 32]> {
    const BUFFER_BYTES: usize = 64 * 1024;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; BUFFER_BYTES];
    let mut offset = 0_u64;
    while offset < size {
        let remaining = size - offset;
        let request = usize::try_from(remaining.min(BUFFER_BYTES as u64)).map_err(|_| {
            NativeSysErrorV8::InvalidInput("executable read size overflow".to_string())
        })?;
        let offset_i64 = i64::try_from(offset).map_err(|_| {
            NativeSysErrorV8::InvalidInput("executable offset exceeds off_t".to_string())
        })?;
        // SAFETY: `buffer` is writable for `request` bytes, the descriptor is
        // live, and pread does not mutate the shared file offset.
        let read = unsafe { libc::pread(fd, buffer.as_mut_ptr().cast(), request, offset_i64) };
        if read < 0 {
            return Err(io_error(
                "pread verified executable",
                std::io::Error::last_os_error(),
            ));
        }
        if read == 0 {
            return Err(NativeSysErrorV8::RaceDetected(
                "verified executable became shorter while hashing".to_string(),
            ));
        }
        let read = usize::try_from(read).map_err(|_| {
            NativeSysErrorV8::InvalidInput("pread returned an invalid length".to_string())
        })?;
        hasher.update(&buffer[..read]);
        offset = offset
            .checked_add(u64::try_from(read).map_err(|_| {
                NativeSysErrorV8::InvalidInput("pread length exceeds u64".to_string())
            })?)
            .ok_or_else(|| {
                NativeSysErrorV8::InvalidInput("executable offset overflow".to_string())
            })?;
    }
    Ok(hasher.finalize().into())
}

#[cfg(target_os = "linux")]
fn create_sealed_executable_memfd(bytes: &[u8]) -> NativeSysResultV8<OwnedFd> {
    let name = c"hepta-linux-v8-verified-executable";
    // SAFETY: name is a static C string; a successful syscall returns a new
    // anonymous descriptor and retains no pointer.
    let raw_fd = unsafe {
        libc::syscall(
            libc::SYS_memfd_create,
            name.as_ptr(),
            libc::MFD_CLOEXEC | libc::MFD_ALLOW_SEALING,
        )
    };
    if raw_fd < 0 {
        return Err(io_error(
            "memfd_create sealed executable",
            std::io::Error::last_os_error(),
        ));
    }
    let raw_fd = libc::c_int::try_from(raw_fd).map_err(|_| {
        NativeSysErrorV8::InvalidInput("memfd_create returned an invalid descriptor".to_string())
    })?;
    // SAFETY: successful memfd_create returned a uniquely owned descriptor.
    let mut file = unsafe { File::from_raw_fd(raw_fd) };
    file.write_all(bytes)
        .map_err(|source| io_error("write sealed executable memfd", source))?;
    // SAFETY: fchmod receives a live descriptor and scalar mode.
    if unsafe { libc::fchmod(file.as_raw_fd(), 0o500) } != 0 {
        return Err(io_error(
            "fchmod sealed executable memfd",
            std::io::Error::last_os_error(),
        ));
    }
    let required_seals = required_seals();
    // SAFETY: F_ADD_SEALS receives only a live descriptor and scalar mask.
    if unsafe { libc::fcntl(file.as_raw_fd(), libc::F_ADD_SEALS, required_seals) } != 0 {
        return Err(io_error(
            "F_ADD_SEALS verified executable memfd",
            std::io::Error::last_os_error(),
        ));
    }
    verify_required_seals(file.as_raw_fd())?;
    Ok(file.into())
}

#[cfg(target_os = "linux")]
fn required_seals() -> libc::c_int {
    libc::F_SEAL_SEAL | libc::F_SEAL_WRITE | libc::F_SEAL_GROW | libc::F_SEAL_SHRINK
}

#[cfg(target_os = "linux")]
fn verify_required_seals(fd: libc::c_int) -> NativeSysResultV8<()> {
    // SAFETY: F_GET_SEALS reads a scalar mask from a live descriptor.
    let observed = unsafe { libc::fcntl(fd, libc::F_GET_SEALS) };
    if observed < 0 {
        return Err(io_error(
            "F_GET_SEALS verified executable memfd",
            std::io::Error::last_os_error(),
        ));
    }
    if observed & required_seals() != required_seals() {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "verified executable memfd lacks mandatory immutable seals".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn validate_x86_64_elf(bytes: &[u8]) -> NativeSysResultV8<()> {
    if bytes.len() < 64
        || bytes.get(..4) != Some(b"\x7fELF")
        || bytes[4] != 2
        || bytes[5] != 1
        || bytes[6] != 1
    {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "verified executable is not an ELF64 little-endian image".to_string(),
        ));
    }
    let elf_type = u16::from_le_bytes([bytes[16], bytes[17]]);
    let machine = u16::from_le_bytes([bytes[18], bytes[19]]);
    let version = u32::from_le_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
    if !matches!(elf_type, 2 | 3) || machine != 62 || version != 1 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "verified executable ELF type, machine, or version is not exact x86_64".to_string(),
        ));
    }
    Ok(())
}
