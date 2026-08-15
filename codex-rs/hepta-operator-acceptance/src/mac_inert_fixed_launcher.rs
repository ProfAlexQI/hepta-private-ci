//! Fixed, closed-world Darwin launcher for the inert one-shot runner.
//!
//! The production path is a compile-time constant under a root-owned,
//! non-writable directory chain.  Its full-file SHA-256 is also a mandatory
//! compile-time packaging input.  Darwin has no usable `fexecve(2)`, so the
//! retained descriptors do not replace pathname execution: they make every
//! ancestor, the final inode, and the exact bytes a fail-closed pre/post
//! condition around `posix_spawn(3)`.
//!
//! `POSIX_SPAWN_CLOEXEC_DEFAULT` is the closed-world FD primitive.  The only
//! child descriptors are `/dev/null` at 0..=2 and three explicit `dup2` file
//! actions at 900..=902.  No caller path, inherited environment, PATH search,
//! or `current_exe()` value can construct the production binding.

use crate::mac_inert_runner_executable::FixedExecutableEvidenceV3;
use crate::mac_inert_runner_executable::FixedFileIdentityV3;
use crate::mac_inert_runner_executable::SpawnedInertChildV3;
use crate::mac_inert_runner_executable::executable_evidence;
use crate::mac_inert_runner_executable::file_identity;
#[cfg(test)]
use crate::mac_inert_runner_executable::inspect_executable_path;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::os::fd::RawFd;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;
use std::rc::Rc;

pub(crate) const FIXED_INERT_RUNNER_PATH_V3: &str =
    "/Library/PrivilegedHelperTools/com.hepta.inert-one-shot-runner-v3";
const FIXED_INERT_RUNNER_SHA256_ENV_V3: &str = "HEPTA_FIXED_INERT_RUNNER_SHA256_V3";
const MAX_FIXED_RUNNER_BYTES_V3: u64 = 512 * 1024 * 1024;
const CHILD_STDIO_FDS_V3: [RawFd; 3] = [0, 1, 2];
const POSIX_SPAWN_FLAGS_V3: libc::c_short = (libc::POSIX_SPAWN_CLOEXEC_DEFAULT
    | libc::POSIX_SPAWN_SETPGROUP
    | libc::POSIX_SPAWN_SETSIGDEF
    | libc::POSIX_SPAWN_SETSIGMASK) as libc::c_short;

struct RetainedDirectoryV3 {
    component: Option<CString>,
    descriptor: File,
    identity: FixedFileIdentityV3,
}

/// Non-cloneable retained capability for the one packaging-defined runner.
/// It deliberately exposes neither its descriptors nor its path.
pub(crate) struct FixedInertRunnerBindingV3 {
    binary: File,
    evidence: FixedExecutableEvidenceV3,
    expected_sha256: String,
    parent_chain: Vec<RetainedDirectoryV3>,
    spawn_path: CString,
    strict_production_policy: bool,
    _not_send_or_sync: PhantomData<Rc<()>>,
}

impl FixedInertRunnerBindingV3 {
    /// The only production constructor.  A packaging build which does not
    /// inject the expected full-file digest has no runner-launch capability.
    pub(crate) fn open_installed() -> io::Result<Self> {
        let expected_sha256 = option_env!("HEPTA_FIXED_INERT_RUNNER_SHA256_V3")
            .ok_or_else(|| {
                invalid(format!(
                    "missing compile-time {FIXED_INERT_RUNNER_SHA256_ENV_V3}; fixed runner is unavailable",
                ))
            })?;
        require_sha256(expected_sha256)?;
        Self::open_exact(Path::new(FIXED_INERT_RUNNER_PATH_V3), expected_sha256, true)
    }

    #[cfg(test)]
    pub(crate) fn open_for_test(path: &Path) -> io::Result<Self> {
        let canonical = path.canonicalize()?;
        let observed = inspect_executable_path(&canonical)?;
        Self::open_exact(&canonical, &observed.sha256, false)
    }

