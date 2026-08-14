use super::NativeSysErrorV8;
use super::NativeSysResultV8;
use super::PidHandleV8;
#[cfg(target_os = "linux")]
use super::pidfd_open_verified_with_procfs_v8;

#[cfg(target_os = "linux")]
use super::FileIdentityV8;
#[cfg(target_os = "linux")]
use super::io_error;
#[cfg(not(target_os = "linux"))]
use super::unsupported;
#[cfg(target_os = "linux")]
use sha2::Digest as _;
#[cfg(any(target_os = "linux", test))]
use std::collections::BTreeSet;
#[cfg(target_os = "linux")]
use std::ffi::CStr;
#[cfg(target_os = "linux")]
use std::ffi::CString;
#[cfg(target_os = "linux")]
use std::os::fd::AsRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::FromRawFd;
#[cfg(target_os = "linux")]
use std::os::fd::OwnedFd;
#[cfg(target_os = "linux")]
use std::time::Duration;
#[cfg(target_os = "linux")]
use std::time::Instant;

#[cfg(target_os = "linux")]
const PROC_SUPER_MAGIC_V8: libc::c_long = 0x9fa0;
#[cfg(target_os = "linux")]
const MAX_PROC_STAT_BYTES_V8: usize = 16 * 1024;
#[cfg(target_os = "linux")]
const MAX_PROC_SMALL_FILE_BYTES_V8: usize = 64 * 1024;
#[cfg(any(target_os = "linux", test))]
const MAX_PROC_MOUNTINFO_ENTRIES_V8: usize = 1024;
#[cfg(target_os = "linux")]
const MAX_PROC_NUMERIC_ENTRIES_V8: usize = 131_072;
#[cfg(target_os = "linux")]
const MAX_PROC_SCAN_ELAPSED_V8: Duration = Duration::from_secs(5);
#[cfg(target_os = "linux")]
const NSFS_SUPER_MAGIC_V8: libc::c_long = 0x6e73_6673;
#[cfg(target_os = "linux")]
const MAX_PROCESS_EXECUTABLE_BYTES_V8: u64 = 512 * 1024 * 1024;
#[cfg(target_os = "linux")]
const MAX_PROCESS_CAPTURE_ELAPSED_V8: Duration = Duration::from_secs(10);
#[cfg(all(test, target_os = "linux"))]
static FULL_EXECUTABLE_HASH_COUNT_V8: std::sync::atomic::AtomicUsize =
    std::sync::atomic::AtomicUsize::new(0);

/// Numeric mount and namespace binding for the process doing the observation.
/// The mount IDs come from that numeric process's own mountinfo and are
/// cross-checked against the retained descriptors through numeric fdinfo.
#[cfg(target_os = "linux")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcfsObserverBindingV8 {
    pub(crate) observer_pid: u32,
    pub(crate) observer_tid: u32,
    pub(crate) proc_mount_id: u64,
    pub(crate) cgroup_mount_id: u64,
    pub(crate) pid_namespace_device: u64,
    pub(crate) pid_namespace_inode: u64,
    pub(crate) cgroup_namespace_device: u64,
    pub(crate) cgroup_namespace_inode: u64,
    pub(crate) mount_namespace_device: u64,
    pub(crate) mount_namespace_inode: u64,
}

#[cfg(any(target_os = "linux", test))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ParsedMountTopologyV8 {
    pub(super) proc_mount_id: u64,
    pub(super) cgroup_mount_id: u64,
}

/// Shared aggregate budget for one exact multi-process capture.  Every full
/// executable pass is charged before I/O, including the second stability pass.
#[cfg(target_os = "linux")]
#[derive(Debug)]
pub(crate) struct ProcessCaptureBudgetV8 {
    remaining_processes: usize,
    remaining_hash_bytes: u64,
    deadline: Instant,
}

#[cfg(target_os = "linux")]
impl ProcessCaptureBudgetV8 {
    pub(crate) fn new(
        maximum_processes: usize,
        maximum_hash_bytes: u64,
        maximum_elapsed: Duration,
    ) -> NativeSysResultV8<Self> {
        if maximum_processes == 0
            || maximum_processes > 128
            || maximum_hash_bytes == 0
            || maximum_hash_bytes > 2 * 1024 * 1024 * 1024
            || maximum_elapsed.is_zero()
            || maximum_elapsed > MAX_PROCESS_CAPTURE_ELAPSED_V8
        {
            return Err(NativeSysErrorV8::InvalidInput(
                "process capture aggregate budget is outside frozen bounds".to_string(),
            ));
        }
        Ok(Self {
            remaining_processes: maximum_processes,
            remaining_hash_bytes: maximum_hash_bytes,
            deadline: Instant::now() + maximum_elapsed,
        })
    }

    fn begin_process(&mut self) -> NativeSysResultV8<()> {
        self.check_time()?;
        self.remaining_processes = self.remaining_processes.checked_sub(1).ok_or_else(|| {
            NativeSysErrorV8::IdentityMismatch(
                "process capture exceeds aggregate entry bound".to_string(),
            )
        })?;
        Ok(())
    }

    fn charge_hash(&mut self, bytes: u64) -> NativeSysResultV8<()> {
        self.check_time()?;
        self.remaining_hash_bytes =
            self.remaining_hash_bytes
                .checked_sub(bytes)
                .ok_or_else(|| {
                    NativeSysErrorV8::IdentityMismatch(
                        "process capture exceeds aggregate executable hash-byte bound".to_string(),
                    )
                })?;
        Ok(())
    }

    fn check_time(&self) -> NativeSysResultV8<()> {
        if Instant::now() > self.deadline {
            return Err(NativeSysErrorV8::RaceDetected(
                "process capture exceeded aggregate elapsed-time bound".to_string(),
            ));
        }
        Ok(())
    }
}

/// Kernel process state parsed from the exact third field of `/proc/PID/stat`.
/// Unknown state bytes are rejected instead of being treated as runnable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessStateV8 {
    Running,
    Sleeping,
    DiskSleep,
    Zombie,
    Stopped,
    TracingStop,
    Dead,
    Wakekill,
    Waking,
    Parked,
    Idle,
}

impl ProcessStateV8 {
    pub fn is_stopped(self) -> bool {
        matches!(self, Self::Stopped | Self::TracingStop)
    }

    pub fn as_proc_stat_byte(self) -> u8 {
        match self {
            Self::Running => b'R',
            Self::Sleeping => b'S',
            Self::DiskSleep => b'D',
            Self::Zombie => b'Z',
            Self::Stopped => b'T',
            Self::TracingStop => b't',
            Self::Dead => b'X',
            Self::Wakekill => b'K',
            Self::Waking => b'W',
            Self::Parked => b'P',
            Self::Idle => b'I',
        }
    }
}

/// Descriptor-derived executable identity for a live process.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessExecutableIdentityV8 {
    device: u64,
    inode: u64,
    size: u64,
    mode: u32,
    owner_uid: u32,
    owner_gid: u32,
    link_count: u64,
    mtime_seconds: i64,
    mtime_nanoseconds: i64,
    ctime_seconds: i64,
    ctime_nanoseconds: i64,
    sha256: [u8; 32],
}

impl ProcessExecutableIdentityV8 {
    pub fn device(&self) -> u64 {
        self.device
    }

    pub fn inode(&self) -> u64 {
        self.inode
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn sha256(&self) -> [u8; 32] {
        self.sha256
    }

    #[cfg(target_os = "linux")]
    fn matches_metadata(&self, metadata: &ProcessExecutableMetadataV8) -> bool {
        self.device == metadata.device
            && self.inode == metadata.inode
            && self.size == metadata.size
            && self.mode == metadata.mode
            && self.owner_uid == metadata.owner_uid
            && self.owner_gid == metadata.owner_gid
            && self.link_count == metadata.link_count
            && self.mtime_seconds == metadata.mtime_seconds
            && self.mtime_nanoseconds == metadata.mtime_nanoseconds
            && self.ctime_seconds == metadata.ctime_seconds
            && self.ctime_nanoseconds == metadata.ctime_nanoseconds
    }
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ProcessExecutableMetadataV8 {
    device: u64,
    inode: u64,
    size: u64,
    mode: u32,
    owner_uid: u32,
    owner_gid: u32,
    link_count: u64,
    mtime_seconds: i64,
    mtime_nanoseconds: i64,
    ctime_seconds: i64,
    ctime_nanoseconds: i64,
}

/// Exact stable and current-state fields for one process observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessIdentityV8 {
    pid: u32,
    start_ticks: u64,
    parent_pid: u32,
    process_group_id: u32,
    session_id: u32,
    state: ProcessStateV8,
    executable: ProcessExecutableIdentityV8,
}

#[cfg(target_os = "linux")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessRuntimeBindingV8 {
    pub(crate) uid: u32,
    pub(crate) gid: u32,
    pub(crate) argv_sha256: [u8; 32],
    pub(crate) cwd_device: u64,
    pub(crate) cwd_inode: u64,
}

/// Explicit union selector for one detached workload.  It is observation
/// scope, not authority: a future run profile must bind these exact fields to
/// independently verified systemd-terminal evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DetachedWorkloadSelectorV8 {
    cgroup_v2_path: String,
    process_group_id: u32,
    session_id: u32,
}

impl DetachedWorkloadSelectorV8 {
    pub fn new(
        cgroup_v2_path: String,
        process_group_id: u32,
        session_id: u32,
    ) -> NativeSysResultV8<Self> {
        validate_detached_workload_selector_v8(&cgroup_v2_path, process_group_id, session_id)?;
        Ok(Self {
            cgroup_v2_path,
            process_group_id,
            session_id,
        })
    }

    pub fn cgroup_v2_path(&self) -> &str {
        &self.cgroup_v2_path
    }

    pub fn process_group_id(&self) -> u32 {
        self.process_group_id
    }

    pub fn session_id(&self) -> u32 {
        self.session_id
    }
}

/// One pidfd-bound member of the selector's global PGID/SID/cgroup union.
#[derive(Debug)]
pub struct DetachedWorkloadProcessV8 {
    cgroup_v2_path: String,
    observation: ProcessObservationV8,
}

impl DetachedWorkloadProcessV8 {
    pub fn cgroup_v2_path(&self) -> &str {
        &self.cgroup_v2_path
    }

    pub fn identity(&self) -> &ProcessIdentityV8 {
        self.observation.identity()
    }
}

