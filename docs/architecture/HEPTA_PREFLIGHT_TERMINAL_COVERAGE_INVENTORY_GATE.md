# Hepta Preflight Terminal Coverage Inventory Gate

The full light preflight is intentionally long. Its terminal coverage matters:
JSON report capture diagnostics, migration inventory, latest upstream safety and
release-governance locks, operator briefing non-persistence, gateway/native
tests, control-ui smoke, explicit native/release skip branches, and the final
`Hepta preflight passed` marker must not disappear or reorder silently.

`scripts/hepta-preflight-terminal-coverage-inventory-gate.sh` is a static,
stdout-only inventory gate for that coverage. It reads
`scripts/hepta-preflight.sh`, validates shell syntax, extracts
`[hepta-preflight]` markers, and verifies a required terminal marker sequence.

The gate checks:

- the canonical preflight script exists and parses with `bash -n`;
- the marker count is at least the configured minimum;
- critical markers are present exactly once and in order;
- JSON report capture diagnostic and inventory gates stay before latest
  upstream safety/governance/operator-briefing gates;
- gateway/native/control-ui verification markers remain wired;
- native app and release build branches keep explicit skip controls;
- the final `Hepta preflight passed` marker remains present.

The gate does not run the full preflight, run native app gates, build release
artifacts, restart services, mutate launchd, fetch or merge upstream, invoke a
provider/model, read credentials, or send externally. It only makes the
terminal preflight coverage machine-readable so future edits cannot shrink the
verified tail unnoticed.
