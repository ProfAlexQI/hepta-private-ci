# Exact source acceptance review lane

This directory is reserved for a future separately reviewed source-only pointer.

Expected final paths:

```text
ACCEPTED_SOURCE_POINTER.json
candidates/<candidate-id-digest>.json
```

Neither path exists in the current qualification-only candidate. Creating them is permitted only in a dedicated non-draft review PR that satisfies `SOURCE_ACCEPTANCE_REVIEW_POLICY_V1.json` and the live review verifier.

An accepted source pointer does not authorize a Servo build. Worker source/API topology acceptance remains a separate subsequent gate.
