# Hepta Upstream Codex Activation Evidence Receipt Filesystem Output Path Evidence Binding

Gate id: `upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding`

Source filesystem output path allowlist gate:
`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-allowlist.sh`

Filesystem output path evidence binding gate:
`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-output-path-evidence-binding.sh`

This gate binds every future receipt output destination to fresh live evidence
and the active Hepta binary SHA before any filesystem sink can select a path.
It is still a report-only contract: the binding fields are required, but no
binding record is recorded by default, no output path is selected, and no
filesystem persistence is allowed.

Current contract:

- Source filesystem output path allowlist ready: `true`
- Required path binding count: `8`
- Path binding count: `8`
- Allowed output path entry count: `3`
- Selected output path count: `0`
- Recorded path binding count: `0`
- Fresh live evidence bound count: `0`
- Active binary SHA bound count: `0`
- Redacted or hashed binding count: `8`
- Trusted source bound count: `0`
- Source tree path binding allowed: `false`
- Home directory path binding allowed: `false`
- Release artifact path binding allowed: `false`
- Public artifact path binding allowed: `false`
- Output path evidence binding ready: `true`
- Filesystem persistence allowed: `false`
- Filesystem persistence execution performed: `false`
- Workspace write performed: `false`
- Evidence receipt persisted: `false`
- Activation blocked by output path evidence binding: `true`
- Activation allowed by output path evidence binding: `false`
- Active wiring allowed: `false`

Required evidence bindings:

- `activation_request_id`
- `operator_approval_id`
- `operator_identity_hash`
- `live_dependency_isolation_evidence_id`
- `watchdog_evidence_id`
- `browser_smoke_evidence_id`
- `long_soak_evidence_id`
- `rollback_plan_id`

Allowed output path bindings:

- `activation_evidence_receipts_root`
- `activation_evidence_dry_run_root`
- `activation_evidence_operator_packet_root`

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

This gate separates output-path eligibility from evidence freshness. An
allowlisted root is not enough to choose a filesystem destination; the future
receipt sink must also bind fresh live evidence, active binary SHA, trusted
source verification, operator approval, activation request identity, and
rollback evidence before persistence can be considered.

Next gate:
`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh`
models redacted sink write previews with deterministic payload hashes while
keeping filesystem persistence and workspace writes blocked.
