# AuthBus P1.3 canonical quota-registry qualification

This nested crate proves the default-off P1.3 quota-registry tranche.

It verifies that:

- the canonical AuthBus quota vector has exactly six ordered dimensions;
- one descriptor registry owns wire, SQLite, receipt and metric names;
- P0.3 re-exports the contract-owned canonical type instead of defining a copy;
- B4 and P0.2 five-dimensional values are explicit legacy projections;
- missing `request_count` fails closed unless an explicit one-request-per-permit
  migration policy is selected;
- lossy downgrade is rejected;
- schema and migration evidence are digest-bound;
- no listener, provider call, OpenBao client or production authority is enabled.
