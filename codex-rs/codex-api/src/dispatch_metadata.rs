use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

/// Shared witness that a prepared request reached its transport invocation.
///
/// `false` proves the host may classify an aborted attempt as not dispatched.
/// `true` is deliberately conservative: it means the transport was invoked,
/// not that bytes reached a remote peer, so an unobserved outcome is
/// indeterminate rather than safely retryable.
#[derive(Clone, Default)]
pub struct RequestDispatchMetadata {
    transport_invoked: Arc<AtomicBool>,
}

impl RequestDispatchMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn transport_invoked(&self) -> bool {
        self.transport_invoked.load(Ordering::Acquire)
    }

    pub(crate) fn mark_transport_invoked(&self) {
        self.transport_invoked.store(true, Ordering::Release);
    }
}