/// Stable, globally enumerated union closure.  An empty member list proves
/// sampled absence across the exact observation window; it does not prove a
/// continuous admission barrier and grants no qualification/run authority.
#[derive(Debug)]
pub struct DetachedWorkloadClosureV8 {
    selector: DetachedWorkloadSelectorV8,
    processes: Vec<DetachedWorkloadProcessV8>,
    binding_sha256: [u8; 32],
}

impl DetachedWorkloadClosureV8 {
    pub fn selector(&self) -> &DetachedWorkloadSelectorV8 {
        &self.selector
    }

    pub fn processes(&self) -> &[DetachedWorkloadProcessV8] {
        &self.processes
    }

    pub fn binding_sha256(&self) -> [u8; 32] {
        self.binding_sha256
    }

    pub fn revalidate(&self) -> NativeSysResultV8<()> {
        revalidate_detached_workload_closure_impl_v8(self)
    }
}

impl ProcessIdentityV8 {
    pub fn pid(&self) -> u32 {
        self.pid
    }

    pub fn start_ticks(&self) -> u64 {
        self.start_ticks
    }

    pub fn parent_pid(&self) -> u32 {
        self.parent_pid
    }

    pub fn process_group_id(&self) -> u32 {
        self.process_group_id
    }

    pub fn session_id(&self) -> u32 {
        self.session_id
    }

    pub fn state(&self) -> ProcessStateV8 {
        self.state
    }

    pub fn executable(&self) -> &ProcessExecutableIdentityV8 {
        &self.executable
    }

    #[cfg(target_os = "linux")]
    pub(crate) fn has_same_stable_identity(&self, other: &Self) -> bool {
        self.pid == other.pid
            && self.start_ticks == other.start_ticks
            && self.parent_pid == other.parent_pid
            && self.process_group_id == other.process_group_id
            && self.session_id == other.session_id
            && self.executable == other.executable
    }
}

/// A pidfd-bound observation. It is intentionally not cloneable or
/// deserializable: revalidation must retain the original kernel handle.
#[derive(Debug)]
pub struct ProcessObservationV8 {
    identity: ProcessIdentityV8,
    handle: PidHandleV8,
}

impl ProcessObservationV8 {
    pub fn identity(&self) -> &ProcessIdentityV8 {
        &self.identity
    }

    pub fn pid(&self) -> u32 {
        self.identity.pid
    }

    pub fn is_exited(&self) -> NativeSysResultV8<bool> {
        self.handle.is_exited()
    }

    /// Re-reads stat and the executable through the fixed procfs mount, then
    /// proves that every stable field still matches this pidfd-bound token.
    /// State is returned rather than pinned because SIGSTOP/SIGCONT are the
    /// only consumers permitted to change it.
    pub fn revalidate(&self) -> NativeSysResultV8<ProcessIdentityV8> {
        revalidate_process_observation_impl_v8(self)
    }

    /// Fast pidfd/stat/executable-metadata revalidation used only for bounded
    /// state polling. The original full digest remains pinned and a complete
    /// digest revalidation is mandatory before any success is returned.
    #[cfg(target_os = "linux")]
    pub(crate) fn revalidate_fast_with_root(
        &self,
        root: &ProcfsRootV8,
    ) -> NativeSysResultV8<ProcessIdentityV8> {
        revalidate_process_observation_fast_v8(self, root)
    }
}

/// Opens a pidfd and captures an exact process identity through the fixed
/// procfs mount. Numeric PID reuse, exec replacement, reparenting, group/session
/// drift, and executable mutation are all fail-closed observations.
pub fn observe_process_exact_v8(pid: u32) -> NativeSysResultV8<ProcessObservationV8> {
    observe_process_exact_impl_v8(pid)
}

/// Enumerates all numeric `/proc` entries and captures the exact union of
/// processes that match the supplied PGID, SID, or cgroup-v2 path. This is the
/// native preflight needed to detect a detached compiler/build tree after its
/// systemd unit has already reported terminal.
pub fn observe_detached_workload_closure_v8(
    selector: DetachedWorkloadSelectorV8,
) -> NativeSysResultV8<DetachedWorkloadClosureV8> {
    observe_detached_workload_closure_impl_v8(selector)
}

#[cfg(target_os = "linux")]
#[derive(Debug)]
pub(crate) struct ProcfsRootV8 {
    descriptor: OwnedFd,
    identity: FileIdentityV8,
}

#[cfg(target_os = "linux")]
impl ProcfsRootV8 {
    pub(crate) fn open_fixed() -> NativeSysResultV8<Self> {
        let descriptor = open_fixed_procfs_descriptor_v8()?;
        let identity = super::openat2::identity_for_fd(descriptor.as_raw_fd())?;
        require_procfs_root_v8(descriptor.as_raw_fd(), identity)?;
        Ok(Self {
            descriptor,
            identity,
        })
    }

    pub(crate) fn revalidate(&self) -> NativeSysResultV8<()> {
        let observed = super::openat2::identity_for_fd(self.descriptor.as_raw_fd())?;
        if !observed.matches_stable_directory(self.identity) {
            return Err(NativeSysErrorV8::RaceDetected(
                "opened procfs root identity changed or was unlinked".to_string(),
            ));
        }
        require_procfs_root_v8(self.descriptor.as_raw_fd(), observed)?;

        let reopened = open_fixed_procfs_descriptor_v8()?;
        let reopened_identity = super::openat2::identity_for_fd(reopened.as_raw_fd())?;
        require_procfs_root_v8(reopened.as_raw_fd(), reopened_identity)?;
        if !reopened_identity.matches_stable_directory(self.identity) {
            return Err(NativeSysErrorV8::RaceDetected(
                "fixed /proc pathname no longer names the opened procfs root".to_string(),
            ));
        }
        Ok(())
    }

    pub(crate) fn read_regular_beneath(
        &self,
        relative: &str,
        maximum_bytes: usize,
    ) -> NativeSysResultV8<Vec<u8>> {
        self.revalidate()?;
        let descriptor = openat2_procfs_v8(
            self.descriptor.as_raw_fd(),
            relative,
            libc::O_RDONLY | libc::O_CLOEXEC,
        )?;
        require_procfs_file_v8(descriptor.as_raw_fd())?;
        let bytes = read_fd_bounded_v8(descriptor.as_raw_fd(), maximum_bytes)?;
        self.revalidate()?;
        Ok(bytes)
    }

