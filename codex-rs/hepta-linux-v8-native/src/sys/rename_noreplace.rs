use super::DirectoryAnchorV8;
use super::NativeSysResultV8;
#[cfg(not(target_os = "linux"))]
use super::unsupported;
use std::ffi::OsStr;

#[cfg(target_os = "linux")]
use super::NativeSysErrorV8;
#[cfg(target_os = "linux")]
use super::io_error;
#[cfg(target_os = "linux")]
use std::ffi::CString;
#[cfg(target_os = "linux")]
use std::os::unix::ffi::OsStrExt;

/// Proof that the kernel atomically moved one leaf name without replacing an
/// existing destination. The fields are private so callers cannot fabricate a
/// successful publication token.
#[derive(Debug)]
pub struct NoReplaceRenameV8 {
    source_leaf: Box<OsStr>,
    destination_leaf: Box<OsStr>,
}

impl NoReplaceRenameV8 {
    pub fn source_leaf(&self) -> &OsStr {
        &self.source_leaf
    }

    pub fn destination_leaf(&self) -> &OsStr {
        &self.destination_leaf
    }
}

/// Atomically renames a single leaf between already verified directory
/// anchors. Existing destinations are never replaced.
pub fn rename_noreplace_at(
    source_directory: &DirectoryAnchorV8,
    source_leaf: &OsStr,
    destination_directory: &DirectoryAnchorV8,
    destination_leaf: &OsStr,
) -> NativeSysResultV8<NoReplaceRenameV8> {
    rename_noreplace_impl(
        source_directory,
        source_leaf,
        destination_directory,
        destination_leaf,
    )
}

#[cfg(target_os = "linux")]
fn rename_noreplace_impl(
    source_directory: &DirectoryAnchorV8,
    source_leaf: &OsStr,
    destination_directory: &DirectoryAnchorV8,
    destination_leaf: &OsStr,
) -> NativeSysResultV8<NoReplaceRenameV8> {
    let source = leaf_to_cstring(source_leaf)?;
    let destination = leaf_to_cstring(destination_leaf)?;
    const RENAME_NOREPLACE: libc::c_uint = 1;
    // SAFETY: both C strings remain live for the syscall, both directory
    // descriptors are owned anchors, and renameat2 retains no pointer.
    let rc = unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            source_directory.raw_fd(),
            source.as_ptr(),
            destination_directory.raw_fd(),
            destination.as_ptr(),
            RENAME_NOREPLACE,
        )
    };
    if rc != 0 {
        return Err(io_error(
            "renameat2 RENAME_NOREPLACE",
            std::io::Error::last_os_error(),
        ));
    }
    Ok(NoReplaceRenameV8 {
        source_leaf: source_leaf.into(),
        destination_leaf: destination_leaf.into(),
    })
}

#[cfg(not(target_os = "linux"))]
fn rename_noreplace_impl(
    _source_directory: &DirectoryAnchorV8,
    _source_leaf: &OsStr,
    _destination_directory: &DirectoryAnchorV8,
    _destination_leaf: &OsStr,
) -> NativeSysResultV8<NoReplaceRenameV8> {
    Err(unsupported("renameat2 RENAME_NOREPLACE"))
}

#[cfg(target_os = "linux")]
fn leaf_to_cstring(leaf: &OsStr) -> NativeSysResultV8<CString> {
    let bytes = leaf.as_bytes();
    if bytes.is_empty() || bytes == b"." || bytes == b".." || bytes.contains(&b'/') {
        return Err(NativeSysErrorV8::InvalidInput(
            "renameat2 names must be non-empty single leaf components".to_string(),
        ));
    }
    CString::new(bytes)
        .map_err(|_| NativeSysErrorV8::InvalidInput("renameat2 leaf contains NUL".to_string()))
}
