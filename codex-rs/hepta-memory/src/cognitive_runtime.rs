use std::fmt;
use std::sync::Arc;

use crate::CognitiveStore;
use crate::CognitiveStoreError;

/// Sanitized reason why an owning runtime could not open its Cognitive Plane.
///
/// This type deliberately carries no filesystem paths, database text, or raw
/// error strings, so it is safe to expose through health and tool surfaces.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CognitiveUnavailableReason {
    InvalidStoreConfiguration,
    AccessDenied,
    RevisionConflict,
    CorruptStore,
    StorageUnavailable,
}

impl From<&CognitiveStoreError> for CognitiveUnavailableReason {
    fn from(error: &CognitiveStoreError) -> Self {
        match error {
            CognitiveStoreError::Invalid(_) => Self::InvalidStoreConfiguration,
            CognitiveStoreError::AccessDenied(_) => Self::AccessDenied,
            CognitiveStoreError::Conflict(_) => Self::RevisionConflict,
            CognitiveStoreError::Corrupt(_) => Self::CorruptStore,
            CognitiveStoreError::Unavailable(_) => Self::StorageUnavailable,
        }
    }
}

/// Process-scoped Cognitive Plane capability supplied to Codex extensions.
///
/// Plain Codex uses `Absent`. An owning Hepta agent uses `Available` after a
/// successful open, or `Unavailable` when opening failed. The latter remains
/// distinct from absence so typed tools can stay visible and report a bounded
/// unavailable state while normal Codex execution continues.
#[derive(Clone, Default)]
pub enum CognitiveRuntime {
    #[default]
    Absent,
    Available(Arc<CognitiveStore>),
    Unavailable(CognitiveUnavailableReason),
}

impl CognitiveRuntime {
    pub fn from_open_result(result: Result<CognitiveStore, CognitiveStoreError>) -> Self {
        match result {
            Ok(store) => Self::Available(Arc::new(store)),
            Err(error) => Self::Unavailable(CognitiveUnavailableReason::from(&error)),
        }
    }

    pub fn available_store(&self) -> Option<&Arc<CognitiveStore>> {
        match self {
            Self::Available(store) => Some(store),
            Self::Absent | Self::Unavailable(_) => None,
        }
    }

    pub fn unavailable_reason(&self) -> Option<CognitiveUnavailableReason> {
        match self {
            Self::Unavailable(reason) => Some(*reason),
            Self::Absent | Self::Available(_) => None,
        }
    }
}

impl fmt::Debug for CognitiveRuntime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Absent => formatter.write_str("CognitiveRuntime::Absent"),
            Self::Available(_) => formatter.write_str("CognitiveRuntime::Available(<owned store>)"),
            Self::Unavailable(reason) => formatter
                .debug_tuple("CognitiveRuntime::Unavailable")
                .field(reason)
                .finish(),
        }
    }
}
