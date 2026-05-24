# Hepta Upstream Codex Activation Evidence Receipt Filesystem Output Path Allowlist

Gate id: `upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist`

Source filesystem persistence approval packet gate:
`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-approval-packet.sh`

Filesystem output path allowlist gate:
`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh`

This gate defines the redacted output-path allowlist that must exist before any
future upstream Codex activation evidence receipt can target a filesystem sink.
It is still a report-only contract: allowed roots are modeled, but no output
path is selected by default and no filesystem persistence is allowed.

Current contract:

- Source filesystem persistence approval packet ready: `true`
- Required allowlist entry count: `6`
- Allowlist entry count: `6`
- Allowed output path entry count: `3`
- Blocked output path entry count: `3`
- Redacted output path entry count: `6`
- Default selected output path count: `0`
- Source tree path allowed: `false`
- Home directory path allowed: `false`
- Release artifact path allowed: `false`
- Public artifact path allowed: `false`
- Receipt output path allowlist ready: `true`
- Filesystem persistence allowed: `false`
- Filesystem persistence execution performed: `false`
- Workspace write performed: `false`
- Evidence receipt persisted: `false`
- Activation blocked by output path allowlist: `true`
- Activation allowed by output path allowlist: `false`
- Active wiring allowed: `false`

Allowlist entries:

- `activation_evidence_receipts_root`
- `activation_evidence_dry_run_root`
- `activation_evidence_operator_packet_root`
- `source_tree_root`
- `home_directory_root`
- `release_artifact_root`

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

This gate separates path eligibility from write authority. A redacted output
root may be eligible for receipt persistence only after the approval packet is
complete, but source-tree paths, home-directory paths, release artifacts, and
public artifact paths stay blocked from the receipt persistence sink.
