use std::io;
use std::path::Path;

/// Opens `authority_root` once, walks only normal relative path components
/// beneath that retained directory descriptor, and reads at most
/// `max_bytes + 1` bytes from a regular final file.
///
/// Platforms without the required descriptor-relative, no-follow primitives
/// fail closed with [`io::ErrorKind::Unsupported`].
pub fn read_file_beneath(
    authority_root: &Path,
    relative_path: &Path,
    max_bytes: u64,
) -> io::Result<Vec<u8>> {
    #[cfg(unix)]
    {
        read_file_beneath_unix(authority_root, relative_path, max_bytes)
    }
    #[cfg(not(unix))]
    {
        let _ = (authority_root, relative_path, max_bytes);
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "atomic bounded reads beneath an authority root are unsupported on this platform",
        ))
    }
}

#[cfg(unix)]
fn read_file_beneath_unix(
    authority_root: &Path,
    relative_path: &Path,
    max_bytes: u64,
) -> io::Result<Vec<u8>> {
    read_file_beneath_unix_with_hook(authority_root, relative_path, max_bytes, |_| Ok(()))
}

#[cfg(unix)]
fn read_file_beneath_unix_with_hook(
    authority_root: &Path,
    relative_path: &Path,
    max_bytes: u64,
    mut after_directory_open: impl FnMut(usize) -> io::Result<()>,
) -> io::Result<Vec<u8>> {
    use std::ffi::OsString;
    use std::io::Read as _;
    use std::os::unix::fs::MetadataExt as _;
    use std::path::Component;

    if !authority_root.is_absolute() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "bounded read authority root must be absolute",
        ));
    }
    let max_plus_one = max_bytes.checked_add(1).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "bounded read limit is too large",
        )
    })?;
    if max_plus_one > usize::MAX as u64 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "bounded read limit exceeds this platform's address space",
        ));
    }

    let components = relative_path
        .components()
        .map(|component| match component {
            Component::Normal(value) => Ok(value.to_os_string()),
            Component::Prefix(_)
            | Component::RootDir
            | Component::CurDir
            | Component::ParentDir => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "bounded read path must contain only normal relative components",
            )),
        })
        .collect::<io::Result<Vec<OsString>>>()?;
    let (leaf, directories) = components.split_last().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "bounded read path must name a file",
        )
    })?;

    let mut directory = open_authority_root(authority_root)?;
    after_directory_open(0)?;
    for (index, component) in directories.iter().enumerate() {
        directory = openat(
            &directory,
            component,
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
        .map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("bounded read refused directory component: {error}"),
            )
        })?;
        after_directory_open(index + 1)?;
    }

    let mut file = openat(
        &directory,
        leaf,
        libc::O_RDONLY | libc::O_NONBLOCK | libc::O_NOCTTY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
    )
    .map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("bounded read refused final component: {error}"),
        )
    })?;
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "bounded read final component is not a regular file",
        ));
    }
    if metadata.len() > max_bytes {
        return Err(limit_exceeded(max_bytes));
    }

    let before_identity = (metadata.dev(), metadata.ino());
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    (&mut file)
        .take(max_plus_one)
        .read_to_end(&mut bytes)
        .map_err(|error| {
            io::Error::new(
                error.kind(),
                format!("bounded read failed while reading retained file: {error}"),
            )
        })?;
    if bytes.len() as u64 > max_bytes {
        return Err(limit_exceeded(max_bytes));
    }

    let after_metadata = file.metadata()?;
    if !after_metadata.is_file() || (after_metadata.dev(), after_metadata.ino()) != before_identity
    {
        return Err(io::Error::other(
            "bounded read retained file identity changed unexpectedly",
        ));
    }
    Ok(bytes)
}

#[cfg(unix)]
fn open_authority_root(authority_root: &Path) -> io::Result<std::fs::File> {
    use std::os::unix::ffi::OsStrExt as _;

    let path = std::ffi::CString::new(authority_root.as_os_str().as_bytes()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "authority root contains a NUL byte",
        )
    })?;
    // SAFETY: `path` is NUL terminated. A successful descriptor is transferred
    // exactly once into the returned `File`.
    let descriptor = unsafe {
        libc::openat(
            libc::AT_FDCWD,
            path.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            0,
        )
    };
    file_from_descriptor(descriptor).map_err(|error| {
        io::Error::new(
            error.kind(),
            format!("bounded read refused authority root: {error}"),
        )
    })
}

