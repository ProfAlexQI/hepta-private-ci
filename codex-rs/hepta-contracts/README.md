# Hepta vNext Contract and Status

Hepta vNext uses upstream Codex as its only execution spine. The old Hepta
implementation and the raw-U run remain read-only oracle/evidence sources, not
merge targets or authority roots.

## Frozen baselines

- Upstream spine: `5d89ab65dc9d4d0c55796c11df112b54157922b4`.
- vNext committed checkpoint: `07e7dc1974351baab9a399220ba2f3365a8dc21e`.
- S1-S6 implementation head before this status-only documentation slice:
  `1aeeb0e9a58141aaddf632d6c8342db93d2844be`.
- Old Hepta oracle: `464c87520acc`, plus its separately tracked report
  canary.
- The review series is not a promotion, retirement receipt, or claim that old
  Hepta compatibility/provenance has closed.

## Non-negotiable invariants

1. Codex owns sessions, turns, routing, tools, state, rollouts, and thread
   storage. Hepta extends those seams; it does not build a second kernel.
2. Product behavior is explicit. Ordinary `codex` keeps upstream behavior
   unless trusted configuration enables a Hepta extension.
3. Raw payloads, prompts, model output, tool arguments, and Memory content do
   not enter Hepta evidence rows. Evidence uses typed IDs, bounded reason codes,
   and domain-separated digests.
4. A digest is evidence identity, not authority. Executable authority must be
   opaque, host-owned, non-serializable, exact-bound, short-lived where
   applicable, and consumed at the real side-effect boundary.
5. Enforce paths freeze indeterminate outcomes until explicit reconciliation.
   Governance/provider Shadow replay remains observational and may run, but it
   cannot borrow the original durable claim. Handler completion never proves
   an external effect.
6. Canonical digest fixtures, restart/concurrency/corruption tests, immutable
   triggers, and generated protocol projections are ratchets, not scaffolding.
7. Retirement requires caller-zero telemetry, oracle closure, soak, exact
   provenance, governance/operator acceptance, and a non-rollbackable registry.
   Classification or local lineage observations cannot retire code.

## S1-S6 exact-SHA review series

The preserved M1/M2 base and reconstructed dirty increments form these ordered,
independently reviewable commits:

| Slice | Exact commits | Proven boundary |
| --- | --- | --- |
| M1/M2 base | `b3f49f829f`, `07e7dc1974` | Product shell plus governance/provider spine |
| S1 Memory substrate | `b5e2790428`, `06a90f2cd8` | Typed deterministic recall plus extension seam |
| S2 read composition | `d1d2e57218`, `7cb3dae594`, `60ed080f45`, `c30c6e97a4`, `8ec30c9491` | Host composition, provider-send gate, admitted-turn and exact-parent binding, same-thread activation |
| S3 mutation shadow | `3bd8f4a8ec`, `1e853b5bbd`, `cb3d13531c`, `1190ff40fb` | Proposal/evidence contracts, pure simulation, append-only observation |
| S4 channel ingress | `e90bab8d07`, `32558de97c`, `561935a28d`, `dac73db5c8`, `d66dce665d`, `925f8b2d02` | Digest-only contract/storage, no-turn preflight, bounded local capability/facade/adapter |
| S5 evidence reads | `e13aae77db`, `194490a9f3`, `84a690988f`, `95bf17185d`, `4a3803d8af`, `434e9fb5c3` | Atomic summary, exact history, protocol/API projections, caller-supplied migration snapshot metadata |
| S6 local proof | `947d99061d`, `54967abe2d`, `1aeeb0e9a5` | Bounded local proof observations, exact-history binding, dual-store reread lineage |

This map proves review reconstruction only. It does not prove product completion,
old-vNext equivalence, hosted reproducibility, or retirement readiness.

## Reachability matrix

| Area | Product/default | Explicit feature or local seam | Contract/test-only | Blocked |
| --- | --- | --- | --- | --- |
| M1/M2 governance/provider | `hepta` enables governance; App Server/MCP use the shared Codex lifecycle; durable decisions and provider intents/terminals are live | Enforce mode remains explicit | Replay, corruption, cancellation, HTTP/WS/compact/prewarm adversarial oracles | Final promotion receipts |
| S1/S2 Memory read | `hepta` enables digest-only same-thread shadow recall | Model-visible attachment additionally requires governance + Memory + `hepta_memory_read_only`; it is bounded, quoted, witnessed, and same-thread | Cross-thread witness shape | Cross-thread recall and a complete Memory/Intelligence/KG pipeline |
| S3 Memory mutation | None | None | Create/supersede/tombstone proposals, pure current-state dry-run, append-only no-write observations | Producer, writer, host minter, commit-time CAS, live authorization, reconciliation |
| S4 channels | None | Hidden bounded in-process preflight/facade; caller-zero outside the channel crates | Canonical ingress storage and Native loopback observation adapter | Turn admission, Accepted receipt, default install, Telegram/Matrix, outbound, reconciliation, external transport |
| S5 evidence reads | App Server read-only summary/history for governance, provider, and channel ingress | Migration snapshot metadata is supplied by the caller | Transaction, digest, substitution, and schema-drift oracles | Mutation/delivery/reconciliation families, old-Hepta compatibility and provenance equivalence |
| S6 proof/provenance | None | Bounded local proof store and dual-store lineage builder are caller-zero; command execution is Unix-only and returns `NotStarted` elsewhere | Fixed command/receipt/lineage and failure oracles | WorkGraph/gate/Nix/hosted/raw-U composition, attestation, authority, anti-rollback root |
| Retirement | None | None | None in this clean series | Trusted minter, irreversible registry, soak, operator acceptance, route deletion, old implementation retirement |