    /// Binds this retained procfs descriptor to the numeric observer's mount
    /// table and PID/cgroup namespaces.  Magic `self` links are deliberately
    /// never used.
    pub(crate) fn observer_binding(&self) -> NativeSysResultV8<ProcfsObserverBindingV8> {
        // SAFETY: getpid has no pointer arguments or preconditions.
        let observer_pid = unsafe { libc::getpid() };
        let observer_pid = u32::try_from(observer_pid).map_err(|_| {
            NativeSysErrorV8::IdentityMismatch(
                "observer pid does not fit a positive u32".to_string(),
            )
        })?;
        // Mount namespaces are per-thread attributes. Bind mountinfo, fdinfo,
        // and ns/mnt to this exact numeric TID while retaining the process ID
        // separately for cgroup.procs membership checks.
        // SAFETY: gettid has no pointer arguments or preconditions.
        let observer_tid = unsafe { libc::gettid() };
        let observer_tid = u32::try_from(observer_tid).map_err(|_| {
            NativeSysErrorV8::IdentityMismatch(
                "observer tid does not fit a positive u32".to_string(),
            )
        })?;
        if observer_pid == 0 || observer_tid == 0 {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "numeric observer pid/tid must both be positive".to_string(),
            ));
        }
        let stat_before = self.read_process_stat(observer_tid)?;
        let mountinfo = self.read_regular_beneath(
            &format!("{observer_tid}/mountinfo"),
            MAX_PROC_SMALL_FILE_BYTES_V8,
        )?;
        let topology = parse_mountinfo_topology_v8(&mountinfo)?;
        let proc_mount_id =
            self.mount_id_for_observer_fd(observer_tid, self.descriptor.as_raw_fd())?;
        if proc_mount_id != topology.proc_mount_id {
            return Err(NativeSysErrorV8::IdentityMismatch(format!(
                "retained procfs descriptor mount id {proc_mount_id} differs from numeric mountinfo id {}",
                topology.proc_mount_id
            )));
        }
        let mount_namespace = self.open_process_namespace(observer_tid, "mnt")?;
        let mount_namespace_identity =
            super::openat2::identity_for_fd(mount_namespace.as_raw_fd())?;
        require_namespace_descriptor_v8(mount_namespace.as_raw_fd())?;
        let pid_namespace = self.open_process_namespace(observer_tid, "pid")?;
        let pid_namespace_identity = super::openat2::identity_for_fd(pid_namespace.as_raw_fd())?;
        require_namespace_descriptor_v8(pid_namespace.as_raw_fd())?;
        let cgroup_namespace = self.open_process_namespace(observer_tid, "cgroup")?;
        let cgroup_namespace_identity =
            super::openat2::identity_for_fd(cgroup_namespace.as_raw_fd())?;
        require_namespace_descriptor_v8(cgroup_namespace.as_raw_fd())?;
        // Reopen the numeric mount namespace and prove the retained descriptor
        // still names the same nsfs dev+ino before returning plain evidence.
        let reopened_mount_namespace = self.open_process_namespace(observer_tid, "mnt")?;
        let reopened_mount_namespace_identity =
            super::openat2::identity_for_fd(reopened_mount_namespace.as_raw_fd())?;
        require_namespace_descriptor_v8(reopened_mount_namespace.as_raw_fd())?;
        if reopened_mount_namespace_identity != mount_namespace_identity {
            return Err(NativeSysErrorV8::RaceDetected(
                "numeric observer mount namespace changed during binding".to_string(),
            ));
        }
        let stat_after = self.read_process_stat(observer_tid)?;
        require_same_stable_stat_v8(&stat_before, &stat_after)?;
        self.revalidate()?;
        Ok(ProcfsObserverBindingV8 {
            observer_pid,
            observer_tid,
            proc_mount_id,
            cgroup_mount_id: topology.cgroup_mount_id,
            pid_namespace_device: pid_namespace_identity.device(),
            pid_namespace_inode: pid_namespace_identity.inode(),
            cgroup_namespace_device: cgroup_namespace_identity.device(),
            cgroup_namespace_inode: cgroup_namespace_identity.inode(),
            mount_namespace_device: mount_namespace_identity.device(),
            mount_namespace_inode: mount_namespace_identity.inode(),
        })
    }

    pub(crate) fn mount_id_for_observer_fd(
        &self,
        observer_pid: u32,
        fd: libc::c_int,
    ) -> NativeSysResultV8<u64> {
        if fd < 0 {
            return Err(NativeSysErrorV8::InvalidInput(
                "descriptor for mount-id binding must be nonnegative".to_string(),
            ));
        }
        let relative = format!("{observer_pid}/fdinfo/{fd}");
        let before = parse_fdinfo_mount_id_v8(
            &self.read_regular_beneath(&relative, MAX_PROC_STAT_BYTES_V8)?,
        )?;
        let after = parse_fdinfo_mount_id_v8(
            &self.read_regular_beneath(&relative, MAX_PROC_STAT_BYTES_V8)?,
        )?;
        if before != after {
            return Err(NativeSysErrorV8::RaceDetected(
                "descriptor mount id changed across numeric fdinfo reads".to_string(),
            ));
        }
        Ok(after)
    }

    pub(crate) fn open_process_directory(&self, pid: u32) -> NativeSysResultV8<OwnedFd> {
        self.revalidate()?;
        let descriptor = openat2_procfs_v8(
            self.descriptor.as_raw_fd(),
            &pid.to_string(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
        )?;
        require_procfs_directory_v8(descriptor.as_raw_fd())?;
        Ok(descriptor)
    }

    fn open_process_namespace(&self, pid: u32, namespace: &str) -> NativeSysResultV8<OwnedFd> {
        if !matches!(namespace, "pid" | "cgroup" | "mnt") {
            return Err(NativeSysErrorV8::InvalidInput(
                "only pid, cgroup, and mount namespace identities are admitted".to_string(),
            ));
        }
        let process = self.open_process_directory(pid)?;
        let relative = CString::new(format!("ns/{namespace}")).map_err(|_| {
            NativeSysErrorV8::InvalidInput("namespace path contains NUL".to_string())
        })?;
        // Namespace entries are kernel magic links.  Following one relative to
        // the already pidfd/stat-bound numeric process directory yields an
        // nsfs descriptor whose dev+inode is the namespace identity.
        // SAFETY: both descriptor and C string remain live; openat retains no
        // pointer or ownership.
        let raw_fd = unsafe {
            libc::openat(
                process.as_raw_fd(),
                relative.as_ptr(),
                libc::O_RDONLY | libc::O_CLOEXEC,
            )
        };
        if raw_fd < 0 {
            return Err(io_error(
                "open numeric process namespace descriptor",
                std::io::Error::last_os_error(),
            ));
        }
        // SAFETY: successful openat returns a uniquely owned descriptor.
        Ok(unsafe { OwnedFd::from_raw_fd(raw_fd) })
    }

    pub(super) fn read_process_stat(&self, pid: u32) -> NativeSysResultV8<ParsedProcStatV8> {
        let process = self.open_process_directory(pid)?;
        let descriptor = openat2_procfs_v8(
            process.as_raw_fd(),
            "stat",
            libc::O_RDONLY | libc::O_CLOEXEC,
        )?;
        require_procfs_file_v8(descriptor.as_raw_fd())?;
        let bytes = read_fd_bounded_v8(descriptor.as_raw_fd(), MAX_PROC_STAT_BYTES_V8)?;
        let parsed = parse_proc_stat_v8(&bytes, Some(pid))?;
        self.revalidate()?;
        Ok(parsed)
    }

    fn read_process_regular(
        &self,
        pid: u32,
        name: &str,
        maximum_bytes: usize,
    ) -> NativeSysResultV8<Vec<u8>> {
        let process = self.open_process_directory(pid)?;
        let descriptor =
            openat2_procfs_v8(process.as_raw_fd(), name, libc::O_RDONLY | libc::O_CLOEXEC)?;
        require_procfs_file_v8(descriptor.as_raw_fd())?;
        let bytes = read_fd_bounded_v8(descriptor.as_raw_fd(), maximum_bytes)?;
        self.revalidate()?;
        Ok(bytes)
    }

    fn read_process_cgroup_path(&self, pid: u32) -> NativeSysResultV8<String> {
        let bytes = self.read_process_regular(pid, "cgroup", MAX_PROC_SMALL_FILE_BYTES_V8)?;
        super::parse_self_cgroup_path_v8(&bytes)
    }

    pub(crate) fn observe_process_runtime_binding_v8(
        &self,
        process: &ProcessObservationV8,
    ) -> NativeSysResultV8<ProcessRuntimeBindingV8> {
        let before = process.revalidate_fast_with_root(self)?;
        let status =
            self.read_process_regular(process.pid(), "status", MAX_PROC_SMALL_FILE_BYTES_V8)?;
        let (uid, gid) = parse_process_status_credentials_v8(&status)?;
        let cmdline =
            self.read_process_regular(process.pid(), "cmdline", MAX_PROC_SMALL_FILE_BYTES_V8)?;
        validate_process_cmdline_v8(&cmdline)?;
        let argv_sha256 = sha2::Sha256::digest(&cmdline).into();
        let cwd_before = self.open_process_magic_directory(process.pid(), "cwd")?;
        let cwd_before_identity = super::openat2::identity_for_fd(cwd_before.as_raw_fd())?;
        require_process_directory_target_v8(cwd_before.as_raw_fd(), cwd_before_identity)?;
        let cwd_after = self.open_process_magic_directory(process.pid(), "cwd")?;
        let cwd_after_identity = super::openat2::identity_for_fd(cwd_after.as_raw_fd())?;
        require_process_directory_target_v8(cwd_after.as_raw_fd(), cwd_after_identity)?;
        if !cwd_after_identity.matches_stable_directory(cwd_before_identity) {
            return Err(NativeSysErrorV8::RaceDetected(format!(
                "pid {} cwd identity changed during runtime binding",
                process.pid()
            )));
        }
        let after = process.revalidate_fast_with_root(self)?;
        if !before.has_same_stable_identity(&after) {
            return Err(NativeSysErrorV8::RaceDetected(format!(
                "pid {} stable identity changed during runtime binding",
                process.pid()
            )));
        }
        Ok(ProcessRuntimeBindingV8 {
            uid,
            gid,
            argv_sha256,
            cwd_device: cwd_after_identity.device(),
            cwd_inode: cwd_after_identity.inode(),
        })
    }

    fn open_process_magic_directory(&self, pid: u32, name: &str) -> NativeSysResultV8<OwnedFd> {
        if name != "cwd" {
            return Err(NativeSysErrorV8::InvalidInput(
                "unsupported numeric process magic directory".to_string(),
            ));
        }
        let process = self.open_process_directory(pid)?;
        let name = CString::new(name).map_err(|_| {
            NativeSysErrorV8::InvalidInput("process magic directory contains NUL".to_string())
        })?;
        // SAFETY: descriptor and C string remain live; openat follows the
        // kernel cwd magic link to one pinned directory descriptor.
        let raw_fd = unsafe {
            libc::openat(
                process.as_raw_fd(),
                name.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC,
            )
        };
        if raw_fd < 0 {
            return Err(io_error(
                "open numeric process cwd descriptor",
                std::io::Error::last_os_error(),
            ));
        }
        // SAFETY: successful openat returned a uniquely owned descriptor.
        Ok(unsafe { OwnedFd::from_raw_fd(raw_fd) })
    }

    fn observe_executable(
        &self,
        pid: u32,
        budget: &mut ProcessCaptureBudgetV8,
    ) -> NativeSysResultV8<ProcessExecutableIdentityV8> {
        let descriptor = self.open_process_executable(pid)?;
        let before_identity = super::openat2::identity_for_fd(descriptor.as_raw_fd())?;
        let before = process_executable_metadata_v8(descriptor.as_raw_fd(), before_identity)?;
        budget.charge_hash(before.size)?;
        #[cfg(test)]
        FULL_EXECUTABLE_HASH_COUNT_V8.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let sha256 = sha256_exact_fd_v8(descriptor.as_raw_fd(), before.size)?;
        let after_identity = super::openat2::identity_for_fd(descriptor.as_raw_fd())?;
        let after = process_executable_metadata_v8(descriptor.as_raw_fd(), after_identity)?;
        if before != after {
            return Err(NativeSysErrorV8::RaceDetected(
                "process executable metadata changed while hashing".to_string(),
            ));
        }
        self.revalidate()?;
        Ok(ProcessExecutableIdentityV8 {
            device: before.device,
            inode: before.inode,
            size: before.size,
            mode: before.mode,
            owner_uid: before.owner_uid,
            owner_gid: before.owner_gid,
            link_count: before.link_count,
            mtime_seconds: before.mtime_seconds,
            mtime_nanoseconds: before.mtime_nanoseconds,
            ctime_seconds: before.ctime_seconds,
            ctime_nanoseconds: before.ctime_nanoseconds,
            sha256,
        })
    }

    fn observe_executable_metadata(
        &self,
        pid: u32,
    ) -> NativeSysResultV8<ProcessExecutableMetadataV8> {
        let descriptor = self.open_process_executable(pid)?;
        let before_identity = super::openat2::identity_for_fd(descriptor.as_raw_fd())?;
        let before = process_executable_metadata_v8(descriptor.as_raw_fd(), before_identity)?;
        let after_identity = super::openat2::identity_for_fd(descriptor.as_raw_fd())?;
        let after = process_executable_metadata_v8(descriptor.as_raw_fd(), after_identity)?;
        if before != after {
            return Err(NativeSysErrorV8::RaceDetected(
                "process executable metadata changed during fast observation".to_string(),
            ));
        }
        self.revalidate()?;
        Ok(after)
    }

    fn open_process_executable(&self, pid: u32) -> NativeSysResultV8<OwnedFd> {
        let process = self.open_process_directory(pid)?;
        let exe = c"exe";
        // `/proc/PID/exe` is a kernel-owned magic link. Following it with
        // openat gives an exact target descriptor; the surrounding pidfd and
        // stat revalidation detect PID reuse or a concurrent exec transition.
        // SAFETY: the process descriptor and static C string remain live and
        // openat retains neither pointer nor descriptor ownership.
        let raw_fd = unsafe {
            libc::openat(
                process.as_raw_fd(),
                exe.as_ptr(),
                libc::O_RDONLY | libc::O_CLOEXEC,
            )
        };
        if raw_fd < 0 {
            return Err(io_error(
                "open procfs executable descriptor",
                std::io::Error::last_os_error(),
            ));
        }
        // SAFETY: successful openat returned a uniquely owned descriptor.
        Ok(unsafe { OwnedFd::from_raw_fd(raw_fd) })
    }

    pub(crate) fn process_group_pids(&self, pgid: u32) -> NativeSysResultV8<Vec<u32>> {
        if pgid <= 1 || pgid > libc::pid_t::MAX as u32 {
            return Err(NativeSysErrorV8::InvalidInput(
                "process group id must fit pid_t and be greater than one".to_string(),
            ));
        }
        let pids = self.numeric_process_ids()?;
        let mut members = Vec::new();
        for pid in pids {
            match self.read_process_stat(pid) {
                Ok(stat) if stat.process_group_id == pgid => members.push(pid),
                Ok(_) => {}
                Err(error) if process_disappeared_v8(&error) => {}
                Err(error) => return Err(error),
            }
        }
        members.sort_unstable();
        self.revalidate()?;
        Ok(members)
    }

    pub(crate) fn process_group_and_session_pids(
        &self,
        pgid: u32,
        session_id: u32,
    ) -> NativeSysResultV8<(Vec<u32>, Vec<u32>)> {
        if pgid <= 1
            || session_id <= 1
            || pgid > libc::pid_t::MAX as u32
            || session_id > libc::pid_t::MAX as u32
        {
            return Err(NativeSysErrorV8::InvalidInput(
                "process group/session ids must fit pid_t and be greater than one".to_string(),
            ));
        }
        let mut group = Vec::new();
        let mut session = Vec::new();
        for pid in self.numeric_process_ids()? {
            match self.read_process_stat(pid) {
                Ok(stat) => {
                    if stat.process_group_id == pgid {
                        group.push(pid);
                    }
                    if stat.session_id == session_id {
                        session.push(pid);
                    }
                }
                Err(error) if process_disappeared_v8(&error) => {}
                Err(error) => return Err(error),
            }
        }
        group.sort_unstable();
        session.sort_unstable();
        self.revalidate()?;
        Ok((group, session))
    }

    pub(super) fn numeric_process_ids(&self) -> NativeSysResultV8<Vec<u32>> {
        self.revalidate()?;
        let deadline = Instant::now() + MAX_PROC_SCAN_ELAPSED_V8;
        let dot = c".";
        // A dup would share the directory offset with the anchor's open file
        // description, making the second enumeration start at EOF. openat(".")
        // creates an independently offset directory description while staying
        // descriptor-anchored to the already verified procfs root.
        // SAFETY: the root fd and static C string remain live; openat retains
        // neither and returns a fresh descriptor on success.
        let enumeration_fd = unsafe {
            libc::openat(
                self.descriptor.as_raw_fd(),
                dot.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            )
        };
        if enumeration_fd < 0 {
            return Err(io_error(
                "open independent procfs descriptor for enumeration",
                std::io::Error::last_os_error(),
            ));
        }
        let enumeration_identity = match super::openat2::identity_for_fd(enumeration_fd) {
            Ok(identity) => identity,
            Err(error) => {
                // SAFETY: fdopendir has not consumed this descriptor.
                unsafe { libc::close(enumeration_fd) };
                return Err(error);
            }
        };
        if !enumeration_identity.matches_stable_directory(self.identity) {
            // SAFETY: fdopendir has not consumed this descriptor.
            unsafe { libc::close(enumeration_fd) };
            return Err(NativeSysErrorV8::RaceDetected(
                "independent procfs enumeration descriptor differs from the anchor".to_string(),
            ));
        }
        if let Err(error) = require_procfs_root_v8(enumeration_fd, enumeration_identity) {
            // SAFETY: fdopendir has not consumed this descriptor.
            unsafe { libc::close(enumeration_fd) };
            return Err(error);
        }
        // SAFETY: fdopendir consumes the independently opened descriptor on
        // success and gives it an independent directory offset.
        let stream = unsafe { libc::fdopendir(enumeration_fd) };
        if stream.is_null() {
            // SAFETY: fdopendir failed and therefore did not consume it.
            unsafe { libc::close(enumeration_fd) };
            return Err(io_error(
                "fdopendir procfs root",
                std::io::Error::last_os_error(),
            ));
        }
        let stream = DirectoryStreamV8(stream);
        let mut pids = BTreeSet::new();
        loop {
            // SAFETY: this thread exclusively owns `stream`; errno is reset so
            // null can be distinguished between EOF and failure.
            unsafe { *libc::__errno_location() = 0 };
            // SAFETY: the DIR remains live and exclusively used here.
            let entry = unsafe { libc::readdir(stream.0) };
            if entry.is_null() {
                // SAFETY: errno is thread-local and read immediately.
                let errno = unsafe { *libc::__errno_location() };
                if errno != 0 {
                    return Err(io_error(
                        "readdir procfs root",
                        std::io::Error::from_raw_os_error(errno),
                    ));
                }
                break;
            }
            // SAFETY: d_name is a NUL-terminated array owned by the live DIR
            // and valid until the next readdir call.
            let name = unsafe { CStr::from_ptr((*entry).d_name.as_ptr()) }.to_bytes();
            if name.is_empty() || !name.iter().all(u8::is_ascii_digit) {
                continue;
            }
            let text = std::str::from_utf8(name).map_err(|_| {
                NativeSysErrorV8::IdentityMismatch(
                    "numeric procfs entry is not valid ASCII".to_string(),
                )
            })?;
            let pid = text.parse::<u32>().map_err(|_| {
                NativeSysErrorV8::IdentityMismatch(
                    "numeric procfs entry does not fit u32".to_string(),
                )
            })?;
            if pid == 0 || pid > libc::pid_t::MAX as u32 {
                return Err(NativeSysErrorV8::IdentityMismatch(
                    "numeric procfs entry does not fit a positive pid_t".to_string(),
                ));
            }
            if !pids.insert(pid) {
                return Err(NativeSysErrorV8::RaceDetected(
                    "procfs enumeration returned a duplicate numeric pid".to_string(),
                ));
            }
            if pids.len() > MAX_PROC_NUMERIC_ENTRIES_V8 || Instant::now() > deadline {
                return Err(NativeSysErrorV8::RaceDetected(format!(
                    "numeric procfs scan exceeds {MAX_PROC_NUMERIC_ENTRIES_V8} entries or {} seconds",
                    MAX_PROC_SCAN_ELAPSED_V8.as_secs()
                )));
            }
        }
        self.revalidate()?;
        Ok(pids.into_iter().collect())
    }
}

