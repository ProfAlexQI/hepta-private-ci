# AuthBus P1.2 Executable Qualification Receipt

Date: 2026-08-29  
Stage: P1.2 durable identity and evidence replay state  
Disposition: **EXECUTABLE-QUALIFIED / QUALIFICATION-ONLY**

## Exact source binding

```text
branch=integration/vnext-main-full-ci-authbus-p1-2-20260828
source_head=1efd9bcdc946625f2d27c03080840c3638e855c2
source_tree=86d0b9ca39c41dd8ffa05f2e94039b31dbb199b8
workflow=.github/workflows/authbus-p1-2-qualification.yml
run_id=33233070831
run_attempt=1
```

This receipt is evidence only for the exact source head above. Relative to that
source head, the receipt commit contains only this Markdown file and the
adjacent machine-readable JSON receipt.

## Hosted execution evidence

All three required jobs completed successfully on non-zero hosted runner IDs
with non-empty step records:

| Job | Job ID | Runner ID | Runner class | Result |
|---|---:|---:|---|---|
| Exact source, cleanup, receipt, and negative-authority contracts | 99049152741 | 1000034658 | `ubuntu-24.04` | PASS |
| Inherited P1.1 signed identity and evidence matrix | 99050020546 | 1000034808 | `ubuntu-24.04` | PASS |
| AuthBus P1.2 durable replay Rust qualification | 99052281710 | 1000035026 | `ubuntu-24.04` | PASS |

The exact-head run passed:

- source/schema and one-shot cleanup contracts;
- inherited P1.1 signature, key-purpose, identity and evidence gates;
- Rust 1.95 committed-lock verification and rustfmt;
- default-feature-off, no-authority posture;
- purpose-inclusive verification-key identity and cross-purpose isolation;
- key rotation and revocation durability across reopen;
- nonce replay durability and fail-closed capacity;
- provider-status replay/conflict and terminal tombstone durability;
- independent manual-evidence revisions and lookup-only resume;
- writer-generation takeover and stale-handle fencing;
- seven explicit pre-commit rollback sites;
- bounded CAS garbage collection;
- row-digest corruption detection and fail-closed reopen;
- all-target `cargo check`;
- strict all-target Clippy with `-D warnings`.

## Artifact bindings

| Artifact | Artifact ID | SHA-256 |
|---|---:|---|
| `authbus-p1-2-source-33233070831` | 9709170308 | `82f1d66ff009ffbe7d1872bcdb2ff4dc9f649acd28869fda9f49592bfdb7dd80` |
| `authbus-p1-1-inherited-33233070831` | 9709413682 | `ca3e426f744ffa860a7955b6170abec95286cb13564e9a1f676af1ce407e921e` |
| `authbus-p1-2-rust-33233070831` | 9709747656 | `5b8710d69716f99b6cecaf39a6d6414c647bf00a5aa807dab40310d45a4b146e` |

The adjacent JSON receipt carries the same job, runner and artifact bindings in
machine-readable form.

## Authority boundary

Executable qualification does **not** activate product or effect authority:

```text
qualification_only=true
authority=false
effect_authority=false
production_caller=false
production_writer=false
operator_acceptance=false
promotion=false
g5_allowed=false
execute_allowed=false
listener_enabled=false
provider_call_enabled=false
openbao_enabled=false
parent_workspace_wired=false
private_key_storage=false
raw_signature_storage=false
secret_storage=false
```

P1.2 is closed as an isolated executable-qualified tranche. Product workspace
membership, listener/provider/OpenBao wiring, production authority and any
subsequent P1.x tranche remain separate plan, review and qualification
boundaries.
