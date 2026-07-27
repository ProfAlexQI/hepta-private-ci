# Execution Admission Architecture Budget Delta

- Scope: `codex-rs/hepta-runtime/src/execution_admission.rs`, the Native Gateway operator-mutation adapter, and the exact operator-authorized Telegram model adapter.
- Reason: B-stage authority closure adds the typed admission → effect plan → provider ACK → terminal receipt boundary; this is new safety behavior, not compatibility duplication.
- Module ceiling: 360 production-source lines.
- Whole-tree ceiling: 1,109,200 lines, a bounded 600-line increase over the R11 A-stage baseline.
- Tests live in `codex-rs/hepta-runtime/tests/execution_admission.rs` and remain mandatory even though integration tests are outside the production-source line count.
- Repayment: E-stage semantic compaction must return the Hepta-owned Rust whole-tree ceiling to 1,108,600 lines or lower before controlled-live cutover.
- Repayment status: completed by `c4a78cd27e`; the E-stage measured total is 1,107,900 lines and the enforced ceiling is restored to 1,108,600.
- Prohibition: the delta cannot be reused by unrelated modules or used to expand WorkGraph, report, fixture, or compatibility ladders.
