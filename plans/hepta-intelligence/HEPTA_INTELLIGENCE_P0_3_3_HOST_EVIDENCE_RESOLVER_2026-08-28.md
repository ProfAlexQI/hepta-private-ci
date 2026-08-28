# P0.3.3 — HOST_OWNED_EVIDENCE_RESOLUTION

**Repository**: `ProfHepta/hepta-private-ci`  
**State**: restacked / exact qualification pending

## Contract

The dormant model-facing input contains either:

```json
{"quote":"Project Aurora uses Rust","occurrence":0}
```

or:

```json
{"segment_id":"source-segment:v1:<64 lowercase hex>"}
```

The model never supplies `start_byte`, `end_byte`, `sha256`, or a verification boolean. Rust resolves the exact witness, computes UTF-8 byte ranges and SHA-256, rejects duplicate or overlapping ranges within one fact, and lowers the result into the existing grounded v3 contract.

## Dependency

P0.3.2 is executable-qualified at:

```text
fa59bb090043ba8d6fbf0991b167779d2385888c
run 33190943793
```

The P0.3.3 branch is stacked directly on that exact head.

## Qualification

The v5 workflow binds repository identity, exact head/tree, the P0.3.2 qualified state, all compile/test/format/Clippy exits, and a clean source tree. A stale pending run cannot serialize a newer exact head because the concurrency group includes `github.sha`.

## Boundary

```text
tool_v4_registered=false
production_authority=false
production_projection_gate=false
external_effects=false
operator_accepted=false
promoted=false
callers_ratchet=false
```

P0.3.4 inventory/backfill/quarantine must not start until the exact P0.3.3 receipt is all-green.
