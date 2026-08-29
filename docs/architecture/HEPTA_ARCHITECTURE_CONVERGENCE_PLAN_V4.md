# Hepta architecture convergence and full gap-closure plan V4

**Date:** 2026-08-30  
**Status:** normative source-execution candidate; not runtime, operator, promotion or release authority.  
**Repository:** `ProfHepta/hepta-private-ci`  
**Stack base:** Draft PR #77, exact source head `2f0b7bb517c6b2678b6d642e460f4f37ba3625c2`, tree `96cbdbf4e115f1a6e1be808b5b5d377bf42c39c7`.  
**Implementation branch:** `codex/hepta-architecture-v4-p0-7a-closure-20260830`.

## 0. Why V4 exists

V3 established the correct P0.7/P0.8/P0.9 delivery order, but several entries
were still chapter-level intentions rather than executable package contracts.
V4 keeps all V3 safety invariants and adds:

- exact package inputs, code ownership, touched boundaries and dependency order;
- explicit source-controlled versus externally issued closure states;
- a same-commit Cargo/Bazel lock invariant;
- a transport and crash-recovery closure package for the signed runtime bootstrap;
- a dedicated dependency-inversion and wire-isolation package that V3 omitted;
- row-level fault, resource and real-process evidence requirements;
- immutable rollback and supersession rules for every package.

Source presence is not executable qualification. Executable qualification is
not operator acceptance. Operator acceptance is not promotion or release.

## 1. Non-negotiable architecture invariants

1. Codex App Server is the sole session, thread, turn, model-response and tool
   execution spine.
2. Every durable fact has exactly one writer and one schema/migration owner.
3. Cross-owner work is a local transaction plus durable intent/outbox,
   destination dedupe, acknowledgement and lookup-only reconciliation.
4. Runtime bootstrap authenticates start identity only; it is not a reusable
   operation capability.
5. Every irreversible physical crossing verifies a current, one-operation,
   non-serializable capability after the final payload is known.
6. No component may mint the capability it consumes.
7. `Queued` and `DispatchAccepted` are never terminal success.
8. `Indeterminate` remains open until a current-fence reconciler commits a legal
   terminal transition.
9. Qualification code and artifacts have no production caller, writer, model,
   provider, tool, network, fleet, operator, promotion or release authority.
10. A dependency, schema or build-lock change is incomplete until all generated
    lock and projection files are updated in the same candidate commit.

## 2. Package operating contract

Every source package must declare and satisfy all of the following:

| Field | Required meaning |
|---|---|
| exact base | immutable parent commit and tree |
| owner | one accountable bounded context |
| prerequisites | exact predecessor package states |
| touched paths | closed allowlist; unrelated stacks are forbidden |
| authority delta | normally all false; any non-false value requires an external gate |
| state owner | durable writer and migration owner, when applicable |
| fault rows | applicable IDs from the common durable fault matrix |
| resource rows | admission/release/recovery bounds that change |
| source verifier | deterministic, fail-closed and candidate-clean |
| executable checks | fmt, locked metadata, tests, check and strict Clippy |
| exact-head evidence | runner assigned, non-empty steps and exact commit/tree |
| merge-candidate evidence | exact synthetic merge identity and non-empty steps |
| rollback | source-only revert or versioned data migration; never reinterpret state |

A package state may advance only in this order:

```text
open -> source_in_progress -> source_implemented -> source_verified
     -> qualified_exact -> merge_candidate_qualified
```

External states are independent:

```text
not_issued -> issued | rejected | revoked | superseded
```

## 3. Current delivery graph

```text
P0.7a signed runtime bootstrap closure
  -> P0.7b per-use physical capability boundaries
  -> P0.7c Memory bounded-context extraction
  -> P0.7d common durable fault matrix execution
  -> P0.7e Codex/Hepta dependency inversion and wire isolation
  -> P0.8a AST/compiler authority caller ratchet
  -> P0.8b fleet runtime-instance projection
  -> P0.8c runtime resource-budget enforcement
  -> P0.8d exact real-process vertical slice
  -> P0.9 repository/operator/promotion/release gates
```

P0.7c and P0.7d may run in parallel only after P0.7b shared contracts freeze.
P0.7e may inventory dependencies earlier, but product call-site migration waits
for the affected P0.7c facades to stabilize.

## 4. P0.7a — signed runtime bootstrap closure

### 4.1 Source already present on the base

The base contains the canonical envelope and runtime-profile binding, immutable
trust/provenance registry, Supervisor issuer, owner-only file handoff, Agentd
pre-composition verification, durable reservation/claim and a dedicated source
workflow. This is implementation source, not executable evidence.

### 4.2 Remaining source-controlled blockers

#### A-LOCK-01 — lock coherence

`codex-rs/hepta-contracts`, `hepta-fleet` and `hepta-agentd` changed dependency
sets without updating `codex-rs/Cargo.lock` and `MODULE.bazel.lock`. The candidate
must carry generated locks in the same commit. A workflow that merely uploads a
lock patch does not close this gap.