#[cfg(target_os = "linux")]
struct DirectoryStreamV8(*mut libc::DIR);

#[cfg(target_os = "linux")]
impl Drop for DirectoryStreamV8 {
    fn drop(&mut self) {
        // SAFETY: this wrapper uniquely owns the live DIR pointer.
        unsafe { libc::closedir(self.0) };
    }
}

fn validate_detached_workload_selector_v8(
    cgroup_v2_path: &str,
    process_group_id: u32,
    session_id: u32,
) -> NativeSysResultV8<()> {
    if cgroup_v2_path.len() < 2
        || cgroup_v2_path.len() > 4096
        || !cgroup_v2_path.starts_with('/')
        || cgroup_v2_path.ends_with('/')
        || cgroup_v2_path.contains("//")
        || cgroup_v2_path.contains('\0')
        || cgroup_v2_path
            .split('/')
            .skip(1)
            .any(|component| component.is_empty() || component == "." || component == "..")
        || process_group_id <= 1
        || session_id <= 1
        || process_group_id > libc::pid_t::MAX as u32
        || session_id > libc::pid_t::MAX as u32
    {
        return Err(NativeSysErrorV8::InvalidInput(
            "detached workload selector must bind one canonical non-root cgroup and positive pid_t PGID/SID"
                .to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn observe_detached_workload_closure_impl_v8(
    selector: DetachedWorkloadSelectorV8,
) -> NativeSysResultV8<DetachedWorkloadClosureV8> {
    const MAX_MATCHED_PROCESSES_V8: usize = 64;
    validate_detached_workload_selector_v8(
        selector.cgroup_v2_path(),
        selector.process_group_id(),
        selector.session_id(),
    )?;
    let root = ProcfsRootV8::open_fixed()?;
    let observer = root.observer_binding()?;
    let first = scan_detached_workload_members_v8(&root, &selector, MAX_MATCHED_PROCESSES_V8)?;
    let maximum_processes = first.len().max(1);
    let mut budget = ProcessCaptureBudgetV8::new(
        maximum_processes,
        1024 * 1024 * 1024,
        MAX_PROCESS_CAPTURE_ELAPSED_V8,
    )?;
    let mut processes = Vec::with_capacity(first.len());
    for (pid, first_cgroup) in &first {
        let observation = observe_process_exact_with_root_and_budget_v8(&root, *pid, &mut budget)?;
        let current_cgroup = root.read_process_cgroup_path(*pid)?;
        let identity = observation.identity();
        if current_cgroup != *first_cgroup
            || !matches_detached_workload_selector_v8(
                identity.process_group_id(),
                identity.session_id(),
                &current_cgroup,
                &selector,
            )
        {
            return Err(NativeSysErrorV8::RaceDetected(format!(
                "pid {pid} left the detached-workload union during exact capture"
            )));
        }
        processes.push(DetachedWorkloadProcessV8 {
            cgroup_v2_path: current_cgroup,
            observation,
        });
    }
    let second = scan_detached_workload_members_v8(&root, &selector, MAX_MATCHED_PROCESSES_V8)?;
    if second != first {
        return Err(NativeSysErrorV8::RaceDetected(
            "detached-workload PGID/SID/cgroup union changed during global proc scan".to_string(),
        ));
    }
    for process in &processes {
        let current = process.observation.revalidate_fast_with_root(&root)?;
        let cgroup = root.read_process_cgroup_path(process.identity().pid())?;
        if !process.identity().has_same_stable_identity(&current)
            || cgroup != process.cgroup_v2_path
            || !matches_detached_workload_selector_v8(
                current.process_group_id(),
                current.session_id(),
                &cgroup,
                &selector,
            )
        {
            return Err(NativeSysErrorV8::RaceDetected(format!(
                "pid {} changed after detached-workload global closure",
                process.identity().pid()
            )));
        }
    }
    let final_observer = root.observer_binding()?;
    if final_observer != observer {
        return Err(NativeSysErrorV8::RaceDetected(
            "procfs observer mount or namespace identity changed during detached-workload scan"
                .to_string(),
        ));
    }
    let binding_sha256 = detached_workload_binding_sha256_v8(&selector, &observer, &processes);
    Ok(DetachedWorkloadClosureV8 {
        selector,
        processes,
        binding_sha256,
    })
}

#[cfg(not(target_os = "linux"))]
fn observe_detached_workload_closure_impl_v8(
    _selector: DetachedWorkloadSelectorV8,
) -> NativeSysResultV8<DetachedWorkloadClosureV8> {
    Err(unsupported("observe detached-workload global proc closure"))
}

#[cfg(target_os = "linux")]
fn revalidate_detached_workload_closure_impl_v8(
    closure: &DetachedWorkloadClosureV8,
) -> NativeSysResultV8<()> {
    let current = observe_detached_workload_closure_impl_v8(closure.selector.clone())?;
    if current.binding_sha256 != closure.binding_sha256 {
        return Err(NativeSysErrorV8::RaceDetected(
            "detached-workload global closure changed after pinning".to_string(),
        ));
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn revalidate_detached_workload_closure_impl_v8(
    _closure: &DetachedWorkloadClosureV8,
) -> NativeSysResultV8<()> {
    Err(unsupported(
        "revalidate detached-workload global proc closure",
    ))
}

#[cfg(target_os = "linux")]
fn scan_detached_workload_members_v8(
    root: &ProcfsRootV8,
    selector: &DetachedWorkloadSelectorV8,
    maximum_matches: usize,
) -> NativeSysResultV8<Vec<(u32, String)>> {
    let deadline = Instant::now() + MAX_PROC_SCAN_ELAPSED_V8;
    let mut matches = Vec::new();
    for pid in root.numeric_process_ids()? {
        let stat = match root.read_process_stat(pid) {
            Ok(stat) => stat,
            Err(error) if process_disappeared_v8(&error) => continue,
            Err(error) => return Err(error),
        };
        let cgroup = match root.read_process_cgroup_path(pid) {
            Ok(cgroup) => cgroup,
            Err(error) if process_disappeared_v8(&error) => continue,
            Err(error) => return Err(error),
        };
        if matches_detached_workload_selector_v8(
            stat.process_group_id,
            stat.session_id,
            &cgroup,
            selector,
        ) {
            matches.push((pid, cgroup));
            if matches.len() > maximum_matches {
                return Err(NativeSysErrorV8::IdentityMismatch(format!(
                    "detached-workload union exceeds {maximum_matches} processes"
                )));
            }
        }
        if Instant::now() > deadline {
            return Err(NativeSysErrorV8::RaceDetected(
                "detached-workload global proc scan exceeded its elapsed-time bound".to_string(),
            ));
        }
    }
    matches.sort_by_key(|(pid, _)| *pid);
    root.revalidate()?;
    Ok(matches)
}

#[cfg(target_os = "linux")]
fn matches_detached_workload_selector_v8(
    process_group_id: u32,
    session_id: u32,
    cgroup_v2_path: &str,
    selector: &DetachedWorkloadSelectorV8,
) -> bool {
    process_group_id == selector.process_group_id()
        || session_id == selector.session_id()
        || cgroup_v2_path == selector.cgroup_v2_path()
}

#[cfg(target_os = "linux")]
fn detached_workload_binding_sha256_v8(
    selector: &DetachedWorkloadSelectorV8,
    observer: &ProcfsObserverBindingV8,
    processes: &[DetachedWorkloadProcessV8],
) -> [u8; 32] {
    let mut bytes = b"hepta_linux_v8_detached_workload_closure_v1\0".to_vec();
    append_topology_field_v8(&mut bytes, selector.cgroup_v2_path().as_bytes());
    append_topology_u64_v8(&mut bytes, u64::from(selector.process_group_id()));
    append_topology_u64_v8(&mut bytes, u64::from(selector.session_id()));
    for value in [
        u64::from(observer.observer_pid),
        u64::from(observer.observer_tid),
        observer.proc_mount_id,
        observer.cgroup_mount_id,
        observer.pid_namespace_device,
        observer.pid_namespace_inode,
        observer.cgroup_namespace_device,
        observer.cgroup_namespace_inode,
        observer.mount_namespace_device,
        observer.mount_namespace_inode,
        processes.len() as u64,
    ] {
        append_topology_u64_v8(&mut bytes, value);
    }
    for process in processes {
        let identity = process.identity();
        for value in [
            u64::from(identity.pid()),
            identity.start_ticks(),
            u64::from(identity.parent_pid()),
            u64::from(identity.process_group_id()),
            u64::from(identity.session_id()),
            identity.executable().device(),
            identity.executable().inode(),
            identity.executable().size(),
        ] {
            append_topology_u64_v8(&mut bytes, value);
        }
        append_topology_field_v8(&mut bytes, &identity.executable().sha256());
        append_topology_field_v8(&mut bytes, process.cgroup_v2_path.as_bytes());
    }
    sha2::Sha256::digest(bytes).into()
}

#[cfg(target_os = "linux")]
fn append_topology_field_v8(bytes: &mut Vec<u8>, value: &[u8]) {
    bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
    bytes.extend_from_slice(value);
}

#[cfg(target_os = "linux")]
fn append_topology_u64_v8(bytes: &mut Vec<u8>, value: u64) {
    append_topology_field_v8(bytes, &value.to_be_bytes());
}

#[cfg(target_os = "linux")]
fn observe_process_exact_impl_v8(pid: u32) -> NativeSysResultV8<ProcessObservationV8> {
    validate_observable_pid_v8(pid)?;
    let root = ProcfsRootV8::open_fixed()?;
    let mut budget = ProcessCaptureBudgetV8::new(
        1,
        MAX_PROCESS_EXECUTABLE_BYTES_V8 * 2,
        Duration::from_secs(5),
    )?;
    observe_process_exact_with_root_and_budget_v8(&root, pid, &mut budget)
}

#[cfg(not(target_os = "linux"))]
fn observe_process_exact_impl_v8(_pid: u32) -> NativeSysResultV8<ProcessObservationV8> {
    Err(unsupported("observe pidfd-bound process identity"))
}

#[cfg(target_os = "linux")]
fn revalidate_process_observation_impl_v8(
    observation: &ProcessObservationV8,
) -> NativeSysResultV8<ProcessIdentityV8> {
    let root = ProcfsRootV8::open_fixed()?;
    let current = capture_process_identity_v8(&root, &observation.handle)?;
    if !observation.identity.has_same_stable_identity(&current) {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "stable process identity changed for pid {}",
            observation.identity.pid
        )));
    }
    Ok(current)
}

#[cfg(target_os = "linux")]
fn revalidate_process_observation_fast_v8(
    observation: &ProcessObservationV8,
    root: &ProcfsRootV8,
) -> NativeSysResultV8<ProcessIdentityV8> {
    if observation.handle.is_exited()? {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "pid {} exited before fast process revalidation",
            observation.pid()
        )));
    }
    let stat_before = root.read_process_stat(observation.pid())?;
    require_stat_matches_pidfd_v8(&stat_before, &observation.handle)?;
    let executable = root.observe_executable_metadata(observation.pid())?;
    if !observation
        .identity
        .executable
        .matches_metadata(&executable)
    {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "pid {} executable metadata differs from the digest-bound identity",
            observation.pid()
        )));
    }
    let stat_after = root.read_process_stat(observation.pid())?;
    require_stat_matches_pidfd_v8(&stat_after, &observation.handle)?;
    require_same_stable_stat_v8(&stat_before, &stat_after)?;
    if observation.handle.is_exited()? {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "pid {} exited during fast process revalidation",
            observation.pid()
        )));
    }
    let current = ProcessIdentityV8 {
        pid: stat_after.pid,
        start_ticks: stat_after.start_ticks,
        parent_pid: stat_after.parent_pid,
        process_group_id: stat_after.process_group_id,
        session_id: stat_after.session_id,
        state: stat_after.state,
        executable: observation.identity.executable.clone(),
    };
    if !observation.identity.has_same_stable_identity(&current) {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "stable process identity changed for pid {} during fast revalidation",
            observation.pid()
        )));
    }
    Ok(current)
}

