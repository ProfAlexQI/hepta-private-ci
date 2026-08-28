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

## Hosted source result and matcher repair

The exact-head hosted source job executes real steps and passes the inherited
aggregate gate, runtime source gate, claim-boundary verification, and artifact
upload.

Successive patch-materialization runs exposed three anchors with the same
underlying defect:

- `correlated WindowHandle setter`;
- `public hook reexports`;
- `explicit close destruction hook`.

The canonical patcher uses squiggly Rust heredocs and then requires byte-exact
matching. Depending on the target block, the heredoc removes all or part of the
Rust indentation, so a single fixed indentation shift is not correct.

The final repair changes only the temporary patcher's `replace_once!` matching
function and remains bounded by the canonical patcher Git blob:

- try the canonical byte-exact match first;
- only when the exact match count is zero, construct a whole-block pattern that
  ignores leading horizontal whitespace on each nonblank line;
- require exactly one indentation-flexible match; zero or more than one aborts;
- derive the target base indentation from the matched first nonblank line;
- normalize the replacement's common indentation and reapply that target base;
- preserve the original Makepad revision/blob checks, changed-file set, diff
  checks, receipt semantics, and all authority boundaries;
- keep the default product dependency unchanged.

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
