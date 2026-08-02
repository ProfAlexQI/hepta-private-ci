# Hepta UI current readiness authority

The only active UI readiness entry point is:

```sh
scripts/hepta-ui-current-readiness.sh --evidence-dir <new-directory> --verify
```

It binds every generated receipt to the current Git HEAD and a deterministic
fingerprint of the UI source worktree. A source change during the run, a dirty
worktree, an unaccounted Robrix drift path, a missing feature configuration, or
a stale receipt fails closed.

The readiness levels are deliberately separate:

- `source`: strict upstream ledger, Native product shell, canonical tokens,
  no-default/default/all-feature checks, and package metadata.
- `local`: source plus a current-source unsigned package, Control browser smoke,
  and current Native-window evidence.
- `full`: local plus live Matrix, authoritative Hepta adapter, real-device, and
  accessibility receipts.
- `ga`: full plus signed, notarized, stapled, explicitly authorized public
  distribution evidence.

Historical screenshot folders and closeout reports remain immutable audit
records. They are never consumed by the current orchestrator. The recursive
visual-referee stages v8–v41 are catalogued in
`scripts/hepta-ui-legacy-visual-gates.json` and are compatibility/reproduction
tools only.

Useful focused commands:

```sh
scripts/hepta-native-robrix-upstream-sync-check-v2.sh --json --strict
scripts/hepta-native-product-shell-gate-v2.sh --json
scripts/hepta-ui-light-glass-token-sync.rb --check
scripts/hepta-native-feature-matrix-gate.sh --output /tmp/native-features.json
scripts/hepta-native-current-package-gate.sh --build --output /tmp/native-package.json
```

None of these commands signs, notarizes, staples, uploads, publishes, sends a
message, or mutates a backend.
