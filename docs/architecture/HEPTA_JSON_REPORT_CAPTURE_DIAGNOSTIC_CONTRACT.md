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
- never write evidence, mutate the workspace, restart services, mutate launchd,
  read credentials, read secret files, invoke providers, or send externally.

The diagnostic gate uses synthetic child commands only. It does not call live
services and does not perform recovery. It exists so future preflight failures
point at the failing child report or output tail instead of collapsing into a
generic parse error.
