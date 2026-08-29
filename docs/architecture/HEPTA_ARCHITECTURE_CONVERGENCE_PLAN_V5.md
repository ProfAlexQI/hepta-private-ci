# Hepta architecture convergence and full gap-closure plan V5

**Date:** 2026-08-30  
**Status:** normative source-execution plan; no runtime, production, operator,
promotion or release authority.  
**Repository:** `ProfHepta/hepta-private-ci`  
**Parent:** Draft PR #79, commit `b1bdd44ee6cbedbcc249150968448e25f5ce859c`, tree `14f6db9aeb747aa0c3c468799d945a83a661e1d1`.  
**Implementation branch:** `codex/architecture-v2-blocker-closure-20260830`.

## 0. V5 decision and audited starting point

V5 supersedes V4 as the only current development plan. It preserves every V4
safety invariant and converts the remaining chapter-level work into package
contracts that can be implemented, reviewed and qualified independently.

The audited parent already contains the signed runtime-bootstrap envelope,
immutable trust/provenance registry, Supervisor issuer, owner-only handoff,
Agentd verification and durable reservation/claim recovery. It also contains
committed `codex-rs/Cargo.lock` and `MODULE.bazel.lock` changes. Therefore
`A-LOCK-01` is no longer a missing-source blocker; it is `source_implemented`
until locked metadata and regeneration run on an assigned runner.

The parent still contains a temporary source-mutating workflow and has no
executable exact-head result: the observed jobs are queued with no assigned
runner and empty steps. V5 treats those facts as separate gaps rather than
letting source, CI and external authority states overwrite one another.

## 1. Completion model

Every gap belongs to exactly one plane:

| Plane | What can close it | Examples |
|---|---|---|
| source | reviewed repository bytes | contracts, adapters, tests, workflows |
| executable | non-empty runner execution on an exact identity | fmt, tests, Clippy, fault rows |
| repository administration | live GitHub settings | ruleset, required checks, code owners |
| independent decision | distinct signer/reviewer | operator acceptance, promotion, release |

A source package advances only through:

```text
open -> source_in_progress -> source_implemented -> source_verified
     -> qualified_exact -> merge_candidate_qualified
```

An external decision advances independently through:

```text
not_issued -> issued | rejected | revoked | superseded
```

`source_implemented` never means a test passed. `queued`, `runner_id=0`,
`steps=[]`, cancelled, skipped or synthetic-only runs are `not_run`.
“All gaps closed” means every source package is merge-candidate qualified and
every external decision is issued for the same immutable candidate.

## 2. Non-negotiable invariants

1. Codex App Server is the sole owner of session, thread, turn, model-response
   and tool-execution state.
2. Every durable record has one writer, one schema owner and one migration
   owner. A coordinator may not directly commit two owners' databases.
3. Cross-owner work is: local transaction, durable intent/outbox, destination
   dedupe/apply, acknowledgement and lookup-only reconciliation.
4. A runtime bootstrap authenticates process start identity only. It is never a
   reusable operation capability.
5. Every physical crossing consumes one current operation-bound capability
   after the final payload and target facts are known.
6. A component cannot mint the capability it consumes. Tokens are not cloneable,
   serializable or persistent.
7. `Queued` and `DispatchAccepted` are not terminal. Unknown external outcomes
   remain `Indeterminate` until a current-fence reconciler commits a legal
   terminal transition.
8. Qualification code, fixtures and signers are excluded from production
   composition roots.
9. A dependency, schema, generated projection or build-lock change is incomplete
   until all derived files are committed in the same candidate.
10. Any drift in source commit/tree, release, binary, profile, authority epoch,
    owner epoch, generation, fence, payload or revocation revision fails closed.

## 3. Package operating contract

Every package must declare:

- exact parent commit and tree;
- one accountable owner and a closed touched-path allowlist;
- prerequisites and blocked-upstream outcomes;
- durable state and migration ownership;
- authority delta, which defaults to all false;
- applicable physical-boundary, durable-fault and resource-budget rows;
- public API and persisted-schema compatibility strategy;
- deterministic source verifier;
- locked metadata, format, tests, check and strict Clippy commands;
- exact-head and merge-candidate evidence requirements;
- rollback, supersession and data recovery rules.

A package cannot change production, operator, promotion or release truth. Any
such change is a separately signed P0.9 decision.

## 4. Delivery graph and safe parallelism

