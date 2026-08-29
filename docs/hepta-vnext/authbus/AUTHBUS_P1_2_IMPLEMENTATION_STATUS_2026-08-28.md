# AuthBus P1.2 implementation status — 2026-08-28

Current claim:

```text
IMPLEMENTED_SOURCE / EXECUTABLE_QUALIFICATION_PENDING / NO_AUTHORITY
```

The branch contains an isolated Rust 1.95 nested workspace, SQLite WAL schema,
purpose-inclusive verification-key identity across primary/foreign keys, Rust
APIs, receipts and GC, persistent nonce replay claims, provider/manual evidence
ledgers, terminal tombstones, writer-generation fencing, bounded CAS GC,
row-digest integrity verification and crash-window regressions. Heavy inherited
and P1.2 Rust gates use `ubuntu-24.04`; only the lightweight source job remains
on `ubuntu-slim`.

This status is deliberately not `qualified` until the exact final branch head
has a committed lockfile and non-empty hosted runner steps passing source,
rustfmt, default-off, the complete restart/failpoint/GC/corruption matrix,
all-target check and strict Clippy.

All listener, provider, OpenBao, product workspace, production, effect,
operator-acceptance, promotion, G5 and execute authority remains false.
