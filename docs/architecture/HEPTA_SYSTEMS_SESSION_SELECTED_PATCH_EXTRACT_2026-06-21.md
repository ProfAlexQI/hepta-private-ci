# Hepta Systems Session Selected Patch Extract - 2026-06-21

## Purpose

This note defines the selected patch extraction step after the ordered session
patch queue. The extractor retrieves exact historical `apply_patch` bodies for a
chosen Phase 0 anchor without applying them.

## Local Commands

- Report: `scripts/hepta-systems-session-selected-patch-extract-report.sh`
- Gate: `scripts/hepta-systems-session-selected-patch-extract-gate.sh`

Useful environment variables:

- `HEPTA_SESSION_PATCH_ANCHOR_ID`
- `HEPTA_SESSION_PATCH_CALL_ID`
- `HEPTA_SESSION_PATCH_EXTRACT_LIMIT`

The default anchor is `plugin_contribution_point_abi` with a bounded extract
limit. Patch bodies are emitted only by this explicit extractor, not by the
queue report.

## Boundary

The extractor is still report-only. It does not invoke `apply_patch`, `patch`,
`git apply`, `git add`, commit, push, release, deploy, or any live mutation path.

The next local step is to compare one selected patch body against the current
checkout and reconstruct the smallest compatible Phase 0 surface through normal
reviewed file edits.