#[cfg(not(target_os = "linux"))]
fn revalidate_process_observation_impl_v8(
    _observation: &ProcessObservationV8,
) -> NativeSysResultV8<ProcessIdentityV8> {
    Err(unsupported("revalidate pidfd-bound process identity"))
}

#[cfg(target_os = "linux")]
pub(crate) fn capture_process_identity_v8(
    root: &ProcfsRootV8,
    handle: &PidHandleV8,
) -> NativeSysResultV8<ProcessIdentityV8> {
    let mut budget = ProcessCaptureBudgetV8::new(
        1,
        MAX_PROCESS_EXECUTABLE_BYTES_V8 * 2,
        Duration::from_secs(5),
    )?;
    capture_process_identity_with_budget_v8(root, handle, &mut budget)
}

#[cfg(target_os = "linux")]
pub(crate) fn observe_process_exact_with_root_and_budget_v8(
    root: &ProcfsRootV8,
    pid: u32,
    budget: &mut ProcessCaptureBudgetV8,
) -> NativeSysResultV8<ProcessObservationV8> {
    validate_observable_pid_v8(pid)?;
    budget.begin_process()?;
    let handle = pidfd_open_verified_with_procfs_v8(root, pid)?;
    let identity = capture_process_identity_with_budget_v8(root, &handle, budget)?;
    Ok(ProcessObservationV8 { identity, handle })
}

