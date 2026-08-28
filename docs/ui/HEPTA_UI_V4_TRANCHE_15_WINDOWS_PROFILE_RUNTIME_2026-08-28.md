# Hepta UI v4 Tranche 15 — Windows material-profile runtime producer

## Scope

This tranche adds one isolated Windows producer that executes the complete
pre-product evidence path in a single Makepad UI-thread-ordered process:

1. correlated root-window Mica request and DWM backdrop readback;
2. creation of a dedicated popup window with a different HWND;
3. correlated popup Acrylic request and DWM backdrop readback;
4. correlated explicit `WindowBackdrop::None` rollback and readback;
5. popup close and exact `Destroyed` acknowledgement;
6. export through `HeptaWindowsTransientMaterialHost::profile_evidence`;
7. fail-closed aggregation through `aggregate_windows_material_profile`.

The producer uses the existing exact-revision vendored Makepad hook and does
not modify the default product dependency.

## Receipt identity

A successful receipt binds:

- the exact Hepta commit and tree;
- Makepad revision `c4335cee10b22aca768510c9d072b0ca1bba15c8`;
- root and popup `WindowId` index/generation;
- explicit non-zero root and popup HWND values;
- request sequences 1, 2 and 3;
- Mica, Acrylic and `None` backdrop-only readbacks;
- the exact popup destroyed identity.

The pass and fail shapes share the same Schema. A failure requires the exact
probe phase and at least one bounded failure reason, while keeping every
qualification and authority flag false.

## Hosted source result and bounded canonical-patcher repair

The exact-head hosted source job executes real steps and passes the inherited
aggregate gate, runtime source gate, claim-boundary verification, and artifact
upload.

Successive patch-materialization runs exposed four deterministic bugs in the
inherited canonical patcher:

1. `correlated WindowHandle setter` indentation mismatch;
2. `public hook reexports` indentation mismatch;
3. `explicit close destruction hook` indentation mismatch;
4. `git diff --name-only` paths retained newline terminators because
   `String#lines` was used without `chomp`.

The first three share one root cause: squiggly Rust heredocs and byte-exact
matching do not preserve every target block's intended indentation. A single
fixed indentation shift is not correct for all blocks.

The final repair modifies only a temporary copy of the canonical patcher and is
locked to Git blob `369e607f4f80d08d739d2f83778fb4e37aa50d4e`:

- canonical byte-exact matching remains the first path;
- only when exact count is zero, matching ignores leading horizontal whitespace
  per nonblank line across the entire block;
- exactly one flexible match is required; zero or ambiguity aborts;
- replacement common indentation is normalized and the matched target base
  indentation is reapplied;
- changed-file output uses `lines(chomp: true)` before comparing against the
  unchanged exact allow-list;
- Makepad revision/blob checks, the exact changed-file allow-list, `diff
  --check`, receipt semantics, and every authority boundary remain active;
- the default product dependency remains unchanged.

The materializer invokes
`scripts/hepta-ui-v4-run-fixed-makepad-windows-ack-patch` and records the wrapper
and canonical patcher blob in its receipt.

## Runtime environment

Compilation remains portable across Ubuntu, Windows and macOS. The actual DWM
runtime producer is intentionally assigned to a governed interactive Windows
runner carrying all labels:

```text
self-hosted
Windows
X64
hepta-ui-dwm
```

A generic Windows Server hosted runner is not treated as Windows 11 compositor
evidence.

## Authority boundary

Even `PASS_WINDOWS_MATERIAL_PROFILE_AGGREGATE` means only that the isolated
evidence set is internally consistent and eligible for a later product
integration review. The receipt fixes the following to false:

```text
productBound
transientSystemMaterialBound
completeProfileBound
systemMaterialBound
nativeProductRuntime
deviceValidation
network
mutation
effect
liveAdapter
production
operatorAcceptance
promotion
release
```

This tranche does not register a Script module, modify
`hepta_material_app_lifecycle.rs`, or attach the aggregate to
`HeptaPlatformMaterialHost`.
