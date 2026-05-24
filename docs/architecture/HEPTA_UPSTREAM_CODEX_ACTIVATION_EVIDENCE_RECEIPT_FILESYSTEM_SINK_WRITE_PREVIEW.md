# Hepta Upstream Codex Activation Evidence Receipt Filesystem Sink Write Preview

Gate id: `upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview`

Source filesystem output path evidence binding gate:
`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh`

Filesystem sink write preview gate:
`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh`

This gate models a dry-run sink write preview after output-path evidence
binding. It can assemble redacted output paths and deterministic payload hashes
for the allowed receipt roots, but it still cannot execute filesystem
persistence, write the workspace, persist evidence receipts, activate wiring, or
write public release artifacts.

Current contract:

- Source filesystem output path evidence binding ready: `true`
- Required preview fixture count: `3`
- Preview fixture count: `3`
- Allowed output path entry count: `3`
- Previewed output path count: `3`
- Deterministic payload hash count: `3`
- Redacted output path preview count: `3`
- Fresh live evidence bound fixture count: `3`
- Active binary SHA bound fixture count: `3`
- Trusted source bound fixture count: `3`
- Operator approval bound fixture count: `3`
- Blocked preview fixture count: `3`
- Allowed preview fixture count: `0`
- Public claim attempt fixture count: `1`
- Release artifact write attempt fixture count: `1`
- Filesystem persistence allowed count: `0`
- Workspace write performed count: `0`
- Evidence receipt persisted count: `0`
- Sink write preview ready: `true`
- Activation blocked by sink write preview: `true`
- Activation allowed by sink write preview: `false`
- Active wiring allowed: `false`

Preview fixtures:

- `receipt-root-sink-write-preview`
  - Output path root: `activation_evidence_receipts_root`
  - Redacted output path is present.
  - Deterministic payload hash is present.
  - Preview status: `blocked_preview`
- `dry-run-root-sink-write-preview`
  - Output path root: `activation_evidence_dry_run_root`
  - Redacted output path is present.
  - Deterministic payload hash is present.
  - Preview status: `blocked_preview`
- `public-artifact-sink-write-preview-attempt`
  - Output path root: `activation_evidence_operator_packet_root`
  - Public claim and release artifact write are requested.
  - Preview status: `blocked_preview`

Side-effect boundary:

- No upstream fetch/merge/checkout
- No command invocation performed
- No receipt persistence execution
- No materialization execution
- No filesystem persistence execution
- No workspace write
- No evidence receipt persistence
- No active service restart
- No credential or secret read
- No provider or model invocation
- No channel delivery
- No gateway RPC
- No public release publication
- No public GA claim
- No release artifact write

This gate separates sink preview planning from filesystem authority.
Deterministic payload hashes and redacted output paths are preview evidence, not
write authority. A later execution-denial matrix must still prove that any
future persistence execution path remains blocked until explicit approval,
fresh evidence, and write-path enablement are all present.