    #[cfg(test)]
    pub(crate) fn open_for_test_with_expected(
        path: &Path,
        expected_sha256: &str,
    ) -> io::Result<Self> {
        let canonical = path.canonicalize()?;
        Self::open_exact(&canonical, expected_sha256, false)
    }

    fn open_exact(path: &Path, expected_sha256: &str, strict: bool) -> io::Result<Self> {
        require_sha256(expected_sha256)?;
        if !path.is_absolute() {
            return Err(invalid("fixed runner path is not absolute"));
        }
        let spawn_path = CString::new(path.as_os_str().as_bytes())
            .map_err(|_| invalid("fixed runner path contains NUL"))?;
        let components = absolute_components(path)?;
        let (binary_name, parent_components) = components
            .split_last()
            .ok_or_else(|| invalid("fixed runner path has no basename"))?;

        let root = open_cstr(c"/", libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC)?;
        let root_identity = file_identity(root.as_raw_fd())?;
        validate_directory_policy(&root_identity, strict)?;
        if strict {
            verify_acl_absent(root.as_raw_fd())?;
        }
        let mut parent_chain = vec![RetainedDirectoryV3 {
            component: None,
            descriptor: root,
            identity: root_identity,
        }];
        for component in parent_components {
            let parent = parent_chain
                .last()
                .ok_or_else(|| invalid("fixed runner parent chain disappeared"))?;
            let descriptor = openat_cstr(
                parent.descriptor.as_raw_fd(),
                component,
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            )?;
            let identity = file_identity(descriptor.as_raw_fd())?;
            validate_directory_policy(&identity, strict)?;
            if strict {
                verify_acl_absent(descriptor.as_raw_fd())?;
            }
            parent_chain.push(RetainedDirectoryV3 {
                component: Some(component.clone()),
                descriptor,
                identity,
            });
        }
        let parent = parent_chain
            .last()
            .ok_or_else(|| invalid("fixed runner parent is missing"))?;
        let binary = openat_cstr(
            parent.descriptor.as_raw_fd(),
            binary_name,
            libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )?;
        let evidence = executable_evidence(&binary)?;
        validate_binary_policy(&evidence.identity, &parent.identity, strict)?;
        if strict {
            verify_acl_absent(binary.as_raw_fd())?;
        }
        if evidence.sha256 != expected_sha256 {
            return Err(invalid("fixed runner bytes differ from packaging digest"));
        }
        let binding = Self {
            binary,
            evidence,
            expected_sha256: expected_sha256.to_string(),
            parent_chain,
            spawn_path,
            strict_production_policy: strict,
            _not_send_or_sync: PhantomData,
        };
        binding.revalidate()?;
        Ok(binding)
    }

    pub(crate) fn evidence(&self) -> &FixedExecutableEvidenceV3 {
        &self.evidence
    }

