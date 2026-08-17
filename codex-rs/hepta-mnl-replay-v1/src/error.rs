#[derive(Debug, thiserror::Error)]
pub enum ReplayStoreErrorV1 {
    #[error("MNL replay store is blocked: {0}")]
    Blocked(&'static str),
    #[error("invalid MNL replay-store material: {0}")]
    Invalid(String),
    #[error("MNL replay slot already has a final claim and is blocked")]
    ExistingFinalBlocksReplay,
    #[error("MNL replay slot has an incoming residue and is uncertain")]
    IncomingResidueBlocks,
    #[error("MNL replay-store identity mismatch: {0}")]
    IdentityMismatch(String),
    #[error("MNL replay-store race or replacement detected: {0}")]
    RaceDetected(String),
    #[error("{operation} failed with errno {errno}")]
    Syscall { operation: &'static str, errno: i32 },
    #[error("{operation}: {source}")]
    Io {
        operation: &'static str,
        #[source]
        source: std::io::Error,
    },
}

pub type ReplayStoreResultV1<T> = Result<T, ReplayStoreErrorV1>;

#[cfg(target_os = "linux")]
pub(crate) fn io_error(operation: &'static str, source: std::io::Error) -> ReplayStoreErrorV1 {
    ReplayStoreErrorV1::Io { operation, source }
}

#[cfg(target_os = "linux")]
pub(crate) fn syscall_error(
    operation: &'static str,
    source: rustix::io::Errno,
) -> ReplayStoreErrorV1 {
    ReplayStoreErrorV1::Syscall {
        operation,
        errno: source.raw_os_error(),
    }
}

#[cfg(target_os = "linux")]
pub(crate) fn invalid(message: impl Into<String>) -> ReplayStoreErrorV1 {
    ReplayStoreErrorV1::Invalid(message.into())
}
