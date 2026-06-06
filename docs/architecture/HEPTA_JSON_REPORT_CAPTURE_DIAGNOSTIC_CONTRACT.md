# Hepta JSON Report Capture Diagnostic Contract

Hepta's shell gates often compose other gates by extracting the first JSON
object from child output. When a child command fails, the parent gate must keep
the failure diagnosable without taking recovery actions.

This contract adds a reusable helper at
`scripts/lib/hepta-json-report-capture.sh` and a synthetic diagnostic gate at
`scripts/hepta-json-report-capture-diagnostic-contract-gate.sh`.

The helper contract is:

- preserve a successful child JSON report even when the child prints log lines
  before or after the report;
- when a child exits non-zero after emitting JSON, preserve the child exit code
  and print the parseable JSON report to stderr for diagnosis;
- when a child emits no JSON, print a bounded tail of the child output;
- tolerate stderr/stdout interleaving around the JSON report;
- preserve the first JSON object deterministically when later output contains
  additional JSON-shaped noise;
- expose a bounded tail for malformed JSON-shaped output instead of collapsing
  into a silent parse failure;
- optionally reuse successful reports from an ephemeral preflight cache when
  `HEPTA_JSON_REPORT_CAPTURE_CACHE_DIR` is set, so nested source gates are not
  rerun repeatedly in one full preflight;
- never write evidence, mutate the workspace, restart services, mutate launchd,
  read credentials, read secret files, invoke providers, or send externally.

The diagnostic gate uses synthetic child commands only. It does not call live
services and does not perform recovery. It exists so future preflight failures
point at the failing child report or output tail instead of collapsing into a
generic parse error.

The v3 diagnostic fixture matrix covers eight cases:

- successful JSON with surrounding log lines;
- child failure after a parseable JSON report;
- child success with no JSON report;
- stderr/stdout interleaving around a valid report;
- multiple JSON objects where only the first is the report;
- malformed JSON-shaped output;
- bounded diagnostic tail emission.
- ephemeral cache hit reuse: two identical successful captures return the same
  cached report while the synthetic child command runs only once.

The cache is opt-in and intended for `scripts/hepta-preflight.sh`. It uses a
temporary directory plus a per-run salt, and the preflight removes the directory
on exit. Cached reports are an execution optimization only; they are not
approval records, evidence persistence, operator delivery, or authority.
