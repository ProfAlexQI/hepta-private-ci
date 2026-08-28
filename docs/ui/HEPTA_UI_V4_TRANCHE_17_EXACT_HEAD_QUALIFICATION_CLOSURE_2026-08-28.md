# Hepta UI v4 Tranche 17 — exact-head qualification closure

## Scope

## Exact stack

- base PR: #36 `ci(ui): bind Windows material evidence to exact PR head`
- base branch: `codex/ui-v4-exact-candidate-materialization-20260828`
- base commit: `2a94681a66f5c7c62fe9e26570e6a61ed876d28b`
- base tree: `2c8b99a004eaf337b608448d5cc0f56d66aa9709`
- successor branch: `codex/ui-v4-exact-head-qualification-closure-20260828`


This tranche closes three gaps left after the first exact-candidate hardening:

1. the canonical workflow reruns the inherited Windows aggregate and runtime
   source gates for the same checked-out head;
2. all matrix commands use an explicit cross-platform shell and run the full
   focused evidence test set;
3. the governed Windows job validates the generated PASS receipt instead of
   treating a zero process exit as evidence.

The materialization receipt is bound to the checked-out Hepta commit/tree,
removes stale outputs before every run, records UTF-8-safe bounded failure
bytes, and is checked by a strict PASS/FAIL Schema.

## Evidence order

```text
exact PR head checkout
→ inherited aggregate/runtime source gates
→ patcher self-test and exact materialization gates
→ candidate-bound Makepad materialization
→ Ubuntu/Windows/macOS compile and focused tests
→ governed Windows Mica/Acrylic/None/Destroyed producer
→ validated aggregate PASS receipt
```

A validated aggregate remains fixture-only and does not bind the product
material host, complete the Windows profile, qualify a device, or grant any
network, mutation, effect, production, promotion, or release authority.
