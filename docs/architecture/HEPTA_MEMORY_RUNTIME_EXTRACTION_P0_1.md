# Hepta Memory Runtime Extraction P0.1

## Purpose

This tranche introduces `codex-hepta-memory-runtime` as the only Agent-facing runtime facade for opening and shutting down the cognitive-memory runtime. It is an ownership-boundary extraction, not a production-authority promotion.

## Exact source prerequisites

The extraction is eligible to run only after the architecture-convergence branch contains both of these fail-closed source ratchets:

1. legacy `ProductionAuthorityLease` conversion is verified by an external `CapabilityVerifier` before a typed `Authorized<CognitiveWriteCapability>` can be returned;
2. every cross-owner `ProductionDurableWriter::open` call is routed through the Agentd production-writer host, and attaching a production outbox target requires a separately verified `Authorized<ExternalEffectCapability>`.

The prerequisites were landed on PR #53 by commits:

- `d7f4d2e739f94238321dfb759ad1ddcf353c702e`
- `77e54498885c0bbee72ac268f8a111d06a8320d7`

## Allowed mutation surface

The bounded bootstrap may change only:

- `codex-rs/Cargo.toml`
- `codex-rs/Cargo.lock`
- `codex-rs/hepta-agentd/Cargo.toml`
- `codex-rs/hepta-agentd/src/composition.rs`
- `codex-rs/hepta-agentd/src/memory_service.rs`
- `docs/architecture/HEPTA_ARCHITECTURE_CONVERGENCE_P0_2_STATUS.json`
- `scripts/verify-hepta-memory-runtime-extraction-p0-1.py`
- the bootstrap workflow itself when it retires after a successful exact-head writeback

No schema, store, retrieval, projection, provider, automation, or production-effect implementation is moved in this tranche.

## Required gates

A writeback is valid only when all of the following pass on the same checked-out head:

- architecture master-plan verification;
- authority-caller ratchet verification;
- Memory Runtime extraction status verification;
- Rust formatting;
- targeted `codex-hepta-memory-runtime` tests;
- targeted Agentd memory-service tests;
- `cargo check --locked` for both affected crates;
- strict Clippy with warnings denied for both affected crates;
- exact-head compare before a non-force fast-forward push.

## Authority boundary

This tranche grants no runtime authority, cognitive-write authority, external-effect authority, operator acceptance, release promotion, or production-writer authority. Hosted qualification, operator acceptance, and promotion remain independent fail-closed gates.
