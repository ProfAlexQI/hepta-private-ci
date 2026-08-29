# Hepta architecture convergence and full gap-closure plan V3

**Date:** 2026-08-29  
**Status:** normative source-execution plan; not runtime, operator, promotion or release authority.  
**Repository:** `ProfHepta/hepta-private-ci`  
**Stack base:** Draft PR #71, exact source head at tranche creation
`7baed12589c7af3572622b0ab076517d525e3e58`.  
**Implementation branch:** `codex/hepta-gap-closure-p0-7a-20260829`.

## 0. Decision summary

V3 replaces plan V2 as the active implementation order. It retains the V2
architecture model and deterministic projections, closes ambiguity in document
authority, and converts every remaining product gap into a package with an
owner, dependency graph, negative authority boundary, executable exit criteria
and evidence class.

The delivery graph is strictly ordered:

```text
P0.5 architecture authority and profile closure       source present
P0.6 runtime context / cross-owner operation closure  source present
P0.7a signed runtime-grant bootstrap                   current package
P0.7b physical capability boundaries                  next
P0.7c Memory bounded-context extraction               after P0.7b contracts
P0.7d common durable fault matrix                      parallel after stable APIs
P0.8a AST authority caller ratchet                     after constructor freeze
P0.8b fleet runtime-instance projection               after bootstrap/instance freeze
P0.8c resource-budget enforcement                     after service boundaries freeze
P0.8d exact real-process vertical slice                after P0.7/P0.8 source closure
P0.9 repository/operator/promotion/release             external independent gates
```

A later package cannot reinterpret a blocked predecessor as pass. Source
presence never equals executable qualification. Qualification never equals
operator acceptance, promotion or release.

## 1. Product architecture invariants

### 1.1 One execution spine

Codex App Server remains the only session, thread, turn, model-response and tool
execution spine. Hepta supplies typed Agent-local control, automation, memory,
workflow, evidence and lifecycle adapters. No second generic agent kernel or
fleet-wide message bus may be introduced.

### 1.2 One owner per durable fact

| Durable fact | Sole owner | Non-owner interaction |
|---|---|---|
| session/thread/turn | Codex state runtime | typed read/projection or command |
| Agent lifecycle/release generation | Fleet/Supervisor registry | fenced control request |
| Automation task/run | Automation store | command, event and outbox |
| TaskFlow run/step | owning Agent TaskFlow store | command, event and outbox |
| Memory/KG head | CognitiveStore writer | candidate, admission and query |
| Matrix ingress/delivery | Matrix store | operation journal and App Server reconcile |
| provider effect operation | provider operation owner | intent, one dispatch, lookup-only reconcile |
| governance/provider evidence | Evidence store | append-only projection |
| model/policy artifact | signed artifact registry | digest-bound load only |

A process must not directly mutate two owners in one logical operation. Cross-
owner work uses a local transaction, durable intent/outbox, destination dedupe,
acknowledgement and lookup-only reconciliation.

### 1.3 Authority is typed, bounded and current

- Runtime profiles are closed-world and may not contain unlisted actions.
- Start-time bootstrap binds the exact Agent, release, source, binary, profile,
  graph, grant, epochs, generation and fence.
- Every physical boundary independently verifies current authority immediately
  before crossing.
- No component may mint the capability it consumes.
- `Queued` and `DispatchAccepted` are never terminal success.
- `Indeterminate` remains open until a current-fence reconciler commits a legal
  terminal transition.

### 1.4 Qualification is read-only

Qualification workflows may checkout, build, test, inspect and upload evidence.
They may not commit, push, update refs, edit candidate source, apply repository
settings, issue signing keys, accept an operator gate, promote or release.

## 2. Document authority and change protocol

The active pointer is `HEPTA_CURRENT_PLAN.json`. Resolution order is defined in
`HEPTA_DOCUMENT_AUTHORITY_INDEX_V2.json`.

1. Change the normative architecture model before changing generated
   projections.
2. Change this plan and V3 gap ledger together when package ordering or closure
   criteria change.
3. Never hand-edit generated `ARCHITECTURE.md`, `DATA_AUTHORITY_MAP.md` or the V1
   compatibility projection.
4. Treat dated Dropbox snapshots, old receipts, Draft PR bodies and empty CI
   jobs as non-authoritative historical inputs.
5. A plan change carries only document authority. It does not change runtime
   flags or production callers.

Required source checks:

```shell
python3 scripts/generate-hepta-architecture-projections.py --check
python3 scripts/verify-hepta-p0-5-gap-closure.py
python3 scripts/verify-hepta-cross-owner-operation-wiring.py
python3 scripts/verify-hepta-p0-6-runtime-authority.py
python3 scripts/verify-hepta-architecture-plan-v3.py
```

