# AuthBus P1.3 implementation status

Date: 2026-08-29
Stage: P1.3 canonical quota registry
State: **IMPLEMENTED IN SOURCE / NOT YET EXECUTABLE-QUALIFIED**

## Implemented

- one six-dimensional descriptor registry in `codex-hepta-contracts`;
- stable registry schema and SHA-256 digest;
- wire, SQLite-limit, SQLite-reserved, SQLite-used, receipt and metric
  projections generated from the same descriptors;
- canonical vector arithmetic, fail-closed limits and terminal usage;
- explicit five-to-six-dimensional migration policy;
- lossy downgrade rejection;
- P0.3 canonical type duplication removed;
- B4 and P0.2 retained only as explicit legacy projections;
- isolated P1.3 qualification crate, tests, verifier and hosted workflow.

## Still required before closure

- a real exact-head hosted qualification result;
- non-zero runner IDs and non-empty steps for every required job;
- a digest-bound executable qualification receipt;
- receipt-head self-verification.

## Authority boundary

```text
qualification_only=true
qualified=false
wired=false
authority=false
effect_authority=false
production_caller=false
production_writer=false
operator_acceptance=false
promotion=false
g5_allowed=false
execute_allowed=false
listener_enabled=false
provider_call_enabled=false
openbao_enabled=false
parent_workspace_wired=false
private_key_storage=false
raw_signature_storage=false
secret_storage=false
```


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