#[cfg(target_os = "linux")]
fn capture_process_identity_with_budget_v8(
    root: &ProcfsRootV8,
    handle: &PidHandleV8,
    budget: &mut ProcessCaptureBudgetV8,
) -> NativeSysResultV8<ProcessIdentityV8> {
    budget.check_time()?;
    if handle.is_exited()? {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "pid {} exited before process observation",
            handle.pid()
        )));
    }
    let stat_before = root.read_process_stat(handle.pid())?;
    require_stat_matches_pidfd_v8(&stat_before, handle)?;
    let executable_before = root.observe_executable(handle.pid(), budget)?;
    let stat_after = root.read_process_stat(handle.pid())?;
    require_stat_matches_pidfd_v8(&stat_after, handle)?;
    require_same_stable_stat_v8(&stat_before, &stat_after)?;
    let executable_after = root.observe_executable(handle.pid(), budget)?;
    if executable_before != executable_after {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "pid {} executable identity changed during observation",
            handle.pid()
        )));
    }
    if handle.is_exited()? {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "pid {} exited during process observation",
            handle.pid()
        )));
    }
    let stat_final = root.read_process_stat(handle.pid())?;
    require_stat_matches_pidfd_v8(&stat_final, handle)?;
    require_same_stable_stat_v8(&stat_after, &stat_final)?;
    budget.check_time()?;
    Ok(ProcessIdentityV8 {
        pid: stat_final.pid,
        start_ticks: stat_final.start_ticks,
        parent_pid: stat_final.parent_pid,
        process_group_id: stat_final.process_group_id,
        session_id: stat_final.session_id,
        state: stat_final.state,
        executable: executable_after,
    })
}

#[cfg(target_os = "linux")]
fn validate_observable_pid_v8(pid: u32) -> NativeSysResultV8<()> {
    if pid <= 1 || pid > libc::pid_t::MAX as u32 {
        return Err(NativeSysErrorV8::InvalidInput(
            "observable pid must fit pid_t and be greater than one".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn require_stat_matches_pidfd_v8(
    stat: &ParsedProcStatV8,
    handle: &PidHandleV8,
) -> NativeSysResultV8<()> {
    if stat.pid != handle.pid()
        || stat.start_ticks != handle.start_ticks()
        || stat.process_group_id == 0
        || stat.session_id == 0
    {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "procfs stat no longer matches pidfd for pid {}",
            handle.pid()
        )));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn require_same_stable_stat_v8(
    before: &ParsedProcStatV8,
    after: &ParsedProcStatV8,
) -> NativeSysResultV8<()> {
    if before.pid != after.pid
        || before.start_ticks != after.start_ticks
        || before.parent_pid != after.parent_pid
        || before.process_group_id != after.process_group_id
        || before.session_id != after.session_id
    {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "stable procfs stat identity changed for pid {}",
            before.pid
        )));
    }
    Ok(())
}

#[cfg(any(target_os = "linux", test))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ParsedProcStatV8 {
    pub(super) pid: u32,
    pub(super) state: ProcessStateV8,
    pub(super) parent_pid: u32,
    pub(super) process_group_id: u32,
    pub(super) session_id: u32,
    pub(super) start_ticks: u64,
}

#[cfg(any(target_os = "linux", test))]
pub(super) fn parse_proc_stat_v8(
    bytes: &[u8],
    expected_pid: Option<u32>,
) -> NativeSysResultV8<ParsedProcStatV8> {
    let bytes = bytes.strip_suffix(b"\n").unwrap_or(bytes);
    if bytes.is_empty() || bytes.contains(&b'\n') || bytes.contains(&0) {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "proc stat must be one non-empty NUL-free line".to_string(),
        ));
    }
    let opening = bytes.iter().position(|byte| *byte == b'(').ok_or_else(|| {
        NativeSysErrorV8::IdentityMismatch("proc stat lacks opening comm delimiter".to_string())
    })?;
    if opening < 2 || bytes.get(opening - 1) != Some(&b' ') {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "proc stat pid and comm delimiter are not canonical".to_string(),
        ));
    }
    let pid_bytes = &bytes[..opening - 1];
    if pid_bytes.first() == Some(&b'0') || !pid_bytes.iter().all(u8::is_ascii_digit) {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "proc stat pid is not canonical positive decimal".to_string(),
        ));
    }
    let pid = parse_positive_pid_t_v8(pid_bytes, "proc stat pid")?;
    if expected_pid.is_some_and(|expected| expected != pid) {
        return Err(NativeSysErrorV8::RaceDetected(format!(
            "proc stat pid {pid} differs from expected pid {}",
            expected_pid.unwrap_or_default()
        )));
    }

    let closing = bytes
        .windows(2)
        .rposition(|window| window == b") ")
        .ok_or_else(|| {
            NativeSysErrorV8::IdentityMismatch(
                "proc stat lacks canonical closing comm delimiter".to_string(),
            )
        })?;
    if closing <= opening || closing + 2 > bytes.len() {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "proc stat comm field is malformed".to_string(),
        ));
    }
    let suffix = std::str::from_utf8(&bytes[closing + 2..]).map_err(|_| {
        NativeSysErrorV8::IdentityMismatch("proc stat fields are not ASCII".to_string())
    })?;
    if suffix.starts_with(' ') || suffix.ends_with(' ') || suffix.contains("  ") {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "proc stat numeric fields are not canonically separated".to_string(),
        ));
    }
    let fields: Vec<&str> = suffix.split(' ').collect();
    if fields.len() < 20 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "proc stat lacks mandatory field 22 starttime".to_string(),
        ));
    }
    let state = parse_process_state_v8(fields[0].as_bytes())?;
    let parent_pid = parse_nonnegative_pid_t_v8(fields[1].as_bytes(), "proc stat parent pid")?;
    // Kernel threads encountered during a complete `/proc` scan can report
    // pgrp/session zero. Exact user-process observation imposes its stricter
    // positive policy only after this lossless parse.
    let process_group_id =
        parse_nonnegative_pid_t_v8(fields[2].as_bytes(), "proc stat process group id")?;
    let session_id = parse_nonnegative_pid_t_v8(fields[3].as_bytes(), "proc stat session id")?;
    let start_ticks = parse_canonical_u64_v8(fields[19].as_bytes(), "proc stat start ticks")?;
    if start_ticks == 0 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "proc stat start ticks must be non-zero".to_string(),
        ));
    }
    Ok(ParsedProcStatV8 {
        pid,
        state,
        parent_pid,
        process_group_id,
        session_id,
        start_ticks,
    })
}

#[cfg(any(target_os = "linux", test))]
fn parse_process_state_v8(bytes: &[u8]) -> NativeSysResultV8<ProcessStateV8> {
    match bytes {
        b"R" => Ok(ProcessStateV8::Running),
        b"S" => Ok(ProcessStateV8::Sleeping),
        b"D" => Ok(ProcessStateV8::DiskSleep),
        b"Z" => Ok(ProcessStateV8::Zombie),
        b"T" => Ok(ProcessStateV8::Stopped),
        b"t" => Ok(ProcessStateV8::TracingStop),
        b"X" | b"x" => Ok(ProcessStateV8::Dead),
        b"K" => Ok(ProcessStateV8::Wakekill),
        b"W" => Ok(ProcessStateV8::Waking),
        b"P" => Ok(ProcessStateV8::Parked),
        b"I" => Ok(ProcessStateV8::Idle),
        _ => Err(NativeSysErrorV8::IdentityMismatch(
            "proc stat contains an unknown process state".to_string(),
        )),
    }
}

#[cfg(any(target_os = "linux", test))]
fn parse_positive_pid_t_v8(bytes: &[u8], field: &'static str) -> NativeSysResultV8<u32> {
    let value = parse_canonical_u64_v8(bytes, field)?;
    if value == 0 || value > i32::MAX as u64 {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "{field} does not fit a positive pid_t"
        )));
    }
    u32::try_from(value)
        .map_err(|_| NativeSysErrorV8::IdentityMismatch(format!("{field} does not fit u32")))
}

#[cfg(any(target_os = "linux", test))]
fn parse_nonnegative_pid_t_v8(bytes: &[u8], field: &'static str) -> NativeSysResultV8<u32> {
    let value = parse_canonical_u64_v8(bytes, field)?;
    if value > i32::MAX as u64 {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "{field} does not fit pid_t"
        )));
    }
    u32::try_from(value)
        .map_err(|_| NativeSysErrorV8::IdentityMismatch(format!("{field} does not fit u32")))
}

#[cfg(any(target_os = "linux", test))]
fn parse_canonical_u64_v8(bytes: &[u8], field: &'static str) -> NativeSysResultV8<u64> {
    if bytes.is_empty()
        || (bytes.len() > 1 && bytes.first() == Some(&b'0'))
        || !bytes.iter().all(u8::is_ascii_digit)
    {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "{field} is not canonical unsigned decimal"
        )));
    }
    let text = std::str::from_utf8(bytes)
        .map_err(|_| NativeSysErrorV8::IdentityMismatch(format!("{field} is not ASCII")))?;
    text.parse::<u64>()
        .map_err(|_| NativeSysErrorV8::IdentityMismatch(format!("{field} does not fit u64")))
}