    pub(crate) fn revalidate(&self) -> io::Result<()> {
        require_sha256(&self.expected_sha256)?;
        let retained_binary = executable_evidence(&self.binary)?;
        if retained_binary != self.evidence || retained_binary.sha256 != self.expected_sha256 {
            return Err(invalid("retained fixed runner inode or bytes changed"));
        }
        for retained in &self.parent_chain {
            let identity = file_identity(retained.descriptor.as_raw_fd())?;
            validate_directory_policy(&identity, self.strict_production_policy)?;
            if self.strict_production_policy {
                verify_acl_absent(retained.descriptor.as_raw_fd())?;
            }
            if identity != retained.identity {
                return Err(invalid("retained fixed runner ancestor changed"));
            }
        }

        let fresh_root = open_cstr(c"/", libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC)?;
        let fresh_root_identity = file_identity(fresh_root.as_raw_fd())?;
        if fresh_root_identity != self.parent_chain[0].identity {
            return Err(invalid("named root differs from retained root"));
        }
        if self.strict_production_policy {
            verify_acl_absent(fresh_root.as_raw_fd())?;
        }
        let mut fresh_parent = fresh_root;
        for retained in self.parent_chain.iter().skip(1) {
            let component = retained
                .component
                .as_ref()
                .ok_or_else(|| invalid("fixed runner ancestor lost its component"))?;
            let fresh = openat_cstr(
                fresh_parent.as_raw_fd(),
                component,
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            )?;
            if file_identity(fresh.as_raw_fd())? != retained.identity {
                return Err(invalid("named fixed runner ancestor was replaced"));
            }
            if self.strict_production_policy {
                verify_acl_absent(fresh.as_raw_fd())?;
            }
            fresh_parent = fresh;
        }
        let basename = self
            .spawn_path
            .as_c_str()
            .to_bytes()
            .rsplit(|byte| *byte == b'/')
            .next()
            .ok_or_else(|| invalid("fixed runner basename disappeared"))?;
        let basename =
            CString::new(basename).map_err(|_| invalid("fixed runner basename contains NUL"))?;
        let fresh_binary = openat_cstr(
            fresh_parent.as_raw_fd(),
            &basename,
            libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )?;
        let fresh_evidence = executable_evidence(&fresh_binary)?;
        validate_binary_policy(
            &fresh_evidence.identity,
            &self
                .parent_chain
                .last()
                .ok_or_else(|| invalid("fixed runner parent disappeared"))?
                .identity,
            self.strict_production_policy,
        )?;
        if self.strict_production_policy {
            verify_acl_absent(fresh_binary.as_raw_fd())?;
        }
        if fresh_evidence != self.evidence || fresh_evidence.sha256 != self.expected_sha256 {
            return Err(invalid("named fixed runner inode or bytes were replaced"));
        }
        Ok(())
    }

    /// Spawn one independent process group with a closed descriptor table.
    /// `argv` and `environment` are caller-built canonical C strings; the
    /// runner layer supplies only its fixed protocol variables (never the
    /// ambient environment).
    pub(crate) fn spawn_closed_world(
        &self,
        source_fds: [RawFd; 3],
        target_fds: [RawFd; 3],
        arguments: &[CString],
        environment: &[CString],
    ) -> io::Result<SpawnedInertChildV3> {
        self.revalidate()?;
        validate_fd_mapping(source_fds, target_fds)?;

        let mut actions = SpawnFileActionsV3::new()?;
        for fd in CHILD_STDIO_FDS_V3 {
            actions.add_open(fd, c"/dev/null", libc::O_RDWR, 0)?;
        }
        for (source, target) in source_fds.into_iter().zip(target_fds) {
            actions.add_dup2(source, target)?;
            actions.add_close(source)?;
        }
        let mut attributes = SpawnAttributesV3::new()?;
        attributes.configure_closed_world_process_group()?;
        let mut argv_values = Vec::with_capacity(arguments.len() + 1);
        argv_values.push(self.spawn_path.clone());
        argv_values.extend(arguments.iter().cloned());
        let mut argv_pointers = argv_values
            .iter()
            .map(|value| value.as_ptr().cast_mut())
            .collect::<Vec<_>>();
        argv_pointers.push(ptr::null_mut());
        let mut env_pointers = environment
            .iter()
            .map(|value| value.as_ptr().cast_mut())
            .collect::<Vec<_>>();
        env_pointers.push(ptr::null_mut());
        let mut pid = 0;
        let rc = unsafe {
            libc::posix_spawn(
                &mut pid,
                self.spawn_path.as_ptr(),
                &actions.raw,
                &attributes.raw,
                argv_pointers.as_ptr(),
                env_pointers.as_ptr(),
            )
        };
        if rc != 0 {
            return Err(io::Error::from_raw_os_error(rc));
        }
        if pid <= 0 {
            return Err(invalid("posix_spawn returned an invalid PID"));
        }
        let mut child = SpawnedInertChildV3 { pid, reaped: None };
        if let Err(error) = self.revalidate() {
            child.terminate_group_and_reap()?;
            return Err(error);
        }
        Ok(child)
    }
}

