# Hepta UI Actions queue recovery

At the Tranche 24 audit point the repository API reported 280 queued workflow
runs. The current PR #52 source job still had `runner_id=0` and `steps=[]`.
This is an account/repository execution-capacity blocker, not a source-code
failure.

`scripts/hepta-actions-prune-queued-ui-runs` is an admin-side, dry-run-first
recovery tool. It scopes to `codex/ui-v4-` by default, preserves the newest run
for every workflow/branch/event key, preserves explicitly protected run IDs,
and only proposes queued runs older than a grace period.

Execution requires a second invocation with the exact SHA-256 plan digest:

```text
GITHUB_TOKEN=... ruby scripts/hepta-actions-prune-queued-ui-runs \
  --protect-run-id 33178135102 \
  --output queue-plan.json

GITHUB_TOKEN=... ruby scripts/hepta-actions-prune-queued-ui-runs \
  --protect-run-id 33178135102 \
  --execute \
  --expected-digest <planDigest> \
  --output queue-execution.json
```

The tool can cancel workflow runs only. It does not change repository content,
product state, production authority, or release state. It is intentionally not
implemented as another Actions workflow because a workflow cannot recover a
queue when no runner can start it.
