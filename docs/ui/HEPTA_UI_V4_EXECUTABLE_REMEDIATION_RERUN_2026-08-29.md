# Hepta UI v4 executable remediation rerun

This marker records the user-authorized rerun boundary after the first Rust 1.95 exact-head execution exposed and the branch fixed:

- repository-wide Native Rust formatting drift;
- Windows checkout failures caused by long paths;
- the isolated iOS target harness compiling target FFI under a crate-level unsafe prohibition;
- false-positive executable receipts written after failed compile steps.

The current source-closure harness uses a host-independent fail-closed preference shim, while the product preference module retains its platform-scoped FFI. The canonical `hepta-ui-v4-final-gap-closure-v4` workflow performs exact-head checkout, Rust 1.95 compilation, Windows long-path setup, Ubuntu/Windows/macOS contract tests, and iOS/Android target compilation before producing the same-run runner receipt.

This commit exists to trigger a user-authored exact-head push execution after bot-authored formatter commits caused pull-request runs to require approval. It grants no product, material, effect, production, promotion, or release authority.