```text
P0.7a bootstrap closure
  -> P0.7b verified-use boundary kernel
       -> P0.7c Memory extraction --------┐
       -> P0.7d durable fault harness ----┼-> P0.7e dependency/wire convergence
                                          │       -> P0.8a caller ratchet
P0.7a ------------------------------------┘       -> P0.8b instance projection
P0.7c + P0.7d + P0.8b --------------------------> P0.8c resource enforcement
P0.8a + P0.8b + P0.8c --------------------------> P0.8d real-process slice
P0.8d ------------------------------------------------> P0.9 external gates
```

P0.7c and P0.7d may run in parallel after B0 contracts freeze. Dependency
inventory and negative CI may land before C7, but removing compatibility debt
waits for the extracted facades. No package may silently absorb another
package's durable owner or authority.

## 5. P0.7a — signed runtime-bootstrap closure

### A0 source normalization

Remove every temporary workflow that commits or pushes reviewed source.
Qualification workflows are read-only. Restore unrelated formatter churn or
record an exact, reviewed reason for each retained file.

### A1 lock coherence

The candidate carries Cargo and Bazel locks. Exact qualification must prove:

```shell
cargo metadata --manifest-path codex-rs/Cargo.toml --locked --no-deps
just bazel-lock-check
git diff --exit-code
```

Regenerated output is comparison evidence only; it may not be pushed by CI.

### A2 transport identity

Publication and consumption must use no-follow regular-file opens, owner-bound
directories, exact mode and link policy, stable opened/path identity, bounded
size, content digest, and durable parent-directory sync. Partial reservation,
published document and durable claim are distinguishable states.

On platforms lacking equivalent kernel guarantees, the source returns
`unsupported_physical_guarantee`; it must not downgrade to a length-only
identity and call the result qualified.

### A3 crash recovery

The following windows are explicit tests and recovery states:

1. before reservation commit: no durable state;
2. after reservation fsync, before document publication: partial, fenced;
3. after document publication, before spawn: exact generation may consume;
4. after claim publication, before cleanup/readiness: consumed, recovery needed;
5. after cleanup, before readiness: claim remains replay fence;
6. new generation: fresh nonce and names; old evidence remains immutable.

### A4 package exit

Source exit requires A0–A3 present and the V5 verifier clean. Executable exit
requires real locked metadata, format, tests, check, strict Clippy and physical
fault tests on the exact source and merge candidate.

## 6. P0.7b — per-use physical capability boundaries

### B0 verified-use kernel

Add pure types for:

- capability kind and issuer;
- operation ID and canonical payload digest;
- authority/profile/release identity;
- authority epoch, owner epoch, generation and fence;
- issued/not-before/expires timestamps;
- revocation stream and revision;
- one-use nonce and verification policy.

The verifier consumes an admission object and returns a non-`Clone`,
non-`Serialize`, non-`Deserialize` token. The physical adapter consumes that
token by value and receives a serializable witness only after revalidating the
final operation. Verification and consumption are separate so payload or
revocation drift between them is rejected.

Required negative tests cover wrong kind, issuer, subject, payload, target,
epoch, generation, fence, time window, revocation revision and token reuse.

### B1 model/provider

Model submission binds model artifact, provider route, policy, request digest,
budget reservation and idempotency key. Effectful provider dispatch additionally
requires an external-effect capability. Unknown results become lookup-only
reconciliation; no blind retry or budget release is permitted.

### B2 tool/network/filesystem

Tool spawn binds executable, argv, cwd, environment policy, sandbox, approval
and resource reservation. Network connect binds canonical destination,
protocol, DNS answer set and resolved socket address. External filesystem
mutation binds no-follow target, parent identity, device/mount and mutation
digest. The final syscall adapter consumes the token.

### B3 secret/Matrix/fleet

Secrets remain opaque `SecretRef` values. Matrix send binds room, event,
payload and operation journal. Fleet mutation binds signed grant, current
registry revision and exact CAS. None may infer success from local enqueue.

### B4 call-site proof

A machine inventory names all constructors, verifiers and consumers. Local and
qualification profiles have compile-fail fixtures proving they cannot create
dangerous tokens. Only physical adapters may consume verified-use tokens.

## 7. P0.7c — Memory bounded-context extraction

The current `codex-hepta-memory` facade is decomposed without changing stored
facts or creating a second writer:

