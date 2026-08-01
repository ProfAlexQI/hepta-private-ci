//! Quarantined upload-progress state adapted from Robrix.
//!
//! The upstream widget owns an abort handle and submits retry requests. Hepta's accepted Matrix
//! send queue is owned by the timeline, while composer retry is confirmation-gated. This adapter
//! therefore exposes a deterministic presentation model only and performs no queue mutation.

use crate::shared::file_upload_modal::FileUploadAttemptId;

pub const ROBRIX_UPSTREAM_COMMIT: &str = "a5a664da569c577ab1a3e5a33f45dcc9364954a0";
pub const INTAKE_STATUS: &str = "quarantined_progress_model_no_queue_control";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum UploadProgressState {
    #[default]
    Ready,
    Uploading,
    Queued,
    Failed {
        message: String,
    },
    Cancelled,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UploadProgressSnapshot {
    pub upload_id: FileUploadAttemptId,
    pub file_name: String,
    pub bytes_sent: u64,
    pub bytes_total: u64,
    pub state: UploadProgressState,
}

impl UploadProgressSnapshot {
    pub fn new(
        upload_id: FileUploadAttemptId,
        file_name: impl Into<String>,
        bytes_total: u64,
    ) -> Self {
        Self {
            upload_id,
            file_name: file_name.into(),
            bytes_sent: 0,
            bytes_total,
            state: UploadProgressState::Ready,
        }
    }

    /// Returns a stable UI fraction, including a defined value for empty files.
    pub fn fraction(&self) -> f64 {
        if self.bytes_total == 0 {
            return 0.0;
        }
        (self.bytes_sent.min(self.bytes_total) as f64 / self.bytes_total as f64).clamp(0.0, 1.0)
    }

    /// Applies read-only progress received from the actual queue owner.
    pub fn observe_progress(&mut self, bytes_sent: u64) {
        self.bytes_sent = bytes_sent.min(self.bytes_total);
        self.state = UploadProgressState::Uploading;
    }

    pub fn can_cancel(&self) -> bool {
        matches!(self.state, UploadProgressState::Uploading)
    }

    pub fn can_retry(&self) -> bool {
        matches!(self.state, UploadProgressState::Failed { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robrix_intake_progress_is_clamped_and_does_not_imply_delivery() {
        let mut snapshot = UploadProgressSnapshot::new(7, "photo.png", 100);
        snapshot.observe_progress(140);
        assert_eq!(snapshot.fraction(), 1.0);
        assert_eq!(snapshot.state, UploadProgressState::Uploading);
        assert!(snapshot.can_cancel());
        assert!(!snapshot.can_retry());
    }

    #[test]
    fn robrix_intake_empty_file_fraction_is_defined() {
        assert_eq!(
            UploadProgressSnapshot::new(1, "empty.txt", 0).fraction(),
            0.0
        );
    }
}
