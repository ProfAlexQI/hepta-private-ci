# Exact source acceptance review lane

This directory is reserved for a future separately reviewed source-only pointer.

Expected final paths:

```text
ACCEPTED_SOURCE_POINTER.json
candidates/<candidate-id-digest>.json
challenges/<challenge-id-digest>.json
```

No accepted pointer exists in the current qualification-only candidate. Creating them is permitted only in a dedicated non-draft review PR that satisfies the self-bound policy and live status check.

A real proposal must:

1. open a non-draft `review/hepta-servo-source-acceptance-*` PR;
2. retain the deterministic candidate and challenge snapshots;
3. record the assigned PR number and exact head ref in the final pointer commit;
4. change only policy-allowed paths;
5. obtain a distinct current-head trusted-collaborator approval containing the exact challenge line;
6. pass `Source-only accepted pointer live review`.

The live verifier does not claim CODEOWNER identity from `author_association`; repository rules may independently add that requirement.

An accepted source pointer does not authorize a Servo build. Worker source/API topology acceptance remains a separate subsequent gate.