| Package | Physical crate | Owns | Must not depend on |
|---|---|---|---|
| C1 | `codex-hepta-cognitive-types` | pure IDs, records, receipts | SQL, runtime, daemon |
| C2 | `codex-hepta-cognitive-store` | migrations and write transactions | shadow learning |
| C3 | `codex-hepta-memory-retrieval` | bounded read/rank/explain | write authority |
| C4 | `codex-hepta-memory-federation` | capability-bound federated reads | fleet mutation |
| C5 | `codex-hepta-compact-engine` | checkpoint/replay/rehydration | provider effects |
| C6 | `codex-hepta-trajectory-store` | trajectory append/read | artifact promotion |
| C7 | `codex-hepta-learning-shadow` | H6/H7 proposal and evaluation | production writer |
| C8 | `codex-hepta-production-writer` | authority-checked write/outbox | test signer, shadow |
| C9 | `codex-hepta-memory-runtime` | thin composition/facade | migration ownership |

Sequence: freeze symbol/table/caller inventory; extract pure values; move all
migrations once; extract read planes; move compact/trajectory; isolate shadow;
move writer last; migrate callers; then shrink the compatibility facade.

Every step proves old/new reopen, schema byte compatibility, backup/restore,
corruption and interrupted migration. A migration file has one physical owner
at every commit.

## 8. P0.7d — common durable fault matrix

### D0 harness

Create test-only fault primitives for process kill, filesystem publication,
SQLite full/read-only/corruption, WAL reopen, clock, queue saturation and
migration interruption. The harness emits one result for every owner/row pair;
no row can disappear.

### D1 owner execution

Memory/KG, Automation/TaskFlow, Matrix, Evidence and Fleet/Supervisor run every
applicable F01–F18 row against real storage. `not_applicable` requires a
reviewed reason. Each result binds before/after canonical digests, intent/event/
outbox/receipt counts, fence identity, reopen result and raw-log digest.

### D2 aggregate

The aggregate fails unless every required cell is `passed`. It additionally
proves no duplicate external effect, false terminal success, stale-owner write,
partial projection or reservation leak.

## 9. P0.7e — dependency inversion and wire isolation

### E0 executable dependency policy

A checked-in policy and verifier derive the Cargo graph from all manifests.
Forbidden edges and exact temporary debts are explicit. A debt entry has owner,
rationale, replacement port and removal package; changing or adding a debt
fails review.

### E1 Codex ports

Generic Codex owners expose narrow ports for evidence reads, lifecycle hooks,
memory context and governance decisions. They do not depend on concrete Hepta
stores, extensions or daemons. Hepta adapters implement the ports outside the
generic owners. Compatibility features default off and carry removal criteria.

### E2 wire/domain split

The stable wire layer contains versioned DTOs and pure IDs only. It has no SQL,
Automation store, Fleet registry, daemon or binary dependencies. Domain-to-wire
mapping lives in an adapter. Golden fixtures prove strict decoding, bounded
frames and forward/backward compatibility.

### E3 product roots

`codex` and `hepta` have distinct `main` composition roots and explicit command
allowlists. Shared libraries are allowed; `include!("main.rs")` is forbidden.
Binary crates do not depend on another binary implementation crate.

### E4 exit

The policy has zero forbidden unwaived edges and zero expired debts. Upstream
Codex sync can update generic owners without importing concrete Hepta code.

## 10. P0.8a — AST/compiler authority caller ratchet

A Rust-aware analyzer resolves constructors, reexports, aliases, wrappers,
methods and macro-expanded call sites. The checked-in inventory binds symbol,
owner, purpose, source path and permitted profile. Analyzer/toolchain digests
are evidence. Compile-fail fixtures prove denied crates cannot mint or consume
authority. Text grep is only a diagnostic and cannot qualify this package.

## 11. P0.8b — fleet runtime-instance projection

Supervisor publishes a bounded read-only projection with desired, observed and
readiness graphs; exact release/source/binary/profile/grant digests; epochs,
generation, fence, observation sequence and timestamps. Agent observations are
signed and generation-bound. Missing, stale, conflicting or unverifiable
observations are `unknown`/`stale`, never `ready`. Projection input cannot
mutate lifecycle or grant authority.

## 12. P0.8c — resource-budget enforcement

Each row in `HEPTA_RESOURCE_BUDGETS_V1.md` is enforced at the owning admission
transaction. Durable reservations release exactly once on legal terminal
states. `Indeterminate` retains effect capacity unless no-effect evidence is
committed. Reopen reconstructs counts from durable owner state. Budget changes
advance generation and fence.

Required tests: N+1, concurrent terminalization, kill/reopen, DB hard limit,
queue/outbox saturation, memory pressure and reservation leak detection.

## 13. P0.8d — exact real-process vertical slice

The exact candidate starts Supervisor, Agentd, App Server and one ingress
adapter as real processes. It must:

1. issue and consume one signed start bootstrap;
2. reach a generation-fenced instance graph;
3. admit one session and Automation wakeup;
4. read/write Memory only through declared ports and authority;
5. cross one governed physical boundary;
6. inject every applicable durable crash point;
7. restart under a new generation;
8. reconcile without duplicate effect or stale-owner mutation.

