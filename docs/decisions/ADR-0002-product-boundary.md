# ADR-0002: Hepta remains an OpenClaw-governed backend

- Status: accepted
- Date: 2026-07-27
- Decision owner: Hepta architecture

## Context

Hepta now has a hardened execution admission path, durable runtime state, mutation journals, rollback anchors, release preflight, and local operational recovery. It does not yet own the production Telegram poller, does not enable external Telegram read/send or native real mutation by default, and the formal `hepta` service intentionally does not link the full Codex provider/tool runner.

Treating that state as an independent product would create two authorities for Telegram and would weaken the explicit live-mutation boundary. Treating every upstream Codex module as mandatory would also import product surfaces that Hepta does not currently need.

## Decision

Hepta is the governed backend and local runtime behind OpenClaw. OpenClaw remains the channel owner and user-facing orchestration plane until a separately approved controlled-live migration proves single ownership, provider isolation, rollback, and terminal receipts.

The following remain disabled by default:

- Telegram external read and send
- Hepta Telegram poll-loop ownership
- Native real mutation
- Full Codex provider/tool-runner linkage in the formal service
- Public release, signing, notarization, and remote publication

Upstream Codex intake remains semantic and selective. Hepta absorbs safety, lifecycle, replay, MCP-catalog, and app-server protocol improvements when they preserve the governed boundary. It does not merge the upstream tree wholesale.

## Module policy

Adopt now:

- explicit in-process shutdown
- terminal turn-error replay
- pre-start cached MCP tool visibility with live-call revalidation
- thread search, item lineage, and writer-lock improvements after local contract review
- connector catalog persistence and App Server V2 improvements that do not activate providers

Adopt only after a product requirement:

- code mode
- WebSocket transport
- cloud configuration and external migration
- in-app updates

Do not adopt as a bundle:

- the full provider/tool runner
- upstream TUI, SDK, or cloud surfaces without a Hepta owner and acceptance contract
- a second Telegram poller

## Exit criteria

Reconsider standalone-product status only after one release candidate proves all of the following with live gates still fail-closed by default:

1. one Telegram owner with transactional handoff and rollback
2. provider execution through `ExecutionAdmission` and `EffectBroker`
3. durable terminal receipts and cross-restart replay protection
4. no plan-only mutation routes presented as implemented product behavior
5. an operator-approved controlled-live deployment
