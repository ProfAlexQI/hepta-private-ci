# WEB-C1 exact Servo source review candidate v2

Status: **implemented as fixture-only evidence tooling; no exact source candidate is accepted and no build is authorized**

## Purpose

The exact-source qualification workflow produces a two-fetch deterministic Servo source packet. A green workflow is necessary but not sufficient to accept those bytes. The v2 candidate compiler converts retained source bytes plus raw GitHub workflow, job, and artifact API responses into one self-bound review candidate.

The compiler never modifies the canonical accepted-source pointer. Its only successful decision is:

```text
EVIDENCE_COMPLETE_REVIEW_REQUIRED_BUILD_NOT_AUTHORIZED
```

## Required packet

One canonical evidence root contains:

```text
workflow-run.json
workflow-jobs.json
workflow-artifacts.json
source/
  SHA256SUMS
  fetch-a.receipt.json
  fetch-b.receipt.json
  independent-source-bundle.receipt.json
  license-packet.json
  servo-source-a.tar.gz
  source-bundle.verification.json
```

The three workflow API files are raw GitHub REST responses captured after the run. Source JSON files remain compact canonical JSON. `SHA256SUMS` contains only bytewise-sorted basenames, never absolute or relative directories.

## Workflow requirements

The compiler requires:

- the exact canonical v3 workflow path and name;
- `workflow_dispatch`, not a PR or push acquisition;
- one completed successful workflow run;
- the requested branch and exact Hepta commit/tree;
- one completed successful source job;
- a positive runner ID and non-empty runner name;
- all eight release-blocking source steps recorded and successful;
- exactly the receipts and compressed-source artifacts;
- both artifacts present, non-empty, and unexpired.

A job with `steps=[]`, `steps=null`, `runner_id=0`, no logs, or no retained artifacts is rejected. This explicitly prevents the repository's current Actions allocation failure from being mistaken for source evidence.

## Retained-byte requirements

The compiler:

1. validates a portable, sorted, basename-only `SHA256SUMS`;
2. rehashes every retained file;
3. checks the two acquisition nonces are distinct;
4. checks both fetches used standalone object stores without alternates;
5. checks all source-only authority remains false;
6. requires the offline verification receipt to bind and recompute the pinned tree;
7. reruns `hepta-servo-source-bundle-verify-v2.py` against the same retained bytes;
8. binds raw workflow API JSON, receipts, archive, Hepta commit/tree, workflow run/job, and artifact IDs into one domain-separated candidate ID.

## Separate review

Workflow success is necessary but not sufficient. A candidate remains:

```text
review.status=PENDING_SEPARATE_REVIEW
review.candidate_accepted=false
review.pointer_update_performed=false
claims.exact_servo_source_accepted=false
claims.build_authorized=false
```

A separate human-reviewed commit must compare the candidate against `C1_SOURCE_ACCEPTANCE_POLICY.md` and update a canonical accepted-source pointer. This compiler has no command that performs that update.

## Fail-closed coverage

The fixture suite covers:

- valid self-bound candidate generation;
- create-only private output;
- zero-step and runner-zero rejection;
- wrong head, workflow, or event rejection;
- missing and expired artifact rejection;
- absolute-path and unsorted checksum rejection;
- retained-byte digest drift;
- non-canonical source JSON;
- recomputed-tree drift;
- positive authority;
- duplicate JSON keys;
- candidate-ID tampering;
- hardlinked retained evidence.

No fixture output is source acceptance evidence.

## Authority

The compiler grants no source acceptance, topology acceptance, build recipe acceptance, build permission, runtime authority, external network, production caller, effect authority, operator acceptance, promotion, or release qualification.
