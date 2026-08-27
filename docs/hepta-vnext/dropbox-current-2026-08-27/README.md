# Dropbox Hepta development-document snapshot (2026-08-27)

This directory is a byte-for-byte snapshot of the latest readable Hepta
development documents visible in the local Dropbox File Provider at capture
time.  It accompanies the canonical Hepta integration source tree; it is a
documentation/transport snapshot, not a release or runtime-authority input.

## Source and binding

- Source: `/Users/qianqi/Dropbox/OpenClaw`
- Plan: `hepta-vnext-development-plan-final-2026-08-23.md`
  - SHA-256: `91cdd48ddf11bb2e322cc9dcce443d6089cbd31e55f8d471825dd7e6a83fdf3a`
- Qualification index: `HEPTA_VNEXT_QUALIFICATION_INDEX.md`
  - SHA-256: `344fd9bcce14fc225b668f8e9601bb3742a619fe23d8c0946b2e408d21159c0c`
- Canonical source snapshot: `HEAD a85612afb43af722c61b54efe73570b25e9e4031`,
  tree `71026adff61523660d953867188f094184cee2e9`, parent
  `9138ea52a4683489b1a1012cc6da1f2bcde469bf`

The E.45 plan/index binding is still `UNRECEIPTED_E45_FAIL_CLOSED`.  The
effective-index and qualification-authority files that still point to older
inputs are retained and marked as stale in the manifest.  Nothing in this
snapshot enables production/effect/model/NPU/fleet/promotion authority.

## Layout

- `root/` — current readable root-level development documents.
- `historical/qualification/` — the newest readable E44 successor, tree
  hygiene, and HNL/INF qualification snapshots, explicitly historical.
- `archive/` — the superseded browser-plan archive when readable.
- `DROPBOX-SOURCE-MANIFEST.json` — per-file byte counts, SHA-256 digests,
  source mtimes, unavailable cloud-only files, and exclusion rationale.

## Deliberate omissions

Several browser-plan files remained cloud-only and returned macOS
`Resource deadlock avoided` when read.  They are listed with their advertised
size/mtime and error in `DROPBOX-SOURCE-MANIFEST.json`; no empty placeholders
were created.  Large generated qualification/worktree exports and older
duplicate rebind iterations are also listed there and were not recursively
copied.  Credentials, build outputs, caches, and symlinks were excluded.

For machine-verifiable details, treat the manifest and Git commit together;
do not treat this archival snapshot as a current authority receipt.