fn absolute_components(path: &Path) -> io::Result<Vec<CString>> {
    use std::path::Component;
    let mut result = Vec::new();
    for component in path.components() {
        match component {
            Component::RootDir => {}
            Component::Normal(value) => result.push(
                CString::new(value.as_bytes())
                    .map_err(|_| invalid("fixed runner component contains NUL"))?,
            ),
            _ => return Err(invalid("fixed runner path is not canonical absolute form")),
        }
    }
    Ok(result)
}

fn validate_fd_mapping(source_fds: [RawFd; 3], target_fds: [RawFd; 3]) -> io::Result<()> {
    if target_fds != [900, 901, 902] {
        return Err(invalid("fixed runner target FD allowlist changed"));
    }
    let mut all = source_fds.to_vec();
    all.extend(target_fds);
    all.extend(CHILD_STDIO_FDS_V3);
    all.sort_unstable();
    if all.iter().any(|fd| *fd < 0) || all.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(invalid("fixed runner source/target/stdin FD aliases"));
    }
    Ok(())
}

fn validate_directory_policy(identity: &FixedFileIdentityV3, strict: bool) -> io::Result<()> {
    if identity.file_type_and_mode & libc::S_IFMT != libc::S_IFDIR
        || identity.link_count == 0
        || identity.size < 0
    {
        return Err(invalid("fixed runner ancestor is not a stable directory"));
    }
    if strict
        && (identity.owner != 0
            || identity.group != 0
            || identity.file_type_and_mode & 0o777 != 0o755
            || identity.file_type_and_mode & (libc::S_ISUID | libc::S_ISGID) != 0)
    {
        return Err(invalid(
            "fixed runner ancestor is not exact root:wheel mode 0755",
        ));
    }
    Ok(())
}

fn validate_binary_policy(
    identity: &FixedFileIdentityV3,
    parent: &FixedFileIdentityV3,
    strict: bool,
) -> io::Result<()> {
    if identity.file_type_and_mode & libc::S_IFMT != libc::S_IFREG
        || identity.link_count != 1
        || identity.size <= 0
        || identity.size as u64 > MAX_FIXED_RUNNER_BYTES_V3
        || identity.device != parent.device
        || identity.file_type_and_mode & (libc::S_ISUID | libc::S_ISGID | libc::S_ISVTX) != 0
    {
        return Err(invalid("fixed runner inode policy is invalid"));
    }
    if strict
        && (identity.owner != 0
            || identity.group != 0
            || identity.file_type_and_mode & 0o777 != 0o755)
    {
        return Err(invalid("fixed runner is not exact root:wheel mode 0755"));
    }
    Ok(())
}

