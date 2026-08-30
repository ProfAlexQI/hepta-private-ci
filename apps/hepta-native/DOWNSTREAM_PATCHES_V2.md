# Hepta Native downstream patch ledger v2

This is the canonical strict-path ledger selected by
`apps/hepta-native/UPSTREAM_ROBRIX.lock.json`. It classifies every current
path-level deviation from the committed Robrix lineage import without claiming
byte-for-byte upstream equivalence.

The rule set is intentionally minimal and mutually exclusive. It was derived
from the top-level Git tree delta between the committed lineage import
`a9cf73726c892cec5e3fae12792f8b98ba93ea58` and candidate
`322200c8c1acb66ff9d010b5ecca9e9b9a9b26fe`. A changed baseline subtree or an
added subtree receives one rule; changed or added top-level files receive exact
rules. Canonical provenance files selected by the lock are governance inputs,
not downstream drift.

DOWNSTREAM_PATCHES_V1_BEGIN
| Local path or glob | Class | Purpose | Verification |
| --- | --- | --- | --- |
| `.cargo/**` | build configuration | Bind Hepta package identity and target-specific Rust configuration. | Strict Robrix sync gate plus locked Cargo builds. |
| `Cargo.lock` | dependency lock | Preserve the reviewed Hepta Native dependency graph. | Locked fetch build test and dependency-policy gates. |
| `Cargo.toml` | dependency manifest | Declare Hepta Native features binaries patches and platform integrations. | Cargo metadata build and dependency-policy gates. |
| `README.md` | product documentation | Document Hepta Native operation packaging and non-claims. | Documentation and repository contract review. |
| `build.rs` | build integration | Generate and bind Hepta-specific build metadata and resources. | Locked native build and product-shell contract. |
| `canonical-assets-v1.tsv` | asset governance | Bind canonical Hepta product assets and retired Robrix assets. | Canonical asset and legacy visual archive gates. |
| `licenses/**` | compliance | Record Hepta additions to attribution and third-party notices. | Dependency security and attribution review. |
| `packaging/**` | packaging and release | Provide Hepta desktop mobile bundle metadata scripts fixtures and assets. | Packaging self-tests release verifiers and platform-specific build gates. |
| `resources/**` | platform resources | Replace product branding and bind Android Windows and shared resources. | Canonical asset Android manifest and platform resource gates. |
| `rust-toolchain.toml` | toolchain | Pin the reviewed Rust toolchain required by Hepta Native. | Exact toolchain and locked build gates. |
| `src/**` | product runtime | Implement Hepta product shell bridge accessibility material and runtime behavior. | Native unit tests product-shell gate and bounded platform qualification. |
| `deny.toml` | dependency policy | Define Hepta Native license source and advisory policy. | cargo-deny and native dependency-policy gates. |
| `dependency-source-policy-v1.json` | dependency policy | Bind allowed dependency origins and vendored-source receipts. | Native dependency-policy self-test and dependency security scan. |
| `hepta-live-bridge-backend-contract-v1.json` | bridge contract | Bind the live bridge backend interface and fail-closed authority boundary. | Live bridge contract gate and Rust bridge tests. |
| `hepta-native-dependency-policy` | policy tooling | Enforce the native dependency source and provenance contract. | Native dependency-policy self-test. |
| `hepta-native-dependency-policy-self-test` | policy tooling | Exercise positive and adversarial dependency-policy cases. | Direct self-test execution. |
| `mobile-readiness-policy-v1.json` | mobile contract | Bind mobile readiness prerequisites and evidence semantics. | Mobile readiness and emulator self-tests. |
| `promotion-trust-policy-v1.json` | promotion governance | Define trusted producer and verifier boundaries for promotion evidence. | Release device-lab accessibility matrix and bridge verifier self-tests. |
| `tests/**` | contract tests | Add repository-owned platform and packaging contract tests. | Direct test execution in UI and backend CI jobs. |
| `third_party/**` | vendored dependencies | Carry bounded vendored crates with provenance and license receipts. | Native dependency-policy and dependency security gates. |
DOWNSTREAM_PATCHES_V1_END

## Safety properties

- The strict checker must observe zero undeclared paths.
- No drift path may match more than one rule.
- Every rule must match at least one current drift path.
- The checker remains read-only and the lock continues to bind the source
  commit, manifest, lineage import, and this ledger.
- This ledger does not authorize release, deployment, live enablement, or
  promotion and does not replace device or human evidence.
