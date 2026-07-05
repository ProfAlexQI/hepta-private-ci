# Terminal Public Claim Delivery Receipt Artifact Signing Receipt Replay/Idempotency Attachment

This attachment consumes the terminal public claim delivery receipt artifact signing receipt non-persistence final index and proves that the next artifact signing receipt replay/idempotency target is present for this branch.

Status: ready-but-blocked. The attachment only source-probes the signing receipt replay/idempotency denial gate and architecture note; it does not invoke signing replay gates, signing receipt target gates, or terminal live gates.

Denied surfaces remain false: signing receipt replay, duplicate identity, idempotency key/state, nonce replay, cross-scope reuse, status upgrade, ack replay, ledger/index/query/export/observability replay, hash/status rebind, artifact/package/signature/notarization/ticket/stapling/installer/release/CDN/registry receipt replay, external/Telegram receipt replay, approval, release publication authority, activation authority, install, restart, active binary, provider, and credential access.
