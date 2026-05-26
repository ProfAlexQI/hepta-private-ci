# Hepta JSON Report Capture Migration Inventory Gate

`scripts/hepta-json-report-capture-migration-inventory-gate.sh` is a
static inventory gate for the gradual migration from duplicated local shell
JSON capture helpers to `scripts/lib/hepta-json-report-capture.sh`.

The gate does not migrate scripts by itself. It records the current local
helper definition budget so new duplicated `capture_json_report()` or
`extract_first_json_object()` definitions cannot be added silently.

## Contract

- Confirm the shared helper exists and is executable.
- Confirm the diagnostic contract gate exists and is executable.
- Count remaining local `capture_json_report()` definitions outside the shared
  helper.
- Count remaining local `extract_first_json_object()` definitions outside the
  shared helper.
- Require those counts to stay within the configured migration budget.
- Emit the remaining file lists so the next migration slice has a concrete
  target set.

## Defaults

- `HEPTA_JSON_CAPTURE_MAX_LOCAL_CAPTURE_DEFINITION_COUNT=42`
- `HEPTA_JSON_CAPTURE_MAX_LOCAL_EXTRACT_DEFINITION_COUNT=10`

These values intentionally match the inventory at introduction time. Future
migration slices should lower the budgets when they remove local definitions.

## Side-Effect Boundary

The gate performs static repository inspection only. It does not write
workspace files, persist evidence, restart services, mutate launchd, touch the
Gateway, read credentials or secrets, or send external messages.
