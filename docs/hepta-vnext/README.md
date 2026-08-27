# Hepta vNext repository-native development authority

This directory is the repository-native entry point for current Hepta development.
It exists to remove ambiguity between code, qualification evidence, release authority,
and the historical Dropbox transport snapshot.

## Authority order

For changes created after 2026-08-27, consumers MUST apply this order:

1. `HEPTA_VNEXT_CURRENT.yaml` — current machine-readable phase and negative authority.
2. `HEPTA_BROWSER_ACTIVE_PLAN.md` — the active browser successor and document precedence.
3. `HEPTA_BROWSER_IMPLEMENTATION_PLAN_2026-08-27.md` — the executable browser plan.
4. `HEPTA_BROWSER_STAGE_MATRIX_v1_4.yaml` — stage entry/exit gates.
5. `HEPTA_BROWSER_TRACEABILITY_v1.yaml` — requirement-to-code/test/CI mapping.
6. `HEPTA_BROWSER_THREAT_MODEL.md` — trust boundaries and required negative tests.
7. `hepta.browser_receipt.v2.schema.json` — machine-readable receipt contract.
8. Git source and CI evidence for the exact implementation commit.
9. `dropbox-current-2026-08-27/` — historical/provenance snapshot only.

The Dropbox snapshot is byte-preserving evidence of what was visible at capture time. It is
not runtime authority, release authority, or a substitute for an exact Git commit and CI result.

## Current phase

The current phase is `DEVELOPMENT`. The browser lane is qualification-only:

- production caller: false
- production writer: false
- effect authority: false
- external effect: false
- operator acceptance: false
- promotion: false
- G5 allowed: false
- execute allowed: false

No plan document, test fixture, chat instruction, branch creation, pull request, or merge changes
those fields. A later release candidate needs an exact source/tree binding, artifact digest,
platform qualification, operator acceptance, and promotion receipt.

## Active browser route

The only active browser route is Servo-only and keeps the stage identifiers `WEB-C0` through
`WEB-C7`. One browser session has exactly one process, one `BrowserActor`, one Servo runtime,
one `WebView`, one rendering context, and one page owner. Human UI and Agent semantic operations
must reach the same page state through the same ordered actor.

The current implementation tranche deliberately separates portable work from engine work:

- `WEB-C0`: repository-native contracts, authority, threat model, stage matrix, traceability.
- `WEB-C1`: pinned/forked headed Servo integration. Not implemented in this tranche.
- `WEB-C2`: qualification-only semantic contract and deterministic fixture adapter.
- `WEB-C3`: qualification-only single-owner actor, epoch/revision fences, human lease,
  idempotency, receipts, and shared-page vertical-slice tests.

`WEB-C2` and the fixture portion of `WEB-C3` may be implemented before Servo is linked, but they
cannot claim the integrated `WEB-C3` exit gate until the same actor controls one real Servo
`WebView` and headed human input reaches that exact page.

## Code location

The initial portable browser contract is incubated in:

```text
codex-rs/hepta-shadow-qualification/src/browser_contracts.rs
codex-rs/hepta-shadow-qualification/src/browser_runtime/
codex-rs/hepta-shadow-qualification/src/browser_tests.rs
```

This location is intentional. It makes the first vertical slice executable without adding Servo
or a production listener to the product dependency graph. Once C1 is qualified, stable portable
types should move to a dedicated `hepta-browser-contracts` crate and the real process should live
in `hepta-browserd`.

## Verification

The repository verifier is dependency-free:

```sh
python3 scripts/verify-hepta-browser-plan.py
```

The focused Rust checks are:

```sh
cd codex-rs
cargo test --locked -p codex-hepta-shadow-qualification --lib browser_ -- --nocapture
cargo check --locked -p codex-hepta-shadow-qualification --lib
cargo clippy --locked -p codex-hepta-shadow-qualification --lib --tests -- -D warnings
cargo fmt --all -- --check
```

Passing these checks proves only the portable qualification fixture. It does not prove a Servo
build, native window behavior, external website compatibility, authentication, downloads,
network sandboxing, or production effects.
