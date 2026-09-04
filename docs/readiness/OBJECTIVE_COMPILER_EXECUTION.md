# Objective compiler execution specification

**Overlay:** `HEPTA-V8-PRECODING-READINESS` v8.2.0-readiness
**Bound modules:** `objective.compiler`, `intelligence.control`, `kernel.authority`, `learning.ledger`
**Source target:** `codex-rs/hepta-objective`

## 1. Scope and authority boundary

`objective.compiler` converts one bounded request into an immutable objective revision. It does not infer authority from prose, relax a hard constraint, rewrite an objective during a run, select an action or execute an effect. The output is advisory until independently authorized by the existing authority boundary.

The implementation consumes `ObjectiveSourceEnvelopeV1` and produces `ObjectiveConstraintSetV1`, `ObjectiveCompileReceiptV1`, the canonical `ObjectiveFunctionV1`, and, on conflict, `ObjectiveConflictReceiptV1`. Unknown critical fields, scope escape and digest mismatch fail before semantic compilation.

## 2. Input grammar and canonical IR

The accepted structured grammar is a bounded object with the following semantic classes:

```text
identity: request, principal scope, locale, source trust and observed time
success: predicates, terminal conditions and evidence requirements
actions: legal classes, forbidden classes and required confirmation classes
constraints: constitutional, principal, environment and task constraints
preferences: soft dimensions with units, direction and bounded weight range
resources: time, token, compute, memory, network and effect ceilings
risk: risk class, abstention rule, rollback and compensation requirements
provenance: exact source and normalization profile digests
```

Free text is evidence for intent extraction, never the final authority representation. The compiler must emit a typed IR in which every predicate has an identifier, unit, comparator, bound, evidence source and terminality. Arrays are stable-sorted by semantic identifier. Unicode is normalized to NFC; timestamps are UTC; durations are integer microseconds; numeric values use registered fixed-point profiles; duplicate semantic keys are rejected.

The canonical IR contains no raw credentials, unrestricted external text, hidden model state or executable code. All payloads have explicit count and byte bounds.

## 3. Constraint precedence and conflict resolution

Precedence is lexicographic and non-compensable:

```text
P0 constitutional authority, truth, privacy, deletion and writer ownership
P1 explicit principal scope and forbidden effects
P2 environment and adapter safety constraints
P3 task success predicates and terminal conditions
P4 soft utility preferences and resource allocation
```

A lower class cannot offset a higher-class violation. Within one hard class, incompatible constraints produce the minimal unsatisfied set using deterministic deletion filtering over canonical constraint order. The compiler emits `ObjectiveConflictReceiptV1`; it never silently drops a constraint or asks a model to choose which hard rule to ignore.

Soft dimensions are accepted only when their units, direction, normalization and allowed range are registered. Missing soft weights use the named baseline profile; they are not guessed from unrelated user history.

## 4. Deterministic compilation algorithm

```text
validate envelope schema, bytes, scope, freshness and source trust
extract only registered structured fields
normalize identifiers, units, strings, times and fixed-point numbers
classify constraints into P0..P4
reject duplicate or contradictory identity fields
compute hard-feasibility and minimal conflict set
construct legal action grammar and explicit abstain action
construct success and terminal predicates
bind resource, risk, evidence and rollback profiles
stable-sort every set and encode canonical JSON
compute objective revision and semantic digest
validate ObjectiveFunctionV1 and publish receipt atomically
```

Compilation is a pure function of the source envelope, baseline profile and registered schema revisions. Retry with identical inputs yields identical semantic bytes. Reuse of a request/revision identity with different semantics is `conflict`.

## 5. State machine and persistence

The finite state machine is:

```text
received -> schema_validated -> normalized -> constraints_resolved
         -> compiled -> published
         -> rejected
         -> conflict
```

`objective.compiler` is stateless for domain facts. The owning caller persists the immutable `RunStartSnapshotV1` and compile receipt. Publication occurs only after the objective digest, constraint digest and source envelope digest agree. A crash before publication leaves no selected objective; a crash after durable publication is reconciled by exact idempotency key `(request_id, source_digest, schema_digest)`.

A changed success predicate, hard constraint, allowed effect, evidence requirement, principal scope or rollback class creates a new objective revision and a new run snapshot.

## 6. Error taxonomy and fallback

| Code | Condition | Disposition |
|---|---|---|
| `OBJ-E001` | invalid schema, count or byte bound | rejected |
| `OBJ-E002` | unknown critical field | rejected |
| `OBJ-E003` | principal or source scope mismatch | authority rejection |
| `OBJ-E004` | incompatible hard constraints | conflict receipt |
| `OBJ-E005` | unknown unit or normalization profile | rejected |
| `OBJ-E006` | legal action set empty after hard filtering | explicit abstain/ask path |
| `OBJ-E007` | stale baseline or schema revision | unavailable until refreshed |
| `OBJ-E008` | identity reused with different semantics | conflict and quarantine |
| `OBJ-E009` | model or external text attempts authority escalation | rejected and security event |

