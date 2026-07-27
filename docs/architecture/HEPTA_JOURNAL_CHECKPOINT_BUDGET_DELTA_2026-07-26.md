# Journal Checkpoint Architecture Budget Delta

- Scope: C-stage checkpoint, compaction, and rotation support for operator mutation, Telegram authority, and external monotonic anchor journals.
- Reason: the existing fixed 4,096-record ceilings can stop the service after valid sustained operation; terminal history must compact without weakening exact replay denial, in-doubt recovery, or monotonic rollback detection.
- Reserved delta: 1,200 Rust lines for the three journal domains and their focused tests.
- Whole-tree ceiling: 1,111,100 lines, a cumulative bounded 2,500-line increase over the R11 A-stage baseline.
- Repayment: E-stage semantic compaction must return the Hepta-owned Rust whole-tree ceiling to 1,108,600 lines or lower before controlled-live cutover.
- Repayment status: completed by `c4a78cd27e`; the E-stage measured total is 1,107,900 lines and the enforced ceiling is restored to 1,108,600.
- Prohibition: only journal checkpoint/compaction/rotation code and tests may consume this reservation; unresolved and in-doubt records may not be compacted.
