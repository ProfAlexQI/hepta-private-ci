# Hepta UI v4 Tranche 28 — Executable Final Gap Closure v3

This tranche replaces runner-presence-only evidence with one exact-candidate executable matrix.

## Closed repository blockers

- The generic UI workflow checks out the pull-request head rather than GitHub's temporary merge ref.
- The UI toolchain is Rust 1.95.0, which satisfies the current SQLx 0.9 MSRV.
- The same-run runner receipt is emitted only after the Rust-served bundle, Native fail-closed contracts, converged platform harnesses, Windows safety harness, and iOS/Android target surfaces execute.
- Platform device receipts require named semantics rather than a single arbitrary `verified=true` flag.
- Physical human browser acceptance covers Chrome, Edge, Firefox, and Safari.

## Exact executable set

1. source contract;
2. Rust-served bundle and Native contracts on Ubuntu;
3. converged platform and Windows safety contracts on Ubuntu;
4. the same contracts on Windows;
5. the same contracts on macOS;
6. iOS target compilation;
7. Android target compilation.

All jobs bind the same commit/tree, use nonzero assigned runners, contain nonempty successful steps, and use Rust 1.95.0.

## Permanent boundary

The resulting `runnerExecution` receipt is only one of eight final evidence slots. Physical platform receipts, candidate-bound human acceptance, cross-browser runtime, and physical browser human acceptance remain non-synthesizable external evidence. Even `PASS_ALL_UI_GAPS_CLOSED` is an audit conclusion and does not enable product wiring, system material, effect, production, promotion, or release.
