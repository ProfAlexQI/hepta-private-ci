# Hepta Channel Adapter Status Inventory

Date: 2026-05-20
Scope: old standalone Hepta channel/runtime adapter ops modules versus current `hepta-codex`
Status: disabled/live-gated status inventory landed; no channel read/send enabled

## Summary

The runtime/session dry-run slice left the channel adapter family as the next
safe migration target. This slice exposes those old modules only as disabled
status entries:

- `/api/hepta-channel-adapter-status-inventory`
- source-command equivalent: `/hepta-channel-adapter-status-inventory --json`
- validation script: `scripts/hepta-codex-channel-adapter-status-inventory.sh`

The route does not read connector credentials, poll external channels, send
messages, perform Telegram owner handoff, place voice calls, play TTS audio,
deliver webhooks, transfer files, mutate Gateway/native POST state, or write
files.

## Inventory Counts

- old channel/runtime adapter ops files covered: `13`
- channel adapters inventoried: `13`
- disabled status entries ready: `13`
- live adapters enabled: `0`
- current `hepta-codex` scripts: `11`
- current native gateway source commands: `58`
- Control UI route parity after memory/capability absorption inventory continuation: `58/58`, missing `0`

## Files Covered

- `bonjour_ops.rs`
- `discord_ops.rs`
- `feishu_ops.rs`
- `file_transfer_ops.rs`
- `google_meet_ops.rs`
- `googlechat_ops.rs`
- `imessage_ops.rs`
- `message_ops.rs`
- `native_channel_metadata_ops.rs`
- `telegram_ops.rs`
- `tts_local_cli_ops.rs`
- `voice_call_ops.rs`
- `webhooks_ops.rs`

## Boundary

This is a visibility and sequencing surface only. It intentionally keeps old
CLI invocation compatibility unclaimed and keeps all channel live reads/sends
disabled. Telegram remains under the legacy OpenClaw owner until a separate
explicit operator handoff request exists.

## Remaining Blockers

- channel live read is not operator-approved
- channel live send is not operator-approved
- Telegram owner handoff is not requested
- channel credentials are not read by this inventory
- old channel CLI invocation compatibility is not claimed

## Safe Next Slice

Port remaining external release and hardening scripts as local-only status gates before any
process execution, filesystem mutation, network read, file transfer, channel
delivery, or webhook smoke.
