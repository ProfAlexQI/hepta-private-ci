# Hepta browser visual smoke v1

`scripts/hepta-browser-visual-smoke.sh` is the canonical public entrypoint. It
sources the shell stages in execution order and preserves the existing
environment variables, screenshots, JSON probe files, stdout, exit codes, and
receipt schema.

Responsibilities are intentionally separated:

- `config.sh` resolves the historical `HEPTA_*` environment contract.
- `static-contract.sh` validates the served HTML, CSS, JavaScript, assets, CSP,
  and merge-completion truth before launching Chrome.
- `capture.sh` and `capture-viewport.cjs` own deterministic screenshots.
- `scenarios.sh` dispatches the Chrome/CDP and Playwright probes.
- `density-qa.cjs` owns Chrome lifecycle and report aggregation. Its ordered
  `density-probe/` fragments retain one shared in-page lexical scope while
  keeping visual-audit domains reviewable.
- `progressive-enhancement-*.cjs` own read-only and adversarial browser flows.
- `validate-results.sh` enforces the fail-closed jq truth tables.
- `receipt.sh` emits the stable `hepta-browser-visual-smoke` receipt.

The legacy `scripts/hepta-codex-browser-visual-smoke.sh` is only a compatibility
symlink. New behavior belongs in the canonical modules, never in the alias.
