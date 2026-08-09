# Frozen-oracle qualification/conformance generator

This fixture is a bounded, offline conformance corpus. It is not a live product
Shadow collector, does not observe product traffic or elapsed soak duration,
does not qualify a candidate, and grants no promotion or Enforce authority.

The generator runs from an isolated `git archive` of frozen governance commit
`2f704dc7c1172cefca908852456beccf4d02a5d1` (tree
`7be9a382b2610790838eef874cb4d381b5025490`). For every case it invokes the
real `HeptaGovernanceExtension` `ToolPolicyContributor` extension-callback
semantic surface:

1. admission;
2. optional authorization;
3. terminal callback;
4. durable read from `HeptaEvidenceStore`.

The harness calls the contributor directly. It bypasses `ToolRegistry`, host
attempt-state transitions, and the real dispatch path. Product reachability is
therefore not executed and is not part of this
`extension_callback_semantic_conformance` gate.

It does not manually construct `ToolAction`, `HandlerOutcome`, decision
records, or `GovernanceReceipt`. The 252 cases are the complete Cartesian
product of 3 payloads, 2 receipt phases, 3 sources, 7 terminal outcomes, and 2
`host_accepted` values. The corpus explicitly preserves both Aborted phase
semantics. Product-invalid host/outcome combinations remain visible as durable
pending decisions with no fabricated receipt.

The executing generator reads and hashes its own copied `src/tests.rs` bytes,
compares that digest with the runner-provided tracked digest, and embeds the
computed value in the corpus. Reproduce without editing the frozen worktree:

```sh
git archive 2f704dc7c1172cefca908852456beccf4d02a5d1 | tar -x -C <archive>
cp frozen_oracle_conformance_2f704_generator.rs.txt \
  <archive>/codex-rs/ext/hepta-governance/src/tests.rs
HEPTA_FROZEN_ORACLE_GENERATOR_SOURCE_SHA256=<tracked-generator-sha256> \
HEPTA_FROZEN_ORACLE_CONFORMANCE_OUTPUT=<output> \
CARGO_TARGET_DIR=<isolated-target> \
cargo test --manifest-path <archive>/codex-rs/Cargo.toml --locked \
  -p codex-hepta-governance \
  tests::emit_frozen_oracle_conformance_corpus_v2 -- --exact --nocapture
```

The final tracked generator and corpus SHA-256 values are recorded in the
corresponding frozen-oracle audit bundle. Every in-process result remains an
`identity_claim` with `exact_verified=false`, including a clean archive run.
Only an independent external verifier may upgrade that claim after rehashing
the full source manifest, rebuilding in isolation, and binding the resulting
binary, driver, and exact command.
