# Worker source/API topology acceptance lane

This directory is reserved for a future separately reviewed topology-only pointer.

Expected final paths:

```text
ACCEPTED_WORKER_SOURCE_TOPOLOGY_POINTER.json
receipts/<topology-receipt-id-digest>.json
challenges/<challenge-id-digest>.json
```

No accepted pointer or real receipt/challenge snapshot exists in the current qualification-only branch. Creation is allowed only after an accepted exact-source pointer exists and only in a dedicated, non-draft PR satisfying `WORKER_SOURCE_TOPOLOGY_ACCEPTANCE_REVIEW_POLICY_V1.json` and the trusted-base live-review workflow.

An accepted topology pointer does not authorize a build. Build recipe review, exact toolchain, sealed inputs, bounded offline build, artifact/SBOM/reproducibility, private launch and one-WebView qualification remain separate gates.
