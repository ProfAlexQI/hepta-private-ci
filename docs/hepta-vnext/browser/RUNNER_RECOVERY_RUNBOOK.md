# Hepta Browser exact-head runner recovery runbook

Status: `DEVELOPMENT / L1_QUALIFICATION_ONLY`

This runbook restores **CI execution evidence only**. It never accepts Servo
source, authorizes a build, creates an artifact, qualifies runtime behavior,
grants operator authority, merges a pull request, promotes, or releases.

## Canonical scope

Repository:

```text
ProfAlexQI/hepta-private-ci
```

Required workflow/job pairs:

```text
blocking-ci                         / CI required
hepta-browser next required v9      / Hepta Browser next required v9
hepta-vnext qualification           / Hepta vNext required
```

The version-controlled policy is
`RUNNER_QUALIFICATION_POLICY_V2.json`; the recovery contract is
`RUNNER_RECOVERY_CONTRACT_V1.json`.

## 1. Administrative recovery checks

A repository or organization administrator must verify all of the following
outside PR-head code:

1. GitHub Actions is enabled for the repository and the pinned actions used by
   the three required graphs are allowed.
2. The account billing state and Actions spending limit permit hosted-runner
   allocation.
3. Obsolete queued runs are cancelled without cancelling the current exact-head
   required runs.
4. A current-head required job records a positive `runner_id` and at least one
   job step.
5. All three required workflows execute against the same current PR head SHA.

Queued, pending, `runner_id=0`, `steps=[]`, or `steps=null` is an environment
blocker. None is a PASS.

## 2. Capture exact GitHub API snapshots

Use a trusted operator workstation with GitHub CLI authentication. Set a
private output root and the exact current PR head:

```bash
set -euo pipefail
umask 077
repo=ProfAlexQI/hepta-private-ci
head="$(gh api "repos/${repo}/pulls/1" --jq .head.sha)"
out="runner-evidence-${head}"
test ! -e "${out}"
mkdir -m 700 "${out}"

gh api --paginate --slurp   "repos/${repo}/actions/runs?head_sha=${head}&per_page=100"   > "${out}/run-pages.json"
```

Convert paged responses into the classifier input without trusting shell text
processing:

```bash
python3 - "${out}/run-pages.json" "${out}/runs.json" <<'PY'
import json
import pathlib
import sys

source = pathlib.Path(sys.argv[1])
target = pathlib.Path(sys.argv[2])
pages = json.loads(source.read_text(encoding="utf-8"))
if not isinstance(pages, list) or not pages:
    raise SystemExit("run-pages snapshot must contain at least one page")
runs = []
for page in pages:
    if not isinstance(page, dict) or not isinstance(page.get("workflow_runs"), list):
        raise SystemExit("invalid workflow-runs page")
    runs.extend(page["workflow_runs"])
target.write_text(
    json.dumps({"workflow_runs": runs}, sort_keys=True, separators=(",", ":")),
    encoding="utf-8",
)
PY
```

Resolve the **latest exact-head run ID** for each canonical workflow from the
captured snapshot, then capture its jobs response:

```bash
python3 - "${out}/runs.json" "${head}" "${out}/selected-runs.json" <<'PY'
import json
import pathlib
import sys

runs_path = pathlib.Path(sys.argv[1])
head = sys.argv[2]
output_path = pathlib.Path(sys.argv[3])
payload = json.loads(runs_path.read_text(encoding="utf-8"))
required = [
    "blocking-ci",
    "hepta-browser next required v9",
    "hepta-vnext qualification",
]
selected = {}
for name in required:
    matches = [
        run for run in payload["workflow_runs"]
        if run.get("name") == name and run.get("head_sha") == head
    ]
    if matches:
        selected[name] = max(matches, key=lambda run: int(run["id"]))["id"]
output_path.write_text(
    json.dumps(selected, sort_keys=True, separators=(",", ":")),
    encoding="utf-8",
)
PY

while IFS=$'\t' read -r name run_id; do
  gh api "repos/${repo}/actions/runs/${run_id}/jobs?per_page=100"     > "${out}/jobs-${run_id}.json"
done < <(
  jq -r 'to_entries[] | [.key, (.value|tostring)] | @tsv'     "${out}/selected-runs.json"
)
```

If a required workflow is absent, do not invent a run ID or reuse an older
head. The classifier will retain a fail-closed blocker result.

## 3. Compile create-only evidence

Pass only the selected exact-head jobs snapshots:

```bash
args=()
while read -r run_id; do
  args+=(--jobs-json "${run_id}=${out}/jobs-${run_id}.json")
done < <(jq -r '.[]' "${out}/selected-runs.json")

python3 scripts/verify-hepta-browser-runner-evidence.py   --head-sha "${head}"   --runs-json "${out}/runs.json"   "${args[@]}"   --output "${out}/evidence.json"
```

Exit codes are contractual:

```text
0  PASS_CI_EXECUTION_ONLY
1  GATE_FAILURE
2  INVALID_EVIDENCE or malformed/tampered snapshot
3  ENVIRONMENT_BLOCKER_ZERO_STEPS or EXECUTING_NOT_QUALIFIED
```

Verify the self-bound receipt before retaining it:

```bash
python3 scripts/verify-hepta-browser-runner-evidence.py   --verify-evidence "${out}/evidence.json"
```

The evidence file is create-only, mode `0600`, fsynced, and contains a SHA-256
over its canonical unsigned content.

## 4. Closure rule

The Runner blocker closes only when the receipt has:

```json
{
  "disposition": "PASS_CI_EXECUTION_ONLY",
  "required_checks_passed": true
}
```

and every workflow observation simultaneously shows:

```text
exact head SHA
completed/success workflow
completed/success required job
runner_id > 0
one or more steps
all steps completed
no failed/cancelled/timed-out/startup-failure step
at least one successful step
jobs total_count exactly matches the captured list
repository/run-bound jobs_url
valid evidence_sha256
```

Operator rule: only after PASS_CI_EXECUTION_ONLY dispatch exact-source qualification v3.

Only then may the plan advance to canonical exact-source qualification v3.
All Browser, Servo, build, runtime, operator, merge, promotion, and release
authority remains false.