/// Parses the numeric observer's mount table and admits exactly one `/proc`
/// proc mount and one `/sys/fs/cgroup` cgroup2 mount, both rooted at `/`.
/// Bind-mounted subtrees, duplicate target rows, malformed octal escapes and
/// reused mount IDs all fail closed.
#[cfg(any(target_os = "linux", test))]
pub(super) fn parse_mountinfo_topology_v8(
    bytes: &[u8],
) -> NativeSysResultV8<ParsedMountTopologyV8> {
    if bytes.is_empty() || bytes.contains(&0) || !bytes.ends_with(b"\n") {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "numeric mountinfo must be nonempty, NUL-free and LF-terminated".to_string(),
        ));
    }
    let mut proc_mount_id = None;
    let mut cgroup_mount_id = None;
    let mut seen_ids = BTreeSet::new();
    let mut entry_count = 0_usize;
    for line in bytes[..bytes.len() - 1].split(|byte| *byte == b'\n') {
        entry_count = entry_count.checked_add(1).ok_or_else(|| {
            NativeSysErrorV8::IdentityMismatch("mountinfo entry count overflow".to_string())
        })?;
        if entry_count > MAX_PROC_MOUNTINFO_ENTRIES_V8 || line.is_empty() || line.len() > 4096 {
            return Err(NativeSysErrorV8::IdentityMismatch(format!(
                "mountinfo exceeds {MAX_PROC_MOUNTINFO_ENTRIES_V8} entries or 4096 bytes per row"
            )));
        }
        let fields: Vec<&[u8]> = line.split(|byte| *byte == b' ').collect();
        if fields.len() < 10 || fields.iter().any(|field| field.is_empty()) {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "mountinfo row is truncated or noncanonically spaced".to_string(),
            ));
        }
        let separators: Vec<usize> = fields
            .iter()
            .enumerate()
            .filter_map(|(index, field)| (*field == b"-").then_some(index))
            .collect();
        if separators.len() != 1 || separators[0] < 6 || separators[0] + 3 >= fields.len() {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "mountinfo row lacks one canonical optional-field separator".to_string(),
            ));
        }
        let mount_id = parse_canonical_u64_v8(fields[0], "mountinfo mount id")?;
        let parent_id = parse_canonical_u64_v8(fields[1], "mountinfo parent id")?;
        if mount_id == 0 || parent_id == 0 || !seen_ids.insert(mount_id) {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "mountinfo contains zero or duplicate mount identity".to_string(),
            ));
        }
        parse_mountinfo_device_v8(fields[2])?;
        let root = decode_mountinfo_field_v8(fields[3], false)?;
        let mount_point = decode_mountinfo_field_v8(fields[4], true)?;
        let filesystem = fields[separators[0] + 1];
        let target = if mount_point == b"/proc" {
            Some((b"proc".as_slice(), &mut proc_mount_id, "procfs"))
        } else if mount_point == b"/sys/fs/cgroup" {
            Some((b"cgroup2".as_slice(), &mut cgroup_mount_id, "cgroup2"))
        } else {
            None
        };
        if let Some((expected_filesystem, slot, label)) = target {
            if root != b"/" || filesystem != expected_filesystem || slot.is_some() {
                return Err(NativeSysErrorV8::IdentityMismatch(format!(
                    "numeric mountinfo {label} target is shifted, duplicated, or has wrong filesystem"
                )));
            }
            *slot = Some(mount_id);
        }
    }
    let proc_mount_id = proc_mount_id.ok_or_else(|| {
        NativeSysErrorV8::IdentityMismatch(
            "numeric mountinfo lacks exact root=/ /proc proc row".to_string(),
        )
    })?;
    let cgroup_mount_id = cgroup_mount_id.ok_or_else(|| {
        NativeSysErrorV8::IdentityMismatch(
            "numeric mountinfo lacks exact root=/ /sys/fs/cgroup cgroup2 row".to_string(),
        )
    })?;
    if proc_mount_id == cgroup_mount_id {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "procfs and cgroup2 unexpectedly share one mount id".to_string(),
        ));
    }
    Ok(ParsedMountTopologyV8 {
        proc_mount_id,
        cgroup_mount_id,
    })
}

#[cfg(any(target_os = "linux", test))]
fn parse_mountinfo_device_v8(bytes: &[u8]) -> NativeSysResultV8<(u64, u64)> {
    let mut parts = bytes.split(|byte| *byte == b':');
    let major = parts.next().unwrap_or_default();
    let minor = parts.next().unwrap_or_default();
    if parts.next().is_some() {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "mountinfo device is not canonical major:minor".to_string(),
        ));
    }
    Ok((
        parse_canonical_u64_v8(major, "mountinfo device major")?,
        parse_canonical_u64_v8(minor, "mountinfo device minor")?,
    ))
}

#[cfg(any(target_os = "linux", test))]
fn decode_mountinfo_field_v8(bytes: &[u8], require_absolute: bool) -> NativeSysResultV8<Vec<u8>> {
    if bytes.is_empty() || (require_absolute && !bytes.starts_with(b"/")) {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "mountinfo field is empty or mount point is not absolute".to_string(),
        ));
    }
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0_usize;
    while index < bytes.len() {
        if bytes[index] != b'\\' {
            let byte = bytes[index];
            if byte < 0x20 || byte == 0x7f {
                return Err(NativeSysErrorV8::IdentityMismatch(
                    "mountinfo path contains an unescaped control byte".to_string(),
                ));
            }
            decoded.push(byte);
            index += 1;
            continue;
        }
        let escape = bytes.get(index + 1..index + 4).ok_or_else(|| {
            NativeSysErrorV8::IdentityMismatch("mountinfo path has truncated escape".to_string())
        })?;
        let value = match escape {
            b"040" => b' ',
            b"011" => b'\t',
            b"012" => b'\n',
            b"134" => b'\\',
            _ => {
                return Err(NativeSysErrorV8::IdentityMismatch(
                    "mountinfo path has a noncanonical escape".to_string(),
                ));
            }
        };
        decoded.push(value);
        index += 4;
    }
    Ok(decoded)
}

#[cfg(any(target_os = "linux", test))]
pub(super) fn parse_fdinfo_mount_id_v8(bytes: &[u8]) -> NativeSysResultV8<u64> {
    if bytes.is_empty() || bytes.contains(&0) || !bytes.ends_with(b"\n") {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "numeric fdinfo must be nonempty, NUL-free and LF-terminated".to_string(),
        ));
    }
    let mut mount_id = None;
    for line in bytes[..bytes.len() - 1].split(|byte| *byte == b'\n') {
        let Some(value) = line.strip_prefix(b"mnt_id:\t") else {
            continue;
        };
        let parsed = parse_canonical_u64_v8(value, "fdinfo mount id")?;
        if parsed == 0 || mount_id.replace(parsed).is_some() {
            return Err(NativeSysErrorV8::IdentityMismatch(
                "numeric fdinfo has zero or duplicate mount id".to_string(),
            ));
        }
    }
    mount_id.ok_or_else(|| {
        NativeSysErrorV8::IdentityMismatch("numeric fdinfo lacks exact mnt_id".to_string())
    })
}

#[cfg(any(target_os = "linux", test))]
pub(super) fn parse_process_status_credentials_v8(bytes: &[u8]) -> NativeSysResultV8<(u32, u32)> {
    if bytes.is_empty() || bytes.contains(&0) || !bytes.ends_with(b"\n") {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "process status must be nonempty, NUL-free and LF-terminated".to_string(),
        ));
    }
    let mut uid = None;
    let mut gid = None;
    for line in bytes[..bytes.len() - 1].split(|byte| *byte == b'\n') {
        let slot = if let Some(values) = line.strip_prefix(b"Uid:\t") {
            Some((values, &mut uid, "process status uid"))
        } else {
            line.strip_prefix(b"Gid:\t")
                .map(|values| (values, &mut gid, "process status gid"))
        };
        let Some((values, slot, label)) = slot else {
            continue;
        };
        if slot.is_some() {
            return Err(NativeSysErrorV8::IdentityMismatch(format!(
                "{label} appears more than once"
            )));
        }
        let fields: Vec<&[u8]> = values.split(|byte| *byte == b'\t').collect();
        if fields.len() != 4 || fields.iter().any(|field| field.is_empty()) {
            return Err(NativeSysErrorV8::IdentityMismatch(format!(
                "{label} lacks four canonical credential values"
            )));
        }
        let parsed: Vec<u32> = fields
            .iter()
            .map(|field| {
                parse_canonical_u64_v8(field, label).and_then(|value| {
                    u32::try_from(value).map_err(|_| {
                        NativeSysErrorV8::IdentityMismatch(format!("{label} exceeds u32"))
                    })
                })
            })
            .collect::<NativeSysResultV8<_>>()?;
        if parsed.iter().any(|value| *value != parsed[0]) {
            return Err(NativeSysErrorV8::IdentityMismatch(format!(
                "{label} real/effective/saved/fs values differ"
            )));
        }
        *slot = Some(parsed[0]);
    }
    Ok((
        uid.ok_or_else(|| {
            NativeSysErrorV8::IdentityMismatch("process status lacks exact Uid row".to_string())
        })?,
        gid.ok_or_else(|| {
            NativeSysErrorV8::IdentityMismatch("process status lacks exact Gid row".to_string())
        })?,
    ))
}

#[cfg(any(target_os = "linux", test))]
pub(super) fn validate_process_cmdline_v8(bytes: &[u8]) -> NativeSysResultV8<()> {
    if bytes.is_empty() || bytes[0] == 0 || !bytes.ends_with(&[0]) {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "process cmdline must contain a nonempty argv[0] and terminal NUL".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn open_fixed_procfs_descriptor_v8() -> NativeSysResultV8<OwnedFd> {
    let path = c"/proc";
    // SAFETY: the static path remains live and open retains no pointer.
    let raw_fd = unsafe {
        libc::open(
            path.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if raw_fd < 0 {
        return Err(io_error(
            "open fixed procfs root",
            std::io::Error::last_os_error(),
        ));
    }
    // SAFETY: successful open returned a uniquely owned descriptor.
    Ok(unsafe { OwnedFd::from_raw_fd(raw_fd) })
}

#[cfg(target_os = "linux")]
fn require_procfs_root_v8(fd: libc::c_int, identity: FileIdentityV8) -> NativeSysResultV8<()> {
    require_procfs_directory_v8(fd)?;
    if identity.owner_uid() != 0 || identity.owner_gid() != 0 || identity.mode() != 0o555 {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "fixed /proc must be root-owned mode 0555; observed uid={}:gid={} mode={:04o}",
            identity.owner_uid(),
            identity.owner_gid(),
            identity.mode()
        )));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn require_procfs_directory_v8(fd: libc::c_int) -> NativeSysResultV8<()> {
    // SAFETY: zero initializes writable stat/statfs buffers.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: stat is writable and fd remains live.
    if unsafe { libc::fstat(fd, &mut stat) } != 0 {
        return Err(io_error(
            "fstat procfs directory",
            std::io::Error::last_os_error(),
        ));
    }
    if stat.st_mode & libc::S_IFMT != libc::S_IFDIR {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "procfs descriptor is not a directory".to_string(),
        ));
    }
    require_procfs_magic_v8(fd)
}

#[cfg(target_os = "linux")]
fn require_procfs_file_v8(fd: libc::c_int) -> NativeSysResultV8<()> {
    // SAFETY: zero initializes a writable stat buffer.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: stat is writable and fd remains live.
    if unsafe { libc::fstat(fd, &mut stat) } != 0 {
        return Err(io_error(
            "fstat procfs file",
            std::io::Error::last_os_error(),
        ));
    }
    if stat.st_mode & libc::S_IFMT != libc::S_IFREG {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "procfs control descriptor is not a regular file".to_string(),
        ));
    }
    require_procfs_magic_v8(fd)
}

