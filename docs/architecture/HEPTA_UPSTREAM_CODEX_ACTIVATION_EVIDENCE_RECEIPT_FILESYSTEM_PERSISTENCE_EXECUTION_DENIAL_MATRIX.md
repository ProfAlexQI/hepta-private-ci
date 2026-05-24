# Hepta Upstream Codex Activation Evidence Receipt Filesystem Persistence Execution Denial Matrix

Gate id: `upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix`

Source filesystem sink write preview gate:
`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-sink-write-preview.sh`

Filesystem persistence execution denial matrix gate:
`scripts/hepta-upstream-codex-activation-evidence-receipt-filesystem-persistence-execution-denial-matrix.sh`

This gate follows sink write preview planning and proves that preview payload
hashes still do not become filesystem write authority. It binds every attempted
execution fixture to a future persistence approval id slot, then denies the
execution path unless all required authority is present and the target remains a
receipt sink rather than workspace, public artifact, or release artifact space.

Current contract:

- Source filesystem sink write preview ready: `true`
- Required denial fixture count: `4`
- Denial fixture count: `4`
- Source preview fixture count: `3`
- Execution requested fixture count: `4`
- Future persistence approval slot count: `4`
- Explicit persistence approval id present count: `3`
- Explicit persistence approval id missing count: `1`
- Stale or missing fresh evidence fixture count: `1`
- Active binary SHA bound fixture count: `4`
- Trusted source bound fixture count: `4`
- Operator approval bound fixture count: `3`
- Workspace path attempt fixture count: `1`
- Public claim attempt fixture count: `1`
- Release artifact write attempt fixture count: `1`
- Blocked execution fixture count: `4`
- Allowed execution fixture count: `0`
- Filesystem persistence allowed count: `0`
- Filesystem persistence execution performed count: `0`
- Workspace write performed count: `0`
- Evidence receipt persisted count: `0`
- Execution denial matrix ready: `true`
- Activation blocked by execution denial matrix: `true`
- Activation allowed by execution denial matrix: `false`
- Active wiring allowed: `false`

Denial fixtures:

- `missing-persistence-approval-id-execution-attempt`
  - Source preview fixture: `receipt-root-sink-write-preview`
  - Payload hash is bound to a future persistence approval id slot.
  - Denial reason: explicit persistence approval id is absent.
  - Execution status: `blocked_execution`
- `stale-live-evidence-execution-attempt`
  - Source preview fixture: `dry-run-root-sink-write-preview`
  - Payload hash is bound to a future persistence approval id slot.
  - Denial reason: fresh live evidence binding is stale or missing.
  - Execution status: `blocked_execution`
- `workspace-path-execution-attempt`
  - Source preview fixture: `receipt-root-sink-write-preview`
  - Payload hash is bound to a future persistence approval id slot.
  - Denial reason: workspace path write is outside receipt sink authority.
  - Execution status: `blocked_execution`
- `public-artifact-execution-attempt`
  - Source preview fixture: `public-artifact-sink-write-preview-attempt`
  - Payload hash is bound to a future persistence approval id slot.
  - Denial reason: public release and artifact writes require separate release governance.
  - Execution status: `blocked_execution`

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

Preview payload hashes are bound to future persistence approval slots, not
write authority. Missing approval, stale evidence, workspace path attempts, and
public artifact attempts all remain blocked before any filesystem persistence
executor can be considered.