## 3. P0.7a — Supervisor-signed runtime-grant bootstrap

### Goal

Replace ambient or in-process inference of externally governed runtime identity
with one strict signed envelope consumed before Agentd product composition.
Local closed profiles remain locally constructible for development, but they
cannot be interpreted as production authority.

### Source deliverables

- `hepta.runtime-authority-bootstrap.v1` canonical envelope and digest framing;
- detached signature metadata and injected signature-verifier trait;
- exact expected runtime binding containing Agent, release, source commit/tree,
  executable, profile, grant, ProductGraph, epochs, generation and fence;
- Supervisor issuer using an externally supplied signing key/trust ceremony;
- owner-only transport implementation;
- Agentd consumer before any service is opened;
- durable nonce reservation and compare-and-claim;
- tests for every mismatch, expiry, malformed input, transport attack and replay;
- required CI package that runs exact-source and merge-candidate tests.

### Exit criteria

`source_implemented` requires all source deliverables and static call-path
verification. `qualified_exact` additionally requires real, non-empty successful
runner steps for both exact source head and PR merge candidate. A test-only
fixture or a signature generated inside Agentd is rejected.

### Negative authority

P0.7a must leave model invocation, provider dispatch, external effect, fleet
mutation, operator acceptance, promotion and release false.

## 4. P0.7b — all physical boundaries checked per use

### Goal

Make the typed authority graph coincide with actual irreversible exits.

### Packages

- `model_boundary`: request is fully formed, model artifact/policy digests are
  known, then a one-operation verified-use token is consumed.
- `provider_boundary`: provider and effect capabilities are separate; effectful
  dispatch requires both when applicable.
- `tool_boundary`: process spawn, sandbox, cwd, environment and approval are
  bound to the operation.
- `network_boundary`: destination, resolution result, protocol and policy are
  bound before connect.
- `filesystem_boundary`: non-Agent-root mutation uses no-follow canonical target
  and mount identity.
- `secret_boundary`: only opaque SecretRef operations; raw secret bytes never
  cross the public contract.
- `fleet_boundary`: lifecycle mutation verifies registry revision and signed
  grant at the final CAS.

### Exit criteria

The matrix in `HEPTA_PHYSICAL_CAPABILITY_BOUNDARY_MATRIX_V1.md` is implemented
row by row. Compile-fail tests prove local profiles cannot construct dangerous
capabilities. Revocation between admission and physical crossing must reject the
crossing without corrupting the operation journal.

## 5. P0.7c — physical Memory bounded-context extraction

### Goal

Remove the remaining memory mega-crate coupling while preserving schema and
facade compatibility.

### Target crates

```text
hepta-cognitive-types          pure values and stable wire-neutral contracts
hepta-cognitive-store          SQLite owner and migrations
hepta-memory-retrieval         bounded query/ranking/explanation
hepta-memory-federation        capability-bound cross-Agent reads
hepta-compact-engine           checkpoint, loss and rehydration
hepta-trajectory-store         append-only learning observations
hepta-learning-shadow          H6/H7 proposal/evaluation only
hepta-production-writer        externally authorized write/outbox boundary
hepta-memory-runtime           compatibility facade and composition
```

### Dependency rules

- Store owns migrations and depends only on types/contracts/path abstractions.
- Retrieval has no write or migration authority.
- Federation reads through a capability-bound port.
- Shadow learning cannot be a dependency of production writer.
- Production writer cannot link test signer or qualification-only model runtime.
- App Server depends on a generic memory port, not the concrete Hepta store.
- One compatibility facade is retained during migration, then reduced.

### Migration sequence

1. Freeze public API and schema inventory.
2. Extract pure types with no behavior change.
3. Extract store/migrations and run old/new open/reopen fixtures.
4. Extract retrieval and federation.
5. Extract compact and trajectory owners.
6. Move shadow learning behind proposal interfaces.
7. Move production writer last, retaining exact authority checks.
8. Remove legacy reexports only after downstream callers migrate.

### Exit criteria

Cargo metadata contains the intended acyclic graph; compatibility, migration,
backup/restore, fault and API tests pass; no duplicate migration owner exists.

## 6. P0.7d — common durable fault matrix

Every durable owner implements all applicable rows in
`HEPTA_COMMON_DURABLE_FAULT_MATRIX_V1.md`. Test-only in-memory models may support
development but cannot substitute for rows requiring a real filesystem,
SQLite connection, WAL or process kill.

