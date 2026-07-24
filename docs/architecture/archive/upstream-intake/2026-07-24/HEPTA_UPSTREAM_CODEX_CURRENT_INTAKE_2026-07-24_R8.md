# Hepta Upstream Codex Current Intake R8

## Observation

- The latest recorded upstream observation advanced from frozen R7
  `f201c30c52a3` to `c8957bbf0f79`.
- The R8 delta contains 1 non-merge commit, 6 net changed paths, 81 insertions,
  63 deletions, and 6 commit-level file touches.
- R7 is preserved byte-for-byte. R8 is a new immutable observation and claims
  zero imports at observation time.
- This is an offline exact-SHA intake. Network freshness requires a separately
  generated receipt and is not inferred from this document.
- Hepta and upstream still have unrelated roots and no merge base. Ordinary
  merge/rebase remains forbidden; integration requires selective semantic
  transplantation or bounded semantic-equivalence evidence.

## Classification

- `c8957bbf0f79`: candidate P1. Upstream moves MCP invalidation, serialized
  refresh ownership, and cancellation recovery into one `McpRefresh`
  abstraction.
- Hepta already has a richer desired/applied generation state, a serialized
  refresh lock, cancellation-safe pending intent, and manager-generation
  publication. The correct intake question is semantic equivalence, not
  copying the upstream `AtomicBool` implementation over the local model.

## Ordered Intake

1. Preserve the R7 generation-bound coordinator and prove that one session gate
   serializes publication.
2. Prove cancellation retains the exact pending intent and that only the latest
   valid generation publishes.
3. Keep R6 authentication-generation and background prewarm prerequisites
   separate; this structural R8 delta does not close either one.

## Frozen Evidence

- Predecessor manifest:
  `docs/architecture/HEPTA_UPSTREAM_CODEX_CURRENT_INTAKE_2026-07-24_R7.json`
  (`63d15fc05f42605dceeb29899cb357f664ec99e691d24b8d296a0033c359feee`).
- Frozen R7 ref:
  `refs/remotes/upstream/hepta-intake-20260724-r7` at
  `f201c30c52a35f819262865a53df94b6f4ea7a50`.
- Required R8 frozen ref:
  `refs/remotes/upstream/hepta-intake-20260724-r8` at
  `c8957bbf0f79fa29c5e08b8c0b942c12ea3893f2`.
- Range digest:
  `aa5aeda987db109b75fdcb1fb130da8a9996cdfdab87bf897eaa2215b0a8c1f5`.
- Net path-surface digest:
  `3fdd6e10977611d16a25338b9270615f187a5ef666040dbed2a33147262904e4`.
- Commit identity digest:
  `dd8a95f921dd891b5750022edf1fedb44b5fa0e9957f06cd29d78fc786acff6a`.
- Related-path inventory digest:
  `dac025271875834e3bc7d6fd778054a13105961411324665177b22860ffacf03`.

## Non-Claims

- Candidate does not mean imported, integrated, enabled, or production-ready.
- R8 encapsulation does not close R6 authentication refresh or prewarm.
- No merge, rebase, cherry-pick, deployment, restart, publication, or live
  enablement occurred during observation.
