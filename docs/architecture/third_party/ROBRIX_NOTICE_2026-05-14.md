# Robrix / Project Robius Notice

Hepta Native initially incorporates code from Robrix.

- Source: https://github.com/project-robius/robrix
- Source commit: b2bb6cf
- License: MIT
- Copyright: Copyright (c) 2023-2026 Project Robius Developers

The MIT license text is preserved at `apps/hepta-native/LICENSE-MIT`.

This copy is used for the Matrix-heart fast path described in `docs/architecture/HEPTA_ROBRIX_DESKTOP_MOBILE_UI_DEVELOPMENT_2026-05-14.md`.

## Selective UI intake update (2026-08-01)

Six additional Robrix UI modules were selectively adapted from exact upstream commit
`a5a664da569c577ab1a3e5a33f45dcc9364954a0`, also under MIT:

- `src/shared/file_upload_modal.rs`
- `src/home/upload_progress.rs`
- `src/shared/attachment_download.rs`
- `src/shared/mention_popup.rs`
- `src/shared/room_input_popup_menu.rs`
- `src/shared/slash_commands.rs`

The corresponding Hepta files preserve source constants with that exact commit. They are modified
to retain Hepta's confirmation-first composer and existing Matrix queue ownership: only the room
input popup is registered as a dormant widget. The file-upload preflight may open, stat, and read up
to 128 KiB from an explicit caller-supplied local path; it performs no upload, send, picker, or
external mutation. The remaining upload/download/mention/progress adapters are parser- or
presentation-only until their platform and worker contracts are explicitly promoted.
The per-file modification and quarantine status is recorded in
`docs/architecture/third_party/ROBRIX_COPY_MANIFEST_2026-05-14.md`.