The first package adds reusable fault harness primitives without giving them
production authority. Each owner then contributes physical tests using its own
real transaction boundary. A single aggregate receipt reports missing rows as
`not_run`, never as skipped pass.

## 7. P0.8a — AST/compiler authority caller ratchet

### Goal

Replace string-search caller checking with Rust-aware symbol resolution.

### Required proof

- discover constructors, authorization functions and reexports by resolved item;
- reject aliases, wrapper functions, method references and macro-generated
  unauthorized calls;
- allow only the checked-in owner/call-purpose inventory;
- run compile-fail fixtures outside the allowed module set;
- bind toolchain and analyzer digest in the result;
- keep the source-string verifier temporarily as defense in depth, not authority.

## 8. P0.8b — fleet-queryable runtime-instance projection

Agentd already maintains a generation-bound in-process instance graph. This
package adds a read-only projection containing desired, observed and readiness
state, exact release/candidate identity, profile/graph digests and timestamps.
Supervisor aggregates signed observations without treating them as lifecycle or
authority commands.

The projection endpoint is bounded, owner-only and read-only. Missing or stale
Agent observations produce `unknown/stale`, not ready. An observer cannot mutate
Agent state by writing a projection file or replaying a snapshot.

## 9. P0.8c — runtime resource-budget enforcement

Implement `HEPTA_RESOURCE_BUDGETS_V1.md` at each admission point. Reservations
are durable when accepting durable work and are released exactly once by legal
terminal transitions. Crash recovery reconstructs counts from owner state.

Required tests include N+1 admission, concurrent cancellation/terminalization,
kill/reopen, memory pressure, disk hard limit, outbox saturation and budget
change forcing a new generation/fence.

## 10. P0.8d — exact real-process vertical slice

### Process topology

```text
hepta-supervisord
  └─ signed bootstrap → codex-hepta-agentd
       ├─ Codex App Server
       ├─ Automation owner
       ├─ Memory runtime
       └─ one ingress adapter
             └─ one governed physical model/provider boundary
```

### Campaign

1. Materialize an immutable candidate and release manifest.
2. Start Supervisor and reserve a new generation.
3. Issue and consume the signed bootstrap.
4. Reach required runtime-instance readiness.
5. Admit one ingress message and one Automation wakeup.
6. Read/write Memory only through declared authority.
7. Cross one approved physical boundary with current per-use verification.
8. Kill at each durable fault point.
9. Restart under a new generation and reconcile lookup-only.
10. Prove one terminal effect, no stale-owner write and deterministic state.

The campaign publishes exact binary, source, tree, profile, graph, grant,
schema, runner and raw-log digests. Physical external resources are explicit;
when unavailable the result is `blocked_external` or `not_run`.

## 11. P0.9 — repository, operator, promotion and release gates

These are independent external gates and cannot be closed by a source commit:

- live GitHub default-branch ruleset equals the versioned contract;
- exact source-head and merge-candidate checks have real runners and non-empty
  successful steps;
- an independent current-head code owner approves;
- operator acceptance signs the exact candidate and evidence manifest;
- promotion signs the accepted candidate without changing it;
- release publishes provenance, SBOM, immutable inventory and rollback evidence.

Any source change after acceptance invalidates downstream signatures.

## 12. CI and evidence tiers

| Tier | Evidence | Permitted claim |
|---|---|---|
| S0 | file/static verifier only | source shape present |
| S1 | unit/property tests | local semantics on tested implementation |
| S2 | real SQLite/filesystem/process fault test | physical local durability semantics |
| S3 | exact-source multi-process candidate | candidate system behavior |
| S4 | PR merge-candidate rerun | merge candidate behavior |
| S5 | independent operator acceptance | accepted candidate |
| S6 | signed promotion/release | promoted/released candidate |

Higher claims may not be inferred from lower evidence. A queued run, runner ID
zero, `steps=[]`, cancelled job or artifact without exact identity is `not_run`.

## 13. Completion semantics and stop conditions

“All source gaps closed” means every P0.7/P0.8 package is
`source_implemented` and all source verifiers pass. “All gaps closed” additionally
requires every P0.9 external gate to be independently issued. The gap ledger
must keep these states separate.

Stop and report rather than fabricate progress when:

- base branch/head/tree drifts from the package binding;
- required upstream API or schema is absent;
- a real runner never starts or returns empty execution evidence;
- a physical device/provider/operator/reviewer is unavailable;
- repository administration cannot be mutated by the available tool;
- signing or production authority has not been independently supplied;
- a failure would require weakening fail-closed semantics.

A valid stop is a precise `blocked_external`, `not_run`, `base_drift` or
`rejected` record with reproduction steps. It is not a pass.
