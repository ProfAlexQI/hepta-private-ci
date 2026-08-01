//! Quarantined attachment-download presentation state adapted from Robrix.
//!
//! Upstream download and share execution depends on newer `robius-file-picker`, `robius-share`,
//! and Matrix worker contracts. Those side effects are intentionally not imported here. The
//! compile-ready metadata and state machine can be wired later by the platform adapter lane.

use ruma::{OwnedMxcUri, events::room::MediaSource};

pub const ROBRIX_UPSTREAM_COMMIT: &str = "a5a664da569c577ab1a3e5a33f45dcc9364954a0";
pub const INTAKE_STATUS: &str = "quarantined_download_model_no_io_or_share";
pub const DOWNLOAD_RESULT_DURATION_SECS: f64 = 5.0;

/// The MXC URI inside a plain or encrypted media source.
pub fn media_source_mxc(source: &MediaSource) -> &OwnedMxcUri {
    match source {
        MediaSource::Plain(uri) => uri,
        MediaSource::Encrypted(file) => &file.url,
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TransferKind {
    Download,
    Share,
}

#[derive(Clone, Debug)]
pub struct PendingDownload {
    pub mxc: OwnedMxcUri,
    pub state: PendingDownloadState,
    pub kind: TransferKind,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PendingDownloadState {
    InProgress,
    JustSucceeded,
    JustFailed,
}

impl PendingDownloadState {
    pub fn display(self, kind: TransferKind) -> DownloadDisplayState {
        match self {
            Self::InProgress => DownloadDisplayState::InProgress,
            Self::JustSucceeded => DownloadDisplayState::Succeeded(kind),
            Self::JustFailed => DownloadDisplayState::Failed,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum DownloadDisplayState {
    #[default]
    Idle,
    InProgress,
    Succeeded(TransferKind),
    Failed,
}

#[derive(Clone, Debug)]
pub struct DownloadableAttachment {
    pub media_source: MediaSource,
    pub filename: String,
    pub size: Option<u64>,
    pub kind: DownloadKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DownloadKind {
    File,
    Audio,
    Video,
    Image,
}

impl DownloadKind {
    pub fn basic_mime_type(self) -> Option<&'static str> {
        match self {
            Self::Image => Some("image/*"),
            Self::Audio => Some("audio/*"),
            Self::Video => Some("video/*"),
            Self::File => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robrix_intake_completion_copy_preserves_transfer_kind() {
        assert_eq!(
            PendingDownloadState::JustSucceeded.display(TransferKind::Share),
            DownloadDisplayState::Succeeded(TransferKind::Share)
        );
        assert_eq!(
            PendingDownloadState::JustFailed.display(TransferKind::Download),
            DownloadDisplayState::Failed
        );
    }

    #[test]
    fn robrix_intake_file_download_does_not_invent_a_mime_type() {
        assert_eq!(DownloadKind::File.basic_mime_type(), None);
        assert_eq!(DownloadKind::Image.basic_mime_type(), Some("image/*"));
    }
}
