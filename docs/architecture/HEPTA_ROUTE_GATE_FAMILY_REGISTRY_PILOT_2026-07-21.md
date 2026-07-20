# Hepta route-gate family registry pilot (2026-07-21)

## Baseline

At `578d7d2d53899d8cced666d700b491db66bf4623`, the route-gate parameterization layer has:

- 10 top-level `*-route-gate-runner` executors totaling 1,883 lines;
- 10 matching `*-route-gate-specs-v1.json` registries totaling 3,592 lines;
- 9 specialized registries that each contain exactly three receipt-state specs;
- 30 existing wrapper paths that are part of compatibility contracts.

The runner and registry families repeat registry validation, route lookup, native source checks,
focused tests, optional live GET validation, report construction, and normalized-output receipts.

## Pilot boundary

The pilot migrates two families in separate atomic commits:

- `memory_live_mutation_activation_command_result_receipt_v1`, whose specialized executor was the
  shortest at 154 lines and has no source-report capture dependency;
- `runtime_provider_router_activation_command_result_receipt_v1`, whose 172-line executor shares
  the same report base but declares source capture, a 24-sample parameter floor, no terminal fields,
  and a pass message.

- `scripts/hepta-route-gate-specs-v1.json` is the central family registry.
- `scripts/hepta-route-gate-runner` resolves embedded and referenced family specs and executes the
  configurable `native_requirements_report_v1` profile.
- The old family registries remain the referenced spec payloads so their JSON paths and schemas stay
  compatible.
- Both old family runners remain at their original paths as 36-line compatibility dispatchers.
- All six existing route wrappers remain byte-for-byte unchanged.

Across the central executor and these two specialized executors, production runner code moves from
545 lines at the baseline to 509 lines after the second migration, while adding reusable validation
and report-profile controls for subsequent families.

No compatibility entry is deleted in this pilot. The central registry rejects duplicate family or
spec IDs, unknown profiles, path traversal, missing registries, incompatible schemas, and invalid
normalized-output receipts.

## Compatibility proof

`scripts/hepta-route-gate-family-registry-self-test` checks:

- exact legacy `--validate`, usage, unknown-ID output, and exit status;
- eight fail-closed registry fixtures;
- direct unified-runner versus legacy-runner normalized output for all six states;
- every existing per-spec `baseline_normalized_output_sha256` receipt.

The existing native gateway contract continues to compare each external `GateSpec` with the
canonical route registry. Architecture budgets cap the compatibility runner at 40 lines and the
central executor at 500 lines.

## Follow-up boundary

Future migrations should add one family per commit, reuse an existing execution profile where
possible, and retain the old runner/registry/wrapper paths until all script, Rust, and documentation
callers have moved. Consolidating referenced spec payloads and retiring compatibility paths are
separate later steps; neither is implied by this pilot.
