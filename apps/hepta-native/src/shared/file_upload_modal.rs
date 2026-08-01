//! Quarantined file-upload preflight adapted from Robrix's upload modal.
//!
//! Hepta already owns a confirmation-first attachment review and Matrix send path in
//! `RoomInputBar`. Importing the upstream widget and worker request wholesale would bypass that
//! product contract. This module therefore lands only local-file inspection and a bounded text
//! preview for a caller-supplied path. It may open, stat, and read up to 128 KiB from that explicit
//! path, but performs no external mutation, upload, send, or picker operation.

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

pub const ROBRIX_UPSTREAM_COMMIT: &str = "a5a664da569c577ab1a3e5a33f45dcc9364954a0";
pub const INTAKE_STATUS: &str = "quarantined_preflight_ready_no_picker_or_send";
pub const LARGE_ATTACHMENT_WARNING_THRESHOLD_BYTES: u64 = 10 * 1_000 * 1_000;
pub const TEXT_PREVIEW_MAX_BYTES: u64 = 128 * 1_024;

pub type FileUploadAttemptId = u64;

/// Generates a process-local identifier for a staged upload attempt.
pub fn next_file_upload_attempt_id() -> FileUploadAttemptId {
    static NEXT_FILE_UPLOAD_ATTEMPT_ID: AtomicU64 = AtomicU64::new(1);
    NEXT_FILE_UPLOAD_ATTEMPT_ID.fetch_add(1, Ordering::Relaxed)
}

/// Metadata produced by local preflight. Ownership remains local until the existing Hepta
/// confirmation and send path explicitly consumes it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileUploadMetadata {
    pub source: PathBuf,
    pub caption: Option<String>,
    pub mime_type: String,
    pub preview: FilePreview,
    pub size: u64,
}

impl FileUploadMetadata {
    pub fn file_name(&self) -> &str {
        self.source
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown file")
    }

    pub fn is_large(&self) -> bool {
        self.size > LARGE_ATTACHMENT_WARNING_THRESHOLD_BYTES
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum FilePreview {
    #[default]
    None,
    /// Image metadata is safe to display; decoding remains owned by the active Hepta review UI.
    Image {
        mime_type: String,
    },
    Text(TextPreview),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextPreview {
    pub content: String,
    pub truncated: bool,
    pub is_code: bool,
}

/// Inspects an already-selected local path without opening a picker or sending anything.
///
/// Run this outside the UI event handler for large or remote-mounted files.
pub fn inspect_local_file(path: impl Into<PathBuf>) -> Result<FileUploadMetadata, String> {
    let source = path.into();
    let metadata =
        std::fs::metadata(&source).map_err(|error| format!("Unable to access file: {error}"))?;
    if !metadata.is_file() {
        return Err("Cannot upload directories or special files".to_string());
    }

    let size = metadata.len();
    let mime_type = mime_guess::from_path(&source)
        .first_or_octet_stream()
        .to_string();
    let preview = if mime_type.starts_with("image/") {
        FilePreview::Image {
            mime_type: mime_type.clone(),
        }
    } else {
        read_text_preview(&source, &mime_type, size)
            .map(FilePreview::Text)
            .unwrap_or_default()
    };

    Ok(FileUploadMetadata {
        source,
        caption: None,
        mime_type,
        preview,
        size,
    })
}

fn read_text_preview(path: &Path, mime_type: &str, file_size: u64) -> Option<TextPreview> {
    if !mimetype_might_be_text(path, mime_type) {
        return None;
    }
    let mut file = File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.by_ref()
        .take(TEXT_PREVIEW_MAX_BYTES)
        .read_to_end(&mut bytes)
        .ok()?;
    let truncated = file_size > bytes.len() as u64;
    let content = bytes_to_string_excerpt(&bytes, truncated)?;
    if content.trim().is_empty() {
        return None;
    }
    Some(TextPreview {
        content,
        truncated,
        is_code: is_code_file(path),
    })
}

fn mimetype_might_be_text(path: &Path, mime_type: &str) -> bool {
    mime_type.starts_with("text/")
        || matches!(
            path.extension()
                .and_then(|extension| extension.to_str())
                .map(str::to_ascii_lowercase)
                .as_deref(),
            Some(
                "c" | "cc"
                    | "cpp"
                    | "css"
                    | "go"
                    | "h"
                    | "hpp"
                    | "html"
                    | "java"
                    | "js"
                    | "json"
                    | "jsx"
                    | "kt"
                    | "md"
                    | "py"
                    | "rb"
                    | "rs"
                    | "sh"
                    | "swift"
                    | "toml"
                    | "ts"
                    | "tsx"
                    | "xml"
                    | "yaml"
                    | "yml"
            )
        )
}

fn is_code_file(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some(
            "c" | "cc"
                | "cpp"
                | "css"
                | "go"
                | "h"
                | "hpp"
                | "html"
                | "java"
                | "js"
                | "json"
                | "jsx"
                | "kt"
                | "py"
                | "rb"
                | "rs"
                | "sh"
                | "swift"
                | "toml"
                | "ts"
                | "tsx"
                | "xml"
                | "yaml"
                | "yml"
        )
    )
}

fn bytes_to_string_excerpt(bytes: &[u8], was_capped: bool) -> Option<String> {
    if bytes.contains(&0) {
        return None;
    }
    match std::str::from_utf8(bytes) {
        Ok(text) => Some(text.to_string()),
        Err(error) if was_capped && error.error_len().is_none() => {
            std::str::from_utf8(&bytes[..error.valid_up_to()])
                .ok()
                .map(ToString::to_string)
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robrix_intake_local_text_preflight_is_bounded_and_performs_no_external_mutation() {
        let directory = tempfile::tempdir().expect("temp directory should be available");
        let path = directory.path().join("message.rs");
        std::fs::write(&path, "fn main() {}\n").expect("fixture should be writable");

        let metadata = inspect_local_file(&path).expect("text fixture should pass preflight");
        assert_eq!(metadata.file_name(), "message.rs");
        assert_eq!(metadata.mime_type, "text/x-rust");
        assert_eq!(
            metadata.preview,
            FilePreview::Text(TextPreview {
                content: "fn main() {}\n".to_string(),
                truncated: false,
                is_code: true,
            })
        );
        assert!(!metadata.is_large());
    }

    #[test]
    fn robrix_intake_directories_are_rejected() {
        let directory = tempfile::tempdir().expect("temp directory should be available");
        assert_eq!(
            inspect_local_file(directory.path()),
            Err("Cannot upload directories or special files".to_string())
        );
    }

    #[test]
    fn robrix_intake_binary_bytes_do_not_become_a_text_preview() {
        assert_eq!(bytes_to_string_excerpt(b"abc\0def", false), None);
    }
}
