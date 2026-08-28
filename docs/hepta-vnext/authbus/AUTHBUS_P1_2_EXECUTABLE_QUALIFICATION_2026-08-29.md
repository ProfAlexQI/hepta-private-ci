# AuthBus P1.2 Executable Qualification Receipt

Date: 2026-08-29
Stage: P1.2 durable identity and evidence replay state
Disposition: **EXECUTABLE-QUALIFIED / QUALIFICATION-ONLY**

## Exact source binding

```text
branch=integration/vnext-main-full-ci-authbus-p1-2-20260828
source_head=a8fbf6ee5daf6321365c487a1540438f07a524f0
source_tree=5ea8ac165f9670de7b25427ae72966c0ded99ada
workflow=.github/workflows/authbus-p1-2-qualification.yml
run_id=33194146682
run_attempt=1
```

This receipt is evidence for the exact source head above. The receipt commit may
contain only this Markdown file and its adjacent JSON receipt relative to the
source head.

## Hosted execution evidence

All required jobs completed successfully with non-zero hosted runner IDs and
non-empty steps:

| Job | Job ID | Runner ID | Result |
|---|---:|---:|---|
| Exact source, cleanup, receipt, and negative-authority contracts | 98926869344 | 1000030368 | PASS |
| Inherited P1.1 signed identity and evidence matrix | 98927608040 | 1000030465 | PASS |
| AuthBus P1.2 durable replay Rust qualification | 98930833938 | 1000030516 | PASS |

The exact-head run passed:

- source/schema and one-shot cleanup contracts;
- inherited P1.1 signature, key-purpose, identity and evidence gates;
- Rust 1.95 committed-lock verification and rustfmt;
- default-feature-off, no-authority posture;
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

The archived Rust receipt binds `rustc 1.95.0`, `cargo 1.95.0`, the exact commit
and tree, and all negative-authority fields. Artifact IDs and SHA-256 digests are
recorded in the adjacent JSON receipt.

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
membership, listener/provider/OpenBao wiring and P1.3 quota-registry work remain
separate review and qualification boundaries.