“Product” means a real reachable path, not that the milestone is complete.
“Explicit feature or local seam” is not default behavior or authority. A
caller-zero seam must remain described as caller-zero until composition lands.

## Current authority boundaries

### Governance and provider

- Governance decisions and terminal receipts are distinct typed records.
- Provider intent is persisted before transport in Enforce mode. HTTP,
  WebSocket, compact, and prewarm sends revalidate the same logical binding.
- In Enforce mode, pending or indeterminate attempts block replay of the same
  identity. Shadow replay remains observational and cannot finish the original
  claim.
- Shadow observations and digest bindings cannot mint execution authority.

### Memory

- Product default is digest-only same-thread shadow recall. It does not attach
  Memory content to a model-visible prompt.
- Read-only attachment requires all three host features, exact installation,
  thread, workspace, source revision, budget, and provider-attempt bindings.
  The quoted reference is bounded and does not enter local conversation or
  rollout history.
- Mutation is deliberately limited to pure simulation and durable shadow
  observation. No record authorizes a write, and the snapshot digest is not a
  write-transaction CAS.
- `codex-hepta-contracts` owns mutation DTOs. `codex-hepta-memory` owns only the
  evaluator; Evidence does not depend on the evaluator in its normal graph.

### Channel ingress

- Storage validates canonical event/receipt payloads, projection columns,
  append-only schema, per-scope cursor chains, replay, and corruption before
  using rows in control flow.
- The Core preflight and in-process facade share one bounded FIFO. They do not
  reserve or start a turn and cannot emit an Accepted receipt.
- The Native adapter accepts only the concrete local facade. There is no
  generic UserInput fallback, Remote/WebSocket field, external transport, or
  default product installation.

### Historical evidence

- Summary and history reads cover only families with store-authoritative rows
  in an intact local root: governance, provider, and channel ingress.
- Supported historical records are read in one SQLite transaction and bind
  schema, family, exact ID, state, and evidence digest.
- App Server projects store-produced digests. It does not recompute authority.
- A `MigrationFamilySnapshot` binds family, disposition, and caller-supplied
  old-implementation, vNext-implementation, and candidate digests. It is not a
  trusted migration observation or old-Hepta compatibility oracle.

### Local proof lineage

- Proof commands bind a declared executable path, a pre-execution regular-file
  digest, and bounded argument/environment/input material. Intents are appended
  before execution and terminal receipts are immutable, but path check/exec
  TOCTOU remains.
- `HistoricalStoreResolved` is a typed API-path origin. It is not a secret or
  authentication credential; the public generic constructor always resets to
  `CallerSupplied`.
- `LocalProofProvenanceLineage` rereads one exact positive historical record
  and one exact successful proof receipt from the two typed stores.
- The two reads are mixed-time and the local roots are independently
  replaceable or rollbackable. The result is diagnostic wire self-consistency,
  not build attestation, causal provenance, execution authority, hermeticity,
  anti-rollback, or retirement closure.

## Evidence-store guarantees and limits

- SQLite uses WAL, `synchronous=FULL`, foreign keys, immutable triggers,
  migration checksums, required-object/schema-fragment manifests,
  `quick_check`, and `foreign_key_check`.
- Exact replay is idempotent; identity substitution is a hard conflict.
- Transactional reads prevent torn projections within one evidence store.
- These checks detect many row/schema corruptions. They do not resist complete
  database deletion, directory replacement, or rollback to an older valid
  store. Rollback-resistant claims require an external durable root.

## Promotion and retirement gates

Review-series completion is only the start of candidate qualification. Before
promotion or old-code retirement, the exact clean candidate still needs:

1. Full workspace/local gates, all-features/all-targets Clippy, Cargo/Bazel
   locks, generated schemas, formatting, and diff checks.
2. Nix and hosted exact-SHA receipts plus Windows/non-Unix containment checks.
3. Old-vNext canonical oracle closure and explicit migration-family coverage.
4. Static caller inventory plus persistent caller-zero telemetry.
5. Shadow soak, governance/operator acceptance, signing, and promotion receipt.
6. A host-private minter and non-rollbackable retirement ledger/root before any
   executable retirement gate or route deletion.

Until those gates close, old Hepta and raw-U remain untouched. Caller-chosen
keys, deletable JSON/SQLite registries, digest-only permits, and rollbackable
local roots must not be represented as authority.