#[cfg(target_os = "linux")]
fn require_procfs_magic_v8(fd: libc::c_int) -> NativeSysResultV8<()> {
    // SAFETY: zero initializes a writable statfs buffer.
    let mut statfs: libc::statfs = unsafe { std::mem::zeroed() };
    // SAFETY: statfs is writable and fd remains live.
    if unsafe { libc::fstatfs(fd, &mut statfs) } != 0 {
        return Err(io_error(
            "fstatfs procfs descriptor",
            std::io::Error::last_os_error(),
        ));
    }
    if statfs.f_type != PROC_SUPER_MAGIC_V8 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "descriptor is not on the procfs superblock".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn require_namespace_descriptor_v8(fd: libc::c_int) -> NativeSysResultV8<()> {
    // SAFETY: zero initializes a writable statfs buffer.
    let mut statfs: libc::statfs = unsafe { std::mem::zeroed() };
    // SAFETY: statfs is writable and fd remains live.
    if unsafe { libc::fstatfs(fd, &mut statfs) } != 0 {
        return Err(io_error(
            "fstatfs namespace descriptor",
            std::io::Error::last_os_error(),
        ));
    }
    if statfs.f_type != NSFS_SUPER_MAGIC_V8 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "numeric process namespace descriptor is not on nsfs".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn openat2_procfs_v8(
    anchor_fd: libc::c_int,
    relative: &str,
    flags: libc::c_int,
) -> NativeSysResultV8<OwnedFd> {
    if relative.is_empty()
        || relative.starts_with('/')
        || relative
            .split('/')
            .any(|part| part.is_empty() || matches!(part, "." | ".."))
    {
        return Err(NativeSysErrorV8::InvalidInput(
            "procfs relative path is not canonical beneath its anchor".to_string(),
        ));
    }
    let path = CString::new(relative).map_err(|_| {
        NativeSysErrorV8::InvalidInput("procfs relative path contains NUL".to_string())
    })?;
    // SAFETY: zero is the kernel-defined baseline; fields are set explicitly.
    let mut how: libc::open_how = unsafe { std::mem::zeroed() };
    how.flags = u64::try_from(flags | libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .map_err(|_| NativeSysErrorV8::InvalidInput("invalid procfs open flags".to_string()))?;
    how.resolve = libc::RESOLVE_BENEATH
        | libc::RESOLVE_NO_SYMLINKS
        | libc::RESOLVE_NO_MAGICLINKS
        | libc::RESOLVE_NO_XDEV;
    // SAFETY: the C string and open_how remain live; openat2 retains neither.
    let raw_fd = unsafe {
        libc::syscall(
            libc::SYS_openat2,
            anchor_fd,
            path.as_ptr(),
            &how as *const libc::open_how,
            std::mem::size_of::<libc::open_how>(),
        )
    };
    if raw_fd < 0 {
        return Err(io_error(
            "openat2 beneath procfs anchor",
            std::io::Error::last_os_error(),
        ));
    }
    let raw_fd = libc::c_int::try_from(raw_fd).map_err(|_| {
        NativeSysErrorV8::InvalidInput("openat2 returned an invalid procfs descriptor".to_string())
    })?;
    // SAFETY: successful openat2 returned a uniquely owned descriptor.
    Ok(unsafe { OwnedFd::from_raw_fd(raw_fd) })
}

#[cfg(target_os = "linux")]
fn read_fd_bounded_v8(fd: libc::c_int, maximum_bytes: usize) -> NativeSysResultV8<Vec<u8>> {
    if maximum_bytes == 0 || maximum_bytes > MAX_PROC_SMALL_FILE_BYTES_V8 {
        return Err(NativeSysErrorV8::InvalidInput(
            "procfs read bound is outside the frozen range".to_string(),
        ));
    }
    let mut output = Vec::new();
    let mut buffer = [0_u8; 4096];
    let mut offset = 0_i64;
    loop {
        let remaining = maximum_bytes.saturating_add(1).saturating_sub(output.len());
        if remaining == 0 {
            return Err(NativeSysErrorV8::IdentityMismatch(format!(
                "procfs file exceeds the frozen {maximum_bytes}-byte limit"
            )));
        }
        let request = remaining.min(buffer.len());
        // SAFETY: buffer is writable, fd is live, and pread retains no pointer.
        let read = unsafe { libc::pread(fd, buffer.as_mut_ptr().cast(), request, offset) };
        if read < 0 {
            return Err(io_error(
                "pread procfs descriptor",
                std::io::Error::last_os_error(),
            ));
        }
        if read == 0 {
            break;
        }
        let read = usize::try_from(read).map_err(|_| {
            NativeSysErrorV8::InvalidInput("procfs read returned an invalid length".to_string())
        })?;
        output.extend_from_slice(&buffer[..read]);
        offset = offset
            .checked_add(i64::try_from(read).map_err(|_| {
                NativeSysErrorV8::InvalidInput("procfs read offset overflow".to_string())
            })?)
            .ok_or_else(|| {
                NativeSysErrorV8::InvalidInput("procfs read offset overflow".to_string())
            })?;
    }
    if output.len() > maximum_bytes {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "procfs file exceeds the frozen {maximum_bytes}-byte limit"
        )));
    }
    Ok(output)
}

#[cfg(target_os = "linux")]
fn require_process_executable_v8(
    fd: libc::c_int,
    identity: FileIdentityV8,
) -> NativeSysResultV8<()> {
    // SAFETY: zero initializes a writable stat buffer.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: stat is writable and fd remains live.
    if unsafe { libc::fstat(fd, &mut stat) } != 0 {
        return Err(io_error(
            "fstat process executable",
            std::io::Error::last_os_error(),
        ));
    }
    if stat.st_mode & libc::S_IFMT != libc::S_IFREG
        || identity.link_count() == 0
        || identity.mode() & 0o111 == 0
        || identity.mode() & 0o6022 != 0
        || identity.size() == 0
        || identity.size() > MAX_PROCESS_EXECUTABLE_BYTES_V8
    {
        return Err(NativeSysErrorV8::IdentityMismatch(format!(
            "process executable must be linked, nonempty, nonprivileged, non-group/world-writable, executable, and at most {MAX_PROCESS_EXECUTABLE_BYTES_V8} bytes; observed mode={:04o} nlink={} size={}",
            identity.mode(),
            identity.link_count(),
            identity.size()
        )));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn require_process_directory_target_v8(
    fd: libc::c_int,
    identity: FileIdentityV8,
) -> NativeSysResultV8<()> {
    // SAFETY: zero initializes a writable stat buffer.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: stat is writable and fd remains live.
    if unsafe { libc::fstat(fd, &mut stat) } != 0 {
        return Err(io_error(
            "fstat numeric process directory target",
            std::io::Error::last_os_error(),
        ));
    }
    if stat.st_mode & libc::S_IFMT != libc::S_IFDIR || identity.link_count() == 0 {
        return Err(NativeSysErrorV8::IdentityMismatch(
            "numeric process directory target is not a linked directory".to_string(),
        ));
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn process_executable_metadata_v8(
    fd: libc::c_int,
    identity: FileIdentityV8,
) -> NativeSysResultV8<ProcessExecutableMetadataV8> {
    require_process_executable_v8(fd, identity)?;
    // SAFETY: zero initializes a writable stat buffer.
    let mut stat: libc::stat = unsafe { std::mem::zeroed() };
    // SAFETY: stat is writable and fd remains live.
    if unsafe { libc::fstat(fd, &mut stat) } != 0 {
        return Err(io_error(
            "fstat process executable metadata",
            std::io::Error::last_os_error(),
        ));
    }
    Ok(ProcessExecutableMetadataV8 {
        device: identity.device(),
        inode: identity.inode(),
        size: identity.size(),
        mode: identity.mode(),
        owner_uid: identity.owner_uid(),
        owner_gid: identity.owner_gid(),
        link_count: identity.link_count(),
        mtime_seconds: stat.st_mtime,
        mtime_nanoseconds: stat.st_mtime_nsec,
        ctime_seconds: stat.st_ctime,
        ctime_nanoseconds: stat.st_ctime_nsec,
    })
}

#[cfg(target_os = "linux")]
fn sha256_exact_fd_v8(fd: libc::c_int, size: u64) -> NativeSysResultV8<[u8; 32]> {
    let mut hasher = sha2::Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    let mut offset = 0_u64;
    while offset < size {
        let request = usize::try_from((size - offset).min(buffer.len() as u64)).map_err(|_| {
            NativeSysErrorV8::InvalidInput("process executable read size overflow".to_string())
        })?;
        let file_offset = i64::try_from(offset).map_err(|_| {
            NativeSysErrorV8::InvalidInput("process executable offset overflow".to_string())
        })?;
        // SAFETY: buffer is writable, fd is live, and pread retains no pointer.
        let read = unsafe { libc::pread(fd, buffer.as_mut_ptr().cast(), request, file_offset) };
        if read < 0 {
            return Err(io_error(
                "pread process executable",
                std::io::Error::last_os_error(),
            ));
        }
        if read == 0 {
            return Err(NativeSysErrorV8::RaceDetected(
                "process executable became shorter while hashing".to_string(),
            ));
        }
        let read = usize::try_from(read).map_err(|_| {
            NativeSysErrorV8::InvalidInput(
                "process executable read returned an invalid length".to_string(),
            )
        })?;
        hasher.update(&buffer[..read]);
        offset = offset
            .checked_add(u64::try_from(read).map_err(|_| {
                NativeSysErrorV8::InvalidInput(
                    "process executable read length overflow".to_string(),
                )
            })?)
            .ok_or_else(|| {
                NativeSysErrorV8::InvalidInput("process executable offset overflow".to_string())
            })?;
    }
    Ok(hasher.finalize().into())
}

#[cfg(target_os = "linux")]
fn process_disappeared_v8(error: &NativeSysErrorV8) -> bool {
    matches!(
        error,
        NativeSysErrorV8::Io { source, .. }
            if matches!(source.raw_os_error(), Some(libc::ENOENT | libc::ESRCH))
    )
}

#[cfg(all(test, target_os = "linux"))]
pub(super) fn reset_full_executable_hash_count_for_test_v8() {
    FULL_EXECUTABLE_HASH_COUNT_V8.store(0, std::sync::atomic::Ordering::SeqCst);
}

#[cfg(all(test, target_os = "linux"))]
pub(super) fn full_executable_hash_count_for_test_v8() -> usize {
    FULL_EXECUTABLE_HASH_COUNT_V8.load(std::sync::atomic::Ordering::SeqCst)
}
