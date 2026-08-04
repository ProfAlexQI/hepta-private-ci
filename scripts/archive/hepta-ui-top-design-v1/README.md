# Archived Hepta UI top-design gates

This directory owns the historical HTML-fixture visual referee implementation.
It is not part of current-source readiness.

The archived set contains:

- 40 harsh top-design implementations, stages v2 through v41;
- the 2026 HTML Native fixture visual-smoke implementation;
- the dependency inventory in `manifest.json`.

## Compatibility entrypoints

The old paths under `scripts/` remain as relative symlinks. They are deliberate
thin compatibility entrypoints, not duplicate implementations. Historical
evidence replay and source-marker checks must continue to use those old paths:

```sh
bash scripts/hepta-ui-harsh-top-design-referee-v41-exhaustive-small-control-submenu-gate.sh <evidence-dir>
scripts/hepta-native-fixture-visual-smoke.sh
```

Invoke the compatibility paths rather than the implementation paths in this
directory. Several frozen implementations determine the repository root from
`$0`; invoking through the old path preserves that original behavior and keeps
their receipts reproducible.

## Dependency audit

The complete direct-reference scan covered `scripts/`, `.github/`, `codex-rs/`,
`apps/`, and `docs/` before the move.

| Consumer class | Result | Compatibility decision |
| --- | --- | --- |
| Current readiness | No v2-v41 or HTML-fixture execution | No current orchestrator change |
| CI workflows | No direct v2-v41 or HTML-fixture execution | No CI workflow change |
| Rust runtime | No direct script execution | Historical migration-input test remains catalog-only |
| Historical v4-v41 chain | Each stage calls its predecessor through `scripts/`; v12 also runs the fixture | Keep all old paths as symlinks |
| Active static source checks | Migrated to `apps/hepta-native/packaging/native-fixture-contract-v1.json` | Archive no longer supplies current static gates |
| Historical report readers | Replay and referee checks still read old receipt names | Receipt names and behavior remain unchanged |
| Docs and evidence assets | Preserve old paths as evidence-era identifiers | Do not rewrite immutable evidence |

Stages v2 and v3 are independent roots. Stages v4-v41 form the predecessor
chain `vN -> v(N-1)`. Stage v12 is the only historical stage that executes the
Native fixture script. The exact non-chain consumers and evidence documents are
listed in `manifest.json`.

## Retired root report generators

`scripts/hepta-ui-demo-evidence-gate.sh` and
`scripts/hepta-ui-top-design-referee-refresh-gate.sh` were retired on
2026-08-04 after the dependency scan proved that neither current readiness nor
CI invoked them. Their local-fixture reports were superseded by
`scripts/hepta-ui-current-readiness.sh`. The frozen migration catalog and dated
audit documents retain the old names as historical identifiers; they are not
runtime consumers and must not be rewritten as current evidence.

Eight compatibility/replay gates still consume the frozen demo or top-design
receipt names. They remain intentionally available for historical replay and
negative release-chain tests, but do not make either retired generator part of
current readiness. Their exact inventory is ratcheted in `manifest.json` and
the archive self-test.

## Verification

Run:

```sh
scripts/hepta-ui-legacy-visual-archive-self-test.sh
```

The self-test checks the manifest, exact stage set, symlink targets, script
syntax, recursive dependency closure, fixture marker transparency, the absence
of retired root generators, and the absence of direct legacy execution from
current readiness, CI, and Rust.