Evidence binds source commit/tree, merge candidate, binary digests, profiles,
grants, schemas, runner/job/step IDs and raw logs. An in-process substitute
cannot close this package.

## 14. State ownership and cross-owner sagas

| Durable state | Sole writer/migration owner | Cross-owner interface | Recovery truth |
|---|---|---|---|
| session/thread/turn | Codex state runtime | typed port/events | Codex replay |
| fleet/release/generation | Supervisor registry | fenced commands | registry revision |
| start bootstrap | Supervisor handoff namespace | signed document | reservation/claim |
| Automation/TaskFlow | Agent Automation store | command/outbox | event ledger |
| Memory/KG | cognitive store writer | admission/write intent | revision ledger |
| Matrix ingress/send | Matrix store | operation journal | remote lookup |
| provider effect | provider operation owner | effect intent | lookup reconcile |
| evidence | evidence store | append-only adapter | digest chain |

The canonical cross-owner state machine is:

```text
Prepared -> IntentCommitted -> Dispatching -> DispatchAccepted
         -> Running -> Succeeded | Failed | Cancelled | Indeterminate
Indeterminate -> Reconciling -> ReconciledSucceeded | ReconciledFailed
```

A destination acknowledgement binds the original intent digest. Same key/same
digest returns `AlreadyApplied`; same key/different digest is `Conflict`.

## 15. Schema, API and migration policy

- persisted schemas use explicit versions and additive migrations;
- no migration is edited after qualification;
- unknown fields are rejected on authority-bearing envelopes;
- public wire removals require a versioned decoder and sunset receipt;
- all canonical encodings reject duplicate JSON keys and non-canonical forms;
- backup/restore rederive destination binding and reject source replay;
- a compatibility facade may forward calls but may not own migrations, authority
  or hidden background tasks.

## 16. CI and evidence topology

### Pull request

- exact checkout identity and clean candidate;
- generated architecture projections;
- V5 plan, ledger, dependency and authority verifiers;
- locked Cargo metadata and Bazel lock check;
- package-scoped format, tests, check and strict Clippy;
- changed-path scope and forbidden-edge checks;
- merge-candidate job for the synthetic merge identity.

### Main/nightly

- full workspace compatibility;
- all-platform protocol and physical-file tests;
- common fault aggregate;
- real-process vertical slice;
- upstream Codex rebase/sync compatibility;
- resource and soak suites.

All workflows are read-only. No qualification workflow commits, pushes,
updates refs, changes repository administration or emits acceptance/promotion/
release decisions.

## 17. Repository integration and PR order

Each package uses one branch/PR stacked on the immediately preceding exact
head. It revalidates parent commit/tree before every write. A package does not
merge itself. Base drift stops the package; it is rebased and requalified rather
than force-combined with stale evidence.

Preferred review units are:

```text
V5-doc-authority
P0.7a-normalize
P0.7b-B0-kernel
P0.7b-B1-B3-adapters
P0.7c-C1-C3
P0.7c-C4-C9
P0.7d-harness-and-owners
P0.7e-ports-wire-roots
P0.8a-caller-ratchet
P0.8b-instance-projection
P0.8c-resource-enforcement
P0.8d-real-process
```

## 18. P0.9 external gates

Source cannot self-close:

- live default-branch ruleset equals the checked-in contract;
- exact-head and merge-candidate jobs have assigned runners and non-empty steps;
- a distinct current-head code owner approves and resolves conversations;
- operator acceptance signs the exact candidate/evidence manifest;
- promotion signs the unchanged accepted candidate;
- release publishes signed provenance, SBOM, immutable inventory and rollback
  evidence.

Any source change invalidates downstream signatures.

## 19. Stop conditions

Record `base_drift`, `blocked_upstream`, `blocked_external`, `not_run`,
`rejected` or `resume_required` rather than weakening a guard. In particular,
lack of a runner, reviewer, physical platform, live repository setting, operator
key or release signer is not a source pass.

## 20. Definition of done

Source completion requires P0.7a–P0.8d at `source_verified` on one exact
candidate, zero unowned migrations, zero forbidden dependency edges, zero
omitted fault/resource rows and all authority flags still false.

Executable completion requires exact-head and merge-candidate runs with assigned
runners, non-empty successful steps and candidate-bound raw evidence.

Full completion additionally requires all P0.9 decisions issued for that same
candidate. Until then the only truthful overall state is
`source_execution_in_progress_external_gates_open`.
