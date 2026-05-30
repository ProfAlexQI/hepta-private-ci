# Hepta Preflight Terminal Coverage Inventory Gate

The full light preflight is intentionally long. Its spine and terminal coverage
matter: metadata/fmt/check, adapter replay, name/repository closure, active
dependency isolation, legacy entrypoint migration, memory/intelligence closure,
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
- phase-family budgets remain satisfied across early core spine, legacy
  migration closure, KG prompt-preview readiness, live-mutation denial,
  upstream Codex absorption/activation, terminal governance/release, Core
  activation tail, JSON/terminal coverage, and latest regression/test families;
- each phase-family budget retains named anchor markers, so a family cannot
  stay green by replacing real gate anchors with generic count filler;
- the Core activation tail includes the terminal closure decision, terminal
  closure gap evidence index, operator packet template gate, and operator packet
  dry-run validator gate, so the blocked closure surface remains directly
  inspectable, mapped into a report-only packet entrance checklist, and guarded
  against future packet shapes being mistaken for approval;
- phase-family anchor evidence is summarized per family, including required,
  ready, and missing anchor counts plus missing anchor names, so operators can
  see which family lost which real gate anchor;
- critical markers are present exactly once and in order;
- early spine markers for fmt/check, adapters, name/repository closure, active
  dependency isolation, legacy entrypoint migration, and memory/intelligence
  closure remain wired before KG prompt-preview gates;
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
- missing early spine marker;
- duplicated required marker;
- out-of-order required marker;
- marker count budget shrinkage;
- phase-family budget shrinkage with all required markers still present;
- missing phase-family anchor with all required markers and count budgets still
  present, including per-family missing-anchor evidence;
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