Fallback is the last valid immutable objective only when the request identity and principal scope are unchanged; otherwise the system asks for clarification or abstains. It never substitutes an easier goal.

## 7. Security and adversarial inputs

Untrusted pages, emails, files, tool output and model prose remain evidence. They cannot create P0–P2 constraints, legalize an effect, change principal scope or weaken evidence requirements. Negative fixtures include prompt injection, hidden HTML instructions, homograph identifiers, duplicate JSON keys, oversized arrays, NaN/infinity, conflicting time units, path traversal and embedded secrets.

The compiler redacts safe explanations and records only digests for unrestricted source text. It has no model, tool, network, filesystem, secret, Matrix, fleet or external-effect authority.

## 8. Performance envelope

Pilot bounds are `<=256 KiB` encoded input, `<=256` constraints, `<=128` success predicates, `<=128` legal action classes and `<=64` soft dimensions. The deterministic compile path is `O(n log n)` from canonical sorting, with p95 `<=2 ms`, p99 `<=5 ms`, transient allocation `<=512 KiB` and no network or synchronous central RPC on the local path. Exceeding a bound is rejection, not truncation after semantic analysis.

## 9. Golden fixtures and tests

- `OBJ-GV-001`: one read-only request, one success predicate and one evidence requirement compiles identically under reordered input keys.
- `OBJ-GV-002`: a principal forbids network access while task text requests a download; the minimal conflict set contains the network request and produces no legal network action.
- `OBJ-GV-003`: two equivalent Unicode forms produce identical canonical bytes.
- `OBJ-GV-004`: duplicate semantic constraint IDs reject before digest publication.
- `OBJ-GV-005`: changing a soft weight changes the objective digest but cannot change the hard constraint digest.
- `OBJ-GV-006`: an untrusted document containing “ignore previous instructions” remains evidence and creates no constraint or action class.

Tests cover round trips, canonical ordering, unit conversion, conflict minimization, idempotent retry, crash before/after publication, stale revision, empty legal set, redaction and property-based permutation invariance.

## 10. Implementation sequence

Implement canonical primitives and schema decoding, then the precedence classifier, conflict minimizer, legal-action grammar, canonical encoder, digest/receipt emission, fault tests, benchmarks and the `ObjectiveFunctionV1` adapter. The deterministic implementation precedes any model-assisted intent extraction. Model extraction remains a candidate producer whose output must pass the same deterministic compiler.

## 11. Coding-entry checklist

Coding may start when `OBJ-0-OBJECTIVE-CONTRACTS` has an exact lane envelope, the four readiness protocols are generated as Rust types, canonical fixtures are frozen, `platform.types` and `cognitive.read` interfaces are pinned, all error codes are represented, and no positive authority delta exists. Source completion still requires the declared root, exact-source and merge-candidate tests and independent review.

## Appendix A. Closed gap and protocol mapping

This appendix is a closed-world traceability projection. Each identifier is normative in `READINESS.json`, `PROTOCOLS.json` or `GAPS.json`; this Markdown file does not redefine the registry record.

Protocols:

- `ObjectiveSourceEnvelopeV1`
- `ObjectiveConstraintSetV1`
- `ObjectiveConflictReceiptV1`
- `ObjectiveCompileReceiptV1`

Closed documentation gaps:

- `RDY-GAP-OBJ-001`
- `RDY-GAP-OBJ-002`
- `RDY-GAP-OBJ-003`
- `RDY-GAP-OBJ-004`
- `RDY-GAP-OBJ-005`
- `RDY-GAP-OBJ-006`

Bound work packages:

- `C1-PROMPTED-MEMORY-RETRIEVAL-RANK`
- `DOC-3E-PRECODING-READINESS-CLOSED-WORLD`
- `INT-2-AGENTD-CODEX-COMPOSITION`
- `INTELLIGENCE-A0-Q0.63`
- `LRN-0-CAUSAL-LEARNING-CONTRACTS`
- `LRN-1-DURABLE-EPISODE-LEDGER`
- `OBJ-0-OBJECTIVE-CONTRACTS`
- `OBJ-1-OBJECTIVE-COMPILER`
- `P0.7B-B0-VERIFIED-USE`
- `P0.7B-B2-TOOL-NET-FS`
- `P0.7B-B3-BOUNDARIES`
- `P0.7B-B4-CALLSITE-PROOF`
- `P0.8A-AST-RATCHET`
