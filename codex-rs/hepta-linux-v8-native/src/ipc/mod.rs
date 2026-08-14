//! Kernel-identified, one-request Linux `AF_UNIX/SOCK_SEQPACKET` transport.
//!
//! The transport verifies kernel-provided peer credentials, binds the peer PID
//! to a pidfd/start-time token, requires one exact `SCM_CREDENTIALS` record on
//! every packet, and rejects truncated payload or ancillary data.
//! It does not decide whether the observed peer is authorized; a future
//! admission boundary must compare it with a frozen expected-peer profile and
//! one-shot capability before transferring sensitive data or descriptors.

mod frame;
mod peer;

pub use frame::*;
pub use peer::*;

#[derive(Debug, thiserror::Error)]
pub enum NativeIpcErrorV8 {
    #[error("Linux seqpacket boundary is unsupported on this platform: {0}")]
    UnsupportedPlatform(&'static str),
    #[error("invalid seqpacket input: {0}")]
    InvalidInput(String),
    #[error("seqpacket protocol violation: {0}")]
    ProtocolViolation(String),
    #[error("{operation}: {source}")]
    Io {
        operation: &'static str,
        #[source]
        source: std::io::Error,
    },
    #[error("peer process verification failed: {0}")]
    PeerProcess(#[from] crate::NativeSysErrorV8),
}

pub type NativeIpcResultV8<T> = Result<T, NativeIpcErrorV8>;

#[cfg(target_os = "linux")]
pub(super) fn ipc_io(operation: &'static str, source: std::io::Error) -> NativeIpcErrorV8 {
    NativeIpcErrorV8::Io { operation, source }
}

#[cfg(not(target_os = "linux"))]
pub(super) fn ipc_unsupported(operation: &'static str) -> NativeIpcErrorV8 {
    NativeIpcErrorV8::UnsupportedPlatform(operation)
}