#[cfg(unix)]
fn openat(
    directory: &std::fs::File,
    component: &std::ffi::OsStr,
    flags: libc::c_int,
) -> io::Result<std::fs::File> {
    use std::os::fd::AsRawFd as _;
    use std::os::unix::ffi::OsStrExt as _;

    let component = std::ffi::CString::new(component.as_bytes()).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "bounded read path component contains a NUL byte",
        )
    })?;
    // SAFETY: `component` is NUL terminated, `directory` owns a valid
    // descriptor, and a successful descriptor is transferred exactly once.
    let descriptor = unsafe { libc::openat(directory.as_raw_fd(), component.as_ptr(), flags, 0) };
    file_from_descriptor(descriptor)
}

#[cfg(unix)]
fn file_from_descriptor(descriptor: libc::c_int) -> io::Result<std::fs::File> {
    use std::os::fd::FromRawFd as _;

    if descriptor < 0 {
        Err(io::Error::last_os_error())
    } else {
        // SAFETY: the successful `openat` call returned a new owned descriptor.
        Ok(unsafe { std::fs::File::from_raw_fd(descriptor) })
    }
}

#[cfg(unix)]
fn limit_exceeded(max_bytes: u64) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("bounded read exceeds the {max_bytes}-byte limit"),
    )
}

#[cfg(all(test, unix))]
mod tests {
    use std::io;
    use std::path::Path;

    use super::read_file_beneath;
    use super::read_file_beneath_unix_with_hook;

    #[test]
    fn reads_a_regular_file_beneath_the_opened_root() -> io::Result<()> {
        let temp = tempfile::tempdir()?;
        std::fs::create_dir(temp.path().join("package"))?;
        std::fs::write(temp.path().join("package/resource.md"), b"expected")?;

        let bytes = read_file_beneath(temp.path(), Path::new("package/resource.md"), 8)?;

        assert_eq!(bytes, b"expected");
        Ok(())
    }

    #[test]
    fn rejects_oversize_sparse_file_without_reading_the_whole_file() -> io::Result<()> {
        let temp = tempfile::tempdir()?;
        let file = std::fs::File::create(temp.path().join("oversize"))?;
        file.set_len(1024 * 1024 * 1024)?;

        let error =
            read_file_beneath(temp.path(), Path::new("oversize"), 1024).expect_err("oversize");

        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert!(error.to_string().contains("1024-byte limit"));
        Ok(())
    }

    #[test]
    fn rejects_final_symlink() -> io::Result<()> {
        let temp = tempfile::tempdir()?;
        std::fs::write(temp.path().join("target"), b"outside")?;
        std::os::unix::fs::symlink("target", temp.path().join("link"))?;

        read_file_beneath(temp.path(), Path::new("link"), 64).expect_err("final symlink");
        Ok(())
    }

    #[test]
    fn rejects_intermediate_symlink() -> io::Result<()> {
        let temp = tempfile::tempdir()?;
        let outside = tempfile::tempdir()?;
        std::fs::write(outside.path().join("target"), b"outside")?;
        std::os::unix::fs::symlink(outside.path(), temp.path().join("link"))?;

        read_file_beneath(temp.path(), Path::new("link/target"), 64)
            .expect_err("intermediate symlink");
        Ok(())
    }

    #[test]
    fn retained_directory_fd_prevents_rename_swap_redirect() -> io::Result<()> {
        let temp = tempfile::tempdir()?;
        let nested = temp.path().join("nested");
        let retained = temp.path().join("retained");
        let outside = temp.path().join("outside");
        std::fs::create_dir(&nested)?;
        std::fs::create_dir(&outside)?;
        std::fs::write(nested.join("resource"), b"authorized")?;
        std::fs::write(outside.join("resource"), b"redirected")?;
        let mut swapped = false;

        let bytes = read_file_beneath_unix_with_hook(
            temp.path(),
            Path::new("nested/resource"),
            64,
            |depth| {
                if depth == 1 && !swapped {
                    std::fs::rename(&nested, &retained)?;
                    std::os::unix::fs::symlink(&outside, &nested)?;
                    swapped = true;
                }
                Ok(())
            },
        )?;

        assert!(swapped);
        assert_eq!(bytes, b"authorized");
        assert_eq!(std::fs::read(nested.join("resource"))?, b"redirected");
        Ok(())
    }
}