#### A-TRANSPORT-01 — physical file identity

The fallback handoff reader and registry must verify no-follow open semantics,
regular-file identity, owner UID where available, link count, mode, size and
metadata/content stability. Any mismatch fails before authority construction.

#### A-RECOVERY-01 — crash windows

Tests must cover:

- crash after reservation fsync and before document publication;
- crash after document publication and before process spawn;
- crash after claim publication and before cleanup/readiness;
- stale generation retry with retained claim;
- new generation issuance without reinterpreting old state.

A retained partial or claimed handoff is evidence/recovery-required, never a
retryable success.

#### A-CI-01 — exact execution

The package workflow must run locked metadata, format, tests, check, strict
Clippy, V4 source verification and candidate-clean checks. Runner ID zero,
`jobs=[]`, `steps=[]`, queued-only or cancelled evidence remains `not_run`.

### 4.3 Exit criteria

`source_implemented` requires A-LOCK-01, A-TRANSPORT-01 and A-RECOVERY-01 in the
candidate source. `source_verified` additionally requires deterministic static
verification on the checked-out candidate. `qualified_exact` and
`merge_candidate_qualified` require real non-empty runner execution.

P0.7a keeps every production and external authority flag false.

## 5. P0.7b — per-use physical capability boundaries

P0.7b is split into reviewable packages.

### B0 — shared verified-use kernel

Create pure contracts for capability kind, operation ID, payload digest,
authority context, revocation revision, expiry and one-use consumption. The
verified-use token must not implement `Clone`, `Serialize` or `Deserialize`.
Consumption is by value and returns an operation-bound witness.

### B1 — model and provider boundaries

- model submission binds model artifact, policy, request and provider identity;
- provider dispatch binds namespace, operation key and request digest;
- effectful dispatch requires a separate external-effect capability;
- revocation between admission and crossing rejects without deleting the intent;
- unknown results enter lookup-only reconciliation.

### B2 — tool, network and filesystem boundaries

- tool spawn binds command, cwd, environment, sandbox and approval;
- network connect binds canonical destination, protocol and resolved addresses;
- external filesystem mutation binds no-follow target and device/mount identity;
- the final adapter consumes the token immediately before the syscall.

### B3 — secret, Matrix and fleet boundaries

- secrets remain opaque `SecretRef` values;
- Matrix send binds room, event and durable operation journal;
- fleet mutation verifies signed grant and registry revision at the final CAS.

### B4 — negative construction and call-site proof

Compile-fail fixtures prove local profiles cannot construct dangerous
capabilities. Static inventory proves only named boundary adapters consume them.

### Exit criteria

Every row in `HEPTA_PHYSICAL_CAPABILITY_BOUNDARY_MATRIX_V1.md` has a concrete
owner, constructor, verifier, consumer, revocation test and unknown-outcome
rule. Missing physical resources are `blocked_external`, not pass.

## 6. P0.7c — Memory bounded-context extraction

### Target crates

```text
codex-hepta-cognitive-types
codex-hepta-cognitive-store
codex-hepta-memory-retrieval
codex-hepta-memory-federation
codex-hepta-compact-engine
codex-hepta-trajectory-store
codex-hepta-learning-shadow
codex-hepta-production-writer
codex-hepta-memory-runtime
```

### Sequence

1. Freeze and machine-inventory every public symbol, migration, table and caller.
2. Extract pure values with no SQL, file, runtime or authority dependency.
3. Move all cognitive migrations to one store crate and prove old/new reopen.
4. Extract KG/retrieval and federation read ports; neither can write.
5. Extract compact and trajectory owners with exact replay fixtures.
6. Move H6/H7 proposal/evaluation into shadow-only crates.
7. Move production writer last; it must not depend on shadow learning or test signers.
8. Reduce the compatibility facade only after all downstream callers migrate.

### Required evidence

- acyclic Cargo graph and forbidden-edge verifier;
- schema byte equality and migration owner uniqueness;
- backup/restore, corruption, disk-full and kill/reopen tests;
- public API compatibility report with explicit approved removals;
- no duplicate database opener or migration runner.

## 7. P0.7d — common durable fault matrix execution

First add reusable test-only process, filesystem and SQLite fault primitives.
Then each owner executes all applicable F01-F18 rows:

- Memory and KG;
- Automation and TaskFlow;
- Matrix;
- Evidence;
- Fleet/Supervisor.

The aggregate result records each owner/row as `passed`, `failed`, `not_run` or
`not_applicable_with_reason`. Missing rows cannot be omitted or converted to a
skipped pass. Backup/restore must preserve canonical digests, ownership and
fences.

## 8. P0.7e — dependency inversion and wire isolation

This package closes two architecture gaps not explicitly scheduled by V3.

### E1 — Codex boundary inversion

