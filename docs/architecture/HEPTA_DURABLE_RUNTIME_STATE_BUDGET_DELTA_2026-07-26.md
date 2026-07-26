# Durable Runtime State Architecture Budget Delta

- Scope: authenticated atomic persistence for production session, memory, and transcript state, plus RuntimeKernel and Native Gateway composition wiring.
- Reason: C-stage durability closure replaces the production-only in-memory state boundary with fail-closed bootstrap/open, keyed integrity, atomic publication, path-identity checks, and startup hydration.
- Module ceilings: 440 production-source lines and 120 unit-test lines.
- Whole-tree ceiling: 1,109,900 lines, a cumulative bounded 1,300-line increase over the R11 A-stage baseline.
- Repayment: E-stage semantic compaction must still return the Hepta-owned Rust whole-tree ceiling to 1,108,600 lines or lower before controlled-live cutover.
- Prohibition: this delta cannot fund unrelated features, compatibility ladders, report expansion, or WorkGraph growth.
