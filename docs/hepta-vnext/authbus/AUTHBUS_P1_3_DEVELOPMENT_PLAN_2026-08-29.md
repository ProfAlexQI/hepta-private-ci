# AuthBus P1.3 canonical quota-registry development plan

Date: 2026-08-29
Parent tranche: P1.2 executable-qualified durable identity and evidence replay state
Disposition: **IMPLEMENT IN AN ISOLATED, DEFAULT-OFF QUALIFICATION TRANCHE**

## 1. Problem statement

The inherited AuthBus qualification stack contains three incompatible quota
representations:

1. B4 exposes a five-dimensional `QuotaVector`;
2. P0.2 SQLite WAL exposes a second five-dimensional `QualificationQuota`;
3. P0.3 defines a private six-dimensional `CanonicalQuotaVector` and silently
   assigns `request_count = 1` when it upgrades B4.

That split allows wire, persistence, receipt and metric names to drift. It also
allows `request_count` to be lost during downgrade without an explicit
compatibility decision.

## 2. P1.3 target

P1.3 establishes one contract-owned six-dimensional registry in
`codex-hepta-contracts`:

```text
request_count
rpm
tpm
concurrency
day_budget
context
```

The registry owns, in stable order:

- canonical and wire keys;
- SQLite limit, reserved and used columns;
- durable receipt keys;
- metric suffixes;
- semantic units;
- legacy-v0 presence;
- a versioned SHA-256 schema digest.

## 3. Compatibility rules

The B4 and P0.2 five-dimensional structs remain legacy compatibility values,
not schema authorities.

Missing `request_count` has exactly two policies:

```text
RejectMissing
AssumeOnePerPermit
```

The default-safe policy is rejection. A migration may assume one request only
when the caller explicitly selects `AssumeOnePerPermit`. Downgrade to five
dimensions is permitted only when `request_count == 1`; every other value fails
as a lossy downgrade.

## 4. Source changes

- add `quota_registry.rs` to `codex-hepta-contracts`;
- re-export the registry, canonical vector, limits, projections and migration
  types from `codex-hepta-contracts`;
- remove the duplicate P0.3 canonical vector and limits definitions;
- make P0.3 re-export the contract-owned canonical types;
- add explicit legacy adapters to B4 and P0.2;
- add the isolated `codex-hepta-authbus-p1-3-qualification` crate;
- add a source verifier and exact-head hosted qualification workflow.

No product workspace membership, listener, provider adapter or OpenBao path is
added.

## 5. Required executable gates

The exact final head must pass on non-zero hosted runners:

1. source/cleanup/negative-authority verification;
2. `codex-hepta-contracts` tests, check and strict Clippy;
3. inherited P0.3 scheduler tests, check and strict Clippy;
4. inherited P0.2 SQLite WAL tests, check and strict Clippy;
5. P1.3 default-off test;
6. P1.3 registry/adaptor matrix, all-target check and strict Clippy;
7. digest-bound executable receipt validation.

## 6. Exit criteria

P1.3 closes only when:

- the P0.3 source contains no local `CanonicalQuotaVector` or
  `CanonicalQuotaLimits` struct;
- all six projection surfaces originate from the one descriptor registry;
- both five-dimensional adapters reject missing request count by default;
- lossy downgrade is executable-tested;
- all authority and product-wiring fields remain false;
- an exact-head hosted run has non-empty steps, non-zero runners and success;
- a two-file executable receipt commit verifies against that prior source run.

P1.3 does not authorize production activation or P1.4.


## Canonical B2 semantic closure

The executable tranche also binds the registry to the active AUTHBUS.11-v1.3
source registry and domain projection. The canonical wire value distinguishes
`known`, `explicit_unknown`, and `not_declared`; absence is never converted to
zero. Descriptor metadata owns exact units, lifecycle, window requirements,
integer round-up before hold, and integer-exact terminal finalization. The old
four-dimensional B2 reservation remains decode-only compatibility material;
new source uses `QuotaReservationV1_3` and the six-dimensional `UsageVector`.
A request-count-only scalar may be decoded or advertised only as
request-count-only and cannot imply rpm, tpm, concurrency, day-budget, or
context authority.