No new upstream `codex-*` crate may depend on a concrete Hepta implementation.
Existing App Server integrations migrate behind generic ports for evidence,
lifecycle hooks, memory context and governance decisions. Hepta adapter crates
implement those ports. A temporary compatibility feature must default off and
carry a removal deadline.

### E2 — wire/domain separation

The stable Agent control wire crate contains only versioned DTOs and pure value
types. It must not depend on Automation, Fleet registry, SQL, daemon or binary
implementation crates. Domain-to-wire mapping lives in adapter crates.

### E3 — composition roots

`codex` and `hepta` use distinct composition roots and explicit command
allowlists. Sharing libraries is allowed; including the same `main.rs` is not.
Binary crates do not depend on other binary implementation crates.

### Exit criteria

A generated Cargo dependency policy rejects all forbidden edges; public wire
goldens and compatibility tests pass; upstream sync no longer requires Hepta
implementation changes inside generic Codex owners except approved port seams.

## 9. P0.8a — AST/compiler authority caller ratchet

Use Rust-aware resolved symbols rather than source-string matching. The analyzer
must discover constructors, reexports, aliases, wrappers, method references and
macro-expanded calls. It compares them with a checked-in owner/purpose inventory
and includes analyzer/toolchain digests in the result. Compile-fail fixtures
prove denied callers cannot mint or consume authority.

## 10. P0.8b — fleet runtime-instance projection

Publish a bounded, read-only, generation-fenced projection containing desired,
observed and readiness graphs; exact release, source, binary, profile and grant
digests; observation sequence and timestamps. Supervisor aggregates signed
observations. Missing/stale observations are `unknown` or `stale`, never ready.
Projection input cannot mutate lifecycle or grant authority.

## 11. P0.8c — resource-budget enforcement

Implement every row in `HEPTA_RESOURCE_BUDGETS_V1.md` at the owning admission
point. Durable work reserves capacity in the same transaction as admission.
Cancellation and legal terminal transitions release once. Reopen reconstructs
counts from owner state. Budget changes require a new generation and fence.

Required tests include N+1, concurrent terminalization, kill/reopen, memory and
disk pressure, queue/outbox saturation and reservation leak detection.

## 12. P0.8d — exact real-process vertical slice

The exact candidate must start Supervisor, Agentd, App Server and one ingress
adapter as real processes. It must issue/consume a signed bootstrap, reach the
required instance graph, admit one session and Automation wakeup, read/write
Memory only through declared authority, cross one governed physical boundary,
kill at every durable point, restart under a new generation and reconcile
without duplicate effect or stale-owner write.

Evidence binds exact source commit/tree, binaries, merge candidate, profiles,
graphs, grants, schemas, runner IDs, step lists and raw-log digests.

## 13. P0.9 — independently issued external gates

Source cannot close these gates:

- live default-branch ruleset equals the checked-in contract;
- exact-head and merge-candidate jobs have assigned runners and non-empty steps;
- a distinct current-head code owner approves with conversations resolved;
- operator acceptance signs the exact candidate/evidence manifest;
- promotion signs the accepted, unchanged candidate;
- release publishes signed provenance, SBOM, immutable inventory and rollback evidence.

Any source change invalidates downstream acceptance and signatures.

## 14. State ownership and recovery rule

| Durable state | Sole writer | Recovery authority |
|---|---|---|
| thread/session/turn | Codex state runtime | Codex replay/history |
| fleet lifecycle/release | Supervisor registry | registry revision and process lease |
| runtime bootstrap | Supervisor handoff namespace | generation plus retained reservation/claim |
| Automation/TaskFlow | owning Agent store | event ledger and outbox |
| Memory/KG | cognitive store writer | revision ledger and projection rebuild |
| Matrix ingress/send | Matrix store | operation journal and remote lookup |
| provider effect | provider operation owner | intent, operation key and lookup-only reconcile |
| evidence | evidence store | append-only digest chain |

A process may coordinate multiple owners but may not directly commit their
state in one logical transaction.

## 15. Documentation and change protocol

`HEPTA_CURRENT_PLAN.json` is the only current-plan selector. The document
authority index resolves model, plan, ledger, status and component contracts.
Generated projections are never hand-edited. A plan version and its gap ledger
advance together. Dated snapshots, old PR bodies, source-only receipts and
empty Actions runs remain historical/non-authoritative.

Required source commands are selected by the current pointer and include the
architecture projection check, inherited P0.5/P0.6 verifiers, V4 verifier and
package-specific verifiers.

## 16. Completion semantics and stop conditions

“All source gaps closed” requires P0.7a through P0.8d, including P0.7e, to be at
least `source_verified` on one exact candidate. “All gaps closed” additionally
requires every P0.9 external decision to be independently issued for that same
candidate.

Stop and record `base_drift`, `blocked_external`, `not_run` or `rejected` rather
than fabricate progress when the exact base drifts, an upstream seam is absent,
a runner has no jobs/steps, a physical resource or reviewer is unavailable,
repository administration cannot be changed through the available authority,
or closure would require weakening fail-closed semantics.
