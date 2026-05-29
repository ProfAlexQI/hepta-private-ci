# Hepta Preflight Terminal Coverage Inventory Gate

The full light preflight is intentionally long. Its terminal coverage matters:
JSON report capture diagnostics, migration inventory, latest upstream safety and
release-governance locks, operator briefing non-persistence, gateway/native
tests, control-ui smoke, explicit native/release skip branches, and the final
`Hepta preflight passed` marker must not disappear or reorder silently.

`scripts/hepta-preflight-terminal-coverage-inventory-gate.sh` is a static,
stdout-only inventory gate for that coverage. It reads
`scripts/hepta-preflight.sh` by default, validates shell syntax, extracts
`[hepta-preflight]` markers, and verifies a required terminal marker sequence.
For synthetic diagnostics only, `HEPTA_PREFLIGHT_TERMINAL_COVERAGE_PREFLIGHT_TEXT`
can provide inline preflight text without writing fixture files.

The gate checks:

- the canonical preflight script exists and parses with `bash -n`;
- the marker count is at least the configured minimum;
- critical markers are present exactly once and in order;
- JSON report capture diagnostic and inventory gates stay before latest
  upstream safety/governance/operator-briefing gates;
- gateway/native/control-ui verification markers remain wired;
- native app and release build branches keep explicit skip controls;
- the final whitespace/status block runs workspace diff, cached diff, and git
  status checks in that order before the pass marker;
- the final `Hepta preflight passed` marker remains present.

`scripts/hepta-preflight-terminal-coverage-diagnostic-contract-gate.sh` is a
synthetic fixture contract for the inventory gate. It proves the good fixture
passes and negative fixtures fail closed for:

- missing required marker;
- duplicated required marker;
- out-of-order required marker;
- marker count budget shrinkage;
- missing terminal pass marker;
- missing native/release skip branches.
- missing final workspace diff check;
- missing final cached diff check;
- missing final git status check.
- out-of-order final whitespace/status checks.

The gate does not run the full preflight, run native app gates, build release
artifacts, restart services, mutate launchd, fetch or merge upstream, invoke a
provider/model, read credentials, or send externally. It only makes the
terminal preflight coverage machine-readable so future edits cannot shrink the
verified tail unnoticed.