fn open_cstr(path: &CStr, flags: libc::c_int) -> io::Result<File> {
    let fd = unsafe { libc::open(path.as_ptr(), flags) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(unsafe { File::from_raw_fd(fd) })
}

fn openat_cstr(parent: RawFd, name: &CStr, flags: libc::c_int) -> io::Result<File> {
    let fd = unsafe { libc::openat(parent, name.as_ptr(), flags) };
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(unsafe { File::from_raw_fd(fd) })
}

fn require_sha256(value: &str) -> io::Result<()> {
    if value.len() != 64
        || !value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
    {
        return Err(invalid("fixed runner digest is not lowercase SHA-256"));
    }
    Ok(())
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

struct SpawnFileActionsV3 {
    raw: libc::posix_spawn_file_actions_t,
}

impl SpawnFileActionsV3 {
    fn new() -> io::Result<Self> {
        let mut raw = ptr::null_mut();
        check_spawn_rc(unsafe { libc::posix_spawn_file_actions_init(&mut raw) })?;
        Ok(Self { raw })
    }

    fn add_open(
        &mut self,
        fd: RawFd,
        path: &CStr,
        flags: libc::c_int,
        mode: libc::mode_t,
    ) -> io::Result<()> {
        check_spawn_rc(unsafe {
            libc::posix_spawn_file_actions_addopen(&mut self.raw, fd, path.as_ptr(), flags, mode)
        })
    }

    fn add_dup2(&mut self, source: RawFd, target: RawFd) -> io::Result<()> {
        check_spawn_rc(unsafe {
            libc::posix_spawn_file_actions_adddup2(&mut self.raw, source, target)
        })
    }

    fn add_close(&mut self, fd: RawFd) -> io::Result<()> {
        check_spawn_rc(unsafe { libc::posix_spawn_file_actions_addclose(&mut self.raw, fd) })
    }
}

impl Drop for SpawnFileActionsV3 {
    fn drop(&mut self) {
        unsafe {
            libc::posix_spawn_file_actions_destroy(&mut self.raw);
        }
    }
}

struct SpawnAttributesV3 {
    raw: libc::posix_spawnattr_t,
}

impl SpawnAttributesV3 {
    fn new() -> io::Result<Self> {
        let mut raw = ptr::null_mut();
        check_spawn_rc(unsafe { libc::posix_spawnattr_init(&mut raw) })?;
        Ok(Self { raw })
    }

    fn configure_closed_world_process_group(&mut self) -> io::Result<()> {
        check_spawn_rc(unsafe {
            libc::posix_spawnattr_setflags(&mut self.raw, POSIX_SPAWN_FLAGS_V3)
        })?;
        check_spawn_rc(unsafe { libc::posix_spawnattr_setpgroup(&mut self.raw, 0) })?;
        let mut empty = MaybeUninit::<libc::sigset_t>::zeroed();
        check_errno_rc(unsafe { libc::sigemptyset(empty.as_mut_ptr()) })?;
        let empty = unsafe { empty.assume_init() };
        check_spawn_rc(unsafe { libc::posix_spawnattr_setsigmask(&mut self.raw, &empty) })?;
        let mut defaults = MaybeUninit::<libc::sigset_t>::zeroed();
        check_errno_rc(unsafe { libc::sigfillset(defaults.as_mut_ptr()) })?;
        let mut defaults = unsafe { defaults.assume_init() };
        check_errno_rc(unsafe { libc::sigdelset(&mut defaults, libc::SIGKILL) })?;
        check_errno_rc(unsafe { libc::sigdelset(&mut defaults, libc::SIGSTOP) })?;
        check_spawn_rc(unsafe { libc::posix_spawnattr_setsigdefault(&mut self.raw, &defaults) })
    }
}

impl Drop for SpawnAttributesV3 {
    fn drop(&mut self) {
        unsafe {
            libc::posix_spawnattr_destroy(&mut self.raw);
        }
    }
}

fn check_spawn_rc(rc: libc::c_int) -> io::Result<()> {
    if rc == 0 {
        Ok(())
    } else {
        Err(io::Error::from_raw_os_error(rc))
    }
}

fn check_errno_rc(rc: libc::c_int) -> io::Result<()> {
    if rc == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

fn verify_acl_absent(fd: RawFd) -> io::Result<()> {
    const ACL_FIRST_ENTRY: libc::c_int = 0;
    const ACL_TYPE_EXTENDED: libc::c_int = 0x0000_0100;
    let acl = unsafe { acl_get_fd_np(fd, ACL_TYPE_EXTENDED) };
    if acl.is_null() {
        let error = io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ENOENT) {
            return Ok(());
        }
        return Err(error);
    }
    let mut entry = ptr::null_mut();
    let rc = unsafe { acl_get_entry(acl, ACL_FIRST_ENTRY, &mut entry) };
    let error = io::Error::last_os_error();
    if unsafe { acl_free(acl) } != 0 {
        return Err(io::Error::last_os_error());
    }
    match rc {
        0 => Err(invalid("fixed runner path has an extended ACL")),
        -1 if error.raw_os_error() == Some(libc::EINVAL) => Ok(()),
        _ => Err(error),
    }
}

unsafe extern "C" {
    fn acl_free(object: *mut libc::c_void) -> libc::c_int;
    fn acl_get_entry(
        acl: *mut libc::c_void,
        entry_id: libc::c_int,
        entry: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn acl_get_fd_np(fd: libc::c_int, acl_type: libc::c_int) -> *mut libc::c_void;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use std::sync::Mutex;
    use tempfile::TempDir;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn copied_runner() -> (TempDir, PathBuf) {
        let directory = TempDir::new().expect("fixed runner test root");
        let parent = directory.path().join("trusted");
        fs::create_dir(&parent).expect("fixed runner test parent");
        let runner = parent.join("runner");
        fs::copy(
            std::env::current_exe().expect("current test executable"),
            &runner,
        )
        .expect("copy fixed runner fixture");
        fs::set_permissions(&runner, fs::Permissions::from_mode(0o755))
            .expect("fixed runner fixture mode");
        (directory, runner)
    }

    #[test]
    fn fixed_binding_rejects_wrong_packaging_digest() {
        let _guard = TEST_LOCK.lock().expect("fixed launcher test lock");
        let (_directory, runner) = copied_runner();
        let error =
            FixedInertRunnerBindingV3::open_for_test_with_expected(&runner, &"00".repeat(32))
                .err()
                .expect("wrong fixed runner digest");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn fixed_binding_rejects_same_bytes_at_a_replacement_inode() {
        let _guard = TEST_LOCK.lock().expect("fixed launcher test lock");
        let (_directory, runner) = copied_runner();
        let binding = FixedInertRunnerBindingV3::open_for_test(&runner).expect("fixed binding");
        let displaced = runner.with_extension("displaced");
        fs::rename(&runner, &displaced).expect("displace retained runner");
        fs::copy(&displaced, &runner).expect("publish same fixed runner bytes at a new inode");
        fs::set_permissions(&runner, fs::Permissions::from_mode(0o755))
            .expect("replacement runner mode");
        assert!(binding.revalidate().is_err());
    }

    #[test]
    fn fixed_binding_rejects_a_replaced_parent_directory() {
        let _guard = TEST_LOCK.lock().expect("fixed launcher test lock");
        let (directory, runner) = copied_runner();
        let binding = FixedInertRunnerBindingV3::open_for_test(&runner).expect("fixed binding");
        let retained_parent = runner.parent().expect("runner parent");
        let displaced = directory.path().join("trusted-displaced");
        fs::rename(retained_parent, &displaced).expect("displace retained runner parent");
        fs::create_dir(retained_parent).expect("replacement runner parent");
        let replacement = retained_parent.join("runner");
        fs::copy(displaced.join("runner"), &replacement).expect("replacement runner bytes");
        fs::set_permissions(&replacement, fs::Permissions::from_mode(0o755))
            .expect("replacement runner mode");
        assert!(binding.revalidate().is_err());
    }

    #[test]
    fn fixed_binding_rejects_every_binary_special_mode_bit() {
        let _guard = TEST_LOCK.lock().expect("fixed launcher test lock");
        for special in [libc::S_ISUID, libc::S_ISGID, libc::S_ISVTX] {
            let (_directory, runner) = copied_runner();
            fs::set_permissions(
                &runner,
                fs::Permissions::from_mode(0o755 | u32::from(special)),
            )
            .expect("special runner mode");
            assert!(FixedInertRunnerBindingV3::open_for_test(&runner).is_err());
        }
    }

    #[test]
    fn closed_world_mapping_is_exact_and_alias_free() {
        let _guard = TEST_LOCK.lock().expect("fixed launcher test lock");
        let sources = [
            File::open("/dev/null").expect("first source"),
            File::open("/dev/null").expect("second source"),
            File::open("/dev/null").expect("third source"),
        ];
        let source_fds = sources.map(|source| source.as_raw_fd());
        validate_fd_mapping(source_fds, [900, 901, 902]).expect("exact mapping");
        assert!(validate_fd_mapping(source_fds, [900, 901, 903]).is_err());
        assert!(
            validate_fd_mapping(
                [source_fds[0], source_fds[0], source_fds[2]],
                [900, 901, 902]
            )
            .is_err()
        );
    }
}
